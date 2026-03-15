// #region Header

// js/semio/sketchpad/Quality.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// #region Imports

import { DragEndEvent, useDraggable, useDroppable } from "@dnd-kit/core";
import { AddIcon, AwardIcon, ChatIcon, CodeIcon, HandIcon, IntersectIcon, MonitorIcon, MoonIcon, MousePointerIcon, RemoveIcon, SettingsIcon, SunIcon, TutorialIcon, UserIcon } from "@semio/assets";
import React, { createContext, FC, memo, useCallback, useContext, useEffect, useMemo, useRef } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";
import { useLabel } from "../i18n";
import { guid, Guid, Kit, Quality, QualityDiff, sumQualityInDesign } from "@semio/js/semio";
import type { Connection, Edge, Node, NodeTypes, ReactFlowInstance } from "../../../semio-elements/ui";
import {
  Diagram as BaseDiagram,
  calculateDiagramLayout,
  BasicChatPanel,
  DiagramNode,
  DraggableAvatar,
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
  Input,
  PlaceholderDiagramNode,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Textarea,
  Toggle,
  ToggleGroup,
  Tree,
  TreeContent,
  TreeItem,
  TreeRow,
  TreeStateProvider,
} from "../../../semio-elements/ui";
import type { AppWindowConfig, HookNoSetResult, HookResult, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, QualityAppId } from "./shared";
import { AppConfig, applySelectionComposition, AppPlugin, createPanelDefinition, Expertise, isSelectionToolKind, Mode, PanelKind, registerAppPlugin, registerEventHandler, resolveSelectionCompositionKind, Theme, ToolKind, toSelectionToolKind } from "./shared";
import type { KitStore, QualityStore, SketchpadStore } from "./Sketchpad";
import {
  Canvas,
  createDefaultQualityAppState,
  identitySelector,
  KitScopeProvider,
  LayoutCanvas,
  PlainKitDiffAppStore,
  QualityScopeProvider,
  registerQualityAppStoreFactory,
  useActiveInteraction,
  useAddPanelSection,
  useAddSidePanelTab,
  useAppType,
  useDevice,
  useExpertise,
  useKit,
  useKitScope,
  useLanguage,
  useMode,
  useQuality,
  useQualityScope,
  useRemovePanelSection,
  useRemoveSidePanelTab,
  useSketchpadCommands,
  useSketchpadStore,
  useSyncDeep,
  useTheme,
} from "./Sketchpad";

// #endregion Imports

// #region Types

// [👤semio📚js🗃️sketchpad💻qualitytsx🔖types](semiorepo://section/SEMIO/JS/SKETCHPAD/QUALITY.TSX/TYPES)
// Type definitions MUST declare quality app state, selections, and formula structures.

/**
 * Node in a formula graph with position, kind, and child references.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️formulanode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/FormulaNode)
 **/
export interface FormulaNode {
  id: Guid;
  kind: "function" | "quality" | "variable" | "unit" | "value";
  name: string;
  children?: Guid[];
  x?: number;
  y?: number;
}

/**
 * Selected formula node IDs in the quality app.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityappselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppSelection)
 **/
export interface QualityAppSelection {
  formulaNodes?: Guid[];
}

/**
 * Diff for added and removed formula node selections.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityappselectionformulanodesdiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppSelectionFormulaNodesDiff)
 **/
export interface QualityAppSelectionFormulaNodesDiff {
  added?: Guid[];
  removed?: Guid[];
}

/**
 * Diff for quality app selection changes.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityappselectiondiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppSelectionDiff)
 **/
export interface QualityAppSelectionDiff {
  formulaNodes?: QualityAppSelectionFormulaNodesDiff;
}

/**
 * Fullscreen window state for the quality app panels.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityappfullscreenwindow](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppFullscreenWindow)
 **/
export enum QualityAppFullscreenWindow {
  None = "none",
  Formula = "formula",
  Diagram = "diagram",
}

/**
 * Window kind identifiers for quality app layout.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityappwindowkind](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppWindowKind)
 **/
export enum QualityAppWindowKind {
  Formula = "formula",
  Diagram = "diagram",
}

/**
 * Hover state tracking the currently hovered formula node.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityapphover](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppHover)
 **/
export interface QualityAppHover {
  formulaNode?: Guid;
}

/**
 * Diff describing partial quality app state changes.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityappdiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppDiff)
 **/
export interface QualityAppDiff {
  selection?: QualityAppSelectionDiff;
  hover?: QualityAppHover;
  fullscreenWindow?: QualityAppFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  activeTool?: ToolKind;
  formulaNodes?: FormulaNode[];
  windowLayout?: any;
}

export interface QualityAppEdit extends KitDiffAppEdit<QualityAppSelectionDiff> { }

/**
 * Complete quality app state including selection, hover, formula nodes, and layout.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityappstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppState)
 **/
export interface QualityAppState {
  fullscreenWindow: QualityAppFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool: ToolKind;
  selection?: QualityAppSelection;
  hover?: QualityAppHover;
  formulaNodes: FormulaNode[];
  windowLayout?: any;
}

/**
 * Context passed to quality app command handlers.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityappcommandcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppCommandContext)
 **/
export interface QualityAppCommandContext extends KitCommandContext {
  qualityApp: QualityAppState;
  Guid: Guid;
}

/**
 * Result returned by quality app command handlers.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️qualityappcommandresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/QualityAppCommandResult)
 **/
export interface QualityAppCommandResult {
  diff?: QualityAppDiff;
  qualityDiff?: QualityDiff;
}

/**
 * Definition of a formula function with calculation and LaTeX rendering.
 * [👤semio📚js🗃️sketchpad💻quality🔖types🛠️formulafunction](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Types/d/i/FormulaFunction)
 **/
export interface FormulaFunction {
  name: string;
  category: "numeric" | "branching" | "data" | "text" | "comparison";
  arity: number | "variadic";
  icon?: string;
  description: string;
  calculate: (...operands: any[]) => any;
  toLatex: (...operands: string[]) => string;
}

// #endregion Types

// #region Functions

// [👤semio📚js🗃️sketchpad💻qualitytsx🔖functions](semiorepo://section/SEMIO/JS/SKETCHPAD/QUALITY.TSX/FUNCTIONS)
// Formula function definitions, parsing, and LaTeX conversion utilities MUST be declared here.

/**
 * Registry of available formula functions keyed by name.
 * [👤semio📚js🗃️sketchpad💻quality🔖functions🪨formulafunctions](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Functions/d/i/formulaFunctions)
 **/
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
  SumQualityInDesign: {
    name: "SumQualityInDesign",
    category: "data",
    arity: 3,
    icon: "sigma",
    description: "Sum the values of a quality across all pieces in a design",
    calculate: (kit: any, designGuid: string, qualityGuid: string) => sumQualityInDesign(kit, designGuid, qualityGuid),
    toLatex: (kit: string, designGuid: string, qualityGuid: string) => `\\sum_{\\text{pieces}} \\text{Quality}(${qualityGuid})`,
  },
};

/**
 * Parses a formula string into an S-expression AST.
 *The formula string MUST be a valid S-expression.
 * [👤semio📚js🗃️sketchpad💻quality🔖functions🛠️parseformula](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Functions/d/i/parseFormula)
 **/
