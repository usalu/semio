/** 🧬️ ProgramSnapshot diff schema — sparse field delta. */

export interface ProgramDiff {
  /** @state persistent */
  artifact?: ProgramArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  meta?: ProgramMeta;
  /** @state persistent */
  project?: ProjectDefinition;
  /** @state persistent */
  stakeholders?: ProgramStakeholdersDelta;
  /** @state persistent */
  users?: ProgramUsersDelta;
  /** @state persistent */
  activities?: ProgramActivitiesDelta;
  /** @state persistent */
  functions?: ProgramFunctionsDelta;
  /** @state persistent */
  elements?: ProgramElementsDelta;
  /** @state persistent */
  quantities?: ProgramQuantitiesDelta;
  /** @state persistent */
  relationships?: ProgramRelationshipsDelta;
  /** @state persistent */
  adjacencies?: ProgramAdjacenciesDelta;
  /** @state persistent */
  processes?: ProgramProcessesDelta;
  /** @state persistent */
  flows?: ProgramFlowsDelta;
  /** @state persistent */
  accessRules?: ProgramAccessRulesDelta;
  /** @state persistent */
  operations?: ProgramOperationsDelta;
  /** @state persistent */
  equipment?: ProgramEquipmentDelta;
  /** @state persistent */
  resources?: ProgramResourcesDelta;
  /** @state persistent */
  storage?: ProgramStorageDelta;
  /** @state persistent */
  environmental?: ProgramEnvironmentalDelta;
  /** @state persistent */
  humanFactors?: ProgramHumanFactorsDelta;
  /** @state persistent */
  accessibility?: ProgramAccessibilityDelta;
  /** @state persistent */
  privacy?: ProgramPrivacyDelta;
  /** @state persistent */
  safety?: ProgramSafetyDelta;
  /** @state persistent */
  security?: ProgramSecurityDelta;
  /** @state persistent */
  regulatory?: ProgramRegulatoryDelta;
  /** @state persistent */
  siteContext?: ProgramSiteContextDelta;
  /** @state persistent */
  organizational?: ProgramOrganizationalDelta;
  /** @state persistent */
  services?: ProgramServicesDelta;
  /** @state persistent */
  infrastructure?: ProgramInfrastructureDelta;
  /** @state persistent */
  information?: ProgramInformationDelta;
  /** @state persistent */
  communication?: ProgramCommunicationDelta;
  /** @state persistent */
  wayfinding?: ProgramWayfindingDelta;
  /** @state persistent */
  schedules?: ProgramSchedulesDelta;
  /** @state persistent */
  flexibility?: ProgramFlexibilityDelta;
  /** @state persistent */
  growth?: ProgramGrowthDelta;
  /** @state persistent */
  sustainability?: ProgramSustainabilityDelta;
  /** @state persistent */
  resilience?: ProgramResilienceDelta;
  /** @state persistent */
  costs?: ProgramCostsDelta;
  /** @state persistent */
  delivery?: ProgramDeliveryDelta;
  /** @state persistent */
  risks?: ProgramRisksDelta;
  /** @state persistent */
  conflicts?: ProgramConflictsDelta;
  /** @state persistent */
  requirements?: ProgramRequirementsDelta;
  /** @state persistent */
  priorities?: ProgramPrioritiesDelta;
  /** @state persistent */
  scenarios?: ProgramScenariosDelta;
  /** @state persistent */
  options?: ProgramOptionsDelta;
  /** @state persistent */
  decisions?: ProgramDecisionsDelta;
  /** @state persistent */
  validations?: ProgramValidationsDelta;
  /** @state persistent */
  performance?: ProgramPerformanceDelta;
  /** @state persistent */
  quality?: ProgramQualityDelta;
  /** @state persistent */
  documents?: ProgramDocumentsDelta;
  /** @state persistent */
  assumptions?: ProgramAssumptionsDelta;
  /** @state persistent */
  constraints?: ProgramConstraintsDelta;
  /** @state persistent */
  complianceRecords?: ProgramComplianceRecordsDelta;
  /** @state persistent */
  approvals?: ProgramApprovalsDelta;
  /** @state persistent */
  meetings?: ProgramMeetingsDelta;
  /** @state persistent */
  changes?: ProgramChangesDelta;
  /** @state persistent */
  collaboration?: ProgramCollaborationDelta;
  /** @state persistent */
  analyses?: ProgramAnalysesDelta;
  /** @state persistent */
  reports?: ProgramReportsDelta;
  /** @state persistent */
  searchFilters?: ProgramSearchFiltersDelta;
  /** @state persistent */
  statusRecords?: ProgramStatusRecordsDelta;
  /** @state persistent */
  workshops?: ProgramWorkshopsDelta;
  /** @state persistent */
  surveys?: ProgramSurveysDelta;
  /** @state persistent */
  issues?: ProgramIssuesDelta;
  /** @state persistent */
  auditEvents?: ProgramAuditEventsDelta;
  /** @state persistent */
  templates?: ProgramTemplatesDelta;
  /** @state persistent */
  knowledge?: ProgramKnowledgeDelta;
  /** @state persistent */
  benchmarks?: ProgramBenchmarksDelta;
  /** @state persistent */
  traces?: ProgramTracesDelta;
  /** @state persistent */
  governance?: Governance;
  /** @state shared-ui */
  selectedIds?: string;
  /** @state shared-ui */
  activeRegister?: string;
  /** @state shared-ui */
  adjacencyKindFilter?: AdjacencyKind;
  /** @state shared-ui */
  activeReportJson?: string;
  /** @state local-ui */
  searchQuery?: string;
  /** @state local-ui */
  searchHistoryJson?: string;
  /** @state local-ui */
  lastResultJson?: string;
  /** @state local-ui */
  lastAnalysisJson?: string;
  /** @state local-ui */
  graphCameraX?: number;
  /** @state local-ui */
  graphCameraY?: number;
  /** @state local-ui */
  graphCameraZoom?: number;
}

