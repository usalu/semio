/** 📐️ Cad direct-mutation discriminated union. */
import type { CadNode } from "../🟦️.ts";

/** 📎️ One reference overlay — mirrors `crate::artifacts::cad::CadReference`. */
export interface CadReference {
  id: string;
  sourceUrl: string;
  mediaKind: string;
  origin: [number, number, number];
  orientation: [number, number, number, number] | null;
  scale: number | null;
  widthWorld: number;
  hidden: boolean;
  locked: boolean;
  opacity: number | null;
}

/** 🧱️ `create-shape-model` payload — sets the `shape_model` child slot; overwrites if occupied. */
export interface CreateShapeModel {
  childId: string;
  target: string;
}

/** 🧨️ `delete-shape-model` payload — clears the `shape_model` child slot. */
export type DeleteShapeModel = Record<string, never>;

/** 🏢️ `create-building-model` payload — sets the `building_model` child slot; overwrites if occupied. */
export interface CreateBuildingModel {
  childId: string;
  target: string;
}

/** 💥️ `delete-building-model` payload — clears the `building_model` child slot. */
export type DeleteBuildingModel = Record<string, never>;

/** ⚡️ `create-energy-model` payload — sets the `energy_model` child slot; overwrites if occupied. */
export interface CreateEnergyModel {
  childId: string;
  target: string;
}

/** 🔌️ `delete-energy-model` payload — clears the `energy_model` child slot. */
export type DeleteEnergyModel = Record<string, never>;

/** 🏛️ `create-structure-classic-model` payload — sets the `structure_classic_model` child slot; overwrites if occupied. */
export interface CreateStructureClassicModel {
  childId: string;
  target: string;
}

/** 💣️ `delete-structure-classic-model` payload — clears the `structure_classic_model` child slot. */
export type DeleteStructureClassicModel = Record<string, never>;

/** 📐️ `create-drawing` payload — appends a new owned drawing child handle. */
export interface CreateDrawing {
  childId: string;
  target: string;
}

/** 🧹️ `delete-drawing` payload — removes the entry matching `childId` from `drawings`. */
export interface DeleteDrawing {
  childId: string;
}

/** ➕️ `create-node` payload — brings a new node into existence in the scene graph tree. */
export interface CreateNode {
  node: CadNode;
}

/** 🗑️ `delete-node` payload — removes an existing node from the scene graph tree. */
export interface DeleteNode {
  nodeId: string;
}

/** 🏷️ `rename-node` payload — renames an existing node's `label`. */
export interface RenameNode {
  nodeId: string;
  newLabel: string;
}

/** 👁️ `change-reference-hidden` payload — changes one reference overlay's `hidden` field. */
export interface ChangeReferenceHidden {
  modelDefinitionId: string;
  referenceId: string;
  newHidden: boolean;
}

/** 🔒️ `change-reference-locked` payload — changes one reference overlay's `locked` field. */
export interface ChangeReferenceLocked {
  modelDefinitionId: string;
  referenceId: string;
  newLocked: boolean;
}

/** 📏️ `change-reference-width` payload — changes one reference overlay's `widthWorld` field. */
export interface ChangeReferenceWidth {
  modelDefinitionId: string;
  referenceId: string;
  newWidthWorld: number;
}

/** 📍️ `move-reference` payload — moves one reference overlay's `origin` field. */
export interface MoveReference {
  modelDefinitionId: string;
  referenceId: string;
  newOrigin: [number, number, number];
}

/** 🖇️ `replace-reference-media` payload — whole-value swap of a reference's media-identity/appearance bundle. */
export interface ReplaceReferenceMedia {
  modelDefinitionId: string;
  referenceId: string;
  newSourceUrl: string;
  newMediaKind: string;
  newOrientation: [number, number, number, number] | null;
  newScale: number | null;
  newOpacity: number | null;
}

/** 📎️ `replace-references` payload — whole-value swap of one model definition's entire reference-overlay list. */
export interface ReplaceReferences {
  modelDefinitionId: string;
  references: CadReference[];
}

