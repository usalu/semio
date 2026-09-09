/** 🧬️ ProgramSnapshot artifact schema — every field with its state class. */

import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
export { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

//#region 🔖️Entities
/** 🧬️ Register entity types, mirrored field-for-field from `🗄️registers/🦀️.rs` and
 * `🧱️kernel/🦀️.rs` (`EntityHeader` etc, `#[serde(flatten)]`d onto every register row —
 * hence `extends EntityHeader` below instead of a nested `header` field). `ArtifactRecord` mirrors
 * Rust `ArtifactRecord` (this schema's own field name for that register). */
export interface TextField {
  text: string;
  format?: string;
}

export interface TimestampMeta {
  created: string;
  updated: string;
  createdBy?: string;
  updatedBy?: string;
}

export interface ProgramMeta {
  schema: string;
  documentId: string;
  title: string;
  subtitle: string | null;
  purpose: TextField;
  terminology: string[];
  classification: string[];
  industrySector: string;
  projectType: string;
  locale: string;
  revision: string;
  authorIds: string[];
  sourceSystem: string | null;
  exportProfile: string | null;
  timestamps: TimestampMeta;
}

export type Priority = "mandatory" | "essential" | "preferred" | "optional" | "deferred" | "prohibited";

export interface Ownership {
  ownerId: string | null;
  authorityId: string | null;
  consultantIds: string[];
  participantIds: string[];
}

export interface ProjectDefinition {
  id: string;
  code: string;
  clientName: string;
  ownerOrganization: string;
  briefSummary: TextField;
  problemStatement: TextField;
  vision: TextField;
  mission: TextField;
  objectives: string[];
  successCriteria: string[];
  projectPriorities: Priority[];
  completionCriteria: string[];
  decisionCriteria: string[];
  scopeInclusions: string[];
  scopeExclusions: string[];
  assumptions: string[];
  constraintsSummary: string[];
  dependencies: string[];
  deliverables: string[];
  phases: string[];
  geographicContext: TextField;
  developmentContext: TextField;
  operationalContext: TextField;
  regulatoryContext: string[];
  fundingModel: string;
  ownership: Ownership;
  timestamps: TimestampMeta;
}

export type LifecycleStatus = "draft" | "proposed" | "underReview" | "validated" | "approved" | "rejected" | "deferred" | "superseded" | "archived" | "open" | "closed" | "atRisk" | "blocked" | "inProgress" | "complete";

export interface TaggedNote {
  tag: string;
  text: string;
}

export interface EntityHeader {
  id: string;
  name: string;
  description?: TextField;
  status: LifecycleStatus;
  priority: Priority;
  ownership: Ownership;
  tags?: string[];
  notes?: TaggedNote[];
  timestamps: TimestampMeta;
}

export type InfluenceLevel = "low" | "medium" | "high" | "critical";

export type EngagementLevel = "unaware" | "resistant" | "neutral" | "supportive" | "leading";

export interface Stakeholder extends EntityHeader {
  role: string;
  organization: string;
  department: string | null;
  contactEmail: string | null;
  contactPhone: string | null;
  influence: InfluenceLevel;
  interest: InfluenceLevel;
  engagement: EngagementLevel;
  expectations: string[];
  concerns: string[];
  requirementIds: string[];
  decisionAuthority: boolean;
  communicationPreferences: string[];
  reportingFrequency: string | null;
  involvementPhases: string[];
  availability: string | null;
  representativeOf: string | null;
  delegatedTo: string | null;
  relationshipToClient: string | null;
  powerInterestNotes: TaggedNote[];
  stakeholderType: string;
  influenceStrategy: string | null;
  communicationChannels: string[];
  successMetrics: string[];
}

export type UserCategory = "primary" | "secondary" | "occasional" | "service" | "visitor" | "staff" | "public";

export interface UserProfile extends EntityHeader {
  category: UserCategory;
  demographic: string | null;
  ageRange: string | null;
  abilities: string[];
  disabilities: string[];
  occupation: string | null;
  roleTitle: string | null;
  department: string | null;
  mobilityProfile: string[];
  sensoryProfile: string[];
  cognitiveProfile: string[];
  behavioralPatterns: string[];
  usageFrequency: string | null;
  usageDuration: string | null;
  peakUsageTimes: string[];
  technologyProficiency: string | null;
  preferences: string[];
  painPoints: string[];
  goals: string[];
  activityIds: string[];
  researchMethod: string | null;
  personaArchetype: string | null;
  validated: boolean;
  stakeholderIds: string[];
}

export interface QuantitySpec {
  min?: number;
  max?: number;
  target?: number;
  current?: number;
  forecast?: number;
  peak?: number;
  average?: number;
  unit: string;
}

export interface Activity extends EntityHeader {
  code: string;
  category: string;
  frequency: string | null;
  duration: string | null;
  intensity: string | null;
  participants: QuantitySpec;
  equipmentIds: string[];
  spaceRequirements: string[];
  environmentalNeeds: string[];
  privacyNeeds: string[];
  accessibilityNeeds: string[];
  adjacentActivities: string[];
  sequencing: string[];
  peakPeriods: string[];
  workflowSteps: string[];
  inputs: string[];
  outputs: string[];
  userProfileIds: string[];
  functionIds: string[];
  performanceIndicators: string[];
  activityType: string;
  locationContext: string | null;
  temporalPattern: string | null;
  supervisionLevel: string | null;
}

export type FunctionKind = "primary" | "secondary" | "support" | "administrative" | "service" | "technical" | "public" | "private" | "shared" | "restricted" | "temporary" | "future" | "operational" | "circulation";

export interface Function extends EntityHeader {
  code: string;
  kind: FunctionKind;
  purpose: TextField;
  criticality: Priority;
  performanceTargets: string[];
  serviceLevel: string | null;
  operatingHours: string | null;
  staffing: QuantitySpec;
  equipmentIds: string[];
  resourceIds: string[];
  activityIds: string[];
  elementIds: string[];
  dependencies: string[];
  interfaces: string[];
  constraints: string[];
  qualityCriteria: string[];
  regulatoryRefs: string[];
  futureChanges: string[];
  ownerStakeholderId: string | null;
  successMetrics: string[];
  hierarchyParentId: string | null;
  conflictIds: string[];
}

export type ProgramElementKind = "building" | "campus" | "floor" | "zone" | "room" | "suite" | "department" | "system" | "circulation" | "support" | "outdoor" | "furnitureGroup" | "other";

export interface ProgramElement extends EntityHeader {
  code: string;
  kind: ProgramElementKind;
  parentId: string | null;
  level: string | null;
  area: QuantitySpec;
  volume: QuantitySpec;
  height: QuantitySpec;
  occupancy: QuantitySpec;
  functionIds: string[];
  activityIds: string[];
  userProfileIds: string[];
  adjacencyIds: string[];
  quantityIds: string[];
  requirementIds: string[];
  locationHint: string | null;
  orientation: string | null;
  daylightRequirement: string | null;
  acousticClass: string | null;
  securityZone: string | null;
  flexibilityNotes: string[];
  growthAllocation: string | null;
  circulationRole: string | null;
  visibilityLevel: string | null;
  adjacencyPreferences: string[];
  environmentalZone: string | null;
}

export interface QuantityRequirement extends EntityHeader {
  targetElementId: string;
  metric: string;
  quantity: QuantitySpec;
  basis: string | null;
  calculationMethod: string | null;
  source: string | null;
  benchmarkRef: string | null;
  tolerancePercent: number | null;
  peakFactor: number | null;
  growthFactor: number | null;
  unitCost: number | null;
  currency: string | null;
  verificationMethod: string | null;
  relatedRequirementIds: string[];
  assumptions: string[];
  constraints: string[];
  schedulePhase: string | null;
  responsibleParty: string | null;
  lastVerified: string | null;
  varianceNotes: TaggedNote[];
}

export type RelationshipKind = "contains" | "serves" | "supports" | "dependsOn" | "conflictsWith" | "equivalentTo" | "adjacentTo" | "feeds" | "receives" | "controls" | "monitors" | "functional" | "operational" | "organizational" | "user" | "service" | "information" | "access" | "security" | "supervision" | "communication" | "dependency" | "sequential" | "sharedResource";

export type TraceKind = "objectiveToRequirement" | "stakeholderToRequirement" | "userToActivity" | "activityToFunction" | "functionToProgramElement" | "requirementToDecision" | "requirementToRisk" | "requirementToStandard" | "requirementToValidation" | "requirementToApproval" | "requirementToChange" | "equipmentToActivity" | "processToResource" | "constraintToImpact" | "scenarioToDecision" | "issueToAction" | "actionToOwner" | "decisionToOutcome" | "versionToChange" | "fullAuditTrail";

export interface TraceLink {
  id: string;
  fromId: string;
  toId: string;
  kind: TraceKind;
  label?: string;
}

export type SeparationKind = "acoustic" | "visual" | "security" | "olfactory" | "thermal" | "fire" | "hygienic" | "circulation" | "operational" | "infectionControl";

export interface Relationship extends EntityHeader {
  sourceId: string;
  targetId: string;
  kind: RelationshipKind;
  strength: number | null;
  directional: boolean;
  rationale: TextField | null;
  constraints: string[];
  conditions: string[];
  relationshipPriority: Priority;
  validFrom: string | null;
  validUntil: string | null;
  evidence: string[];
  conflictIds: string[];
  traceLinks: TraceLink[];
  bidirectional: boolean;
  distanceConstraintM: number | null;
  capacityConstraint: string | null;
  regulatoryBasis: string[];
  reviewCycle: string | null;
  ownerId: string | null;
  proximityRequirement: TextField | null;
  compatibilityRequirement: TextField | null;
  incompatibilityRequirement: TextField | null;
  separationRequirements: SeparationKind[];
}

export type AdjacencyKind = "required" | "preferred" | "optional" | "prohibited";

export type ConnectionKind = "direct" | "indirect" | "controlled" | "sharedAccess" | "none";

export type ValidationStatus = "pending" | "passed" | "failed" | "waived" | "deferred";

export interface Adjacency extends EntityHeader {
  elementAId: string;
  elementBId: string;
  kind: AdjacencyKind;
  connection: ConnectionKind;
  separations: SeparationKind[];
  weight: number;
  rationale: TextField | null;
  distanceMaxM: number | null;
  distanceMinM: number | null;
  levelConstraint: string | null;
  accessPath: string | null;
  sharedWall: boolean;
  sharedEntry: boolean;
  trafficIsolation: boolean;
  circulationOverlap: boolean;
  conflictIds: string[];
  normalized: boolean;
  verificationStatus: ValidationStatus;
  sourceRelationshipId: string | null;
  internalExternalAccess: string | null;
}

export interface Process extends EntityHeader {
  code: string;
  category: string;
  trigger: string | null;
  inputs: string[];
  outputs: string[];
  steps: string[];
  actors: string[];
  equipmentIds: string[];
  elementIds: string[];
  duration: string | null;
  frequency: string | null;
  criticalPath: boolean;
  bottlenecks: string[];
  dependencies: string[];
  kpis: string[];
  automationLevel: string | null;
  failureModes: string[];
  improvementOpportunities: string[];
  regulatoryRefs: string[];
  ownerId: string | null;
  workflowType: string | null;
  handoffPoints: string[];
  qualityGates: string[];
}

export type FlowKind = "people" | "material" | "information" | "service" | "equipment" | "waste" | "emergency" | "vehicle";

export type FlowDirection = "oneWay" | "twoWay" | "bidirectionalPeak" | "restricted";

export type AccessLevel = "public" | "restricted" | "controlled" | "private" | "secure" | "emergencyOnly";

export interface FlowRequirement extends EntityHeader {
  fromElementId: string;
  toElementId: string;
  kind: FlowKind;
  flowType: string;
  direction: FlowDirection;
  volume: QuantitySpec;
  peakRate: number | null;
  clearWidthM: number | null;
  clearHeightM: number | null;
  separationRequirements: SeparationKind[];
  accessLevel: AccessLevel;
  timeWindows: string[];
  equipmentClearance: string | null;
  signageRequired: boolean;
  escortRequired: boolean;
  emergencyRoute: boolean;
  barrierFree: boolean;
  monitoringRequired: boolean;
  processId: string | null;
  conflictIds: string[];
  verificationMethod: string | null;
}

export type AccessMode = "unrestricted" | "cardControlled" | "biometric" | "keyed" | "escortRequired" | "timeRestricted" | "roleBased" | "emergencyOnly";

export interface AccessRule extends EntityHeader {
  subjectIds: string[];
  resourceIds: string[];
  accessLevel: AccessLevel;
  accessMode: AccessMode;
  authentication: string[];
  authorization: string[];
  timeRestrictions: string[];
  escortPolicy: string | null;
  visitorPolicy: string | null;
  emergencyOverride: boolean;
  auditRequired: boolean;
  badgeRequired: boolean;
  biometricRequired: boolean;
  zoneIds: string[];
  exceptions: string[];
  regulatoryBasis: string[];
  enforcementMethod: string | null;
  revocationPolicy: string | null;
  trainingRequired: boolean;
  ownerId: string | null;
}

export interface OperationalRequirement extends EntityHeader {
  operation: string;
  serviceLevel: string | null;
  operatingHours: string | null;
  staffing: QuantitySpec;
  maintenanceInterval: string | null;
  cleaningRegime: string | null;
  turnaroundTime: string | null;
  redundancy: string | null;
  uptimeTarget: number | null;
  responseTime: string | null;
  equipmentIds: string[];
  elementIds: string[];
  processIds: string[];
  utilities: string[];
  wasteStreams: string[];
  contingencyPlan: string[];
  trainingRequirements: string[];
  sopReferences: string[];
  kpiTargets: string[];
  ownerId: string | null;
  serviceCategory: string | null;
  shiftPattern: string | null;
  slaTarget: string | null;
  escalationContactId: string | null;
}

export interface Equipment extends EntityHeader {
  code: string;
  category: string;
  manufacturer: string | null;
  model: string | null;
  quantity: QuantitySpec;
  dimensions: string | null;
  weightKg: number | null;
  powerKw: number | null;
  utilityConnections: string[];
  ventilation: string | null;
  noiseLevelDb: number | null;
  clearance: string | null;
  mounting: string | null;
  elementIds: string[];
  activityIds: string[];
  maintenanceAccess: string[];
  lifecycleYears: number | null;
  replacementCost: number | null;
  standards: string[];
  supplier: string | null;
  activityLinkIds: string[];
  installationRequirements: string[];
  commissioningNotes: string[];
  spareParts: string[];
}

export interface Resource extends EntityHeader {
  code: string;
  category: string;
  resourceType: string;
  quantity: QuantitySpec;
  mobility: string | null;
  sharingModel: string | null;
  allocation: string | null;
  elementIds: string[];
  activityIds: string[];
  userProfileIds: string[];
  storageRequirementId: string | null;
  durability: string | null;
  cleaningRequirements: string[];
  replacementCycle: string | null;
  costPerUnit: number | null;
  supplier: string | null;
  standards: string[];
  ergonomicNotes: string[];
  customization: string[];
  disposalNotes: string[];
  furnitureClass: string | null;
  ergonomicsRating: string | null;
  sharingRatio: number | null;
}

export type StorageClass = "general" | "secure" | "climateControlled" | "hazardous" | "archive" | "mobile" | "fixed" | "shared" | "coldChain" | "flammable";

export interface StorageRequirement extends EntityHeader {
  storedItem: string;
  storageClass: StorageClass;
  quantity: QuantitySpec;
  volumeM3: number | null;
  weightKg: number | null;
  temperatureRange: string | null;
  humidityRange: string | null;
  securityLevel: AccessLevel;
  hazardClass: string | null;
  retentionPeriod: string | null;
  accessFrequency: string | null;
  elementIds: string[];
  equipmentIds: string[];
  handlingEquipment: string[];
  fireProtection: string[];
  ventilation: string | null;
  organizationSystem: string | null;
  growthAllowance: number | null;
  regulatoryRefs: string[];
  ownerId: string | null;
}

export type EnvironmentalParameter = "temperature" | "humidity" | "airQuality" | "lighting" | "acoustics" | "ventilation" | "radiation" | "vibration" | "pressure" | "iaq";

export interface EnvironmentalRequirement extends EntityHeader {
  parameterKind: EnvironmentalParameter;
  parameter: string;
  targetValue: number | null;
  unit: string | null;
  minValue: number | null;
  maxValue: number | null;
  comfortBand: string | null;
  measurementMethod: string | null;
  monitoringFrequency: string | null;
  elementIds: string[];
  occupancyBasis: string | null;
  seasonalVariation: string[];
  energyImplications: string[];
  standards: string[];
  certificationTargets: string[];
  outdoorConditions: string[];
  ventilationStrategy: string | null;
  daylightTarget: string | null;
  acousticTarget: string | null;
  iaqTarget: string | null;
  verificationPlan: string | null;
}

export type HumanFactorAspect = "ergonomics" | "cognition" | "sensory" | "social" | "cultural" | "behavioral" | "physical" | "psychological" | "fatigue" | "stress";

export interface HumanFactorRequirement extends EntityHeader {
  aspect: HumanFactorAspect;
  factor: string;
  userProfileIds: string[];
  activityIds: string[];
  ergonomicCriteria: string[];
  cognitiveLoad: string | null;
  visualDemands: string[];
  auditoryDemands: string[];
  postureRequirements: string[];
  reachEnvelope: string | null;
  lightingForTasks: string[];
  thermalComfort: string[];
  privacyNeeds: string[];
  socialInteraction: string[];
  stressFactors: string[];
  mitigationMeasures: string[];
  trainingNeeds: string[];
  standards: string[];
  researchBasis: string[];
  elementIds: string[];
  verificationMethod: string | null;
}

export interface AccessibilityRequirement extends EntityHeader {
  standard: string;
  level: string | null;
  userProfileIds: string[];
  elementIds: string[];
  routeIds: string[];
  clearWidthM: number | null;
  clearHeightM: number | null;
  turningCircleM: number | null;
  rampSlope: number | null;
  liftRequired: boolean;
  tactileGuidance: boolean;
  hearingLoop: boolean;
  visualContrast: boolean;
  signageRequirements: string[];
  controlsHeight: string | null;
  emergencyEvacuation: string[];
  serviceAnimalPolicy: string | null;
  companionSeating: boolean;
  verificationPlan: string | null;
  exceptions: string[];
  wcagConformance: string | null;
  universalDesignPrinciples: string[];
}

export type PrivacyKind = "public" | "semiPublic" | "semiPrivate" | "private" | "confidential" | "restricted" | "anonymous";

export interface PrivacyRequirement extends EntityHeader {
  privacyKind: PrivacyKind;
  privacyType: string;
  level: string | null;
  subjectIds: string[];
  elementIds: string[];
  visualPrivacy: string[];
  acousticPrivacy: string[];
  dataPrivacy: string[];
  screeningRequired: boolean;
  enclosureRequired: boolean;
  accessRestrictions: string[];
  observationRisk: string | null;
  regulatoryBasis: string[];
  culturalConsiderations: string[];
  technologyControls: string[];
  signage: string[];
  monitoringRestrictions: string[];
  retentionPolicy: string | null;
  breachResponse: string[];
  ownerId: string | null;
}

export type SafetyDomain = "lifeSafety" | "occupationalHealth" | "fire" | "structural" | "electrical" | "chemical" | "radiation" | "ergonomics" | "biological" | "environmental";

export type RiskLevel = "negligible" | "low" | "medium" | "high" | "critical";

export interface SafetyRequirement extends EntityHeader {
  safetyDomain: SafetyDomain;
  hazard: string;
  riskLevel: RiskLevel;
  affectedElementIds: string[];
  affectedUserIds: string[];
  mitigationMeasures: string[];
  ppeRequirements: string[];
  emergencyProcedures: string[];
  evacuationRequirements: string[];
  fireProtection: string[];
  structuralSafety: string[];
  slipTripFall: string[];
  chemicalSafety: string[];
  electricalSafety: string[];
  machinerySafety: string[];
  standards: string[];
  inspectionFrequency: string | null;
  trainingRequirements: string[];
  incidentReporting: string[];
  residualRisk: string | null;
}

export type SecurityControlKind = "accessControl" | "surveillance" | "perimeter" | "cyber" | "personnel" | "information" | "physical" | "procedural" | "screening" | "keyManagement";

export interface SecurityRequirement extends EntityHeader {
  controlKind: SecurityControlKind;
  threat: string;
  riskLevel: RiskLevel;
  assetIds: string[];
  zoneIds: string[];
  accessLevel: AccessLevel;
  perimeterControls: string[];
  surveillance: string[];
  intrusionDetection: string[];
  cybersecurity: string[];
  screening: string[];
  visitorManagement: string[];
  keyManagement: string[];
  standards: string[];
  responseProcedures: string[];
  drillFrequency: string | null;
  liaisonContacts: string[];
  classifiedLevel: string | null;
  redundancy: string[];
  auditRequirements: string[];
}

export interface RegulatoryRequirement extends EntityHeader {
  jurisdiction: string;
  code: string;
  clause: string | null;
  title: string;
  requirementText: TextField;
  applicability: string[];
  elementIds: string[];
  complianceMethod: string | null;
  evidenceRequired: string[];
  authority: string | null;
  effectiveDate: string | null;
  expiryDate: string | null;
  penalties: string[];
  exemptions: string[];
  relatedRequirementIds: string[];
  interpretationNotes: TaggedNote[];
  verificationStatus: ValidationStatus;
  consultantRefs: string[];
  updateSource: string | null;
}

export interface SiteContext extends EntityHeader {
  siteName: string;
  address: string | null;
  latitude: number | null;
  longitude: number | null;
  elevationM: number | null;
  climateZone: string | null;
  seismicZone: string | null;
  floodRisk: string | null;
  soilConditions: string[];
  utilitiesAvailable: string[];
  accessRoads: string[];
  publicTransit: string[];
  neighbors: string[];
  views: string[];
  noiseSources: string[];
  environmentalConstraints: string[];
  heritageConstraints: string[];
  zoning: string | null;
  maxHeightM: number | null;
  maxCoverage: number | null;
}

export interface OrganizationalRequirement extends EntityHeader {
  department: string;
  reportingLine: string | null;
  headcount: QuantitySpec;
  growthPlanId: string | null;
  workPatterns: string[];
  collaborationModel: string | null;
  hierarchyLevels: string[];
  decisionMaking: string[];
  cultureNotes: string[];
  changeReadiness: string | null;
  unionConsiderations: string[];
  trainingNeeds: string[];
  elementIds: string[];
  stakeholderIds: string[];
  serviceRequirementIds: string[];
  brandingRequirements: string[];
  wellnessPlugins: string[];
  diversityGoals: string[];
  ownerId: string | null;
}

export interface ServiceRequirement extends EntityHeader {
  serviceName: string;
  serviceType: string;
  provider: string | null;
  serviceLevel: string | null;
  operatingHours: string | null;
  capacity: QuantitySpec;
  responseTime: string | null;
  queueManagement: string[];
  customerProfiles: string[];
  elementIds: string[];
  equipmentIds: string[];
  staffing: QuantitySpec;
  qualityMetrics: string[];
  costModel: string | null;
  contractRefs: string[];
  dependencies: string[];
  failureImpact: string | null;
  backupService: string[];
  feedbackChannels: string[];
}

export interface InfrastructureRequirement extends EntityHeader {
  system: string;
  category: string;
  capacity: QuantitySpec;
  redundancy: string | null;
  distribution: string[];
  entryPoints: string[];
  utilitySource: string | null;
  standbyPower: boolean;
  monitoring: string[];
  maintenanceAccess: string[];
  standards: string[];
  elementIds: string[];
  peakDemand: number | null;
  diversityFactor: number | null;
  futureExpansion: string[];
  interfaceRequirements: string[];
  commissioning: string[];
  lifecycleCost: number | null;
  ownerId: string | null;
}

export interface InformationRequirement extends EntityHeader {
  informationType: string;
  format: string | null;
  sourceSystem: string | null;
  destinationSystems: string[];
  updateFrequency: string | null;
  retentionPeriod: string | null;
  accessControls: string[];
  classification: string | null;
  qualityCriteria: string[];
  metadataRequirements: string[];
  integrationPoints: string[];
  backupRequirements: string[];
  disasterRecovery: string[];
  privacyControls: string[];
  auditTrail: boolean;
  elementIds: string[];
  stakeholderIds: string[];
  standards: string[];
  ownerId: string | null;
}

export interface CommunicationRequirement extends EntityHeader {
  channel: string;
  audienceIds: string[];
  messageTypes: string[];
  frequency: string | null;
  medium: string[];
  language: string[];
  accessibility: string[];
  emergencyUse: boolean;
  twoWay: boolean;
  recordingPolicy: string | null;
  signageLocations: string[];
  technology: string[];
  escalationPath: string[];
  feedbackLoop: boolean;
  privacyControls: string[];
  elementIds: string[];
  standards: string[];
  ownerId: string | null;
  templates: string[];
}

export interface WayfindingRequirement extends EntityHeader {
  userProfileIds: string[];
  elementIds: string[];
  destinationTypes: string[];
  signageTypes: string[];
  languages: string[];
  tactileRequired: boolean;
  audioRequired: boolean;
  digitalWayfinding: boolean;
  landmarkStrategy: string[];
  colorCoding: string[];
  symbolStandards: string[];
  decisionPoints: string[];
  maximumSignageDistanceM: number | null;
  lightingRequirements: string[];
  maintenancePlan: string | null;
  emergencyEgress: string[];
  visitorJourney: string[];
  staffJourney: string[];
  brandIntegration: string[];
}

export type DeliveryPhase = "concept" | "schematic" | "designDevelopment" | "constructionArtifacts" | "procurement" | "construction" | "commissioning" | "occupancy";

export interface ScheduleRequirement extends EntityHeader {
  milestone: string;
  phase: DeliveryPhase;
  startDate: string | null;
  endDate: string | null;
  duration: string | null;
  dependencies: string[];
  predecessors: string[];
  successors: string[];
  critical: boolean;
  floatDays: number | null;
  resourceRequirements: string[];
  occupancyImpact: string[];
  phasingStrategy: string | null;
  decantRequirements: string[];
  commissioningWindow: string | null;
  stakeholderIds: string[];
  riskIds: string[];
  contingencyDays: number | null;
  reportingCadence: string | null;
  ownerId: string | null;
}

export interface FlexibilityRequirement extends EntityHeader {
  flexibilityType: string;
  elementIds: string[];
  adaptationScenarios: string[];
  modularityLevel: string | null;
  reconfigurationTime: string | null;
  costOfChange: number | null;
  technologyReadiness: string | null;
  futureFunctionIds: string[];
  demountablePartitions: boolean;
  raisedFloor: boolean;
  overheadServices: boolean;
  expansionDirection: string[];
  contractionScenario: string[];
  multiUsePotential: string[];
  furnitureStrategy: string[];
  infrastructureSpareCapacity: string[];
  leaseImplications: string[];
  ownerId: string | null;
}

export interface GrowthPlan extends EntityHeader {
  horizonYears: number;
  growthRate: number | null;
  headcountGrowth: QuantitySpec;
  areaGrowth: QuantitySpec;
  phases: string[];
  triggerEvents: string[];
  expansionElementIds: string[];
  reserveAreas: string[];
  infrastructureHeadroom: string[];
  budgetEnvelope: number | null;
  fundingSources: string[];
  riskFactors: string[];
  decisionPoints: string[];
  scenarioIds: string[];
  decommissionPlan: string[];
  relocationStrategy: string[];
  stakeholderImpact: string[];
  regulatoryConsiderations: string[];
  ownerId: string | null;
}

export interface SustainabilityRequirement extends EntityHeader {
  topic: string;
  target: string | null;
  metric: string | null;
  baseline: number | null;
  targetValue: number | null;
  unit: string | null;
  certification: string[];
  standards: string[];
  elementIds: string[];
  strategies: string[];
  materialsPreferences: string[];
  energyStrategy: string[];
  waterStrategy: string[];
  wasteStrategy: string[];
  biodiversity: string[];
  embodiedCarbon: number | null;
  operationalCarbon: number | null;
  reportingRequirements: string[];
  verificationPlan: string | null;
  ownerId: string | null;
}

export interface ResilienceRequirement extends EntityHeader {
  hazard: string;
  riskLevel: RiskLevel;
  scenario: string | null;
  recoveryTime: string | null;
  recoveryPoint: string | null;
  redundancy: string[];
  hardeningMeasures: string[];
  backupSystems: string[];
  alternateSites: string[];
  supplyChain: string[];
  communicationPlan: string[];
  drillRequirements: string[];
  elementIds: string[];
  infrastructureIds: string[];
  standards: string[];
  insuranceImplications: string[];
  climateAdaptation: string[];
  ownerId: string | null;
  verificationPlan: string | null;
}

export type CostBasis = "capital" | "operational" | "lifecycle" | "replacement" | "maintenance";

export interface CostRequirement extends EntityHeader {
  costItem: string;
  basis: CostBasis;
  amount: number | null;
  currency: string;
  quantityBasis: string | null;
  unitCost: number | null;
  contingencyPercent: number | null;
  escalationRate: number | null;
  fundingSource: string | null;
  elementIds: string[];
  requirementIds: string[];
  phase: DeliveryPhase | null;
  cashFlowProfile: string[];
  valueEngineeringNotes: string[];
  benchmarkRef: string | null;
  approvalStatus: ValidationStatus;
  ownerId: string | null;
  assumptions: string[];
  sensitivityFactors: string[];
}

export interface DeliveryConstraint extends EntityHeader {
  constraintType: string;
  constraintDetails: TextField;
  phase: DeliveryPhase;
  hardDeadline: string | null;
  softDeadline: string | null;
  impactedElementIds: string[];
  impactedRequirementIds: string[];
  workHours: string | null;
  noiseRestrictions: string[];
  accessRestrictions: string[];
  siteLogistics: string[];
  procurementLeadTime: string | null;
  approvalGates: string[];
  occupancyConstraints: string[];
  weatherWindows: string[];
  penaltyClauses: string[];
  mitigationOptions: string[];
  ownerId: string | null;
  riskIds: string[];
  constraintStatus: LifecycleStatus;
}

export interface Risk extends EntityHeader {
  riskStatement: TextField;
  category: string;
  probability: RiskLevel;
  impact: RiskLevel;
  riskScore: number | null;
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

export type IssueSeverity = "cosmetic" | "minor" | "major" | "critical" | "blocker";

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
  costImpact: number | null;
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
  costEstimate: number | null;
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
  weight: number | null;
  rationale: TextField | null;
  decisionId: string | null;
  stakeholderIds: string[];
  effectiveFrom: string | null;
  effectiveUntil: string | null;
  reviewCycle: string | null;
  dependencies: string[];
  conflicts: string[];
  scoringMethod: string | null;
  score: number | null;
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
  probability: number | null;
  impactSummary: TextField | null;
  costDelta: number | null;
  areaDelta: number | null;
  headcountDelta: number | null;
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
  scores: number[];
  weightedScore: number | null;
  costEstimate: number | null;
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
  costImpact: number | null;
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
  target: number | null;
  unit: string | null;
  minimum: number | null;
  maximum: number | null;
  measurementMethod: string | null;
  frequency: string | null;
  requirementIds: string[];
  elementIds: string[];
  baseline: number | null;
  benchmarkRef: string | null;
  weight: number | null;
  dataSource: string | null;
  reportingCadence: string | null;
  ownerId: string | null;
  verificationPlan: string | null;
  penaltyThreshold: number | null;
  incentiveThreshold: number | null;
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
  costImpact: number | null;
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
  durationMs: number | null;
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
  useCount: number;
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
  progressPercent: number | null;
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
  budget: number | null;
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
  responseRate: number | null;
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
  usageCount: number;
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
  usageCount: number;
}

export interface BenchmarkRecord extends EntityHeader {
  benchmarkName: string;
  sector: string;
  metric: string;
  value: number;
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

export const PROGRAM_ARTIFACT_FIELDS = [
  "schema", "meta", "project", "stakeholders", "users", "activities", "functions", "elements", "quantities", "relationships", "adjacencies", "processes", "flows", "accessRules", "operations", "equipment", "resources", "storage", "environmental", "humanFactors", "accessibility", "privacy", "safety", "security", "regulatory", "siteContext", "organizational", "services", "infrastructure", "information", "communication", "wayfinding", "schedules", "flexibility", "growth", "sustainability", "resilience", "costs", "delivery", "risks", "conflicts", "requirements", "priorities", "scenarios", "options", "decisions", "validations", "performance", "quality", "artifacts", "assumptions", "constraints", "complianceRecords", "approvals", "meetings", "changes", "collaboration", "analyses", "reports", "searchFilters", "statusRecords", "workshops", "surveys", "issues", "auditEvents", "templates", "knowledge", "benchmarks", "traces", "governance",
] as const;

export interface ProgramArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  meta: ProgramMeta;
  /** @state artifact */
  project: ProjectDefinition;
  /** @state artifact */
  stakeholders: Stakeholder[];
  /** @state artifact */
  users: UserProfile[];
  /** @state artifact */
  activities: Activity[];
  /** @state artifact */
  functions: Function[];
  /** @state artifact */
  elements: ProgramElement[];
  /** @state artifact */
  quantities: QuantityRequirement[];
  /** @state artifact */
  relationships: Relationship[];
  /** @state artifact */
  adjacencies: Adjacency[];
  /** @state artifact */
  processes: Process[];
  /** @state artifact */
  flows: FlowRequirement[];
  /** @state artifact */
  accessRules: AccessRule[];
  /** @state artifact */
  operations: OperationalRequirement[];
  /** @state artifact */
  equipment: Equipment[];
  /** @state artifact */
  resources: Resource[];
  /** @state artifact */
  storage: StorageRequirement[];
  /** @state artifact */
  environmental: EnvironmentalRequirement[];
  /** @state artifact */
  humanFactors: HumanFactorRequirement[];
  /** @state artifact */
  accessibility: AccessibilityRequirement[];
  /** @state artifact */
  privacy: PrivacyRequirement[];
  /** @state artifact */
  safety: SafetyRequirement[];
  /** @state artifact */
  security: SecurityRequirement[];
  /** @state artifact */
  regulatory: RegulatoryRequirement[];
  /** @state artifact */
  siteContext: SiteContext[];
  /** @state artifact */
  organizational: OrganizationalRequirement[];
  /** @state artifact */
  services: ServiceRequirement[];
  /** @state artifact */
  infrastructure: InfrastructureRequirement[];
  /** @state artifact */
  information: InformationRequirement[];
  /** @state artifact */
  communication: CommunicationRequirement[];
  /** @state artifact */
  wayfinding: WayfindingRequirement[];
  /** @state artifact */
  schedules: ScheduleRequirement[];
  /** @state artifact */
  flexibility: FlexibilityRequirement[];
  /** @state artifact */
  growth: GrowthPlan[];
  /** @state artifact */
  sustainability: SustainabilityRequirement[];
  /** @state artifact */
  resilience: ResilienceRequirement[];
  /** @state artifact */
  costs: CostRequirement[];
  /** @state artifact */
  delivery: DeliveryConstraint[];
  /** @state artifact */
  risks: Risk[];
  /** @state artifact */
  conflicts: Conflict[];
  /** @state artifact */
  requirements: Requirement[];
  /** @state artifact */
  priorities: PriorityRecord[];
  /** @state artifact */
  scenarios: Scenario[];
  /** @state artifact */
  options: OptionEvaluation[];
  /** @state artifact */
  decisions: Decision[];
  /** @state artifact */
  validations: ValidationRecord[];
  /** @state artifact */
  performance: PerformanceCriterion[];
  /** @state artifact */
  quality: QualityRecord[];
  /** @state artifact */
  artifacts: ArtifactRecord[];
  /** @state artifact */
  assumptions: Assumption[];
  /** @state artifact */
  constraints: ConstraintRecord[];
  /** @state artifact */
  complianceRecords: ComplianceRecord[];
  /** @state artifact */
  approvals: ApprovalRecord[];
  /** @state artifact */
  meetings: MeetingRecord[];
  /** @state artifact */
  changes: ChangeRecord[];
  /** @state artifact */
  collaboration: CollaborationRecord[];
  /** @state artifact */
  analyses: AnalysisRecord[];
  /** @state artifact */
  reports: ReportRecord[];
  /** @state artifact */
  searchFilters: SearchFilter[];
  /** @state artifact */
  statusRecords: StatusRecord[];
  /** @state artifact */
  workshops: Workshop[];
  /** @state artifact */
  surveys: Survey[];
  /** @state artifact */
  issues: Issue[];
  /** @state artifact */
  auditEvents: AuditEvent[];
  /** @state artifact */
  templates: TemplateRecord[];
  /** @state artifact */
  knowledge: ArtifactChild;
  /** @state artifact */
  benchmarks: ArtifactChild;
  /** @state artifact */
  traces: TraceLink[];
  /** @state artifact */
  governance: Governance;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramArtifactGuardReject = (at: string, why: string): never => {
  throw new architectProgramArtifactGuardRefusal(at, why);
};

type architectProgramArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectProgramArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectProgramArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectProgramArtifactGuardReject(at, "value is not an object");
export const architectProgramArtifactGuardExactObject = (value: unknown, at: string, fields: readonly string[], required: readonly string[] = fields): Readonly<Record<string, unknown>> => {
  const row = parseSchemaRecord(value, fields, at);
  for (const field of required) if (!Object.hasOwn(row, field)) architectProgramArtifactGuardReject(`${at}.${field}`, "field is missing");
  return row;
};
export const architectProgramArtifactGuardArray = (value: unknown, at: string, bounds: architectProgramArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectProgramArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectProgramArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectProgramArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectProgramArtifactGuardString = (value: unknown, at: string, bounds: architectProgramArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectProgramArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectProgramArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectProgramArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectProgramArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectProgramArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectProgramArtifactGuardReject(at, "value is not a boolean"));
export const architectProgramArtifactGuardNumber = (value: unknown, at: string, bounds: architectProgramArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectProgramArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectProgramArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectProgramArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectProgramArtifactGuardInteger = (value: unknown, at: string, bounds: architectProgramArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramArtifactGuardNumber(value, at, bounds) : architectProgramArtifactGuardReject(at, "value is not an integer");
export const architectProgramArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProgramArtifact(value: unknown, at = "$"): ProgramArtifact {
  const row = architectProgramArtifactGuardExactObject(value, at, PROGRAM_ARTIFACT_FIELDS);
  return {
    schema: architectProgramArtifactGuardString(row["schema"], `${at}.schema`),
    meta: parseProgramMeta(row["meta"], `${at}.meta`),
    project: parseProjectDefinition(row["project"], `${at}.project`),
    stakeholders: architectProgramArtifactGuardArray(row["stakeholders"], `${at}.stakeholders`).map((item, index) => parseStakeholder(item, `${at}.stakeholders[${index}]`)),
    users: architectProgramArtifactGuardArray(row["users"], `${at}.users`).map((item, index) => parseUserProfile(item, `${at}.users[${index}]`)),
    activities: architectProgramArtifactGuardArray(row["activities"], `${at}.activities`).map((item, index) => parseActivity(item, `${at}.activities[${index}]`)),
    functions: architectProgramArtifactGuardArray(row["functions"], `${at}.functions`).map((item, index) => parseFunction(item, `${at}.functions[${index}]`)),
    elements: architectProgramArtifactGuardArray(row["elements"], `${at}.elements`).map((item, index) => parseProgramElement(item, `${at}.elements[${index}]`)),
    quantities: architectProgramArtifactGuardArray(row["quantities"], `${at}.quantities`).map((item, index) => parseQuantityRequirement(item, `${at}.quantities[${index}]`)),
    relationships: architectProgramArtifactGuardArray(row["relationships"], `${at}.relationships`).map((item, index) => parseRelationship(item, `${at}.relationships[${index}]`)),
    adjacencies: architectProgramArtifactGuardArray(row["adjacencies"], `${at}.adjacencies`).map((item, index) => parseAdjacency(item, `${at}.adjacencies[${index}]`)),
    processes: architectProgramArtifactGuardArray(row["processes"], `${at}.processes`).map((item, index) => parseProcess(item, `${at}.processes[${index}]`)),
    flows: architectProgramArtifactGuardArray(row["flows"], `${at}.flows`).map((item, index) => parseFlowRequirement(item, `${at}.flows[${index}]`)),
    accessRules: architectProgramArtifactGuardArray(row["accessRules"], `${at}.accessRules`).map((item, index) => parseAccessRule(item, `${at}.accessRules[${index}]`)),
    operations: architectProgramArtifactGuardArray(row["operations"], `${at}.operations`).map((item, index) => parseOperationalRequirement(item, `${at}.operations[${index}]`)),
    equipment: architectProgramArtifactGuardArray(row["equipment"], `${at}.equipment`).map((item, index) => parseEquipment(item, `${at}.equipment[${index}]`)),
    resources: architectProgramArtifactGuardArray(row["resources"], `${at}.resources`).map((item, index) => parseResource(item, `${at}.resources[${index}]`)),
    storage: architectProgramArtifactGuardArray(row["storage"], `${at}.storage`).map((item, index) => parseStorageRequirement(item, `${at}.storage[${index}]`)),
    environmental: architectProgramArtifactGuardArray(row["environmental"], `${at}.environmental`).map((item, index) => parseEnvironmentalRequirement(item, `${at}.environmental[${index}]`)),
    humanFactors: architectProgramArtifactGuardArray(row["humanFactors"], `${at}.humanFactors`).map((item, index) => parseHumanFactorRequirement(item, `${at}.humanFactors[${index}]`)),
    accessibility: architectProgramArtifactGuardArray(row["accessibility"], `${at}.accessibility`).map((item, index) => parseAccessibilityRequirement(item, `${at}.accessibility[${index}]`)),
    privacy: architectProgramArtifactGuardArray(row["privacy"], `${at}.privacy`).map((item, index) => parsePrivacyRequirement(item, `${at}.privacy[${index}]`)),
    safety: architectProgramArtifactGuardArray(row["safety"], `${at}.safety`).map((item, index) => parseSafetyRequirement(item, `${at}.safety[${index}]`)),
    security: architectProgramArtifactGuardArray(row["security"], `${at}.security`).map((item, index) => parseSecurityRequirement(item, `${at}.security[${index}]`)),
    regulatory: architectProgramArtifactGuardArray(row["regulatory"], `${at}.regulatory`).map((item, index) => parseRegulatoryRequirement(item, `${at}.regulatory[${index}]`)),
    siteContext: architectProgramArtifactGuardArray(row["siteContext"], `${at}.siteContext`).map((item, index) => parseSiteContext(item, `${at}.siteContext[${index}]`)),
    organizational: architectProgramArtifactGuardArray(row["organizational"], `${at}.organizational`).map((item, index) => parseOrganizationalRequirement(item, `${at}.organizational[${index}]`)),
    services: architectProgramArtifactGuardArray(row["services"], `${at}.services`).map((item, index) => parseServiceRequirement(item, `${at}.services[${index}]`)),
    infrastructure: architectProgramArtifactGuardArray(row["infrastructure"], `${at}.infrastructure`).map((item, index) => parseInfrastructureRequirement(item, `${at}.infrastructure[${index}]`)),
    information: architectProgramArtifactGuardArray(row["information"], `${at}.information`).map((item, index) => parseInformationRequirement(item, `${at}.information[${index}]`)),
    communication: architectProgramArtifactGuardArray(row["communication"], `${at}.communication`).map((item, index) => parseCommunicationRequirement(item, `${at}.communication[${index}]`)),
    wayfinding: architectProgramArtifactGuardArray(row["wayfinding"], `${at}.wayfinding`).map((item, index) => parseWayfindingRequirement(item, `${at}.wayfinding[${index}]`)),
    schedules: architectProgramArtifactGuardArray(row["schedules"], `${at}.schedules`).map((item, index) => parseScheduleRequirement(item, `${at}.schedules[${index}]`)),
    flexibility: architectProgramArtifactGuardArray(row["flexibility"], `${at}.flexibility`).map((item, index) => parseFlexibilityRequirement(item, `${at}.flexibility[${index}]`)),
    growth: architectProgramArtifactGuardArray(row["growth"], `${at}.growth`).map((item, index) => parseGrowthPlan(item, `${at}.growth[${index}]`)),
    sustainability: architectProgramArtifactGuardArray(row["sustainability"], `${at}.sustainability`).map((item, index) => parseSustainabilityRequirement(item, `${at}.sustainability[${index}]`)),
    resilience: architectProgramArtifactGuardArray(row["resilience"], `${at}.resilience`).map((item, index) => parseResilienceRequirement(item, `${at}.resilience[${index}]`)),
    costs: architectProgramArtifactGuardArray(row["costs"], `${at}.costs`).map((item, index) => parseCostRequirement(item, `${at}.costs[${index}]`)),
    delivery: architectProgramArtifactGuardArray(row["delivery"], `${at}.delivery`).map((item, index) => parseDeliveryConstraint(item, `${at}.delivery[${index}]`)),
    risks: architectProgramArtifactGuardArray(row["risks"], `${at}.risks`).map((item, index) => parseRisk(item, `${at}.risks[${index}]`)),
    conflicts: architectProgramArtifactGuardArray(row["conflicts"], `${at}.conflicts`).map((item, index) => parseConflict(item, `${at}.conflicts[${index}]`)),
    requirements: architectProgramArtifactGuardArray(row["requirements"], `${at}.requirements`).map((item, index) => parseRequirement(item, `${at}.requirements[${index}]`)),
    priorities: architectProgramArtifactGuardArray(row["priorities"], `${at}.priorities`).map((item, index) => parsePriorityRecord(item, `${at}.priorities[${index}]`)),
    scenarios: architectProgramArtifactGuardArray(row["scenarios"], `${at}.scenarios`).map((item, index) => parseScenario(item, `${at}.scenarios[${index}]`)),
    options: architectProgramArtifactGuardArray(row["options"], `${at}.options`).map((item, index) => parseOptionEvaluation(item, `${at}.options[${index}]`)),
    decisions: architectProgramArtifactGuardArray(row["decisions"], `${at}.decisions`).map((item, index) => parseDecision(item, `${at}.decisions[${index}]`)),
    validations: architectProgramArtifactGuardArray(row["validations"], `${at}.validations`).map((item, index) => parseValidationRecord(item, `${at}.validations[${index}]`)),
    performance: architectProgramArtifactGuardArray(row["performance"], `${at}.performance`).map((item, index) => parsePerformanceCriterion(item, `${at}.performance[${index}]`)),
    quality: architectProgramArtifactGuardArray(row["quality"], `${at}.quality`).map((item, index) => parseQualityRecord(item, `${at}.quality[${index}]`)),
    artifacts: architectProgramArtifactGuardArray(row["artifacts"], `${at}.artifacts`).map((item, index) => parseArtifactRecord(item, `${at}.artifacts[${index}]`)),
    assumptions: architectProgramArtifactGuardArray(row["assumptions"], `${at}.assumptions`).map((item, index) => parseAssumption(item, `${at}.assumptions[${index}]`)),
    constraints: architectProgramArtifactGuardArray(row["constraints"], `${at}.constraints`).map((item, index) => parseConstraintRecord(item, `${at}.constraints[${index}]`)),
    complianceRecords: architectProgramArtifactGuardArray(row["complianceRecords"], `${at}.complianceRecords`).map((item, index) => parseComplianceRecord(item, `${at}.complianceRecords[${index}]`)),
    approvals: architectProgramArtifactGuardArray(row["approvals"], `${at}.approvals`).map((item, index) => parseApprovalRecord(item, `${at}.approvals[${index}]`)),
    meetings: architectProgramArtifactGuardArray(row["meetings"], `${at}.meetings`).map((item, index) => parseMeetingRecord(item, `${at}.meetings[${index}]`)),
    changes: architectProgramArtifactGuardArray(row["changes"], `${at}.changes`).map((item, index) => parseChangeRecord(item, `${at}.changes[${index}]`)),
    collaboration: architectProgramArtifactGuardArray(row["collaboration"], `${at}.collaboration`).map((item, index) => parseCollaborationRecord(item, `${at}.collaboration[${index}]`)),
    analyses: architectProgramArtifactGuardArray(row["analyses"], `${at}.analyses`).map((item, index) => parseAnalysisRecord(item, `${at}.analyses[${index}]`)),
    reports: architectProgramArtifactGuardArray(row["reports"], `${at}.reports`).map((item, index) => parseReportRecord(item, `${at}.reports[${index}]`)),
    searchFilters: architectProgramArtifactGuardArray(row["searchFilters"], `${at}.searchFilters`).map((item, index) => parseSearchFilter(item, `${at}.searchFilters[${index}]`)),
    statusRecords: architectProgramArtifactGuardArray(row["statusRecords"], `${at}.statusRecords`).map((item, index) => parseStatusRecord(item, `${at}.statusRecords[${index}]`)),
    workshops: architectProgramArtifactGuardArray(row["workshops"], `${at}.workshops`).map((item, index) => parseWorkshop(item, `${at}.workshops[${index}]`)),
    surveys: architectProgramArtifactGuardArray(row["surveys"], `${at}.surveys`).map((item, index) => parseSurvey(item, `${at}.surveys[${index}]`)),
    issues: architectProgramArtifactGuardArray(row["issues"], `${at}.issues`).map((item, index) => parseIssue(item, `${at}.issues[${index}]`)),
    auditEvents: architectProgramArtifactGuardArray(row["auditEvents"], `${at}.auditEvents`).map((item, index) => parseAuditEvent(item, `${at}.auditEvents[${index}]`)),
    templates: architectProgramArtifactGuardArray(row["templates"], `${at}.templates`).map((item, index) => parseTemplateRecord(item, `${at}.templates[${index}]`)),
    knowledge: parseArtifactChild(row["knowledge"]),
    benchmarks: parseArtifactChild(row["benchmarks"]),
    traces: architectProgramArtifactGuardArray(row["traces"], `${at}.traces`).map((item, index) => parseTraceLink(item, `${at}.traces[${index}]`)),
    governance: parseGovernance(row["governance"], `${at}.governance`),
  };
}

export function parseAccessRule(value: unknown, at = "$"): AccessRule {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","subjectIds","resourceIds","accessLevel","accessMode","authentication","authorization","timeRestrictions","escortPolicy","visitorPolicy","emergencyOverride","auditRequired","badgeRequired","biometricRequired","zoneIds","exceptions","regulatoryBasis","enforcementMethod","revocationPolicy","trainingRequired","ownerId"], ["id","name","status","priority","ownership","timestamps","subjectIds","resourceIds","accessLevel","accessMode","authentication","authorization","timeRestrictions","escortPolicy","visitorPolicy","emergencyOverride","auditRequired","badgeRequired","biometricRequired","zoneIds","exceptions","regulatoryBasis","enforcementMethod","revocationPolicy","trainingRequired","ownerId"]) as unknown as AccessRule;
}

export function parseAccessibilityRequirement(value: unknown, at = "$"): AccessibilityRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","standard","level","userProfileIds","elementIds","routeIds","clearWidthM","clearHeightM","turningCircleM","rampSlope","liftRequired","tactileGuidance","hearingLoop","visualContrast","signageRequirements","controlsHeight","emergencyEvacuation","serviceAnimalPolicy","companionSeating","verificationPlan","exceptions","wcagConformance","universalDesignPrinciples"], ["id","name","status","priority","ownership","timestamps","standard","level","userProfileIds","elementIds","routeIds","clearWidthM","clearHeightM","turningCircleM","rampSlope","liftRequired","tactileGuidance","hearingLoop","visualContrast","signageRequirements","controlsHeight","emergencyEvacuation","serviceAnimalPolicy","companionSeating","verificationPlan","exceptions","wcagConformance","universalDesignPrinciples"]) as unknown as AccessibilityRequirement;
}

export function parseActivity(value: unknown, at = "$"): Activity {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","category","frequency","duration","intensity","participants","equipmentIds","spaceRequirements","environmentalNeeds","privacyNeeds","accessibilityNeeds","adjacentActivities","sequencing","peakPeriods","workflowSteps","inputs","outputs","userProfileIds","functionIds","performanceIndicators","activityType","locationContext","temporalPattern","supervisionLevel"], ["id","name","status","priority","ownership","timestamps","code","category","frequency","duration","intensity","participants","equipmentIds","spaceRequirements","environmentalNeeds","privacyNeeds","accessibilityNeeds","adjacentActivities","sequencing","peakPeriods","workflowSteps","inputs","outputs","userProfileIds","functionIds","performanceIndicators","activityType","locationContext","temporalPattern","supervisionLevel"]) as unknown as Activity;
}

export function parseAdjacency(value: unknown, at = "$"): Adjacency {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","elementAId","elementBId","kind","connection","separations","weight","rationale","distanceMaxM","distanceMinM","levelConstraint","accessPath","sharedWall","sharedEntry","trafficIsolation","circulationOverlap","conflictIds","normalized","verificationStatus","sourceRelationshipId","internalExternalAccess"], ["id","name","status","priority","ownership","timestamps","elementAId","elementBId","kind","connection","separations","weight","rationale","distanceMaxM","distanceMinM","levelConstraint","accessPath","sharedWall","sharedEntry","trafficIsolation","circulationOverlap","conflictIds","normalized","verificationStatus","sourceRelationshipId","internalExternalAccess"]) as unknown as Adjacency;
}

export function parseAdjacencyKind(value: unknown, at = "$"): AdjacencyKind {
  return architectProgramArtifactGuardMember(value, at, ["required","preferred","optional","prohibited"]) as AdjacencyKind;
}

export function parseAnalysisRecord(value: unknown, at = "$"): AnalysisRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","kind","title","parameters","inputEntityIds","outputSummary","findings","metrics","charts","runBy","runAt","durationMs","toolVersion","scenarioId","reportId","confidence","limitations","recommendations","rawResultRef"], ["id","name","status","priority","ownership","timestamps","kind","title","parameters","inputEntityIds","outputSummary","findings","metrics","charts","runBy","runAt","durationMs","toolVersion","scenarioId","reportId","confidence","limitations","recommendations","rawResultRef"]) as unknown as AnalysisRecord;
}

export function parseApprovalRecord(value: unknown, at = "$"): ApprovalRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","approvalType","subjectId","approverIds","approvalDate","conditions","approvalStatus","expiryDate","delegationChain","evidenceRefs","relatedDecisionId","relatedChangeId","authorityBasis","signatureMethod","rejectionReason","resubmissionDate","notificationList","workflowStep","version","auditTrailRef"], ["id","name","status","priority","ownership","timestamps","approvalType","subjectId","approverIds","approvalDate","conditions","approvalStatus","expiryDate","delegationChain","evidenceRefs","relatedDecisionId","relatedChangeId","authorityBasis","signatureMethod","rejectionReason","resubmissionDate","notificationList","workflowStep","version","auditTrailRef"]) as unknown as ApprovalRecord;
}

export function parseAssumption(value: unknown, at = "$"): Assumption {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","statement","basis","confidenceLevel","impactIfFalse","relatedEntityIds","validationStatus","validatedBy","validationDate","ownerId","reviewCycle","source","category","dependencies","mitigation","linkedRequirementIds","linkedRiskIds","expirationDate","statusNotes","artifactRefs"], ["id","name","status","priority","ownership","timestamps","statement","basis","confidenceLevel","impactIfFalse","relatedEntityIds","validationStatus","validatedBy","validationDate","ownerId","reviewCycle","source","category","dependencies","mitigation","linkedRequirementIds","linkedRiskIds","expirationDate","statusNotes","artifactRefs"]) as unknown as Assumption;
}

export function parseAuditEvent(value: unknown, at = "$"): AuditEvent {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","action","actorId","subjectId","subjectKind","timestamp","details","beforeState","afterState","ipAddress","client","sessionId","changeRecordId","traceLink","success","errorMessage","correlationId","complianceTags","retentionUntil"], ["id","name","status","priority","ownership","timestamps","action","actorId","subjectId","subjectKind","timestamp","details","beforeState","afterState","ipAddress","client","sessionId","changeRecordId","traceLink","success","errorMessage","correlationId","complianceTags","retentionUntil"]) as unknown as AuditEvent;
}

export function parseBenchmarkRecord(value: unknown, at = "$"): BenchmarkRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","benchmarkName","sector","metric","value","unit","sampleSize","source","collectionYear","geography","buildingType","confidence","methodology","applicableElementKinds","relatedRequirementIds","comparisonNotes","limitations","license","knowledgeId","lastVerified"], ["id","name","status","priority","ownership","timestamps","benchmarkName","sector","metric","value","unit","sampleSize","source","collectionYear","geography","buildingType","confidence","methodology","applicableElementKinds","relatedRequirementIds","comparisonNotes","limitations","license","knowledgeId","lastVerified"]) as unknown as BenchmarkRecord;
}

export function parseChangeRecord(value: unknown, at = "$"): ChangeRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","changeType","summary","reason","requestedBy","approvedBy","changeDate","effectiveDate","impactedEntityIds","beforeSnapshot","afterSnapshot","costImpact","scheduleImpact","riskImpact","approvalStatus","rollbackPlan","communicationPlan","versionFrom","versionTo","auditEventIds"], ["id","name","status","priority","ownership","timestamps","changeType","summary","reason","requestedBy","approvedBy","changeDate","effectiveDate","impactedEntityIds","beforeSnapshot","afterSnapshot","costImpact","scheduleImpact","riskImpact","approvalStatus","rollbackPlan","communicationPlan","versionFrom","versionTo","auditEventIds"]) as unknown as ChangeRecord;
}

