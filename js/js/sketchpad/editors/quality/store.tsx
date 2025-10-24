// #region Header

// store.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import React, { createContext, useContext } from "react";
import * as Y from "yjs";
import { Guid, QualityDiff } from "../../../semio";
import { QualityStore } from "../../kits/store";
import {
  identitySelector,
  KitCommandContext,
  KitDiffEditorEdit,
  KitDiffEditorStore,
  KitStore,
  PanelVisibility,
  registerQualityEditorStoreFactory,
  SketchpadStore,
  ToolType,
  Transact,
  useKitScope,
  useQualityScope,
  useSketchpadStore,
  useSyncDeep,
  YAttributes,
  YLeafMapNumber,
  YLeafMapString,
  YStringArray,
} from "../../store";
import { commands as qualityEditorCommands } from "./commands";

type YQualityEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YQualityEditor = Y.Map<YQualityEditorVal>;
type YQualityEditors = Y.Map<YQualityEditor>;

export interface QualityEditorId {
  kit: Guid;
  quality: Guid;
}
export interface FormulaNode {
  id: Guid;
  type: "function" | "quality" | "variable" | "unit" | "value";
  name: string;
  children?: Guid[];
  x?: number;
  y?: number;
}
export interface QualityEditorSelection {
  formulaNodes?: Guid[];
}
export interface QualityEditorSelectionFormulaNodesDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface QualityEditorSelectionDiff {
  formulaNodes?: QualityEditorSelectionFormulaNodesDiff;
}
export enum QualityEditorFullscreenWindow {
  None = "none",
  Formula = "formula",
  Diagram = "diagram",
}
export interface QualityEditorHover {
  formulaNode?: Guid;
}
export interface QualityEditorDiff {
  selection?: QualityEditorSelectionDiff;
  hover?: QualityEditorHover;
  fullscreenWindow?: QualityEditorFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  activeTool?: ToolType;
  formulaNodes?: FormulaNode[];
}
export interface QualityEditorEdit extends KitDiffEditorEdit<QualityEditorSelectionDiff> {}
export interface QualityEditorState {
  fullscreenWindow: QualityEditorFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool: ToolType;
  selection?: QualityEditorSelection;
  hover?: QualityEditorHover;
  formulaNodes: FormulaNode[];
}

export interface QualityEditorCommandContext extends KitCommandContext {
  qualityEditor: QualityEditorState;
  Guid: Guid;
}
export interface QualityEditorCommandResult {
  diff?: QualityEditorDiff;
  qualityDiff?: QualityDiff;
}

function inverseQualityEditorSelectionDiff(selection: QualityEditorSelection, diff: QualityEditorSelectionDiff): QualityEditorSelectionDiff {
  const inverse: QualityEditorSelectionDiff = {};
  if (diff.formulaNodes) {
    inverse.formulaNodes = {
      added: diff.formulaNodes.removed ?? [],
      removed: diff.formulaNodes.added ?? [],
    };
  }
  return inverse;
}