/** 🧱️ Public wire twin of the ephemeral working object `create-object` carries. */
export interface CadObjectSpec {
  id: string;
  label: string;
  typology: string;
  visible: boolean;
  locked: boolean;
  origin: [number, number, number];
  orientation: [number, number, number, number] | null;
  scale: [number, number, number] | null;
  meshUrl: string | null;
  extent: [number, number, number] | null;
  solidHandle: string | null;
}

/** 🧱️ One object's exact next origin — absolute, so the inverse restores it bit-for-bit. */
export interface CadObjectOrigin {
  objectId: string;
  newOrigin: [number, number, number];
}

/** 🧱️ One object's exact next orientation quaternion. */
export interface CadObjectOrientation {
  objectId: string;
  newOrientation: [number, number, number, number];
}

/** 🧱️ One object's exact next per-axis scale. */
export interface CadObjectScale {
  objectId: string;
  newScale: [number, number, number];
}

/** 🪆️ The pane whose composed `s.stdio.semio.model` child an object mutation re-materializes. */
export type CadPaneId = "shape" | "building" | "energy" | "structure-classic";

/** 🧱️ One authored primitive slot of an object. */
export interface CadObjectPrimitive {
  slot: string;
  primitiveId: string;
  kind: string;
}

/** 🆕️ `create-object` payload — brings one object into a pane's composed model child at `index`. */
export interface CreateObject {
  pane: CadPaneId;
  index: number;
  object: CadObjectSpec;
  primitives: CadObjectPrimitive[];
}

/** ❌️ `delete-object` payload — removes one object from a pane's composed model child. */
export interface DeleteObject {
  pane: CadPaneId;
  objectId: string;
}

/** 🚚️ `move-objects` payload — absolute next origins for the touched objects of one pane. */
export interface MoveObjects {
  pane: CadPaneId;
  placements: CadObjectOrigin[];
}

/** 🌀️ `rotate-objects` payload — absolute next orientations for the touched objects of one pane. */
export interface RotateObjects {
  pane: CadPaneId;
  placements: CadObjectOrientation[];
}

/** ⚖️ `scale-objects` payload — absolute next scales for the touched objects of one pane. */
export interface ScaleObjects {
  pane: CadPaneId;
  placements: CadObjectScale[];
}

export type CadMutation =
  | ({ mutation: "createShapeModel" } & CreateShapeModel)
  | ({ mutation: "deleteShapeModel" } & DeleteShapeModel)
  | ({ mutation: "createBuildingModel" } & CreateBuildingModel)
  | ({ mutation: "deleteBuildingModel" } & DeleteBuildingModel)
  | ({ mutation: "createEnergyModel" } & CreateEnergyModel)
  | ({ mutation: "deleteEnergyModel" } & DeleteEnergyModel)
  | ({ mutation: "createStructureClassicModel" } & CreateStructureClassicModel)
  | ({ mutation: "deleteStructureClassicModel" } & DeleteStructureClassicModel)
  | ({ mutation: "createDrawing" } & CreateDrawing)
  | ({ mutation: "deleteDrawing" } & DeleteDrawing)
  | ({ mutation: "createNode" } & CreateNode)
  | ({ mutation: "deleteNode" } & DeleteNode)
  | ({ mutation: "renameNode" } & RenameNode)
  | ({ mutation: "changeReferenceHidden" } & ChangeReferenceHidden)
  | ({ mutation: "changeReferenceLocked" } & ChangeReferenceLocked)
  | ({ mutation: "changeReferenceWidth" } & ChangeReferenceWidth)
  | ({ mutation: "moveReference" } & MoveReference)
  | ({ mutation: "replaceReferenceMedia" } & ReplaceReferenceMedia)
  | ({ mutation: "replaceReferences" } & ReplaceReferences)
  | ({ mutation: "createObject" } & CreateObject)
  | ({ mutation: "deleteObject" } & DeleteObject)
  | ({ mutation: "moveObjects" } & MoveObjects)
  | ({ mutation: "rotateObjects" } & RotateObjects)
  | ({ mutation: "scaleObjects" } & ScaleObjects);
