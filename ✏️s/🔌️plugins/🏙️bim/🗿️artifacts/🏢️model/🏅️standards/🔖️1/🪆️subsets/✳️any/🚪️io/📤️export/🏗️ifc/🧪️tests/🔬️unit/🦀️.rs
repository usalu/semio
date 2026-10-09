use super::testkit::{document, house, quantity, rows, string, tags};
use super::*;
use protocol::Inference;
use semio_s_artifact_stdio_ifc::part21::Part21Value;
use std::collections::BTreeSet;

fn walk(value: &Part21Value, visit: &mut impl FnMut(u64)) {
    match value {
        Part21Value::Ref(id) => visit(*id),
        Part21Value::List(items) | Part21Value::Typed { items, .. } => items.iter().for_each(|item| walk(item, visit)),
        _ => {}
    }
}

#[test]
fn the_house_exports_without_skipping_anything() {
    let (bytes, notes) = export_ifc2x3(&house()).expect("the house exports");
    assert!(notes.is_empty(), "nothing is skipped: {notes:?}");
    assert!(bytes.len() > 10_000);
}

#[test]
fn the_exported_bytes_are_an_ifc2x3_file_that_the_stdio_reader_decodes_back_to_the_same_graph() {
    let model = house();
    let (bytes, _) = export_ifc2x3(&model).expect("the house exports");
    let decoded = codec::decode_document(&bytes).expect("the stdio reader decodes it");
    assert_eq!(decoded.instances, document(&model).instances);
    assert!(decoded.header.file_schema.iter().any(|value| value.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some("IFC2X3")))));
}

#[test]
fn the_export_is_deterministic_byte_for_byte() {
    let model = house();
    assert_eq!(export_ifc2x3(&model).expect("first").0, export_ifc2x3(&model).expect("second").0);
}

#[test]
fn every_reference_resolves_and_every_global_id_is_unique() {
    let document = document(&house());
    let ids: BTreeSet<u64> = document.instances.iter().map(|instance| instance.id).collect();
    assert_eq!(ids.len(), document.instances.len());
    for instance in &document.instances {
        for (_, args) in &instance.entities {
            for arg in args {
                walk(arg, &mut |target| assert!(ids.contains(&target), "#{} refers to the missing #{target}", instance.id));
            }
        }
    }
    let mut global_ids = BTreeSet::new();
    for instance in &document.instances {
        let Some((name, args)) = instance.primary() else { continue };
        let rooted = !name.starts_with("IFCPROPERTYSINGLE") && args.len() > 3 && args[1].as_ref_id().is_some() && args[0].as_str().is_some_and(|guid| guid.len() == 22);
        if rooted {
            let guid = args[0].as_str().expect("a guid").to_string();
            assert!(global_ids.insert(guid.clone()), "{name} repeats the GlobalId {guid}");
        }
    }
    assert!(global_ids.len() > 100);
}

#[test]
fn every_placed_product_is_contained_in_exactly_one_storey_or_aggregated_below_a_whole() {
    let document = document(&house());
    let mut homes: std::collections::BTreeMap<u64, usize> = std::collections::BTreeMap::new();
    for (_, args) in rows(&document, "IFCRELCONTAINEDINSPATIALSTRUCTURE") {
        for item in args[4].as_list().expect("elements") {
            *homes.entry(item.as_ref_id().expect("a ref")).or_default() += 1;
        }
    }
    assert!(homes.values().all(|count| *count == 1));
    for entity in ["IFCWALL", "IFCWALLSTANDARDCASE", "IFCCOLUMN", "IFCBEAM", "IFCSLAB", "IFCROOF", "IFCSTAIR", "IFCRAILING", "IFCCURTAINWALL", "IFCWINDOW", "IFCDOOR"] {
        for (instance, args) in rows(&document, entity) {
            let part = string(args, 7).is_some_and(|tag| tag.contains(':'));
            assert!(part || homes.contains_key(&instance.id), "{entity} #{} is contained", instance.id);
        }
    }
}

