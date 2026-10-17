/** 📨️ CAD editor artifact: the model document, the interaction messages and responses, the document history, the interaction snapshot and the runtime options. */
import { SolidRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { EMPTY_MODEL_DIFF, ModelDiff, SpatialKernel, SpatialPreviewKernel, applyModelDiff, isEmptyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { DisplayModel } from "../../🎬️actions/🟦️.ts";
import { ActionRegistry } from "../../🎬️actions/🧬️action-contract/🟦️.ts";
import { ConstructRunner } from "../../🎬️actions/🏃️action-execution/🟦️.ts";
import { InteractionSpatialResolved, StateEngineProvider } from "../../🎬️actions/🎰️interaction-state/🟦️.ts";
import type { InteractionRegistry } from "../📚️interaction-catalog/🟦️.ts";

// #region 📦️📄️document
// #region 📄️Document
/** 📄️ Single committed modeling operation node. */
export interface ShapeNode {
  readonly id: string;
  readonly operationKind: string;
  readonly solidRef?: SolidRef;
}

/** 📄️ Working document: model + committed shape nodes + command stack. */
export interface ModelDocument {
  readonly model: Model;
  nodes: ShapeNode[];
}

// #endregion 📄️Document
// #region 📨️Response
/** 📨️ Portable command outcome envelope (`diff` + `data` + messages). */
export interface InteractionMessage {
  readonly code: string;
  readonly message: string;
  readonly path?: string;
}

/** 📨️ Result returned by `InteractionRuntime.commit` — modeling output is always `diff` (model geometry); `data` is auxiliary. */
export interface InteractionResponse<TData = unknown> {
  readonly ok: boolean;
  readonly errors: readonly InteractionMessage[];
  readonly warnings: readonly InteractionMessage[];
  readonly infos: readonly InteractionMessage[];
  readonly diff: ModelDiff;
  readonly data: TData | null;
  /** 📦️ Context clone immediately before the post-commit `confirm` transition; null when commit aborted before confirm. */
  readonly archiveContext: Record<string, unknown> | null;
}

/** 📨️ Default empty success payload for guards and early returns. */
export const EMPTY_INTERACTION_RESPONSE: InteractionResponse<null> = {
  ok: true,
  errors: [],
  warnings: [],
  infos: [],
  diff: EMPTY_MODEL_DIFF,
  data: null,
  archiveContext: null,
};

/** 📄️ One committed model change plus inverse diff for document-level undo/redo. */
export interface Modification {
  readonly id: string;
  readonly interactionId: string;
  readonly label: string;
  readonly result: InteractionResponse;
  readonly backwardsDiff: ModelDiff;
}

/** 📄️ Two-stack modification history (undo / redo) keyed by model diffs. */
export class DocumentHistory {
  private undoStack: Modification[] = [];
  private redoStack: Modification[] = [];

  record(mod: Modification): void {
    if (isEmptyModelDiff(mod.result.diff)) return;
    this.undoStack.push(mod);
    this.redoStack = [];
  }

  peekUndo(): Modification | null {
    const n = this.undoStack.length;
    return n ? this.undoStack[n - 1]! : null;
  }

  peekRedo(): Modification | null {
    const n = this.redoStack.length;
    return n ? this.redoStack[n - 1]! : null;
  }

  /** 📚️ Committed undo stack in document order for renderer views. */
  entries(): readonly Modification[] {
    return [...this.undoStack];
  }

  /** 🧹️ Drops undo and redo stacks when the host swaps the base document. */
  clear(): void {
    this.undoStack = [];
    this.redoStack = [];
  }

  undo(doc: ModelDocument): Modification | null {
    const mod = this.undoStack.pop();
    if (!mod) return null;
    applyModelDiff(doc.model, mod.backwardsDiff);
    this.redoStack.push(mod);
    return mod;
  }

  redo(doc: ModelDocument): Modification | null {
    const mod = this.redoStack.pop();
    if (!mod) return null;
    applyModelDiff(doc.model, mod.result.diff);
    this.undoStack.push(mod);
    return mod;
  }
}

// #endregion 📨️Response
// #region 📜️Interaction
/** 🩺️ Non-fatal runtime diagnostic surfaced in snapshots. */
export interface Diagnostic {
  readonly severity: "info" | "warning" | "error";
  readonly code: string;
  readonly message: string;
}

/** 📜️ Host frame while a nested interaction session is active (chain via `outer`). */
export interface InteractionNestedHostFrame {
  readonly hostInteractionId: string;
  readonly hostState: string;
  readonly hostContext: Record<string, unknown>;
  readonly outer?: InteractionNestedHostFrame;
}

/** 📜️ Serializable interaction snapshot for hosts and renderers. */
export interface InteractionSnapshot {
  readonly interactionId: string;
  readonly state: string;
  readonly revision: number;
  readonly context: Record<string, unknown>;
  readonly display: DisplayModel;
  readonly spatialInteraction: InteractionSpatialResolved;
  readonly capabilities: { readonly canCommit: boolean; readonly canCancel: boolean; readonly canUndo: boolean; readonly canRedo: boolean };
  readonly diagnostics: readonly Diagnostic[];
  readonly lastResponse: InteractionResponse | null;
  readonly nested?: InteractionNestedHostFrame;
}

export type SpatialComputeMode = "fast" | "precise";

export interface InteractionRuntimeOptions {
  readonly kernel: SpatialKernel;
  readonly previewKernel?: SpatialPreviewKernel;
  readonly mode?: SpatialComputeMode;
  readonly document: ModelDocument;
  readonly history?: DocumentHistory;
  readonly stateEngine?: StateEngineProvider;
  readonly actions?: ActionRegistry;
  readonly interactions?: InteractionRegistry;
  readonly query?: ConstructRunner;
  readonly activeModelDefinitionId?: string | null;
}
// #endregion 📜️Interaction
// #endregion 📦️📄️document
