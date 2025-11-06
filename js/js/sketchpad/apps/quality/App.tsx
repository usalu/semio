// #region Header

// App.tsx

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

// #region Imports

import { DragEndEvent, useDraggable, useDroppable } from "@dnd-kit/core";
import {} from "@semio/assets";
import { Connection, Edge, Node, NodeTypes, ReactFlowInstance } from "@xyflow/react";
import React, { createContext, FC, memo, useCallback, useContext, useEffect, useMemo, useRef } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";
import * as Y from "yjs";
import { guid, Guid, Kit, Quality, QualityDiff } from "../../../semio";
import type { KitStore, QualityStore, SketchpadStore } from "../../App";
import {
  Canvas,
  createDefaultLayout,
  identitySelector,
  KitDiffAppStore,
  KitScopeProvider,
  LayoutCanvas,
  QualityScopeProvider,
  registerQualityAppStoreFactory,
  useActiveInteraction,
  useAddPanelSection,
  useAppType,
  useKit,
  useKitScope,
  useQuality,
  useQualityScope,
  useRemovePanelSection,
  useSketchpadCommands,
  useSketchpadStore,
  useSyncDeep,
} from "../../App";
import { Diagram as BaseDiagram, calculateDiagramLayout, DiagramNode, DraggableAvatar, HoverCard, HoverCardContent, HoverCardTrigger, Input, PlaceholderDiagramNode, Textarea, TreeContent, TreeItem } from "../../elements";
import type { AppWindowConfig, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, QualityAppId, Transact, YAttributes, YLeafMapNumber, YLeafMapString, YStringArray } from "../../sketchpad";
import { createPanelDefinition, PanelKind, ToolType } from "../../sketchpad";
import { AppConfig } from "../index";

// #endregion

// #region Types

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

export enum QualityAppWindowType {
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
  windowLayout?: any;
}

export interface QualityAppEdit extends KitDiffAppEdit<QualityAppSelectionDiff> {}

export interface QualityAppState {
  fullscreenWindow: QualityAppFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool: ToolType;
  selection?: QualityAppSelection;
  hover?: QualityAppHover;
  formulaNodes: FormulaNode[];
  windowLayout?: any;
}

export interface QualityAppCommandContext extends KitCommandContext {
  qualityApp: QualityAppState;
  Guid: Guid;
}

export interface QualityAppCommandResult {
  diff?: QualityAppDiff;
  qualityDiff?: QualityDiff;
}

export interface FormulaFunction {
  name: string;
  category: "numeric" | "branching" | "data" | "text" | "comparison";
  arity: number | "variadic";
  icon?: string;
  description: string;
  calculate: (...operands: any[]) => any;
  toLatex: (...operands: string[]) => string;
}

// #endregion

// #region Functions