export interface ProgramStakeholdersDelta {
  added: Stakeholder[];
  removed: string[];
  patched: ProgramStakeholdersPatchEntry[];
  reordered?: string[];
}

export interface ProgramStakeholdersPatchEntry {
  id: string;
  item: Stakeholder;
}

export interface ProgramUsersDelta {
  added: UserProfile[];
  removed: string[];
  patched: ProgramUsersPatchEntry[];
  reordered?: string[];
}

export interface ProgramUsersPatchEntry {
  id: string;
  item: UserProfile;
}

export interface ProgramActivitiesDelta {
  added: Activity[];
  removed: string[];
  patched: ProgramActivitiesPatchEntry[];
  reordered?: string[];
}

export interface ProgramActivitiesPatchEntry {
  id: string;
  item: Activity;
}

export interface ProgramFunctionsDelta {
  added: Function[];
  removed: string[];
  patched: ProgramFunctionsPatchEntry[];
  reordered?: string[];
}

export interface ProgramFunctionsPatchEntry {
  id: string;
  item: Function;
}

export interface ProgramElementsDelta {
  added: ProgramElement[];
  removed: string[];
  patched: ProgramElementsPatchEntry[];
  reordered?: string[];
}

export interface ProgramElementsPatchEntry {
  id: string;
  item: ProgramElement;
}

export interface ProgramQuantitiesDelta {
  added: QuantityRequirement[];
  removed: string[];
  patched: ProgramQuantitiesPatchEntry[];
  reordered?: string[];
}

export interface ProgramQuantitiesPatchEntry {
  id: string;
  item: QuantityRequirement;
}

export interface ProgramRelationshipsDelta {
  added: Relationship[];
  removed: string[];
  patched: ProgramRelationshipsPatchEntry[];
  reordered?: string[];
}

export interface ProgramRelationshipsPatchEntry {
  id: string;
  item: Relationship;
}

export interface ProgramAdjacenciesDelta {
  added: Adjacency[];
  removed: string[];
  patched: ProgramAdjacenciesPatchEntry[];
  reordered?: string[];
}

export interface ProgramAdjacenciesPatchEntry {
  id: string;
  item: Adjacency;
}

