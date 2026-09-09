/** 🧬️ ProgramSnapshot diff schema — sparse field delta. */

export interface ProgramDiff {
  /** @state artifact */
  artifact?: ProgramArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  meta?: ProgramMeta;
  /** @state artifact */
  project?: ProjectDefinition;
  /** @state artifact */
  stakeholders?: ProgramStakeholdersDelta;
  /** @state artifact */
  users?: ProgramUsersDelta;
  /** @state artifact */
  activities?: ProgramActivitiesDelta;
  /** @state artifact */
  functions?: ProgramFunctionsDelta;
  /** @state artifact */
  elements?: ProgramElementsDelta;
  /** @state artifact */
  quantities?: ProgramQuantitiesDelta;
  /** @state artifact */
  relationships?: ProgramRelationshipsDelta;
  /** @state artifact */
  adjacencies?: ProgramAdjacenciesDelta;
  /** @state artifact */
  processes?: ProgramProcessesDelta;
  /** @state artifact */
  flows?: ProgramFlowsDelta;
  /** @state artifact */
  accessRules?: ProgramAccessRulesDelta;
  /** @state artifact */
  operations?: ProgramOperationsDelta;
  /** @state artifact */
  equipment?: ProgramEquipmentDelta;
  /** @state artifact */
  resources?: ProgramResourcesDelta;
  /** @state artifact */
  storage?: ProgramStorageDelta;
  /** @state artifact */
  environmental?: ProgramEnvironmentalDelta;
  /** @state artifact */
  humanFactors?: ProgramHumanFactorsDelta;
  /** @state artifact */
  accessibility?: ProgramAccessibilityDelta;
  /** @state artifact */
  privacy?: ProgramPrivacyDelta;
  /** @state artifact */
  safety?: ProgramSafetyDelta;
  /** @state artifact */
  security?: ProgramSecurityDelta;
  /** @state artifact */
  regulatory?: ProgramRegulatoryDelta;
  /** @state artifact */
  siteContext?: ProgramSiteContextDelta;
  /** @state artifact */
  organizational?: ProgramOrganizationalDelta;
  /** @state artifact */
  services?: ProgramServicesDelta;
  /** @state artifact */
  infrastructure?: ProgramInfrastructureDelta;
  /** @state artifact */
  information?: ProgramInformationDelta;
  /** @state artifact */
  communication?: ProgramCommunicationDelta;
  /** @state artifact */
  wayfinding?: ProgramWayfindingDelta;
  /** @state artifact */
  schedules?: ProgramSchedulesDelta;
  /** @state artifact */
  flexibility?: ProgramFlexibilityDelta;
  /** @state artifact */
  growth?: ProgramGrowthDelta;
  /** @state artifact */
  sustainability?: ProgramSustainabilityDelta;
  /** @state artifact */
  resilience?: ProgramResilienceDelta;
  /** @state artifact */
  costs?: ProgramCostsDelta;
  /** @state artifact */
  delivery?: ProgramDeliveryDelta;
  /** @state artifact */
  risks?: ProgramRisksDelta;
  /** @state artifact */
  conflicts?: ProgramConflictsDelta;
  /** @state artifact */
  requirements?: ProgramRequirementsDelta;
  /** @state artifact */
  priorities?: ProgramPrioritiesDelta;
  /** @state artifact */
  scenarios?: ProgramScenariosDelta;
  /** @state artifact */
  options?: ProgramOptionsDelta;
  /** @state artifact */
  decisions?: ProgramDecisionsDelta;
  /** @state artifact */
  validations?: ProgramValidationsDelta;
  /** @state artifact */
  performance?: ProgramPerformanceDelta;
  /** @state artifact */
  quality?: ProgramQualityDelta;
  /** @state artifact */
  documents?: ProgramDocumentsDelta;
  /** @state artifact */
  assumptions?: ProgramAssumptionsDelta;
  /** @state artifact */
  constraints?: ProgramConstraintsDelta;
  /** @state artifact */
  complianceRecords?: ProgramComplianceRecordsDelta;
  /** @state artifact */
  approvals?: ProgramApprovalsDelta;
  /** @state artifact */
  meetings?: ProgramMeetingsDelta;
  /** @state artifact */
  changes?: ProgramChangesDelta;
  /** @state artifact */
  collaboration?: ProgramCollaborationDelta;
  /** @state artifact */
  analyses?: ProgramAnalysesDelta;
  /** @state artifact */
  reports?: ProgramReportsDelta;
  /** @state artifact */
  searchFilters?: ProgramSearchFiltersDelta;
  /** @state artifact */
  statusRecords?: ProgramStatusRecordsDelta;
  /** @state artifact */
  workshops?: ProgramWorkshopsDelta;
  /** @state artifact */
  surveys?: ProgramSurveysDelta;
  /** @state artifact */
  issues?: ProgramIssuesDelta;
  /** @state artifact */
  auditEvents?: ProgramAuditEventsDelta;
  /** @state artifact */
  templates?: ProgramTemplatesDelta;
  /** @state artifact */
  knowledge?: ProgramKnowledgeDelta;
  /** @state artifact */
  benchmarks?: ProgramBenchmarksDelta;
  /** @state artifact */
  traces?: ProgramTracesDelta;
  /** @state artifact */
  governance?: Governance;
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

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramDiffGuardReject = (at: string, why: string): never => {
  throw new architectProgramDiffGuardRefusal(at, why);
};

type architectProgramDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectProgramDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectProgramDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectProgramDiffGuardReject(at, "value is not an object");
export const architectProgramDiffGuardArray = (value: unknown, at: string, bounds: architectProgramDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectProgramDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectProgramDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectProgramDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectProgramDiffGuardString = (value: unknown, at: string, bounds: architectProgramDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectProgramDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectProgramDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectProgramDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectProgramDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectProgramDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectProgramDiffGuardReject(at, "value is not a boolean"));
export const architectProgramDiffGuardNumber = (value: unknown, at: string, bounds: architectProgramDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectProgramDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectProgramDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectProgramDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectProgramDiffGuardInteger = (value: unknown, at: string, bounds: architectProgramDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramDiffGuardNumber(value, at, bounds) : architectProgramDiffGuardReject(at, "value is not an integer");
export const architectProgramDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface ProgramStringList {
  readonly values: readonly string[];
}

export function parseProgramStringList(value: unknown, at = "$"): ProgramStringList {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    values: architectProgramDiffGuardArray(row["values"], `${at}.values`).map((item, index) => architectProgramDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export type StakeholderPatch = Readonly<Record<string, unknown>>;

export function parseStakeholderPatch(value: unknown, at = "$"): StakeholderPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramStakeholdersPatchEntry(value: unknown, at = "$"): ProgramStakeholdersPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStakeholderPatch(row["patch"], `${at}.patch`),
  };
}

export type UserProfilePatch = Readonly<Record<string, unknown>>;

export function parseUserProfilePatch(value: unknown, at = "$"): UserProfilePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramUsersPatchEntry(value: unknown, at = "$"): ProgramUsersPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseUserProfilePatch(row["patch"], `${at}.patch`),
  };
}

export type ActivityPatch = Readonly<Record<string, unknown>>;

export function parseActivityPatch(value: unknown, at = "$"): ActivityPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramActivitiesPatchEntry(value: unknown, at = "$"): ProgramActivitiesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseActivityPatch(row["patch"], `${at}.patch`),
  };
}

