/** 🗃️ ProgramSnapshot diff schema: the sparse deltas of the risk, decision, quality, compliance, collaboration, audit, knowledge and benchmark registers and of the three singleton edits. */
import type { ProgramMeta, ProjectDefinition, TraceLink } from "../../🪪️entity-foundation/🟦️.ts";
import type { AnalysisRecord, ApprovalRecord, ArtifactRecord, Assumption, AuditEvent, BenchmarkRecord, ChangeRecord, CollaborationRecord, ComplianceRecord, Conflict, ConstraintRecord, Decision, Governance, Issue, KnowledgeRecord, MeetingRecord, OptionEvaluation, PerformanceCriterion, PriorityRecord, QualityRecord, ReportRecord, Requirement, Risk, Scenario, SearchFilter, StatusRecord, Survey, TemplateRecord, ValidationRecord, Workshop } from "../../🗃️decision-records/🟦️.ts";
import type { AnalysisRecordPatch, ApprovalRecordPatch, ArtifactRecordPatch, AssumptionPatch, AuditEventPatch, BenchmarkRecordPatch, ChangeRecordPatch, CollaborationRecordPatch, ComplianceRecordPatch, ConflictPatch, ConstraintRecordPatch, DecisionPatch, GovernancePatch, IssuePatch, KnowledgeRecordPatch, MeetingRecordPatch, OptionEvaluationPatch, PerformanceCriterionPatch, PriorityRecordPatch, ProgramMetaPatch, ProjectDefinitionPatch, QualityRecordPatch, ReportRecordPatch, RequirementPatch, RiskPatch, ScenarioPatch, SearchFilterPatch, StatusRecordPatch, SurveyPatch, TemplateRecordPatch, TraceLinkPatch, ValidationRecordPatch, WorkshopPatch } from "../🩹️patch-types/🟦️.ts";

export interface ProgramRisksDelta {
  removed: ProgramRisksRemoval[];
  inserted: ProgramRisksInsertion[];
  moved: ProgramRisksRelocation[];
  modified: ProgramRisksPatchEntry[];
}

export interface ProgramRisksInsertion {
  index: number;
  row: Risk;
}

export interface ProgramRisksRemoval {
  id: string;
  index: number;
}

export interface ProgramRisksRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramRisksPatchEntry {
  id: string;
  patch: RiskPatch;
}

export interface ProgramConflictsDelta {
  removed: ProgramConflictsRemoval[];
  inserted: ProgramConflictsInsertion[];
  moved: ProgramConflictsRelocation[];
  modified: ProgramConflictsPatchEntry[];
}

export interface ProgramConflictsInsertion {
  index: number;
  row: Conflict;
}

export interface ProgramConflictsRemoval {
  id: string;
  index: number;
}

export interface ProgramConflictsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramConflictsPatchEntry {
  id: string;
  patch: ConflictPatch;
}

export interface ProgramRequirementsDelta {
  removed: ProgramRequirementsRemoval[];
  inserted: ProgramRequirementsInsertion[];
  moved: ProgramRequirementsRelocation[];
  modified: ProgramRequirementsPatchEntry[];
}

export interface ProgramRequirementsInsertion {
  index: number;
  row: Requirement;
}

export interface ProgramRequirementsRemoval {
  id: string;
  index: number;
}

export interface ProgramRequirementsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramRequirementsPatchEntry {
  id: string;
  patch: RequirementPatch;
}

export interface ProgramPrioritiesDelta {
  removed: ProgramPrioritiesRemoval[];
  inserted: ProgramPrioritiesInsertion[];
  moved: ProgramPrioritiesRelocation[];
  modified: ProgramPrioritiesPatchEntry[];
}

export interface ProgramPrioritiesInsertion {
  index: number;
  row: PriorityRecord;
}

export interface ProgramPrioritiesRemoval {
  id: string;
  index: number;
}

export interface ProgramPrioritiesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramPrioritiesPatchEntry {
  id: string;
  patch: PriorityRecordPatch;
}

export interface ProgramScenariosDelta {
  removed: ProgramScenariosRemoval[];
  inserted: ProgramScenariosInsertion[];
  moved: ProgramScenariosRelocation[];
  modified: ProgramScenariosPatchEntry[];
}

export interface ProgramScenariosInsertion {
  index: number;
  row: Scenario;
}

export interface ProgramScenariosRemoval {
  id: string;
  index: number;
}

export interface ProgramScenariosRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramScenariosPatchEntry {
  id: string;
  patch: ScenarioPatch;
}

export interface ProgramOptionsDelta {
  removed: ProgramOptionsRemoval[];
  inserted: ProgramOptionsInsertion[];
  moved: ProgramOptionsRelocation[];
  modified: ProgramOptionsPatchEntry[];
}

