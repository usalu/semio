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
import { useTypeApp, useTypeAppCommands } from "./store";
import { TypeAppTools } from "./tools_registry";

const getTypeTools = (t: (key: string) => string): ToolDefinition[] => [
  {
    id: "selection",
    defaultMode: ToolType.SELECTION_NORMAL,
    modes: TypeAppTools.filter((tool) => tool.id.startsWith("selection")).map((tool) => ({
      id: tool.id,
      label: t(tool.label),
      icon: tool.icon,
      tooltip: tool.tooltip ? t(tool.tooltip) : undefined,
      hotkey: tool.hotkey,
    })),
  },
  {
    id: "port",
    defaultMode: ToolType.PORT,
    modes: TypeAppTools.filter((tool) => tool.id === ToolType.PORT).map((tool) => ({
      id: tool.id,
      label: t(tool.label),
      icon: tool.icon,
      tooltip: tool.tooltip ? t(tool.tooltip) : undefined,
      hotkey: tool.hotkey,
    })),
  },
];

export const ToolsToggleGroup: FC = () => {
  const { t } = useTranslation();
  const { kit, type } = useParams();
  const app = useTypeApp((s) => s, { kit, type });
  const { setActiveTool } = useTypeAppCommands({ kit, type });

  if (!kit || !type || !app) return null;

  const activeTool = app?.activeTool ?? ToolType.SELECTION_NORMAL;
  return <ToolGroup tools={getTypeTools(t)} activeTool={activeTool} onToolChange={setActiveTool} level="panel" />;
};
