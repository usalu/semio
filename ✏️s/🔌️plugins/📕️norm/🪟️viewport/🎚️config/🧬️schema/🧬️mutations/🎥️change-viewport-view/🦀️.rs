//! 🎥️ Replaces the viewport view (camera preset, hidden layers, colour map, unit system) without touching other windows.

use crate::diff::NormViewportWindowConfigDiff;
use crate::schema::NormViewportWindowConfig;
use crate::mutations::NormViewportWindowConfigMutation;

#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "change-viewport-view")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeViewportView {
    pub camera_preset: Option<String>,
    pub hidden_layers: Vec<String>,
    pub colour_map: Option<String>,
    pub unit_system: Option<String>,
}

impl protocol::MutationKind<NormViewportWindowConfig, NormViewportWindowConfigMutation> for ChangeViewportView {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "viewport-view", kind: "change-viewport-view", record: "ChangedViewportView" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change viewport view", "Ansicht des Modellfensters ändern")
    }
}

impl protocol::ColdMutationKind<NormViewportWindowConfig, NormViewportWindowConfigMutation> for ChangeViewportView {
    fn diff(&self, base: &NormViewportWindowConfig) -> protocol::MutationOutcome<NormViewportWindowConfigDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &NormViewportWindowConfig) -> Result<Vec<NormViewportWindowConfigMutation>, semio_framework_value::ValueError> {
        inverse::inverse(self, base)
    }
}

#[cfg(test)]
#[path = "🧪️tests/✅apply/🦀️.rs"]
mod named_test;