export interface ProgramProcessesDelta {
  added: Process[];
  removed: string[];
  patched: ProgramProcessesPatchEntry[];
  reordered?: string[];
}

export interface ProgramProcessesPatchEntry {
  id: string;
  item: Process;
}

export interface ProgramFlowsDelta {
  added: FlowRequirement[];
  removed: string[];
  patched: ProgramFlowsPatchEntry[];
  reordered?: string[];
}

export interface ProgramFlowsPatchEntry {
  id: string;
  item: FlowRequirement;
}

export interface ProgramAccessRulesDelta {
  added: AccessRule[];
  removed: string[];
  patched: ProgramAccessRulesPatchEntry[];
  reordered?: string[];
}

export interface ProgramAccessRulesPatchEntry {
  id: string;
  item: AccessRule;
}

export interface ProgramOperationsDelta {
  added: OperationalRequirement[];
  removed: string[];
  patched: ProgramOperationsPatchEntry[];
  reordered?: string[];
}

export interface ProgramOperationsPatchEntry {
  id: string;
  item: OperationalRequirement;
}

export interface ProgramEquipmentDelta {
  added: Equipment[];
  removed: string[];
  patched: ProgramEquipmentPatchEntry[];
  reordered?: string[];
}

export interface ProgramEquipmentPatchEntry {
  id: string;
  item: Equipment;
}

export interface ProgramResourcesDelta {
  added: Resource[];
  removed: string[];
  patched: ProgramResourcesPatchEntry[];
  reordered?: string[];
}

export interface ProgramResourcesPatchEntry {
  id: string;
  item: Resource;
}

export interface ProgramStorageDelta {
  added: StorageRequirement[];
  removed: string[];
  patched: ProgramStoragePatchEntry[];
  reordered?: string[];
}

export interface ProgramStoragePatchEntry {
  id: string;
  item: StorageRequirement;
}

export interface ProgramEnvironmentalDelta {
  added: EnvironmentalRequirement[];
  removed: string[];
  patched: ProgramEnvironmentalPatchEntry[];
  reordered?: string[];
}

export interface ProgramEnvironmentalPatchEntry {
  id: string;
  item: EnvironmentalRequirement;
}

export interface ProgramHumanFactorsDelta {
  added: HumanFactorRequirement[];
  removed: string[];
  patched: ProgramHumanFactorsPatchEntry[];
  reordered?: string[];
}

export interface ProgramHumanFactorsPatchEntry {
  id: string;
  item: HumanFactorRequirement;
}

export interface ProgramAccessibilityDelta {
  added: AccessibilityRequirement[];
  removed: string[];
  patched: ProgramAccessibilityPatchEntry[];
  reordered?: string[];
}

export interface ProgramAccessibilityPatchEntry {
  id: string;
  item: AccessibilityRequirement;
}

export interface ProgramPrivacyDelta {
  added: PrivacyRequirement[];
  removed: string[];
  patched: ProgramPrivacyPatchEntry[];
  reordered?: string[];
}

export interface ProgramPrivacyPatchEntry {
  id: string;
  item: PrivacyRequirement;
}

export interface ProgramSafetyDelta {
  added: SafetyRequirement[];
  removed: string[];
  patched: ProgramSafetyPatchEntry[];
  reordered?: string[];
}

export interface ProgramSafetyPatchEntry {
  id: string;
  item: SafetyRequirement;
}

export interface ProgramSecurityDelta {
  added: SecurityRequirement[];
  removed: string[];
  patched: ProgramSecurityPatchEntry[];
  reordered?: string[];
}

export interface ProgramSecurityPatchEntry {
  id: string;
  item: SecurityRequirement;
}

export interface ProgramRegulatoryDelta {
  added: RegulatoryRequirement[];
  removed: string[];
  patched: ProgramRegulatoryPatchEntry[];
  reordered?: string[];
}

export interface ProgramRegulatoryPatchEntry {
  id: string;
  item: RegulatoryRequirement;
}

export interface ProgramSiteContextDelta {
  added: SiteContext[];
  removed: string[];
  patched: ProgramSiteContextPatchEntry[];
  reordered?: string[];
}

