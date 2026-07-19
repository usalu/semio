//! 🔁 Program VCS operations — `CollectionOp` per register plus meta, adjacency, and bulk set.

use crate::adjacency::{clear_adjacency, set_adjacency};
use crate::kernel::{EntityId, TraceLink};
use crate::program::Program;
use crate::registers::*;
use serde::{Deserialize, Serialize};
use vcs::{
    apply_collection_op, invert_collection_op, CollectionOp, Operation, OperationDiff, Patchable,
};

// #region 🔖ProgramOp
/// @emoji 🧩 Typed program document operation for VCS replay and undo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum ProgramOp {
    Stakeholders(CollectionOp<EntityId, Stakeholder, StakeholderPatch>),
    Users(CollectionOp<EntityId, UserProfile, UserProfilePatch>),
    Activities(CollectionOp<EntityId, Activity, ActivityPatch>),
    Functions(CollectionOp<EntityId, Function, FunctionPatch>),
    Elements(CollectionOp<EntityId, ProgramElement, ProgramElementPatch>),
    Quantities(CollectionOp<EntityId, QuantityRequirement, QuantityRequirementPatch>),
    Relationships(CollectionOp<EntityId, Relationship, RelationshipPatch>),
    Adjacencies(CollectionOp<EntityId, Adjacency, AdjacencyPatch>),
    Processes(CollectionOp<EntityId, Process, ProcessPatch>),
    Flows(CollectionOp<EntityId, FlowRequirement, FlowRequirementPatch>),
    AccessRules(CollectionOp<EntityId, AccessRule, AccessRulePatch>),
    Operations(CollectionOp<EntityId, OperationalRequirement, OperationalRequirementPatch>),
    Equipment(CollectionOp<EntityId, Equipment, EquipmentPatch>),
    Resources(CollectionOp<EntityId, Resource, ResourcePatch>),
    Storage(CollectionOp<EntityId, StorageRequirement, StorageRequirementPatch>),
    Environmental(CollectionOp<EntityId, EnvironmentalRequirement, EnvironmentalRequirementPatch>),
    HumanFactors(CollectionOp<EntityId, HumanFactorRequirement, HumanFactorRequirementPatch>),
    Accessibility(CollectionOp<EntityId, AccessibilityRequirement, AccessibilityRequirementPatch>),
    Privacy(CollectionOp<EntityId, PrivacyRequirement, PrivacyRequirementPatch>),
    Safety(CollectionOp<EntityId, SafetyRequirement, SafetyRequirementPatch>),
    Security(CollectionOp<EntityId, SecurityRequirement, SecurityRequirementPatch>),
    Regulatory(CollectionOp<EntityId, RegulatoryRequirement, RegulatoryRequirementPatch>),
    SiteContext(CollectionOp<EntityId, SiteContext, SiteContextPatch>),
    Organizational(CollectionOp<EntityId, OrganizationalRequirement, OrganizationalRequirementPatch>),
    Services(CollectionOp<EntityId, ServiceRequirement, ServiceRequirementPatch>),
    Infrastructure(CollectionOp<EntityId, InfrastructureRequirement, InfrastructureRequirementPatch>),
    Information(CollectionOp<EntityId, InformationRequirement, InformationRequirementPatch>),
    Communication(CollectionOp<EntityId, CommunicationRequirement, CommunicationRequirementPatch>),
    Wayfinding(CollectionOp<EntityId, WayfindingRequirement, WayfindingRequirementPatch>),
    Schedules(CollectionOp<EntityId, ScheduleRequirement, ScheduleRequirementPatch>),
    Flexibility(CollectionOp<EntityId, FlexibilityRequirement, FlexibilityRequirementPatch>),
    Growth(CollectionOp<EntityId, GrowthPlan, GrowthPlanPatch>),
    Sustainability(CollectionOp<EntityId, SustainabilityRequirement, SustainabilityRequirementPatch>),
    Resilience(CollectionOp<EntityId, ResilienceRequirement, ResilienceRequirementPatch>),
    Costs(CollectionOp<EntityId, CostRequirement, CostRequirementPatch>),
    Delivery(CollectionOp<EntityId, DeliveryConstraint, DeliveryConstraintPatch>),
    Risks(CollectionOp<EntityId, Risk, RiskPatch>),
    Conflicts(CollectionOp<EntityId, Conflict, ConflictPatch>),
    Requirements(CollectionOp<EntityId, Requirement, RequirementPatch>),
    Priorities(CollectionOp<EntityId, PriorityRecord, PriorityRecordPatch>),
    Scenarios(CollectionOp<EntityId, Scenario, ScenarioPatch>),
    Options(CollectionOp<EntityId, OptionEvaluation, OptionEvaluationPatch>),
    Decisions(CollectionOp<EntityId, Decision, DecisionPatch>),
    Validations(CollectionOp<EntityId, ValidationRecord, ValidationRecordPatch>),
    Performance(CollectionOp<EntityId, PerformanceCriterion, PerformanceCriterionPatch>),
    Quality(CollectionOp<EntityId, QualityRecord, QualityRecordPatch>),
    Documents(CollectionOp<EntityId, DocumentRecord, DocumentRecordPatch>),
    Changes(CollectionOp<EntityId, ChangeRecord, ChangeRecordPatch>),
    Collaboration(CollectionOp<EntityId, CollaborationRecord, CollaborationRecordPatch>),
    Analyses(CollectionOp<EntityId, AnalysisRecord, AnalysisRecordPatch>),
    Reports(CollectionOp<EntityId, ReportRecord, ReportRecordPatch>),
    SearchFilters(CollectionOp<EntityId, SearchFilter, SearchFilterPatch>),
    StatusRecords(CollectionOp<EntityId, StatusRecord, StatusRecordPatch>),
    Workshops(CollectionOp<EntityId, Workshop, WorkshopPatch>),
    Surveys(CollectionOp<EntityId, Survey, SurveyPatch>),
    Issues(CollectionOp<EntityId, Issue, IssuePatch>),
    AuditEvents(CollectionOp<EntityId, AuditEvent, AuditEventPatch>),
    Templates(CollectionOp<EntityId, TemplateRecord, TemplateRecordPatch>),
    Knowledge(CollectionOp<EntityId, KnowledgeRecord, KnowledgeRecordPatch>),
    Benchmarks(CollectionOp<EntityId, BenchmarkRecord, BenchmarkRecordPatch>),
    Assumptions(CollectionOp<EntityId, Assumption, AssumptionPatch>),
    Constraints(CollectionOp<EntityId, ConstraintRecord, ConstraintRecordPatch>),
    ComplianceRecords(CollectionOp<EntityId, ComplianceRecord, ComplianceRecordPatch>),
    Approvals(CollectionOp<EntityId, ApprovalRecord, ApprovalRecordPatch>),
    Meetings(CollectionOp<EntityId, MeetingRecord, MeetingRecordPatch>),
    Traces(CollectionOp<EntityId, TraceLink, TraceLinkPatch>),
    UpdateMeta { patch: ProgramMetaPatch },
    UpdateProject { patch: ProjectDefinitionPatch },
    UpdateGovernance { patch: GovernancePatch },
    SetAdjacency { adjacency: Adjacency },
    ClearAdjacency { id: EntityId },
    SetProgram { program: Program },
}

