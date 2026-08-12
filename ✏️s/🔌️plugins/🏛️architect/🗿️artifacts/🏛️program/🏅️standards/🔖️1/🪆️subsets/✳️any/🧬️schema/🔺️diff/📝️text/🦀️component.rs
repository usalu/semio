//! 📦️ Architect program artifact — sparse field-delta runtime (constitutional: diff).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::program::schema::diff::*;


use crate::artifacts::program::kernel::*;
use crate::artifacts::program::registers::*;
use crate::artifacts::program::schema::ProgramArtifact;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{CollectionMutation, Identified, MutationDiff, Patchable};

//#region 🔖️Apply
impl ProgramDiff {
    /// 🧬️ Apply every field entry onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &ProgramArtifact) -> ProgramArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(v) = &self.schema { next.schema = v.clone(); }
        if let Some(v) = &self.meta { next.meta = v.clone(); }
        if let Some(v) = &self.project { next.project = v.clone(); }
        if let Some(v) = &self.governance { next.governance = v.clone(); }

        if let Some(delta) = &self.stakeholders {
            apply_collection_delta(
                &mut next.stakeholders,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.users {
            apply_collection_delta(
                &mut next.users,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.activities {
            apply_collection_delta(
                &mut next.activities,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.functions {
            apply_collection_delta(
                &mut next.functions,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.elements {
            apply_collection_delta(
                &mut next.elements,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.quantities {
            apply_collection_delta(
                &mut next.quantities,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.relationships {
            apply_collection_delta(
                &mut next.relationships,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.adjacencies {
            apply_collection_delta(
                &mut next.adjacencies,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.processes {
            apply_collection_delta(
                &mut next.processes,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.flows {
            apply_collection_delta(
                &mut next.flows,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.access_rules {
            apply_collection_delta(
                &mut next.access_rules,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.operations {
            apply_collection_delta(
                &mut next.operations,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.equipment {
            apply_collection_delta(
                &mut next.equipment,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.resources {
            apply_collection_delta(
                &mut next.resources,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.storage {
            apply_collection_delta(
                &mut next.storage,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.environmental {
            apply_collection_delta(
                &mut next.environmental,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.human_factors {
            apply_collection_delta(
                &mut next.human_factors,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.accessibility {
            apply_collection_delta(
                &mut next.accessibility,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.privacy {
            apply_collection_delta(
                &mut next.privacy,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.safety {
            apply_collection_delta(
                &mut next.safety,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.security {
            apply_collection_delta(
                &mut next.security,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.regulatory {
            apply_collection_delta(
                &mut next.regulatory,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.site_context {
            apply_collection_delta(
                &mut next.site_context,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.organizational {
            apply_collection_delta(
                &mut next.organizational,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.services {
            apply_collection_delta(
                &mut next.services,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.infrastructure {
            apply_collection_delta(
                &mut next.infrastructure,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.information {
            apply_collection_delta(
                &mut next.information,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.communication {
            apply_collection_delta(
                &mut next.communication,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.wayfinding {
            apply_collection_delta(
                &mut next.wayfinding,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.schedules {
            apply_collection_delta(
                &mut next.schedules,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.flexibility {
            apply_collection_delta(
                &mut next.flexibility,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.growth {
            apply_collection_delta(
                &mut next.growth,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.sustainability {
            apply_collection_delta(
                &mut next.sustainability,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.resilience {
            apply_collection_delta(
                &mut next.resilience,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.costs {
            apply_collection_delta(
                &mut next.costs,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.delivery {
            apply_collection_delta(
                &mut next.delivery,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.risks {
            apply_collection_delta(
                &mut next.risks,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.conflicts {
            apply_collection_delta(
                &mut next.conflicts,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.requirements {
            apply_collection_delta(
                &mut next.requirements,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.priorities {
            apply_collection_delta(
                &mut next.priorities,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.scenarios {
            apply_collection_delta(
                &mut next.scenarios,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.options {
            apply_collection_delta(
                &mut next.options,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.decisions {
            apply_collection_delta(
                &mut next.decisions,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.validations {
            apply_collection_delta(
                &mut next.validations,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.performance {
            apply_collection_delta(
                &mut next.performance,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.quality {
            apply_collection_delta(
                &mut next.quality,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.documents {
            apply_collection_delta(
                &mut next.artifacts,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.assumptions {
            apply_collection_delta(
                &mut next.assumptions,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.constraints {
            apply_collection_delta(
                &mut next.constraints,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.compliance_records {
            apply_collection_delta(
                &mut next.compliance_records,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.approvals {
            apply_collection_delta(
                &mut next.approvals,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.meetings {
            apply_collection_delta(
                &mut next.meetings,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.changes {
            apply_collection_delta(
                &mut next.changes,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.collaboration {
            apply_collection_delta(
                &mut next.collaboration,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.analyses {
            apply_collection_delta(
                &mut next.analyses,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.reports {
            apply_collection_delta(
                &mut next.reports,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.search_filters {
            apply_collection_delta(
                &mut next.search_filters,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.status_records {
            apply_collection_delta(
                &mut next.status_records,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.workshops {
            apply_collection_delta(
                &mut next.workshops,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.surveys {
            apply_collection_delta(
                &mut next.surveys,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.issues {
            apply_collection_delta(
                &mut next.issues,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.audit_events {
            apply_collection_delta(
                &mut next.audit_events,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.templates {
            apply_collection_delta(
                &mut next.templates,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.knowledge {
            apply_collection_delta(
                &mut next.knowledge,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.benchmarks {
            apply_collection_delta(
                &mut next.benchmarks,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(delta) = &self.traces {
            apply_collection_delta(
                &mut next.traces,
                &delta.added,
                &delta.removed,
                &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(),
                &delta.reordered,
            );
        }
        if let Some(v) = &self.selected_ids { next.selected_ids = v.values.clone(); }
        if let Some(v) = &self.active_register { next.active_register = v.clone(); }
        if let Some(v) = &self.adjacency_kind_filter { next.adjacency_kind_filter = v.clone(); }
        if let Some(v) = &self.active_report_json { next.active_report_json = v.clone(); }
        if let Some(v) = &self.search_query { next.search_query = v.clone(); }
        if let Some(v) = &self.search_history_json { next.search_history_json = v.clone(); }
        if let Some(v) = &self.last_result_json { next.last_result_json = v.clone(); }
        if let Some(v) = &self.last_analysis_json { next.last_analysis_json = v.clone(); }
        if let Some(v) = &self.graph_camera_x { next.graph_camera_x = *v; }
        if let Some(v) = &self.graph_camera_y { next.graph_camera_y = *v; }
        if let Some(v) = &self.graph_camera_zoom { next.graph_camera_zoom = *v; }
        next
    }
}

impl MutationDiff<ProgramSnapshot> for ProgramDiff {
    fn apply(&self, base: &ProgramSnapshot) -> ProgramSnapshot {
        self.apply_to_artifact(&ProgramArtifact::from_snapshot(base.clone())).to_snapshot()
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! absorb_opt { ($f:ident) => { if other.$f.is_some() { self.$f = other.$f; } }; }
        absorb_opt!(schema);
        absorb_opt!(meta);
        absorb_opt!(project);
        absorb_opt!(governance);

        if let Some(delta) = other.stakeholders {
            match &mut self.stakeholders {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.stakeholders = Some(delta),
            }
        }
        if let Some(delta) = other.users {
            match &mut self.users {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.users = Some(delta),
            }
        }
        if let Some(delta) = other.activities {
            match &mut self.activities {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.activities = Some(delta),
            }
        }
        if let Some(delta) = other.functions {
            match &mut self.functions {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.functions = Some(delta),
            }
        }
        if let Some(delta) = other.elements {
            match &mut self.elements {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.elements = Some(delta),
            }
        }
        if let Some(delta) = other.quantities {
            match &mut self.quantities {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.quantities = Some(delta),
            }
        }
        if let Some(delta) = other.relationships {
            match &mut self.relationships {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.relationships = Some(delta),
            }
        }
        if let Some(delta) = other.adjacencies {
            match &mut self.adjacencies {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.adjacencies = Some(delta),
            }
        }
        if let Some(delta) = other.processes {
            match &mut self.processes {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.processes = Some(delta),
            }
        }
        if let Some(delta) = other.flows {
            match &mut self.flows {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.flows = Some(delta),
            }
        }
        if let Some(delta) = other.access_rules {
            match &mut self.access_rules {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.access_rules = Some(delta),
            }
        }
        if let Some(delta) = other.operations {
            match &mut self.operations {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.operations = Some(delta),
            }
        }
        if let Some(delta) = other.equipment {
            match &mut self.equipment {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.equipment = Some(delta),
            }
        }
        if let Some(delta) = other.resources {
            match &mut self.resources {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.resources = Some(delta),
            }
        }
        if let Some(delta) = other.storage {
            match &mut self.storage {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.storage = Some(delta),
            }
        }
        if let Some(delta) = other.environmental {
            match &mut self.environmental {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.environmental = Some(delta),
            }
        }
        if let Some(delta) = other.human_factors {
            match &mut self.human_factors {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.human_factors = Some(delta),
            }
        }
        if let Some(delta) = other.accessibility {
            match &mut self.accessibility {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.accessibility = Some(delta),
            }
        }
        if let Some(delta) = other.privacy {
            match &mut self.privacy {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.privacy = Some(delta),
            }
        }
        if let Some(delta) = other.safety {
            match &mut self.safety {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.safety = Some(delta),
            }
        }
        if let Some(delta) = other.security {
            match &mut self.security {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.security = Some(delta),
            }
        }
        if let Some(delta) = other.regulatory {
            match &mut self.regulatory {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.regulatory = Some(delta),
            }
        }
        if let Some(delta) = other.site_context {
            match &mut self.site_context {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.site_context = Some(delta),
            }
        }
        if let Some(delta) = other.organizational {
            match &mut self.organizational {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.organizational = Some(delta),
            }
        }
        if let Some(delta) = other.services {
            match &mut self.services {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.services = Some(delta),
            }
        }
        if let Some(delta) = other.infrastructure {
            match &mut self.infrastructure {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.infrastructure = Some(delta),
            }
        }
        if let Some(delta) = other.information {
            match &mut self.information {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.information = Some(delta),
            }
        }
        if let Some(delta) = other.communication {
            match &mut self.communication {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.communication = Some(delta),
            }
        }
        if let Some(delta) = other.wayfinding {
            match &mut self.wayfinding {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.wayfinding = Some(delta),
            }
        }
        if let Some(delta) = other.schedules {
            match &mut self.schedules {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.schedules = Some(delta),
            }
        }
        if let Some(delta) = other.flexibility {
            match &mut self.flexibility {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.flexibility = Some(delta),
            }
        }
        if let Some(delta) = other.growth {
            match &mut self.growth {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.growth = Some(delta),
            }
        }
        if let Some(delta) = other.sustainability {
            match &mut self.sustainability {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.sustainability = Some(delta),
            }
        }
        if let Some(delta) = other.resilience {
            match &mut self.resilience {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.resilience = Some(delta),
            }
        }
        if let Some(delta) = other.costs {
            match &mut self.costs {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.costs = Some(delta),
            }
        }
        if let Some(delta) = other.delivery {
            match &mut self.delivery {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.delivery = Some(delta),
            }
        }
        if let Some(delta) = other.risks {
            match &mut self.risks {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.risks = Some(delta),
            }
        }
        if let Some(delta) = other.conflicts {
            match &mut self.conflicts {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.conflicts = Some(delta),
            }
        }
        if let Some(delta) = other.requirements {
            match &mut self.requirements {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.requirements = Some(delta),
            }
        }
        if let Some(delta) = other.priorities {
            match &mut self.priorities {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.priorities = Some(delta),
            }
        }
        if let Some(delta) = other.scenarios {
            match &mut self.scenarios {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.scenarios = Some(delta),
            }
        }
        if let Some(delta) = other.options {
            match &mut self.options {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.options = Some(delta),
            }
        }
        if let Some(delta) = other.decisions {
            match &mut self.decisions {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.decisions = Some(delta),
            }
        }
        if let Some(delta) = other.validations {
            match &mut self.validations {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.validations = Some(delta),
            }
        }
        if let Some(delta) = other.performance {
            match &mut self.performance {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.performance = Some(delta),
            }
        }
        if let Some(delta) = other.quality {
            match &mut self.quality {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.quality = Some(delta),
            }
        }
        if let Some(delta) = other.documents {
            match &mut self.documents {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.documents = Some(delta),
            }
        }
        if let Some(delta) = other.assumptions {
            match &mut self.assumptions {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.assumptions = Some(delta),
            }
        }
        if let Some(delta) = other.constraints {
            match &mut self.constraints {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.constraints = Some(delta),
            }
        }
        if let Some(delta) = other.compliance_records {
            match &mut self.compliance_records {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.compliance_records = Some(delta),
            }
        }
        if let Some(delta) = other.approvals {
            match &mut self.approvals {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.approvals = Some(delta),
            }
        }
        if let Some(delta) = other.meetings {
            match &mut self.meetings {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.meetings = Some(delta),
            }
        }
        if let Some(delta) = other.changes {
            match &mut self.changes {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.changes = Some(delta),
            }
        }
        if let Some(delta) = other.collaboration {
            match &mut self.collaboration {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.collaboration = Some(delta),
            }
        }
        if let Some(delta) = other.analyses {
            match &mut self.analyses {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.analyses = Some(delta),
            }
        }
        if let Some(delta) = other.reports {
            match &mut self.reports {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.reports = Some(delta),
            }
        }
        if let Some(delta) = other.search_filters {
            match &mut self.search_filters {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.search_filters = Some(delta),
            }
        }
        if let Some(delta) = other.status_records {
            match &mut self.status_records {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.status_records = Some(delta),
            }
        }
        if let Some(delta) = other.workshops {
            match &mut self.workshops {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.workshops = Some(delta),
            }
        }
        if let Some(delta) = other.surveys {
            match &mut self.surveys {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.surveys = Some(delta),
            }
        }
        if let Some(delta) = other.issues {
            match &mut self.issues {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.issues = Some(delta),
            }
        }
        if let Some(delta) = other.audit_events {
            match &mut self.audit_events {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.audit_events = Some(delta),
            }
        }
        if let Some(delta) = other.templates {
            match &mut self.templates {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.templates = Some(delta),
            }
        }
        if let Some(delta) = other.knowledge {
            match &mut self.knowledge {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.knowledge = Some(delta),
            }
        }
        if let Some(delta) = other.benchmarks {
            match &mut self.benchmarks {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.benchmarks = Some(delta),
            }
        }
        if let Some(delta) = other.traces {
            match &mut self.traces {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.traces = Some(delta),
            }
        }
        absorb_opt!(selected_ids);
        absorb_opt!(active_register);
        absorb_opt!(adjacency_kind_filter);
        absorb_opt!(active_report_json);
        absorb_opt!(search_query);
        absorb_opt!(search_history_json);
        absorb_opt!(last_result_json);
        absorb_opt!(last_analysis_json);
        absorb_opt!(graph_camera_x);
        absorb_opt!(graph_camera_y);
        absorb_opt!(graph_camera_zoom);
    }
}

fn apply_collection_delta<T, P>(
    items: &mut Vec<T>,
    added: &[T],
    removed: &[String],
    patched: &[(String, P)],
    reordered: &Option<Vec<String>>,
) where
    T: Identified<EntityId> + Clone + Patchable<P>,
    P: Clone,
{
    for id in removed {
        let eid = EntityId(id.clone());
        items.retain(|item| item.id() != &eid);
    }
    for (id, patch) in patched {
        let eid = EntityId(id.clone());
        if let Some(item) = items.iter_mut().find(|item| item.id() == &eid) {
            item.apply_patch(patch);
        }
    }
    items.extend(added.iter().cloned());
    if let Some(order) = reordered {
        let mut map: std::collections::BTreeMap<String, T> = std::collections::BTreeMap::new();
        for item in items.drain(..) {
            map.insert(item.id().0.clone(), item);
        }
        for id in order {
            if let Some(item) = map.remove(id) {
                items.push(item);
            }
        }
        items.extend(map.into_values());
    }
}
//#endregion 🔖️Apply

//#region 🔖️Constructors

fn collection_delta_from_mutation<T, P>(
    mutation: &CollectionMutation<EntityId, T, P>,
    base: &[T],
) -> (Vec<T>, Vec<String>, Vec<(String, P)>, Option<Vec<String>>)
where
    T: Identified<EntityId> + Clone + Patchable<P>,
    P: Clone,
{
    match mutation {
        CollectionMutation::Add { index, item } => {
            let mut order: Vec<String> = base.iter().map(|row| row.id().0.clone()).collect();
            let at = (*index).min(order.len());
            order.insert(at, item.id().0.clone());
            (vec![item.clone()], Vec::new(), Vec::new(), Some(order))
        }
        CollectionMutation::Remove { id } => (Vec::new(), vec![id.0.clone()], Vec::new(), None),
        CollectionMutation::Patch { id, patch } => (Vec::new(), Vec::new(), vec![(id.0.clone(), patch.clone())], None),
        CollectionMutation::Move { id, to_index } => {
            let mut order: Vec<String> = base.iter().map(|row| row.id().0.clone()).collect();
            if let Some(from) = order.iter().position(|x| x == &id.0) {
                let moved = order.remove(from);
                let at = (*to_index).min(order.len());
                order.insert(at, moved);
            }
            (Vec::new(), Vec::new(), Vec::new(), Some(order))
        }
    }
}

/// 🏗️ Field delta for `stakeholders` collection mutation.
pub fn diff_stakeholders(mutation: &CollectionMutation<EntityId, Stakeholder, StakeholderPatch>, base: &[Stakeholder]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        stakeholders: Some(ProgramStakeholdersDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramStakeholdersPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `users` collection mutation.
pub fn diff_users(mutation: &CollectionMutation<EntityId, UserProfile, UserProfilePatch>, base: &[UserProfile]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        users: Some(ProgramUsersDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramUsersPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `activities` collection mutation.
pub fn diff_activities(mutation: &CollectionMutation<EntityId, Activity, ActivityPatch>, base: &[Activity]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        activities: Some(ProgramActivitiesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramActivitiesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `functions` collection mutation.
pub fn diff_functions(mutation: &CollectionMutation<EntityId, Function, FunctionPatch>, base: &[Function]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        functions: Some(ProgramFunctionsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramFunctionsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `elements` collection mutation.
pub fn diff_elements(mutation: &CollectionMutation<EntityId, ProgramElement, ProgramElementPatch>, base: &[ProgramElement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        elements: Some(ProgramElementsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramElementsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `quantities` collection mutation.
pub fn diff_quantities(mutation: &CollectionMutation<EntityId, QuantityRequirement, QuantityRequirementPatch>, base: &[QuantityRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        quantities: Some(ProgramQuantitiesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramQuantitiesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `relationships` collection mutation.
pub fn diff_relationships(mutation: &CollectionMutation<EntityId, Relationship, RelationshipPatch>, base: &[Relationship]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        relationships: Some(ProgramRelationshipsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramRelationshipsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `adjacencies` collection mutation.
pub fn diff_adjacencies(mutation: &CollectionMutation<EntityId, Adjacency, AdjacencyPatch>, base: &[Adjacency]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        adjacencies: Some(ProgramAdjacenciesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramAdjacenciesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `processes` collection mutation.
pub fn diff_processes(mutation: &CollectionMutation<EntityId, Process, ProcessPatch>, base: &[Process]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        processes: Some(ProgramProcessesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramProcessesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `flows` collection mutation.
pub fn diff_flows(mutation: &CollectionMutation<EntityId, FlowRequirement, FlowRequirementPatch>, base: &[FlowRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        flows: Some(ProgramFlowsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramFlowsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `access_rules` collection mutation.
pub fn diff_access_rules(mutation: &CollectionMutation<EntityId, AccessRule, AccessRulePatch>, base: &[AccessRule]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        access_rules: Some(ProgramAccessRulesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramAccessRulesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `operations` collection mutation.
pub fn diff_operations(mutation: &CollectionMutation<EntityId, OperationalRequirement, OperationalRequirementPatch>, base: &[OperationalRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        operations: Some(ProgramOperationsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramOperationsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `equipment` collection mutation.
pub fn diff_equipment(mutation: &CollectionMutation<EntityId, Equipment, EquipmentPatch>, base: &[Equipment]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        equipment: Some(ProgramEquipmentDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramEquipmentPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `resources` collection mutation.
pub fn diff_resources(mutation: &CollectionMutation<EntityId, Resource, ResourcePatch>, base: &[Resource]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        resources: Some(ProgramResourcesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramResourcesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `storage` collection mutation.
pub fn diff_storage(mutation: &CollectionMutation<EntityId, StorageRequirement, StorageRequirementPatch>, base: &[StorageRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        storage: Some(ProgramStorageDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramStoragePatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `environmental` collection mutation.
pub fn diff_environmental(mutation: &CollectionMutation<EntityId, EnvironmentalRequirement, EnvironmentalRequirementPatch>, base: &[EnvironmentalRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        environmental: Some(ProgramEnvironmentalDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramEnvironmentalPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `human_factors` collection mutation.
pub fn diff_human_factors(mutation: &CollectionMutation<EntityId, HumanFactorRequirement, HumanFactorRequirementPatch>, base: &[HumanFactorRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        human_factors: Some(ProgramHumanFactorsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramHumanFactorsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `accessibility` collection mutation.
pub fn diff_accessibility(mutation: &CollectionMutation<EntityId, AccessibilityRequirement, AccessibilityRequirementPatch>, base: &[AccessibilityRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        accessibility: Some(ProgramAccessibilityDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramAccessibilityPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `privacy` collection mutation.
pub fn diff_privacy(mutation: &CollectionMutation<EntityId, PrivacyRequirement, PrivacyRequirementPatch>, base: &[PrivacyRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        privacy: Some(ProgramPrivacyDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramPrivacyPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `safety` collection mutation.
pub fn diff_safety(mutation: &CollectionMutation<EntityId, SafetyRequirement, SafetyRequirementPatch>, base: &[SafetyRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        safety: Some(ProgramSafetyDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramSafetyPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `security` collection mutation.
pub fn diff_security(mutation: &CollectionMutation<EntityId, SecurityRequirement, SecurityRequirementPatch>, base: &[SecurityRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        security: Some(ProgramSecurityDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramSecurityPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `regulatory` collection mutation.
pub fn diff_regulatory(mutation: &CollectionMutation<EntityId, RegulatoryRequirement, RegulatoryRequirementPatch>, base: &[RegulatoryRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        regulatory: Some(ProgramRegulatoryDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramRegulatoryPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `site_context` collection mutation.
pub fn diff_site_context(mutation: &CollectionMutation<EntityId, SiteContext, SiteContextPatch>, base: &[SiteContext]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        site_context: Some(ProgramSiteContextDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramSiteContextPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `organizational` collection mutation.
pub fn diff_organizational(mutation: &CollectionMutation<EntityId, OrganizationalRequirement, OrganizationalRequirementPatch>, base: &[OrganizationalRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        organizational: Some(ProgramOrganizationalDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramOrganizationalPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `services` collection mutation.
pub fn diff_services(mutation: &CollectionMutation<EntityId, ServiceRequirement, ServiceRequirementPatch>, base: &[ServiceRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        services: Some(ProgramServicesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramServicesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `infrastructure` collection mutation.
pub fn diff_infrastructure(mutation: &CollectionMutation<EntityId, InfrastructureRequirement, InfrastructureRequirementPatch>, base: &[InfrastructureRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        infrastructure: Some(ProgramInfrastructureDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramInfrastructurePatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `information` collection mutation.
pub fn diff_information(mutation: &CollectionMutation<EntityId, InformationRequirement, InformationRequirementPatch>, base: &[InformationRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        information: Some(ProgramInformationDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramInformationPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `communication` collection mutation.
pub fn diff_communication(mutation: &CollectionMutation<EntityId, CommunicationRequirement, CommunicationRequirementPatch>, base: &[CommunicationRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        communication: Some(ProgramCommunicationDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramCommunicationPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `wayfinding` collection mutation.
pub fn diff_wayfinding(mutation: &CollectionMutation<EntityId, WayfindingRequirement, WayfindingRequirementPatch>, base: &[WayfindingRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        wayfinding: Some(ProgramWayfindingDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramWayfindingPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `schedules` collection mutation.
pub fn diff_schedules(mutation: &CollectionMutation<EntityId, ScheduleRequirement, ScheduleRequirementPatch>, base: &[ScheduleRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        schedules: Some(ProgramSchedulesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramSchedulesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `flexibility` collection mutation.
pub fn diff_flexibility(mutation: &CollectionMutation<EntityId, FlexibilityRequirement, FlexibilityRequirementPatch>, base: &[FlexibilityRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        flexibility: Some(ProgramFlexibilityDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramFlexibilityPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `growth` collection mutation.
pub fn diff_growth(mutation: &CollectionMutation<EntityId, GrowthPlan, GrowthPlanPatch>, base: &[GrowthPlan]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        growth: Some(ProgramGrowthDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramGrowthPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `sustainability` collection mutation.
pub fn diff_sustainability(mutation: &CollectionMutation<EntityId, SustainabilityRequirement, SustainabilityRequirementPatch>, base: &[SustainabilityRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        sustainability: Some(ProgramSustainabilityDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramSustainabilityPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `resilience` collection mutation.
pub fn diff_resilience(mutation: &CollectionMutation<EntityId, ResilienceRequirement, ResilienceRequirementPatch>, base: &[ResilienceRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        resilience: Some(ProgramResilienceDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramResiliencePatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `costs` collection mutation.
pub fn diff_costs(mutation: &CollectionMutation<EntityId, CostRequirement, CostRequirementPatch>, base: &[CostRequirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        costs: Some(ProgramCostsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramCostsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `delivery` collection mutation.
pub fn diff_delivery(mutation: &CollectionMutation<EntityId, DeliveryConstraint, DeliveryConstraintPatch>, base: &[DeliveryConstraint]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        delivery: Some(ProgramDeliveryDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramDeliveryPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `risks` collection mutation.
pub fn diff_risks(mutation: &CollectionMutation<EntityId, Risk, RiskPatch>, base: &[Risk]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        risks: Some(ProgramRisksDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramRisksPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `conflicts` collection mutation.
pub fn diff_conflicts(mutation: &CollectionMutation<EntityId, Conflict, ConflictPatch>, base: &[Conflict]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        conflicts: Some(ProgramConflictsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramConflictsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `requirements` collection mutation.
pub fn diff_requirements(mutation: &CollectionMutation<EntityId, Requirement, RequirementPatch>, base: &[Requirement]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        requirements: Some(ProgramRequirementsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramRequirementsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `priorities` collection mutation.
pub fn diff_priorities(mutation: &CollectionMutation<EntityId, PriorityRecord, PriorityRecordPatch>, base: &[PriorityRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        priorities: Some(ProgramPrioritiesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramPrioritiesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `scenarios` collection mutation.
pub fn diff_scenarios(mutation: &CollectionMutation<EntityId, Scenario, ScenarioPatch>, base: &[Scenario]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        scenarios: Some(ProgramScenariosDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramScenariosPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `options` collection mutation.
pub fn diff_options(mutation: &CollectionMutation<EntityId, OptionEvaluation, OptionEvaluationPatch>, base: &[OptionEvaluation]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        options: Some(ProgramOptionsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramOptionsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `decisions` collection mutation.
pub fn diff_decisions(mutation: &CollectionMutation<EntityId, Decision, DecisionPatch>, base: &[Decision]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        decisions: Some(ProgramDecisionsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramDecisionsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `validations` collection mutation.
pub fn diff_validations(mutation: &CollectionMutation<EntityId, ValidationRecord, ValidationRecordPatch>, base: &[ValidationRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        validations: Some(ProgramValidationsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramValidationsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `performance` collection mutation.
pub fn diff_performance(mutation: &CollectionMutation<EntityId, PerformanceCriterion, PerformanceCriterionPatch>, base: &[PerformanceCriterion]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        performance: Some(ProgramPerformanceDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramPerformancePatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `quality` collection mutation.
pub fn diff_quality(mutation: &CollectionMutation<EntityId, QualityRecord, QualityRecordPatch>, base: &[QualityRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        quality: Some(ProgramQualityDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramQualityPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `documents` collection mutation.
pub fn diff_documents(mutation: &CollectionMutation<EntityId, ArtifactRecord, ArtifactRecordPatch>, base: &[ArtifactRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        documents: Some(ProgramArtifactsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramArtifactsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `assumptions` collection mutation.
pub fn diff_assumptions(mutation: &CollectionMutation<EntityId, Assumption, AssumptionPatch>, base: &[Assumption]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        assumptions: Some(ProgramAssumptionsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramAssumptionsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `constraints` collection mutation.
pub fn diff_constraints(mutation: &CollectionMutation<EntityId, ConstraintRecord, ConstraintRecordPatch>, base: &[ConstraintRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        constraints: Some(ProgramConstraintsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramConstraintsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `compliance_records` collection mutation.
pub fn diff_compliance_records(mutation: &CollectionMutation<EntityId, ComplianceRecord, ComplianceRecordPatch>, base: &[ComplianceRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        compliance_records: Some(ProgramComplianceRecordsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramComplianceRecordsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `approvals` collection mutation.
pub fn diff_approvals(mutation: &CollectionMutation<EntityId, ApprovalRecord, ApprovalRecordPatch>, base: &[ApprovalRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        approvals: Some(ProgramApprovalsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramApprovalsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `meetings` collection mutation.
pub fn diff_meetings(mutation: &CollectionMutation<EntityId, MeetingRecord, MeetingRecordPatch>, base: &[MeetingRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        meetings: Some(ProgramMeetingsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramMeetingsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `changes` collection mutation.
pub fn diff_changes(mutation: &CollectionMutation<EntityId, ChangeRecord, ChangeRecordPatch>, base: &[ChangeRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        changes: Some(ProgramChangesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramChangesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `collaboration` collection mutation.
pub fn diff_collaboration(mutation: &CollectionMutation<EntityId, CollaborationRecord, CollaborationRecordPatch>, base: &[CollaborationRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        collaboration: Some(ProgramCollaborationDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramCollaborationPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `analyses` collection mutation.
pub fn diff_analyses(mutation: &CollectionMutation<EntityId, AnalysisRecord, AnalysisRecordPatch>, base: &[AnalysisRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        analyses: Some(ProgramAnalysesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramAnalysesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `reports` collection mutation.
pub fn diff_reports(mutation: &CollectionMutation<EntityId, ReportRecord, ReportRecordPatch>, base: &[ReportRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        reports: Some(ProgramReportsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramReportsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `search_filters` collection mutation.
pub fn diff_search_filters(mutation: &CollectionMutation<EntityId, SearchFilter, SearchFilterPatch>, base: &[SearchFilter]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        search_filters: Some(ProgramSearchFiltersDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramSearchFiltersPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `status_records` collection mutation.
pub fn diff_status_records(mutation: &CollectionMutation<EntityId, StatusRecord, StatusRecordPatch>, base: &[StatusRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        status_records: Some(ProgramStatusRecordsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramStatusRecordsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `workshops` collection mutation.
pub fn diff_workshops(mutation: &CollectionMutation<EntityId, Workshop, WorkshopPatch>, base: &[Workshop]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        workshops: Some(ProgramWorkshopsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramWorkshopsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `surveys` collection mutation.
pub fn diff_surveys(mutation: &CollectionMutation<EntityId, Survey, SurveyPatch>, base: &[Survey]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        surveys: Some(ProgramSurveysDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramSurveysPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `issues` collection mutation.
pub fn diff_issues(mutation: &CollectionMutation<EntityId, Issue, IssuePatch>, base: &[Issue]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        issues: Some(ProgramIssuesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramIssuesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `audit_events` collection mutation.
pub fn diff_audit_events(mutation: &CollectionMutation<EntityId, AuditEvent, AuditEventPatch>, base: &[AuditEvent]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        audit_events: Some(ProgramAuditEventsDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramAuditEventsPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `templates` collection mutation.
pub fn diff_templates(mutation: &CollectionMutation<EntityId, TemplateRecord, TemplateRecordPatch>, base: &[TemplateRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        templates: Some(ProgramTemplatesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramTemplatesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `knowledge` collection mutation.
pub fn diff_knowledge(mutation: &CollectionMutation<EntityId, KnowledgeRecord, KnowledgeRecordPatch>, base: &[KnowledgeRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        knowledge: Some(ProgramKnowledgeDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramKnowledgePatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `benchmarks` collection mutation.
pub fn diff_benchmarks(mutation: &CollectionMutation<EntityId, BenchmarkRecord, BenchmarkRecordPatch>, base: &[BenchmarkRecord]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        benchmarks: Some(ProgramBenchmarksDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramBenchmarksPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Field delta for `traces` collection mutation.
pub fn diff_traces(mutation: &CollectionMutation<EntityId, TraceLink, TraceLinkPatch>, base: &[TraceLink]) -> ProgramDiff {
    let (added, removed, patched, reordered) = collection_delta_from_mutation(mutation, base);
    ProgramDiff {
        traces: Some(ProgramTracesDelta {
            added,
            removed,
            patched: patched.into_iter().map(|(id, patch)| ProgramTracesPatchEntry { id, patch }).collect(),
            reordered,
        }),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Whole snapshot replacement.
pub fn diff_replace_snapshot(_before: &ProgramSnapshot, after: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff {
        artifact: Some(Box::new(ProgramArtifact::from_snapshot(after.clone()))),
        ..ProgramDiff::default()
    }
}

/// 🏗️ Meta / project / governance whole-record replacements.
pub fn diff_meta(meta: ProgramMeta) -> ProgramDiff {
    ProgramDiff { meta: Some(meta), ..ProgramDiff::default() }
}
pub fn diff_project(project: ProjectDefinition) -> ProgramDiff {
    ProgramDiff { project: Some(project), ..ProgramDiff::default() }
}
pub fn diff_governance(governance: Governance) -> ProgramDiff {
    ProgramDiff { governance: Some(governance), ..ProgramDiff::default() }
}
pub fn diff_adjacencies_set(adjacency: Adjacency, base: &[Adjacency]) -> ProgramDiff {
    // upsert: patch if exists else add
    if base.iter().any(|row| row.header.id == adjacency.header.id) {
        // represent as remove+add via patched whole replacement — use remove+add
        let id = adjacency.header.id.0.clone();
        ProgramDiff {
            adjacencies: Some(ProgramAdjacenciesDelta {
                added: vec![adjacency],
                removed: vec![id],
                patched: Vec::new(),
                reordered: None,
            }),
            ..ProgramDiff::default()
        }
    } else {
        ProgramDiff {
            adjacencies: Some(ProgramAdjacenciesDelta {
                added: vec![adjacency],
                removed: Vec::new(),
                patched: Vec::new(),
                reordered: None,
            }),
            ..ProgramDiff::default()
        }
    }
}
pub fn diff_adjacencies_clear(id: &EntityId) -> ProgramDiff {
    ProgramDiff {
        adjacencies: Some(ProgramAdjacenciesDelta {
            added: Vec::new(),
            removed: vec![id.0.clone()],
            patched: Vec::new(),
            reordered: None,
        }),
        ..ProgramDiff::default()
    }
}
//#endregion 🔖️Constructors

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;
    use protocol::CollectionMutation;

    #[test]
    fn a_diff_applies_element_removal() {
        let program = sample_plugin();
        let element_id = program.elements[0].header.id.clone();
        let diff = diff_elements(
            &CollectionMutation::Remove { id: element_id.clone() },
            &program.elements,
        );
        let next = diff.apply(&program);
        assert!(!next.elements.iter().any(|row| row.header.id == element_id));
    }

    #[test]
    fn absorb_merges_collection_deltas() {
        let mut left = diff_adjacencies_clear(&EntityId("a".into()));
        left.absorb(diff_adjacencies_clear(&EntityId("b".into())));
        assert_eq!(left.adjacencies.as_ref().unwrap().removed.len(), 2);
    }
}
//#endregion 🧪️Tests