export interface ProgramSiteContextPatchEntry {
  id: string;
  item: SiteContext;
}

export interface ProgramOrganizationalDelta {
  added: OrganizationalRequirement[];
  removed: string[];
  patched: ProgramOrganizationalPatchEntry[];
  reordered?: string[];
}

export interface ProgramOrganizationalPatchEntry {
  id: string;
  item: OrganizationalRequirement;
}

export interface ProgramServicesDelta {
  added: ServiceRequirement[];
  removed: string[];
  patched: ProgramServicesPatchEntry[];
  reordered?: string[];
}

export interface ProgramServicesPatchEntry {
  id: string;
  item: ServiceRequirement;
}

export interface ProgramInfrastructureDelta {
  added: InfrastructureRequirement[];
  removed: string[];
  patched: ProgramInfrastructurePatchEntry[];
  reordered?: string[];
}

export interface ProgramInfrastructurePatchEntry {
  id: string;
  item: InfrastructureRequirement;
}

export interface ProgramInformationDelta {
  added: InformationRequirement[];
  removed: string[];
  patched: ProgramInformationPatchEntry[];
  reordered?: string[];
}

export interface ProgramInformationPatchEntry {
  id: string;
  item: InformationRequirement;
}

export interface ProgramCommunicationDelta {
  added: CommunicationRequirement[];
  removed: string[];
  patched: ProgramCommunicationPatchEntry[];
  reordered?: string[];
}

export interface ProgramCommunicationPatchEntry {
  id: string;
  item: CommunicationRequirement;
}

export interface ProgramWayfindingDelta {
  added: WayfindingRequirement[];
  removed: string[];
  patched: ProgramWayfindingPatchEntry[];
  reordered?: string[];
}

export interface ProgramWayfindingPatchEntry {
  id: string;
  item: WayfindingRequirement;
}

export interface ProgramSchedulesDelta {
  added: ScheduleRequirement[];
  removed: string[];
  patched: ProgramSchedulesPatchEntry[];
  reordered?: string[];
}

export interface ProgramSchedulesPatchEntry {
  id: string;
  item: ScheduleRequirement;
}

export interface ProgramFlexibilityDelta {
  added: FlexibilityRequirement[];
  removed: string[];
  patched: ProgramFlexibilityPatchEntry[];
  reordered?: string[];
}

export interface ProgramFlexibilityPatchEntry {
  id: string;
  item: FlexibilityRequirement;
}

export interface ProgramGrowthDelta {
  added: GrowthPlan[];
  removed: string[];
  patched: ProgramGrowthPatchEntry[];
  reordered?: string[];
}

export interface ProgramGrowthPatchEntry {
  id: string;
  item: GrowthPlan;
}

export interface ProgramSustainabilityDelta {
  added: SustainabilityRequirement[];
  removed: string[];
  patched: ProgramSustainabilityPatchEntry[];
  reordered?: string[];
}

export interface ProgramSustainabilityPatchEntry {
  id: string;
  item: SustainabilityRequirement;
}

export interface ProgramResilienceDelta {
  added: ResilienceRequirement[];
  removed: string[];
  patched: ProgramResiliencePatchEntry[];
  reordered?: string[];
}

export interface ProgramResiliencePatchEntry {
  id: string;
  item: ResilienceRequirement;
}

export interface ProgramCostsDelta {
  added: CostRequirement[];
  removed: string[];
  patched: ProgramCostsPatchEntry[];
  reordered?: string[];
}

export interface ProgramCostsPatchEntry {
  id: string;
  item: CostRequirement;
}

export interface ProgramDeliveryDelta {
  added: DeliveryConstraint[];
  removed: string[];
  patched: ProgramDeliveryPatchEntry[];
  reordered?: string[];
}

export interface ProgramDeliveryPatchEntry {
  id: string;
  item: DeliveryConstraint;
}

export interface ProgramRisksDelta {
  added: Risk[];
  removed: string[];
  patched: ProgramRisksPatchEntry[];
  reordered?: string[];
}