export function parseCollaborationRecord(value: unknown, at = "$"): CollaborationRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","sessionType","title","participants","facilitatorId","startTime","endTime","location","agenda","outcomes","actionItems","decisionIds","issueIds","documentIds","recordingRef","feedback","followUpDate","workshopId","surveyId"], ["id","name","status","priority","ownership","timestamps","sessionType","title","participants","facilitatorId","startTime","endTime","location","agenda","outcomes","actionItems","decisionIds","issueIds","documentIds","recordingRef","feedback","followUpDate","workshopId","surveyId"]) as unknown as CollaborationRecord;
}

export function parseCommunicationRequirement(value: unknown, at = "$"): CommunicationRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","channel","audienceIds","messageTypes","frequency","medium","language","accessibility","emergencyUse","twoWay","recordingPolicy","signageLocations","technology","escalationPath","feedbackLoop","privacyControls","elementIds","standards","ownerId","templates"], ["id","name","status","priority","ownership","timestamps","channel","audienceIds","messageTypes","frequency","medium","language","accessibility","emergencyUse","twoWay","recordingPolicy","signageLocations","technology","escalationPath","feedbackLoop","privacyControls","elementIds","standards","ownerId","templates"]) as unknown as CommunicationRequirement;
}

export function parseComplianceRecord(value: unknown, at = "$"): ComplianceRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","standardRef","obligation","complianceStatus","evidenceRefs","auditorId","auditDate","nextReview","affectedEntityIds","gapAnalysis","remediationPlan","ownerId","severity","regulatoryBody","certificationTarget","waiverStatus","relatedRequirementIds","monitoringMethod","reportingFrequency","penalties","correctiveActions","artifactRefs"], ["id","name","status","priority","ownership","timestamps","standardRef","obligation","complianceStatus","evidenceRefs","auditorId","auditDate","nextReview","affectedEntityIds","gapAnalysis","remediationPlan","ownerId","severity","regulatoryBody","certificationTarget","waiverStatus","relatedRequirementIds","monitoringMethod","reportingFrequency","penalties","correctiveActions","artifactRefs"]) as unknown as ComplianceRecord;
}