export function parseFormula(formula: string): any {
  const tokens = tokenizeFormula(formula);
  const [ast] = parseTokens(tokens, 0);
  return ast;
}

/** tokenizeFormula holds the data fields for a tokenizeFormula record.
 **/
// [👤semio📚js🗃️sketchpad💻quality🔖functions🛠️tokenizeformula](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Functions/d/i/tokenizeFormula)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖functions🪨tokenizeformula](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Functions/d/i/tokenizeFormula)
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻quality🔖functions🛠️parsetokens](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Functions/d/i/parseTokens)
 * parseTokens holds the data fields for a parseTokens record.
 **/
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

/**
 * Converts a formula AST to a LaTeX string.
 *The AST MUST be produced by parseFormula.
 * [👤semio📚js🗃️sketchpad💻quality🔖functions🛠️formulatolatex](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Functions/d/i/formulaToLatex)
 **/
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

// [👤semio📚js🗃️sketchpad💻quality🔖functions🛠️inversequalityappselectiondiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Functions/d/i/inverseQualityAppSelectionDiff)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖functions🪨inversequalityappselectiondiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Functions/d/i/inverseQualityAppSelectionDiff)
 * inverseQualityAppSelectionDiff holds the data fields for a inverseQualityAppSelectionDiff record.
 **/
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

// #endregion Functions

// #region Commands

// [👤semio📚js🗃️sketchpad💻qualitytsx🔖commands](semiorepo://section/SEMIO/JS/SKETCHPAD/QUALITY.TSX/COMMANDS)
// Quality app command handlers MUST modify state through diff objects.

/**
 * [👤semio📚js🗃️sketchpad💻quality🔖commands🪨qualityappcommands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Commands/d/i/qualityAppCommands)
 * qualityAppCommands holds the data fields for a qualityAppCommands record.
 **/
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
  "semio.qualityApp.setActiveTool": (context: QualityAppCommandContext, tool: ToolKind): QualityAppCommandResult => {
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

// #endregion Commands

// #region Store

// [👤semio📚js🗃️sketchpad💻qualitytsx🔖store](semiorepo://section/SEMIO/JS/SKETCHPAD/QUALITY.TSX/STORE)
// Quality app store, hooks, and reactive state management MUST be declared here.

/**
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️qualityappstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/QualityAppStore)
 * QualityAppStore holds the data fields for a QualityAppStore record.
 **/
class QualityAppStore extends PlainKitDiffAppStore<QualityAppState, QualityAppDiff, QualityAppSelectionDiff, QualityAppEdit, QualityAppCommandContext, QualityAppCommandResult> {
  private readonly Guid: QualityAppId;

  constructor(parent: SketchpadStore, id: QualityAppId) {
    const defaultState: QualityAppState = {
      fullscreenWindow: QualityAppFullscreenWindow.None,
      panelVisibility: { toolbar: true, leftSidePanel: true, rightSidePanel: true, details: true },
      activeTool: ToolKind.SELECTION_NORMAL,
      selection: undefined,
      hover: undefined,
      formulaNodes: [],
      windowLayout: undefined,
    };
    super(parent, defaultState);
    this.Guid = id;

    Object.entries(qualityAppCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  quality(): QualityStore | undefined {
    return this.parentStore.kit(this.Guid.kit).quality(this.Guid.quality);
  }

  kit(): KitStore {
    return this.parentStore.kit(this.Guid.kit);
  }

  protected getSelection(): QualityAppSelection {
    return this.state.selection || {};
  }

  protected inverseSelectionDiff(selection: QualityAppSelection, diff: QualityAppSelectionDiff): QualityAppSelectionDiff {
    return inverseQualityAppSelectionDiff(selection, diff);
  }

  protected applySelectionDiff(selectionDiff: QualityAppSelectionDiff): void {
    const currentSelection = this.state.selection || {};
    const newSelection: QualityAppSelection = { ...currentSelection };

    if (selectionDiff.formulaNodes) {
      const currentNodes = new Set(currentSelection.formulaNodes || []);
      if (selectionDiff.formulaNodes.added) {
        selectionDiff.formulaNodes.added.forEach((n) => currentNodes.add(n));
      }
      if (selectionDiff.formulaNodes.removed) {
        selectionDiff.formulaNodes.removed.forEach((n) => currentNodes.delete(n));
      }
      newSelection.formulaNodes = currentNodes.size > 0 ? Array.from(currentNodes) : undefined;
    }

    this.state = { ...this.state, selection: newSelection };
    this.notify();
  }

  change(diff: QualityAppDiff): void {
    const newState = { ...this.state };

    if (diff.fullscreenWindow !== undefined) newState.fullscreenWindow = diff.fullscreenWindow;
    if (diff.activeTool !== undefined) newState.activeTool = diff.activeTool;
    if (diff.panelVisibility !== undefined) {
      newState.panelVisibility = { ...newState.panelVisibility, ...diff.panelVisibility };
    }
    if (diff.selection) {
      this.applySelectionDiff(diff.selection);
      return;
    }
    if (diff.hover !== undefined) {
      newState.hover = Object.keys(diff.hover).length === 0 ? undefined : diff.hover;
    }
    if (diff.formulaNodes !== undefined) {
      newState.formulaNodes = diff.formulaNodes;
    }
    if (diff.windowLayout !== undefined) {
      newState.windowLayout = diff.windowLayout;
    }

    this.state = newState;
    this.notify();
  }

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
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.qualityApp.finalizeTransaction") {
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.qualityApp.abortTransaction") {
      this.abortTransaction();
      return {} as T;
    }
    if (command === "semio.qualityApp.undo") {
      this.undo();
      return {} as T;
    }
    if (command === "semio.qualityApp.redo") {
      this.redo();
      return {} as T;
    }

    const callback = this.commandRegistry.get(command);
    if (!callback) {
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
    const qualityKitDiff = result.qualityDiff ? {
      qualities: {
        updated: [{ quality: { guid: this.Guid.quality }, diff: result.qualityDiff }],
      },
    } : undefined;
    if (qualityKitDiff) (result as any).kitDiff = qualityKitDiff;
    this.recordEdit(result);
    if (qualityKitDiff) {
      kitStore.change(qualityKitDiff);
    }
    return result as T;
  }
}

if (typeof window !== "undefined") {
  registerQualityAppStoreFactory((parent, id) => new QualityAppStore(parent, id));
}

// #region Quality App Plugin Registration

// [👤semio📚js🗃️sketchpad💻qualitytsx🔖store🔖qualityapppluginregistration](semiorepo://section/SEMIO/JS/SKETCHPAD/QUALITY.TSX/STORE/QUALITY-APP-PLUGIN-REGISTRATION)
// Plugin registration and event handler wiring MUST initialize quality app context.

/**
 * qualityAppPlugin holds the data fields for a qualityAppPlugin record.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🔖qualityapppluginregistration🪨qualityappplugin](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/s/Quality%20App%20Plugin%20Registration/d/i/qualityAppPlugin)
 **/
const qualityAppPlugin: AppPlugin = {
  id: "quality",
  namespace: "QUALITY",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: (): QualityAppState => ({
      fullscreenWindow: QualityAppFullscreenWindow.None,
      panelVisibility: { toolbar: true, leftSidePanel: true, rightSidePanel: true, details: true },
      activeTool: ToolKind.SELECTION_NORMAL,
      selection: undefined,
      hover: undefined,
      formulaNodes: [],
      windowLayout: undefined,
    }),
  },
  registerStores: () => { },
};

if (typeof window !== "undefined") {
  registerAppPlugin(qualityAppPlugin);
  registerEventHandler("QUALITY.TOGGLE_PANEL", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.qualityGuid}`;
      const app = context.qualityApps[key] || createDefaultQualityAppState();
      return { qualityApps: { ...context.qualityApps, [key]: { ...app, panelVisibility: { ...app.panelVisibility, [event.panel]: !app.panelVisibility[event.panel] } } } };
    },
  });
  registerEventHandler("QUALITY.TOGGLE_BENCHMARK", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.qualityGuid}`;
      const app = context.qualityApps[key] || createDefaultQualityAppState();
      const expanded = new Set(app.expandedBenchmarks);
      if (expanded.has(event.benchmarkGuid)) expanded.delete(event.benchmarkGuid);
      else expanded.add(event.benchmarkGuid);
      return { qualityApps: { ...context.qualityApps, [key]: { ...app, expandedBenchmarks: expanded } } };
    },
  });
}