/// @emoji 🩹 Inverse patch carrier for trace link collection ops.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceLinkPatch {
    pub from_id: Option<EntityId>,
    pub to_id: Option<EntityId>,
    pub kind: Option<crate::kernel::TraceKind>,
    pub label: Option<Option<String>>,
}

impl Patchable<TraceLinkPatch> for TraceLink {
    fn apply_patch(&mut self, patch: &TraceLinkPatch) -> TraceLinkPatch {
        let mut inverse = TraceLinkPatch::default();
        if let Some(value) = &patch.from_id {
            inverse.from_id = Some(self.from_id.clone());
            self.from_id = value.clone();
        }
        if let Some(value) = &patch.to_id {
            inverse.to_id = Some(self.to_id.clone());
            self.to_id = value.clone();
        }
        if let Some(value) = &patch.kind {
            inverse.kind = Some(self.kind.clone());
            self.kind = value.clone();
        }
        if let Some(value) = &patch.label {
            inverse.label = Some(self.label.clone());
            self.label = value.clone();
        }
        inverse
    }
}
// #endregion

// #region 🔖Apply
/// @emoji ▶️ Applies one program op to the document in place.
pub fn apply_program_op(program: &mut Program, op: &ProgramOp) {
    match op {
        ProgramOp::Stakeholders(collection_op) => apply_collection_op(&mut program.stakeholders, collection_op),
        ProgramOp::Users(collection_op) => apply_collection_op(&mut program.users, collection_op),
        ProgramOp::Activities(collection_op) => apply_collection_op(&mut program.activities, collection_op),
        ProgramOp::Functions(collection_op) => apply_collection_op(&mut program.functions, collection_op),
        ProgramOp::Elements(collection_op) => apply_collection_op(&mut program.elements, collection_op),
        ProgramOp::Quantities(collection_op) => apply_collection_op(&mut program.quantities, collection_op),
        ProgramOp::Relationships(collection_op) => apply_collection_op(&mut program.relationships, collection_op),
        ProgramOp::Adjacencies(collection_op) => apply_collection_op(&mut program.adjacencies, collection_op),
        ProgramOp::Processes(collection_op) => apply_collection_op(&mut program.processes, collection_op),
        ProgramOp::Flows(collection_op) => apply_collection_op(&mut program.flows, collection_op),
        ProgramOp::AccessRules(collection_op) => apply_collection_op(&mut program.access_rules, collection_op),
        ProgramOp::Operations(collection_op) => apply_collection_op(&mut program.operations, collection_op),
        ProgramOp::Equipment(collection_op) => apply_collection_op(&mut program.equipment, collection_op),
        ProgramOp::Resources(collection_op) => apply_collection_op(&mut program.resources, collection_op),
        ProgramOp::Storage(collection_op) => apply_collection_op(&mut program.storage, collection_op),
        ProgramOp::Environmental(collection_op) => apply_collection_op(&mut program.environmental, collection_op),
        ProgramOp::HumanFactors(collection_op) => apply_collection_op(&mut program.human_factors, collection_op),
        ProgramOp::Accessibility(collection_op) => apply_collection_op(&mut program.accessibility, collection_op),
        ProgramOp::Privacy(collection_op) => apply_collection_op(&mut program.privacy, collection_op),
        ProgramOp::Safety(collection_op) => apply_collection_op(&mut program.safety, collection_op),
        ProgramOp::Security(collection_op) => apply_collection_op(&mut program.security, collection_op),
        ProgramOp::Regulatory(collection_op) => apply_collection_op(&mut program.regulatory, collection_op),
        ProgramOp::SiteContext(collection_op) => apply_collection_op(&mut program.site_context, collection_op),
        ProgramOp::Organizational(collection_op) => apply_collection_op(&mut program.organizational, collection_op),
        ProgramOp::Services(collection_op) => apply_collection_op(&mut program.services, collection_op),
        ProgramOp::Infrastructure(collection_op) => apply_collection_op(&mut program.infrastructure, collection_op),
        ProgramOp::Information(collection_op) => apply_collection_op(&mut program.information, collection_op),
        ProgramOp::Communication(collection_op) => apply_collection_op(&mut program.communication, collection_op),
        ProgramOp::Wayfinding(collection_op) => apply_collection_op(&mut program.wayfinding, collection_op),
        ProgramOp::Schedules(collection_op) => apply_collection_op(&mut program.schedules, collection_op),
        ProgramOp::Flexibility(collection_op) => apply_collection_op(&mut program.flexibility, collection_op),
        ProgramOp::Growth(collection_op) => apply_collection_op(&mut program.growth, collection_op),
        ProgramOp::Sustainability(collection_op) => apply_collection_op(&mut program.sustainability, collection_op),
        ProgramOp::Resilience(collection_op) => apply_collection_op(&mut program.resilience, collection_op),
        ProgramOp::Costs(collection_op) => apply_collection_op(&mut program.costs, collection_op),
        ProgramOp::Delivery(collection_op) => apply_collection_op(&mut program.delivery, collection_op),
        ProgramOp::Risks(collection_op) => apply_collection_op(&mut program.risks, collection_op),
        ProgramOp::Conflicts(collection_op) => apply_collection_op(&mut program.conflicts, collection_op),
        ProgramOp::Requirements(collection_op) => apply_collection_op(&mut program.requirements, collection_op),
        ProgramOp::Priorities(collection_op) => apply_collection_op(&mut program.priorities, collection_op),
        ProgramOp::Scenarios(collection_op) => apply_collection_op(&mut program.scenarios, collection_op),
        ProgramOp::Options(collection_op) => apply_collection_op(&mut program.options, collection_op),
        ProgramOp::Decisions(collection_op) => apply_collection_op(&mut program.decisions, collection_op),
        ProgramOp::Validations(collection_op) => apply_collection_op(&mut program.validations, collection_op),
        ProgramOp::Performance(collection_op) => apply_collection_op(&mut program.performance, collection_op),
        ProgramOp::Quality(collection_op) => apply_collection_op(&mut program.quality, collection_op),
        ProgramOp::Documents(collection_op) => apply_collection_op(&mut program.documents, collection_op),
        ProgramOp::Changes(collection_op) => apply_collection_op(&mut program.changes, collection_op),
        ProgramOp::Collaboration(collection_op) => apply_collection_op(&mut program.collaboration, collection_op),
        ProgramOp::Analyses(collection_op) => apply_collection_op(&mut program.analyses, collection_op),
        ProgramOp::Reports(collection_op) => apply_collection_op(&mut program.reports, collection_op),
        ProgramOp::SearchFilters(collection_op) => apply_collection_op(&mut program.search_filters, collection_op),
        ProgramOp::StatusRecords(collection_op) => apply_collection_op(&mut program.status_records, collection_op),
        ProgramOp::Workshops(collection_op) => apply_collection_op(&mut program.workshops, collection_op),
        ProgramOp::Surveys(collection_op) => apply_collection_op(&mut program.surveys, collection_op),
        ProgramOp::Issues(collection_op) => apply_collection_op(&mut program.issues, collection_op),
        ProgramOp::AuditEvents(collection_op) => apply_collection_op(&mut program.audit_events, collection_op),
        ProgramOp::Templates(collection_op) => apply_collection_op(&mut program.templates, collection_op),
        ProgramOp::Knowledge(collection_op) => apply_collection_op(&mut program.knowledge, collection_op),
        ProgramOp::Benchmarks(collection_op) => apply_collection_op(&mut program.benchmarks, collection_op),
        ProgramOp::Assumptions(collection_op) => apply_collection_op(&mut program.assumptions, collection_op),
        ProgramOp::Constraints(collection_op) => apply_collection_op(&mut program.constraints, collection_op),
        ProgramOp::ComplianceRecords(collection_op) => {
            apply_collection_op(&mut program.compliance_records, collection_op)
        }
        ProgramOp::Approvals(collection_op) => apply_collection_op(&mut program.approvals, collection_op),
        ProgramOp::Meetings(collection_op) => apply_collection_op(&mut program.meetings, collection_op),
        ProgramOp::Traces(collection_op) => apply_collection_op(&mut program.traces, collection_op),
        ProgramOp::UpdateMeta { patch } => {
            program.meta.apply_patch(patch);
        }
        ProgramOp::UpdateProject { patch } => {
            program.project.apply_patch(patch);
        }
        ProgramOp::UpdateGovernance { patch } => {
            program.governance.apply_patch(patch);
        }
        ProgramOp::SetAdjacency { adjacency } => set_adjacency(program, adjacency.clone()),
        ProgramOp::ClearAdjacency { id } => clear_adjacency(program, id),
        ProgramOp::SetProgram { program: replacement } => *program = replacement.clone(),
    }
}

