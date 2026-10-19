import type {Binary64} from "@semio-tech/framework/value/ieee754";
import type { Fem2dLoadPatch } from "../🔺️diff/🟦️.ts";
import type { Fem2dElementPatch } from "../🔺️diff/🟦️.ts";
import type { Fem2dSupportPatch } from "../🔺️diff/🟦️.ts";
import type { Fem2dRegionPatch } from "../🔺️diff/🟦️.ts";
import type { Fem2dSectionPatch } from "../🔺️diff/🟦️.ts";
import type { Fem2dNodePatch } from "../🔺️diff/🟦️.ts";
import type { Fem2dMaterialPatch } from "../🔺️diff/🟦️.ts";
/** ⚡️ Fem2d direct-mutation discriminated union — TS mirror of the Rust `Fem2dMutation` dispatch enum. */

import type {FemNode,FemDof,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemRegion,FemCombinationTerm,FemCombination,FemAnalysisSettings} from "../📸️snapshot/🟦️.ts";
export type {FemNode,FemDof,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemRegion,FemCombinationTerm,FemCombination,FemAnalysisSettings} from "../📸️snapshot/🟦️.ts";

/** 🩹️ Changes supplied node fields without replacing concurrent fields. */
export interface ChangeNode { id: string; patch: Fem2dNodePatch; }
/** 🩹️ Changes supplied material fields without replacing concurrent fields. */
export interface ChangeMaterial { id: string; patch: Fem2dMaterialPatch; }
/** 🩹️ Changes supplied section fields without replacing concurrent fields. */
export interface ChangeSection { id: string; patch: Fem2dSectionPatch; }
/** 🩹️ Changes only supplied support fields. */
export interface ChangeSupport { id: string; patch: Fem2dSupportPatch; }
/** 🩹️ Changes only supplied region fields. */
export interface ChangeRegion { id: string; patch: Fem2dRegionPatch; }
/** 🩹️ Changes only supplied element fields. */
export interface ChangeElement { id: string; patch: Fem2dElementPatch; }
/** 🩹️ Changes only supplied fields of a load with the addressed current kind. */
export interface ChangeLoad { caseId: string; id: string; patch: Fem2dLoadPatch; }
/** 🏷️ Owns one combination name. */
export interface ChangeCombinationName { id: string; name: string; }
/** ⚖️ Owns one keyed factor; null removes the term. */
export interface ChangeCombinationTerm { id: string; caseId: string; factor: Binary64 | null; index?: number; }

//#region 🔖️Mutations
/** 🌱⚪️ Brings a new structural node into existence. */
export interface CreateNode {
  node: FemNode;
  index?: number | null;
}

/** 🗑⚪️ Removes an existing structural node by id. */
export interface DeleteNode {
  id: string;
}

/** 🌱🧩️ Brings a new structural member (bar/beam) into existence. */
export interface CreateElement {
  element: FemElement;
  index?: number | null;
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
  index?: number | null;
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
  index?: number | null;
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
  index?: number | null;
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
  index?: number | null;
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
  index?: number | null;
}

/** 🗑📋️ Removes an existing load case by id. */
export interface DeleteLoadCase {
  id: string;
}

/** ➕️ Attaches a load to an existing load case's `loads` member collection. */
export interface AddLoad {
  caseId: string;
  load: FemLoad;
  index?: number | null;
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
  index?: number | null;
}

/** 🗑🔗️ Removes an existing load combination by id. */
export interface DeleteCombination {
  id: string;
}

/** 🎛️ Atomically updates the document's inseparable analysis-settings facet. */
export interface UpdateAnalysisSettings {
  settings: FemAnalysisSettings;
}

/** 🔁📍️ Whole-value swap of an existing node's payload — the one gesture that moves a node. */
export interface ReplaceNode {
  id: string;
  newNode: FemNode;
}

/** 🔁🏋️ Whole-value swap of one load inside an existing load case's `loads` member collection. */
export interface ReplaceLoad {
  caseId: string;
  loadId: string;
  newLoad: FemLoad;
}

/** 🏷️ Sets an existing load case's human-readable name; its `id` is untouched. */
export interface ChangeLoadCaseName {
  caseId: string;
  newName: string;
}

/** 🔁🔗️ Whole-value swap of an existing load combination's payload. */
export interface ReplaceCombination {
  id: string;
  newCombination: FemCombination;
}
/** 🧭️ One relative gumball transform of named nodes and regions: scale and rotate about the pivot, then offset. */
export interface MoveSelection {
  nodeIds: string[];
  regionIds: string[];
  pivotX: Binary64;
  pivotY: Binary64;
  dx: Binary64;
  dy: Binary64;
  angle: Binary64;
  sx: Binary64;
  sy: Binary64;
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
  | ({ mutation: "changeMaterial" } & ChangeMaterial)
  | ({ mutation: "createSection" } & CreateSection)
  | ({ mutation: "deleteSection" } & DeleteSection)
  | ({ mutation: "replaceSection" } & ReplaceSection)
  | ({ mutation: "changeSection" } & ChangeSection)
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
  | ({ mutation: "updateAnalysisSettings" } & UpdateAnalysisSettings)
  | ({ mutation: "replaceNode" } & ReplaceNode)
  | ({ mutation: "changeNode" } & ChangeNode)
  | ({ mutation: "replaceLoad" } & ReplaceLoad)
  | ({ mutation: "changeLoadCaseName" } & ChangeLoadCaseName)
  | ({ mutation: "replaceCombination" } & ReplaceCombination)
  | ({ mutation: "moveSelection" } & MoveSelection)
  | ({ mutation: "changeRegion" } & ChangeRegion)
  | ({ mutation: "changeSupport" } & ChangeSupport)
  | ({ mutation: "changeElement" } & ChangeElement)
  | ({ mutation: "changeLoad" } & ChangeLoad)
  | ({ mutation: "changeCombinationName" } & ChangeCombinationName)
  | ({ mutation: "changeCombinationTerm" } & ChangeCombinationTerm);
