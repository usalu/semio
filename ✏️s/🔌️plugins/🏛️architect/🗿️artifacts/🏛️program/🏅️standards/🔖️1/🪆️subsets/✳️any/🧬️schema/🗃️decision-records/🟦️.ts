/** 🗃️ ProgramSnapshot artifact schema: the risk, conflict, requirement, decision, quality, compliance, collaboration, audit, knowledge and benchmark entities and the governance record. */
import { type Binary64 } from "@semio-tech/framework/value/ieee754";
import type { EntityHeader, IssueSeverity, LifecycleStatus, Priority, RiskLevel, TaggedNote, TextField, TraceLink, ValidationStatus } from "../🪪️entity-foundation/🟦️.ts";

//#region 🔖️Entities
export interface Risk extends EntityHeader {
  riskStatement: TextField;
  category: string;
  probability: RiskLevel;
  impact: RiskLevel;
  riskScore: Binary64 | null;
  causes: string[];
  effects: string[];
  affectedElementIds: string[];
  affectedRequirementIds: string[];
  mitigation: string[];
  contingency: string[];
  ownerId: string | null;
  reviewDate: string | null;
  triggerIndicators: string[];
  residualProbability: RiskLevel | null;
  residualImpact: RiskLevel | null;
  relatedConflictIds: string[];
  escalationPath: string[];
  monitoringPlan: string | null;
}

export type ConflictKind = "adjacency" | "capacity" | "schedule" | "budget" | "regulatory" | "operational" | "environmental" | "security" | "priority";

export interface Conflict extends EntityHeader {
  kind: ConflictKind;
  summary: TextField;
  entityAId: string;
  entityBId: string;
  severity: IssueSeverity;
  detectedBy: string | null;
  detectionDate: string | null;
  tradeOffOptions: string[];
  recommendedResolution: TextField | null;
  decisionId: string | null;
  stakeholderIds: string[];
  requirementIds: string[];
  costImpact: Binary64 | null;
  scheduleImpact: string | null;
  qualityImpact: string[];
  resolutionStatus: ValidationStatus;
  ownerId: string | null;
  escalationLevel: string | null;
  relatedRiskIds: string[];
}

export type RequirementKind = "functional" | "spatial" | "performance" | "regulatory" | "operational" | "technical" | "aesthetic" | "sustainability";

export interface Requirement extends EntityHeader {
  code: string;
  kind: RequirementKind;
  statement: TextField;
  rationale: TextField | null;
  source: string | null;
  stakeholderIds: string[];
  elementIds: string[];
  functionIds: string[];
  parentRequirementId: string | null;
  childRequirementIds: string[];
  acceptanceCriteria: string[];
  verificationMethod: string | null;
  validationStatus: ValidationStatus;
  conflictIds: string[];
  riskIds: string[];
  costEstimate: Binary64 | null;
  scheduleConstraint: string | null;
  regulatoryRefs: string[];
  traceLinks: TraceLink[];
  supersededBy: string | null;
}

export interface PriorityRecord extends EntityHeader {
  subjectId: string;
  subjectKind: string;
  rankedPriority: Priority;
  rank: number | null;
  weight: Binary64 | null;
  rationale: TextField | null;
  decisionId: string | null;
  stakeholderIds: string[];
  effectiveFrom: string | null;
  effectiveUntil: string | null;
  reviewCycle: string | null;
  dependencies: string[];
  conflicts: string[];
  scoringMethod: string | null;
  score: Binary64 | null;
  criteria: string[];
  approvedBy: string | null;
  approvalDate: string | null;
  rankingNotes: TaggedNote[];
}

export interface Scenario extends EntityHeader {
  code: string;
  hypothesis: TextField;
  assumptions: string[];
  variables: string[];
  elementIds: string[];
  requirementIds: string[];
  growthPlanId: string | null;
  probability: Binary64 | null;
  impactSummary: TextField | null;
  costDelta: Binary64 | null;
  areaDelta: Binary64 | null;
  headcountDelta: Binary64 | null;
  scheduleDelta: string | null;
  riskIds: string[];
  optionIds: string[];
  baseline: boolean;
  preferred: boolean;
  analysisIds: string[];
  ownerId: string | null;
}

