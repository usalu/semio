/** 🧩️ Fem3d direct-mutation discriminated union — mirrors the Rust `Fem3dMutation` dispatch enum
 * (sibling `🦀️component.rs`, `#[serde(tag = "mutation", rename_all = "camelCase")]`), same
 * declaration order and camelCase discriminant per variant. No mutation leaf has a TS file on disk
 * for this artifact, so every payload interface — and every shared document entity type it
 * references (`FemDof`, `FemNode`, `FemElement`, `FemMaterial`, `FemSection`, `FemSupport`,
 * `FemSolid`, `FemLoad`, `FemLoadCase`, `FemCombination`, `FemAnalysisSettings`) — is declared
 * locally, each annotated with its Rust source. None of these entity shapes are declared yet in the
 * sibling `../📸️snapshot/🟦️component.ts` (checked first, per convention) to import instead.
 */

/** 🔒️ Mirrors Rust `FemDof` (`🗿️artifacts/🧊️3d/🦀️component.rs`, re-exported from `fem2d::FemDof`) — a
 * fieldless enum, so it serializes as its bare variant name. */
export type FemDof = "Tx" | "Ty" | "Tz" | "Rx" | "Ry" | "Rz";

/** 📍️ Mirrors Rust `FemNode` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemNode {
  id: string;
  x: number;
  y: number;
  z: number;
}

/** 🌱️ Mirrors Rust `CreateNode` (`🌱⚪️create-node/🦀️.rs`). */
export interface CreateNode {
  node: FemNode;
}

/** 🗑️ Mirrors Rust `DeleteNode` (`🗑️⚪️delete-node/🦀️.rs`). */
export interface DeleteNode {
  id: string;
}

/** 🔩️ Mirrors Rust `FemElement` (`🗿️artifacts/🧊️3d/🦀️component.rs`), tagged on `kind`. */
export type FemElement =
  | { kind: "bar"; id: string; start: string; end: string; materialId: string; sectionId: string }
  | { kind: "frame"; id: string; start: string; end: string; materialId: string; sectionId: string; roll: number };

/** 🌱️ Mirrors Rust `CreateElement` (`🌱🧩️create-element/🦀️.rs`). */
export interface CreateElement {
  element: FemElement;
}

