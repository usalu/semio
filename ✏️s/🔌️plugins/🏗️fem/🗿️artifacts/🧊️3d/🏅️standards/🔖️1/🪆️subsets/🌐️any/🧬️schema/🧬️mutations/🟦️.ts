import type { FemNode, FemDof, FemAxis, FemElement, FemMaterial, FemSection, FemSupport, FemLoad, FemLoadCase, FemSolid, FemCombination, FemAnalysisSettings } from "../📸️snapshot/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type {FemNode,FemDof,FemAxis,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemSolid,FemCombination,FemAnalysisSettings} from "../📸️snapshot/🟦️.ts";
/** 🧩️ Direct mutations reuse the canonical persisted Snapshot entity types. */

/** 🌱️ Mirrors Rust `CreateNode` (`⚪️create-node/🦀️.rs`). */
export interface CreateNode {
  node: FemNode;
}

/** 🗑️ Mirrors Rust `DeleteNode` (`🗑️⚪️delete-node/🦀️.rs`). */
export interface DeleteNode {
  id: string;
}

/** 🌱️ Mirrors Rust `CreateElement` (`🧩️create-element/🦀️.rs`). */
export interface CreateElement {
  element: FemElement;
}

/** 🗑️ Mirrors Rust `DeleteElement` (`🗑️🧩️delete-element/🦀️.rs`). */
export interface DeleteElement {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceElement` (`♻️replace-element/🦀️.rs`). */
export interface ReplaceElement {
  id: string;
  newElement: FemElement;
}

/** 🌱️ Mirrors Rust `CreateMaterial` (`🌱️create-material/🦀️.rs`). */
export interface CreateMaterial {
  material: FemMaterial;
}

/** 🗑️ Mirrors Rust `DeleteMaterial` (`🗑️🧱️delete-material/🦀️.rs`). */
export interface DeleteMaterial {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceMaterial` (`🔁️replace-material/🦀️.rs`). */
export interface ReplaceMaterial {
  id: string;
  newMaterial: FemMaterial;
}

/** 🌱️ Mirrors Rust `CreateSection` (`📐️create-section/🦀️.rs`). */
export interface CreateSection {
  section: FemSection;
}

/** 🗑️ Mirrors Rust `DeleteSection` (`🗑️📐️delete-section/🦀️.rs`). */
export interface DeleteSection {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceSection` (`📏️replace-section/🦀️.rs`). */
export interface ReplaceSection {
  id: string;
  newSection: FemSection;
}

/** 🌱️ Mirrors Rust `CreateSupport` (`🛡️create-support/🦀️.rs`). */
export interface CreateSupport {
  support: FemSupport;
}

/** 🗑️ Mirrors Rust `DeleteSupport` (`🗑️delete-support/🦀️.rs`). */
export interface DeleteSupport {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceSupport` (`🔁️replace-support/🦀️.rs`). */
export interface ReplaceSupport {
  id: string;
  newSupport: FemSupport;
}

/** 🌱️ Mirrors Rust `CreateSolid` (`🧊️create-solid/🦀️.rs`). */
export interface CreateSolid {
  solid: FemSolid;
}

/** 🗑️ Mirrors Rust `DeleteSolid` (`🗑️🧊️delete-solid/🦀️.rs`). */
export interface DeleteSolid {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceSolid` (`🔄️replace-solid/🦀️.rs`). */
export interface ReplaceSolid {
  id: string;
  newSolid: FemSolid;
}

/** 🌱️ Mirrors Rust `CreateLoadCase` (`📋️create-load-case/🦀️.rs`). */
export interface CreateLoadCase {
  loadCase: FemLoadCase;
}

/** 🗑️ Mirrors Rust `DeleteLoadCase` (`🗑️📋️delete-load-case/🦀️.rs`). */
export interface DeleteLoadCase {
  id: string;
}

/** ➕️ Mirrors Rust `AddLoad` (`➕️add-load/🦀️.rs`). */
export interface AddLoad {
  caseId: string;
  load: FemLoad;
}

/** ➖️ Mirrors Rust `RemoveLoad` (`➖️remove-load/🦀️.rs`). */
export interface RemoveLoad {
  caseId: string;
  loadId: string;
}

/** ⚖️ Mirrors Rust `ChangeLoadCaseSelfWeight` (`⚖️change-load-case-self-weight/🦀️.rs`). */
export interface ChangeLoadCaseSelfWeight {
  caseId: string;
  newSelfWeight: boolean;
}

/** 🌱️ Mirrors Rust `CreateCombination` (`🔗️create-combination/🦀️.rs`). */
export interface CreateCombination {
  combination: FemCombination;
}

/** 🗑️ Mirrors Rust `DeleteCombination` (`🗑️🔗️delete-combination/🦀️.rs`). */
export interface DeleteCombination {
  id: string;
}

/** 🔁️ Mirrors Rust `ReplaceNode` (`🔁️replace-node/🦀️.rs`). */
export interface ReplaceNode {
  id: string;
  newNode: FemNode;
}

/** 🔁️ Mirrors Rust `ReplaceLoad` (`🔁️replace-load/🦀️.rs`). */
export interface ReplaceLoad {
  caseId: string;
  loadId: string;
  newLoad: FemLoad;
}

/** 🏷️ Mirrors Rust `ChangeLoadCaseName` (`🏷️change-load-case-name/🦀️.rs`). */
export interface ChangeLoadCaseName {
  caseId: string;
  newName: string;
}

/** 🔁️ Mirrors Rust `ReplaceCombination` (`🔁️replace-combination/🦀️.rs`). */
export interface ReplaceCombination {
  id: string;
  newCombination: FemCombination;
}

/** 🧭️ Mirrors Rust `MoveSelection` (`🧭️move-selection/🦀️.rs`). */
export interface MoveSelection {
  nodeIds: string[];
  solidIds: string[];
  pivotX: Binary64;
  pivotY: Binary64;
  pivotZ: Binary64;
  dx: Binary64;
  dy: Binary64;
  dz: Binary64;
  axisX: Binary64;
  axisY: Binary64;
  axisZ: Binary64;
  angle: Binary64;
  sx: Binary64;
  sy: Binary64;
  sz: Binary64;
}

/** 🎛️ Mirrors Rust `UpdateAnalysisSettings` (`🎛️update-analysis-settings/🦀️.rs`). */
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
  | ({ mutation: "updateAnalysisSettings" } & UpdateAnalysisSettings)
  | ({ mutation: "replaceNode" } & ReplaceNode)
  | ({ mutation: "replaceLoad" } & ReplaceLoad)
  | ({ mutation: "changeLoadCaseName" } & ChangeLoadCaseName)
  | ({ mutation: "replaceCombination" } & ReplaceCombination)
  | ({ mutation: "moveSelection" } & MoveSelection);
