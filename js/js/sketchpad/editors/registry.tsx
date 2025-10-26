// #region Header

// registry.tsx

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

import { ComponentType, ReactNode } from "react";

export interface PanelDefinition {
  key: string;
  icon: ComponentType<{ size?: number }>;
  tooltip: string;
  hotkey: string;
}

export interface RouteSegment {
  path: string;
  paramName?: string;
  scopeProvider?: ComponentType<{ guid: string; children: ReactNode }>;
}

export interface EditorRegistration {
  id: string;
  component: ComponentType;
  routeSegments: RouteSegment[];
  additionalPaths?: string[];
  getPanels: (t: (key: string) => string) => PanelDefinition[];
  matchesPath?: (pathParts: string[]) => boolean;
  order?: number;
}

class EditorRegistry {
  private editors: Map<string, EditorRegistration> = new Map();

  register(registration: EditorRegistration): void {
    if (this.editors.has(registration.id)) return;
    this.editors.set(registration.id, registration);
  }

  unregister(id: string): void {
    this.editors.delete(id);
  }

  getEditor(id: string): EditorRegistration | undefined {
    return this.editors.get(id);
  }

  getAllEditors(): EditorRegistration[] {
    return Array.from(this.editors.values()).sort((a, b) => (a.order || 0) - (b.order || 0));
  }

  getEditorForPath(pathParts: string[]): EditorRegistration | undefined {
    for (const editor of this.editors.values()) {
      if (editor.matchesPath && editor.matchesPath(pathParts)) {
        return editor;
      }
    }
    return undefined;
  }

  getPanelConfigs(t: (key: string) => string): Record<string, PanelDefinition[]> {
    const configs: Record<string, PanelDefinition[]> = {};
    for (const [id, editor] of this.editors.entries()) {
      configs[id] = editor.getPanels(t);
    }
    return configs;
  }
}

export const editorRegistry = new EditorRegistry();