// #endregion Quality App Plugin Registration

/**
 * QualityAppScope holds the data fields for a QualityAppScope record.
 * [👤semio📚js🗃️sketchpad💻quality🔖store✂️qualityappscope](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/QualityAppScope)
 **/
type QualityAppScope = { guid: string };
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖store🪨qualityappscopecontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/QualityAppScopeContext)
 * QualityAppScopeContext holds the data fields for a QualityAppScopeContext record.
 **/
const QualityAppScopeContext = createContext<QualityAppScope | null>(null);
/**
 * React context provider scoping quality app state by GUID.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🪨qualityappscopeprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/QualityAppScopeProvider)
 **/
export const QualityAppScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(QualityAppScopeContext.Provider, { value }, props.children as any);
};
/** useQualityAppScope holds the data fields for a useQualityAppScope record.
 **/
// [👤semio📚js🗃️sketchpad💻quality🔖store🪨usequalityappscope](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppScope)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖store🪨usequalityappscope](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppScope)
 **/
const useQualityAppScope = () => useContext(QualityAppScopeContext);

/**
 * Returns the quality app store instance, optionally applying a selector.
 *The hook MUST be called within a KitScopeProvider and QualityScopeProvider.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityappstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppStore)
 **/
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

/**
 * Returns reactive quality app state, optionally applying a selector.
 *The hook MUST be called within a quality app scope.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityapp](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityApp)
 **/
export function useQualityApp<T>(selector?: (state: QualityAppState) => T, id?: QualityAppId): T | QualityAppState | null {
  const store = useQualityAppStore(identitySelector, id);
  if (!store) return null;
  const selectedSelector = selector || identitySelector;
  return useSyncDeep<QualityAppState>(store as any, selectedSelector as (value: QualityAppState) => QualityAppState);
}

/**
 * Returns command handlers for the quality app.
 *Functions MUST be called with an origin string for tracking.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityappcommands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppCommands)
 **/
