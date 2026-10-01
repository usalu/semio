//! 🗺️ Schema-owned Web Mercator tile planning with a finite coordinate inventory.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapTileBounds { pub west: f64, pub south: f64, pub east: f64, pub north: f64 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapTileCoordinate { pub z: u32, pub x: u32, pub y: u32 }

/// 🧭️ Resolves one finite geographic coordinate into the owner's bounded tile lattice.
pub fn map_tile_coordinate(longitude: f64, latitude: f64, zoom: u32) -> Result<MapTileCoordinate, String> {
    if !longitude.is_finite() || !latitude.is_finite() || !(-180.0..=180.0).contains(&longitude) || latitude.abs() > 85.0511287798066 || zoom > 19 { return Err("invalid map tile coordinate".into()); }
    let n = f64::from(1_u32 << zoom);
    let latitude = latitude.to_radians();
    let x = ((longitude + 180.0) / 360.0 * n).floor().clamp(0.0, n - 1.0) as u32;
    let y = ((1.0 - (latitude.tan() + 1.0 / latitude.cos()).ln() / std::f64::consts::PI) / 2.0 * n).floor().clamp(0.0, n - 1.0) as u32;
    Ok(MapTileCoordinate { z: zoom, x, y })
}

/// 📋️ Admits the complete coordinate count before allocating or enumerating its jobs.
pub fn map_tile_plan(bounds: MapTileBounds, minimum: u32, maximum: u32, limit: usize) -> Result<Vec<MapTileCoordinate>, String> {
    if minimum > maximum || maximum > 19 || limit == 0 || limit > 65_536 || bounds.west > bounds.east || bounds.south > bounds.north { return Err("invalid map tile plan".into()); }
    let mut ranges = Vec::new();
    let mut count = 0_u64;
    for zoom in minimum..=maximum {
        let southwest = map_tile_coordinate(bounds.west, bounds.south, zoom)?;
        let northeast = map_tile_coordinate(bounds.east, bounds.north, zoom)?;
        let (x0, x1) = (southwest.x.min(northeast.x), southwest.x.max(northeast.x));
        let (y0, y1) = (southwest.y.min(northeast.y), southwest.y.max(northeast.y));
        count += u64::from(x1 - x0 + 1) * u64::from(y1 - y0 + 1);
        if count > limit as u64 { return Err("map tile plan exceeds its job budget".into()); }
        ranges.push((zoom, x0, x1, y0, y1));
    }
    let mut output = Vec::with_capacity(count as usize);
    for (z, x0, x1, y0, y1) in ranges { for x in x0..=x1 { for y in y0..=y1 { output.push(MapTileCoordinate { z, x, y }); } } }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::BoundingRect;
    #[derive(serde::Deserialize)]
    struct WireBounds { west: f64, south: f64, east: f64, north: f64 }
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct WirePlan { bounds: WireBounds, z_min: u32, z_max: u32, max_tiles: Option<usize> }
    impl WirePlan {
        fn resolve(self) -> Result<Vec<MapTileCoordinate>, String> {
            map_tile_plan(MapTileBounds { west: self.bounds.west, south: self.bounds.south, east: self.bounds.east, north: self.bounds.north }, self.z_min, self.z_max, self.max_tiles.unwrap_or(65_536))
        }
    }
    #[test]
    fn portable_tile_coordinates_and_ranges_match_json_and_geo_oracles() {
        let corpus: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("portable corpus");
        for row in corpus["coordinates"].as_array().unwrap() {
            let tile = map_tile_coordinate(row["longitude"].as_f64().unwrap(), row["latitude"].as_f64().unwrap(), row["zoom"].as_u64().unwrap() as u32).unwrap();
            assert_eq!((tile.x, tile.y), (row["expected"]["x"].as_u64().unwrap() as u32, row["expected"]["y"].as_u64().unwrap() as u32));
        }
        for row in corpus["plans"].as_array().unwrap() {
            let input = &row["input"];
            let b = &input["bounds"];
            let bounds = MapTileBounds { west: b["west"].as_f64().unwrap(), south: b["south"].as_f64().unwrap(), east: b["east"].as_f64().unwrap(), north: b["north"].as_f64().unwrap() };
            let tiles = map_tile_plan(bounds, input["zMin"].as_u64().unwrap() as u32, input["zMax"].as_u64().unwrap() as u32, 65_536).unwrap();
            assert_eq!(tiles.len(), row["count"].as_u64().unwrap() as usize);
            let points = geo::LineString::from(tiles.iter().map(|t| (f64::from(t.x), f64::from(t.y))).collect::<Vec<_>>());
            let rect = points.bounding_rect().unwrap();
            assert_eq!((rect.min().x, rect.min().y), (row["first"]["x"].as_f64().unwrap(), row["first"]["y"].as_f64().unwrap()));
            assert_eq!((rect.max().x, rect.max().y), (row["last"]["x"].as_f64().unwrap(), row["last"]["y"].as_f64().unwrap()));
        }
        for row in corpus["hostile"].as_array().unwrap() {
            let admitted = serde_json::from_value::<WirePlan>(row.clone()).map_err(|error| error.to_string()).and_then(WirePlan::resolve);
            assert!(admitted.is_err(), "hostile portable tile plan: {row}");
        }
        let schema: serde_json::Value = serde_json::from_str(include_str!("🧬️schema/🔣️.json")).expect("tile plan schema");
        let zoom_limit = schema["properties"]["zMax"]["maximum"].as_u64().unwrap() as u32;
        let latitude_limit = schema["properties"]["bounds"]["properties"]["north"]["maximum"].as_f64().unwrap();
        let job_limit = schema["properties"]["maxTiles"]["maximum"].as_u64().unwrap() as usize;
        assert!(map_tile_coordinate(0.0, latitude_limit, zoom_limit).is_ok());
        assert!(map_tile_coordinate(0.0, latitude_limit + 0.000001, zoom_limit).is_err());
        assert!(map_tile_coordinate(0.0, 0.0, zoom_limit + 1).is_err());
        let point = MapTileBounds { west: 0.0, south: 0.0, east: 0.0, north: 0.0 };
        assert!(map_tile_plan(point, 0, 0, job_limit).is_ok());
        assert!(map_tile_plan(point, 0, 0, job_limit + 1).is_err());
        assert!(map_tile_coordinate(f64::NAN, 0.0, 0).is_err());
        assert!(map_tile_plan(MapTileBounds { west: -180.0, south: -80.0, east: 180.0, north: 80.0 }, 19, 19, 65_536).is_err());
    }
}
