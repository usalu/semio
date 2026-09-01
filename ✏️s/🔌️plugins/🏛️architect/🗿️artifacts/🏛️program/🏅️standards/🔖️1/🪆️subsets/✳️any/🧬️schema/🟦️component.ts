/** 🧬️ ProgramSnapshot artifact schema — every field with its state class. */

//#region 🔖️Entities
/** 🧬️ Register entity types, mirrored field-for-field from `🗄️registers/🦀️component.rs` and
 * `🧱️kernel/🦀️component.rs` (`EntityHeader` etc, `#[serde(flatten)]`d onto every register row —
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
  documents: DocumentRecord[];
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
  knowledge: KnowledgeRecord[];
  /** @state artifact */
  benchmarks: BenchmarkRecord[];
  /** @state artifact */
  traces: TraceLink[];
  /** @state artifact */
  governance: Governance;
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  activeRegister: string;
  /** @state presence */
  adjacencyKindFilter?: AdjacencyKind;
  /** @state presence */
  activeReportJson: string;
  /** @state config */
  searchQuery: string;
  /** @state config */
  searchHistoryJson: string;
  /** @state config */
  lastResultJson: string;
  /** @state config */
  lastAnalysisJson: string;
  /** @state config */
  graphCameraX: number;
  /** @state config */
  graphCameraY: number;
  /** @state config */
  graphCameraZoom: number;
}

