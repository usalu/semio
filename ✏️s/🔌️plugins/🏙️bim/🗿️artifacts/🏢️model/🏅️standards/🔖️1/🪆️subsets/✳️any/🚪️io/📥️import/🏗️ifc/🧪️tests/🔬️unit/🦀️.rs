use super::*;
use crate::{Axis, LocationLine, Point2, SpaceBoundary, TopConstraint};
use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, model_to_part21};
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{house, psets};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance};
use semio_framework_os_kernel::io::io_mechanism::Deserializer;

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
        assert!((crate::standards::v1::subsets::any::schema::authored::plan::axis_length(&row.axis) - crate::standards::v1::subsets::any::schema::authored::plan::axis_length(&wall.axis)).abs() < 1e-8, "{id}");
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
    assert!(matches!(beam.axis, crate::Axis::Line { start, end } if (start.x - 4.0).abs() < 1e-9 && (end.x - 6.0).abs() < 1e-9) && beam.top_offset.abs() < 1e-9 && beam.end_top_offset.is_none());
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
    assert_eq!(back.classification_systems, model.classification_systems);
    for (id, material) in &model.materials {
        let row = &back.materials[id];
        assert_eq!((row.name.as_str(), row.category), (material.name.as_str(), material.category), "{id}");
        assert!((row.density - material.density).abs() < 1e-9 && (row.color.g - material.color.g).abs() < 1e-9, "{id}");
    }
}

#[test]
fn every_family_of_the_house_comes_back_from_its_authored_record() {
    let model = house();
    let (back, notes) = imported(&model);
    assert_eq!((&back.roofs, &back.stairs, &back.railings, &back.curtain_walls, &back.curtain_panel_overrides), (&model.roofs, &model.stairs, &model.railings, &model.curtain_walls, &model.curtain_panel_overrides));
    assert_eq!(back.slabs.keys().collect::<Vec<_>>(), model.slabs.keys().collect::<Vec<_>>(), "the sloped balcony comes back from its record, the roof layers are parts");
    assert_eq!(back.spaces.keys().collect::<Vec<_>>(), model.spaces.keys().collect::<Vec<_>>());
    for class in ["IFCROOF", "IFCSTAIR", "IFCSTAIRFLIGHT", "IFCRAILING", "IFCCURTAINWALL", "IFCMEMBER", "IFCPLATE"] {
        assert!(notes.iter().all(|note| !note.starts_with(class)), "{class} is imported: {notes:?}");
    }
    assert!(back.walls.contains_key("w-south"));
}

