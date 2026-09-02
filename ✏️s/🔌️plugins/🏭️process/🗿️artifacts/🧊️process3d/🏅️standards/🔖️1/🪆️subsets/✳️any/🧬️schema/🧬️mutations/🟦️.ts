/** 🏭️ Process3d direct-mutation discriminated union. */
import type { ArtifactChildHandle, Process3dCapability, Process3dPose, Process3dStep, Process3dStepOrigin, Process3dWorkshopMachine } from "../🟦️.ts";

/** 📋️ `create-step` payload — full initial payload for a new step appended to the timeline. */
export interface CreateStep {
  index: number;
  step: Process3dStep;
}

/** 🗑️ `delete-step` payload — the step's id. */
export interface DeleteStep {
  id: string;
}

/** 🏷️ `rename-step` payload — the step's new `label`. */
export interface RenameStep {
  id: string;
  newLabel: string;
}

/** 🔘️ `change-step-enabled` payload — the step's new `enabled` flag. */
export interface ChangeStepEnabled {
  id: string;
  newEnabled: boolean;
}

/** 🧷️ `change-step-origin` payload — the step's new (or cleared) provenance. */
export interface ChangeStepOrigin {
  id: string;
  newOrigin: Process3dStepOrigin | null;
}

/** 📐️ `replace-step-measure` payload — whole-value swap of a step's tool/pose geometry. */
export interface ReplaceStepMeasure {
  id: string;
  newMeasure: Record<string, unknown>;
}

/** 🔀️ `reorder-steps` payload — `toIndex` is final-state, clamped to the list length. */
export interface ReorderSteps {
  id: string;
  toIndex: number;
}

/** 🏭️ `create-machine` payload — full initial payload for a new workshop machine. */
export interface CreateMachine {
  index: number;
  machine: Process3dWorkshopMachine;
}

/** ❌️ `delete-machine` payload — the machine's id. */
export interface DeleteMachine {
  id: string;
}

/** 🔖️ `rename-machine` payload — the machine's new `label`. */
export interface RenameMachine {
  id: string;
  newLabel: string;
}

/** 🎨️ `change-machine-icon` payload — the machine's new `iconId`. */
export interface ChangeMachineIcon {
  id: string;
  newIconId: string;
}

/** 🔁️ `replace-machine-capabilities` payload — whole-value swap of a machine's capabilities list. */
export interface ReplaceMachineCapabilities {
  id: string;
  newCapabilities: Process3dCapability[];
}

/** 📍️ `move-stock` payload — absolute spatial reposition of the document's single stock workpiece. */
export interface MoveStock {
  newPose: Process3dPose;
}

/** 🔤️ `change-stock-label` payload — the stock's new `label`. */
export interface ChangeStockLabel {
  newLabel: string;
}

/** 🧊️ `replace-stock-solid` payload — pure handle swap of the composed brep stock-solid child. */
export interface ReplaceStockSolid {
  newSolid: ArtifactChildHandle;
}

/** ⏱️ `change-cursor` payload — the document-level "resolved up to" playback cursor. */
export interface ChangeCursor {
  newResolvedUpTo: number | null;
}

export type Process3dMutation =
  | ({ mutation: "createStep" } & CreateStep)
  | ({ mutation: "deleteStep" } & DeleteStep)
  | ({ mutation: "renameStep" } & RenameStep)
  | ({ mutation: "changeStepEnabled" } & ChangeStepEnabled)
  | ({ mutation: "changeStepOrigin" } & ChangeStepOrigin)
  | ({ mutation: "replaceStepMeasure" } & ReplaceStepMeasure)
  | ({ mutation: "reorderSteps" } & ReorderSteps)
  | ({ mutation: "createMachine" } & CreateMachine)
  | ({ mutation: "deleteMachine" } & DeleteMachine)
  | ({ mutation: "renameMachine" } & RenameMachine)
  | ({ mutation: "changeMachineIcon" } & ChangeMachineIcon)
  | ({ mutation: "replaceMachineCapabilities" } & ReplaceMachineCapabilities)
  | ({ mutation: "moveStock" } & MoveStock)
  | ({ mutation: "changeStockLabel" } & ChangeStockLabel)
  | ({ mutation: "replaceStockSolid" } & ReplaceStockSolid)
  | ({ mutation: "changeCursor" } & ChangeCursor);