/** 🗑️ Mirrors Rust `DeleteElement` (`🗑️🧩️delete-element/🦀️.rs`). */
export interface DeleteElement {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceElement` (`🔁🧩️replace-element/🦀️.rs`). */
export interface ReplaceElement {
  id: string;
  newElement: FemElement;
}

/** 🧱️ Mirrors Rust `FemMaterial` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemMaterial {
  id: string;
  name: string;
  e: number;
  g: number;
  nu: number;
  rho: number;
}

/** 🌱️ Mirrors Rust `CreateMaterial` (`🌱🧱️create-material/🦀️.rs`). */
export interface CreateMaterial {
  material: FemMaterial;
}

/** 🗑️ Mirrors Rust `DeleteMaterial` (`🗑️🧱️delete-material/🦀️.rs`). */
export interface DeleteMaterial {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceMaterial` (`🔁🧱️replace-material/🦀️.rs`). */
export interface ReplaceMaterial {
  id: string;
  newMaterial: FemMaterial;
}

/** 📐️ Mirrors Rust `FemSection` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemSection {
  id: string;
  name: string;
  area: number;
  iy: number;
  iz: number;
  j: number;
}

/** 🌱️ Mirrors Rust `CreateSection` (`🌱create-section/🦀️.rs`). */
export interface CreateSection {
  section: FemSection;
}

/** 🗑️ Mirrors Rust `DeleteSection` (`🗑️📐️delete-section/🦀️.rs`). */
export interface DeleteSection {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceSection` (`🔁📐️replace-section/🦀️.rs`). */
export interface ReplaceSection {
  id: string;
  newSection: FemSection;
}

/** 🔒️ Mirrors Rust `FemSupport` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemSupport {
  id: string;
  nodeId: string;
  fixed: FemDof[];
}

/** 🌱️ Mirrors Rust `CreateSupport` (`🌱🛡️create-support/🦀️.rs`). */
export interface CreateSupport {
  support: FemSupport;
}

/** 🗑️ Mirrors Rust `DeleteSupport` (`🗑️delete-support/🦀️.rs`). */
export interface DeleteSupport {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceSupport` (`🔁🛡️replace-support/🦀️.rs`). */
export interface ReplaceSupport {
  id: string;
  newSupport: FemSupport;
}

/** 🧱️ Mirrors Rust `FemSolid` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
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

/** 🌱️ Mirrors Rust `CreateSolid` (`🌱🧊️create-solid/🦀️.rs`). */
export interface CreateSolid {
  solid: FemSolid;
}

/** 🗑️ Mirrors Rust `DeleteSolid` (`🗑️🧊️delete-solid/🦀️.rs`). */
export interface DeleteSolid {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceSolid` (`🔁replace-solid/🦀️.rs`). */
export interface ReplaceSolid {
  id: string;
  newSolid: FemSolid;
}

/** 🏋️ Mirrors Rust `FemLoad` (`🗿️artifacts/🧊️3d/🦀️component.rs`), tagged on `kind`. */
export type FemLoad =
  | { kind: "nodal"; id: string; nodeId: string; dof: FemDof; value: number }
  | { kind: "memberUdl"; id: string; elementId: string; wx: number; wy: number; wz: number }
  | { kind: "area"; id: string; solidId: string; pressure: number };

/** 📦️ Mirrors Rust `FemLoadCase` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemLoadCase {
  id: string;
  name: string;
  loads: FemLoad[];
  selfWeight: boolean;
}

/** 🌱️ Mirrors Rust `CreateLoadCase` (`🌱📋️create-load-case/🦀️.rs`). */
export interface CreateLoadCase {
  loadCase: FemLoadCase;
}

/** 🗑️ Mirrors Rust `DeleteLoadCase` (`🗑️📋️delete-load-case/🦀️.rs`). */
export interface DeleteLoadCase {
  id: string;
}

/** ➕️ Mirrors Rust `AddLoad` (`➕add-load/🦀️.rs`). */
export interface AddLoad {
  caseId: string;
  load: FemLoad;
}

/** ➖️ Mirrors Rust `RemoveLoad` (`➖remove-load/🦀️.rs`). */
export interface RemoveLoad {
  caseId: string;
  loadId: string;
}

/** ⚖️ Mirrors Rust `ChangeLoadCaseSelfWeight` (`⚖change-load-case-self-weight/🦀️.rs`). */
export interface ChangeLoadCaseSelfWeight {
  caseId: string;
  newSelfWeight: boolean;
}

/** 📦️ Mirrors Rust `FemCombination` (`🗿️artifacts/🧊️3d/🦀️component.rs`). */
export interface FemCombination {
  id: string;
  name: string;
  terms: Record<string, number>;
}

/** 🌱️ Mirrors Rust `CreateCombination` (`🌱🔗️create-combination/🦀️.rs`). */
export interface CreateCombination {
  combination: FemCombination;
}

/** 🗑️ Mirrors Rust `DeleteCombination` (`🗑️🔗️delete-combination/🦀️.rs`). */
export interface DeleteCombination {
  id: string;
}

/** ⚙️ Mirrors Rust `FemAnalysisSettings` (`🗿️artifacts/🧊️3d/🦀️component.rs`, re-exported from
 * `fem2d::FemAnalysisSettings`). */
export interface FemAnalysisSettings {
  modalCount: number;
  bucklingCount: number;
  deformationScale: number;
}

/** 🎛️ Mirrors Rust `UpdateAnalysisSettings` (`🎛update-analysis-settings/🦀️.rs`). */
export interface UpdateAnalysisSettings {
  settings: FemAnalysisSettings;
}

/** 🧩️ One arm per `Fem3dMutation` variant, same declaration order as the Rust enum. */
export type Fem3dMutation =
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
  | ({ mutation: "createSolid" } & CreateSolid)
  | ({ mutation: "deleteSolid" } & DeleteSolid)
  | ({ mutation: "replaceSolid" } & ReplaceSolid)
  | ({ mutation: "createLoadCase" } & CreateLoadCase)
  | ({ mutation: "deleteLoadCase" } & DeleteLoadCase)
  | ({ mutation: "addLoad" } & AddLoad)
  | ({ mutation: "removeLoad" } & RemoveLoad)
  | ({ mutation: "changeLoadCaseSelfWeight" } & ChangeLoadCaseSelfWeight)
  | ({ mutation: "createCombination" } & CreateCombination)
  | ({ mutation: "deleteCombination" } & DeleteCombination)
  | ({ mutation: "updateAnalysisSettings" } & UpdateAnalysisSettings);
