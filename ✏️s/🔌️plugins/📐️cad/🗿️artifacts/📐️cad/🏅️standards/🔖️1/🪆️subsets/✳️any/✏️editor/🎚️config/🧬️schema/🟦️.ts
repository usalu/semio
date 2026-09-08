/** 🧬️ CadConfig */
export interface CadHoverTarget {
  objectId?: string;
  mode?: string;
  id?: number;
}
export interface CadSelectionTargets {
  mesh: boolean;
  vertex: boolean;
  edge: boolean;
  face: boolean;
}
export interface CadComponentSelection {
  targets: CadSelectionTargets;
  mode: string;
  ids: number[];
}
export interface CadSunConfig {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}
export interface CadProjectionDsl {
  kind: string;
  [key: string]: unknown;
}
export interface CadCamera {
  position: number[];
  target: number[];
  zoom: number;
  fov: number;
  projection: CadProjectionDsl;
}
export interface CadDislocateOptions {
  moveEnabled: boolean;
  rotateEnabled: boolean;
}


export interface CadConfig {
  /** @state config */
  selectedObjectIds: string[];
  /** @state config */
  selectedNodeIds: string[];
  /** @state config */
  selectionMethod: string;
  /** @state config */
  hoveredObjectId?: string;
  /** @state config */
  hoveredTarget?: CadHoverTarget;
  /** @state config */
  activeObjectId?: string;
  /** @state config */
  componentSelection: CadComponentSelection;
  /** @state config */
  engagementInput: string;
  /** @state config */
  engagementStep: string;
  /** @state config */
  activeExampleId?: string;
  /** @state config */
  selectedReferenceModelDefinitionId?: string;
  /** @state config */
  selectedReferenceId?: string;
  /** @state config */
  selectedPrimitiveId?: string;
  /** @state config */
  selectedPrimitiveKind?: string;
  /** @state config */
  engagementPane?: string;
  /** @state config */
  engagementSessionJson?: string;
  /** @state config */
  engagementPreviewOperationJson?: string;
  /** @state config @minimum 0 @maximum 2147483647 Exact signed-32 generation. */
  engagementPreviewGeneration: number;
  /** @state config */
  lastFinalizedInteractionId?: string;
  /** @state config */
  sun: CadSunConfig;
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
  contributionsJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class cadCadConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const cadCadConfigGuardReject = (at: string, why: string): never => {
  throw new cadCadConfigGuardRefusal(at, why);
};

type cadCadConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type cadCadConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type cadCadConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const cadCadConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : cadCadConfigGuardReject(at, "value is not an object");
export const cadCadConfigGuardArray = (value: unknown, at: string, bounds: cadCadConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return cadCadConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) cadCadConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) cadCadConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const cadCadConfigGuardString = (value: unknown, at: string, bounds: cadCadConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return cadCadConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) cadCadConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) cadCadConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) cadCadConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const cadCadConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : cadCadConfigGuardReject(at, "value is not a boolean"));
export const cadCadConfigGuardNumber = (value: unknown, at: string, bounds: cadCadConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return cadCadConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) cadCadConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) cadCadConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const cadCadConfigGuardInteger = (value: unknown, at: string, bounds: cadCadConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? cadCadConfigGuardNumber(value, at, bounds) : cadCadConfigGuardReject(at, "value is not an integer");
export const cadCadConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : cadCadConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const cadCadConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : cadCadConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCadHoverTarget(value: unknown, at = "$"): CadHoverTarget {
  const row = cadCadConfigGuardObject(value, at);
  return {
    objectId: row["objectId"] === undefined ? undefined : cadCadConfigGuardString(row["objectId"], `${at}.objectId`),
    mode: row["mode"] === undefined ? undefined : cadCadConfigGuardString(row["mode"], `${at}.mode`),
    id: row["id"] === undefined ? undefined : cadCadConfigGuardInteger(row["id"], `${at}.id`, {"minimum": 0}),
  };
}

export function parseCadSelectionTargets(value: unknown, at = "$"): CadSelectionTargets {
  const row = cadCadConfigGuardObject(value, at);
  return {
    mesh: cadCadConfigGuardBoolean(row["mesh"], `${at}.mesh`),
    vertex: cadCadConfigGuardBoolean(row["vertex"], `${at}.vertex`),
    edge: cadCadConfigGuardBoolean(row["edge"], `${at}.edge`),
    face: cadCadConfigGuardBoolean(row["face"], `${at}.face`),
  };
}

export function parseCadComponentSelection(value: unknown, at = "$"): CadComponentSelection {
  const row = cadCadConfigGuardObject(value, at);
  return {
    targets: parseCadSelectionTargets(row["targets"], `${at}.targets`),
    mode: cadCadConfigGuardString(row["mode"], `${at}.mode`),
    ids: cadCadConfigGuardArray(row["ids"], `${at}.ids`).map((item, index) => cadCadConfigGuardInteger(item, `${at}.ids[${index}]`, {"minimum": 0})),
  };
}

export function parseCadSunConfig(value: unknown, at = "$"): CadSunConfig {
  const row = cadCadConfigGuardObject(value, at);
  return {
    enabled: cadCadConfigGuardBoolean(row["enabled"], `${at}.enabled`),
    azimuth: cadCadConfigGuardNumber(row["azimuth"], `${at}.azimuth`),
    elevation: cadCadConfigGuardNumber(row["elevation"], `${at}.elevation`),
    intensity: cadCadConfigGuardNumber(row["intensity"], `${at}.intensity`),
    color: cadCadConfigGuardString(row["color"], `${at}.color`),
  };
}

export function parseCadProjectionDsl(value: unknown, at = "$"): CadProjectionDsl {
  const row = cadCadConfigGuardObject(value, at);
  return {
    kind: cadCadConfigGuardString(row["kind"], `${at}.kind`),
    orthographicView: row["orthographicView"] === undefined ? undefined : cadCadConfigGuardString(row["orthographicView"], `${at}.orthographicView`),
    axonometricVariant: row["axonometricVariant"] === undefined ? undefined : cadCadConfigGuardString(row["axonometricVariant"], `${at}.axonometricVariant`),
    axonometricAngleA: row["axonometricAngleA"] === undefined ? undefined : cadCadConfigGuardNumber(row["axonometricAngleA"], `${at}.axonometricAngleA`),
    axonometricAngleB: row["axonometricAngleB"] === undefined ? undefined : cadCadConfigGuardNumber(row["axonometricAngleB"], `${at}.axonometricAngleB`),
    axonometricQuadrant: row["axonometricQuadrant"] === undefined ? undefined : cadCadConfigGuardString(row["axonometricQuadrant"], `${at}.axonometricQuadrant`),
    obliqueVariant: row["obliqueVariant"] === undefined ? undefined : cadCadConfigGuardString(row["obliqueVariant"], `${at}.obliqueVariant`),
    obliqueAngle: row["obliqueAngle"] === undefined ? undefined : cadCadConfigGuardNumber(row["obliqueAngle"], `${at}.obliqueAngle`),
    obliqueDepth: row["obliqueDepth"] === undefined ? undefined : cadCadConfigGuardNumber(row["obliqueDepth"], `${at}.obliqueDepth`),
    onePointAxis: row["onePointAxis"] === undefined ? undefined : cadCadConfigGuardString(row["onePointAxis"], `${at}.onePointAxis`),
    fov: row["fov"] === undefined ? undefined : cadCadConfigGuardNumber(row["fov"], `${at}.fov`),
    twoPointShift: row["twoPointShift"] === undefined ? undefined : cadCadConfigGuardNumber(row["twoPointShift"], `${at}.twoPointShift`),
    curvilinearFov: row["curvilinearFov"] === undefined ? undefined : cadCadConfigGuardNumber(row["curvilinearFov"], `${at}.curvilinearFov`),
    curvilinearStrength: row["curvilinearStrength"] === undefined ? undefined : cadCadConfigGuardNumber(row["curvilinearStrength"], `${at}.curvilinearStrength`),
    curvilinearMapping: row["curvilinearMapping"] === undefined ? undefined : cadCadConfigGuardString(row["curvilinearMapping"], `${at}.curvilinearMapping`),
  };
}

export function parseCadCamera(value: unknown, at = "$"): CadCamera {
  const row = cadCadConfigGuardObject(value, at);
  return {
    position: cadCadConfigGuardArray(row["position"], `${at}.position`, {"minItems": 3, "maxItems": 3}).map((item, index) => cadCadConfigGuardNumber(item, `${at}.position[${index}]`)),
    target: cadCadConfigGuardArray(row["target"], `${at}.target`, {"minItems": 3, "maxItems": 3}).map((item, index) => cadCadConfigGuardNumber(item, `${at}.target[${index}]`)),
    zoom: cadCadConfigGuardNumber(row["zoom"], `${at}.zoom`),
    fov: cadCadConfigGuardNumber(row["fov"], `${at}.fov`),
    projection: parseCadProjectionDsl(row["projection"], `${at}.projection`),
  };
}
