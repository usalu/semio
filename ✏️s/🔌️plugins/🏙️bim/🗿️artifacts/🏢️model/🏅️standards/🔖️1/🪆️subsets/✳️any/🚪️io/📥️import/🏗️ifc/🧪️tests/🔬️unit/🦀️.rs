use super::*;
use crate::{Axis, LocationLine, Point2, SpaceBoundary, TopConstraint};
use crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::house;
use semio_framework::io::io_mechanism::Deserializer;

/// 🔁️ The part of the house that the import understands: no roof, stair, railing, curtain wall, sloped slab or inferred space.
fn importable() -> ModelSnapshot {
    let mut model = house();
    model.roofs.clear();
    model.stairs.clear();
    model.railings.clear();
    model.curtain_walls.clear();
    model.slabs.remove("sl-balcony");
    model.spaces.remove("sp-1");
    model
}

fn imported(model: &ModelSnapshot) -> (ModelSnapshot, Vec<String>) {
    let (bytes, _) = export_ifc2x3(model).expect("the model exports");
    import_ifc2x3(&bytes).expect("the file imports")
}

#[test]
fn export_import_export_is_byte_stable() {
    let model = importable();
    let (first, _) = export_ifc2x3(&model).expect("first export");
    let (back, notes) = import_ifc2x3(&first).expect("the import");
    assert!(notes.is_empty(), "everything exported is understood: {notes:?}");
    let (second, _) = export_ifc2x3(&back).expect("second export");
    assert_eq!(String::from_utf8_lossy(&first).lines().count(), String::from_utf8_lossy(&second).lines().count(), "same number of lines");
    assert_eq!(first, second, "the imported model exports to the same bytes");
}

#[test]
fn the_spatial_structure_survives_with_levels_heights_and_geographic_position() {
    let model = importable();
    let (back, _) = imported(&model);
    assert_eq!(back.project, model.project);
    let (site, original) = (&back.sites["site-1"], &model.sites["site-1"]);
    assert!((site.latitude - original.latitude).abs() < 1e-9 && (site.longitude - original.longitude).abs() < 1e-9);
    assert_eq!((site.elevation, site.name.as_str(), site.boundary.len()), (408.0, "Plot", 4));
    assert!((site.true_north - 0.2).abs() < 1e-9);
    let (building, original) = (&back.buildings["bldg-1"], &model.buildings["bldg-1"]);
    assert_eq!((building.origin, building.elevation, building.name.as_str()), (original.origin, original.elevation, "House"));
    assert!((building.rotation - original.rotation).abs() < 1e-9);
    for (id, storey) in &model.storeys {
        let row = &back.storeys[id];
        assert_eq!((row.level, row.name.as_str(), row.building.as_str()), (storey.level, storey.name.as_str(), storey.building.as_str()), "{id}");
        assert!((row.height - storey.height).abs() < 1e-9, "{id}");
    }
}

#[test]
fn walls_keep_axis_type_location_height_and_phase() {
    let model = importable();
    let (back, _) = imported(&model);
    assert_eq!(back.walls.keys().collect::<Vec<_>>(), model.walls.keys().collect::<Vec<_>>());
    for (id, wall) in &model.walls {
        let row = &back.walls[id];
        assert_eq!((row.storey.as_str(), row.wall_type.as_str(), row.location, row.phase), (wall.storey.as_str(), wall.wall_type.as_str(), wall.location, wall.phase), "{id}");
        assert!((crate::standards::v1::subsets::any::schema::inferences::wall_layout::axis_length(&row.axis) - crate::standards::v1::subsets::any::schema::inferences::wall_layout::axis_length(&wall.axis)).abs() < 1e-8, "{id}");
    }
    let free = &back.walls["w-free"];
    assert!(matches!(free.top, TopConstraint::Unconnected { height } if (height - 2.4).abs() < 1e-9));
    assert!((free.base_offset - 0.1).abs() < 1e-9);
    assert!(matches!(back.walls["w-arc"].axis, Axis::Arc { bulge, .. } if (bulge - 0.4).abs() < 1e-8));
    assert_eq!(back.wall_types, model.wall_types, "layer sets, materials and layer functions come back");
}

#[test]
fn openings_keep_host_kind_offset_size_overrides_and_flips() {
    let model = importable();
    let (back, _) = imported(&model);
    assert_eq!(back.openings.keys().collect::<Vec<_>>(), model.openings.keys().collect::<Vec<_>>());
    for (id, opening) in &model.openings {
        let row = &back.openings[id];
        assert_eq!((&row.host, &row.kind, row.flip_hand, row.flip_facing), (&opening.host, &opening.kind, opening.flip_hand, opening.flip_facing), "{id}");
        assert!((row.offset - opening.offset).abs() < 1e-6, "{id}: {} against {}", row.offset, opening.offset);
        assert_eq!(row.sill_override.map(|sill| (sill * 1e6).round()), opening.sill_override.map(|sill| (sill * 1e6).round()), "{id}");
        assert_eq!((row.width.map(|w| (w * 1e6).round()), row.height.map(|h| (h * 1e6).round())), (opening.width.map(|w| (w * 1e6).round()), opening.height.map(|h| (h * 1e6).round())), "{id}");
    }
    assert_eq!(back.window_types, model.window_types);
    assert_eq!(back.door_types, model.door_types);
}

