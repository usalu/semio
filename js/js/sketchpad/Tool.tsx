// #region Header

// Tool.tsx

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

import { FC, ReactNode } from "react";
import { Toggle, ToggleItem } from "../elements/input/Toggle";
import { ToggleGroup, ToggleGroupItem } from "../elements/input/ToggleGroup";
import { ToolType } from "./store";

export interface ToolMode {
  id: ToolType;
  label: string;
  icon: ReactNode;
  tooltip?: string;
  hotkey?: string;
}

export interface ToolModeComponentProps {
  mode: ToolMode;
  isActive: boolean;
  onActivate: (id: ToolType) => void;
  level?: "base" | "panel" | "temporary";
}

export const ToolModeComponent: FC<ToolModeComponentProps> = ({ mode, isActive, onActivate, level = "panel" }) => (
  <ToggleGroupItem value={mode.id} tooltip={mode.tooltip} hotkey={mode.hotkey}>
    {mode.icon}
  </ToggleGroupItem>
);

export interface ToolDefinition {
  id: string;
  modes: ToolMode[];
  defaultMode?: ToolType;
  group?: string;
}

export interface ToolGroupProps {
  tools: ToolDefinition[];
  activeTool: ToolType;
  onToolChange: (toolType: ToolType) => void;
  level?: "base" | "panel" | "temporary";
}

export const ToolGroup: FC<ToolGroupProps> = ({ tools, activeTool, onToolChange, level = "panel" }) => {
  const allModes = tools.flatMap((tool) => tool.modes);
  const activeMode = allModes.find((mode) => mode.id === activeTool);
  const singleModeTools = tools.filter((tool) => tool.modes.length === 1);
  const multiModeTools = tools.filter((tool) => tool.modes.length > 1);
  const hasSingleModeTools = singleModeTools.length > 0;
  const hasMultiModeTools = multiModeTools.length > 0;
  if (!hasSingleModeTools && multiModeTools.length === 1) {
    const tool = multiModeTools[0];
    const currentMode = tool.modes.find((mode) => mode.id === activeTool);
    const isPressed = !!currentMode;
    const defaultMode = tool.defaultMode || tool.modes[0].id;
    const dropdownItems: ToggleItem<ToolType>[] = tool.modes.map((mode) => ({ value: mode.id, label: mode.icon, tooltip: mode.tooltip, hotkey: mode.hotkey }));
    return <Toggle type="dropdown" level={level} pressed={isPressed} onPressedChange={(pressed) => !pressed && onToolChange(defaultMode)} value={activeTool} onValueChange={onToolChange} items={dropdownItems} tooltip={currentMode?.tooltip} />;
  }
  return (
    <div className="flex items-center gap-0">
      {hasSingleModeTools && (
        <ToggleGroup type="single" level={level} value={activeTool} onValueChange={(value: string) => value && onToolChange(value as ToolType)}>
          {singleModeTools.map((tool) => {
            const mode = tool.modes[0];
            return <ToolModeComponent key={mode.id} mode={mode} isActive={activeTool === mode.id} onActivate={onToolChange} level={level} />;
          })}
        </ToggleGroup>
      )}
      {hasMultiModeTools &&
        multiModeTools.map((tool) => {
          const currentMode = tool.modes.find((mode) => mode.id === activeTool);
          const isPressed = !!currentMode;
          const defaultMode = tool.defaultMode || tool.modes[0].id;
          const dropdownItems: ToggleItem<ToolType>[] = tool.modes.map((mode) => ({ value: mode.id, label: mode.icon, tooltip: mode.tooltip, hotkey: mode.hotkey }));
          return (
            <Toggle
              key={tool.id}
              type="dropdown"
              level={level}
              pressed={isPressed}
              onPressedChange={(pressed) => !pressed && onToolChange(defaultMode)}
              value={activeTool}
              onValueChange={onToolChange}
              items={dropdownItems}
              tooltip={currentMode?.tooltip}
            />
          );
        })}
    </div>
  );
};