#[semio_framework_async_macros::async_test]
async fn the_deserializer_sniffs_ifc_2x3_and_returns_the_model_with_diagnostics() {
    let (bytes, _) = export_ifc2x3(&house()).expect("the house exports");
    assert_eq!(Ifc2x3IntoModel::sniff(&IoPayload::Binary(bytes.clone())).await, Confidence::High);
    assert_eq!(Ifc2x3IntoModel::sniff(&IoPayload::Text("hello".into())).await, Confidence::None);
    let outcome = Ifc2x3IntoModel::deserialize(&IoPayload::Binary(bytes)).await.expect("it imports");
    assert!(outcome.value.walls.contains_key("w-south"));
    assert!(outcome.diagnostics.iter().all(|diagnostic| !diagnostic.message.starts_with("IFCROOF")));
    let broken = Ifc2x3IntoModel::deserialize(&IoPayload::Text("not a file".into())).await;
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

#[test]
fn the_phase_of_every_importable_element_survives_the_round_trip() {
    use crate::Phase;
    let mut model = importable();
    macro_rules! cycle {
        ($($collection:ident),+) => { $( for (index, row) in model.$collection.values_mut().enumerate() { row.phase = Phase::ALL[(index + 1) % 4]; } )+ };
    }
    cycle!(walls, columns, beams, slabs, spaces);
    let (back, _) = imported(&model);
    macro_rules! same {
        ($($collection:ident),+) => { $( for (id, row) in &model.$collection { assert_eq!(back.$collection[id].phase, row.phase, "{id}"); } )+ };
    }
    same!(walls, columns, beams, slabs, spaces);
    assert!(model.walls.values().any(|wall| wall.phase == Phase::Demolished) && model.slabs.values().any(|slab| slab.phase != Phase::New));
    let (first, _) = export_ifc2x3(&model).expect("first export");
    let (second, _) = export_ifc2x3(&back).expect("second export");
    assert_eq!(first, second, "the phases keep the round trip byte-stable");
}

/// 🏷️ The part of the psets model that the import understands: the house without roofs, stairs, railings, curtain walls, the sloped slab and the inferred space.
fn psets_importable() -> ModelSnapshot {
    let mut model = psets();
    model.roofs.clear();
    model.stairs.clear();
    model.railings.clear();
    model.curtain_walls.clear();
    model.slabs.remove("sl-balcony");
    model.spaces.remove("sp-1");
    model
}

/// 🌍️ The psets model exported, changed as a foreign tool would write it, and imported.
fn foreign(change: impl FnOnce(&mut Part21Document)) -> (ModelSnapshot, Vec<String>) {
    let mut document = model_to_part21(crate::standards::v1::subsets::any::io::export::ifc::Schema::Ifc2x3, &psets_importable()).expect("the model exports").0;
    change(&mut document);
    import_ifc2x3(&codec::encode_document(document).expect("the document encodes")).expect("the file imports")
}

fn instance_where(document: &Part21Document, entity: &str, matches: impl Fn(&[Part21Value]) -> bool) -> usize {
    document.instances.iter().position(|instance| instance.entity(entity).is_some_and(|args| matches(args))).unwrap_or_else(|| panic!("an {entity} instance"))
}

fn args_of(document: &mut Part21Document, at: usize) -> &mut Vec<Part21Value> {
    &mut document.instances[at].entities[0].1
}

fn text_is(value: &Part21Value, expected: &str) -> bool {
    value.as_str() == Some(expected)
}

#[test]
fn classification_systems_with_parents_and_many_codes_per_holder_survive_the_round_trip() {
    let model = psets_importable();
    let (back, notes) = imported(&model);
    assert!(notes.is_empty(), "everything exported is understood: {notes:?}");
    assert_eq!(back.classification_systems, model.classification_systems, "the tables, parents, editions and sources come back");
    assert_eq!(back.classifications, model.classifications);
    assert_eq!(back.classifications["w-south"].len(), 2, "one code per system, many systems per holder");
    assert_eq!(back.classifications["wt-300"]["cs-din-276"], "330", "a type record carries its own codes");
    assert_eq!(back.classifications["st-ground"]["cs-din-276"], "300", "so does a storey");
    assert_eq!(back.classification_systems["cs-din-276"].lineage("331").iter().map(|entry| entry.code.as_str()).collect::<Vec<_>>(), ["300", "330", "331"]);
}

#[test]
fn type_level_property_sets_come_back_on_the_type_record() {
    let model = psets_importable();
    let (back, _) = imported(&model);
    for id in ["wt-300", "dr-180", "wnd-120", "ct-rect"] {
        assert_eq!(back.properties[id], model.properties[id], "{id}");
    }
    assert_eq!(back.properties, model.properties, "own sets of elements and sets of types, nothing inherited is added to an instance");
}

#[test]
fn export_import_export_of_the_psets_model_is_byte_stable() {
    let model = psets_importable();
    let (first, _) = export_ifc2x3(&model).expect("first export");
    let (back, notes) = import_ifc2x3(&first).expect("the import");
    assert!(notes.is_empty(), "{notes:?}");
    assert_eq!(export_ifc2x3(&back).expect("second export").0, first);
}

#[test]
fn a_reference_without_a_classification_is_skipped_with_a_note() {
    let (back, notes) = foreign(|document| {
        let at = instance_where(document, "IFCCLASSIFICATIONREFERENCE", |args| text_is(&args[1], "331"));
        args_of(document, at)[3] = Part21Value::Unset;
    });
    assert!(notes.iter().any(|note| note.starts_with("IFCCLASSIFICATIONREFERENCE 331:")), "{notes:?}");
    assert!(!back.classifications["w-south"].contains_key("cs-din-276"), "the code of the skipped reference is not assigned");
    assert!(back.classification_systems["cs-din-276"].entry("331").is_none());
    assert_eq!(back.classifications["w-south"]["cs-uniclass-2015"], "EF_25_10");
}

#[test]
fn a_holder_with_two_codes_of_one_system_keeps_the_first_and_notes_the_other() {
    let (back, notes) = foreign(|document| {
        let wall = document.instances.iter().find(|instance| instance.primary().is_some_and(|(name, args)| name.starts_with("IFCWALL") && args.get(7).is_some_and(|tag| text_is(tag, "w-east")))).map(|instance| instance.id).expect("the east wall");
        let reference = document.instances[instance_where(document, "IFCCLASSIFICATIONREFERENCE", |args| text_is(&args[1], "EF_25"))].id;
        let owner = document.instances[instance_where(document, "IFCRELASSOCIATESCLASSIFICATION", |_| true)].entities[0].1[1].clone();
        let id = document.instances.iter().map(|instance| instance.id).max().unwrap_or(0) + 1;
        document.instances.push(Part21Instance { id, entities: vec![("IFCRELASSOCIATESCLASSIFICATION".to_string(), vec![Part21Value::Str("3vB2YO$MX4xv5uCqZZG05x".into()), owner, Part21Value::Unset, Part21Value::Unset, Part21Value::List(vec![Part21Value::Ref(wall)]), Part21Value::Ref(reference)])] });
    });
    assert_eq!(back.classifications["w-east"]["cs-uniclass-2015"], "EF_25_10");
    assert!(notes.iter().any(|note| note.starts_with("classification w-east: the system cs-uniclass-2015 already has the code EF_25_10; EF_25 is ignored")), "{notes:?}");
}

#[test]
fn two_classifications_of_one_name_get_distinct_system_ids() {
    let (back, _) = foreign(|document| {
        let at = instance_where(document, "IFCCLASSIFICATION", |args| text_is(&args[3], "DIN 276"));
        let mut copy = document.instances[at].clone();
        copy.id = document.instances.iter().map(|instance| instance.id).max().unwrap_or(0) + 1;
        copy.entities[0].1[1] = Part21Value::Str("2008".into());
        document.instances.push(copy);
    });
    assert_eq!(back.classification_systems["cs-din-276"].edition, "2018-12");
    assert_eq!(back.classification_systems["cs-din-276-2"].edition, "2008");
    assert!(back.classification_systems["cs-din-276-2"].entries.is_empty());
}

#[test]
fn a_parent_column_that_is_no_forest_is_dropped_for_its_system_only() {
    let (back, notes) = foreign(|document| {
        let at = instance_where(document, "IFCPROPERTYSINGLEVALUE", |args| text_is(&args[0], "DIN 276|2018-12|330"));
        args_of(document, at)[2] = Part21Value::Typed { name: "IFCLABEL".into(), items: vec![Part21Value::Str("331".into())] };
    });
    assert!(notes.iter().any(|note| note.starts_with("classification cs-din-276: the parent column is dropped")), "{notes:?}");
    assert!(back.classification_systems["cs-din-276"].entries.iter().all(|entry| entry.parent.is_none()));
    assert_eq!(back.classification_systems["cs-uniclass-2015"].entry("EF_25_10").and_then(|entry| entry.parent.as_deref()), Some("EF_25"));
}

#[test]
fn a_file_without_the_parents_set_imports_flat_tables_with_every_row() {
    let (back, notes) = foreign(|document| {
        let at = instance_where(document, "IFCPROPERTYSET", |args| text_is(&args[2], "Semio_ClassificationParents"));
        args_of(document, at)[2] = Part21Value::Str("Other".into());
    });
    assert!(notes.is_empty(), "{notes:?}");
    assert_eq!(back.classification_systems["cs-din-276"].entries.len(), 4, "the unattached row 340 is kept");
    assert!(back.classification_systems.values().flat_map(|system| system.entries.iter()).all(|entry| entry.parent.is_none()));
    assert_eq!(back.classifications["w-south"].len(), 2);
}