#[test]
fn slabs_columns_beams_spaces_and_grids_keep_their_geometry_and_types() {
    let model = importable();
    let (back, _) = imported(&model);
    let slab = &back.slabs["sl-ground"];
    assert_eq!((slab.slab_type.as_str(), slab.boundary.len(), slab.holes.len()), ("st-floor", 4, 1));
    assert!((slab.offset - model.slabs["sl-ground"].offset).abs() < 1e-9);
    let column = &back.columns["c-1"];
    assert_eq!((column.column_type.as_str(), column.storey.as_str()), ("ct-rect", "st-ground"));
    assert!((column.rotation - 0.3).abs() < 1e-9 && (column.position.x - 4.0).abs() < 1e-9);
    assert!(matches!(back.columns["c-2"].top, TopConstraint::Unconnected { height } if (height - 2.5).abs() < 1e-9));
    assert_eq!(back.column_types, model.column_types);
    let beam = &back.beams["b-1"];
    assert_eq!(beam.beam_type, "bt-i");
    assert!((beam.start.x - 4.0).abs() < 1e-9 && (beam.end.x - 6.0).abs() < 1e-9 && beam.top_offset.abs() < 1e-9);
    assert!((back.beams["b-2"].top_offset + 0.1).abs() < 1e-9);
    assert_eq!(back.beam_types, model.beam_types);
    assert!(matches!(&back.spaces["sp-2"].boundary, SpaceBoundary::Explicit { outline } if outline.len() == 4));
    assert_eq!((back.spaces["sp-2"].number.as_str(), back.spaces["sp-2"].name.as_str(), back.spaces["sp-2"].usage.as_str()), ("1.01", "Bedroom", "bedroom"));
    assert_eq!(back.grids.keys().collect::<Vec<_>>(), model.grids.keys().collect::<Vec<_>>());
    assert_eq!(back.grids["g-b"].label, "B");
}

#[test]
fn properties_classifications_and_materials_come_back() {
    let model = importable();
    let (back, _) = imported(&model);
    assert_eq!(back.properties, model.properties);
    assert_eq!(back.classifications, model.classifications);
    for (id, material) in &model.materials {
        let row = &back.materials[id];
        assert_eq!((row.name.as_str(), row.category), (material.name.as_str(), material.category), "{id}");
        assert!((row.density - material.density).abs() < 1e-9 && (row.color.g - material.color.g).abs() < 1e-9, "{id}");
    }
}

#[test]
fn what_has_no_parametric_equivalent_is_reported_with_its_count() {
    let (back, notes) = imported(&house());
    for class in ["IFCROOF", "IFCSTAIR", "IFCSTAIRFLIGHT", "IFCRAILING", "IFCCURTAINWALL", "IFCMEMBER", "IFCPLATE"] {
        assert!(notes.iter().any(|note| note.starts_with(class)), "{class} is reported: {notes:?}");
    }
    assert!(notes.iter().any(|note| note.starts_with("IFCSLAB")), "the roof slabs and the sloped balcony are reported");
    assert!(back.roofs.is_empty() && back.stairs.is_empty());
    assert!(back.walls.contains_key("w-south"), "the rest is still imported");
}

#[semio_framework_async_macros::async_test]
async fn the_deserializer_sniffs_ifc_2x3_and_returns_the_model_with_diagnostics() {
    let (bytes, _) = export_ifc2x3(&house()).expect("the house exports");
    assert_eq!(IfcIntoModel::sniff(&IoPayload::Binary(bytes.clone())).await, Confidence::High);
    assert_eq!(IfcIntoModel::sniff(&IoPayload::Text("hello".into())).await, Confidence::None);
    let outcome = IfcIntoModel::deserialize(&IoPayload::Binary(bytes)).await.expect("it imports");
    assert!(outcome.value.walls.contains_key("w-south"));
    assert!(outcome.diagnostics.iter().any(|diagnostic| diagnostic.message.starts_with("IFCROOF")));
    let broken = IfcIntoModel::deserialize(&IoPayload::Text("not a file".into())).await;
    assert!(broken.is_err());
}