export function parseConflict(value: unknown, at = "$"): Conflict {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","kind","summary","entityAId","entityBId","severity","detectedBy","detectionDate","tradeOffOptions","recommendedResolution","decisionId","stakeholderIds","requirementIds","costImpact","scheduleImpact","qualityImpact","resolutionStatus","ownerId","escalationLevel","relatedRiskIds"], ["id","name","status","priority","ownership","timestamps","kind","summary","entityAId","entityBId","severity","detectedBy","detectionDate","tradeOffOptions","recommendedResolution","decisionId","stakeholderIds","requirementIds","costImpact","scheduleImpact","qualityImpact","resolutionStatus","ownerId","escalationLevel","relatedRiskIds"]) as unknown as Conflict;
}

export function parseConstraintRecord(value: unknown, at = "$"): ConstraintRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","constraintType","summary","severity","affectedEntityIds","source","regulatoryBasis","mitigationOptions","ownerId","effectiveDate","expiryDate","waiverStatus","waiverApprover","impactAssessment","resolutionPlan","relatedRequirementIds","relatedDecisionIds","monitoringFrequency","complianceStatus","exceptions","traceLinks","escalationContactId"], ["id","name","status","priority","ownership","timestamps","constraintType","summary","severity","affectedEntityIds","source","regulatoryBasis","mitigationOptions","ownerId","effectiveDate","expiryDate","waiverStatus","waiverApprover","impactAssessment","resolutionPlan","relatedRequirementIds","relatedDecisionIds","monitoringFrequency","complianceStatus","exceptions","traceLinks","escalationContactId"]) as unknown as ConstraintRecord;
}

