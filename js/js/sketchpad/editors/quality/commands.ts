// #region Header

// commands.ts

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

import { Guid } from "../../../semio";
import { ToolType } from "../../store";
import { FormulaNode, QualityEditorCommandContext, QualityEditorCommandResult, QualityEditorFullscreenWindow } from "./store";

export const commands = {
  "semio.qualityEditor.toggleFormulaFullscreen": (context: QualityEditorCommandContext): QualityEditorCommandResult => {
    const currentPanel = context.qualityEditor.fullscreenWindow;
    const newPanel = currentPanel === QualityEditorFullscreenWindow.Formula ? QualityEditorFullscreenWindow.None : QualityEditorFullscreenWindow.Formula;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.qualityEditor.toggleDiagramFullscreen": (context: QualityEditorCommandContext): QualityEditorCommandResult => {
    const currentPanel = context.qualityEditor.fullscreenWindow;
    const newPanel = currentPanel === QualityEditorFullscreenWindow.Diagram ? QualityEditorFullscreenWindow.None : QualityEditorFullscreenWindow.Diagram;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.qualityEditor.setActiveTool": (context: QualityEditorCommandContext, tool: ToolType): QualityEditorCommandResult => {
    return {
      diff: {
        activeTool: tool,
      },
    };
  },
  "semio.qualityEditor.updateFormula": (context: QualityEditorCommandContext, formula: string): QualityEditorCommandResult => {
    return {
      qualityDiff: {
        formula,
      },
    };
  },
  "semio.qualityEditor.addFormulaNode": (context: QualityEditorCommandContext, node: FormulaNode): QualityEditorCommandResult => {
    const currentNodes = context.qualityEditor.formulaNodes || [];
    return {
      diff: {
        formulaNodes: [...currentNodes, node],
      },
    };
  },
  "semio.qualityEditor.removeFormulaNode": (context: QualityEditorCommandContext, nodeId: Guid): QualityEditorCommandResult => {
    const currentNodes = context.qualityEditor.formulaNodes || [];
    return {
      diff: {
        formulaNodes: currentNodes.filter((n) => n.id !== nodeId),
      },
    };
  },
  "semio.qualityEditor.selectFormulaNode": (context: QualityEditorCommandContext, nodeId: Guid): QualityEditorCommandResult => {
    const currentSelection = context.qualityEditor.selection;
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
  "semio.qualityEditor.deselectAll": (context: QualityEditorCommandContext): QualityEditorCommandResult => {
    const currentSelection = context.qualityEditor.selection;
    return {
      diff: {
        selection: {
          formulaNodes: { removed: currentSelection?.formulaNodes ?? [] },
        },
      },
    };
  },
  "semio.qualityEditor.hoverFormulaNode": (context: QualityEditorCommandContext, nodeId: Guid): QualityEditorCommandResult => {
    return {
      diff: {
        hover: { formulaNode: nodeId },
      },
    };
  },
  "semio.qualityEditor.clearHover": (context: QualityEditorCommandContext): QualityEditorCommandResult => {
    return {
      diff: {
        hover: {},
      },
    };
  },
  "semio.qualityEditor.connectNodes": (context: QualityEditorCommandContext, sourceId: Guid, targetId: Guid): QualityEditorCommandResult => {
    const currentNodes = context.qualityEditor.formulaNodes || [];
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