export type FunctionPatch = Readonly<Record<string, unknown>>;

export function parseFunctionPatch(value: unknown, at = "$"): FunctionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramFunctionsPatchEntry(value: unknown, at = "$"): ProgramFunctionsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFunctionPatch(row["patch"], `${at}.patch`),
  };
}

export type ProgramElementPatch = Readonly<Record<string, unknown>>;

export function parseProgramElementPatch(value: unknown, at = "$"): ProgramElementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramElementsPatchEntry(value: unknown, at = "$"): ProgramElementsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProgramElementPatch(row["patch"], `${at}.patch`),
  };
}

export type QuantityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseQuantityRequirementPatch(value: unknown, at = "$"): QuantityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramQuantitiesPatchEntry(value: unknown, at = "$"): ProgramQuantitiesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQuantityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type RelationshipPatch = Readonly<Record<string, unknown>>;

export function parseRelationshipPatch(value: unknown, at = "$"): RelationshipPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRelationshipsPatchEntry(value: unknown, at = "$"): ProgramRelationshipsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRelationshipPatch(row["patch"], `${at}.patch`),
  };
}

export type AdjacencyPatch = Readonly<Record<string, unknown>>;

export function parseAdjacencyPatch(value: unknown, at = "$"): AdjacencyPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAdjacenciesPatchEntry(value: unknown, at = "$"): ProgramAdjacenciesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAdjacencyPatch(row["patch"], `${at}.patch`),
  };
}

