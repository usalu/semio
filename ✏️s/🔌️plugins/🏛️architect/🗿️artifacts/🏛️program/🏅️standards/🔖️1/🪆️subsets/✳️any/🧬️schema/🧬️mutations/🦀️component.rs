//! 🧬️ Architect program artifact — document mutation dispatch enum.
//!
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s ~65 generic per-collection
//! add/remove/patch registers plus the three document-level meta facets, per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`/`📓️derivation-rules.md`:
//! each header-shaped id-keyed register (rule 2) becomes `create`/`delete`/`rename`/`replace`;
//! the two edge-shaped registers (`adjacencies`, `traces` — rule 4) become `connect`/`disconnect`;
//! the three document-level scalar facets (`meta`/`project`/`governance` — rule 1) become
//! `rename`/`replace`. The old whole-document-replace variant is deleted outright (banned per
//! taxonomy — whole-document replace is not an in-history mutation; it goes through
//! `ArtifactStore::reset`).
//!
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<ProgramSnapshot>` and
//! `impl protocol::SemanticMutation<ProgramSnapshot>` for `ProgramMutation` by delegating each
//! variant to its payload's `protocol::MutationKind` impl — see the triad leaves
//! (`<slug>/{🦠️mutation,🔺️diff,↩️inverse}`) for the handcrafted logic. This file is dispatch-only;
//! the old hand-written `apply_program_mutation`/`inverse_program_mutation`/`impl Mutation` are
//! deleted, replaced by the derive.
//!
//! Physical directory layout: Wave C (this overhaul's directory-restructure pass) split the
//! wave-2 pass's 72 pre-migration noun-keyed triad directories (one dir hosting all 4 verbs of a
//! register, e.g. `👥stakeholders` hosting `CreateStakeholder`/`DeleteStakeholder`/
//! `RenameStakeholder`/`ReplaceStakeholder` together) into 266 one-triad-dir-per-variant
//! directories (e.g. `🌱👥create-stakeholder`, `🗑️👥delete-stakeholder`, …), each
//! `#[path]`-mounted individually in `📦️glue.rs`, satisfying the dispatch-coverage policy rule's
//! 1:1 variant-to-triad-dir comparison in both directions. The two orphan stub directories the
//! wave-2 pass could not remove (`🔀adjacencies`, `🖼️set-snapshot` — kept alive only because
//! `glue.rs` still wired them) are deleted along with their `glue.rs` mounts.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ProgramMutation
/// 🧩️ Every variant wraps exactly one `protocol::MutationKind<ProgramSnapshot, ProgramMutation>`
/// payload struct declared in the corresponding triad leaf's `🦠️mutation/🦀️component.rs`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = ProgramSnapshot, diff = ProgramDiff, schema = "s.architect.program")]
pub enum ProgramMutation {
    CreateInformationRequirement(super::create_information_requirement::CreateInformationRequirement),
    DeleteInformationRequirement(super::delete_information_requirement::DeleteInformationRequirement),
    RenameInformationRequirement(super::rename_information_requirement::RenameInformationRequirement),
    ReplaceInformationRequirement(super::replace_information_requirement::ReplaceInformationRequirement),
    CreateSustainabilityRequirement(super::create_sustainability_requirement::CreateSustainabilityRequirement),
    DeleteSustainabilityRequirement(super::delete_sustainability_requirement::DeleteSustainabilityRequirement),
    RenameSustainabilityRequirement(super::rename_sustainability_requirement::RenameSustainabilityRequirement),
    ReplaceSustainabilityRequirement(super::replace_sustainability_requirement::ReplaceSustainabilityRequirement),
    CreateAccessibilityRequirement(super::create_accessibility_requirement::CreateAccessibilityRequirement),
    DeleteAccessibilityRequirement(super::delete_accessibility_requirement::DeleteAccessibilityRequirement),
    RenameAccessibilityRequirement(super::rename_accessibility_requirement::RenameAccessibilityRequirement),
    ReplaceAccessibilityRequirement(super::replace_accessibility_requirement::ReplaceAccessibilityRequirement),
    CreateConflict(super::create_conflict::CreateConflict),
    DeleteConflict(super::delete_conflict::DeleteConflict),
    RenameConflict(super::rename_conflict::RenameConflict),
    ReplaceConflict(super::replace_conflict::ReplaceConflict),
    CreateOptionEvaluation(super::create_option_evaluation::CreateOptionEvaluation),
    DeleteOptionEvaluation(super::delete_option_evaluation::DeleteOptionEvaluation),
    RenameOptionEvaluation(super::rename_option_evaluation::RenameOptionEvaluation),
    ReplaceOptionEvaluation(super::replace_option_evaluation::ReplaceOptionEvaluation),
    CreateFunction(super::create_function::CreateFunction),
    DeleteFunction(super::delete_function::DeleteFunction),
    RenameFunction(super::rename_function::RenameFunction),
    ReplaceFunction(super::replace_function::ReplaceFunction),
    CreateRisk(super::create_risk::CreateRisk),
    DeleteRisk(super::delete_risk::DeleteRisk),
    RenameRisk(super::rename_risk::RenameRisk),
    ReplaceRisk(super::replace_risk::ReplaceRisk),
    CreateDecision(super::create_decision::CreateDecision),
    DeleteDecision(super::delete_decision::DeleteDecision),
    RenameDecision(super::rename_decision::RenameDecision),
    ReplaceDecision(super::replace_decision::ReplaceDecision),
    CreateValidationRecord(super::create_validation_record::CreateValidationRecord),
    DeleteValidationRecord(super::delete_validation_record::DeleteValidationRecord),
    RenameValidationRecord(super::rename_validation_record::RenameValidationRecord),
    ReplaceValidationRecord(super::replace_validation_record::ReplaceValidationRecord),
    CreatePriorityRecord(super::create_priority_record::CreatePriorityRecord),
    DeletePriorityRecord(super::delete_priority_record::DeletePriorityRecord),
    RenamePriorityRecord(super::rename_priority_record::RenamePriorityRecord),
    ReplacePriorityRecord(super::replace_priority_record::ReplacePriorityRecord),
    CreateFlowRequirement(super::create_flow_requirement::CreateFlowRequirement),
    DeleteFlowRequirement(super::delete_flow_requirement::DeleteFlowRequirement),
    RenameFlowRequirement(super::rename_flow_requirement::RenameFlowRequirement),
    ReplaceFlowRequirement(super::replace_flow_requirement::ReplaceFlowRequirement),
    CreateEnvironmentalRequirement(super::create_environmental_requirement::CreateEnvironmentalRequirement),
    DeleteEnvironmentalRequirement(super::delete_environmental_requirement::DeleteEnvironmentalRequirement),
    RenameEnvironmentalRequirement(super::rename_environmental_requirement::RenameEnvironmentalRequirement),
    ReplaceEnvironmentalRequirement(super::replace_environmental_requirement::ReplaceEnvironmentalRequirement),
    CreateWorkshop(super::create_workshop::CreateWorkshop),
    DeleteWorkshop(super::delete_workshop::DeleteWorkshop),
    RenameWorkshop(super::rename_workshop::RenameWorkshop),
    ReplaceWorkshop(super::replace_workshop::ReplaceWorkshop),
    CreateScenario(super::create_scenario::CreateScenario),
    DeleteScenario(super::delete_scenario::DeleteScenario),
    RenameScenario(super::rename_scenario::RenameScenario),
    ReplaceScenario(super::replace_scenario::ReplaceScenario),
    CreateBenchmarkRecord(super::create_benchmark_record::CreateBenchmarkRecord),
    DeleteBenchmarkRecord(super::delete_benchmark_record::DeleteBenchmarkRecord),
    RenameBenchmarkRecord(super::rename_benchmark_record::RenameBenchmarkRecord),
    ReplaceBenchmarkRecord(super::replace_benchmark_record::ReplaceBenchmarkRecord),
    CreateActivity(super::create_activity::CreateActivity),
    DeleteActivity(super::delete_activity::DeleteActivity),
    RenameActivity(super::rename_activity::RenameActivity),
    ReplaceActivity(super::replace_activity::ReplaceActivity),
    CreateInfrastructureRequirement(super::create_infrastructure_requirement::CreateInfrastructureRequirement),
    DeleteInfrastructureRequirement(super::delete_infrastructure_requirement::DeleteInfrastructureRequirement),
    RenameInfrastructureRequirement(super::rename_infrastructure_requirement::RenameInfrastructureRequirement),
    ReplaceInfrastructureRequirement(super::replace_infrastructure_requirement::ReplaceInfrastructureRequirement),
    CreateOrganizationalRequirement(super::create_organizational_requirement::CreateOrganizationalRequirement),
    DeleteOrganizationalRequirement(super::delete_organizational_requirement::DeleteOrganizationalRequirement),
    RenameOrganizationalRequirement(super::rename_organizational_requirement::RenameOrganizationalRequirement),
    ReplaceOrganizationalRequirement(super::replace_organizational_requirement::ReplaceOrganizationalRequirement),
    CreateIssue(super::create_issue::CreateIssue),
    DeleteIssue(super::delete_issue::DeleteIssue),
    RenameIssue(super::rename_issue::RenameIssue),
    ReplaceIssue(super::replace_issue::ReplaceIssue),
    CreateApprovalRecord(super::create_approval_record::CreateApprovalRecord),
    DeleteApprovalRecord(super::delete_approval_record::DeleteApprovalRecord),
    RenameApprovalRecord(super::rename_approval_record::RenameApprovalRecord),
    ReplaceApprovalRecord(super::replace_approval_record::ReplaceApprovalRecord),
    CreateStakeholder(super::create_stakeholder::CreateStakeholder),
    DeleteStakeholder(super::delete_stakeholder::DeleteStakeholder),
    RenameStakeholder(super::rename_stakeholder::RenameStakeholder),
    ReplaceStakeholder(super::replace_stakeholder::ReplaceStakeholder),
    CreateQualityRecord(super::create_quality_record::CreateQualityRecord),
    DeleteQualityRecord(super::delete_quality_record::DeleteQualityRecord),
    RenameQualityRecord(super::rename_quality_record::RenameQualityRecord),
    ReplaceQualityRecord(super::replace_quality_record::ReplaceQualityRecord),
    CreateResilienceRequirement(super::create_resilience_requirement::CreateResilienceRequirement),
    DeleteResilienceRequirement(super::delete_resilience_requirement::DeleteResilienceRequirement),
    RenameResilienceRequirement(super::rename_resilience_requirement::RenameResilienceRequirement),
    ReplaceResilienceRequirement(super::replace_resilience_requirement::ReplaceResilienceRequirement),
    CreateAssumption(super::create_assumption::CreateAssumption),
    DeleteAssumption(super::delete_assumption::DeleteAssumption),
    RenameAssumption(super::rename_assumption::RenameAssumption),
    ReplaceAssumption(super::replace_assumption::ReplaceAssumption),
    CreateCostRequirement(super::create_cost_requirement::CreateCostRequirement),
    DeleteCostRequirement(super::delete_cost_requirement::DeleteCostRequirement),
    RenameCostRequirement(super::rename_cost_requirement::RenameCostRequirement),
    ReplaceCostRequirement(super::replace_cost_requirement::ReplaceCostRequirement),
    CreateDocument(super::create_document::CreateDocument),
    DeleteDocument(super::delete_document::DeleteDocument),
    RenameDocument(super::rename_document::RenameDocument),
    ReplaceDocument(super::replace_document::ReplaceDocument),
    CreateScheduleRequirement(super::create_schedule_requirement::CreateScheduleRequirement),
    DeleteScheduleRequirement(super::delete_schedule_requirement::DeleteScheduleRequirement),
    RenameScheduleRequirement(super::rename_schedule_requirement::RenameScheduleRequirement),
    ReplaceScheduleRequirement(super::replace_schedule_requirement::ReplaceScheduleRequirement),
    CreateGrowthPlan(super::create_growth_plan::CreateGrowthPlan),
    DeleteGrowthPlan(super::delete_growth_plan::DeleteGrowthPlan),
    RenameGrowthPlan(super::rename_growth_plan::RenameGrowthPlan),
    ReplaceGrowthPlan(super::replace_growth_plan::ReplaceGrowthPlan),
    CreatePerformanceCriterion(super::create_performance_criterion::CreatePerformanceCriterion),
    DeletePerformanceCriterion(super::delete_performance_criterion::DeletePerformanceCriterion),
    RenamePerformanceCriterion(super::rename_performance_criterion::RenamePerformanceCriterion),
    ReplacePerformanceCriterion(super::replace_performance_criterion::ReplacePerformanceCriterion),
    CreateOperationalRequirement(super::create_operational_requirement::CreateOperationalRequirement),
    DeleteOperationalRequirement(super::delete_operational_requirement::DeleteOperationalRequirement),
    RenameOperationalRequirement(super::rename_operational_requirement::RenameOperationalRequirement),
    ReplaceOperationalRequirement(super::replace_operational_requirement::ReplaceOperationalRequirement),
    CreateRequirement(super::create_requirement::CreateRequirement),
    DeleteRequirement(super::delete_requirement::DeleteRequirement),
    RenameRequirement(super::rename_requirement::RenameRequirement),
    ReplaceRequirement(super::replace_requirement::ReplaceRequirement),
    CreateSiteContext(super::create_site_context::CreateSiteContext),
    DeleteSiteContext(super::delete_site_context::DeleteSiteContext),
    RenameSiteContext(super::rename_site_context::RenameSiteContext),
    ReplaceSiteContext(super::replace_site_context::ReplaceSiteContext),
    CreateTemplateRecord(super::create_template_record::CreateTemplateRecord),
    DeleteTemplateRecord(super::delete_template_record::DeleteTemplateRecord),
    RenameTemplateRecord(super::rename_template_record::RenameTemplateRecord),
    ReplaceTemplateRecord(super::replace_template_record::ReplaceTemplateRecord),
    CreateReportRecord(super::create_report_record::CreateReportRecord),
    DeleteReportRecord(super::delete_report_record::DeleteReportRecord),
    RenameReportRecord(super::rename_report_record::RenameReportRecord),
    ReplaceReportRecord(super::replace_report_record::ReplaceReportRecord),
    CreateAuditEvent(super::create_audit_event::CreateAuditEvent),
    DeleteAuditEvent(super::delete_audit_event::DeleteAuditEvent),
    RenameAuditEvent(super::rename_audit_event::RenameAuditEvent),
    ReplaceAuditEvent(super::replace_audit_event::ReplaceAuditEvent),
    CreateKnowledgeRecord(super::create_knowledge_record::CreateKnowledgeRecord),
    DeleteKnowledgeRecord(super::delete_knowledge_record::DeleteKnowledgeRecord),
    RenameKnowledgeRecord(super::rename_knowledge_record::RenameKnowledgeRecord),
    ReplaceKnowledgeRecord(super::replace_knowledge_record::ReplaceKnowledgeRecord),
    CreateRegulatoryRequirement(super::create_regulatory_requirement::CreateRegulatoryRequirement),
    DeleteRegulatoryRequirement(super::delete_regulatory_requirement::DeleteRegulatoryRequirement),
    RenameRegulatoryRequirement(super::rename_regulatory_requirement::RenameRegulatoryRequirement),
    ReplaceRegulatoryRequirement(super::replace_regulatory_requirement::ReplaceRegulatoryRequirement),
    CreateChangeRecord(super::create_change_record::CreateChangeRecord),
    DeleteChangeRecord(super::delete_change_record::DeleteChangeRecord),
    RenameChangeRecord(super::rename_change_record::RenameChangeRecord),
    ReplaceChangeRecord(super::replace_change_record::ReplaceChangeRecord),
    CreateCommunicationRequirement(super::create_communication_requirement::CreateCommunicationRequirement),
    DeleteCommunicationRequirement(super::delete_communication_requirement::DeleteCommunicationRequirement),
    RenameCommunicationRequirement(super::rename_communication_requirement::RenameCommunicationRequirement),
    ReplaceCommunicationRequirement(super::replace_communication_requirement::ReplaceCommunicationRequirement),
    CreateResource(super::create_resource::CreateResource),
    DeleteResource(super::delete_resource::DeleteResource),
    RenameResource(super::rename_resource::RenameResource),
    ReplaceResource(super::replace_resource::ReplaceResource),
    CreateStatusRecord(super::create_status_record::CreateStatusRecord),
    DeleteStatusRecord(super::delete_status_record::DeleteStatusRecord),
    RenameStatusRecord(super::rename_status_record::RenameStatusRecord),
    ReplaceStatusRecord(super::replace_status_record::ReplaceStatusRecord),
    CreateProcess(super::create_process::CreateProcess),
    DeleteProcess(super::delete_process::DeleteProcess),
    RenameProcess(super::rename_process::RenameProcess),
    ReplaceProcess(super::replace_process::ReplaceProcess),
    CreateSearchFilter(super::create_search_filter::CreateSearchFilter),
    DeleteSearchFilter(super::delete_search_filter::DeleteSearchFilter),
    RenameSearchFilter(super::rename_search_filter::RenameSearchFilter),
    ReplaceSearchFilter(super::replace_search_filter::ReplaceSearchFilter),
    CreateAccessRule(super::create_access_rule::CreateAccessRule),
    DeleteAccessRule(super::delete_access_rule::DeleteAccessRule),
    RenameAccessRule(super::rename_access_rule::RenameAccessRule),
    ReplaceAccessRule(super::replace_access_rule::ReplaceAccessRule),
    CreatePrivacyRequirement(super::create_privacy_requirement::CreatePrivacyRequirement),
    DeletePrivacyRequirement(super::delete_privacy_requirement::DeletePrivacyRequirement),
    RenamePrivacyRequirement(super::rename_privacy_requirement::RenamePrivacyRequirement),
    ReplacePrivacyRequirement(super::replace_privacy_requirement::ReplacePrivacyRequirement),
    CreateRelationship(super::create_relationship::CreateRelationship),
    DeleteRelationship(super::delete_relationship::DeleteRelationship),
    RenameRelationship(super::rename_relationship::RenameRelationship),
    ReplaceRelationship(super::replace_relationship::ReplaceRelationship),
    CreateQuantityRequirement(super::create_quantity_requirement::CreateQuantityRequirement),
    DeleteQuantityRequirement(super::delete_quantity_requirement::DeleteQuantityRequirement),
    RenameQuantityRequirement(super::rename_quantity_requirement::RenameQuantityRequirement),
    ReplaceQuantityRequirement(super::replace_quantity_requirement::ReplaceQuantityRequirement),
    CreateAnalysisRecord(super::create_analysis_record::CreateAnalysisRecord),
    DeleteAnalysisRecord(super::delete_analysis_record::DeleteAnalysisRecord),
    RenameAnalysisRecord(super::rename_analysis_record::RenameAnalysisRecord),
    ReplaceAnalysisRecord(super::replace_analysis_record::ReplaceAnalysisRecord),
    CreateStorageRequirement(super::create_storage_requirement::CreateStorageRequirement),
    DeleteStorageRequirement(super::delete_storage_requirement::DeleteStorageRequirement),
    RenameStorageRequirement(super::rename_storage_requirement::RenameStorageRequirement),
    ReplaceStorageRequirement(super::replace_storage_requirement::ReplaceStorageRequirement),
    CreateMeetingRecord(super::create_meeting_record::CreateMeetingRecord),
    DeleteMeetingRecord(super::delete_meeting_record::DeleteMeetingRecord),
    RenameMeetingRecord(super::rename_meeting_record::RenameMeetingRecord),
    ReplaceMeetingRecord(super::replace_meeting_record::ReplaceMeetingRecord),
    CreateSurvey(super::create_survey::CreateSurvey),
    DeleteSurvey(super::delete_survey::DeleteSurvey),
    RenameSurvey(super::rename_survey::RenameSurvey),
    ReplaceSurvey(super::replace_survey::ReplaceSurvey),
    CreateDeliveryConstraint(super::create_delivery_constraint::CreateDeliveryConstraint),
    DeleteDeliveryConstraint(super::delete_delivery_constraint::DeleteDeliveryConstraint),
    RenameDeliveryConstraint(super::rename_delivery_constraint::RenameDeliveryConstraint),
    ReplaceDeliveryConstraint(super::replace_delivery_constraint::ReplaceDeliveryConstraint),
    CreateConstraintRecord(super::create_constraint_record::CreateConstraintRecord),
    DeleteConstraintRecord(super::delete_constraint_record::DeleteConstraintRecord),
    RenameConstraintRecord(super::rename_constraint_record::RenameConstraintRecord),
    ReplaceConstraintRecord(super::replace_constraint_record::ReplaceConstraintRecord),
    CreateComplianceRecord(super::create_compliance_record::CreateComplianceRecord),
    DeleteComplianceRecord(super::delete_compliance_record::DeleteComplianceRecord),
    RenameComplianceRecord(super::rename_compliance_record::RenameComplianceRecord),
    ReplaceComplianceRecord(super::replace_compliance_record::ReplaceComplianceRecord),
    CreateServiceRequirement(super::create_service_requirement::CreateServiceRequirement),
    DeleteServiceRequirement(super::delete_service_requirement::DeleteServiceRequirement),
    RenameServiceRequirement(super::rename_service_requirement::RenameServiceRequirement),
    ReplaceServiceRequirement(super::replace_service_requirement::ReplaceServiceRequirement),
    CreateEquipment(super::create_equipment::CreateEquipment),
    DeleteEquipment(super::delete_equipment::DeleteEquipment),
    RenameEquipment(super::rename_equipment::RenameEquipment),
    ReplaceEquipment(super::replace_equipment::ReplaceEquipment),
    CreateSecurityRequirement(super::create_security_requirement::CreateSecurityRequirement),
    DeleteSecurityRequirement(super::delete_security_requirement::DeleteSecurityRequirement),
    RenameSecurityRequirement(super::rename_security_requirement::RenameSecurityRequirement),
    ReplaceSecurityRequirement(super::replace_security_requirement::ReplaceSecurityRequirement),
    CreateCollaborationRecord(super::create_collaboration_record::CreateCollaborationRecord),
    DeleteCollaborationRecord(super::delete_collaboration_record::DeleteCollaborationRecord),
    RenameCollaborationRecord(super::rename_collaboration_record::RenameCollaborationRecord),
    ReplaceCollaborationRecord(super::replace_collaboration_record::ReplaceCollaborationRecord),
    CreateSafetyRequirement(super::create_safety_requirement::CreateSafetyRequirement),
    DeleteSafetyRequirement(super::delete_safety_requirement::DeleteSafetyRequirement),
    RenameSafetyRequirement(super::rename_safety_requirement::RenameSafetyRequirement),
    ReplaceSafetyRequirement(super::replace_safety_requirement::ReplaceSafetyRequirement),
    CreateUserProfile(super::create_user_profile::CreateUserProfile),
    DeleteUserProfile(super::delete_user_profile::DeleteUserProfile),
    RenameUserProfile(super::rename_user_profile::RenameUserProfile),
    ReplaceUserProfile(super::replace_user_profile::ReplaceUserProfile),
    CreateHumanFactorRequirement(super::create_human_factor_requirement::CreateHumanFactorRequirement),
    DeleteHumanFactorRequirement(super::delete_human_factor_requirement::DeleteHumanFactorRequirement),
    RenameHumanFactorRequirement(super::rename_human_factor_requirement::RenameHumanFactorRequirement),
    ReplaceHumanFactorRequirement(super::replace_human_factor_requirement::ReplaceHumanFactorRequirement),
    CreateFlexibilityRequirement(super::create_flexibility_requirement::CreateFlexibilityRequirement),
    DeleteFlexibilityRequirement(super::delete_flexibility_requirement::DeleteFlexibilityRequirement),
    RenameFlexibilityRequirement(super::rename_flexibility_requirement::RenameFlexibilityRequirement),
    ReplaceFlexibilityRequirement(super::replace_flexibility_requirement::ReplaceFlexibilityRequirement),
    CreateWayfindingRequirement(super::create_wayfinding_requirement::CreateWayfindingRequirement),
    DeleteWayfindingRequirement(super::delete_wayfinding_requirement::DeleteWayfindingRequirement),
    RenameWayfindingRequirement(super::rename_wayfinding_requirement::RenameWayfindingRequirement),
    ReplaceWayfindingRequirement(super::replace_wayfinding_requirement::ReplaceWayfindingRequirement),
    CreateProgramElement(super::create_program_element::CreateProgramElement),
    DeleteProgramElement(super::delete_program_element::DeleteProgramElement),
    RenameProgramElement(super::rename_program_element::RenameProgramElement),
    ReplaceProgramElement(super::replace_program_element::ReplaceProgramElement),
    ConnectAdjacency(super::connect_adjacency::ConnectAdjacency),
    DisconnectAdjacency(super::disconnect_adjacency::DisconnectAdjacency),
    ConnectTrace(super::connect_trace::ConnectTrace),
    DisconnectTrace(super::disconnect_trace::DisconnectTrace),
    RenameMeta(super::rename_meta::RenameMeta),
    ReplaceMeta(super::replace_meta::ReplaceMeta),
    RenameProject(super::rename_project::RenameProject),
    ReplaceProject(super::replace_project::ReplaceProject),
    RenameGovernance(super::rename_governance::RenameGovernance),
    ReplaceGovernance(super::replace_governance::ReplaceGovernance),
}
//#endregion 🔖️ProgramMutation

