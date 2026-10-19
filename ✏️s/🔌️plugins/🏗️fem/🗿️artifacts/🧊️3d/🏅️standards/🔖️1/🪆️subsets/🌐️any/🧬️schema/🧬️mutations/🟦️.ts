import type { Fem3dLoadPatch } from "../🔺️diff/🟦️.ts";
export type { Fem3dLoadPatch } from "../🔺️diff/🟦️.ts";
/** 🩹️ Changes only supplied fields of a load with the addressed current kind. */
export interface ChangeLoad { caseId: string; id: string; patch: Fem3dLoadPatch; }
/** 🏷️ Owns one combination name. */
export interface ChangeCombinationName { id: string; name: string; }
/** ⚖️ Owns one keyed factor; null removes the term. */
export interface ChangeCombinationTerm { id: string; caseId: string; factor: Binary64 | null; }

import type { Fem3dElementPatch } from "../🔺️diff/🟦️.ts";
export type { Fem3dElementPatch } from "../🔺️diff/🟦️.ts";

/** 🩹️ Changes only supplied element fields. */
export interface ChangeElement {
  id: string;
  patch: Fem3dElementPatch;
}

import type { Fem3dSupportPatch } from "../🔺️diff/🟦️.ts";
export type { Fem3dSupportPatch } from "../🔺️diff/🟦️.ts";

/** 🩹️ Changes only supplied support fields. */
export interface ChangeSupport {
  id: string;
  patch: Fem3dSupportPatch;
}

import type { Fem3dSolidPatch } from "../🔺️diff/🟦️.ts";
export type { Fem3dSolidPatch } from "../🔺️diff/🟦️.ts";

/** 🩹️ Changes only supplied solid fields. */
export interface ChangeSolid {
  id: string;
  patch: Fem3dSolidPatch;
}

import type { Fem3dSectionPatch } from "../🔺️diff/🟦️.ts";
export type { Fem3dSectionPatch } from "../🔺️diff/🟦️.ts";

/** 🩹️ Changes supplied section fields without replacing concurrent fields. */
export interface ChangeSection {
  id: string;
  patch: Fem3dSectionPatch;
}

import type { Fem3dNodePatch } from "../🔺️diff/🟦️.ts";
export type { Fem3dNodePatch } from "../🔺️diff/🟦️.ts";

/** 🩹️ Changes supplied node fields without replacing concurrent fields. */
export interface ChangeNode {
  id: string;
  patch: Fem3dNodePatch;
}

import type { Fem3dMaterialPatch } from "../🔺️diff/🟦️.ts";
export type { Fem3dMaterialPatch } from "../🔺️diff/🟦️.ts";

/** 🩹️ Changes supplied material fields without replacing concurrent fields. */
export interface ChangeMaterial {
  id: string;
  patch: Fem3dMaterialPatch;
}

import type { FemNode, FemDof, FemAxis, FemElement, FemMaterial, FemSection, FemSupport, FemLoad, FemLoadCase, FemSolid, FemCombination, FemAnalysisSettings } from "../📸️snapshot/🟦️.ts";
import type {Binary64} from "@semio-tech/framework/value/ieee754";
export type {FemNode,FemDof,FemAxis,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemSolid,FemCombination,FemAnalysisSettings} from "../📸️snapshot/🟦️.ts";
/** 🧩️ Direct mutations reuse the canonical persisted Snapshot entity types. */

/** 🌱️ Mirrors Rust `CreateNode` (`⚪️create-node/🦀️.rs`). */
export interface CreateNode {
  node: FemNode;
  index?: number | null;
}

/** 🗑️ Mirrors Rust `DeleteNode` (`🗑️⚪️delete-node/🦀️.rs`). */
export interface DeleteNode {
  id: string;
}

/** 🌱️ Mirrors Rust `CreateElement` (`🧩️create-element/🦀️.rs`). */
export interface CreateElement {
  element: FemElement;
  index?: number | null;
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
  index?: number | null;
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
  index?: number | null;
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
  index?: number | null;
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
  index?: number | null;
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
  index?: number | null;
}

/** 🗑️ Mirrors Rust `DeleteLoadCase` (`🗑️📋️delete-load-case/🦀️.rs`). */
export interface DeleteLoadCase {
  id: string;
}

/** ➕️ Mirrors Rust `AddLoad` (`➕️add-load/🦀️.rs`). */
export interface AddLoad {
  caseId: string;
  load: FemLoad;
  index?: number | null;
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
  index?: number | null;
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
  | ({ mutation: "changeMaterial" } & ChangeMaterial)
  | ({ mutation: "createSection" } & CreateSection)
  | ({ mutation: "deleteSection" } & DeleteSection)
  | ({ mutation: "replaceSection" } & ReplaceSection)
  | ({ mutation: "changeSection" } & ChangeSection)
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
  | ({ mutation: "changeNode" } & ChangeNode)
  | ({ mutation: "replaceLoad" } & ReplaceLoad)
  | ({ mutation: "changeLoadCaseName" } & ChangeLoadCaseName)
  | ({ mutation: "replaceCombination" } & ReplaceCombination)
  | ({ mutation: "moveSelection" } & MoveSelection)
  | ({ mutation: "changeSolid" } & ChangeSolid)
  | ({ mutation: "changeSupport" } & ChangeSupport)
  | ({ mutation: "changeElement" } & ChangeElement)
  | ({ mutation: "changeLoad" } & ChangeLoad)
  | ({ mutation: "changeCombinationName" } & ChangeCombinationName)
  | ({ mutation: "changeCombinationTerm" } & ChangeCombinationTerm);