export function parseCostRequirement(value: unknown, at = "$"): CostRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","costItem","basis","amount","currency","quantityBasis","unitCost","contingencyPercent","escalationRate","fundingSource","elementIds","requirementIds","phase","cashFlowProfile","valueEngineeringNotes","benchmarkRef","approvalStatus","ownerId","assumptions","sensitivityFactors"], ["id","name","status","priority","ownership","timestamps","costItem","basis","amount","currency","quantityBasis","unitCost","contingencyPercent","escalationRate","fundingSource","elementIds","requirementIds","phase","cashFlowProfile","valueEngineeringNotes","benchmarkRef","approvalStatus","ownerId","assumptions","sensitivityFactors"]) as unknown as CostRequirement;
}

export function parseDecision(value: unknown, at = "$"): Decision {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","decisionStatement","context","optionsConsidered","selectedOptionId","rationale","decisionMakerIds","consultedIds","informedIds","decisionDate","effectiveDate","reversalConditions","impactedRequirementIds","impactedElementIds","costImpact","scheduleImpact","riskImpact","approvalStatus","meetingRef","artifactRefs"], ["id","name","status","priority","ownership","timestamps","decisionStatement","context","optionsConsidered","selectedOptionId","rationale","decisionMakerIds","consultedIds","informedIds","decisionDate","effectiveDate","reversalConditions","impactedRequirementIds","impactedElementIds","costImpact","scheduleImpact","riskImpact","approvalStatus","meetingRef","artifactRefs"]) as unknown as Decision;
}

