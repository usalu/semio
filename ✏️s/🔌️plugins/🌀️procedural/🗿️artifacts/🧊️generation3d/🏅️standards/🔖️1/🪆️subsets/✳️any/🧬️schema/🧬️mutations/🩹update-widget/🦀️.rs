//! 🔁 `update-widget` payload — replaces the whole body of an EXISTING id-keyed [`Widget`]
//! atomically (`Widget` is a discriminated union with no independently-settable scalar fields
//! exposed here, so whole-body replace is the cohesive facet per `📓️taxonomy.md`'s `update` row).

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;
//#region 🔖️UpdateWidget
/// 🔁 The widget's own id (via [`crate::widget_id`]) addresses the target
/// — no separate `id` field, since `Widget` already carries its identity.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateWidget {
    pub widget: Widget,
}

/// 🔗️ Variable renames preserve attached channel identities in sparse diffs and retained replay.
pub(crate) fn variable_synapses(previous: &Widget, next: &Widget, synapses: &[semio_framework_artifact_flow_flow::SynapseSpec]) -> Vec<(usize, semio_framework_artifact_flow_flow::SynapseSpec)> {
    let (Widget::Variable { id, name: old, schema: old_schema }, Widget::Variable { name, schema, .. }) = (previous, next) else { return Vec::new(); };
    let old = semio_framework_artifact_flow_flow::variable_io_ports(old, old_schema).0[0].id.clone();
    let next = semio_framework_artifact_flow_flow::variable_io_ports(name, schema).0[0].id.clone();
    if old == next { return Vec::new(); }
    synapses.iter().enumerate().filter_map(|(index, synapse)| {
        let mut changed = synapse.clone();
        if changed.from == *id && changed.from_port == old { changed.from_port = next.clone(); }
        if changed.to == *id && changed.to_port == old { changed.to_port = next.clone(); }
        (changed != *synapse).then_some((index, changed))
    }).collect()
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for UpdateWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "widget", kind: "update-widget", record: "UpdatedWidget" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::update_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::update_widget::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update widget \"{}\"", crate::widget_id(&self.widget)), &format!("Widget \"{}\" aktualisieren", crate::widget_id(&self.widget)))
    }

    fn target(&self) -> Vec<String> {
        vec![crate::widget_id(&self.widget).to_string()]
    }
}
//#endregion 🔖️UpdateWidget