export interface ProgramOptionsInsertion {
  index: number;
  row: OptionEvaluation;
}

export interface ProgramOptionsRemoval {
  id: string;
  index: number;
}

export interface ProgramOptionsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramOptionsPatchEntry {
  id: string;
  patch: OptionEvaluationPatch;
}

export interface ProgramDecisionsDelta {
  removed: ProgramDecisionsRemoval[];
  inserted: ProgramDecisionsInsertion[];
  moved: ProgramDecisionsRelocation[];
  modified: ProgramDecisionsPatchEntry[];
}

export interface ProgramDecisionsInsertion {
  index: number;
  row: Decision;
}

export interface ProgramDecisionsRemoval {
  id: string;
  index: number;
}

export interface ProgramDecisionsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramDecisionsPatchEntry {
  id: string;
  patch: DecisionPatch;
}

export interface ProgramValidationsDelta {
  removed: ProgramValidationsRemoval[];
  inserted: ProgramValidationsInsertion[];
  moved: ProgramValidationsRelocation[];
  modified: ProgramValidationsPatchEntry[];
}

export interface ProgramValidationsInsertion {
  index: number;
  row: ValidationRecord;
}

export interface ProgramValidationsRemoval {
  id: string;
  index: number;
}

export interface ProgramValidationsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramValidationsPatchEntry {
  id: string;
  patch: ValidationRecordPatch;
}

export interface ProgramPerformanceDelta {
  removed: ProgramPerformanceRemoval[];
  inserted: ProgramPerformanceInsertion[];
  moved: ProgramPerformanceRelocation[];
  modified: ProgramPerformancePatchEntry[];
}

export interface ProgramPerformanceInsertion {
  index: number;
  row: PerformanceCriterion;
}

export interface ProgramPerformanceRemoval {
  id: string;
  index: number;
}

export interface ProgramPerformanceRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramPerformancePatchEntry {
  id: string;
  patch: PerformanceCriterionPatch;
}

export interface ProgramQualityDelta {
  removed: ProgramQualityRemoval[];
  inserted: ProgramQualityInsertion[];
  moved: ProgramQualityRelocation[];
  modified: ProgramQualityPatchEntry[];
}

export interface ProgramQualityInsertion {
  index: number;
  row: QualityRecord;
}

export interface ProgramQualityRemoval {
  id: string;
  index: number;
}

export interface ProgramQualityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramQualityPatchEntry {
  id: string;
  patch: QualityRecordPatch;
}

export interface ProgramArtifactsDelta {
  removed: ProgramArtifactsRemoval[];
  inserted: ProgramArtifactsInsertion[];
  moved: ProgramArtifactsRelocation[];
  modified: ProgramArtifactsPatchEntry[];
}

export interface ProgramArtifactsInsertion {
  index: number;
  row: ArtifactRecord;
}

export interface ProgramArtifactsRemoval {
  id: string;
  index: number;
}

export interface ProgramArtifactsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramArtifactsPatchEntry {
  id: string;
  patch: ArtifactRecordPatch;
}

export interface ProgramAssumptionsDelta {
  removed: ProgramAssumptionsRemoval[];
  inserted: ProgramAssumptionsInsertion[];
  moved: ProgramAssumptionsRelocation[];
  modified: ProgramAssumptionsPatchEntry[];
}

export interface ProgramAssumptionsInsertion {
  index: number;
  row: Assumption;
}

export interface ProgramAssumptionsRemoval {
  id: string;
  index: number;
}

export interface ProgramAssumptionsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAssumptionsPatchEntry {
  id: string;
  patch: AssumptionPatch;
}

export interface ProgramConstraintsDelta {
  removed: ProgramConstraintsRemoval[];
  inserted: ProgramConstraintsInsertion[];
  moved: ProgramConstraintsRelocation[];
  modified: ProgramConstraintsPatchEntry[];
}

export interface ProgramConstraintsInsertion {
  index: number;
  row: ConstraintRecord;
}

export interface ProgramConstraintsRemoval {
  id: string;
  index: number;
}

export interface ProgramConstraintsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramConstraintsPatchEntry {
  id: string;
  patch: ConstraintRecordPatch;
}

export interface ProgramComplianceRecordsDelta {
  removed: ProgramComplianceRecordsRemoval[];
  inserted: ProgramComplianceRecordsInsertion[];
  moved: ProgramComplianceRecordsRelocation[];
  modified: ProgramComplianceRecordsPatchEntry[];
}

export interface ProgramComplianceRecordsInsertion {
  index: number;
  row: ComplianceRecord;
}

export interface ProgramComplianceRecordsRemoval {
  id: string;
  index: number;
}