export function parseDeliveryConstraint(value: unknown, at = "$"): DeliveryConstraint {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","constraintType","constraintDetails","phase","hardDeadline","softDeadline","impactedElementIds","impactedRequirementIds","workHours","noiseRestrictions","accessRestrictions","siteLogistics","procurementLeadTime","approvalGates","occupancyConstraints","weatherWindows","penaltyClauses","mitigationOptions","ownerId","riskIds","constraintStatus"], ["id","name","status","priority","ownership","timestamps","constraintType","constraintDetails","phase","hardDeadline","softDeadline","impactedElementIds","impactedRequirementIds","workHours","noiseRestrictions","accessRestrictions","siteLogistics","procurementLeadTime","approvalGates","occupancyConstraints","weatherWindows","penaltyClauses","mitigationOptions","ownerId","riskIds","constraintStatus"]) as unknown as DeliveryConstraint;
}

export function parseArtifactRecord(value: unknown, at = "$"): ArtifactRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","documentType","title","version","fileRef","format","authorIds","reviewerIds","approverIds","issueDate","revisionDate","distributionList","relatedEntityIds","classification","retentionPeriod","accessControls","supersedes","documentStatus","checksum","sourceSystem"], ["id","name","status","priority","ownership","timestamps","documentType","title","version","fileRef","format","authorIds","reviewerIds","approverIds","issueDate","revisionDate","distributionList","relatedEntityIds","classification","retentionPeriod","accessControls","supersedes","documentStatus","checksum","sourceSystem"]) as unknown as ArtifactRecord;
}