export const formulaFunctions: Record<string, FormulaFunction> = {
  Add: {
    name: "Add",
    category: "numeric",
    arity: "variadic",
    icon: "plus",
    description: "Add two or more numbers",
    calculate: (...operands: number[]) => operands.reduce((sum, val) => sum + val, 0),
    toLatex: (...operands: string[]) => operands.join(" + "),
  },
  Subtract: {
    name: "Subtract",
    category: "numeric",
    arity: 2,
    icon: "minus",
    description: "Subtract second number from first",
    calculate: (a: number, b: number) => a - b,
    toLatex: (a: string, b: string) => `${a} - ${b}`,
  },
  Multiply: {
    name: "Multiply",
    category: "numeric",
    arity: "variadic",
    icon: "times",
    description: "Multiply two or more numbers",
    calculate: (...operands: number[]) => operands.reduce((product, val) => product * val, 1),
    toLatex: (...operands: string[]) => operands.join(" \\times "),
  },
  Divide: {
    name: "Divide",
    category: "numeric",
    arity: 2,
    icon: "divide",
    description: "Divide first number by second",
    calculate: (a: number, b: number) => (b !== 0 ? a / b : NaN),
    toLatex: (a: string, b: string) => `\\frac{${a}}{${b}}`,
  },
  Power: {
    name: "Power",
    category: "numeric",
    arity: 2,
    icon: "superscript",
    description: "Raise first number to the power of second",
    calculate: (a: number, b: number) => Math.pow(a, b),
    toLatex: (a: string, b: string) => `{${a}}^{${b}}`,
  },
  Sqrt: {
    name: "Sqrt",
    category: "numeric",
    arity: 1,
    icon: "square-root",
    description: "Calculate square root",
    calculate: (a: number) => Math.sqrt(a),
    toLatex: (a: string) => `\\sqrt{${a}}`,
  },
  Smaller: {
    name: "Smaller",
    category: "comparison",
    arity: 2,
    icon: "less-than",
    description: "Check if first value is smaller than second",
    calculate: (a: any, b: any) => a < b,
    toLatex: (a: string, b: string) => `${a} < ${b}`,
  },
  Greater: {
    name: "Greater",
    category: "comparison",
    arity: 2,
    icon: "greater-than",
    description: "Check if first value is greater than second",
    calculate: (a: any, b: any) => a > b,
    toLatex: (a: string, b: string) => `${a} > ${b}`,
  },
  Equal: {
    name: "Equal",
    category: "comparison",
    arity: 2,
    icon: "equals",
    description: "Check if two values are equal",
    calculate: (a: any, b: any) => a === b,
    toLatex: (a: string, b: string) => `${a} = ${b}`,
  },
  If: {
    name: "If",
    category: "branching",
    arity: 3,
    icon: "question",
    description: "If condition is true, return first value, else return second",
    calculate: (condition: boolean, thenValue: any, elseValue: any) => (condition ? thenValue : elseValue),
    toLatex: (condition: string, thenValue: string, elseValue: string) => `\\text{if } ${condition} \\text{ then } ${thenValue} \\text{ else } ${elseValue}`,
  },
  Switch: {
    name: "Switch",
    category: "branching",
    arity: "variadic",
    icon: "switch",
    description: "Match value against cases and return corresponding result",
    calculate: (value: any, ...cases: any[]) => {
      for (let i = 0; i < cases.length - 1; i += 2) {
        if (value === cases[i]) return cases[i + 1];
      }
      return cases.length % 2 === 1 ? cases[cases.length - 1] : undefined;
    },
    toLatex: (value: string, ...cases: string[]) => {
      const casesLatex = [];
      for (let i = 0; i < cases.length - 1; i += 2) {
        casesLatex.push(`${cases[i]} \\rightarrow ${cases[i + 1]}`);
      }
      if (cases.length % 2 === 1) {
        casesLatex.push(`\\text{default} \\rightarrow ${cases[cases.length - 1]}`);
      }
      return `\\text{switch}(${value}) \\{ ${casesLatex.join(", ")} \\}`;
    },
  },
  StartsWith: {
    name: "StartsWith",
    category: "text",
    arity: 2,
    icon: "text",
    description: "Check if string starts with prefix",
    calculate: (str: string, prefix: string) => str.startsWith(prefix),
    toLatex: (str: string, prefix: string) => `\\text{StartsWith}(${str}, ${prefix})`,
  },
  Name: {
    name: "Name",
    category: "text",
    arity: 1,
    icon: "tag",
    description: "Get the name of an entity",
    calculate: (entity: any) => entity?.name || "",
    toLatex: (entity: string) => `\\text{Name}(${entity})`,
  },
  List: {
    name: "List",
    category: "data",
    arity: "variadic",
    icon: "list",
    description: "Create a list of values",
    calculate: (...values: any[]) => values,
    toLatex: (...values: string[]) => `[${values.join(", ")}]`,
  },
  Dictionary: {
    name: "Dictionary",
    category: "data",
    arity: "variadic",
    icon: "book",
    description: "Create a dictionary from key-value pairs",
    calculate: (...pairs: any[]) => {
      const dict: Record<string, any> = {};
      for (const pair of pairs) {
        if (pair && typeof pair === "object" && "key" in pair && "value" in pair) {
          dict[pair.key] = pair.value;
        }
      }
      return dict;
    },
    toLatex: (...pairs: string[]) => `\\{${pairs.join(", ")}\\}`,
  },
  KeyValuePair: {
    name: "KeyValuePair",
    category: "data",
    arity: 2,
    icon: "key",
    description: "Create a key-value pair for a dictionary",
    calculate: (key: any, value: any) => ({ key, value }),
    toLatex: (key: string, value: string) => `${key}: ${value}`,
  },
  Key: {
    name: "Key",
    category: "data",
    arity: 1,
    icon: "key",
    description: "Extract key from a key-value pair",
    calculate: (pair: any) => pair?.key,
    toLatex: (pair: string) => `\\text{Key}(${pair})`,
  },
  Value: {
    name: "Value",
    category: "data",
    arity: 1,
    icon: "value",
    description: "Extract value from a key-value pair",
    calculate: (pair: any) => pair?.value,
    toLatex: (pair: string) => `\\text{Value}(${pair})`,
  },
  InList: {
    name: "InList",
    category: "data",
    arity: 2,
    icon: "list-check",
    description: "Check if value is in list",
    calculate: (value: any, list: any[]) => Array.isArray(list) && list.includes(value),
    toLatex: (value: string, list: string) => `${value} \\in ${list}`,
  },
  HasKey: {
    name: "HasKey",
    category: "data",
    arity: 2,
    icon: "key-check",
    description: "Check if dictionary has key",
    calculate: (key: any, dict: any) => typeof dict === "object" && dict !== null && key in dict,
    toLatex: (key: string, dict: string) => `${key} \\in \\text{keys}(${dict})`,
  },
};

export function parseFormula(formula: string): any {
  const tokens = tokenizeFormula(formula);
  const [ast] = parseTokens(tokens, 0);
  return ast;
}

function tokenizeFormula(formula: string): string[] {
  const tokens: string[] = [];
  let current = "";
  let inString = false;

  for (let i = 0; i < formula.length; i++) {
    const char = formula[i];

    if (char === "'") {
      if (inString) {
        tokens.push(current + char);
        current = "";
        inString = false;
      } else {
        if (current.trim()) tokens.push(current.trim());
        current = char;
        inString = true;
      }
    } else if (inString) {
      current += char;
    } else if (char === "(") {
      if (current.trim()) tokens.push(current.trim());
      tokens.push("(");
      current = "";
    } else if (char === ")") {
      if (current.trim()) tokens.push(current.trim());
      tokens.push(")");
      current = "";
    } else if (char === " " || char === "\t" || char === "\n") {
      if (current.trim()) tokens.push(current.trim());
      current = "";
    } else {
      current += char;
    }
  }

  if (current.trim()) tokens.push(current.trim());
  return tokens;
}