export type ProcessPatch = Readonly<Record<string, unknown>>;

export function parseProcessPatch(value: unknown, at = "$"): ProcessPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramProcessesPatchEntry(value: unknown, at = "$"): ProgramProcessesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProcessPatch(row["patch"], `${at}.patch`),
  };
}

export type FlowRequirementPatch = Readonly<Record<string, unknown>>;

export function parseFlowRequirementPatch(value: unknown, at = "$"): FlowRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramFlowsPatchEntry(value: unknown, at = "$"): ProgramFlowsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlowRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type AccessRulePatch = Readonly<Record<string, unknown>>;

export function parseAccessRulePatch(value: unknown, at = "$"): AccessRulePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAccessRulesPatchEntry(value: unknown, at = "$"): ProgramAccessRulesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessRulePatch(row["patch"], `${at}.patch`),
  };
}

export type OperationalRequirementPatch = Readonly<Record<string, unknown>>;

export function parseOperationalRequirementPatch(value: unknown, at = "$"): OperationalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramOperationsPatchEntry(value: unknown, at = "$"): ProgramOperationsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOperationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type EquipmentPatch = Readonly<Record<string, unknown>>;

export function parseEquipmentPatch(value: unknown, at = "$"): EquipmentPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramEquipmentPatchEntry(value: unknown, at = "$"): ProgramEquipmentPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEquipmentPatch(row["patch"], `${at}.patch`),
  };
}

export type ResourcePatch = Readonly<Record<string, unknown>>;

export function parseResourcePatch(value: unknown, at = "$"): ResourcePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramResourcesPatchEntry(value: unknown, at = "$"): ProgramResourcesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResourcePatch(row["patch"], `${at}.patch`),
  };
}

export type StorageRequirementPatch = Readonly<Record<string, unknown>>;

export function parseStorageRequirementPatch(value: unknown, at = "$"): StorageRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramStoragePatchEntry(value: unknown, at = "$"): ProgramStoragePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStorageRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type EnvironmentalRequirementPatch = Readonly<Record<string, unknown>>;

export function parseEnvironmentalRequirementPatch(value: unknown, at = "$"): EnvironmentalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramEnvironmentalPatchEntry(value: unknown, at = "$"): ProgramEnvironmentalPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEnvironmentalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type HumanFactorRequirementPatch = Readonly<Record<string, unknown>>;

export function parseHumanFactorRequirementPatch(value: unknown, at = "$"): HumanFactorRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramHumanFactorsPatchEntry(value: unknown, at = "$"): ProgramHumanFactorsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseHumanFactorRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type AccessibilityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseAccessibilityRequirementPatch(value: unknown, at = "$"): AccessibilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAccessibilityPatchEntry(value: unknown, at = "$"): ProgramAccessibilityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type PrivacyRequirementPatch = Readonly<Record<string, unknown>>;

export function parsePrivacyRequirementPatch(value: unknown, at = "$"): PrivacyRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramPrivacyPatchEntry(value: unknown, at = "$"): ProgramPrivacyPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePrivacyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type SafetyRequirementPatch = Readonly<Record<string, unknown>>;

export function parseSafetyRequirementPatch(value: unknown, at = "$"): SafetyRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSafetyPatchEntry(value: unknown, at = "$"): ProgramSafetyPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSafetyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type SecurityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseSecurityRequirementPatch(value: unknown, at = "$"): SecurityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSecurityPatchEntry(value: unknown, at = "$"): ProgramSecurityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSecurityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type RegulatoryRequirementPatch = Readonly<Record<string, unknown>>;

