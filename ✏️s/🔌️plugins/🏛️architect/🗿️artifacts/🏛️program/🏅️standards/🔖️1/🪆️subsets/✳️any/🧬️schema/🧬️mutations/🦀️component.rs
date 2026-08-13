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
    CreateInformationRequirement(super::create_information_requirement::mutation::CreateInformationRequirement),
    DeleteInformationRequirement(super::delete_information_requirement::mutation::DeleteInformationRequirement),
    RenameInformationRequirement(super::rename_information_requirement::mutation::RenameInformationRequirement),
    ReplaceInformationRequirement(super::replace_information_requirement::mutation::ReplaceInformationRequirement),
    CreateSustainabilityRequirement(super::create_sustainability_requirement::mutation::CreateSustainabilityRequirement),
    DeleteSustainabilityRequirement(super::delete_sustainability_requirement::mutation::DeleteSustainabilityRequirement),
    RenameSustainabilityRequirement(super::rename_sustainability_requirement::mutation::RenameSustainabilityRequirement),
    ReplaceSustainabilityRequirement(super::replace_sustainability_requirement::mutation::ReplaceSustainabilityRequirement),
    CreateAccessibilityRequirement(super::create_accessibility_requirement::mutation::CreateAccessibilityRequirement),
    DeleteAccessibilityRequirement(super::delete_accessibility_requirement::mutation::DeleteAccessibilityRequirement),
    RenameAccessibilityRequirement(super::rename_accessibility_requirement::mutation::RenameAccessibilityRequirement),
    ReplaceAccessibilityRequirement(super::replace_accessibility_requirement::mutation::ReplaceAccessibilityRequirement),
    CreateConflict(super::create_conflict::mutation::CreateConflict),
    DeleteConflict(super::delete_conflict::mutation::DeleteConflict),
    RenameConflict(super::rename_conflict::mutation::RenameConflict),
    ReplaceConflict(super::replace_conflict::mutation::ReplaceConflict),
    CreateOptionEvaluation(super::create_option_evaluation::mutation::CreateOptionEvaluation),
    DeleteOptionEvaluation(super::delete_option_evaluation::mutation::DeleteOptionEvaluation),
    RenameOptionEvaluation(super::rename_option_evaluation::mutation::RenameOptionEvaluation),
    ReplaceOptionEvaluation(super::replace_option_evaluation::mutation::ReplaceOptionEvaluation),
    CreateFunction(super::create_function::mutation::CreateFunction),
    DeleteFunction(super::delete_function::mutation::DeleteFunction),
    RenameFunction(super::rename_function::mutation::RenameFunction),
    ReplaceFunction(super::replace_function::mutation::ReplaceFunction),
    CreateRisk(super::create_risk::mutation::CreateRisk),
    DeleteRisk(super::delete_risk::mutation::DeleteRisk),
    RenameRisk(super::rename_risk::mutation::RenameRisk),
    ReplaceRisk(super::replace_risk::mutation::ReplaceRisk),
    CreateDecision(super::create_decision::mutation::CreateDecision),
    DeleteDecision(super::delete_decision::mutation::DeleteDecision),
    RenameDecision(super::rename_decision::mutation::RenameDecision),
    ReplaceDecision(super::replace_decision::mutation::ReplaceDecision),
    CreateValidationRecord(super::create_validation_record::mutation::CreateValidationRecord),
    DeleteValidationRecord(super::delete_validation_record::mutation::DeleteValidationRecord),
    RenameValidationRecord(super::rename_validation_record::mutation::RenameValidationRecord),
    ReplaceValidationRecord(super::replace_validation_record::mutation::ReplaceValidationRecord),
    CreatePriorityRecord(super::create_priority_record::mutation::CreatePriorityRecord),
    DeletePriorityRecord(super::delete_priority_record::mutation::DeletePriorityRecord),
    RenamePriorityRecord(super::rename_priority_record::mutation::RenamePriorityRecord),
    ReplacePriorityRecord(super::replace_priority_record::mutation::ReplacePriorityRecord),
    CreateFlowRequirement(super::create_flow_requirement::mutation::CreateFlowRequirement),
    DeleteFlowRequirement(super::delete_flow_requirement::mutation::DeleteFlowRequirement),
    RenameFlowRequirement(super::rename_flow_requirement::mutation::RenameFlowRequirement),
    ReplaceFlowRequirement(super::replace_flow_requirement::mutation::ReplaceFlowRequirement),
    CreateEnvironmentalRequirement(super::create_environmental_requirement::mutation::CreateEnvironmentalRequirement),
    DeleteEnvironmentalRequirement(super::delete_environmental_requirement::mutation::DeleteEnvironmentalRequirement),
    RenameEnvironmentalRequirement(super::rename_environmental_requirement::mutation::RenameEnvironmentalRequirement),
    ReplaceEnvironmentalRequirement(super::replace_environmental_requirement::mutation::ReplaceEnvironmentalRequirement),
    CreateWorkshop(super::create_workshop::mutation::CreateWorkshop),
    DeleteWorkshop(super::delete_workshop::mutation::DeleteWorkshop),
    RenameWorkshop(super::rename_workshop::mutation::RenameWorkshop),
    ReplaceWorkshop(super::replace_workshop::mutation::ReplaceWorkshop),
    CreateScenario(super::create_scenario::mutation::CreateScenario),
    DeleteScenario(super::delete_scenario::mutation::DeleteScenario),
    RenameScenario(super::rename_scenario::mutation::RenameScenario),
    ReplaceScenario(super::replace_scenario::mutation::ReplaceScenario),
    CreateBenchmarkRecord(super::create_benchmark_record::mutation::CreateBenchmarkRecord),
    DeleteBenchmarkRecord(super::delete_benchmark_record::mutation::DeleteBenchmarkRecord),
    RenameBenchmarkRecord(super::rename_benchmark_record::mutation::RenameBenchmarkRecord),
    ReplaceBenchmarkRecord(super::replace_benchmark_record::mutation::ReplaceBenchmarkRecord),
    CreateActivity(super::create_activity::mutation::CreateActivity),
    DeleteActivity(super::delete_activity::mutation::DeleteActivity),
    RenameActivity(super::rename_activity::mutation::RenameActivity),
    ReplaceActivity(super::replace_activity::mutation::ReplaceActivity),
    CreateInfrastructureRequirement(super::create_infrastructure_requirement::mutation::CreateInfrastructureRequirement),
    DeleteInfrastructureRequirement(super::delete_infrastructure_requirement::mutation::DeleteInfrastructureRequirement),
    RenameInfrastructureRequirement(super::rename_infrastructure_requirement::mutation::RenameInfrastructureRequirement),
    ReplaceInfrastructureRequirement(super::replace_infrastructure_requirement::mutation::ReplaceInfrastructureRequirement),
    CreateOrganizationalRequirement(super::create_organizational_requirement::mutation::CreateOrganizationalRequirement),
    DeleteOrganizationalRequirement(super::delete_organizational_requirement::mutation::DeleteOrganizationalRequirement),
    RenameOrganizationalRequirement(super::rename_organizational_requirement::mutation::RenameOrganizationalRequirement),
    ReplaceOrganizationalRequirement(super::replace_organizational_requirement::mutation::ReplaceOrganizationalRequirement),
    CreateIssue(super::create_issue::mutation::CreateIssue),
    DeleteIssue(super::delete_issue::mutation::DeleteIssue),
    RenameIssue(super::rename_issue::mutation::RenameIssue),
    ReplaceIssue(super::replace_issue::mutation::ReplaceIssue),
    CreateApprovalRecord(super::create_approval_record::mutation::CreateApprovalRecord),
    DeleteApprovalRecord(super::delete_approval_record::mutation::DeleteApprovalRecord),
    RenameApprovalRecord(super::rename_approval_record::mutation::RenameApprovalRecord),
    ReplaceApprovalRecord(super::replace_approval_record::mutation::ReplaceApprovalRecord),
    CreateStakeholder(super::create_stakeholder::mutation::CreateStakeholder),
    DeleteStakeholder(super::delete_stakeholder::mutation::DeleteStakeholder),
    RenameStakeholder(super::rename_stakeholder::mutation::RenameStakeholder),
    ReplaceStakeholder(super::replace_stakeholder::mutation::ReplaceStakeholder),
    CreateQualityRecord(super::create_quality_record::mutation::CreateQualityRecord),
    DeleteQualityRecord(super::delete_quality_record::mutation::DeleteQualityRecord),
    RenameQualityRecord(super::rename_quality_record::mutation::RenameQualityRecord),
    ReplaceQualityRecord(super::replace_quality_record::mutation::ReplaceQualityRecord),
    CreateResilienceRequirement(super::create_resilience_requirement::mutation::CreateResilienceRequirement),
    DeleteResilienceRequirement(super::delete_resilience_requirement::mutation::DeleteResilienceRequirement),
    RenameResilienceRequirement(super::rename_resilience_requirement::mutation::RenameResilienceRequirement),
    ReplaceResilienceRequirement(super::replace_resilience_requirement::mutation::ReplaceResilienceRequirement),
    CreateAssumption(super::create_assumption::mutation::CreateAssumption),
    DeleteAssumption(super::delete_assumption::mutation::DeleteAssumption),
    RenameAssumption(super::rename_assumption::mutation::RenameAssumption),
    ReplaceAssumption(super::replace_assumption::mutation::ReplaceAssumption),
    CreateCostRequirement(super::create_cost_requirement::mutation::CreateCostRequirement),
    DeleteCostRequirement(super::delete_cost_requirement::mutation::DeleteCostRequirement),
    RenameCostRequirement(super::rename_cost_requirement::mutation::RenameCostRequirement),
    ReplaceCostRequirement(super::replace_cost_requirement::mutation::ReplaceCostRequirement),
    CreateDocument(super::create_document::mutation::CreateDocument),
    DeleteDocument(super::delete_document::mutation::DeleteDocument),
    RenameDocument(super::rename_document::mutation::RenameDocument),
    ReplaceDocument(super::replace_document::mutation::ReplaceDocument),
    CreateScheduleRequirement(super::create_schedule_requirement::mutation::CreateScheduleRequirement),
    DeleteScheduleRequirement(super::delete_schedule_requirement::mutation::DeleteScheduleRequirement),
    RenameScheduleRequirement(super::rename_schedule_requirement::mutation::RenameScheduleRequirement),
    ReplaceScheduleRequirement(super::replace_schedule_requirement::mutation::ReplaceScheduleRequirement),
    CreateGrowthPlan(super::create_growth_plan::mutation::CreateGrowthPlan),
    DeleteGrowthPlan(super::delete_growth_plan::mutation::DeleteGrowthPlan),
    RenameGrowthPlan(super::rename_growth_plan::mutation::RenameGrowthPlan),
    ReplaceGrowthPlan(super::replace_growth_plan::mutation::ReplaceGrowthPlan),
    CreatePerformanceCriterion(super::create_performance_criterion::mutation::CreatePerformanceCriterion),
    DeletePerformanceCriterion(super::delete_performance_criterion::mutation::DeletePerformanceCriterion),
    RenamePerformanceCriterion(super::rename_performance_criterion::mutation::RenamePerformanceCriterion),
    ReplacePerformanceCriterion(super::replace_performance_criterion::mutation::ReplacePerformanceCriterion),
    CreateOperationalRequirement(super::create_operational_requirement::mutation::CreateOperationalRequirement),
    DeleteOperationalRequirement(super::delete_operational_requirement::mutation::DeleteOperationalRequirement),
    RenameOperationalRequirement(super::rename_operational_requirement::mutation::RenameOperationalRequirement),
    ReplaceOperationalRequirement(super::replace_operational_requirement::mutation::ReplaceOperationalRequirement),
    CreateRequirement(super::create_requirement::mutation::CreateRequirement),
    DeleteRequirement(super::delete_requirement::mutation::DeleteRequirement),
    RenameRequirement(super::rename_requirement::mutation::RenameRequirement),
    ReplaceRequirement(super::replace_requirement::mutation::ReplaceRequirement),
    CreateSiteContext(super::create_site_context::mutation::CreateSiteContext),
    DeleteSiteContext(super::delete_site_context::mutation::DeleteSiteContext),
    RenameSiteContext(super::rename_site_context::mutation::RenameSiteContext),
    ReplaceSiteContext(super::replace_site_context::mutation::ReplaceSiteContext),
    CreateTemplateRecord(super::create_template_record::mutation::CreateTemplateRecord),
    DeleteTemplateRecord(super::delete_template_record::mutation::DeleteTemplateRecord),
    RenameTemplateRecord(super::rename_template_record::mutation::RenameTemplateRecord),
    ReplaceTemplateRecord(super::replace_template_record::mutation::ReplaceTemplateRecord),
    CreateReportRecord(super::create_report_record::mutation::CreateReportRecord),
    DeleteReportRecord(super::delete_report_record::mutation::DeleteReportRecord),
    RenameReportRecord(super::rename_report_record::mutation::RenameReportRecord),
    ReplaceReportRecord(super::replace_report_record::mutation::ReplaceReportRecord),
    CreateAuditEvent(super::create_audit_event::mutation::CreateAuditEvent),
    DeleteAuditEvent(super::delete_audit_event::mutation::DeleteAuditEvent),
    RenameAuditEvent(super::rename_audit_event::mutation::RenameAuditEvent),
    ReplaceAuditEvent(super::replace_audit_event::mutation::ReplaceAuditEvent),
    CreateKnowledgeRecord(super::create_knowledge_record::mutation::CreateKnowledgeRecord),
    DeleteKnowledgeRecord(super::delete_knowledge_record::mutation::DeleteKnowledgeRecord),
    RenameKnowledgeRecord(super::rename_knowledge_record::mutation::RenameKnowledgeRecord),
    ReplaceKnowledgeRecord(super::replace_knowledge_record::mutation::ReplaceKnowledgeRecord),
    CreateRegulatoryRequirement(super::create_regulatory_requirement::mutation::CreateRegulatoryRequirement),
    DeleteRegulatoryRequirement(super::delete_regulatory_requirement::mutation::DeleteRegulatoryRequirement),
    RenameRegulatoryRequirement(super::rename_regulatory_requirement::mutation::RenameRegulatoryRequirement),
    ReplaceRegulatoryRequirement(super::replace_regulatory_requirement::mutation::ReplaceRegulatoryRequirement),
    CreateChangeRecord(super::create_change_record::mutation::CreateChangeRecord),
    DeleteChangeRecord(super::delete_change_record::mutation::DeleteChangeRecord),
    RenameChangeRecord(super::rename_change_record::mutation::RenameChangeRecord),
    ReplaceChangeRecord(super::replace_change_record::mutation::ReplaceChangeRecord),
    CreateCommunicationRequirement(super::create_communication_requirement::mutation::CreateCommunicationRequirement),
    DeleteCommunicationRequirement(super::delete_communication_requirement::mutation::DeleteCommunicationRequirement),
    RenameCommunicationRequirement(super::rename_communication_requirement::mutation::RenameCommunicationRequirement),
    ReplaceCommunicationRequirement(super::replace_communication_requirement::mutation::ReplaceCommunicationRequirement),
    CreateResource(super::create_resource::mutation::CreateResource),
    DeleteResource(super::delete_resource::mutation::DeleteResource),
    RenameResource(super::rename_resource::mutation::RenameResource),
    ReplaceResource(super::replace_resource::mutation::ReplaceResource),
    CreateStatusRecord(super::create_status_record::mutation::CreateStatusRecord),
    DeleteStatusRecord(super::delete_status_record::mutation::DeleteStatusRecord),
    RenameStatusRecord(super::rename_status_record::mutation::RenameStatusRecord),
    ReplaceStatusRecord(super::replace_status_record::mutation::ReplaceStatusRecord),
    CreateProcess(super::create_process::mutation::CreateProcess),
    DeleteProcess(super::delete_process::mutation::DeleteProcess),
    RenameProcess(super::rename_process::mutation::RenameProcess),
    ReplaceProcess(super::replace_process::mutation::ReplaceProcess),
    CreateSearchFilter(super::create_search_filter::mutation::CreateSearchFilter),
    DeleteSearchFilter(super::delete_search_filter::mutation::DeleteSearchFilter),
    RenameSearchFilter(super::rename_search_filter::mutation::RenameSearchFilter),
    ReplaceSearchFilter(super::replace_search_filter::mutation::ReplaceSearchFilter),
    CreateAccessRule(super::create_access_rule::mutation::CreateAccessRule),
    DeleteAccessRule(super::delete_access_rule::mutation::DeleteAccessRule),
    RenameAccessRule(super::rename_access_rule::mutation::RenameAccessRule),
    ReplaceAccessRule(super::replace_access_rule::mutation::ReplaceAccessRule),
    CreatePrivacyRequirement(super::create_privacy_requirement::mutation::CreatePrivacyRequirement),
    DeletePrivacyRequirement(super::delete_privacy_requirement::mutation::DeletePrivacyRequirement),
    RenamePrivacyRequirement(super::rename_privacy_requirement::mutation::RenamePrivacyRequirement),
    ReplacePrivacyRequirement(super::replace_privacy_requirement::mutation::ReplacePrivacyRequirement),
    CreateRelationship(super::create_relationship::mutation::CreateRelationship),
    DeleteRelationship(super::delete_relationship::mutation::DeleteRelationship),
    RenameRelationship(super::rename_relationship::mutation::RenameRelationship),
    ReplaceRelationship(super::replace_relationship::mutation::ReplaceRelationship),
    CreateQuantityRequirement(super::create_quantity_requirement::mutation::CreateQuantityRequirement),
    DeleteQuantityRequirement(super::delete_quantity_requirement::mutation::DeleteQuantityRequirement),
    RenameQuantityRequirement(super::rename_quantity_requirement::mutation::RenameQuantityRequirement),
    ReplaceQuantityRequirement(super::replace_quantity_requirement::mutation::ReplaceQuantityRequirement),
    CreateAnalysisRecord(super::create_analysis_record::mutation::CreateAnalysisRecord),
    DeleteAnalysisRecord(super::delete_analysis_record::mutation::DeleteAnalysisRecord),
    RenameAnalysisRecord(super::rename_analysis_record::mutation::RenameAnalysisRecord),
    ReplaceAnalysisRecord(super::replace_analysis_record::mutation::ReplaceAnalysisRecord),
    CreateStorageRequirement(super::create_storage_requirement::mutation::CreateStorageRequirement),
    DeleteStorageRequirement(super::delete_storage_requirement::mutation::DeleteStorageRequirement),
    RenameStorageRequirement(super::rename_storage_requirement::mutation::RenameStorageRequirement),
    ReplaceStorageRequirement(super::replace_storage_requirement::mutation::ReplaceStorageRequirement),
    CreateMeetingRecord(super::create_meeting_record::mutation::CreateMeetingRecord),
    DeleteMeetingRecord(super::delete_meeting_record::mutation::DeleteMeetingRecord),
    RenameMeetingRecord(super::rename_meeting_record::mutation::RenameMeetingRecord),
    ReplaceMeetingRecord(super::replace_meeting_record::mutation::ReplaceMeetingRecord),
    CreateSurvey(super::create_survey::mutation::CreateSurvey),
    DeleteSurvey(super::delete_survey::mutation::DeleteSurvey),
    RenameSurvey(super::rename_survey::mutation::RenameSurvey),
    ReplaceSurvey(super::replace_survey::mutation::ReplaceSurvey),
    CreateDeliveryConstraint(super::create_delivery_constraint::mutation::CreateDeliveryConstraint),
    DeleteDeliveryConstraint(super::delete_delivery_constraint::mutation::DeleteDeliveryConstraint),
    RenameDeliveryConstraint(super::rename_delivery_constraint::mutation::RenameDeliveryConstraint),
    ReplaceDeliveryConstraint(super::replace_delivery_constraint::mutation::ReplaceDeliveryConstraint),
    CreateConstraintRecord(super::create_constraint_record::mutation::CreateConstraintRecord),
    DeleteConstraintRecord(super::delete_constraint_record::mutation::DeleteConstraintRecord),
    RenameConstraintRecord(super::rename_constraint_record::mutation::RenameConstraintRecord),
    ReplaceConstraintRecord(super::replace_constraint_record::mutation::ReplaceConstraintRecord),
    CreateComplianceRecord(super::create_compliance_record::mutation::CreateComplianceRecord),
    DeleteComplianceRecord(super::delete_compliance_record::mutation::DeleteComplianceRecord),
    RenameComplianceRecord(super::rename_compliance_record::mutation::RenameComplianceRecord),
    ReplaceComplianceRecord(super::replace_compliance_record::mutation::ReplaceComplianceRecord),
    CreateServiceRequirement(super::create_service_requirement::mutation::CreateServiceRequirement),
    DeleteServiceRequirement(super::delete_service_requirement::mutation::DeleteServiceRequirement),
    RenameServiceRequirement(super::rename_service_requirement::mutation::RenameServiceRequirement),
    ReplaceServiceRequirement(super::replace_service_requirement::mutation::ReplaceServiceRequirement),
    CreateEquipment(super::create_equipment::mutation::CreateEquipment),
    DeleteEquipment(super::delete_equipment::mutation::DeleteEquipment),
    RenameEquipment(super::rename_equipment::mutation::RenameEquipment),
    ReplaceEquipment(super::replace_equipment::mutation::ReplaceEquipment),
    CreateSecurityRequirement(super::create_security_requirement::mutation::CreateSecurityRequirement),
    DeleteSecurityRequirement(super::delete_security_requirement::mutation::DeleteSecurityRequirement),
    RenameSecurityRequirement(super::rename_security_requirement::mutation::RenameSecurityRequirement),
    ReplaceSecurityRequirement(super::replace_security_requirement::mutation::ReplaceSecurityRequirement),
    CreateCollaborationRecord(super::create_collaboration_record::mutation::CreateCollaborationRecord),
    DeleteCollaborationRecord(super::delete_collaboration_record::mutation::DeleteCollaborationRecord),
    RenameCollaborationRecord(super::rename_collaboration_record::mutation::RenameCollaborationRecord),
    ReplaceCollaborationRecord(super::replace_collaboration_record::mutation::ReplaceCollaborationRecord),
    CreateSafetyRequirement(super::create_safety_requirement::mutation::CreateSafetyRequirement),
    DeleteSafetyRequirement(super::delete_safety_requirement::mutation::DeleteSafetyRequirement),
    RenameSafetyRequirement(super::rename_safety_requirement::mutation::RenameSafetyRequirement),
    ReplaceSafetyRequirement(super::replace_safety_requirement::mutation::ReplaceSafetyRequirement),
    CreateUserProfile(super::create_user_profile::mutation::CreateUserProfile),
    DeleteUserProfile(super::delete_user_profile::mutation::DeleteUserProfile),
    RenameUserProfile(super::rename_user_profile::mutation::RenameUserProfile),
    ReplaceUserProfile(super::replace_user_profile::mutation::ReplaceUserProfile),
    CreateHumanFactorRequirement(super::create_human_factor_requirement::mutation::CreateHumanFactorRequirement),
    DeleteHumanFactorRequirement(super::delete_human_factor_requirement::mutation::DeleteHumanFactorRequirement),
    RenameHumanFactorRequirement(super::rename_human_factor_requirement::mutation::RenameHumanFactorRequirement),
    ReplaceHumanFactorRequirement(super::replace_human_factor_requirement::mutation::ReplaceHumanFactorRequirement),
    CreateFlexibilityRequirement(super::create_flexibility_requirement::mutation::CreateFlexibilityRequirement),
    DeleteFlexibilityRequirement(super::delete_flexibility_requirement::mutation::DeleteFlexibilityRequirement),
    RenameFlexibilityRequirement(super::rename_flexibility_requirement::mutation::RenameFlexibilityRequirement),
    ReplaceFlexibilityRequirement(super::replace_flexibility_requirement::mutation::ReplaceFlexibilityRequirement),
    CreateWayfindingRequirement(super::create_wayfinding_requirement::mutation::CreateWayfindingRequirement),
    DeleteWayfindingRequirement(super::delete_wayfinding_requirement::mutation::DeleteWayfindingRequirement),
    RenameWayfindingRequirement(super::rename_wayfinding_requirement::mutation::RenameWayfindingRequirement),
    ReplaceWayfindingRequirement(super::replace_wayfinding_requirement::mutation::ReplaceWayfindingRequirement),
    CreateProgramElement(super::create_program_element::mutation::CreateProgramElement),
    DeleteProgramElement(super::delete_program_element::mutation::DeleteProgramElement),
    RenameProgramElement(super::rename_program_element::mutation::RenameProgramElement),
    ReplaceProgramElement(super::replace_program_element::mutation::ReplaceProgramElement),
    ConnectAdjacency(super::connect_adjacency::mutation::ConnectAdjacency),
    DisconnectAdjacency(super::disconnect_adjacency::mutation::DisconnectAdjacency),
    ConnectTrace(super::connect_trace::mutation::ConnectTrace),
    DisconnectTrace(super::disconnect_trace::mutation::DisconnectTrace),
    RenameMeta(super::rename_meta::mutation::RenameMeta),
    ReplaceMeta(super::replace_meta::mutation::ReplaceMeta),
    RenameProject(super::rename_project::mutation::RenameProject),
    ReplaceProject(super::replace_project::mutation::ReplaceProject),
    RenameGovernance(super::rename_governance::mutation::RenameGovernance),
    ReplaceGovernance(super::replace_governance::mutation::ReplaceGovernance),
}
//#endregion 🔖️ProgramMutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::*;
    use crate::artifacts::program::registers::*;
    use crate::artifacts::program::{empty_plugin, sample_plugin};
    use protocol::{Mutation, MutationDiff, OpText, SemanticMutation};

    fn round_trip(snapshot: &ProgramSnapshot, operation: &ProgramMutation) -> ProgramSnapshot {
        let forward = operation.diff(snapshot).apply(snapshot);
        let mut backward = operation.inverse(snapshot);
        backward.reverse();
        let mut restored = forward.clone();
        for undo in &backward {
            restored = undo.diff(&restored).apply(&restored);
        }
        assert_eq!(&restored, snapshot, "inverse (reversed) must exactly restore the pre-operation fixture");
        forward
    }

    //#region 👥stakeholders
    #[test]
    fn stakeholders_create_rename_replace_delete_round_trip() {
        let snapshot = sample_plugin();
        let new_id = EntityId::new_serial("stakeholder", "stakeholder");
        let mut new_stakeholder = snapshot.stakeholders[0].clone();
        new_stakeholder.header.id = new_id.clone();
        new_stakeholder.header.name = "New Stakeholder".into();

        let create = ProgramMutation::CreateStakeholder(super::super::create_stakeholder::mutation::CreateStakeholder { stakeholder: new_stakeholder });
        let with_new = round_trip(&snapshot, &create);
        assert_eq!(with_new.stakeholders.len(), snapshot.stakeholders.len() + 1);

        let rename = ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::mutation::RenameStakeholder { id: new_id.clone(), new_name: "Renamed".into() });
        let renamed = round_trip(&with_new, &rename);
        assert_eq!(renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().header.name, "Renamed");

        let mut replacement = renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().clone();
        replacement.role = "Sponsor".into();
        let replace = ProgramMutation::ReplaceStakeholder(super::super::replace_stakeholder::mutation::ReplaceStakeholder { stakeholder: replacement });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().role, "Sponsor");

        let delete = ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::mutation::DeleteStakeholder { id: new_id });
        let deleted = round_trip(&replaced, &delete);
        assert_eq!(deleted.stakeholders.len(), snapshot.stakeholders.len());
    }

    #[test]
    fn delete_stakeholder_of_a_missing_id_has_an_empty_inverse() {
        let snapshot = sample_plugin();
        let delete = ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::mutation::DeleteStakeholder { id: EntityId("nope".into()) });
        assert!(delete.inverse(&snapshot).is_empty(), "deleting an absent id has nothing to undo");
    }
    //#endregion 👥stakeholders

    //#region 🧱elements
    #[test]
    fn elements_create_rename_replace_delete_round_trip() {
        let snapshot = sample_plugin();
        let new_id = EntityId::new_serial("element", "element");
        let mut new_element = snapshot.elements[0].clone();
        new_element.header.id = new_id.clone();
        new_element.header.name = "Storage".into();

        let create = ProgramMutation::CreateProgramElement(super::super::create_program_element::mutation::CreateProgramElement { program_element: new_element });
        let with_new = round_trip(&snapshot, &create);
        assert_eq!(with_new.elements.len(), snapshot.elements.len() + 1);

        let rename = ProgramMutation::RenameProgramElement(super::super::rename_program_element::mutation::RenameProgramElement { id: new_id.clone(), new_name: "Storage Room".into() });
        let renamed = round_trip(&with_new, &rename);
        assert_eq!(renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().header.name, "Storage Room");

        let mut replacement = renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().clone();
        replacement.code = "STO".into();
        let replace = ProgramMutation::ReplaceProgramElement(super::super::replace_program_element::mutation::ReplaceProgramElement { program_element: replacement });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.elements.iter().find(|e| e.header.id == new_id).unwrap().code, "STO");

        let delete = ProgramMutation::DeleteProgramElement(super::super::delete_program_element::mutation::DeleteProgramElement { id: new_id });
        let deleted = round_trip(&replaced, &delete);
        assert_eq!(deleted.elements.len(), snapshot.elements.len());
    }
    //#endregion 🧱elements

    //#region 🏷️📁🏛️meta-project-governance
    #[test]
    fn update_meta_rename_and_replace_round_trip() {
        let snapshot = empty_plugin();
        let rename = ProgramMutation::RenameMeta(super::super::rename_meta::mutation::RenameMeta { new_title: "Clinic".into() });
        let renamed = round_trip(&snapshot, &rename);
        assert_eq!(renamed.meta.title, "Clinic");

        let mut new_meta = renamed.meta.clone();
        new_meta.industry_sector = "healthcare".into();
        let replace = ProgramMutation::ReplaceMeta(super::super::replace_meta::mutation::ReplaceMeta { new_meta });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.meta.industry_sector, "healthcare");
    }

    #[test]
    fn update_project_rename_and_replace_round_trip() {
        let snapshot = empty_plugin();
        let rename = ProgramMutation::RenameProject(super::super::rename_project::mutation::RenameProject { new_code: "CLN-001".into() });
        let renamed = round_trip(&snapshot, &rename);
        assert_eq!(renamed.project.code, "CLN-001");

        let mut new_project = renamed.project.clone();
        new_project.client_name = "Sample Health".into();
        let replace = ProgramMutation::ReplaceProject(super::super::replace_project::mutation::ReplaceProject { new_project });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.project.client_name, "Sample Health");
    }

    #[test]
    fn update_governance_rename_and_replace_round_trip() {
        let snapshot = empty_plugin();
        let rename = ProgramMutation::RenameGovernance(super::super::rename_governance::mutation::RenameGovernance { new_framework: "ISO 41001".into() });
        let renamed = round_trip(&snapshot, &rename);
        assert_eq!(renamed.governance.framework, "ISO 41001");

        let mut new_governance = renamed.governance.clone();
        new_governance.risk_appetite = Some("Low".into());
        let replace = ProgramMutation::ReplaceGovernance(super::super::replace_governance::mutation::ReplaceGovernance { new_governance });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.governance.risk_appetite, Some("Low".into()));
    }
    //#endregion 🏷️📁🏛️meta-project-governance

    //#region 🗺️🧹connect-disconnect-adjacency
    #[test]
    fn connect_and_disconnect_adjacency_round_trip() {
        let snapshot = sample_plugin();
        let a = EntityId::new_serial("element", "element");
        let b = EntityId::new_serial("element", "element");
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
        let connect = ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::mutation::ConnectAdjacency { adjacency: new_adjacency });
        let connected = round_trip(&snapshot, &connect);
        assert_eq!(connected.adjacencies.len(), snapshot.adjacencies.len() + 1);

        let disconnect = ProgramMutation::DisconnectAdjacency(super::super::disconnect_adjacency::mutation::DisconnectAdjacency { id: new_id });
        let disconnected = round_trip(&connected, &disconnect);
        assert_eq!(disconnected.adjacencies.len(), snapshot.adjacencies.len());
    }

    #[test]
    fn connect_adjacency_upserts_an_existing_pair_by_endpoint_identity() {
        let snapshot = sample_plugin();
        let existing = &snapshot.adjacencies[0];
        let mut updated = existing.clone();
        updated.header.id = EntityId::new_serial("adjacency", "adjacency");
        updated.weight = 5.0;
        let connect = ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::mutation::ConnectAdjacency { adjacency: updated });
        let connected = round_trip(&snapshot, &connect);
        assert_eq!(connected.adjacencies.len(), snapshot.adjacencies.len(), "same-pair connect patches in place, it does not add a row");
        assert_eq!(connected.adjacencies[0].weight, 5.0);
        assert_eq!(connected.adjacencies[0].header.id, existing.header.id, "the pre-existing edge keeps its own id");
    }
    //#endregion 🗺️🧹connect-disconnect-adjacency

    //#region 🧵connect-disconnect-trace
    #[test]
    fn connect_and_disconnect_trace_round_trip() {
        let snapshot = sample_plugin();
        let trace = TraceLink::new(snapshot.elements[0].header.id.clone(), snapshot.elements[1].header.id.clone(), TraceKind::FullAuditTrail);
        let id = trace.id.clone();
        let connect = ProgramMutation::ConnectTrace(super::super::connect_trace::mutation::ConnectTrace { trace });
        let connected = round_trip(&snapshot, &connect);
        assert_eq!(connected.traces.len(), 1);

        let disconnect = ProgramMutation::DisconnectTrace(super::super::disconnect_trace::mutation::DisconnectTrace { id });
        let disconnected = round_trip(&connected, &disconnect);
        assert!(disconnected.traces.is_empty());
    }
    //#endregion 🧵connect-disconnect-trace

    //#region 🗣️OpText
    #[test]
    fn program_mutation_op_text_round_trips_a_sample_of_variants() {
        let stakeholder = sample_plugin().stakeholders[0].clone();
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::CreateStakeholder(super::super::create_stakeholder::mutation::CreateStakeholder { stakeholder: stakeholder.clone() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::mutation::DeleteStakeholder { id: stakeholder.header.id.clone() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::mutation::RenameStakeholder { id: stakeholder.header.id.clone(), new_name: "X".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::ReplaceStakeholder(super::super::replace_stakeholder::mutation::ReplaceStakeholder { stakeholder }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameMeta(super::super::rename_meta::mutation::RenameMeta { new_title: "Clinic".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DisconnectAdjacency(super::super::disconnect_adjacency::mutation::DisconnectAdjacency { id: EntityId("a1".into()) }));
    }
    //#endregion 🗣️OpText

    //#region ⚖️SemanticLaws
    /// ⚖️ `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law`
    /// (`protocol::os_spr::testkit`, added by the Wave 0 mechanism pass) against the three most
    /// structurally distinct new kinds: an id-keyed collection create/delete pair, a document-level
    /// scalar facet rename, and an edge upsert.
    #[test]
    fn create_stakeholder_obeys_the_inverse_and_absorb_laws() {
        let base = sample_plugin();
        let mut new_stakeholder = base.stakeholders[0].clone();
        new_stakeholder.header.id = EntityId::new_serial("stakeholder", "stakeholder");
        let create = ProgramMutation::CreateStakeholder(super::super::create_stakeholder::mutation::CreateStakeholder { stakeholder: new_stakeholder.clone() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &create);
        let d1 = create.diff(&base);
        let after = d1.apply(&base);
        let d2 = ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::mutation::RenameStakeholder { id: new_stakeholder.header.id, new_name: "Renamed".into() }).diff(&after);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn rename_meta_obeys_the_inverse_law() {
        let base = sample_plugin();
        let rename = ProgramMutation::RenameMeta(super::super::rename_meta::mutation::RenameMeta { new_title: "Renamed Program".into() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &rename);
    }

    #[test]
    fn connect_adjacency_obeys_the_inverse_law() {
        let base = sample_plugin();
        let mut updated = base.adjacencies[0].clone();
        updated.weight = 9.0;
        let connect = ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::mutation::ConnectAdjacency { adjacency: updated });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &connect);
    }
    //#endregion ⚖️SemanticLaws

    //#region 📋️DescriptorLaws
    #[test]
    fn semantic_kinds_cover_every_variant() {
        assert_eq!(ProgramMutation::kinds().len(), 266);
        let stakeholder = sample_plugin().stakeholders[0].clone();
        let mutation = ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::mutation::RenameStakeholder { id: stakeholder.header.id, new_name: "X".into() });
        assert_eq!(mutation.semantics().kind, "rename-stakeholder");
        assert_eq!(mutation.semantics().record, "RenamedStakeholder");
    }
    //#endregion 📋️DescriptorLaws
}
//#endregion 🧪️Tests