export function parseEnvironmentalRequirement(value: unknown, at = "$"): EnvironmentalRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","parameterKind","parameter","targetValue","unit","minValue","maxValue","comfortBand","measurementMethod","monitoringFrequency","elementIds","occupancyBasis","seasonalVariation","energyImplications","standards","certificationTargets","outdoorConditions","ventilationStrategy","daylightTarget","acousticTarget","iaqTarget","verificationPlan"], ["id","name","status","priority","ownership","timestamps","parameterKind","parameter","targetValue","unit","minValue","maxValue","comfortBand","measurementMethod","monitoringFrequency","elementIds","occupancyBasis","seasonalVariation","energyImplications","standards","certificationTargets","outdoorConditions","ventilationStrategy","daylightTarget","acousticTarget","iaqTarget","verificationPlan"]) as unknown as EnvironmentalRequirement;
}

export function parseEquipment(value: unknown, at = "$"): Equipment {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","category","manufacturer","model","quantity","dimensions","weightKg","powerKw","utilityConnections","ventilation","noiseLevelDb","clearance","mounting","elementIds","activityIds","maintenanceAccess","lifecycleYears","replacementCost","standards","supplier","activityLinkIds","installationRequirements","commissioningNotes","spareParts"], ["id","name","status","priority","ownership","timestamps","code","category","manufacturer","model","quantity","dimensions","weightKg","powerKw","utilityConnections","ventilation","noiseLevelDb","clearance","mounting","elementIds","activityIds","maintenanceAccess","lifecycleYears","replacementCost","standards","supplier","activityLinkIds","installationRequirements","commissioningNotes","spareParts"]) as unknown as Equipment;
}

export function parseFlexibilityRequirement(value: unknown, at = "$"): FlexibilityRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","flexibilityType","elementIds","adaptationScenarios","modularityLevel","reconfigurationTime","costOfChange","technologyReadiness","futureFunctionIds","demountablePartitions","raisedFloor","overheadServices","expansionDirection","contractionScenario","multiUsePotential","furnitureStrategy","infrastructureSpareCapacity","leaseImplications","ownerId"], ["id","name","status","priority","ownership","timestamps","flexibilityType","elementIds","adaptationScenarios","modularityLevel","reconfigurationTime","costOfChange","technologyReadiness","futureFunctionIds","demountablePartitions","raisedFloor","overheadServices","expansionDirection","contractionScenario","multiUsePotential","furnitureStrategy","infrastructureSpareCapacity","leaseImplications","ownerId"]) as unknown as FlexibilityRequirement;
}

export function parseFlowRequirement(value: unknown, at = "$"): FlowRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","fromElementId","toElementId","kind","flowType","direction","volume","peakRate","clearWidthM","clearHeightM","separationRequirements","accessLevel","timeWindows","equipmentClearance","signageRequired","escortRequired","emergencyRoute","barrierFree","monitoringRequired","processId","conflictIds","verificationMethod"], ["id","name","status","priority","ownership","timestamps","fromElementId","toElementId","kind","flowType","direction","volume","peakRate","clearWidthM","clearHeightM","separationRequirements","accessLevel","timeWindows","equipmentClearance","signageRequired","escortRequired","emergencyRoute","barrierFree","monitoringRequired","processId","conflictIds","verificationMethod"]) as unknown as FlowRequirement;
}

export function parseFunction(value: unknown, at = "$"): Function {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","kind","purpose","criticality","performanceTargets","serviceLevel","operatingHours","staffing","equipmentIds","resourceIds","activityIds","elementIds","dependencies","interfaces","constraints","qualityCriteria","regulatoryRefs","futureChanges","ownerStakeholderId","successMetrics","hierarchyParentId","conflictIds"], ["id","name","status","priority","ownership","timestamps","code","kind","purpose","criticality","performanceTargets","serviceLevel","operatingHours","staffing","equipmentIds","resourceIds","activityIds","elementIds","dependencies","interfaces","constraints","qualityCriteria","regulatoryRefs","futureChanges","ownerStakeholderId","successMetrics","hierarchyParentId","conflictIds"]) as unknown as Function;
}

export function parseGovernance(value: unknown, at = "$"): Governance {
  return architectProgramArtifactGuardExactObject(value, at, ["id","framework","roles","responsibilities","approvalMatrix","escalationPaths","meetingCadence","decisionRights","changeControlProcess","qualityPolicy","riskAppetite","complianceObligations","auditSchedule","documentControl","stakeholderEngagementPlan","ethicsPolicy","dataGovernance","ownerId","reviewCycle","reviewHierarchy","policyOwnershipId","requirementOwnershipId","riskOwnershipId","reportingFrequency","accountabilityRules","exceptionManagement","governancePerformance"], ["id","framework","roles","responsibilities","approvalMatrix","escalationPaths","meetingCadence","decisionRights","changeControlProcess","qualityPolicy","riskAppetite","complianceObligations","auditSchedule","documentControl","stakeholderEngagementPlan","ethicsPolicy","dataGovernance","ownerId","reviewCycle","reviewHierarchy","policyOwnershipId","requirementOwnershipId","riskOwnershipId","reportingFrequency","accountabilityRules","exceptionManagement","governancePerformance"]) as unknown as Governance;
}

export function parseGrowthPlan(value: unknown, at = "$"): GrowthPlan {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","horizonYears","growthRate","headcountGrowth","areaGrowth","phases","triggerEvents","expansionElementIds","reserveAreas","infrastructureHeadroom","budgetEnvelope","fundingSources","riskFactors","decisionPoints","scenarioIds","decommissionPlan","relocationStrategy","stakeholderImpact","regulatoryConsiderations","ownerId"], ["id","name","status","priority","ownership","timestamps","horizonYears","growthRate","headcountGrowth","areaGrowth","phases","triggerEvents","expansionElementIds","reserveAreas","infrastructureHeadroom","budgetEnvelope","fundingSources","riskFactors","decisionPoints","scenarioIds","decommissionPlan","relocationStrategy","stakeholderImpact","regulatoryConsiderations","ownerId"]) as unknown as GrowthPlan;
}

export function parseHumanFactorRequirement(value: unknown, at = "$"): HumanFactorRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","aspect","factor","userProfileIds","activityIds","ergonomicCriteria","cognitiveLoad","visualDemands","auditoryDemands","postureRequirements","reachEnvelope","lightingForTasks","thermalComfort","privacyNeeds","socialInteraction","stressFactors","mitigationMeasures","trainingNeeds","standards","researchBasis","elementIds","verificationMethod"], ["id","name","status","priority","ownership","timestamps","aspect","factor","userProfileIds","activityIds","ergonomicCriteria","cognitiveLoad","visualDemands","auditoryDemands","postureRequirements","reachEnvelope","lightingForTasks","thermalComfort","privacyNeeds","socialInteraction","stressFactors","mitigationMeasures","trainingNeeds","standards","researchBasis","elementIds","verificationMethod"]) as unknown as HumanFactorRequirement;
}

export function parseInformationRequirement(value: unknown, at = "$"): InformationRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","informationType","format","sourceSystem","destinationSystems","updateFrequency","retentionPeriod","accessControls","classification","qualityCriteria","metadataRequirements","integrationPoints","backupRequirements","disasterRecovery","privacyControls","auditTrail","elementIds","stakeholderIds","standards","ownerId"], ["id","name","status","priority","ownership","timestamps","informationType","format","sourceSystem","destinationSystems","updateFrequency","retentionPeriod","accessControls","classification","qualityCriteria","metadataRequirements","integrationPoints","backupRequirements","disasterRecovery","privacyControls","auditTrail","elementIds","stakeholderIds","standards","ownerId"]) as unknown as InformationRequirement;
}

export function parseInfrastructureRequirement(value: unknown, at = "$"): InfrastructureRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","system","category","capacity","redundancy","distribution","entryPoints","utilitySource","standbyPower","monitoring","maintenanceAccess","standards","elementIds","peakDemand","diversityFactor","futureExpansion","interfaceRequirements","commissioning","lifecycleCost","ownerId"], ["id","name","status","priority","ownership","timestamps","system","category","capacity","redundancy","distribution","entryPoints","utilitySource","standbyPower","monitoring","maintenanceAccess","standards","elementIds","peakDemand","diversityFactor","futureExpansion","interfaceRequirements","commissioning","lifecycleCost","ownerId"]) as unknown as InfrastructureRequirement;
}

export function parseIssue(value: unknown, at = "$"): Issue {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","issueType","summary","issueDescription","severity","issuePriority","reporterId","assigneeId","affectedEntityIds","rootCause","resolution","workaround","dueDate","resolvedDate","relatedConflictIds","relatedRiskIds","decisionId","comments","attachments","escalationLevel"], ["id","name","status","priority","ownership","timestamps","issueType","summary","issueDescription","severity","issuePriority","reporterId","assigneeId","affectedEntityIds","rootCause","resolution","workaround","dueDate","resolvedDate","relatedConflictIds","relatedRiskIds","decisionId","comments","attachments","escalationLevel"]) as unknown as Issue;
}

export function parseKnowledgeRecord(value: unknown, at = "$"): KnowledgeRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","topic","category","summary","content","sources","references","lessonsLearned","bestPractices","applicableSectors","relatedEntityKinds","authorIds","expertiseLevel","validationStatus","lastReviewed","keywords","attachments","citations","usageCount"], ["id","name","status","priority","ownership","timestamps","topic","category","summary","content","sources","references","lessonsLearned","bestPractices","applicableSectors","relatedEntityKinds","authorIds","expertiseLevel","validationStatus","lastReviewed","keywords","attachments","citations","usageCount"]) as unknown as KnowledgeRecord;
}

export function parseMeetingRecord(value: unknown, at = "$"): MeetingRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","meetingType","scheduledDate","duration","location","chairId","attendeeIds","agendaItems","minutes","actionItems","decisionsMade","artifactRefs","followUpDate","recordingRef","quorumMet","meetingStatus","workshopId","stakeholderIds","requirementIds","issueIds","approvalIds"], ["id","name","status","priority","ownership","timestamps","meetingType","scheduledDate","duration","location","chairId","attendeeIds","agendaItems","minutes","actionItems","decisionsMade","artifactRefs","followUpDate","recordingRef","quorumMet","meetingStatus","workshopId","stakeholderIds","requirementIds","issueIds","approvalIds"]) as unknown as MeetingRecord;
}

export function parseOperationalRequirement(value: unknown, at = "$"): OperationalRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","operation","serviceLevel","operatingHours","staffing","maintenanceInterval","cleaningRegime","turnaroundTime","redundancy","uptimeTarget","responseTime","equipmentIds","elementIds","processIds","utilities","wasteStreams","contingencyPlan","trainingRequirements","sopReferences","kpiTargets","ownerId","serviceCategory","shiftPattern","slaTarget","escalationContactId"], ["id","name","status","priority","ownership","timestamps","operation","serviceLevel","operatingHours","staffing","maintenanceInterval","cleaningRegime","turnaroundTime","redundancy","uptimeTarget","responseTime","equipmentIds","elementIds","processIds","utilities","wasteStreams","contingencyPlan","trainingRequirements","sopReferences","kpiTargets","ownerId","serviceCategory","shiftPattern","slaTarget","escalationContactId"]) as unknown as OperationalRequirement;
}

