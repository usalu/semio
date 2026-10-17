/** 📋️ ProgramSnapshot artifact schema: the operational, resource, human-factor, safety, security, site, service, schedule, sustainability, cost and delivery requirement entities. */
import { type Binary64 } from "@semio-tech/framework/value/ieee754";
import type { AccessLevel, DeliveryPhase, EntityHeader, LifecycleStatus, QuantitySpec, RiskLevel, TaggedNote, TextField, ValidationStatus } from "../🪪️entity-foundation/🟦️.ts";

//#region 🔖️Entities
export interface OperationalRequirement extends EntityHeader {
  operation: string;
  serviceLevel: string | null;
  operatingHours: string | null;
  staffing: QuantitySpec;
  maintenanceInterval: string | null;
  cleaningRegime: string | null;
  turnaroundTime: string | null;
  redundancy: string | null;
  uptimeTarget: Binary64 | null;
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
  weightKg: Binary64 | null;
  powerKw: Binary64 | null;
  utilityConnections: string[];
  ventilation: string | null;
  noiseLevelDb: Binary64 | null;
  clearance: string | null;
  mounting: string | null;
  elementIds: string[];
  activityIds: string[];
  maintenanceAccess: string[];
  lifecycleYears: number | null;
  replacementCost: Binary64 | null;
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
  costPerUnit: Binary64 | null;
  supplier: string | null;
  standards: string[];
  ergonomicNotes: string[];
  customization: string[];
  disposalNotes: string[];
  furnitureClass: string | null;
  ergonomicsRating: string | null;
  sharingRatio: Binary64 | null;
}

export type StorageClass = "general" | "secure" | "climateControlled" | "hazardous" | "archive" | "mobile" | "fixed" | "shared" | "coldChain" | "flammable";

export interface StorageRequirement extends EntityHeader {
  storedItem: string;
  storageClass: StorageClass;
  quantity: QuantitySpec;
  volumeM3: Binary64 | null;
  weightKg: Binary64 | null;
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
  growthAllowance: Binary64 | null;
  regulatoryRefs: string[];
  ownerId: string | null;
}

export type EnvironmentalParameter = "temperature" | "humidity" | "airQuality" | "lighting" | "acoustics" | "ventilation" | "radiation" | "vibration" | "pressure" | "iaq";

export interface EnvironmentalRequirement extends EntityHeader {
  parameterKind: EnvironmentalParameter;
  parameter: string;
  targetValue: Binary64 | null;
  unit: string | null;
  minValue: Binary64 | null;
  maxValue: Binary64 | null;
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
  clearWidthM: Binary64 | null;
  clearHeightM: Binary64 | null;
  turningCircleM: Binary64 | null;
  rampSlope: Binary64 | null;
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
  latitude: Binary64 | null;
  longitude: Binary64 | null;
  elevationM: Binary64 | null;
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
  maxHeightM: Binary64 | null;
  maxCoverage: Binary64 | null;
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
  peakDemand: Binary64 | null;
  diversityFactor: Binary64 | null;
  futureExpansion: string[];
  interfaceRequirements: string[];
  commissioning: string[];
  lifecycleCost: Binary64 | null;
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
  maximumSignageDistanceM: Binary64 | null;
  lightingRequirements: string[];
  maintenancePlan: string | null;
  emergencyEgress: string[];
  visitorJourney: string[];
  staffJourney: string[];
  brandIntegration: string[];
}

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
  costOfChange: Binary64 | null;
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
  growthRate: Binary64 | null;
  headcountGrowth: QuantitySpec;
  areaGrowth: QuantitySpec;
  phases: string[];
  triggerEvents: string[];
  expansionElementIds: string[];
  reserveAreas: string[];
  infrastructureHeadroom: string[];
  budgetEnvelope: Binary64 | null;
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
  baseline: Binary64 | null;
  targetValue: Binary64 | null;
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
  embodiedCarbon: Binary64 | null;
  operationalCarbon: Binary64 | null;
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
  amount: Binary64 | null;
  currency: string;
  quantityBasis: string | null;
  unitCost: Binary64 | null;
  contingencyPercent: Binary64 | null;
  escalationRate: Binary64 | null;
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
//#endregion 🔖️Entities