/// @emoji ↩️ Computes the inverse op from pre-state for undo.
pub fn invert_program_op(program: &Program, op: &ProgramOp) -> ProgramOp {
    match op {
        ProgramOp::Stakeholders(collection_op) => {
            ProgramOp::Stakeholders(invert_collection_op(&program.stakeholders, collection_op))
        }
        ProgramOp::Users(collection_op) => ProgramOp::Users(invert_collection_op(&program.users, collection_op)),
        ProgramOp::Activities(collection_op) => {
            ProgramOp::Activities(invert_collection_op(&program.activities, collection_op))
        }
        ProgramOp::Functions(collection_op) => ProgramOp::Functions(invert_collection_op(&program.functions, collection_op)),
        ProgramOp::Elements(collection_op) => ProgramOp::Elements(invert_collection_op(&program.elements, collection_op)),
        ProgramOp::Quantities(collection_op) => {
            ProgramOp::Quantities(invert_collection_op(&program.quantities, collection_op))
        }
        ProgramOp::Relationships(collection_op) => {
            ProgramOp::Relationships(invert_collection_op(&program.relationships, collection_op))
        }
        ProgramOp::Adjacencies(collection_op) => {
            ProgramOp::Adjacencies(invert_collection_op(&program.adjacencies, collection_op))
        }
        ProgramOp::Processes(collection_op) => ProgramOp::Processes(invert_collection_op(&program.processes, collection_op)),
        ProgramOp::Flows(collection_op) => ProgramOp::Flows(invert_collection_op(&program.flows, collection_op)),
        ProgramOp::AccessRules(collection_op) => {
            ProgramOp::AccessRules(invert_collection_op(&program.access_rules, collection_op))
        }
        ProgramOp::Operations(collection_op) => {
            ProgramOp::Operations(invert_collection_op(&program.operations, collection_op))
        }
        ProgramOp::Equipment(collection_op) => ProgramOp::Equipment(invert_collection_op(&program.equipment, collection_op)),
        ProgramOp::Resources(collection_op) => ProgramOp::Resources(invert_collection_op(&program.resources, collection_op)),
        ProgramOp::Storage(collection_op) => ProgramOp::Storage(invert_collection_op(&program.storage, collection_op)),
        ProgramOp::Environmental(collection_op) => {
            ProgramOp::Environmental(invert_collection_op(&program.environmental, collection_op))
        }
        ProgramOp::HumanFactors(collection_op) => {
            ProgramOp::HumanFactors(invert_collection_op(&program.human_factors, collection_op))
        }
        ProgramOp::Accessibility(collection_op) => {
            ProgramOp::Accessibility(invert_collection_op(&program.accessibility, collection_op))
        }
        ProgramOp::Privacy(collection_op) => ProgramOp::Privacy(invert_collection_op(&program.privacy, collection_op)),
        ProgramOp::Safety(collection_op) => ProgramOp::Safety(invert_collection_op(&program.safety, collection_op)),
        ProgramOp::Security(collection_op) => ProgramOp::Security(invert_collection_op(&program.security, collection_op)),
        ProgramOp::Regulatory(collection_op) => {
            ProgramOp::Regulatory(invert_collection_op(&program.regulatory, collection_op))
        }
        ProgramOp::SiteContext(collection_op) => {
            ProgramOp::SiteContext(invert_collection_op(&program.site_context, collection_op))
        }
        ProgramOp::Organizational(collection_op) => {
            ProgramOp::Organizational(invert_collection_op(&program.organizational, collection_op))
        }
        ProgramOp::Services(collection_op) => ProgramOp::Services(invert_collection_op(&program.services, collection_op)),
        ProgramOp::Infrastructure(collection_op) => {
            ProgramOp::Infrastructure(invert_collection_op(&program.infrastructure, collection_op))
        }
        ProgramOp::Information(collection_op) => {
            ProgramOp::Information(invert_collection_op(&program.information, collection_op))
        }
        ProgramOp::Communication(collection_op) => {
            ProgramOp::Communication(invert_collection_op(&program.communication, collection_op))
        }
        ProgramOp::Wayfinding(collection_op) => {
            ProgramOp::Wayfinding(invert_collection_op(&program.wayfinding, collection_op))
        }
        ProgramOp::Schedules(collection_op) => ProgramOp::Schedules(invert_collection_op(&program.schedules, collection_op)),
        ProgramOp::Flexibility(collection_op) => {
            ProgramOp::Flexibility(invert_collection_op(&program.flexibility, collection_op))
        }
        ProgramOp::Growth(collection_op) => ProgramOp::Growth(invert_collection_op(&program.growth, collection_op)),
        ProgramOp::Sustainability(collection_op) => {
            ProgramOp::Sustainability(invert_collection_op(&program.sustainability, collection_op))
        }
        ProgramOp::Resilience(collection_op) => {
            ProgramOp::Resilience(invert_collection_op(&program.resilience, collection_op))
        }
        ProgramOp::Costs(collection_op) => ProgramOp::Costs(invert_collection_op(&program.costs, collection_op)),
        ProgramOp::Delivery(collection_op) => ProgramOp::Delivery(invert_collection_op(&program.delivery, collection_op)),
        ProgramOp::Risks(collection_op) => ProgramOp::Risks(invert_collection_op(&program.risks, collection_op)),
        ProgramOp::Conflicts(collection_op) => ProgramOp::Conflicts(invert_collection_op(&program.conflicts, collection_op)),
        ProgramOp::Requirements(collection_op) => {
            ProgramOp::Requirements(invert_collection_op(&program.requirements, collection_op))
        }
        ProgramOp::Priorities(collection_op) => {
            ProgramOp::Priorities(invert_collection_op(&program.priorities, collection_op))
        }
        ProgramOp::Scenarios(collection_op) => ProgramOp::Scenarios(invert_collection_op(&program.scenarios, collection_op)),
        ProgramOp::Options(collection_op) => ProgramOp::Options(invert_collection_op(&program.options, collection_op)),
        ProgramOp::Decisions(collection_op) => ProgramOp::Decisions(invert_collection_op(&program.decisions, collection_op)),
        ProgramOp::Validations(collection_op) => {
            ProgramOp::Validations(invert_collection_op(&program.validations, collection_op))
        }
        ProgramOp::Performance(collection_op) => {
            ProgramOp::Performance(invert_collection_op(&program.performance, collection_op))
        }
        ProgramOp::Quality(collection_op) => ProgramOp::Quality(invert_collection_op(&program.quality, collection_op)),
        ProgramOp::Documents(collection_op) => ProgramOp::Documents(invert_collection_op(&program.documents, collection_op)),
        ProgramOp::Changes(collection_op) => ProgramOp::Changes(invert_collection_op(&program.changes, collection_op)),
        ProgramOp::Collaboration(collection_op) => {
            ProgramOp::Collaboration(invert_collection_op(&program.collaboration, collection_op))
        }
        ProgramOp::Analyses(collection_op) => ProgramOp::Analyses(invert_collection_op(&program.analyses, collection_op)),
        ProgramOp::Reports(collection_op) => ProgramOp::Reports(invert_collection_op(&program.reports, collection_op)),
        ProgramOp::SearchFilters(collection_op) => {
            ProgramOp::SearchFilters(invert_collection_op(&program.search_filters, collection_op))
        }
        ProgramOp::StatusRecords(collection_op) => {
            ProgramOp::StatusRecords(invert_collection_op(&program.status_records, collection_op))
        }
        ProgramOp::Workshops(collection_op) => ProgramOp::Workshops(invert_collection_op(&program.workshops, collection_op)),
        ProgramOp::Surveys(collection_op) => ProgramOp::Surveys(invert_collection_op(&program.surveys, collection_op)),
        ProgramOp::Issues(collection_op) => ProgramOp::Issues(invert_collection_op(&program.issues, collection_op)),
        ProgramOp::AuditEvents(collection_op) => {
            ProgramOp::AuditEvents(invert_collection_op(&program.audit_events, collection_op))
        }
        ProgramOp::Templates(collection_op) => ProgramOp::Templates(invert_collection_op(&program.templates, collection_op)),
        ProgramOp::Knowledge(collection_op) => ProgramOp::Knowledge(invert_collection_op(&program.knowledge, collection_op)),
        ProgramOp::Benchmarks(collection_op) => {
            ProgramOp::Benchmarks(invert_collection_op(&program.benchmarks, collection_op))
        }
        ProgramOp::Assumptions(collection_op) => {
            ProgramOp::Assumptions(invert_collection_op(&program.assumptions, collection_op))
        }
        ProgramOp::Constraints(collection_op) => {
            ProgramOp::Constraints(invert_collection_op(&program.constraints, collection_op))
        }
        ProgramOp::ComplianceRecords(collection_op) => ProgramOp::ComplianceRecords(invert_collection_op(
            &program.compliance_records,
            collection_op,
        )),
        ProgramOp::Approvals(collection_op) => {
            ProgramOp::Approvals(invert_collection_op(&program.approvals, collection_op))
        }
        ProgramOp::Meetings(collection_op) => ProgramOp::Meetings(invert_collection_op(&program.meetings, collection_op)),
        ProgramOp::Traces(collection_op) => ProgramOp::Traces(invert_collection_op(&program.traces, collection_op)),
        ProgramOp::UpdateMeta { patch } => {
            let mut probe = program.meta.clone();
            let inverse = probe.apply_patch(patch);
            ProgramOp::UpdateMeta { patch: inverse }
        }
        ProgramOp::UpdateProject { patch } => {
            let mut probe = program.project.clone();
            let inverse = probe.apply_patch(patch);
            ProgramOp::UpdateProject { patch: inverse }
        }
        ProgramOp::UpdateGovernance { patch } => {
            let mut probe = program.governance.clone();
            let inverse = probe.apply_patch(patch);
            ProgramOp::UpdateGovernance { patch: inverse }
        }
        ProgramOp::SetAdjacency { adjacency } => {
            if let Some(existing) = program.adjacencies.iter().find(|row| row.header.id == adjacency.header.id) {
                ProgramOp::SetAdjacency {
                    adjacency: existing.clone(),
                }
            } else {
                ProgramOp::ClearAdjacency {
                    id: adjacency.header.id.clone(),
                }
            }
        }
        ProgramOp::ClearAdjacency { id } => match program.adjacencies.iter().find(|row| &row.header.id == id).cloned() {
            Some(existing) => ProgramOp::SetAdjacency { adjacency: existing },
            None => ProgramOp::ClearAdjacency { id: id.clone() },
        },
        ProgramOp::SetProgram { .. } => ProgramOp::SetProgram {
            program: program.clone(),
        },
    }
}
// #endregion