export function parseOptionEvaluation(value: unknown, at = "$"): OptionEvaluation {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","optionName","optionDescription","scenarioId","criteriaIds","scores","weightedScore","costEstimate","scheduleEstimate","riskSummary","benefits","drawbacks","assumptions","dependencies","stakeholderFeedback","recommendation","decisionId","evaluationStatus","evaluatorIds","evaluationDate"], ["id","name","status","priority","ownership","timestamps","optionName","optionDescription","scenarioId","criteriaIds","scores","weightedScore","costEstimate","scheduleEstimate","riskSummary","benefits","drawbacks","assumptions","dependencies","stakeholderFeedback","recommendation","decisionId","evaluationStatus","evaluatorIds","evaluationDate"]) as unknown as OptionEvaluation;
}

export function parseOrganizationalRequirement(value: unknown, at = "$"): OrganizationalRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","department","reportingLine","headcount","growthPlanId","workPatterns","collaborationModel","hierarchyLevels","decisionMaking","cultureNotes","changeReadiness","unionConsiderations","trainingNeeds","elementIds","stakeholderIds","serviceRequirementIds","brandingRequirements","wellnessPlugins","diversityGoals","ownerId"], ["id","name","status","priority","ownership","timestamps","department","reportingLine","headcount","growthPlanId","workPatterns","collaborationModel","hierarchyLevels","decisionMaking","cultureNotes","changeReadiness","unionConsiderations","trainingNeeds","elementIds","stakeholderIds","serviceRequirementIds","brandingRequirements","wellnessPlugins","diversityGoals","ownerId"]) as unknown as OrganizationalRequirement;
}

export function parsePerformanceCriterion(value: unknown, at = "$"): PerformanceCriterion {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","criterion","metric","target","unit","minimum","maximum","measurementMethod","frequency","requirementIds","elementIds","baseline","benchmarkRef","weight","dataSource","reportingCadence","ownerId","verificationPlan","penaltyThreshold","incentiveThreshold"], ["id","name","status","priority","ownership","timestamps","criterion","metric","target","unit","minimum","maximum","measurementMethod","frequency","requirementIds","elementIds","baseline","benchmarkRef","weight","dataSource","reportingCadence","ownerId","verificationPlan","penaltyThreshold","incentiveThreshold"]) as unknown as PerformanceCriterion;
}

export function parsePriorityRecord(value: unknown, at = "$"): PriorityRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","subjectId","subjectKind","rankedPriority","rank","weight","rationale","decisionId","stakeholderIds","effectiveFrom","effectiveUntil","reviewCycle","dependencies","conflicts","scoringMethod","score","criteria","approvedBy","approvalDate","rankingNotes"], ["id","name","status","priority","ownership","timestamps","subjectId","subjectKind","rankedPriority","rank","weight","rationale","decisionId","stakeholderIds","effectiveFrom","effectiveUntil","reviewCycle","dependencies","conflicts","scoringMethod","score","criteria","approvedBy","approvalDate","rankingNotes"]) as unknown as PriorityRecord;
}

export function parsePrivacyRequirement(value: unknown, at = "$"): PrivacyRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","privacyKind","privacyType","level","subjectIds","elementIds","visualPrivacy","acousticPrivacy","dataPrivacy","screeningRequired","enclosureRequired","accessRestrictions","observationRisk","regulatoryBasis","culturalConsiderations","technologyControls","signage","monitoringRestrictions","retentionPolicy","breachResponse","ownerId"], ["id","name","status","priority","ownership","timestamps","privacyKind","privacyType","level","subjectIds","elementIds","visualPrivacy","acousticPrivacy","dataPrivacy","screeningRequired","enclosureRequired","accessRestrictions","observationRisk","regulatoryBasis","culturalConsiderations","technologyControls","signage","monitoringRestrictions","retentionPolicy","breachResponse","ownerId"]) as unknown as PrivacyRequirement;
}

export function parseProcess(value: unknown, at = "$"): Process {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","category","trigger","inputs","outputs","steps","actors","equipmentIds","elementIds","duration","frequency","criticalPath","bottlenecks","dependencies","kpis","automationLevel","failureModes","improvementOpportunities","regulatoryRefs","ownerId","workflowType","handoffPoints","qualityGates"], ["id","name","status","priority","ownership","timestamps","code","category","trigger","inputs","outputs","steps","actors","equipmentIds","elementIds","duration","frequency","criticalPath","bottlenecks","dependencies","kpis","automationLevel","failureModes","improvementOpportunities","regulatoryRefs","ownerId","workflowType","handoffPoints","qualityGates"]) as unknown as Process;
}

export function parseProgramElement(value: unknown, at = "$"): ProgramElement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","kind","parentId","level","area","volume","height","occupancy","functionIds","activityIds","userProfileIds","adjacencyIds","quantityIds","requirementIds","locationHint","orientation","daylightRequirement","acousticClass","securityZone","flexibilityNotes","growthAllocation","circulationRole","visibilityLevel","adjacencyPreferences","environmentalZone"], ["id","name","status","priority","ownership","timestamps","code","kind","parentId","level","area","volume","height","occupancy","functionIds","activityIds","userProfileIds","adjacencyIds","quantityIds","requirementIds","locationHint","orientation","daylightRequirement","acousticClass","securityZone","flexibilityNotes","growthAllocation","circulationRole","visibilityLevel","adjacencyPreferences","environmentalZone"]) as unknown as ProgramElement;
}

export function parseProgramMeta(value: unknown, at = "$"): ProgramMeta {
  return architectProgramArtifactGuardExactObject(value, at, ["schema","documentId","title","subtitle","purpose","terminology","classification","industrySector","projectType","locale","revision","authorIds","sourceSystem","exportProfile","timestamps"], ["schema","documentId","title","subtitle","purpose","terminology","classification","industrySector","projectType","locale","revision","authorIds","sourceSystem","exportProfile","timestamps"]) as unknown as ProgramMeta;
}

export function parseProjectDefinition(value: unknown, at = "$"): ProjectDefinition {
  return architectProgramArtifactGuardExactObject(value, at, ["id","code","clientName","ownerOrganization","briefSummary","problemStatement","vision","mission","objectives","successCriteria","projectPriorities","completionCriteria","decisionCriteria","scopeInclusions","scopeExclusions","assumptions","constraintsSummary","dependencies","deliverables","phases","geographicContext","developmentContext","operationalContext","regulatoryContext","fundingModel","ownership","timestamps"], ["id","code","clientName","ownerOrganization","briefSummary","problemStatement","vision","mission","objectives","successCriteria","projectPriorities","completionCriteria","decisionCriteria","scopeInclusions","scopeExclusions","assumptions","constraintsSummary","dependencies","deliverables","phases","geographicContext","developmentContext","operationalContext","regulatoryContext","fundingModel","ownership","timestamps"]) as unknown as ProjectDefinition;
}

export function parseQualityRecord(value: unknown, at = "$"): QualityRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","qualityTopic","standard","targetLevel","inspectionPoints","acceptanceCriteria","testingRequirements","sampleRate","defectCategories","correctiveActionProcess","elementIds","requirementIds","supplierRequirements","documentationRequirements","trainingRequirements","auditSchedule","kpis","ownerId","certificationTargets","continuousImprovement"], ["id","name","status","priority","ownership","timestamps","qualityTopic","standard","targetLevel","inspectionPoints","acceptanceCriteria","testingRequirements","sampleRate","defectCategories","correctiveActionProcess","elementIds","requirementIds","supplierRequirements","documentationRequirements","trainingRequirements","auditSchedule","kpis","ownerId","certificationTargets","continuousImprovement"]) as unknown as QualityRecord;
}

export function parseQuantityRequirement(value: unknown, at = "$"): QuantityRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","targetElementId","metric","quantity","basis","calculationMethod","source","benchmarkRef","tolerancePercent","peakFactor","growthFactor","unitCost","currency","verificationMethod","relatedRequirementIds","assumptions","constraints","schedulePhase","responsibleParty","lastVerified","varianceNotes"], ["id","name","status","priority","ownership","timestamps","targetElementId","metric","quantity","basis","calculationMethod","source","benchmarkRef","tolerancePercent","peakFactor","growthFactor","unitCost","currency","verificationMethod","relatedRequirementIds","assumptions","constraints","schedulePhase","responsibleParty","lastVerified","varianceNotes"]) as unknown as QuantityRequirement;
}

export function parseRegulatoryRequirement(value: unknown, at = "$"): RegulatoryRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","jurisdiction","code","clause","title","requirementText","applicability","elementIds","complianceMethod","evidenceRequired","authority","effectiveDate","expiryDate","penalties","exemptions","relatedRequirementIds","interpretationNotes","verificationStatus","consultantRefs","updateSource"], ["id","name","status","priority","ownership","timestamps","jurisdiction","code","clause","title","requirementText","applicability","elementIds","complianceMethod","evidenceRequired","authority","effectiveDate","expiryDate","penalties","exemptions","relatedRequirementIds","interpretationNotes","verificationStatus","consultantRefs","updateSource"]) as unknown as RegulatoryRequirement;
}

export function parseRelationship(value: unknown, at = "$"): Relationship {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","sourceId","targetId","kind","strength","directional","rationale","constraints","conditions","relationshipPriority","validFrom","validUntil","evidence","conflictIds","traceLinks","bidirectional","distanceConstraintM","capacityConstraint","regulatoryBasis","reviewCycle","ownerId","proximityRequirement","compatibilityRequirement","incompatibilityRequirement","separationRequirements"], ["id","name","status","priority","ownership","timestamps","sourceId","targetId","kind","strength","directional","rationale","constraints","conditions","relationshipPriority","validFrom","validUntil","evidence","conflictIds","traceLinks","bidirectional","distanceConstraintM","capacityConstraint","regulatoryBasis","reviewCycle","ownerId","proximityRequirement","compatibilityRequirement","incompatibilityRequirement","separationRequirements"]) as unknown as Relationship;
}

export function parseReportRecord(value: unknown, at = "$"): ReportRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","kind","title","audience","sections","generatedAt","generatedBy","analysisIds","format","fileRef","distributionList","approvalStatus","approverId","version","templateId","parameters","confidentiality","expiryDate","relatedDecisionIds"], ["id","name","status","priority","ownership","timestamps","kind","title","audience","sections","generatedAt","generatedBy","analysisIds","format","fileRef","distributionList","approvalStatus","approverId","version","templateId","parameters","confidentiality","expiryDate","relatedDecisionIds"]) as unknown as ReportRecord;
}

export function parseRequirement(value: unknown, at = "$"): Requirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","kind","statement","rationale","source","stakeholderIds","elementIds","functionIds","parentRequirementId","childRequirementIds","acceptanceCriteria","verificationMethod","validationStatus","conflictIds","riskIds","costEstimate","scheduleConstraint","regulatoryRefs","traceLinks","supersededBy"], ["id","name","status","priority","ownership","timestamps","code","kind","statement","rationale","source","stakeholderIds","elementIds","functionIds","parentRequirementId","childRequirementIds","acceptanceCriteria","verificationMethod","validationStatus","conflictIds","riskIds","costEstimate","scheduleConstraint","regulatoryRefs","traceLinks","supersededBy"]) as unknown as Requirement;
}

export function parseResilienceRequirement(value: unknown, at = "$"): ResilienceRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","hazard","riskLevel","scenario","recoveryTime","recoveryPoint","redundancy","hardeningMeasures","backupSystems","alternateSites","supplyChain","communicationPlan","drillRequirements","elementIds","infrastructureIds","standards","insuranceImplications","climateAdaptation","ownerId","verificationPlan"], ["id","name","status","priority","ownership","timestamps","hazard","riskLevel","scenario","recoveryTime","recoveryPoint","redundancy","hardeningMeasures","backupSystems","alternateSites","supplyChain","communicationPlan","drillRequirements","elementIds","infrastructureIds","standards","insuranceImplications","climateAdaptation","ownerId","verificationPlan"]) as unknown as ResilienceRequirement;
}

export function parseResource(value: unknown, at = "$"): Resource {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","category","resourceType","quantity","mobility","sharingModel","allocation","elementIds","activityIds","userProfileIds","storageRequirementId","durability","cleaningRequirements","replacementCycle","costPerUnit","supplier","standards","ergonomicNotes","customization","disposalNotes","furnitureClass","ergonomicsRating","sharingRatio"], ["id","name","status","priority","ownership","timestamps","code","category","resourceType","quantity","mobility","sharingModel","allocation","elementIds","activityIds","userProfileIds","storageRequirementId","durability","cleaningRequirements","replacementCycle","costPerUnit","supplier","standards","ergonomicNotes","customization","disposalNotes","furnitureClass","ergonomicsRating","sharingRatio"]) as unknown as Resource;
}

export function parseRisk(value: unknown, at = "$"): Risk {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","riskStatement","category","probability","impact","riskScore","causes","effects","affectedElementIds","affectedRequirementIds","mitigation","contingency","ownerId","reviewDate","triggerIndicators","residualProbability","residualImpact","relatedConflictIds","escalationPath","monitoringPlan"], ["id","name","status","priority","ownership","timestamps","riskStatement","category","probability","impact","riskScore","causes","effects","affectedElementIds","affectedRequirementIds","mitigation","contingency","ownerId","reviewDate","triggerIndicators","residualProbability","residualImpact","relatedConflictIds","escalationPath","monitoringPlan"]) as unknown as Risk;
}

