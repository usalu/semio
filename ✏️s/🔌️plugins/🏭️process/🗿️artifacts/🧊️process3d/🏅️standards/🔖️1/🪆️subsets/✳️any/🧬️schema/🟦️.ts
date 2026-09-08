/** 🧬️ Process3d artifact schema — every field with its state class. */

export interface Process3dArtifact {
  /** @state artifact */
  workshop: Process3dWorkshop;
  /** @state artifact */
  stockId: string;
  stockLabel: string;
  stockPose: Process3dPose;
  stockPayload: Process3dStock;
  stockSolid: ArtifactChildHandle;
  /** @state artifact */
  steps: ArtifactChildHandle;
  stepPayloads: Process3dStep[];
  toolSolids: ArtifactChildHandle[];
  /** @state artifact */
  resolvedUpTo?: number;
  /** @state presence */
  selectedId?: string;
  /** @state presence */
  selectedFaceId?: number;
  /** @state presence */
  /** @state config */
  selectionMethod: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  cameraPositionX: number;
  /** @state config */
  cameraPositionY: number;
  /** @state config */
  cameraPositionZ: number;
  /** @state config */
  cameraTargetX: number;
  /** @state config */
  cameraTargetY: number;
  /** @state config */
  cameraTargetZ: number;
  /** @state config */
  cameraFov: number;
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
  /** @state config */
  contributionsJson: string;
  /** @state artifact */
  hoveredId?: string;
}

export interface Process3dWorkshop { machines: Process3dWorkshopMachine[]; }
export interface Process3dWorkshopMachine { id: string; label: string; iconId: string; catalogId?: string; capabilities: Process3dCapability[]; }
export interface Process3dCapability { id: string; label: string; iconId: string; recipe: Record<string, unknown>; parameters: Process3dCapabilityParameter[]; rules: Record<string, unknown>[]; }
export interface Process3dCapabilityParameter { id: string; label: string; value: number; }
export interface Process3dStock { id: string; label: string; solid: Record<string, unknown>; pose: Process3dPose; }
export interface Process3dPose { position: [number, number, number]; axis: [number, number, number]; angle: number; }
export interface Process3dStep { id: string; label: string; enabled: boolean; origin?: Process3dStepOrigin; measure: Record<string, unknown>; }
export interface Process3dStepOrigin { machineId: string; capabilityId: string; }
export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}
/** 🌉️ Mirrors `store::ArtifactChild<S>` — `childId`/`target` only; `local_owner` and
 *  `PhantomData<S>` are `#[serde(skip)]`. */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class processProcess3dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const processProcess3dArtifactGuardReject = (at: string, why: string): never => {
  throw new processProcess3dArtifactGuardRefusal(at, why);
};

