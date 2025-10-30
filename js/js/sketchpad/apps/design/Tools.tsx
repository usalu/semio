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

import { FC } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { ToolDefinition, ToolGroup } from "../../Tool";
import { ToolType } from "../../store";
import { useDesignApp, useDesignAppCommands } from "./store";
import { DesignAppTools } from "./tools_registry";

const getDesignTools = (t: (key: string) => string): ToolDefinition[] => [
  {
    id: "selection",
    defaultMode: ToolType.SELECTION_NORMAL,
    modes: DesignAppTools.filter((tool) => tool.id.startsWith("selection")).map((tool) => ({
      id: tool.id,
      label: t(tool.label),
      icon: tool.icon,
      tooltipId: tool.tooltipId,
      hotkey: tool.hotkey,
    })),
  },
  {
    id: "lasso",
    defaultMode: ToolType.LASSO_RECTANGULAR,
    modes: DesignAppTools.filter((tool) => tool.id.startsWith("lasso")).map((tool) => ({
      id: tool.id,
      label: t(tool.label),
      icon: tool.icon,
      tooltipId: tool.tooltipId,
      hotkey: tool.hotkey,
    })),
  },
];

export const ToolsToggleGroup: FC = () => {
  const { t } = useTranslation();
  const { kit, design } = useParams();
  const app = useDesignApp((s) => s, { kit, design });
  const { setActiveTool } = useDesignAppCommands({ kit, design });

  if (!kit || !design || !app) return null;

  const activeTool = app?.activeTool || ToolType.SELECTION_NORMAL;

  const handleToolChange = (toolType: ToolType) => {
    setActiveTool("toolbar", toolType);
  };

  return <ToolGroup tools={getDesignTools(t)} activeTool={activeTool} onToolChange={handleToolChange} level="panel" />;
};