//#region 🧫️FixtureTests
/// 🧫️ Handcrafted per-mutation fixture cases (contract D1, ticket
/// `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`) — one `🧪️tests/<case>/🦀️component.rs` per mutation
/// leaf, mounted here rather than in `📦️glue.rs` so this tree owns its own test wiring.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🔗🧲connect-adjacency/🧪️tests/connects-reception-to-waiting/🦀️component.rs"]
    mod tests_connect_adjacency;
    #[path = "🔗🧵connect-trace/🧪️tests/connects-requirement-a-to-decision-a/🦀️component.rs"]
    mod tests_connect_trace;
    #[path = "🌱🔑create-access-rule/🧪️tests/creates-access-rule-a/🦀️component.rs"]
    mod tests_create_access_rule;
    #[path = "🌱♿create-accessibility-requirement/🧪️tests/creates-accessibility-requirement-a/🦀️component.rs"]
    mod tests_create_accessibility_requirement;
    #[path = "🌱🏃create-activity/🧪️tests/creates-activity-a/🦀️component.rs"]
    mod tests_create_activity;
    #[path = "🌱🔬create-analysis-record/🧪️tests/creates-analysis-record-a/🦀️component.rs"]
    mod tests_create_analysis_record;
    #[path = "🌱👍create-approval-record/🧪️tests/creates-approval-record-a/🦀️component.rs"]
    mod tests_create_approval_record;
    #[path = "🌱💭create-assumption/🧪️tests/creates-assumption-a/🦀️component.rs"]
    mod tests_create_assumption;
    #[path = "🌱📒create-audit-event/🧪️tests/creates-audit-event-a/🦀️component.rs"]
    mod tests_create_audit_event;
    #[path = "🌱🏁create-benchmark-record/🧪️tests/creates-benchmark-record-a/🦀️component.rs"]
    mod tests_create_benchmark_record;
    #[path = "🌱📝create-change-record/🧪️tests/creates-change-record-a/🦀️component.rs"]
    mod tests_create_change_record;
    #[path = "🌱🤝create-collaboration-record/🧪️tests/creates-collaboration-record-a/🦀️component.rs"]
    mod tests_create_collaboration_record;
    #[path = "🌱📡create-communication-requirement/🧪️tests/creates-communication-requirement-a/🦀️component.rs"]
    mod tests_create_communication_requirement;
    #[path = "🌱🛂create-compliance-record/🧪️tests/creates-compliance-record-a/🦀️component.rs"]
    mod tests_create_compliance_record;
    #[path = "🌱⚔️create-conflict/🧪️tests/creates-conflict-a/🦀️component.rs"]
    mod tests_create_conflict;
    #[path = "🌱🚧create-constraint-record/🧪️tests/creates-constraint-record-a/🦀️component.rs"]
    mod tests_create_constraint_record;
    #[path = "🌱💰create-cost-requirement/🧪️tests/creates-cost-requirement-a/🦀️component.rs"]
    mod tests_create_cost_requirement;
    #[path = "🌱✅create-decision/🧪️tests/creates-decision-a/🦀️component.rs"]
    mod tests_create_decision;
    #[path = "🌱🚚create-delivery-constraint/🧪️tests/creates-delivery-constraint-a/🦀️component.rs"]
    mod tests_create_delivery_constraint;
    #[path = "🌱📄create-document/🧪️tests/creates-document-a/🦀️component.rs"]
    mod tests_create_document;
    #[path = "🌱🌿create-environmental-requirement/🧪️tests/creates-environmental-requirement-a/🦀️component.rs"]
    mod tests_create_environmental_requirement;
    #[path = "🌱🛠️create-equipment/🧪️tests/creates-equipment-a/🦀️component.rs"]
    mod tests_create_equipment;
    #[path = "🌱🧩create-flexibility-requirement/🧪️tests/creates-flexibility-requirement-a/🦀️component.rs"]
    mod tests_create_flexibility_requirement;
    #[path = "🌱🌊create-flow-requirement/🧪️tests/creates-flow-requirement-a/🦀️component.rs"]
    mod tests_create_flow_requirement;
    #[path = "🌱⚙️create-function/🧪️tests/creates-function-a/🦀️component.rs"]
    mod tests_create_function;
    #[path = "🌱📈create-growth-plan/🧪️tests/creates-growth-plan-a/🦀️component.rs"]
    mod tests_create_growth_plan;
    #[path = "🌱🧠create-human-factor-requirement/🧪️tests/creates-human-factor-requirement-a/🦀️component.rs"]
    mod tests_create_human_factor_requirement;
    #[path = "🌱ℹ️create-information-requirement/🧪️tests/creates-information-requirement-a/🦀️component.rs"]
    mod tests_create_information_requirement;
    #[path = "🌱🏗️create-infrastructure-requirement/🧪️tests/creates-infrastructure-requirement-a/🦀️component.rs"]
    mod tests_create_infrastructure_requirement;
    #[path = "🌱🐛create-issue/🧪️tests/creates-issue-a/🦀️component.rs"]
    mod tests_create_issue;
    #[path = "🌱📚create-knowledge-record/🧪️tests/creates-knowledge-record-a/🦀️component.rs"]
    mod tests_create_knowledge_record;
    #[path = "🌱🗓️create-meeting-record/🧪️tests/creates-meeting-record-a/🦀️component.rs"]
    mod tests_create_meeting_record;
    #[path = "🌱📋create-operational-requirement/🧪️tests/creates-operational-requirement-a/🦀️component.rs"]
    mod tests_create_operational_requirement;
    #[path = "🌱⚖️create-option-evaluation/🧪️tests/creates-option-evaluation-a/🦀️component.rs"]
    mod tests_create_option_evaluation;
    #[path = "🌱🏢create-organizational-requirement/🧪️tests/creates-organizational-requirement-a/🦀️component.rs"]
    mod tests_create_organizational_requirement;
    #[path = "🌱📊create-performance-criterion/🧪️tests/creates-performance-criterion-a/🦀️component.rs"]
    mod tests_create_performance_criterion;
    #[path = "🌱⭐create-priority-record/🧪️tests/creates-priority-record-a/🦀️component.rs"]
    mod tests_create_priority_record;
    #[path = "🌱🔒create-privacy-requirement/🧪️tests/creates-privacy-requirement-a/🦀️component.rs"]
    mod tests_create_privacy_requirement;
    #[path = "🌱🔄create-process/🧪️tests/creates-process-a/🦀️component.rs"]
    mod tests_create_process;
    #[path = "🌱🧱create-program-element/🧪️tests/creates-program-element-a/🦀️component.rs"]
    mod tests_create_program_element;
    #[path = "🌱💎create-quality-record/🧪️tests/creates-quality-record-a/🦀️component.rs"]
    mod tests_create_quality_record;
    #[path = "🌱🔢create-quantity-requirement/🧪️tests/creates-quantity-requirement-a/🦀️component.rs"]
    mod tests_create_quantity_requirement;
    #[path = "🌱📜create-regulatory-requirement/🧪️tests/creates-regulatory-requirement-a/🦀️component.rs"]
    mod tests_create_regulatory_requirement;
    #[path = "🌱🔗create-relationship/🧪️tests/creates-relationship-a/🦀️component.rs"]
    mod tests_create_relationship;
    #[path = "🌱📑create-report-record/🧪️tests/creates-report-record-a/🦀️component.rs"]
    mod tests_create_report_record;
    #[path = "🌱📌create-requirement/🧪️tests/creates-requirement-a/🦀️component.rs"]
    mod tests_create_requirement;
    #[path = "🌱💪create-resilience-requirement/🧪️tests/creates-resilience-requirement-a/🦀️component.rs"]
    mod tests_create_resilience_requirement;
    #[path = "🌱📦create-resource/🧪️tests/creates-resource-a/🦀️component.rs"]
    mod tests_create_resource;
    #[path = "🌱⚠️create-risk/🧪️tests/creates-risk-a/🦀️component.rs"]
    mod tests_create_risk;
    #[path = "🌱🦺create-safety-requirement/🧪️tests/creates-safety-requirement-a/🦀️component.rs"]
    mod tests_create_safety_requirement;
    #[path = "🌱🎬create-scenario/🧪️tests/creates-scenario-a/🦀️component.rs"]
    mod tests_create_scenario;
    #[path = "🌱📅create-schedule-requirement/🧪️tests/creates-schedule-requirement-a/🦀️component.rs"]
    mod tests_create_schedule_requirement;
    #[path = "🌱🔍create-search-filter/🧪️tests/creates-search-filter-a/🦀️component.rs"]
    mod tests_create_search_filter;
    #[path = "🌱🛡️create-security-requirement/🧪️tests/creates-security-requirement-a/🦀️component.rs"]
    mod tests_create_security_requirement;
    #[path = "🌱🛎️create-service-requirement/🧪️tests/creates-service-requirement-a/🦀️component.rs"]
    mod tests_create_service_requirement;
    #[path = "🌱📍create-site-context/🧪️tests/creates-site-context-a/🦀️component.rs"]
    mod tests_create_site_context;
    #[path = "🌱👥create-stakeholder/🧪️tests/creates-stakeholder-a/🦀️component.rs"]
    mod tests_create_stakeholder;
    #[path = "🌱📶create-status-record/🧪️tests/creates-status-record-a/🦀️component.rs"]
    mod tests_create_status_record;
    #[path = "🌱🗄️create-storage-requirement/🧪️tests/creates-storage-requirement-a/🦀️component.rs"]
    mod tests_create_storage_requirement;
    #[path = "🌱🗳️create-survey/🧪️tests/creates-survey-a/🦀️component.rs"]
    mod tests_create_survey;
    #[path = "🌱♻️create-sustainability-requirement/🧪️tests/creates-sustainability-requirement-a/🦀️component.rs"]
    mod tests_create_sustainability_requirement;
    #[path = "🌱📐create-template-record/🧪️tests/creates-template-record-a/🦀️component.rs"]
    mod tests_create_template_record;
    #[path = "🌱🧑create-user-profile/🧪️tests/creates-user-profile-a/🦀️component.rs"]
    mod tests_create_user_profile;
    #[path = "🌱✔️create-validation-record/🧪️tests/creates-validation-record-a/🦀️component.rs"]
    mod tests_create_validation_record;
    #[path = "🌱🧭create-wayfinding-requirement/🧪️tests/creates-wayfinding-requirement-a/🦀️component.rs"]
    mod tests_create_wayfinding_requirement;
    #[path = "🌱🎓create-workshop/🧪️tests/creates-workshop-a/🦀️component.rs"]
    mod tests_create_workshop;
    #[path = "🗑️🔑delete-access-rule/🧪️tests/deletes-access-rule-a/🦀️component.rs"]
    mod tests_delete_access_rule;
    #[path = "🗑️♿delete-accessibility-requirement/🧪️tests/deletes-accessibility-requirement-a/🦀️component.rs"]
    mod tests_delete_accessibility_requirement;
    #[path = "🗑️🏃delete-activity/🧪️tests/deletes-activity-a/🦀️component.rs"]
    mod tests_delete_activity;
    #[path = "🗑️🔬delete-analysis-record/🧪️tests/deletes-analysis-record-a/🦀️component.rs"]
    mod tests_delete_analysis_record;
    #[path = "🗑️👍delete-approval-record/🧪️tests/deletes-approval-record-a/🦀️component.rs"]
    mod tests_delete_approval_record;
    #[path = "🗑️💭delete-assumption/🧪️tests/deletes-assumption-a/🦀️component.rs"]
    mod tests_delete_assumption;
    #[path = "🗑️📒delete-audit-event/🧪️tests/deletes-audit-event-a/🦀️component.rs"]
    mod tests_delete_audit_event;
    #[path = "🗑️🏁delete-benchmark-record/🧪️tests/rejects-deleting-absent-benchmark-record-a/🦀️component.rs"]
    mod tests_delete_benchmark_record;
    #[path = "🗑️📝delete-change-record/🧪️tests/deletes-change-record-a/🦀️component.rs"]
    mod tests_delete_change_record;
    #[path = "🗑️🤝delete-collaboration-record/🧪️tests/deletes-collaboration-record-a/🦀️component.rs"]
    mod tests_delete_collaboration_record;
    #[path = "🗑️📡delete-communication-requirement/🧪️tests/deletes-communication-requirement-a/🦀️component.rs"]
    mod tests_delete_communication_requirement;
    #[path = "🗑️🛂delete-compliance-record/🧪️tests/deletes-compliance-record-a/🦀️component.rs"]
    mod tests_delete_compliance_record;
    #[path = "🗑️⚔️delete-conflict/🧪️tests/deletes-conflict-a/🦀️component.rs"]
    mod tests_delete_conflict;
    #[path = "🗑️🚧delete-constraint-record/🧪️tests/deletes-constraint-record-a/🦀️component.rs"]
    mod tests_delete_constraint_record;
    #[path = "🗑️💰delete-cost-requirement/🧪️tests/deletes-cost-requirement-a/🦀️component.rs"]
    mod tests_delete_cost_requirement;
    #[path = "🗑️✅delete-decision/🧪️tests/deletes-decision-a/🦀️component.rs"]
    mod tests_delete_decision;
    #[path = "🗑️🚚delete-delivery-constraint/🧪️tests/deletes-delivery-constraint-a/🦀️component.rs"]
    mod tests_delete_delivery_constraint;
    #[path = "🗑️📄delete-document/🧪️tests/deletes-document-a/🦀️component.rs"]
    mod tests_delete_document;
    #[path = "🗑️🌿delete-environmental-requirement/🧪️tests/deletes-environmental-requirement-a/🦀️component.rs"]
    mod tests_delete_environmental_requirement;
    #[path = "🗑️🛠️delete-equipment/🧪️tests/deletes-equipment-a/🦀️component.rs"]
    mod tests_delete_equipment;
    #[path = "🗑️🧩delete-flexibility-requirement/🧪️tests/deletes-flexibility-requirement-a/🦀️component.rs"]
    mod tests_delete_flexibility_requirement;
    #[path = "🗑️🌊delete-flow-requirement/🧪️tests/deletes-flow-requirement-a/🦀️component.rs"]
    mod tests_delete_flow_requirement;
    #[path = "🗑️⚙️delete-function/🧪️tests/deletes-function-a/🦀️component.rs"]
    mod tests_delete_function;
    #[path = "🗑️📈delete-growth-plan/🧪️tests/deletes-growth-plan-a/🦀️component.rs"]
    mod tests_delete_growth_plan;
    #[path = "🗑️🧠delete-human-factor-requirement/🧪️tests/deletes-human-factor-requirement-a/🦀️component.rs"]
    mod tests_delete_human_factor_requirement;
    #[path = "🗑️ℹ️delete-information-requirement/🧪️tests/deletes-information-requirement-a/🦀️component.rs"]
    mod tests_delete_information_requirement;
    #[path = "🗑️🏗️delete-infrastructure-requirement/🧪️tests/deletes-infrastructure-requirement-a/🦀️component.rs"]
    mod tests_delete_infrastructure_requirement;
    #[path = "🗑️🐛delete-issue/🧪️tests/deletes-issue-a/🦀️component.rs"]
    mod tests_delete_issue;
    #[path = "🗑️📚delete-knowledge-record/🧪️tests/rejects-deleting-absent-knowledge-record-a/🦀️component.rs"]
    mod tests_delete_knowledge_record;
    #[path = "🗑️🗓️delete-meeting-record/🧪️tests/deletes-meeting-record-a/🦀️component.rs"]
    mod tests_delete_meeting_record;
    #[path = "🗑️📋delete-operational-requirement/🧪️tests/deletes-operational-requirement-a/🦀️component.rs"]
    mod tests_delete_operational_requirement;
    #[path = "🗑️⚖️delete-option-evaluation/🧪️tests/deletes-option-evaluation-a/🦀️component.rs"]
    mod tests_delete_option_evaluation;
    #[path = "🗑️🏢delete-organizational-requirement/🧪️tests/deletes-organizational-requirement-a/🦀️component.rs"]
    mod tests_delete_organizational_requirement;
    #[path = "🗑️📊delete-performance-criterion/🧪️tests/deletes-performance-criterion-a/🦀️component.rs"]
    mod tests_delete_performance_criterion;
    #[path = "🗑️⭐delete-priority-record/🧪️tests/deletes-priority-record-a/🦀️component.rs"]
    mod tests_delete_priority_record;
    #[path = "🗑️🔒delete-privacy-requirement/🧪️tests/deletes-privacy-requirement-a/🦀️component.rs"]
    mod tests_delete_privacy_requirement;
    #[path = "🗑️🔄delete-process/🧪️tests/deletes-process-a/🦀️component.rs"]
    mod tests_delete_process;
    #[path = "🗑️🧱delete-program-element/🧪️tests/deletes-program-element-a/🦀️component.rs"]
    mod tests_delete_program_element;
    #[path = "🗑️💎delete-quality-record/🧪️tests/deletes-quality-record-a/🦀️component.rs"]
    mod tests_delete_quality_record;
    #[path = "🗑️🔢delete-quantity-requirement/🧪️tests/deletes-quantity-requirement-a/🦀️component.rs"]
    mod tests_delete_quantity_requirement;
    #[path = "🗑️📜delete-regulatory-requirement/🧪️tests/deletes-regulatory-requirement-a/🦀️component.rs"]
    mod tests_delete_regulatory_requirement;
    #[path = "🗑️🔗delete-relationship/🧪️tests/deletes-relationship-a/🦀️component.rs"]
    mod tests_delete_relationship;
    #[path = "🗑️📑delete-report-record/🧪️tests/deletes-report-record-a/🦀️component.rs"]
    mod tests_delete_report_record;
    #[path = "🗑️📌delete-requirement/🧪️tests/deletes-requirement-a/🦀️component.rs"]
    mod tests_delete_requirement;
    #[path = "🗑️💪delete-resilience-requirement/🧪️tests/deletes-resilience-requirement-a/🦀️component.rs"]
    mod tests_delete_resilience_requirement;
    #[path = "🗑️📦delete-resource/🧪️tests/deletes-resource-a/🦀️component.rs"]
    mod tests_delete_resource;
    #[path = "🗑️⚠️delete-risk/🧪️tests/deletes-risk-a/🦀️component.rs"]
    mod tests_delete_risk;
    #[path = "🗑️🦺delete-safety-requirement/🧪️tests/deletes-safety-requirement-a/🦀️component.rs"]
    mod tests_delete_safety_requirement;
    #[path = "🗑️🎬delete-scenario/🧪️tests/deletes-scenario-a/🦀️component.rs"]
    mod tests_delete_scenario;
    #[path = "🗑️📅delete-schedule-requirement/🧪️tests/deletes-schedule-requirement-a/🦀️component.rs"]
    mod tests_delete_schedule_requirement;
    #[path = "🗑️🔍delete-search-filter/🧪️tests/deletes-search-filter-a/🦀️component.rs"]
    mod tests_delete_search_filter;
    #[path = "🗑️🛡️delete-security-requirement/🧪️tests/deletes-security-requirement-a/🦀️component.rs"]
    mod tests_delete_security_requirement;
    #[path = "🗑️🛎️delete-service-requirement/🧪️tests/deletes-service-requirement-a/🦀️component.rs"]
    mod tests_delete_service_requirement;
    #[path = "🗑️📍delete-site-context/🧪️tests/deletes-site-context-a/🦀️component.rs"]
    mod tests_delete_site_context;
    #[path = "🗑️👥delete-stakeholder/🧪️tests/deletes-stakeholder-a/🦀️component.rs"]
    mod tests_delete_stakeholder;
    #[path = "🗑️📶delete-status-record/🧪️tests/deletes-status-record-a/🦀️component.rs"]
    mod tests_delete_status_record;
    #[path = "🗑️🗄️delete-storage-requirement/🧪️tests/deletes-storage-requirement-a/🦀️component.rs"]
    mod tests_delete_storage_requirement;
    #[path = "🗑️🗳️delete-survey/🧪️tests/deletes-survey-a/🦀️component.rs"]
    mod tests_delete_survey;
    #[path = "🗑️♻️delete-sustainability-requirement/🧪️tests/deletes-sustainability-requirement-a/🦀️component.rs"]
    mod tests_delete_sustainability_requirement;
    #[path = "🗑️📐delete-template-record/🧪️tests/deletes-template-record-a/🦀️component.rs"]
    mod tests_delete_template_record;
    #[path = "🗑️🧑delete-user-profile/🧪️tests/deletes-user-profile-a/🦀️component.rs"]
    mod tests_delete_user_profile;
    #[path = "🗑️✔️delete-validation-record/🧪️tests/deletes-validation-record-a/🦀️component.rs"]
    mod tests_delete_validation_record;
    #[path = "🗑️🧭delete-wayfinding-requirement/🧪️tests/deletes-wayfinding-requirement-a/🦀️component.rs"]
    mod tests_delete_wayfinding_requirement;
    #[path = "🗑️🎓delete-workshop/🧪️tests/deletes-workshop-a/🦀️component.rs"]
    mod tests_delete_workshop;
    #[path = "✂️🧲disconnect-adjacency/🧪️tests/disconnects-reception-from-waiting/🦀️component.rs"]
    mod tests_disconnect_adjacency;
    #[path = "✂️🧵disconnect-trace/🧪️tests/disconnects-requirement-a-from-decision-a/🦀️component.rs"]
    mod tests_disconnect_trace;
    #[path = "✏️🔑rename-access-rule/🧪️tests/renames-access-rule-a/🦀️component.rs"]
    mod tests_rename_access_rule;
    #[path = "✏️♿rename-accessibility-requirement/🧪️tests/renames-accessibility-requirement-a/🦀️component.rs"]
    mod tests_rename_accessibility_requirement;
    #[path = "✏️🏃rename-activity/🧪️tests/renames-activity-a/🦀️component.rs"]
    mod tests_rename_activity;
    #[path = "✏️🔬rename-analysis-record/🧪️tests/renames-analysis-record-a/🦀️component.rs"]
    mod tests_rename_analysis_record;
    #[path = "✏️👍rename-approval-record/🧪️tests/renames-approval-record-a/🦀️component.rs"]
    mod tests_rename_approval_record;
    #[path = "✏️💭rename-assumption/🧪️tests/renames-assumption-a/🦀️component.rs"]
    mod tests_rename_assumption;
    #[path = "✏️📒rename-audit-event/🧪️tests/renames-audit-event-a/🦀️component.rs"]
    mod tests_rename_audit_event;
    #[path = "✏️🏁rename-benchmark-record/🧪️tests/rejects-renaming-absent-benchmark-record-a/🦀️component.rs"]
    mod tests_rename_benchmark_record;
    #[path = "✏️📝rename-change-record/🧪️tests/renames-change-record-a/🦀️component.rs"]
    mod tests_rename_change_record;
    #[path = "✏️🤝rename-collaboration-record/🧪️tests/renames-collaboration-record-a/🦀️component.rs"]
    mod tests_rename_collaboration_record;
    #[path = "✏️📡rename-communication-requirement/🧪️tests/renames-communication-requirement-a/🦀️component.rs"]
    mod tests_rename_communication_requirement;
    #[path = "✏️🛂rename-compliance-record/🧪️tests/renames-compliance-record-a/🦀️component.rs"]
    mod tests_rename_compliance_record;
    #[path = "✏️⚔️rename-conflict/🧪️tests/renames-conflict-a/🦀️component.rs"]
    mod tests_rename_conflict;
    #[path = "✏️🚧rename-constraint-record/🧪️tests/renames-constraint-record-a/🦀️component.rs"]
    mod tests_rename_constraint_record;
    #[path = "✏️💰rename-cost-requirement/🧪️tests/renames-cost-requirement-a/🦀️component.rs"]
    mod tests_rename_cost_requirement;
    #[path = "✏️✅rename-decision/🧪️tests/renames-decision-a/🦀️component.rs"]
    mod tests_rename_decision;
    #[path = "✏️🚚rename-delivery-constraint/🧪️tests/renames-delivery-constraint-a/🦀️component.rs"]
    mod tests_rename_delivery_constraint;
    #[path = "✏️📄rename-document/🧪️tests/renames-document-a/🦀️component.rs"]
    mod tests_rename_document;
    #[path = "✏️🌿rename-environmental-requirement/🧪️tests/renames-environmental-requirement-a/🦀️component.rs"]
    mod tests_rename_environmental_requirement;
    #[path = "✏️🛠️rename-equipment/🧪️tests/renames-equipment-a/🦀️component.rs"]
    mod tests_rename_equipment;
    #[path = "✏️🧩rename-flexibility-requirement/🧪️tests/renames-flexibility-requirement-a/🦀️component.rs"]
    mod tests_rename_flexibility_requirement;
    #[path = "✏️🌊rename-flow-requirement/🧪️tests/renames-flow-requirement-a/🦀️component.rs"]
    mod tests_rename_flow_requirement;
    #[path = "✏️⚙️rename-function/🧪️tests/renames-function-a/🦀️component.rs"]
    mod tests_rename_function;
    #[path = "✏️🏛️rename-governance/🧪️tests/renames-the-governance-framework/🦀️component.rs"]
    mod tests_rename_governance;
    #[path = "✏️📈rename-growth-plan/🧪️tests/renames-growth-plan-a/🦀️component.rs"]
    mod tests_rename_growth_plan;
    #[path = "✏️🧠rename-human-factor-requirement/🧪️tests/renames-human-factor-requirement-a/🦀️component.rs"]
    mod tests_rename_human_factor_requirement;
    #[path = "✏️ℹ️rename-information-requirement/🧪️tests/renames-information-requirement-a/🦀️component.rs"]
    mod tests_rename_information_requirement;
    #[path = "✏️🏗️rename-infrastructure-requirement/🧪️tests/renames-infrastructure-requirement-a/🦀️component.rs"]
    mod tests_rename_infrastructure_requirement;
    #[path = "✏️🐛rename-issue/🧪️tests/renames-issue-a/🦀️component.rs"]
    mod tests_rename_issue;
    #[path = "✏️📚rename-knowledge-record/🧪️tests/rejects-renaming-absent-knowledge-record-a/🦀️component.rs"]
    mod tests_rename_knowledge_record;
    #[path = "✏️🗓️rename-meeting-record/🧪️tests/renames-meeting-record-a/🦀️component.rs"]
    mod tests_rename_meeting_record;
    #[path = "✏️🏷️rename-meta/🧪️tests/renames-the-document-title/🦀️component.rs"]
    mod tests_rename_meta;
    #[path = "✏️📋rename-operational-requirement/🧪️tests/renames-operational-requirement-a/🦀️component.rs"]
    mod tests_rename_operational_requirement;
    #[path = "✏️⚖️rename-option-evaluation/🧪️tests/renames-option-evaluation-a/🦀️component.rs"]
    mod tests_rename_option_evaluation;
    #[path = "✏️🏢rename-organizational-requirement/🧪️tests/renames-organizational-requirement-a/🦀️component.rs"]
    mod tests_rename_organizational_requirement;
    #[path = "✏️📊rename-performance-criterion/🧪️tests/renames-performance-criterion-a/🦀️component.rs"]
    mod tests_rename_performance_criterion;
    #[path = "✏️⭐rename-priority-record/🧪️tests/renames-priority-record-a/🦀️component.rs"]
    mod tests_rename_priority_record;
    #[path = "✏️🔒rename-privacy-requirement/🧪️tests/renames-privacy-requirement-a/🦀️component.rs"]
    mod tests_rename_privacy_requirement;
    #[path = "✏️🔄rename-process/🧪️tests/renames-process-a/🦀️component.rs"]
    mod tests_rename_process;
    #[path = "✏️🧱rename-program-element/🧪️tests/renames-program-element-a/🦀️component.rs"]
    mod tests_rename_program_element;
    #[path = "✏️📁rename-project/🧪️tests/renames-the-project-code/🦀️component.rs"]
    mod tests_rename_project;
    #[path = "✏️💎rename-quality-record/🧪️tests/renames-quality-record-a/🦀️component.rs"]
    mod tests_rename_quality_record;
    #[path = "✏️🔢rename-quantity-requirement/🧪️tests/renames-quantity-requirement-a/🦀️component.rs"]
    mod tests_rename_quantity_requirement;
    #[path = "✏️📜rename-regulatory-requirement/🧪️tests/renames-regulatory-requirement-a/🦀️component.rs"]
    mod tests_rename_regulatory_requirement;
    #[path = "✏️🔗rename-relationship/🧪️tests/renames-relationship-a/🦀️component.rs"]
    mod tests_rename_relationship;
    #[path = "✏️📑rename-report-record/🧪️tests/renames-report-record-a/🦀️component.rs"]
    mod tests_rename_report_record;
    #[path = "✏️📌rename-requirement/🧪️tests/renames-requirement-a/🦀️component.rs"]
    mod tests_rename_requirement;
    #[path = "✏️💪rename-resilience-requirement/🧪️tests/renames-resilience-requirement-a/🦀️component.rs"]
    mod tests_rename_resilience_requirement;
    #[path = "✏️📦rename-resource/🧪️tests/renames-resource-a/🦀️component.rs"]
    mod tests_rename_resource;
    #[path = "✏️⚠️rename-risk/🧪️tests/renames-risk-a/🦀️component.rs"]
    mod tests_rename_risk;
    #[path = "✏️🦺rename-safety-requirement/🧪️tests/renames-safety-requirement-a/🦀️component.rs"]
    mod tests_rename_safety_requirement;
    #[path = "✏️🎬rename-scenario/🧪️tests/renames-scenario-a/🦀️component.rs"]
    mod tests_rename_scenario;
    #[path = "✏️📅rename-schedule-requirement/🧪️tests/renames-schedule-requirement-a/🦀️component.rs"]
    mod tests_rename_schedule_requirement;
    #[path = "✏️🔍rename-search-filter/🧪️tests/renames-search-filter-a/🦀️component.rs"]
    mod tests_rename_search_filter;
    #[path = "✏️🛡️rename-security-requirement/🧪️tests/renames-security-requirement-a/🦀️component.rs"]
    mod tests_rename_security_requirement;
    #[path = "✏️🛎️rename-service-requirement/🧪️tests/renames-service-requirement-a/🦀️component.rs"]
    mod tests_rename_service_requirement;
    #[path = "✏️📍rename-site-context/🧪️tests/renames-site-context-a/🦀️component.rs"]
    mod tests_rename_site_context;
    #[path = "✏️👥rename-stakeholder/🧪️tests/renames-stakeholder-a/🦀️component.rs"]
    mod tests_rename_stakeholder;
    #[path = "✏️📶rename-status-record/🧪️tests/renames-status-record-a/🦀️component.rs"]
    mod tests_rename_status_record;
    #[path = "✏️🗄️rename-storage-requirement/🧪️tests/renames-storage-requirement-a/🦀️component.rs"]
    mod tests_rename_storage_requirement;
    #[path = "✏️🗳️rename-survey/🧪️tests/renames-survey-a/🦀️component.rs"]
    mod tests_rename_survey;
    #[path = "✏️♻️rename-sustainability-requirement/🧪️tests/renames-sustainability-requirement-a/🦀️component.rs"]
    mod tests_rename_sustainability_requirement;
    #[path = "✏️📐rename-template-record/🧪️tests/renames-template-record-a/🦀️component.rs"]
    mod tests_rename_template_record;
    #[path = "✏️🧑rename-user-profile/🧪️tests/renames-user-profile-a/🦀️component.rs"]
    mod tests_rename_user_profile;
    #[path = "✏️✔️rename-validation-record/🧪️tests/renames-validation-record-a/🦀️component.rs"]
    mod tests_rename_validation_record;
    #[path = "✏️🧭rename-wayfinding-requirement/🧪️tests/renames-wayfinding-requirement-a/🦀️component.rs"]
    mod tests_rename_wayfinding_requirement;
    #[path = "✏️🎓rename-workshop/🧪️tests/renames-workshop-a/🦀️component.rs"]
    mod tests_rename_workshop;
    #[path = "🔁🔑replace-access-rule/🧪️tests/replaces-access-rule-a/🦀️component.rs"]
    mod tests_replace_access_rule;
    #[path = "🔁♿replace-accessibility-requirement/🧪️tests/replaces-accessibility-requirement-a/🦀️component.rs"]
    mod tests_replace_accessibility_requirement;
    #[path = "🔁🏃replace-activity/🧪️tests/replaces-activity-a/🦀️component.rs"]
    mod tests_replace_activity;
    #[path = "🔁🔬replace-analysis-record/🧪️tests/replaces-analysis-record-a/🦀️component.rs"]
    mod tests_replace_analysis_record;
    #[path = "🔁👍replace-approval-record/🧪️tests/replaces-approval-record-a/🦀️component.rs"]
    mod tests_replace_approval_record;
    #[path = "🔁💭replace-assumption/🧪️tests/replaces-assumption-a/🦀️component.rs"]
    mod tests_replace_assumption;
    #[path = "🔁📒replace-audit-event/🧪️tests/replaces-audit-event-a/🦀️component.rs"]
    mod tests_replace_audit_event;
    #[path = "🔁🏁replace-benchmark-record/🧪️tests/rejects-replacing-absent-benchmark-record-a/🦀️component.rs"]
    mod tests_replace_benchmark_record;
    #[path = "🔁📝replace-change-record/🧪️tests/replaces-change-record-a/🦀️component.rs"]
    mod tests_replace_change_record;
    #[path = "🔁🤝replace-collaboration-record/🧪️tests/replaces-collaboration-record-a/🦀️component.rs"]
    mod tests_replace_collaboration_record;
    #[path = "🔁📡replace-communication-requirement/🧪️tests/replaces-communication-requirement-a/🦀️component.rs"]
    mod tests_replace_communication_requirement;
    #[path = "🔁🛂replace-compliance-record/🧪️tests/replaces-compliance-record-a/🦀️component.rs"]
    mod tests_replace_compliance_record;
    #[path = "🔁⚔️replace-conflict/🧪️tests/replaces-conflict-a/🦀️component.rs"]
    mod tests_replace_conflict;
    #[path = "🔁🚧replace-constraint-record/🧪️tests/replaces-constraint-record-a/🦀️component.rs"]
    mod tests_replace_constraint_record;
    #[path = "🔁💰replace-cost-requirement/🧪️tests/replaces-cost-requirement-a/🦀️component.rs"]
    mod tests_replace_cost_requirement;
    #[path = "🔁✅replace-decision/🧪️tests/replaces-decision-a/🦀️component.rs"]
    mod tests_replace_decision;
    #[path = "🔁🚚replace-delivery-constraint/🧪️tests/replaces-delivery-constraint-a/🦀️component.rs"]
    mod tests_replace_delivery_constraint;
    #[path = "🔁📄replace-document/🧪️tests/replaces-document-a/🦀️component.rs"]
    mod tests_replace_document;
    #[path = "🔁🌿replace-environmental-requirement/🧪️tests/replaces-environmental-requirement-a/🦀️component.rs"]
    mod tests_replace_environmental_requirement;
    #[path = "🔁🛠️replace-equipment/🧪️tests/replaces-equipment-a/🦀️component.rs"]
    mod tests_replace_equipment;
    #[path = "🔁🧩replace-flexibility-requirement/🧪️tests/replaces-flexibility-requirement-a/🦀️component.rs"]
    mod tests_replace_flexibility_requirement;
    #[path = "🔁🌊replace-flow-requirement/🧪️tests/replaces-flow-requirement-a/🦀️component.rs"]
    mod tests_replace_flow_requirement;
    #[path = "🔁⚙️replace-function/🧪️tests/replaces-function-a/🦀️component.rs"]
    mod tests_replace_function;
    #[path = "🔁🏛️replace-governance/🧪️tests/replaces-the-governance-block/🦀️component.rs"]
    mod tests_replace_governance;
    #[path = "🔁📈replace-growth-plan/🧪️tests/replaces-growth-plan-a/🦀️component.rs"]
    mod tests_replace_growth_plan;
    #[path = "🔁🧠replace-human-factor-requirement/🧪️tests/replaces-human-factor-requirement-a/🦀️component.rs"]
    mod tests_replace_human_factor_requirement;
    #[path = "🔁ℹ️replace-information-requirement/🧪️tests/replaces-information-requirement-a/🦀️component.rs"]
    mod tests_replace_information_requirement;
    #[path = "🔁🏗️replace-infrastructure-requirement/🧪️tests/replaces-infrastructure-requirement-a/🦀️component.rs"]
    mod tests_replace_infrastructure_requirement;
    #[path = "🔁🐛replace-issue/🧪️tests/replaces-issue-a/🦀️component.rs"]
    mod tests_replace_issue;
    #[path = "🔁📚replace-knowledge-record/🧪️tests/rejects-replacing-absent-knowledge-record-a/🦀️component.rs"]
    mod tests_replace_knowledge_record;
    #[path = "🔁🗓️replace-meeting-record/🧪️tests/replaces-meeting-record-a/🦀️component.rs"]
    mod tests_replace_meeting_record;
    #[path = "🔁🏷️replace-meta/🧪️tests/replaces-the-document-meta-block/🦀️component.rs"]
    mod tests_replace_meta;
    #[path = "🔁📋replace-operational-requirement/🧪️tests/replaces-operational-requirement-a/🦀️component.rs"]
    mod tests_replace_operational_requirement;
    #[path = "🔁⚖️replace-option-evaluation/🧪️tests/replaces-option-evaluation-a/🦀️component.rs"]
    mod tests_replace_option_evaluation;
    #[path = "🔁🏢replace-organizational-requirement/🧪️tests/replaces-organizational-requirement-a/🦀️component.rs"]
    mod tests_replace_organizational_requirement;
    #[path = "🔁📊replace-performance-criterion/🧪️tests/replaces-performance-criterion-a/🦀️component.rs"]
    mod tests_replace_performance_criterion;
    #[path = "🔁⭐replace-priority-record/🧪️tests/replaces-priority-record-a/🦀️component.rs"]
    mod tests_replace_priority_record;
    #[path = "🔁🔒replace-privacy-requirement/🧪️tests/replaces-privacy-requirement-a/🦀️component.rs"]
    mod tests_replace_privacy_requirement;
    #[path = "🔁🔄replace-process/🧪️tests/replaces-process-a/🦀️component.rs"]
    mod tests_replace_process;
    #[path = "🔁🧱replace-program-element/🧪️tests/replaces-program-element-a/🦀️component.rs"]
    mod tests_replace_program_element;
    #[path = "🔁📁replace-project/🧪️tests/replaces-the-project-definition/🦀️component.rs"]
    mod tests_replace_project;
    #[path = "🔁💎replace-quality-record/🧪️tests/replaces-quality-record-a/🦀️component.rs"]
    mod tests_replace_quality_record;
    #[path = "🔁🔢replace-quantity-requirement/🧪️tests/replaces-quantity-requirement-a/🦀️component.rs"]
    mod tests_replace_quantity_requirement;
    #[path = "🔁📜replace-regulatory-requirement/🧪️tests/replaces-regulatory-requirement-a/🦀️component.rs"]
    mod tests_replace_regulatory_requirement;
    #[path = "🔁🔗replace-relationship/🧪️tests/replaces-relationship-a/🦀️component.rs"]
    mod tests_replace_relationship;
    #[path = "🔁📑replace-report-record/🧪️tests/replaces-report-record-a/🦀️component.rs"]
    mod tests_replace_report_record;
    #[path = "🔁📌replace-requirement/🧪️tests/replaces-requirement-a/🦀️component.rs"]
    mod tests_replace_requirement;
    #[path = "🔁💪replace-resilience-requirement/🧪️tests/replaces-resilience-requirement-a/🦀️component.rs"]
    mod tests_replace_resilience_requirement;
    #[path = "🔁📦replace-resource/🧪️tests/replaces-resource-a/🦀️component.rs"]
    mod tests_replace_resource;
    #[path = "🔁⚠️replace-risk/🧪️tests/replaces-risk-a/🦀️component.rs"]
    mod tests_replace_risk;
    #[path = "🔁🦺replace-safety-requirement/🧪️tests/replaces-safety-requirement-a/🦀️component.rs"]
    mod tests_replace_safety_requirement;
    #[path = "🔁🎬replace-scenario/🧪️tests/replaces-scenario-a/🦀️component.rs"]
    mod tests_replace_scenario;
    #[path = "🔁📅replace-schedule-requirement/🧪️tests/replaces-schedule-requirement-a/🦀️component.rs"]
    mod tests_replace_schedule_requirement;
    #[path = "🔁🔍replace-search-filter/🧪️tests/replaces-search-filter-a/🦀️component.rs"]
    mod tests_replace_search_filter;
    #[path = "🔁🛡️replace-security-requirement/🧪️tests/replaces-security-requirement-a/🦀️component.rs"]
    mod tests_replace_security_requirement;
    #[path = "🔁🛎️replace-service-requirement/🧪️tests/replaces-service-requirement-a/🦀️component.rs"]
    mod tests_replace_service_requirement;
    #[path = "🔁📍replace-site-context/🧪️tests/replaces-site-context-a/🦀️component.rs"]
    mod tests_replace_site_context;
    #[path = "🔁👥replace-stakeholder/🧪️tests/replaces-stakeholder-a/🦀️component.rs"]
    mod tests_replace_stakeholder;
    #[path = "🔁📶replace-status-record/🧪️tests/replaces-status-record-a/🦀️component.rs"]
    mod tests_replace_status_record;
    #[path = "🔁🗄️replace-storage-requirement/🧪️tests/replaces-storage-requirement-a/🦀️component.rs"]
    mod tests_replace_storage_requirement;
    #[path = "🔁🗳️replace-survey/🧪️tests/replaces-survey-a/🦀️component.rs"]
    mod tests_replace_survey;
    #[path = "🔁♻️replace-sustainability-requirement/🧪️tests/replaces-sustainability-requirement-a/🦀️component.rs"]
    mod tests_replace_sustainability_requirement;
    #[path = "🔁📐replace-template-record/🧪️tests/replaces-template-record-a/🦀️component.rs"]
    mod tests_replace_template_record;
    #[path = "🔁🧑replace-user-profile/🧪️tests/replaces-user-profile-a/🦀️component.rs"]
    mod tests_replace_user_profile;
    #[path = "🔁✔️replace-validation-record/🧪️tests/replaces-validation-record-a/🦀️component.rs"]
    mod tests_replace_validation_record;
    #[path = "🔁🧭replace-wayfinding-requirement/🧪️tests/replaces-wayfinding-requirement-a/🦀️component.rs"]
    mod tests_replace_wayfinding_requirement;
    #[path = "🔁🎓replace-workshop/🧪️tests/replaces-workshop-a/🦀️component.rs"]
    mod tests_replace_workshop;
}
//#endregion 🧫️FixtureTests

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::*;
    use crate::artifacts::program::registers::*;
    use crate::artifacts::program::{empty_plugin, sample_plugin};
    use protocol::{Mutation, MutationDiff, OpText, SemanticMutation};

    async fn round_trip(snapshot: &ProgramSnapshot, operation: &ProgramMutation) -> ProgramSnapshot {
        let forward = operation.diff(snapshot).diff().apply(snapshot).expect("valid mutation diff");
        let mut backward = operation.inverse(snapshot);
        backward.reverse();
        let mut restored = forward.clone();
        for undo in &backward {
            restored = undo.diff(&restored).diff().apply(&restored).expect("valid mutation diff");
        }
        assert_eq!(&restored, snapshot, "inverse (reversed) must exactly restore the pre-operation fixture");
        forward
    }

    //#region 👥stakeholders
    #[semio_framework_async_macros::async_test]
    async fn stakeholders_create_rename_replace_delete_round_trip() {
        let snapshot = sample_plugin();
        let new_id = EntityId::new_serial("stakeholder", "stakeholder");
        let mut new_stakeholder = snapshot.stakeholders[0].clone();
        new_stakeholder.header.id = new_id.clone();
        new_stakeholder.header.name = "New Stakeholder".into();

        let create = ProgramMutation::CreateStakeholder(super::super::create_stakeholder::CreateStakeholder { stakeholder: new_stakeholder });
        let with_new = round_trip(&snapshot, &create);
        assert_eq!(with_new.stakeholders.len(), snapshot.stakeholders.len() + 1);

        let rename = ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::RenameStakeholder { id: new_id.clone(), new_name: "Renamed".into() });
        let renamed = round_trip(&with_new, &rename);
        assert_eq!(renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().header.name, "Renamed");

        let mut replacement = renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().clone();
        replacement.role = "Sponsor".into();
        let replace = ProgramMutation::ReplaceStakeholder(super::super::replace_stakeholder::ReplaceStakeholder { stakeholder: replacement });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().role, "Sponsor");

        let delete = ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::DeleteStakeholder { id: new_id });
        let deleted = round_trip(&replaced, &delete);
        assert_eq!(deleted.stakeholders.len(), snapshot.stakeholders.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_stakeholder_of_a_missing_id_has_an_empty_inverse() {
        let snapshot = sample_plugin();
        let delete = ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::DeleteStakeholder { id: EntityId("nope".into()) });
        assert!(delete.inverse(&snapshot).is_empty(), "deleting an absent id has nothing to undo");
    }
    //#endregion 👥stakeholders

    //#region 🧱elements
    #[semio_framework_async_macros::async_test]
    async fn elements_create_rename_replace_delete_round_trip() {
        let snapshot = sample_plugin();
        let new_id = EntityId::new_serial("element", "element");
        let mut new_element = snapshot.elements[0].clone();
        new_element.header.id = new_id.clone();
        new_element.header.name = "Storage".into();

        let create = ProgramMutation::CreateProgramElement(super::super::create_program_element::CreateProgramElement { program_element: new_element });
        let with_new = round_trip(&snapshot, &create);
        assert_eq!(with_new.elements.len(), snapshot.elements.len() + 1);

        let rename = ProgramMutation::RenameProgramElement(super::super::rename_program_element::RenameProgramElement { id: new_id.clone(), new_name: "Storage Room".into() });
        let renamed = round_trip(&with_new, &rename);
        assert_eq!(renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().header.name, "Storage Room");

        let mut replacement = renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().clone();
        replacement.code = "STO".into();
        let replace = ProgramMutation::ReplaceProgramElement(super::super::replace_program_element::ReplaceProgramElement { program_element: replacement });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.elements.iter().find(|e| e.header.id == new_id).unwrap().code, "STO");

        let delete = ProgramMutation::DeleteProgramElement(super::super::delete_program_element::DeleteProgramElement { id: new_id });
        let deleted = round_trip(&replaced, &delete);
        assert_eq!(deleted.elements.len(), snapshot.elements.len());
    }
    //#endregion 🧱elements

    //#region 🏷️📁🏛️meta-project-governance
    #[semio_framework_async_macros::async_test]
    async fn update_meta_rename_and_replace_round_trip() {
        let snapshot = empty_plugin();
        let rename = ProgramMutation::RenameMeta(super::super::rename_meta::RenameMeta { new_title: "Clinic".into() });
        let renamed = round_trip(&snapshot, &rename);
        assert_eq!(renamed.meta.title, "Clinic");

        let mut new_meta = renamed.meta.clone();
        new_meta.industry_sector = "healthcare".into();
        let replace = ProgramMutation::ReplaceMeta(super::super::replace_meta::ReplaceMeta { new_meta });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.meta.industry_sector, "healthcare");
    }

    #[semio_framework_async_macros::async_test]
    async fn update_project_rename_and_replace_round_trip() {
        let snapshot = empty_plugin();
        let rename = ProgramMutation::RenameProject(super::super::rename_project::RenameProject { new_code: "CLN-001".into() });
        let renamed = round_trip(&snapshot, &rename);
        assert_eq!(renamed.project.code, "CLN-001");

        let mut new_project = renamed.project.clone();
        new_project.client_name = "Sample Health".into();
        let replace = ProgramMutation::ReplaceProject(super::super::replace_project::ReplaceProject { new_project });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.project.client_name, "Sample Health");
    }

    #[semio_framework_async_macros::async_test]
    async fn update_governance_rename_and_replace_round_trip() {
        let snapshot = empty_plugin();
        let rename = ProgramMutation::RenameGovernance(super::super::rename_governance::RenameGovernance { new_framework: "ISO 41001".into() });
        let renamed = round_trip(&snapshot, &rename);
        assert_eq!(renamed.governance.framework, "ISO 41001");

        let mut new_governance = renamed.governance.clone();
        new_governance.risk_appetite = Some("Low".into());
        let replace = ProgramMutation::ReplaceGovernance(super::super::replace_governance::ReplaceGovernance { new_governance });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.governance.risk_appetite, Some("Low".into()));
    }
    //#endregion 🏷️📁🏛️meta-project-governance

    //#region 🗺️🧹connect-disconnect-adjacency
    #[semio_framework_async_macros::async_test]
    async fn connect_and_disconnect_adjacency_round_trip() {
        let snapshot = sample_plugin();
        let a = snapshot.elements[0].header.id.clone();
        let b = snapshot.elements[1].header.id.clone();
        let new_adjacency = Adjacency {
            header: EntityHeader::new(EntityId::new_serial("adjacency", "adjacency"), "New Adjacency"),
            element_a_id: a,
            element_b_id: b,
            kind: AdjacencyKind::Preferred,
            connection: ConnectionKind::Direct,
            separations: Vec::new(),
            weight: 1.0,
            rationale: None,
            distance_max_m: None,
            distance_min_m: None,
            level_constraint: None,
            access_path: None,
            shared_wall: false,
            shared_entry: false,
            traffic_isolation: false,
            circulation_overlap: false,
            conflict_ids: Vec::new(),
            normalized: false,
            verification_status: ValidationStatus::Pending,
            source_relationship_id: None,
            internal_external_access: None,
        };
        let new_id = new_adjacency.header.id.clone();
        let connect = ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::ConnectAdjacency { adjacency: new_adjacency });
        let connected = round_trip(&snapshot, &connect);
        assert_eq!(connected.adjacencies.len(), snapshot.adjacencies.len() + 1);

        let disconnect = ProgramMutation::DisconnectAdjacency(super::super::disconnect_adjacency::DisconnectAdjacency { id: new_id });
        let disconnected = round_trip(&connected, &disconnect);
        assert_eq!(disconnected.adjacencies.len(), snapshot.adjacencies.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_adjacency_upserts_an_existing_pair_by_endpoint_identity() {
        let snapshot = sample_plugin();
        let existing = &snapshot.adjacencies[0];
        let mut updated = existing.clone();
        updated.header.id = EntityId::new_serial("adjacency", "adjacency");
        updated.weight = 5.0;
        let connect = ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::ConnectAdjacency { adjacency: updated });
        let connected = round_trip(&snapshot, &connect);
        assert_eq!(connected.adjacencies.len(), snapshot.adjacencies.len(), "same-pair connect patches in place, it does not add a row");
        assert_eq!(connected.adjacencies[0].weight, 5.0);
        assert_eq!(connected.adjacencies[0].header.id, existing.header.id, "the pre-existing edge keeps its own id");
    }
    //#endregion 🗺️🧹connect-disconnect-adjacency

    //#region 🧵connect-disconnect-trace
    #[semio_framework_async_macros::async_test]
    async fn connect_and_disconnect_trace_round_trip() {
        let snapshot = sample_plugin();
        let trace = TraceLink::new(snapshot.elements[0].header.id.clone(), snapshot.elements[1].header.id.clone(), TraceKind::FullAuditTrail);
        let id = trace.id.clone();
        let connect = ProgramMutation::ConnectTrace(super::super::connect_trace::ConnectTrace { trace });
        let connected = round_trip(&snapshot, &connect);
        assert_eq!(connected.traces.len(), 1);

        let disconnect = ProgramMutation::DisconnectTrace(super::super::disconnect_trace::DisconnectTrace { id });
        let disconnected = round_trip(&connected, &disconnect);
        assert!(disconnected.traces.is_empty());
    }
    //#endregion 🧵connect-disconnect-trace

    //#region 🗣️OpText
    #[semio_framework_async_macros::async_test]
    async fn program_mutation_op_text_round_trips_a_sample_of_variants() {
        let stakeholder = sample_plugin().stakeholders[0].clone();
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::CreateStakeholder(super::super::create_stakeholder::CreateStakeholder { stakeholder: stakeholder.clone() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::DeleteStakeholder { id: stakeholder.header.id.clone() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::RenameStakeholder { id: stakeholder.header.id.clone(), new_name: "X".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::ReplaceStakeholder(super::super::replace_stakeholder::ReplaceStakeholder { stakeholder }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameMeta(super::super::rename_meta::RenameMeta { new_title: "Clinic".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DisconnectAdjacency(super::super::disconnect_adjacency::DisconnectAdjacency { id: EntityId("a1".into()) }));
    }
    //#endregion 🗣️OpText

    //#region ⚖️SemanticLaws
    /// ⚖️ `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law`
    /// (`protocol::os_spr::testkit`, added by the Wave 0 mechanism pass) against the three most
    /// structurally distinct new kinds: an id-keyed collection create/delete pair, a document-level
    /// scalar facet rename, and an edge upsert.
    #[semio_framework_async_macros::async_test]
    async fn create_stakeholder_obeys_the_inverse_and_absorb_laws() {
        let base = sample_plugin();
        let mut new_stakeholder = base.stakeholders[0].clone();
        new_stakeholder.header.id = EntityId::new_serial("stakeholder", "stakeholder");
        let create = ProgramMutation::CreateStakeholder(super::super::create_stakeholder::CreateStakeholder { stakeholder: new_stakeholder.clone() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &create);
        let d1 = create.diff(&base).into_parts().0;
        let after = d1.apply(&base).expect("valid mutation diff");
        let d2 = ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::RenameStakeholder { id: new_stakeholder.header.id, new_name: "Renamed".into() }).diff(&after).into_parts().0;
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_meta_obeys_the_inverse_law() {
        let base = sample_plugin();
        let rename = ProgramMutation::RenameMeta(super::super::rename_meta::RenameMeta { new_title: "Renamed Program".into() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &rename);
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_adjacency_obeys_the_inverse_law() {
        let base = sample_plugin();
        let mut updated = base.adjacencies[0].clone();
        updated.weight = 9.0;
        let connect = ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::ConnectAdjacency { adjacency: updated });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &connect);
    }
    //#endregion ⚖️SemanticLaws

    //#region 📋️DescriptorLaws
    #[semio_framework_async_macros::async_test]
    async fn semantic_kinds_cover_every_variant() {
        assert_eq!(ProgramMutation::kinds().len(), 266);
        let stakeholder = sample_plugin().stakeholders[0].clone();
        let mutation = ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::RenameStakeholder { id: stakeholder.header.id, new_name: "X".into() });
        assert_eq!(mutation.semantics().kind, "rename-stakeholder");
        assert_eq!(mutation.semantics().record, "RenamedStakeholder");
    }
    //#endregion 📋️DescriptorLaws
}
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every [`ProgramMutation`] variant, in declaration order — the vocabulary the
/// `program-1-any` mutation catalog (`../../🧪️oracle/🔣️.json`) declares and the
/// exhaustive `mutate-*` case measures itself against (66 registers × create/delete/rename/replace, minus the four the two edge-shaped registers replace with connect/disconnect). The framework never
/// parses Rust, so `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest
/// against both the enum and the committed catalog.
pub const KINDS: &[&str] = &[
    "create-information-requirement",
    "delete-information-requirement",
    "rename-information-requirement",
    "replace-information-requirement",
    "create-sustainability-requirement",
    "delete-sustainability-requirement",
    "rename-sustainability-requirement",
    "replace-sustainability-requirement",
    "create-accessibility-requirement",
    "delete-accessibility-requirement",
    "rename-accessibility-requirement",
    "replace-accessibility-requirement",
    "create-conflict",
    "delete-conflict",
    "rename-conflict",
    "replace-conflict",
    "create-option-evaluation",
    "delete-option-evaluation",
    "rename-option-evaluation",
    "replace-option-evaluation",
    "create-function",
    "delete-function",
    "rename-function",
    "replace-function",
    "create-risk",
    "delete-risk",
    "rename-risk",
    "replace-risk",
    "create-decision",
    "delete-decision",
    "rename-decision",
    "replace-decision",
    "create-validation-record",
    "delete-validation-record",
    "rename-validation-record",
    "replace-validation-record",
    "create-priority-record",
    "delete-priority-record",
    "rename-priority-record",
    "replace-priority-record",
    "create-flow-requirement",
    "delete-flow-requirement",
    "rename-flow-requirement",
    "replace-flow-requirement",
    "create-environmental-requirement",
    "delete-environmental-requirement",
    "rename-environmental-requirement",
    "replace-environmental-requirement",
    "create-workshop",
    "delete-workshop",
    "rename-workshop",
    "replace-workshop",
    "create-scenario",
    "delete-scenario",
    "rename-scenario",
    "replace-scenario",
    "create-benchmark-record",
    "delete-benchmark-record",
    "rename-benchmark-record",
    "replace-benchmark-record",
    "create-activity",
    "delete-activity",
    "rename-activity",
    "replace-activity",
    "create-infrastructure-requirement",
    "delete-infrastructure-requirement",
    "rename-infrastructure-requirement",
    "replace-infrastructure-requirement",
    "create-organizational-requirement",
    "delete-organizational-requirement",
    "rename-organizational-requirement",
    "replace-organizational-requirement",
    "create-issue",
    "delete-issue",
    "rename-issue",
    "replace-issue",
    "create-approval-record",
    "delete-approval-record",
    "rename-approval-record",
    "replace-approval-record",
    "create-stakeholder",
    "delete-stakeholder",
    "rename-stakeholder",
    "replace-stakeholder",
    "create-quality-record",
    "delete-quality-record",
    "rename-quality-record",
    "replace-quality-record",
    "create-resilience-requirement",
    "delete-resilience-requirement",
    "rename-resilience-requirement",
    "replace-resilience-requirement",
    "create-assumption",
    "delete-assumption",
    "rename-assumption",
    "replace-assumption",
    "create-cost-requirement",
    "delete-cost-requirement",
    "rename-cost-requirement",
    "replace-cost-requirement",
    "create-document",
    "delete-document",
    "rename-document",
    "replace-document",
    "create-schedule-requirement",
    "delete-schedule-requirement",
    "rename-schedule-requirement",
    "replace-schedule-requirement",
    "create-growth-plan",
    "delete-growth-plan",
    "rename-growth-plan",
    "replace-growth-plan",
    "create-performance-criterion",
    "delete-performance-criterion",
    "rename-performance-criterion",
    "replace-performance-criterion",
    "create-operational-requirement",
    "delete-operational-requirement",
    "rename-operational-requirement",
    "replace-operational-requirement",
    "create-requirement",
    "delete-requirement",
    "rename-requirement",
    "replace-requirement",
    "create-site-context",
    "delete-site-context",
    "rename-site-context",
    "replace-site-context",
    "create-template-record",
    "delete-template-record",
    "rename-template-record",
    "replace-template-record",
    "create-report-record",
    "delete-report-record",
    "rename-report-record",
    "replace-report-record",
    "create-audit-event",
    "delete-audit-event",
    "rename-audit-event",
    "replace-audit-event",
    "create-knowledge-record",
    "delete-knowledge-record",
    "rename-knowledge-record",
    "replace-knowledge-record",
    "create-regulatory-requirement",
    "delete-regulatory-requirement",
    "rename-regulatory-requirement",
    "replace-regulatory-requirement",
    "create-change-record",
    "delete-change-record",
    "rename-change-record",
    "replace-change-record",
    "create-communication-requirement",
    "delete-communication-requirement",
    "rename-communication-requirement",
    "replace-communication-requirement",
    "create-resource",
    "delete-resource",
    "rename-resource",
    "replace-resource",
    "create-status-record",
    "delete-status-record",
    "rename-status-record",
    "replace-status-record",
    "create-process",
    "delete-process",
    "rename-process",
    "replace-process",
    "create-search-filter",
    "delete-search-filter",
    "rename-search-filter",
    "replace-search-filter",
    "create-access-rule",
    "delete-access-rule",
    "rename-access-rule",
    "replace-access-rule",
    "create-privacy-requirement",
    "delete-privacy-requirement",
    "rename-privacy-requirement",
    "replace-privacy-requirement",
    "create-relationship",
    "delete-relationship",
    "rename-relationship",
    "replace-relationship",
    "create-quantity-requirement",
    "delete-quantity-requirement",
    "rename-quantity-requirement",
    "replace-quantity-requirement",
    "create-analysis-record",
    "delete-analysis-record",
    "rename-analysis-record",
    "replace-analysis-record",
    "create-storage-requirement",
    "delete-storage-requirement",
    "rename-storage-requirement",
    "replace-storage-requirement",
    "create-meeting-record",
    "delete-meeting-record",
    "rename-meeting-record",
    "replace-meeting-record",
    "create-survey",
    "delete-survey",
    "rename-survey",
    "replace-survey",
    "create-delivery-constraint",
    "delete-delivery-constraint",
    "rename-delivery-constraint",
    "replace-delivery-constraint",
    "create-constraint-record",
    "delete-constraint-record",
    "rename-constraint-record",
    "replace-constraint-record",
    "create-compliance-record",
    "delete-compliance-record",
    "rename-compliance-record",
    "replace-compliance-record",
    "create-service-requirement",
    "delete-service-requirement",
    "rename-service-requirement",
    "replace-service-requirement",
    "create-equipment",
    "delete-equipment",
    "rename-equipment",
    "replace-equipment",
    "create-security-requirement",
    "delete-security-requirement",
    "rename-security-requirement",
    "replace-security-requirement",
    "create-collaboration-record",
    "delete-collaboration-record",
    "rename-collaboration-record",
    "replace-collaboration-record",
    "create-safety-requirement",
    "delete-safety-requirement",
    "rename-safety-requirement",
    "replace-safety-requirement",
    "create-user-profile",
    "delete-user-profile",
    "rename-user-profile",
    "replace-user-profile",
    "create-human-factor-requirement",
    "delete-human-factor-requirement",
    "rename-human-factor-requirement",
    "replace-human-factor-requirement",
    "create-flexibility-requirement",
    "delete-flexibility-requirement",
    "rename-flexibility-requirement",
    "replace-flexibility-requirement",
    "create-wayfinding-requirement",
    "delete-wayfinding-requirement",
    "rename-wayfinding-requirement",
    "replace-wayfinding-requirement",
    "create-program-element",
    "delete-program-element",
    "rename-program-element",
    "replace-program-element",
    "connect-adjacency",
    "disconnect-adjacency",
    "connect-trace",
    "disconnect-trace",
    "rename-meta",
    "replace-meta",
    "rename-project",
    "replace-project",
    "rename-governance",
    "replace-governance",
];

/// 🧮️ Applies `mutation` to `snapshot` and hands back the whole `protocol::MutationOutcome`, the
/// diagnostics included — the shape an external conformance host needs, since a committed
/// `🎯️outcome` vector declares a status AND its diagnostic codes. This facet is dispatch-only and
/// has never carried an apply helper of its own; every in-crate caller goes through
/// `store::ArtifactStore`, which an external host cannot construct.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn apply_program_mutation_outcome(snapshot: &mut ProgramSnapshot, mutation: &ProgramMutation) -> protocol::MutationOutcome<ProgramDiff> {
    let outcome = <ProgramMutation as protocol::Mutation<ProgramSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ `mutation`'s own inverse against `base`, as the step LIST `protocol::Mutation::inverse`
/// returns. Reachable from outside this crate, which `protocol::Mutation` itself is not — the
/// `protocol` extern-crate alias is private to `📦️glue.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_program_mutation_steps(mutation: &ProgramMutation, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    <ProgramMutation as protocol::Mutation<ProgramSnapshot>>::inverse(mutation, base)
}

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_program_mutation_json(text: &str) -> Result<ProgramMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️component.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_program_snapshot_json(text: &str) -> Result<ProgramSnapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_program_snapshot_json(snapshot: &ProgramSnapshot) -> String {
    serde_json::to_string(snapshot).expect("a ProgramSnapshot is always serializable")
}
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
mod kinds_catalog {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in the
    /// committed `program-1-any` catalog. The framework reads the catalog and never the enum, so
    /// this is the only thing standing between a renamed variant and a mutation catalog that
    /// silently measures a vocabulary the code no longer has.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <ProgramMutation as protocol::SemanticMutation<ProgramSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared ProgramMutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsCatalog
