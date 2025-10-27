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
import { ToggleGroup, ToggleGroupItem } from "../../../elements/input/ToggleGroup";
import { ToolType } from "../../store";
import { useQualityApp, useQualityAppCommands } from "./store";
import { tools } from "./tools_registry";

export const ToolsToggleGroup: FC = () => {
  const activeTool = useQualityApp((s) => s.activeTool) as ToolType;
  const { setActiveTool } = useQualityAppCommands();

  return (
    <ToggleGroup
      type="single"
      value={activeTool}
      onValueChange={(value: string) => {
        if (value) setActiveTool(value as ToolType);
      }}
    >
      {tools.map((tool) => (
        <ToggleGroupItem key={(tool as any).type} value={(tool as any).type} aria-label={(tool as any).name}>
          {(tool as any).icon}
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  );
};