type processProcess3dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type processProcess3dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type processProcess3dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const processProcess3dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : processProcess3dArtifactGuardReject(at, "value is not an object");
export const processProcess3dArtifactGuardArray = (value: unknown, at: string, bounds: processProcess3dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return processProcess3dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) processProcess3dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) processProcess3dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const processProcess3dArtifactGuardString = (value: unknown, at: string, bounds: processProcess3dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return processProcess3dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) processProcess3dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) processProcess3dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) processProcess3dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const processProcess3dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : processProcess3dArtifactGuardReject(at, "value is not a boolean"));
export const processProcess3dArtifactGuardNumber = (value: unknown, at: string, bounds: processProcess3dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return processProcess3dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) processProcess3dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) processProcess3dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const processProcess3dArtifactGuardInteger = (value: unknown, at: string, bounds: processProcess3dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? processProcess3dArtifactGuardNumber(value, at, bounds) : processProcess3dArtifactGuardReject(at, "value is not an integer");
export const processProcess3dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : processProcess3dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const processProcess3dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : processProcess3dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcess3dArtifact(value: unknown, at = "$"): Process3dArtifact {
  const row = processProcess3dArtifactGuardObject(value, at);
  return {
    workshop: parseProcess3dWorkshop(row["workshop"], `${at}.workshop`),
    stockId: processProcess3dArtifactGuardString(row["stockId"], `${at}.stockId`),
    stockLabel: processProcess3dArtifactGuardString(row["stockLabel"], `${at}.stockLabel`),
    stockPose: parseProcess3dPose(row["stockPose"], `${at}.stockPose`),
    stockPayload: parseProcess3dStock(row["stockPayload"], `${at}.stockPayload`),
    stockSolid: parseArtifactChildHandle(row["stockSolid"], `${at}.stockSolid`),
    steps: parseArtifactChildHandle(row["steps"], `${at}.steps`),
    stepPayloads: processProcess3dArtifactGuardArray(row["stepPayloads"], `${at}.stepPayloads`).map((item, index) => parseProcess3dStep(item, `${at}.stepPayloads[${index}]`)),
    toolSolids: processProcess3dArtifactGuardArray(row["toolSolids"], `${at}.toolSolids`).map((item, index) => parseArtifactChildHandle(item, `${at}.toolSolids[${index}]`)),
    resolvedUpTo: row["resolvedUpTo"] === undefined ? undefined : processProcess3dArtifactGuardInteger(row["resolvedUpTo"], `${at}.resolvedUpTo`),
    selectedId: row["selectedId"] === undefined ? undefined : processProcess3dArtifactGuardString(row["selectedId"], `${at}.selectedId`),
    selectedFaceId: row["selectedFaceId"] === undefined ? undefined : processProcess3dArtifactGuardInteger(row["selectedFaceId"], `${at}.selectedFaceId`),
    selectionMethod: processProcess3dArtifactGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    engagementInput: processProcess3dArtifactGuardString(row["engagementInput"], `${at}.engagementInput`),
    cameraPositionX: processProcess3dArtifactGuardNumber(row["cameraPositionX"], `${at}.cameraPositionX`),
    cameraPositionY: processProcess3dArtifactGuardNumber(row["cameraPositionY"], `${at}.cameraPositionY`),
    cameraPositionZ: processProcess3dArtifactGuardNumber(row["cameraPositionZ"], `${at}.cameraPositionZ`),
    cameraTargetX: processProcess3dArtifactGuardNumber(row["cameraTargetX"], `${at}.cameraTargetX`),
    cameraTargetY: processProcess3dArtifactGuardNumber(row["cameraTargetY"], `${at}.cameraTargetY`),
    cameraTargetZ: processProcess3dArtifactGuardNumber(row["cameraTargetZ"], `${at}.cameraTargetZ`),
    cameraFov: processProcess3dArtifactGuardNumber(row["cameraFov"], `${at}.cameraFov`),
    sunEnabled: processProcess3dArtifactGuardBoolean(row["sunEnabled"], `${at}.sunEnabled`),
    sunAzimuth: processProcess3dArtifactGuardNumber(row["sunAzimuth"], `${at}.sunAzimuth`),
    sunElevation: processProcess3dArtifactGuardNumber(row["sunElevation"], `${at}.sunElevation`),
    sunIntensity: processProcess3dArtifactGuardNumber(row["sunIntensity"], `${at}.sunIntensity`),
    sunColor: processProcess3dArtifactGuardString(row["sunColor"], `${at}.sunColor`),
    contributionsJson: processProcess3dArtifactGuardString(row["contributionsJson"], `${at}.contributionsJson`),
    hoveredId: row["hoveredId"] === undefined ? undefined : processProcess3dArtifactGuardString(row["hoveredId"], `${at}.hoveredId`),
  };
}

export function parseProcess3dWorkshop(value: unknown, at = "$"): Process3dWorkshop {
  return processProcess3dArtifactGuardObject(value, `${at}`);
}

export function parseProcess3dStock(value: unknown, at = "$"): Process3dStock {
  return processProcess3dArtifactGuardObject(value, `${at}`);
}

export function parseProcess3dPose(value: unknown, at = "$"): Process3dPose {
  return processProcess3dArtifactGuardObject(value, `${at}`);
}

export function parseArtifactChildHandle(value: unknown, at = "$"): ArtifactChildHandle {
  const row = processProcess3dArtifactGuardObject(value, at);
  return {
    childId: processProcess3dArtifactGuardString(row["childId"], `${at}.childId`),
    target: processProcess3dArtifactGuardString(row["target"], `${at}.target`),
  };
}

export function parseProcess3dStep(value: unknown, at = "$"): Process3dStep {
  return processProcess3dArtifactGuardObject(value, `${at}`);
}
