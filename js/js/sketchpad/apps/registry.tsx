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

export interface AppConfig {
  id: string;
  component: ComponentType;
  routeSegments: RouteSegment[];
  additionalPaths?: string[];
  getPanels: (t: (key: string) => string) => PanelDefinition[];
  matchesPath?: (pathParts: string[]) => boolean;
  order?: number;
}

export interface AppRegistration extends AppConfig {}

class AppRegistry {
  private apps: Map<string, AppRegistration> = new Map();
  private autoDiscovered = false;

  private autoDiscover(): void {
    if (this.autoDiscovered) return;
    this.autoDiscovered = true;

    const appModules = import.meta.glob<{ config: AppConfig }>("./*/config.ts", { eager: true });

    for (const [path, module] of Object.entries(appModules)) {
      if (module.config) {
        this.register(module.config);
      }
    }
  }

  register(registration: AppRegistration): void {
    if (this.apps.has(registration.id)) return;
    this.apps.set(registration.id, registration);
  }

  unregister(id: string): void {
    this.apps.delete(id);
  }

  getApp(id: string): AppRegistration | undefined {
    return this.apps.get(id);
  }

  getAllApps(): AppRegistration[] {
    return Array.from(this.apps.values()).sort((a, b) => (a.order || 0) - (b.order || 0));
  }

  getAppForPath(pathParts: string[]): AppRegistration | undefined {
    for (const app of this.apps.values()) {
      if (app.matchesPath && app.matchesPath(pathParts)) {
        return app;
      }
    }
    return undefined;
  }

  getPanelConfigs(t: (key: string) => string): Record<string, PanelDefinition[]> {
    const configs: Record<string, PanelDefinition[]> = {};
    for (const [id, app] of this.apps.entries()) {
      configs[id] = app.getPanels(t);
    }
    return configs;
  }

  initialize(): void {
    this.autoDiscover();
  }
}

export const appRegistry = new AppRegistry();
