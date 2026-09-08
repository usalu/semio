/** 🧬️ Fem2d snapshot schema — artifact-lane fields only. */

//#region 🔖️Entities
/** 📍️ A structural node in plan (x, y in meters). Mirrors Rust `FemNode` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemNode {
  id: string;
  x: number;
  y: number;
}

/** 🔒️ A DOF tag mirroring the FEM 2D degrees of freedom. Mirrors Rust `FemDof`
 * (`🗿️artifacts/◻️2d/🦀️.rs`). */
export type FemDof = "Tx" | "Ty" | "Tz" | "Rx" | "Ry" | "Rz";

/** 🔩️ A 2-node structural member — axial-only bar or axial+bending beam. Mirrors Rust `FemElement`
 * (`🗿️artifacts/◻️2d/🦀️.rs`), tagged on `kind`. */
export type FemElement =
  | { kind: "bar"; id: string; start: string; end: string; materialId: string; sectionId: string }
  | { kind: "beam"; id: string; start: string; end: string; materialId: string; sectionId: string };

/** 🧱️ An isotropic material — Young's modulus `e` (Pa), Poisson's ratio `nu`, density `rho` (kg/m3).
 * Mirrors Rust `FemMaterial` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemMaterial {
  id: string;
  name: string;
  e: number;
  nu: number;
  rho: number;
}

/** 📏️ A cross-section — area (m2) and strong-axis moment of inertia `iy` (m4). Mirrors Rust
 * `FemSection` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemSection {
  id: string;
  name: string;
  area: number;
  iy: number;
}

/** 🛡️ A support: the subset of a node's DOFs restrained to zero displacement. Mirrors Rust
 * `FemSupport` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemSupport {
  id: string;
  nodeId: string;
  fixed: FemDof[];
}

/** 🏋️ A load — a concentrated nodal force/moment, a member UDL, or a pressure over a meshed region.
 * Mirrors Rust `FemLoad` (`🗿️artifacts/◻️2d/🦀️.rs`), tagged on `kind`. */
export type FemLoad =
  | { kind: "nodal"; id: string; nodeId: string; dof: FemDof; value: number }
  | { kind: "memberUdl"; id: string; elementId: string; wx: number; wy: number }
  | { kind: "area"; id: string; regionId: string; pressure: number };

/** 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
 * Mirrors Rust `FemLoadCase` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemLoadCase {
  id: string;
  name: string;
  loads: FemLoad[];
  selfWeight: boolean;
}

/** 🟩️ A meshed continuum region — a polygon (with optional holes) filled at solve time. Mirrors Rust
 * `FemRegion` (`🗿️artifacts/◻️2d/🦀️.rs`). */
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
 * `FemCombinationTerm` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemCombinationTerm {
  caseId: string;
  factor: number;
}

/** 🧮️ A linear combination of load cases — terms superposed at solve time. Mirrors Rust
 * `FemCombination` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemCombination {
  id: string;
  name: string;
  terms: FemCombinationTerm[];
}

/** ⚙️ Analysis settings — modal/buckling mode counts and the viewport deformation scale factor.
 * Mirrors Rust `FemAnalysisSettings` (`🗿️artifacts/◻️2d/🦀️.rs`). */
export interface FemAnalysisSettings {
  modalCount: number;
  bucklingCount: number;
  deformationScale: number;
}
//#endregion 🔖️Entities

export interface Fem2dSnapshot {
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
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem2dSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem2dSnapshotGuardReject = (at: string, why: string): never => {
  throw new femFem2dSnapshotGuardRefusal(at, why);
};

type femFem2dSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem2dSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem2dSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem2dSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem2dSnapshotGuardReject(at, "value is not an object");
export const femFem2dSnapshotGuardArray = (value: unknown, at: string, bounds: femFem2dSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem2dSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem2dSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem2dSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem2dSnapshotGuardString = (value: unknown, at: string, bounds: femFem2dSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem2dSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem2dSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem2dSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem2dSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem2dSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem2dSnapshotGuardReject(at, "value is not a boolean"));
export const femFem2dSnapshotGuardNumber = (value: unknown, at: string, bounds: femFem2dSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem2dSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem2dSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem2dSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem2dSnapshotGuardInteger = (value: unknown, at: string, bounds: femFem2dSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem2dSnapshotGuardNumber(value, at, bounds) : femFem2dSnapshotGuardReject(at, "value is not an integer");
export const femFem2dSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem2dSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem2dSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem2dSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFemAnalysisSettings(value: unknown, at = "$"): FemAnalysisSettings {
  const row = femFem2dSnapshotGuardObject(value, at);
  return {
    modalCount: femFem2dSnapshotGuardInteger(row["modalCount"], `${at}.modalCount`, {"minimum": 1}),
    bucklingCount: femFem2dSnapshotGuardInteger(row["bucklingCount"], `${at}.bucklingCount`, {"minimum": 1}),
    deformationScale: femFem2dSnapshotGuardNumber(row["deformationScale"], `${at}.deformationScale`),
  };
}
