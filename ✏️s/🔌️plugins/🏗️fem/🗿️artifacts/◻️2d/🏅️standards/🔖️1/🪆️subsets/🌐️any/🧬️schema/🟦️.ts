/** 🧬️ Fem2d artifact schema — every field with its state class. */

//#region 🔖️Entities
/** 📍️ A structural node in plan (x, y in meters). Mirrors Rust `FemNode` (`🦀️.rs`). */
export interface FemNode {
  id: string;
  x: number;
  y: number;
}

/** 🔒️ A DOF tag mirroring the FEM 2D degrees of freedom. Mirrors Rust `FemDof` (`🦀️.rs`). */
export type FemDof = "Tx" | "Ty" | "Tz" | "Rx" | "Ry" | "Rz";

/** 🔩️ A 2-node structural member — axial-only bar or axial+bending beam. Mirrors Rust `FemElement`
 * (`🦀️.rs`), tagged on `kind`. */
export type FemElement =
  | { kind: "bar"; id: string; start: string; end: string; materialId: string; sectionId: string }
  | { kind: "beam"; id: string; start: string; end: string; materialId: string; sectionId: string };

/** 🧱️ An isotropic material — Young's modulus `e` (Pa), Poisson's ratio `nu`, density `rho` (kg/m3).
 * Mirrors Rust `FemMaterial` (`🦀️.rs`). */
export interface FemMaterial {
  id: string;
  name: string;
  e: number;
  nu: number;
  rho: number;
}

/** 📏️ A cross-section — area (m2) and strong-axis moment of inertia `iy` (m4). Mirrors Rust
 * `FemSection` (`🦀️.rs`). */
export interface FemSection {
  id: string;
  name: string;
  area: number;
  iy: number;
}

/** 🛡️ A support: the subset of a node's DOFs restrained to zero displacement. Mirrors Rust
 * `FemSupport` (`🦀️.rs`). */
export interface FemSupport {
  id: string;
  nodeId: string;
  fixed: FemDof[];
}

/** 🏋️ A load — a concentrated nodal force/moment, a member UDL, or a pressure over a meshed region.
 * Mirrors Rust `FemLoad` (`🦀️.rs`), tagged on `kind`. */
export type FemLoad =
  | { kind: "nodal"; id: string; nodeId: string; dof: FemDof; value: number }
  | { kind: "memberUdl"; id: string; elementId: string; wx: number; wy: number }
  | { kind: "area"; id: string; regionId: string; pressure: number };

/** 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
 * Mirrors Rust `FemLoadCase` (`🦀️.rs`). */
export interface FemLoadCase {
  id: string;
  name: string;
  loads: FemLoad[];
  selfWeight: boolean;
}

/** 🟩️ A meshed continuum region — a polygon (with optional holes) filled at solve time. Mirrors Rust
 * `FemRegion` (`🦀️.rs`). */
export interface FemRegion {
  id: string;
  name: string;
  outline: [number, number][];
  holes: [number, number][][];
  thickness: number;
  materialId: string;
  meshSize: number;
}

/** 🔗️ One combination term — a referenced load case id and its scale factor. Mirrors Rust
 * `FemCombinationTerm` (`🦀️.rs`). */
export interface FemCombinationTerm {
  caseId: string;
  factor: number;
}

/** 🧮️ A linear combination of load cases — terms superposed at solve time. Mirrors Rust
 * `FemCombination` (`🦀️.rs`). */
export interface FemCombination {
  id: string;
  name: string;
  terms: FemCombinationTerm[];
}

/** ⚙️ Analysis settings — modal/buckling mode counts and the viewport deformation scale factor.
 * Mirrors Rust `FemAnalysisSettings` (`🦀️.rs`). */
export interface FemAnalysisSettings {
  modalCount: number;
  bucklingCount: number;
  deformationScale: number;
}

/** 🎥️ The canvas camera (pan/zoom) for the plugin viewport. Mirrors Rust `FemCamera`
 * (`🦀️.rs`). */
export interface FemCamera {
  x: number;
  y: number;
  zoom: number;
}
//#endregion 🔖️Entities

