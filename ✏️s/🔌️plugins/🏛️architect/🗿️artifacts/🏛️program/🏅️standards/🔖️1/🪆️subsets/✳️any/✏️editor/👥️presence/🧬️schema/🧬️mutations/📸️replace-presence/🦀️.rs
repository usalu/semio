//! 🧬️ Replace Presence in the architect.presence channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: ArchitectPresence,
}

impl protocol::MutationKind<ArchitectPresence, ArchitectPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, base: &ArchitectPresence) -> protocol::MutationOutcome<ArchitectPresenceDiff> {
        if &self.presence == base {
            return protocol::MutationOutcome::new(ArchitectPresenceDiff::default()).warning("mutation.no-op", "Requested presence already matches.");
        }
        let requested = &self.presence;
        protocol::MutationOutcome::new(ArchitectPresenceDiff {
            active_register: (base.active_register != requested.active_register).then(|| requested.active_register.clone()),
            adjacency_kind_filter: (base.adjacency_kind_filter != requested.adjacency_kind_filter).then(|| AdjacencyKindFilterSet { value: requested.adjacency_kind_filter.clone() }),
            graph_camera_x: (base.graph_camera_x != requested.graph_camera_x).then_some(requested.graph_camera_x),
            graph_camera_y: (base.graph_camera_y != requested.graph_camera_y).then_some(requested.graph_camera_y),
            graph_camera_zoom: (base.graph_camera_zoom != requested.graph_camera_zoom).then_some(requested.graph_camera_zoom),
        })
    }
    fn inverse(&self, base: &ArchitectPresence) -> Result<Vec<ArchitectPresenceMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ArchitectPresenceMutation::ReplacePresence(Self { presence: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Presence", "Präsenz ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}
