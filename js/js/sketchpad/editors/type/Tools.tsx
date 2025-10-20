// #region Header

// Tools.tsx

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

import { Crosshair, Minus, MousePointer2, Plus } from "lucide-react";
import { FC } from "react";
import { useParams } from "react-router";
import { ToolDefinition, ToolGroup } from "../../Tool";
import { ToolType } from "../../store";
import { useTypeEditor, useTypeEditorCommands } from "./store";

const TYPE_TOOLS: ToolDefinition[] = [
  {
    id: "selection",
    defaultMode: ToolType.SELECTION_NORMAL,
    modes: [
      {
        id: ToolType.SELECTION_NORMAL,
        label: "Normal",
        icon: <MousePointer2 className="h-4 w-4" />,
        tooltip: "Click to select ports",
        hotkey: "Click",
      },
      {
        id: ToolType.SELECTION_ADDITIVE,
        label: "Additive",
        icon: <Plus className="h-4 w-4" />,
        tooltip: "Add to selection",
        hotkey: "Shift",
      },
      {
        id: ToolType.SELECTION_SUBTRACTIVE,
        label: "Subtractive",
        icon: <Minus className="h-4 w-4" />,
        tooltip: "Remove from selection",
        hotkey: "Ctrl",
      },
    ],
  },
  {
    id: "port",
    defaultMode: ToolType.PORT,
    modes: [
      {
        id: ToolType.PORT,
        label: "Port",
        icon: <Crosshair className="h-4 w-4" />,
        tooltip: "Add and edit ports",
      },
    ],
  },
];

export const ToolsToggleGroup: FC = () => {
  const { kit, type } = useParams();
  const editor = useTypeEditor((s) => s, kit && type ? { kit, type } : undefined);
  const { setActiveTool } = useTypeEditorCommands(kit && type ? { kit, type } : undefined);
  if (!editor) return null;
  const activeTool = editor.activeTool ?? ToolType.PORT;
  return <ToolGroup tools={TYPE_TOOLS} activeTool={activeTool} onToolChange={setActiveTool} level="panel" />;
};
