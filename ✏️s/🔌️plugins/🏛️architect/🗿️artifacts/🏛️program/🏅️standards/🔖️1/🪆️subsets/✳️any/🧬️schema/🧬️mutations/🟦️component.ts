/** 🧩 ProgramMutation dispatch union — mirrors 🦀️component.rs's `ProgramMutation` enum. Internally
 *  tagged (`#[serde(tag = "mutation", rename_all = "camelCase")]`): each payload struct's own
 *  fields flatten onto the tag, matching every committed `🧪️tests/<case>/🦠️mutation/component.json`
 *  fixture — there is no nested `payload` property. */
import type {
  AccessRule,
  AccessibilityRequirement,
  Activity,
  Adjacency,
  AnalysisRecord,
  ApprovalRecord,
  Assumption,
  AuditEvent,
  BenchmarkRecord,
  ChangeRecord,
  CollaborationRecord,
  CommunicationRequirement,
  ComplianceRecord,
  Conflict,
  ConstraintRecord,
  CostRequirement,
  Decision,
  DeliveryConstraint,
  DocumentRecord,
  EnvironmentalRequirement,
  Equipment,
  FlexibilityRequirement,
  FlowRequirement,
  Function as FunctionEntity,
  Governance,
  GrowthPlan,
  HumanFactorRequirement,
  InformationRequirement,
  InfrastructureRequirement,
  Issue,
  KnowledgeRecord,
  MeetingRecord,
  OperationalRequirement,
  OptionEvaluation,
  OrganizationalRequirement,
  PerformanceCriterion,
  PriorityRecord,
  PrivacyRequirement,
  Process,
  ProgramElement,
  ProgramMeta,
  ProjectDefinition,
  QualityRecord,
  QuantityRequirement,
  RegulatoryRequirement,
  Relationship,
  ReportRecord,
  Requirement,
  ResilienceRequirement,
  Resource,
  Risk,
  SafetyRequirement,
  Scenario,
  ScheduleRequirement,
  SearchFilter,
  SecurityRequirement,
  ServiceRequirement,
  SiteContext,
  Stakeholder,
  StatusRecord,
  StorageRequirement,
  Survey,
  SustainabilityRequirement,
  TemplateRecord,
  TraceLink,
  UserProfile,
  ValidationRecord,
  WayfindingRequirement,
  Workshop,
} from "../🟦️component.ts";

