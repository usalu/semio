//! 🧬️ `change-element-inclination-deg`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeElementInclinationDeg {
    pub element_id: String,
    pub new_inclination_deg: f64,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeElementInclinationDeg where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_element_inclination_deg::ChangeElementInclinationDeg> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "element-inclination-deg",
        kind: "change-element-inclination-deg",
        record: "ChangedElementInclinationDeg",
    };
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Din4108Snapshot>>::Diff> {
        crate::change_element_inclination_deg::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_element_inclination_deg::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change element inclination", "Bauteilneigung ändern")
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