export function parseRegulatoryRequirementPatch(value: unknown, at = "$"): RegulatoryRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRegulatoryPatchEntry(value: unknown, at = "$"): ProgramRegulatoryPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRegulatoryRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type SiteContextPatch = Readonly<Record<string, unknown>>;

export function parseSiteContextPatch(value: unknown, at = "$"): SiteContextPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSiteContextPatchEntry(value: unknown, at = "$"): ProgramSiteContextPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSiteContextPatch(row["patch"], `${at}.patch`),
  };
}

export type OrganizationalRequirementPatch = Readonly<Record<string, unknown>>;

export function parseOrganizationalRequirementPatch(value: unknown, at = "$"): OrganizationalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramOrganizationalPatchEntry(value: unknown, at = "$"): ProgramOrganizationalPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOrganizationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type ServiceRequirementPatch = Readonly<Record<string, unknown>>;

export function parseServiceRequirementPatch(value: unknown, at = "$"): ServiceRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramServicesPatchEntry(value: unknown, at = "$"): ProgramServicesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseServiceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type InfrastructureRequirementPatch = Readonly<Record<string, unknown>>;

export function parseInfrastructureRequirementPatch(value: unknown, at = "$"): InfrastructureRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramInfrastructurePatchEntry(value: unknown, at = "$"): ProgramInfrastructurePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInfrastructureRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type InformationRequirementPatch = Readonly<Record<string, unknown>>;

export function parseInformationRequirementPatch(value: unknown, at = "$"): InformationRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramInformationPatchEntry(value: unknown, at = "$"): ProgramInformationPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInformationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type CommunicationRequirementPatch = Readonly<Record<string, unknown>>;

export function parseCommunicationRequirementPatch(value: unknown, at = "$"): CommunicationRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramCommunicationPatchEntry(value: unknown, at = "$"): ProgramCommunicationPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCommunicationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type WayfindingRequirementPatch = Readonly<Record<string, unknown>>;

export function parseWayfindingRequirementPatch(value: unknown, at = "$"): WayfindingRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramWayfindingPatchEntry(value: unknown, at = "$"): ProgramWayfindingPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWayfindingRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type ScheduleRequirementPatch = Readonly<Record<string, unknown>>;

export function parseScheduleRequirementPatch(value: unknown, at = "$"): ScheduleRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSchedulesPatchEntry(value: unknown, at = "$"): ProgramSchedulesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScheduleRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type FlexibilityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseFlexibilityRequirementPatch(value: unknown, at = "$"): FlexibilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramFlexibilityPatchEntry(value: unknown, at = "$"): ProgramFlexibilityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlexibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type GrowthPlanPatch = Readonly<Record<string, unknown>>;

export function parseGrowthPlanPatch(value: unknown, at = "$"): GrowthPlanPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramGrowthPatchEntry(value: unknown, at = "$"): ProgramGrowthPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseGrowthPlanPatch(row["patch"], `${at}.patch`),
  };
}

export type SustainabilityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseSustainabilityRequirementPatch(value: unknown, at = "$"): SustainabilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSustainabilityPatchEntry(value: unknown, at = "$"): ProgramSustainabilityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSustainabilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type ResilienceRequirementPatch = Readonly<Record<string, unknown>>;

export function parseResilienceRequirementPatch(value: unknown, at = "$"): ResilienceRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramResiliencePatchEntry(value: unknown, at = "$"): ProgramResiliencePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResilienceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type CostRequirementPatch = Readonly<Record<string, unknown>>;

export function parseCostRequirementPatch(value: unknown, at = "$"): CostRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramCostsPatchEntry(value: unknown, at = "$"): ProgramCostsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCostRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type DeliveryConstraintPatch = Readonly<Record<string, unknown>>;

export function parseDeliveryConstraintPatch(value: unknown, at = "$"): DeliveryConstraintPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramDeliveryPatchEntry(value: unknown, at = "$"): ProgramDeliveryPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDeliveryConstraintPatch(row["patch"], `${at}.patch`),
  };
}

export type RiskPatch = Readonly<Record<string, unknown>>;