export interface ProgramRisksPatchEntry {
  id: string;
  item: Risk;
}

export interface ProgramConflictsDelta {
  added: Conflict[];
  removed: string[];
  patched: ProgramConflictsPatchEntry[];
  reordered?: string[];
}

export interface ProgramConflictsPatchEntry {
  id: string;
  item: Conflict;
}

export interface ProgramRequirementsDelta {
  added: Requirement[];
  removed: string[];
  patched: ProgramRequirementsPatchEntry[];
  reordered?: string[];
}

export interface ProgramRequirementsPatchEntry {
  id: string;
  item: Requirement;
}

export interface ProgramPrioritiesDelta {
  added: PriorityRecord[];
  removed: string[];
  patched: ProgramPrioritiesPatchEntry[];
  reordered?: string[];
}

export interface ProgramPrioritiesPatchEntry {
  id: string;
  item: PriorityRecord;
}

export interface ProgramScenariosDelta {
  added: Scenario[];
  removed: string[];
  patched: ProgramScenariosPatchEntry[];
  reordered?: string[];
}

export interface ProgramScenariosPatchEntry {
  id: string;
  item: Scenario;
}

export interface ProgramOptionsDelta {
  added: OptionEvaluation[];
  removed: string[];
  patched: ProgramOptionsPatchEntry[];
  reordered?: string[];
}

export interface ProgramOptionsPatchEntry {
  id: string;
  item: OptionEvaluation;
}

export interface ProgramDecisionsDelta {
  added: Decision[];
  removed: string[];
  patched: ProgramDecisionsPatchEntry[];
  reordered?: string[];
}

export interface ProgramDecisionsPatchEntry {
  id: string;
  item: Decision;
}

export interface ProgramValidationsDelta {
  added: ValidationRecord[];
  removed: string[];
  patched: ProgramValidationsPatchEntry[];
  reordered?: string[];
}

export interface ProgramValidationsPatchEntry {
  id: string;
  item: ValidationRecord;
}

export interface ProgramPerformanceDelta {
  added: PerformanceCriterion[];
  removed: string[];
  patched: ProgramPerformancePatchEntry[];
  reordered?: string[];
}

export interface ProgramPerformancePatchEntry {
  id: string;
  item: PerformanceCriterion;
}

export interface ProgramQualityDelta {
  added: QualityRecord[];
  removed: string[];
  patched: ProgramQualityPatchEntry[];
  reordered?: string[];
}

export interface ProgramQualityPatchEntry {
  id: string;
  item: QualityRecord;
}

export interface ProgramDocumentsDelta {
  added: DocumentRecord[];
  removed: string[];
  patched: ProgramDocumentsPatchEntry[];
  reordered?: string[];
}

export interface ProgramDocumentsPatchEntry {
  id: string;
  item: DocumentRecord;
}

export interface ProgramAssumptionsDelta {
  added: Assumption[];
  removed: string[];
  patched: ProgramAssumptionsPatchEntry[];
  reordered?: string[];
}

export interface ProgramAssumptionsPatchEntry {
  id: string;
  item: Assumption;
}

export interface ProgramConstraintsDelta {
  added: ConstraintRecord[];
  removed: string[];
  patched: ProgramConstraintsPatchEntry[];
  reordered?: string[];
}

export interface ProgramConstraintsPatchEntry {
  id: string;
  item: ConstraintRecord;
}

export interface ProgramComplianceRecordsDelta {
  added: ComplianceRecord[];
  removed: string[];
  patched: ProgramComplianceRecordsPatchEntry[];
  reordered?: string[];
}

export interface ProgramComplianceRecordsPatchEntry {
  id: string;
  item: ComplianceRecord;
}

export interface ProgramApprovalsDelta {
  added: ApprovalRecord[];
  removed: string[];
  patched: ProgramApprovalsPatchEntry[];
  reordered?: string[];
}

export interface ProgramApprovalsPatchEntry {
  id: string;
  item: ApprovalRecord;
}

export interface ProgramMeetingsDelta {
  added: MeetingRecord[];
  removed: string[];
  patched: ProgramMeetingsPatchEntry[];
  reordered?: string[];
}