export interface OptionEvaluation extends EntityHeader {
  optionName: string;
  optionDescription: TextField;
  scenarioId: string | null;
  criteriaIds: string[];
  scores: Binary64[];
  weightedScore: Binary64 | null;
  costEstimate: Binary64 | null;
  scheduleEstimate: string | null;
  riskSummary: string[];
  benefits: string[];
  drawbacks: string[];
  assumptions: string[];
  dependencies: string[];
  stakeholderFeedback: TaggedNote[];
  recommendation: string | null;
  decisionId: string | null;
  evaluationStatus: ValidationStatus;
  evaluatorIds: string[];
  evaluationDate: string | null;
}

export interface Decision extends EntityHeader {
  decisionStatement: TextField;
  context: TextField;
  optionsConsidered: string[];
  selectedOptionId: string | null;
  rationale: TextField;
  decisionMakerIds: string[];
  consultedIds: string[];
  informedIds: string[];
  decisionDate: string | null;
  effectiveDate: string | null;
  reversalConditions: string[];
  impactedRequirementIds: string[];
  impactedElementIds: string[];
  costImpact: Binary64 | null;
  scheduleImpact: string | null;
  riskImpact: string[];
  approvalStatus: ValidationStatus;
  meetingRef: string | null;
  artifactRefs: string[];
}

export interface ValidationRecord extends EntityHeader {
  subjectId: string;
  subjectKind: string;
  validationType: string;
  method: string | null;
  criteria: string[];
  result: ValidationStatus;
  evidence: string[];
  validatorIds: string[];
  validationDate: string | null;
  nextReviewDate: string | null;
  findings: string[];
  nonConformities: string[];
  correctiveActions: string[];
  waivers: string[];
  standards: string[];
  traceLinks: TraceLink[];
  reportId: string | null;
  confidenceLevel: string | null;
  validationNotes: TaggedNote[];
}

export interface PerformanceCriterion extends EntityHeader {
  criterion: string;
  metric: string;
  target: Binary64 | null;
  unit: string | null;
  minimum: Binary64 | null;
  maximum: Binary64 | null;
  measurementMethod: string | null;
  frequency: string | null;
  requirementIds: string[];
  elementIds: string[];
  baseline: Binary64 | null;
  benchmarkRef: string | null;
  weight: Binary64 | null;
  dataSource: string | null;
  reportingCadence: string | null;
  ownerId: string | null;
  verificationPlan: string | null;
  penaltyThreshold: Binary64 | null;
  incentiveThreshold: Binary64 | null;
}

export interface QualityRecord extends EntityHeader {
  qualityTopic: string;
  standard: string | null;
  targetLevel: string | null;
  inspectionPoints: string[];
  acceptanceCriteria: string[];
  testingRequirements: string[];
  sampleRate: string | null;
  defectCategories: string[];
  correctiveActionProcess: string[];
  elementIds: string[];
  requirementIds: string[];
  supplierRequirements: string[];
  documentationRequirements: string[];
  trainingRequirements: string[];
  auditSchedule: string | null;
  kpis: string[];
  ownerId: string | null;
  certificationTargets: string[];
  continuousImprovement: string[];
}

export interface ArtifactRecord extends EntityHeader {
  documentType: string;
  title: string;
  version: string;
  fileRef: string | null;
  format: string | null;
  authorIds: string[];
  reviewerIds: string[];
  approverIds: string[];
  issueDate: string | null;
  revisionDate: string | null;
  distributionList: string[];
  relatedEntityIds: string[];
  classification: string | null;
  retentionPeriod: string | null;
  accessControls: string[];
  supersedes: string | null;
  documentStatus: LifecycleStatus;
  checksum: string | null;
  sourceSystem: string | null;
}

