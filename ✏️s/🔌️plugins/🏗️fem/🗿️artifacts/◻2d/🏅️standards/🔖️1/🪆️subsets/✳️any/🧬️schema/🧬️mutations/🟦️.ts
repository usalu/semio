/** ⚡️ Fem2d direct-mutation discriminated union — TS mirror of the Rust `Fem2dMutation` dispatch enum. */

//#region 🔖️Entities
/** 📍️ A structural node in plan (x, y in meters). */
export interface FemNode {
  id: string;
  x: number;
  y: number;
}

/** 🔒️ A DOF tag mirroring the FEM 2D degrees of freedom. */
export type FemDof = "Tx" | "Ty" | "Tz" | "Rx" | "Ry" | "Rz";

/** 🔩️ A 2-node structural member — axial-only bar or axial+bending beam. */
export type FemElement =
  | { kind: "bar"; id: string; start: string; end: string; materialId: string; sectionId: string }
  | { kind: "beam"; id: string; start: string; end: string; materialId: string; sectionId: string };

/** 🧱️ An isotropic material — Young's modulus `e` (Pa), Poisson's ratio `nu`, density `rho` (kg/m3). */
export interface FemMaterial {
  id: string;
  name: string;
  e: number;
  nu: number;
  rho: number;
}

/** 📏️ A cross-section — area (m2) and strong-axis moment of inertia `iy` (m4). */
export interface FemSection {
  id: string;
  name: string;
  area: number;
  iy: number;
}

/** 🛡️ A support: the subset of a node's DOFs restrained to zero displacement. */
export interface FemSupport {
  id: string;
  nodeId: string;
  fixed: FemDof[];
}

/** 🏋️ A load — a concentrated nodal force/moment, a member UDL, or a pressure over a meshed region. */
export type FemLoad =
  | { kind: "nodal"; id: string; nodeId: string; dof: FemDof; value: number }
  | { kind: "memberUdl"; id: string; elementId: string; wx: number; wy: number }
  | { kind: "area"; id: string; regionId: string; pressure: number };

/** 📦️ A named set of loads applied together for one analysis run, optionally including self-weight. */
export interface FemLoadCase {
  id: string;
  name: string;
  loads: FemLoad[];
  selfWeight: boolean;
}

/** 🟩️ A meshed continuum region — a polygon (with optional holes) filled at solve time. */
export interface FemRegion {
  id: string;
  name: string;
  outline: [number, number][];
  holes: [number, number][][];
  thickness: number;
  materialId: string;
  meshSize: number;
}

/** 🔗️ One combination term — a referenced load case id and its scale factor. */
export interface FemCombinationTerm {
  caseId: string;
  factor: number;
}

/** 🧮️ A linear combination of load cases — terms superposed at solve time. */
export interface FemCombination {
  id: string;
  name: string;
  terms: FemCombinationTerm[];
}

/** ⚙️ Analysis settings — modal/buckling mode counts and the viewport deformation scale factor. */
export interface FemAnalysisSettings {
  modalCount: number;
  bucklingCount: number;
  deformationScale: number;
}
//#endregion 🔖️Entities

//#region 🔖️Mutations
/** 🌱⚪️ Brings a new structural node into existence. */
export interface CreateNode {
  node: FemNode;
}

/** 🗑⚪️ Removes an existing structural node by id. */
export interface DeleteNode {
  id: string;
}

/** 🌱🧩️ Brings a new structural member (bar/beam) into existence. */
export interface CreateElement {
  element: FemElement;
}

/** 🗑🧩️ Removes an existing element by id. */
export interface DeleteElement {
  id: string;
}

/** 🔁🧩️ Whole-value swap of an existing element's payload. */
export interface ReplaceElement {
  id: string;
  newElement: FemElement;
}

/** 🌱🧱️ Brings a new material into existence. */
export interface CreateMaterial {
  material: FemMaterial;
}

/** 🗑🧱️ Removes an existing material by id. */
export interface DeleteMaterial {
  id: string;
}

/** 🔁🧱️ Whole-value swap of an existing material's payload. */
export interface ReplaceMaterial {
  id: string;
  newMaterial: FemMaterial;
}