#[test]
fn global_ids_do_not_change_when_unrelated_elements_are_added() {
    let model = house();
    let mut larger = model.clone();
    let mut extra = larger.walls["w-free"].clone();
    extra.name = "Extra".into();
    larger.walls.insert("w-extra".into(), extra);
    let guids = |model: &crate::ModelSnapshot| -> std::collections::BTreeMap<String, String> {
        let document = document(model);
        let mut map = std::collections::BTreeMap::new();
        for entity in ["IFCWALL", "IFCWALLSTANDARDCASE", "IFCCOLUMN", "IFCSLAB", "IFCOPENINGELEMENT", "IFCBUILDINGSTOREY"] {
            for (_, args) in rows(&document, entity) {
                if let (Some(guid), Some(tag)) = (string(args, 0), string(args, if entity == "IFCBUILDINGSTOREY" { 4 } else { 7 })) {
                    map.insert(format!("{entity}:{tag}"), guid);
                }
            }
        }
        map
    };
    let (before, after) = (guids(&model), guids(&larger));
    for (key, guid) in &before {
        if key.starts_with("IFCWALL") && key.ends_with("w-free") {
            continue;
        }
        assert_eq!(after.get(key), Some(guid), "{key} keeps its GlobalId");
    }
    assert!(after.keys().any(|key| key.ends_with("w-extra")));
}

#[test]
fn a_model_without_any_element_exports_the_project_skeleton_only() {
    let mut model = house();
    model.walls.clear();
    model.curtain_walls.clear();
    model.columns.clear();
    model.beams.clear();
    model.slabs.clear();
    model.roofs.clear();
    model.openings.clear();
    model.stairs.clear();
    model.railings.clear();
    model.spaces.clear();
    model.properties.clear();
    model.classifications.clear();
    let document = document(&model);
    assert_eq!(tags(&document, "IFCWALL"), Vec::<String>::new());
    assert_eq!(document.by_type("IFCBUILDINGSTOREY").count(), 4);
    assert_eq!(document.by_type("IFCRELCONTAINEDINSPATIALSTRUCTURE").count(), 1, "only the grid remains");
}

