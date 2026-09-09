/** 🧬️ Fem3d artifact schema — every field with its state class. */

//#region 🔖️Entities
/** 📍️ A structural node: a stable id and a global position, plain SI meters. Mirrors Rust `FemNode`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemNode {
  id: string;
  x: number;
  y: number;
  z: number;
}

/** 🔒️ A DOF tag mirroring the FEM 3D degrees of freedom. Mirrors Rust `FemDof`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`, re-exported from `fem2d::FemDof`). */
export type FemDof = "Tx" | "Ty" | "Tz" | "Rx" | "Ry" | "Rz";

/** 🔩️ A two-node member: an axial `Bar` or a full 6-DOF `Frame` with a local-axis `roll` angle
 * (radians). Mirrors Rust `FemElement` (`🗿️artifacts/🧊️3d/🦀️.rs`), tagged on `kind`. */
export type FemElement =
  | { kind: "bar"; id: string; start: string; end: string; materialId: string; sectionId: string }
  | { kind: "frame"; id: string; start: string; end: string; materialId: string; sectionId: string; roll: number };

/** 🧱️ Linear-elastic isotropic material: Young's modulus `e`, shear modulus `g` (Pa), Poisson's ratio
 * `nu` (dimensionless), density `rho` (kg/m3). Mirrors Rust `FemMaterial`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemMaterial {
  id: string;
  name: string;
  e: number;
  g: number;
  nu: number;
  rho: number;
}

/** 📐️ Cross-section properties: area (m2), second moments of area about local y/z (m4), torsion
 * constant (m4). Mirrors Rust `FemSection` (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemSection {
  id: string;
  name: string;
  area: number;
  iy: number;
  iz: number;
  j: number;
}

/** 🛡️ A support: the subset of a node's DOFs restrained to zero displacement. Mirrors Rust
 * `FemSupport` (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemSupport {
  id: string;
  nodeId: string;
  fixed: FemDof[];
}

/** 🧱️ A meshed continuum solid — a polygon footprint (with optional holes) extruded upward from
 * `baseZ` by `height` across `layers` equal-height layers. Mirrors Rust `FemSolid`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemSolid {
  id: string;
  name: string;
  outline: [number, number][];
  holes: [number, number][][];
  baseZ: number;
  height: number;
  layers: number;
  meshSize: number;
  materialId: string;
}

/** 🏋️ A load — a concentrated nodal force/moment, a member UDL on a bar/frame element, or a normal
 * pressure over a meshed solid's top face. Mirrors Rust `FemLoad` (`🗿️artifacts/🧊️3d/🦀️.rs`),
 * tagged on `kind`. */
export type FemLoad =
  | { kind: "nodal"; id: string; nodeId: string; dof: FemDof; value: number }
  | { kind: "memberUdl"; id: string; elementId: string; wx: number; wy: number; wz: number }
  | { kind: "area"; id: string; solidId: string; pressure: number };

/** 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
 * Mirrors Rust `FemLoadCase` (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemLoadCase {
  id: string;
  name: string;
  loads: FemLoad[];
  selfWeight: boolean;
}

/** 📦️ A linear combination of load cases — case id → factor terms superposed from already-solved
 * case results. Mirrors Rust `FemCombination` (`🗿️artifacts/🧊️3d/🦀️.rs`, `BTreeMap<String, f64>`). */
export interface FemCombination {
  id: string;
  name: string;
  terms: Record<string, number>;
}

/** ⚙️ Analysis settings: mode/factor counts for modal and buckling analyses, plus a deformation
 * display scale for the UI layer. Mirrors Rust `FemAnalysisSettings`
 * (`🗿️artifacts/🧊️3d/🦀️.rs`, re-exported from `fem2d::FemAnalysisSettings`). */
export interface FemAnalysisSettings {
  modalCount: number;
  bucklingCount: number;
  deformationScale: number;
}

/** 🎥️ Opaque camera state string; the plugin layer owns and interprets its shape. Mirrors Rust
 * `FemCamera` (`🗿️artifacts/🧊️3d/🦀️.rs`). */
export interface FemCamera {
  json: string;
}
//#endregion 🔖️Entities

