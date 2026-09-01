/** 🧬️ Fem3d diff schema — sparse field delta. */

//#region 🔖️Entities
/** 📍️ A structural node: a stable id and a global position, plain SI meters. Mirrors Rust `FemNode`
 * (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemNode {
  id: string;
  x: number;
  y: number;
  z: number;
}

/** 🔒️ A DOF tag mirroring the FEM 3D degrees of freedom. Mirrors Rust `FemDof`
 * (`🗿️artifacts/🧊️3d/🦀️component.rs`, re-exported from `fem2d::FemDof`). */
export type FemDof = "Tx" | "Ty" | "Tz" | "Rx" | "Ry" | "Rz";

/** 🔩️ A two-node member: an axial `Bar` or a full 6-DOF `Frame` with a local-axis `roll` angle
 * (radians). Mirrors Rust `FemElement` (`🗿️artifacts/🧊️3d/🦀️component.rs`), tagged on `kind`. */
export type FemElement =
  | { kind: "bar"; id: string; start: string; end: string; materialId: string; sectionId: string }
  | { kind: "frame"; id: string; start: string; end: string; materialId: string; sectionId: string; roll: number };

/** 🧱️ Linear-elastic isotropic material: Young's modulus `e`, shear modulus `g` (Pa), Poisson's ratio
 * `nu` (dimensionless), density `rho` (kg/m3). Mirrors Rust `FemMaterial`
 * (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemMaterial {
  id: string;
  name: string;
  e: number;
  g: number;
  nu: number;
  rho: number;
}

/** 📐️ Cross-section properties: area (m2), second moments of area about local y/z (m4), torsion
 * constant (m4). Mirrors Rust `FemSection` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemSection {
  id: string;
  name: string;
  area: number;
  iy: number;
  iz: number;
  j: number;
}

/** 🛡️ A support: the subset of a node's DOFs restrained to zero displacement. Mirrors Rust
 * `FemSupport` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemSupport {
  id: string;
  nodeId: string;
  fixed: FemDof[];
}

/** 🧱️ A meshed continuum solid — a polygon footprint (with optional holes) extruded upward from
 * `baseZ` by `height` across `layers` equal-height layers. Mirrors Rust `FemSolid`
 * (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
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
 * pressure over a meshed solid's top face. Mirrors Rust `FemLoad` (`🗿️artifacts/🧊️3d/🦀️component.rs`),
 * tagged on `kind`. */
export type FemLoad =
  | { kind: "nodal"; id: string; nodeId: string; dof: FemDof; value: number }
  | { kind: "memberUdl"; id: string; elementId: string; wx: number; wy: number; wz: number }
  | { kind: "area"; id: string; solidId: string; pressure: number };

/** 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
 * Mirrors Rust `FemLoadCase` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemLoadCase {
  id: string;
  name: string;
  loads: FemLoad[];
  selfWeight: boolean;
}

/** 📦️ A linear combination of load cases — case id → factor terms superposed from already-solved
 * case results. Mirrors Rust `FemCombination` (`🗿️artifacts/🧊️3d/🦀️component.rs`, `BTreeMap<String, f64>`). */
export interface FemCombination {
  id: string;
  name: string;
  terms: Record<string, number>;
}

/** ⚙️ Analysis settings: mode/factor counts for modal and buckling analyses, plus a deformation
 * display scale for the UI layer. Mirrors Rust `FemAnalysisSettings`
 * (`🗿️artifacts/🧊️3d/🦀️component.rs`, re-exported from `fem2d::FemAnalysisSettings`). */
export interface FemAnalysisSettings {
  modalCount: number;
  bucklingCount: number;
  deformationScale: number;
}

/** 🎥️ Opaque camera state string; the plugin layer owns and interprets its shape. Mirrors Rust
 * `FemCamera` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemCamera {
  json: string;
}

/** 🧬️ The full `Fem3dArtifact` shape, duplicated here for the sparse diff's `artifact` replacement
 * field. Mirrors `../🟦️component.ts`'s `Fem3dArtifact`. */
export interface Fem3dArtifact {
  nodes: FemNode[];
  elements: FemElement[];
  materials: FemMaterial[];
  sections: FemSection[];
  solids: FemSolid[];
  supports: FemSupport[];
  loadCases: FemLoadCase[];
  combinations: FemCombination[];
  analysis: FemAnalysisSettings;
  resultSourceId?: string;
  resultMode: string;
  resultModeIndex: number;
  camera: FemCamera;
  solverResultsJson: string;
  meshPreviewJson: string;
}
//#endregion 🔖️Entities

export interface Fem3dDiff {
  /** @state artifact */
  artifact?: Fem3dArtifact;
  /** @state artifact */
  nodes?: Fem3dNodesDelta;
  /** @state artifact */
  elements?: Fem3dElementsDelta;
  /** @state artifact */
  materials?: Fem3dMaterialsDelta;
  /** @state artifact */
  sections?: Fem3dSectionsDelta;
  /** @state artifact */
  solids?: Fem3dSolidsDelta;
  /** @state artifact */
  supports?: Fem3dSupportsDelta;
  /** @state artifact */
  loadCases?: Fem3dLoadCasesDelta;
  /** @state artifact */
  combinations?: Fem3dCombinationsDelta;
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
  /** @state artifact */
  solverResultsJson?: string;
  /** @state artifact */
  meshPreviewJson?: string;
}

export interface Fem3dNodesDelta {
  added: FemNode[];
  removed: string[];
  patched: Fem3dNodesPatchEntry[];
  reordered?: string[];
}

export interface Fem3dNodesPatchEntry {
  id: string;
  item: FemNode;
}

export interface Fem3dElementsDelta {
  added: FemElement[];
  removed: string[];
  patched: Fem3dElementsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dElementsPatchEntry {
  id: string;
  item: FemElement;
}

export interface Fem3dMaterialsDelta {
  added: FemMaterial[];
  removed: string[];
  patched: Fem3dMaterialsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dMaterialsPatchEntry {
  id: string;
  item: FemMaterial;
}

export interface Fem3dSectionsDelta {
  added: FemSection[];
  removed: string[];
  patched: Fem3dSectionsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dSectionsPatchEntry {
  id: string;
  item: FemSection;
}

export interface Fem3dSolidsDelta {
  added: FemSolid[];
  removed: string[];
  patched: Fem3dSolidsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dSolidsPatchEntry {
  id: string;
  item: FemSolid;
}

export interface Fem3dSupportsDelta {
  added: FemSupport[];
  removed: string[];
  patched: Fem3dSupportsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dSupportsPatchEntry {
  id: string;
  item: FemSupport;
}

export interface Fem3dLoadCasesDelta {
  added: FemLoadCase[];
  removed: string[];
  patched: Fem3dLoadCasesPatchEntry[];
  reordered?: string[];
}

export interface Fem3dLoadCasesPatchEntry {
  id: string;
  item: FemLoadCase;
}

export interface Fem3dCombinationsDelta {
  added: FemCombination[];
  removed: string[];
  patched: Fem3dCombinationsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dCombinationsPatchEntry {
  id: string;
  item: FemCombination;
}

