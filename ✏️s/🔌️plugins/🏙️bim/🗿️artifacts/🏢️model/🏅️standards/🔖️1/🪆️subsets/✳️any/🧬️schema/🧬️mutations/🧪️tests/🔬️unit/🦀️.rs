use super::*;
use crate::standards::v1::subsets::any::io::binary::mutations::{decode_op, encode_op};
use protocol::{Mutation, OpText};

fn descriptors() -> Vec<(&'static str, u32)> {
    <ModelMutation as Mutation<ModelSnapshot>>::DESCRIPTORS.iter().map(|descriptor| (descriptor.semantic_kind, descriptor.binary_tag.expect("every leaf declares a wire tag"))).collect()
}

#[semio_framework_async_macros::async_test]
async fn kinds_match_the_enum_and_the_catalog() {
    let mut declared: Vec<&str> = descriptors().into_iter().map(|(kind, _)| kind).collect();
    let mut listed = KINDS.to_vec();
    declared.sort_unstable();
    listed.sort_unstable();
    assert_eq!(declared, listed, "KINDS lists every variant exactly once");
}

#[semio_framework_async_macros::async_test]
async fn binary_tags_are_unique_and_registered_in_the_protocol() {
    let mut tags: Vec<u32> = descriptors().into_iter().map(|(_, tag)| tag).collect();
    tags.sort_unstable();
    tags.dedup();
    assert_eq!(tags.len(), descriptors().len(), "every leaf owns its own wire tag");
    let protocol = include_str!("../../../../🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio");
    for (kind, tag) in descriptors() {
        assert!(protocol.contains(&format!("record {kind} tag={tag}\n")), "{kind} has its wire tag in the protocol");
    }
}

#[semio_framework_async_macros::async_test]
async fn every_committed_mutation_round_trips_text_and_binary() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    let (operations, files) = protocol::mutation_fixture_ops::<ModelMutation>(&root);
    assert!(files >= KINDS.len() * 2, "every kind commits at least an applied and a rejected case, found {files} fixtures");
    assert_eq!(operations.len(), files, "every committed mutation decodes as the aggregate");
    for operation in operations {
        assert_eq!(ModelMutation::parse_op(&operation.print_op()).expect("text op parses"), operation);
        assert_eq!(decode_op(&encode_op(&operation).expect("binary op encodes")).expect("binary op decodes"), operation);
    }
}

#[semio_framework_async_macros::async_test]
async fn delete_storey_declares_a_bounded_footprint_and_the_others_one_row() {
    use super::super::delete_storey::DeleteStorey;
    use super::super::set_storey_height::SetStoreyHeight;
    assert_eq!(Mutation::<ModelSnapshot>::inverse_rows(&ModelMutation::DeleteStorey(DeleteStorey { id: "st".into() })), 8191);
    assert_eq!(Mutation::<ModelSnapshot>::inverse_rows(&ModelMutation::SetStoreyHeight(SetStoreyHeight { id: "st".into(), height: 3.0 })), 1);
}

fn house() -> ModelSnapshot {
    semio_framework_pack_json::from_json_str(include_str!("../../../../🖼️assets/🏡️house/📸️snapshot.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the house decodes")
}

#[semio_framework_async_macros::async_test]
async fn every_id_of_every_collection_is_taken_by_its_kind() {
    let base = house();
    macro_rules! taken {
        ($($collection:ident => $noun:literal),+ $(,)?) => {$(
            assert!(!base.$collection.is_empty(), "the house has {}", stringify!($collection));
            for id in base.$collection.keys() {
                assert_eq!(elements::taken(&base, id), Some($noun), "{id}");
            }
        )+};
    }
    taken!(
        sites => "Site", buildings => "Building", storeys => "Storey", grids => "Grid line", walls => "Wall", columns => "Column", beams => "Beam", slabs => "Slab", roofs => "Roof", openings => "Opening", stairs => "Stair",
        railings => "Railing", spaces => "Space", materials => "Material", wall_types => "Wall type", slab_types => "Slab type", roof_types => "Roof type", column_types => "Column type", beam_types => "Beam type",
        window_types => "Window type", door_types => "Door type",
    );
    assert_eq!(elements::taken(&base, "an-id-nobody-uses"), None);
}

#[semio_framework_async_macros::async_test]
async fn the_pure_storey_elevation_equals_the_inference_engine() {
    let base = house();
    let levels = crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels(&base);
    assert!(base.storeys.values().any(|storey| storey.level < 0), "the house stacks downward too");
    for id in base.storeys.keys() {
        assert_eq!(placement::storey_elevation(&base, id), Some(levels[id].elevation), "{id}");
    }
    assert_eq!(placement::storey_elevation(&base, "st-nowhere"), None);
}

#[semio_framework_async_macros::async_test]
async fn the_rise_to_another_storey_is_the_elevation_gap() {
    use crate::TopConstraint;
    let base = house();
    let gap = placement::storey_elevation(&base, "st-upper").unwrap() - placement::storey_elevation(&base, "st-ground").unwrap();
    let rise = placement::rise(&base, "st-ground", 0.1, &TopConstraint::Storey { storey: "st-upper".into(), offset: -0.2 }).expect("both storeys exist");
    assert!((rise - (gap - 0.2 - 0.1)).abs() < 1e-12);
    assert_eq!(placement::rise(&base, "st-ground", 0.0, &TopConstraint::Storey { storey: "st-nowhere".into(), offset: 0.0 }), None);
    assert_eq!(placement::rise(&base, "st-ground", 0.0, &TopConstraint::StoreyTop { offset: 0.0 }), Some(base.storeys["st-ground"].height));
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_of_every_committed_applied_modify_and_delete_case_fits_the_rows_its_leaf_declares() {
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    let kinds = ["copy-elements", "mirror-elements", "array-elements", "align-elements", "offset-wall", "trim-extend-wall", "split-slab", "split-beam", "set-wall-end-join", "delete-elements", "delete-site", "delete-building", "delete-storey"];
    let mut checked = 0;
    for leaf in std::fs::read_dir(&root).expect("the fixture root").flatten().filter(|leaf| kinds.contains(&leaf.file_name().to_string_lossy().trim_start_matches(|character: char| !character.is_ascii())))
    {
        for case in std::fs::read_dir(leaf.path()).expect("the leaf cases").flatten() {
            let read = |relative: &str| std::fs::read_to_string(case.path().join(relative)).expect("a fixture document");
            if !read("🎯️outcome/🔣️.json").contains("\"applied\"") {
                continue;
            }
            let mutation: ModelMutation = from_json_str(&read("🦠️mutation/🔣️.json"), JsonMemberPolicy::Reject).expect("the mutation decodes");
            let before: ModelSnapshot = from_json_str(&read("📸️snapshot/⬅️before/🔣️.json"), JsonMemberPolicy::Reject).expect("the before snapshot decodes");
            let rows = mutation.inverse(&before).expect("an inverse").len();
            let declared = Mutation::<ModelSnapshot>::inverse_rows(&mutation);
            assert!(rows >= 1 && rows <= declared, "{}: the inverse has {rows} rows where the leaf declares {declared}", case.path().display());
            checked += 1;
        }
    }
    assert!(checked >= 30, "the applied cases of the modify and delete leaves were found: {checked}");
}
