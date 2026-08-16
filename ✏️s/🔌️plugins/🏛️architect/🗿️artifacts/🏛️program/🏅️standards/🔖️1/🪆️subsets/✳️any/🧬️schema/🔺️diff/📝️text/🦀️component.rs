//! 📦️ Architect program artifact — sparse field-delta runtime (constitutional: diff).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::program::schema::diff::*;

use crate::artifacts::program::kernel::*;
use crate::artifacts::program::schema::ProgramArtifact;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{Identified, MutationDiff, Patchable};

//#region 🔖️Apply
impl ProgramDiff {
    /// 🧬️ Apply every field entry onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &ProgramArtifact) -> ProgramArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(v) = &self.schema {
            next.schema = v.clone();
        }
        if let Some(v) = &self.meta {
            next.meta = v.clone();
        }
        if let Some(v) = &self.project {
            next.project = v.clone();
        }
        if let Some(v) = &self.governance {
            next.governance = v.clone();
        }

        if let Some(delta) = &self.stakeholders {
            apply_collection_delta(&mut next.stakeholders, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.users {
            apply_collection_delta(&mut next.users, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.activities {
            apply_collection_delta(&mut next.activities, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.functions {
            apply_collection_delta(&mut next.functions, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.elements {
            apply_collection_delta(&mut next.elements, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.quantities {
            apply_collection_delta(&mut next.quantities, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.relationships {
            apply_collection_delta(&mut next.relationships, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.adjacencies {
            apply_collection_delta(&mut next.adjacencies, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.processes {
            apply_collection_delta(&mut next.processes, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.flows {
            apply_collection_delta(&mut next.flows, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.access_rules {
            apply_collection_delta(&mut next.access_rules, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.operations {
            apply_collection_delta(&mut next.operations, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.equipment {
            apply_collection_delta(&mut next.equipment, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.resources {
            apply_collection_delta(&mut next.resources, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.storage {
            apply_collection_delta(&mut next.storage, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.environmental {
            apply_collection_delta(&mut next.environmental, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.human_factors {
            apply_collection_delta(&mut next.human_factors, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.accessibility {
            apply_collection_delta(&mut next.accessibility, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.privacy {
            apply_collection_delta(&mut next.privacy, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.safety {
            apply_collection_delta(&mut next.safety, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.security {
            apply_collection_delta(&mut next.security, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.regulatory {
            apply_collection_delta(&mut next.regulatory, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.site_context {
            apply_collection_delta(&mut next.site_context, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.organizational {
            apply_collection_delta(&mut next.organizational, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.services {
            apply_collection_delta(&mut next.services, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.infrastructure {
            apply_collection_delta(&mut next.infrastructure, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.information {
            apply_collection_delta(&mut next.information, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.communication {
            apply_collection_delta(&mut next.communication, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.wayfinding {
            apply_collection_delta(&mut next.wayfinding, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.schedules {
            apply_collection_delta(&mut next.schedules, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.flexibility {
            apply_collection_delta(&mut next.flexibility, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.growth {
            apply_collection_delta(&mut next.growth, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.sustainability {
            apply_collection_delta(&mut next.sustainability, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.resilience {
            apply_collection_delta(&mut next.resilience, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.costs {
            apply_collection_delta(&mut next.costs, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.delivery {
            apply_collection_delta(&mut next.delivery, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.risks {
            apply_collection_delta(&mut next.risks, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.conflicts {
            apply_collection_delta(&mut next.conflicts, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.requirements {
            apply_collection_delta(&mut next.requirements, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.priorities {
            apply_collection_delta(&mut next.priorities, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.scenarios {
            apply_collection_delta(&mut next.scenarios, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.options {
            apply_collection_delta(&mut next.options, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.decisions {
            apply_collection_delta(&mut next.decisions, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.validations {
            apply_collection_delta(&mut next.validations, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.performance {
            apply_collection_delta(&mut next.performance, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.quality {
            apply_collection_delta(&mut next.quality, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.documents {
            apply_collection_delta(&mut next.artifacts, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.assumptions {
            apply_collection_delta(&mut next.assumptions, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.constraints {
            apply_collection_delta(&mut next.constraints, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.compliance_records {
            apply_collection_delta(&mut next.compliance_records, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.approvals {
            apply_collection_delta(&mut next.approvals, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.meetings {
            apply_collection_delta(&mut next.meetings, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.changes {
            apply_collection_delta(&mut next.changes, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.collaboration {
            apply_collection_delta(&mut next.collaboration, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.analyses {
            apply_collection_delta(&mut next.analyses, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.reports {
            apply_collection_delta(&mut next.reports, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.search_filters {
            apply_collection_delta(&mut next.search_filters, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.status_records {
            apply_collection_delta(&mut next.status_records, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.workshops {
            apply_collection_delta(&mut next.workshops, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.surveys {
            apply_collection_delta(&mut next.surveys, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.issues {
            apply_collection_delta(&mut next.issues, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.audit_events {
            apply_collection_delta(&mut next.audit_events, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(delta) = &self.templates {
            apply_collection_delta(&mut next.templates, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(child) = &self.knowledge {
            next.knowledge = child.clone();
        }
        if let Some(child) = &self.benchmarks {
            next.benchmarks = child.clone();
        }
        if let Some(delta) = &self.traces {
            apply_collection_delta(&mut next.traces, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered);
        }
        if let Some(v) = &self.selected_ids {
            next.selected_ids = v.values.clone();
        }
        if let Some(v) = &self.active_register {
            next.active_register = v.clone();
        }
        if let Some(v) = &self.adjacency_kind_filter {
            next.adjacency_kind_filter = v.clone();
        }
        if let Some(v) = &self.active_report_json {
            next.active_report_json = v.clone();
        }
        if let Some(v) = &self.search_query {
            next.search_query = v.clone();
        }
        if let Some(v) = &self.search_history_json {
            next.search_history_json = v.clone();
        }
        if let Some(v) = &self.last_result_json {
            next.last_result_json = v.clone();
        }
        if let Some(v) = &self.last_analysis_json {
            next.last_analysis_json = v.clone();
        }
        if let Some(v) = &self.graph_camera_x {
            next.graph_camera_x = *v;
        }
        if let Some(v) = &self.graph_camera_y {
            next.graph_camera_y = *v;
        }
        if let Some(v) = &self.graph_camera_zoom {
            next.graph_camera_zoom = *v;
        }
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
        macro_rules! absorb_opt {
            ($f:ident) => {
                if other.$f.is_some() {
                    self.$f = other.$f;
                }
            };
        }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.templates = Some(delta),
            }
        }
        if other.knowledge.is_some() {
            self.knowledge = other.knowledge;
        }
        if other.benchmarks.is_some() {
            self.benchmarks = other.benchmarks;
        }
        if let Some(delta) = other.traces {
            match &mut self.traces {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
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

fn apply_collection_delta<T, P>(items: &mut Vec<T>, added: &[T], removed: &[String], patched: &[(String, P)], reordered: &Option<Vec<String>>)
where
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧬️ `Wave C` (SEMANTIC-MUTATIONS-OVERHAUL) removed this facet's `🔖️Constructors` region — a
    /// dead helper set parametrized over the old generic per-collection add/remove/patch wrapper,
    /// with zero external callers
    /// (superseded by the semantic `🧬️mutations` triads' own diff leaves) — per the ticket's banned-
    /// vocabulary final sweep. `apply_to_artifact` (the one real, still-live function in this file)
    /// keeps its own coverage here instead.
    #[test]
    fn apply_to_artifact_applies_a_scalar_field() {
        let artifact = ProgramArtifact::default();
        let mut renamed_meta = artifact.meta.clone();
        renamed_meta.title = "Renamed".into();
        let diff = ProgramDiff { meta: Some(renamed_meta.clone()), ..Default::default() };
        let next = diff.apply_to_artifact(&artifact);
        assert_eq!(next.meta.title, "Renamed");
    }

    #[test]
    fn apply_to_artifact_full_replacement_wins_over_field_entries() {
        let artifact = ProgramArtifact::default();
        let mut replacement = artifact.clone();
        replacement.schema = "s.architect.program@2".into();
        let diff = ProgramDiff { artifact: Some(Box::new(replacement.clone())), schema: Some("ignored".into()), ..Default::default() };
        let next = diff.apply_to_artifact(&artifact);
        assert_eq!(next, replacement);
    }
}
//#endregion 🧪️Tests