export function parseSafetyRequirement(value: unknown, at = "$"): SafetyRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","safetyDomain","hazard","riskLevel","affectedElementIds","affectedUserIds","mitigationMeasures","ppeRequirements","emergencyProcedures","evacuationRequirements","fireProtection","structuralSafety","slipTripFall","chemicalSafety","electricalSafety","machinerySafety","standards","inspectionFrequency","trainingRequirements","incidentReporting","residualRisk"], ["id","name","status","priority","ownership","timestamps","safetyDomain","hazard","riskLevel","affectedElementIds","affectedUserIds","mitigationMeasures","ppeRequirements","emergencyProcedures","evacuationRequirements","fireProtection","structuralSafety","slipTripFall","chemicalSafety","electricalSafety","machinerySafety","standards","inspectionFrequency","trainingRequirements","incidentReporting","residualRisk"]) as unknown as SafetyRequirement;
}

export function parseScenario(value: unknown, at = "$"): Scenario {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","hypothesis","assumptions","variables","elementIds","requirementIds","growthPlanId","probability","impactSummary","costDelta","areaDelta","headcountDelta","scheduleDelta","riskIds","optionIds","baseline","preferred","analysisIds","ownerId"], ["id","name","status","priority","ownership","timestamps","code","hypothesis","assumptions","variables","elementIds","requirementIds","growthPlanId","probability","impactSummary","costDelta","areaDelta","headcountDelta","scheduleDelta","riskIds","optionIds","baseline","preferred","analysisIds","ownerId"]) as unknown as Scenario;
}

export function parseScheduleRequirement(value: unknown, at = "$"): ScheduleRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","milestone","phase","startDate","endDate","duration","dependencies","predecessors","successors","critical","floatDays","resourceRequirements","occupancyImpact","phasingStrategy","decantRequirements","commissioningWindow","stakeholderIds","riskIds","contingencyDays","reportingCadence","ownerId"], ["id","name","status","priority","ownership","timestamps","milestone","phase","startDate","endDate","duration","dependencies","predecessors","successors","critical","floatDays","resourceRequirements","occupancyImpact","phasingStrategy","decantRequirements","commissioningWindow","stakeholderIds","riskIds","contingencyDays","reportingCadence","ownerId"]) as unknown as ScheduleRequirement;
}

export function parseSearchFilter(value: unknown, at = "$"): SearchFilter {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","filterName","filterDescription","keywords","categories","ownerIds","statuses","priorities","sources","dateFrom","dateTo","entityKinds","tagFilters","sortField","sortDirection","isPublic","createdBy","lastUsed","useCount","pinned"], ["id","name","status","priority","ownership","timestamps","filterName","filterDescription","keywords","categories","ownerIds","statuses","priorities","sources","dateFrom","dateTo","entityKinds","tagFilters","sortField","sortDirection","isPublic","createdBy","lastUsed","useCount","pinned"]) as unknown as SearchFilter;
}

export function parseSecurityRequirement(value: unknown, at = "$"): SecurityRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","controlKind","threat","riskLevel","assetIds","zoneIds","accessLevel","perimeterControls","surveillance","intrusionDetection","cybersecurity","screening","visitorManagement","keyManagement","standards","responseProcedures","drillFrequency","liaisonContacts","classifiedLevel","redundancy","auditRequirements"], ["id","name","status","priority","ownership","timestamps","controlKind","threat","riskLevel","assetIds","zoneIds","accessLevel","perimeterControls","surveillance","intrusionDetection","cybersecurity","screening","visitorManagement","keyManagement","standards","responseProcedures","drillFrequency","liaisonContacts","classifiedLevel","redundancy","auditRequirements"]) as unknown as SecurityRequirement;
}

export function parseServiceRequirement(value: unknown, at = "$"): ServiceRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","serviceName","serviceType","provider","serviceLevel","operatingHours","capacity","responseTime","queueManagement","customerProfiles","elementIds","equipmentIds","staffing","qualityMetrics","costModel","contractRefs","dependencies","failureImpact","backupService","feedbackChannels"], ["id","name","status","priority","ownership","timestamps","serviceName","serviceType","provider","serviceLevel","operatingHours","capacity","responseTime","queueManagement","customerProfiles","elementIds","equipmentIds","staffing","qualityMetrics","costModel","contractRefs","dependencies","failureImpact","backupService","feedbackChannels"]) as unknown as ServiceRequirement;
}

export function parseSiteContext(value: unknown, at = "$"): SiteContext {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","siteName","address","latitude","longitude","elevationM","climateZone","seismicZone","floodRisk","soilConditions","utilitiesAvailable","accessRoads","publicTransit","neighbors","views","noiseSources","environmentalConstraints","heritageConstraints","zoning","maxHeightM","maxCoverage"], ["id","name","status","priority","ownership","timestamps","siteName","address","latitude","longitude","elevationM","climateZone","seismicZone","floodRisk","soilConditions","utilitiesAvailable","accessRoads","publicTransit","neighbors","views","noiseSources","environmentalConstraints","heritageConstraints","zoning","maxHeightM","maxCoverage"]) as unknown as SiteContext;
}

export function parseStakeholder(value: unknown, at = "$"): Stakeholder {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","role","organization","department","contactEmail","contactPhone","influence","interest","engagement","expectations","concerns","requirementIds","decisionAuthority","communicationPreferences","reportingFrequency","involvementPhases","availability","representativeOf","delegatedTo","relationshipToClient","powerInterestNotes","stakeholderType","influenceStrategy","communicationChannels","successMetrics"], ["id","name","status","priority","ownership","timestamps","role","organization","department","contactEmail","contactPhone","influence","interest","engagement","expectations","concerns","requirementIds","decisionAuthority","communicationPreferences","reportingFrequency","involvementPhases","availability","representativeOf","delegatedTo","relationshipToClient","powerInterestNotes","stakeholderType","influenceStrategy","communicationChannels","successMetrics"]) as unknown as Stakeholder;
}

export function parseStatusRecord(value: unknown, at = "$"): StatusRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","subjectId","subjectKind","recordStatus","previousStatus","changedBy","changedAt","reason","blockers","nextActions","dueDate","progressPercent","health","escalationLevel","relatedIssueIds","relatedRiskIds","milestoneId","reportingPeriod","statusNotes"], ["id","name","status","priority","ownership","timestamps","subjectId","subjectKind","recordStatus","previousStatus","changedBy","changedAt","reason","blockers","nextActions","dueDate","progressPercent","health","escalationLevel","relatedIssueIds","relatedRiskIds","milestoneId","reportingPeriod","statusNotes"]) as unknown as StatusRecord;
}

export function parseStorageRequirement(value: unknown, at = "$"): StorageRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","storedItem","storageClass","quantity","volumeM3","weightKg","temperatureRange","humidityRange","securityLevel","hazardClass","retentionPeriod","accessFrequency","elementIds","equipmentIds","handlingEquipment","fireProtection","ventilation","organizationSystem","growthAllowance","regulatoryRefs","ownerId"], ["id","name","status","priority","ownership","timestamps","storedItem","storageClass","quantity","volumeM3","weightKg","temperatureRange","humidityRange","securityLevel","hazardClass","retentionPeriod","accessFrequency","elementIds","equipmentIds","handlingEquipment","fireProtection","ventilation","organizationSystem","growthAllowance","regulatoryRefs","ownerId"]) as unknown as StorageRequirement;
}

export function parseSurvey(value: unknown, at = "$"): Survey {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","surveyType","title","objectives","questions","targetAudience","distributionChannels","launchDate","closeDate","responseCount","responseRate","findings","themes","recommendations","confidentiality","consentProcess","analysisId","workshopId","ownerId","surveyStatus"], ["id","name","status","priority","ownership","timestamps","surveyType","title","objectives","questions","targetAudience","distributionChannels","launchDate","closeDate","responseCount","responseRate","findings","themes","recommendations","confidentiality","consentProcess","analysisId","workshopId","ownerId","surveyStatus"]) as unknown as Survey;
}

export function parseSustainabilityRequirement(value: unknown, at = "$"): SustainabilityRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","topic","target","metric","baseline","targetValue","unit","certification","standards","elementIds","strategies","materialsPreferences","energyStrategy","waterStrategy","wasteStrategy","biodiversity","embodiedCarbon","operationalCarbon","reportingRequirements","verificationPlan","ownerId"], ["id","name","status","priority","ownership","timestamps","topic","target","metric","baseline","targetValue","unit","certification","standards","elementIds","strategies","materialsPreferences","energyStrategy","waterStrategy","wasteStrategy","biodiversity","embodiedCarbon","operationalCarbon","reportingRequirements","verificationPlan","ownerId"]) as unknown as SustainabilityRequirement;
}

export function parseTemplateRecord(value: unknown, at = "$"): TemplateRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","templateType","sector","projectType","version","contentRef","entityKinds","defaultFields","checklists","standards","applicability","authorId","approvalStatus","usageCount","lastApplied","customizationNotes","relatedKnowledgeIds","benchmarkIds","license","sourceOrganization"], ["id","name","status","priority","ownership","timestamps","templateType","sector","projectType","version","contentRef","entityKinds","defaultFields","checklists","standards","applicability","authorId","approvalStatus","usageCount","lastApplied","customizationNotes","relatedKnowledgeIds","benchmarkIds","license","sourceOrganization"]) as unknown as TemplateRecord;
}

export function parseTraceLink(value: unknown, at = "$"): TraceLink {
  return architectProgramArtifactGuardExactObject(value, at, ["id","fromId","toId","kind","label"], ["id","fromId","toId","kind"]) as unknown as TraceLink;
}

export function parseUserProfile(value: unknown, at = "$"): UserProfile {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","category","demographic","ageRange","abilities","disabilities","occupation","roleTitle","department","mobilityProfile","sensoryProfile","cognitiveProfile","behavioralPatterns","usageFrequency","usageDuration","peakUsageTimes","technologyProficiency","preferences","painPoints","goals","activityIds","researchMethod","personaArchetype","validated","stakeholderIds"], ["id","name","status","priority","ownership","timestamps","category","demographic","ageRange","abilities","disabilities","occupation","roleTitle","department","mobilityProfile","sensoryProfile","cognitiveProfile","behavioralPatterns","usageFrequency","usageDuration","peakUsageTimes","technologyProficiency","preferences","painPoints","goals","activityIds","researchMethod","personaArchetype","validated","stakeholderIds"]) as unknown as UserProfile;
}

export function parseValidationRecord(value: unknown, at = "$"): ValidationRecord {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","subjectId","subjectKind","validationType","method","criteria","result","evidence","validatorIds","validationDate","nextReviewDate","findings","nonConformities","correctiveActions","waivers","standards","traceLinks","reportId","confidenceLevel","validationNotes"], ["id","name","status","priority","ownership","timestamps","subjectId","subjectKind","validationType","method","criteria","result","evidence","validatorIds","validationDate","nextReviewDate","findings","nonConformities","correctiveActions","waivers","standards","traceLinks","reportId","confidenceLevel","validationNotes"]) as unknown as ValidationRecord;
}

export function parseWayfindingRequirement(value: unknown, at = "$"): WayfindingRequirement {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","userProfileIds","elementIds","destinationTypes","signageTypes","languages","tactileRequired","audioRequired","digitalWayfinding","landmarkStrategy","colorCoding","symbolStandards","decisionPoints","maximumSignageDistanceM","lightingRequirements","maintenancePlan","emergencyEgress","visitorJourney","staffJourney","brandIntegration"], ["id","name","status","priority","ownership","timestamps","userProfileIds","elementIds","destinationTypes","signageTypes","languages","tactileRequired","audioRequired","digitalWayfinding","landmarkStrategy","colorCoding","symbolStandards","decisionPoints","maximumSignageDistanceM","lightingRequirements","maintenancePlan","emergencyEgress","visitorJourney","staffJourney","brandIntegration"]) as unknown as WayfindingRequirement;
}

export function parseWorkshop(value: unknown, at = "$"): Workshop {
  return architectProgramArtifactGuardExactObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","workshopType","objectives","agenda","facilitatorId","participants","scheduledStart","scheduledEnd","location","materials","methods","outputs","decisions","issues","followUpActions","feedback","recordingRef","budget","workshopStatus","surveyIds"], ["id","name","status","priority","ownership","timestamps","workshopType","objectives","agenda","facilitatorId","participants","scheduledStart","scheduledEnd","location","materials","methods","outputs","decisions","issues","followUpActions","feedback","recordingRef","budget","workshopStatus","surveyIds"]) as unknown as Workshop;
}