class QualityEditorStore extends KitDiffEditorStore<QualityEditorState, QualityEditorDiff, QualityEditorSelectionDiff, QualityEditorEdit, QualityEditorCommandContext, QualityEditorCommandResult> {
  private readonly Guid: QualityEditorId;

  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: Transact, id: QualityEditorId) {
    super(parent, yMap, transact);
    this.Guid = id;

    transact(() => {
      if (!yMap.has("fullscreenWindow")) {
        yMap.set("fullscreenWindow", QualityEditorFullscreenWindow.None);
      }
      if (!yMap.has("activeTool")) {
        yMap.set("activeTool", ToolType.SELECTION_NORMAL);
      }
      if (!yMap.has("panelVisibility")) {
        const yPanelVisibility = new Y.Map<boolean>();
        yPanelVisibility.set("toolbar", false);
        yPanelVisibility.set("workbench", true);
        yPanelVisibility.set("details", true);
        yPanelVisibility.set("chat", false);
        yPanelVisibility.set("settings", false);
        yMap.set("panelVisibility", yPanelVisibility);
      }
      if (!yMap.has("formulaNodes")) {
        yMap.set("formulaNodes", new Y.Array<any>());
      }
    });

    Object.entries(qualityEditorCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  quality(): QualityStore | undefined {
    return this.parent.kit(this.Guid.kit).quality(this.Guid.quality);
  }

  kit(): KitStore {
    return this.parent.kit(this.Guid.kit);
  }

  get fullscreenWindow(): QualityEditorFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as QualityEditorFullscreenWindow;
  }

  get activeTool(): ToolType {
    const value = this.yMap.get("activeTool") as ToolType;
    if (value === undefined) {
      this.transact(() => {
        this.yMap.set("activeTool", ToolType.SELECTION_NORMAL);
      });
      return ToolType.SELECTION_NORMAL;
    }
    return value;
  }

  get panelVisibility(): PanelVisibility {
    const yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
    if (!yPanelVisibility) {
      return {
        toolbar: false,
        workbench: true,
        details: true,
        chat: false,
        settings: false,
      };
    }
    return {
      toolbar: yPanelVisibility.get("toolbar") ?? false,
      workbench: yPanelVisibility.get("workbench") ?? true,
      details: yPanelVisibility.get("details") ?? true,
      chat: yPanelVisibility.get("chat") ?? false,
      settings: yPanelVisibility.get("settings") ?? false,
    };
  }

  get selection(): QualityEditorSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};
    const result: QualityEditorSelection = {};
    const formulaNodes = selection.get("formulaNodes") as Y.Array<string>;
    if (formulaNodes && formulaNodes.length > 0) {
      result.formulaNodes = formulaNodes.toArray();
    }
    return result;
  }

  get hover(): QualityEditorHover | undefined {
    const hover = this.yMap.get("hover") as Y.Map<any> | undefined;
    if (!hover) return undefined;
    const result: QualityEditorHover = {};
    const formulaNode = hover.get("formulaNode") as Guid | undefined;
    if (formulaNode) result.formulaNode = formulaNode;
    return Object.keys(result).length > 0 ? result : undefined;
  }

  get formulaNodes(): FormulaNode[] {
    const yFormulaNodes = this.yMap.get("formulaNodes") as Y.Array<any>;
    if (!yFormulaNodes) return [];
    return yFormulaNodes.toArray().map((yNode: any) => ({
      id: yNode.get("id"),
      type: yNode.get("type"),
      name: yNode.get("name"),
      children: yNode.get("children") ? (yNode.get("children") as Y.Array<Guid>).toArray() : undefined,
      x: yNode.get("x"),
      y: yNode.get("y"),
    }));
  }

  protected getSelection(): QualityEditorSelection {
    return this.selection;
  }

  protected hash(state: QualityEditorState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): QualityEditorState {
    return {
      fullscreenWindow: this.fullscreenWindow,
      panelVisibility: this.panelVisibility,
      activeTool: this.activeTool,
      selection: this.selection,
      hover: this.hover,
      formulaNodes: this.formulaNodes,
    };
  }

  protected inverseSelectionDiff(selection: QualityEditorSelection, diff: QualityEditorSelectionDiff): QualityEditorSelectionDiff {
    return inverseQualityEditorSelectionDiff(selection, diff);
  }

  protected applySelectionDiff(selectionDiff: QualityEditorSelectionDiff): void {
    let selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) {
      selection = new Y.Map();
      this.yMap.set("selection", selection);
    }
    if (selectionDiff.formulaNodes) {
      let formulaNodes = (selection.get("formulaNodes") as Y.Array<Guid>) || new Y.Array<Guid>();
      if (!selection.has("formulaNodes")) {
        selection.set("formulaNodes", formulaNodes);
      }
      if (selectionDiff.formulaNodes.added) {
        for (const node of selectionDiff.formulaNodes.added) {
          if (!formulaNodes.toArray().includes(node)) {
            formulaNodes.push([node]);
          }
        }
      }
      if (selectionDiff.formulaNodes.removed) {
        for (const node of selectionDiff.formulaNodes.removed) {
          const index = formulaNodes.toArray().indexOf(node);
          if (index !== -1) {
            formulaNodes.delete(index, 1);
          }
        }
      }
    }
  }

  change = (diff: QualityEditorDiff) => {
    this.transact(() => {
      if (diff.fullscreenWindow) this.yMap.set("fullscreenWindow", diff.fullscreenWindow);
      if (diff.activeTool) this.yMap.set("activeTool", diff.activeTool);
      if (diff.panelVisibility !== undefined) {
        let yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
        if (!yPanelVisibility) {
          yPanelVisibility = new Y.Map<boolean>();
          this.yMap.set("panelVisibility", yPanelVisibility);
        }
        Object.entries(diff.panelVisibility).forEach(([key, value]) => {
          if (value !== undefined) {
            yPanelVisibility.set(key, value);
          }
        });
      }
      if (diff.selection) {
        this.applySelectionDiff(diff.selection);
      }
      if (diff.hover) {
        if (Object.keys(diff.hover).length === 0) {
          this.yMap.delete("hover");
        } else {
          let yHover = this.yMap.get("hover") as Y.Map<any>;
          if (!yHover) {
            yHover = new Y.Map<any>();
            this.yMap.set("hover", yHover);
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "formulaNode")) {
            const nodeValue = diff.hover.formulaNode;
            if (nodeValue) {
              yHover.set("formulaNode", nodeValue);
            } else {
              yHover.delete("formulaNode");
            }
          }
        }
      }
      if (diff.formulaNodes) {
        const yFormulaNodes = new Y.Array<any>();
        diff.formulaNodes.forEach((node) => {
          const yNode = new Y.Map<any>();
          yNode.set("id", node.id);
          yNode.set("type", node.type);
          yNode.set("name", node.name);
          if (node.children) {
            const yChildren = new Y.Array<Guid>();
            yChildren.push(node.children);
            yNode.set("children", yChildren);
          }
          if (node.x !== undefined) yNode.set("x", node.x);
          if (node.y !== undefined) yNode.set("y", node.y);
          yFormulaNodes.push([yNode]);
        });
        this.yMap.set("formulaNodes", yFormulaNodes);
      }
    });
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.qualityEditor.startTransaction") {
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.qualityEditor.finalizeTransaction") {
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.qualityEditor.abortTransaction") {
      this.abortTransaction();
      return {} as T;
    }
    if (command === "semio.qualityEditor.undo") {
      this.undo();
      return {} as T;
    }
    if (command === "semio.qualityEditor.redo") {
      this.redo();
      return {} as T;
    }

    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in quality editor store`);

    const kitStore = this.kit();
    const state = this.snapshot();
    const kitState = kitStore.snapshot();
    const quality = this.quality();

    const context: QualityEditorCommandContext = {
      qualityEditor: state,
      kit: kitState,
      Guid: quality?.guid || this.Guid.quality,
      fileUrls: kitStore.fileUrls,
    };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
    }
    if (result.qualityDiff) {
      kitStore.change({
        qualities: {
          updated: [{ id: this.Guid.quality, diff: result.qualityDiff }],
        },
      });
    }
    this.recordEdit(result);
    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }
}

registerQualityEditorStoreFactory((parent, yMap, transact, id, state) => new QualityEditorStore(parent, yMap, transact, id));

type QualityEditorScope = { guid: string };
const QualityEditorScopeContext = createContext<QualityEditorScope | null>(null);
export const QualityEditorScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(QualityEditorScopeContext.Provider, { value }, props.children as any);
};
const useQualityEditorScope = () => useContext(QualityEditorScopeContext);

export function useQualityEditorStore<T>(selector?: (store: QualityEditorStore) => T, id?: QualityEditorId): T | QualityEditorStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const qualityScope = useQualityScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  const resolvedQualityId = qualityScope?.guid ?? id?.quality;
  if (!resolvedKitId || !resolvedQualityId) {
    return null;
  }
  const qualityEditorStore = store.qualityEditor(resolvedKitId, resolvedQualityId);
  return selector ? selector(qualityEditorStore) : qualityEditorStore;
}

export function useQualityEditor<T>(selector?: (state: QualityEditorState) => T, id?: QualityEditorId): T | QualityEditorState | null {
  const store = useQualityEditorStore(identitySelector, id);
  return useSyncDeep<QualityEditorState, T>(store as QualityEditorStore, selector ? selector : identitySelector);
}

export function useQualityEditorCommands(id?: QualityEditorId) {
  const store = useQualityEditorStore(undefined, id) as QualityEditorStore | null;
  if (!store) {
    return {
      startTransaction: () => {},
      finalizeTransaction: () => {},
      abortTransaction: () => {},
      undo: () => {},
      redo: () => {},
      toggleFormulaFullscreen: () => Promise.resolve(),
      toggleDiagramFullscreen: () => Promise.resolve(),
      setActiveTool: (tool: ToolType) => Promise.resolve(),
      updateFormula: (formula: string) => Promise.resolve(),
      addFormulaNode: (node: FormulaNode) => Promise.resolve(),
      removeFormulaNode: (nodeId: Guid) => Promise.resolve(),
      selectFormulaNode: (nodeId: Guid) => Promise.resolve(),
      deselectAll: () => Promise.resolve(),
      hoverFormulaNode: (nodeId: Guid) => Promise.resolve(),
      clearHover: () => Promise.resolve(),
      connectNodes: (sourceId: Guid, targetId: Guid) => Promise.resolve(),
      togglePanel: (panelKey: keyof PanelVisibility) => {},
      execute: (command: string, ...args: any[]) => Promise.resolve(),
    };
  }
  return {
    startTransaction: () => store.startTransaction(),
    finalizeTransaction: () => store.finalizeTransaction(),
    abortTransaction: () => store.abortTransaction(),
    undo: () => store.undo(),
    redo: () => store.redo(),
    toggleFormulaFullscreen: () => store.execute("semio.qualityEditor.toggleFormulaFullscreen"),
    toggleDiagramFullscreen: () => store.execute("semio.qualityEditor.toggleDiagramFullscreen"),
    setActiveTool: (tool: ToolType) => store.execute("semio.qualityEditor.setActiveTool", tool),
    updateFormula: (formula: string) => store.execute("semio.qualityEditor.updateFormula", formula),
    addFormulaNode: (node: FormulaNode) => store.execute("semio.qualityEditor.addFormulaNode", node),
    removeFormulaNode: (nodeId: Guid) => store.execute("semio.qualityEditor.removeFormulaNode", nodeId),
    selectFormulaNode: (nodeId: Guid) => store.execute("semio.qualityEditor.selectFormulaNode", nodeId),
    deselectAll: () => store.execute("semio.qualityEditor.deselectAll"),
    hoverFormulaNode: (nodeId: Guid) => store.execute("semio.qualityEditor.hoverFormulaNode", nodeId),
    clearHover: () => store.execute("semio.qualityEditor.clearHover"),
    connectNodes: (sourceId: Guid, targetId: Guid) => store.execute("semio.qualityEditor.connectNodes", sourceId, targetId),
    togglePanel: (panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    execute: (command: string, ...args: any[]) => store.execute(command, ...args),
  };
}
