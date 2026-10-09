use crate::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, print_dsl, BIM_EXAMPLE_TEXT};
use crate::{ModelInference, ModelSnapshot};
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn the_demo_parses_and_is_the_documented_scene() {
    let model = parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses");
    assert_eq!((model.sites.len(), model.buildings.len(), model.storeys.len(), model.walls.len()), (1, 1, 2, 4));
    assert_eq!((model.materials.len(), model.wall_types.len()), (2, 1));
    assert_eq!(model.wall_types.values().next().expect("a wall type").layers.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn the_demo_text_is_the_codec_fixed_point() {
    let model = parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses");
    assert_eq!(print_dsl(&model), BIM_EXAMPLE_TEXT, "the committed demo is this codec's own output");
}

#[semio_framework_async_macros::async_test]
async fn the_demo_infers_a_level_per_storey_and_a_layout_per_wall() {
    let model: ModelSnapshot = parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses");
    let inferred = ModelInference::infer(&model).expect("infers");
    assert_eq!((inferred.storey_levels.len(), inferred.wall_layout.len()), (2, 4));
}

/// 🖨️ `BIM_BLESS=1` rewrites the committed demo text from `🖼️assets/🎬️demo/📸️snapshot.json` through the codec's own printer.
#[semio_framework_async_macros::async_test]
async fn bless_the_demo_text() {
    if std::env::var_os("BIM_BLESS").is_none() {
        return;
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo");
    let json = std::fs::read_to_string(root.join("📸️snapshot.json")).expect("the demo snapshot source exists");
    let model: ModelSnapshot = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the demo snapshot decodes");
    std::fs::write(root.join("🗣️.dsl.semio"), print_dsl(&model)).expect("the demo text is written");
}

mod shared {
    use crate::examples::checks::Asset;
    use crate::examples::demo::{ASSET_DIR, PRIMARY_TEXT, SNAPSHOT_JSON};

    const ASSET: Asset = Asset { dir: ASSET_DIR, text: PRIMARY_TEXT, json: SNAPSHOT_JSON };

    #[semio_framework_async_macros::async_test]
    async fn the_demo_round_trips_validates_and_is_closed() {
        ASSET.assert_text_is_the_codec_fixed_point();
        ASSET.assert_pack_round_trip();
        ASSET.assert_schema_valid();
        assert_eq!(crate::examples::checks::dangling(&ASSET.model()), Vec::<String>::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn the_demo_infers_cleanly_and_is_reachable_through_the_mutation_api() {
        let model = ASSET.model();
        crate::examples::checks::infer(&model);
        crate::examples::checks::assert_replay("demo", &model);
    }

    #[semio_framework_async_macros::async_test]
    async fn the_example_views_are_the_ones_the_command_makes() {
        crate::examples::checks::assert_views_are_the_commands("demo", &ASSET.model());
    }
}