export interface Fem2dArtifact {
  /** @state artifact */
  nodes: FemNode[];
  /** @state artifact */
  elements: FemElement[];
  /** @state artifact */
  regions: FemRegion[];
  /** @state artifact */
  materials: FemMaterial[];
  /** @state artifact */
  sections: FemSection[];
  /** @state artifact */
  supports: FemSupport[];
  /** @state artifact */
  loadCases: FemLoadCase[];
  /** @state artifact */
  combinations: FemCombination[];
  /** @state artifact */
  analysis: FemAnalysisSettings;
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem2dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem2dArtifactGuardReject = (at: string, why: string): never => {
  throw new femFem2dArtifactGuardRefusal(at, why);
};

type femFem2dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem2dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem2dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem2dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem2dArtifactGuardReject(at, "value is not an object");
export const femFem2dArtifactGuardArray = (value: unknown, at: string, bounds: femFem2dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem2dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem2dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem2dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem2dArtifactGuardString = (value: unknown, at: string, bounds: femFem2dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem2dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem2dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem2dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem2dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem2dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem2dArtifactGuardReject(at, "value is not a boolean"));
export const femFem2dArtifactGuardNumber = (value: unknown, at: string, bounds: femFem2dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem2dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem2dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem2dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem2dArtifactGuardInteger = (value: unknown, at: string, bounds: femFem2dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem2dArtifactGuardNumber(value, at, bounds) : femFem2dArtifactGuardReject(at, "value is not an integer");
export const femFem2dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem2dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem2dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem2dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem2dArtifact(value: unknown, at = "$"): Fem2dArtifact {
  const row = femFem2dArtifactGuardObject(value, at);
  return {
    nodes: femFem2dArtifactGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseFemNode(item, `${at}.nodes[${index}]`)),
    elements: femFem2dArtifactGuardArray(row["elements"], `${at}.elements`).map((item, index) => parseFemElement(item, `${at}.elements[${index}]`)),
    regions: femFem2dArtifactGuardArray(row["regions"], `${at}.regions`).map((item, index) => parseFemRegion(item, `${at}.regions[${index}]`)),
    materials: femFem2dArtifactGuardArray(row["materials"], `${at}.materials`).map((item, index) => parseFemMaterial(item, `${at}.materials[${index}]`)),
    sections: femFem2dArtifactGuardArray(row["sections"], `${at}.sections`).map((item, index) => parseFemSection(item, `${at}.sections[${index}]`)),
    supports: femFem2dArtifactGuardArray(row["supports"], `${at}.supports`).map((item, index) => parseFemSupport(item, `${at}.supports[${index}]`)),
    loadCases: femFem2dArtifactGuardArray(row["loadCases"], `${at}.loadCases`).map((item, index) => parseFemLoadCase(item, `${at}.loadCases[${index}]`)),
    combinations: femFem2dArtifactGuardArray(row["combinations"], `${at}.combinations`).map((item, index) => parseFemCombination(item, `${at}.combinations[${index}]`)),
    analysis: parseFemAnalysisSettings(row["analysis"], `${at}.analysis`),
  };
}

export function parseFemCamera(value: unknown, at = "$"): FemCamera {
  const row = femFem2dArtifactGuardObject(value, at);
  return {
    x: femFem2dArtifactGuardNumber(row["x"], `${at}.x`),
    y: femFem2dArtifactGuardNumber(row["y"], `${at}.y`),
    zoom: femFem2dArtifactGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

export function parseFemAnalysisSettings(value: unknown, at = "$"): FemAnalysisSettings {
  const row = femFem2dArtifactGuardObject(value, at);
  return {
    modalCount: femFem2dArtifactGuardInteger(row["modalCount"], `${at}.modalCount`, {"minimum": 0}),
    bucklingCount: femFem2dArtifactGuardInteger(row["bucklingCount"], `${at}.bucklingCount`, {"minimum": 0}),
    deformationScale: femFem2dArtifactGuardNumber(row["deformationScale"], `${at}.deformationScale`),
  };
}

export function parseFemNode(value: unknown, at = "$"): FemNode {
  const row = femFem2dArtifactGuardObject(value, at);
  return {
    id: femFem2dArtifactGuardString(row["id"], `${at}.id`),
    x: femFem2dArtifactGuardNumber(row["x"], `${at}.x`),
    y: femFem2dArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseFemRegion(value: unknown, at = "$"): FemRegion {
  const row = femFem2dArtifactGuardObject(value, at);
  return {
    id: femFem2dArtifactGuardString(row["id"], `${at}.id`),
    name: femFem2dArtifactGuardString(row["name"], `${at}.name`),
    outline: femFem2dArtifactGuardArray(row["outline"], `${at}.outline`, {"minItems": 3}).map((item, index) => femFem2dArtifactGuardArray(item, `${at}.outline[${index}]`, {"minItems": 2, "maxItems": 2}).map((item, index) => femFem2dArtifactGuardNumber(item, `${at}.outline[${index}][${index}]`))),
    holes: femFem2dArtifactGuardArray(row["holes"], `${at}.holes`).map((item, index) => femFem2dArtifactGuardArray(item, `${at}.holes[${index}]`, {"minItems": 3}).map((item, index) => femFem2dArtifactGuardArray(item, `${at}.holes[${index}][${index}]`, {"minItems": 2, "maxItems": 2}).map((item, index) => femFem2dArtifactGuardNumber(item, `${at}.holes[${index}][${index}][${index}]`)))),
    thickness: femFem2dArtifactGuardNumber(row["thickness"], `${at}.thickness`),
    materialId: femFem2dArtifactGuardString(row["materialId"], `${at}.materialId`),
    meshSize: femFem2dArtifactGuardNumber(row["meshSize"], `${at}.meshSize`),
  };
}

export function parseFemMaterial(value: unknown, at = "$"): FemMaterial {
  const row = femFem2dArtifactGuardObject(value, at);
  return {
    id: femFem2dArtifactGuardString(row["id"], `${at}.id`),
    name: femFem2dArtifactGuardString(row["name"], `${at}.name`),
    e: femFem2dArtifactGuardNumber(row["e"], `${at}.e`),
    nu: femFem2dArtifactGuardNumber(row["nu"], `${at}.nu`),
    rho: femFem2dArtifactGuardNumber(row["rho"], `${at}.rho`),
  };
}

export function parseFemSection(value: unknown, at = "$"): FemSection {
  const row = femFem2dArtifactGuardObject(value, at);
  return {
    id: femFem2dArtifactGuardString(row["id"], `${at}.id`),
    name: femFem2dArtifactGuardString(row["name"], `${at}.name`),
    area: femFem2dArtifactGuardNumber(row["area"], `${at}.area`),
    iy: femFem2dArtifactGuardNumber(row["iy"], `${at}.iy`),
  };
}

export function parseFemSupport(value: unknown, at = "$"): FemSupport {
  const row = femFem2dArtifactGuardObject(value, at);
  return {
    id: femFem2dArtifactGuardString(row["id"], `${at}.id`),
    nodeId: femFem2dArtifactGuardString(row["nodeId"], `${at}.nodeId`),
    fixed: femFem2dArtifactGuardArray(row["fixed"], `${at}.fixed`).map((item, index) => femFem2dArtifactGuardMember(item, `${at}.fixed[${index}]`, ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const)),
  };
}