function parseTokens(tokens: string[], start: number): [any, number] {
  if (start >= tokens.length) return [null, start];

  const token = tokens[start];

  if (token === "(") {
    const list: any[] = [];
    let i = start + 1;
    while (i < tokens.length && tokens[i] !== ")") {
      const [item, newI] = parseTokens(tokens, i);
      list.push(item);
      i = newI;
    }
    return [list, i + 1];
  }

  return [token, start + 1];
}

export function formulaToLatex(ast: any): string {
  if (typeof ast === "string") {
    if (ast.startsWith("'") && ast.endsWith("'")) {
      const content = ast.slice(1, -1);
      if (/\d+\s*[a-zA-Z²³°]+/.test(content)) {
        return `\\text{${content}}`;
      }
      return `\\text{"${content}"}`;
    } else if (ast.startsWith("$")) {
      return `\\textit{${ast.substring(1)}}`;
    } else if (ast.includes(".")) {
      return `\\mathit{${ast}}`;
    } else if (!isNaN(Number(ast))) {
      return ast;
    } else {
      return `\\text{${ast}}`;
    }
  }

  if (Array.isArray(ast) && ast.length > 0) {
    const functionName = ast[0];
    const operands = ast.slice(1);

    const fn = formulaFunctions[functionName];
    if (fn) {
      const operandLatex = operands.map((op) => formulaToLatex(op));
      return fn.toLatex(...operandLatex);
    }

    const operandLatex = operands.map((op) => formulaToLatex(op));
    return `\\text{${functionName}}(${operandLatex.join(", ")})`;
  }

  return "\\text{?}";
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

// #endregion

// #region Commands

const qualityAppCommands = {
  "semio.qualityApp.toggleFormulaFullscreen": (context: QualityAppCommandContext): QualityAppCommandResult => {
    const currentPanel = context.qualityApp.fullscreenWindow;
    const newPanel = currentPanel === QualityAppFullscreenWindow.Formula ? QualityAppFullscreenWindow.None : QualityAppFullscreenWindow.Formula;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.qualityApp.toggleDiagramFullscreen": (context: QualityAppCommandContext): QualityAppCommandResult => {
    const currentPanel = context.qualityApp.fullscreenWindow;
    const newPanel = currentPanel === QualityAppFullscreenWindow.Diagram ? QualityAppFullscreenWindow.None : QualityAppFullscreenWindow.Diagram;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.qualityApp.setActiveTool": (context: QualityAppCommandContext, tool: ToolType): QualityAppCommandResult => {
    return {
      diff: {
        activeTool: tool,
      },
    };
  },
  "semio.qualityApp.updateFormula": (context: QualityAppCommandContext, formula: string): QualityAppCommandResult => {
    return {
      qualityDiff: {
        formula,
      },
    };
  },
  "semio.qualityApp.addFormulaNode": (context: QualityAppCommandContext, node: FormulaNode): QualityAppCommandResult => {
    const currentNodes = context.qualityApp.formulaNodes || [];
    return {
      diff: {
        formulaNodes: [...currentNodes, node],
      },
    };
  },
  "semio.qualityApp.removeFormulaNode": (context: QualityAppCommandContext, nodeId: Guid): QualityAppCommandResult => {
    const currentNodes = context.qualityApp.formulaNodes || [];
    return {
      diff: {
        formulaNodes: currentNodes.filter((n) => n.id !== nodeId),
      },
    };
  },
  "semio.qualityApp.selectFormulaNode": (context: QualityAppCommandContext, nodeId: Guid): QualityAppCommandResult => {
    const currentSelection = context.qualityApp.selection;
    return {
      diff: {
        selection: {
          formulaNodes: {
            removed: currentSelection?.formulaNodes ?? [],
            added: [nodeId],
          },
        },
      },
    };
  },
  "semio.qualityApp.deselectAll": (context: QualityAppCommandContext): QualityAppCommandResult => {
    const currentSelection = context.qualityApp.selection;
    return {
      diff: {
        selection: {
          formulaNodes: { removed: currentSelection?.formulaNodes ?? [] },
        },
      },
    };
  },
  "semio.qualityApp.hoverFormulaNode": (context: QualityAppCommandContext, nodeId: Guid): QualityAppCommandResult => {
    return {
      diff: {
        hover: { formulaNode: nodeId },
      },
    };
  },
  "semio.qualityApp.clearHover": (context: QualityAppCommandContext): QualityAppCommandResult => {
    return {
      diff: {
        hover: {},
      },
    };
  },
  "semio.qualityApp.connectNodes": (context: QualityAppCommandContext, sourceId: Guid, targetId: Guid): QualityAppCommandResult => {
    const currentNodes = context.qualityApp.formulaNodes || [];
    const updatedNodes = currentNodes.map((node) => {
      if (node.id === sourceId) {
        const currentChildren = node.children || [];
        if (!currentChildren.includes(targetId)) {
          return { ...node, children: [...currentChildren, targetId] };
        }
      }
      return node;
    });
    return {
      diff: {
        formulaNodes: updatedNodes,
      },
    };
  },
};

// #endregion

// #region Store

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

  get windowLayout(): any {
    const layoutStr = this.yMap.get("windowLayout") as string | undefined;
    return layoutStr ? JSON.parse(layoutStr) : undefined;
  }
  set windowLayout(layout: any) {
    if (layout) {
      this.yMap.set("windowLayout", JSON.stringify(layout));
    } else {
      this.yMap.delete("windowLayout");
    }
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
      windowLayout: this.windowLayout,
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
      if (diff.windowLayout !== undefined) {
        this.windowLayout = diff.windowLayout;
      }
    });
  };

  async executeCommand<T>(command: string, ...args: any[]): Promise<T> {
    let origin: string | undefined;
    let rest: any[];

    if (typeof args[0] === "string" && args[0].startsWith("semio.sketchpad.")) {
      origin = args[0];
      rest = args.slice(1);
    } else {
      origin = undefined;
      rest = args;
    }

    if (command === "semio.qualityApp.startTransaction") {
      console.group(`[${origin || "unknown"}] Transaction: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.qualityApp.finalizeTransaction") {
      this.finalizeTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.qualityApp.abortTransaction") {
      this.abortTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.qualityApp.undo") {
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
      this.undo();
      return {} as T;
    }
    if (command === "semio.qualityApp.redo") {
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
      this.redo();
      return {} as T;
    }

    console.group(`[${origin || "unknown"}] Executing command: "${command}"`);
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
      origin,
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

if (typeof window !== "undefined") {
  registerQualityAppStoreFactory((parent, yMap, transact, id, state) => new QualityAppStore(parent, yMap, transact, id));
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
  if (!store) return null;
  const selectedSelector = selector || identitySelector;
  return useSyncDeep<QualityAppState>(store as QualityAppStore, selectedSelector as (value: QualityAppState) => QualityAppState);
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
      setActiveTool: (_origin: string, _tool: ToolType) => Promise.resolve(),
      updateFormula: (_origin: string, _formula: string) => Promise.resolve(),
      addFormulaNode: (_origin: string, _node: FormulaNode) => Promise.resolve(),
      removeFormulaNode: (_origin: string, _nodeId: Guid) => Promise.resolve(),
      selectFormulaNode: (_origin: string, _nodeId: Guid) => Promise.resolve(),
      deselectAll: () => Promise.resolve(),
      hoverFormulaNode: (_origin: string, _nodeId: Guid) => Promise.resolve(),
      clearHover: () => Promise.resolve(),
      connectNodes: (_origin: string, _sourceId: Guid, _targetId: Guid) => Promise.resolve(),
      togglePanel: (_origin: string, _panelKey: keyof PanelVisibility) => {},
      execute: (_origin: string, _command: string, ..._args: any[]) => Promise.resolve(),
    };
  }
  return {
    startTransaction: (origin: string) => store.startTransaction(),
    finalizeTransaction: (origin: string) => store.finalizeTransaction(),
    abortTransaction: (origin: string) => store.abortTransaction(),
    undo: (origin: string) => store.undo(),
    redo: (origin: string) => store.redo(),
    toggleFormulaFullscreen: (origin: string) => store.execute("semio.qualityApp.toggleFormulaFullscreen", origin),
    toggleDiagramFullscreen: (origin: string) => store.execute("semio.qualityApp.toggleDiagramFullscreen", origin),
    setActiveTool: (origin: string, tool: ToolType) => store.execute("semio.qualityApp.setActiveTool", origin, tool),
    updateFormula: (origin: string, formula: string) => store.execute("semio.qualityApp.updateFormula", origin, formula),
    addFormulaNode: (origin: string, node: FormulaNode) => store.execute("semio.qualityApp.addFormulaNode", origin, node),
    removeFormulaNode: (origin: string, nodeId: Guid) => store.execute("semio.qualityApp.removeFormulaNode", origin, nodeId),
    selectFormulaNode: (origin: string, nodeId: Guid) => store.execute("semio.qualityApp.selectFormulaNode", origin, nodeId),
    deselectAll: (origin: string) => store.execute("semio.qualityApp.deselectAll", origin),
    hoverFormulaNode: (origin: string, nodeId: Guid) => store.execute("semio.qualityApp.hoverFormulaNode", origin, nodeId),
    clearHover: (origin: string) => store.execute("semio.qualityApp.clearHover", origin),
    connectNodes: (origin: string, sourceId: Guid, targetId: Guid) => store.execute("semio.qualityApp.connectNodes", origin, sourceId, targetId),
    togglePanel: (origin: string, panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    execute: (origin: string, command: string, ...args: any[]) => store.execute(command, origin, ...args),
  };
}

// #endregion

// #region Components

declare global {
  interface Window {
    MathJax?: any;
  }
}

const FunctionNodeComponent: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const initials = data.label.substring(0, 2).toUpperCase();
  return <DiagramNode content={initials} selected={selected} showTopHandle showBottomHandle />;
};

const QualityNodeComponent: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const initials = data.label
    .split(".")
    .map((part: string) => part[0])
    .join("")
    .substring(0, 2)
    .toUpperCase();
  return <DiagramNode content={initials} selected={selected} showTopHandle showBottomHandle />;
};

const VariableNodeComponent: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const varName = data.label.startsWith("$") ? data.label.substring(1) : data.label;
  const initials = varName.substring(0, 2).toUpperCase();
  return <DiagramNode content={initials} selected={selected} showTopHandle showBottomHandle />;
};

const ValueNodeComponent: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const display = data.label.length > 4 ? data.label.substring(0, 4) : data.label;
  return <DiagramNode content={display} selected={selected} showTopHandle />;
};

const PlaceholderNodeComponent: FC<{ data: any }> = ({ data }) => {
  return <PlaceholderDiagramNode id={data.id} />;
};

const nodeTypes: NodeTypes = {
  function: FunctionNodeComponent,
  quality: QualityNodeComponent,
  variable: VariableNodeComponent,
  unit: ValueNodeComponent,
  value: ValueNodeComponent,
  placeholder: PlaceholderNodeComponent,
};

interface QualityDiagramProps {
  reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null>;
}

const QualityDiagram: FC<QualityDiagramProps> = ({ reactFlowInstanceRef }) => {
  const formulaNodes = useQualityApp((s) => s.formulaNodes) as any[];
  const { selectFormulaNode, hoverFormulaNode, clearHover, connectNodes } = useQualityAppCommands();
  const { setNodeRef: setDroppableRef } = useDroppable({ id: "quality-diagram-drop-zone" });

  const { nodes: initialNodes, edges: initialEdges } = useMemo(() => {
    if (!formulaNodes || formulaNodes.length === 0) {
      const placeholderNode: Node = {
        id: "root-placeholder",
        type: "placeholder",
        position: { x: 0, y: 0 },
        data: { id: "root-placeholder" },
      };
      return { nodes: [placeholderNode], edges: [] };
    }

    const nodes: Node[] = [];
    const edges: Edge[] = [];
    const placeholderNodes: Node[] = [];
    const placeholderEdges: Edge[] = [];

    formulaNodes.forEach((node) => {
      nodes.push({
        id: node.id,
        type: node.type,
        position: { x: node.x ?? 0, y: node.y ?? 0 },
        data: { label: node.name },
      });

      if (node.children) {
        node.children.forEach((childId: string) => {
          edges.push({
            id: `${node.id}-${childId}`,
            source: node.id,
            target: childId,
          });
        });
      }

      if (node.type === "function") {
        const fn = formulaFunctions[node.name];
        const arity = fn?.arity;
        const currentChildCount = node.children?.length || 0;

        if (arity === "variadic" || (typeof arity === "number" && currentChildCount < arity)) {
          const maxPlaceholders = arity === "variadic" ? 1 : arity - currentChildCount;

          for (let i = 0; i < maxPlaceholders; i++) {
            const placeholderId = `${node.id}-placeholder-${currentChildCount + i}`;
            placeholderNodes.push({
              id: placeholderId,
              type: "placeholder",
              position: { x: 0, y: 0 },
              data: {
                id: placeholderId,
                parentId: node.id,
                operandIndex: currentChildCount + i,
              },
            });

            placeholderEdges.push({
              id: `${node.id}-${placeholderId}`,
              source: node.id,
              target: placeholderId,
              style: { strokeDasharray: "5 5", opacity: 0.5 },
              animated: false,
            });
          }
        }
      }
    });

    const allNodes = [...nodes, ...placeholderNodes];
    const allEdges = [...edges, ...placeholderEdges];

    return calculateDiagramLayout(allNodes, allEdges, {
      direction: "TB",
      nodeWidth: 48,
      nodeHeight: 48,
      rankSep: 80,
      nodeSep: 50,
    });
  }, [formulaNodes]);

  const handleConnect = useCallback(
    (connection: Connection) => {
      if (connection.source && connection.target) {
        connectNodes?.("semio.sketchpad.app.quality.diagram.connect", connection.source, connection.target);
      }
    },
    [connectNodes],
  );

  return (
    <div ref={setDroppableRef} className="h-full w-full">
      <BaseDiagram
        nodeTypes={nodeTypes}
        initialNodes={initialNodes}
        initialEdges={initialEdges}
        onConnect={handleConnect}
        onNodeClick={(_: React.MouseEvent, node: any) => selectFormulaNode("semio.sketchpad.app.quality.diagram.nodeClick", node.id)}
        onNodeMouseEnter={(_: React.MouseEvent, node: any) => hoverFormulaNode("semio.sketchpad.app.quality.diagram.nodeMouseEnter", node.id)}
        onNodeMouseLeave={() => clearHover("semio.sketchpad.app.quality.diagram.nodeMouseLeave")}
        reactFlowInstanceRef={reactFlowInstanceRef}
      />
    </div>
  );
};

const Formula: FC = () => {
  const quality = useQuality(undefined, undefined, true) as Quality | undefined;
  const mathRef = useRef<HTMLDivElement>(null);

  const formulaToLatexString = (formula?: string): string => {
    if (!formula) return "\\text{No formula defined}";

    try {
      const ast = parseFormula(formula);
      return formulaToLatex(ast);
    } catch {
      return `\\text{${formula}}`;
    }
  };

  useEffect(() => {
    const loadMathJax = () => {
      if (window.MathJax) {
        if (mathRef.current) {
          window.MathJax.typesetPromise([mathRef.current]).catch(() => {});
        }
        return;
      }
      const script = document.createElement("script");
      script.src = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js";
      script.async = true;
      script.onload = () => {
        if (window.MathJax && mathRef.current) {
          window.MathJax.typesetPromise([mathRef.current]).catch(() => {});
        }
      };
      script.onerror = () => {};
      document.head.appendChild(script);
    };
    loadMathJax();
  }, []);

  useEffect(() => {
    if (window.MathJax && mathRef.current) {
      mathRef.current.innerHTML = "";
      const latex = formulaToLatexString(quality?.formula);
      mathRef.current.textContent = `\\[${latex}\\]`;
      window.MathJax.typesetPromise([mathRef.current]).catch(() => {});
    }
  }, [quality?.formula, formulaToLatexString]);

  return (
    <div className="h-full w-full border-b border-foreground bg-base flex items-center justify-center overflow-auto">
      <div ref={mathRef} className="text-foreground p-4" style={{ fontSize: "1.5rem" }}></div>
    </div>
  );
};

export const QualityDetails: FC = () => {
  const quality = useQuality(undefined, undefined, true) as Quality | undefined;
  const { updateFormula } = useQualityAppCommands();

  if (!quality) return null;

  return (
    <>
      <TreeItem id="semio.sketchpad.app.quality.key">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.key" value={quality.key ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.name">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.name" value={quality.name ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.description">
        <TreeContent>
          <Textarea id="semio.sketchpad.app.quality.panel.details.description" value={quality.description ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.formula">
        <TreeContent>
          <Textarea
            id="semio.sketchpad.app.quality.panel.details.formula"
            value={quality.formula ?? ""}
            onChange={(e) => updateFormula("semio.sketchpad.app.quality.panel.details.formula", e.target.value)}
            className="w-full font-mono text-xs"
            rows={5}
            placeholderId="semio.sketchpad.app.quality.formulaPlaceholder"
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.defaultSiUnit">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.defaultSiUnit" value={quality.defaultSiUnit ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.defaultImperialUnit">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.defaultImperialUnit" value={quality.defaultImperialUnit ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.kind">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.kind" type="number" value={quality.kind?.toString() ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.canScale">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.canScale" type="checkbox" checked={quality.canScale ?? false} disabled className="size-tiny" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.defaultValue">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.defaultValue" type="number" value={quality.defaultValue?.toString() ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.min">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.min" type="number" value={quality.min?.toString() ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.isMinExcluded">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.isMinExcluded" type="checkbox" checked={quality.isMinExcluded ?? false} disabled className="size-tiny" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.max">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.max" type="number" value={quality.max?.toString() ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.isMaxExcluded">
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.isMaxExcluded" type="checkbox" checked={quality.isMaxExcluded ?? false} disabled className="size-tiny" showLabel />
        </TreeContent>
      </TreeItem>
    </>
  );
};

interface FunctionNodeProps {
  name: string;
  type: "function" | "quality" | "variable" | "unit" | "value";
  label: string;
}

const FunctionNode: FC<FunctionNodeProps> = ({ name, type, label }) => {
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();
  const interactionId = `formula-${type}-${name}`;

  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { name, type },
  });

  const isInteracting = activeInteraction === interactionId;
  const shouldFade = !!(activeInteraction && !isInteracting);

  const enhancedListeners = {
    ...listeners,
    onPointerDown: (e: React.PointerEvent) => {
      setActiveInteraction(interactionId);
      listeners?.onPointerDown?.(e);
    },
  };

  const initials = name.substring(0, 2).toUpperCase();
  const fn = formulaFunctions[name];

  return (
    <HoverCard openDelay={500}>
      <HoverCardTrigger asChild>
        <div>
          <DraggableAvatar content={initials} shouldFade={shouldFade} title={label} dragRef={setNodeRef} dragListeners={enhancedListeners} dragAttributes={attributes} />
        </div>
      </HoverCardTrigger>
      <HoverCardContent className="w-80">
        <div className="space-y-1">
          <h4 className="text-sm font-semibold">{label}</h4>
          {fn?.description && <p className="text-sm">{fn.description}</p>}
        </div>
      </HoverCardContent>
    </HoverCard>
  );
};

interface QualityAvatarProps {
  qualityId?: Guid;
  quality?: Quality;
  showHoverCard?: boolean;
}

export const QualityAvatar: FC<QualityAvatarProps> = ({ qualityId, quality: qualityProp, showHoverCard = false }) => {
  const qualityFromStore = qualityId && !qualityProp ? (useQuality(undefined, qualityId) as Quality | null) : null;
  const quality = qualityProp || qualityFromStore;
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();

  const interactionId = quality ? `quality-${quality.key}` : "quality-unknown";
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { quality, type: "quality" },
  });

  const isInteracting = activeInteraction === interactionId;
  const shouldFade = !!(activeInteraction && !isInteracting);

  const enhancedListeners = {
    ...listeners,
    onPointerDown: (e: React.PointerEvent) => {
      setActiveInteraction(interactionId);
      listeners?.onPointerDown?.(e);
    },
  };

  if (!quality) {
    return null;
  }

  const displayName = quality.name || quality.key || "Q";
  const initials =
    displayName
      .split(".")
      .map((part) => part[0])
      .filter(Boolean)
      .join("")
      .substring(0, 2)
      .toUpperCase() || "Q";

  if (!showHoverCard) {
    return <DraggableAvatar content={initials} shouldFade={shouldFade} title={quality.name || quality.key} dragRef={setNodeRef} dragListeners={enhancedListeners} dragAttributes={attributes} />;
  }

  return (
    <HoverCard openDelay={500}>
      <HoverCardTrigger asChild>
        <div>
          <DraggableAvatar content={initials} shouldFade={shouldFade} title={quality.name || quality.key} dragRef={setNodeRef} dragListeners={enhancedListeners} dragAttributes={attributes} />
        </div>
      </HoverCardTrigger>
      <HoverCardContent className="w-80">
        <div className="space-y-1">
          <h4 className="text-sm font-semibold">{quality.name}</h4>
          <p className="text-xs text-muted-foreground">{quality.key}</p>
          {quality.description && <p className="text-sm">{quality.description}</p>}
          {quality.formula && (
            <div className="text-xs text-muted-foreground mt-2">
              <span className="font-mono">{quality.formula}</span>
            </div>
          )}
        </div>
      </HoverCardContent>
    </HoverCard>
  );
};

export const QualityWorkbench: FC = () => {
  const { t } = useTranslation();
  const kit = useKit(undefined, undefined, true) as Kit | null;
  const qualities = kit?.qualities || [];

  return (
    <>
      <TreeItem id="semio.sketchpad.app.quality.numericFunctions">
        <TreeContent>
          <div className="flex flex-wrap gap-unit p-unit">
            <FunctionNode name="Add" type="function" label={t("semio.sketchpad.app.quality.add")} />
            <FunctionNode name="Subtract" type="function" label={t("semio.sketchpad.app.quality.subtract")} />
            <FunctionNode name="Multiply" type="function" label={t("semio.sketchpad.app.quality.multiply")} />
            <FunctionNode name="Divide" type="function" label={t("semio.sketchpad.app.quality.divide")} />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.branchingFunctions">
        <TreeContent>
          <div className="flex flex-wrap gap-unit p-unit">
            <FunctionNode name="If" type="function" label={t("semio.sketchpad.app.quality.if")} />
            <FunctionNode name="Switch" type="function" label={t("semio.sketchpad.app.quality.switch")} />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.quality.dataStructures">
        <TreeContent>
          <div className="flex flex-wrap gap-unit p-unit">
            <FunctionNode name="List" type="function" label={t("semio.sketchpad.app.quality.list")} />
            <FunctionNode name="Dictionary" type="function" label={t("semio.sketchpad.app.quality.dictionary")} />
          </div>
        </TreeContent>
      </TreeItem>
    </>
  );
};

interface QualityTreeNode {
  key: string;
  qualities: Quality[];
  children: Map<string, QualityTreeNode>;
}

const buildQualityTree = (qualities: Quality[]): Map<string, QualityTreeNode> => {
  const root = new Map<string, QualityTreeNode>();

  qualities.forEach((quality) => {
    if (!quality.key) return;

    const parts = quality.key.split(".");
    let currentLevel = root;

    parts.forEach((part, index) => {
      if (!currentLevel.has(part)) {
        currentLevel.set(part, {
          key: parts.slice(0, index + 1).join("."),
          qualities: [],
          children: new Map(),
        });
      }

      const node = currentLevel.get(part)!;

      if (index === parts.length - 1) {
        node.qualities.push(quality);
      }

      currentLevel = node.children;
    });
  });

  return root;
};

const QualityTree: FC<{ qualities: Quality[] }> = ({ qualities }) => {
  const tree = buildQualityTree(qualities);

  const renderNode = (key: string, node: QualityTreeNode, level: number = 0) => {
    const hasChildren = node.children.size > 0;
    const hasQualities = node.qualities.length > 0;

    if (hasChildren) {
      return (
        <TreeItem key={key} label={key}>
          <TreeContent>
            {hasQualities && (
              <div className="flex flex-wrap gap-unit p-unit">
                {node.qualities.map((quality) => (
                  <QualityAvatar key={quality.guid} quality={quality} showHoverCard={true} />
                ))}
              </div>
            )}
            {Array.from(node.children.entries()).map(([childKey, childNode]) => renderNode(childKey, childNode, level + 1))}
          </TreeContent>
        </TreeItem>
      );
    } else if (hasQualities) {
      return (
        <TreeContent key={key}>
          <div className="flex flex-wrap gap-unit p-unit">
            {node.qualities.map((quality) => (
              <QualityAvatar key={quality.guid} quality={quality} showHoverCard={true} />
            ))}
          </div>
        </TreeContent>
      );
    }

    return <></>;
  };

  return <>{Array.from(tree.entries()).map(([key, node]) => renderNode(key, node))}</>;
};

const QualityWorkbenchQualities: FC = () => {
  const { t } = useTranslation();
  const kit = useKit(undefined, undefined, true) as Kit | null;
  const qualities = kit?.qualities || [];

  if (qualities.length === 0) {
    return (
      <TreeContent>
        <div className="text-sm text-muted-foreground p-double">{t("semio.sketchpad.app.quality.noQualities")}</div>
      </TreeContent>
    );
  }

  return <QualityTree qualities={qualities} />;
};

// #endregion

// #region App

export interface AppProps {}

const FormulaWindow = memo(() => <Formula />);
FormulaWindow.displayName = "FormulaWindow";

const DiagramWindow = memo<{ reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null> }>(({ reactFlowInstanceRef }) => <QualityDiagram reactFlowInstanceRef={reactFlowInstanceRef} />);
DiagramWindow.displayName = "DiagramWindow";

const App: FC<AppProps> = () => {
  const fullscreenWindow = useQualityApp((s) => s.fullscreenWindow) as QualityAppFullscreenWindow;
  const { undo, redo, toggleFormulaFullscreen, toggleDiagramFullscreen, deselectAll, togglePanel, addFormulaNode, connectNodes, startTransaction, finalizeTransaction } = useQualityAppCommands();
  const quality = useQuality() as Quality | undefined;
  const appType = useAppType();

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  useHotkeys("ctrl+d", () => deselectAll("semio.sketchpad.app.quality.hotkey"));
  useHotkeys("ctrl+z", () => undo("semio.sketchpad.app.quality.hotkey"));
  useHotkeys("ctrl+y", () => redo("semio.sketchpad.app.quality.hotkey"));
  useHotkeys("ctrl+shift+z", () => redo("semio.sketchpad.app.quality.hotkey"));

  useEffect(() => {
    if (appType !== "quality") return;

    addSection("details", {
      id: "semio.sketchpad.app.quality.title",
      order: 0,
      content: () => <QualityDetails />,
    });

    return () => {
      removeSection("details", "semio.sketchpad.app.quality.title");
    };
  }, [appType, addSection, removeSection]);

  useEffect(() => {
    if (appType !== "quality") return;

    addSection("workbench", {
      id: "semio.sketchpad.app.quality.functions",
      order: 0,
      content: () => <QualityWorkbench />,
    });

    addSection("workbench", {
      id: "semio.sketchpad.app.quality.qualities",
      order: 1,
      content: () => <QualityWorkbenchQualities />,
    });

    return () => {
      removeSection("workbench", "semio.sketchpad.app.quality.functions");
      removeSection("workbench", "semio.sketchpad.app.quality.qualities");
    };
  }, [appType, addSection, removeSection]);

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over, delta } = event;

    if (over && over.id === "quality-diagram-drop-zone" && reactFlowInstanceRef.current) {
      if (!(event.activatorEvent instanceof PointerEvent)) {
        return;
      }

      const { x, y } = reactFlowInstanceRef.current.screenToFlowPosition({
        x: event.activatorEvent.clientX + delta.x,
        y: event.activatorEvent.clientY + delta.y,
      });

      const dragData = active.data.current as any;

      if (dragData) {
        startTransaction("semio.sketchpad.app.quality.drag");

        const targetNode = reactFlowInstanceRef.current.getNodes().find((n) => {
          const nodeBounds = {
            left: n.position.x,
            right: n.position.x + 48,
            top: n.position.y,
            bottom: n.position.y + 48,
          };
          return x >= nodeBounds.left && x <= nodeBounds.right && y >= nodeBounds.top && y <= nodeBounds.bottom;
        });

        const isPlaceholder = targetNode?.type === "placeholder";
        const parentId = isPlaceholder ? (targetNode?.data as any)?.parentId : undefined;
        const operandIndex = isPlaceholder ? (targetNode?.data as any)?.operandIndex : undefined;

        let node: FormulaNode;

        if (dragData.quality) {
          node = {
            id: guid(),
            type: "quality",
            name: dragData.quality.key,
            x: isPlaceholder ? 0 : x,
            y: isPlaceholder ? 0 : y,
          };
        } else if (dragData.type && dragData.name) {
          node = {
            id: guid(),
            type: dragData.type,
            name: dragData.name,
            x: isPlaceholder ? 0 : x,
            y: isPlaceholder ? 0 : y,
          };
        } else {
          finalizeTransaction("semio.sketchpad.app.quality.drag");
          return;
        }

        addFormulaNode("semio.sketchpad.app.quality.drag", node);

        if (isPlaceholder && parentId) {
          connectNodes("semio.sketchpad.app.quality.drag", parentId, node.id);
        }

        finalizeTransaction("semio.sketchpad.app.quality.drag");
      }
    }
  };

  useEffect(() => {
    const listener = (e: Event) => {
      const customEvent = e as CustomEvent<DragEndEvent>;
      handleDragEnd(customEvent.detail);
    };
    window.addEventListener("quality-drag-end", listener);
    return () => window.removeEventListener("quality-drag-end", listener);
  }, [handleDragEnd]);

  const store = useQualityAppStore() as QualityAppStore | null;
  const windowLayout = useQualityApp((s) => s.windowLayout);

  const defaultLayout = useMemo(() => {
    return createDefaultLayout([QualityAppWindowType.Formula, QualityAppWindowType.Diagram], "row", [20, 80]);
  }, []);

  const windowConfig: AppWindowConfig = useMemo(() => {
    return {
      windowTypes: [
        {
          id: QualityAppWindowType.Formula,
          label: "Formula",
          component: (props: any) => <FormulaWindow />,
        },
        {
          id: QualityAppWindowType.Diagram,
          label: "Diagram",
          component: (props: any) => <DiagramWindow reactFlowInstanceRef={reactFlowInstanceRef} />,
        },
      ],
      defaultLayout,
    };
  }, [defaultLayout, reactFlowInstanceRef]);

  const handleLayoutChange = useCallback(
    (config: any) => {
      if (store && typeof store.change === "function") {
        store.change({ windowLayout: config });
      }
    },
    [store],
  );

  return (
    <Canvas>
      <LayoutCanvas windowConfig={windowConfig} layoutState={windowLayout} onLayoutChange={handleLayoutChange} />
    </Canvas>
  );
};

export default App;

// #endregion

// #region Config

export const config: AppConfig = {
  id: "quality",
  component: App,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
    {
      path: "qualities/:quality",
      paramName: "quality",
      scopeProvider: QualityScopeProvider,
    },
  ],
  getPanels: (): PanelDefinition[] => [
    createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show"),
    createPanelDefinition(PanelKind.TOOLS, "semio.sketchpad.navbar.panelToggle.tools.show"),
    createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
    createPanelDefinition(PanelKind.HUD, "semio.sketchpad.navbar.panelToggle.hud.show"),
    createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
    createPanelDefinition(PanelKind.CHAT, "semio.sketchpad.navbar.panelToggle.chat.show"),
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show"),
  ],
  matchesPath: (pathParts: string[]) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "qualities" && isUuidPattern(pathParts[3]);
  },
  order: 40,
};

// #endregion