// #region 🔖ProgramDiff
/// @emoji 📦 Ordered list of program ops materializing a document diff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDiff {
    pub ops: Vec<ProgramOp>,
}

impl OperationDiff<Program> for ProgramDiff {
    fn apply(&self, projection: &Program) -> Program {
        let mut next = projection.clone();
        for op in &self.ops {
            apply_program_op(&mut next, op);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.ops.extend(other.ops);
    }
}

impl Operation<Program> for ProgramOp {
    type Diff = ProgramDiff;

    fn diff(&self, _projection: &Program) -> ProgramDiff {
        ProgramDiff { ops: vec![self.clone()] }
    }

    fn backwards(&self, projection: &Program) -> Vec<Self> {
        vec![invert_program_op(projection, self)]
    }
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::EntityHeader;
    use crate::program::{empty_program, sample_program};

    #[test]
    fn update_meta_round_trips_undo() {
        let mut program = empty_program();
        let op = ProgramOp::UpdateMeta {
            patch: ProgramMetaPatch {
                title: Some("Clinic".into()),
                ..Default::default()
            },
        };
        let inverse = invert_program_op(&program, &op);
        apply_program_op(&mut program, &op);
        assert_eq!(program.meta.title, "Clinic");
        apply_program_op(&mut program, &inverse);
        assert_ne!(program.meta.title, "Clinic");
    }