export function useQualityAppCommands(id?: QualityAppId) {
  const store = useQualityAppStore(undefined, id) as QualityAppStore | null;
  if (!store) {
    return {
      startTransaction: () => { },
      finalizeTransaction: () => { },
      abortTransaction: () => { },
      undo: () => { },
      redo: () => { },
      toggleFormulaFullscreen: () => Promise.resolve(),
      toggleDiagramFullscreen: () => Promise.resolve(),
      setActiveTool: (_origin: string, _tool: ToolKind) => Promise.resolve(),
      updateFormula: (_origin: string, _formula: string) => Promise.resolve(),
      addFormulaNode: (_origin: string, _node: FormulaNode) => Promise.resolve(),
      removeFormulaNode: (_origin: string, _nodeId: Guid) => Promise.resolve(),
      selectFormulaNode: (_origin: string, _nodeId: Guid) => Promise.resolve(),
      deselectAll: () => Promise.resolve(),
      hoverFormulaNode: (_origin: string, _nodeId: Guid) => Promise.resolve(),
      clearHover: () => Promise.resolve(),
      connectNodes: (_origin: string, _sourceId: Guid, _targetId: Guid) => Promise.resolve(),
      togglePanel: (_origin: string, _panelKey: keyof PanelVisibility) => { },
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
    setActiveTool: (origin: string, tool: ToolKind) => store.execute("semio.qualityApp.setActiveTool", origin, tool),
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

/**
 * Returns the fullscreen window state with a setter.
 *The setter MUST receive a valid QualityAppFullscreenWindow value.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityappfullscreen](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppFullscreen)
 **/
export function useQualityAppFullscreen(): HookResult<QualityAppFullscreenWindow> {
  const qualityScope = useQualityScope();
  const store = useQualityAppStore() as QualityAppStore | null;
  const fullscreen = useQualityApp((s) => s.fullscreenWindow) as QualityAppFullscreenWindow;
  const canSet = qualityScope !== null && store !== null;
  const setFullscreen = useCallback(
    (value: QualityAppFullscreenWindow) => {
      if (store) store.execute("semio.qualityApp.setFullscreen", value);
    },
    [store],
  );
  return [fullscreen ?? QualityAppFullscreenWindow.None, setFullscreen, canSet];
}

/**
 * Returns the current selection state with a setter.
 *The setter MUST receive a valid QualityAppSelection object.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityappselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppSelection)
 **/
export function useQualityAppSelection(): HookResult<QualityAppSelection> {
  const qualityScope = useQualityScope();
  const store = useQualityAppStore() as QualityAppStore | null;
  const selection = useQualityApp((s) => s.selection) as QualityAppSelection | undefined;
  const canSet = qualityScope !== null && store !== null;
  const setSelection = useCallback(
    (value: QualityAppSelection) => {
      if (store) store.execute("semio.qualityApp.setSelection", value);
    },
    [store],
  );
  return [selection ?? {}, setSelection, canSet];
}

/**
 * Returns the hover state with a setter.
 *The setter MUST receive a QualityAppHover or undefined to clear.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityapphover](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppHover)
 **/
export function useQualityAppHover(): HookResult<QualityAppHover | undefined> {
  const qualityScope = useQualityScope();
  const store = useQualityAppStore() as QualityAppStore | null;
  const hover = useQualityApp((s) => s.hover) as QualityAppHover | undefined;
  const canSet = qualityScope !== null && store !== null;
  const setHover = useCallback(
    (value: QualityAppHover | undefined) => {
      if (store) store.execute("semio.qualityApp.setHover", value);
    },
    [store],
  );
  return [hover, setHover, canSet];
}

/**
 * Returns the active tool kind with a setter.
 *The setter MUST receive a valid ToolKind value.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityappactivetool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppActiveTool)
 **/
export function useQualityAppActiveTool(): HookResult<ToolKind> {
  const qualityScope = useQualityScope();
  const store = useQualityAppStore() as QualityAppStore | null;
  const activeTool = useQualityApp((s) => s.activeTool) as ToolKind;
  const canSet = qualityScope !== null && store !== null;
  const setActiveTool = useCallback(
    (value: ToolKind) => {
      if (store) store.execute("semio.qualityApp.setActiveTool", value);
    },
    [store],
  );
  return [activeTool ?? ToolKind.SELECTION_NORMAL, setActiveTool, canSet];
}

/**
 * Returns the current formula nodes as a read-only hook result.
 *The hook MUST be called within a quality app scope.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityappformulanodes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppFormulaNodes)
 **/
export function useQualityAppFormulaNodes(): HookNoSetResult<FormulaNode[]> {
  const qualityScope = useQualityScope();
  const formulaNodes = useQualityApp((s) => s.formulaNodes) as FormulaNode[];
  const canRead = qualityScope !== null;
  return [formulaNodes ?? [], undefined, canRead];
}

/**
 * Returns the panel visibility state with a setter.
 *The setter MUST receive a complete PanelVisibility object.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityapppanelvisibility](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppPanelVisibility)
 **/
export function useQualityAppPanelVisibility(): HookResult<PanelVisibility> {
  const qualityScope = useQualityScope();
  const store = useQualityAppStore() as QualityAppStore | null;
  const panelVisibility = useQualityApp((s) => s.panelVisibility) as PanelVisibility;
  const canSet = qualityScope !== null && store !== null;
  const setPanelVisibility = useCallback(
    (value: PanelVisibility) => {
      if (store) store.change({ panelVisibility: value });
    },
    [store],
  );
  return [panelVisibility ?? { toolbar: true, details: false }, setPanelVisibility, canSet];
}

/**
 * Returns the window layout state with a setter.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🛠️usequalityappwindowlayout](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/d/i/useQualityAppWindowLayout)
 **/
export function useQualityAppWindowLayout(): HookResult<any> {
  const qualityScope = useQualityScope();
  const store = useQualityAppStore() as QualityAppStore | null;
  const windowLayout = useQualityApp((s) => s.windowLayout);
  const canSet = qualityScope !== null && store !== null;
  const setWindowLayout = useCallback(
    (value: any) => {
      if (store) store.change({ windowLayout: value });
    },
    [store],
  );
  return [windowLayout, setWindowLayout, canSet];
}

//#region Action Hooks

// [👤semio📚js🗃️sketchpad💻qualitytsx🔖store🔖actionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/QUALITY.TSX/STORE/ACTION-HOOKS)
// Memoized action hooks MUST provide formula node interaction callbacks.

/**
 * Result tuple from an action hook with optional action callback and availability flag.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🔖actionhooks🛠️actionhookresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/s/Action%20Hooks/d/i/ActionHookResult)
 **/
export type ActionHookResult<TArgs extends any[]> = readonly [action: ((...args: TArgs) => void) | undefined, canAct: boolean];

/**
 * Action hook to select a formula node by ID.
 *The nodeId MUST reference an existing formula node.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🔖actionhooks🛠️usequalityappselectformulanode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/s/Action%20Hooks/d/i/useQualityAppSelectFormulaNode)
 **/
export function useQualityAppSelectFormulaNode(): ActionHookResult<[nodeId: string]> {
  const [, setSelection, canSetSelection] = useQualityAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (nodeId: string) => setSelection({ formulaNodes: [nodeId] });
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

/**
 * Action hook to hover a formula node by ID.
 *The nodeId MUST reference an existing formula node.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🔖actionhooks🛠️usequalityapphoverformulanode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/s/Action%20Hooks/d/i/useQualityAppHoverFormulaNode)
 **/
export function useQualityAppHoverFormulaNode(): ActionHookResult<[nodeId: string]> {
  const [, setHover, canSetHover] = useQualityAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (nodeId: string) => setHover({ formulaNode: nodeId });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Action hook to clear the current hover state.
 *The action MUST reset hover to undefined.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🔖actionhooks🛠️usequalityappclearhover](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/s/Action%20Hooks/d/i/useQualityAppClearHover)
 **/
export function useQualityAppClearHover(): ActionHookResult<[]> {
  const [, setHover, canSetHover] = useQualityAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return () => setHover(undefined);
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Action hook to deselect all formula nodes.
 *The action MUST clear the entire selection.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🔖actionhooks🛠️usequalityappdeselectall](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/s/Action%20Hooks/d/i/useQualityAppDeselectAll)
 **/
export function useQualityAppDeselectAll(): ActionHookResult<[]> {
  const [, setSelection, canSetSelection] = useQualityAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return () => setSelection({});
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

/**
 * Action hook to toggle a panel's visibility.
 *The panelKey MUST be a valid PanelVisibility key.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🔖actionhooks🛠️usequalityapptogglepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/s/Action%20Hooks/d/i/useQualityAppTogglePanel)
 **/
export function useQualityAppTogglePanel(): ActionHookResult<[panelKey: keyof PanelVisibility]> {
  const [panelVisibility, setPanelVisibility, canSetPanelVisibility] = useQualityAppPanelVisibility();
  const action = useMemo(() => {
    if (!canSetPanelVisibility || !setPanelVisibility) return undefined;
    return (panelKey: keyof PanelVisibility) => {
      setPanelVisibility({ ...panelVisibility, [panelKey]: !panelVisibility[panelKey] });
    };
  }, [setPanelVisibility, canSetPanelVisibility, panelVisibility]);
  return [action, canSetPanelVisibility];
}

/**
 * Action hook to toggle the formula window fullscreen state.
 *The action MUST toggle between Formula and None fullscreen modes.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🔖actionhooks🛠️usequalityapptoggleformulafullscreen](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/s/Action%20Hooks/d/i/useQualityAppToggleFormulaFullscreen)
 **/
export function useQualityAppToggleFormulaFullscreen(): ActionHookResult<[]> {
  const [fullscreen, setFullscreen, canSetFullscreen] = useQualityAppFullscreen();
  const action = useMemo(() => {
    if (!canSetFullscreen || !setFullscreen) return undefined;
    return () => setFullscreen(fullscreen === QualityAppFullscreenWindow.Formula ? QualityAppFullscreenWindow.None : QualityAppFullscreenWindow.Formula);
  }, [setFullscreen, canSetFullscreen, fullscreen]);
  return [action, canSetFullscreen];
}

/**
 * Action hook to toggle the diagram window fullscreen state.
 * [👤semio📚js🗃️sketchpad💻quality🔖store🔖actionhooks🛠️usequalityapptogglediagramfullscreen](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Store/s/Action%20Hooks/d/i/useQualityAppToggleDiagramFullscreen)
 **/
export function useQualityAppToggleDiagramFullscreen(): ActionHookResult<[]> {
  const [fullscreen, setFullscreen, canSetFullscreen] = useQualityAppFullscreen();
  const action = useMemo(() => {
    if (!canSetFullscreen || !setFullscreen) return undefined;
    return () => setFullscreen(fullscreen === QualityAppFullscreenWindow.Diagram ? QualityAppFullscreenWindow.None : QualityAppFullscreenWindow.Diagram);
  }, [setFullscreen, canSetFullscreen, fullscreen]);
  return [action, canSetFullscreen];
}

//#endregion Action Hooks

// #endregion Store

// #region Components

// [👤semio📚js🗃️sketchpad💻qualitytsx🔖components](semiorepo://section/SEMIO/JS/SKETCHPAD/QUALITY.TSX/COMPONENTS)
// React components MUST render the quality app formula diagram, details panel, and workbench.

declare global {
  interface Window {
    MathJax?: any;
  }
}

/** FunctionNodeComponent holds the data fields for a FunctionNodeComponent record.
 **/
// [👤semio📚js🗃️sketchpad💻quality🔖components🪨functionnodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/FunctionNodeComponent)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨functionnodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/FunctionNodeComponent)
 **/
const FunctionNodeComponent: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const initials = data.label.substring(0, 2).toUpperCase();
  return <DiagramNode content={initials} selected={selected} showTopHandle showBottomHandle />;
};

// [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualitynodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityNodeComponent)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualitynodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityNodeComponent)
 * QualityNodeComponent holds the data fields for a QualityNodeComponent record.
 **/
const QualityNodeComponent: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const initials = data.label
    .split(".")
    .map((part: string) => part[0])
    .join("")
    .substring(0, 2)
    .toUpperCase();
  return <DiagramNode content={initials} selected={selected} showTopHandle showBottomHandle />;
};

// [👤semio📚js🗃️sketchpad💻quality🔖components🪨variablenodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/VariableNodeComponent)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨variablenodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/VariableNodeComponent)
 * VariableNodeComponent holds the data fields for a VariableNodeComponent record.
 **/
const VariableNodeComponent: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const varName = data.label.startsWith("$") ? data.label.substring(1) : data.label;
  const initials = varName.substring(0, 2).toUpperCase();
  return <DiagramNode content={initials} selected={selected} showTopHandle showBottomHandle />;
};

// [👤semio📚js🗃️sketchpad💻quality🔖components🪨valuenodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/ValueNodeComponent)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨valuenodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/ValueNodeComponent)
 * ValueNodeComponent holds the data fields for a ValueNodeComponent record.
 **/
const ValueNodeComponent: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const display = data.label.length > 4 ? data.label.substring(0, 4) : data.label;
  return <DiagramNode content={display} selected={selected} showTopHandle />;
};

// [👤semio📚js🗃️sketchpad💻quality🔖components🪨placeholdernodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/PlaceholderNodeComponent)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨placeholdernodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/PlaceholderNodeComponent)
 * PlaceholderNodeComponent holds the data fields for a PlaceholderNodeComponent record.
 **/
const PlaceholderNodeComponent: FC<{ data: any }> = ({ data }) => {
  return <PlaceholderDiagramNode id={data.id} />;
};

/**
 * nodeTypes holds the data fields for a nodeTypes record.
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨nodetypes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/nodeTypes)
 **/
const nodeTypes: NodeTypes = {
  function: FunctionNodeComponent,
  quality: QualityNodeComponent,
  variable: VariableNodeComponent,
  unit: ValueNodeComponent,
  value: ValueNodeComponent,
  placeholder: PlaceholderNodeComponent,
};

/**
 * QualityDiagramProps holds the data fields for a QualityDiagramProps record.
 * [👤semio📚js🗃️sketchpad💻quality🔖components✂️qualitydiagramprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityDiagramProps)
 **/
interface QualityDiagramProps {
  reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null>;
}

/** QualityDiagram holds the data fields for a QualityDiagram record.
 **/
// [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualitydiagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityDiagram)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualitydiagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityDiagram)
 **/
const QualityDiagram: FC<QualityDiagramProps> = ({ reactFlowInstanceRef }) => {
  const [formulaNodes] = useQualityAppFormulaNodes();
  const [selection, setSelection] = useQualityAppSelection();
  const [activeTool] = useQualityAppActiveTool();
  const [hoverFormulaNode] = useQualityAppHoverFormulaNode();
  const [clearHover] = useQualityAppClearHover();
  const { connectNodes } = useQualityAppCommands();
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
        type: node.kind,
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

      if (node.kind === "function") {
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
        onNodeClick={(e: React.MouseEvent, node: any) => {
          if (!setSelection) return;
          const compositionKind = resolveSelectionCompositionKind(activeTool, {
            shiftKey: e.shiftKey,
            altKey: e.altKey,
            ctrlKey: e.ctrlKey,
            metaKey: e.metaKey,
          });
          const currentNodes = selection?.formulaNodes || [];
          const newNodes = applySelectionComposition(currentNodes, [node.id], compositionKind);
          setSelection({ formulaNodes: newNodes });
        }}
        onNodeMouseEnter={(_: React.MouseEvent, node: any) => hoverFormulaNode && hoverFormulaNode(node.id)}
        onNodeMouseLeave={() => clearHover && clearHover()}
        reactFlowInstanceRef={reactFlowInstanceRef}
      />
    </div>
  );
};

/** Formula holds the data fields for a Formula record.
 **/
// [👤semio📚js🗃️sketchpad💻quality🔖components🪨formula](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/Formula)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨formula](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/Formula)
 **/
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
          window.MathJax.typesetPromise([mathRef.current]).catch(() => { });
        }
        return;
      }
      const script = document.createElement("script");
      script.src = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js";
      script.async = true;
      script.onload = () => {
        if (window.MathJax && mathRef.current) {
          window.MathJax.typesetPromise([mathRef.current]).catch(() => { });
        }
      };
      script.onerror = () => { };
      document.head.appendChild(script);
    };
    loadMathJax();
  }, []);

  useEffect(() => {
    if (window.MathJax && mathRef.current) {
      mathRef.current.innerHTML = "";
      const latex = formulaToLatexString(quality?.formula);
      mathRef.current.textContent = `\\[${latex}\\]`;
      window.MathJax.typesetPromise([mathRef.current]).catch(() => { });
    }
  }, [quality?.formula, formulaToLatexString]);

  return (
    <div className="h-full w-full border-b border-foreground flex items-center justify-center overflow-auto">
      <div ref={mathRef} className="text-foreground p-4" style={{ fontSize: "var(--size-medium)" }}></div>
    </div>
  );
};

