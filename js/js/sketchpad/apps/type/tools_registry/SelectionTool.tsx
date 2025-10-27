// #region Header

// SelectionTool.tsx

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

import { Minus, MousePointer2, Plus } from "lucide-react";
import { Tool, ToolRenderContext } from "../../../Tool";
import { ToolType } from "../../../store";
import { TypeAppState } from "../store";

export const SelectionNormalTool: Tool<TypeAppState> = {
  id: ToolType.SELECTION_NORMAL,
  label: "tools.selection.normal",
  icon: <MousePointer2 className="h-4 w-4" />,
  tooltip: "tools.selection.selectPorts",
  hotkey: "Click",
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};

export const SelectionAdditiveTool: Tool<TypeAppState> = {
  id: ToolType.SELECTION_ADDITIVE,
  label: "tools.selection.additive",
  icon: <Plus className="h-4 w-4" />,
  tooltip: "tools.selection.addToSelection",
  hotkey: "Shift",
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};

export const SelectionSubtractiveTool: Tool<TypeAppState> = {
  id: ToolType.SELECTION_SUBTRACTIVE,
  label: "tools.selection.subtractive",
  icon: <Minus className="h-4 w-4" />,
  tooltip: "tools.selection.removeFromSelection",
  hotkey: "Ctrl",
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};