export function parseRiskPatch(value: unknown, at = "$"): RiskPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRisksPatchEntry(value: unknown, at = "$"): ProgramRisksPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRiskPatch(row["patch"], `${at}.patch`),
  };
}

export type ConflictPatch = Readonly<Record<string, unknown>>;

export function parseConflictPatch(value: unknown, at = "$"): ConflictPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramConflictsPatchEntry(value: unknown, at = "$"): ProgramConflictsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConflictPatch(row["patch"], `${at}.patch`),
  };
}

export type RequirementPatch = Readonly<Record<string, unknown>>;

export function parseRequirementPatch(value: unknown, at = "$"): RequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRequirementsPatchEntry(value: unknown, at = "$"): ProgramRequirementsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type PriorityRecordPatch = Readonly<Record<string, unknown>>;

export function parsePriorityRecordPatch(value: unknown, at = "$"): PriorityRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramPrioritiesPatchEntry(value: unknown, at = "$"): ProgramPrioritiesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePriorityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ScenarioPatch = Readonly<Record<string, unknown>>;

export function parseScenarioPatch(value: unknown, at = "$"): ScenarioPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramScenariosPatchEntry(value: unknown, at = "$"): ProgramScenariosPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScenarioPatch(row["patch"], `${at}.patch`),
  };
}

export type OptionEvaluationPatch = Readonly<Record<string, unknown>>;

export function parseOptionEvaluationPatch(value: unknown, at = "$"): OptionEvaluationPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramOptionsPatchEntry(value: unknown, at = "$"): ProgramOptionsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOptionEvaluationPatch(row["patch"], `${at}.patch`),
  };
}

export type DecisionPatch = Readonly<Record<string, unknown>>;

export function parseDecisionPatch(value: unknown, at = "$"): DecisionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramDecisionsPatchEntry(value: unknown, at = "$"): ProgramDecisionsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDecisionPatch(row["patch"], `${at}.patch`),
  };
}

export type ValidationRecordPatch = Readonly<Record<string, unknown>>;

export function parseValidationRecordPatch(value: unknown, at = "$"): ValidationRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramValidationsPatchEntry(value: unknown, at = "$"): ProgramValidationsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseValidationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type PerformanceCriterionPatch = Readonly<Record<string, unknown>>;

export function parsePerformanceCriterionPatch(value: unknown, at = "$"): PerformanceCriterionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramPerformancePatchEntry(value: unknown, at = "$"): ProgramPerformancePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePerformanceCriterionPatch(row["patch"], `${at}.patch`),
  };
}

export type QualityRecordPatch = Readonly<Record<string, unknown>>;

export function parseQualityRecordPatch(value: unknown, at = "$"): QualityRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramQualityPatchEntry(value: unknown, at = "$"): ProgramQualityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQualityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type DocumentRecordPatch = Readonly<Record<string, unknown>>;

export function parseDocumentRecordPatch(value: unknown, at = "$"): DocumentRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramDocumentsPatchEntry(value: unknown, at = "$"): ProgramDocumentsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDocumentRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type AssumptionPatch = Readonly<Record<string, unknown>>;

export function parseAssumptionPatch(value: unknown, at = "$"): AssumptionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAssumptionsPatchEntry(value: unknown, at = "$"): ProgramAssumptionsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAssumptionPatch(row["patch"], `${at}.patch`),
  };
}

export type ConstraintRecordPatch = Readonly<Record<string, unknown>>;

export function parseConstraintRecordPatch(value: unknown, at = "$"): ConstraintRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramConstraintsPatchEntry(value: unknown, at = "$"): ProgramConstraintsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConstraintRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ComplianceRecordPatch = Readonly<Record<string, unknown>>;

export function parseComplianceRecordPatch(value: unknown, at = "$"): ComplianceRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramComplianceRecordsPatchEntry(value: unknown, at = "$"): ProgramComplianceRecordsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseComplianceRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ApprovalRecordPatch = Readonly<Record<string, unknown>>;

export function parseApprovalRecordPatch(value: unknown, at = "$"): ApprovalRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramApprovalsPatchEntry(value: unknown, at = "$"): ProgramApprovalsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseApprovalRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type MeetingRecordPatch = Readonly<Record<string, unknown>>;