/**
 * Detail panel component displaying quality property fields.
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualitydetails](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityDetails)
 **/
export const QualityDetails: FC = () => {
  const quality = useQuality(undefined, undefined, true) as Quality | undefined;
  const { updateFormula } = useQualityAppCommands();

  if (!quality) return null;

  return (
    <>
      <TreeRow id="semio.sketchpad.app.quality.key">
        <Input id="semio.sketchpad.app.quality.panel.details.key" value={quality.key ?? ""} readOnly className="w-full" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.name">
        <Input id="semio.sketchpad.app.quality.panel.details.name" value={quality.name ?? ""} readOnly className="w-full" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.description">
        <Textarea id="semio.sketchpad.app.quality.panel.details.description" value={quality.description ?? ""} readOnly className="w-full" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.formula">
        <Textarea
          id="semio.sketchpad.app.quality.panel.details.formula"
          value={quality.formula ?? ""}
          onChange={(e) => updateFormula("semio.sketchpad.app.quality.panel.details.formula", e.target.value)}
          className="w-full font-mono text-xs"
          rows={5}
          placeholderId="semio.sketchpad.app.quality.formulaPlaceholder"
          showLabel
        />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.defaultSiUnit">
        <Input id="semio.sketchpad.app.quality.panel.details.defaultSiUnit" value={quality.defaultSiUnit ?? ""} readOnly className="w-full" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.defaultImperialUnit">
        <Input id="semio.sketchpad.app.quality.panel.details.defaultImperialUnit" value={quality.defaultImperialUnit ?? ""} readOnly className="w-full" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.kind">
        <Input id="semio.sketchpad.app.quality.panel.details.kind" type="number" value={quality.kind?.toString() ?? ""} readOnly className="w-full" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.canScale">
        <Input id="semio.sketchpad.app.quality.panel.details.canScale" type="checkbox" checked={quality.canScale ?? false} disabled className="size-tiny" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.defaultValue">
        <Input id="semio.sketchpad.app.quality.panel.details.defaultValue" type="number" value={quality.defaultValue?.toString() ?? ""} readOnly className="w-full" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.min">
        <Input id="semio.sketchpad.app.quality.panel.details.min" type="number" value={quality.min?.toString() ?? ""} readOnly className="w-full" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.isMinExcluded">
        <Input id="semio.sketchpad.app.quality.panel.details.isMinExcluded" type="checkbox" checked={quality.isMinExcluded ?? false} disabled className="size-tiny" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.max">
        <Input id="semio.sketchpad.app.quality.panel.details.max" type="number" value={quality.max?.toString() ?? ""} readOnly className="w-full" showLabel />
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.isMaxExcluded">
        <Input id="semio.sketchpad.app.quality.panel.details.isMaxExcluded" type="checkbox" checked={quality.isMaxExcluded ?? false} disabled className="size-tiny" showLabel />
      </TreeRow>
    </>
  );
};

