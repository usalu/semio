/** 🧬️ Process3d diff schema — sparse field delta over the artifact. */

export interface Process3dDiff {
  /** @state artifact */
  artifact?: Process3dArtifact;
  /** @state artifact */
  workshop?: Process3dWorkshop;
  /** @state artifact */
  stockId?: string;
  stockLabel?: string;
  stockPose?: Record<string, unknown>;
  stockPayload?: Process3dStock;
  stockSolid?: ArtifactChildHandle;
  /** @state artifact */
  steps?: ArtifactChildHandle;
  stepPayloads?: Process3dStep[];
  toolSolids?: ArtifactChildHandle[];
  /** @state artifact */
  resolvedUpTo?: number | null;
  /** @state presence */
  selectedId?: string | null;
  /** @state presence */
  selectedFaceId?: number | null;
  /** @state presence */
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  cameraPositionX?: number;
  /** @state config */
  cameraPositionY?: number;
  /** @state config */
  cameraPositionZ?: number;
  /** @state config */
  cameraTargetX?: number;
  /** @state config */
  cameraTargetY?: number;
  /** @state config */
  cameraTargetZ?: number;
  /** @state config */
  cameraFov?: number;
  /** @state config */
  sunEnabled?: boolean;
  /** @state config */
  sunAzimuth?: number;
  /** @state config */
  sunElevation?: number;
  /** @state config */
  sunIntensity?: number;
  /** @state config */
  sunColor?: string;
  /** @state config */
  /** @state config */
  contributionsJson?: string;
  /** @state artifact */
  hoveredId?: string | null;
}

export interface Process3dArtifact {
  workshop: Process3dWorkshop;
  stockId: string;
  stockLabel: string;
  stockPose: Record<string, unknown>;
  stockPayload: Process3dStock;
  stockSolid: ArtifactChildHandle;
  steps: ArtifactChildHandle;
  stepPayloads: Process3dStep[];
  toolSolids: ArtifactChildHandle[];
  resolvedUpTo?: number;
}
export interface Process3dWorkshop { machines: unknown[]; }
export interface Process3dStock { id: string; label: string; solid: Record<string, unknown>; pose: Record<string, unknown>; }
export interface Process3dStep { id: string; label: string; enabled: boolean; }
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
export interface Process3dStepsDelta {
  added: Process3dStep[];
  removed: string[];
  patched: Process3dStepPatchEntry[];
  reordered?: string[];
}
export interface Process3dStepPatchEntry { id: string; patch: Record<string, unknown>; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class processProcess3dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const processProcess3dDiffGuardReject = (at: string, why: string): never => {
  throw new processProcess3dDiffGuardRefusal(at, why);
};

type processProcess3dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type processProcess3dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type processProcess3dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const processProcess3dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : processProcess3dDiffGuardReject(at, "value is not an object");
export const processProcess3dDiffGuardArray = (value: unknown, at: string, bounds: processProcess3dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return processProcess3dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) processProcess3dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) processProcess3dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const processProcess3dDiffGuardString = (value: unknown, at: string, bounds: processProcess3dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return processProcess3dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) processProcess3dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) processProcess3dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) processProcess3dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const processProcess3dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : processProcess3dDiffGuardReject(at, "value is not a boolean"));
export const processProcess3dDiffGuardNumber = (value: unknown, at: string, bounds: processProcess3dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return processProcess3dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) processProcess3dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) processProcess3dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const processProcess3dDiffGuardInteger = (value: unknown, at: string, bounds: processProcess3dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? processProcess3dDiffGuardNumber(value, at, bounds) : processProcess3dDiffGuardReject(at, "value is not an integer");
export const processProcess3dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : processProcess3dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const processProcess3dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : processProcess3dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcess3dStepsDelta(value: unknown, at = "$"): Process3dStepsDelta {
  const row = processProcess3dDiffGuardObject(value, at);
  return {
    added: processProcess3dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => processProcess3dDiffGuardObject(item, `${at}.added[${index}]`)),
    removed: processProcess3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => processProcess3dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: processProcess3dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseProcess3dStepPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : processProcess3dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => processProcess3dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseProcess3dStepPatchEntry(value: unknown, at = "$"): Process3dStepPatchEntry {
  const row = processProcess3dDiffGuardObject(value, at);
  return {
    id: processProcess3dDiffGuardString(row["id"], `${at}.id`),
    patch: processProcess3dDiffGuardObject(row["patch"], `${at}.patch`),
  };
}