export function parseMeetingRecordPatch(value: unknown, at = "$"): MeetingRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramMeetingsPatchEntry(value: unknown, at = "$"): ProgramMeetingsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseMeetingRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ChangeRecordPatch = Readonly<Record<string, unknown>>;

export function parseChangeRecordPatch(value: unknown, at = "$"): ChangeRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramChangesPatchEntry(value: unknown, at = "$"): ProgramChangesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseChangeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type CollaborationRecordPatch = Readonly<Record<string, unknown>>;

export function parseCollaborationRecordPatch(value: unknown, at = "$"): CollaborationRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramCollaborationPatchEntry(value: unknown, at = "$"): ProgramCollaborationPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCollaborationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type AnalysisRecordPatch = Readonly<Record<string, unknown>>;

export function parseAnalysisRecordPatch(value: unknown, at = "$"): AnalysisRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAnalysesPatchEntry(value: unknown, at = "$"): ProgramAnalysesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAnalysisRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ReportRecordPatch = Readonly<Record<string, unknown>>;

export function parseReportRecordPatch(value: unknown, at = "$"): ReportRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramReportsPatchEntry(value: unknown, at = "$"): ProgramReportsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseReportRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type SearchFilterPatch = Readonly<Record<string, unknown>>;

export function parseSearchFilterPatch(value: unknown, at = "$"): SearchFilterPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSearchFiltersPatchEntry(value: unknown, at = "$"): ProgramSearchFiltersPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSearchFilterPatch(row["patch"], `${at}.patch`),
  };
}

export type StatusRecordPatch = Readonly<Record<string, unknown>>;

export function parseStatusRecordPatch(value: unknown, at = "$"): StatusRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramStatusRecordsPatchEntry(value: unknown, at = "$"): ProgramStatusRecordsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStatusRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type WorkshopPatch = Readonly<Record<string, unknown>>;

export function parseWorkshopPatch(value: unknown, at = "$"): WorkshopPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramWorkshopsPatchEntry(value: unknown, at = "$"): ProgramWorkshopsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWorkshopPatch(row["patch"], `${at}.patch`),
  };
}

export type SurveyPatch = Readonly<Record<string, unknown>>;

export function parseSurveyPatch(value: unknown, at = "$"): SurveyPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSurveysPatchEntry(value: unknown, at = "$"): ProgramSurveysPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSurveyPatch(row["patch"], `${at}.patch`),
  };
}

export type IssuePatch = Readonly<Record<string, unknown>>;

export function parseIssuePatch(value: unknown, at = "$"): IssuePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramIssuesPatchEntry(value: unknown, at = "$"): ProgramIssuesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseIssuePatch(row["patch"], `${at}.patch`),
  };
}

export type AuditEventPatch = Readonly<Record<string, unknown>>;

export function parseAuditEventPatch(value: unknown, at = "$"): AuditEventPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAuditEventsPatchEntry(value: unknown, at = "$"): ProgramAuditEventsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAuditEventPatch(row["patch"], `${at}.patch`),
  };
}

export type TemplateRecordPatch = Readonly<Record<string, unknown>>;

export function parseTemplateRecordPatch(value: unknown, at = "$"): TemplateRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramTemplatesPatchEntry(value: unknown, at = "$"): ProgramTemplatesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTemplateRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type KnowledgeRecordPatch = Readonly<Record<string, unknown>>;

export function parseKnowledgeRecordPatch(value: unknown, at = "$"): KnowledgeRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramKnowledgePatchEntry(value: unknown, at = "$"): ProgramKnowledgePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseKnowledgeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type BenchmarkRecordPatch = Readonly<Record<string, unknown>>;

export function parseBenchmarkRecordPatch(value: unknown, at = "$"): BenchmarkRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramBenchmarksPatchEntry(value: unknown, at = "$"): ProgramBenchmarksPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseBenchmarkRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type TraceLinkPatch = Readonly<Record<string, unknown>>;

export function parseTraceLinkPatch(value: unknown, at = "$"): TraceLinkPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramTracesPatchEntry(value: unknown, at = "$"): ProgramTracesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTraceLinkPatch(row["patch"], `${at}.patch`),
  };
}
