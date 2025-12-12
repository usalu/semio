// #region Header

// apps/index.ts

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

/**
 * App Plugin Registration Entry Point
 * 
 * This file imports all app modules to trigger their plugin registrations.
 * Each app module registers its plugin as a side-effect on import.
 * 
 * Import this file once in the application entry point to ensure all
 * app plugins are registered before the sketchpad machine is created.
 * 
 * Architecture:
 * - Each app (Home, Kit, Type, Design, Quality, Docs) has a plugin
 * - Plugins provide: events, actions, guards, selectors, default state
 * - Plugins are composed into the sketchpad machine at runtime
 * - No edits to Sketchpad.tsx needed for new apps (open/closed principle)
 */

// Import all app modules to trigger plugin registrations
import "../Design";
import "../Docs";
import "../Home";
import "../Kit";
import "../Quality";
import "../Type";

// Re-export plugin utilities for external use
export { composePluginContributions, getAppPlugin, getAppPlugins, hasAppPlugin, registerAppPlugin } from "../shared";
export type { AppMachineContribution, AppPlugin } from "../shared";