export interface Assumption extends EntityHeader {
  statement: TextField;
  basis: TextField | null;
  confidenceLevel: string | null;
  impactIfFalse: TextField | null;
  relatedEntityIds: string[];
  validationStatus: ValidationStatus;
  validatedBy: string | null;
  validationDate: string | null;
  ownerId: string | null;
  reviewCycle: string | null;
  source: string | null;
  category: string | null;
  dependencies: string[];
  mitigation: string[];
  linkedRequirementIds: string[];
  linkedRiskIds: string[];
  expirationDate: string | null;
  statusNotes: TaggedNote[];
  artifactRefs: string[];
}

export interface ConstraintRecord extends EntityHeader {
  constraintType: string;
  summary: TextField;
  severity: RiskLevel;
  affectedEntityIds: string[];
  source: string | null;
  regulatoryBasis: string[];
  mitigationOptions: string[];
  ownerId: string | null;
  effectiveDate: string | null;
  expiryDate: string | null;
  waiverStatus: string | null;
  waiverApprover: string | null;
  impactAssessment: TextField | null;
  resolutionPlan: string[];
  relatedRequirementIds: string[];
  relatedDecisionIds: string[];
  monitoringFrequency: string | null;
  complianceStatus: ValidationStatus;
  exceptions: string[];
  traceLinks: TraceLink[];
  escalationContactId: string | null;
}

export interface ComplianceRecord extends EntityHeader {
  standardRef: string;
  obligation: TextField;
  complianceStatus: ValidationStatus;
  evidenceRefs: string[];
  auditorId: string | null;
  auditDate: string | null;
  nextReview: string | null;
  affectedEntityIds: string[];
  gapAnalysis: string[];
  remediationPlan: string[];
  ownerId: string | null;
  severity: RiskLevel;
  regulatoryBody: string | null;
  certificationTarget: string | null;
  waiverStatus: string | null;
  relatedRequirementIds: string[];
  monitoringMethod: string | null;
  reportingFrequency: string | null;
  penalties: string[];
  correctiveActions: string[];
  artifactRefs: string[];
}

export interface ApprovalRecord extends EntityHeader {
  approvalType: string;
  subjectId: string;
  approverIds: string[];
  approvalDate: string | null;
  conditions: string[];
  approvalStatus: LifecycleStatus;
  expiryDate: string | null;
  delegationChain: string[];
  evidenceRefs: string[];
  relatedDecisionId: string | null;
  relatedChangeId: string | null;
  authorityBasis: string[];
  signatureMethod: string | null;
  rejectionReason: TextField | null;
  resubmissionDate: string | null;
  notificationList: string[];
  workflowStep: string | null;
  version: string | null;
  auditTrailRef: string | null;
}

export interface MeetingRecord extends EntityHeader {
  meetingType: string;
  scheduledDate: string | null;
  duration: string | null;
  location: string | null;
  chairId: string | null;
  attendeeIds: string[];
  agendaItems: string[];
  minutes: TextField | null;
  actionItems: string[];
  decisionsMade: string[];
  artifactRefs: string[];
  followUpDate: string | null;
  recordingRef: string | null;
  quorumMet: boolean;
  meetingStatus: LifecycleStatus;
  workshopId: string | null;
  stakeholderIds: string[];
  requirementIds: string[];
  issueIds: string[];
  approvalIds: string[];
}

export interface ChangeRecord extends EntityHeader {
  changeType: string;
  summary: TextField;
  reason: TextField;
  requestedBy: string | null;
  approvedBy: string | null;
  changeDate: string | null;
  effectiveDate: string | null;
  impactedEntityIds: string[];
  beforeSnapshot: string | null;
  afterSnapshot: string | null;
  costImpact: Binary64 | null;
  scheduleImpact: string | null;
  riskImpact: string[];
  approvalStatus: ValidationStatus;
  rollbackPlan: string[];
  communicationPlan: string[];
  versionFrom: string | null;
  versionTo: string | null;
  auditEventIds: string[];
}

export interface CollaborationRecord extends EntityHeader {
  sessionType: string;
  title: string;
  participants: string[];
  facilitatorId: string | null;
  startTime: string | null;
  endTime: string | null;
  location: string | null;
  agenda: string[];
  outcomes: string[];
  actionItems: string[];
  decisionIds: string[];
  issueIds: string[];
  documentIds: string[];
  recordingRef: string | null;
  feedback: TaggedNote[];
  followUpDate: string | null;
  workshopId: string | null;
  surveyId: string | null;
}

