/** 🏗️ ProgramSnapshot artifact schema: the stakeholder, user, activity, function, element, quantity, relationship, adjacency, process, flow and access-rule entities. */
import { type Binary64 } from "@semio-tech/framework/value/ieee754";
import type { AccessLevel, EntityHeader, Priority, QuantitySpec, SeparationKind, TaggedNote, TextField, TraceLink, ValidationStatus } from "../🪪️entity-foundation/🟦️.ts";

//#region 🔖️Entities
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
  tolerancePercent: Binary64 | null;
  peakFactor: Binary64 | null;
  growthFactor: Binary64 | null;
  unitCost: Binary64 | null;
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

export interface Relationship extends EntityHeader {
  sourceId: string;
  targetId: string;
  kind: RelationshipKind;
  strength: Binary64 | null;
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
  distanceConstraintM: Binary64 | null;
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

export interface Adjacency extends EntityHeader {
  elementAId: string;
  elementBId: string;
  kind: AdjacencyKind;
  connection: ConnectionKind;
  separations: SeparationKind[];
  weight: Binary64;
  rationale: TextField | null;
  distanceMaxM: Binary64 | null;
  distanceMinM: Binary64 | null;
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

export interface FlowRequirement extends EntityHeader {
  fromElementId: string;
  toElementId: string;
  kind: FlowKind;
  flowType: string;
  direction: FlowDirection;
  volume: QuantitySpec;
  peakRate: Binary64 | null;
  clearWidthM: Binary64 | null;
  clearHeightM: Binary64 | null;
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
//#endregion 🔖️Entities