export interface CreateInformationRequirement { informationRequirement: InformationRequirement; }
export interface CreateInformationRequirementOp extends CreateInformationRequirement { mutation: "createInformationRequirement"; }
export interface DeleteInformationRequirement { id: string; }
export interface DeleteInformationRequirementOp extends DeleteInformationRequirement { mutation: "deleteInformationRequirement"; }
export interface RenameInformationRequirement { id: string; newName: string; }
export interface RenameInformationRequirementOp extends RenameInformationRequirement { mutation: "renameInformationRequirement"; }
export interface ReplaceInformationRequirement { informationRequirement: InformationRequirement; }
export interface ReplaceInformationRequirementOp extends ReplaceInformationRequirement { mutation: "replaceInformationRequirement"; }
export interface CreateSustainabilityRequirement { sustainabilityRequirement: SustainabilityRequirement; }
export interface CreateSustainabilityRequirementOp extends CreateSustainabilityRequirement { mutation: "createSustainabilityRequirement"; }
export interface DeleteSustainabilityRequirement { id: string; }
export interface DeleteSustainabilityRequirementOp extends DeleteSustainabilityRequirement { mutation: "deleteSustainabilityRequirement"; }
export interface RenameSustainabilityRequirement { id: string; newName: string; }
export interface RenameSustainabilityRequirementOp extends RenameSustainabilityRequirement { mutation: "renameSustainabilityRequirement"; }
export interface ReplaceSustainabilityRequirement { sustainabilityRequirement: SustainabilityRequirement; }
export interface ReplaceSustainabilityRequirementOp extends ReplaceSustainabilityRequirement { mutation: "replaceSustainabilityRequirement"; }
export interface CreateAccessibilityRequirement { accessibilityRequirement: AccessibilityRequirement; }
export interface CreateAccessibilityRequirementOp extends CreateAccessibilityRequirement { mutation: "createAccessibilityRequirement"; }
export interface DeleteAccessibilityRequirement { id: string; }
export interface DeleteAccessibilityRequirementOp extends DeleteAccessibilityRequirement { mutation: "deleteAccessibilityRequirement"; }
export interface RenameAccessibilityRequirement { id: string; newName: string; }
export interface RenameAccessibilityRequirementOp extends RenameAccessibilityRequirement { mutation: "renameAccessibilityRequirement"; }
export interface ReplaceAccessibilityRequirement { accessibilityRequirement: AccessibilityRequirement; }
export interface ReplaceAccessibilityRequirementOp extends ReplaceAccessibilityRequirement { mutation: "replaceAccessibilityRequirement"; }
export interface CreateConflict { conflict: Conflict; }
export interface CreateConflictOp extends CreateConflict { mutation: "createConflict"; }
export interface DeleteConflict { id: string; }
export interface DeleteConflictOp extends DeleteConflict { mutation: "deleteConflict"; }
export interface RenameConflict { id: string; newName: string; }
export interface RenameConflictOp extends RenameConflict { mutation: "renameConflict"; }
export interface ReplaceConflict { conflict: Conflict; }
export interface ReplaceConflictOp extends ReplaceConflict { mutation: "replaceConflict"; }
export interface CreateOptionEvaluation { optionEvaluation: OptionEvaluation; }
export interface CreateOptionEvaluationOp extends CreateOptionEvaluation { mutation: "createOptionEvaluation"; }
export interface DeleteOptionEvaluation { id: string; }
export interface DeleteOptionEvaluationOp extends DeleteOptionEvaluation { mutation: "deleteOptionEvaluation"; }
export interface RenameOptionEvaluation { id: string; newName: string; }
export interface RenameOptionEvaluationOp extends RenameOptionEvaluation { mutation: "renameOptionEvaluation"; }
export interface ReplaceOptionEvaluation { optionEvaluation: OptionEvaluation; }
export interface ReplaceOptionEvaluationOp extends ReplaceOptionEvaluation { mutation: "replaceOptionEvaluation"; }
export interface CreateFunction { function: FunctionEntity; }
export interface CreateFunctionOp extends CreateFunction { mutation: "createFunction"; }
export interface DeleteFunction { id: string; }
export interface DeleteFunctionOp extends DeleteFunction { mutation: "deleteFunction"; }
export interface RenameFunction { id: string; newName: string; }
export interface RenameFunctionOp extends RenameFunction { mutation: "renameFunction"; }
export interface ReplaceFunction { function: FunctionEntity; }
export interface ReplaceFunctionOp extends ReplaceFunction { mutation: "replaceFunction"; }
export interface CreateRisk { risk: Risk; }
export interface CreateRiskOp extends CreateRisk { mutation: "createRisk"; }
export interface DeleteRisk { id: string; }
export interface DeleteRiskOp extends DeleteRisk { mutation: "deleteRisk"; }
export interface RenameRisk { id: string; newName: string; }
export interface RenameRiskOp extends RenameRisk { mutation: "renameRisk"; }
export interface ReplaceRisk { risk: Risk; }
export interface ReplaceRiskOp extends ReplaceRisk { mutation: "replaceRisk"; }
export interface CreateDecision { decision: Decision; }
export interface CreateDecisionOp extends CreateDecision { mutation: "createDecision"; }
export interface DeleteDecision { id: string; }
export interface DeleteDecisionOp extends DeleteDecision { mutation: "deleteDecision"; }
export interface RenameDecision { id: string; newName: string; }
export interface RenameDecisionOp extends RenameDecision { mutation: "renameDecision"; }
export interface ReplaceDecision { decision: Decision; }
export interface ReplaceDecisionOp extends ReplaceDecision { mutation: "replaceDecision"; }
export interface CreateValidationRecord { validationRecord: ValidationRecord; }
export interface CreateValidationRecordOp extends CreateValidationRecord { mutation: "createValidationRecord"; }
export interface DeleteValidationRecord { id: string; }
export interface DeleteValidationRecordOp extends DeleteValidationRecord { mutation: "deleteValidationRecord"; }
export interface RenameValidationRecord { id: string; newName: string; }
export interface RenameValidationRecordOp extends RenameValidationRecord { mutation: "renameValidationRecord"; }
export interface ReplaceValidationRecord { validationRecord: ValidationRecord; }
export interface ReplaceValidationRecordOp extends ReplaceValidationRecord { mutation: "replaceValidationRecord"; }
export interface CreatePriorityRecord { priorityRecord: PriorityRecord; }
export interface CreatePriorityRecordOp extends CreatePriorityRecord { mutation: "createPriorityRecord"; }
export interface DeletePriorityRecord { id: string; }
export interface DeletePriorityRecordOp extends DeletePriorityRecord { mutation: "deletePriorityRecord"; }
export interface RenamePriorityRecord { id: string; newName: string; }
export interface RenamePriorityRecordOp extends RenamePriorityRecord { mutation: "renamePriorityRecord"; }
export interface ReplacePriorityRecord { priorityRecord: PriorityRecord; }
export interface ReplacePriorityRecordOp extends ReplacePriorityRecord { mutation: "replacePriorityRecord"; }
export interface CreateFlowRequirement { flowRequirement: FlowRequirement; }
export interface CreateFlowRequirementOp extends CreateFlowRequirement { mutation: "createFlowRequirement"; }
export interface DeleteFlowRequirement { id: string; }
export interface DeleteFlowRequirementOp extends DeleteFlowRequirement { mutation: "deleteFlowRequirement"; }
export interface RenameFlowRequirement { id: string; newName: string; }
export interface RenameFlowRequirementOp extends RenameFlowRequirement { mutation: "renameFlowRequirement"; }
export interface ReplaceFlowRequirement { flowRequirement: FlowRequirement; }
export interface ReplaceFlowRequirementOp extends ReplaceFlowRequirement { mutation: "replaceFlowRequirement"; }
export interface CreateEnvironmentalRequirement { environmentalRequirement: EnvironmentalRequirement; }
export interface CreateEnvironmentalRequirementOp extends CreateEnvironmentalRequirement { mutation: "createEnvironmentalRequirement"; }
export interface DeleteEnvironmentalRequirement { id: string; }
export interface DeleteEnvironmentalRequirementOp extends DeleteEnvironmentalRequirement { mutation: "deleteEnvironmentalRequirement"; }
export interface RenameEnvironmentalRequirement { id: string; newName: string; }
export interface RenameEnvironmentalRequirementOp extends RenameEnvironmentalRequirement { mutation: "renameEnvironmentalRequirement"; }
export interface ReplaceEnvironmentalRequirement { environmentalRequirement: EnvironmentalRequirement; }
export interface ReplaceEnvironmentalRequirementOp extends ReplaceEnvironmentalRequirement { mutation: "replaceEnvironmentalRequirement"; }
export interface CreateWorkshop { workshop: Workshop; }
export interface CreateWorkshopOp extends CreateWorkshop { mutation: "createWorkshop"; }
export interface DeleteWorkshop { id: string; }
export interface DeleteWorkshopOp extends DeleteWorkshop { mutation: "deleteWorkshop"; }
export interface RenameWorkshop { id: string; newName: string; }
export interface RenameWorkshopOp extends RenameWorkshop { mutation: "renameWorkshop"; }
export interface ReplaceWorkshop { workshop: Workshop; }
export interface ReplaceWorkshopOp extends ReplaceWorkshop { mutation: "replaceWorkshop"; }
export interface CreateScenario { scenario: Scenario; }
export interface CreateScenarioOp extends CreateScenario { mutation: "createScenario"; }
export interface DeleteScenario { id: string; }
export interface DeleteScenarioOp extends DeleteScenario { mutation: "deleteScenario"; }
export interface RenameScenario { id: string; newName: string; }
export interface RenameScenarioOp extends RenameScenario { mutation: "renameScenario"; }
export interface ReplaceScenario { scenario: Scenario; }
export interface ReplaceScenarioOp extends ReplaceScenario { mutation: "replaceScenario"; }
export interface CreateBenchmarkRecord { benchmarkRecord: BenchmarkRecord; }
export interface CreateBenchmarkRecordOp extends CreateBenchmarkRecord { mutation: "createBenchmarkRecord"; }
export interface DeleteBenchmarkRecord { id: string; }
export interface DeleteBenchmarkRecordOp extends DeleteBenchmarkRecord { mutation: "deleteBenchmarkRecord"; }
export interface RenameBenchmarkRecord { id: string; newName: string; }
export interface RenameBenchmarkRecordOp extends RenameBenchmarkRecord { mutation: "renameBenchmarkRecord"; }
export interface ReplaceBenchmarkRecord { benchmarkRecord: BenchmarkRecord; }
export interface ReplaceBenchmarkRecordOp extends ReplaceBenchmarkRecord { mutation: "replaceBenchmarkRecord"; }
export interface CreateActivity { activity: Activity; }
export interface CreateActivityOp extends CreateActivity { mutation: "createActivity"; }
export interface DeleteActivity { id: string; }
export interface DeleteActivityOp extends DeleteActivity { mutation: "deleteActivity"; }
export interface RenameActivity { id: string; newName: string; }
export interface RenameActivityOp extends RenameActivity { mutation: "renameActivity"; }
export interface ReplaceActivity { activity: Activity; }
export interface ReplaceActivityOp extends ReplaceActivity { mutation: "replaceActivity"; }
export interface CreateInfrastructureRequirement { infrastructureRequirement: InfrastructureRequirement; }
export interface CreateInfrastructureRequirementOp extends CreateInfrastructureRequirement { mutation: "createInfrastructureRequirement"; }
export interface DeleteInfrastructureRequirement { id: string; }
export interface DeleteInfrastructureRequirementOp extends DeleteInfrastructureRequirement { mutation: "deleteInfrastructureRequirement"; }
export interface RenameInfrastructureRequirement { id: string; newName: string; }
export interface RenameInfrastructureRequirementOp extends RenameInfrastructureRequirement { mutation: "renameInfrastructureRequirement"; }
export interface ReplaceInfrastructureRequirement { infrastructureRequirement: InfrastructureRequirement; }
export interface ReplaceInfrastructureRequirementOp extends ReplaceInfrastructureRequirement { mutation: "replaceInfrastructureRequirement"; }
export interface CreateOrganizationalRequirement { organizationalRequirement: OrganizationalRequirement; }
export interface CreateOrganizationalRequirementOp extends CreateOrganizationalRequirement { mutation: "createOrganizationalRequirement"; }
export interface DeleteOrganizationalRequirement { id: string; }
export interface DeleteOrganizationalRequirementOp extends DeleteOrganizationalRequirement { mutation: "deleteOrganizationalRequirement"; }
export interface RenameOrganizationalRequirement { id: string; newName: string; }
export interface RenameOrganizationalRequirementOp extends RenameOrganizationalRequirement { mutation: "renameOrganizationalRequirement"; }
export interface ReplaceOrganizationalRequirement { organizationalRequirement: OrganizationalRequirement; }
export interface ReplaceOrganizationalRequirementOp extends ReplaceOrganizationalRequirement { mutation: "replaceOrganizationalRequirement"; }
export interface CreateIssue { issue: Issue; }
export interface CreateIssueOp extends CreateIssue { mutation: "createIssue"; }
export interface DeleteIssue { id: string; }
export interface DeleteIssueOp extends DeleteIssue { mutation: "deleteIssue"; }
export interface RenameIssue { id: string; newName: string; }
export interface RenameIssueOp extends RenameIssue { mutation: "renameIssue"; }
export interface ReplaceIssue { issue: Issue; }
export interface ReplaceIssueOp extends ReplaceIssue { mutation: "replaceIssue"; }
export interface CreateApprovalRecord { approvalRecord: ApprovalRecord; }
export interface CreateApprovalRecordOp extends CreateApprovalRecord { mutation: "createApprovalRecord"; }
export interface DeleteApprovalRecord { id: string; }
export interface DeleteApprovalRecordOp extends DeleteApprovalRecord { mutation: "deleteApprovalRecord"; }
export interface RenameApprovalRecord { id: string; newName: string; }
export interface RenameApprovalRecordOp extends RenameApprovalRecord { mutation: "renameApprovalRecord"; }
export interface ReplaceApprovalRecord { approvalRecord: ApprovalRecord; }
export interface ReplaceApprovalRecordOp extends ReplaceApprovalRecord { mutation: "replaceApprovalRecord"; }
export interface CreateStakeholder { stakeholder: Stakeholder; }
export interface CreateStakeholderOp extends CreateStakeholder { mutation: "createStakeholder"; }
export interface DeleteStakeholder { id: string; }
export interface DeleteStakeholderOp extends DeleteStakeholder { mutation: "deleteStakeholder"; }
export interface RenameStakeholder { id: string; newName: string; }
export interface RenameStakeholderOp extends RenameStakeholder { mutation: "renameStakeholder"; }
export interface ReplaceStakeholder { stakeholder: Stakeholder; }
export interface ReplaceStakeholderOp extends ReplaceStakeholder { mutation: "replaceStakeholder"; }
export interface CreateQualityRecord { qualityRecord: QualityRecord; }
export interface CreateQualityRecordOp extends CreateQualityRecord { mutation: "createQualityRecord"; }
export interface DeleteQualityRecord { id: string; }
export interface DeleteQualityRecordOp extends DeleteQualityRecord { mutation: "deleteQualityRecord"; }
export interface RenameQualityRecord { id: string; newName: string; }
export interface RenameQualityRecordOp extends RenameQualityRecord { mutation: "renameQualityRecord"; }
export interface ReplaceQualityRecord { qualityRecord: QualityRecord; }
export interface ReplaceQualityRecordOp extends ReplaceQualityRecord { mutation: "replaceQualityRecord"; }
export interface CreateResilienceRequirement { resilienceRequirement: ResilienceRequirement; }
export interface CreateResilienceRequirementOp extends CreateResilienceRequirement { mutation: "createResilienceRequirement"; }
export interface DeleteResilienceRequirement { id: string; }
export interface DeleteResilienceRequirementOp extends DeleteResilienceRequirement { mutation: "deleteResilienceRequirement"; }
export interface RenameResilienceRequirement { id: string; newName: string; }
export interface RenameResilienceRequirementOp extends RenameResilienceRequirement { mutation: "renameResilienceRequirement"; }
export interface ReplaceResilienceRequirement { resilienceRequirement: ResilienceRequirement; }
export interface ReplaceResilienceRequirementOp extends ReplaceResilienceRequirement { mutation: "replaceResilienceRequirement"; }
export interface CreateAssumption { assumption: Assumption; }
export interface CreateAssumptionOp extends CreateAssumption { mutation: "createAssumption"; }
export interface DeleteAssumption { id: string; }
export interface DeleteAssumptionOp extends DeleteAssumption { mutation: "deleteAssumption"; }
export interface RenameAssumption { id: string; newName: string; }
export interface RenameAssumptionOp extends RenameAssumption { mutation: "renameAssumption"; }
export interface ReplaceAssumption { assumption: Assumption; }
export interface ReplaceAssumptionOp extends ReplaceAssumption { mutation: "replaceAssumption"; }
export interface CreateCostRequirement { costRequirement: CostRequirement; }
export interface CreateCostRequirementOp extends CreateCostRequirement { mutation: "createCostRequirement"; }
export interface DeleteCostRequirement { id: string; }
export interface DeleteCostRequirementOp extends DeleteCostRequirement { mutation: "deleteCostRequirement"; }
export interface RenameCostRequirement { id: string; newName: string; }
export interface RenameCostRequirementOp extends RenameCostRequirement { mutation: "renameCostRequirement"; }
export interface ReplaceCostRequirement { costRequirement: CostRequirement; }
export interface ReplaceCostRequirementOp extends ReplaceCostRequirement { mutation: "replaceCostRequirement"; }
export interface CreateDocument { document: DocumentRecord; }
export interface CreateDocumentOp extends CreateDocument { mutation: "createDocument"; }
export interface DeleteDocument { id: string; }
export interface DeleteDocumentOp extends DeleteDocument { mutation: "deleteDocument"; }
export interface RenameDocument { id: string; newName: string; }
export interface RenameDocumentOp extends RenameDocument { mutation: "renameDocument"; }
export interface ReplaceDocument { document: DocumentRecord; }
export interface ReplaceDocumentOp extends ReplaceDocument { mutation: "replaceDocument"; }
export interface CreateScheduleRequirement { scheduleRequirement: ScheduleRequirement; }
export interface CreateScheduleRequirementOp extends CreateScheduleRequirement { mutation: "createScheduleRequirement"; }
export interface DeleteScheduleRequirement { id: string; }
export interface DeleteScheduleRequirementOp extends DeleteScheduleRequirement { mutation: "deleteScheduleRequirement"; }
export interface RenameScheduleRequirement { id: string; newName: string; }
export interface RenameScheduleRequirementOp extends RenameScheduleRequirement { mutation: "renameScheduleRequirement"; }
export interface ReplaceScheduleRequirement { scheduleRequirement: ScheduleRequirement; }
export interface ReplaceScheduleRequirementOp extends ReplaceScheduleRequirement { mutation: "replaceScheduleRequirement"; }
export interface CreateGrowthPlan { growthPlan: GrowthPlan; }
export interface CreateGrowthPlanOp extends CreateGrowthPlan { mutation: "createGrowthPlan"; }
export interface DeleteGrowthPlan { id: string; }
export interface DeleteGrowthPlanOp extends DeleteGrowthPlan { mutation: "deleteGrowthPlan"; }
export interface RenameGrowthPlan { id: string; newName: string; }
export interface RenameGrowthPlanOp extends RenameGrowthPlan { mutation: "renameGrowthPlan"; }
export interface ReplaceGrowthPlan { growthPlan: GrowthPlan; }
export interface ReplaceGrowthPlanOp extends ReplaceGrowthPlan { mutation: "replaceGrowthPlan"; }
export interface CreatePerformanceCriterion { performanceCriterion: PerformanceCriterion; }
export interface CreatePerformanceCriterionOp extends CreatePerformanceCriterion { mutation: "createPerformanceCriterion"; }
export interface DeletePerformanceCriterion { id: string; }
export interface DeletePerformanceCriterionOp extends DeletePerformanceCriterion { mutation: "deletePerformanceCriterion"; }
export interface RenamePerformanceCriterion { id: string; newName: string; }
export interface RenamePerformanceCriterionOp extends RenamePerformanceCriterion { mutation: "renamePerformanceCriterion"; }
export interface ReplacePerformanceCriterion { performanceCriterion: PerformanceCriterion; }
export interface ReplacePerformanceCriterionOp extends ReplacePerformanceCriterion { mutation: "replacePerformanceCriterion"; }
export interface CreateOperationalRequirement { operationalRequirement: OperationalRequirement; }
export interface CreateOperationalRequirementOp extends CreateOperationalRequirement { mutation: "createOperationalRequirement"; }
export interface DeleteOperationalRequirement { id: string; }
export interface DeleteOperationalRequirementOp extends DeleteOperationalRequirement { mutation: "deleteOperationalRequirement"; }
export interface RenameOperationalRequirement { id: string; newName: string; }
export interface RenameOperationalRequirementOp extends RenameOperationalRequirement { mutation: "renameOperationalRequirement"; }
export interface ReplaceOperationalRequirement { operationalRequirement: OperationalRequirement; }
export interface ReplaceOperationalRequirementOp extends ReplaceOperationalRequirement { mutation: "replaceOperationalRequirement"; }
export interface CreateRequirement { requirement: Requirement; }
export interface CreateRequirementOp extends CreateRequirement { mutation: "createRequirement"; }
export interface DeleteRequirement { id: string; }
export interface DeleteRequirementOp extends DeleteRequirement { mutation: "deleteRequirement"; }
export interface RenameRequirement { id: string; newName: string; }
export interface RenameRequirementOp extends RenameRequirement { mutation: "renameRequirement"; }
export interface ReplaceRequirement { requirement: Requirement; }
export interface ReplaceRequirementOp extends ReplaceRequirement { mutation: "replaceRequirement"; }
export interface CreateSiteContext { siteContext: SiteContext; }
export interface CreateSiteContextOp extends CreateSiteContext { mutation: "createSiteContext"; }
export interface DeleteSiteContext { id: string; }
export interface DeleteSiteContextOp extends DeleteSiteContext { mutation: "deleteSiteContext"; }
export interface RenameSiteContext { id: string; newName: string; }
export interface RenameSiteContextOp extends RenameSiteContext { mutation: "renameSiteContext"; }
export interface ReplaceSiteContext { siteContext: SiteContext; }
export interface ReplaceSiteContextOp extends ReplaceSiteContext { mutation: "replaceSiteContext"; }
export interface CreateTemplateRecord { templateRecord: TemplateRecord; }
export interface CreateTemplateRecordOp extends CreateTemplateRecord { mutation: "createTemplateRecord"; }
export interface DeleteTemplateRecord { id: string; }
export interface DeleteTemplateRecordOp extends DeleteTemplateRecord { mutation: "deleteTemplateRecord"; }
export interface RenameTemplateRecord { id: string; newName: string; }
export interface RenameTemplateRecordOp extends RenameTemplateRecord { mutation: "renameTemplateRecord"; }
export interface ReplaceTemplateRecord { templateRecord: TemplateRecord; }
export interface ReplaceTemplateRecordOp extends ReplaceTemplateRecord { mutation: "replaceTemplateRecord"; }
export interface CreateReportRecord { reportRecord: ReportRecord; }
export interface CreateReportRecordOp extends CreateReportRecord { mutation: "createReportRecord"; }
export interface DeleteReportRecord { id: string; }
export interface DeleteReportRecordOp extends DeleteReportRecord { mutation: "deleteReportRecord"; }
export interface RenameReportRecord { id: string; newName: string; }
export interface RenameReportRecordOp extends RenameReportRecord { mutation: "renameReportRecord"; }
export interface ReplaceReportRecord { reportRecord: ReportRecord; }
export interface ReplaceReportRecordOp extends ReplaceReportRecord { mutation: "replaceReportRecord"; }
export interface CreateAuditEvent { auditEvent: AuditEvent; }
export interface CreateAuditEventOp extends CreateAuditEvent { mutation: "createAuditEvent"; }
export interface DeleteAuditEvent { id: string; }
export interface DeleteAuditEventOp extends DeleteAuditEvent { mutation: "deleteAuditEvent"; }
export interface RenameAuditEvent { id: string; newName: string; }
export interface RenameAuditEventOp extends RenameAuditEvent { mutation: "renameAuditEvent"; }
export interface ReplaceAuditEvent { auditEvent: AuditEvent; }
export interface ReplaceAuditEventOp extends ReplaceAuditEvent { mutation: "replaceAuditEvent"; }
export interface CreateKnowledgeRecord { knowledgeRecord: KnowledgeRecord; }
export interface CreateKnowledgeRecordOp extends CreateKnowledgeRecord { mutation: "createKnowledgeRecord"; }
export interface DeleteKnowledgeRecord { id: string; }
export interface DeleteKnowledgeRecordOp extends DeleteKnowledgeRecord { mutation: "deleteKnowledgeRecord"; }
export interface RenameKnowledgeRecord { id: string; newName: string; }
export interface RenameKnowledgeRecordOp extends RenameKnowledgeRecord { mutation: "renameKnowledgeRecord"; }
export interface ReplaceKnowledgeRecord { knowledgeRecord: KnowledgeRecord; }
export interface ReplaceKnowledgeRecordOp extends ReplaceKnowledgeRecord { mutation: "replaceKnowledgeRecord"; }
export interface CreateRegulatoryRequirement { regulatoryRequirement: RegulatoryRequirement; }
export interface CreateRegulatoryRequirementOp extends CreateRegulatoryRequirement { mutation: "createRegulatoryRequirement"; }
export interface DeleteRegulatoryRequirement { id: string; }
export interface DeleteRegulatoryRequirementOp extends DeleteRegulatoryRequirement { mutation: "deleteRegulatoryRequirement"; }
export interface RenameRegulatoryRequirement { id: string; newName: string; }
export interface RenameRegulatoryRequirementOp extends RenameRegulatoryRequirement { mutation: "renameRegulatoryRequirement"; }
export interface ReplaceRegulatoryRequirement { regulatoryRequirement: RegulatoryRequirement; }
export interface ReplaceRegulatoryRequirementOp extends ReplaceRegulatoryRequirement { mutation: "replaceRegulatoryRequirement"; }
export interface CreateChangeRecord { changeRecord: ChangeRecord; }
export interface CreateChangeRecordOp extends CreateChangeRecord { mutation: "createChangeRecord"; }
export interface DeleteChangeRecord { id: string; }
export interface DeleteChangeRecordOp extends DeleteChangeRecord { mutation: "deleteChangeRecord"; }
export interface RenameChangeRecord { id: string; newName: string; }
export interface RenameChangeRecordOp extends RenameChangeRecord { mutation: "renameChangeRecord"; }
export interface ReplaceChangeRecord { changeRecord: ChangeRecord; }
export interface ReplaceChangeRecordOp extends ReplaceChangeRecord { mutation: "replaceChangeRecord"; }
export interface CreateCommunicationRequirement { communicationRequirement: CommunicationRequirement; }
export interface CreateCommunicationRequirementOp extends CreateCommunicationRequirement { mutation: "createCommunicationRequirement"; }
export interface DeleteCommunicationRequirement { id: string; }
export interface DeleteCommunicationRequirementOp extends DeleteCommunicationRequirement { mutation: "deleteCommunicationRequirement"; }
export interface RenameCommunicationRequirement { id: string; newName: string; }
export interface RenameCommunicationRequirementOp extends RenameCommunicationRequirement { mutation: "renameCommunicationRequirement"; }
export interface ReplaceCommunicationRequirement { communicationRequirement: CommunicationRequirement; }
export interface ReplaceCommunicationRequirementOp extends ReplaceCommunicationRequirement { mutation: "replaceCommunicationRequirement"; }
export interface CreateResource { resource: Resource; }
export interface CreateResourceOp extends CreateResource { mutation: "createResource"; }
export interface DeleteResource { id: string; }
export interface DeleteResourceOp extends DeleteResource { mutation: "deleteResource"; }
export interface RenameResource { id: string; newName: string; }
export interface RenameResourceOp extends RenameResource { mutation: "renameResource"; }
export interface ReplaceResource { resource: Resource; }
export interface ReplaceResourceOp extends ReplaceResource { mutation: "replaceResource"; }
export interface CreateStatusRecord { statusRecord: StatusRecord; }
export interface CreateStatusRecordOp extends CreateStatusRecord { mutation: "createStatusRecord"; }
export interface DeleteStatusRecord { id: string; }
export interface DeleteStatusRecordOp extends DeleteStatusRecord { mutation: "deleteStatusRecord"; }
export interface RenameStatusRecord { id: string; newName: string; }
export interface RenameStatusRecordOp extends RenameStatusRecord { mutation: "renameStatusRecord"; }
export interface ReplaceStatusRecord { statusRecord: StatusRecord; }
export interface ReplaceStatusRecordOp extends ReplaceStatusRecord { mutation: "replaceStatusRecord"; }
export interface CreateProcess { process: Process; }
export interface CreateProcessOp extends CreateProcess { mutation: "createProcess"; }
export interface DeleteProcess { id: string; }
export interface DeleteProcessOp extends DeleteProcess { mutation: "deleteProcess"; }
export interface RenameProcess { id: string; newName: string; }
export interface RenameProcessOp extends RenameProcess { mutation: "renameProcess"; }
export interface ReplaceProcess { process: Process; }
export interface ReplaceProcessOp extends ReplaceProcess { mutation: "replaceProcess"; }
export interface CreateSearchFilter { searchFilter: SearchFilter; }
export interface CreateSearchFilterOp extends CreateSearchFilter { mutation: "createSearchFilter"; }
export interface DeleteSearchFilter { id: string; }
export interface DeleteSearchFilterOp extends DeleteSearchFilter { mutation: "deleteSearchFilter"; }
export interface RenameSearchFilter { id: string; newName: string; }
export interface RenameSearchFilterOp extends RenameSearchFilter { mutation: "renameSearchFilter"; }
export interface ReplaceSearchFilter { searchFilter: SearchFilter; }
export interface ReplaceSearchFilterOp extends ReplaceSearchFilter { mutation: "replaceSearchFilter"; }
export interface CreateAccessRule { accessRule: AccessRule; }
export interface CreateAccessRuleOp extends CreateAccessRule { mutation: "createAccessRule"; }
export interface DeleteAccessRule { id: string; }
export interface DeleteAccessRuleOp extends DeleteAccessRule { mutation: "deleteAccessRule"; }
export interface RenameAccessRule { id: string; newName: string; }
export interface RenameAccessRuleOp extends RenameAccessRule { mutation: "renameAccessRule"; }
export interface ReplaceAccessRule { accessRule: AccessRule; }
export interface ReplaceAccessRuleOp extends ReplaceAccessRule { mutation: "replaceAccessRule"; }
export interface CreatePrivacyRequirement { privacyRequirement: PrivacyRequirement; }
export interface CreatePrivacyRequirementOp extends CreatePrivacyRequirement { mutation: "createPrivacyRequirement"; }
export interface DeletePrivacyRequirement { id: string; }
export interface DeletePrivacyRequirementOp extends DeletePrivacyRequirement { mutation: "deletePrivacyRequirement"; }
export interface RenamePrivacyRequirement { id: string; newName: string; }
export interface RenamePrivacyRequirementOp extends RenamePrivacyRequirement { mutation: "renamePrivacyRequirement"; }
export interface ReplacePrivacyRequirement { privacyRequirement: PrivacyRequirement; }
export interface ReplacePrivacyRequirementOp extends ReplacePrivacyRequirement { mutation: "replacePrivacyRequirement"; }
export interface CreateRelationship { relationship: Relationship; }
export interface CreateRelationshipOp extends CreateRelationship { mutation: "createRelationship"; }
export interface DeleteRelationship { id: string; }
export interface DeleteRelationshipOp extends DeleteRelationship { mutation: "deleteRelationship"; }
export interface RenameRelationship { id: string; newName: string; }
export interface RenameRelationshipOp extends RenameRelationship { mutation: "renameRelationship"; }
export interface ReplaceRelationship { relationship: Relationship; }
export interface ReplaceRelationshipOp extends ReplaceRelationship { mutation: "replaceRelationship"; }
export interface CreateQuantityRequirement { quantityRequirement: QuantityRequirement; }
export interface CreateQuantityRequirementOp extends CreateQuantityRequirement { mutation: "createQuantityRequirement"; }
export interface DeleteQuantityRequirement { id: string; }
export interface DeleteQuantityRequirementOp extends DeleteQuantityRequirement { mutation: "deleteQuantityRequirement"; }
export interface RenameQuantityRequirement { id: string; newName: string; }
export interface RenameQuantityRequirementOp extends RenameQuantityRequirement { mutation: "renameQuantityRequirement"; }
export interface ReplaceQuantityRequirement { quantityRequirement: QuantityRequirement; }
export interface ReplaceQuantityRequirementOp extends ReplaceQuantityRequirement { mutation: "replaceQuantityRequirement"; }
export interface CreateAnalysisRecord { analysisRecord: AnalysisRecord; }
export interface CreateAnalysisRecordOp extends CreateAnalysisRecord { mutation: "createAnalysisRecord"; }
export interface DeleteAnalysisRecord { id: string; }
export interface DeleteAnalysisRecordOp extends DeleteAnalysisRecord { mutation: "deleteAnalysisRecord"; }
export interface RenameAnalysisRecord { id: string; newName: string; }
export interface RenameAnalysisRecordOp extends RenameAnalysisRecord { mutation: "renameAnalysisRecord"; }
export interface ReplaceAnalysisRecord { analysisRecord: AnalysisRecord; }
export interface ReplaceAnalysisRecordOp extends ReplaceAnalysisRecord { mutation: "replaceAnalysisRecord"; }
export interface CreateStorageRequirement { storageRequirement: StorageRequirement; }
export interface CreateStorageRequirementOp extends CreateStorageRequirement { mutation: "createStorageRequirement"; }
export interface DeleteStorageRequirement { id: string; }
export interface DeleteStorageRequirementOp extends DeleteStorageRequirement { mutation: "deleteStorageRequirement"; }
export interface RenameStorageRequirement { id: string; newName: string; }
export interface RenameStorageRequirementOp extends RenameStorageRequirement { mutation: "renameStorageRequirement"; }
export interface ReplaceStorageRequirement { storageRequirement: StorageRequirement; }
export interface ReplaceStorageRequirementOp extends ReplaceStorageRequirement { mutation: "replaceStorageRequirement"; }
export interface CreateMeetingRecord { meetingRecord: MeetingRecord; }
export interface CreateMeetingRecordOp extends CreateMeetingRecord { mutation: "createMeetingRecord"; }
export interface DeleteMeetingRecord { id: string; }
export interface DeleteMeetingRecordOp extends DeleteMeetingRecord { mutation: "deleteMeetingRecord"; }
export interface RenameMeetingRecord { id: string; newName: string; }
export interface RenameMeetingRecordOp extends RenameMeetingRecord { mutation: "renameMeetingRecord"; }
export interface ReplaceMeetingRecord { meetingRecord: MeetingRecord; }
export interface ReplaceMeetingRecordOp extends ReplaceMeetingRecord { mutation: "replaceMeetingRecord"; }
export interface CreateSurvey { survey: Survey; }
export interface CreateSurveyOp extends CreateSurvey { mutation: "createSurvey"; }
export interface DeleteSurvey { id: string; }
export interface DeleteSurveyOp extends DeleteSurvey { mutation: "deleteSurvey"; }
export interface RenameSurvey { id: string; newName: string; }
export interface RenameSurveyOp extends RenameSurvey { mutation: "renameSurvey"; }
export interface ReplaceSurvey { survey: Survey; }
export interface ReplaceSurveyOp extends ReplaceSurvey { mutation: "replaceSurvey"; }
export interface CreateDeliveryConstraint { deliveryConstraint: DeliveryConstraint; }
export interface CreateDeliveryConstraintOp extends CreateDeliveryConstraint { mutation: "createDeliveryConstraint"; }
export interface DeleteDeliveryConstraint { id: string; }
export interface DeleteDeliveryConstraintOp extends DeleteDeliveryConstraint { mutation: "deleteDeliveryConstraint"; }
export interface RenameDeliveryConstraint { id: string; newName: string; }
export interface RenameDeliveryConstraintOp extends RenameDeliveryConstraint { mutation: "renameDeliveryConstraint"; }
export interface ReplaceDeliveryConstraint { deliveryConstraint: DeliveryConstraint; }
export interface ReplaceDeliveryConstraintOp extends ReplaceDeliveryConstraint { mutation: "replaceDeliveryConstraint"; }
export interface CreateConstraintRecord { constraintRecord: ConstraintRecord; }
export interface CreateConstraintRecordOp extends CreateConstraintRecord { mutation: "createConstraintRecord"; }
export interface DeleteConstraintRecord { id: string; }
export interface DeleteConstraintRecordOp extends DeleteConstraintRecord { mutation: "deleteConstraintRecord"; }
export interface RenameConstraintRecord { id: string; newName: string; }
export interface RenameConstraintRecordOp extends RenameConstraintRecord { mutation: "renameConstraintRecord"; }
export interface ReplaceConstraintRecord { constraintRecord: ConstraintRecord; }
export interface ReplaceConstraintRecordOp extends ReplaceConstraintRecord { mutation: "replaceConstraintRecord"; }
export interface CreateComplianceRecord { complianceRecord: ComplianceRecord; }
export interface CreateComplianceRecordOp extends CreateComplianceRecord { mutation: "createComplianceRecord"; }
export interface DeleteComplianceRecord { id: string; }
export interface DeleteComplianceRecordOp extends DeleteComplianceRecord { mutation: "deleteComplianceRecord"; }
export interface RenameComplianceRecord { id: string; newName: string; }
export interface RenameComplianceRecordOp extends RenameComplianceRecord { mutation: "renameComplianceRecord"; }
export interface ReplaceComplianceRecord { complianceRecord: ComplianceRecord; }
export interface ReplaceComplianceRecordOp extends ReplaceComplianceRecord { mutation: "replaceComplianceRecord"; }
export interface CreateServiceRequirement { serviceRequirement: ServiceRequirement; }
export interface CreateServiceRequirementOp extends CreateServiceRequirement { mutation: "createServiceRequirement"; }
export interface DeleteServiceRequirement { id: string; }
export interface DeleteServiceRequirementOp extends DeleteServiceRequirement { mutation: "deleteServiceRequirement"; }
export interface RenameServiceRequirement { id: string; newName: string; }
export interface RenameServiceRequirementOp extends RenameServiceRequirement { mutation: "renameServiceRequirement"; }
export interface ReplaceServiceRequirement { serviceRequirement: ServiceRequirement; }
export interface ReplaceServiceRequirementOp extends ReplaceServiceRequirement { mutation: "replaceServiceRequirement"; }
export interface CreateEquipment { equipment: Equipment; }
export interface CreateEquipmentOp extends CreateEquipment { mutation: "createEquipment"; }
export interface DeleteEquipment { id: string; }
export interface DeleteEquipmentOp extends DeleteEquipment { mutation: "deleteEquipment"; }
export interface RenameEquipment { id: string; newName: string; }
export interface RenameEquipmentOp extends RenameEquipment { mutation: "renameEquipment"; }
export interface ReplaceEquipment { equipment: Equipment; }
export interface ReplaceEquipmentOp extends ReplaceEquipment { mutation: "replaceEquipment"; }
export interface CreateSecurityRequirement { securityRequirement: SecurityRequirement; }
export interface CreateSecurityRequirementOp extends CreateSecurityRequirement { mutation: "createSecurityRequirement"; }
export interface DeleteSecurityRequirement { id: string; }
export interface DeleteSecurityRequirementOp extends DeleteSecurityRequirement { mutation: "deleteSecurityRequirement"; }
export interface RenameSecurityRequirement { id: string; newName: string; }
export interface RenameSecurityRequirementOp extends RenameSecurityRequirement { mutation: "renameSecurityRequirement"; }
export interface ReplaceSecurityRequirement { securityRequirement: SecurityRequirement; }
export interface ReplaceSecurityRequirementOp extends ReplaceSecurityRequirement { mutation: "replaceSecurityRequirement"; }
export interface CreateCollaborationRecord { collaborationRecord: CollaborationRecord; }
export interface CreateCollaborationRecordOp extends CreateCollaborationRecord { mutation: "createCollaborationRecord"; }
export interface DeleteCollaborationRecord { id: string; }
export interface DeleteCollaborationRecordOp extends DeleteCollaborationRecord { mutation: "deleteCollaborationRecord"; }
export interface RenameCollaborationRecord { id: string; newName: string; }
export interface RenameCollaborationRecordOp extends RenameCollaborationRecord { mutation: "renameCollaborationRecord"; }
export interface ReplaceCollaborationRecord { collaborationRecord: CollaborationRecord; }
export interface ReplaceCollaborationRecordOp extends ReplaceCollaborationRecord { mutation: "replaceCollaborationRecord"; }
export interface CreateSafetyRequirement { safetyRequirement: SafetyRequirement; }
export interface CreateSafetyRequirementOp extends CreateSafetyRequirement { mutation: "createSafetyRequirement"; }
export interface DeleteSafetyRequirement { id: string; }
export interface DeleteSafetyRequirementOp extends DeleteSafetyRequirement { mutation: "deleteSafetyRequirement"; }
export interface RenameSafetyRequirement { id: string; newName: string; }
export interface RenameSafetyRequirementOp extends RenameSafetyRequirement { mutation: "renameSafetyRequirement"; }
export interface ReplaceSafetyRequirement { safetyRequirement: SafetyRequirement; }
export interface ReplaceSafetyRequirementOp extends ReplaceSafetyRequirement { mutation: "replaceSafetyRequirement"; }
export interface CreateUserProfile { userProfile: UserProfile; }
export interface CreateUserProfileOp extends CreateUserProfile { mutation: "createUserProfile"; }
export interface DeleteUserProfile { id: string; }
export interface DeleteUserProfileOp extends DeleteUserProfile { mutation: "deleteUserProfile"; }
export interface RenameUserProfile { id: string; newName: string; }
export interface RenameUserProfileOp extends RenameUserProfile { mutation: "renameUserProfile"; }
export interface ReplaceUserProfile { userProfile: UserProfile; }
export interface ReplaceUserProfileOp extends ReplaceUserProfile { mutation: "replaceUserProfile"; }
export interface CreateHumanFactorRequirement { humanFactorRequirement: HumanFactorRequirement; }
export interface CreateHumanFactorRequirementOp extends CreateHumanFactorRequirement { mutation: "createHumanFactorRequirement"; }
export interface DeleteHumanFactorRequirement { id: string; }
export interface DeleteHumanFactorRequirementOp extends DeleteHumanFactorRequirement { mutation: "deleteHumanFactorRequirement"; }
export interface RenameHumanFactorRequirement { id: string; newName: string; }
export interface RenameHumanFactorRequirementOp extends RenameHumanFactorRequirement { mutation: "renameHumanFactorRequirement"; }
export interface ReplaceHumanFactorRequirement { humanFactorRequirement: HumanFactorRequirement; }
export interface ReplaceHumanFactorRequirementOp extends ReplaceHumanFactorRequirement { mutation: "replaceHumanFactorRequirement"; }
export interface CreateFlexibilityRequirement { flexibilityRequirement: FlexibilityRequirement; }
export interface CreateFlexibilityRequirementOp extends CreateFlexibilityRequirement { mutation: "createFlexibilityRequirement"; }
export interface DeleteFlexibilityRequirement { id: string; }
export interface DeleteFlexibilityRequirementOp extends DeleteFlexibilityRequirement { mutation: "deleteFlexibilityRequirement"; }
export interface RenameFlexibilityRequirement { id: string; newName: string; }
export interface RenameFlexibilityRequirementOp extends RenameFlexibilityRequirement { mutation: "renameFlexibilityRequirement"; }
export interface ReplaceFlexibilityRequirement { flexibilityRequirement: FlexibilityRequirement; }
export interface ReplaceFlexibilityRequirementOp extends ReplaceFlexibilityRequirement { mutation: "replaceFlexibilityRequirement"; }
export interface CreateWayfindingRequirement { wayfindingRequirement: WayfindingRequirement; }
export interface CreateWayfindingRequirementOp extends CreateWayfindingRequirement { mutation: "createWayfindingRequirement"; }
export interface DeleteWayfindingRequirement { id: string; }
export interface DeleteWayfindingRequirementOp extends DeleteWayfindingRequirement { mutation: "deleteWayfindingRequirement"; }
export interface RenameWayfindingRequirement { id: string; newName: string; }
export interface RenameWayfindingRequirementOp extends RenameWayfindingRequirement { mutation: "renameWayfindingRequirement"; }
export interface ReplaceWayfindingRequirement { wayfindingRequirement: WayfindingRequirement; }
export interface ReplaceWayfindingRequirementOp extends ReplaceWayfindingRequirement { mutation: "replaceWayfindingRequirement"; }
export interface CreateProgramElement { programElement: ProgramElement; }
export interface CreateProgramElementOp extends CreateProgramElement { mutation: "createProgramElement"; }
export interface DeleteProgramElement { id: string; }
export interface DeleteProgramElementOp extends DeleteProgramElement { mutation: "deleteProgramElement"; }
export interface RenameProgramElement { id: string; newName: string; }
export interface RenameProgramElementOp extends RenameProgramElement { mutation: "renameProgramElement"; }
export interface ReplaceProgramElement { programElement: ProgramElement; }
export interface ReplaceProgramElementOp extends ReplaceProgramElement { mutation: "replaceProgramElement"; }
export interface ConnectAdjacency { adjacency: Adjacency; }
export interface ConnectAdjacencyOp extends ConnectAdjacency { mutation: "connectAdjacency"; }
export interface DisconnectAdjacency { id: string; }
export interface DisconnectAdjacencyOp extends DisconnectAdjacency { mutation: "disconnectAdjacency"; }
export interface ConnectTrace { trace: TraceLink; }
export interface ConnectTraceOp extends ConnectTrace { mutation: "connectTrace"; }
export interface DisconnectTrace { id: string; }
export interface DisconnectTraceOp extends DisconnectTrace { mutation: "disconnectTrace"; }
export interface RenameMeta { newTitle: string; }
export interface RenameMetaOp extends RenameMeta { mutation: "renameMeta"; }
export interface ReplaceMeta { newMeta: ProgramMeta; }
export interface ReplaceMetaOp extends ReplaceMeta { mutation: "replaceMeta"; }
export interface RenameProject { newCode: string; }
export interface RenameProjectOp extends RenameProject { mutation: "renameProject"; }
export interface ReplaceProject { newProject: ProjectDefinition; }
export interface ReplaceProjectOp extends ReplaceProject { mutation: "replaceProject"; }
export interface RenameGovernance { newFramework: string; }
export interface RenameGovernanceOp extends RenameGovernance { mutation: "renameGovernance"; }
export interface ReplaceGovernance { newGovernance: Governance; }
export interface ReplaceGovernanceOp extends ReplaceGovernance { mutation: "replaceGovernance"; }

