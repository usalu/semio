/** 🧬️ Fem2d diff schema — sparse field delta. */

//#region 🔖️Entities
/** 📍️ A structural node in plan (x, y in meters). Mirrors Rust `FemNode` (`🗿️artifacts/◻2d/🦀️component.rs`). */
export interface FemNode {
  id: string;
  x: number;
  y: number;
}

/** 🔒️ A DOF tag mirroring the FEM 2D degrees of freedom. Mirrors Rust `FemDof`
 * (`🗿️artifacts/◻2d/🦀️component.rs`). */
export type FemDof = "Tx" | "Ty" | "Tz" | "Rx" | "Ry" | "Rz";

/** 🔩️ A 2-node structural member — axial-only bar or axial+bending beam. Mirrors Rust `FemElement`
 * (`🗿️artifacts/◻2d/🦀️component.rs`), tagged on `kind`. */
export type FemElement =
  | { kind: "bar"; id: string; start: string; end: string; materialId: string; sectionId: string }
  | { kind: "beam"; id: string; start: string; end: string; materialId: string; sectionId: string };

/** 🧱️ An isotropic material — Young's modulus `e` (Pa), Poisson's ratio `nu`, density `rho` (kg/m3).
 * Mirrors Rust `FemMaterial` (`🗿️artifacts/◻2d/🦀️component.rs`). */
export interface FemMaterial {
  id: string;
  name: string;
  e: number;
  nu: number;
  rho: number;
}

/** 📏️ A cross-section — area (m2) and strong-axis moment of inertia `iy` (m4). Mirrors Rust
 * `FemSection` (`🗿️artifacts/◻2d/🦀️component.rs`). */
export interface FemSection {
  id: string;
  name: string;
  area: number;
  iy: number;
}

/** 🛡️ A support: the subset of a node's DOFs restrained to zero displacement. Mirrors Rust
 * `FemSupport` (`🗿️artifacts/◻2d/🦀️component.rs`). */
export interface FemSupport {
  id: string;
  nodeId: string;
  fixed: FemDof[];
}

/** 🏋️ A load — a concentrated nodal force/moment, a member UDL, or a pressure over a meshed region.
 * Mirrors Rust `FemLoad` (`🗿️artifacts/◻2d/🦀️component.rs`), tagged on `kind`. */
export type FemLoad =
  | { kind: "nodal"; id: string; nodeId: string; dof: FemDof; value: number }
  | { kind: "memberUdl"; id: string; elementId: string; wx: number; wy: number }
  | { kind: "area"; id: string; regionId: string; pressure: number };

/** 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
 * Mirrors Rust `FemLoadCase` (`🗿️artifacts/◻2d/🦀️component.rs`). */
export interface FemLoadCase {
  id: string;
  name: string;
  loads: FemLoad[];
  selfWeight: boolean;
}

/** 🟩️ A meshed continuum region — a polygon (with optional holes) filled at solve time. Mirrors Rust
 * `FemRegion` (`🗿️artifacts/◻2d/🦀️component.rs`). */
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
 * `FemCombinationTerm` (`🗿️artifacts/◻2d/🦀️component.rs`). */
export interface FemCombinationTerm {
  caseId: string;
  factor: number;
}

/** 🧮️ A linear combination of load cases — terms superposed at solve time. Mirrors Rust
 * `FemCombination` (`🗿️artifacts/◻2d/🦀️component.rs`). */
export interface FemCombination {
  id: string;
  name: string;
  terms: FemCombinationTerm[];
}

/** ⚙️ Analysis settings — modal/buckling mode counts and the viewport deformation scale factor.
 * Mirrors Rust `FemAnalysisSettings` (`🗿️artifacts/◻2d/🦀️component.rs`). */
export interface FemAnalysisSettings {
  modalCount: number;
  bucklingCount: number;
  deformationScale: number;
}

/** 🎥️ The canvas camera (pan/zoom) for the plugin viewport. Mirrors Rust `FemCamera`
 * (`🗿️artifacts/◻2d/🦀️component.rs`). */
export interface FemCamera {
  x: number;
  y: number;
  zoom: number;
}

/** 🧬️ The full `Fem2dArtifact` shape, duplicated here for the sparse diff's `artifact` replacement
 * field. Mirrors `../🟦️component.ts`'s `Fem2dArtifact`. */
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
  resultSourceId?: string;
  resultMode: string;
  resultModeIndex: number;
  camera: FemCamera;
  locale: string;
  solverResultsJson: string;
  meshPreviewJson: string;
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
  /** @state presence */
  resultSourceId?: string | null;
  /** @state presence */
  resultMode?: string;
  /** @state presence */
  resultModeIndex?: number;
  /** @state config */
  camera?: FemCamera;
  /** @state config */
  locale?: string;
  /** @state artifact */
  solverResultsJson?: string;
  /** @state artifact */
  meshPreviewJson?: string;
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