/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components✂️functionnodeprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/FunctionNodeProps)
 * FunctionNodeProps holds the data fields for a FunctionNodeProps record.
 **/
interface FunctionNodeProps {
  name: string;
  kind: "function" | "quality" | "variable" | "unit" | "value";
  label: string;
}

// [👤semio📚js🗃️sketchpad💻quality🔖components🪨functionnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/FunctionNode)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨functionnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/FunctionNode)
 * FunctionNode holds the data fields for a FunctionNode record.
 **/
const FunctionNode: FC<FunctionNodeProps> = ({ name, kind, label }) => {
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();
  const interactionId = `formula-${kind}-${name}`;

  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { name, kind },
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

/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components✂️qualityavatarprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityAvatarProps)
 * QualityAvatarProps holds the data fields for a QualityAvatarProps record.
 **/
interface QualityAvatarProps {
  qualityId?: Guid;
  quality?: Quality;
  showHoverCard?: boolean;
}

/**
 * Draggable avatar component for a quality with optional hover card.
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualityavatar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityAvatar)
 **/
export const QualityAvatar: FC<QualityAvatarProps> = ({ qualityId, quality: qualityProp, showHoverCard = false }) => {
  const qualityFromStore = qualityId && !qualityProp ? (useQuality(undefined, qualityId) as Quality | null) : null;
  const quality = qualityProp || qualityFromStore;
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();

  const interactionId = quality ? `quality-${quality.key}` : "quality-unknown";
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { quality, kind: "quality" },
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

/**
 * Workbench panel component listing formula function nodes by category.
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualityworkbench](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityWorkbench)
 **/
export const QualityWorkbench: FC = () => {
  const { t } = useTranslation();
  const kit = useKit(undefined, undefined, true) as Kit | null;
  const qualities = kit?.qualities || [];

  return (
    <>
      <TreeRow id="semio.sketchpad.app.quality.numericFunctions">
        <div className="flex flex-wrap gap-single p-single">
          <FunctionNode name="Add" kind="function" label={useLabel("semio.sketchpad.app.quality.add") ?? ""} />
          <FunctionNode name="Subtract" kind="function" label={useLabel("semio.sketchpad.app.quality.subtract") ?? ""} />
          <FunctionNode name="Multiply" kind="function" label={useLabel("semio.sketchpad.app.quality.multiply") ?? ""} />
          <FunctionNode name="Divide" kind="function" label={useLabel("semio.sketchpad.app.quality.divide") ?? ""} />
        </div>
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.branchingFunctions">
        <div className="flex flex-wrap gap-single p-single">
          <FunctionNode name="If" kind="function" label={useLabel("semio.sketchpad.app.quality.if") ?? ""} />
          <FunctionNode name="Switch" kind="function" label={useLabel("semio.sketchpad.app.quality.switch") ?? ""} />
        </div>
      </TreeRow>
      <TreeRow id="semio.sketchpad.app.quality.dataStructures">
        <div className="flex flex-wrap gap-single p-single">
          <FunctionNode name="List" kind="function" label={useLabel("semio.sketchpad.app.quality.list") ?? ""} />
          <FunctionNode name="Dictionary" kind="function" label={useLabel("semio.sketchpad.app.quality.dictionary") ?? ""} />
        </div>
      </TreeRow>
    </>
  );
};

/**
 * QualityTreeNode holds the data fields for a QualityTreeNode record.
 * [👤semio📚js🗃️sketchpad💻quality🔖components✂️qualitytreenode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityTreeNode)
 **/
interface QualityTreeNode {
  key: string;
  qualities: Quality[];
  children: Map<string, QualityTreeNode>;
}

// [👤semio📚js🗃️sketchpad💻quality🔖components🪨buildqualitytree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/buildQualityTree)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨buildqualitytree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/buildQualityTree)
 * buildQualityTree holds the data fields for a buildQualityTree record.
 **/
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

// [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualitytree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityTree)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualitytree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityTree)
 * QualityTree holds the data fields for a QualityTree record.
 **/
const QualityTree: FC<{ qualities: Quality[] }> = ({ qualities }) => {
  const tree = buildQualityTree(qualities);

  const renderNode = (key: string, node: QualityTreeNode, level: number = 0) => {
    const hasChildren = node.children.size > 0;
    const hasQualities = node.qualities.length > 0;

    if (hasChildren) {
      return (
        <TreeItem key={key} label={key}>
          <TreeContent>
            {node.qualities.length > 0 && (
              <div className="flex flex-wrap gap-single p-single">
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
          <div className="flex flex-wrap gap-single p-single">
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

// [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualityworkbenchqualities](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityWorkbenchQualities)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖components🪨qualityworkbenchqualities](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Components/d/i/QualityWorkbenchQualities)
 * QualityWorkbenchQualities holds the data fields for a QualityWorkbenchQualities record.
 **/
const QualityWorkbenchQualities: FC = () => {
  const { t } = useTranslation();
  const kit = useKit(undefined, undefined, true) as Kit | null;
  const qualities = kit?.qualities || [];

  if (qualities.length === 0) {
    return (
      <TreeContent>
        <div className="text-sm text-muted-foreground p-single">{useLabel("semio.sketchpad.app.quality.noQualities")}</div>
      </TreeContent>
    );
  }

  return <QualityTree qualities={qualities} />;
};

/**
 * Settings component for the selection tool group with mode toggles.
 *MUST render toggle buttons for each selection sub-mode.
 * [👤semio📚js🗃️sketchpad💻qualitytsx🔖components🪨qualityselectsettings](semiorepo://definition/SEMIO/JS/SKETCHPAD/QUALITY.TSX/COMPONENTS/QUALITY-SELECT-SETTINGS)
 **/
export const QualitySelectSettings: FC = () => {
  const [activeTool, setActiveTool] = useQualityAppActiveTool();
  const additiveLabel = useLabel("semio.sketchpad.app.quality.tools.select.additive");
  const subtractiveLabel = useLabel("semio.sketchpad.app.quality.tools.select.subtractive");
  const intersectLabel = useLabel("semio.sketchpad.app.quality.tools.select.intersect");
  return (
    <div className="flex shrink-0 items-center gap-single h-full px-single">
      <Toggle
        id="semio.sketchpad.app.quality.tools.select.additive"
        icon={<AddIcon className="size-tiny" />}
        text={additiveLabel}
        pressed={activeTool === ToolKind.SELECTION_ADDITIVE}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_ADDITIVE : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.quality.tools.select.subtractive"
        icon={<RemoveIcon className="size-tiny" />}
        text={subtractiveLabel}
        pressed={activeTool === ToolKind.SELECTION_SUBTRACTIVE}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_SUBTRACTIVE : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.quality.tools.select.intersect"
        icon={<IntersectIcon className="size-tiny" />}
        text={intersectLabel}
        pressed={activeTool === ToolKind.SELECTION_INTERSECT}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_INTERSECT : ToolKind.SELECTION_NORMAL)}
      />
    </div>
  );
};

// #endregion Components

// #region Settings

const QualitySettingsContent: FC = () => {
  const [theme, setTheme, canSetTheme] = useTheme();
  const [language, setLanguage, canSetLanguage] = useLanguage();
  const [device, setDevice, canSetDevice] = useDevice();
  const [expertise, setExpertise, canSetExpertise] = useExpertise();
  const [mode, setMode, canSetMode] = useMode();
  const languageEnLabel = useLabel("semio.sketchpad.settings.language.en");
  const languageDeLabel = useLabel("semio.sketchpad.settings.language.de");
  const languagePlaceholder = useLabel("semio.sketchpad.app.home.settings.language.placeholder");
  return (
    <>
      <TreeRow>
        <ToggleGroup
          id="semio.sketchpad.settings.theme"
          value={theme}
          onValueChange={(value: string) => setTheme?.(value as Theme)}
          showLabel
          kind="single"
          disabled={!canSetTheme}
          items={[
            { value: Theme.SYSTEM, id: "semio.sketchpad.settings.theme.system", icon: <MonitorIcon className="size-small" /> },
            { value: Theme.LIGHT, id: "semio.sketchpad.settings.theme.light", icon: <SunIcon className="size-small" /> },
            { value: Theme.DARK, id: "semio.sketchpad.settings.theme.dark", icon: <MoonIcon className="size-small" /> },
          ]}
        />
      </TreeRow>
      <TreeRow>
        <Select id="semio.sketchpad.settings.language" value={language || "en"} onValueChange={(value: string) => setLanguage?.(value)} showLabel disabled={!canSetLanguage}>
          <SelectTrigger>
            <SelectValue placeholder={languagePlaceholder} />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="en">{languageEnLabel}</SelectItem>
            <SelectItem value="de">{languageDeLabel}</SelectItem>
          </SelectContent>
        </Select>
      </TreeRow>
      <TreeRow>
        <ToggleGroup
          id="semio.sketchpad.settings.device"
          value={typeof device === "object" ? "desktop" : device}
          onValueChange={(value: string) => setDevice?.(value as "desktop" | "tablet")}
          showLabel
          kind="single"
          disabled={!canSetDevice}
          items={[
            { value: "desktop", id: "semio.sketchpad.settings.device.desktop", icon: <MousePointerIcon className="size-small" /> },
            { value: "tablet", id: "semio.sketchpad.settings.device.tablet", icon: <HandIcon className="size-small" /> },
          ]}
        />
      </TreeRow>
      <TreeRow>
        <ToggleGroup
          id="semio.sketchpad.settings.expertise"
          value={expertise}
          onValueChange={(value: string) => setExpertise?.(value as Expertise)}
          showLabel
          kind="single"
          disabled={!canSetExpertise}
          items={[
            { value: Expertise.BEGINNER, id: "semio.sketchpad.settings.expertise.beginner", icon: <TutorialIcon className="size-small" /> },
            { value: Expertise.NORMAL, id: "semio.sketchpad.settings.expertise.normal", icon: <UserIcon className="size-small" /> },
            { value: Expertise.EXPERT, id: "semio.sketchpad.settings.expertise.expert", icon: <AwardIcon className="size-small" /> },
          ]}
        />
      </TreeRow>
      <TreeRow>
        <ToggleGroup
          id="semio.sketchpad.settings.mode"
          value={mode}
          onValueChange={(value: string) => setMode?.(value as Mode)}
          showLabel
          kind="single"
          disabled={!canSetMode}
          items={[
            { value: Mode.USER, id: "semio.sketchpad.settings.mode.user", icon: <UserIcon className="size-small" /> },
            { value: Mode.DEV, id: "semio.sketchpad.settings.mode.dev", icon: <CodeIcon className="size-small" /> },
          ]}
        />
      </TreeRow>
    </>
  );
};

// #endregion Settings

// #region App

export interface AppProps { }

const FormulaWindow = memo(() => <Formula />);
FormulaWindow.displayName = "FormulaWindow";

/**
 * [👤semio📚js🗃️sketchpad💻quality🔖app🪨diagramwindow](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/App/d/i/DiagramWindow)
 * DiagramWindow holds the data fields for a DiagramWindow record.
 **/
const DiagramWindow = memo<{ reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null> }>(({ reactFlowInstanceRef }) => <QualityDiagram reactFlowInstanceRef={reactFlowInstanceRef} />);
DiagramWindow.displayName = "DiagramWindow";

// [👤semio📚js🗃️sketchpad💻quality🔖app🪨app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/App/d/i/App)
/**
 * [👤semio📚js🗃️sketchpad💻quality🔖app🪨app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/App/d/i/App)
 * App holds the data fields for a App record.
 **/
const App: FC<AppProps> = () => {
  const [fullscreenWindow] = useQualityAppFullscreen();
  const [deselectAll] = useQualityAppDeselectAll();
  const [toggleFormulaFullscreen] = useQualityAppToggleFormulaFullscreen();
  const [toggleDiagramFullscreen] = useQualityAppToggleDiagramFullscreen();
  const [togglePanel] = useQualityAppTogglePanel();
  const { undo, redo, addFormulaNode, connectNodes, startTransaction, finalizeTransaction } = useQualityAppCommands();
  const quality = useQuality() as Quality | undefined;
  const appType = useAppType();
  const [activeTool, setActiveTool] = useQualityAppActiveTool();

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const sketchpadCommands = useSketchpadCommands();
  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  useHotkeys("ctrl+d", () => deselectAll && deselectAll());
  useHotkeys("ctrl+z", () => undo("semio.sketchpad.app.quality.hotkey"));
  useHotkeys("ctrl+y", () => redo("semio.sketchpad.app.quality.hotkey"));
  useHotkeys("ctrl+shift+z", () => redo("semio.sketchpad.app.quality.hotkey"));

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!setActiveTool || !isSelectionToolKind(activeTool)) return;
      const nextToolKind = toSelectionToolKind(
        resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, {
          shiftKey: e.shiftKey,
          altKey: e.altKey,
          ctrlKey: e.ctrlKey,
          metaKey: e.metaKey,
        }),
      );
      if (nextToolKind !== ToolKind.SELECTION_NORMAL && nextToolKind !== activeTool) setActiveTool(nextToolKind);
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (!setActiveTool || !isSelectionToolKind(activeTool)) return;
      const nextToolKind = toSelectionToolKind(
        resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, {
          shiftKey: e.shiftKey,
          altKey: e.altKey,
          ctrlKey: e.ctrlKey,
          metaKey: e.metaKey,
        }),
      );
      if (nextToolKind === ToolKind.SELECTION_NORMAL && activeTool !== ToolKind.SELECTION_NORMAL) setActiveTool(ToolKind.SELECTION_NORMAL);
      if (nextToolKind !== ToolKind.SELECTION_NORMAL && nextToolKind !== activeTool) setActiveTool(nextToolKind);
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [activeTool, setActiveTool]);

  useEffect(() => {
    if (appType !== "quality") return;

    addSection("details", {
      id: "semio.sketchpad.app.quality.title",
      specificity: 20,
      order: 0,
      content: () => <QualityDetails />,
    });

    return () => {
      removeSection("details", "semio.sketchpad.app.quality.title");
    };
  }, [appType, addSection, removeSection]);

  useEffect(() => {
    if (appType !== "quality") return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.quality.tools.selection",
      specificity: 20,
      order: 0,
      toolbarGroup: {
        id: "selection",
        labelId: "semio.sketchpad.toolbar.parent.selection",
        order: 10,
      },
      content: <QualitySelectSettings />,
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.quality.toolbar.view",
      specificity: 20,
      order: 0,
      toolbarGroup: {
        id: "view",
        labelId: "semio.sketchpad.toolbar.parent.view",
        order: 40,
      },
      content: () => null,
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.quality.toolbar.actions",
      specificity: 20,
      order: 0,
      toolbarGroup: {
        id: "actions",
        labelId: "semio.sketchpad.toolbar.parent.actions",
        order: 50,
      },
      content: () => null,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.quality.tools.selection");
      removeSection("toolbar", "semio.sketchpad.app.quality.toolbar.view");
      removeSection("toolbar", "semio.sketchpad.app.quality.toolbar.actions");
    };
  }, [appType, addSection, removeSection]);

  useEffect(() => {
    if (appType !== "quality") return;
    addSection("workbench", {
      id: "semio.sketchpad.app.quality.workbench.nodes",
      specificity: 20,
      order: 1,
      content: () => <QualityWorkbench />,
    });
    addSection("workbench", {
      id: "semio.sketchpad.app.quality.workbench.qualities",
      specificity: 20,
      order: 2,
      content: () => <QualityWorkbenchQualities />,
    });

    return () => {
      removeSection("workbench", "semio.sketchpad.app.quality.workbench.nodes");
      removeSection("workbench", "semio.sketchpad.app.quality.workbench.qualities");
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
            kind: "quality",
            name: dragData.quality.key,
            x: isPlaceholder ? 0 : x,
            y: isPlaceholder ? 0 : y,
          };
        } else if (dragData.kind && dragData.name) {
          node = {
            id: guid(),
            kind: dragData.kind,
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
  const [windowLayout] = useQualityAppWindowLayout();
  const addSidePanelTab = useAddSidePanelTab();
  const removeSidePanelTab = useRemoveSidePanelTab();
  const migratedWindowLayout = useMemo(() => {
    if (!windowLayout) return windowLayout;
    const removeWorkbenchWindowFromLayout = (layoutNode: any): any => {
      if (!layoutNode || typeof layoutNode !== "object") return layoutNode;
      if (
        layoutNode.type === "component" &&
        (layoutNode.componentType === "workbench" || layoutNode.componentType === "settings" || layoutNode.componentType === "chat")
      ) {
        return null;
      }
      if (Array.isArray(layoutNode.content)) {
        const content = layoutNode.content.map((item: any) => removeWorkbenchWindowFromLayout(item)).filter(Boolean);
        if (content.length === 0 && (layoutNode.type === "stack" || layoutNode.type === "row" || layoutNode.type === "column")) return null;
        return { ...layoutNode, content };
      }
      if (Array.isArray(layoutNode.contentItems)) {
        const contentItems = layoutNode.contentItems.map((item: any) => removeWorkbenchWindowFromLayout(item)).filter(Boolean);
        if (contentItems.length === 0 && (layoutNode.type === "stack" || layoutNode.type === "row" || layoutNode.type === "column")) return null;
        return { ...layoutNode, contentItems };
      }
      return layoutNode;
    };
    return removeWorkbenchWindowFromLayout(windowLayout);
  }, [windowLayout]);

  useEffect(() => {
    if (appType !== "quality") return;
    addSidePanelTab("right", {
      id: "semio.sketchpad.app.quality.settings",
      icon: SettingsIcon,
      order: 100,
      content: () => (
        <TreeStateProvider>
          <Tree className="min-w-0 overflow-hidden p-double">
            <QualitySettingsContent />
          </Tree>
        </TreeStateProvider>
      ),
    });
    addSidePanelTab("right", {
      id: "semio.sketchpad.app.quality.chat",
      icon: ChatIcon,
      order: 101,
      content: () => <BasicChatPanel id="semio.sketchpad.app.quality.chat" title="Quality" />,
    });
    return () => {
      removeSidePanelTab("right", "semio.sketchpad.app.quality.settings");
      removeSidePanelTab("right", "semio.sketchpad.app.quality.chat");
    };
  }, [appType, addSidePanelTab, removeSidePanelTab]);

  const defaultLayout = useMemo(() => {
    return {
      type: "row",
      content: [
        {
          type: "stack",
          width: 25,
          content: [
            {
              type: "component",
              componentType: QualityAppWindowKind.Formula,
              title: "Formula",
            },
          ],
        },
        {
          type: "stack",
          width: 75,
          content: [
            {
              type: "component",
              componentType: QualityAppWindowKind.Diagram,
              title: "Diagram",
            },
          ],
        },
      ],
    };
  }, []);

  const windowConfig: AppWindowConfig = useMemo(() => {
    return {
      windowKinds: [
        {
          id: QualityAppWindowKind.Formula,
          label: "Formula",
          component: (props: any) => <FormulaWindow />,
        },
        {
          id: QualityAppWindowKind.Diagram,
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
      <LayoutCanvas windowConfig={windowConfig} layoutState={migratedWindowLayout} onLayoutChange={handleLayoutChange} />
    </Canvas>
  );
};

export default App;


// #endregion App

// #region Config

// [👤semio📚js🗃️sketchpad💻qualitytsx🔖config](semiorepo://section/SEMIO/JS/SKETCHPAD/QUALITY.TSX/CONFIG)
// Quality app route, panel, and path matching configuration MUST be exported.

/**
 * Quality app configuration for routing, panels, and path matching.
 * [👤semio📚js🗃️sketchpad💻quality🔖config🪨config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Quality.tsx/s/Config/d/i/config)
 **/
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
    createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
  ],
  matchesPath: (pathParts: string[]) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "qualities" && isUuidPattern(pathParts[3]);
  },
  order: 40,
};

// #endregion Config