export interface ProgramComplianceRecordsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramComplianceRecordsPatchEntry {
  id: string;
  patch: ComplianceRecordPatch;
}

export interface ProgramApprovalsDelta {
  removed: ProgramApprovalsRemoval[];
  inserted: ProgramApprovalsInsertion[];
  moved: ProgramApprovalsRelocation[];
  modified: ProgramApprovalsPatchEntry[];
}

export interface ProgramApprovalsInsertion {
  index: number;
  row: ApprovalRecord;
}

export interface ProgramApprovalsRemoval {
  id: string;
  index: number;
}

export interface ProgramApprovalsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramApprovalsPatchEntry {
  id: string;
  patch: ApprovalRecordPatch;
}

export interface ProgramMeetingsDelta {
  removed: ProgramMeetingsRemoval[];
  inserted: ProgramMeetingsInsertion[];
  moved: ProgramMeetingsRelocation[];
  modified: ProgramMeetingsPatchEntry[];
}

export interface ProgramMeetingsInsertion {
  index: number;
  row: MeetingRecord;
}

export interface ProgramMeetingsRemoval {
  id: string;
  index: number;
}

export interface ProgramMeetingsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramMeetingsPatchEntry {
  id: string;
  patch: MeetingRecordPatch;
}

export interface ProgramChangesDelta {
  removed: ProgramChangesRemoval[];
  inserted: ProgramChangesInsertion[];
  moved: ProgramChangesRelocation[];
  modified: ProgramChangesPatchEntry[];
}

export interface ProgramChangesInsertion {
  index: number;
  row: ChangeRecord;
}

export interface ProgramChangesRemoval {
  id: string;
  index: number;
}

export interface ProgramChangesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramChangesPatchEntry {
  id: string;
  patch: ChangeRecordPatch;
}

export interface ProgramCollaborationDelta {
  removed: ProgramCollaborationRemoval[];
  inserted: ProgramCollaborationInsertion[];
  moved: ProgramCollaborationRelocation[];
  modified: ProgramCollaborationPatchEntry[];
}

export interface ProgramCollaborationInsertion {
  index: number;
  row: CollaborationRecord;
}

export interface ProgramCollaborationRemoval {
  id: string;
  index: number;
}

export interface ProgramCollaborationRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramCollaborationPatchEntry {
  id: string;
  patch: CollaborationRecordPatch;
}

export interface ProgramAnalysesDelta {
  removed: ProgramAnalysesRemoval[];
  inserted: ProgramAnalysesInsertion[];
  moved: ProgramAnalysesRelocation[];
  modified: ProgramAnalysesPatchEntry[];
}

export interface ProgramAnalysesInsertion {
  index: number;
  row: AnalysisRecord;
}

export interface ProgramAnalysesRemoval {
  id: string;
  index: number;
}

export interface ProgramAnalysesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAnalysesPatchEntry {
  id: string;
  patch: AnalysisRecordPatch;
}

export interface ProgramReportsDelta {
  removed: ProgramReportsRemoval[];
  inserted: ProgramReportsInsertion[];
  moved: ProgramReportsRelocation[];
  modified: ProgramReportsPatchEntry[];
}

export interface ProgramReportsInsertion {
  index: number;
  row: ReportRecord;
}

export interface ProgramReportsRemoval {
  id: string;
  index: number;
}

export interface ProgramReportsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramReportsPatchEntry {
  id: string;
  patch: ReportRecordPatch;
}

export interface ProgramSearchFiltersDelta {
  removed: ProgramSearchFiltersRemoval[];
  inserted: ProgramSearchFiltersInsertion[];
  moved: ProgramSearchFiltersRelocation[];
  modified: ProgramSearchFiltersPatchEntry[];
}

export interface ProgramSearchFiltersInsertion {
  index: number;
  row: SearchFilter;
}

export interface ProgramSearchFiltersRemoval {
  id: string;
  index: number;
}

export interface ProgramSearchFiltersRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSearchFiltersPatchEntry {
  id: string;
  patch: SearchFilterPatch;
}

export interface ProgramStatusRecordsDelta {
  removed: ProgramStatusRecordsRemoval[];
  inserted: ProgramStatusRecordsInsertion[];
  moved: ProgramStatusRecordsRelocation[];
  modified: ProgramStatusRecordsPatchEntry[];
}

export interface ProgramStatusRecordsInsertion {
  index: number;
  row: StatusRecord;
}

export interface ProgramStatusRecordsRemoval {
  id: string;
  index: number;
}

export interface ProgramStatusRecordsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramStatusRecordsPatchEntry {
  id: string;
  patch: StatusRecordPatch;
}

