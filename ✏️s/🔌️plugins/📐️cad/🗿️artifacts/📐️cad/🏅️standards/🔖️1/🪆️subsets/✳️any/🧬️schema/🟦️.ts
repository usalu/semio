/** 🧬️ CadArtifact schema. */

export interface CadArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  objects: CadObject[];
  /** @state artifact */
  buildingObjects: CadObject[];
  /** @state artifact */
  energyObjects: CadObject[];
  /** @state artifact */
  structureClassicObjects: CadObject[];
  /** @state artifact */
  referencesByModelDefinitionId: Record<string, CadReferenceList>;
  /** @state artifact */
  nodes: CadNode[];
  /** @state artifact */
  shapeGeometry?: CadGeometry;
  /** @state artifact */
  buildingGeometry?: CadGeometry;
  /** @state artifact */
  energyGeometry?: CadGeometry;
  /** @state artifact */
  structureClassicGeometry?: CadGeometry;
  /** @state artifact */
  activeModelDefinitionId: string;
  /** @state presence */
  selectedObjectIds: string[];
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  activeObjectId?: string;
  /** @state presence */
  componentSelection: CadComponentSelection;
  /** @state presence */
  selectedReferenceModelDefinitionId?: string;
  /** @state presence */
  selectedReferenceId?: string;
  /** @state presence */
  selectedPrimitiveId?: string;
  /** @state presence */
  selectedPrimitiveKind?: string;
  /** @state presence */
  /** @state presence */
  activeExampleId?: string;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  engagementStep: string;
  /** @state config */
  engagementPane?: string;
  /** @state config */
  engagementSessionJson?: string;
  /** @state config */
  lastFinalizedInteractionId?: string;
  /** @state config */
  sunEnabled: boolean;
  /** @state config */
  sunAzimuth: number;
  /** @state config */
  sunElevation: number;
  /** @state config */
  sunIntensity: number;
  /** @state config */
  sunColor: string;
  /** @state config */
  camera: CadCamera;
  /** @state config */
  cameraBuilding: CadCamera;
  /** @state config */
  cameraEnergy: CadCamera;
  /** @state config */
  cameraStructureClassic: CadCamera;
  /** @state config */
  dislocateShape: CadDislocateOptions;
  /** @state config */
  dislocateBuilding: CadDislocateOptions;
  /** @state config */
  dislocateEnergy: CadDislocateOptions;
  /** @state config */
  dislocateStructureClassic: CadDislocateOptions;
  /** @state config */
  /** @state config */
  /** @state config */
  contributionsJson: string;
  /** @state artifact */
  hoveredObjectId?: string;
  /** @state artifact */
  hoveredTargetObjectId?: string;
  /** @state artifact */
  hoveredTargetMode?: string;
  /** @state artifact */
  hoveredTargetId?: number;
}

export interface CadObject { id: string; [key: string]: unknown }
export interface CadNode { id: string; [key: string]: unknown }
export interface CadReferenceList { values: unknown[] }
export interface CadGeometry { [key: string]: unknown }
export interface CadCamera { [key: string]: unknown }
export interface CadComponentSelection { [key: string]: unknown }
export interface CadDislocateOptions { moveEnabled: boolean; rotateEnabled: boolean }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class cadCadArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const cadCadArtifactGuardReject = (at: string, why: string): never => {
  throw new cadCadArtifactGuardRefusal(at, why);
};

type cadCadArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type cadCadArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type cadCadArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const cadCadArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : cadCadArtifactGuardReject(at, "value is not an object");
export const cadCadArtifactGuardArray = (value: unknown, at: string, bounds: cadCadArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return cadCadArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) cadCadArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) cadCadArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const cadCadArtifactGuardString = (value: unknown, at: string, bounds: cadCadArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return cadCadArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) cadCadArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) cadCadArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) cadCadArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const cadCadArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : cadCadArtifactGuardReject(at, "value is not a boolean"));
export const cadCadArtifactGuardNumber = (value: unknown, at: string, bounds: cadCadArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return cadCadArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) cadCadArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) cadCadArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const cadCadArtifactGuardInteger = (value: unknown, at: string, bounds: cadCadArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? cadCadArtifactGuardNumber(value, at, bounds) : cadCadArtifactGuardReject(at, "value is not an integer");
export const cadCadArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : cadCadArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const cadCadArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : cadCadArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCadArtifact(value: unknown, at = "$"): CadArtifact {
  const row = cadCadArtifactGuardObject(value, at);
  return {
    schema: cadCadArtifactGuardString(row["schema"], `${at}.schema`),
    id: cadCadArtifactGuardString(row["id"], `${at}.id`),
    objects: cadCadArtifactGuardArray(row["objects"], `${at}.objects`).map((item, index) => parseCadObject(item, `${at}.objects[${index}]`)),
    buildingObjects: cadCadArtifactGuardArray(row["buildingObjects"], `${at}.buildingObjects`).map((item, index) => parseCadObject(item, `${at}.buildingObjects[${index}]`)),
    energyObjects: cadCadArtifactGuardArray(row["energyObjects"], `${at}.energyObjects`).map((item, index) => parseCadObject(item, `${at}.energyObjects[${index}]`)),
    structureClassicObjects: cadCadArtifactGuardArray(row["structureClassicObjects"], `${at}.structureClassicObjects`).map((item, index) => parseCadObject(item, `${at}.structureClassicObjects[${index}]`)),
    referencesByModelDefinitionId: cadCadArtifactGuardObject(row["referencesByModelDefinitionId"], `${at}.referencesByModelDefinitionId`),
    nodes: cadCadArtifactGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseCadNode(item, `${at}.nodes[${index}]`)),
    shapeGeometry: row["shapeGeometry"] === undefined ? undefined : parseCadGeometry(row["shapeGeometry"], `${at}.shapeGeometry`),
    buildingGeometry: row["buildingGeometry"] === undefined ? undefined : parseCadGeometry(row["buildingGeometry"], `${at}.buildingGeometry`),
    energyGeometry: row["energyGeometry"] === undefined ? undefined : parseCadGeometry(row["energyGeometry"], `${at}.energyGeometry`),
    structureClassicGeometry: row["structureClassicGeometry"] === undefined ? undefined : parseCadGeometry(row["structureClassicGeometry"], `${at}.structureClassicGeometry`),
    activeModelDefinitionId: cadCadArtifactGuardString(row["activeModelDefinitionId"], `${at}.activeModelDefinitionId`),
    selectedObjectIds: cadCadArtifactGuardArray(row["selectedObjectIds"], `${at}.selectedObjectIds`).map((item, index) => cadCadArtifactGuardString(item, `${at}.selectedObjectIds[${index}]`)),
    selectedNodeIds: cadCadArtifactGuardArray(row["selectedNodeIds"], `${at}.selectedNodeIds`).map((item, index) => cadCadArtifactGuardString(item, `${at}.selectedNodeIds[${index}]`)),
    activeObjectId: row["activeObjectId"] === undefined ? undefined : cadCadArtifactGuardString(row["activeObjectId"], `${at}.activeObjectId`),
    componentSelection: parseCadComponentSelection(row["componentSelection"], `${at}.componentSelection`),
    selectedReferenceModelDefinitionId: row["selectedReferenceModelDefinitionId"] === undefined ? undefined : cadCadArtifactGuardString(row["selectedReferenceModelDefinitionId"], `${at}.selectedReferenceModelDefinitionId`),
    selectedReferenceId: row["selectedReferenceId"] === undefined ? undefined : cadCadArtifactGuardString(row["selectedReferenceId"], `${at}.selectedReferenceId`),
    selectedPrimitiveId: row["selectedPrimitiveId"] === undefined ? undefined : cadCadArtifactGuardString(row["selectedPrimitiveId"], `${at}.selectedPrimitiveId`),
    selectedPrimitiveKind: row["selectedPrimitiveKind"] === undefined ? undefined : cadCadArtifactGuardString(row["selectedPrimitiveKind"], `${at}.selectedPrimitiveKind`),
    activeExampleId: row["activeExampleId"] === undefined ? undefined : cadCadArtifactGuardString(row["activeExampleId"], `${at}.activeExampleId`),
    selectionMethod: cadCadArtifactGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    engagementInput: cadCadArtifactGuardString(row["engagementInput"], `${at}.engagementInput`),
    engagementStep: cadCadArtifactGuardString(row["engagementStep"], `${at}.engagementStep`),
    engagementPane: row["engagementPane"] === undefined ? undefined : cadCadArtifactGuardString(row["engagementPane"], `${at}.engagementPane`),
    engagementSessionJson: row["engagementSessionJson"] === undefined ? undefined : cadCadArtifactGuardString(row["engagementSessionJson"], `${at}.engagementSessionJson`),
    lastFinalizedInteractionId: row["lastFinalizedInteractionId"] === undefined ? undefined : cadCadArtifactGuardString(row["lastFinalizedInteractionId"], `${at}.lastFinalizedInteractionId`),
    sunEnabled: cadCadArtifactGuardBoolean(row["sunEnabled"], `${at}.sunEnabled`),
    sunAzimuth: cadCadArtifactGuardNumber(row["sunAzimuth"], `${at}.sunAzimuth`),
    sunElevation: cadCadArtifactGuardNumber(row["sunElevation"], `${at}.sunElevation`),
    sunIntensity: cadCadArtifactGuardNumber(row["sunIntensity"], `${at}.sunIntensity`),
    sunColor: cadCadArtifactGuardString(row["sunColor"], `${at}.sunColor`),
    camera: parseCadCamera(row["camera"], `${at}.camera`),
    cameraBuilding: parseCadCamera(row["cameraBuilding"], `${at}.cameraBuilding`),
    cameraEnergy: parseCadCamera(row["cameraEnergy"], `${at}.cameraEnergy`),
    cameraStructureClassic: parseCadCamera(row["cameraStructureClassic"], `${at}.cameraStructureClassic`),
    dislocateShape: parseCadDislocateOptions(row["dislocateShape"], `${at}.dislocateShape`),
    dislocateBuilding: parseCadDislocateOptions(row["dislocateBuilding"], `${at}.dislocateBuilding`),
    dislocateEnergy: parseCadDislocateOptions(row["dislocateEnergy"], `${at}.dislocateEnergy`),
    dislocateStructureClassic: parseCadDislocateOptions(row["dislocateStructureClassic"], `${at}.dislocateStructureClassic`),
    contributionsJson: cadCadArtifactGuardString(row["contributionsJson"], `${at}.contributionsJson`),
    hoveredObjectId: row["hoveredObjectId"] === undefined ? undefined : cadCadArtifactGuardString(row["hoveredObjectId"], `${at}.hoveredObjectId`),
    hoveredTargetObjectId: row["hoveredTargetObjectId"] === undefined ? undefined : cadCadArtifactGuardString(row["hoveredTargetObjectId"], `${at}.hoveredTargetObjectId`),
    hoveredTargetMode: row["hoveredTargetMode"] === undefined ? undefined : cadCadArtifactGuardString(row["hoveredTargetMode"], `${at}.hoveredTargetMode`),
    hoveredTargetId: row["hoveredTargetId"] === undefined ? undefined : cadCadArtifactGuardInteger(row["hoveredTargetId"], `${at}.hoveredTargetId`, {"minimum": 0}),
  };
}