const FOREIGN: &str = "ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');
FILE_NAME('f.ifc','',(''),(''),'x','x','');
FILE_SCHEMA(('IFC2X3'));
ENDSEC;
DATA;
#1=IFCPROJECT('p',$,'Foreign',$,$,$,$,$,$);
#2=IFCSITE('s',$,'Site',$,$,#20,$,$,.ELEMENT.,(47,0,0),(8,0,0),12.5,$,$);
#3=IFCBUILDING('b',$,'Building',$,$,#20,$,$,.ELEMENT.,0.,$,$);
#4=IFCBUILDINGSTOREY('g',$,'Ground',$,$,#21,$,$,.ELEMENT.,0.);
#5=IFCBUILDINGSTOREY('u',$,'Upper',$,$,#22,$,$,.ELEMENT.,3.2);
#6=IFCRELAGGREGATES('a1',$,$,$,#1,(#2));
#7=IFCRELAGGREGATES('a2',$,$,$,#2,(#3));
#8=IFCRELAGGREGATES('a3',$,$,$,#3,(#4,#5));
#10=IFCCARTESIANPOINT((0.,0.,0.));
#11=IFCCARTESIANPOINT((0.,0.,3.2));
#12=IFCAXIS2PLACEMENT3D(#10,$,$);
#13=IFCAXIS2PLACEMENT3D(#11,$,$);
#20=IFCLOCALPLACEMENT($,#12);
#21=IFCLOCALPLACEMENT(#20,#12);
#22=IFCLOCALPLACEMENT(#20,#13);
#30=IFCMATERIAL('Brick');
#31=IFCMATERIALLAYER(#30,0.24,$);
#32=IFCMATERIALLAYERSET((#31),'Brick 240');
#33=IFCCARTESIANPOINT((0.,0.));
#34=IFCCARTESIANPOINT((5.,0.));
#35=IFCPOLYLINE((#33,#34));
#36=IFCSHAPEREPRESENTATION($,'Axis','Curve2D',(#35));
#37=IFCCARTESIANPOINT((2.5,0.));
#38=IFCAXIS2PLACEMENT2D(#37,$);
#39=IFCRECTANGLEPROFILEDEF(.AREA.,$,#38,5.,0.24);
#40=IFCDIRECTION((0.,0.,1.));
#41=IFCEXTRUDEDAREASOLID(#39,#12,#40,2.8);
#42=IFCSHAPEREPRESENTATION($,'Body','SweptSolid',(#41));
#43=IFCPRODUCTDEFINITIONSHAPE($,$,(#36,#42));
#44=IFCCARTESIANPOINT((1.,2.,0.));
#45=IFCAXIS2PLACEMENT3D(#44,$,$);
#46=IFCLOCALPLACEMENT(#21,#45);
#47=IFCWALLSTANDARDCASE('w',$,'Wall A',$,$,#46,#43,$);
#48=IFCRELCONTAINEDINSPATIALSTRUCTURE('c',$,$,$,(#47),#4);
#49=IFCMATERIALLAYERSETUSAGE(#32,.AXIS2.,.POSITIVE.,-0.12);
#50=IFCRELASSOCIATESMATERIAL('m',$,$,$,(#47),#49);
#51=IFCFURNISHINGELEMENT('f',$,'Chair',$,$,#46,$,$);
#52=IFCRELCONTAINEDINSPATIALSTRUCTURE('c2',$,$,$,(#51),#4);
ENDSEC;
END-ISO-10303-21;
";

#[test]
fn a_foreign_file_without_semio_identity_is_imported_with_derived_levels_types_and_ids() {
    let (model, notes) = import_ifc2x3(FOREIGN.as_bytes()).expect("the foreign file imports");
    assert_eq!(model.project.name, "Foreign");
    let site = model.sites.values().next().expect("a site");
    assert_eq!((site.latitude, site.longitude, site.elevation), (47.0, 8.0, 12.5));
    let mut storeys: Vec<_> = model.storeys.values().map(|row| (row.name.as_str(), row.level, (row.height * 10.0).round() / 10.0)).collect();
    storeys.sort_by_key(|row| row.1);
    assert_eq!(storeys, [("Ground", 0, 3.2), ("Upper", 1, 3.0)], "heights from the elevation gap, the top storey gets the default");
    let (id, wall) = model.walls.iter().next().expect("a wall");
    assert!(id.starts_with("w-"), "{id}");
    let kind = &model.wall_types[&wall.wall_type];
    assert_eq!((kind.layers.len(), kind.layers[0].thickness), (1, 0.24));
    assert_eq!(model.materials.values().next().map(|material| material.name.as_str()), Some("Brick"));
    assert_eq!(wall.location, LocationLine::Center, "a positive layer set starting 0.12 right of the axis spans the 0.24 thickness symmetrically");
    let Axis::Line { start, end } = wall.axis else { panic!("a straight axis") };
    assert_eq!((start, end), (Point2 { x: 1.0, y: 2.0 }, Point2 { x: 6.0, y: 2.0 }));
    assert!(matches!(wall.top, TopConstraint::Unconnected { height } if (height - 2.8).abs() < 1e-9));
    assert!(notes.iter().any(|note| note.starts_with("IFCFURNISHINGELEMENT: 1")), "{notes:?}");
}