export type ProgramMutation = CreateInformationRequirementOp | DeleteInformationRequirementOp
   | RenameInformationRequirementOp | ReplaceInformationRequirementOp | CreateSustainabilityRequirementOp
   | DeleteSustainabilityRequirementOp | RenameSustainabilityRequirementOp | ReplaceSustainabilityRequirementOp
   | CreateAccessibilityRequirementOp | DeleteAccessibilityRequirementOp | RenameAccessibilityRequirementOp
   | ReplaceAccessibilityRequirementOp | CreateConflictOp | DeleteConflictOp | RenameConflictOp | ReplaceConflictOp
   | CreateOptionEvaluationOp | DeleteOptionEvaluationOp | RenameOptionEvaluationOp | ReplaceOptionEvaluationOp
   | CreateFunctionOp | DeleteFunctionOp | RenameFunctionOp | ReplaceFunctionOp | CreateRiskOp | DeleteRiskOp
   | RenameRiskOp | ReplaceRiskOp | CreateDecisionOp | DeleteDecisionOp | RenameDecisionOp | ReplaceDecisionOp
   | CreateValidationRecordOp | DeleteValidationRecordOp | RenameValidationRecordOp | ReplaceValidationRecordOp
   | CreatePriorityRecordOp | DeletePriorityRecordOp | RenamePriorityRecordOp | ReplacePriorityRecordOp
   | CreateFlowRequirementOp | DeleteFlowRequirementOp | RenameFlowRequirementOp | ReplaceFlowRequirementOp
   | CreateEnvironmentalRequirementOp | DeleteEnvironmentalRequirementOp | RenameEnvironmentalRequirementOp
   | ReplaceEnvironmentalRequirementOp | CreateWorkshopOp | DeleteWorkshopOp | RenameWorkshopOp | ReplaceWorkshopOp
   | CreateScenarioOp | DeleteScenarioOp | RenameScenarioOp | ReplaceScenarioOp | CreateBenchmarkRecordOp
   | DeleteBenchmarkRecordOp | RenameBenchmarkRecordOp | ReplaceBenchmarkRecordOp | CreateActivityOp
   | DeleteActivityOp | RenameActivityOp | ReplaceActivityOp | CreateInfrastructureRequirementOp
   | DeleteInfrastructureRequirementOp | RenameInfrastructureRequirementOp | ReplaceInfrastructureRequirementOp
   | CreateOrganizationalRequirementOp | DeleteOrganizationalRequirementOp | RenameOrganizationalRequirementOp
   | ReplaceOrganizationalRequirementOp | CreateIssueOp | DeleteIssueOp | RenameIssueOp | ReplaceIssueOp
   | CreateApprovalRecordOp | DeleteApprovalRecordOp | RenameApprovalRecordOp | ReplaceApprovalRecordOp
   | CreateStakeholderOp | DeleteStakeholderOp | RenameStakeholderOp | ReplaceStakeholderOp | CreateQualityRecordOp
   | DeleteQualityRecordOp | RenameQualityRecordOp | ReplaceQualityRecordOp | CreateResilienceRequirementOp
   | DeleteResilienceRequirementOp | RenameResilienceRequirementOp | ReplaceResilienceRequirementOp
   | CreateAssumptionOp | DeleteAssumptionOp | RenameAssumptionOp | ReplaceAssumptionOp | CreateCostRequirementOp
   | DeleteCostRequirementOp | RenameCostRequirementOp | ReplaceCostRequirementOp | CreateDocumentOp
   | DeleteDocumentOp | RenameDocumentOp | ReplaceDocumentOp | CreateScheduleRequirementOp
   | DeleteScheduleRequirementOp | RenameScheduleRequirementOp | ReplaceScheduleRequirementOp | CreateGrowthPlanOp
   | DeleteGrowthPlanOp | RenameGrowthPlanOp | ReplaceGrowthPlanOp | CreatePerformanceCriterionOp
   | DeletePerformanceCriterionOp | RenamePerformanceCriterionOp | ReplacePerformanceCriterionOp
   | CreateOperationalRequirementOp | DeleteOperationalRequirementOp | RenameOperationalRequirementOp
   | ReplaceOperationalRequirementOp | CreateRequirementOp | DeleteRequirementOp | RenameRequirementOp
   | ReplaceRequirementOp | CreateSiteContextOp | DeleteSiteContextOp | RenameSiteContextOp | ReplaceSiteContextOp
   | CreateTemplateRecordOp | DeleteTemplateRecordOp | RenameTemplateRecordOp | ReplaceTemplateRecordOp
   | CreateReportRecordOp | DeleteReportRecordOp | RenameReportRecordOp | ReplaceReportRecordOp | CreateAuditEventOp
   | DeleteAuditEventOp | RenameAuditEventOp | ReplaceAuditEventOp | CreateKnowledgeRecordOp
   | DeleteKnowledgeRecordOp | RenameKnowledgeRecordOp | ReplaceKnowledgeRecordOp | CreateRegulatoryRequirementOp
   | DeleteRegulatoryRequirementOp | RenameRegulatoryRequirementOp | ReplaceRegulatoryRequirementOp
   | CreateChangeRecordOp | DeleteChangeRecordOp | RenameChangeRecordOp | ReplaceChangeRecordOp
   | CreateCommunicationRequirementOp | DeleteCommunicationRequirementOp | RenameCommunicationRequirementOp
   | ReplaceCommunicationRequirementOp | CreateResourceOp | DeleteResourceOp | RenameResourceOp | ReplaceResourceOp
   | CreateStatusRecordOp | DeleteStatusRecordOp | RenameStatusRecordOp | ReplaceStatusRecordOp | CreateProcessOp
   | DeleteProcessOp | RenameProcessOp | ReplaceProcessOp | CreateSearchFilterOp | DeleteSearchFilterOp
   | RenameSearchFilterOp | ReplaceSearchFilterOp | CreateAccessRuleOp | DeleteAccessRuleOp | RenameAccessRuleOp
   | ReplaceAccessRuleOp | CreatePrivacyRequirementOp | DeletePrivacyRequirementOp | RenamePrivacyRequirementOp
   | ReplacePrivacyRequirementOp | CreateRelationshipOp | DeleteRelationshipOp | RenameRelationshipOp
   | ReplaceRelationshipOp | CreateQuantityRequirementOp | DeleteQuantityRequirementOp | RenameQuantityRequirementOp
   | ReplaceQuantityRequirementOp | CreateAnalysisRecordOp | DeleteAnalysisRecordOp | RenameAnalysisRecordOp
   | ReplaceAnalysisRecordOp | CreateStorageRequirementOp | DeleteStorageRequirementOp | RenameStorageRequirementOp
   | ReplaceStorageRequirementOp | CreateMeetingRecordOp | DeleteMeetingRecordOp | RenameMeetingRecordOp
   | ReplaceMeetingRecordOp | CreateSurveyOp | DeleteSurveyOp | RenameSurveyOp | ReplaceSurveyOp
   | CreateDeliveryConstraintOp | DeleteDeliveryConstraintOp | RenameDeliveryConstraintOp
   | ReplaceDeliveryConstraintOp | CreateConstraintRecordOp | DeleteConstraintRecordOp | RenameConstraintRecordOp
   | ReplaceConstraintRecordOp | CreateComplianceRecordOp | DeleteComplianceRecordOp | RenameComplianceRecordOp
   | ReplaceComplianceRecordOp | CreateServiceRequirementOp | DeleteServiceRequirementOp | RenameServiceRequirementOp
   | ReplaceServiceRequirementOp | CreateEquipmentOp | DeleteEquipmentOp | RenameEquipmentOp | ReplaceEquipmentOp
   | CreateSecurityRequirementOp | DeleteSecurityRequirementOp | RenameSecurityRequirementOp
   | ReplaceSecurityRequirementOp | CreateCollaborationRecordOp | DeleteCollaborationRecordOp
   | RenameCollaborationRecordOp | ReplaceCollaborationRecordOp | CreateSafetyRequirementOp
   | DeleteSafetyRequirementOp | RenameSafetyRequirementOp | ReplaceSafetyRequirementOp | CreateUserProfileOp
   | DeleteUserProfileOp | RenameUserProfileOp | ReplaceUserProfileOp | CreateHumanFactorRequirementOp
   | DeleteHumanFactorRequirementOp | RenameHumanFactorRequirementOp | ReplaceHumanFactorRequirementOp
   | CreateFlexibilityRequirementOp | DeleteFlexibilityRequirementOp | RenameFlexibilityRequirementOp
   | ReplaceFlexibilityRequirementOp | CreateWayfindingRequirementOp | DeleteWayfindingRequirementOp
   | RenameWayfindingRequirementOp | ReplaceWayfindingRequirementOp | CreateProgramElementOp | DeleteProgramElementOp
   | RenameProgramElementOp | ReplaceProgramElementOp | ConnectAdjacencyOp | DisconnectAdjacencyOp | ConnectTraceOp
   | DisconnectTraceOp | RenameMetaOp | ReplaceMetaOp | RenameProjectOp | ReplaceProjectOp | RenameGovernanceOp
   | ReplaceGovernanceOp;
