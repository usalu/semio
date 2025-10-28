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
  KitDiffAppEdit,
  KitDiffAppStore,
  KitStore,
  PanelVisibility,
  QualityAppId,
  registerQualityAppStoreFactory,
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
import { commands as qualityAppCommands } from "./commands";

type YQualityAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YQualityApp = Y.Map<YQualityAppVal>;
type YQualityApps = Y.Map<YQualityApp>;

export interface FormulaNode {
  id: Guid;
  type: "function" | "quality" | "variable" | "unit" | "value";
  name: string;
  children?: Guid[];
  x?: number;
  y?: number;
}
export interface QualityAppSelection {
  formulaNodes?: Guid[];
}
export interface QualityAppSelectionFormulaNodesDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface QualityAppSelectionDiff {
  formulaNodes?: QualityAppSelectionFormulaNodesDiff;
}
export enum QualityAppFullscreenWindow {
  None = "none",
  Formula = "formula",
  Diagram = "diagram",
}
export interface QualityAppHover {
  formulaNode?: Guid;
}
export interface QualityAppDiff {
  selection?: QualityAppSelectionDiff;
  hover?: QualityAppHover;
  fullscreenWindow?: QualityAppFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  activeTool?: ToolType;
  formulaNodes?: FormulaNode[];
}
export interface QualityAppEdit extends KitDiffAppEdit<QualityAppSelectionDiff> {}
export interface QualityAppState {
  fullscreenWindow: QualityAppFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool: ToolType;
  selection?: QualityAppSelection;
  hover?: QualityAppHover;
  formulaNodes: FormulaNode[];
}

export interface QualityAppCommandContext extends KitCommandContext {
  qualityApp: QualityAppState;
  Guid: Guid;
}
export interface QualityAppCommandResult {
  diff?: QualityAppDiff;
  qualityDiff?: QualityDiff;
}

function inverseQualityAppSelectionDiff(selection: QualityAppSelection, diff: QualityAppSelectionDiff): QualityAppSelectionDiff {
  const inverse: QualityAppSelectionDiff = {};
  if (diff.formulaNodes) {
    inverse.formulaNodes = {
      added: diff.formulaNodes.removed ?? [],
      removed: diff.formulaNodes.added ?? [],
    };
  }
  return inverse;
}

