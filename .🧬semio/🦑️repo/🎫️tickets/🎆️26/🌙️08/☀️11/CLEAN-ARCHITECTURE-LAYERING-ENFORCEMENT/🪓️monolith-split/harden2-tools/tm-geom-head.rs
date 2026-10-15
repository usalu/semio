mod decode_geometry_tests {
    use super::{GeomType, decode_geometry};

    fn zigzag(n: i32) -> u32 {
        ((n << 1) ^ (n >> 31)) as u32
    }

    fn varint(mut value: u64) -> Vec<u8> {
        let mut bytes = Vec::new();
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            bytes.push(if value == 0 { byte } else { byte | 0x80 });
            if value == 0 {
                return bytes;
            }
        }
    }

    fn push_varint_field(bytes: &mut Vec<u8>, tag: u8, value: u64) {
        bytes.extend(varint(u64::from(tag) << 3));
        bytes.extend(varint(value));
    }

    fn push_bytes_field(bytes: &mut Vec<u8>, tag: u8, value: &[u8]) {
        bytes.extend(varint((u64::from(tag) << 3) | 2));
        bytes.extend(varint(value.len() as u64));
        bytes.extend(value);
    }

    #[test]
    fn mvt_segment_is_tile_seam_on_extent_bbox() {
        let extent = 4096;
        assert!(super::mvt_segment_is_tile_seam(extent, (0.0, 0.0), (4096.0, 0.0)));
        assert!(super::mvt_segment_is_tile_seam(extent, (4096.0, 100.0), (4096.0, 900.0)));
        assert!(!super::mvt_segment_is_tile_seam(extent, (0.0, 0.0), (200.0, 200.0)));
        assert!(!super::mvt_segment_is_tile_seam(extent, (200.0, 200.0), (300.0, 400.0)));
        assert!(super::mvt_segment_touches_tile_bbox(extent, (0.0, 0.0), (200.0, 200.0)));
        assert!(!super::mvt_segment_touches_tile_bbox(extent, (200.0, 200.0), (300.0, 400.0)));
        assert!(super::mvt_ring_is_tile_bbox_cover(extent, &[(0.0, 0.0), (4096.0, 0.0), (4096.0, 4096.0), (0.0, 4096.0), (0.0, 0.0)],));
    }

    #[test]
    fn continent_water_filter_drops_inland_polygons() {
        let mut props = std::collections::BTreeMap::new();
        props.insert("class".to_string(), "lake".to_string());
        assert!(super::water_polygon_visible_for_lod(0, &props));
        assert!(!super::water_polygon_visible_for_lod(1, &props));
        assert!(super::water_polygon_visible_for_lod(2, &props));
        props.insert("class".to_string(), "ocean".to_string());
        assert!(super::water_polygon_visible_for_lod(1, &props));
        assert!(!super::waterway_visible_for_lod(1));
        assert!(super::waterway_visible_for_lod(2));
        assert!(super::country_polygon_holes_visible_for_lod(0));
        assert!(!super::country_polygon_holes_visible_for_lod(1));
        assert!(super::country_polygon_holes_visible_for_lod(2));
    }

    #[test]
    fn weighted_opaque_fill_keeps_alpha_solid() {
        let c = super::weighted_opaque_fill(super::Color::from_rgba8(40, 50, 60, 128), 1.0);
        assert_eq!(c.to_rgba8().a, 255);
    }

    #[test]
    fn linestring_moveto_starts_new_part() {
        let geometry = vec![(1 << 3) | 1, zigzag(0), zigzag(0), (1 << 3) | 2, zigzag(10), zigzag(0), (1 << 3) | 1, zigzag(90), zigzag(100), (1 << 3) | 2, zigzag(10), zigzag(0)];
        let lines = decode_geometry(&geometry, GeomType::LineString).lines;
        assert_eq!(lines.len(), 2, "each MoveTo must start a new line part");
        assert_eq!(lines[0], vec![(0.0, 0.0), (10.0, 0.0)]);
        assert_eq!(lines[1], vec![(100.0, 100.0), (110.0, 100.0)]);
    }

    #[test]
    fn owned_protobuf_decodes_layer_properties_and_point_geometry() {
        let mut value = Vec::new();
        push_bytes_field(&mut value, 1, b"road");

        let mut feature = Vec::new();
        push_varint_field(&mut feature, 1, 42);
        let mut tags = Vec::new();
        tags.extend(varint(0));
        tags.extend(varint(0));
        push_bytes_field(&mut feature, 2, &tags);
        push_varint_field(&mut feature, 3, 1);
        let mut geometry = Vec::new();
        geometry.extend(varint(9));
        geometry.extend(varint(zigzag(1) as u64));
        geometry.extend(varint(zigzag(2) as u64));
        push_bytes_field(&mut feature, 4, &geometry);

        let mut layer = Vec::new();
        push_bytes_field(&mut layer, 1, b"places");
        push_bytes_field(&mut layer, 2, &feature);
        push_bytes_field(&mut layer, 3, b"name");
        push_bytes_field(&mut layer, 4, &value);
        push_varint_field(&mut layer, 5, 8192);
        push_varint_field(&mut layer, 15, 2);

        let mut tile = Vec::new();
        push_bytes_field(&mut tile, 3, &layer);
        let decoded = super::decode_mvt(&tile).expect("owned protobuf tile");
        assert_eq!(decoded.layers.len(), 1);
        assert_eq!(decoded.layers[0].name, "places");
        assert_eq!(decoded.layers[0].extent, 8192);
        assert_eq!(decoded.layers[0].features[0].id, Some(42));
        assert_eq!(decoded.layers[0].features[0].geom_type, GeomType::Point);
        assert_eq!(decoded.layers[0].features[0].points, [(1.0, 2.0)]);
        assert_eq!(decoded.layers[0].features[0].properties.get("name").map(String::as_str), Some("road"));
    }

    #[test]
    fn owned_protobuf_skips_unknown_fields_and_rejects_malformed_wire_data() {
        let mut empty_layer = Vec::new();
        empty_layer.extend(varint((9 << 3) | 5));
        empty_layer.extend([1, 2, 3, 4]);
        let mut tile = Vec::new();
        push_bytes_field(&mut tile, 3, &empty_layer);
        assert_eq!(super::decode_mvt(&tile).expect("unknown fixed field").layers.len(), 1);
        assert!(super::decode_mvt(&[0x1a, 0x02, 0x0a]).is_err());
        assert!(super::decode_mvt(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x02]).is_err());
    }
}
