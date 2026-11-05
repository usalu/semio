//! 🧪️ Public viewport-config protocol laws: descriptor, apply and inverse, and the text and binary wire round trips.

use semio_os_kernel::{apply_diff_cold, ColdMutation, Mutation, OpBinary, OpText};
use semio_s_plugin_norm_viewport_config::mutations::NormViewportWindowConfigMutation;
use semio_s_plugin_norm_viewport_config::schema::NormViewportWindowConfig;
use semio_s_plugin_norm_viewport_config::ChangeViewportView;

fn view(preset: Option<&str>, hidden: &[&str], map: Option<&str>, units: Option<&str>) -> NormViewportWindowConfigMutation {
    ChangeViewportView { camera_preset: preset.map(str::to_owned), hidden_layers: hidden.iter().map(|layer| (*layer).to_owned()).collect(), colour_map: map.map(str::to_owned), unit_system: units.map(str::to_owned) }.into()
}

#[test]
fn the_view_mutation_is_described_applied_inverted_and_round_trips_on_both_wires() {
    let descriptors = <NormViewportWindowConfigMutation as Mutation<NormViewportWindowConfig>>::DESCRIPTORS;
    assert_eq!(descriptors.len(), 1, "one concrete semantic mutation");
    assert_eq!(descriptors[0].semantic_kind, "change-viewport-view");
    assert_eq!(descriptors[0].text_opcode, Some("change-viewport-view"));
    assert_eq!(descriptors[0].binary_tag, Some(0));
    assert!(descriptors[0].validate().is_ok());
    let base = NormViewportWindowConfig { camera_preset: Some("top".into()), hidden_layers: vec!["supports".into()], colour_map: None, unit_system: Some("si".into()) };
    for mutation in [view(Some("iso"), &["loads", "annotations"], Some("viridis"), Some("engineering")), view(None, &[], None, None), view(Some("top"), &["supports"], None, Some("si"))] {
        let outcome = mutation.diff(&base);
        let next = apply_diff_cold(outcome.diff(), &base).unwrap();
        let backwards = mutation.inverse(&base).expect("inverse");
        assert_eq!(backwards.len(), 1);
        assert_eq!(apply_diff_cold(backwards[0].diff(&next).diff(), &next).unwrap(), base);
        assert_eq!(NormViewportWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(NormViewportWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    assert!(!view(Some("top"), &["supports"], None, Some("si")).diff(&base).messages().is_empty(), "an unchanged view is a no-op warning");
    for text in ["snapshot", "change-viewport", "unknown-operation"] {
        assert!(NormViewportWindowConfigMutation::parse_op(text).is_err());
    }
    for bytes in [&[][..], &[1][..], &[1, 1][..]] {
        assert!(NormViewportWindowConfigMutation::decode_op(bytes).is_err());
    }
}
