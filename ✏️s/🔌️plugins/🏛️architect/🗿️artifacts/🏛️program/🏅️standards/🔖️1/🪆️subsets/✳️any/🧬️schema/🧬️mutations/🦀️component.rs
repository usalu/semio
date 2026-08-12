//! 🧬️ Architect program artifact — document mutation dispatch enum.
//!
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s ~65 `CollectionMutation<EntityId,
//! T, TPatch>` registers plus the three document-level meta facets, per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`/`📓️derivation-rules.md`:
//! each header-shaped id-keyed register (rule 2) becomes `create`/`delete`/`rename`/`replace`;
//! the two edge-shaped registers (`adjacencies`, `traces` — rule 4) become `connect`/`disconnect`;
//! the three document-level scalar facets (`meta`/`project`/`governance` — rule 1) become
//! `rename`/`replace`. `SetSnapshot` is deleted outright (banned per taxonomy — whole-document
//! replace is not an in-history mutation; it goes through `ArtifactStore::reset`).
//!
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<ProgramSnapshot>` and
//! `impl protocol::SemanticMutation<ProgramSnapshot>` for `ProgramMutation` by delegating each
//! variant to its payload's `protocol::MutationKind` impl — see the triad leaves
//! (`<slug>/{🦠️mutation,🔺️diff,↩️inverse}`) for the handcrafted logic. This file is dispatch-only;
//! the old hand-written `apply_program_mutation`/`inverse_program_mutation`/`impl Mutation` are
//! deleted, replaced by the derive.
//!
//! Physical directory layout note (same precedent as the shooting/playground facets migrated in
//! this same overhaul): `📦️glue.rs` (outside this facet's package boundary) `#[path]`-wires
//! exactly the pre-migration set of triad directories, one per register NOUN (e.g. `👥stakeholders`,
//! `ℹ️information`) — so each such directory now hosts EVERY semantic mutation kind derived from
//! that register (`👥stakeholders` hosts `CreateStakeholder`/`DeleteStakeholder`/`RenameStakeholder`/
//! `ReplaceStakeholder`) rather than one-triad-dir-per-verb. Renaming directories to verb-first
//! slugs and re-wiring `glue.rs` is a later pass, tracked via this ticket's wave2 report
//! `sharedFileRequests`. Two directories are now orphan stubs kept only because `glue.rs` still
//! `#[path]`-wires them: `🔀adjacencies` (superseded by `connect`/`disconnect-adjacency` in
//! `🗺️set-adjacency`/`🧹clear-adjacency`) and `🖼️set-snapshot` (banned outright, no replacement).

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
    CreateInformationRequirement(super::information::mutation::CreateInformationRequirement),
    DeleteInformationRequirement(super::information::mutation::DeleteInformationRequirement),
    RenameInformationRequirement(super::information::mutation::RenameInformationRequirement),
    ReplaceInformationRequirement(super::information::mutation::ReplaceInformationRequirement),
    CreateSustainabilityRequirement(super::sustainability::mutation::CreateSustainabilityRequirement),
    DeleteSustainabilityRequirement(super::sustainability::mutation::DeleteSustainabilityRequirement),
    RenameSustainabilityRequirement(super::sustainability::mutation::RenameSustainabilityRequirement),
    ReplaceSustainabilityRequirement(super::sustainability::mutation::ReplaceSustainabilityRequirement),
    CreateAccessibilityRequirement(super::accessibility::mutation::CreateAccessibilityRequirement),
    DeleteAccessibilityRequirement(super::accessibility::mutation::DeleteAccessibilityRequirement),
    RenameAccessibilityRequirement(super::accessibility::mutation::RenameAccessibilityRequirement),
    ReplaceAccessibilityRequirement(super::accessibility::mutation::ReplaceAccessibilityRequirement),
    CreateConflict(super::conflicts::mutation::CreateConflict),
    DeleteConflict(super::conflicts::mutation::DeleteConflict),
    RenameConflict(super::conflicts::mutation::RenameConflict),
    ReplaceConflict(super::conflicts::mutation::ReplaceConflict),
    CreateOptionEvaluation(super::options::mutation::CreateOptionEvaluation),
    DeleteOptionEvaluation(super::options::mutation::DeleteOptionEvaluation),
    RenameOptionEvaluation(super::options::mutation::RenameOptionEvaluation),
    ReplaceOptionEvaluation(super::options::mutation::ReplaceOptionEvaluation),
    CreateFunction(super::functions::mutation::CreateFunction),
    DeleteFunction(super::functions::mutation::DeleteFunction),
    RenameFunction(super::functions::mutation::RenameFunction),
    ReplaceFunction(super::functions::mutation::ReplaceFunction),
    CreateRisk(super::risks::mutation::CreateRisk),
    DeleteRisk(super::risks::mutation::DeleteRisk),
    RenameRisk(super::risks::mutation::RenameRisk),
    ReplaceRisk(super::risks::mutation::ReplaceRisk),
    CreateDecision(super::decisions::mutation::CreateDecision),
    DeleteDecision(super::decisions::mutation::DeleteDecision),
    RenameDecision(super::decisions::mutation::RenameDecision),
    ReplaceDecision(super::decisions::mutation::ReplaceDecision),
    CreateValidationRecord(super::validations::mutation::CreateValidationRecord),
    DeleteValidationRecord(super::validations::mutation::DeleteValidationRecord),
    RenameValidationRecord(super::validations::mutation::RenameValidationRecord),
    ReplaceValidationRecord(super::validations::mutation::ReplaceValidationRecord),
    CreatePriorityRecord(super::priorities::mutation::CreatePriorityRecord),
    DeletePriorityRecord(super::priorities::mutation::DeletePriorityRecord),
    RenamePriorityRecord(super::priorities::mutation::RenamePriorityRecord),
    ReplacePriorityRecord(super::priorities::mutation::ReplacePriorityRecord),
    CreateFlowRequirement(super::flows::mutation::CreateFlowRequirement),
    DeleteFlowRequirement(super::flows::mutation::DeleteFlowRequirement),
    RenameFlowRequirement(super::flows::mutation::RenameFlowRequirement),
    ReplaceFlowRequirement(super::flows::mutation::ReplaceFlowRequirement),
    CreateEnvironmentalRequirement(super::environmental::mutation::CreateEnvironmentalRequirement),
    DeleteEnvironmentalRequirement(super::environmental::mutation::DeleteEnvironmentalRequirement),
    RenameEnvironmentalRequirement(super::environmental::mutation::RenameEnvironmentalRequirement),
    ReplaceEnvironmentalRequirement(super::environmental::mutation::ReplaceEnvironmentalRequirement),
    CreateWorkshop(super::workshops::mutation::CreateWorkshop),
    DeleteWorkshop(super::workshops::mutation::DeleteWorkshop),
    RenameWorkshop(super::workshops::mutation::RenameWorkshop),
    ReplaceWorkshop(super::workshops::mutation::ReplaceWorkshop),
    CreateScenario(super::scenarios::mutation::CreateScenario),
    DeleteScenario(super::scenarios::mutation::DeleteScenario),
    RenameScenario(super::scenarios::mutation::RenameScenario),
    ReplaceScenario(super::scenarios::mutation::ReplaceScenario),
    CreateBenchmarkRecord(super::benchmarks::mutation::CreateBenchmarkRecord),
    DeleteBenchmarkRecord(super::benchmarks::mutation::DeleteBenchmarkRecord),
    RenameBenchmarkRecord(super::benchmarks::mutation::RenameBenchmarkRecord),
    ReplaceBenchmarkRecord(super::benchmarks::mutation::ReplaceBenchmarkRecord),
    CreateActivity(super::activities::mutation::CreateActivity),
    DeleteActivity(super::activities::mutation::DeleteActivity),
    RenameActivity(super::activities::mutation::RenameActivity),
    ReplaceActivity(super::activities::mutation::ReplaceActivity),
    CreateInfrastructureRequirement(super::infrastructure::mutation::CreateInfrastructureRequirement),
    DeleteInfrastructureRequirement(super::infrastructure::mutation::DeleteInfrastructureRequirement),
    RenameInfrastructureRequirement(super::infrastructure::mutation::RenameInfrastructureRequirement),
    ReplaceInfrastructureRequirement(super::infrastructure::mutation::ReplaceInfrastructureRequirement),
    CreateOrganizationalRequirement(super::organizational::mutation::CreateOrganizationalRequirement),
    DeleteOrganizationalRequirement(super::organizational::mutation::DeleteOrganizationalRequirement),
    RenameOrganizationalRequirement(super::organizational::mutation::RenameOrganizationalRequirement),
    ReplaceOrganizationalRequirement(super::organizational::mutation::ReplaceOrganizationalRequirement),
    CreateIssue(super::issues::mutation::CreateIssue),
    DeleteIssue(super::issues::mutation::DeleteIssue),
    RenameIssue(super::issues::mutation::RenameIssue),
    ReplaceIssue(super::issues::mutation::ReplaceIssue),
    CreateApprovalRecord(super::approvals::mutation::CreateApprovalRecord),
    DeleteApprovalRecord(super::approvals::mutation::DeleteApprovalRecord),
    RenameApprovalRecord(super::approvals::mutation::RenameApprovalRecord),
    ReplaceApprovalRecord(super::approvals::mutation::ReplaceApprovalRecord),
    CreateStakeholder(super::stakeholders::mutation::CreateStakeholder),
    DeleteStakeholder(super::stakeholders::mutation::DeleteStakeholder),
    RenameStakeholder(super::stakeholders::mutation::RenameStakeholder),
    ReplaceStakeholder(super::stakeholders::mutation::ReplaceStakeholder),
    CreateQualityRecord(super::quality::mutation::CreateQualityRecord),
    DeleteQualityRecord(super::quality::mutation::DeleteQualityRecord),
    RenameQualityRecord(super::quality::mutation::RenameQualityRecord),
    ReplaceQualityRecord(super::quality::mutation::ReplaceQualityRecord),
    CreateResilienceRequirement(super::resilience::mutation::CreateResilienceRequirement),
    DeleteResilienceRequirement(super::resilience::mutation::DeleteResilienceRequirement),
    RenameResilienceRequirement(super::resilience::mutation::RenameResilienceRequirement),
    ReplaceResilienceRequirement(super::resilience::mutation::ReplaceResilienceRequirement),
    CreateAssumption(super::assumptions::mutation::CreateAssumption),
    DeleteAssumption(super::assumptions::mutation::DeleteAssumption),
    RenameAssumption(super::assumptions::mutation::RenameAssumption),
    ReplaceAssumption(super::assumptions::mutation::ReplaceAssumption),
    CreateCostRequirement(super::costs::mutation::CreateCostRequirement),
    DeleteCostRequirement(super::costs::mutation::DeleteCostRequirement),
    RenameCostRequirement(super::costs::mutation::RenameCostRequirement),
    ReplaceCostRequirement(super::costs::mutation::ReplaceCostRequirement),
    CreateDocument(super::documents::mutation::CreateDocument),
    DeleteDocument(super::documents::mutation::DeleteDocument),
    RenameDocument(super::documents::mutation::RenameDocument),
    ReplaceDocument(super::documents::mutation::ReplaceDocument),
    CreateScheduleRequirement(super::schedules::mutation::CreateScheduleRequirement),
    DeleteScheduleRequirement(super::schedules::mutation::DeleteScheduleRequirement),
    RenameScheduleRequirement(super::schedules::mutation::RenameScheduleRequirement),
    ReplaceScheduleRequirement(super::schedules::mutation::ReplaceScheduleRequirement),
    CreateGrowthPlan(super::growth::mutation::CreateGrowthPlan),
    DeleteGrowthPlan(super::growth::mutation::DeleteGrowthPlan),
    RenameGrowthPlan(super::growth::mutation::RenameGrowthPlan),
    ReplaceGrowthPlan(super::growth::mutation::ReplaceGrowthPlan),
    CreatePerformanceCriterion(super::performance::mutation::CreatePerformanceCriterion),
    DeletePerformanceCriterion(super::performance::mutation::DeletePerformanceCriterion),
    RenamePerformanceCriterion(super::performance::mutation::RenamePerformanceCriterion),
    ReplacePerformanceCriterion(super::performance::mutation::ReplacePerformanceCriterion),
    CreateOperationalRequirement(super::operations::mutation::CreateOperationalRequirement),
    DeleteOperationalRequirement(super::operations::mutation::DeleteOperationalRequirement),
    RenameOperationalRequirement(super::operations::mutation::RenameOperationalRequirement),
    ReplaceOperationalRequirement(super::operations::mutation::ReplaceOperationalRequirement),
    CreateRequirement(super::requirements::mutation::CreateRequirement),
    DeleteRequirement(super::requirements::mutation::DeleteRequirement),
    RenameRequirement(super::requirements::mutation::RenameRequirement),
    ReplaceRequirement(super::requirements::mutation::ReplaceRequirement),
    CreateSiteContext(super::site_context::mutation::CreateSiteContext),
    DeleteSiteContext(super::site_context::mutation::DeleteSiteContext),
    RenameSiteContext(super::site_context::mutation::RenameSiteContext),
    ReplaceSiteContext(super::site_context::mutation::ReplaceSiteContext),
    CreateTemplateRecord(super::templates::mutation::CreateTemplateRecord),
    DeleteTemplateRecord(super::templates::mutation::DeleteTemplateRecord),
    RenameTemplateRecord(super::templates::mutation::RenameTemplateRecord),
    ReplaceTemplateRecord(super::templates::mutation::ReplaceTemplateRecord),
    CreateReportRecord(super::reports::mutation::CreateReportRecord),
    DeleteReportRecord(super::reports::mutation::DeleteReportRecord),
    RenameReportRecord(super::reports::mutation::RenameReportRecord),
    ReplaceReportRecord(super::reports::mutation::ReplaceReportRecord),
    CreateAuditEvent(super::audit_events::mutation::CreateAuditEvent),
    DeleteAuditEvent(super::audit_events::mutation::DeleteAuditEvent),
    RenameAuditEvent(super::audit_events::mutation::RenameAuditEvent),
    ReplaceAuditEvent(super::audit_events::mutation::ReplaceAuditEvent),
    CreateKnowledgeRecord(super::knowledge::mutation::CreateKnowledgeRecord),
    DeleteKnowledgeRecord(super::knowledge::mutation::DeleteKnowledgeRecord),
    RenameKnowledgeRecord(super::knowledge::mutation::RenameKnowledgeRecord),
    ReplaceKnowledgeRecord(super::knowledge::mutation::ReplaceKnowledgeRecord),
    CreateRegulatoryRequirement(super::regulatory::mutation::CreateRegulatoryRequirement),
    DeleteRegulatoryRequirement(super::regulatory::mutation::DeleteRegulatoryRequirement),
    RenameRegulatoryRequirement(super::regulatory::mutation::RenameRegulatoryRequirement),
    ReplaceRegulatoryRequirement(super::regulatory::mutation::ReplaceRegulatoryRequirement),
    CreateChangeRecord(super::changes::mutation::CreateChangeRecord),
    DeleteChangeRecord(super::changes::mutation::DeleteChangeRecord),
    RenameChangeRecord(super::changes::mutation::RenameChangeRecord),
    ReplaceChangeRecord(super::changes::mutation::ReplaceChangeRecord),
    CreateCommunicationRequirement(super::communication::mutation::CreateCommunicationRequirement),
    DeleteCommunicationRequirement(super::communication::mutation::DeleteCommunicationRequirement),
    RenameCommunicationRequirement(super::communication::mutation::RenameCommunicationRequirement),
    ReplaceCommunicationRequirement(super::communication::mutation::ReplaceCommunicationRequirement),
    CreateResource(super::resources::mutation::CreateResource),
    DeleteResource(super::resources::mutation::DeleteResource),
    RenameResource(super::resources::mutation::RenameResource),
    ReplaceResource(super::resources::mutation::ReplaceResource),
    CreateStatusRecord(super::status_records::mutation::CreateStatusRecord),
    DeleteStatusRecord(super::status_records::mutation::DeleteStatusRecord),
    RenameStatusRecord(super::status_records::mutation::RenameStatusRecord),
    ReplaceStatusRecord(super::status_records::mutation::ReplaceStatusRecord),
    CreateProcess(super::processes::mutation::CreateProcess),
    DeleteProcess(super::processes::mutation::DeleteProcess),
    RenameProcess(super::processes::mutation::RenameProcess),
    ReplaceProcess(super::processes::mutation::ReplaceProcess),
    CreateSearchFilter(super::search_filters::mutation::CreateSearchFilter),
    DeleteSearchFilter(super::search_filters::mutation::DeleteSearchFilter),
    RenameSearchFilter(super::search_filters::mutation::RenameSearchFilter),
    ReplaceSearchFilter(super::search_filters::mutation::ReplaceSearchFilter),
    CreateAccessRule(super::access_rules::mutation::CreateAccessRule),
    DeleteAccessRule(super::access_rules::mutation::DeleteAccessRule),
    RenameAccessRule(super::access_rules::mutation::RenameAccessRule),
    ReplaceAccessRule(super::access_rules::mutation::ReplaceAccessRule),
    CreatePrivacyRequirement(super::privacy::mutation::CreatePrivacyRequirement),
    DeletePrivacyRequirement(super::privacy::mutation::DeletePrivacyRequirement),
    RenamePrivacyRequirement(super::privacy::mutation::RenamePrivacyRequirement),
    ReplacePrivacyRequirement(super::privacy::mutation::ReplacePrivacyRequirement),
    CreateRelationship(super::relationships::mutation::CreateRelationship),
    DeleteRelationship(super::relationships::mutation::DeleteRelationship),
    RenameRelationship(super::relationships::mutation::RenameRelationship),
    ReplaceRelationship(super::relationships::mutation::ReplaceRelationship),
    CreateQuantityRequirement(super::quantities::mutation::CreateQuantityRequirement),
    DeleteQuantityRequirement(super::quantities::mutation::DeleteQuantityRequirement),
    RenameQuantityRequirement(super::quantities::mutation::RenameQuantityRequirement),
    ReplaceQuantityRequirement(super::quantities::mutation::ReplaceQuantityRequirement),
    CreateAnalysisRecord(super::analyses::mutation::CreateAnalysisRecord),
    DeleteAnalysisRecord(super::analyses::mutation::DeleteAnalysisRecord),
    RenameAnalysisRecord(super::analyses::mutation::RenameAnalysisRecord),
    ReplaceAnalysisRecord(super::analyses::mutation::ReplaceAnalysisRecord),
    CreateStorageRequirement(super::storage::mutation::CreateStorageRequirement),
    DeleteStorageRequirement(super::storage::mutation::DeleteStorageRequirement),
    RenameStorageRequirement(super::storage::mutation::RenameStorageRequirement),
    ReplaceStorageRequirement(super::storage::mutation::ReplaceStorageRequirement),
    CreateMeetingRecord(super::meetings::mutation::CreateMeetingRecord),
    DeleteMeetingRecord(super::meetings::mutation::DeleteMeetingRecord),
    RenameMeetingRecord(super::meetings::mutation::RenameMeetingRecord),
    ReplaceMeetingRecord(super::meetings::mutation::ReplaceMeetingRecord),
    CreateSurvey(super::surveys::mutation::CreateSurvey),
    DeleteSurvey(super::surveys::mutation::DeleteSurvey),
    RenameSurvey(super::surveys::mutation::RenameSurvey),
    ReplaceSurvey(super::surveys::mutation::ReplaceSurvey),
    CreateDeliveryConstraint(super::delivery::mutation::CreateDeliveryConstraint),
    DeleteDeliveryConstraint(super::delivery::mutation::DeleteDeliveryConstraint),
    RenameDeliveryConstraint(super::delivery::mutation::RenameDeliveryConstraint),
    ReplaceDeliveryConstraint(super::delivery::mutation::ReplaceDeliveryConstraint),
    CreateConstraintRecord(super::constraints::mutation::CreateConstraintRecord),
    DeleteConstraintRecord(super::constraints::mutation::DeleteConstraintRecord),
    RenameConstraintRecord(super::constraints::mutation::RenameConstraintRecord),
    ReplaceConstraintRecord(super::constraints::mutation::ReplaceConstraintRecord),
    CreateComplianceRecord(super::compliance_records::mutation::CreateComplianceRecord),
    DeleteComplianceRecord(super::compliance_records::mutation::DeleteComplianceRecord),
    RenameComplianceRecord(super::compliance_records::mutation::RenameComplianceRecord),
    ReplaceComplianceRecord(super::compliance_records::mutation::ReplaceComplianceRecord),
    CreateServiceRequirement(super::services::mutation::CreateServiceRequirement),
    DeleteServiceRequirement(super::services::mutation::DeleteServiceRequirement),
    RenameServiceRequirement(super::services::mutation::RenameServiceRequirement),
    ReplaceServiceRequirement(super::services::mutation::ReplaceServiceRequirement),
    CreateEquipment(super::equipment::mutation::CreateEquipment),
    DeleteEquipment(super::equipment::mutation::DeleteEquipment),
    RenameEquipment(super::equipment::mutation::RenameEquipment),
    ReplaceEquipment(super::equipment::mutation::ReplaceEquipment),
    CreateSecurityRequirement(super::security::mutation::CreateSecurityRequirement),
    DeleteSecurityRequirement(super::security::mutation::DeleteSecurityRequirement),
    RenameSecurityRequirement(super::security::mutation::RenameSecurityRequirement),
    ReplaceSecurityRequirement(super::security::mutation::ReplaceSecurityRequirement),
    CreateCollaborationRecord(super::collaboration::mutation::CreateCollaborationRecord),
    DeleteCollaborationRecord(super::collaboration::mutation::DeleteCollaborationRecord),
    RenameCollaborationRecord(super::collaboration::mutation::RenameCollaborationRecord),
    ReplaceCollaborationRecord(super::collaboration::mutation::ReplaceCollaborationRecord),
    CreateSafetyRequirement(super::safety::mutation::CreateSafetyRequirement),
    DeleteSafetyRequirement(super::safety::mutation::DeleteSafetyRequirement),
    RenameSafetyRequirement(super::safety::mutation::RenameSafetyRequirement),
    ReplaceSafetyRequirement(super::safety::mutation::ReplaceSafetyRequirement),
    CreateUserProfile(super::users::mutation::CreateUserProfile),
    DeleteUserProfile(super::users::mutation::DeleteUserProfile),
    RenameUserProfile(super::users::mutation::RenameUserProfile),
    ReplaceUserProfile(super::users::mutation::ReplaceUserProfile),
    CreateHumanFactorRequirement(super::human_factors::mutation::CreateHumanFactorRequirement),
    DeleteHumanFactorRequirement(super::human_factors::mutation::DeleteHumanFactorRequirement),
    RenameHumanFactorRequirement(super::human_factors::mutation::RenameHumanFactorRequirement),
    ReplaceHumanFactorRequirement(super::human_factors::mutation::ReplaceHumanFactorRequirement),
    CreateFlexibilityRequirement(super::flexibility::mutation::CreateFlexibilityRequirement),
    DeleteFlexibilityRequirement(super::flexibility::mutation::DeleteFlexibilityRequirement),
    RenameFlexibilityRequirement(super::flexibility::mutation::RenameFlexibilityRequirement),
    ReplaceFlexibilityRequirement(super::flexibility::mutation::ReplaceFlexibilityRequirement),
    CreateWayfindingRequirement(super::wayfinding::mutation::CreateWayfindingRequirement),
    DeleteWayfindingRequirement(super::wayfinding::mutation::DeleteWayfindingRequirement),
    RenameWayfindingRequirement(super::wayfinding::mutation::RenameWayfindingRequirement),
    ReplaceWayfindingRequirement(super::wayfinding::mutation::ReplaceWayfindingRequirement),
    CreateProgramElement(super::elements::mutation::CreateProgramElement),
    DeleteProgramElement(super::elements::mutation::DeleteProgramElement),
    RenameProgramElement(super::elements::mutation::RenameProgramElement),
    ReplaceProgramElement(super::elements::mutation::ReplaceProgramElement),
    ConnectAdjacency(super::set_adjacency::mutation::ConnectAdjacency),
    DisconnectAdjacency(super::clear_adjacency::mutation::DisconnectAdjacency),
    ConnectTrace(super::traces::mutation::ConnectTrace),
    DisconnectTrace(super::traces::mutation::DisconnectTrace),
    RenameMeta(super::update_meta::mutation::RenameMeta),
    ReplaceMeta(super::update_meta::mutation::ReplaceMeta),
    RenameProject(super::update_project::mutation::RenameProject),
    ReplaceProject(super::update_project::mutation::ReplaceProject),
    RenameGovernance(super::update_governance::mutation::RenameGovernance),
    ReplaceGovernance(super::update_governance::mutation::ReplaceGovernance),
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

        let create = ProgramMutation::CreateStakeholder(super::stakeholders::mutation::CreateStakeholder { stakeholder: new_stakeholder });
        let with_new = round_trip(&snapshot, &create);
        assert_eq!(with_new.stakeholders.len(), snapshot.stakeholders.len() + 1);

        let rename = ProgramMutation::RenameStakeholder(super::stakeholders::mutation::RenameStakeholder { id: new_id.clone(), new_name: "Renamed".into() });
        let renamed = round_trip(&with_new, &rename);
        assert_eq!(renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().header.name, "Renamed");

        let mut replacement = renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().clone();
        replacement.role = "Sponsor".into();
        let replace = ProgramMutation::ReplaceStakeholder(super::stakeholders::mutation::ReplaceStakeholder { stakeholder: replacement });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().role, "Sponsor");

        let delete = ProgramMutation::DeleteStakeholder(super::stakeholders::mutation::DeleteStakeholder { id: new_id });
        let deleted = round_trip(&replaced, &delete);
        assert_eq!(deleted.stakeholders.len(), snapshot.stakeholders.len());
    }

    #[test]
    fn delete_stakeholder_of_a_missing_id_has_an_empty_inverse() {
        let snapshot = sample_plugin();
        let delete = ProgramMutation::DeleteStakeholder(super::stakeholders::mutation::DeleteStakeholder { id: EntityId("nope".into()) });
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

        let create = ProgramMutation::CreateProgramElement(super::elements::mutation::CreateProgramElement { program_element: new_element });
        let with_new = round_trip(&snapshot, &create);
        assert_eq!(with_new.elements.len(), snapshot.elements.len() + 1);

        let rename = ProgramMutation::RenameProgramElement(super::elements::mutation::RenameProgramElement { id: new_id.clone(), new_name: "Storage Room".into() });
        let renamed = round_trip(&with_new, &rename);
        assert_eq!(renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().header.name, "Storage Room");

        let mut replacement = renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().clone();
        replacement.code = "STO".into();
        let replace = ProgramMutation::ReplaceProgramElement(super::elements::mutation::ReplaceProgramElement { program_element: replacement });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.elements.iter().find(|e| e.header.id == new_id).unwrap().code, "STO");

        let delete = ProgramMutation::DeleteProgramElement(super::elements::mutation::DeleteProgramElement { id: new_id });
        let deleted = round_trip(&replaced, &delete);
        assert_eq!(deleted.elements.len(), snapshot.elements.len());
    }
    //#endregion 🧱elements

    //#region 🏷️📁🏛️meta-project-governance
    #[test]
    fn update_meta_rename_and_replace_round_trip() {
        let snapshot = empty_plugin();
        let rename = ProgramMutation::RenameMeta(super::update_meta::mutation::RenameMeta { new_title: "Clinic".into() });
        let renamed = round_trip(&snapshot, &rename);
        assert_eq!(renamed.meta.title, "Clinic");

        let mut new_meta = renamed.meta.clone();
        new_meta.industry_sector = "healthcare".into();
        let replace = ProgramMutation::ReplaceMeta(super::update_meta::mutation::ReplaceMeta { new_meta });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.meta.industry_sector, "healthcare");
    }

    #[test]
    fn update_project_rename_and_replace_round_trip() {
        let snapshot = empty_plugin();
        let rename = ProgramMutation::RenameProject(super::update_project::mutation::RenameProject { new_code: "CLN-001".into() });
        let renamed = round_trip(&snapshot, &rename);
        assert_eq!(renamed.project.code, "CLN-001");

        let mut new_project = renamed.project.clone();
        new_project.client_name = "Sample Health".into();
        let replace = ProgramMutation::ReplaceProject(super::update_project::mutation::ReplaceProject { new_project });
        let replaced = round_trip(&renamed, &replace);
        assert_eq!(replaced.project.client_name, "Sample Health");
    }

    #[test]
    fn update_governance_rename_and_replace_round_trip() {
        let snapshot = empty_plugin();
        let rename = ProgramMutation::RenameGovernance(super::update_governance::mutation::RenameGovernance { new_framework: "ISO 41001".into() });
        let renamed = round_trip(&snapshot, &rename);
        assert_eq!(renamed.governance.framework, "ISO 41001");

        let mut new_governance = renamed.governance.clone();
        new_governance.risk_appetite = Some("Low".into());
        let replace = ProgramMutation::ReplaceGovernance(super::update_governance::mutation::ReplaceGovernance { new_governance });
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
        let connect = ProgramMutation::ConnectAdjacency(super::set_adjacency::mutation::ConnectAdjacency { adjacency: new_adjacency });
        let connected = round_trip(&snapshot, &connect);
        assert_eq!(connected.adjacencies.len(), snapshot.adjacencies.len() + 1);

        let disconnect = ProgramMutation::DisconnectAdjacency(super::clear_adjacency::mutation::DisconnectAdjacency { id: new_id });
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
        let connect = ProgramMutation::ConnectAdjacency(super::set_adjacency::mutation::ConnectAdjacency { adjacency: updated });
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
        let connect = ProgramMutation::ConnectTrace(super::traces::mutation::ConnectTrace { trace });
        let connected = round_trip(&snapshot, &connect);
        assert_eq!(connected.traces.len(), 1);

        let disconnect = ProgramMutation::DisconnectTrace(super::traces::mutation::DisconnectTrace { id });
        let disconnected = round_trip(&connected, &disconnect);
        assert!(disconnected.traces.is_empty());
    }
    //#endregion 🧵connect-disconnect-trace

    //#region 🗣️OpText
    #[test]
    fn program_mutation_op_text_round_trips_a_sample_of_variants() {
        let stakeholder = sample_plugin().stakeholders[0].clone();
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::CreateStakeholder(super::stakeholders::mutation::CreateStakeholder { stakeholder: stakeholder.clone() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DeleteStakeholder(super::stakeholders::mutation::DeleteStakeholder { id: stakeholder.header.id.clone() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameStakeholder(super::stakeholders::mutation::RenameStakeholder { id: stakeholder.header.id.clone(), new_name: "X".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::ReplaceStakeholder(super::stakeholders::mutation::ReplaceStakeholder { stakeholder }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameMeta(super::update_meta::mutation::RenameMeta { new_title: "Clinic".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DisconnectAdjacency(super::clear_adjacency::mutation::DisconnectAdjacency { id: EntityId("a1".into()) }));
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
        let create = ProgramMutation::CreateStakeholder(super::stakeholders::mutation::CreateStakeholder { stakeholder: new_stakeholder.clone() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &create);
        let d1 = create.diff(&base);
        let after = d1.apply(&base);
        let d2 = ProgramMutation::RenameStakeholder(super::stakeholders::mutation::RenameStakeholder { id: new_stakeholder.header.id, new_name: "Renamed".into() }).diff(&after);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn rename_meta_obeys_the_inverse_law() {
        let base = sample_plugin();
        let rename = ProgramMutation::RenameMeta(super::update_meta::mutation::RenameMeta { new_title: "Renamed Program".into() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &rename);
    }

    #[test]
    fn connect_adjacency_obeys_the_inverse_law() {
        let base = sample_plugin();
        let mut updated = base.adjacencies[0].clone();
        updated.weight = 9.0;
        let connect = ProgramMutation::ConnectAdjacency(super::set_adjacency::mutation::ConnectAdjacency { adjacency: updated });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &connect);
    }
    //#endregion ⚖️SemanticLaws

    //#region 📋️DescriptorLaws
    #[test]
    fn semantic_kinds_cover_every_variant() {
        assert_eq!(ProgramMutation::kinds().len(), 266);
        let stakeholder = sample_plugin().stakeholders[0].clone();
        let mutation = ProgramMutation::RenameStakeholder(super::stakeholders::mutation::RenameStakeholder { id: stakeholder.header.id, new_name: "X".into() });
        assert_eq!(mutation.semantics().kind, "rename-stakeholder");
        assert_eq!(mutation.semantics().record, "RenamedStakeholder");
    }
    //#endregion 📋️DescriptorLaws
}
//#endregion 🧪️Tests
