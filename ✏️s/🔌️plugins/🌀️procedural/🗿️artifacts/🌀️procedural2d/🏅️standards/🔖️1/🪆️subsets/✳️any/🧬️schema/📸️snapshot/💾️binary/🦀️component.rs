//! 📦️ Procedural2d artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::procedural2d::Procedural2dSnapshot;
use store::PackError;

/// 📦️ Encodes a `Procedural2dSnapshot` to its binary pack form.
pub async fn encode(document: &Procedural2dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Procedural2dSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<Procedural2dSnapshot, PackError> {
    <Procedural2dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural2d::dsl;
    use flow::Widget;
    use semio_framework_os_kernel::os_store::test_support;

    #[semio_framework_async_macros::async_test]
    async fn dsl_pack_equivalence_empty_projection() {
        test_support::assert_dsl_pack_equivalence(&Procedural2dSnapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_pack_equivalence_example_fixture() {
        let projection = dsl::parse_dsl(dsl::PROCEDURAL2D_EXAMPLE_TEXT).expect("parse 🌀️default.procedural2d fixture");
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_pack_equivalence_with_generation_state() {
        let mut projection = Procedural2dSnapshot::default();
        let mut values = serde_json::Map::new();
        // 🌱️ Fractional (not whole-number) so `dsl::from_dsl_value`'s int-normalization of whole
        // `DslValue::Number`s (an engine-owned behavior, see the sibling dsl test) doesn't make this
        // round trip spuriously unequal.
        values.insert("count".into(), serde_json::json!(3.5));
        projection.generation.generations.push(flow::playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.selected_generation_id = Some("generation-1".into());
        projection.generation.preview_text = Some("42".into());
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_pack_equivalence_covers_every_widget_kind() {
        let mut projection = Procedural2dSnapshot::default();
        projection.fixture.widgets = vec![
            Widget::InputSlider { id: "slider".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
            Widget::InputImage { id: "image".into(), src: "data:image/png;base64,abc".into() },
            Widget::Variable { id: "variable".into(), name: "value".into(), schema: "dictionary".into() },
            Widget::OutputAction { id: "action".into(), action: "export".into() },
            Widget::OutputExport { id: "export".into(), format: "svg".into() },
            Widget::Cluster { id: "cluster".into(), name: "Group".into(), tree: Default::default(), flow: Default::default() },
        ];
        projection.fixture.synapses = vec![];
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips() {
        let projection = dsl::parse_dsl(dsl::PROCEDURAL2D_EXAMPLE_TEXT).expect("parse fixture");
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }
}
//#endregion 🧪️Tests