export interface ProgramWorkshopsDelta {
  removed: ProgramWorkshopsRemoval[];
  inserted: ProgramWorkshopsInsertion[];
  moved: ProgramWorkshopsRelocation[];
  modified: ProgramWorkshopsPatchEntry[];
}

export interface ProgramWorkshopsInsertion {
  index: number;
  row: Workshop;
}

export interface ProgramWorkshopsRemoval {
  id: string;
  index: number;
}

export interface ProgramWorkshopsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramWorkshopsPatchEntry {
  id: string;
  patch: WorkshopPatch;
}

export interface ProgramSurveysDelta {
  removed: ProgramSurveysRemoval[];
  inserted: ProgramSurveysInsertion[];
  moved: ProgramSurveysRelocation[];
  modified: ProgramSurveysPatchEntry[];
}

export interface ProgramSurveysInsertion {
  index: number;
  row: Survey;
}

export interface ProgramSurveysRemoval {
  id: string;
  index: number;
}

export interface ProgramSurveysRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSurveysPatchEntry {
  id: string;
  patch: SurveyPatch;
}

export interface ProgramIssuesDelta {
  removed: ProgramIssuesRemoval[];
  inserted: ProgramIssuesInsertion[];
  moved: ProgramIssuesRelocation[];
  modified: ProgramIssuesPatchEntry[];
}

export interface ProgramIssuesInsertion {
  index: number;
  row: Issue;
}

export interface ProgramIssuesRemoval {
  id: string;
  index: number;
}

export interface ProgramIssuesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramIssuesPatchEntry {
  id: string;
  patch: IssuePatch;
}

export interface ProgramAuditEventsDelta {
  removed: ProgramAuditEventsRemoval[];
  inserted: ProgramAuditEventsInsertion[];
  moved: ProgramAuditEventsRelocation[];
  modified: ProgramAuditEventsPatchEntry[];
}

export interface ProgramAuditEventsInsertion {
  index: number;
  row: AuditEvent;
}

export interface ProgramAuditEventsRemoval {
  id: string;
  index: number;
}

export interface ProgramAuditEventsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAuditEventsPatchEntry {
  id: string;
  patch: AuditEventPatch;
}

export interface ProgramTemplatesDelta {
  removed: ProgramTemplatesRemoval[];
  inserted: ProgramTemplatesInsertion[];
  moved: ProgramTemplatesRelocation[];
  modified: ProgramTemplatesPatchEntry[];
}

export interface ProgramTemplatesInsertion {
  index: number;
  row: TemplateRecord;
}

export interface ProgramTemplatesRemoval {
  id: string;
  index: number;
}

export interface ProgramTemplatesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramTemplatesPatchEntry {
  id: string;
  patch: TemplateRecordPatch;
}

export interface ProgramTracesDelta {
  removed: ProgramTracesRemoval[];
  inserted: ProgramTracesInsertion[];
  moved: ProgramTracesRelocation[];
  modified: ProgramTracesPatchEntry[];
}

export interface ProgramTracesInsertion {
  index: number;
  row: TraceLink;
}

export interface ProgramTracesRemoval {
  id: string;
  index: number;
}

export interface ProgramTracesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramTracesPatchEntry {
  id: string;
  patch: TraceLinkPatch;
}

export interface ProgramKnowledgeDelta {
  removed: ProgramKnowledgeRemoval[];
  inserted: ProgramKnowledgeInsertion[];
  moved: ProgramKnowledgeRelocation[];
  modified: ProgramKnowledgePatchEntry[];
}

export interface ProgramKnowledgeInsertion {
  index: number;
  row: KnowledgeRecord;
}

export interface ProgramKnowledgeRemoval {
  id: string;
  index: number;
}

export interface ProgramKnowledgeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramKnowledgePatchEntry {
  id: string;
  patch: KnowledgeRecordPatch;
}

export interface ProgramBenchmarksDelta {
  removed: ProgramBenchmarksRemoval[];
  inserted: ProgramBenchmarksInsertion[];
  moved: ProgramBenchmarksRelocation[];
  modified: ProgramBenchmarksPatchEntry[];
}

export interface ProgramBenchmarksInsertion {
  index: number;
  row: BenchmarkRecord;
}

export interface ProgramBenchmarksRemoval {
  id: string;
  index: number;
}

export interface ProgramBenchmarksRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramBenchmarksPatchEntry {
  id: string;
  patch: BenchmarkRecordPatch;
}

export interface ProgramMetaEdit {
  set: ProgramMeta | null;
  patch: ProgramMetaPatch | null;
}

export interface ProjectDefinitionEdit {
  set: ProjectDefinition | null;
  patch: ProjectDefinitionPatch | null;
}

export interface GovernanceEdit {
  set: Governance | null;
  patch: GovernancePatch | null;
}