export interface Fem3dArtifact {
  /** @state artifact */
  nodes: FemNode[];
  /** @state artifact */
  elements: FemElement[];
  /** @state artifact */
  materials: FemMaterial[];
  /** @state artifact */
  sections: FemSection[];
  /** @state artifact */
  solids: FemSolid[];
  /** @state artifact */
  supports: FemSupport[];
  /** @state artifact */
  loadCases: FemLoadCase[];
  /** @state artifact */
  combinations: FemCombination[];
  /** @state artifact */
  analysis: FemAnalysisSettings;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem3dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem3dArtifactGuardReject = (at: string, why: string): never => {
  throw new femFem3dArtifactGuardRefusal(at, why);
};

type femFem3dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem3dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem3dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem3dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem3dArtifactGuardReject(at, "value is not an object");
export const femFem3dArtifactGuardArray = (value: unknown, at: string, bounds: femFem3dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem3dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem3dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem3dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem3dArtifactGuardString = (value: unknown, at: string, bounds: femFem3dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem3dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem3dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem3dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem3dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem3dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem3dArtifactGuardReject(at, "value is not a boolean"));
export const femFem3dArtifactGuardNumber = (value: unknown, at: string, bounds: femFem3dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem3dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem3dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem3dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem3dArtifactGuardInteger = (value: unknown, at: string, bounds: femFem3dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem3dArtifactGuardNumber(value, at, bounds) : femFem3dArtifactGuardReject(at, "value is not an integer");
export const femFem3dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem3dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem3dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem3dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem3dArtifact(value: unknown, at = "$"): Fem3dArtifact {
  const row = femFem3dArtifactGuardObject(value, at);
  return {
    nodes: femFem3dArtifactGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseFemNode(item, `${at}.nodes[${index}]`)),
    elements: femFem3dArtifactGuardArray(row["elements"], `${at}.elements`).map((item, index) => parseFemElement(item, `${at}.elements[${index}]`)),
    materials: femFem3dArtifactGuardArray(row["materials"], `${at}.materials`).map((item, index) => parseFemMaterial(item, `${at}.materials[${index}]`)),
    sections: femFem3dArtifactGuardArray(row["sections"], `${at}.sections`).map((item, index) => parseFemSection(item, `${at}.sections[${index}]`)),
    solids: femFem3dArtifactGuardArray(row["solids"], `${at}.solids`).map((item, index) => parseFemSolid(item, `${at}.solids[${index}]`)),
    supports: femFem3dArtifactGuardArray(row["supports"], `${at}.supports`).map((item, index) => parseFemSupport(item, `${at}.supports[${index}]`)),
    loadCases: femFem3dArtifactGuardArray(row["loadCases"], `${at}.loadCases`).map((item, index) => parseFemLoadCase(item, `${at}.loadCases[${index}]`)),
    combinations: femFem3dArtifactGuardArray(row["combinations"], `${at}.combinations`).map((item, index) => parseFemCombination(item, `${at}.combinations[${index}]`)),
    analysis: parseFemAnalysisSettings(row["analysis"], `${at}.analysis`),
  };
}

export function parseFemCamera(value: unknown, at = "$"): FemCamera {
  const row = femFem3dArtifactGuardObject(value, at);
  return {
    json: femFem3dArtifactGuardString(row["json"], `${at}.json`),
  };
}

export function parseFemAnalysisSettings(value: unknown, at = "$"): FemAnalysisSettings {
  const row = femFem3dArtifactGuardObject(value, at);
  return {
    modalCount: femFem3dArtifactGuardInteger(row["modalCount"], `${at}.modalCount`, {"minimum": 0}),
    bucklingCount: femFem3dArtifactGuardInteger(row["bucklingCount"], `${at}.bucklingCount`, {"minimum": 0}),
    deformationScale: femFem3dArtifactGuardNumber(row["deformationScale"], `${at}.deformationScale`),
  };
}

export function parseFemNode(value: unknown, at = "$"): FemNode {
  const row = femFem3dArtifactGuardObject(value, at);
  return {
    id: femFem3dArtifactGuardString(row["id"], `${at}.id`),
    x: femFem3dArtifactGuardNumber(row["x"], `${at}.x`),
    y: femFem3dArtifactGuardNumber(row["y"], `${at}.y`),
    z: femFem3dArtifactGuardNumber(row["z"], `${at}.z`),
  };
}

export function parseFemSolid(value: unknown, at = "$"): FemSolid {
  const row = femFem3dArtifactGuardObject(value, at);
  return {
    id: femFem3dArtifactGuardString(row["id"], `${at}.id`),
    name: femFem3dArtifactGuardString(row["name"], `${at}.name`),
    outline: femFem3dArtifactGuardArray(row["outline"], `${at}.outline`).map((item, index) => femFem3dArtifactGuardArray(item, `${at}.outline[${index}]`, {"minItems": 2, "maxItems": 2}).map((item, index) => femFem3dArtifactGuardNumber(item, `${at}.outline[${index}][${index}]`))),
    holes: femFem3dArtifactGuardArray(row["holes"], `${at}.holes`).map((item, index) => femFem3dArtifactGuardArray(item, `${at}.holes[${index}]`).map((item, index) => femFem3dArtifactGuardArray(item, `${at}.holes[${index}][${index}]`, {"minItems": 2, "maxItems": 2}).map((item, index) => femFem3dArtifactGuardNumber(item, `${at}.holes[${index}][${index}][${index}]`)))),
    baseZ: femFem3dArtifactGuardNumber(row["baseZ"], `${at}.baseZ`),
    height: femFem3dArtifactGuardNumber(row["height"], `${at}.height`),
    layers: femFem3dArtifactGuardInteger(row["layers"], `${at}.layers`, {"minimum": 0}),
    meshSize: femFem3dArtifactGuardNumber(row["meshSize"], `${at}.meshSize`),
    materialId: femFem3dArtifactGuardString(row["materialId"], `${at}.materialId`),
  };
}

export function parseFemMaterial(value: unknown, at = "$"): FemMaterial {
  const row = femFem3dArtifactGuardObject(value, at);
  return {
    id: femFem3dArtifactGuardString(row["id"], `${at}.id`),
    name: femFem3dArtifactGuardString(row["name"], `${at}.name`),
    e: femFem3dArtifactGuardNumber(row["e"], `${at}.e`),
    g: femFem3dArtifactGuardNumber(row["g"], `${at}.g`),
    nu: femFem3dArtifactGuardNumber(row["nu"], `${at}.nu`),
    rho: femFem3dArtifactGuardNumber(row["rho"], `${at}.rho`),
  };
}

export function parseFemSection(value: unknown, at = "$"): FemSection {
  const row = femFem3dArtifactGuardObject(value, at);
  return {
    id: femFem3dArtifactGuardString(row["id"], `${at}.id`),
    name: femFem3dArtifactGuardString(row["name"], `${at}.name`),
    area: femFem3dArtifactGuardNumber(row["area"], `${at}.area`),
    iy: femFem3dArtifactGuardNumber(row["iy"], `${at}.iy`),
    iz: femFem3dArtifactGuardNumber(row["iz"], `${at}.iz`),
    j: femFem3dArtifactGuardNumber(row["j"], `${at}.j`),
  };
}

export function parseFemSupport(value: unknown, at = "$"): FemSupport {
  const row = femFem3dArtifactGuardObject(value, at);
  return {
    id: femFem3dArtifactGuardString(row["id"], `${at}.id`),
    nodeId: femFem3dArtifactGuardString(row["nodeId"], `${at}.nodeId`),
    fixed: femFem3dArtifactGuardArray(row["fixed"], `${at}.fixed`).map((item, index) => femFem3dArtifactGuardMember(item, `${at}.fixed[${index}]`, ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const)),
  };
}

export function parseFemCombination(value: unknown, at = "$"): FemCombination {
  const row = femFem3dArtifactGuardObject(value, at);
  return {
    id: femFem3dArtifactGuardString(row["id"], `${at}.id`),
    name: femFem3dArtifactGuardString(row["name"], `${at}.name`),
    terms: femFem3dArtifactGuardObject(row["terms"], `${at}.terms`),
  };
}