#[test]
fn a_wall_with_a_missing_type_is_skipped_with_a_note_instead_of_failing() {
    let mut model = house();
    model.walls.get_mut("w-free").expect("the free wall").wall_type = "missing".into();
    let (document, notes) = model_to_part21(&model).expect("the model exports");
    assert!(notes.iter().any(|note| note.starts_with("wall w-free")), "{notes:?}");
    assert!(!tags(&document, "IFCWALLSTANDARDCASE").contains(&"w-free".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn the_serializer_returns_the_file_as_a_binary_payload_with_a_diagnostic_per_skipped_item() {
    let clean = ModelIntoIfc2x3::serialize(&house(), &ArchiveChildren::empty()).await.expect("the house serializes");
    assert!(matches!(clean.value, IoPayload::Binary(ref bytes) if bytes.starts_with(b"ISO-10303-21")));
    assert!(clean.diagnostics.is_empty());
    let mut broken = house();
    broken.walls.get_mut("w-free").expect("the free wall").wall_type = "missing".into();
    let outcome = ModelIntoIfc2x3::serialize(&broken, &ArchiveChildren::empty()).await.expect("it still serializes");
    assert!(outcome.diagnostics.iter().any(|diagnostic| diagnostic.message.contains("w-free")));
}

#[test]
fn the_document_of_the_export_is_the_document_of_the_inference_it_is_built_from() {
    let model = house();
    let inferred = ModelInference::infer(&model).expect("the house infers");
    let (from_inference, notes) = inferred_to_part21(&model, &inferred);
    let (from_model, again) = model_to_part21(&model).expect("the house exports");
    assert_eq!((from_inference, notes), (from_model, again), "the export reads only the inference it is given");
}

#[test]
fn every_written_base_quantity_is_a_value_of_the_inferred_take_off() {
    use crate::standards::v1::subsets::any::schema::inferences::quantities::{ElementQuantity, QuantityKind};
    let model = house();
    let inferred = ModelInference::infer(&model).expect("the house infers");
    let (document, _) = inferred_to_part21(&model, &inferred);
    let sets: [(QuantityKind, &str, fn(&ElementQuantity) -> Vec<(&'static str, f64)>); 10] = [
        (QuantityKind::Wall, "Qto_WallBaseQuantities", |q| vec![("Length", q.length), ("Width", q.width), ("Height", q.height), ("GrossFootprintArea", q.gross_area), ("GrossSideArea", q.gross_side_area), ("NetSideArea", q.net_side_area), ("GrossVolume", q.gross_volume), ("NetVolume", q.net_volume)]),
        (QuantityKind::CurtainWall, "Qto_CurtainWallQuantities", |q| vec![("Length", q.length), ("Height", q.height), ("GrossSideArea", q.gross_side_area)]),
        (QuantityKind::Slab, "Qto_SlabBaseQuantities", |q| vec![("Width", q.width), ("Perimeter", q.perimeter), ("GrossArea", q.gross_area), ("NetArea", q.net_area), ("GrossVolume", q.gross_volume), ("NetVolume", q.net_volume)]),
        (QuantityKind::Roof, "Qto_RoofBaseQuantities", |q| vec![("ProjectedArea", q.gross_area)]),
        (QuantityKind::Column, "Qto_ColumnBaseQuantities", |q| vec![("Length", q.length), ("CrossSectionArea", q.gross_area), ("GrossVolume", q.gross_volume)]),
        (QuantityKind::Beam, "Qto_BeamBaseQuantities", |q| vec![("Length", q.length), ("CrossSectionArea", q.gross_area), ("GrossVolume", q.gross_volume)]),
        (QuantityKind::Window, "Qto_WindowBaseQuantities", |q| vec![("Height", q.height), ("Width", q.width), ("Area", q.gross_area)]),
        (QuantityKind::Door, "Qto_DoorBaseQuantities", |q| vec![("Height", q.height), ("Width", q.width), ("Area", q.gross_area)]),
        (QuantityKind::Railing, "Qto_RailingBaseQuantities", |q| vec![("Length", q.length), ("Height", q.height)]),
        (QuantityKind::Space, "Qto_SpaceBaseQuantities", |q| vec![("Height", q.height), ("GrossFloorArea", q.gross_area), ("NetFloorArea", q.net_area), ("GrossVolume", q.gross_volume), ("GrossPerimeter", q.perimeter)]),
    ];
    let mut checked = BTreeSet::new();
    for (id, row) in &inferred.quantities.elements {
        let Some((_, set, pick)) = sets.iter().find(|(kind, _, _)| *kind == row.kind) else { continue };
        for (name, expected) in pick(row) {
            let written = quantity(&document, id, set, name).unwrap_or_else(|| panic!("{set}.{name} of {id}"));
            assert!((written - expected).abs() < 1e-9, "{set}.{name} of {id}: {written} against the take-off {expected}");
        }
        checked.insert(row.kind);
    }
    assert!(checked.len() >= 9, "the house covers the families: {checked:?}");
}

#[test]
fn a_wall_net_side_area_discounts_only_the_openings_the_take_off_clips_to_the_wall() {
    let mut model = house();
    let (host, id) = model.openings.iter().find(|(_, opening)| model.walls.contains_key(&opening.host)).map(|(id, opening)| (opening.host.clone(), id.clone())).expect("a hosted opening");
    let before = ModelInference::infer(&model).expect("infers");
    model.openings.get_mut(&id).expect("the opening").offset = 1.0e3;
    let after = ModelInference::infer(&model).expect("infers");
    let (document, _) = inferred_to_part21(&model, &after);
    let written = quantity(&document, &host, "Qto_WallBaseQuantities", "NetSideArea").expect("the wall net side area");
    assert!((written - after.quantities.elements[&host].net_side_area).abs() < 1e-9);
    assert!(written > before.quantities.elements[&host].net_side_area, "an opening that left the wall no longer discounts it: {written}");
}

#[test]
fn the_layers_of_a_roof_carry_the_volumes_of_the_take_off_and_of_their_solid() {
    let model = house();
    let inferred = ModelInference::infer(&model).expect("the house infers");
    let (document, _) = inferred_to_part21(&model, &inferred);
    for (id, roof) in &model.roofs {
        let solid = &inferred.element_solids[id];
        let storey = &inferred.storey_levels[&roof.storey];
        for (index, group) in solid.groups.iter().enumerate() {
            let written = quantity(&document, &format!("{id}:layer{index}"), "Qto_SlabBaseQuantities", "GrossVolume").expect("the layer volume");
            let brep = crate::standards::v1::subsets::any::io::export::ifc::brep::mesh_where(solid, storey.elevation, |candidate, _| candidate == group).volume();
            assert!((written - brep).abs() < 1e-6, "{id} layer {index}: take-off {written} against the written brep {brep}");
        }
    }
    assert!(!model.roofs.is_empty());
}

#[test]
fn every_phase_that_is_not_new_work_is_written_once_per_product_and_an_opening_carries_the_phase_of_its_host() {
    use crate::Phase;
    let mut model = house();
    macro_rules! cycle {
        ($($collection:ident),+) => { $( for (index, row) in model.$collection.values_mut().enumerate() { row.phase = Phase::ALL[(index + 1) % 4]; } )+ };
    }
    cycle!(walls, curtain_walls, columns, beams, slabs, roofs, stairs, railings, spaces);
    let hosted = |host: &str| model.walls.get(host).map(|wall| wall.phase).or_else(|| model.curtain_walls.get(host).map(|curtain| curtain.phase)).expect("a host");
    let fillers = model.openings.values().filter(|opening| !matches!(opening.kind, crate::OpeningKind::Void { .. })).filter(|opening| hosted(&opening.host) != Phase::New).count();
    let own = model.walls.values().map(|row| row.phase).chain(model.curtain_walls.values().map(|row| row.phase)).chain(model.columns.values().map(|row| row.phase)).chain(model.beams.values().map(|row| row.phase)).chain(model.slabs.values().map(|row| row.phase)).chain(model.roofs.values().map(|row| row.phase)).chain(model.stairs.values().map(|row| row.phase)).chain(model.railings.values().map(|row| row.phase)).chain(model.spaces.values().map(|row| row.phase)).filter(|phase| *phase != Phase::New).count();
    let document = document(&model);
    let written: Vec<String> = rows(&document, "IFCPROPERTYSINGLEVALUE").into_iter().filter(|(_, args)| string(args, 0).as_deref() == Some("Phase")).filter_map(|(_, args)| args.get(2).and_then(|value| value.as_typed()).and_then(|(_, items)| items.first()).and_then(|value| value.as_str()).map(str::to_string)).collect();
    assert_eq!(written.len(), own + fillers, "{written:?}");
    assert!(written.iter().all(|phase| ["Existing", "Demolished", "Temporary"].contains(&phase.as_str())), "new work writes no row: {written:?}");
    for phase in ["Existing", "Demolished", "Temporary"] {
        assert!(written.iter().any(|row| row == phase), "{phase} is written");
    }
}

#[test]
fn a_model_of_new_work_writes_no_phase_row() {
    let document = document(&house());
    assert!(rows(&document, "IFCPROPERTYSINGLEVALUE").iter().all(|(_, args)| string(args, 0).as_deref() != Some("Phase")));
}
