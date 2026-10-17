#[allow(unused_imports)] use protocol::{MutationKind, SemanticDescriptor};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_mutations_content::update_spread::*;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn update_spread_renames_and_inverse_restores_it() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let mutation = LayoutMutation::UpdateSpread(UpdateSpread { id: "spread-1".into(), name: "Opening".into() });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("spread applies");
    assert_eq!(next.spreads[0].name, "Opening");
    let restored = protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert_eq!(restored.spreads[0].name, "Spread 1");
}