export function parseCadObject(value: unknown, at = "$"): CadObject {
  const row = cadCadArtifactGuardObject(value, at);
  return {
    id: cadCadArtifactGuardString(row["id"], `${at}.id`),
  };
}

export function parseCadNode(value: unknown, at = "$"): CadNode {
  const row = cadCadArtifactGuardObject(value, at);
  return {
    id: cadCadArtifactGuardString(row["id"], `${at}.id`),
  };
}

export function parseCadReferenceList(value: unknown, at = "$"): CadReferenceList {
  const row = cadCadArtifactGuardObject(value, at);
  return {
    values: cadCadArtifactGuardArray(row["values"], `${at}.values`).map((item, index) => cadCadArtifactGuardObject(item, `${at}.values[${index}]`)),
  };
}

export function parseCadGeometry(value: unknown, at = "$"): CadGeometry {
  return cadCadArtifactGuardObject(value, `${at}`);
}

export function parseCadCamera(value: unknown, at = "$"): CadCamera {
  return cadCadArtifactGuardObject(value, `${at}`);
}

export function parseCadComponentSelection(value: unknown, at = "$"): CadComponentSelection {
  return cadCadArtifactGuardObject(value, `${at}`);
}

export function parseCadDislocateOptions(value: unknown, at = "$"): CadDislocateOptions {
  const row = cadCadArtifactGuardObject(value, at);
  return {
    moveEnabled: cadCadArtifactGuardBoolean(row["moveEnabled"], `${at}.moveEnabled`),
    rotateEnabled: cadCadArtifactGuardBoolean(row["rotateEnabled"], `${at}.rotateEnabled`),
  };
}

export interface CadStringList {
  readonly values: readonly string[];
}

export function parseCadStringList(value: unknown, at = "$"): CadStringList {
  const row = cadCadArtifactGuardObject(value, at);
  return {
    values: cadCadArtifactGuardArray(row["values"], `${at}.values`).map((item, index) => cadCadArtifactGuardString(item, `${at}.values[${index}]`)),
  };
}

export interface CadObjectsDelta {
  readonly added: readonly CadObject[];
  readonly removed: readonly string[];
  readonly patched: readonly Readonly<Record<string, unknown>>[];
  readonly reordered?: readonly string[];
}

export function parseCadObjectsDelta(value: unknown, at = "$"): CadObjectsDelta {
  const row = cadCadArtifactGuardObject(value, at);
  return {
    added: cadCadArtifactGuardArray(row["added"], `${at}.added`).map((item, index) => parseCadObject(item, `${at}.added[${index}]`)),
    removed: cadCadArtifactGuardArray(row["removed"], `${at}.removed`).map((item, index) => cadCadArtifactGuardString(item, `${at}.removed[${index}]`)),
    patched: cadCadArtifactGuardArray(row["patched"], `${at}.patched`).map((item, index) => cadCadArtifactGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : cadCadArtifactGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => cadCadArtifactGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export interface CadNodesDelta {
  readonly added: readonly CadNode[];
  readonly removed: readonly string[];
  readonly patched: readonly Readonly<Record<string, unknown>>[];
  readonly reordered?: readonly string[];
}

export function parseCadNodesDelta(value: unknown, at = "$"): CadNodesDelta {
  const row = cadCadArtifactGuardObject(value, at);
  return {
    added: cadCadArtifactGuardArray(row["added"], `${at}.added`).map((item, index) => parseCadNode(item, `${at}.added[${index}]`)),
    removed: cadCadArtifactGuardArray(row["removed"], `${at}.removed`).map((item, index) => cadCadArtifactGuardString(item, `${at}.removed[${index}]`)),
    patched: cadCadArtifactGuardArray(row["patched"], `${at}.patched`).map((item, index) => cadCadArtifactGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : cadCadArtifactGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => cadCadArtifactGuardString(item, `${at}.reordered[${index}]`)),
  };
}
