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
import { FormulaNode, QualityAppCommandContext, QualityAppCommandResult, QualityAppFullscreenWindow } from "./store";

export const commands = {
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