export type AnalysisKind = "gap" | "conflict" | "dependency" | "capacity" | "demand" | "utilization" | "workflow" | "risk" | "cost" | "scenario" | "sensitivity" | "impact" | "trend" | "requirementComparison" | "requirementClustering" | "requirementFiltering" | "requirementSorting" | "requirementScoring" | "requirementWeighting" | "relationshipAnalysis";

export interface AnalysisRecord extends EntityHeader {
  kind: AnalysisKind;
  title: string;
  parameters: string[];
  inputEntityIds: string[];
  outputSummary: TextField;
  findings: string[];
  metrics: string[];
  charts: string[];
  runBy: string | null;
  runAt: string | null;
  durationMs: bigint | null;
  toolVersion: string | null;
  scenarioId: string | null;
  reportId: string | null;
  confidence: string | null;
  limitations: string[];
  recommendations: string[];
  rawResultRef: string | null;
}

export type ReportKind = "executiveSummary" | "programOverview" | "stakeholderSummary" | "requirementsMatrix" | "adjacencyMatrix" | "gapAnalysis" | "riskRegister" | "decisionLog" | "validationSummary" | "recommendation" | "userSummary" | "functionalSummary" | "capacitySummary" | "workflowSummary" | "complianceSummary" | "costSummary" | "scheduleSummary" | "changeSummary" | "openIssueSummary" | "prioritySummary" | "scenarioSummary";

export interface ReportRecord extends EntityHeader {
  kind: ReportKind;
  title: string;
  audience: string[];
  sections: string[];
  generatedAt: string | null;
  generatedBy: string | null;
  analysisIds: string[];
  format: string | null;
  fileRef: string | null;
  distributionList: string[];
  approvalStatus: ValidationStatus;
  approverId: string | null;
  version: string;
  templateId: string | null;
  parameters: string[];
  confidentiality: string | null;
  expiryDate: string | null;
  relatedDecisionIds: string[];
}

export interface SearchFilter extends EntityHeader {
  filterName: string;
  filterDescription: TextField | null;
  keywords: string[];
  categories: string[];
  ownerIds: string[];
  statuses: LifecycleStatus[];
  priorities: Priority[];
  sources: string[];
  dateFrom: string | null;
  dateTo: string | null;
  entityKinds: string[];
  tagFilters: string[];
  sortField: string | null;
  sortDirection: string | null;
  isPublic: boolean;
  createdBy: string | null;
  lastUsed: string | null;
  useCount: bigint;
  pinned: boolean;
}

export interface StatusRecord extends EntityHeader {
  subjectId: string;
  subjectKind: string;
  recordStatus: LifecycleStatus;
  previousStatus: LifecycleStatus | null;
  changedBy: string | null;
  changedAt: string | null;
  reason: TextField | null;
  blockers: string[];
  nextActions: string[];
  dueDate: string | null;
  progressPercent: Binary64 | null;
  health: string | null;
  escalationLevel: string | null;
  relatedIssueIds: string[];
  relatedRiskIds: string[];
  milestoneId: string | null;
  reportingPeriod: string | null;
  statusNotes: TaggedNote[];
}

export interface Workshop extends EntityHeader {
  workshopType: string;
  objectives: string[];
  agenda: string[];
  facilitatorId: string | null;
  participants: string[];
  scheduledStart: string | null;
  scheduledEnd: string | null;
  location: string | null;
  materials: string[];
  methods: string[];
  outputs: string[];
  decisions: string[];
  issues: string[];
  followUpActions: string[];
  feedback: TaggedNote[];
  recordingRef: string | null;
  budget: Binary64 | null;
  workshopStatus: LifecycleStatus;
  surveyIds: string[];
}

export interface Survey extends EntityHeader {
  surveyType: string;
  title: string;
  objectives: string[];
  questions: string[];
  targetAudience: string[];
  distributionChannels: string[];
  launchDate: string | null;
  closeDate: string | null;
  responseCount: number;
  responseRate: Binary64 | null;
  findings: string[];
  themes: string[];
  recommendations: string[];
  confidentiality: string | null;
  consentProcess: string[];
  analysisId: string | null;
  workshopId: string | null;
  ownerId: string | null;
  surveyStatus: LifecycleStatus;
}

