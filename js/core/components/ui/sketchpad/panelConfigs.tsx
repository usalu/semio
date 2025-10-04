// #region Header

// panelConfigs.tsx

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

import { Info, MessageCircle, Settings, Terminal, Wrench } from "lucide-react";
import { EditorType } from "../../../store";

export interface PanelDefinition {
  key: string;
  icon: React.ComponentType<{ size?: number }>;
  tooltip: string;
  hotkey: string;
}

export const PANEL_CONFIGS: Record<EditorType, PanelDefinition[]> = {
  [EditorType.HOME]: [],
  [EditorType.KIT]: [
    { key: "console", icon: Terminal, tooltip: "Click to toggle console panel", hotkey: "⌘K" },
    { key: "settings", icon: Settings, tooltip: "Click to toggle settings panel", hotkey: "⌘," },
  ],
  [EditorType.DESIGN]: [
    { key: "workbench", icon: Wrench, tooltip: "Click to toggle workbench panel", hotkey: "⌘J" },
    { key: "console", icon: Terminal, tooltip: "Click to toggle console panel", hotkey: "⌘K" },
    { key: "details", icon: Info, tooltip: "Click to toggle details panel", hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: "Click to toggle chat panel", hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: "Click to toggle settings panel", hotkey: "⌘," },
  ],
  [EditorType.TYPE]: [
    { key: "workbench", icon: Wrench, tooltip: "Click to toggle workbench panel", hotkey: "⌘J" },
    { key: "console", icon: Terminal, tooltip: "Click to toggle console panel", hotkey: "⌘K" },
    { key: "settings", icon: Settings, tooltip: "Click to toggle settings panel", hotkey: "⌘," },
  ],
};