    #[test]
    fn add_stakeholder_via_collection_op() {
        let mut program = sample_program();
        let before = program.stakeholders.len();
        let stakeholder = Stakeholder {
            header: EntityHeader::new(EntityId::new_serial("stakeholder"), "Nurse Lead"),
            role: "Clinical".into(),
            organization: "Sample Health".into(),
            department: None,
            contact_email: None,
            contact_phone: None,
            influence: InfluenceLevel::Medium,
            interest: InfluenceLevel::High,
            engagement: EngagementLevel::Supportive,
            expectations: Vec::new(),
            concerns: Vec::new(),
            requirement_ids: Vec::new(),
            decision_authority: false,
            communication_preferences: Vec::new(),
            reporting_frequency: None,
            involvement_phases: Vec::new(),
            availability: None,
            representative_of: None,
            delegated_to: None,
            relationship_to_client: None,
            power_interest_notes: Vec::new(),
            stakeholder_type: "Clinical".into(),
            influence_strategy: None,
            communication_channels: Vec::new(),
            success_metrics: Vec::new(),
        };
        let id = stakeholder.header.id.clone();
        let op = ProgramOp::Stakeholders(CollectionOp::Add {
            index: program.stakeholders.len(),
            item: stakeholder,
        });
        apply_program_op(&mut program, &op);
        assert_eq!(program.stakeholders.len(), before + 1);
        let undo = invert_program_op(&program, &op);
        apply_program_op(&mut program, &undo);
        assert!(!program.stakeholders.iter().any(|s| s.header.id == id));
    }

    #[test]
    fn set_program_bulk_replace() {
        let mut program = empty_program();
        let sample = sample_program();
        apply_program_op(
            &mut program,
            &ProgramOp::SetProgram {
                program: sample.clone(),
            },
        );
        assert_eq!(program.elements.len(), sample.elements.len());
    }
}
