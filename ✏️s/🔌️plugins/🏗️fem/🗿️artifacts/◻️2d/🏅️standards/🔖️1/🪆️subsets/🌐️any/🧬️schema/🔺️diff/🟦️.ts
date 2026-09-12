/** 🧬️ Fem2d diff schema — sparse field delta. */

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


/** 🧬️ The full `Fem2dArtifact` shape, duplicated here for the sparse diff's `artifact` replacement
 * field. Mirrors `../🟦️.ts`'s `Fem2dArtifact`. */
export interface Fem2dArtifact {
  nodes: FemNode[];
  elements: FemElement[];
  regions: FemRegion[];
  materials: FemMaterial[];
  sections: FemSection[];
  supports: FemSupport[];
  loadCases: FemLoadCase[];
  combinations: FemCombination[];
  analysis: FemAnalysisSettings;
}
//#endregion 🔖️Entities

export interface Fem2dDiff {
  /** @state artifact */
  artifact?: Fem2dArtifact;
  /** @state artifact */
  nodes?: Fem2dNodesDelta;
  /** @state artifact */
  elements?: Fem2dElementsDelta;
  /** @state artifact */
  regions?: Fem2dRegionsDelta;
  /** @state artifact */
  materials?: Fem2dMaterialsDelta;
  /** @state artifact */
  sections?: Fem2dSectionsDelta;
  /** @state artifact */
  supports?: Fem2dSupportsDelta;
  /** @state artifact */
  loadCases?: Fem2dLoadCasesDelta;
  /** @state artifact */
  combinations?: Fem2dCombinationsDelta;
  /** @state artifact */
  analysis?: FemAnalysisSettings;
  /** @state config */
}

export interface Fem2dNodesDelta {
  added: FemNode[];
  removed: string[];
  patched: Fem2dNodesPatchEntry[];
  reordered?: string[];
}

export interface Fem2dNodesPatchEntry {
  id: string;
  item: FemNode;
}

export interface Fem2dElementsDelta {
  added: FemElement[];
  removed: string[];
  patched: Fem2dElementsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dElementsPatchEntry {
  id: string;
  item: FemElement;
}

export interface Fem2dRegionsDelta {
  added: FemRegion[];
  removed: string[];
  patched: Fem2dRegionsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dRegionsPatchEntry {
  id: string;
  item: FemRegion;
}

export interface Fem2dMaterialsDelta {
  added: FemMaterial[];
  removed: string[];
  patched: Fem2dMaterialsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dMaterialsPatchEntry {
  id: string;
  item: FemMaterial;
}

export interface Fem2dSectionsDelta {
  added: FemSection[];
  removed: string[];
  patched: Fem2dSectionsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dSectionsPatchEntry {
  id: string;
  item: FemSection;
}

export interface Fem2dSupportsDelta {
  added: FemSupport[];
  removed: string[];
  patched: Fem2dSupportsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dSupportsPatchEntry {
  id: string;
  item: FemSupport;
}

export interface Fem2dLoadCasesDelta {
  added: FemLoadCase[];
  removed: string[];
  patched: Fem2dLoadCasesPatchEntry[];
  reordered?: string[];
}

export interface Fem2dLoadCasesPatchEntry {
  id: string;
  item: FemLoadCase;
}

export interface Fem2dCombinationsDelta {
  added: FemCombination[];
  removed: string[];
  patched: Fem2dCombinationsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dCombinationsPatchEntry {
  id: string;
  item: FemCombination;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem2dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem2dDiffGuardReject = (at: string, why: string): never => {
  throw new femFem2dDiffGuardRefusal(at, why);
};

type femFem2dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem2dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem2dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem2dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem2dDiffGuardReject(at, "value is not an object");
export const femFem2dDiffGuardArray = (value: unknown, at: string, bounds: femFem2dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem2dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem2dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem2dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem2dDiffGuardString = (value: unknown, at: string, bounds: femFem2dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem2dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem2dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem2dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem2dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem2dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem2dDiffGuardReject(at, "value is not a boolean"));
export const femFem2dDiffGuardNumber = (value: unknown, at: string, bounds: femFem2dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem2dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem2dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem2dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem2dDiffGuardInteger = (value: unknown, at: string, bounds: femFem2dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem2dDiffGuardNumber(value, at, bounds) : femFem2dDiffGuardReject(at, "value is not an integer");
export const femFem2dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem2dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem2dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem2dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem2dNodesDelta(value: unknown, at = "$"): Fem2dNodesDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dElementsDelta(value: unknown, at = "$"): Fem2dElementsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dRegionsDelta(value: unknown, at = "$"): Fem2dRegionsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dMaterialsDelta(value: unknown, at = "$"): Fem2dMaterialsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dSectionsDelta(value: unknown, at = "$"): Fem2dSectionsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dSupportsDelta(value: unknown, at = "$"): Fem2dSupportsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dLoadCasesDelta(value: unknown, at = "$"): Fem2dLoadCasesDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dCombinationsDelta(value: unknown, at = "$"): Fem2dCombinationsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => femFem2dDiffGuardObject(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}