export interface Issue extends EntityHeader {
  issueType: string;
  summary: TextField;
  issueDescription: TextField;
  severity: IssueSeverity;
  issuePriority: Priority;
  reporterId: string | null;
  assigneeId: string | null;
  affectedEntityIds: string[];
  rootCause: TextField | null;
  resolution: TextField | null;
  workaround: TextField | null;
  dueDate: string | null;
  resolvedDate: string | null;
  relatedConflictIds: string[];
  relatedRiskIds: string[];
  decisionId: string | null;
  comments: TaggedNote[];
  attachments: string[];
  escalationLevel: string | null;
}

export type AuditAction = "created" | "updated" | "deleted" | "reviewed" | "approved" | "rejected" | "exported" | "imported" | "merged" | "archived";

export interface AuditEvent extends EntityHeader {
  action: AuditAction;
  actorId: string | null;
  subjectId: string;
  subjectKind: string;
  timestamp: string;
  details: TextField;
  beforeState: string | null;
  afterState: string | null;
  ipAddress: string | null;
  client: string | null;
  sessionId: string | null;
  changeRecordId: string | null;
  traceLink: TraceLink | null;
  success: boolean;
  errorMessage: string | null;
  correlationId: string | null;
  complianceTags: string[];
  retentionUntil: string | null;
}

export interface TemplateRecord extends EntityHeader {
  templateType: string;
  sector: string | null;
  projectType: string | null;
  version: string;
  contentRef: string | null;
  entityKinds: string[];
  defaultFields: string[];
  checklists: string[];
  standards: string[];
  applicability: string[];
  authorId: string | null;
  approvalStatus: ValidationStatus;
  usageCount: bigint;
  lastApplied: string | null;
  customizationNotes: string[];
  relatedKnowledgeIds: string[];
  benchmarkIds: string[];
  license: string | null;
  sourceOrganization: string | null;
}

export interface KnowledgeRecord extends EntityHeader {
  topic: string;
  category: string;
  summary: TextField;
  content: TextField;
  sources: string[];
  references: string[];
  lessonsLearned: string[];
  bestPractices: string[];
  applicableSectors: string[];
  relatedEntityKinds: string[];
  authorIds: string[];
  expertiseLevel: string | null;
  validationStatus: ValidationStatus;
  lastReviewed: string | null;
  keywords: string[];
  attachments: string[];
  citations: string[];
  usageCount: bigint;
}

export interface BenchmarkRecord extends EntityHeader {
  benchmarkName: string;
  sector: string;
  metric: string;
  value: Binary64;
  unit: string;
  sampleSize: number | null;
  source: string | null;
  collectionYear: number | null;
  geography: string | null;
  buildingType: string | null;
  confidence: string | null;
  methodology: string | null;
  applicableElementKinds: string[];
  relatedRequirementIds: string[];
  comparisonNotes: string[];
  limitations: string[];
  license: string | null;
  knowledgeId: string | null;
  lastVerified: string | null;
}

export interface Governance {
  id: string;
  framework: string;
  roles: string[];
  responsibilities: string[];
  approvalMatrix: string[];
  escalationPaths: string[];
  meetingCadence: string[];
  decisionRights: string[];
  changeControlProcess: string[];
  qualityPolicy: TextField;
  riskAppetite: string | null;
  complianceObligations: string[];
  auditSchedule: string | null;
  documentControl: string[];
  stakeholderEngagementPlan: string[];
  ethicsPolicy: string[];
  dataGovernance: string[];
  ownerId: string | null;
  reviewCycle: string | null;
  reviewHierarchy: string[];
  policyOwnershipId: string | null;
  requirementOwnershipId: string | null;
  riskOwnershipId: string | null;
  reportingFrequency: string | null;
  accountabilityRules: string[];
  exceptionManagement: string[];
  governancePerformance: string[];
}
//#endregion 🔖️Entities
