/** 🧩 ProgramMutation dispatch union — mirrors 🦀️.rs's `ProgramMutation` enum. Internally
 *  tagged (`#[serde(tag = "mutation", rename_all = "camelCase")]`): each payload struct's own
 *  fields flatten onto the tag, matching every committed `🧪️tests/<case>/🦠️mutation/component.json`
 *  fixture — there is no nested `payload` property. */
//#region 🔖️Entities
/** 🧬️ Register entity types, mirrored field-for-field from `🗄️registers/🦀️.rs` and
 * `🧱️kernel/🦀️.rs` (`EntityHeader` etc, `#[serde(flatten)]`d onto every register row —
 * hence `extends EntityHeader` below instead of a nested `header` field). `DocumentRecord` mirrors
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
  subtitle?: string;
  purpose: TextField;
  terminology: string[];
  classification: string[];
  industrySector: string;
  projectType: string;
  locale: string;
  revision: string;
  authorIds: string[];
  sourceSystem?: string;
  exportProfile?: string;
  timestamps: TimestampMeta;
}

export type Priority = "mandatory" | "essential" | "preferred" | "optional" | "deferred" | "prohibited";

export interface Ownership {
  ownerId?: string;
  authorityId?: string;
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
  tags: string[];
  notes: TaggedNote[];
  timestamps: TimestampMeta;
}

export type InfluenceLevel = "low" | "medium" | "high" | "critical";

export type EngagementLevel = "unaware" | "resistant" | "neutral" | "supportive" | "leading";

export interface Stakeholder extends EntityHeader {
  role: string;
  organization: string;
  department?: string;
  contactEmail?: string;
  contactPhone?: string;
  influence: InfluenceLevel;
  interest: InfluenceLevel;
  engagement: EngagementLevel;
  expectations: string[];
  concerns: string[];
  requirementIds: string[];
  decisionAuthority: boolean;
  communicationPreferences: string[];
  reportingFrequency?: string;
  involvementPhases: string[];
  availability?: string;
  representativeOf?: string;
  delegatedTo?: string;
  relationshipToClient?: string;
  powerInterestNotes: TaggedNote[];
  stakeholderType: string;
  influenceStrategy?: string;
  communicationChannels: string[];
  successMetrics: string[];
}

export type UserCategory = "primary" | "secondary" | "occasional" | "service" | "visitor" | "staff" | "public";

export interface UserProfile extends EntityHeader {
  category: UserCategory;
  demographic?: string;
  ageRange?: string;
  abilities: string[];
  disabilities: string[];
  occupation?: string;
  roleTitle?: string;
  department?: string;
  mobilityProfile: string[];
  sensoryProfile: string[];
  cognitiveProfile: string[];
  behavioralPatterns: string[];
  usageFrequency?: string;
  usageDuration?: string;
  peakUsageTimes: string[];
  technologyProficiency?: string;
  preferences: string[];
  painPoints: string[];
  goals: string[];
  activityIds: string[];
  researchMethod?: string;
  personaArchetype?: string;
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
  frequency?: string;
  duration?: string;
  intensity?: string;
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
  locationContext?: string;
  temporalPattern?: string;
  supervisionLevel?: string;
}

export type FunctionKind = "primary" | "secondary" | "support" | "administrative" | "service" | "technical" | "public" | "private" | "shared" | "restricted" | "temporary" | "future" | "operational" | "circulation";

export interface Function extends EntityHeader {
  code: string;
  kind: FunctionKind;
  purpose: TextField;
  criticality: Priority;
  performanceTargets: string[];
  serviceLevel?: string;
  operatingHours?: string;
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
  ownerStakeholderId?: string;
  successMetrics: string[];
  hierarchyParentId?: string;
  conflictIds: string[];
}

export type ProgramElementKind = "building" | "campus" | "floor" | "zone" | "room" | "suite" | "department" | "system" | "circulation" | "support" | "outdoor" | "furnitureGroup" | "other";

export interface ProgramElement extends EntityHeader {
  code: string;
  kind: ProgramElementKind;
  parentId?: string;
  level?: string;
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
  locationHint?: string;
  orientation?: string;
  daylightRequirement?: string;
  acousticClass?: string;
  securityZone?: string;
  flexibilityNotes: string[];
  growthAllocation?: string;
  circulationRole?: string;
  visibilityLevel?: string;
  adjacencyPreferences: string[];
  environmentalZone?: string;
}

export interface QuantityRequirement extends EntityHeader {
  targetElementId: string;
  metric: string;
  quantity: QuantitySpec;
  basis?: string;
  calculationMethod?: string;
  source?: string;
  benchmarkRef?: string;
  tolerancePercent?: number;
  peakFactor?: number;
  growthFactor?: number;
  unitCost?: number;
  currency?: string;
  verificationMethod?: string;
  relatedRequirementIds: string[];
  assumptions: string[];
  constraints: string[];
  schedulePhase?: string;
  responsibleParty?: string;
  lastVerified?: string;
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
  strength?: number;
  directional: boolean;
  rationale?: TextField;
  constraints: string[];
  conditions: string[];
  relationshipPriority: Priority;
  validFrom?: string;
  validUntil?: string;
  evidence: string[];
  conflictIds: string[];
  traceLinks: TraceLink[];
  bidirectional: boolean;
  distanceConstraintM?: number;
  capacityConstraint?: string;
  regulatoryBasis: string[];
  reviewCycle?: string;
  ownerId?: string;
  proximityRequirement?: TextField;
  compatibilityRequirement?: TextField;
  incompatibilityRequirement?: TextField;
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
  rationale?: TextField;
  distanceMaxM?: number;
  distanceMinM?: number;
  levelConstraint?: string;
  accessPath?: string;
  sharedWall: boolean;
  sharedEntry: boolean;
  trafficIsolation: boolean;
  circulationOverlap: boolean;
  conflictIds: string[];
  normalized: boolean;
  verificationStatus: ValidationStatus;
  sourceRelationshipId?: string;
  internalExternalAccess?: string;
}

export interface Process extends EntityHeader {
  code: string;
  category: string;
  trigger?: string;
  inputs: string[];
  outputs: string[];
  steps: string[];
  actors: string[];
  equipmentIds: string[];
  elementIds: string[];
  duration?: string;
  frequency?: string;
  criticalPath: boolean;
  bottlenecks: string[];
  dependencies: string[];
  kpis: string[];
  automationLevel?: string;
  failureModes: string[];
  improvementOpportunities: string[];
  regulatoryRefs: string[];
  ownerId?: string;
  workflowType?: string;
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
  peakRate?: number;
  clearWidthM?: number;
  clearHeightM?: number;
  separationRequirements: SeparationKind[];
  accessLevel: AccessLevel;
  timeWindows: string[];
  equipmentClearance?: string;
  signageRequired: boolean;
  escortRequired: boolean;
  emergencyRoute: boolean;
  barrierFree: boolean;
  monitoringRequired: boolean;
  processId?: string;
  conflictIds: string[];
  verificationMethod?: string;
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
  escortPolicy?: string;
  visitorPolicy?: string;
  emergencyOverride: boolean;
  auditRequired: boolean;
  badgeRequired: boolean;
  biometricRequired: boolean;
  zoneIds: string[];
  exceptions: string[];
  regulatoryBasis: string[];
  enforcementMethod?: string;
  revocationPolicy?: string;
  trainingRequired: boolean;
  ownerId?: string;
}

export interface OperationalRequirement extends EntityHeader {
  operation: string;
  serviceLevel?: string;
  operatingHours?: string;
  staffing: QuantitySpec;
  maintenanceInterval?: string;
  cleaningRegime?: string;
  turnaroundTime?: string;
  redundancy?: string;
  uptimeTarget?: number;
  responseTime?: string;
  equipmentIds: string[];
  elementIds: string[];
  processIds: string[];
  utilities: string[];
  wasteStreams: string[];
  contingencyPlan: string[];
  trainingRequirements: string[];
  sopReferences: string[];
  kpiTargets: string[];
  ownerId?: string;
  serviceCategory?: string;
  shiftPattern?: string;
  slaTarget?: string;
  escalationContactId?: string;
}

export interface Equipment extends EntityHeader {
  code: string;
  category: string;
  manufacturer?: string;
  model?: string;
  quantity: QuantitySpec;
  dimensions?: string;
  weightKg?: number;
  powerKw?: number;
  utilityConnections: string[];
  ventilation?: string;
  noiseLevelDb?: number;
  clearance?: string;
  mounting?: string;
  elementIds: string[];
  activityIds: string[];
  maintenanceAccess: string[];
  lifecycleYears?: number;
  replacementCost?: number;
  standards: string[];
  supplier?: string;
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
  mobility?: string;
  sharingModel?: string;
  allocation?: string;
  elementIds: string[];
  activityIds: string[];
  userProfileIds: string[];
  storageRequirementId?: string;
  durability?: string;
  cleaningRequirements: string[];
  replacementCycle?: string;
  costPerUnit?: number;
  supplier?: string;
  standards: string[];
  ergonomicNotes: string[];
  customization: string[];
  disposalNotes: string[];
  furnitureClass?: string;
  ergonomicsRating?: string;
  sharingRatio?: number;
}

export type StorageClass = "general" | "secure" | "climateControlled" | "hazardous" | "archive" | "mobile" | "fixed" | "shared" | "coldChain" | "flammable";

export interface StorageRequirement extends EntityHeader {
  storedItem: string;
  storageClass: StorageClass;
  quantity: QuantitySpec;
  volumeM3?: number;
  weightKg?: number;
  temperatureRange?: string;
  humidityRange?: string;
  securityLevel: AccessLevel;
  hazardClass?: string;
  retentionPeriod?: string;
  accessFrequency?: string;
  elementIds: string[];
  equipmentIds: string[];
  handlingEquipment: string[];
  fireProtection: string[];
  ventilation?: string;
  organizationSystem?: string;
  growthAllowance?: number;
  regulatoryRefs: string[];
  ownerId?: string;
}

export type EnvironmentalParameter = "temperature" | "humidity" | "airQuality" | "lighting" | "acoustics" | "ventilation" | "radiation" | "vibration" | "pressure" | "iaq";

export interface EnvironmentalRequirement extends EntityHeader {
  parameterKind: EnvironmentalParameter;
  parameter: string;
  targetValue?: number;
  unit?: string;
  minValue?: number;
  maxValue?: number;
  comfortBand?: string;
  measurementMethod?: string;
  monitoringFrequency?: string;
  elementIds: string[];
  occupancyBasis?: string;
  seasonalVariation: string[];
  energyImplications: string[];
  standards: string[];
  certificationTargets: string[];
  outdoorConditions: string[];
  ventilationStrategy?: string;
  daylightTarget?: string;
  acousticTarget?: string;
  iaqTarget?: string;
  verificationPlan?: string;
}

export type HumanFactorAspect = "ergonomics" | "cognition" | "sensory" | "social" | "cultural" | "behavioral" | "physical" | "psychological" | "fatigue" | "stress";

export interface HumanFactorRequirement extends EntityHeader {
  aspect: HumanFactorAspect;
  factor: string;
  userProfileIds: string[];
  activityIds: string[];
  ergonomicCriteria: string[];
  cognitiveLoad?: string;
  visualDemands: string[];
  auditoryDemands: string[];
  postureRequirements: string[];
  reachEnvelope?: string;
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
  verificationMethod?: string;
}

export interface AccessibilityRequirement extends EntityHeader {
  standard: string;
  level?: string;
  userProfileIds: string[];
  elementIds: string[];
  routeIds: string[];
  clearWidthM?: number;
  clearHeightM?: number;
  turningCircleM?: number;
  rampSlope?: number;
  liftRequired: boolean;
  tactileGuidance: boolean;
  hearingLoop: boolean;
  visualContrast: boolean;
  signageRequirements: string[];
  controlsHeight?: string;
  emergencyEvacuation: string[];
  serviceAnimalPolicy?: string;
  companionSeating: boolean;
  verificationPlan?: string;
  exceptions: string[];
  wcagConformance?: string;
  universalDesignPrinciples: string[];
}

export type PrivacyKind = "public" | "semiPublic" | "semiPrivate" | "private" | "confidential" | "restricted" | "anonymous";

export interface PrivacyRequirement extends EntityHeader {
  privacyKind: PrivacyKind;
  privacyType: string;
  level?: string;
  subjectIds: string[];
  elementIds: string[];
  visualPrivacy: string[];
  acousticPrivacy: string[];
  dataPrivacy: string[];
  screeningRequired: boolean;
  enclosureRequired: boolean;
  accessRestrictions: string[];
  observationRisk?: string;
  regulatoryBasis: string[];
  culturalConsiderations: string[];
  technologyControls: string[];
  signage: string[];
  monitoringRestrictions: string[];
  retentionPolicy?: string;
  breachResponse: string[];
  ownerId?: string;
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
  inspectionFrequency?: string;
  trainingRequirements: string[];
  incidentReporting: string[];
  residualRisk?: string;
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
  drillFrequency?: string;
  liaisonContacts: string[];
  classifiedLevel?: string;
  redundancy: string[];
  auditRequirements: string[];
}

export interface RegulatoryRequirement extends EntityHeader {
  jurisdiction: string;
  code: string;
  clause?: string;
  title: string;
  requirementText: TextField;
  applicability: string[];
  elementIds: string[];
  complianceMethod?: string;
  evidenceRequired: string[];
  authority?: string;
  effectiveDate?: string;
  expiryDate?: string;
  penalties: string[];
  exemptions: string[];
  relatedRequirementIds: string[];
  interpretationNotes: TaggedNote[];
  verificationStatus: ValidationStatus;
  consultantRefs: string[];
  updateSource?: string;
}

export interface SiteContext extends EntityHeader {
  siteName: string;
  address?: string;
  latitude?: number;
  longitude?: number;
  elevationM?: number;
  climateZone?: string;
  seismicZone?: string;
  floodRisk?: string;
  soilConditions: string[];
  utilitiesAvailable: string[];
  accessRoads: string[];
  publicTransit: string[];
  neighbors: string[];
  views: string[];
  noiseSources: string[];
  environmentalConstraints: string[];
  heritageConstraints: string[];
  zoning?: string;
  maxHeightM?: number;
  maxCoverage?: number;
}

export interface OrganizationalRequirement extends EntityHeader {
  department: string;
  reportingLine?: string;
  headcount: QuantitySpec;
  growthPlanId?: string;
  workPatterns: string[];
  collaborationModel?: string;
  hierarchyLevels: string[];
  decisionMaking: string[];
  cultureNotes: string[];
  changeReadiness?: string;
  unionConsiderations: string[];
  trainingNeeds: string[];
  elementIds: string[];
  stakeholderIds: string[];
  serviceRequirementIds: string[];
  brandingRequirements: string[];
  wellnessPlugins: string[];
  diversityGoals: string[];
  ownerId?: string;
}

export interface ServiceRequirement extends EntityHeader {
  serviceName: string;
  serviceType: string;
  provider?: string;
  serviceLevel?: string;
  operatingHours?: string;
  capacity: QuantitySpec;
  responseTime?: string;
  queueManagement: string[];
  customerProfiles: string[];
  elementIds: string[];
  equipmentIds: string[];
  staffing: QuantitySpec;
  qualityMetrics: string[];
  costModel?: string;
  contractRefs: string[];
  dependencies: string[];
  failureImpact?: string;
  backupService: string[];
  feedbackChannels: string[];
}

export interface InfrastructureRequirement extends EntityHeader {
  system: string;
  category: string;
  capacity: QuantitySpec;
  redundancy?: string;
  distribution: string[];
  entryPoints: string[];
  utilitySource?: string;
  standbyPower: boolean;
  monitoring: string[];
  maintenanceAccess: string[];
  standards: string[];
  elementIds: string[];
  peakDemand?: number;
  diversityFactor?: number;
  futureExpansion: string[];
  interfaceRequirements: string[];
  commissioning: string[];
  lifecycleCost?: number;
  ownerId?: string;
}

export interface InformationRequirement extends EntityHeader {
  informationType: string;
  format?: string;
  sourceSystem?: string;
  destinationSystems: string[];
  updateFrequency?: string;
  retentionPeriod?: string;
  accessControls: string[];
  classification?: string;
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
  ownerId?: string;
}

export interface CommunicationRequirement extends EntityHeader {
  channel: string;
  audienceIds: string[];
  messageTypes: string[];
  frequency?: string;
  medium: string[];
  language: string[];
  accessibility: string[];
  emergencyUse: boolean;
  twoWay: boolean;
  recordingPolicy?: string;
  signageLocations: string[];
  technology: string[];
  escalationPath: string[];
  feedbackLoop: boolean;
  privacyControls: string[];
  elementIds: string[];
  standards: string[];
  ownerId?: string;
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
  maximumSignageDistanceM?: number;
  lightingRequirements: string[];
  maintenancePlan?: string;
  emergencyEgress: string[];
  visitorJourney: string[];
  staffJourney: string[];
  brandIntegration: string[];
}

export type DeliveryPhase = "concept" | "schematic" | "designDevelopment" | "constructionDocuments" | "procurement" | "construction" | "commissioning" | "occupancy";

export interface ScheduleRequirement extends EntityHeader {
  milestone: string;
  phase: DeliveryPhase;
  startDate?: string;
  endDate?: string;
  duration?: string;
  dependencies: string[];
  predecessors: string[];
  successors: string[];
  critical: boolean;
  floatDays?: number;
  resourceRequirements: string[];
  occupancyImpact: string[];
  phasingStrategy?: string;
  decantRequirements: string[];
  commissioningWindow?: string;
  stakeholderIds: string[];
  riskIds: string[];
  contingencyDays?: number;
  reportingCadence?: string;
  ownerId?: string;
}

export interface FlexibilityRequirement extends EntityHeader {
  flexibilityType: string;
  elementIds: string[];
  adaptationScenarios: string[];
  modularityLevel?: string;
  reconfigurationTime?: string;
  costOfChange?: number;
  technologyReadiness?: string;
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
  ownerId?: string;
}

export interface GrowthPlan extends EntityHeader {
  horizonYears: number;
  growthRate?: number;
  headcountGrowth: QuantitySpec;
  areaGrowth: QuantitySpec;
  phases: string[];
  triggerEvents: string[];
  expansionElementIds: string[];
  reserveAreas: string[];
  infrastructureHeadroom: string[];
  budgetEnvelope?: number;
  fundingSources: string[];
  riskFactors: string[];
  decisionPoints: string[];
  scenarioIds: string[];
  decommissionPlan: string[];
  relocationStrategy: string[];
  stakeholderImpact: string[];
  regulatoryConsiderations: string[];
  ownerId?: string;
}

export interface SustainabilityRequirement extends EntityHeader {
  topic: string;
  target?: string;
  metric?: string;
  baseline?: number;
  targetValue?: number;
  unit?: string;
  certification: string[];
  standards: string[];
  elementIds: string[];
  strategies: string[];
  materialsPreferences: string[];
  energyStrategy: string[];
  waterStrategy: string[];
  wasteStrategy: string[];
  biodiversity: string[];
  embodiedCarbon?: number;
  operationalCarbon?: number;
  reportingRequirements: string[];
  verificationPlan?: string;
  ownerId?: string;
}

export interface ResilienceRequirement extends EntityHeader {
  hazard: string;
  riskLevel: RiskLevel;
  scenario?: string;
  recoveryTime?: string;
  recoveryPoint?: string;
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
  ownerId?: string;
  verificationPlan?: string;
}

export type CostBasis = "capital" | "operational" | "lifecycle" | "replacement" | "maintenance";

export interface CostRequirement extends EntityHeader {
  costItem: string;
  basis: CostBasis;
  amount?: number;
  currency: string;
  quantityBasis?: string;
  unitCost?: number;
  contingencyPercent?: number;
  escalationRate?: number;
  fundingSource?: string;
  elementIds: string[];
  requirementIds: string[];
  phase?: DeliveryPhase;
  cashFlowProfile: string[];
  valueEngineeringNotes: string[];
  benchmarkRef?: string;
  approvalStatus: ValidationStatus;
  ownerId?: string;
  assumptions: string[];
  sensitivityFactors: string[];
}

export interface DeliveryConstraint extends EntityHeader {
  constraintType: string;
  constraintDetails: TextField;
  phase: DeliveryPhase;
  hardDeadline?: string;
  softDeadline?: string;
  impactedElementIds: string[];
  impactedRequirementIds: string[];
  workHours?: string;
  noiseRestrictions: string[];
  accessRestrictions: string[];
  siteLogistics: string[];
  procurementLeadTime?: string;
  approvalGates: string[];
  occupancyConstraints: string[];
  weatherWindows: string[];
  penaltyClauses: string[];
  mitigationOptions: string[];
  ownerId?: string;
  riskIds: string[];
  constraintStatus: LifecycleStatus;
}

export interface Risk extends EntityHeader {
  riskStatement: TextField;
  category: string;
  probability: RiskLevel;
  impact: RiskLevel;
  riskScore?: number;
  causes: string[];
  effects: string[];
  affectedElementIds: string[];
  affectedRequirementIds: string[];
  mitigation: string[];
  contingency: string[];
  ownerId?: string;
  reviewDate?: string;
  triggerIndicators: string[];
  residualProbability?: RiskLevel;
  residualImpact?: RiskLevel;
  relatedConflictIds: string[];
  escalationPath: string[];
  monitoringPlan?: string;
}

export type ConflictKind = "adjacency" | "capacity" | "schedule" | "budget" | "regulatory" | "operational" | "environmental" | "security" | "priority";

export type IssueSeverity = "cosmetic" | "minor" | "major" | "critical" | "blocker";

export interface Conflict extends EntityHeader {
  kind: ConflictKind;
  summary: TextField;
  entityAId: string;
  entityBId: string;
  severity: IssueSeverity;
  detectedBy?: string;
  detectionDate?: string;
  tradeOffOptions: string[];
  recommendedResolution?: TextField;
  decisionId?: string;
  stakeholderIds: string[];
  requirementIds: string[];
  costImpact?: number;
  scheduleImpact?: string;
  qualityImpact: string[];
  resolutionStatus: ValidationStatus;
  ownerId?: string;
  escalationLevel?: string;
  relatedRiskIds: string[];
}

export type RequirementKind = "functional" | "spatial" | "performance" | "regulatory" | "operational" | "technical" | "aesthetic" | "sustainability";

export interface Requirement extends EntityHeader {
  code: string;
  kind: RequirementKind;
  statement: TextField;
  rationale?: TextField;
  source?: string;
  stakeholderIds: string[];
  elementIds: string[];
  functionIds: string[];
  parentRequirementId?: string;
  childRequirementIds: string[];
  acceptanceCriteria: string[];
  verificationMethod?: string;
  validationStatus: ValidationStatus;
  conflictIds: string[];
  riskIds: string[];
  costEstimate?: number;
  scheduleConstraint?: string;
  regulatoryRefs: string[];
  traceLinks: TraceLink[];
  supersededBy?: string;
}

export interface PriorityRecord extends EntityHeader {
  subjectId: string;
  subjectKind: string;
  rankedPriority: Priority;
  rank?: number;
  weight?: number;
  rationale?: TextField;
  decisionId?: string;
  stakeholderIds: string[];
  effectiveFrom?: string;
  effectiveUntil?: string;
  reviewCycle?: string;
  dependencies: string[];
  conflicts: string[];
  scoringMethod?: string;
  score?: number;
  criteria: string[];
  approvedBy?: string;
  approvalDate?: string;
  rankingNotes: TaggedNote[];
}

export interface Scenario extends EntityHeader {
  code: string;
  hypothesis: TextField;
  assumptions: string[];
  variables: string[];
  elementIds: string[];
  requirementIds: string[];
  growthPlanId?: string;
  probability?: number;
  impactSummary?: TextField;
  costDelta?: number;
  areaDelta?: number;
  headcountDelta?: number;
  scheduleDelta?: string;
  riskIds: string[];
  optionIds: string[];
  baseline: boolean;
  preferred: boolean;
  analysisIds: string[];
  ownerId?: string;
}

export interface OptionEvaluation extends EntityHeader {
  optionName: string;
  optionDescription: TextField;
  scenarioId?: string;
  criteriaIds: string[];
  scores: number[];
  weightedScore?: number;
  costEstimate?: number;
  scheduleEstimate?: string;
  riskSummary: string[];
  benefits: string[];
  drawbacks: string[];
  assumptions: string[];
  dependencies: string[];
  stakeholderFeedback: TaggedNote[];
  recommendation?: string;
  decisionId?: string;
  evaluationStatus: ValidationStatus;
  evaluatorIds: string[];
  evaluationDate?: string;
}

export interface Decision extends EntityHeader {
  decisionStatement: TextField;
  context: TextField;
  optionsConsidered: string[];
  selectedOptionId?: string;
  rationale: TextField;
  decisionMakerIds: string[];
  consultedIds: string[];
  informedIds: string[];
  decisionDate?: string;
  effectiveDate?: string;
  reversalConditions: string[];
  impactedRequirementIds: string[];
  impactedElementIds: string[];
  costImpact?: number;
  scheduleImpact?: string;
  riskImpact: string[];
  approvalStatus: ValidationStatus;
  meetingRef?: string;
  artifactRefs: string[];
}

export interface ValidationRecord extends EntityHeader {
  subjectId: string;
  subjectKind: string;
  validationType: string;
  method?: string;
  criteria: string[];
  result: ValidationStatus;
  evidence: string[];
  validatorIds: string[];
  validationDate?: string;
  nextReviewDate?: string;
  findings: string[];
  nonConformities: string[];
  correctiveActions: string[];
  waivers: string[];
  standards: string[];
  traceLinks: TraceLink[];
  reportId?: string;
  confidenceLevel?: string;
  validationNotes: TaggedNote[];
}

export interface PerformanceCriterion extends EntityHeader {
  criterion: string;
  metric: string;
  target?: number;
  unit?: string;
  minimum?: number;
  maximum?: number;
  measurementMethod?: string;
  frequency?: string;
  requirementIds: string[];
  elementIds: string[];
  baseline?: number;
  benchmarkRef?: string;
  weight?: number;
  dataSource?: string;
  reportingCadence?: string;
  ownerId?: string;
  verificationPlan?: string;
  penaltyThreshold?: number;
  incentiveThreshold?: number;
}

export interface QualityRecord extends EntityHeader {
  qualityTopic: string;
  standard?: string;
  targetLevel?: string;
  inspectionPoints: string[];
  acceptanceCriteria: string[];
  testingRequirements: string[];
  sampleRate?: string;
  defectCategories: string[];
  correctiveActionProcess: string[];
  elementIds: string[];
  requirementIds: string[];
  supplierRequirements: string[];
  documentationRequirements: string[];
  trainingRequirements: string[];
  auditSchedule?: string;
  kpis: string[];
  ownerId?: string;
  certificationTargets: string[];
  continuousImprovement: string[];
}

export interface DocumentRecord extends EntityHeader {
  documentType: string;
  title: string;
  version: string;
  fileRef?: string;
  format?: string;
  authorIds: string[];
  reviewerIds: string[];
  approverIds: string[];
  issueDate?: string;
  revisionDate?: string;
  distributionList: string[];
  relatedEntityIds: string[];
  classification?: string;
  retentionPeriod?: string;
  accessControls: string[];
  supersedes?: string;
  documentStatus: LifecycleStatus;
  checksum?: string;
  sourceSystem?: string;
}

export interface Assumption extends EntityHeader {
  statement: TextField;
  basis?: TextField;
  confidenceLevel?: string;
  impactIfFalse?: TextField;
  relatedEntityIds: string[];
  validationStatus: ValidationStatus;
  validatedBy?: string;
  validationDate?: string;
  ownerId?: string;
  reviewCycle?: string;
  source?: string;
  category?: string;
  dependencies: string[];
  mitigation: string[];
  linkedRequirementIds: string[];
  linkedRiskIds: string[];
  expirationDate?: string;
  statusNotes: TaggedNote[];
  artifactRefs: string[];
}

export interface ConstraintRecord extends EntityHeader {
  constraintType: string;
  summary: TextField;
  severity: RiskLevel;
  affectedEntityIds: string[];
  source?: string;
  regulatoryBasis: string[];
  mitigationOptions: string[];
  ownerId?: string;
  effectiveDate?: string;
  expiryDate?: string;
  waiverStatus?: string;
  waiverApprover?: string;
  impactAssessment?: TextField;
  resolutionPlan: string[];
  relatedRequirementIds: string[];
  relatedDecisionIds: string[];
  monitoringFrequency?: string;
  complianceStatus: ValidationStatus;
  exceptions: string[];
  traceLinks: TraceLink[];
  escalationContactId?: string;
}

export interface ComplianceRecord extends EntityHeader {
  standardRef: string;
  obligation: TextField;
  complianceStatus: ValidationStatus;
  evidenceRefs: string[];
  auditorId?: string;
  auditDate?: string;
  nextReview?: string;
  affectedEntityIds: string[];
  gapAnalysis: string[];
  remediationPlan: string[];
  ownerId?: string;
  severity: RiskLevel;
  regulatoryBody?: string;
  certificationTarget?: string;
  waiverStatus?: string;
  relatedRequirementIds: string[];
  monitoringMethod?: string;
  reportingFrequency?: string;
  penalties: string[];
  correctiveActions: string[];
  artifactRefs: string[];
}

export interface ApprovalRecord extends EntityHeader {
  approvalType: string;
  subjectId: string;
  approverIds: string[];
  approvalDate?: string;
  conditions: string[];
  approvalStatus: LifecycleStatus;
  expiryDate?: string;
  delegationChain: string[];
  evidenceRefs: string[];
  relatedDecisionId?: string;
  relatedChangeId?: string;
  authorityBasis: string[];
  signatureMethod?: string;
  rejectionReason?: TextField;
  resubmissionDate?: string;
  notificationList: string[];
  workflowStep?: string;
  version?: string;
  auditTrailRef?: string;
}

export interface MeetingRecord extends EntityHeader {
  meetingType: string;
  scheduledDate?: string;
  duration?: string;
  location?: string;
  chairId?: string;
  attendeeIds: string[];
  agendaItems: string[];
  minutes?: TextField;
  actionItems: string[];
  decisionsMade: string[];
  artifactRefs: string[];
  followUpDate?: string;
  recordingRef?: string;
  quorumMet: boolean;
  meetingStatus: LifecycleStatus;
  workshopId?: string;
  stakeholderIds: string[];
  requirementIds: string[];
  issueIds: string[];
  approvalIds: string[];
}

export interface ChangeRecord extends EntityHeader {
  changeType: string;
  summary: TextField;
  reason: TextField;
  requestedBy?: string;
  approvedBy?: string;
  changeDate?: string;
  effectiveDate?: string;
  impactedEntityIds: string[];
  beforeSnapshot?: string;
  afterSnapshot?: string;
  costImpact?: number;
  scheduleImpact?: string;
  riskImpact: string[];
  approvalStatus: ValidationStatus;
  rollbackPlan: string[];
  communicationPlan: string[];
  versionFrom?: string;
  versionTo?: string;
  auditEventIds: string[];
}

export interface CollaborationRecord extends EntityHeader {
  sessionType: string;
  title: string;
  participants: string[];
  facilitatorId?: string;
  startTime?: string;
  endTime?: string;
  location?: string;
  agenda: string[];
  outcomes: string[];
  actionItems: string[];
  decisionIds: string[];
  issueIds: string[];
  documentIds: string[];
  recordingRef?: string;
  feedback: TaggedNote[];
  followUpDate?: string;
  workshopId?: string;
  surveyId?: string;
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
  runBy?: string;
  runAt?: string;
  durationMs?: number;
  toolVersion?: string;
  scenarioId?: string;
  reportId?: string;
  confidence?: string;
  limitations: string[];
  recommendations: string[];
  rawResultRef?: string;
}

export type ReportKind = "executiveSummary" | "programOverview" | "stakeholderSummary" | "requirementsMatrix" | "adjacencyMatrix" | "gapAnalysis" | "riskRegister" | "decisionLog" | "validationSummary" | "recommendation" | "userSummary" | "functionalSummary" | "capacitySummary" | "workflowSummary" | "complianceSummary" | "costSummary" | "scheduleSummary" | "changeSummary" | "openIssueSummary" | "prioritySummary" | "scenarioSummary";

export interface ReportRecord extends EntityHeader {
  kind: ReportKind;
  title: string;
  audience: string[];
  sections: string[];
  generatedAt?: string;
  generatedBy?: string;
  analysisIds: string[];
  format?: string;
  fileRef?: string;
  distributionList: string[];
  approvalStatus: ValidationStatus;
  approverId?: string;
  version: string;
  templateId?: string;
  parameters: string[];
  confidentiality?: string;
  expiryDate?: string;
  relatedDecisionIds: string[];
}

export interface SearchFilter extends EntityHeader {
  filterName: string;
  filterDescription?: TextField;
  keywords: string[];
  categories: string[];
  ownerIds: string[];
  statuses: LifecycleStatus[];
  priorities: Priority[];
  sources: string[];
  dateFrom?: string;
  dateTo?: string;
  entityKinds: string[];
  tagFilters: string[];
  sortField?: string;
  sortDirection?: string;
  isPublic: boolean;
  createdBy?: string;
  lastUsed?: string;
  useCount: number;
  pinned: boolean;
}

export interface StatusRecord extends EntityHeader {
  subjectId: string;
  subjectKind: string;
  recordStatus: LifecycleStatus;
  previousStatus?: LifecycleStatus;
  changedBy?: string;
  changedAt?: string;
  reason?: TextField;
  blockers: string[];
  nextActions: string[];
  dueDate?: string;
  progressPercent?: number;
  health?: string;
  escalationLevel?: string;
  relatedIssueIds: string[];
  relatedRiskIds: string[];
  milestoneId?: string;
  reportingPeriod?: string;
  statusNotes: TaggedNote[];
}

export interface Workshop extends EntityHeader {
  workshopType: string;
  objectives: string[];
  agenda: string[];
  facilitatorId?: string;
  participants: string[];
  scheduledStart?: string;
  scheduledEnd?: string;
  location?: string;
  materials: string[];
  methods: string[];
  outputs: string[];
  decisions: string[];
  issues: string[];
  followUpActions: string[];
  feedback: TaggedNote[];
  recordingRef?: string;
  budget?: number;
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
  launchDate?: string;
  closeDate?: string;
  responseCount: number;
  responseRate?: number;
  findings: string[];
  themes: string[];
  recommendations: string[];
  confidentiality?: string;
  consentProcess: string[];
  analysisId?: string;
  workshopId?: string;
  ownerId?: string;
  surveyStatus: LifecycleStatus;
}

export interface Issue extends EntityHeader {
  issueType: string;
  summary: TextField;
  issueDescription: TextField;
  severity: IssueSeverity;
  issuePriority: Priority;
  reporterId?: string;
  assigneeId?: string;
  affectedEntityIds: string[];
  rootCause?: TextField;
  resolution?: TextField;
  workaround?: TextField;
  dueDate?: string;
  resolvedDate?: string;
  relatedConflictIds: string[];
  relatedRiskIds: string[];
  decisionId?: string;
  comments: TaggedNote[];
  attachments: string[];
  escalationLevel?: string;
}

export type AuditAction = "created" | "updated" | "deleted" | "reviewed" | "approved" | "rejected" | "exported" | "imported" | "merged" | "archived";

export interface AuditEvent extends EntityHeader {
  action: AuditAction;
  actorId?: string;
  subjectId: string;
  subjectKind: string;
  timestamp: string;
  details: TextField;
  beforeState?: string;
  afterState?: string;
  ipAddress?: string;
  client?: string;
  sessionId?: string;
  changeRecordId?: string;
  traceLink?: TraceLink;
  success: boolean;
  errorMessage?: string;
  correlationId?: string;
  complianceTags: string[];
  retentionUntil?: string;
}

export interface TemplateRecord extends EntityHeader {
  templateType: string;
  sector?: string;
  projectType?: string;
  version: string;
  contentRef?: string;
  entityKinds: string[];
  defaultFields: string[];
  checklists: string[];
  standards: string[];
  applicability: string[];
  authorId?: string;
  approvalStatus: ValidationStatus;
  usageCount: number;
  lastApplied?: string;
  customizationNotes: string[];
  relatedKnowledgeIds: string[];
  benchmarkIds: string[];
  license?: string;
  sourceOrganization?: string;
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
  expertiseLevel?: string;
  validationStatus: ValidationStatus;
  lastReviewed?: string;
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
  sampleSize?: number;
  source?: string;
  collectionYear?: number;
  geography?: string;
  buildingType?: string;
  confidence?: string;
  methodology?: string;
  applicableElementKinds: string[];
  relatedRequirementIds: string[];
  comparisonNotes: string[];
  limitations: string[];
  license?: string;
  knowledgeId?: string;
  lastVerified?: string;
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
  riskAppetite?: string;
  complianceObligations: string[];
  auditSchedule?: string;
  documentControl: string[];
  stakeholderEngagementPlan: string[];
  ethicsPolicy: string[];
  dataGovernance: string[];
  ownerId?: string;
  reviewCycle?: string;
  reviewHierarchy: string[];
  policyOwnershipId?: string;
  requirementOwnershipId?: string;
  riskOwnershipId?: string;
  reportingFrequency?: string;
  accountabilityRules: string[];
  exceptionManagement: string[];
  governancePerformance: string[];
}
//#endregion 🔖️Entities

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
export interface CreateFunction { function: Function; }
export interface CreateFunctionOp extends CreateFunction { mutation: "createFunction"; }
export interface DeleteFunction { id: string; }
export interface DeleteFunctionOp extends DeleteFunction { mutation: "deleteFunction"; }
export interface RenameFunction { id: string; newName: string; }
export interface RenameFunctionOp extends RenameFunction { mutation: "renameFunction"; }
export interface ReplaceFunction { function: Function; }
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