/** 🌱️ Brings a new cross-section into existence. */
export interface CreateSection {
  section: FemSection;
}

/** 🗑📐️ Removes an existing cross-section by id. */
export interface DeleteSection {
  id: string;
}

/** 🔁📐️ Whole-value swap of an existing cross-section's payload. */
export interface ReplaceSection {
  id: string;
  newSection: FemSection;
}

/** 🌱🛡️ Brings a new support into existence. */
export interface CreateSupport {
  support: FemSupport;
}

/** 🗑️ Removes an existing support by id. */
export interface DeleteSupport {
  id: string;
}

/** 🔁️ Whole-value swap of an existing support's payload. */
export interface ReplaceSupport {
  id: string;
  newSupport: FemSupport;
}

/** 🌱🗺️ Brings a new meshed continuum region into existence. */
export interface CreateRegion {
  region: FemRegion;
}

/** 🗑🗺️ Removes an existing meshed region by id. */
export interface DeleteRegion {
  id: string;
}

/** 🔁🗺️ Whole-value swap of an existing meshed region's payload. */
export interface ReplaceRegion {
  id: string;
  newRegion: FemRegion;
}

/** 🌱📋️ Brings a new load case into existence. */
export interface CreateLoadCase {
  loadCase: FemLoadCase;
}

/** 🗑📋️ Removes an existing load case by id. */
export interface DeleteLoadCase {
  id: string;
}

/** ➕️ Attaches a load to an existing load case's `loads` member collection. */
export interface AddLoad {
  caseId: string;
  load: FemLoad;
}

/** ➖️ Detaches a load from an existing load case's `loads` member collection by id. */
export interface RemoveLoad {
  caseId: string;
  loadId: string;
}

/** ⚖️ Sets an existing load case's self-weight flag. */
export interface ChangeLoadCaseSelfWeight {
  caseId: string;
  newSelfWeight: boolean;
}

/** 🌱🔗️ Brings a new load combination into existence. */
export interface CreateCombination {
  combination: FemCombination;
}

/** 🗑🔗️ Removes an existing load combination by id. */
export interface DeleteCombination {
  id: string;
}

/** 🎛️ Atomically updates the document's inseparable analysis-settings facet. */
export interface UpdateAnalysisSettings {
  settings: FemAnalysisSettings;
}
//#endregion 🔖️Mutations

export type Fem2dMutation =
  | ({ mutation: "createNode" } & CreateNode)
  | ({ mutation: "deleteNode" } & DeleteNode)
  | ({ mutation: "createElement" } & CreateElement)
  | ({ mutation: "deleteElement" } & DeleteElement)
  | ({ mutation: "replaceElement" } & ReplaceElement)
  | ({ mutation: "createMaterial" } & CreateMaterial)
  | ({ mutation: "deleteMaterial" } & DeleteMaterial)
  | ({ mutation: "replaceMaterial" } & ReplaceMaterial)
  | ({ mutation: "createSection" } & CreateSection)
  | ({ mutation: "deleteSection" } & DeleteSection)
  | ({ mutation: "replaceSection" } & ReplaceSection)
  | ({ mutation: "createSupport" } & CreateSupport)
  | ({ mutation: "deleteSupport" } & DeleteSupport)
  | ({ mutation: "replaceSupport" } & ReplaceSupport)
  | ({ mutation: "createRegion" } & CreateRegion)
  | ({ mutation: "deleteRegion" } & DeleteRegion)
  | ({ mutation: "replaceRegion" } & ReplaceRegion)
  | ({ mutation: "createLoadCase" } & CreateLoadCase)
  | ({ mutation: "deleteLoadCase" } & DeleteLoadCase)
  | ({ mutation: "addLoad" } & AddLoad)
  | ({ mutation: "removeLoad" } & RemoveLoad)
  | ({ mutation: "changeLoadCaseSelfWeight" } & ChangeLoadCaseSelfWeight)
  | ({ mutation: "createCombination" } & CreateCombination)
  | ({ mutation: "deleteCombination" } & DeleteCombination)
  | ({ mutation: "updateAnalysisSettings" } & UpdateAnalysisSettings);