class QualityAppStore extends KitDiffAppStore<QualityAppState, QualityAppDiff, QualityAppSelectionDiff, QualityAppEdit, QualityAppCommandContext, QualityAppCommandResult> {
  private readonly Guid: QualityAppId;

  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: Transact, id: QualityAppId) {
    super(parent, yMap, transact);
    this.Guid = id;

    transact(() => {
      if (!yMap.has("fullscreenWindow")) {
        yMap.set("fullscreenWindow", QualityAppFullscreenWindow.None);
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

    Object.entries(qualityAppCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  quality(): QualityStore | undefined {
    return this.parent.kit(this.Guid.kit).quality(this.Guid.quality);
  }

  kit(): KitStore {
    return this.parent.kit(this.Guid.kit);
  }

  get fullscreenWindow(): QualityAppFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as QualityAppFullscreenWindow;
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

  get selection(): QualityAppSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};
    const result: QualityAppSelection = {};
    const formulaNodes = selection.get("formulaNodes") as Y.Array<string>;
    if (formulaNodes && formulaNodes.length > 0) {
      result.formulaNodes = formulaNodes.toArray();
    }
    return result;
  }

  get hover(): QualityAppHover | undefined {
    const hover = this.yMap.get("hover") as Y.Map<any> | undefined;
    if (!hover) return undefined;
    const result: QualityAppHover = {};
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

  protected getSelection(): QualityAppSelection {
    return this.selection;
  }

  protected hash(state: QualityAppState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): QualityAppState {
    return {
      fullscreenWindow: this.fullscreenWindow,
      panelVisibility: this.panelVisibility,
      activeTool: this.activeTool,
      selection: this.selection,
      hover: this.hover,
      formulaNodes: this.formulaNodes,
    };
  }

  protected inverseSelectionDiff(selection: QualityAppSelection, diff: QualityAppSelectionDiff): QualityAppSelectionDiff {
    return inverseQualityAppSelectionDiff(selection, diff);
  }

  protected applySelectionDiff(selectionDiff: QualityAppSelectionDiff): void {
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

  change = (diff: QualityAppDiff) => {
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
    if (command === "semio.qualityApp.startTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.qualityApp.finalizeTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.qualityApp.abortTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.abortTransaction();
      return {} as T;
    }
    if (command === "semio.qualityApp.undo") {
      console.log(`Executing (special) command: "${command}"`);
      this.undo();
      return {} as T;
    }
    if (command === "semio.qualityApp.redo") {
      console.log(`Executing (special) command: "${command}"`);
      this.redo();
      return {} as T;
    }

    console.group(`Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) {
      console.groupEnd();
      throw new Error(`Command "${command}" not found in quality app store`);
    }

    const kitStore = this.kit();
    const state = this.snapshot();
    const kitState = kitStore.snapshot();
    const quality = this.quality();

    const context: QualityAppCommandContext = {
      qualityApp: state,
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
    console.groupEnd();
    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }
}

// Register the factory - deferred to avoid circular dependency issues
export function initializeQualityAppStore() {
  registerQualityAppStoreFactory((parent, yMap, transact, id, state) => new QualityAppStore(parent, yMap, transact, id));
}

// Auto-initialize if this module is imported
if (typeof window !== 'undefined') {
  setTimeout(() => initializeQualityAppStore(), 0);
}

type QualityAppScope = { guid: string };
const QualityAppScopeContext = createContext<QualityAppScope | null>(null);
export const QualityAppScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(QualityAppScopeContext.Provider, { value }, props.children as any);
};
const useQualityAppScope = () => useContext(QualityAppScopeContext);

export function useQualityAppStore<T>(selector?: (store: QualityAppStore) => T, id?: QualityAppId): T | QualityAppStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const qualityScope = useQualityScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  const resolvedQualityId = qualityScope?.guid ?? id?.quality;
  if (!resolvedKitId || !resolvedQualityId) {
    return null;
  }
  const qualityAppStore = store.qualityApp(resolvedKitId, resolvedQualityId);
  return selector ? selector(qualityAppStore) : qualityAppStore;
}

export function useQualityApp<T>(selector?: (state: QualityAppState) => T, id?: QualityAppId): T | QualityAppState | null {
  const store = useQualityAppStore(identitySelector, id);
  return useSyncDeep<QualityAppState, T>(store as QualityAppStore, selector ? selector : identitySelector);
}

export function useQualityAppCommands(id?: QualityAppId) {
  const store = useQualityAppStore(undefined, id) as QualityAppStore | null;
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
    toggleFormulaFullscreen: () => store.execute("semio.qualityApp.toggleFormulaFullscreen"),
    toggleDiagramFullscreen: () => store.execute("semio.qualityApp.toggleDiagramFullscreen"),
    setActiveTool: (tool: ToolType) => store.execute("semio.qualityApp.setActiveTool", tool),
    updateFormula: (formula: string) => store.execute("semio.qualityApp.updateFormula", formula),
    addFormulaNode: (node: FormulaNode) => store.execute("semio.qualityApp.addFormulaNode", node),
    removeFormulaNode: (nodeId: Guid) => store.execute("semio.qualityApp.removeFormulaNode", nodeId),
    selectFormulaNode: (nodeId: Guid) => store.execute("semio.qualityApp.selectFormulaNode", nodeId),
    deselectAll: () => store.execute("semio.qualityApp.deselectAll"),
    hoverFormulaNode: (nodeId: Guid) => store.execute("semio.qualityApp.hoverFormulaNode", nodeId),
    clearHover: () => store.execute("semio.qualityApp.clearHover"),
    connectNodes: (sourceId: Guid, targetId: Guid) => store.execute("semio.qualityApp.connectNodes", sourceId, targetId),
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
