/** 📸️ Program snapshot shares the exact artifact-lane document contract. */

import { parseProgramArtifact, type ProgramArtifact } from "../🟦️.ts";

export type ProgramSnapshot = ProgramArtifact;

export const parseProgramSnapshot = parseProgramArtifact;

//#region 🚪️Parsers
/** 🛑️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramSnapshotGuardReject = (at: string, why: string): never => {
  throw new architectProgramSnapshotGuardRefusal(at, why);
};

type architectProgramSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number; readonly exclusiveMinimum?: number; readonly exclusiveMaximum?: number };
type architectProgramSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectProgramSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectProgramSnapshotGuardReject(at, "value is not an object");
export const architectProgramSnapshotGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) architectProgramSnapshotGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const architectProgramSnapshotGuardExactObject = (value: unknown, at: string, fields: readonly string[], required: readonly string[] = fields): Readonly<Record<string, unknown>> => {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, fields);
  for (const field of required) if (!Object.hasOwn(row, field)) architectProgramSnapshotGuardReject(`${at}.${field}`, "field is missing");
  return row;
};
export const architectProgramSnapshotGuardArray = (value: unknown, at: string, bounds: architectProgramSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectProgramSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectProgramSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectProgramSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectProgramSnapshotGuardString = (value: unknown, at: string, bounds: architectProgramSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectProgramSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectProgramSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectProgramSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectProgramSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectProgramSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectProgramSnapshotGuardReject(at, "value is not a boolean"));
export const architectProgramSnapshotGuardNumber = (value: unknown, at: string, bounds: architectProgramSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectProgramSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectProgramSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectProgramSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  if (bounds.exclusiveMinimum !== undefined && value <= bounds.exclusiveMinimum) architectProgramSnapshotGuardReject(at, `number is not above ${bounds.exclusiveMinimum}`);
  if (bounds.exclusiveMaximum !== undefined && value >= bounds.exclusiveMaximum) architectProgramSnapshotGuardReject(at, `number is not below ${bounds.exclusiveMaximum}`);
  return value;
};
export const architectProgramSnapshotGuardInteger = (value: unknown, at: string, bounds: architectProgramSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramSnapshotGuardNumber(value, at, bounds) : architectProgramSnapshotGuardReject(at, "value is not an integer");
export const architectProgramSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export type AccessLevel = "public" | "restricted" | "controlled" | "private" | "secure" | "emergencyOnly";

export function parseAccessLevel(value: unknown, at = "$"): AccessLevel {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["public", "restricted", "controlled", "private", "secure", "emergencyOnly"] as const);
}

export type AccessMode = "unrestricted" | "cardControlled" | "biometric" | "keyed" | "escortRequired" | "timeRestricted" | "roleBased" | "emergencyOnly";

export function parseAccessMode(value: unknown, at = "$"): AccessMode {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["unrestricted", "cardControlled", "biometric", "keyed", "escortRequired", "timeRestricted", "roleBased", "emergencyOnly"] as const);
}

export interface AccessRule {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly subjectIds: readonly string[];
  readonly resourceIds: readonly string[];
  readonly accessLevel: AccessLevel;
  readonly accessMode: AccessMode;
  readonly authentication: readonly string[];
  readonly authorization: readonly string[];
  readonly timeRestrictions: readonly string[];
  readonly escortPolicy: string | null;
  readonly visitorPolicy: string | null;
  readonly emergencyOverride: boolean;
  readonly auditRequired: boolean;
  readonly badgeRequired: boolean;
  readonly biometricRequired: boolean;
  readonly zoneIds: readonly string[];
  readonly exceptions: readonly string[];
  readonly regulatoryBasis: readonly string[];
  readonly enforcementMethod: string | null;
  readonly revocationPolicy: string | null;
  readonly trainingRequired: boolean;
  readonly ownerId: string | null;
}

export function parseAccessRule(value: unknown, at = "$"): AccessRule {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "subjectIds", "resourceIds", "accessLevel", "accessMode", "authentication", "authorization", "timeRestrictions", "escortPolicy", "visitorPolicy", "emergencyOverride", "auditRequired", "badgeRequired", "biometricRequired", "zoneIds", "exceptions", "regulatoryBasis", "enforcementMethod", "revocationPolicy", "trainingRequired", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    subjectIds: architectProgramSnapshotGuardArray(row["subjectIds"], `${at}.subjectIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.subjectIds[${index}]`)),
    resourceIds: architectProgramSnapshotGuardArray(row["resourceIds"], `${at}.resourceIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.resourceIds[${index}]`)),
    accessLevel: parseAccessLevel(row["accessLevel"], `${at}.accessLevel`),
    accessMode: parseAccessMode(row["accessMode"], `${at}.accessMode`),
    authentication: architectProgramSnapshotGuardArray(row["authentication"], `${at}.authentication`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.authentication[${index}]`)),
    authorization: architectProgramSnapshotGuardArray(row["authorization"], `${at}.authorization`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.authorization[${index}]`)),
    timeRestrictions: architectProgramSnapshotGuardArray(row["timeRestrictions"], `${at}.timeRestrictions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.timeRestrictions[${index}]`)),
    escortPolicy: (row["escortPolicy"] === null ? null : architectProgramSnapshotGuardString(row["escortPolicy"], `${at}.escortPolicy`)),
    visitorPolicy: (row["visitorPolicy"] === null ? null : architectProgramSnapshotGuardString(row["visitorPolicy"], `${at}.visitorPolicy`)),
    emergencyOverride: architectProgramSnapshotGuardBoolean(row["emergencyOverride"], `${at}.emergencyOverride`),
    auditRequired: architectProgramSnapshotGuardBoolean(row["auditRequired"], `${at}.auditRequired`),
    badgeRequired: architectProgramSnapshotGuardBoolean(row["badgeRequired"], `${at}.badgeRequired`),
    biometricRequired: architectProgramSnapshotGuardBoolean(row["biometricRequired"], `${at}.biometricRequired`),
    zoneIds: architectProgramSnapshotGuardArray(row["zoneIds"], `${at}.zoneIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.zoneIds[${index}]`)),
    exceptions: architectProgramSnapshotGuardArray(row["exceptions"], `${at}.exceptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.exceptions[${index}]`)),
    regulatoryBasis: architectProgramSnapshotGuardArray(row["regulatoryBasis"], `${at}.regulatoryBasis`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryBasis[${index}]`)),
    enforcementMethod: (row["enforcementMethod"] === null ? null : architectProgramSnapshotGuardString(row["enforcementMethod"], `${at}.enforcementMethod`)),
    revocationPolicy: (row["revocationPolicy"] === null ? null : architectProgramSnapshotGuardString(row["revocationPolicy"], `${at}.revocationPolicy`)),
    trainingRequired: architectProgramSnapshotGuardBoolean(row["trainingRequired"], `${at}.trainingRequired`),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export interface AccessibilityRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly standard: string;
  readonly level: string | null;
  readonly userProfileIds: readonly string[];
  readonly elementIds: readonly string[];
  readonly routeIds: readonly string[];
  readonly clearWidthM: Binary64Transport | null;
  readonly clearHeightM: Binary64Transport | null;
  readonly turningCircleM: Binary64Transport | null;
  readonly rampSlope: Binary64Transport | null;
  readonly liftRequired: boolean;
  readonly tactileGuidance: boolean;
  readonly hearingLoop: boolean;
  readonly visualContrast: boolean;
  readonly signageRequirements: readonly string[];
  readonly controlsHeight: string | null;
  readonly emergencyEvacuation: readonly string[];
  readonly serviceAnimalPolicy: string | null;
  readonly companionSeating: boolean;
  readonly verificationPlan: string | null;
  readonly exceptions: readonly string[];
  readonly wcagConformance: string | null;
  readonly universalDesignPrinciples: readonly string[];
}

export function parseAccessibilityRequirement(value: unknown, at = "$"): AccessibilityRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "standard", "level", "userProfileIds", "elementIds", "routeIds", "clearWidthM", "clearHeightM", "turningCircleM", "rampSlope", "liftRequired", "tactileGuidance", "hearingLoop", "visualContrast", "signageRequirements", "controlsHeight", "emergencyEvacuation", "serviceAnimalPolicy", "companionSeating", "verificationPlan", "exceptions", "wcagConformance", "universalDesignPrinciples"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    standard: architectProgramSnapshotGuardString(row["standard"], `${at}.standard`),
    level: (row["level"] === null ? null : architectProgramSnapshotGuardString(row["level"], `${at}.level`)),
    userProfileIds: architectProgramSnapshotGuardArray(row["userProfileIds"], `${at}.userProfileIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.userProfileIds[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    routeIds: architectProgramSnapshotGuardArray(row["routeIds"], `${at}.routeIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.routeIds[${index}]`)),
    clearWidthM: (row["clearWidthM"] === null ? null : parseBinary64Transport(row["clearWidthM"], `${at}.clearWidthM`)),
    clearHeightM: (row["clearHeightM"] === null ? null : parseBinary64Transport(row["clearHeightM"], `${at}.clearHeightM`)),
    turningCircleM: (row["turningCircleM"] === null ? null : parseBinary64Transport(row["turningCircleM"], `${at}.turningCircleM`)),
    rampSlope: (row["rampSlope"] === null ? null : parseBinary64Transport(row["rampSlope"], `${at}.rampSlope`)),
    liftRequired: architectProgramSnapshotGuardBoolean(row["liftRequired"], `${at}.liftRequired`),
    tactileGuidance: architectProgramSnapshotGuardBoolean(row["tactileGuidance"], `${at}.tactileGuidance`),
    hearingLoop: architectProgramSnapshotGuardBoolean(row["hearingLoop"], `${at}.hearingLoop`),
    visualContrast: architectProgramSnapshotGuardBoolean(row["visualContrast"], `${at}.visualContrast`),
    signageRequirements: architectProgramSnapshotGuardArray(row["signageRequirements"], `${at}.signageRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.signageRequirements[${index}]`)),
    controlsHeight: (row["controlsHeight"] === null ? null : architectProgramSnapshotGuardString(row["controlsHeight"], `${at}.controlsHeight`)),
    emergencyEvacuation: architectProgramSnapshotGuardArray(row["emergencyEvacuation"], `${at}.emergencyEvacuation`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.emergencyEvacuation[${index}]`)),
    serviceAnimalPolicy: (row["serviceAnimalPolicy"] === null ? null : architectProgramSnapshotGuardString(row["serviceAnimalPolicy"], `${at}.serviceAnimalPolicy`)),
    companionSeating: architectProgramSnapshotGuardBoolean(row["companionSeating"], `${at}.companionSeating`),
    verificationPlan: (row["verificationPlan"] === null ? null : architectProgramSnapshotGuardString(row["verificationPlan"], `${at}.verificationPlan`)),
    exceptions: architectProgramSnapshotGuardArray(row["exceptions"], `${at}.exceptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.exceptions[${index}]`)),
    wcagConformance: (row["wcagConformance"] === null ? null : architectProgramSnapshotGuardString(row["wcagConformance"], `${at}.wcagConformance`)),
    universalDesignPrinciples: architectProgramSnapshotGuardArray(row["universalDesignPrinciples"], `${at}.universalDesignPrinciples`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.universalDesignPrinciples[${index}]`)),
  };
}

export interface Activity {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly code: string;
  readonly category: string;
  readonly frequency: string | null;
  readonly duration: string | null;
  readonly intensity: string | null;
  readonly participants: QuantitySpec;
  readonly equipmentIds: readonly string[];
  readonly spaceRequirements: readonly string[];
  readonly environmentalNeeds: readonly string[];
  readonly privacyNeeds: readonly string[];
  readonly accessibilityNeeds: readonly string[];
  readonly adjacentActivities: readonly string[];
  readonly sequencing: readonly string[];
  readonly peakPeriods: readonly string[];
  readonly workflowSteps: readonly string[];
  readonly inputs: readonly string[];
  readonly outputs: readonly string[];
  readonly userProfileIds: readonly string[];
  readonly functionIds: readonly string[];
  readonly performanceIndicators: readonly string[];
  readonly activityType: string;
  readonly locationContext: string | null;
  readonly temporalPattern: string | null;
  readonly supervisionLevel: string | null;
}

export function parseActivity(value: unknown, at = "$"): Activity {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "code", "category", "frequency", "duration", "intensity", "participants", "equipmentIds", "spaceRequirements", "environmentalNeeds", "privacyNeeds", "accessibilityNeeds", "adjacentActivities", "sequencing", "peakPeriods", "workflowSteps", "inputs", "outputs", "userProfileIds", "functionIds", "performanceIndicators", "activityType", "locationContext", "temporalPattern", "supervisionLevel"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    category: architectProgramSnapshotGuardString(row["category"], `${at}.category`),
    frequency: (row["frequency"] === null ? null : architectProgramSnapshotGuardString(row["frequency"], `${at}.frequency`)),
    duration: (row["duration"] === null ? null : architectProgramSnapshotGuardString(row["duration"], `${at}.duration`)),
    intensity: (row["intensity"] === null ? null : architectProgramSnapshotGuardString(row["intensity"], `${at}.intensity`)),
    participants: parseQuantitySpec(row["participants"], `${at}.participants`),
    equipmentIds: architectProgramSnapshotGuardArray(row["equipmentIds"], `${at}.equipmentIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.equipmentIds[${index}]`)),
    spaceRequirements: architectProgramSnapshotGuardArray(row["spaceRequirements"], `${at}.spaceRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.spaceRequirements[${index}]`)),
    environmentalNeeds: architectProgramSnapshotGuardArray(row["environmentalNeeds"], `${at}.environmentalNeeds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.environmentalNeeds[${index}]`)),
    privacyNeeds: architectProgramSnapshotGuardArray(row["privacyNeeds"], `${at}.privacyNeeds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.privacyNeeds[${index}]`)),
    accessibilityNeeds: architectProgramSnapshotGuardArray(row["accessibilityNeeds"], `${at}.accessibilityNeeds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.accessibilityNeeds[${index}]`)),
    adjacentActivities: architectProgramSnapshotGuardArray(row["adjacentActivities"], `${at}.adjacentActivities`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.adjacentActivities[${index}]`)),
    sequencing: architectProgramSnapshotGuardArray(row["sequencing"], `${at}.sequencing`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.sequencing[${index}]`)),
    peakPeriods: architectProgramSnapshotGuardArray(row["peakPeriods"], `${at}.peakPeriods`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.peakPeriods[${index}]`)),
    workflowSteps: architectProgramSnapshotGuardArray(row["workflowSteps"], `${at}.workflowSteps`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.workflowSteps[${index}]`)),
    inputs: architectProgramSnapshotGuardArray(row["inputs"], `${at}.inputs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.inputs[${index}]`)),
    outputs: architectProgramSnapshotGuardArray(row["outputs"], `${at}.outputs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.outputs[${index}]`)),
    userProfileIds: architectProgramSnapshotGuardArray(row["userProfileIds"], `${at}.userProfileIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.userProfileIds[${index}]`)),
    functionIds: architectProgramSnapshotGuardArray(row["functionIds"], `${at}.functionIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.functionIds[${index}]`)),
    performanceIndicators: architectProgramSnapshotGuardArray(row["performanceIndicators"], `${at}.performanceIndicators`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.performanceIndicators[${index}]`)),
    activityType: architectProgramSnapshotGuardString(row["activityType"], `${at}.activityType`),
    locationContext: (row["locationContext"] === null ? null : architectProgramSnapshotGuardString(row["locationContext"], `${at}.locationContext`)),
    temporalPattern: (row["temporalPattern"] === null ? null : architectProgramSnapshotGuardString(row["temporalPattern"], `${at}.temporalPattern`)),
    supervisionLevel: (row["supervisionLevel"] === null ? null : architectProgramSnapshotGuardString(row["supervisionLevel"], `${at}.supervisionLevel`)),
  };
}

export interface Adjacency {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly elementAId: string;
  readonly elementBId: string;
  readonly kind: AdjacencyKind;
  readonly connection: ConnectionKind;
  readonly separations: readonly SeparationKind[];
  readonly weight: Binary64Transport;
  readonly rationale: TextField | null;
  readonly distanceMaxM: Binary64Transport | null;
  readonly distanceMinM: Binary64Transport | null;
  readonly levelConstraint: string | null;
  readonly accessPath: string | null;
  readonly sharedWall: boolean;
  readonly sharedEntry: boolean;
  readonly trafficIsolation: boolean;
  readonly circulationOverlap: boolean;
  readonly conflictIds: readonly string[];
  readonly normalized: boolean;
  readonly verificationStatus: ValidationStatus;
  readonly sourceRelationshipId: string | null;
  readonly internalExternalAccess: string | null;
}

export function parseAdjacency(value: unknown, at = "$"): Adjacency {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "elementAId", "elementBId", "kind", "connection", "separations", "weight", "rationale", "distanceMaxM", "distanceMinM", "levelConstraint", "accessPath", "sharedWall", "sharedEntry", "trafficIsolation", "circulationOverlap", "conflictIds", "normalized", "verificationStatus", "sourceRelationshipId", "internalExternalAccess"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    elementAId: architectProgramSnapshotGuardString(row["elementAId"], `${at}.elementAId`),
    elementBId: architectProgramSnapshotGuardString(row["elementBId"], `${at}.elementBId`),
    kind: parseAdjacencyKind(row["kind"], `${at}.kind`),
    connection: parseConnectionKind(row["connection"], `${at}.connection`),
    separations: architectProgramSnapshotGuardArray(row["separations"], `${at}.separations`).map((item, index) => parseSeparationKind(item, `${at}.separations[${index}]`)),
    weight: parseBinary64Transport(row["weight"], `${at}.weight`),
    rationale: (row["rationale"] === null ? null : parseTextField(row["rationale"], `${at}.rationale`)),
    distanceMaxM: (row["distanceMaxM"] === null ? null : parseBinary64Transport(row["distanceMaxM"], `${at}.distanceMaxM`)),
    distanceMinM: (row["distanceMinM"] === null ? null : parseBinary64Transport(row["distanceMinM"], `${at}.distanceMinM`)),
    levelConstraint: (row["levelConstraint"] === null ? null : architectProgramSnapshotGuardString(row["levelConstraint"], `${at}.levelConstraint`)),
    accessPath: (row["accessPath"] === null ? null : architectProgramSnapshotGuardString(row["accessPath"], `${at}.accessPath`)),
    sharedWall: architectProgramSnapshotGuardBoolean(row["sharedWall"], `${at}.sharedWall`),
    sharedEntry: architectProgramSnapshotGuardBoolean(row["sharedEntry"], `${at}.sharedEntry`),
    trafficIsolation: architectProgramSnapshotGuardBoolean(row["trafficIsolation"], `${at}.trafficIsolation`),
    circulationOverlap: architectProgramSnapshotGuardBoolean(row["circulationOverlap"], `${at}.circulationOverlap`),
    conflictIds: architectProgramSnapshotGuardArray(row["conflictIds"], `${at}.conflictIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.conflictIds[${index}]`)),
    normalized: architectProgramSnapshotGuardBoolean(row["normalized"], `${at}.normalized`),
    verificationStatus: parseValidationStatus(row["verificationStatus"], `${at}.verificationStatus`),
    sourceRelationshipId: (row["sourceRelationshipId"] === null ? null : architectProgramSnapshotGuardString(row["sourceRelationshipId"], `${at}.sourceRelationshipId`)),
    internalExternalAccess: (row["internalExternalAccess"] === null ? null : architectProgramSnapshotGuardString(row["internalExternalAccess"], `${at}.internalExternalAccess`)),
  };
}

export type AdjacencyKind = "required" | "preferred" | "optional" | "prohibited";

export function parseAdjacencyKind(value: unknown, at = "$"): AdjacencyKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["required", "preferred", "optional", "prohibited"] as const);
}

export type AnalysisKind = "gap" | "conflict" | "dependency" | "capacity" | "demand" | "utilization" | "workflow" | "risk" | "cost" | "scenario" | "sensitivity" | "impact" | "trend" | "requirementComparison" | "requirementClustering" | "requirementFiltering" | "requirementSorting" | "requirementScoring" | "requirementWeighting" | "relationshipAnalysis";

export function parseAnalysisKind(value: unknown, at = "$"): AnalysisKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["gap", "conflict", "dependency", "capacity", "demand", "utilization", "workflow", "risk", "cost", "scenario", "sensitivity", "impact", "trend", "requirementComparison", "requirementClustering", "requirementFiltering", "requirementSorting", "requirementScoring", "requirementWeighting", "relationshipAnalysis"] as const);
}

export interface AnalysisRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly kind: AnalysisKind;
  readonly title: string;
  readonly parameters: readonly string[];
  readonly inputEntityIds: readonly string[];
  readonly outputSummary: TextField;
  readonly findings: readonly string[];
  readonly metrics: readonly string[];
  readonly charts: readonly string[];
  readonly runBy: string | null;
  readonly runAt: string | null;
  readonly durationMs: Unsigned64Transport | null;
  readonly toolVersion: string | null;
  readonly scenarioId: string | null;
  readonly reportId: string | null;
  readonly confidence: string | null;
  readonly limitations: readonly string[];
  readonly recommendations: readonly string[];
  readonly rawResultRef: string | null;
}

export function parseAnalysisRecord(value: unknown, at = "$"): AnalysisRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "kind", "title", "parameters", "inputEntityIds", "outputSummary", "findings", "metrics", "charts", "runBy", "runAt", "durationMs", "toolVersion", "scenarioId", "reportId", "confidence", "limitations", "recommendations", "rawResultRef"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    kind: parseAnalysisKind(row["kind"], `${at}.kind`),
    title: architectProgramSnapshotGuardString(row["title"], `${at}.title`),
    parameters: architectProgramSnapshotGuardArray(row["parameters"], `${at}.parameters`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.parameters[${index}]`)),
    inputEntityIds: architectProgramSnapshotGuardArray(row["inputEntityIds"], `${at}.inputEntityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.inputEntityIds[${index}]`)),
    outputSummary: parseTextField(row["outputSummary"], `${at}.outputSummary`),
    findings: architectProgramSnapshotGuardArray(row["findings"], `${at}.findings`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.findings[${index}]`)),
    metrics: architectProgramSnapshotGuardArray(row["metrics"], `${at}.metrics`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.metrics[${index}]`)),
    charts: architectProgramSnapshotGuardArray(row["charts"], `${at}.charts`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.charts[${index}]`)),
    runBy: (row["runBy"] === null ? null : architectProgramSnapshotGuardString(row["runBy"], `${at}.runBy`)),
    runAt: (row["runAt"] === null ? null : architectProgramSnapshotGuardString(row["runAt"], `${at}.runAt`)),
    durationMs: (row["durationMs"] === null ? null : parseUnsigned64Transport(row["durationMs"], `${at}.durationMs`)),
    toolVersion: (row["toolVersion"] === null ? null : architectProgramSnapshotGuardString(row["toolVersion"], `${at}.toolVersion`)),
    scenarioId: (row["scenarioId"] === null ? null : architectProgramSnapshotGuardString(row["scenarioId"], `${at}.scenarioId`)),
    reportId: (row["reportId"] === null ? null : architectProgramSnapshotGuardString(row["reportId"], `${at}.reportId`)),
    confidence: (row["confidence"] === null ? null : architectProgramSnapshotGuardString(row["confidence"], `${at}.confidence`)),
    limitations: architectProgramSnapshotGuardArray(row["limitations"], `${at}.limitations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.limitations[${index}]`)),
    recommendations: architectProgramSnapshotGuardArray(row["recommendations"], `${at}.recommendations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.recommendations[${index}]`)),
    rawResultRef: (row["rawResultRef"] === null ? null : architectProgramSnapshotGuardString(row["rawResultRef"], `${at}.rawResultRef`)),
  };
}

export interface ApprovalRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly approvalType: string;
  readonly subjectId: string;
  readonly approverIds: readonly string[];
  readonly approvalDate: string | null;
  readonly conditions: readonly string[];
  readonly approvalStatus: LifecycleStatus;
  readonly expiryDate: string | null;
  readonly delegationChain: readonly string[];
  readonly evidenceRefs: readonly string[];
  readonly relatedDecisionId: string | null;
  readonly relatedChangeId: string | null;
  readonly authorityBasis: readonly string[];
  readonly signatureMethod: string | null;
  readonly rejectionReason: TextField | null;
  readonly resubmissionDate: string | null;
  readonly notificationList: readonly string[];
  readonly workflowStep: string | null;
  readonly version: string | null;
  readonly auditTrailRef: string | null;
}

export function parseApprovalRecord(value: unknown, at = "$"): ApprovalRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "approvalType", "subjectId", "approverIds", "approvalDate", "conditions", "approvalStatus", "expiryDate", "delegationChain", "evidenceRefs", "relatedDecisionId", "relatedChangeId", "authorityBasis", "signatureMethod", "rejectionReason", "resubmissionDate", "notificationList", "workflowStep", "version", "auditTrailRef"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    approvalType: architectProgramSnapshotGuardString(row["approvalType"], `${at}.approvalType`),
    subjectId: architectProgramSnapshotGuardString(row["subjectId"], `${at}.subjectId`),
    approverIds: architectProgramSnapshotGuardArray(row["approverIds"], `${at}.approverIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.approverIds[${index}]`)),
    approvalDate: (row["approvalDate"] === null ? null : architectProgramSnapshotGuardString(row["approvalDate"], `${at}.approvalDate`)),
    conditions: architectProgramSnapshotGuardArray(row["conditions"], `${at}.conditions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.conditions[${index}]`)),
    approvalStatus: parseLifecycleStatus(row["approvalStatus"], `${at}.approvalStatus`),
    expiryDate: (row["expiryDate"] === null ? null : architectProgramSnapshotGuardString(row["expiryDate"], `${at}.expiryDate`)),
    delegationChain: architectProgramSnapshotGuardArray(row["delegationChain"], `${at}.delegationChain`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.delegationChain[${index}]`)),
    evidenceRefs: architectProgramSnapshotGuardArray(row["evidenceRefs"], `${at}.evidenceRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.evidenceRefs[${index}]`)),
    relatedDecisionId: (row["relatedDecisionId"] === null ? null : architectProgramSnapshotGuardString(row["relatedDecisionId"], `${at}.relatedDecisionId`)),
    relatedChangeId: (row["relatedChangeId"] === null ? null : architectProgramSnapshotGuardString(row["relatedChangeId"], `${at}.relatedChangeId`)),
    authorityBasis: architectProgramSnapshotGuardArray(row["authorityBasis"], `${at}.authorityBasis`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.authorityBasis[${index}]`)),
    signatureMethod: (row["signatureMethod"] === null ? null : architectProgramSnapshotGuardString(row["signatureMethod"], `${at}.signatureMethod`)),
    rejectionReason: (row["rejectionReason"] === null ? null : parseTextField(row["rejectionReason"], `${at}.rejectionReason`)),
    resubmissionDate: (row["resubmissionDate"] === null ? null : architectProgramSnapshotGuardString(row["resubmissionDate"], `${at}.resubmissionDate`)),
    notificationList: architectProgramSnapshotGuardArray(row["notificationList"], `${at}.notificationList`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.notificationList[${index}]`)),
    workflowStep: (row["workflowStep"] === null ? null : architectProgramSnapshotGuardString(row["workflowStep"], `${at}.workflowStep`)),
    version: (row["version"] === null ? null : architectProgramSnapshotGuardString(row["version"], `${at}.version`)),
    auditTrailRef: (row["auditTrailRef"] === null ? null : architectProgramSnapshotGuardString(row["auditTrailRef"], `${at}.auditTrailRef`)),
  };
}

export interface ArtifactRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly documentType: string;
  readonly title: string;
  readonly version: string;
  readonly fileRef: string | null;
  readonly format: string | null;
  readonly authorIds: readonly string[];
  readonly reviewerIds: readonly string[];
  readonly approverIds: readonly string[];
  readonly issueDate: string | null;
  readonly revisionDate: string | null;
  readonly distributionList: readonly string[];
  readonly relatedEntityIds: readonly string[];
  readonly classification: string | null;
  readonly retentionPeriod: string | null;
  readonly accessControls: readonly string[];
  readonly supersedes: string | null;
  readonly documentStatus: LifecycleStatus;
  readonly checksum: string | null;
  readonly sourceSystem: string | null;
}

export function parseArtifactRecord(value: unknown, at = "$"): ArtifactRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "documentType", "title", "version", "fileRef", "format", "authorIds", "reviewerIds", "approverIds", "issueDate", "revisionDate", "distributionList", "relatedEntityIds", "classification", "retentionPeriod", "accessControls", "supersedes", "documentStatus", "checksum", "sourceSystem"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    documentType: architectProgramSnapshotGuardString(row["documentType"], `${at}.documentType`),
    title: architectProgramSnapshotGuardString(row["title"], `${at}.title`),
    version: architectProgramSnapshotGuardString(row["version"], `${at}.version`),
    fileRef: (row["fileRef"] === null ? null : architectProgramSnapshotGuardString(row["fileRef"], `${at}.fileRef`)),
    format: (row["format"] === null ? null : architectProgramSnapshotGuardString(row["format"], `${at}.format`)),
    authorIds: architectProgramSnapshotGuardArray(row["authorIds"], `${at}.authorIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.authorIds[${index}]`)),
    reviewerIds: architectProgramSnapshotGuardArray(row["reviewerIds"], `${at}.reviewerIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.reviewerIds[${index}]`)),
    approverIds: architectProgramSnapshotGuardArray(row["approverIds"], `${at}.approverIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.approverIds[${index}]`)),
    issueDate: (row["issueDate"] === null ? null : architectProgramSnapshotGuardString(row["issueDate"], `${at}.issueDate`)),
    revisionDate: (row["revisionDate"] === null ? null : architectProgramSnapshotGuardString(row["revisionDate"], `${at}.revisionDate`)),
    distributionList: architectProgramSnapshotGuardArray(row["distributionList"], `${at}.distributionList`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.distributionList[${index}]`)),
    relatedEntityIds: architectProgramSnapshotGuardArray(row["relatedEntityIds"], `${at}.relatedEntityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedEntityIds[${index}]`)),
    classification: (row["classification"] === null ? null : architectProgramSnapshotGuardString(row["classification"], `${at}.classification`)),
    retentionPeriod: (row["retentionPeriod"] === null ? null : architectProgramSnapshotGuardString(row["retentionPeriod"], `${at}.retentionPeriod`)),
    accessControls: architectProgramSnapshotGuardArray(row["accessControls"], `${at}.accessControls`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.accessControls[${index}]`)),
    supersedes: (row["supersedes"] === null ? null : architectProgramSnapshotGuardString(row["supersedes"], `${at}.supersedes`)),
    documentStatus: parseLifecycleStatus(row["documentStatus"], `${at}.documentStatus`),
    checksum: (row["checksum"] === null ? null : architectProgramSnapshotGuardString(row["checksum"], `${at}.checksum`)),
    sourceSystem: (row["sourceSystem"] === null ? null : architectProgramSnapshotGuardString(row["sourceSystem"], `${at}.sourceSystem`)),
  };
}

export interface Assumption {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly statement: TextField;
  readonly basis: TextField | null;
  readonly confidenceLevel: string | null;
  readonly impactIfFalse: TextField | null;
  readonly relatedEntityIds: readonly string[];
  readonly validationStatus: ValidationStatus;
  readonly validatedBy: string | null;
  readonly validationDate: string | null;
  readonly ownerId: string | null;
  readonly reviewCycle: string | null;
  readonly source: string | null;
  readonly category: string | null;
  readonly dependencies: readonly string[];
  readonly mitigation: readonly string[];
  readonly linkedRequirementIds: readonly string[];
  readonly linkedRiskIds: readonly string[];
  readonly expirationDate: string | null;
  readonly statusNotes: readonly TaggedNote[];
  readonly artifactRefs: readonly string[];
}

export function parseAssumption(value: unknown, at = "$"): Assumption {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "statement", "basis", "confidenceLevel", "impactIfFalse", "relatedEntityIds", "validationStatus", "validatedBy", "validationDate", "ownerId", "reviewCycle", "source", "category", "dependencies", "mitigation", "linkedRequirementIds", "linkedRiskIds", "expirationDate", "statusNotes", "artifactRefs"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    statement: parseTextField(row["statement"], `${at}.statement`),
    basis: (row["basis"] === null ? null : parseTextField(row["basis"], `${at}.basis`)),
    confidenceLevel: (row["confidenceLevel"] === null ? null : architectProgramSnapshotGuardString(row["confidenceLevel"], `${at}.confidenceLevel`)),
    impactIfFalse: (row["impactIfFalse"] === null ? null : parseTextField(row["impactIfFalse"], `${at}.impactIfFalse`)),
    relatedEntityIds: architectProgramSnapshotGuardArray(row["relatedEntityIds"], `${at}.relatedEntityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedEntityIds[${index}]`)),
    validationStatus: parseValidationStatus(row["validationStatus"], `${at}.validationStatus`),
    validatedBy: (row["validatedBy"] === null ? null : architectProgramSnapshotGuardString(row["validatedBy"], `${at}.validatedBy`)),
    validationDate: (row["validationDate"] === null ? null : architectProgramSnapshotGuardString(row["validationDate"], `${at}.validationDate`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    reviewCycle: (row["reviewCycle"] === null ? null : architectProgramSnapshotGuardString(row["reviewCycle"], `${at}.reviewCycle`)),
    source: (row["source"] === null ? null : architectProgramSnapshotGuardString(row["source"], `${at}.source`)),
    category: (row["category"] === null ? null : architectProgramSnapshotGuardString(row["category"], `${at}.category`)),
    dependencies: architectProgramSnapshotGuardArray(row["dependencies"], `${at}.dependencies`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dependencies[${index}]`)),
    mitigation: architectProgramSnapshotGuardArray(row["mitigation"], `${at}.mitigation`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.mitigation[${index}]`)),
    linkedRequirementIds: architectProgramSnapshotGuardArray(row["linkedRequirementIds"], `${at}.linkedRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.linkedRequirementIds[${index}]`)),
    linkedRiskIds: architectProgramSnapshotGuardArray(row["linkedRiskIds"], `${at}.linkedRiskIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.linkedRiskIds[${index}]`)),
    expirationDate: (row["expirationDate"] === null ? null : architectProgramSnapshotGuardString(row["expirationDate"], `${at}.expirationDate`)),
    statusNotes: architectProgramSnapshotGuardArray(row["statusNotes"], `${at}.statusNotes`).map((item, index) => parseTaggedNote(item, `${at}.statusNotes[${index}]`)),
    artifactRefs: architectProgramSnapshotGuardArray(row["artifactRefs"], `${at}.artifactRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.artifactRefs[${index}]`)),
  };
}

export type AuditAction = "created" | "updated" | "deleted" | "reviewed" | "approved" | "rejected" | "exported" | "imported" | "merged" | "archived";

export function parseAuditAction(value: unknown, at = "$"): AuditAction {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["created", "updated", "deleted", "reviewed", "approved", "rejected", "exported", "imported", "merged", "archived"] as const);
}

export interface AuditEvent {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly action: AuditAction;
  readonly actorId: string | null;
  readonly subjectId: string;
  readonly subjectKind: string;
  readonly timestamp: string;
  readonly details: TextField;
  readonly beforeState: string | null;
  readonly afterState: string | null;
  readonly ipAddress: string | null;
  readonly client: string | null;
  readonly sessionId: string | null;
  readonly changeRecordId: string | null;
  readonly traceLink: TraceLink | null;
  readonly success: boolean;
  readonly errorMessage: string | null;
  readonly correlationId: string | null;
  readonly complianceTags: readonly string[];
  readonly retentionUntil: string | null;
}

export function parseAuditEvent(value: unknown, at = "$"): AuditEvent {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "action", "actorId", "subjectId", "subjectKind", "timestamp", "details", "beforeState", "afterState", "ipAddress", "client", "sessionId", "changeRecordId", "traceLink", "success", "errorMessage", "correlationId", "complianceTags", "retentionUntil"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    action: parseAuditAction(row["action"], `${at}.action`),
    actorId: (row["actorId"] === null ? null : architectProgramSnapshotGuardString(row["actorId"], `${at}.actorId`)),
    subjectId: architectProgramSnapshotGuardString(row["subjectId"], `${at}.subjectId`),
    subjectKind: architectProgramSnapshotGuardString(row["subjectKind"], `${at}.subjectKind`),
    timestamp: architectProgramSnapshotGuardString(row["timestamp"], `${at}.timestamp`),
    details: parseTextField(row["details"], `${at}.details`),
    beforeState: (row["beforeState"] === null ? null : architectProgramSnapshotGuardString(row["beforeState"], `${at}.beforeState`)),
    afterState: (row["afterState"] === null ? null : architectProgramSnapshotGuardString(row["afterState"], `${at}.afterState`)),
    ipAddress: (row["ipAddress"] === null ? null : architectProgramSnapshotGuardString(row["ipAddress"], `${at}.ipAddress`)),
    client: (row["client"] === null ? null : architectProgramSnapshotGuardString(row["client"], `${at}.client`)),
    sessionId: (row["sessionId"] === null ? null : architectProgramSnapshotGuardString(row["sessionId"], `${at}.sessionId`)),
    changeRecordId: (row["changeRecordId"] === null ? null : architectProgramSnapshotGuardString(row["changeRecordId"], `${at}.changeRecordId`)),
    traceLink: (row["traceLink"] === null ? null : parseTraceLink(row["traceLink"], `${at}.traceLink`)),
    success: architectProgramSnapshotGuardBoolean(row["success"], `${at}.success`),
    errorMessage: (row["errorMessage"] === null ? null : architectProgramSnapshotGuardString(row["errorMessage"], `${at}.errorMessage`)),
    correlationId: (row["correlationId"] === null ? null : architectProgramSnapshotGuardString(row["correlationId"], `${at}.correlationId`)),
    complianceTags: architectProgramSnapshotGuardArray(row["complianceTags"], `${at}.complianceTags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.complianceTags[${index}]`)),
    retentionUntil: (row["retentionUntil"] === null ? null : architectProgramSnapshotGuardString(row["retentionUntil"], `${at}.retentionUntil`)),
  };
}

export interface BenchmarkRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly benchmarkName: string;
  readonly sector: string;
  readonly metric: string;
  readonly value: Binary64Transport;
  readonly unit: string;
  readonly sampleSize: number | null;
  readonly source: string | null;
  readonly collectionYear: number | null;
  readonly geography: string | null;
  readonly buildingType: string | null;
  readonly confidence: string | null;
  readonly methodology: string | null;
  readonly applicableElementKinds: readonly string[];
  readonly relatedRequirementIds: readonly string[];
  readonly comparisonNotes: readonly string[];
  readonly limitations: readonly string[];
  readonly license: string | null;
  readonly knowledgeId: string | null;
  readonly lastVerified: string | null;
}

export function parseBenchmarkRecord(value: unknown, at = "$"): BenchmarkRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "benchmarkName", "sector", "metric", "value", "unit", "sampleSize", "source", "collectionYear", "geography", "buildingType", "confidence", "methodology", "applicableElementKinds", "relatedRequirementIds", "comparisonNotes", "limitations", "license", "knowledgeId", "lastVerified"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    benchmarkName: architectProgramSnapshotGuardString(row["benchmarkName"], `${at}.benchmarkName`),
    sector: architectProgramSnapshotGuardString(row["sector"], `${at}.sector`),
    metric: architectProgramSnapshotGuardString(row["metric"], `${at}.metric`),
    value: parseBinary64Transport(row["value"], `${at}.value`),
    unit: architectProgramSnapshotGuardString(row["unit"], `${at}.unit`),
    sampleSize: (row["sampleSize"] === null ? null : architectProgramSnapshotGuardInteger(row["sampleSize"], `${at}.sampleSize`, {"minimum": 0, "maximum": 4294967295})),
    source: (row["source"] === null ? null : architectProgramSnapshotGuardString(row["source"], `${at}.source`)),
    collectionYear: (row["collectionYear"] === null ? null : architectProgramSnapshotGuardInteger(row["collectionYear"], `${at}.collectionYear`, {"minimum": 0, "maximum": 4294967295})),
    geography: (row["geography"] === null ? null : architectProgramSnapshotGuardString(row["geography"], `${at}.geography`)),
    buildingType: (row["buildingType"] === null ? null : architectProgramSnapshotGuardString(row["buildingType"], `${at}.buildingType`)),
    confidence: (row["confidence"] === null ? null : architectProgramSnapshotGuardString(row["confidence"], `${at}.confidence`)),
    methodology: (row["methodology"] === null ? null : architectProgramSnapshotGuardString(row["methodology"], `${at}.methodology`)),
    applicableElementKinds: architectProgramSnapshotGuardArray(row["applicableElementKinds"], `${at}.applicableElementKinds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.applicableElementKinds[${index}]`)),
    relatedRequirementIds: architectProgramSnapshotGuardArray(row["relatedRequirementIds"], `${at}.relatedRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedRequirementIds[${index}]`)),
    comparisonNotes: architectProgramSnapshotGuardArray(row["comparisonNotes"], `${at}.comparisonNotes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.comparisonNotes[${index}]`)),
    limitations: architectProgramSnapshotGuardArray(row["limitations"], `${at}.limitations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.limitations[${index}]`)),
    license: (row["license"] === null ? null : architectProgramSnapshotGuardString(row["license"], `${at}.license`)),
    knowledgeId: (row["knowledgeId"] === null ? null : architectProgramSnapshotGuardString(row["knowledgeId"], `${at}.knowledgeId`)),
    lastVerified: (row["lastVerified"] === null ? null : architectProgramSnapshotGuardString(row["lastVerified"], `${at}.lastVerified`)),
  };
}

export type Binary64Transport = number | { readonly bits: string };

export function parseBinary64Transport(value: unknown, at = "$"): Binary64Transport {
  return ((input0: unknown) => { if (typeof input0 === "number") { try { return architectProgramSnapshotGuardNumber(input0, `${at}`); } catch (error) { if (!(error instanceof architectProgramSnapshotGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ bits: architectProgramSnapshotGuardString(row1["bits"], `${at}.bits`, {"pattern": "^[0-9a-f]{16}$"}) }))(architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(input0, `${at}`), `${at}`, ["bits"])); } catch (error) { if (!(error instanceof architectProgramSnapshotGuardRefusal)) throw error; } } return architectProgramSnapshotGuardReject(`${at}`, "value matches no variant"); })(value);
}

export interface ChangeRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly changeType: string;
  readonly summary: TextField;
  readonly reason: TextField;
  readonly requestedBy: string | null;
  readonly approvedBy: string | null;
  readonly changeDate: string | null;
  readonly effectiveDate: string | null;
  readonly impactedEntityIds: readonly string[];
  readonly beforeSnapshot: string | null;
  readonly afterSnapshot: string | null;
  readonly costImpact: Binary64Transport | null;
  readonly scheduleImpact: string | null;
  readonly riskImpact: readonly string[];
  readonly approvalStatus: ValidationStatus;
  readonly rollbackPlan: readonly string[];
  readonly communicationPlan: readonly string[];
  readonly versionFrom: string | null;
  readonly versionTo: string | null;
  readonly auditEventIds: readonly string[];
}

export function parseChangeRecord(value: unknown, at = "$"): ChangeRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "changeType", "summary", "reason", "requestedBy", "approvedBy", "changeDate", "effectiveDate", "impactedEntityIds", "beforeSnapshot", "afterSnapshot", "costImpact", "scheduleImpact", "riskImpact", "approvalStatus", "rollbackPlan", "communicationPlan", "versionFrom", "versionTo", "auditEventIds"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    changeType: architectProgramSnapshotGuardString(row["changeType"], `${at}.changeType`),
    summary: parseTextField(row["summary"], `${at}.summary`),
    reason: parseTextField(row["reason"], `${at}.reason`),
    requestedBy: (row["requestedBy"] === null ? null : architectProgramSnapshotGuardString(row["requestedBy"], `${at}.requestedBy`)),
    approvedBy: (row["approvedBy"] === null ? null : architectProgramSnapshotGuardString(row["approvedBy"], `${at}.approvedBy`)),
    changeDate: (row["changeDate"] === null ? null : architectProgramSnapshotGuardString(row["changeDate"], `${at}.changeDate`)),
    effectiveDate: (row["effectiveDate"] === null ? null : architectProgramSnapshotGuardString(row["effectiveDate"], `${at}.effectiveDate`)),
    impactedEntityIds: architectProgramSnapshotGuardArray(row["impactedEntityIds"], `${at}.impactedEntityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.impactedEntityIds[${index}]`)),
    beforeSnapshot: (row["beforeSnapshot"] === null ? null : architectProgramSnapshotGuardString(row["beforeSnapshot"], `${at}.beforeSnapshot`)),
    afterSnapshot: (row["afterSnapshot"] === null ? null : architectProgramSnapshotGuardString(row["afterSnapshot"], `${at}.afterSnapshot`)),
    costImpact: (row["costImpact"] === null ? null : parseBinary64Transport(row["costImpact"], `${at}.costImpact`)),
    scheduleImpact: (row["scheduleImpact"] === null ? null : architectProgramSnapshotGuardString(row["scheduleImpact"], `${at}.scheduleImpact`)),
    riskImpact: architectProgramSnapshotGuardArray(row["riskImpact"], `${at}.riskImpact`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.riskImpact[${index}]`)),
    approvalStatus: parseValidationStatus(row["approvalStatus"], `${at}.approvalStatus`),
    rollbackPlan: architectProgramSnapshotGuardArray(row["rollbackPlan"], `${at}.rollbackPlan`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.rollbackPlan[${index}]`)),
    communicationPlan: architectProgramSnapshotGuardArray(row["communicationPlan"], `${at}.communicationPlan`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.communicationPlan[${index}]`)),
    versionFrom: (row["versionFrom"] === null ? null : architectProgramSnapshotGuardString(row["versionFrom"], `${at}.versionFrom`)),
    versionTo: (row["versionTo"] === null ? null : architectProgramSnapshotGuardString(row["versionTo"], `${at}.versionTo`)),
    auditEventIds: architectProgramSnapshotGuardArray(row["auditEventIds"], `${at}.auditEventIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.auditEventIds[${index}]`)),
  };
}

export interface CollaborationRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly sessionType: string;
  readonly title: string;
  readonly participants: readonly string[];
  readonly facilitatorId: string | null;
  readonly startTime: string | null;
  readonly endTime: string | null;
  readonly location: string | null;
  readonly agenda: readonly string[];
  readonly outcomes: readonly string[];
  readonly actionItems: readonly string[];
  readonly decisionIds: readonly string[];
  readonly issueIds: readonly string[];
  readonly documentIds: readonly string[];
  readonly recordingRef: string | null;
  readonly feedback: readonly TaggedNote[];
  readonly followUpDate: string | null;
  readonly workshopId: string | null;
  readonly surveyId: string | null;
}

export function parseCollaborationRecord(value: unknown, at = "$"): CollaborationRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "sessionType", "title", "participants", "facilitatorId", "startTime", "endTime", "location", "agenda", "outcomes", "actionItems", "decisionIds", "issueIds", "documentIds", "recordingRef", "feedback", "followUpDate", "workshopId", "surveyId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    sessionType: architectProgramSnapshotGuardString(row["sessionType"], `${at}.sessionType`),
    title: architectProgramSnapshotGuardString(row["title"], `${at}.title`),
    participants: architectProgramSnapshotGuardArray(row["participants"], `${at}.participants`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.participants[${index}]`)),
    facilitatorId: (row["facilitatorId"] === null ? null : architectProgramSnapshotGuardString(row["facilitatorId"], `${at}.facilitatorId`)),
    startTime: (row["startTime"] === null ? null : architectProgramSnapshotGuardString(row["startTime"], `${at}.startTime`)),
    endTime: (row["endTime"] === null ? null : architectProgramSnapshotGuardString(row["endTime"], `${at}.endTime`)),
    location: (row["location"] === null ? null : architectProgramSnapshotGuardString(row["location"], `${at}.location`)),
    agenda: architectProgramSnapshotGuardArray(row["agenda"], `${at}.agenda`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.agenda[${index}]`)),
    outcomes: architectProgramSnapshotGuardArray(row["outcomes"], `${at}.outcomes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.outcomes[${index}]`)),
    actionItems: architectProgramSnapshotGuardArray(row["actionItems"], `${at}.actionItems`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.actionItems[${index}]`)),
    decisionIds: architectProgramSnapshotGuardArray(row["decisionIds"], `${at}.decisionIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decisionIds[${index}]`)),
    issueIds: architectProgramSnapshotGuardArray(row["issueIds"], `${at}.issueIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.issueIds[${index}]`)),
    documentIds: architectProgramSnapshotGuardArray(row["documentIds"], `${at}.documentIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.documentIds[${index}]`)),
    recordingRef: (row["recordingRef"] === null ? null : architectProgramSnapshotGuardString(row["recordingRef"], `${at}.recordingRef`)),
    feedback: architectProgramSnapshotGuardArray(row["feedback"], `${at}.feedback`).map((item, index) => parseTaggedNote(item, `${at}.feedback[${index}]`)),
    followUpDate: (row["followUpDate"] === null ? null : architectProgramSnapshotGuardString(row["followUpDate"], `${at}.followUpDate`)),
    workshopId: (row["workshopId"] === null ? null : architectProgramSnapshotGuardString(row["workshopId"], `${at}.workshopId`)),
    surveyId: (row["surveyId"] === null ? null : architectProgramSnapshotGuardString(row["surveyId"], `${at}.surveyId`)),
  };
}

export interface CommunicationRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly channel: string;
  readonly audienceIds: readonly string[];
  readonly messageTypes: readonly string[];
  readonly frequency: string | null;
  readonly medium: readonly string[];
  readonly language: readonly string[];
  readonly accessibility: readonly string[];
  readonly emergencyUse: boolean;
  readonly twoWay: boolean;
  readonly recordingPolicy: string | null;
  readonly signageLocations: readonly string[];
  readonly technology: readonly string[];
  readonly escalationPath: readonly string[];
  readonly feedbackLoop: boolean;
  readonly privacyControls: readonly string[];
  readonly elementIds: readonly string[];
  readonly standards: readonly string[];
  readonly ownerId: string | null;
  readonly templates: readonly string[];
}

export function parseCommunicationRequirement(value: unknown, at = "$"): CommunicationRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "channel", "audienceIds", "messageTypes", "frequency", "medium", "language", "accessibility", "emergencyUse", "twoWay", "recordingPolicy", "signageLocations", "technology", "escalationPath", "feedbackLoop", "privacyControls", "elementIds", "standards", "ownerId", "templates"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    channel: architectProgramSnapshotGuardString(row["channel"], `${at}.channel`),
    audienceIds: architectProgramSnapshotGuardArray(row["audienceIds"], `${at}.audienceIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.audienceIds[${index}]`)),
    messageTypes: architectProgramSnapshotGuardArray(row["messageTypes"], `${at}.messageTypes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.messageTypes[${index}]`)),
    frequency: (row["frequency"] === null ? null : architectProgramSnapshotGuardString(row["frequency"], `${at}.frequency`)),
    medium: architectProgramSnapshotGuardArray(row["medium"], `${at}.medium`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.medium[${index}]`)),
    language: architectProgramSnapshotGuardArray(row["language"], `${at}.language`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.language[${index}]`)),
    accessibility: architectProgramSnapshotGuardArray(row["accessibility"], `${at}.accessibility`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.accessibility[${index}]`)),
    emergencyUse: architectProgramSnapshotGuardBoolean(row["emergencyUse"], `${at}.emergencyUse`),
    twoWay: architectProgramSnapshotGuardBoolean(row["twoWay"], `${at}.twoWay`),
    recordingPolicy: (row["recordingPolicy"] === null ? null : architectProgramSnapshotGuardString(row["recordingPolicy"], `${at}.recordingPolicy`)),
    signageLocations: architectProgramSnapshotGuardArray(row["signageLocations"], `${at}.signageLocations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.signageLocations[${index}]`)),
    technology: architectProgramSnapshotGuardArray(row["technology"], `${at}.technology`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.technology[${index}]`)),
    escalationPath: architectProgramSnapshotGuardArray(row["escalationPath"], `${at}.escalationPath`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.escalationPath[${index}]`)),
    feedbackLoop: architectProgramSnapshotGuardBoolean(row["feedbackLoop"], `${at}.feedbackLoop`),
    privacyControls: architectProgramSnapshotGuardArray(row["privacyControls"], `${at}.privacyControls`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.privacyControls[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    templates: architectProgramSnapshotGuardArray(row["templates"], `${at}.templates`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.templates[${index}]`)),
  };
}

export interface ComplianceRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly standardRef: string;
  readonly obligation: TextField;
  readonly complianceStatus: ValidationStatus;
  readonly evidenceRefs: readonly string[];
  readonly auditorId: string | null;
  readonly auditDate: string | null;
  readonly nextReview: string | null;
  readonly affectedEntityIds: readonly string[];
  readonly gapAnalysis: readonly string[];
  readonly remediationPlan: readonly string[];
  readonly ownerId: string | null;
  readonly severity: RiskLevel;
  readonly regulatoryBody: string | null;
  readonly certificationTarget: string | null;
  readonly waiverStatus: string | null;
  readonly relatedRequirementIds: readonly string[];
  readonly monitoringMethod: string | null;
  readonly reportingFrequency: string | null;
  readonly penalties: readonly string[];
  readonly correctiveActions: readonly string[];
  readonly artifactRefs: readonly string[];
}

export function parseComplianceRecord(value: unknown, at = "$"): ComplianceRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "standardRef", "obligation", "complianceStatus", "evidenceRefs", "auditorId", "auditDate", "nextReview", "affectedEntityIds", "gapAnalysis", "remediationPlan", "ownerId", "severity", "regulatoryBody", "certificationTarget", "waiverStatus", "relatedRequirementIds", "monitoringMethod", "reportingFrequency", "penalties", "correctiveActions", "artifactRefs"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    standardRef: architectProgramSnapshotGuardString(row["standardRef"], `${at}.standardRef`),
    obligation: parseTextField(row["obligation"], `${at}.obligation`),
    complianceStatus: parseValidationStatus(row["complianceStatus"], `${at}.complianceStatus`),
    evidenceRefs: architectProgramSnapshotGuardArray(row["evidenceRefs"], `${at}.evidenceRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.evidenceRefs[${index}]`)),
    auditorId: (row["auditorId"] === null ? null : architectProgramSnapshotGuardString(row["auditorId"], `${at}.auditorId`)),
    auditDate: (row["auditDate"] === null ? null : architectProgramSnapshotGuardString(row["auditDate"], `${at}.auditDate`)),
    nextReview: (row["nextReview"] === null ? null : architectProgramSnapshotGuardString(row["nextReview"], `${at}.nextReview`)),
    affectedEntityIds: architectProgramSnapshotGuardArray(row["affectedEntityIds"], `${at}.affectedEntityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.affectedEntityIds[${index}]`)),
    gapAnalysis: architectProgramSnapshotGuardArray(row["gapAnalysis"], `${at}.gapAnalysis`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.gapAnalysis[${index}]`)),
    remediationPlan: architectProgramSnapshotGuardArray(row["remediationPlan"], `${at}.remediationPlan`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.remediationPlan[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    severity: parseRiskLevel(row["severity"], `${at}.severity`),
    regulatoryBody: (row["regulatoryBody"] === null ? null : architectProgramSnapshotGuardString(row["regulatoryBody"], `${at}.regulatoryBody`)),
    certificationTarget: (row["certificationTarget"] === null ? null : architectProgramSnapshotGuardString(row["certificationTarget"], `${at}.certificationTarget`)),
    waiverStatus: (row["waiverStatus"] === null ? null : architectProgramSnapshotGuardString(row["waiverStatus"], `${at}.waiverStatus`)),
    relatedRequirementIds: architectProgramSnapshotGuardArray(row["relatedRequirementIds"], `${at}.relatedRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedRequirementIds[${index}]`)),
    monitoringMethod: (row["monitoringMethod"] === null ? null : architectProgramSnapshotGuardString(row["monitoringMethod"], `${at}.monitoringMethod`)),
    reportingFrequency: (row["reportingFrequency"] === null ? null : architectProgramSnapshotGuardString(row["reportingFrequency"], `${at}.reportingFrequency`)),
    penalties: architectProgramSnapshotGuardArray(row["penalties"], `${at}.penalties`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.penalties[${index}]`)),
    correctiveActions: architectProgramSnapshotGuardArray(row["correctiveActions"], `${at}.correctiveActions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.correctiveActions[${index}]`)),
    artifactRefs: architectProgramSnapshotGuardArray(row["artifactRefs"], `${at}.artifactRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.artifactRefs[${index}]`)),
  };
}

export interface Conflict {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly kind: ConflictKind;
  readonly summary: TextField;
  readonly entityAId: string;
  readonly entityBId: string;
  readonly severity: IssueSeverity;
  readonly detectedBy: string | null;
  readonly detectionDate: string | null;
  readonly tradeOffOptions: readonly string[];
  readonly recommendedResolution: TextField | null;
  readonly decisionId: string | null;
  readonly stakeholderIds: readonly string[];
  readonly requirementIds: readonly string[];
  readonly costImpact: Binary64Transport | null;
  readonly scheduleImpact: string | null;
  readonly qualityImpact: readonly string[];
  readonly resolutionStatus: ValidationStatus;
  readonly ownerId: string | null;
  readonly escalationLevel: string | null;
  readonly relatedRiskIds: readonly string[];
}

export function parseConflict(value: unknown, at = "$"): Conflict {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "kind", "summary", "entityAId", "entityBId", "severity", "detectedBy", "detectionDate", "tradeOffOptions", "recommendedResolution", "decisionId", "stakeholderIds", "requirementIds", "costImpact", "scheduleImpact", "qualityImpact", "resolutionStatus", "ownerId", "escalationLevel", "relatedRiskIds"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    kind: parseConflictKind(row["kind"], `${at}.kind`),
    summary: parseTextField(row["summary"], `${at}.summary`),
    entityAId: architectProgramSnapshotGuardString(row["entityAId"], `${at}.entityAId`),
    entityBId: architectProgramSnapshotGuardString(row["entityBId"], `${at}.entityBId`),
    severity: parseIssueSeverity(row["severity"], `${at}.severity`),
    detectedBy: (row["detectedBy"] === null ? null : architectProgramSnapshotGuardString(row["detectedBy"], `${at}.detectedBy`)),
    detectionDate: (row["detectionDate"] === null ? null : architectProgramSnapshotGuardString(row["detectionDate"], `${at}.detectionDate`)),
    tradeOffOptions: architectProgramSnapshotGuardArray(row["tradeOffOptions"], `${at}.tradeOffOptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tradeOffOptions[${index}]`)),
    recommendedResolution: (row["recommendedResolution"] === null ? null : parseTextField(row["recommendedResolution"], `${at}.recommendedResolution`)),
    decisionId: (row["decisionId"] === null ? null : architectProgramSnapshotGuardString(row["decisionId"], `${at}.decisionId`)),
    stakeholderIds: architectProgramSnapshotGuardArray(row["stakeholderIds"], `${at}.stakeholderIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderIds[${index}]`)),
    requirementIds: architectProgramSnapshotGuardArray(row["requirementIds"], `${at}.requirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.requirementIds[${index}]`)),
    costImpact: (row["costImpact"] === null ? null : parseBinary64Transport(row["costImpact"], `${at}.costImpact`)),
    scheduleImpact: (row["scheduleImpact"] === null ? null : architectProgramSnapshotGuardString(row["scheduleImpact"], `${at}.scheduleImpact`)),
    qualityImpact: architectProgramSnapshotGuardArray(row["qualityImpact"], `${at}.qualityImpact`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.qualityImpact[${index}]`)),
    resolutionStatus: parseValidationStatus(row["resolutionStatus"], `${at}.resolutionStatus`),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    escalationLevel: (row["escalationLevel"] === null ? null : architectProgramSnapshotGuardString(row["escalationLevel"], `${at}.escalationLevel`)),
    relatedRiskIds: architectProgramSnapshotGuardArray(row["relatedRiskIds"], `${at}.relatedRiskIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedRiskIds[${index}]`)),
  };
}

export type ConflictKind = "adjacency" | "capacity" | "schedule" | "budget" | "regulatory" | "operational" | "environmental" | "security" | "priority";

export function parseConflictKind(value: unknown, at = "$"): ConflictKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["adjacency", "capacity", "schedule", "budget", "regulatory", "operational", "environmental", "security", "priority"] as const);
}

export type ConnectionKind = "direct" | "indirect" | "controlled" | "sharedAccess" | "none";

export function parseConnectionKind(value: unknown, at = "$"): ConnectionKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["direct", "indirect", "controlled", "sharedAccess", "none"] as const);
}

export interface ConstraintRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly constraintType: string;
  readonly summary: TextField;
  readonly severity: RiskLevel;
  readonly affectedEntityIds: readonly string[];
  readonly source: string | null;
  readonly regulatoryBasis: readonly string[];
  readonly mitigationOptions: readonly string[];
  readonly ownerId: string | null;
  readonly effectiveDate: string | null;
  readonly expiryDate: string | null;
  readonly waiverStatus: string | null;
  readonly waiverApprover: string | null;
  readonly impactAssessment: TextField | null;
  readonly resolutionPlan: readonly string[];
  readonly relatedRequirementIds: readonly string[];
  readonly relatedDecisionIds: readonly string[];
  readonly monitoringFrequency: string | null;
  readonly complianceStatus: ValidationStatus;
  readonly exceptions: readonly string[];
  readonly traceLinks: readonly TraceLink[];
  readonly escalationContactId: string | null;
}

export function parseConstraintRecord(value: unknown, at = "$"): ConstraintRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "constraintType", "summary", "severity", "affectedEntityIds", "source", "regulatoryBasis", "mitigationOptions", "ownerId", "effectiveDate", "expiryDate", "waiverStatus", "waiverApprover", "impactAssessment", "resolutionPlan", "relatedRequirementIds", "relatedDecisionIds", "monitoringFrequency", "complianceStatus", "exceptions", "traceLinks", "escalationContactId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    constraintType: architectProgramSnapshotGuardString(row["constraintType"], `${at}.constraintType`),
    summary: parseTextField(row["summary"], `${at}.summary`),
    severity: parseRiskLevel(row["severity"], `${at}.severity`),
    affectedEntityIds: architectProgramSnapshotGuardArray(row["affectedEntityIds"], `${at}.affectedEntityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.affectedEntityIds[${index}]`)),
    source: (row["source"] === null ? null : architectProgramSnapshotGuardString(row["source"], `${at}.source`)),
    regulatoryBasis: architectProgramSnapshotGuardArray(row["regulatoryBasis"], `${at}.regulatoryBasis`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryBasis[${index}]`)),
    mitigationOptions: architectProgramSnapshotGuardArray(row["mitigationOptions"], `${at}.mitigationOptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.mitigationOptions[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    effectiveDate: (row["effectiveDate"] === null ? null : architectProgramSnapshotGuardString(row["effectiveDate"], `${at}.effectiveDate`)),
    expiryDate: (row["expiryDate"] === null ? null : architectProgramSnapshotGuardString(row["expiryDate"], `${at}.expiryDate`)),
    waiverStatus: (row["waiverStatus"] === null ? null : architectProgramSnapshotGuardString(row["waiverStatus"], `${at}.waiverStatus`)),
    waiverApprover: (row["waiverApprover"] === null ? null : architectProgramSnapshotGuardString(row["waiverApprover"], `${at}.waiverApprover`)),
    impactAssessment: (row["impactAssessment"] === null ? null : parseTextField(row["impactAssessment"], `${at}.impactAssessment`)),
    resolutionPlan: architectProgramSnapshotGuardArray(row["resolutionPlan"], `${at}.resolutionPlan`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.resolutionPlan[${index}]`)),
    relatedRequirementIds: architectProgramSnapshotGuardArray(row["relatedRequirementIds"], `${at}.relatedRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedRequirementIds[${index}]`)),
    relatedDecisionIds: architectProgramSnapshotGuardArray(row["relatedDecisionIds"], `${at}.relatedDecisionIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedDecisionIds[${index}]`)),
    monitoringFrequency: (row["monitoringFrequency"] === null ? null : architectProgramSnapshotGuardString(row["monitoringFrequency"], `${at}.monitoringFrequency`)),
    complianceStatus: parseValidationStatus(row["complianceStatus"], `${at}.complianceStatus`),
    exceptions: architectProgramSnapshotGuardArray(row["exceptions"], `${at}.exceptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.exceptions[${index}]`)),
    traceLinks: architectProgramSnapshotGuardArray(row["traceLinks"], `${at}.traceLinks`).map((item, index) => parseTraceLink(item, `${at}.traceLinks[${index}]`)),
    escalationContactId: (row["escalationContactId"] === null ? null : architectProgramSnapshotGuardString(row["escalationContactId"], `${at}.escalationContactId`)),
  };
}

export type CostBasis = "capital" | "operational" | "lifecycle" | "replacement" | "maintenance";

export function parseCostBasis(value: unknown, at = "$"): CostBasis {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["capital", "operational", "lifecycle", "replacement", "maintenance"] as const);
}

export interface CostRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly costItem: string;
  readonly basis: CostBasis;
  readonly amount: Binary64Transport | null;
  readonly currency: string;
  readonly quantityBasis: string | null;
  readonly unitCost: Binary64Transport | null;
  readonly contingencyPercent: Binary64Transport | null;
  readonly escalationRate: Binary64Transport | null;
  readonly fundingSource: string | null;
  readonly elementIds: readonly string[];
  readonly requirementIds: readonly string[];
  readonly phase: DeliveryPhase | null;
  readonly cashFlowProfile: readonly string[];
  readonly valueEngineeringNotes: readonly string[];
  readonly benchmarkRef: string | null;
  readonly approvalStatus: ValidationStatus;
  readonly ownerId: string | null;
  readonly assumptions: readonly string[];
  readonly sensitivityFactors: readonly string[];
}

export function parseCostRequirement(value: unknown, at = "$"): CostRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "costItem", "basis", "amount", "currency", "quantityBasis", "unitCost", "contingencyPercent", "escalationRate", "fundingSource", "elementIds", "requirementIds", "phase", "cashFlowProfile", "valueEngineeringNotes", "benchmarkRef", "approvalStatus", "ownerId", "assumptions", "sensitivityFactors"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    costItem: architectProgramSnapshotGuardString(row["costItem"], `${at}.costItem`),
    basis: parseCostBasis(row["basis"], `${at}.basis`),
    amount: (row["amount"] === null ? null : parseBinary64Transport(row["amount"], `${at}.amount`)),
    currency: architectProgramSnapshotGuardString(row["currency"], `${at}.currency`),
    quantityBasis: (row["quantityBasis"] === null ? null : architectProgramSnapshotGuardString(row["quantityBasis"], `${at}.quantityBasis`)),
    unitCost: (row["unitCost"] === null ? null : parseBinary64Transport(row["unitCost"], `${at}.unitCost`)),
    contingencyPercent: (row["contingencyPercent"] === null ? null : parseBinary64Transport(row["contingencyPercent"], `${at}.contingencyPercent`)),
    escalationRate: (row["escalationRate"] === null ? null : parseBinary64Transport(row["escalationRate"], `${at}.escalationRate`)),
    fundingSource: (row["fundingSource"] === null ? null : architectProgramSnapshotGuardString(row["fundingSource"], `${at}.fundingSource`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    requirementIds: architectProgramSnapshotGuardArray(row["requirementIds"], `${at}.requirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.requirementIds[${index}]`)),
    phase: (row["phase"] === null ? null : parseDeliveryPhase(row["phase"], `${at}.phase`)),
    cashFlowProfile: architectProgramSnapshotGuardArray(row["cashFlowProfile"], `${at}.cashFlowProfile`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.cashFlowProfile[${index}]`)),
    valueEngineeringNotes: architectProgramSnapshotGuardArray(row["valueEngineeringNotes"], `${at}.valueEngineeringNotes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.valueEngineeringNotes[${index}]`)),
    benchmarkRef: (row["benchmarkRef"] === null ? null : architectProgramSnapshotGuardString(row["benchmarkRef"], `${at}.benchmarkRef`)),
    approvalStatus: parseValidationStatus(row["approvalStatus"], `${at}.approvalStatus`),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    assumptions: architectProgramSnapshotGuardArray(row["assumptions"], `${at}.assumptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.assumptions[${index}]`)),
    sensitivityFactors: architectProgramSnapshotGuardArray(row["sensitivityFactors"], `${at}.sensitivityFactors`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.sensitivityFactors[${index}]`)),
  };
}

export interface Decision {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly decisionStatement: TextField;
  readonly context: TextField;
  readonly optionsConsidered: readonly string[];
  readonly selectedOptionId: string | null;
  readonly rationale: TextField;
  readonly decisionMakerIds: readonly string[];
  readonly consultedIds: readonly string[];
  readonly informedIds: readonly string[];
  readonly decisionDate: string | null;
  readonly effectiveDate: string | null;
  readonly reversalConditions: readonly string[];
  readonly impactedRequirementIds: readonly string[];
  readonly impactedElementIds: readonly string[];
  readonly costImpact: Binary64Transport | null;
  readonly scheduleImpact: string | null;
  readonly riskImpact: readonly string[];
  readonly approvalStatus: ValidationStatus;
  readonly meetingRef: string | null;
  readonly artifactRefs: readonly string[];
}

export function parseDecision(value: unknown, at = "$"): Decision {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "decisionStatement", "context", "optionsConsidered", "selectedOptionId", "rationale", "decisionMakerIds", "consultedIds", "informedIds", "decisionDate", "effectiveDate", "reversalConditions", "impactedRequirementIds", "impactedElementIds", "costImpact", "scheduleImpact", "riskImpact", "approvalStatus", "meetingRef", "artifactRefs"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    decisionStatement: parseTextField(row["decisionStatement"], `${at}.decisionStatement`),
    context: parseTextField(row["context"], `${at}.context`),
    optionsConsidered: architectProgramSnapshotGuardArray(row["optionsConsidered"], `${at}.optionsConsidered`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.optionsConsidered[${index}]`)),
    selectedOptionId: (row["selectedOptionId"] === null ? null : architectProgramSnapshotGuardString(row["selectedOptionId"], `${at}.selectedOptionId`)),
    rationale: parseTextField(row["rationale"], `${at}.rationale`),
    decisionMakerIds: architectProgramSnapshotGuardArray(row["decisionMakerIds"], `${at}.decisionMakerIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decisionMakerIds[${index}]`)),
    consultedIds: architectProgramSnapshotGuardArray(row["consultedIds"], `${at}.consultedIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.consultedIds[${index}]`)),
    informedIds: architectProgramSnapshotGuardArray(row["informedIds"], `${at}.informedIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.informedIds[${index}]`)),
    decisionDate: (row["decisionDate"] === null ? null : architectProgramSnapshotGuardString(row["decisionDate"], `${at}.decisionDate`)),
    effectiveDate: (row["effectiveDate"] === null ? null : architectProgramSnapshotGuardString(row["effectiveDate"], `${at}.effectiveDate`)),
    reversalConditions: architectProgramSnapshotGuardArray(row["reversalConditions"], `${at}.reversalConditions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.reversalConditions[${index}]`)),
    impactedRequirementIds: architectProgramSnapshotGuardArray(row["impactedRequirementIds"], `${at}.impactedRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.impactedRequirementIds[${index}]`)),
    impactedElementIds: architectProgramSnapshotGuardArray(row["impactedElementIds"], `${at}.impactedElementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.impactedElementIds[${index}]`)),
    costImpact: (row["costImpact"] === null ? null : parseBinary64Transport(row["costImpact"], `${at}.costImpact`)),
    scheduleImpact: (row["scheduleImpact"] === null ? null : architectProgramSnapshotGuardString(row["scheduleImpact"], `${at}.scheduleImpact`)),
    riskImpact: architectProgramSnapshotGuardArray(row["riskImpact"], `${at}.riskImpact`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.riskImpact[${index}]`)),
    approvalStatus: parseValidationStatus(row["approvalStatus"], `${at}.approvalStatus`),
    meetingRef: (row["meetingRef"] === null ? null : architectProgramSnapshotGuardString(row["meetingRef"], `${at}.meetingRef`)),
    artifactRefs: architectProgramSnapshotGuardArray(row["artifactRefs"], `${at}.artifactRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.artifactRefs[${index}]`)),
  };
}

export interface DeliveryConstraint {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly constraintType: string;
  readonly constraintDetails: TextField;
  readonly phase: DeliveryPhase;
  readonly hardDeadline: string | null;
  readonly softDeadline: string | null;
  readonly impactedElementIds: readonly string[];
  readonly impactedRequirementIds: readonly string[];
  readonly workHours: string | null;
  readonly noiseRestrictions: readonly string[];
  readonly accessRestrictions: readonly string[];
  readonly siteLogistics: readonly string[];
  readonly procurementLeadTime: string | null;
  readonly approvalGates: readonly string[];
  readonly occupancyConstraints: readonly string[];
  readonly weatherWindows: readonly string[];
  readonly penaltyClauses: readonly string[];
  readonly mitigationOptions: readonly string[];
  readonly ownerId: string | null;
  readonly riskIds: readonly string[];
  readonly constraintStatus: LifecycleStatus;
}

export function parseDeliveryConstraint(value: unknown, at = "$"): DeliveryConstraint {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "constraintType", "constraintDetails", "phase", "hardDeadline", "softDeadline", "impactedElementIds", "impactedRequirementIds", "workHours", "noiseRestrictions", "accessRestrictions", "siteLogistics", "procurementLeadTime", "approvalGates", "occupancyConstraints", "weatherWindows", "penaltyClauses", "mitigationOptions", "ownerId", "riskIds", "constraintStatus"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    constraintType: architectProgramSnapshotGuardString(row["constraintType"], `${at}.constraintType`),
    constraintDetails: parseTextField(row["constraintDetails"], `${at}.constraintDetails`),
    phase: parseDeliveryPhase(row["phase"], `${at}.phase`),
    hardDeadline: (row["hardDeadline"] === null ? null : architectProgramSnapshotGuardString(row["hardDeadline"], `${at}.hardDeadline`)),
    softDeadline: (row["softDeadline"] === null ? null : architectProgramSnapshotGuardString(row["softDeadline"], `${at}.softDeadline`)),
    impactedElementIds: architectProgramSnapshotGuardArray(row["impactedElementIds"], `${at}.impactedElementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.impactedElementIds[${index}]`)),
    impactedRequirementIds: architectProgramSnapshotGuardArray(row["impactedRequirementIds"], `${at}.impactedRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.impactedRequirementIds[${index}]`)),
    workHours: (row["workHours"] === null ? null : architectProgramSnapshotGuardString(row["workHours"], `${at}.workHours`)),
    noiseRestrictions: architectProgramSnapshotGuardArray(row["noiseRestrictions"], `${at}.noiseRestrictions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.noiseRestrictions[${index}]`)),
    accessRestrictions: architectProgramSnapshotGuardArray(row["accessRestrictions"], `${at}.accessRestrictions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.accessRestrictions[${index}]`)),
    siteLogistics: architectProgramSnapshotGuardArray(row["siteLogistics"], `${at}.siteLogistics`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.siteLogistics[${index}]`)),
    procurementLeadTime: (row["procurementLeadTime"] === null ? null : architectProgramSnapshotGuardString(row["procurementLeadTime"], `${at}.procurementLeadTime`)),
    approvalGates: architectProgramSnapshotGuardArray(row["approvalGates"], `${at}.approvalGates`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.approvalGates[${index}]`)),
    occupancyConstraints: architectProgramSnapshotGuardArray(row["occupancyConstraints"], `${at}.occupancyConstraints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.occupancyConstraints[${index}]`)),
    weatherWindows: architectProgramSnapshotGuardArray(row["weatherWindows"], `${at}.weatherWindows`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.weatherWindows[${index}]`)),
    penaltyClauses: architectProgramSnapshotGuardArray(row["penaltyClauses"], `${at}.penaltyClauses`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.penaltyClauses[${index}]`)),
    mitigationOptions: architectProgramSnapshotGuardArray(row["mitigationOptions"], `${at}.mitigationOptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.mitigationOptions[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    riskIds: architectProgramSnapshotGuardArray(row["riskIds"], `${at}.riskIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.riskIds[${index}]`)),
    constraintStatus: parseLifecycleStatus(row["constraintStatus"], `${at}.constraintStatus`),
  };
}

export type DeliveryPhase = "concept" | "schematic" | "designDevelopment" | "constructionDocuments" | "procurement" | "construction" | "commissioning" | "occupancy";

export function parseDeliveryPhase(value: unknown, at = "$"): DeliveryPhase {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["concept", "schematic", "designDevelopment", "constructionDocuments", "procurement", "construction", "commissioning", "occupancy"] as const);
}

export type EngagementLevel = "unaware" | "resistant" | "neutral" | "supportive" | "leading";

export function parseEngagementLevel(value: unknown, at = "$"): EngagementLevel {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["unaware", "resistant", "neutral", "supportive", "leading"] as const);
}

export interface EntityHeader {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
}

export function parseEntityHeader(value: unknown, at = "$"): EntityHeader {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
  };
}

export type EnvironmentalParameter = "temperature" | "humidity" | "airQuality" | "lighting" | "acoustics" | "ventilation" | "radiation" | "vibration" | "pressure" | "iaq";

export function parseEnvironmentalParameter(value: unknown, at = "$"): EnvironmentalParameter {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["temperature", "humidity", "airQuality", "lighting", "acoustics", "ventilation", "radiation", "vibration", "pressure", "iaq"] as const);
}

export interface EnvironmentalRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly parameterKind: EnvironmentalParameter;
  readonly parameter: string;
  readonly targetValue: Binary64Transport | null;
  readonly unit: string | null;
  readonly minValue: Binary64Transport | null;
  readonly maxValue: Binary64Transport | null;
  readonly comfortBand: string | null;
  readonly measurementMethod: string | null;
  readonly monitoringFrequency: string | null;
  readonly elementIds: readonly string[];
  readonly occupancyBasis: string | null;
  readonly seasonalVariation: readonly string[];
  readonly energyImplications: readonly string[];
  readonly standards: readonly string[];
  readonly certificationTargets: readonly string[];
  readonly outdoorConditions: readonly string[];
  readonly ventilationStrategy: string | null;
  readonly daylightTarget: string | null;
  readonly acousticTarget: string | null;
  readonly iaqTarget: string | null;
  readonly verificationPlan: string | null;
}

export function parseEnvironmentalRequirement(value: unknown, at = "$"): EnvironmentalRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "parameterKind", "parameter", "targetValue", "unit", "minValue", "maxValue", "comfortBand", "measurementMethod", "monitoringFrequency", "elementIds", "occupancyBasis", "seasonalVariation", "energyImplications", "standards", "certificationTargets", "outdoorConditions", "ventilationStrategy", "daylightTarget", "acousticTarget", "iaqTarget", "verificationPlan"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    parameterKind: parseEnvironmentalParameter(row["parameterKind"], `${at}.parameterKind`),
    parameter: architectProgramSnapshotGuardString(row["parameter"], `${at}.parameter`),
    targetValue: (row["targetValue"] === null ? null : parseBinary64Transport(row["targetValue"], `${at}.targetValue`)),
    unit: (row["unit"] === null ? null : architectProgramSnapshotGuardString(row["unit"], `${at}.unit`)),
    minValue: (row["minValue"] === null ? null : parseBinary64Transport(row["minValue"], `${at}.minValue`)),
    maxValue: (row["maxValue"] === null ? null : parseBinary64Transport(row["maxValue"], `${at}.maxValue`)),
    comfortBand: (row["comfortBand"] === null ? null : architectProgramSnapshotGuardString(row["comfortBand"], `${at}.comfortBand`)),
    measurementMethod: (row["measurementMethod"] === null ? null : architectProgramSnapshotGuardString(row["measurementMethod"], `${at}.measurementMethod`)),
    monitoringFrequency: (row["monitoringFrequency"] === null ? null : architectProgramSnapshotGuardString(row["monitoringFrequency"], `${at}.monitoringFrequency`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    occupancyBasis: (row["occupancyBasis"] === null ? null : architectProgramSnapshotGuardString(row["occupancyBasis"], `${at}.occupancyBasis`)),
    seasonalVariation: architectProgramSnapshotGuardArray(row["seasonalVariation"], `${at}.seasonalVariation`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.seasonalVariation[${index}]`)),
    energyImplications: architectProgramSnapshotGuardArray(row["energyImplications"], `${at}.energyImplications`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.energyImplications[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    certificationTargets: architectProgramSnapshotGuardArray(row["certificationTargets"], `${at}.certificationTargets`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.certificationTargets[${index}]`)),
    outdoorConditions: architectProgramSnapshotGuardArray(row["outdoorConditions"], `${at}.outdoorConditions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.outdoorConditions[${index}]`)),
    ventilationStrategy: (row["ventilationStrategy"] === null ? null : architectProgramSnapshotGuardString(row["ventilationStrategy"], `${at}.ventilationStrategy`)),
    daylightTarget: (row["daylightTarget"] === null ? null : architectProgramSnapshotGuardString(row["daylightTarget"], `${at}.daylightTarget`)),
    acousticTarget: (row["acousticTarget"] === null ? null : architectProgramSnapshotGuardString(row["acousticTarget"], `${at}.acousticTarget`)),
    iaqTarget: (row["iaqTarget"] === null ? null : architectProgramSnapshotGuardString(row["iaqTarget"], `${at}.iaqTarget`)),
    verificationPlan: (row["verificationPlan"] === null ? null : architectProgramSnapshotGuardString(row["verificationPlan"], `${at}.verificationPlan`)),
  };
}

export interface Equipment {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly code: string;
  readonly category: string;
  readonly manufacturer: string | null;
  readonly model: string | null;
  readonly quantity: QuantitySpec;
  readonly dimensions: string | null;
  readonly weightKg: Binary64Transport | null;
  readonly powerKw: Binary64Transport | null;
  readonly utilityConnections: readonly string[];
  readonly ventilation: string | null;
  readonly noiseLevelDb: Binary64Transport | null;
  readonly clearance: string | null;
  readonly mounting: string | null;
  readonly elementIds: readonly string[];
  readonly activityIds: readonly string[];
  readonly maintenanceAccess: readonly string[];
  readonly lifecycleYears: number | null;
  readonly replacementCost: Binary64Transport | null;
  readonly standards: readonly string[];
  readonly supplier: string | null;
  readonly activityLinkIds: readonly string[];
  readonly installationRequirements: readonly string[];
  readonly commissioningNotes: readonly string[];
  readonly spareParts: readonly string[];
}

export function parseEquipment(value: unknown, at = "$"): Equipment {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "code", "category", "manufacturer", "model", "quantity", "dimensions", "weightKg", "powerKw", "utilityConnections", "ventilation", "noiseLevelDb", "clearance", "mounting", "elementIds", "activityIds", "maintenanceAccess", "lifecycleYears", "replacementCost", "standards", "supplier", "activityLinkIds", "installationRequirements", "commissioningNotes", "spareParts"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    category: architectProgramSnapshotGuardString(row["category"], `${at}.category`),
    manufacturer: (row["manufacturer"] === null ? null : architectProgramSnapshotGuardString(row["manufacturer"], `${at}.manufacturer`)),
    model: (row["model"] === null ? null : architectProgramSnapshotGuardString(row["model"], `${at}.model`)),
    quantity: parseQuantitySpec(row["quantity"], `${at}.quantity`),
    dimensions: (row["dimensions"] === null ? null : architectProgramSnapshotGuardString(row["dimensions"], `${at}.dimensions`)),
    weightKg: (row["weightKg"] === null ? null : parseBinary64Transport(row["weightKg"], `${at}.weightKg`)),
    powerKw: (row["powerKw"] === null ? null : parseBinary64Transport(row["powerKw"], `${at}.powerKw`)),
    utilityConnections: architectProgramSnapshotGuardArray(row["utilityConnections"], `${at}.utilityConnections`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.utilityConnections[${index}]`)),
    ventilation: (row["ventilation"] === null ? null : architectProgramSnapshotGuardString(row["ventilation"], `${at}.ventilation`)),
    noiseLevelDb: (row["noiseLevelDb"] === null ? null : parseBinary64Transport(row["noiseLevelDb"], `${at}.noiseLevelDb`)),
    clearance: (row["clearance"] === null ? null : architectProgramSnapshotGuardString(row["clearance"], `${at}.clearance`)),
    mounting: (row["mounting"] === null ? null : architectProgramSnapshotGuardString(row["mounting"], `${at}.mounting`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    activityIds: architectProgramSnapshotGuardArray(row["activityIds"], `${at}.activityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.activityIds[${index}]`)),
    maintenanceAccess: architectProgramSnapshotGuardArray(row["maintenanceAccess"], `${at}.maintenanceAccess`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.maintenanceAccess[${index}]`)),
    lifecycleYears: (row["lifecycleYears"] === null ? null : architectProgramSnapshotGuardInteger(row["lifecycleYears"], `${at}.lifecycleYears`, {"minimum": 0, "maximum": 4294967295})),
    replacementCost: (row["replacementCost"] === null ? null : parseBinary64Transport(row["replacementCost"], `${at}.replacementCost`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    supplier: (row["supplier"] === null ? null : architectProgramSnapshotGuardString(row["supplier"], `${at}.supplier`)),
    activityLinkIds: architectProgramSnapshotGuardArray(row["activityLinkIds"], `${at}.activityLinkIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.activityLinkIds[${index}]`)),
    installationRequirements: architectProgramSnapshotGuardArray(row["installationRequirements"], `${at}.installationRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.installationRequirements[${index}]`)),
    commissioningNotes: architectProgramSnapshotGuardArray(row["commissioningNotes"], `${at}.commissioningNotes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.commissioningNotes[${index}]`)),
    spareParts: architectProgramSnapshotGuardArray(row["spareParts"], `${at}.spareParts`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.spareParts[${index}]`)),
  };
}

export interface FlexibilityRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly flexibilityType: string;
  readonly elementIds: readonly string[];
  readonly adaptationScenarios: readonly string[];
  readonly modularityLevel: string | null;
  readonly reconfigurationTime: string | null;
  readonly costOfChange: Binary64Transport | null;
  readonly technologyReadiness: string | null;
  readonly futureFunctionIds: readonly string[];
  readonly demountablePartitions: boolean;
  readonly raisedFloor: boolean;
  readonly overheadServices: boolean;
  readonly expansionDirection: readonly string[];
  readonly contractionScenario: readonly string[];
  readonly multiUsePotential: readonly string[];
  readonly furnitureStrategy: readonly string[];
  readonly infrastructureSpareCapacity: readonly string[];
  readonly leaseImplications: readonly string[];
  readonly ownerId: string | null;
}

export function parseFlexibilityRequirement(value: unknown, at = "$"): FlexibilityRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "flexibilityType", "elementIds", "adaptationScenarios", "modularityLevel", "reconfigurationTime", "costOfChange", "technologyReadiness", "futureFunctionIds", "demountablePartitions", "raisedFloor", "overheadServices", "expansionDirection", "contractionScenario", "multiUsePotential", "furnitureStrategy", "infrastructureSpareCapacity", "leaseImplications", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    flexibilityType: architectProgramSnapshotGuardString(row["flexibilityType"], `${at}.flexibilityType`),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    adaptationScenarios: architectProgramSnapshotGuardArray(row["adaptationScenarios"], `${at}.adaptationScenarios`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.adaptationScenarios[${index}]`)),
    modularityLevel: (row["modularityLevel"] === null ? null : architectProgramSnapshotGuardString(row["modularityLevel"], `${at}.modularityLevel`)),
    reconfigurationTime: (row["reconfigurationTime"] === null ? null : architectProgramSnapshotGuardString(row["reconfigurationTime"], `${at}.reconfigurationTime`)),
    costOfChange: (row["costOfChange"] === null ? null : parseBinary64Transport(row["costOfChange"], `${at}.costOfChange`)),
    technologyReadiness: (row["technologyReadiness"] === null ? null : architectProgramSnapshotGuardString(row["technologyReadiness"], `${at}.technologyReadiness`)),
    futureFunctionIds: architectProgramSnapshotGuardArray(row["futureFunctionIds"], `${at}.futureFunctionIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.futureFunctionIds[${index}]`)),
    demountablePartitions: architectProgramSnapshotGuardBoolean(row["demountablePartitions"], `${at}.demountablePartitions`),
    raisedFloor: architectProgramSnapshotGuardBoolean(row["raisedFloor"], `${at}.raisedFloor`),
    overheadServices: architectProgramSnapshotGuardBoolean(row["overheadServices"], `${at}.overheadServices`),
    expansionDirection: architectProgramSnapshotGuardArray(row["expansionDirection"], `${at}.expansionDirection`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.expansionDirection[${index}]`)),
    contractionScenario: architectProgramSnapshotGuardArray(row["contractionScenario"], `${at}.contractionScenario`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.contractionScenario[${index}]`)),
    multiUsePotential: architectProgramSnapshotGuardArray(row["multiUsePotential"], `${at}.multiUsePotential`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.multiUsePotential[${index}]`)),
    furnitureStrategy: architectProgramSnapshotGuardArray(row["furnitureStrategy"], `${at}.furnitureStrategy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.furnitureStrategy[${index}]`)),
    infrastructureSpareCapacity: architectProgramSnapshotGuardArray(row["infrastructureSpareCapacity"], `${at}.infrastructureSpareCapacity`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.infrastructureSpareCapacity[${index}]`)),
    leaseImplications: architectProgramSnapshotGuardArray(row["leaseImplications"], `${at}.leaseImplications`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.leaseImplications[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export type FlowDirection = "oneWay" | "twoWay" | "bidirectionalPeak" | "restricted";

export function parseFlowDirection(value: unknown, at = "$"): FlowDirection {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["oneWay", "twoWay", "bidirectionalPeak", "restricted"] as const);
}

export type FlowKind = "people" | "material" | "information" | "service" | "equipment" | "waste" | "emergency" | "vehicle";

export function parseFlowKind(value: unknown, at = "$"): FlowKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["people", "material", "information", "service", "equipment", "waste", "emergency", "vehicle"] as const);
}

export interface FlowRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly fromElementId: string;
  readonly toElementId: string;
  readonly kind: FlowKind;
  readonly flowType: string;
  readonly direction: FlowDirection;
  readonly volume: QuantitySpec;
  readonly peakRate: Binary64Transport | null;
  readonly clearWidthM: Binary64Transport | null;
  readonly clearHeightM: Binary64Transport | null;
  readonly separationRequirements: readonly SeparationKind[];
  readonly accessLevel: AccessLevel;
  readonly timeWindows: readonly string[];
  readonly equipmentClearance: string | null;
  readonly signageRequired: boolean;
  readonly escortRequired: boolean;
  readonly emergencyRoute: boolean;
  readonly barrierFree: boolean;
  readonly monitoringRequired: boolean;
  readonly processId: string | null;
  readonly conflictIds: readonly string[];
  readonly verificationMethod: string | null;
}

export function parseFlowRequirement(value: unknown, at = "$"): FlowRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "fromElementId", "toElementId", "kind", "flowType", "direction", "volume", "peakRate", "clearWidthM", "clearHeightM", "separationRequirements", "accessLevel", "timeWindows", "equipmentClearance", "signageRequired", "escortRequired", "emergencyRoute", "barrierFree", "monitoringRequired", "processId", "conflictIds", "verificationMethod"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    fromElementId: architectProgramSnapshotGuardString(row["fromElementId"], `${at}.fromElementId`),
    toElementId: architectProgramSnapshotGuardString(row["toElementId"], `${at}.toElementId`),
    kind: parseFlowKind(row["kind"], `${at}.kind`),
    flowType: architectProgramSnapshotGuardString(row["flowType"], `${at}.flowType`),
    direction: parseFlowDirection(row["direction"], `${at}.direction`),
    volume: parseQuantitySpec(row["volume"], `${at}.volume`),
    peakRate: (row["peakRate"] === null ? null : parseBinary64Transport(row["peakRate"], `${at}.peakRate`)),
    clearWidthM: (row["clearWidthM"] === null ? null : parseBinary64Transport(row["clearWidthM"], `${at}.clearWidthM`)),
    clearHeightM: (row["clearHeightM"] === null ? null : parseBinary64Transport(row["clearHeightM"], `${at}.clearHeightM`)),
    separationRequirements: architectProgramSnapshotGuardArray(row["separationRequirements"], `${at}.separationRequirements`).map((item, index) => parseSeparationKind(item, `${at}.separationRequirements[${index}]`)),
    accessLevel: parseAccessLevel(row["accessLevel"], `${at}.accessLevel`),
    timeWindows: architectProgramSnapshotGuardArray(row["timeWindows"], `${at}.timeWindows`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.timeWindows[${index}]`)),
    equipmentClearance: (row["equipmentClearance"] === null ? null : architectProgramSnapshotGuardString(row["equipmentClearance"], `${at}.equipmentClearance`)),
    signageRequired: architectProgramSnapshotGuardBoolean(row["signageRequired"], `${at}.signageRequired`),
    escortRequired: architectProgramSnapshotGuardBoolean(row["escortRequired"], `${at}.escortRequired`),
    emergencyRoute: architectProgramSnapshotGuardBoolean(row["emergencyRoute"], `${at}.emergencyRoute`),
    barrierFree: architectProgramSnapshotGuardBoolean(row["barrierFree"], `${at}.barrierFree`),
    monitoringRequired: architectProgramSnapshotGuardBoolean(row["monitoringRequired"], `${at}.monitoringRequired`),
    processId: (row["processId"] === null ? null : architectProgramSnapshotGuardString(row["processId"], `${at}.processId`)),
    conflictIds: architectProgramSnapshotGuardArray(row["conflictIds"], `${at}.conflictIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.conflictIds[${index}]`)),
    verificationMethod: (row["verificationMethod"] === null ? null : architectProgramSnapshotGuardString(row["verificationMethod"], `${at}.verificationMethod`)),
  };
}

export interface Function {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly code: string;
  readonly kind: FunctionKind;
  readonly purpose: TextField;
  readonly criticality: Priority;
  readonly performanceTargets: readonly string[];
  readonly serviceLevel: string | null;
  readonly operatingHours: string | null;
  readonly staffing: QuantitySpec;
  readonly equipmentIds: readonly string[];
  readonly resourceIds: readonly string[];
  readonly activityIds: readonly string[];
  readonly elementIds: readonly string[];
  readonly dependencies: readonly string[];
  readonly interfaces: readonly string[];
  readonly constraints: readonly string[];
  readonly qualityCriteria: readonly string[];
  readonly regulatoryRefs: readonly string[];
  readonly futureChanges: readonly string[];
  readonly ownerStakeholderId: string | null;
  readonly successMetrics: readonly string[];
  readonly hierarchyParentId: string | null;
  readonly conflictIds: readonly string[];
}

export function parseFunction(value: unknown, at = "$"): Function {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "code", "kind", "purpose", "criticality", "performanceTargets", "serviceLevel", "operatingHours", "staffing", "equipmentIds", "resourceIds", "activityIds", "elementIds", "dependencies", "interfaces", "constraints", "qualityCriteria", "regulatoryRefs", "futureChanges", "ownerStakeholderId", "successMetrics", "hierarchyParentId", "conflictIds"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    kind: parseFunctionKind(row["kind"], `${at}.kind`),
    purpose: parseTextField(row["purpose"], `${at}.purpose`),
    criticality: parsePriority(row["criticality"], `${at}.criticality`),
    performanceTargets: architectProgramSnapshotGuardArray(row["performanceTargets"], `${at}.performanceTargets`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.performanceTargets[${index}]`)),
    serviceLevel: (row["serviceLevel"] === null ? null : architectProgramSnapshotGuardString(row["serviceLevel"], `${at}.serviceLevel`)),
    operatingHours: (row["operatingHours"] === null ? null : architectProgramSnapshotGuardString(row["operatingHours"], `${at}.operatingHours`)),
    staffing: parseQuantitySpec(row["staffing"], `${at}.staffing`),
    equipmentIds: architectProgramSnapshotGuardArray(row["equipmentIds"], `${at}.equipmentIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.equipmentIds[${index}]`)),
    resourceIds: architectProgramSnapshotGuardArray(row["resourceIds"], `${at}.resourceIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.resourceIds[${index}]`)),
    activityIds: architectProgramSnapshotGuardArray(row["activityIds"], `${at}.activityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.activityIds[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    dependencies: architectProgramSnapshotGuardArray(row["dependencies"], `${at}.dependencies`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dependencies[${index}]`)),
    interfaces: architectProgramSnapshotGuardArray(row["interfaces"], `${at}.interfaces`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.interfaces[${index}]`)),
    constraints: architectProgramSnapshotGuardArray(row["constraints"], `${at}.constraints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.constraints[${index}]`)),
    qualityCriteria: architectProgramSnapshotGuardArray(row["qualityCriteria"], `${at}.qualityCriteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.qualityCriteria[${index}]`)),
    regulatoryRefs: architectProgramSnapshotGuardArray(row["regulatoryRefs"], `${at}.regulatoryRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryRefs[${index}]`)),
    futureChanges: architectProgramSnapshotGuardArray(row["futureChanges"], `${at}.futureChanges`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.futureChanges[${index}]`)),
    ownerStakeholderId: (row["ownerStakeholderId"] === null ? null : architectProgramSnapshotGuardString(row["ownerStakeholderId"], `${at}.ownerStakeholderId`)),
    successMetrics: architectProgramSnapshotGuardArray(row["successMetrics"], `${at}.successMetrics`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.successMetrics[${index}]`)),
    hierarchyParentId: (row["hierarchyParentId"] === null ? null : architectProgramSnapshotGuardString(row["hierarchyParentId"], `${at}.hierarchyParentId`)),
    conflictIds: architectProgramSnapshotGuardArray(row["conflictIds"], `${at}.conflictIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.conflictIds[${index}]`)),
  };
}

export type FunctionKind = "primary" | "secondary" | "support" | "administrative" | "service" | "technical" | "public" | "private" | "shared" | "restricted" | "temporary" | "future" | "operational" | "circulation";

export function parseFunctionKind(value: unknown, at = "$"): FunctionKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["primary", "secondary", "support", "administrative", "service", "technical", "public", "private", "shared", "restricted", "temporary", "future", "operational", "circulation"] as const);
}

export interface Governance {
  readonly id: string;
  readonly framework: string;
  readonly roles: readonly string[];
  readonly responsibilities: readonly string[];
  readonly approvalMatrix: readonly string[];
  readonly escalationPaths: readonly string[];
  readonly meetingCadence: readonly string[];
  readonly decisionRights: readonly string[];
  readonly changeControlProcess: readonly string[];
  readonly qualityPolicy: TextField;
  readonly riskAppetite: string | null;
  readonly complianceObligations: readonly string[];
  readonly auditSchedule: string | null;
  readonly documentControl: readonly string[];
  readonly stakeholderEngagementPlan: readonly string[];
  readonly ethicsPolicy: readonly string[];
  readonly dataGovernance: readonly string[];
  readonly ownerId: string | null;
  readonly reviewCycle: string | null;
  readonly reviewHierarchy: readonly string[];
  readonly policyOwnershipId: string | null;
  readonly requirementOwnershipId: string | null;
  readonly riskOwnershipId: string | null;
  readonly reportingFrequency: string | null;
  readonly accountabilityRules: readonly string[];
  readonly exceptionManagement: readonly string[];
  readonly governancePerformance: readonly string[];
}

export function parseGovernance(value: unknown, at = "$"): Governance {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "framework", "roles", "responsibilities", "approvalMatrix", "escalationPaths", "meetingCadence", "decisionRights", "changeControlProcess", "qualityPolicy", "riskAppetite", "complianceObligations", "auditSchedule", "documentControl", "stakeholderEngagementPlan", "ethicsPolicy", "dataGovernance", "ownerId", "reviewCycle", "reviewHierarchy", "policyOwnershipId", "requirementOwnershipId", "riskOwnershipId", "reportingFrequency", "accountabilityRules", "exceptionManagement", "governancePerformance"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    framework: architectProgramSnapshotGuardString(row["framework"], `${at}.framework`),
    roles: architectProgramSnapshotGuardArray(row["roles"], `${at}.roles`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.roles[${index}]`)),
    responsibilities: architectProgramSnapshotGuardArray(row["responsibilities"], `${at}.responsibilities`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.responsibilities[${index}]`)),
    approvalMatrix: architectProgramSnapshotGuardArray(row["approvalMatrix"], `${at}.approvalMatrix`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.approvalMatrix[${index}]`)),
    escalationPaths: architectProgramSnapshotGuardArray(row["escalationPaths"], `${at}.escalationPaths`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.escalationPaths[${index}]`)),
    meetingCadence: architectProgramSnapshotGuardArray(row["meetingCadence"], `${at}.meetingCadence`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.meetingCadence[${index}]`)),
    decisionRights: architectProgramSnapshotGuardArray(row["decisionRights"], `${at}.decisionRights`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decisionRights[${index}]`)),
    changeControlProcess: architectProgramSnapshotGuardArray(row["changeControlProcess"], `${at}.changeControlProcess`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.changeControlProcess[${index}]`)),
    qualityPolicy: parseTextField(row["qualityPolicy"], `${at}.qualityPolicy`),
    riskAppetite: (row["riskAppetite"] === null ? null : architectProgramSnapshotGuardString(row["riskAppetite"], `${at}.riskAppetite`)),
    complianceObligations: architectProgramSnapshotGuardArray(row["complianceObligations"], `${at}.complianceObligations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.complianceObligations[${index}]`)),
    auditSchedule: (row["auditSchedule"] === null ? null : architectProgramSnapshotGuardString(row["auditSchedule"], `${at}.auditSchedule`)),
    documentControl: architectProgramSnapshotGuardArray(row["documentControl"], `${at}.documentControl`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.documentControl[${index}]`)),
    stakeholderEngagementPlan: architectProgramSnapshotGuardArray(row["stakeholderEngagementPlan"], `${at}.stakeholderEngagementPlan`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderEngagementPlan[${index}]`)),
    ethicsPolicy: architectProgramSnapshotGuardArray(row["ethicsPolicy"], `${at}.ethicsPolicy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.ethicsPolicy[${index}]`)),
    dataGovernance: architectProgramSnapshotGuardArray(row["dataGovernance"], `${at}.dataGovernance`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dataGovernance[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    reviewCycle: (row["reviewCycle"] === null ? null : architectProgramSnapshotGuardString(row["reviewCycle"], `${at}.reviewCycle`)),
    reviewHierarchy: architectProgramSnapshotGuardArray(row["reviewHierarchy"], `${at}.reviewHierarchy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.reviewHierarchy[${index}]`)),
    policyOwnershipId: (row["policyOwnershipId"] === null ? null : architectProgramSnapshotGuardString(row["policyOwnershipId"], `${at}.policyOwnershipId`)),
    requirementOwnershipId: (row["requirementOwnershipId"] === null ? null : architectProgramSnapshotGuardString(row["requirementOwnershipId"], `${at}.requirementOwnershipId`)),
    riskOwnershipId: (row["riskOwnershipId"] === null ? null : architectProgramSnapshotGuardString(row["riskOwnershipId"], `${at}.riskOwnershipId`)),
    reportingFrequency: (row["reportingFrequency"] === null ? null : architectProgramSnapshotGuardString(row["reportingFrequency"], `${at}.reportingFrequency`)),
    accountabilityRules: architectProgramSnapshotGuardArray(row["accountabilityRules"], `${at}.accountabilityRules`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.accountabilityRules[${index}]`)),
    exceptionManagement: architectProgramSnapshotGuardArray(row["exceptionManagement"], `${at}.exceptionManagement`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.exceptionManagement[${index}]`)),
    governancePerformance: architectProgramSnapshotGuardArray(row["governancePerformance"], `${at}.governancePerformance`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.governancePerformance[${index}]`)),
  };
}

export interface GrowthPlan {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly horizonYears: number;
  readonly growthRate: Binary64Transport | null;
  readonly headcountGrowth: QuantitySpec;
  readonly areaGrowth: QuantitySpec;
  readonly phases: readonly string[];
  readonly triggerEvents: readonly string[];
  readonly expansionElementIds: readonly string[];
  readonly reserveAreas: readonly string[];
  readonly infrastructureHeadroom: readonly string[];
  readonly budgetEnvelope: Binary64Transport | null;
  readonly fundingSources: readonly string[];
  readonly riskFactors: readonly string[];
  readonly decisionPoints: readonly string[];
  readonly scenarioIds: readonly string[];
  readonly decommissionPlan: readonly string[];
  readonly relocationStrategy: readonly string[];
  readonly stakeholderImpact: readonly string[];
  readonly regulatoryConsiderations: readonly string[];
  readonly ownerId: string | null;
}

export function parseGrowthPlan(value: unknown, at = "$"): GrowthPlan {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "horizonYears", "growthRate", "headcountGrowth", "areaGrowth", "phases", "triggerEvents", "expansionElementIds", "reserveAreas", "infrastructureHeadroom", "budgetEnvelope", "fundingSources", "riskFactors", "decisionPoints", "scenarioIds", "decommissionPlan", "relocationStrategy", "stakeholderImpact", "regulatoryConsiderations", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    horizonYears: architectProgramSnapshotGuardInteger(row["horizonYears"], `${at}.horizonYears`, {"minimum": 0, "maximum": 4294967295}),
    growthRate: (row["growthRate"] === null ? null : parseBinary64Transport(row["growthRate"], `${at}.growthRate`)),
    headcountGrowth: parseQuantitySpec(row["headcountGrowth"], `${at}.headcountGrowth`),
    areaGrowth: parseQuantitySpec(row["areaGrowth"], `${at}.areaGrowth`),
    phases: architectProgramSnapshotGuardArray(row["phases"], `${at}.phases`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.phases[${index}]`)),
    triggerEvents: architectProgramSnapshotGuardArray(row["triggerEvents"], `${at}.triggerEvents`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.triggerEvents[${index}]`)),
    expansionElementIds: architectProgramSnapshotGuardArray(row["expansionElementIds"], `${at}.expansionElementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.expansionElementIds[${index}]`)),
    reserveAreas: architectProgramSnapshotGuardArray(row["reserveAreas"], `${at}.reserveAreas`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.reserveAreas[${index}]`)),
    infrastructureHeadroom: architectProgramSnapshotGuardArray(row["infrastructureHeadroom"], `${at}.infrastructureHeadroom`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.infrastructureHeadroom[${index}]`)),
    budgetEnvelope: (row["budgetEnvelope"] === null ? null : parseBinary64Transport(row["budgetEnvelope"], `${at}.budgetEnvelope`)),
    fundingSources: architectProgramSnapshotGuardArray(row["fundingSources"], `${at}.fundingSources`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.fundingSources[${index}]`)),
    riskFactors: architectProgramSnapshotGuardArray(row["riskFactors"], `${at}.riskFactors`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.riskFactors[${index}]`)),
    decisionPoints: architectProgramSnapshotGuardArray(row["decisionPoints"], `${at}.decisionPoints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decisionPoints[${index}]`)),
    scenarioIds: architectProgramSnapshotGuardArray(row["scenarioIds"], `${at}.scenarioIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.scenarioIds[${index}]`)),
    decommissionPlan: architectProgramSnapshotGuardArray(row["decommissionPlan"], `${at}.decommissionPlan`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decommissionPlan[${index}]`)),
    relocationStrategy: architectProgramSnapshotGuardArray(row["relocationStrategy"], `${at}.relocationStrategy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relocationStrategy[${index}]`)),
    stakeholderImpact: architectProgramSnapshotGuardArray(row["stakeholderImpact"], `${at}.stakeholderImpact`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderImpact[${index}]`)),
    regulatoryConsiderations: architectProgramSnapshotGuardArray(row["regulatoryConsiderations"], `${at}.regulatoryConsiderations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryConsiderations[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export type HumanFactorAspect = "ergonomics" | "cognition" | "sensory" | "social" | "cultural" | "behavioral" | "physical" | "psychological" | "fatigue" | "stress";

export function parseHumanFactorAspect(value: unknown, at = "$"): HumanFactorAspect {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["ergonomics", "cognition", "sensory", "social", "cultural", "behavioral", "physical", "psychological", "fatigue", "stress"] as const);
}

export interface HumanFactorRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly aspect: HumanFactorAspect;
  readonly factor: string;
  readonly userProfileIds: readonly string[];
  readonly activityIds: readonly string[];
  readonly ergonomicCriteria: readonly string[];
  readonly cognitiveLoad: string | null;
  readonly visualDemands: readonly string[];
  readonly auditoryDemands: readonly string[];
  readonly postureRequirements: readonly string[];
  readonly reachEnvelope: string | null;
  readonly lightingForTasks: readonly string[];
  readonly thermalComfort: readonly string[];
  readonly privacyNeeds: readonly string[];
  readonly socialInteraction: readonly string[];
  readonly stressFactors: readonly string[];
  readonly mitigationMeasures: readonly string[];
  readonly trainingNeeds: readonly string[];
  readonly standards: readonly string[];
  readonly researchBasis: readonly string[];
  readonly elementIds: readonly string[];
  readonly verificationMethod: string | null;
}

export function parseHumanFactorRequirement(value: unknown, at = "$"): HumanFactorRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "aspect", "factor", "userProfileIds", "activityIds", "ergonomicCriteria", "cognitiveLoad", "visualDemands", "auditoryDemands", "postureRequirements", "reachEnvelope", "lightingForTasks", "thermalComfort", "privacyNeeds", "socialInteraction", "stressFactors", "mitigationMeasures", "trainingNeeds", "standards", "researchBasis", "elementIds", "verificationMethod"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    aspect: parseHumanFactorAspect(row["aspect"], `${at}.aspect`),
    factor: architectProgramSnapshotGuardString(row["factor"], `${at}.factor`),
    userProfileIds: architectProgramSnapshotGuardArray(row["userProfileIds"], `${at}.userProfileIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.userProfileIds[${index}]`)),
    activityIds: architectProgramSnapshotGuardArray(row["activityIds"], `${at}.activityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.activityIds[${index}]`)),
    ergonomicCriteria: architectProgramSnapshotGuardArray(row["ergonomicCriteria"], `${at}.ergonomicCriteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.ergonomicCriteria[${index}]`)),
    cognitiveLoad: (row["cognitiveLoad"] === null ? null : architectProgramSnapshotGuardString(row["cognitiveLoad"], `${at}.cognitiveLoad`)),
    visualDemands: architectProgramSnapshotGuardArray(row["visualDemands"], `${at}.visualDemands`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.visualDemands[${index}]`)),
    auditoryDemands: architectProgramSnapshotGuardArray(row["auditoryDemands"], `${at}.auditoryDemands`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.auditoryDemands[${index}]`)),
    postureRequirements: architectProgramSnapshotGuardArray(row["postureRequirements"], `${at}.postureRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.postureRequirements[${index}]`)),
    reachEnvelope: (row["reachEnvelope"] === null ? null : architectProgramSnapshotGuardString(row["reachEnvelope"], `${at}.reachEnvelope`)),
    lightingForTasks: architectProgramSnapshotGuardArray(row["lightingForTasks"], `${at}.lightingForTasks`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.lightingForTasks[${index}]`)),
    thermalComfort: architectProgramSnapshotGuardArray(row["thermalComfort"], `${at}.thermalComfort`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.thermalComfort[${index}]`)),
    privacyNeeds: architectProgramSnapshotGuardArray(row["privacyNeeds"], `${at}.privacyNeeds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.privacyNeeds[${index}]`)),
    socialInteraction: architectProgramSnapshotGuardArray(row["socialInteraction"], `${at}.socialInteraction`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.socialInteraction[${index}]`)),
    stressFactors: architectProgramSnapshotGuardArray(row["stressFactors"], `${at}.stressFactors`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stressFactors[${index}]`)),
    mitigationMeasures: architectProgramSnapshotGuardArray(row["mitigationMeasures"], `${at}.mitigationMeasures`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.mitigationMeasures[${index}]`)),
    trainingNeeds: architectProgramSnapshotGuardArray(row["trainingNeeds"], `${at}.trainingNeeds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.trainingNeeds[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    researchBasis: architectProgramSnapshotGuardArray(row["researchBasis"], `${at}.researchBasis`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.researchBasis[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    verificationMethod: (row["verificationMethod"] === null ? null : architectProgramSnapshotGuardString(row["verificationMethod"], `${at}.verificationMethod`)),
  };
}

export type InfluenceLevel = "low" | "medium" | "high" | "critical";

export function parseInfluenceLevel(value: unknown, at = "$"): InfluenceLevel {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["low", "medium", "high", "critical"] as const);
}

export interface InformationRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly informationType: string;
  readonly format: string | null;
  readonly sourceSystem: string | null;
  readonly destinationSystems: readonly string[];
  readonly updateFrequency: string | null;
  readonly retentionPeriod: string | null;
  readonly accessControls: readonly string[];
  readonly classification: string | null;
  readonly qualityCriteria: readonly string[];
  readonly metadataRequirements: readonly string[];
  readonly integrationPoints: readonly string[];
  readonly backupRequirements: readonly string[];
  readonly disasterRecovery: readonly string[];
  readonly privacyControls: readonly string[];
  readonly auditTrail: boolean;
  readonly elementIds: readonly string[];
  readonly stakeholderIds: readonly string[];
  readonly standards: readonly string[];
  readonly ownerId: string | null;
}

export function parseInformationRequirement(value: unknown, at = "$"): InformationRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "informationType", "format", "sourceSystem", "destinationSystems", "updateFrequency", "retentionPeriod", "accessControls", "classification", "qualityCriteria", "metadataRequirements", "integrationPoints", "backupRequirements", "disasterRecovery", "privacyControls", "auditTrail", "elementIds", "stakeholderIds", "standards", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    informationType: architectProgramSnapshotGuardString(row["informationType"], `${at}.informationType`),
    format: (row["format"] === null ? null : architectProgramSnapshotGuardString(row["format"], `${at}.format`)),
    sourceSystem: (row["sourceSystem"] === null ? null : architectProgramSnapshotGuardString(row["sourceSystem"], `${at}.sourceSystem`)),
    destinationSystems: architectProgramSnapshotGuardArray(row["destinationSystems"], `${at}.destinationSystems`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.destinationSystems[${index}]`)),
    updateFrequency: (row["updateFrequency"] === null ? null : architectProgramSnapshotGuardString(row["updateFrequency"], `${at}.updateFrequency`)),
    retentionPeriod: (row["retentionPeriod"] === null ? null : architectProgramSnapshotGuardString(row["retentionPeriod"], `${at}.retentionPeriod`)),
    accessControls: architectProgramSnapshotGuardArray(row["accessControls"], `${at}.accessControls`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.accessControls[${index}]`)),
    classification: (row["classification"] === null ? null : architectProgramSnapshotGuardString(row["classification"], `${at}.classification`)),
    qualityCriteria: architectProgramSnapshotGuardArray(row["qualityCriteria"], `${at}.qualityCriteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.qualityCriteria[${index}]`)),
    metadataRequirements: architectProgramSnapshotGuardArray(row["metadataRequirements"], `${at}.metadataRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.metadataRequirements[${index}]`)),
    integrationPoints: architectProgramSnapshotGuardArray(row["integrationPoints"], `${at}.integrationPoints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.integrationPoints[${index}]`)),
    backupRequirements: architectProgramSnapshotGuardArray(row["backupRequirements"], `${at}.backupRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.backupRequirements[${index}]`)),
    disasterRecovery: architectProgramSnapshotGuardArray(row["disasterRecovery"], `${at}.disasterRecovery`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.disasterRecovery[${index}]`)),
    privacyControls: architectProgramSnapshotGuardArray(row["privacyControls"], `${at}.privacyControls`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.privacyControls[${index}]`)),
    auditTrail: architectProgramSnapshotGuardBoolean(row["auditTrail"], `${at}.auditTrail`),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    stakeholderIds: architectProgramSnapshotGuardArray(row["stakeholderIds"], `${at}.stakeholderIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderIds[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export interface InfrastructureRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly system: string;
  readonly category: string;
  readonly capacity: QuantitySpec;
  readonly redundancy: string | null;
  readonly distribution: readonly string[];
  readonly entryPoints: readonly string[];
  readonly utilitySource: string | null;
  readonly standbyPower: boolean;
  readonly monitoring: readonly string[];
  readonly maintenanceAccess: readonly string[];
  readonly standards: readonly string[];
  readonly elementIds: readonly string[];
  readonly peakDemand: Binary64Transport | null;
  readonly diversityFactor: Binary64Transport | null;
  readonly futureExpansion: readonly string[];
  readonly interfaceRequirements: readonly string[];
  readonly commissioning: readonly string[];
  readonly lifecycleCost: Binary64Transport | null;
  readonly ownerId: string | null;
}

export function parseInfrastructureRequirement(value: unknown, at = "$"): InfrastructureRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "system", "category", "capacity", "redundancy", "distribution", "entryPoints", "utilitySource", "standbyPower", "monitoring", "maintenanceAccess", "standards", "elementIds", "peakDemand", "diversityFactor", "futureExpansion", "interfaceRequirements", "commissioning", "lifecycleCost", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    system: architectProgramSnapshotGuardString(row["system"], `${at}.system`),
    category: architectProgramSnapshotGuardString(row["category"], `${at}.category`),
    capacity: parseQuantitySpec(row["capacity"], `${at}.capacity`),
    redundancy: (row["redundancy"] === null ? null : architectProgramSnapshotGuardString(row["redundancy"], `${at}.redundancy`)),
    distribution: architectProgramSnapshotGuardArray(row["distribution"], `${at}.distribution`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.distribution[${index}]`)),
    entryPoints: architectProgramSnapshotGuardArray(row["entryPoints"], `${at}.entryPoints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.entryPoints[${index}]`)),
    utilitySource: (row["utilitySource"] === null ? null : architectProgramSnapshotGuardString(row["utilitySource"], `${at}.utilitySource`)),
    standbyPower: architectProgramSnapshotGuardBoolean(row["standbyPower"], `${at}.standbyPower`),
    monitoring: architectProgramSnapshotGuardArray(row["monitoring"], `${at}.monitoring`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.monitoring[${index}]`)),
    maintenanceAccess: architectProgramSnapshotGuardArray(row["maintenanceAccess"], `${at}.maintenanceAccess`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.maintenanceAccess[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    peakDemand: (row["peakDemand"] === null ? null : parseBinary64Transport(row["peakDemand"], `${at}.peakDemand`)),
    diversityFactor: (row["diversityFactor"] === null ? null : parseBinary64Transport(row["diversityFactor"], `${at}.diversityFactor`)),
    futureExpansion: architectProgramSnapshotGuardArray(row["futureExpansion"], `${at}.futureExpansion`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.futureExpansion[${index}]`)),
    interfaceRequirements: architectProgramSnapshotGuardArray(row["interfaceRequirements"], `${at}.interfaceRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.interfaceRequirements[${index}]`)),
    commissioning: architectProgramSnapshotGuardArray(row["commissioning"], `${at}.commissioning`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.commissioning[${index}]`)),
    lifecycleCost: (row["lifecycleCost"] === null ? null : parseBinary64Transport(row["lifecycleCost"], `${at}.lifecycleCost`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export interface Issue {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly issueType: string;
  readonly summary: TextField;
  readonly issueDescription: TextField;
  readonly severity: IssueSeverity;
  readonly issuePriority: Priority;
  readonly reporterId: string | null;
  readonly assigneeId: string | null;
  readonly affectedEntityIds: readonly string[];
  readonly rootCause: TextField | null;
  readonly resolution: TextField | null;
  readonly workaround: TextField | null;
  readonly dueDate: string | null;
  readonly resolvedDate: string | null;
  readonly relatedConflictIds: readonly string[];
  readonly relatedRiskIds: readonly string[];
  readonly decisionId: string | null;
  readonly comments: readonly TaggedNote[];
  readonly attachments: readonly string[];
  readonly escalationLevel: string | null;
}

export function parseIssue(value: unknown, at = "$"): Issue {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "issueType", "summary", "issueDescription", "severity", "issuePriority", "reporterId", "assigneeId", "affectedEntityIds", "rootCause", "resolution", "workaround", "dueDate", "resolvedDate", "relatedConflictIds", "relatedRiskIds", "decisionId", "comments", "attachments", "escalationLevel"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    issueType: architectProgramSnapshotGuardString(row["issueType"], `${at}.issueType`),
    summary: parseTextField(row["summary"], `${at}.summary`),
    issueDescription: parseTextField(row["issueDescription"], `${at}.issueDescription`),
    severity: parseIssueSeverity(row["severity"], `${at}.severity`),
    issuePriority: parsePriority(row["issuePriority"], `${at}.issuePriority`),
    reporterId: (row["reporterId"] === null ? null : architectProgramSnapshotGuardString(row["reporterId"], `${at}.reporterId`)),
    assigneeId: (row["assigneeId"] === null ? null : architectProgramSnapshotGuardString(row["assigneeId"], `${at}.assigneeId`)),
    affectedEntityIds: architectProgramSnapshotGuardArray(row["affectedEntityIds"], `${at}.affectedEntityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.affectedEntityIds[${index}]`)),
    rootCause: (row["rootCause"] === null ? null : parseTextField(row["rootCause"], `${at}.rootCause`)),
    resolution: (row["resolution"] === null ? null : parseTextField(row["resolution"], `${at}.resolution`)),
    workaround: (row["workaround"] === null ? null : parseTextField(row["workaround"], `${at}.workaround`)),
    dueDate: (row["dueDate"] === null ? null : architectProgramSnapshotGuardString(row["dueDate"], `${at}.dueDate`)),
    resolvedDate: (row["resolvedDate"] === null ? null : architectProgramSnapshotGuardString(row["resolvedDate"], `${at}.resolvedDate`)),
    relatedConflictIds: architectProgramSnapshotGuardArray(row["relatedConflictIds"], `${at}.relatedConflictIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedConflictIds[${index}]`)),
    relatedRiskIds: architectProgramSnapshotGuardArray(row["relatedRiskIds"], `${at}.relatedRiskIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedRiskIds[${index}]`)),
    decisionId: (row["decisionId"] === null ? null : architectProgramSnapshotGuardString(row["decisionId"], `${at}.decisionId`)),
    comments: architectProgramSnapshotGuardArray(row["comments"], `${at}.comments`).map((item, index) => parseTaggedNote(item, `${at}.comments[${index}]`)),
    attachments: architectProgramSnapshotGuardArray(row["attachments"], `${at}.attachments`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.attachments[${index}]`)),
    escalationLevel: (row["escalationLevel"] === null ? null : architectProgramSnapshotGuardString(row["escalationLevel"], `${at}.escalationLevel`)),
  };
}

export type IssueSeverity = "cosmetic" | "minor" | "major" | "critical" | "blocker";

export function parseIssueSeverity(value: unknown, at = "$"): IssueSeverity {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["cosmetic", "minor", "major", "critical", "blocker"] as const);
}

export interface KnowledgeRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly topic: string;
  readonly category: string;
  readonly summary: TextField;
  readonly content: TextField;
  readonly sources: readonly string[];
  readonly references: readonly string[];
  readonly lessonsLearned: readonly string[];
  readonly bestPractices: readonly string[];
  readonly applicableSectors: readonly string[];
  readonly relatedEntityKinds: readonly string[];
  readonly authorIds: readonly string[];
  readonly expertiseLevel: string | null;
  readonly validationStatus: ValidationStatus;
  readonly lastReviewed: string | null;
  readonly keywords: readonly string[];
  readonly attachments: readonly string[];
  readonly citations: readonly string[];
  readonly usageCount: Unsigned64Transport;
}

export function parseKnowledgeRecord(value: unknown, at = "$"): KnowledgeRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "topic", "category", "summary", "content", "sources", "references", "lessonsLearned", "bestPractices", "applicableSectors", "relatedEntityKinds", "authorIds", "expertiseLevel", "validationStatus", "lastReviewed", "keywords", "attachments", "citations", "usageCount"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    topic: architectProgramSnapshotGuardString(row["topic"], `${at}.topic`),
    category: architectProgramSnapshotGuardString(row["category"], `${at}.category`),
    summary: parseTextField(row["summary"], `${at}.summary`),
    content: parseTextField(row["content"], `${at}.content`),
    sources: architectProgramSnapshotGuardArray(row["sources"], `${at}.sources`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.sources[${index}]`)),
    references: architectProgramSnapshotGuardArray(row["references"], `${at}.references`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.references[${index}]`)),
    lessonsLearned: architectProgramSnapshotGuardArray(row["lessonsLearned"], `${at}.lessonsLearned`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.lessonsLearned[${index}]`)),
    bestPractices: architectProgramSnapshotGuardArray(row["bestPractices"], `${at}.bestPractices`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.bestPractices[${index}]`)),
    applicableSectors: architectProgramSnapshotGuardArray(row["applicableSectors"], `${at}.applicableSectors`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.applicableSectors[${index}]`)),
    relatedEntityKinds: architectProgramSnapshotGuardArray(row["relatedEntityKinds"], `${at}.relatedEntityKinds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedEntityKinds[${index}]`)),
    authorIds: architectProgramSnapshotGuardArray(row["authorIds"], `${at}.authorIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.authorIds[${index}]`)),
    expertiseLevel: (row["expertiseLevel"] === null ? null : architectProgramSnapshotGuardString(row["expertiseLevel"], `${at}.expertiseLevel`)),
    validationStatus: parseValidationStatus(row["validationStatus"], `${at}.validationStatus`),
    lastReviewed: (row["lastReviewed"] === null ? null : architectProgramSnapshotGuardString(row["lastReviewed"], `${at}.lastReviewed`)),
    keywords: architectProgramSnapshotGuardArray(row["keywords"], `${at}.keywords`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.keywords[${index}]`)),
    attachments: architectProgramSnapshotGuardArray(row["attachments"], `${at}.attachments`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.attachments[${index}]`)),
    citations: architectProgramSnapshotGuardArray(row["citations"], `${at}.citations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.citations[${index}]`)),
    usageCount: parseUnsigned64Transport(row["usageCount"], `${at}.usageCount`),
  };
}

export type LifecycleStatus = "draft" | "proposed" | "underReview" | "validated" | "approved" | "rejected" | "deferred" | "superseded" | "archived" | "open" | "closed" | "atRisk" | "blocked" | "inProgress" | "complete";

export function parseLifecycleStatus(value: unknown, at = "$"): LifecycleStatus {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["draft", "proposed", "underReview", "validated", "approved", "rejected", "deferred", "superseded", "archived", "open", "closed", "atRisk", "blocked", "inProgress", "complete"] as const);
}

export interface MeetingRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly meetingType: string;
  readonly scheduledDate: string | null;
  readonly duration: string | null;
  readonly location: string | null;
  readonly chairId: string | null;
  readonly attendeeIds: readonly string[];
  readonly agendaItems: readonly string[];
  readonly minutes: TextField | null;
  readonly actionItems: readonly string[];
  readonly decisionsMade: readonly string[];
  readonly artifactRefs: readonly string[];
  readonly followUpDate: string | null;
  readonly recordingRef: string | null;
  readonly quorumMet: boolean;
  readonly meetingStatus: LifecycleStatus;
  readonly workshopId: string | null;
  readonly stakeholderIds: readonly string[];
  readonly requirementIds: readonly string[];
  readonly issueIds: readonly string[];
  readonly approvalIds: readonly string[];
}

export function parseMeetingRecord(value: unknown, at = "$"): MeetingRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "meetingType", "scheduledDate", "duration", "location", "chairId", "attendeeIds", "agendaItems", "minutes", "actionItems", "decisionsMade", "artifactRefs", "followUpDate", "recordingRef", "quorumMet", "meetingStatus", "workshopId", "stakeholderIds", "requirementIds", "issueIds", "approvalIds"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    meetingType: architectProgramSnapshotGuardString(row["meetingType"], `${at}.meetingType`),
    scheduledDate: (row["scheduledDate"] === null ? null : architectProgramSnapshotGuardString(row["scheduledDate"], `${at}.scheduledDate`)),
    duration: (row["duration"] === null ? null : architectProgramSnapshotGuardString(row["duration"], `${at}.duration`)),
    location: (row["location"] === null ? null : architectProgramSnapshotGuardString(row["location"], `${at}.location`)),
    chairId: (row["chairId"] === null ? null : architectProgramSnapshotGuardString(row["chairId"], `${at}.chairId`)),
    attendeeIds: architectProgramSnapshotGuardArray(row["attendeeIds"], `${at}.attendeeIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.attendeeIds[${index}]`)),
    agendaItems: architectProgramSnapshotGuardArray(row["agendaItems"], `${at}.agendaItems`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.agendaItems[${index}]`)),
    minutes: (row["minutes"] === null ? null : parseTextField(row["minutes"], `${at}.minutes`)),
    actionItems: architectProgramSnapshotGuardArray(row["actionItems"], `${at}.actionItems`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.actionItems[${index}]`)),
    decisionsMade: architectProgramSnapshotGuardArray(row["decisionsMade"], `${at}.decisionsMade`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decisionsMade[${index}]`)),
    artifactRefs: architectProgramSnapshotGuardArray(row["artifactRefs"], `${at}.artifactRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.artifactRefs[${index}]`)),
    followUpDate: (row["followUpDate"] === null ? null : architectProgramSnapshotGuardString(row["followUpDate"], `${at}.followUpDate`)),
    recordingRef: (row["recordingRef"] === null ? null : architectProgramSnapshotGuardString(row["recordingRef"], `${at}.recordingRef`)),
    quorumMet: architectProgramSnapshotGuardBoolean(row["quorumMet"], `${at}.quorumMet`),
    meetingStatus: parseLifecycleStatus(row["meetingStatus"], `${at}.meetingStatus`),
    workshopId: (row["workshopId"] === null ? null : architectProgramSnapshotGuardString(row["workshopId"], `${at}.workshopId`)),
    stakeholderIds: architectProgramSnapshotGuardArray(row["stakeholderIds"], `${at}.stakeholderIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderIds[${index}]`)),
    requirementIds: architectProgramSnapshotGuardArray(row["requirementIds"], `${at}.requirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.requirementIds[${index}]`)),
    issueIds: architectProgramSnapshotGuardArray(row["issueIds"], `${at}.issueIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.issueIds[${index}]`)),
    approvalIds: architectProgramSnapshotGuardArray(row["approvalIds"], `${at}.approvalIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.approvalIds[${index}]`)),
  };
}

export interface OperationalRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly operation: string;
  readonly serviceLevel: string | null;
  readonly operatingHours: string | null;
  readonly staffing: QuantitySpec;
  readonly maintenanceInterval: string | null;
  readonly cleaningRegime: string | null;
  readonly turnaroundTime: string | null;
  readonly redundancy: string | null;
  readonly uptimeTarget: Binary64Transport | null;
  readonly responseTime: string | null;
  readonly equipmentIds: readonly string[];
  readonly elementIds: readonly string[];
  readonly processIds: readonly string[];
  readonly utilities: readonly string[];
  readonly wasteStreams: readonly string[];
  readonly contingencyPlan: readonly string[];
  readonly trainingRequirements: readonly string[];
  readonly sopReferences: readonly string[];
  readonly kpiTargets: readonly string[];
  readonly ownerId: string | null;
  readonly serviceCategory: string | null;
  readonly shiftPattern: string | null;
  readonly slaTarget: string | null;
  readonly escalationContactId: string | null;
}

export function parseOperationalRequirement(value: unknown, at = "$"): OperationalRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "operation", "serviceLevel", "operatingHours", "staffing", "maintenanceInterval", "cleaningRegime", "turnaroundTime", "redundancy", "uptimeTarget", "responseTime", "equipmentIds", "elementIds", "processIds", "utilities", "wasteStreams", "contingencyPlan", "trainingRequirements", "sopReferences", "kpiTargets", "ownerId", "serviceCategory", "shiftPattern", "slaTarget", "escalationContactId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    operation: architectProgramSnapshotGuardString(row["operation"], `${at}.operation`),
    serviceLevel: (row["serviceLevel"] === null ? null : architectProgramSnapshotGuardString(row["serviceLevel"], `${at}.serviceLevel`)),
    operatingHours: (row["operatingHours"] === null ? null : architectProgramSnapshotGuardString(row["operatingHours"], `${at}.operatingHours`)),
    staffing: parseQuantitySpec(row["staffing"], `${at}.staffing`),
    maintenanceInterval: (row["maintenanceInterval"] === null ? null : architectProgramSnapshotGuardString(row["maintenanceInterval"], `${at}.maintenanceInterval`)),
    cleaningRegime: (row["cleaningRegime"] === null ? null : architectProgramSnapshotGuardString(row["cleaningRegime"], `${at}.cleaningRegime`)),
    turnaroundTime: (row["turnaroundTime"] === null ? null : architectProgramSnapshotGuardString(row["turnaroundTime"], `${at}.turnaroundTime`)),
    redundancy: (row["redundancy"] === null ? null : architectProgramSnapshotGuardString(row["redundancy"], `${at}.redundancy`)),
    uptimeTarget: (row["uptimeTarget"] === null ? null : parseBinary64Transport(row["uptimeTarget"], `${at}.uptimeTarget`)),
    responseTime: (row["responseTime"] === null ? null : architectProgramSnapshotGuardString(row["responseTime"], `${at}.responseTime`)),
    equipmentIds: architectProgramSnapshotGuardArray(row["equipmentIds"], `${at}.equipmentIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.equipmentIds[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    processIds: architectProgramSnapshotGuardArray(row["processIds"], `${at}.processIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.processIds[${index}]`)),
    utilities: architectProgramSnapshotGuardArray(row["utilities"], `${at}.utilities`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.utilities[${index}]`)),
    wasteStreams: architectProgramSnapshotGuardArray(row["wasteStreams"], `${at}.wasteStreams`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.wasteStreams[${index}]`)),
    contingencyPlan: architectProgramSnapshotGuardArray(row["contingencyPlan"], `${at}.contingencyPlan`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.contingencyPlan[${index}]`)),
    trainingRequirements: architectProgramSnapshotGuardArray(row["trainingRequirements"], `${at}.trainingRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.trainingRequirements[${index}]`)),
    sopReferences: architectProgramSnapshotGuardArray(row["sopReferences"], `${at}.sopReferences`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.sopReferences[${index}]`)),
    kpiTargets: architectProgramSnapshotGuardArray(row["kpiTargets"], `${at}.kpiTargets`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.kpiTargets[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    serviceCategory: (row["serviceCategory"] === null ? null : architectProgramSnapshotGuardString(row["serviceCategory"], `${at}.serviceCategory`)),
    shiftPattern: (row["shiftPattern"] === null ? null : architectProgramSnapshotGuardString(row["shiftPattern"], `${at}.shiftPattern`)),
    slaTarget: (row["slaTarget"] === null ? null : architectProgramSnapshotGuardString(row["slaTarget"], `${at}.slaTarget`)),
    escalationContactId: (row["escalationContactId"] === null ? null : architectProgramSnapshotGuardString(row["escalationContactId"], `${at}.escalationContactId`)),
  };
}

export interface OptionEvaluation {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly optionName: string;
  readonly optionDescription: TextField;
  readonly scenarioId: string | null;
  readonly criteriaIds: readonly string[];
  readonly scores: readonly Binary64Transport[];
  readonly weightedScore: Binary64Transport | null;
  readonly costEstimate: Binary64Transport | null;
  readonly scheduleEstimate: string | null;
  readonly riskSummary: readonly string[];
  readonly benefits: readonly string[];
  readonly drawbacks: readonly string[];
  readonly assumptions: readonly string[];
  readonly dependencies: readonly string[];
  readonly stakeholderFeedback: readonly TaggedNote[];
  readonly recommendation: string | null;
  readonly decisionId: string | null;
  readonly evaluationStatus: ValidationStatus;
  readonly evaluatorIds: readonly string[];
  readonly evaluationDate: string | null;
}

export function parseOptionEvaluation(value: unknown, at = "$"): OptionEvaluation {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "optionName", "optionDescription", "scenarioId", "criteriaIds", "scores", "weightedScore", "costEstimate", "scheduleEstimate", "riskSummary", "benefits", "drawbacks", "assumptions", "dependencies", "stakeholderFeedback", "recommendation", "decisionId", "evaluationStatus", "evaluatorIds", "evaluationDate"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    optionName: architectProgramSnapshotGuardString(row["optionName"], `${at}.optionName`),
    optionDescription: parseTextField(row["optionDescription"], `${at}.optionDescription`),
    scenarioId: (row["scenarioId"] === null ? null : architectProgramSnapshotGuardString(row["scenarioId"], `${at}.scenarioId`)),
    criteriaIds: architectProgramSnapshotGuardArray(row["criteriaIds"], `${at}.criteriaIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.criteriaIds[${index}]`)),
    scores: architectProgramSnapshotGuardArray(row["scores"], `${at}.scores`).map((item, index) => parseBinary64Transport(item, `${at}.scores[${index}]`)),
    weightedScore: (row["weightedScore"] === null ? null : parseBinary64Transport(row["weightedScore"], `${at}.weightedScore`)),
    costEstimate: (row["costEstimate"] === null ? null : parseBinary64Transport(row["costEstimate"], `${at}.costEstimate`)),
    scheduleEstimate: (row["scheduleEstimate"] === null ? null : architectProgramSnapshotGuardString(row["scheduleEstimate"], `${at}.scheduleEstimate`)),
    riskSummary: architectProgramSnapshotGuardArray(row["riskSummary"], `${at}.riskSummary`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.riskSummary[${index}]`)),
    benefits: architectProgramSnapshotGuardArray(row["benefits"], `${at}.benefits`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.benefits[${index}]`)),
    drawbacks: architectProgramSnapshotGuardArray(row["drawbacks"], `${at}.drawbacks`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.drawbacks[${index}]`)),
    assumptions: architectProgramSnapshotGuardArray(row["assumptions"], `${at}.assumptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.assumptions[${index}]`)),
    dependencies: architectProgramSnapshotGuardArray(row["dependencies"], `${at}.dependencies`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dependencies[${index}]`)),
    stakeholderFeedback: architectProgramSnapshotGuardArray(row["stakeholderFeedback"], `${at}.stakeholderFeedback`).map((item, index) => parseTaggedNote(item, `${at}.stakeholderFeedback[${index}]`)),
    recommendation: (row["recommendation"] === null ? null : architectProgramSnapshotGuardString(row["recommendation"], `${at}.recommendation`)),
    decisionId: (row["decisionId"] === null ? null : architectProgramSnapshotGuardString(row["decisionId"], `${at}.decisionId`)),
    evaluationStatus: parseValidationStatus(row["evaluationStatus"], `${at}.evaluationStatus`),
    evaluatorIds: architectProgramSnapshotGuardArray(row["evaluatorIds"], `${at}.evaluatorIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.evaluatorIds[${index}]`)),
    evaluationDate: (row["evaluationDate"] === null ? null : architectProgramSnapshotGuardString(row["evaluationDate"], `${at}.evaluationDate`)),
  };
}

export interface OrganizationalRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly department: string;
  readonly reportingLine: string | null;
  readonly headcount: QuantitySpec;
  readonly growthPlanId: string | null;
  readonly workPatterns: readonly string[];
  readonly collaborationModel: string | null;
  readonly hierarchyLevels: readonly string[];
  readonly decisionMaking: readonly string[];
  readonly cultureNotes: readonly string[];
  readonly changeReadiness: string | null;
  readonly unionConsiderations: readonly string[];
  readonly trainingNeeds: readonly string[];
  readonly elementIds: readonly string[];
  readonly stakeholderIds: readonly string[];
  readonly serviceRequirementIds: readonly string[];
  readonly brandingRequirements: readonly string[];
  readonly wellnessPlugins: readonly string[];
  readonly diversityGoals: readonly string[];
  readonly ownerId: string | null;
}

export function parseOrganizationalRequirement(value: unknown, at = "$"): OrganizationalRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "department", "reportingLine", "headcount", "growthPlanId", "workPatterns", "collaborationModel", "hierarchyLevels", "decisionMaking", "cultureNotes", "changeReadiness", "unionConsiderations", "trainingNeeds", "elementIds", "stakeholderIds", "serviceRequirementIds", "brandingRequirements", "wellnessPlugins", "diversityGoals", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    department: architectProgramSnapshotGuardString(row["department"], `${at}.department`),
    reportingLine: (row["reportingLine"] === null ? null : architectProgramSnapshotGuardString(row["reportingLine"], `${at}.reportingLine`)),
    headcount: parseQuantitySpec(row["headcount"], `${at}.headcount`),
    growthPlanId: (row["growthPlanId"] === null ? null : architectProgramSnapshotGuardString(row["growthPlanId"], `${at}.growthPlanId`)),
    workPatterns: architectProgramSnapshotGuardArray(row["workPatterns"], `${at}.workPatterns`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.workPatterns[${index}]`)),
    collaborationModel: (row["collaborationModel"] === null ? null : architectProgramSnapshotGuardString(row["collaborationModel"], `${at}.collaborationModel`)),
    hierarchyLevels: architectProgramSnapshotGuardArray(row["hierarchyLevels"], `${at}.hierarchyLevels`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.hierarchyLevels[${index}]`)),
    decisionMaking: architectProgramSnapshotGuardArray(row["decisionMaking"], `${at}.decisionMaking`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decisionMaking[${index}]`)),
    cultureNotes: architectProgramSnapshotGuardArray(row["cultureNotes"], `${at}.cultureNotes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.cultureNotes[${index}]`)),
    changeReadiness: (row["changeReadiness"] === null ? null : architectProgramSnapshotGuardString(row["changeReadiness"], `${at}.changeReadiness`)),
    unionConsiderations: architectProgramSnapshotGuardArray(row["unionConsiderations"], `${at}.unionConsiderations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.unionConsiderations[${index}]`)),
    trainingNeeds: architectProgramSnapshotGuardArray(row["trainingNeeds"], `${at}.trainingNeeds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.trainingNeeds[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    stakeholderIds: architectProgramSnapshotGuardArray(row["stakeholderIds"], `${at}.stakeholderIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderIds[${index}]`)),
    serviceRequirementIds: architectProgramSnapshotGuardArray(row["serviceRequirementIds"], `${at}.serviceRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.serviceRequirementIds[${index}]`)),
    brandingRequirements: architectProgramSnapshotGuardArray(row["brandingRequirements"], `${at}.brandingRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.brandingRequirements[${index}]`)),
    wellnessPlugins: architectProgramSnapshotGuardArray(row["wellnessPlugins"], `${at}.wellnessPlugins`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.wellnessPlugins[${index}]`)),
    diversityGoals: architectProgramSnapshotGuardArray(row["diversityGoals"], `${at}.diversityGoals`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.diversityGoals[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export interface Ownership {
  readonly ownerId: string | null;
  readonly authorityId: string | null;
  readonly consultantIds: readonly string[];
  readonly participantIds: readonly string[];
}

export function parseOwnership(value: unknown, at = "$"): Ownership {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["ownerId", "authorityId", "consultantIds", "participantIds"]);
  return {
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    authorityId: (row["authorityId"] === null ? null : architectProgramSnapshotGuardString(row["authorityId"], `${at}.authorityId`)),
    consultantIds: architectProgramSnapshotGuardArray(row["consultantIds"], `${at}.consultantIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.consultantIds[${index}]`)),
    participantIds: architectProgramSnapshotGuardArray(row["participantIds"], `${at}.participantIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.participantIds[${index}]`)),
  };
}

export interface PerformanceCriterion {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly criterion: string;
  readonly metric: string;
  readonly target: Binary64Transport | null;
  readonly unit: string | null;
  readonly minimum: Binary64Transport | null;
  readonly maximum: Binary64Transport | null;
  readonly measurementMethod: string | null;
  readonly frequency: string | null;
  readonly requirementIds: readonly string[];
  readonly elementIds: readonly string[];
  readonly baseline: Binary64Transport | null;
  readonly benchmarkRef: string | null;
  readonly weight: Binary64Transport | null;
  readonly dataSource: string | null;
  readonly reportingCadence: string | null;
  readonly ownerId: string | null;
  readonly verificationPlan: string | null;
  readonly penaltyThreshold: Binary64Transport | null;
  readonly incentiveThreshold: Binary64Transport | null;
}

export function parsePerformanceCriterion(value: unknown, at = "$"): PerformanceCriterion {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "criterion", "metric", "target", "unit", "minimum", "maximum", "measurementMethod", "frequency", "requirementIds", "elementIds", "baseline", "benchmarkRef", "weight", "dataSource", "reportingCadence", "ownerId", "verificationPlan", "penaltyThreshold", "incentiveThreshold"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    criterion: architectProgramSnapshotGuardString(row["criterion"], `${at}.criterion`),
    metric: architectProgramSnapshotGuardString(row["metric"], `${at}.metric`),
    target: (row["target"] === null ? null : parseBinary64Transport(row["target"], `${at}.target`)),
    unit: (row["unit"] === null ? null : architectProgramSnapshotGuardString(row["unit"], `${at}.unit`)),
    minimum: (row["minimum"] === null ? null : parseBinary64Transport(row["minimum"], `${at}.minimum`)),
    maximum: (row["maximum"] === null ? null : parseBinary64Transport(row["maximum"], `${at}.maximum`)),
    measurementMethod: (row["measurementMethod"] === null ? null : architectProgramSnapshotGuardString(row["measurementMethod"], `${at}.measurementMethod`)),
    frequency: (row["frequency"] === null ? null : architectProgramSnapshotGuardString(row["frequency"], `${at}.frequency`)),
    requirementIds: architectProgramSnapshotGuardArray(row["requirementIds"], `${at}.requirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.requirementIds[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    baseline: (row["baseline"] === null ? null : parseBinary64Transport(row["baseline"], `${at}.baseline`)),
    benchmarkRef: (row["benchmarkRef"] === null ? null : architectProgramSnapshotGuardString(row["benchmarkRef"], `${at}.benchmarkRef`)),
    weight: (row["weight"] === null ? null : parseBinary64Transport(row["weight"], `${at}.weight`)),
    dataSource: (row["dataSource"] === null ? null : architectProgramSnapshotGuardString(row["dataSource"], `${at}.dataSource`)),
    reportingCadence: (row["reportingCadence"] === null ? null : architectProgramSnapshotGuardString(row["reportingCadence"], `${at}.reportingCadence`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    verificationPlan: (row["verificationPlan"] === null ? null : architectProgramSnapshotGuardString(row["verificationPlan"], `${at}.verificationPlan`)),
    penaltyThreshold: (row["penaltyThreshold"] === null ? null : parseBinary64Transport(row["penaltyThreshold"], `${at}.penaltyThreshold`)),
    incentiveThreshold: (row["incentiveThreshold"] === null ? null : parseBinary64Transport(row["incentiveThreshold"], `${at}.incentiveThreshold`)),
  };
}

export type Priority = "mandatory" | "essential" | "preferred" | "optional" | "deferred" | "prohibited";

export function parsePriority(value: unknown, at = "$"): Priority {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["mandatory", "essential", "preferred", "optional", "deferred", "prohibited"] as const);
}

export interface PriorityRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly subjectId: string;
  readonly subjectKind: string;
  readonly rankedPriority: Priority;
  readonly rank: number | null;
  readonly weight: Binary64Transport | null;
  readonly rationale: TextField | null;
  readonly decisionId: string | null;
  readonly stakeholderIds: readonly string[];
  readonly effectiveFrom: string | null;
  readonly effectiveUntil: string | null;
  readonly reviewCycle: string | null;
  readonly dependencies: readonly string[];
  readonly conflicts: readonly string[];
  readonly scoringMethod: string | null;
  readonly score: Binary64Transport | null;
  readonly criteria: readonly string[];
  readonly approvedBy: string | null;
  readonly approvalDate: string | null;
  readonly rankingNotes: readonly TaggedNote[];
}

export function parsePriorityRecord(value: unknown, at = "$"): PriorityRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "subjectId", "subjectKind", "rankedPriority", "rank", "weight", "rationale", "decisionId", "stakeholderIds", "effectiveFrom", "effectiveUntil", "reviewCycle", "dependencies", "conflicts", "scoringMethod", "score", "criteria", "approvedBy", "approvalDate", "rankingNotes"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    subjectId: architectProgramSnapshotGuardString(row["subjectId"], `${at}.subjectId`),
    subjectKind: architectProgramSnapshotGuardString(row["subjectKind"], `${at}.subjectKind`),
    rankedPriority: parsePriority(row["rankedPriority"], `${at}.rankedPriority`),
    rank: (row["rank"] === null ? null : architectProgramSnapshotGuardInteger(row["rank"], `${at}.rank`, {"minimum": 0, "maximum": 4294967295})),
    weight: (row["weight"] === null ? null : parseBinary64Transport(row["weight"], `${at}.weight`)),
    rationale: (row["rationale"] === null ? null : parseTextField(row["rationale"], `${at}.rationale`)),
    decisionId: (row["decisionId"] === null ? null : architectProgramSnapshotGuardString(row["decisionId"], `${at}.decisionId`)),
    stakeholderIds: architectProgramSnapshotGuardArray(row["stakeholderIds"], `${at}.stakeholderIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderIds[${index}]`)),
    effectiveFrom: (row["effectiveFrom"] === null ? null : architectProgramSnapshotGuardString(row["effectiveFrom"], `${at}.effectiveFrom`)),
    effectiveUntil: (row["effectiveUntil"] === null ? null : architectProgramSnapshotGuardString(row["effectiveUntil"], `${at}.effectiveUntil`)),
    reviewCycle: (row["reviewCycle"] === null ? null : architectProgramSnapshotGuardString(row["reviewCycle"], `${at}.reviewCycle`)),
    dependencies: architectProgramSnapshotGuardArray(row["dependencies"], `${at}.dependencies`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dependencies[${index}]`)),
    conflicts: architectProgramSnapshotGuardArray(row["conflicts"], `${at}.conflicts`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.conflicts[${index}]`)),
    scoringMethod: (row["scoringMethod"] === null ? null : architectProgramSnapshotGuardString(row["scoringMethod"], `${at}.scoringMethod`)),
    score: (row["score"] === null ? null : parseBinary64Transport(row["score"], `${at}.score`)),
    criteria: architectProgramSnapshotGuardArray(row["criteria"], `${at}.criteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.criteria[${index}]`)),
    approvedBy: (row["approvedBy"] === null ? null : architectProgramSnapshotGuardString(row["approvedBy"], `${at}.approvedBy`)),
    approvalDate: (row["approvalDate"] === null ? null : architectProgramSnapshotGuardString(row["approvalDate"], `${at}.approvalDate`)),
    rankingNotes: architectProgramSnapshotGuardArray(row["rankingNotes"], `${at}.rankingNotes`).map((item, index) => parseTaggedNote(item, `${at}.rankingNotes[${index}]`)),
  };
}

export type PrivacyKind = "public" | "semiPublic" | "semiPrivate" | "private" | "confidential" | "restricted" | "anonymous";

export function parsePrivacyKind(value: unknown, at = "$"): PrivacyKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["public", "semiPublic", "semiPrivate", "private", "confidential", "restricted", "anonymous"] as const);
}

export interface PrivacyRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly privacyKind: PrivacyKind;
  readonly privacyType: string;
  readonly level: string | null;
  readonly subjectIds: readonly string[];
  readonly elementIds: readonly string[];
  readonly visualPrivacy: readonly string[];
  readonly acousticPrivacy: readonly string[];
  readonly dataPrivacy: readonly string[];
  readonly screeningRequired: boolean;
  readonly enclosureRequired: boolean;
  readonly accessRestrictions: readonly string[];
  readonly observationRisk: string | null;
  readonly regulatoryBasis: readonly string[];
  readonly culturalConsiderations: readonly string[];
  readonly technologyControls: readonly string[];
  readonly signage: readonly string[];
  readonly monitoringRestrictions: readonly string[];
  readonly retentionPolicy: string | null;
  readonly breachResponse: readonly string[];
  readonly ownerId: string | null;
}

export function parsePrivacyRequirement(value: unknown, at = "$"): PrivacyRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "privacyKind", "privacyType", "level", "subjectIds", "elementIds", "visualPrivacy", "acousticPrivacy", "dataPrivacy", "screeningRequired", "enclosureRequired", "accessRestrictions", "observationRisk", "regulatoryBasis", "culturalConsiderations", "technologyControls", "signage", "monitoringRestrictions", "retentionPolicy", "breachResponse", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    privacyKind: parsePrivacyKind(row["privacyKind"], `${at}.privacyKind`),
    privacyType: architectProgramSnapshotGuardString(row["privacyType"], `${at}.privacyType`),
    level: (row["level"] === null ? null : architectProgramSnapshotGuardString(row["level"], `${at}.level`)),
    subjectIds: architectProgramSnapshotGuardArray(row["subjectIds"], `${at}.subjectIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.subjectIds[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    visualPrivacy: architectProgramSnapshotGuardArray(row["visualPrivacy"], `${at}.visualPrivacy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.visualPrivacy[${index}]`)),
    acousticPrivacy: architectProgramSnapshotGuardArray(row["acousticPrivacy"], `${at}.acousticPrivacy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.acousticPrivacy[${index}]`)),
    dataPrivacy: architectProgramSnapshotGuardArray(row["dataPrivacy"], `${at}.dataPrivacy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dataPrivacy[${index}]`)),
    screeningRequired: architectProgramSnapshotGuardBoolean(row["screeningRequired"], `${at}.screeningRequired`),
    enclosureRequired: architectProgramSnapshotGuardBoolean(row["enclosureRequired"], `${at}.enclosureRequired`),
    accessRestrictions: architectProgramSnapshotGuardArray(row["accessRestrictions"], `${at}.accessRestrictions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.accessRestrictions[${index}]`)),
    observationRisk: (row["observationRisk"] === null ? null : architectProgramSnapshotGuardString(row["observationRisk"], `${at}.observationRisk`)),
    regulatoryBasis: architectProgramSnapshotGuardArray(row["regulatoryBasis"], `${at}.regulatoryBasis`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryBasis[${index}]`)),
    culturalConsiderations: architectProgramSnapshotGuardArray(row["culturalConsiderations"], `${at}.culturalConsiderations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.culturalConsiderations[${index}]`)),
    technologyControls: architectProgramSnapshotGuardArray(row["technologyControls"], `${at}.technologyControls`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.technologyControls[${index}]`)),
    signage: architectProgramSnapshotGuardArray(row["signage"], `${at}.signage`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.signage[${index}]`)),
    monitoringRestrictions: architectProgramSnapshotGuardArray(row["monitoringRestrictions"], `${at}.monitoringRestrictions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.monitoringRestrictions[${index}]`)),
    retentionPolicy: (row["retentionPolicy"] === null ? null : architectProgramSnapshotGuardString(row["retentionPolicy"], `${at}.retentionPolicy`)),
    breachResponse: architectProgramSnapshotGuardArray(row["breachResponse"], `${at}.breachResponse`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.breachResponse[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export interface Process {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly code: string;
  readonly category: string;
  readonly trigger: string | null;
  readonly inputs: readonly string[];
  readonly outputs: readonly string[];
  readonly steps: readonly string[];
  readonly actors: readonly string[];
  readonly equipmentIds: readonly string[];
  readonly elementIds: readonly string[];
  readonly duration: string | null;
  readonly frequency: string | null;
  readonly criticalPath: boolean;
  readonly bottlenecks: readonly string[];
  readonly dependencies: readonly string[];
  readonly kpis: readonly string[];
  readonly automationLevel: string | null;
  readonly failureModes: readonly string[];
  readonly improvementOpportunities: readonly string[];
  readonly regulatoryRefs: readonly string[];
  readonly ownerId: string | null;
  readonly workflowType: string | null;
  readonly handoffPoints: readonly string[];
  readonly qualityGates: readonly string[];
}

export function parseProcess(value: unknown, at = "$"): Process {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "code", "category", "trigger", "inputs", "outputs", "steps", "actors", "equipmentIds", "elementIds", "duration", "frequency", "criticalPath", "bottlenecks", "dependencies", "kpis", "automationLevel", "failureModes", "improvementOpportunities", "regulatoryRefs", "ownerId", "workflowType", "handoffPoints", "qualityGates"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    category: architectProgramSnapshotGuardString(row["category"], `${at}.category`),
    trigger: (row["trigger"] === null ? null : architectProgramSnapshotGuardString(row["trigger"], `${at}.trigger`)),
    inputs: architectProgramSnapshotGuardArray(row["inputs"], `${at}.inputs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.inputs[${index}]`)),
    outputs: architectProgramSnapshotGuardArray(row["outputs"], `${at}.outputs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.outputs[${index}]`)),
    steps: architectProgramSnapshotGuardArray(row["steps"], `${at}.steps`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.steps[${index}]`)),
    actors: architectProgramSnapshotGuardArray(row["actors"], `${at}.actors`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.actors[${index}]`)),
    equipmentIds: architectProgramSnapshotGuardArray(row["equipmentIds"], `${at}.equipmentIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.equipmentIds[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    duration: (row["duration"] === null ? null : architectProgramSnapshotGuardString(row["duration"], `${at}.duration`)),
    frequency: (row["frequency"] === null ? null : architectProgramSnapshotGuardString(row["frequency"], `${at}.frequency`)),
    criticalPath: architectProgramSnapshotGuardBoolean(row["criticalPath"], `${at}.criticalPath`),
    bottlenecks: architectProgramSnapshotGuardArray(row["bottlenecks"], `${at}.bottlenecks`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.bottlenecks[${index}]`)),
    dependencies: architectProgramSnapshotGuardArray(row["dependencies"], `${at}.dependencies`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dependencies[${index}]`)),
    kpis: architectProgramSnapshotGuardArray(row["kpis"], `${at}.kpis`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.kpis[${index}]`)),
    automationLevel: (row["automationLevel"] === null ? null : architectProgramSnapshotGuardString(row["automationLevel"], `${at}.automationLevel`)),
    failureModes: architectProgramSnapshotGuardArray(row["failureModes"], `${at}.failureModes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.failureModes[${index}]`)),
    improvementOpportunities: architectProgramSnapshotGuardArray(row["improvementOpportunities"], `${at}.improvementOpportunities`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.improvementOpportunities[${index}]`)),
    regulatoryRefs: architectProgramSnapshotGuardArray(row["regulatoryRefs"], `${at}.regulatoryRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryRefs[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    workflowType: (row["workflowType"] === null ? null : architectProgramSnapshotGuardString(row["workflowType"], `${at}.workflowType`)),
    handoffPoints: architectProgramSnapshotGuardArray(row["handoffPoints"], `${at}.handoffPoints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.handoffPoints[${index}]`)),
    qualityGates: architectProgramSnapshotGuardArray(row["qualityGates"], `${at}.qualityGates`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.qualityGates[${index}]`)),
  };
}

export interface ProgramElement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly code: string;
  readonly kind: ProgramElementKind;
  readonly parentId: string | null;
  readonly level: string | null;
  readonly area: QuantitySpec;
  readonly volume: QuantitySpec;
  readonly height: QuantitySpec;
  readonly occupancy: QuantitySpec;
  readonly functionIds: readonly string[];
  readonly activityIds: readonly string[];
  readonly userProfileIds: readonly string[];
  readonly adjacencyIds: readonly string[];
  readonly quantityIds: readonly string[];
  readonly requirementIds: readonly string[];
  readonly locationHint: string | null;
  readonly orientation: string | null;
  readonly daylightRequirement: string | null;
  readonly acousticClass: string | null;
  readonly securityZone: string | null;
  readonly flexibilityNotes: readonly string[];
  readonly growthAllocation: string | null;
  readonly circulationRole: string | null;
  readonly visibilityLevel: string | null;
  readonly adjacencyPreferences: readonly string[];
  readonly environmentalZone: string | null;
}

export function parseProgramElement(value: unknown, at = "$"): ProgramElement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "code", "kind", "parentId", "level", "area", "volume", "height", "occupancy", "functionIds", "activityIds", "userProfileIds", "adjacencyIds", "quantityIds", "requirementIds", "locationHint", "orientation", "daylightRequirement", "acousticClass", "securityZone", "flexibilityNotes", "growthAllocation", "circulationRole", "visibilityLevel", "adjacencyPreferences", "environmentalZone"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    kind: parseProgramElementKind(row["kind"], `${at}.kind`),
    parentId: (row["parentId"] === null ? null : architectProgramSnapshotGuardString(row["parentId"], `${at}.parentId`)),
    level: (row["level"] === null ? null : architectProgramSnapshotGuardString(row["level"], `${at}.level`)),
    area: parseQuantitySpec(row["area"], `${at}.area`),
    volume: parseQuantitySpec(row["volume"], `${at}.volume`),
    height: parseQuantitySpec(row["height"], `${at}.height`),
    occupancy: parseQuantitySpec(row["occupancy"], `${at}.occupancy`),
    functionIds: architectProgramSnapshotGuardArray(row["functionIds"], `${at}.functionIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.functionIds[${index}]`)),
    activityIds: architectProgramSnapshotGuardArray(row["activityIds"], `${at}.activityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.activityIds[${index}]`)),
    userProfileIds: architectProgramSnapshotGuardArray(row["userProfileIds"], `${at}.userProfileIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.userProfileIds[${index}]`)),
    adjacencyIds: architectProgramSnapshotGuardArray(row["adjacencyIds"], `${at}.adjacencyIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.adjacencyIds[${index}]`)),
    quantityIds: architectProgramSnapshotGuardArray(row["quantityIds"], `${at}.quantityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.quantityIds[${index}]`)),
    requirementIds: architectProgramSnapshotGuardArray(row["requirementIds"], `${at}.requirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.requirementIds[${index}]`)),
    locationHint: (row["locationHint"] === null ? null : architectProgramSnapshotGuardString(row["locationHint"], `${at}.locationHint`)),
    orientation: (row["orientation"] === null ? null : architectProgramSnapshotGuardString(row["orientation"], `${at}.orientation`)),
    daylightRequirement: (row["daylightRequirement"] === null ? null : architectProgramSnapshotGuardString(row["daylightRequirement"], `${at}.daylightRequirement`)),
    acousticClass: (row["acousticClass"] === null ? null : architectProgramSnapshotGuardString(row["acousticClass"], `${at}.acousticClass`)),
    securityZone: (row["securityZone"] === null ? null : architectProgramSnapshotGuardString(row["securityZone"], `${at}.securityZone`)),
    flexibilityNotes: architectProgramSnapshotGuardArray(row["flexibilityNotes"], `${at}.flexibilityNotes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.flexibilityNotes[${index}]`)),
    growthAllocation: (row["growthAllocation"] === null ? null : architectProgramSnapshotGuardString(row["growthAllocation"], `${at}.growthAllocation`)),
    circulationRole: (row["circulationRole"] === null ? null : architectProgramSnapshotGuardString(row["circulationRole"], `${at}.circulationRole`)),
    visibilityLevel: (row["visibilityLevel"] === null ? null : architectProgramSnapshotGuardString(row["visibilityLevel"], `${at}.visibilityLevel`)),
    adjacencyPreferences: architectProgramSnapshotGuardArray(row["adjacencyPreferences"], `${at}.adjacencyPreferences`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.adjacencyPreferences[${index}]`)),
    environmentalZone: (row["environmentalZone"] === null ? null : architectProgramSnapshotGuardString(row["environmentalZone"], `${at}.environmentalZone`)),
  };
}

export type ProgramElementKind = "building" | "campus" | "floor" | "zone" | "room" | "suite" | "department" | "system" | "circulation" | "support" | "outdoor" | "furnitureGroup" | "other";

export function parseProgramElementKind(value: unknown, at = "$"): ProgramElementKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["building", "campus", "floor", "zone", "room", "suite", "department", "system", "circulation", "support", "outdoor", "furnitureGroup", "other"] as const);
}

export interface ProgramMeta {
  readonly schema: string;
  readonly documentId: string;
  readonly title: string;
  readonly subtitle: string | null;
  readonly purpose: TextField;
  readonly terminology: readonly string[];
  readonly classification: readonly string[];
  readonly industrySector: string;
  readonly projectType: string;
  readonly locale: string;
  readonly revision: string;
  readonly authorIds: readonly string[];
  readonly sourceSystem: string | null;
  readonly exportProfile: string | null;
  readonly timestamps: TimestampMeta;
}

export function parseProgramMeta(value: unknown, at = "$"): ProgramMeta {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["schema", "documentId", "title", "subtitle", "purpose", "terminology", "classification", "industrySector", "projectType", "locale", "revision", "authorIds", "sourceSystem", "exportProfile", "timestamps"]);
  return {
    schema: architectProgramSnapshotGuardString(row["schema"], `${at}.schema`),
    documentId: architectProgramSnapshotGuardString(row["documentId"], `${at}.documentId`),
    title: architectProgramSnapshotGuardString(row["title"], `${at}.title`),
    subtitle: (row["subtitle"] === null ? null : architectProgramSnapshotGuardString(row["subtitle"], `${at}.subtitle`)),
    purpose: parseTextField(row["purpose"], `${at}.purpose`),
    terminology: architectProgramSnapshotGuardArray(row["terminology"], `${at}.terminology`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.terminology[${index}]`)),
    classification: architectProgramSnapshotGuardArray(row["classification"], `${at}.classification`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.classification[${index}]`)),
    industrySector: architectProgramSnapshotGuardString(row["industrySector"], `${at}.industrySector`),
    projectType: architectProgramSnapshotGuardString(row["projectType"], `${at}.projectType`),
    locale: architectProgramSnapshotGuardString(row["locale"], `${at}.locale`),
    revision: architectProgramSnapshotGuardString(row["revision"], `${at}.revision`),
    authorIds: architectProgramSnapshotGuardArray(row["authorIds"], `${at}.authorIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.authorIds[${index}]`)),
    sourceSystem: (row["sourceSystem"] === null ? null : architectProgramSnapshotGuardString(row["sourceSystem"], `${at}.sourceSystem`)),
    exportProfile: (row["exportProfile"] === null ? null : architectProgramSnapshotGuardString(row["exportProfile"], `${at}.exportProfile`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
  };
}

export interface ProjectDefinition {
  readonly id: string;
  readonly code: string;
  readonly clientName: string;
  readonly ownerOrganization: string;
  readonly briefSummary: TextField;
  readonly problemStatement: TextField;
  readonly vision: TextField;
  readonly mission: TextField;
  readonly objectives: readonly string[];
  readonly successCriteria: readonly string[];
  readonly projectPriorities: readonly Priority[];
  readonly completionCriteria: readonly string[];
  readonly decisionCriteria: readonly string[];
  readonly scopeInclusions: readonly string[];
  readonly scopeExclusions: readonly string[];
  readonly assumptions: readonly string[];
  readonly constraintsSummary: readonly string[];
  readonly dependencies: readonly string[];
  readonly deliverables: readonly string[];
  readonly phases: readonly string[];
  readonly geographicContext: TextField;
  readonly developmentContext: TextField;
  readonly operationalContext: TextField;
  readonly regulatoryContext: readonly string[];
  readonly fundingModel: string;
  readonly ownership: Ownership;
  readonly timestamps: TimestampMeta;
}

export function parseProjectDefinition(value: unknown, at = "$"): ProjectDefinition {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "code", "clientName", "ownerOrganization", "briefSummary", "problemStatement", "vision", "mission", "objectives", "successCriteria", "projectPriorities", "completionCriteria", "decisionCriteria", "scopeInclusions", "scopeExclusions", "assumptions", "constraintsSummary", "dependencies", "deliverables", "phases", "geographicContext", "developmentContext", "operationalContext", "regulatoryContext", "fundingModel", "ownership", "timestamps"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    clientName: architectProgramSnapshotGuardString(row["clientName"], `${at}.clientName`),
    ownerOrganization: architectProgramSnapshotGuardString(row["ownerOrganization"], `${at}.ownerOrganization`),
    briefSummary: parseTextField(row["briefSummary"], `${at}.briefSummary`),
    problemStatement: parseTextField(row["problemStatement"], `${at}.problemStatement`),
    vision: parseTextField(row["vision"], `${at}.vision`),
    mission: parseTextField(row["mission"], `${at}.mission`),
    objectives: architectProgramSnapshotGuardArray(row["objectives"], `${at}.objectives`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.objectives[${index}]`)),
    successCriteria: architectProgramSnapshotGuardArray(row["successCriteria"], `${at}.successCriteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.successCriteria[${index}]`)),
    projectPriorities: architectProgramSnapshotGuardArray(row["projectPriorities"], `${at}.projectPriorities`).map((item, index) => parsePriority(item, `${at}.projectPriorities[${index}]`)),
    completionCriteria: architectProgramSnapshotGuardArray(row["completionCriteria"], `${at}.completionCriteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.completionCriteria[${index}]`)),
    decisionCriteria: architectProgramSnapshotGuardArray(row["decisionCriteria"], `${at}.decisionCriteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decisionCriteria[${index}]`)),
    scopeInclusions: architectProgramSnapshotGuardArray(row["scopeInclusions"], `${at}.scopeInclusions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.scopeInclusions[${index}]`)),
    scopeExclusions: architectProgramSnapshotGuardArray(row["scopeExclusions"], `${at}.scopeExclusions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.scopeExclusions[${index}]`)),
    assumptions: architectProgramSnapshotGuardArray(row["assumptions"], `${at}.assumptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.assumptions[${index}]`)),
    constraintsSummary: architectProgramSnapshotGuardArray(row["constraintsSummary"], `${at}.constraintsSummary`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.constraintsSummary[${index}]`)),
    dependencies: architectProgramSnapshotGuardArray(row["dependencies"], `${at}.dependencies`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dependencies[${index}]`)),
    deliverables: architectProgramSnapshotGuardArray(row["deliverables"], `${at}.deliverables`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.deliverables[${index}]`)),
    phases: architectProgramSnapshotGuardArray(row["phases"], `${at}.phases`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.phases[${index}]`)),
    geographicContext: parseTextField(row["geographicContext"], `${at}.geographicContext`),
    developmentContext: parseTextField(row["developmentContext"], `${at}.developmentContext`),
    operationalContext: parseTextField(row["operationalContext"], `${at}.operationalContext`),
    regulatoryContext: architectProgramSnapshotGuardArray(row["regulatoryContext"], `${at}.regulatoryContext`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryContext[${index}]`)),
    fundingModel: architectProgramSnapshotGuardString(row["fundingModel"], `${at}.fundingModel`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
  };
}

export interface QualityRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly qualityTopic: string;
  readonly standard: string | null;
  readonly targetLevel: string | null;
  readonly inspectionPoints: readonly string[];
  readonly acceptanceCriteria: readonly string[];
  readonly testingRequirements: readonly string[];
  readonly sampleRate: string | null;
  readonly defectCategories: readonly string[];
  readonly correctiveActionProcess: readonly string[];
  readonly elementIds: readonly string[];
  readonly requirementIds: readonly string[];
  readonly supplierRequirements: readonly string[];
  readonly documentationRequirements: readonly string[];
  readonly trainingRequirements: readonly string[];
  readonly auditSchedule: string | null;
  readonly kpis: readonly string[];
  readonly ownerId: string | null;
  readonly certificationTargets: readonly string[];
  readonly continuousImprovement: readonly string[];
}

export function parseQualityRecord(value: unknown, at = "$"): QualityRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "qualityTopic", "standard", "targetLevel", "inspectionPoints", "acceptanceCriteria", "testingRequirements", "sampleRate", "defectCategories", "correctiveActionProcess", "elementIds", "requirementIds", "supplierRequirements", "documentationRequirements", "trainingRequirements", "auditSchedule", "kpis", "ownerId", "certificationTargets", "continuousImprovement"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    qualityTopic: architectProgramSnapshotGuardString(row["qualityTopic"], `${at}.qualityTopic`),
    standard: (row["standard"] === null ? null : architectProgramSnapshotGuardString(row["standard"], `${at}.standard`)),
    targetLevel: (row["targetLevel"] === null ? null : architectProgramSnapshotGuardString(row["targetLevel"], `${at}.targetLevel`)),
    inspectionPoints: architectProgramSnapshotGuardArray(row["inspectionPoints"], `${at}.inspectionPoints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.inspectionPoints[${index}]`)),
    acceptanceCriteria: architectProgramSnapshotGuardArray(row["acceptanceCriteria"], `${at}.acceptanceCriteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.acceptanceCriteria[${index}]`)),
    testingRequirements: architectProgramSnapshotGuardArray(row["testingRequirements"], `${at}.testingRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.testingRequirements[${index}]`)),
    sampleRate: (row["sampleRate"] === null ? null : architectProgramSnapshotGuardString(row["sampleRate"], `${at}.sampleRate`)),
    defectCategories: architectProgramSnapshotGuardArray(row["defectCategories"], `${at}.defectCategories`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.defectCategories[${index}]`)),
    correctiveActionProcess: architectProgramSnapshotGuardArray(row["correctiveActionProcess"], `${at}.correctiveActionProcess`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.correctiveActionProcess[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    requirementIds: architectProgramSnapshotGuardArray(row["requirementIds"], `${at}.requirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.requirementIds[${index}]`)),
    supplierRequirements: architectProgramSnapshotGuardArray(row["supplierRequirements"], `${at}.supplierRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.supplierRequirements[${index}]`)),
    documentationRequirements: architectProgramSnapshotGuardArray(row["documentationRequirements"], `${at}.documentationRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.documentationRequirements[${index}]`)),
    trainingRequirements: architectProgramSnapshotGuardArray(row["trainingRequirements"], `${at}.trainingRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.trainingRequirements[${index}]`)),
    auditSchedule: (row["auditSchedule"] === null ? null : architectProgramSnapshotGuardString(row["auditSchedule"], `${at}.auditSchedule`)),
    kpis: architectProgramSnapshotGuardArray(row["kpis"], `${at}.kpis`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.kpis[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    certificationTargets: architectProgramSnapshotGuardArray(row["certificationTargets"], `${at}.certificationTargets`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.certificationTargets[${index}]`)),
    continuousImprovement: architectProgramSnapshotGuardArray(row["continuousImprovement"], `${at}.continuousImprovement`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.continuousImprovement[${index}]`)),
  };
}

export interface QuantityRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly targetElementId: string;
  readonly metric: string;
  readonly quantity: QuantitySpec;
  readonly basis: string | null;
  readonly calculationMethod: string | null;
  readonly source: string | null;
  readonly benchmarkRef: string | null;
  readonly tolerancePercent: Binary64Transport | null;
  readonly peakFactor: Binary64Transport | null;
  readonly growthFactor: Binary64Transport | null;
  readonly unitCost: Binary64Transport | null;
  readonly currency: string | null;
  readonly verificationMethod: string | null;
  readonly relatedRequirementIds: readonly string[];
  readonly assumptions: readonly string[];
  readonly constraints: readonly string[];
  readonly schedulePhase: string | null;
  readonly responsibleParty: string | null;
  readonly lastVerified: string | null;
  readonly varianceNotes: readonly TaggedNote[];
}

export function parseQuantityRequirement(value: unknown, at = "$"): QuantityRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "targetElementId", "metric", "quantity", "basis", "calculationMethod", "source", "benchmarkRef", "tolerancePercent", "peakFactor", "growthFactor", "unitCost", "currency", "verificationMethod", "relatedRequirementIds", "assumptions", "constraints", "schedulePhase", "responsibleParty", "lastVerified", "varianceNotes"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    targetElementId: architectProgramSnapshotGuardString(row["targetElementId"], `${at}.targetElementId`),
    metric: architectProgramSnapshotGuardString(row["metric"], `${at}.metric`),
    quantity: parseQuantitySpec(row["quantity"], `${at}.quantity`),
    basis: (row["basis"] === null ? null : architectProgramSnapshotGuardString(row["basis"], `${at}.basis`)),
    calculationMethod: (row["calculationMethod"] === null ? null : architectProgramSnapshotGuardString(row["calculationMethod"], `${at}.calculationMethod`)),
    source: (row["source"] === null ? null : architectProgramSnapshotGuardString(row["source"], `${at}.source`)),
    benchmarkRef: (row["benchmarkRef"] === null ? null : architectProgramSnapshotGuardString(row["benchmarkRef"], `${at}.benchmarkRef`)),
    tolerancePercent: (row["tolerancePercent"] === null ? null : parseBinary64Transport(row["tolerancePercent"], `${at}.tolerancePercent`)),
    peakFactor: (row["peakFactor"] === null ? null : parseBinary64Transport(row["peakFactor"], `${at}.peakFactor`)),
    growthFactor: (row["growthFactor"] === null ? null : parseBinary64Transport(row["growthFactor"], `${at}.growthFactor`)),
    unitCost: (row["unitCost"] === null ? null : parseBinary64Transport(row["unitCost"], `${at}.unitCost`)),
    currency: (row["currency"] === null ? null : architectProgramSnapshotGuardString(row["currency"], `${at}.currency`)),
    verificationMethod: (row["verificationMethod"] === null ? null : architectProgramSnapshotGuardString(row["verificationMethod"], `${at}.verificationMethod`)),
    relatedRequirementIds: architectProgramSnapshotGuardArray(row["relatedRequirementIds"], `${at}.relatedRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedRequirementIds[${index}]`)),
    assumptions: architectProgramSnapshotGuardArray(row["assumptions"], `${at}.assumptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.assumptions[${index}]`)),
    constraints: architectProgramSnapshotGuardArray(row["constraints"], `${at}.constraints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.constraints[${index}]`)),
    schedulePhase: (row["schedulePhase"] === null ? null : architectProgramSnapshotGuardString(row["schedulePhase"], `${at}.schedulePhase`)),
    responsibleParty: (row["responsibleParty"] === null ? null : architectProgramSnapshotGuardString(row["responsibleParty"], `${at}.responsibleParty`)),
    lastVerified: (row["lastVerified"] === null ? null : architectProgramSnapshotGuardString(row["lastVerified"], `${at}.lastVerified`)),
    varianceNotes: architectProgramSnapshotGuardArray(row["varianceNotes"], `${at}.varianceNotes`).map((item, index) => parseTaggedNote(item, `${at}.varianceNotes[${index}]`)),
  };
}

export interface QuantitySpec {
  readonly min?: Binary64Transport;
  readonly max?: Binary64Transport;
  readonly target?: Binary64Transport;
  readonly current?: Binary64Transport;
  readonly forecast?: Binary64Transport;
  readonly peak?: Binary64Transport;
  readonly average?: Binary64Transport;
  readonly unit: string;
}

export function parseQuantitySpec(value: unknown, at = "$"): QuantitySpec {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["min", "max", "target", "current", "forecast", "peak", "average", "unit"]);
  return {
    min: row["min"] === undefined ? undefined : parseBinary64Transport(row["min"], `${at}.min`),
    max: row["max"] === undefined ? undefined : parseBinary64Transport(row["max"], `${at}.max`),
    target: row["target"] === undefined ? undefined : parseBinary64Transport(row["target"], `${at}.target`),
    current: row["current"] === undefined ? undefined : parseBinary64Transport(row["current"], `${at}.current`),
    forecast: row["forecast"] === undefined ? undefined : parseBinary64Transport(row["forecast"], `${at}.forecast`),
    peak: row["peak"] === undefined ? undefined : parseBinary64Transport(row["peak"], `${at}.peak`),
    average: row["average"] === undefined ? undefined : parseBinary64Transport(row["average"], `${at}.average`),
    unit: architectProgramSnapshotGuardString(row["unit"], `${at}.unit`),
  };
}

export type RegisterKey = "informationRequirement" | "sustainabilityRequirement" | "accessibilityRequirement" | "conflict" | "optionEvaluation" | "function" | "risk" | "decision" | "validationRecord" | "priorityRecord" | "flowRequirement" | "environmentalRequirement" | "workshop" | "scenario" | "benchmarkRecord" | "activity" | "infrastructureRequirement" | "organizationalRequirement" | "issue" | "approvalRecord" | "stakeholder" | "qualityRecord" | "resilienceRequirement" | "assumption" | "costRequirement" | "document" | "scheduleRequirement" | "growthPlan" | "performanceCriterion" | "operationalRequirement" | "requirement" | "siteContext" | "templateRecord" | "reportRecord" | "auditEvent" | "knowledgeRecord" | "regulatoryRequirement" | "communicationRequirement" | "resource" | "statusRecord" | "changeRecord" | "process" | "searchFilter" | "accessRule" | "privacyRequirement" | "quantityRequirement" | "analysisRecord" | "relationship" | "storageRequirement" | "meetingRecord" | "survey" | "deliveryConstraint" | "constraintRecord" | "complianceRecord" | "serviceRequirement" | "equipment" | "securityRequirement" | "collaborationRecord" | "safetyRequirement" | "userProfile" | "humanFactorRequirement" | "flexibilityRequirement" | "wayfindingRequirement" | "programElement";

export function parseRegisterKey(value: unknown, at = "$"): RegisterKey {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["informationRequirement", "sustainabilityRequirement", "accessibilityRequirement", "conflict", "optionEvaluation", "function", "risk", "decision", "validationRecord", "priorityRecord", "flowRequirement", "environmentalRequirement", "workshop", "scenario", "benchmarkRecord", "activity", "infrastructureRequirement", "organizationalRequirement", "issue", "approvalRecord", "stakeholder", "qualityRecord", "resilienceRequirement", "assumption", "costRequirement", "document", "scheduleRequirement", "growthPlan", "performanceCriterion", "operationalRequirement", "requirement", "siteContext", "templateRecord", "reportRecord", "auditEvent", "knowledgeRecord", "regulatoryRequirement", "communicationRequirement", "resource", "statusRecord", "changeRecord", "process", "searchFilter", "accessRule", "privacyRequirement", "quantityRequirement", "analysisRecord", "relationship", "storageRequirement", "meetingRecord", "survey", "deliveryConstraint", "constraintRecord", "complianceRecord", "serviceRequirement", "equipment", "securityRequirement", "collaborationRecord", "safetyRequirement", "userProfile", "humanFactorRequirement", "flexibilityRequirement", "wayfindingRequirement", "programElement"] as const);
}

export type RegisterRow = { readonly register: "informationRequirement"; readonly row: InformationRequirement } | { readonly register: "sustainabilityRequirement"; readonly row: SustainabilityRequirement } | { readonly register: "accessibilityRequirement"; readonly row: AccessibilityRequirement } | { readonly register: "conflict"; readonly row: Conflict } | { readonly register: "optionEvaluation"; readonly row: OptionEvaluation } | { readonly register: "function"; readonly row: Function } | { readonly register: "risk"; readonly row: Risk } | { readonly register: "decision"; readonly row: Decision } | { readonly register: "validationRecord"; readonly row: ValidationRecord } | { readonly register: "priorityRecord"; readonly row: PriorityRecord } | { readonly register: "flowRequirement"; readonly row: FlowRequirement } | { readonly register: "environmentalRequirement"; readonly row: EnvironmentalRequirement } | { readonly register: "workshop"; readonly row: Workshop } | { readonly register: "scenario"; readonly row: Scenario } | { readonly register: "benchmarkRecord"; readonly row: BenchmarkRecord } | { readonly register: "activity"; readonly row: Activity } | { readonly register: "infrastructureRequirement"; readonly row: InfrastructureRequirement } | { readonly register: "organizationalRequirement"; readonly row: OrganizationalRequirement } | { readonly register: "issue"; readonly row: Issue } | { readonly register: "approvalRecord"; readonly row: ApprovalRecord } | { readonly register: "stakeholder"; readonly row: Stakeholder } | { readonly register: "qualityRecord"; readonly row: QualityRecord } | { readonly register: "resilienceRequirement"; readonly row: ResilienceRequirement } | { readonly register: "assumption"; readonly row: Assumption } | { readonly register: "costRequirement"; readonly row: CostRequirement } | { readonly register: "document"; readonly row: ArtifactRecord } | { readonly register: "scheduleRequirement"; readonly row: ScheduleRequirement } | { readonly register: "growthPlan"; readonly row: GrowthPlan } | { readonly register: "performanceCriterion"; readonly row: PerformanceCriterion } | { readonly register: "operationalRequirement"; readonly row: OperationalRequirement } | { readonly register: "requirement"; readonly row: Requirement } | { readonly register: "siteContext"; readonly row: SiteContext } | { readonly register: "templateRecord"; readonly row: TemplateRecord } | { readonly register: "reportRecord"; readonly row: ReportRecord } | { readonly register: "auditEvent"; readonly row: AuditEvent } | { readonly register: "knowledgeRecord"; readonly row: KnowledgeRecord } | { readonly register: "regulatoryRequirement"; readonly row: RegulatoryRequirement } | { readonly register: "communicationRequirement"; readonly row: CommunicationRequirement } | { readonly register: "resource"; readonly row: Resource } | { readonly register: "statusRecord"; readonly row: StatusRecord } | { readonly register: "changeRecord"; readonly row: ChangeRecord } | { readonly register: "process"; readonly row: Process } | { readonly register: "searchFilter"; readonly row: SearchFilter } | { readonly register: "accessRule"; readonly row: AccessRule } | { readonly register: "privacyRequirement"; readonly row: PrivacyRequirement } | { readonly register: "quantityRequirement"; readonly row: QuantityRequirement } | { readonly register: "analysisRecord"; readonly row: AnalysisRecord } | { readonly register: "relationship"; readonly row: Relationship } | { readonly register: "storageRequirement"; readonly row: StorageRequirement } | { readonly register: "meetingRecord"; readonly row: MeetingRecord } | { readonly register: "survey"; readonly row: Survey } | { readonly register: "deliveryConstraint"; readonly row: DeliveryConstraint } | { readonly register: "constraintRecord"; readonly row: ConstraintRecord } | { readonly register: "complianceRecord"; readonly row: ComplianceRecord } | { readonly register: "serviceRequirement"; readonly row: ServiceRequirement } | { readonly register: "equipment"; readonly row: Equipment } | { readonly register: "securityRequirement"; readonly row: SecurityRequirement } | { readonly register: "collaborationRecord"; readonly row: CollaborationRecord } | { readonly register: "safetyRequirement"; readonly row: SafetyRequirement } | { readonly register: "userProfile"; readonly row: UserProfile } | { readonly register: "humanFactorRequirement"; readonly row: HumanFactorRequirement } | { readonly register: "flexibilityRequirement"; readonly row: FlexibilityRequirement } | { readonly register: "wayfindingRequirement"; readonly row: WayfindingRequirement } | { readonly register: "programElement"; readonly row: ProgramElement };

export function parseRegisterRow(value: unknown, at = "$"): RegisterRow {
  return ((row0) => { switch (architectProgramSnapshotGuardMember(row0["register"], `${at}.register`, ["informationRequirement", "sustainabilityRequirement", "accessibilityRequirement", "conflict", "optionEvaluation", "function", "risk", "decision", "validationRecord", "priorityRecord", "flowRequirement", "environmentalRequirement", "workshop", "scenario", "benchmarkRecord", "activity", "infrastructureRequirement", "organizationalRequirement", "issue", "approvalRecord", "stakeholder", "qualityRecord", "resilienceRequirement", "assumption", "costRequirement", "document", "scheduleRequirement", "growthPlan", "performanceCriterion", "operationalRequirement", "requirement", "siteContext", "templateRecord", "reportRecord", "auditEvent", "knowledgeRecord", "regulatoryRequirement", "communicationRequirement", "resource", "statusRecord", "changeRecord", "process", "searchFilter", "accessRule", "privacyRequirement", "quantityRequirement", "analysisRecord", "relationship", "storageRequirement", "meetingRecord", "survey", "deliveryConstraint", "constraintRecord", "complianceRecord", "serviceRequirement", "equipment", "securityRequirement", "collaborationRecord", "safetyRequirement", "userProfile", "humanFactorRequirement", "flexibilityRequirement", "wayfindingRequirement", "programElement"] as const)) { case "informationRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "informationRequirement", row: parseInformationRequirement(row0["row"], `${at}.row`) }; case "sustainabilityRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "sustainabilityRequirement", row: parseSustainabilityRequirement(row0["row"], `${at}.row`) }; case "accessibilityRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "accessibilityRequirement", row: parseAccessibilityRequirement(row0["row"], `${at}.row`) }; case "conflict": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "conflict", row: parseConflict(row0["row"], `${at}.row`) }; case "optionEvaluation": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "optionEvaluation", row: parseOptionEvaluation(row0["row"], `${at}.row`) }; case "function": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "function", row: parseFunction(row0["row"], `${at}.row`) }; case "risk": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "risk", row: parseRisk(row0["row"], `${at}.row`) }; case "decision": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "decision", row: parseDecision(row0["row"], `${at}.row`) }; case "validationRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "validationRecord", row: parseValidationRecord(row0["row"], `${at}.row`) }; case "priorityRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "priorityRecord", row: parsePriorityRecord(row0["row"], `${at}.row`) }; case "flowRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "flowRequirement", row: parseFlowRequirement(row0["row"], `${at}.row`) }; case "environmentalRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "environmentalRequirement", row: parseEnvironmentalRequirement(row0["row"], `${at}.row`) }; case "workshop": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "workshop", row: parseWorkshop(row0["row"], `${at}.row`) }; case "scenario": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "scenario", row: parseScenario(row0["row"], `${at}.row`) }; case "benchmarkRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "benchmarkRecord", row: parseBenchmarkRecord(row0["row"], `${at}.row`) }; case "activity": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "activity", row: parseActivity(row0["row"], `${at}.row`) }; case "infrastructureRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "infrastructureRequirement", row: parseInfrastructureRequirement(row0["row"], `${at}.row`) }; case "organizationalRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "organizationalRequirement", row: parseOrganizationalRequirement(row0["row"], `${at}.row`) }; case "issue": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "issue", row: parseIssue(row0["row"], `${at}.row`) }; case "approvalRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "approvalRecord", row: parseApprovalRecord(row0["row"], `${at}.row`) }; case "stakeholder": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "stakeholder", row: parseStakeholder(row0["row"], `${at}.row`) }; case "qualityRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "qualityRecord", row: parseQualityRecord(row0["row"], `${at}.row`) }; case "resilienceRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "resilienceRequirement", row: parseResilienceRequirement(row0["row"], `${at}.row`) }; case "assumption": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "assumption", row: parseAssumption(row0["row"], `${at}.row`) }; case "costRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "costRequirement", row: parseCostRequirement(row0["row"], `${at}.row`) }; case "document": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "document", row: parseArtifactRecord(row0["row"], `${at}.row`) }; case "scheduleRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "scheduleRequirement", row: parseScheduleRequirement(row0["row"], `${at}.row`) }; case "growthPlan": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "growthPlan", row: parseGrowthPlan(row0["row"], `${at}.row`) }; case "performanceCriterion": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "performanceCriterion", row: parsePerformanceCriterion(row0["row"], `${at}.row`) }; case "operationalRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "operationalRequirement", row: parseOperationalRequirement(row0["row"], `${at}.row`) }; case "requirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "requirement", row: parseRequirement(row0["row"], `${at}.row`) }; case "siteContext": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "siteContext", row: parseSiteContext(row0["row"], `${at}.row`) }; case "templateRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "templateRecord", row: parseTemplateRecord(row0["row"], `${at}.row`) }; case "reportRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "reportRecord", row: parseReportRecord(row0["row"], `${at}.row`) }; case "auditEvent": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "auditEvent", row: parseAuditEvent(row0["row"], `${at}.row`) }; case "knowledgeRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "knowledgeRecord", row: parseKnowledgeRecord(row0["row"], `${at}.row`) }; case "regulatoryRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "regulatoryRequirement", row: parseRegulatoryRequirement(row0["row"], `${at}.row`) }; case "communicationRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "communicationRequirement", row: parseCommunicationRequirement(row0["row"], `${at}.row`) }; case "resource": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "resource", row: parseResource(row0["row"], `${at}.row`) }; case "statusRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "statusRecord", row: parseStatusRecord(row0["row"], `${at}.row`) }; case "changeRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "changeRecord", row: parseChangeRecord(row0["row"], `${at}.row`) }; case "process": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "process", row: parseProcess(row0["row"], `${at}.row`) }; case "searchFilter": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "searchFilter", row: parseSearchFilter(row0["row"], `${at}.row`) }; case "accessRule": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "accessRule", row: parseAccessRule(row0["row"], `${at}.row`) }; case "privacyRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "privacyRequirement", row: parsePrivacyRequirement(row0["row"], `${at}.row`) }; case "quantityRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "quantityRequirement", row: parseQuantityRequirement(row0["row"], `${at}.row`) }; case "analysisRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "analysisRecord", row: parseAnalysisRecord(row0["row"], `${at}.row`) }; case "relationship": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "relationship", row: parseRelationship(row0["row"], `${at}.row`) }; case "storageRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "storageRequirement", row: parseStorageRequirement(row0["row"], `${at}.row`) }; case "meetingRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "meetingRecord", row: parseMeetingRecord(row0["row"], `${at}.row`) }; case "survey": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "survey", row: parseSurvey(row0["row"], `${at}.row`) }; case "deliveryConstraint": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "deliveryConstraint", row: parseDeliveryConstraint(row0["row"], `${at}.row`) }; case "constraintRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "constraintRecord", row: parseConstraintRecord(row0["row"], `${at}.row`) }; case "complianceRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "complianceRecord", row: parseComplianceRecord(row0["row"], `${at}.row`) }; case "serviceRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "serviceRequirement", row: parseServiceRequirement(row0["row"], `${at}.row`) }; case "equipment": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "equipment", row: parseEquipment(row0["row"], `${at}.row`) }; case "securityRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "securityRequirement", row: parseSecurityRequirement(row0["row"], `${at}.row`) }; case "collaborationRecord": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "collaborationRecord", row: parseCollaborationRecord(row0["row"], `${at}.row`) }; case "safetyRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "safetyRequirement", row: parseSafetyRequirement(row0["row"], `${at}.row`) }; case "userProfile": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "userProfile", row: parseUserProfile(row0["row"], `${at}.row`) }; case "humanFactorRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "humanFactorRequirement", row: parseHumanFactorRequirement(row0["row"], `${at}.row`) }; case "flexibilityRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "flexibilityRequirement", row: parseFlexibilityRequirement(row0["row"], `${at}.row`) }; case "wayfindingRequirement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "wayfindingRequirement", row: parseWayfindingRequirement(row0["row"], `${at}.row`) }; case "programElement": architectProgramSnapshotGuardClosed(row0, `${at}`, ["register", "row"]); return { register: "programElement", row: parseProgramElement(row0["row"], `${at}.row`) }; } })(architectProgramSnapshotGuardObject(value, `${at}`));
}

export interface RegulatoryRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly jurisdiction: string;
  readonly code: string;
  readonly clause: string | null;
  readonly title: string;
  readonly requirementText: TextField;
  readonly applicability: readonly string[];
  readonly elementIds: readonly string[];
  readonly complianceMethod: string | null;
  readonly evidenceRequired: readonly string[];
  readonly authority: string | null;
  readonly effectiveDate: string | null;
  readonly expiryDate: string | null;
  readonly penalties: readonly string[];
  readonly exemptions: readonly string[];
  readonly relatedRequirementIds: readonly string[];
  readonly interpretationNotes: readonly TaggedNote[];
  readonly verificationStatus: ValidationStatus;
  readonly consultantRefs: readonly string[];
  readonly updateSource: string | null;
}

export function parseRegulatoryRequirement(value: unknown, at = "$"): RegulatoryRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "jurisdiction", "code", "clause", "title", "requirementText", "applicability", "elementIds", "complianceMethod", "evidenceRequired", "authority", "effectiveDate", "expiryDate", "penalties", "exemptions", "relatedRequirementIds", "interpretationNotes", "verificationStatus", "consultantRefs", "updateSource"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    jurisdiction: architectProgramSnapshotGuardString(row["jurisdiction"], `${at}.jurisdiction`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    clause: (row["clause"] === null ? null : architectProgramSnapshotGuardString(row["clause"], `${at}.clause`)),
    title: architectProgramSnapshotGuardString(row["title"], `${at}.title`),
    requirementText: parseTextField(row["requirementText"], `${at}.requirementText`),
    applicability: architectProgramSnapshotGuardArray(row["applicability"], `${at}.applicability`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.applicability[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    complianceMethod: (row["complianceMethod"] === null ? null : architectProgramSnapshotGuardString(row["complianceMethod"], `${at}.complianceMethod`)),
    evidenceRequired: architectProgramSnapshotGuardArray(row["evidenceRequired"], `${at}.evidenceRequired`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.evidenceRequired[${index}]`)),
    authority: (row["authority"] === null ? null : architectProgramSnapshotGuardString(row["authority"], `${at}.authority`)),
    effectiveDate: (row["effectiveDate"] === null ? null : architectProgramSnapshotGuardString(row["effectiveDate"], `${at}.effectiveDate`)),
    expiryDate: (row["expiryDate"] === null ? null : architectProgramSnapshotGuardString(row["expiryDate"], `${at}.expiryDate`)),
    penalties: architectProgramSnapshotGuardArray(row["penalties"], `${at}.penalties`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.penalties[${index}]`)),
    exemptions: architectProgramSnapshotGuardArray(row["exemptions"], `${at}.exemptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.exemptions[${index}]`)),
    relatedRequirementIds: architectProgramSnapshotGuardArray(row["relatedRequirementIds"], `${at}.relatedRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedRequirementIds[${index}]`)),
    interpretationNotes: architectProgramSnapshotGuardArray(row["interpretationNotes"], `${at}.interpretationNotes`).map((item, index) => parseTaggedNote(item, `${at}.interpretationNotes[${index}]`)),
    verificationStatus: parseValidationStatus(row["verificationStatus"], `${at}.verificationStatus`),
    consultantRefs: architectProgramSnapshotGuardArray(row["consultantRefs"], `${at}.consultantRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.consultantRefs[${index}]`)),
    updateSource: (row["updateSource"] === null ? null : architectProgramSnapshotGuardString(row["updateSource"], `${at}.updateSource`)),
  };
}

export interface Relationship {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly sourceId: string;
  readonly targetId: string;
  readonly kind: RelationshipKind;
  readonly strength: Binary64Transport | null;
  readonly directional: boolean;
  readonly rationale: TextField | null;
  readonly constraints: readonly string[];
  readonly conditions: readonly string[];
  readonly relationshipPriority: Priority;
  readonly validFrom: string | null;
  readonly validUntil: string | null;
  readonly evidence: readonly string[];
  readonly conflictIds: readonly string[];
  readonly traceLinks: readonly TraceLink[];
  readonly bidirectional: boolean;
  readonly distanceConstraintM: Binary64Transport | null;
  readonly capacityConstraint: string | null;
  readonly regulatoryBasis: readonly string[];
  readonly reviewCycle: string | null;
  readonly ownerId: string | null;
  readonly proximityRequirement: TextField | null;
  readonly compatibilityRequirement: TextField | null;
  readonly incompatibilityRequirement: TextField | null;
  readonly separationRequirements: readonly SeparationKind[];
}

export function parseRelationship(value: unknown, at = "$"): Relationship {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "sourceId", "targetId", "kind", "strength", "directional", "rationale", "constraints", "conditions", "relationshipPriority", "validFrom", "validUntil", "evidence", "conflictIds", "traceLinks", "bidirectional", "distanceConstraintM", "capacityConstraint", "regulatoryBasis", "reviewCycle", "ownerId", "proximityRequirement", "compatibilityRequirement", "incompatibilityRequirement", "separationRequirements"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    sourceId: architectProgramSnapshotGuardString(row["sourceId"], `${at}.sourceId`),
    targetId: architectProgramSnapshotGuardString(row["targetId"], `${at}.targetId`),
    kind: parseRelationshipKind(row["kind"], `${at}.kind`),
    strength: (row["strength"] === null ? null : parseBinary64Transport(row["strength"], `${at}.strength`)),
    directional: architectProgramSnapshotGuardBoolean(row["directional"], `${at}.directional`),
    rationale: (row["rationale"] === null ? null : parseTextField(row["rationale"], `${at}.rationale`)),
    constraints: architectProgramSnapshotGuardArray(row["constraints"], `${at}.constraints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.constraints[${index}]`)),
    conditions: architectProgramSnapshotGuardArray(row["conditions"], `${at}.conditions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.conditions[${index}]`)),
    relationshipPriority: parsePriority(row["relationshipPriority"], `${at}.relationshipPriority`),
    validFrom: (row["validFrom"] === null ? null : architectProgramSnapshotGuardString(row["validFrom"], `${at}.validFrom`)),
    validUntil: (row["validUntil"] === null ? null : architectProgramSnapshotGuardString(row["validUntil"], `${at}.validUntil`)),
    evidence: architectProgramSnapshotGuardArray(row["evidence"], `${at}.evidence`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.evidence[${index}]`)),
    conflictIds: architectProgramSnapshotGuardArray(row["conflictIds"], `${at}.conflictIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.conflictIds[${index}]`)),
    traceLinks: architectProgramSnapshotGuardArray(row["traceLinks"], `${at}.traceLinks`).map((item, index) => parseTraceLink(item, `${at}.traceLinks[${index}]`)),
    bidirectional: architectProgramSnapshotGuardBoolean(row["bidirectional"], `${at}.bidirectional`),
    distanceConstraintM: (row["distanceConstraintM"] === null ? null : parseBinary64Transport(row["distanceConstraintM"], `${at}.distanceConstraintM`)),
    capacityConstraint: (row["capacityConstraint"] === null ? null : architectProgramSnapshotGuardString(row["capacityConstraint"], `${at}.capacityConstraint`)),
    regulatoryBasis: architectProgramSnapshotGuardArray(row["regulatoryBasis"], `${at}.regulatoryBasis`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryBasis[${index}]`)),
    reviewCycle: (row["reviewCycle"] === null ? null : architectProgramSnapshotGuardString(row["reviewCycle"], `${at}.reviewCycle`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    proximityRequirement: (row["proximityRequirement"] === null ? null : parseTextField(row["proximityRequirement"], `${at}.proximityRequirement`)),
    compatibilityRequirement: (row["compatibilityRequirement"] === null ? null : parseTextField(row["compatibilityRequirement"], `${at}.compatibilityRequirement`)),
    incompatibilityRequirement: (row["incompatibilityRequirement"] === null ? null : parseTextField(row["incompatibilityRequirement"], `${at}.incompatibilityRequirement`)),
    separationRequirements: architectProgramSnapshotGuardArray(row["separationRequirements"], `${at}.separationRequirements`).map((item, index) => parseSeparationKind(item, `${at}.separationRequirements[${index}]`)),
  };
}

export type RelationshipKind = "contains" | "serves" | "supports" | "dependsOn" | "conflictsWith" | "equivalentTo" | "adjacentTo" | "feeds" | "receives" | "controls" | "monitors" | "functional" | "operational" | "organizational" | "user" | "service" | "information" | "access" | "security" | "supervision" | "communication" | "dependency" | "sequential" | "sharedResource";

export function parseRelationshipKind(value: unknown, at = "$"): RelationshipKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["contains", "serves", "supports", "dependsOn", "conflictsWith", "equivalentTo", "adjacentTo", "feeds", "receives", "controls", "monitors", "functional", "operational", "organizational", "user", "service", "information", "access", "security", "supervision", "communication", "dependency", "sequential", "sharedResource"] as const);
}

export type ReportKind = "executiveSummary" | "programOverview" | "stakeholderSummary" | "requirementsMatrix" | "adjacencyMatrix" | "gapAnalysis" | "riskRegister" | "decisionLog" | "validationSummary" | "recommendation" | "userSummary" | "functionalSummary" | "capacitySummary" | "workflowSummary" | "complianceSummary" | "costSummary" | "scheduleSummary" | "changeSummary" | "openIssueSummary" | "prioritySummary" | "scenarioSummary";

export function parseReportKind(value: unknown, at = "$"): ReportKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["executiveSummary", "programOverview", "stakeholderSummary", "requirementsMatrix", "adjacencyMatrix", "gapAnalysis", "riskRegister", "decisionLog", "validationSummary", "recommendation", "userSummary", "functionalSummary", "capacitySummary", "workflowSummary", "complianceSummary", "costSummary", "scheduleSummary", "changeSummary", "openIssueSummary", "prioritySummary", "scenarioSummary"] as const);
}

export interface ReportRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly kind: ReportKind;
  readonly title: string;
  readonly audience: readonly string[];
  readonly sections: readonly string[];
  readonly generatedAt: string | null;
  readonly generatedBy: string | null;
  readonly analysisIds: readonly string[];
  readonly format: string | null;
  readonly fileRef: string | null;
  readonly distributionList: readonly string[];
  readonly approvalStatus: ValidationStatus;
  readonly approverId: string | null;
  readonly version: string;
  readonly templateId: string | null;
  readonly parameters: readonly string[];
  readonly confidentiality: string | null;
  readonly expiryDate: string | null;
  readonly relatedDecisionIds: readonly string[];
}

export function parseReportRecord(value: unknown, at = "$"): ReportRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "kind", "title", "audience", "sections", "generatedAt", "generatedBy", "analysisIds", "format", "fileRef", "distributionList", "approvalStatus", "approverId", "version", "templateId", "parameters", "confidentiality", "expiryDate", "relatedDecisionIds"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    kind: parseReportKind(row["kind"], `${at}.kind`),
    title: architectProgramSnapshotGuardString(row["title"], `${at}.title`),
    audience: architectProgramSnapshotGuardArray(row["audience"], `${at}.audience`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.audience[${index}]`)),
    sections: architectProgramSnapshotGuardArray(row["sections"], `${at}.sections`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.sections[${index}]`)),
    generatedAt: (row["generatedAt"] === null ? null : architectProgramSnapshotGuardString(row["generatedAt"], `${at}.generatedAt`)),
    generatedBy: (row["generatedBy"] === null ? null : architectProgramSnapshotGuardString(row["generatedBy"], `${at}.generatedBy`)),
    analysisIds: architectProgramSnapshotGuardArray(row["analysisIds"], `${at}.analysisIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.analysisIds[${index}]`)),
    format: (row["format"] === null ? null : architectProgramSnapshotGuardString(row["format"], `${at}.format`)),
    fileRef: (row["fileRef"] === null ? null : architectProgramSnapshotGuardString(row["fileRef"], `${at}.fileRef`)),
    distributionList: architectProgramSnapshotGuardArray(row["distributionList"], `${at}.distributionList`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.distributionList[${index}]`)),
    approvalStatus: parseValidationStatus(row["approvalStatus"], `${at}.approvalStatus`),
    approverId: (row["approverId"] === null ? null : architectProgramSnapshotGuardString(row["approverId"], `${at}.approverId`)),
    version: architectProgramSnapshotGuardString(row["version"], `${at}.version`),
    templateId: (row["templateId"] === null ? null : architectProgramSnapshotGuardString(row["templateId"], `${at}.templateId`)),
    parameters: architectProgramSnapshotGuardArray(row["parameters"], `${at}.parameters`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.parameters[${index}]`)),
    confidentiality: (row["confidentiality"] === null ? null : architectProgramSnapshotGuardString(row["confidentiality"], `${at}.confidentiality`)),
    expiryDate: (row["expiryDate"] === null ? null : architectProgramSnapshotGuardString(row["expiryDate"], `${at}.expiryDate`)),
    relatedDecisionIds: architectProgramSnapshotGuardArray(row["relatedDecisionIds"], `${at}.relatedDecisionIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedDecisionIds[${index}]`)),
  };
}

export interface Requirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly code: string;
  readonly kind: RequirementKind;
  readonly statement: TextField;
  readonly rationale: TextField | null;
  readonly source: string | null;
  readonly stakeholderIds: readonly string[];
  readonly elementIds: readonly string[];
  readonly functionIds: readonly string[];
  readonly parentRequirementId: string | null;
  readonly childRequirementIds: readonly string[];
  readonly acceptanceCriteria: readonly string[];
  readonly verificationMethod: string | null;
  readonly validationStatus: ValidationStatus;
  readonly conflictIds: readonly string[];
  readonly riskIds: readonly string[];
  readonly costEstimate: Binary64Transport | null;
  readonly scheduleConstraint: string | null;
  readonly regulatoryRefs: readonly string[];
  readonly traceLinks: readonly TraceLink[];
  readonly supersededBy: string | null;
}

export function parseRequirement(value: unknown, at = "$"): Requirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "code", "kind", "statement", "rationale", "source", "stakeholderIds", "elementIds", "functionIds", "parentRequirementId", "childRequirementIds", "acceptanceCriteria", "verificationMethod", "validationStatus", "conflictIds", "riskIds", "costEstimate", "scheduleConstraint", "regulatoryRefs", "traceLinks", "supersededBy"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    kind: parseRequirementKind(row["kind"], `${at}.kind`),
    statement: parseTextField(row["statement"], `${at}.statement`),
    rationale: (row["rationale"] === null ? null : parseTextField(row["rationale"], `${at}.rationale`)),
    source: (row["source"] === null ? null : architectProgramSnapshotGuardString(row["source"], `${at}.source`)),
    stakeholderIds: architectProgramSnapshotGuardArray(row["stakeholderIds"], `${at}.stakeholderIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderIds[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    functionIds: architectProgramSnapshotGuardArray(row["functionIds"], `${at}.functionIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.functionIds[${index}]`)),
    parentRequirementId: (row["parentRequirementId"] === null ? null : architectProgramSnapshotGuardString(row["parentRequirementId"], `${at}.parentRequirementId`)),
    childRequirementIds: architectProgramSnapshotGuardArray(row["childRequirementIds"], `${at}.childRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.childRequirementIds[${index}]`)),
    acceptanceCriteria: architectProgramSnapshotGuardArray(row["acceptanceCriteria"], `${at}.acceptanceCriteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.acceptanceCriteria[${index}]`)),
    verificationMethod: (row["verificationMethod"] === null ? null : architectProgramSnapshotGuardString(row["verificationMethod"], `${at}.verificationMethod`)),
    validationStatus: parseValidationStatus(row["validationStatus"], `${at}.validationStatus`),
    conflictIds: architectProgramSnapshotGuardArray(row["conflictIds"], `${at}.conflictIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.conflictIds[${index}]`)),
    riskIds: architectProgramSnapshotGuardArray(row["riskIds"], `${at}.riskIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.riskIds[${index}]`)),
    costEstimate: (row["costEstimate"] === null ? null : parseBinary64Transport(row["costEstimate"], `${at}.costEstimate`)),
    scheduleConstraint: (row["scheduleConstraint"] === null ? null : architectProgramSnapshotGuardString(row["scheduleConstraint"], `${at}.scheduleConstraint`)),
    regulatoryRefs: architectProgramSnapshotGuardArray(row["regulatoryRefs"], `${at}.regulatoryRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryRefs[${index}]`)),
    traceLinks: architectProgramSnapshotGuardArray(row["traceLinks"], `${at}.traceLinks`).map((item, index) => parseTraceLink(item, `${at}.traceLinks[${index}]`)),
    supersededBy: (row["supersededBy"] === null ? null : architectProgramSnapshotGuardString(row["supersededBy"], `${at}.supersededBy`)),
  };
}

export type RequirementKind = "functional" | "spatial" | "performance" | "regulatory" | "operational" | "technical" | "aesthetic" | "sustainability";

export function parseRequirementKind(value: unknown, at = "$"): RequirementKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["functional", "spatial", "performance", "regulatory", "operational", "technical", "aesthetic", "sustainability"] as const);
}

export interface ResilienceRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly hazard: string;
  readonly riskLevel: RiskLevel;
  readonly scenario: string | null;
  readonly recoveryTime: string | null;
  readonly recoveryPoint: string | null;
  readonly redundancy: readonly string[];
  readonly hardeningMeasures: readonly string[];
  readonly backupSystems: readonly string[];
  readonly alternateSites: readonly string[];
  readonly supplyChain: readonly string[];
  readonly communicationPlan: readonly string[];
  readonly drillRequirements: readonly string[];
  readonly elementIds: readonly string[];
  readonly infrastructureIds: readonly string[];
  readonly standards: readonly string[];
  readonly insuranceImplications: readonly string[];
  readonly climateAdaptation: readonly string[];
  readonly ownerId: string | null;
  readonly verificationPlan: string | null;
}

export function parseResilienceRequirement(value: unknown, at = "$"): ResilienceRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "hazard", "riskLevel", "scenario", "recoveryTime", "recoveryPoint", "redundancy", "hardeningMeasures", "backupSystems", "alternateSites", "supplyChain", "communicationPlan", "drillRequirements", "elementIds", "infrastructureIds", "standards", "insuranceImplications", "climateAdaptation", "ownerId", "verificationPlan"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    hazard: architectProgramSnapshotGuardString(row["hazard"], `${at}.hazard`),
    riskLevel: parseRiskLevel(row["riskLevel"], `${at}.riskLevel`),
    scenario: (row["scenario"] === null ? null : architectProgramSnapshotGuardString(row["scenario"], `${at}.scenario`)),
    recoveryTime: (row["recoveryTime"] === null ? null : architectProgramSnapshotGuardString(row["recoveryTime"], `${at}.recoveryTime`)),
    recoveryPoint: (row["recoveryPoint"] === null ? null : architectProgramSnapshotGuardString(row["recoveryPoint"], `${at}.recoveryPoint`)),
    redundancy: architectProgramSnapshotGuardArray(row["redundancy"], `${at}.redundancy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.redundancy[${index}]`)),
    hardeningMeasures: architectProgramSnapshotGuardArray(row["hardeningMeasures"], `${at}.hardeningMeasures`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.hardeningMeasures[${index}]`)),
    backupSystems: architectProgramSnapshotGuardArray(row["backupSystems"], `${at}.backupSystems`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.backupSystems[${index}]`)),
    alternateSites: architectProgramSnapshotGuardArray(row["alternateSites"], `${at}.alternateSites`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.alternateSites[${index}]`)),
    supplyChain: architectProgramSnapshotGuardArray(row["supplyChain"], `${at}.supplyChain`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.supplyChain[${index}]`)),
    communicationPlan: architectProgramSnapshotGuardArray(row["communicationPlan"], `${at}.communicationPlan`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.communicationPlan[${index}]`)),
    drillRequirements: architectProgramSnapshotGuardArray(row["drillRequirements"], `${at}.drillRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.drillRequirements[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    infrastructureIds: architectProgramSnapshotGuardArray(row["infrastructureIds"], `${at}.infrastructureIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.infrastructureIds[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    insuranceImplications: architectProgramSnapshotGuardArray(row["insuranceImplications"], `${at}.insuranceImplications`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.insuranceImplications[${index}]`)),
    climateAdaptation: architectProgramSnapshotGuardArray(row["climateAdaptation"], `${at}.climateAdaptation`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.climateAdaptation[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    verificationPlan: (row["verificationPlan"] === null ? null : architectProgramSnapshotGuardString(row["verificationPlan"], `${at}.verificationPlan`)),
  };
}

export interface Resource {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly code: string;
  readonly category: string;
  readonly resourceType: string;
  readonly quantity: QuantitySpec;
  readonly mobility: string | null;
  readonly sharingModel: string | null;
  readonly allocation: string | null;
  readonly elementIds: readonly string[];
  readonly activityIds: readonly string[];
  readonly userProfileIds: readonly string[];
  readonly storageRequirementId: string | null;
  readonly durability: string | null;
  readonly cleaningRequirements: readonly string[];
  readonly replacementCycle: string | null;
  readonly costPerUnit: Binary64Transport | null;
  readonly supplier: string | null;
  readonly standards: readonly string[];
  readonly ergonomicNotes: readonly string[];
  readonly customization: readonly string[];
  readonly disposalNotes: readonly string[];
  readonly furnitureClass: string | null;
  readonly ergonomicsRating: string | null;
  readonly sharingRatio: Binary64Transport | null;
}

export function parseResource(value: unknown, at = "$"): Resource {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "code", "category", "resourceType", "quantity", "mobility", "sharingModel", "allocation", "elementIds", "activityIds", "userProfileIds", "storageRequirementId", "durability", "cleaningRequirements", "replacementCycle", "costPerUnit", "supplier", "standards", "ergonomicNotes", "customization", "disposalNotes", "furnitureClass", "ergonomicsRating", "sharingRatio"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    category: architectProgramSnapshotGuardString(row["category"], `${at}.category`),
    resourceType: architectProgramSnapshotGuardString(row["resourceType"], `${at}.resourceType`),
    quantity: parseQuantitySpec(row["quantity"], `${at}.quantity`),
    mobility: (row["mobility"] === null ? null : architectProgramSnapshotGuardString(row["mobility"], `${at}.mobility`)),
    sharingModel: (row["sharingModel"] === null ? null : architectProgramSnapshotGuardString(row["sharingModel"], `${at}.sharingModel`)),
    allocation: (row["allocation"] === null ? null : architectProgramSnapshotGuardString(row["allocation"], `${at}.allocation`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    activityIds: architectProgramSnapshotGuardArray(row["activityIds"], `${at}.activityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.activityIds[${index}]`)),
    userProfileIds: architectProgramSnapshotGuardArray(row["userProfileIds"], `${at}.userProfileIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.userProfileIds[${index}]`)),
    storageRequirementId: (row["storageRequirementId"] === null ? null : architectProgramSnapshotGuardString(row["storageRequirementId"], `${at}.storageRequirementId`)),
    durability: (row["durability"] === null ? null : architectProgramSnapshotGuardString(row["durability"], `${at}.durability`)),
    cleaningRequirements: architectProgramSnapshotGuardArray(row["cleaningRequirements"], `${at}.cleaningRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.cleaningRequirements[${index}]`)),
    replacementCycle: (row["replacementCycle"] === null ? null : architectProgramSnapshotGuardString(row["replacementCycle"], `${at}.replacementCycle`)),
    costPerUnit: (row["costPerUnit"] === null ? null : parseBinary64Transport(row["costPerUnit"], `${at}.costPerUnit`)),
    supplier: (row["supplier"] === null ? null : architectProgramSnapshotGuardString(row["supplier"], `${at}.supplier`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    ergonomicNotes: architectProgramSnapshotGuardArray(row["ergonomicNotes"], `${at}.ergonomicNotes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.ergonomicNotes[${index}]`)),
    customization: architectProgramSnapshotGuardArray(row["customization"], `${at}.customization`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.customization[${index}]`)),
    disposalNotes: architectProgramSnapshotGuardArray(row["disposalNotes"], `${at}.disposalNotes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.disposalNotes[${index}]`)),
    furnitureClass: (row["furnitureClass"] === null ? null : architectProgramSnapshotGuardString(row["furnitureClass"], `${at}.furnitureClass`)),
    ergonomicsRating: (row["ergonomicsRating"] === null ? null : architectProgramSnapshotGuardString(row["ergonomicsRating"], `${at}.ergonomicsRating`)),
    sharingRatio: (row["sharingRatio"] === null ? null : parseBinary64Transport(row["sharingRatio"], `${at}.sharingRatio`)),
  };
}

export interface Risk {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly riskStatement: TextField;
  readonly category: string;
  readonly probability: RiskLevel;
  readonly impact: RiskLevel;
  readonly riskScore: Binary64Transport | null;
  readonly causes: readonly string[];
  readonly effects: readonly string[];
  readonly affectedElementIds: readonly string[];
  readonly affectedRequirementIds: readonly string[];
  readonly mitigation: readonly string[];
  readonly contingency: readonly string[];
  readonly ownerId: string | null;
  readonly reviewDate: string | null;
  readonly triggerIndicators: readonly string[];
  readonly residualProbability: RiskLevel | null;
  readonly residualImpact: RiskLevel | null;
  readonly relatedConflictIds: readonly string[];
  readonly escalationPath: readonly string[];
  readonly monitoringPlan: string | null;
}

export function parseRisk(value: unknown, at = "$"): Risk {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "riskStatement", "category", "probability", "impact", "riskScore", "causes", "effects", "affectedElementIds", "affectedRequirementIds", "mitigation", "contingency", "ownerId", "reviewDate", "triggerIndicators", "residualProbability", "residualImpact", "relatedConflictIds", "escalationPath", "monitoringPlan"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    riskStatement: parseTextField(row["riskStatement"], `${at}.riskStatement`),
    category: architectProgramSnapshotGuardString(row["category"], `${at}.category`),
    probability: parseRiskLevel(row["probability"], `${at}.probability`),
    impact: parseRiskLevel(row["impact"], `${at}.impact`),
    riskScore: (row["riskScore"] === null ? null : parseBinary64Transport(row["riskScore"], `${at}.riskScore`)),
    causes: architectProgramSnapshotGuardArray(row["causes"], `${at}.causes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.causes[${index}]`)),
    effects: architectProgramSnapshotGuardArray(row["effects"], `${at}.effects`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.effects[${index}]`)),
    affectedElementIds: architectProgramSnapshotGuardArray(row["affectedElementIds"], `${at}.affectedElementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.affectedElementIds[${index}]`)),
    affectedRequirementIds: architectProgramSnapshotGuardArray(row["affectedRequirementIds"], `${at}.affectedRequirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.affectedRequirementIds[${index}]`)),
    mitigation: architectProgramSnapshotGuardArray(row["mitigation"], `${at}.mitigation`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.mitigation[${index}]`)),
    contingency: architectProgramSnapshotGuardArray(row["contingency"], `${at}.contingency`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.contingency[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    reviewDate: (row["reviewDate"] === null ? null : architectProgramSnapshotGuardString(row["reviewDate"], `${at}.reviewDate`)),
    triggerIndicators: architectProgramSnapshotGuardArray(row["triggerIndicators"], `${at}.triggerIndicators`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.triggerIndicators[${index}]`)),
    residualProbability: (row["residualProbability"] === null ? null : parseRiskLevel(row["residualProbability"], `${at}.residualProbability`)),
    residualImpact: (row["residualImpact"] === null ? null : parseRiskLevel(row["residualImpact"], `${at}.residualImpact`)),
    relatedConflictIds: architectProgramSnapshotGuardArray(row["relatedConflictIds"], `${at}.relatedConflictIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedConflictIds[${index}]`)),
    escalationPath: architectProgramSnapshotGuardArray(row["escalationPath"], `${at}.escalationPath`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.escalationPath[${index}]`)),
    monitoringPlan: (row["monitoringPlan"] === null ? null : architectProgramSnapshotGuardString(row["monitoringPlan"], `${at}.monitoringPlan`)),
  };
}

export type RiskLevel = "negligible" | "low" | "medium" | "high" | "critical";

export function parseRiskLevel(value: unknown, at = "$"): RiskLevel {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["negligible", "low", "medium", "high", "critical"] as const);
}

export type SafetyDomain = "lifeSafety" | "occupationalHealth" | "fire" | "structural" | "electrical" | "chemical" | "radiation" | "ergonomics" | "biological" | "environmental";

export function parseSafetyDomain(value: unknown, at = "$"): SafetyDomain {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["lifeSafety", "occupationalHealth", "fire", "structural", "electrical", "chemical", "radiation", "ergonomics", "biological", "environmental"] as const);
}

export interface SafetyRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly safetyDomain: SafetyDomain;
  readonly hazard: string;
  readonly riskLevel: RiskLevel;
  readonly affectedElementIds: readonly string[];
  readonly affectedUserIds: readonly string[];
  readonly mitigationMeasures: readonly string[];
  readonly ppeRequirements: readonly string[];
  readonly emergencyProcedures: readonly string[];
  readonly evacuationRequirements: readonly string[];
  readonly fireProtection: readonly string[];
  readonly structuralSafety: readonly string[];
  readonly slipTripFall: readonly string[];
  readonly chemicalSafety: readonly string[];
  readonly electricalSafety: readonly string[];
  readonly machinerySafety: readonly string[];
  readonly standards: readonly string[];
  readonly inspectionFrequency: string | null;
  readonly trainingRequirements: readonly string[];
  readonly incidentReporting: readonly string[];
  readonly residualRisk: string | null;
}

export function parseSafetyRequirement(value: unknown, at = "$"): SafetyRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "safetyDomain", "hazard", "riskLevel", "affectedElementIds", "affectedUserIds", "mitigationMeasures", "ppeRequirements", "emergencyProcedures", "evacuationRequirements", "fireProtection", "structuralSafety", "slipTripFall", "chemicalSafety", "electricalSafety", "machinerySafety", "standards", "inspectionFrequency", "trainingRequirements", "incidentReporting", "residualRisk"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    safetyDomain: parseSafetyDomain(row["safetyDomain"], `${at}.safetyDomain`),
    hazard: architectProgramSnapshotGuardString(row["hazard"], `${at}.hazard`),
    riskLevel: parseRiskLevel(row["riskLevel"], `${at}.riskLevel`),
    affectedElementIds: architectProgramSnapshotGuardArray(row["affectedElementIds"], `${at}.affectedElementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.affectedElementIds[${index}]`)),
    affectedUserIds: architectProgramSnapshotGuardArray(row["affectedUserIds"], `${at}.affectedUserIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.affectedUserIds[${index}]`)),
    mitigationMeasures: architectProgramSnapshotGuardArray(row["mitigationMeasures"], `${at}.mitigationMeasures`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.mitigationMeasures[${index}]`)),
    ppeRequirements: architectProgramSnapshotGuardArray(row["ppeRequirements"], `${at}.ppeRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.ppeRequirements[${index}]`)),
    emergencyProcedures: architectProgramSnapshotGuardArray(row["emergencyProcedures"], `${at}.emergencyProcedures`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.emergencyProcedures[${index}]`)),
    evacuationRequirements: architectProgramSnapshotGuardArray(row["evacuationRequirements"], `${at}.evacuationRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.evacuationRequirements[${index}]`)),
    fireProtection: architectProgramSnapshotGuardArray(row["fireProtection"], `${at}.fireProtection`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.fireProtection[${index}]`)),
    structuralSafety: architectProgramSnapshotGuardArray(row["structuralSafety"], `${at}.structuralSafety`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.structuralSafety[${index}]`)),
    slipTripFall: architectProgramSnapshotGuardArray(row["slipTripFall"], `${at}.slipTripFall`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.slipTripFall[${index}]`)),
    chemicalSafety: architectProgramSnapshotGuardArray(row["chemicalSafety"], `${at}.chemicalSafety`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.chemicalSafety[${index}]`)),
    electricalSafety: architectProgramSnapshotGuardArray(row["electricalSafety"], `${at}.electricalSafety`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.electricalSafety[${index}]`)),
    machinerySafety: architectProgramSnapshotGuardArray(row["machinerySafety"], `${at}.machinerySafety`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.machinerySafety[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    inspectionFrequency: (row["inspectionFrequency"] === null ? null : architectProgramSnapshotGuardString(row["inspectionFrequency"], `${at}.inspectionFrequency`)),
    trainingRequirements: architectProgramSnapshotGuardArray(row["trainingRequirements"], `${at}.trainingRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.trainingRequirements[${index}]`)),
    incidentReporting: architectProgramSnapshotGuardArray(row["incidentReporting"], `${at}.incidentReporting`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.incidentReporting[${index}]`)),
    residualRisk: (row["residualRisk"] === null ? null : architectProgramSnapshotGuardString(row["residualRisk"], `${at}.residualRisk`)),
  };
}

export interface Scenario {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly code: string;
  readonly hypothesis: TextField;
  readonly assumptions: readonly string[];
  readonly variables: readonly string[];
  readonly elementIds: readonly string[];
  readonly requirementIds: readonly string[];
  readonly growthPlanId: string | null;
  readonly probability: Binary64Transport | null;
  readonly impactSummary: TextField | null;
  readonly costDelta: Binary64Transport | null;
  readonly areaDelta: Binary64Transport | null;
  readonly headcountDelta: Binary64Transport | null;
  readonly scheduleDelta: string | null;
  readonly riskIds: readonly string[];
  readonly optionIds: readonly string[];
  readonly baseline: boolean;
  readonly preferred: boolean;
  readonly analysisIds: readonly string[];
  readonly ownerId: string | null;
}

export function parseScenario(value: unknown, at = "$"): Scenario {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "code", "hypothesis", "assumptions", "variables", "elementIds", "requirementIds", "growthPlanId", "probability", "impactSummary", "costDelta", "areaDelta", "headcountDelta", "scheduleDelta", "riskIds", "optionIds", "baseline", "preferred", "analysisIds", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    code: architectProgramSnapshotGuardString(row["code"], `${at}.code`),
    hypothesis: parseTextField(row["hypothesis"], `${at}.hypothesis`),
    assumptions: architectProgramSnapshotGuardArray(row["assumptions"], `${at}.assumptions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.assumptions[${index}]`)),
    variables: architectProgramSnapshotGuardArray(row["variables"], `${at}.variables`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.variables[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    requirementIds: architectProgramSnapshotGuardArray(row["requirementIds"], `${at}.requirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.requirementIds[${index}]`)),
    growthPlanId: (row["growthPlanId"] === null ? null : architectProgramSnapshotGuardString(row["growthPlanId"], `${at}.growthPlanId`)),
    probability: (row["probability"] === null ? null : parseBinary64Transport(row["probability"], `${at}.probability`)),
    impactSummary: (row["impactSummary"] === null ? null : parseTextField(row["impactSummary"], `${at}.impactSummary`)),
    costDelta: (row["costDelta"] === null ? null : parseBinary64Transport(row["costDelta"], `${at}.costDelta`)),
    areaDelta: (row["areaDelta"] === null ? null : parseBinary64Transport(row["areaDelta"], `${at}.areaDelta`)),
    headcountDelta: (row["headcountDelta"] === null ? null : parseBinary64Transport(row["headcountDelta"], `${at}.headcountDelta`)),
    scheduleDelta: (row["scheduleDelta"] === null ? null : architectProgramSnapshotGuardString(row["scheduleDelta"], `${at}.scheduleDelta`)),
    riskIds: architectProgramSnapshotGuardArray(row["riskIds"], `${at}.riskIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.riskIds[${index}]`)),
    optionIds: architectProgramSnapshotGuardArray(row["optionIds"], `${at}.optionIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.optionIds[${index}]`)),
    baseline: architectProgramSnapshotGuardBoolean(row["baseline"], `${at}.baseline`),
    preferred: architectProgramSnapshotGuardBoolean(row["preferred"], `${at}.preferred`),
    analysisIds: architectProgramSnapshotGuardArray(row["analysisIds"], `${at}.analysisIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.analysisIds[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export interface ScheduleRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly milestone: string;
  readonly phase: DeliveryPhase;
  readonly startDate: string | null;
  readonly endDate: string | null;
  readonly duration: string | null;
  readonly dependencies: readonly string[];
  readonly predecessors: readonly string[];
  readonly successors: readonly string[];
  readonly critical: boolean;
  readonly floatDays: number | null;
  readonly resourceRequirements: readonly string[];
  readonly occupancyImpact: readonly string[];
  readonly phasingStrategy: string | null;
  readonly decantRequirements: readonly string[];
  readonly commissioningWindow: string | null;
  readonly stakeholderIds: readonly string[];
  readonly riskIds: readonly string[];
  readonly contingencyDays: number | null;
  readonly reportingCadence: string | null;
  readonly ownerId: string | null;
}

export function parseScheduleRequirement(value: unknown, at = "$"): ScheduleRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "milestone", "phase", "startDate", "endDate", "duration", "dependencies", "predecessors", "successors", "critical", "floatDays", "resourceRequirements", "occupancyImpact", "phasingStrategy", "decantRequirements", "commissioningWindow", "stakeholderIds", "riskIds", "contingencyDays", "reportingCadence", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    milestone: architectProgramSnapshotGuardString(row["milestone"], `${at}.milestone`),
    phase: parseDeliveryPhase(row["phase"], `${at}.phase`),
    startDate: (row["startDate"] === null ? null : architectProgramSnapshotGuardString(row["startDate"], `${at}.startDate`)),
    endDate: (row["endDate"] === null ? null : architectProgramSnapshotGuardString(row["endDate"], `${at}.endDate`)),
    duration: (row["duration"] === null ? null : architectProgramSnapshotGuardString(row["duration"], `${at}.duration`)),
    dependencies: architectProgramSnapshotGuardArray(row["dependencies"], `${at}.dependencies`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dependencies[${index}]`)),
    predecessors: architectProgramSnapshotGuardArray(row["predecessors"], `${at}.predecessors`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.predecessors[${index}]`)),
    successors: architectProgramSnapshotGuardArray(row["successors"], `${at}.successors`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.successors[${index}]`)),
    critical: architectProgramSnapshotGuardBoolean(row["critical"], `${at}.critical`),
    floatDays: (row["floatDays"] === null ? null : architectProgramSnapshotGuardInteger(row["floatDays"], `${at}.floatDays`, {"minimum": 0, "maximum": 4294967295})),
    resourceRequirements: architectProgramSnapshotGuardArray(row["resourceRequirements"], `${at}.resourceRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.resourceRequirements[${index}]`)),
    occupancyImpact: architectProgramSnapshotGuardArray(row["occupancyImpact"], `${at}.occupancyImpact`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.occupancyImpact[${index}]`)),
    phasingStrategy: (row["phasingStrategy"] === null ? null : architectProgramSnapshotGuardString(row["phasingStrategy"], `${at}.phasingStrategy`)),
    decantRequirements: architectProgramSnapshotGuardArray(row["decantRequirements"], `${at}.decantRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decantRequirements[${index}]`)),
    commissioningWindow: (row["commissioningWindow"] === null ? null : architectProgramSnapshotGuardString(row["commissioningWindow"], `${at}.commissioningWindow`)),
    stakeholderIds: architectProgramSnapshotGuardArray(row["stakeholderIds"], `${at}.stakeholderIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderIds[${index}]`)),
    riskIds: architectProgramSnapshotGuardArray(row["riskIds"], `${at}.riskIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.riskIds[${index}]`)),
    contingencyDays: (row["contingencyDays"] === null ? null : architectProgramSnapshotGuardInteger(row["contingencyDays"], `${at}.contingencyDays`, {"minimum": 0, "maximum": 4294967295})),
    reportingCadence: (row["reportingCadence"] === null ? null : architectProgramSnapshotGuardString(row["reportingCadence"], `${at}.reportingCadence`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export interface SearchFilter {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly filterName: string;
  readonly filterDescription: TextField | null;
  readonly keywords: readonly string[];
  readonly categories: readonly string[];
  readonly ownerIds: readonly string[];
  readonly statuses: readonly LifecycleStatus[];
  readonly priorities: readonly Priority[];
  readonly sources: readonly string[];
  readonly dateFrom: string | null;
  readonly dateTo: string | null;
  readonly entityKinds: readonly string[];
  readonly tagFilters: readonly string[];
  readonly sortField: string | null;
  readonly sortDirection: string | null;
  readonly isPublic: boolean;
  readonly createdBy: string | null;
  readonly lastUsed: string | null;
  readonly useCount: Unsigned64Transport;
  readonly pinned: boolean;
}

export function parseSearchFilter(value: unknown, at = "$"): SearchFilter {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "filterName", "filterDescription", "keywords", "categories", "ownerIds", "statuses", "priorities", "sources", "dateFrom", "dateTo", "entityKinds", "tagFilters", "sortField", "sortDirection", "isPublic", "createdBy", "lastUsed", "useCount", "pinned"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    filterName: architectProgramSnapshotGuardString(row["filterName"], `${at}.filterName`),
    filterDescription: (row["filterDescription"] === null ? null : parseTextField(row["filterDescription"], `${at}.filterDescription`)),
    keywords: architectProgramSnapshotGuardArray(row["keywords"], `${at}.keywords`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.keywords[${index}]`)),
    categories: architectProgramSnapshotGuardArray(row["categories"], `${at}.categories`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.categories[${index}]`)),
    ownerIds: architectProgramSnapshotGuardArray(row["ownerIds"], `${at}.ownerIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.ownerIds[${index}]`)),
    statuses: architectProgramSnapshotGuardArray(row["statuses"], `${at}.statuses`).map((item, index) => parseLifecycleStatus(item, `${at}.statuses[${index}]`)),
    priorities: architectProgramSnapshotGuardArray(row["priorities"], `${at}.priorities`).map((item, index) => parsePriority(item, `${at}.priorities[${index}]`)),
    sources: architectProgramSnapshotGuardArray(row["sources"], `${at}.sources`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.sources[${index}]`)),
    dateFrom: (row["dateFrom"] === null ? null : architectProgramSnapshotGuardString(row["dateFrom"], `${at}.dateFrom`)),
    dateTo: (row["dateTo"] === null ? null : architectProgramSnapshotGuardString(row["dateTo"], `${at}.dateTo`)),
    entityKinds: architectProgramSnapshotGuardArray(row["entityKinds"], `${at}.entityKinds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.entityKinds[${index}]`)),
    tagFilters: architectProgramSnapshotGuardArray(row["tagFilters"], `${at}.tagFilters`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tagFilters[${index}]`)),
    sortField: (row["sortField"] === null ? null : architectProgramSnapshotGuardString(row["sortField"], `${at}.sortField`)),
    sortDirection: (row["sortDirection"] === null ? null : architectProgramSnapshotGuardString(row["sortDirection"], `${at}.sortDirection`)),
    isPublic: architectProgramSnapshotGuardBoolean(row["isPublic"], `${at}.isPublic`),
    createdBy: (row["createdBy"] === null ? null : architectProgramSnapshotGuardString(row["createdBy"], `${at}.createdBy`)),
    lastUsed: (row["lastUsed"] === null ? null : architectProgramSnapshotGuardString(row["lastUsed"], `${at}.lastUsed`)),
    useCount: parseUnsigned64Transport(row["useCount"], `${at}.useCount`),
    pinned: architectProgramSnapshotGuardBoolean(row["pinned"], `${at}.pinned`),
  };
}

export type SecurityControlKind = "accessControl" | "surveillance" | "perimeter" | "cyber" | "personnel" | "information" | "physical" | "procedural" | "screening" | "keyManagement";

export function parseSecurityControlKind(value: unknown, at = "$"): SecurityControlKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["accessControl", "surveillance", "perimeter", "cyber", "personnel", "information", "physical", "procedural", "screening", "keyManagement"] as const);
}

export interface SecurityRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly controlKind: SecurityControlKind;
  readonly threat: string;
  readonly riskLevel: RiskLevel;
  readonly assetIds: readonly string[];
  readonly zoneIds: readonly string[];
  readonly accessLevel: AccessLevel;
  readonly perimeterControls: readonly string[];
  readonly surveillance: readonly string[];
  readonly intrusionDetection: readonly string[];
  readonly cybersecurity: readonly string[];
  readonly screening: readonly string[];
  readonly visitorManagement: readonly string[];
  readonly keyManagement: readonly string[];
  readonly standards: readonly string[];
  readonly responseProcedures: readonly string[];
  readonly drillFrequency: string | null;
  readonly liaisonContacts: readonly string[];
  readonly classifiedLevel: string | null;
  readonly redundancy: readonly string[];
  readonly auditRequirements: readonly string[];
}

export function parseSecurityRequirement(value: unknown, at = "$"): SecurityRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "controlKind", "threat", "riskLevel", "assetIds", "zoneIds", "accessLevel", "perimeterControls", "surveillance", "intrusionDetection", "cybersecurity", "screening", "visitorManagement", "keyManagement", "standards", "responseProcedures", "drillFrequency", "liaisonContacts", "classifiedLevel", "redundancy", "auditRequirements"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    controlKind: parseSecurityControlKind(row["controlKind"], `${at}.controlKind`),
    threat: architectProgramSnapshotGuardString(row["threat"], `${at}.threat`),
    riskLevel: parseRiskLevel(row["riskLevel"], `${at}.riskLevel`),
    assetIds: architectProgramSnapshotGuardArray(row["assetIds"], `${at}.assetIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.assetIds[${index}]`)),
    zoneIds: architectProgramSnapshotGuardArray(row["zoneIds"], `${at}.zoneIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.zoneIds[${index}]`)),
    accessLevel: parseAccessLevel(row["accessLevel"], `${at}.accessLevel`),
    perimeterControls: architectProgramSnapshotGuardArray(row["perimeterControls"], `${at}.perimeterControls`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.perimeterControls[${index}]`)),
    surveillance: architectProgramSnapshotGuardArray(row["surveillance"], `${at}.surveillance`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.surveillance[${index}]`)),
    intrusionDetection: architectProgramSnapshotGuardArray(row["intrusionDetection"], `${at}.intrusionDetection`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.intrusionDetection[${index}]`)),
    cybersecurity: architectProgramSnapshotGuardArray(row["cybersecurity"], `${at}.cybersecurity`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.cybersecurity[${index}]`)),
    screening: architectProgramSnapshotGuardArray(row["screening"], `${at}.screening`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.screening[${index}]`)),
    visitorManagement: architectProgramSnapshotGuardArray(row["visitorManagement"], `${at}.visitorManagement`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.visitorManagement[${index}]`)),
    keyManagement: architectProgramSnapshotGuardArray(row["keyManagement"], `${at}.keyManagement`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.keyManagement[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    responseProcedures: architectProgramSnapshotGuardArray(row["responseProcedures"], `${at}.responseProcedures`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.responseProcedures[${index}]`)),
    drillFrequency: (row["drillFrequency"] === null ? null : architectProgramSnapshotGuardString(row["drillFrequency"], `${at}.drillFrequency`)),
    liaisonContacts: architectProgramSnapshotGuardArray(row["liaisonContacts"], `${at}.liaisonContacts`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.liaisonContacts[${index}]`)),
    classifiedLevel: (row["classifiedLevel"] === null ? null : architectProgramSnapshotGuardString(row["classifiedLevel"], `${at}.classifiedLevel`)),
    redundancy: architectProgramSnapshotGuardArray(row["redundancy"], `${at}.redundancy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.redundancy[${index}]`)),
    auditRequirements: architectProgramSnapshotGuardArray(row["auditRequirements"], `${at}.auditRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.auditRequirements[${index}]`)),
  };
}

export type SeparationKind = "acoustic" | "visual" | "security" | "olfactory" | "thermal" | "fire" | "hygienic" | "circulation" | "operational" | "infectionControl";

export function parseSeparationKind(value: unknown, at = "$"): SeparationKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["acoustic", "visual", "security", "olfactory", "thermal", "fire", "hygienic", "circulation", "operational", "infectionControl"] as const);
}

export interface ServiceRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly serviceName: string;
  readonly serviceType: string;
  readonly provider: string | null;
  readonly serviceLevel: string | null;
  readonly operatingHours: string | null;
  readonly capacity: QuantitySpec;
  readonly responseTime: string | null;
  readonly queueManagement: readonly string[];
  readonly customerProfiles: readonly string[];
  readonly elementIds: readonly string[];
  readonly equipmentIds: readonly string[];
  readonly staffing: QuantitySpec;
  readonly qualityMetrics: readonly string[];
  readonly costModel: string | null;
  readonly contractRefs: readonly string[];
  readonly dependencies: readonly string[];
  readonly failureImpact: string | null;
  readonly backupService: readonly string[];
  readonly feedbackChannels: readonly string[];
}

export function parseServiceRequirement(value: unknown, at = "$"): ServiceRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "serviceName", "serviceType", "provider", "serviceLevel", "operatingHours", "capacity", "responseTime", "queueManagement", "customerProfiles", "elementIds", "equipmentIds", "staffing", "qualityMetrics", "costModel", "contractRefs", "dependencies", "failureImpact", "backupService", "feedbackChannels"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    serviceName: architectProgramSnapshotGuardString(row["serviceName"], `${at}.serviceName`),
    serviceType: architectProgramSnapshotGuardString(row["serviceType"], `${at}.serviceType`),
    provider: (row["provider"] === null ? null : architectProgramSnapshotGuardString(row["provider"], `${at}.provider`)),
    serviceLevel: (row["serviceLevel"] === null ? null : architectProgramSnapshotGuardString(row["serviceLevel"], `${at}.serviceLevel`)),
    operatingHours: (row["operatingHours"] === null ? null : architectProgramSnapshotGuardString(row["operatingHours"], `${at}.operatingHours`)),
    capacity: parseQuantitySpec(row["capacity"], `${at}.capacity`),
    responseTime: (row["responseTime"] === null ? null : architectProgramSnapshotGuardString(row["responseTime"], `${at}.responseTime`)),
    queueManagement: architectProgramSnapshotGuardArray(row["queueManagement"], `${at}.queueManagement`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.queueManagement[${index}]`)),
    customerProfiles: architectProgramSnapshotGuardArray(row["customerProfiles"], `${at}.customerProfiles`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.customerProfiles[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    equipmentIds: architectProgramSnapshotGuardArray(row["equipmentIds"], `${at}.equipmentIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.equipmentIds[${index}]`)),
    staffing: parseQuantitySpec(row["staffing"], `${at}.staffing`),
    qualityMetrics: architectProgramSnapshotGuardArray(row["qualityMetrics"], `${at}.qualityMetrics`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.qualityMetrics[${index}]`)),
    costModel: (row["costModel"] === null ? null : architectProgramSnapshotGuardString(row["costModel"], `${at}.costModel`)),
    contractRefs: architectProgramSnapshotGuardArray(row["contractRefs"], `${at}.contractRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.contractRefs[${index}]`)),
    dependencies: architectProgramSnapshotGuardArray(row["dependencies"], `${at}.dependencies`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.dependencies[${index}]`)),
    failureImpact: (row["failureImpact"] === null ? null : architectProgramSnapshotGuardString(row["failureImpact"], `${at}.failureImpact`)),
    backupService: architectProgramSnapshotGuardArray(row["backupService"], `${at}.backupService`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.backupService[${index}]`)),
    feedbackChannels: architectProgramSnapshotGuardArray(row["feedbackChannels"], `${at}.feedbackChannels`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.feedbackChannels[${index}]`)),
  };
}

export interface SiteContext {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly siteName: string;
  readonly address: string | null;
  readonly latitude: Binary64Transport | null;
  readonly longitude: Binary64Transport | null;
  readonly elevationM: Binary64Transport | null;
  readonly climateZone: string | null;
  readonly seismicZone: string | null;
  readonly floodRisk: string | null;
  readonly soilConditions: readonly string[];
  readonly utilitiesAvailable: readonly string[];
  readonly accessRoads: readonly string[];
  readonly publicTransit: readonly string[];
  readonly neighbors: readonly string[];
  readonly views: readonly string[];
  readonly noiseSources: readonly string[];
  readonly environmentalConstraints: readonly string[];
  readonly heritageConstraints: readonly string[];
  readonly zoning: string | null;
  readonly maxHeightM: Binary64Transport | null;
  readonly maxCoverage: Binary64Transport | null;
}

export function parseSiteContext(value: unknown, at = "$"): SiteContext {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "siteName", "address", "latitude", "longitude", "elevationM", "climateZone", "seismicZone", "floodRisk", "soilConditions", "utilitiesAvailable", "accessRoads", "publicTransit", "neighbors", "views", "noiseSources", "environmentalConstraints", "heritageConstraints", "zoning", "maxHeightM", "maxCoverage"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    siteName: architectProgramSnapshotGuardString(row["siteName"], `${at}.siteName`),
    address: (row["address"] === null ? null : architectProgramSnapshotGuardString(row["address"], `${at}.address`)),
    latitude: (row["latitude"] === null ? null : parseBinary64Transport(row["latitude"], `${at}.latitude`)),
    longitude: (row["longitude"] === null ? null : parseBinary64Transport(row["longitude"], `${at}.longitude`)),
    elevationM: (row["elevationM"] === null ? null : parseBinary64Transport(row["elevationM"], `${at}.elevationM`)),
    climateZone: (row["climateZone"] === null ? null : architectProgramSnapshotGuardString(row["climateZone"], `${at}.climateZone`)),
    seismicZone: (row["seismicZone"] === null ? null : architectProgramSnapshotGuardString(row["seismicZone"], `${at}.seismicZone`)),
    floodRisk: (row["floodRisk"] === null ? null : architectProgramSnapshotGuardString(row["floodRisk"], `${at}.floodRisk`)),
    soilConditions: architectProgramSnapshotGuardArray(row["soilConditions"], `${at}.soilConditions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.soilConditions[${index}]`)),
    utilitiesAvailable: architectProgramSnapshotGuardArray(row["utilitiesAvailable"], `${at}.utilitiesAvailable`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.utilitiesAvailable[${index}]`)),
    accessRoads: architectProgramSnapshotGuardArray(row["accessRoads"], `${at}.accessRoads`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.accessRoads[${index}]`)),
    publicTransit: architectProgramSnapshotGuardArray(row["publicTransit"], `${at}.publicTransit`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.publicTransit[${index}]`)),
    neighbors: architectProgramSnapshotGuardArray(row["neighbors"], `${at}.neighbors`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.neighbors[${index}]`)),
    views: architectProgramSnapshotGuardArray(row["views"], `${at}.views`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.views[${index}]`)),
    noiseSources: architectProgramSnapshotGuardArray(row["noiseSources"], `${at}.noiseSources`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.noiseSources[${index}]`)),
    environmentalConstraints: architectProgramSnapshotGuardArray(row["environmentalConstraints"], `${at}.environmentalConstraints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.environmentalConstraints[${index}]`)),
    heritageConstraints: architectProgramSnapshotGuardArray(row["heritageConstraints"], `${at}.heritageConstraints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.heritageConstraints[${index}]`)),
    zoning: (row["zoning"] === null ? null : architectProgramSnapshotGuardString(row["zoning"], `${at}.zoning`)),
    maxHeightM: (row["maxHeightM"] === null ? null : parseBinary64Transport(row["maxHeightM"], `${at}.maxHeightM`)),
    maxCoverage: (row["maxCoverage"] === null ? null : parseBinary64Transport(row["maxCoverage"], `${at}.maxCoverage`)),
  };
}

export interface Stakeholder {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly role: string;
  readonly organization: string;
  readonly department: string | null;
  readonly contactEmail: string | null;
  readonly contactPhone: string | null;
  readonly influence: InfluenceLevel;
  readonly interest: InfluenceLevel;
  readonly engagement: EngagementLevel;
  readonly expectations: readonly string[];
  readonly concerns: readonly string[];
  readonly requirementIds: readonly string[];
  readonly decisionAuthority: boolean;
  readonly communicationPreferences: readonly string[];
  readonly reportingFrequency: string | null;
  readonly involvementPhases: readonly string[];
  readonly availability: string | null;
  readonly representativeOf: string | null;
  readonly delegatedTo: string | null;
  readonly relationshipToClient: string | null;
  readonly powerInterestNotes: readonly TaggedNote[];
  readonly stakeholderType: string;
  readonly influenceStrategy: string | null;
  readonly communicationChannels: readonly string[];
  readonly successMetrics: readonly string[];
}

export function parseStakeholder(value: unknown, at = "$"): Stakeholder {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "role", "organization", "department", "contactEmail", "contactPhone", "influence", "interest", "engagement", "expectations", "concerns", "requirementIds", "decisionAuthority", "communicationPreferences", "reportingFrequency", "involvementPhases", "availability", "representativeOf", "delegatedTo", "relationshipToClient", "powerInterestNotes", "stakeholderType", "influenceStrategy", "communicationChannels", "successMetrics"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    role: architectProgramSnapshotGuardString(row["role"], `${at}.role`),
    organization: architectProgramSnapshotGuardString(row["organization"], `${at}.organization`),
    department: (row["department"] === null ? null : architectProgramSnapshotGuardString(row["department"], `${at}.department`)),
    contactEmail: (row["contactEmail"] === null ? null : architectProgramSnapshotGuardString(row["contactEmail"], `${at}.contactEmail`)),
    contactPhone: (row["contactPhone"] === null ? null : architectProgramSnapshotGuardString(row["contactPhone"], `${at}.contactPhone`)),
    influence: parseInfluenceLevel(row["influence"], `${at}.influence`),
    interest: parseInfluenceLevel(row["interest"], `${at}.interest`),
    engagement: parseEngagementLevel(row["engagement"], `${at}.engagement`),
    expectations: architectProgramSnapshotGuardArray(row["expectations"], `${at}.expectations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.expectations[${index}]`)),
    concerns: architectProgramSnapshotGuardArray(row["concerns"], `${at}.concerns`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.concerns[${index}]`)),
    requirementIds: architectProgramSnapshotGuardArray(row["requirementIds"], `${at}.requirementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.requirementIds[${index}]`)),
    decisionAuthority: architectProgramSnapshotGuardBoolean(row["decisionAuthority"], `${at}.decisionAuthority`),
    communicationPreferences: architectProgramSnapshotGuardArray(row["communicationPreferences"], `${at}.communicationPreferences`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.communicationPreferences[${index}]`)),
    reportingFrequency: (row["reportingFrequency"] === null ? null : architectProgramSnapshotGuardString(row["reportingFrequency"], `${at}.reportingFrequency`)),
    involvementPhases: architectProgramSnapshotGuardArray(row["involvementPhases"], `${at}.involvementPhases`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.involvementPhases[${index}]`)),
    availability: (row["availability"] === null ? null : architectProgramSnapshotGuardString(row["availability"], `${at}.availability`)),
    representativeOf: (row["representativeOf"] === null ? null : architectProgramSnapshotGuardString(row["representativeOf"], `${at}.representativeOf`)),
    delegatedTo: (row["delegatedTo"] === null ? null : architectProgramSnapshotGuardString(row["delegatedTo"], `${at}.delegatedTo`)),
    relationshipToClient: (row["relationshipToClient"] === null ? null : architectProgramSnapshotGuardString(row["relationshipToClient"], `${at}.relationshipToClient`)),
    powerInterestNotes: architectProgramSnapshotGuardArray(row["powerInterestNotes"], `${at}.powerInterestNotes`).map((item, index) => parseTaggedNote(item, `${at}.powerInterestNotes[${index}]`)),
    stakeholderType: architectProgramSnapshotGuardString(row["stakeholderType"], `${at}.stakeholderType`),
    influenceStrategy: (row["influenceStrategy"] === null ? null : architectProgramSnapshotGuardString(row["influenceStrategy"], `${at}.influenceStrategy`)),
    communicationChannels: architectProgramSnapshotGuardArray(row["communicationChannels"], `${at}.communicationChannels`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.communicationChannels[${index}]`)),
    successMetrics: architectProgramSnapshotGuardArray(row["successMetrics"], `${at}.successMetrics`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.successMetrics[${index}]`)),
  };
}

export interface StatusRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly subjectId: string;
  readonly subjectKind: string;
  readonly recordStatus: LifecycleStatus;
  readonly previousStatus: LifecycleStatus | null;
  readonly changedBy: string | null;
  readonly changedAt: string | null;
  readonly reason: TextField | null;
  readonly blockers: readonly string[];
  readonly nextActions: readonly string[];
  readonly dueDate: string | null;
  readonly progressPercent: Binary64Transport | null;
  readonly health: string | null;
  readonly escalationLevel: string | null;
  readonly relatedIssueIds: readonly string[];
  readonly relatedRiskIds: readonly string[];
  readonly milestoneId: string | null;
  readonly reportingPeriod: string | null;
  readonly statusNotes: readonly TaggedNote[];
}

export function parseStatusRecord(value: unknown, at = "$"): StatusRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "subjectId", "subjectKind", "recordStatus", "previousStatus", "changedBy", "changedAt", "reason", "blockers", "nextActions", "dueDate", "progressPercent", "health", "escalationLevel", "relatedIssueIds", "relatedRiskIds", "milestoneId", "reportingPeriod", "statusNotes"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    subjectId: architectProgramSnapshotGuardString(row["subjectId"], `${at}.subjectId`),
    subjectKind: architectProgramSnapshotGuardString(row["subjectKind"], `${at}.subjectKind`),
    recordStatus: parseLifecycleStatus(row["recordStatus"], `${at}.recordStatus`),
    previousStatus: (row["previousStatus"] === null ? null : parseLifecycleStatus(row["previousStatus"], `${at}.previousStatus`)),
    changedBy: (row["changedBy"] === null ? null : architectProgramSnapshotGuardString(row["changedBy"], `${at}.changedBy`)),
    changedAt: (row["changedAt"] === null ? null : architectProgramSnapshotGuardString(row["changedAt"], `${at}.changedAt`)),
    reason: (row["reason"] === null ? null : parseTextField(row["reason"], `${at}.reason`)),
    blockers: architectProgramSnapshotGuardArray(row["blockers"], `${at}.blockers`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.blockers[${index}]`)),
    nextActions: architectProgramSnapshotGuardArray(row["nextActions"], `${at}.nextActions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.nextActions[${index}]`)),
    dueDate: (row["dueDate"] === null ? null : architectProgramSnapshotGuardString(row["dueDate"], `${at}.dueDate`)),
    progressPercent: (row["progressPercent"] === null ? null : parseBinary64Transport(row["progressPercent"], `${at}.progressPercent`)),
    health: (row["health"] === null ? null : architectProgramSnapshotGuardString(row["health"], `${at}.health`)),
    escalationLevel: (row["escalationLevel"] === null ? null : architectProgramSnapshotGuardString(row["escalationLevel"], `${at}.escalationLevel`)),
    relatedIssueIds: architectProgramSnapshotGuardArray(row["relatedIssueIds"], `${at}.relatedIssueIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedIssueIds[${index}]`)),
    relatedRiskIds: architectProgramSnapshotGuardArray(row["relatedRiskIds"], `${at}.relatedRiskIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedRiskIds[${index}]`)),
    milestoneId: (row["milestoneId"] === null ? null : architectProgramSnapshotGuardString(row["milestoneId"], `${at}.milestoneId`)),
    reportingPeriod: (row["reportingPeriod"] === null ? null : architectProgramSnapshotGuardString(row["reportingPeriod"], `${at}.reportingPeriod`)),
    statusNotes: architectProgramSnapshotGuardArray(row["statusNotes"], `${at}.statusNotes`).map((item, index) => parseTaggedNote(item, `${at}.statusNotes[${index}]`)),
  };
}

export type StorageClass = "general" | "secure" | "climateControlled" | "hazardous" | "archive" | "mobile" | "fixed" | "shared" | "coldChain" | "flammable";

export function parseStorageClass(value: unknown, at = "$"): StorageClass {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["general", "secure", "climateControlled", "hazardous", "archive", "mobile", "fixed", "shared", "coldChain", "flammable"] as const);
}

export interface StorageRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly storedItem: string;
  readonly storageClass: StorageClass;
  readonly quantity: QuantitySpec;
  readonly volumeM3: Binary64Transport | null;
  readonly weightKg: Binary64Transport | null;
  readonly temperatureRange: string | null;
  readonly humidityRange: string | null;
  readonly securityLevel: AccessLevel;
  readonly hazardClass: string | null;
  readonly retentionPeriod: string | null;
  readonly accessFrequency: string | null;
  readonly elementIds: readonly string[];
  readonly equipmentIds: readonly string[];
  readonly handlingEquipment: readonly string[];
  readonly fireProtection: readonly string[];
  readonly ventilation: string | null;
  readonly organizationSystem: string | null;
  readonly growthAllowance: Binary64Transport | null;
  readonly regulatoryRefs: readonly string[];
  readonly ownerId: string | null;
}

export function parseStorageRequirement(value: unknown, at = "$"): StorageRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "storedItem", "storageClass", "quantity", "volumeM3", "weightKg", "temperatureRange", "humidityRange", "securityLevel", "hazardClass", "retentionPeriod", "accessFrequency", "elementIds", "equipmentIds", "handlingEquipment", "fireProtection", "ventilation", "organizationSystem", "growthAllowance", "regulatoryRefs", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    storedItem: architectProgramSnapshotGuardString(row["storedItem"], `${at}.storedItem`),
    storageClass: parseStorageClass(row["storageClass"], `${at}.storageClass`),
    quantity: parseQuantitySpec(row["quantity"], `${at}.quantity`),
    volumeM3: (row["volumeM3"] === null ? null : parseBinary64Transport(row["volumeM3"], `${at}.volumeM3`)),
    weightKg: (row["weightKg"] === null ? null : parseBinary64Transport(row["weightKg"], `${at}.weightKg`)),
    temperatureRange: (row["temperatureRange"] === null ? null : architectProgramSnapshotGuardString(row["temperatureRange"], `${at}.temperatureRange`)),
    humidityRange: (row["humidityRange"] === null ? null : architectProgramSnapshotGuardString(row["humidityRange"], `${at}.humidityRange`)),
    securityLevel: parseAccessLevel(row["securityLevel"], `${at}.securityLevel`),
    hazardClass: (row["hazardClass"] === null ? null : architectProgramSnapshotGuardString(row["hazardClass"], `${at}.hazardClass`)),
    retentionPeriod: (row["retentionPeriod"] === null ? null : architectProgramSnapshotGuardString(row["retentionPeriod"], `${at}.retentionPeriod`)),
    accessFrequency: (row["accessFrequency"] === null ? null : architectProgramSnapshotGuardString(row["accessFrequency"], `${at}.accessFrequency`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    equipmentIds: architectProgramSnapshotGuardArray(row["equipmentIds"], `${at}.equipmentIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.equipmentIds[${index}]`)),
    handlingEquipment: architectProgramSnapshotGuardArray(row["handlingEquipment"], `${at}.handlingEquipment`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.handlingEquipment[${index}]`)),
    fireProtection: architectProgramSnapshotGuardArray(row["fireProtection"], `${at}.fireProtection`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.fireProtection[${index}]`)),
    ventilation: (row["ventilation"] === null ? null : architectProgramSnapshotGuardString(row["ventilation"], `${at}.ventilation`)),
    organizationSystem: (row["organizationSystem"] === null ? null : architectProgramSnapshotGuardString(row["organizationSystem"], `${at}.organizationSystem`)),
    growthAllowance: (row["growthAllowance"] === null ? null : parseBinary64Transport(row["growthAllowance"], `${at}.growthAllowance`)),
    regulatoryRefs: architectProgramSnapshotGuardArray(row["regulatoryRefs"], `${at}.regulatoryRefs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.regulatoryRefs[${index}]`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export interface Survey {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly surveyType: string;
  readonly title: string;
  readonly objectives: readonly string[];
  readonly questions: readonly string[];
  readonly targetAudience: readonly string[];
  readonly distributionChannels: readonly string[];
  readonly launchDate: string | null;
  readonly closeDate: string | null;
  readonly responseCount: number;
  readonly responseRate: Binary64Transport | null;
  readonly findings: readonly string[];
  readonly themes: readonly string[];
  readonly recommendations: readonly string[];
  readonly confidentiality: string | null;
  readonly consentProcess: readonly string[];
  readonly analysisId: string | null;
  readonly workshopId: string | null;
  readonly ownerId: string | null;
  readonly surveyStatus: LifecycleStatus;
}

export function parseSurvey(value: unknown, at = "$"): Survey {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "surveyType", "title", "objectives", "questions", "targetAudience", "distributionChannels", "launchDate", "closeDate", "responseCount", "responseRate", "findings", "themes", "recommendations", "confidentiality", "consentProcess", "analysisId", "workshopId", "ownerId", "surveyStatus"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    surveyType: architectProgramSnapshotGuardString(row["surveyType"], `${at}.surveyType`),
    title: architectProgramSnapshotGuardString(row["title"], `${at}.title`),
    objectives: architectProgramSnapshotGuardArray(row["objectives"], `${at}.objectives`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.objectives[${index}]`)),
    questions: architectProgramSnapshotGuardArray(row["questions"], `${at}.questions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.questions[${index}]`)),
    targetAudience: architectProgramSnapshotGuardArray(row["targetAudience"], `${at}.targetAudience`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.targetAudience[${index}]`)),
    distributionChannels: architectProgramSnapshotGuardArray(row["distributionChannels"], `${at}.distributionChannels`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.distributionChannels[${index}]`)),
    launchDate: (row["launchDate"] === null ? null : architectProgramSnapshotGuardString(row["launchDate"], `${at}.launchDate`)),
    closeDate: (row["closeDate"] === null ? null : architectProgramSnapshotGuardString(row["closeDate"], `${at}.closeDate`)),
    responseCount: architectProgramSnapshotGuardInteger(row["responseCount"], `${at}.responseCount`, {"minimum": 0, "maximum": 4294967295}),
    responseRate: (row["responseRate"] === null ? null : parseBinary64Transport(row["responseRate"], `${at}.responseRate`)),
    findings: architectProgramSnapshotGuardArray(row["findings"], `${at}.findings`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.findings[${index}]`)),
    themes: architectProgramSnapshotGuardArray(row["themes"], `${at}.themes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.themes[${index}]`)),
    recommendations: architectProgramSnapshotGuardArray(row["recommendations"], `${at}.recommendations`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.recommendations[${index}]`)),
    confidentiality: (row["confidentiality"] === null ? null : architectProgramSnapshotGuardString(row["confidentiality"], `${at}.confidentiality`)),
    consentProcess: architectProgramSnapshotGuardArray(row["consentProcess"], `${at}.consentProcess`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.consentProcess[${index}]`)),
    analysisId: (row["analysisId"] === null ? null : architectProgramSnapshotGuardString(row["analysisId"], `${at}.analysisId`)),
    workshopId: (row["workshopId"] === null ? null : architectProgramSnapshotGuardString(row["workshopId"], `${at}.workshopId`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
    surveyStatus: parseLifecycleStatus(row["surveyStatus"], `${at}.surveyStatus`),
  };
}

export interface SustainabilityRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly topic: string;
  readonly target: string | null;
  readonly metric: string | null;
  readonly baseline: Binary64Transport | null;
  readonly targetValue: Binary64Transport | null;
  readonly unit: string | null;
  readonly certification: readonly string[];
  readonly standards: readonly string[];
  readonly elementIds: readonly string[];
  readonly strategies: readonly string[];
  readonly materialsPreferences: readonly string[];
  readonly energyStrategy: readonly string[];
  readonly waterStrategy: readonly string[];
  readonly wasteStrategy: readonly string[];
  readonly biodiversity: readonly string[];
  readonly embodiedCarbon: Binary64Transport | null;
  readonly operationalCarbon: Binary64Transport | null;
  readonly reportingRequirements: readonly string[];
  readonly verificationPlan: string | null;
  readonly ownerId: string | null;
}

export function parseSustainabilityRequirement(value: unknown, at = "$"): SustainabilityRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "topic", "target", "metric", "baseline", "targetValue", "unit", "certification", "standards", "elementIds", "strategies", "materialsPreferences", "energyStrategy", "waterStrategy", "wasteStrategy", "biodiversity", "embodiedCarbon", "operationalCarbon", "reportingRequirements", "verificationPlan", "ownerId"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    topic: architectProgramSnapshotGuardString(row["topic"], `${at}.topic`),
    target: (row["target"] === null ? null : architectProgramSnapshotGuardString(row["target"], `${at}.target`)),
    metric: (row["metric"] === null ? null : architectProgramSnapshotGuardString(row["metric"], `${at}.metric`)),
    baseline: (row["baseline"] === null ? null : parseBinary64Transport(row["baseline"], `${at}.baseline`)),
    targetValue: (row["targetValue"] === null ? null : parseBinary64Transport(row["targetValue"], `${at}.targetValue`)),
    unit: (row["unit"] === null ? null : architectProgramSnapshotGuardString(row["unit"], `${at}.unit`)),
    certification: architectProgramSnapshotGuardArray(row["certification"], `${at}.certification`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.certification[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    strategies: architectProgramSnapshotGuardArray(row["strategies"], `${at}.strategies`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.strategies[${index}]`)),
    materialsPreferences: architectProgramSnapshotGuardArray(row["materialsPreferences"], `${at}.materialsPreferences`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.materialsPreferences[${index}]`)),
    energyStrategy: architectProgramSnapshotGuardArray(row["energyStrategy"], `${at}.energyStrategy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.energyStrategy[${index}]`)),
    waterStrategy: architectProgramSnapshotGuardArray(row["waterStrategy"], `${at}.waterStrategy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.waterStrategy[${index}]`)),
    wasteStrategy: architectProgramSnapshotGuardArray(row["wasteStrategy"], `${at}.wasteStrategy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.wasteStrategy[${index}]`)),
    biodiversity: architectProgramSnapshotGuardArray(row["biodiversity"], `${at}.biodiversity`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.biodiversity[${index}]`)),
    embodiedCarbon: (row["embodiedCarbon"] === null ? null : parseBinary64Transport(row["embodiedCarbon"], `${at}.embodiedCarbon`)),
    operationalCarbon: (row["operationalCarbon"] === null ? null : parseBinary64Transport(row["operationalCarbon"], `${at}.operationalCarbon`)),
    reportingRequirements: architectProgramSnapshotGuardArray(row["reportingRequirements"], `${at}.reportingRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.reportingRequirements[${index}]`)),
    verificationPlan: (row["verificationPlan"] === null ? null : architectProgramSnapshotGuardString(row["verificationPlan"], `${at}.verificationPlan`)),
    ownerId: (row["ownerId"] === null ? null : architectProgramSnapshotGuardString(row["ownerId"], `${at}.ownerId`)),
  };
}

export interface TaggedNote {
  readonly tag: string;
  readonly text: string;
}

export function parseTaggedNote(value: unknown, at = "$"): TaggedNote {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["tag", "text"]);
  return {
    tag: architectProgramSnapshotGuardString(row["tag"], `${at}.tag`),
    text: architectProgramSnapshotGuardString(row["text"], `${at}.text`),
  };
}

export interface TemplateRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly templateType: string;
  readonly sector: string | null;
  readonly projectType: string | null;
  readonly version: string;
  readonly contentRef: string | null;
  readonly entityKinds: readonly string[];
  readonly defaultFields: readonly string[];
  readonly checklists: readonly string[];
  readonly standards: readonly string[];
  readonly applicability: readonly string[];
  readonly authorId: string | null;
  readonly approvalStatus: ValidationStatus;
  readonly usageCount: Unsigned64Transport;
  readonly lastApplied: string | null;
  readonly customizationNotes: readonly string[];
  readonly relatedKnowledgeIds: readonly string[];
  readonly benchmarkIds: readonly string[];
  readonly license: string | null;
  readonly sourceOrganization: string | null;
}

export function parseTemplateRecord(value: unknown, at = "$"): TemplateRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "templateType", "sector", "projectType", "version", "contentRef", "entityKinds", "defaultFields", "checklists", "standards", "applicability", "authorId", "approvalStatus", "usageCount", "lastApplied", "customizationNotes", "relatedKnowledgeIds", "benchmarkIds", "license", "sourceOrganization"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    templateType: architectProgramSnapshotGuardString(row["templateType"], `${at}.templateType`),
    sector: (row["sector"] === null ? null : architectProgramSnapshotGuardString(row["sector"], `${at}.sector`)),
    projectType: (row["projectType"] === null ? null : architectProgramSnapshotGuardString(row["projectType"], `${at}.projectType`)),
    version: architectProgramSnapshotGuardString(row["version"], `${at}.version`),
    contentRef: (row["contentRef"] === null ? null : architectProgramSnapshotGuardString(row["contentRef"], `${at}.contentRef`)),
    entityKinds: architectProgramSnapshotGuardArray(row["entityKinds"], `${at}.entityKinds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.entityKinds[${index}]`)),
    defaultFields: architectProgramSnapshotGuardArray(row["defaultFields"], `${at}.defaultFields`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.defaultFields[${index}]`)),
    checklists: architectProgramSnapshotGuardArray(row["checklists"], `${at}.checklists`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.checklists[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    applicability: architectProgramSnapshotGuardArray(row["applicability"], `${at}.applicability`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.applicability[${index}]`)),
    authorId: (row["authorId"] === null ? null : architectProgramSnapshotGuardString(row["authorId"], `${at}.authorId`)),
    approvalStatus: parseValidationStatus(row["approvalStatus"], `${at}.approvalStatus`),
    usageCount: parseUnsigned64Transport(row["usageCount"], `${at}.usageCount`),
    lastApplied: (row["lastApplied"] === null ? null : architectProgramSnapshotGuardString(row["lastApplied"], `${at}.lastApplied`)),
    customizationNotes: architectProgramSnapshotGuardArray(row["customizationNotes"], `${at}.customizationNotes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.customizationNotes[${index}]`)),
    relatedKnowledgeIds: architectProgramSnapshotGuardArray(row["relatedKnowledgeIds"], `${at}.relatedKnowledgeIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.relatedKnowledgeIds[${index}]`)),
    benchmarkIds: architectProgramSnapshotGuardArray(row["benchmarkIds"], `${at}.benchmarkIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.benchmarkIds[${index}]`)),
    license: (row["license"] === null ? null : architectProgramSnapshotGuardString(row["license"], `${at}.license`)),
    sourceOrganization: (row["sourceOrganization"] === null ? null : architectProgramSnapshotGuardString(row["sourceOrganization"], `${at}.sourceOrganization`)),
  };
}

export interface TextField {
  readonly text: string;
  readonly format?: string;
}

export function parseTextField(value: unknown, at = "$"): TextField {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["text", "format"]);
  return {
    text: architectProgramSnapshotGuardString(row["text"], `${at}.text`),
    format: row["format"] === undefined ? undefined : architectProgramSnapshotGuardString(row["format"], `${at}.format`),
  };
}

export interface TimestampMeta {
  readonly created: string;
  readonly updated: string;
  readonly createdBy?: string;
  readonly updatedBy?: string;
}

export function parseTimestampMeta(value: unknown, at = "$"): TimestampMeta {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["created", "updated", "createdBy", "updatedBy"]);
  return {
    created: architectProgramSnapshotGuardString(row["created"], `${at}.created`),
    updated: architectProgramSnapshotGuardString(row["updated"], `${at}.updated`),
    createdBy: row["createdBy"] === undefined ? undefined : architectProgramSnapshotGuardString(row["createdBy"], `${at}.createdBy`),
    updatedBy: row["updatedBy"] === undefined ? undefined : architectProgramSnapshotGuardString(row["updatedBy"], `${at}.updatedBy`),
  };
}

export type TraceKind = "objectiveToRequirement" | "stakeholderToRequirement" | "userToActivity" | "activityToFunction" | "functionToProgramElement" | "requirementToDecision" | "requirementToRisk" | "requirementToStandard" | "requirementToValidation" | "requirementToApproval" | "requirementToChange" | "equipmentToActivity" | "processToResource" | "constraintToImpact" | "scenarioToDecision" | "issueToAction" | "actionToOwner" | "decisionToOutcome" | "versionToChange" | "fullAuditTrail";

export function parseTraceKind(value: unknown, at = "$"): TraceKind {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["objectiveToRequirement", "stakeholderToRequirement", "userToActivity", "activityToFunction", "functionToProgramElement", "requirementToDecision", "requirementToRisk", "requirementToStandard", "requirementToValidation", "requirementToApproval", "requirementToChange", "equipmentToActivity", "processToResource", "constraintToImpact", "scenarioToDecision", "issueToAction", "actionToOwner", "decisionToOutcome", "versionToChange", "fullAuditTrail"] as const);
}

export interface TraceLink {
  readonly id: string;
  readonly fromId: string;
  readonly toId: string;
  readonly kind: TraceKind;
  readonly label?: string;
}

export function parseTraceLink(value: unknown, at = "$"): TraceLink {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "fromId", "toId", "kind", "label"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    fromId: architectProgramSnapshotGuardString(row["fromId"], `${at}.fromId`),
    toId: architectProgramSnapshotGuardString(row["toId"], `${at}.toId`),
    kind: parseTraceKind(row["kind"], `${at}.kind`),
    label: row["label"] === undefined ? undefined : architectProgramSnapshotGuardString(row["label"], `${at}.label`),
  };
}

export type Unsigned64Transport = number | string;

export function parseUnsigned64Transport(value: unknown, at = "$"): Unsigned64Transport {
  return ((input0: unknown) => { if (typeof input0 === "number") { try { return architectProgramSnapshotGuardInteger(input0, `${at}`, {"minimum": 0, "maximum": 9007199254740991}); } catch (error) { if (!(error instanceof architectProgramSnapshotGuardRefusal)) throw error; } } if (typeof input0 === "string") { try { return architectProgramSnapshotGuardString(input0, `${at}`, {"pattern": "^(0|[1-9][0-9]{0,19})$"}); } catch (error) { if (!(error instanceof architectProgramSnapshotGuardRefusal)) throw error; } } return architectProgramSnapshotGuardReject(`${at}`, "value matches no variant"); })(value);
}

export type UserCategory = "primary" | "secondary" | "occasional" | "service" | "visitor" | "staff" | "public";

export function parseUserCategory(value: unknown, at = "$"): UserCategory {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["primary", "secondary", "occasional", "service", "visitor", "staff", "public"] as const);
}

export interface UserProfile {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly category: UserCategory;
  readonly demographic: string | null;
  readonly ageRange: string | null;
  readonly abilities: readonly string[];
  readonly disabilities: readonly string[];
  readonly occupation: string | null;
  readonly roleTitle: string | null;
  readonly department: string | null;
  readonly mobilityProfile: readonly string[];
  readonly sensoryProfile: readonly string[];
  readonly cognitiveProfile: readonly string[];
  readonly behavioralPatterns: readonly string[];
  readonly usageFrequency: string | null;
  readonly usageDuration: string | null;
  readonly peakUsageTimes: readonly string[];
  readonly technologyProficiency: string | null;
  readonly preferences: readonly string[];
  readonly painPoints: readonly string[];
  readonly goals: readonly string[];
  readonly activityIds: readonly string[];
  readonly researchMethod: string | null;
  readonly personaArchetype: string | null;
  readonly validated: boolean;
  readonly stakeholderIds: readonly string[];
}

export function parseUserProfile(value: unknown, at = "$"): UserProfile {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "category", "demographic", "ageRange", "abilities", "disabilities", "occupation", "roleTitle", "department", "mobilityProfile", "sensoryProfile", "cognitiveProfile", "behavioralPatterns", "usageFrequency", "usageDuration", "peakUsageTimes", "technologyProficiency", "preferences", "painPoints", "goals", "activityIds", "researchMethod", "personaArchetype", "validated", "stakeholderIds"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    category: parseUserCategory(row["category"], `${at}.category`),
    demographic: (row["demographic"] === null ? null : architectProgramSnapshotGuardString(row["demographic"], `${at}.demographic`)),
    ageRange: (row["ageRange"] === null ? null : architectProgramSnapshotGuardString(row["ageRange"], `${at}.ageRange`)),
    abilities: architectProgramSnapshotGuardArray(row["abilities"], `${at}.abilities`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.abilities[${index}]`)),
    disabilities: architectProgramSnapshotGuardArray(row["disabilities"], `${at}.disabilities`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.disabilities[${index}]`)),
    occupation: (row["occupation"] === null ? null : architectProgramSnapshotGuardString(row["occupation"], `${at}.occupation`)),
    roleTitle: (row["roleTitle"] === null ? null : architectProgramSnapshotGuardString(row["roleTitle"], `${at}.roleTitle`)),
    department: (row["department"] === null ? null : architectProgramSnapshotGuardString(row["department"], `${at}.department`)),
    mobilityProfile: architectProgramSnapshotGuardArray(row["mobilityProfile"], `${at}.mobilityProfile`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.mobilityProfile[${index}]`)),
    sensoryProfile: architectProgramSnapshotGuardArray(row["sensoryProfile"], `${at}.sensoryProfile`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.sensoryProfile[${index}]`)),
    cognitiveProfile: architectProgramSnapshotGuardArray(row["cognitiveProfile"], `${at}.cognitiveProfile`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.cognitiveProfile[${index}]`)),
    behavioralPatterns: architectProgramSnapshotGuardArray(row["behavioralPatterns"], `${at}.behavioralPatterns`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.behavioralPatterns[${index}]`)),
    usageFrequency: (row["usageFrequency"] === null ? null : architectProgramSnapshotGuardString(row["usageFrequency"], `${at}.usageFrequency`)),
    usageDuration: (row["usageDuration"] === null ? null : architectProgramSnapshotGuardString(row["usageDuration"], `${at}.usageDuration`)),
    peakUsageTimes: architectProgramSnapshotGuardArray(row["peakUsageTimes"], `${at}.peakUsageTimes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.peakUsageTimes[${index}]`)),
    technologyProficiency: (row["technologyProficiency"] === null ? null : architectProgramSnapshotGuardString(row["technologyProficiency"], `${at}.technologyProficiency`)),
    preferences: architectProgramSnapshotGuardArray(row["preferences"], `${at}.preferences`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.preferences[${index}]`)),
    painPoints: architectProgramSnapshotGuardArray(row["painPoints"], `${at}.painPoints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.painPoints[${index}]`)),
    goals: architectProgramSnapshotGuardArray(row["goals"], `${at}.goals`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.goals[${index}]`)),
    activityIds: architectProgramSnapshotGuardArray(row["activityIds"], `${at}.activityIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.activityIds[${index}]`)),
    researchMethod: (row["researchMethod"] === null ? null : architectProgramSnapshotGuardString(row["researchMethod"], `${at}.researchMethod`)),
    personaArchetype: (row["personaArchetype"] === null ? null : architectProgramSnapshotGuardString(row["personaArchetype"], `${at}.personaArchetype`)),
    validated: architectProgramSnapshotGuardBoolean(row["validated"], `${at}.validated`),
    stakeholderIds: architectProgramSnapshotGuardArray(row["stakeholderIds"], `${at}.stakeholderIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.stakeholderIds[${index}]`)),
  };
}

export interface ValidationRecord {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly subjectId: string;
  readonly subjectKind: string;
  readonly validationType: string;
  readonly method: string | null;
  readonly criteria: readonly string[];
  readonly result: ValidationStatus;
  readonly evidence: readonly string[];
  readonly validatorIds: readonly string[];
  readonly validationDate: string | null;
  readonly nextReviewDate: string | null;
  readonly findings: readonly string[];
  readonly nonConformities: readonly string[];
  readonly correctiveActions: readonly string[];
  readonly waivers: readonly string[];
  readonly standards: readonly string[];
  readonly traceLinks: readonly TraceLink[];
  readonly reportId: string | null;
  readonly confidenceLevel: string | null;
  readonly validationNotes: readonly TaggedNote[];
}

export function parseValidationRecord(value: unknown, at = "$"): ValidationRecord {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "subjectId", "subjectKind", "validationType", "method", "criteria", "result", "evidence", "validatorIds", "validationDate", "nextReviewDate", "findings", "nonConformities", "correctiveActions", "waivers", "standards", "traceLinks", "reportId", "confidenceLevel", "validationNotes"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    subjectId: architectProgramSnapshotGuardString(row["subjectId"], `${at}.subjectId`),
    subjectKind: architectProgramSnapshotGuardString(row["subjectKind"], `${at}.subjectKind`),
    validationType: architectProgramSnapshotGuardString(row["validationType"], `${at}.validationType`),
    method: (row["method"] === null ? null : architectProgramSnapshotGuardString(row["method"], `${at}.method`)),
    criteria: architectProgramSnapshotGuardArray(row["criteria"], `${at}.criteria`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.criteria[${index}]`)),
    result: parseValidationStatus(row["result"], `${at}.result`),
    evidence: architectProgramSnapshotGuardArray(row["evidence"], `${at}.evidence`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.evidence[${index}]`)),
    validatorIds: architectProgramSnapshotGuardArray(row["validatorIds"], `${at}.validatorIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.validatorIds[${index}]`)),
    validationDate: (row["validationDate"] === null ? null : architectProgramSnapshotGuardString(row["validationDate"], `${at}.validationDate`)),
    nextReviewDate: (row["nextReviewDate"] === null ? null : architectProgramSnapshotGuardString(row["nextReviewDate"], `${at}.nextReviewDate`)),
    findings: architectProgramSnapshotGuardArray(row["findings"], `${at}.findings`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.findings[${index}]`)),
    nonConformities: architectProgramSnapshotGuardArray(row["nonConformities"], `${at}.nonConformities`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.nonConformities[${index}]`)),
    correctiveActions: architectProgramSnapshotGuardArray(row["correctiveActions"], `${at}.correctiveActions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.correctiveActions[${index}]`)),
    waivers: architectProgramSnapshotGuardArray(row["waivers"], `${at}.waivers`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.waivers[${index}]`)),
    standards: architectProgramSnapshotGuardArray(row["standards"], `${at}.standards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.standards[${index}]`)),
    traceLinks: architectProgramSnapshotGuardArray(row["traceLinks"], `${at}.traceLinks`).map((item, index) => parseTraceLink(item, `${at}.traceLinks[${index}]`)),
    reportId: (row["reportId"] === null ? null : architectProgramSnapshotGuardString(row["reportId"], `${at}.reportId`)),
    confidenceLevel: (row["confidenceLevel"] === null ? null : architectProgramSnapshotGuardString(row["confidenceLevel"], `${at}.confidenceLevel`)),
    validationNotes: architectProgramSnapshotGuardArray(row["validationNotes"], `${at}.validationNotes`).map((item, index) => parseTaggedNote(item, `${at}.validationNotes[${index}]`)),
  };
}

export type ValidationStatus = "pending" | "passed" | "failed" | "waived" | "deferred";

export function parseValidationStatus(value: unknown, at = "$"): ValidationStatus {
  return architectProgramSnapshotGuardMember(value, `${at}`, ["pending", "passed", "failed", "waived", "deferred"] as const);
}

export interface WayfindingRequirement {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly userProfileIds: readonly string[];
  readonly elementIds: readonly string[];
  readonly destinationTypes: readonly string[];
  readonly signageTypes: readonly string[];
  readonly languages: readonly string[];
  readonly tactileRequired: boolean;
  readonly audioRequired: boolean;
  readonly digitalWayfinding: boolean;
  readonly landmarkStrategy: readonly string[];
  readonly colorCoding: readonly string[];
  readonly symbolStandards: readonly string[];
  readonly decisionPoints: readonly string[];
  readonly maximumSignageDistanceM: Binary64Transport | null;
  readonly lightingRequirements: readonly string[];
  readonly maintenancePlan: string | null;
  readonly emergencyEgress: readonly string[];
  readonly visitorJourney: readonly string[];
  readonly staffJourney: readonly string[];
  readonly brandIntegration: readonly string[];
}

export function parseWayfindingRequirement(value: unknown, at = "$"): WayfindingRequirement {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "userProfileIds", "elementIds", "destinationTypes", "signageTypes", "languages", "tactileRequired", "audioRequired", "digitalWayfinding", "landmarkStrategy", "colorCoding", "symbolStandards", "decisionPoints", "maximumSignageDistanceM", "lightingRequirements", "maintenancePlan", "emergencyEgress", "visitorJourney", "staffJourney", "brandIntegration"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    userProfileIds: architectProgramSnapshotGuardArray(row["userProfileIds"], `${at}.userProfileIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.userProfileIds[${index}]`)),
    elementIds: architectProgramSnapshotGuardArray(row["elementIds"], `${at}.elementIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.elementIds[${index}]`)),
    destinationTypes: architectProgramSnapshotGuardArray(row["destinationTypes"], `${at}.destinationTypes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.destinationTypes[${index}]`)),
    signageTypes: architectProgramSnapshotGuardArray(row["signageTypes"], `${at}.signageTypes`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.signageTypes[${index}]`)),
    languages: architectProgramSnapshotGuardArray(row["languages"], `${at}.languages`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.languages[${index}]`)),
    tactileRequired: architectProgramSnapshotGuardBoolean(row["tactileRequired"], `${at}.tactileRequired`),
    audioRequired: architectProgramSnapshotGuardBoolean(row["audioRequired"], `${at}.audioRequired`),
    digitalWayfinding: architectProgramSnapshotGuardBoolean(row["digitalWayfinding"], `${at}.digitalWayfinding`),
    landmarkStrategy: architectProgramSnapshotGuardArray(row["landmarkStrategy"], `${at}.landmarkStrategy`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.landmarkStrategy[${index}]`)),
    colorCoding: architectProgramSnapshotGuardArray(row["colorCoding"], `${at}.colorCoding`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.colorCoding[${index}]`)),
    symbolStandards: architectProgramSnapshotGuardArray(row["symbolStandards"], `${at}.symbolStandards`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.symbolStandards[${index}]`)),
    decisionPoints: architectProgramSnapshotGuardArray(row["decisionPoints"], `${at}.decisionPoints`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decisionPoints[${index}]`)),
    maximumSignageDistanceM: (row["maximumSignageDistanceM"] === null ? null : parseBinary64Transport(row["maximumSignageDistanceM"], `${at}.maximumSignageDistanceM`)),
    lightingRequirements: architectProgramSnapshotGuardArray(row["lightingRequirements"], `${at}.lightingRequirements`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.lightingRequirements[${index}]`)),
    maintenancePlan: (row["maintenancePlan"] === null ? null : architectProgramSnapshotGuardString(row["maintenancePlan"], `${at}.maintenancePlan`)),
    emergencyEgress: architectProgramSnapshotGuardArray(row["emergencyEgress"], `${at}.emergencyEgress`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.emergencyEgress[${index}]`)),
    visitorJourney: architectProgramSnapshotGuardArray(row["visitorJourney"], `${at}.visitorJourney`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.visitorJourney[${index}]`)),
    staffJourney: architectProgramSnapshotGuardArray(row["staffJourney"], `${at}.staffJourney`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.staffJourney[${index}]`)),
    brandIntegration: architectProgramSnapshotGuardArray(row["brandIntegration"], `${at}.brandIntegration`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.brandIntegration[${index}]`)),
  };
}

export interface Workshop {
  readonly id: string;
  readonly name: string;
  readonly description?: TextField;
  readonly status: LifecycleStatus;
  readonly priority: Priority;
  readonly ownership: Ownership;
  readonly tags?: readonly string[];
  readonly notes?: readonly TaggedNote[];
  readonly timestamps: TimestampMeta;
  readonly workshopType: string;
  readonly objectives: readonly string[];
  readonly agenda: readonly string[];
  readonly facilitatorId: string | null;
  readonly participants: readonly string[];
  readonly scheduledStart: string | null;
  readonly scheduledEnd: string | null;
  readonly location: string | null;
  readonly materials: readonly string[];
  readonly methods: readonly string[];
  readonly outputs: readonly string[];
  readonly decisions: readonly string[];
  readonly issues: readonly string[];
  readonly followUpActions: readonly string[];
  readonly feedback: readonly TaggedNote[];
  readonly recordingRef: string | null;
  readonly budget: Binary64Transport | null;
  readonly workshopStatus: LifecycleStatus;
  readonly surveyIds: readonly string[];
}

export function parseWorkshop(value: unknown, at = "$"): Workshop {
  const row = architectProgramSnapshotGuardClosed(architectProgramSnapshotGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps", "workshopType", "objectives", "agenda", "facilitatorId", "participants", "scheduledStart", "scheduledEnd", "location", "materials", "methods", "outputs", "decisions", "issues", "followUpActions", "feedback", "recordingRef", "budget", "workshopStatus", "surveyIds"]);
  return {
    id: architectProgramSnapshotGuardString(row["id"], `${at}.id`),
    name: architectProgramSnapshotGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? undefined : architectProgramSnapshotGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
    workshopType: architectProgramSnapshotGuardString(row["workshopType"], `${at}.workshopType`),
    objectives: architectProgramSnapshotGuardArray(row["objectives"], `${at}.objectives`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.objectives[${index}]`)),
    agenda: architectProgramSnapshotGuardArray(row["agenda"], `${at}.agenda`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.agenda[${index}]`)),
    facilitatorId: (row["facilitatorId"] === null ? null : architectProgramSnapshotGuardString(row["facilitatorId"], `${at}.facilitatorId`)),
    participants: architectProgramSnapshotGuardArray(row["participants"], `${at}.participants`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.participants[${index}]`)),
    scheduledStart: (row["scheduledStart"] === null ? null : architectProgramSnapshotGuardString(row["scheduledStart"], `${at}.scheduledStart`)),
    scheduledEnd: (row["scheduledEnd"] === null ? null : architectProgramSnapshotGuardString(row["scheduledEnd"], `${at}.scheduledEnd`)),
    location: (row["location"] === null ? null : architectProgramSnapshotGuardString(row["location"], `${at}.location`)),
    materials: architectProgramSnapshotGuardArray(row["materials"], `${at}.materials`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.materials[${index}]`)),
    methods: architectProgramSnapshotGuardArray(row["methods"], `${at}.methods`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.methods[${index}]`)),
    outputs: architectProgramSnapshotGuardArray(row["outputs"], `${at}.outputs`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.outputs[${index}]`)),
    decisions: architectProgramSnapshotGuardArray(row["decisions"], `${at}.decisions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.decisions[${index}]`)),
    issues: architectProgramSnapshotGuardArray(row["issues"], `${at}.issues`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.issues[${index}]`)),
    followUpActions: architectProgramSnapshotGuardArray(row["followUpActions"], `${at}.followUpActions`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.followUpActions[${index}]`)),
    feedback: architectProgramSnapshotGuardArray(row["feedback"], `${at}.feedback`).map((item, index) => parseTaggedNote(item, `${at}.feedback[${index}]`)),
    recordingRef: (row["recordingRef"] === null ? null : architectProgramSnapshotGuardString(row["recordingRef"], `${at}.recordingRef`)),
    budget: (row["budget"] === null ? null : parseBinary64Transport(row["budget"], `${at}.budget`)),
    workshopStatus: parseLifecycleStatus(row["workshopStatus"], `${at}.workshopStatus`),
    surveyIds: architectProgramSnapshotGuardArray(row["surveyIds"], `${at}.surveyIds`).map((item, index) => architectProgramSnapshotGuardString(item, `${at}.surveyIds[${index}]`)),
  };
}
