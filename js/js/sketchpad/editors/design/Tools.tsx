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

import { Lasso, Minus, MousePointer2, Plus, Square } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { ToolDefinition, ToolGroup } from "../../Tool";
import { ToolType } from "../../store";
import { useDesignEditorSafe, useDesignEditorCommands } from "./store";

const getDesignTools = (t: (key: string) => string): ToolDefinition[] => [
  {
    id: "selection",
    defaultMode: ToolType.SELECTION_NORMAL,
    modes: [
      {
        id: ToolType.SELECTION_NORMAL,
        label: t("tools.selection.normal"),
        icon: <MousePointer2 className="h-4 w-4" />,
        tooltip: t("tools.selection.selectPieces"),
        hotkey: "Click",
      },
      {
        id: ToolType.SELECTION_ADDITIVE,
        label: t("tools.selection.additive"),
        icon: <Plus className="h-4 w-4" />,
        tooltip: t("tools.selection.addToSelection"),
        hotkey: "Shift",
      },
      {
        id: ToolType.SELECTION_SUBTRACTIVE,
        label: t("tools.selection.subtractive"),
        icon: <Minus className="h-4 w-4" />,
        tooltip: t("tools.selection.removeFromSelection"),
        hotkey: "Ctrl",
      },
    ],
  },
  {
    id: "lasso",
    defaultMode: ToolType.LASSO_RECTANGULAR,
    modes: [
      {
        id: ToolType.LASSO_RECTANGULAR,
        label: t("tools.lasso.rectangular"),
        icon: <Square className="h-4 w-4" />,
        tooltip: t("tools.lasso.rectangular.extensive"),
      },
      {
        id: ToolType.LASSO_FREEFORM,
        label: t("tools.lasso.freeform"),
        icon: <Lasso className="h-4 w-4" />,
        tooltip: t("tools.lasso.freeform.extensive"),
      },
    ],
  },
];

export const ToolsToggleGroup: FC = () => {
  const { t } = useTranslation();
  const { kit, design } = useParams();
  const editor = useDesignEditorSafe((s) => s, kit && design ? { kit, design } : undefined);
  const { setActiveTool } = useDesignEditorCommands(kit && design ? { kit, design } : undefined);
  const activeTool = editor?.activeTool || ToolType.SELECTION_NORMAL;
  if (!editor) return <></>;
  return <ToolGroup tools={getDesignTools(t)} activeTool={activeTool} onToolChange={setActiveTool} level="panel" />;
};