export interface ProgramMeetingsPatchEntry {
  id: string;
  item: MeetingRecord;
}

export interface ProgramChangesDelta {
  added: ChangeRecord[];
  removed: string[];
  patched: ProgramChangesPatchEntry[];
  reordered?: string[];
}

export interface ProgramChangesPatchEntry {
  id: string;
  item: ChangeRecord;
}

export interface ProgramCollaborationDelta {
  added: CollaborationRecord[];
  removed: string[];
  patched: ProgramCollaborationPatchEntry[];
  reordered?: string[];
}

export interface ProgramCollaborationPatchEntry {
  id: string;
  item: CollaborationRecord;
}

export interface ProgramAnalysesDelta {
  added: AnalysisRecord[];
  removed: string[];
  patched: ProgramAnalysesPatchEntry[];
  reordered?: string[];
}

export interface ProgramAnalysesPatchEntry {
  id: string;
  item: AnalysisRecord;
}

export interface ProgramReportsDelta {
  added: ReportRecord[];
  removed: string[];
  patched: ProgramReportsPatchEntry[];
  reordered?: string[];
}

export interface ProgramReportsPatchEntry {
  id: string;
  item: ReportRecord;
}

export interface ProgramSearchFiltersDelta {
  added: SearchFilter[];
  removed: string[];
  patched: ProgramSearchFiltersPatchEntry[];
  reordered?: string[];
}

export interface ProgramSearchFiltersPatchEntry {
  id: string;
  item: SearchFilter;
}

export interface ProgramStatusRecordsDelta {
  added: StatusRecord[];
  removed: string[];
  patched: ProgramStatusRecordsPatchEntry[];
  reordered?: string[];
}

export interface ProgramStatusRecordsPatchEntry {
  id: string;
  item: StatusRecord;
}

export interface ProgramWorkshopsDelta {
  added: Workshop[];
  removed: string[];
  patched: ProgramWorkshopsPatchEntry[];
  reordered?: string[];
}

export interface ProgramWorkshopsPatchEntry {
  id: string;
  item: Workshop;
}

export interface ProgramSurveysDelta {
  added: Survey[];
  removed: string[];
  patched: ProgramSurveysPatchEntry[];
  reordered?: string[];
}

export interface ProgramSurveysPatchEntry {
  id: string;
  item: Survey;
}

export interface ProgramIssuesDelta {
  added: Issue[];
  removed: string[];
  patched: ProgramIssuesPatchEntry[];
  reordered?: string[];
}

export interface ProgramIssuesPatchEntry {
  id: string;
  item: Issue;
}

export interface ProgramAuditEventsDelta {
  added: AuditEvent[];
  removed: string[];
  patched: ProgramAuditEventsPatchEntry[];
  reordered?: string[];
}

export interface ProgramAuditEventsPatchEntry {
  id: string;
  item: AuditEvent;
}

export interface ProgramTemplatesDelta {
  added: TemplateRecord[];
  removed: string[];
  patched: ProgramTemplatesPatchEntry[];
  reordered?: string[];
}

export interface ProgramTemplatesPatchEntry {
  id: string;
  item: TemplateRecord;
}

export interface ProgramKnowledgeDelta {
  added: KnowledgeRecord[];
  removed: string[];
  patched: ProgramKnowledgePatchEntry[];
  reordered?: string[];
}

export interface ProgramKnowledgePatchEntry {
  id: string;
  item: KnowledgeRecord;
}

export interface ProgramBenchmarksDelta {
  added: BenchmarkRecord[];
  removed: string[];
  patched: ProgramBenchmarksPatchEntry[];
  reordered?: string[];
}

export interface ProgramBenchmarksPatchEntry {
  id: string;
  item: BenchmarkRecord;
}

export interface ProgramTracesDelta {
  added: TraceLink[];
  removed: string[];
  patched: ProgramTracesPatchEntry[];
  reordered?: string[];
}

export interface ProgramTracesPatchEntry {
  id: string;
  item: TraceLink;
}

