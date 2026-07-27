// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the Electron renderer process mounting the coda React app.

// #endregion 🧲Header

// #region ⛩️Renderer
// Electron renderer process that mounts the coda dashboard React app with window controls.
// MUST resolve the user identity before rendering the dashboard.
// MUST communicate with coda MCP server via the preload bridge.

// #region 🔌Adapters
import React from "react";
import { createRoot } from "react-dom/client";
import {
  ActionBus,
  Controller,
  Platform,
  AppRuntime,
  ModeRuntime,
  WindowKindRuntime,
  buildPanelWindowBody,
  createTabStackLayout,
  registerCornerPanelBody,
  registerWindowBody,
  uiDeclarativeSectionsToTree,
  type UiPanelHostSurfaceNode,
  type UiSectionNode,
  type UiTreeItemNode,
  type UiTreeNode,
  type WindowBodyViewContext,
} from "@semio-tech/framework-platform-core";
import {
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY,
  type ActionDescriptor,
} from "@semio-tech/framework-core";
import { PlatformView, registerUiPanelSurfaceHost } from "@semio-tech/framework-platform-renderer-react";
import { LEVELS, LevelProvider, reactHostPort, registerUiTranslationBundles, Tree, TreeItem, useActionHotkey, useLabel, useLevel, type Level as UiLevel, type UiLocale } from "@semio-tech/ui-react";
// #endregion 🔌Adapters

import "../globals.css";

//#region 🪁CodaUiI18n
/** @emoji 🪁 Coda's own i18n bundle namespace, registered on the shared UI i18n instance the same way compose's sketchpad registers `compose.*` keys. Coda has no entity nouns overlapping puzzle/CAD's "reuse" terminology, so this is locale-only — no terminology dimension. */
type CodaTranslationKey =
  | `coda.nav.${string}`
  | `coda.page.${string}`
  | `coda.section.${string}`
  | `coda.column.${string}`
  | `coda.card.${string}`
  | `coda.welcome.${string}`
  | `coda.common.${string}`
  | `coda.loading.${string}`
  | `coda.empty.${string}`
  | `coda.placeholder.${string}`
  | "coda.titlebar.subtitle"
  | "coda.titlebar.sidecarConnected"
  | "coda.titlebar.sidecarDisconnected"
  | "coda.titlebar.connected"
  | "coda.titlebar.offline"
  | "coda.titlebar.refresh";

const CODA_NAV_LABELS_EN: Readonly<Record<Page, string>> = { dashboard: "Dashboard", config: "Config", runs: "Runs", report: "Report", translations: "Translations", actions: "Actions", events: "Events" };
const CODA_NAV_LABELS_DE: Readonly<Record<Page, string>> = { dashboard: "Übersicht", config: "Konfiguration", runs: "Durchläufe", report: "Bericht", translations: "Übersetzungen", actions: "Aktionen", events: "Ereignisse" };

/** @emoji 🪁 Flat one-off `coda.*` label keys (page/section titles, card titles, placeholders, loading/empty copy) that don't fit the nav/titlebar shape below — mirrors the flat-map style used by `mcp-app.tsx`'s `McpAppTranslationKey` bundle. */
const CODA_MISC_LABELS_EN: Readonly<Record<string, string>> = {
  "coda.page.dashboard.title": "Dashboard",
  "coda.page.config.title": "Configuration",
  "coda.page.runs.title": "Runs & Iterations",
  "coda.page.report.title": "Compliance Report",
  "coda.page.translations.title": "Translations",
  "coda.page.actions.title": "Actions",
  "coda.page.events.title": "Events",
  "coda.section.generalConfig.title": "General Configuration",
  "coda.section.properties.title": "Properties",
  "coda.section.rules.title": "Rules",
  "coda.section.items.title": "Items",
  "coda.section.levels.title": "Levels",
  "coda.section.targets.title": "Targets",
  "coda.column.id": "ID",
  "coda.column.started": "Started",
  "coda.card.latestValidation.title": "Latest Validation",
  "coda.card.currentRun.title": "Current Run",
  "coda.card.runManagement.title": "Run Management",
  "coda.card.translationValidation.title": "Translation & Validation",
  "coda.card.fixDesign.title": "Fix Design",
  "coda.card.manualFixResult.title": "Manual Fix Result",
  "coda.card.actionLog.title": "Action Log",
  "coda.welcome.createProject.title": "Create New Project",
  "coda.welcome.openProject.title": "Open Existing Project",
  "coda.welcome.projectName.label": "Project Name",
  "coda.welcome.projectFolder.label": "Project Folder",
  "coda.common.loading": "Loading...",
  "coda.loading.dashboard": "Loading dashboard...",
  "coda.loading.config": "Loading configuration...",
  "coda.loading.runs": "Loading runs...",
  "coda.loading.report": "Loading report...",
  "coda.empty.noRuns": "No runs yet. Start a run from the Actions page.",
  "coda.empty.noIterations": "No iterations yet.",
  "coda.placeholder.fixResultJson": "Paste fix result JSON here...",
  "coda.placeholder.fixDescriptionExample": "e.g., Increase gross floor area to meet room program requirements",
  "coda.placeholder.filterEvents": "Filter events by kind or content...",
  "coda.placeholder.projectName": "My Project",
};

const CODA_MISC_LABELS_DE: Readonly<Record<string, string>> = {
  "coda.page.dashboard.title": "Übersicht",
  "coda.page.config.title": "Konfiguration",
  "coda.page.runs.title": "Durchläufe & Iterationen",
  "coda.page.report.title": "Konformitätsbericht",
  "coda.page.translations.title": "Übersetzungen",
  "coda.page.actions.title": "Aktionen",
  "coda.page.events.title": "Ereignisse",
  "coda.section.generalConfig.title": "Allgemeine Konfiguration",
  "coda.section.properties.title": "Eigenschaften",
  "coda.section.rules.title": "Regeln",
  "coda.section.items.title": "Elemente",
  "coda.section.levels.title": "Stufen",
  "coda.section.targets.title": "Ziele",
  "coda.column.id": "ID",
  "coda.column.started": "Gestartet",
  "coda.card.latestValidation.title": "Letzte Validierung",
  "coda.card.currentRun.title": "Aktueller Durchlauf",
  "coda.card.runManagement.title": "Durchlauf-Verwaltung",
  "coda.card.translationValidation.title": "Übersetzung & Validierung",
  "coda.card.fixDesign.title": "Entwurf korrigieren",
  "coda.card.manualFixResult.title": "Manuelles Korrekturergebnis",
  "coda.card.actionLog.title": "Aktionsprotokoll",
  "coda.welcome.createProject.title": "Neues Projekt erstellen",
  "coda.welcome.openProject.title": "Bestehendes Projekt öffnen",
  "coda.welcome.projectName.label": "Projektname",
  "coda.welcome.projectFolder.label": "Projektordner",
  "coda.common.loading": "Wird geladen...",
  "coda.loading.dashboard": "Übersicht wird geladen...",
  "coda.loading.config": "Konfiguration wird geladen...",
  "coda.loading.runs": "Durchläufe werden geladen...",
  "coda.loading.report": "Bericht wird geladen...",
  "coda.empty.noRuns": "Noch keine Durchläufe. Starte einen Durchlauf auf der Aktionen-Seite.",
  "coda.empty.noIterations": "Noch keine Iterationen.",
  "coda.placeholder.fixResultJson": "Korrekturergebnis-JSON hier einfügen...",
  "coda.placeholder.fixDescriptionExample": "z. B. Bruttogeschossfläche erhöhen, um die Raumprogrammanforderungen zu erfüllen",
  "coda.placeholder.filterEvents": "Ereignisse nach Art oder Inhalt filtern...",
  "coda.placeholder.projectName": "Mein Projekt",
};

function codaMiscTranslationBundle(misc: Readonly<Record<string, string>>): Readonly<Record<string, { readonly label: { readonly normal: string; readonly beginner: string } }>> {
  const entries: Record<string, { readonly label: { readonly normal: string; readonly beginner: string } }> = {};
  for (const [id, label] of Object.entries(misc)) entries[id] = { label: { normal: label, beginner: label } };
  return entries;
}

function codaTranslationBundle(
  nav: Readonly<Record<Page, string>>,
  titlebar: Readonly<Record<"subtitle" | "sidecarConnected" | "sidecarDisconnected" | "connected" | "offline" | "refresh", string>>,
  misc: Readonly<Record<string, string>>,
): Readonly<Record<CodaTranslationKey, { readonly label: { readonly normal: string; readonly beginner: string } }>> {
  const entries: Record<string, { readonly label: { readonly normal: string; readonly beginner: string } }> = { ...codaMiscTranslationBundle(misc) };
  for (const [id, label] of Object.entries(nav)) entries[`coda.nav.${id}`] = { label: { normal: label, beginner: label } };
  entries["coda.titlebar.subtitle"] = { label: { normal: titlebar.subtitle, beginner: titlebar.subtitle } };
  entries["coda.titlebar.sidecarConnected"] = { label: { normal: titlebar.sidecarConnected, beginner: titlebar.sidecarConnected } };
  entries["coda.titlebar.sidecarDisconnected"] = { label: { normal: titlebar.sidecarDisconnected, beginner: titlebar.sidecarDisconnected } };
  entries["coda.titlebar.connected"] = { label: { normal: titlebar.connected, beginner: titlebar.connected } };
  entries["coda.titlebar.offline"] = { label: { normal: titlebar.offline, beginner: titlebar.offline } };
  entries["coda.titlebar.refresh"] = { label: { normal: titlebar.refresh, beginner: titlebar.refresh } };
  return entries as Readonly<Record<CodaTranslationKey, { readonly label: { readonly normal: string; readonly beginner: string } }>>;
}

/** @emoji 🪁 Casts a `coda.*` key to the branded {@link UiRegisteredTranslationKey} `useLabel` requires —
 * the only way to obtain one, so a typo'd/unregistered key does not type-check. */
const codaKey = registerUiTranslationBundles({
  en: {
    translation: codaTranslationBundle(
      CODA_NAV_LABELS_EN,
      {
        subtitle: "ACC Design Assistant",
        sidecarConnected: "Sidecar connected",
        sidecarDisconnected: "Sidecar disconnected (offline mode)",
        connected: "Connected",
        offline: "Offline",
        refresh: "Refresh data",
      },
      CODA_MISC_LABELS_EN,
    ),
  },
  de: {
    translation: codaTranslationBundle(
      CODA_NAV_LABELS_DE,
      {
        subtitle: "ACC Entwurfsassistent",
        sidecarConnected: "Sidecar verbunden",
        sidecarDisconnected: "Sidecar getrennt (Offline-Modus)",
        connected: "Verbunden",
        offline: "Offline",
        refresh: "Daten aktualisieren",
      },
      CODA_MISC_LABELS_DE,
    ),
  },
});
//#endregion 🪁CodaUiI18n

console.log("[DEBUG] renderer.tsx imports resolved, module body executing");

// #region ⚙️Types
// TypeScript interfaces for coda domain models used in the renderer.
// Types MUST match the coda MCP server data structures.

declare global {
  interface Window {
    windowControls: {
      minimize(): Promise<void>;
      maximize(): Promise<void>;
      close(): Promise<void>;
    };
    os: {
      getUserId(): Promise<string>;
    };
    coda: {
      call(method: string, params?: Record<string, unknown>): Promise<McpResponse>;
      fetch(uri: string): Promise<McpResponse>;
      tool(name: string, args: Record<string, unknown>): Promise<McpResponse>;
      getConnectionStatus(): Promise<boolean>;
      onEvent(callback: (event: CodaEvent) => void): () => void;
      onConnectionStatus(callback: (connected: boolean) => void): () => void;
    };
    dialog: {
      openFolder(): Promise<string | null>;
    };
    project: {
      getPath(): Promise<string | null>;
      open(folder: string): Promise<{ success: boolean; error?: string }>;
      create(name: string, folder: string): Promise<{ success: boolean; error?: string }>;
    };
  }
}

/**
 * An event pushed from the coda sidecar process.
 *MUST have event kind, data, and timestamp.
 **/
interface CodaEvent {
  event: string;
  data: Record<string, unknown>;
  timestamp: number;
}

/**
 * MCP JSON-RPC response wrapper.
 *MUST contain either result or error.
 **/
interface McpResponse {
  jsonrpc: string;
  id: number;
  result?: {
    contents?: Array<{ uri: string; mimeType?: string; text?: string }>;
    content?: Array<{ type: string; text?: string }>;
  };
  error?: { code: number; message: string };
}

/**
 * A platform measure instruction for a specific measure kind on a property.
 *MUST have instructions and optional mcp tools.
 **/
interface PlatformMeasureInstruction {
  instructions?: string;
  mcp?: {
    resources?: Array<{ id: string; instruction: string }>;
    tools?: Array<{ id: string; instruction: string; parameters?: Array<{ id: string; instruction: string }> }>;
  };
}

/**
 * A platform property with measure kind keys mapping to instructions.
 *MUST have id. Measure kinds (increase, decrease, etc.) are dynamic keys.
 **/
interface PlatformProperty {
  id: string;
  [measureKind: string]: string | PlatformMeasureInstruction | undefined;
}

/**
 * A platform with properties and their measure instructions.
 *MUST have an id.
 **/
interface Platform {
  id: string;
  properties?: PlatformProperty[];
}

/**
 * A clause within a compliance rule.
 *MUST have id and description.
 **/
interface Clause {
  id: string;
  description: string;
  status?: "compliant" | "violated" | "unknown";
  properties?: Array<{ id: string; value: string }>;
}

/**
 * A compliance rule belonging to a target.
 *MUST have id and description.
 **/
interface Rule {
  id: string;
  description: string;
  status?: "compliant" | "violated" | "unknown";
  clauses?: Clause[];
  measures?: string[];
  data?: Record<string, unknown>;
}

/**
 * A measure reference with an instruction for a specific level direction.
 *MUST have id and optional instruction.
 **/
interface LevelMeasureRef {
  id: string;
  instruction?: string;
}

/**
 * A level within a property, optionally with measures and instructions for raising/lowering.
 *MUST have a value. May have measures (lower/higher) and instructions (higher).
 **/
interface Level {
  value: string;
  name?: string;
  description?: string;
  measures?: { lower?: LevelMeasureRef[]; raise?: LevelMeasureRef[] };
  instructions?: { raise?: LevelMeasureRef[] };
}

/**
 * A property kind definition mapping kind names to their available measures.
 *MUST map kind id to measures array.
 **/
interface PropertyKindMap {
  [kind: string]: { measures?: string[]; measure?: string[] };
}

/**
 * A correlation matrix between properties.
 *MUST have properties array and matrix.
 **/
interface Correlation {
  properties: string[];
  matrix: number[][];
}

/**
 * A property definition with canonical kind and associated measure_kinds.
 *MUST have id. Kind determines measure_kinds (e.g. number->increase/decrease, level->raise/lower).
 **/
interface Property {
  id: string;
  name?: string;
  kind?: string;
  measure_kinds?: string[];
  description?: string;
  url?: string;
  levels?: Level[];
  properties?: Property[];
  items?: Property;
  values?: (string | { id: string; name?: string; description?: string })[];
}

/**
 * A compliance framework with properties and rules. General (not project-scoped).
 *MUST have an id.
 **/
interface Framework {
  id: string;
  properties?: Property[];
  rules?: Rule[];
}

/**
 * Project configuration from .coda/project.json.
 *MUST have design and targets.
 **/
interface Project {
  design?: { id: string; mcp?: Record<string, unknown> };
  targets?: Array<{ id: string; llm?: unknown[] }>;
  error?: string;
}

/**
 * Run metadata.
 *MUST have an id.
 **/
interface Run {
  id?: string;
  started?: string;
  run_id?: string;
  error?: string;
}

/**
 * Iteration metadata.
 *MUST have an index.
 **/
interface Iteration {
  index?: number | string;
  targets?: string[];
  error?: string;
}

/**
 * Compliance report from an iteration.
 *MUST contain rules array.
 **/
interface Report {
  valid?: boolean;
  validations?: ValidationReport[];
  error?: string;
}

/**
 * Navigation page identifiers.
 *MUST enumerate all navigable pages.
 **/
type Page = "dashboard" | "config" | "runs" | "report" | "translations" | "actions" | "events";

// #endregion ⚙️Types

// #region 🎼Helpers
// Helper functions for parsing MCP responses and formatting data.
// Helpers MUST safely extract data from MCP JSON-RPC responses.

/**
 * Extract parsed JSON from an MCP resource response.
 *MUST return null when the response has no valid content.
 **/
function parseMcpResource<T>(response: McpResponse): T | null {
  try {
    if (response.error) return null;
    const contents = response.result?.contents;
    if (contents && contents.length > 0 && contents[0].text) {
      return JSON.parse(contents[0].text) as T;
    }
    return null;
  } catch {
    return null;
  }
}

/**
 * Extract parsed JSON from an MCP tool response.
 *MUST return null when the response has no valid content.
 **/
function parseMcpTool<T>(response: McpResponse): T | null {
  try {
    if (response.error) return null;
    const content = response.result?.content;
    if (content && content.length > 0 && content[0].text) {
      return JSON.parse(content[0].text) as T;
    }
    return null;
  } catch {
    return null;
  }
}

/**
 * Format a timestamp or ID string for display.
 *MUST return a human-readable string.
 **/
function formatId(id: string | undefined): string {
  if (!id) return "—";
  return id.replace(/_/g, " ").replace(/-/g, " ");
}

// #endregion 🎼Helpers

// #region 🛒Icons
// Inline SVG icon components used across the coda desktop UI.
// Icons MUST be pure functional components with className prop.

function IconDashboard({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <rect x="3" y="3" width="7" height="7" />
      <rect x="14" y="3" width="7" height="7" />
      <rect x="14" y="14" width="7" height="7" />
      <rect x="3" y="14" width="7" height="7" />
    </svg>
  );
}

function IconConfig({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <circle cx="12" cy="12" r="3" />
      <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
    </svg>
  );
}

function IconRuns({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <polyline points="22 12 18 12 15 21 9 3 6 12 2 12" />
    </svg>
  );
}

function IconReport({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
      <polyline points="14 2 14 8 20 8" />
      <line x1="16" y1="13" x2="8" y2="13" />
      <line x1="16" y1="17" x2="8" y2="17" />
      <polyline points="10 9 9 9 8 9" />
    </svg>
  );
}

function IconTranslations({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <line x1="16.5" y1="9.4" x2="7.5" y2="4.21" />
      <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
      <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
      <line x1="12" y1="22.08" x2="12" y2="12" />
    </svg>
  );
}

function IconActions({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
    </svg>
  );
}

function IconMinimize({ className = "w-3 h-3" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <line x1="5" y1="12" x2="19" y2="12" />
    </svg>
  );
}

function IconMaximize({ className = "w-3 h-3" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <rect x="4" y="4" width="16" height="16" rx="1" />
    </svg>
  );
}

function IconClose({ className = "w-3 h-3" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <line x1="18" y1="6" x2="6" y2="18" />
      <line x1="6" y1="6" x2="18" y2="18" />
    </svg>
  );
}

function IconRefresh({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <polyline points="23 4 23 10 17 10" />
      <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
    </svg>
  );
}

function IconPlay({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <polygon points="5 3 19 12 5 21 5 3" />
    </svg>
  );
}

function IconCheck({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <polyline points="20 6 9 17 4 12" />
    </svg>
  );
}

function IconX({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <line x1="18" y1="6" x2="6" y2="18" />
      <line x1="6" y1="6" x2="18" y2="18" />
    </svg>
  );
}

function IconWrench({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" />
    </svg>
  );
}

function IconChevronRight({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <polyline points="9 18 15 12 9 6" />
    </svg>
  );
}

function IconChevronDown({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <polyline points="6 9 12 15 18 9" />
    </svg>
  );
}

function IconEvents({ className = "w-4 h-4" }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
    </svg>
  );
}

// #endregion 🛒Icons

// #region 🎖️Components
// Reusable UI components for the coda desktop application.
// Components MUST use Tailwind CSS classes for styling.

// #region 🪟DockedPanelLevel
// Shared level-bump logic for docked content boxes (Card/StatCard/OntologyTree/ValidationTree).
// See the 6-level contract at .repo/🎫/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt --
// "desktop chrome and docked panels get panel" -- and its nesting rule: exactly one ui-surface
// per level root, so a box already sitting inside another panel-level box must stay transparent
// instead of double-tinting.

/** @emoji 🪟 Resolves whether a docked content box (Card/StatCard/OntologyTree/ValidationTree)
 * must open its own `panel` level (bumping up from a shallower ambient floor like `window`), or
 * stay transparent because it is already nested inside a `panel`-or-deeper ambient box. */
function useDockedPanelLevel(): { level: UiLevel; opensLevel: boolean } {
  const ambient = useLevel();
  const opensLevel = LEVELS.indexOf(ambient) < LEVELS.indexOf("panel");
  return { level: opensLevel ? "panel" : ambient, opensLevel };
}
// #endregion 🪟DockedPanelLevel

// #region 🌀StatusBadge

/**
 * Displays a colored badge for compliance status.
 *MUST render green for compliant, red for violated, gray for unknown.
 **/
function StatusBadge({ status }: { status?: string }) {
  const colors: Record<string, string> = {
    compliant: "bg-success-bg text-success-foreground border-success-border",
    violated: "bg-destructive-bg text-destructive-foreground border-destructive-border",
    unknown: "bg-info-bg text-info-foreground border-info-border",
  };
  const s = status ?? "unknown";
  return (
    <span className={`inline-flex items-center gap-1 rounded-full border px-2 py-0.5 text-xs font-medium ${colors[s] ?? colors.unknown}`}>
      {s === "compliant" && <IconCheck className="w-3 h-3" />}
      {s === "violated" && <IconX className="w-3 h-3" />}
      {s}
    </span>
  );
}

// #endregion 🌀StatusBadge

// #region 🎬Card

/**
 * A card container for dashboard sections.
 *MUST render a bordered container with optional title.
 **/
function Card({ title, children, className = "", action }: { title?: string; children: React.ReactNode; className?: string; action?: React.ReactNode }) {
  const { level, opensLevel } = useDockedPanelLevel();
  const body = (
    <div data-level={opensLevel ? level : undefined} className={opensLevel ? `rounded-lg border border-normal ui-surface ${className}` : `rounded-lg border border-normal bg-transparent ${className}`}>
      {(title || action) && (
        <div className="flex items-center justify-between border-b border-normal px-4 py-3">
          {title && <h3 className="text-sm font-semibold text-foreground">{title}</h3>}
          {action}
        </div>
      )}
      <div className="p-4">{children}</div>
    </div>
  );
  return opensLevel ? <LevelProvider level={level}>{body}</LevelProvider> : body;
}

// #endregion 🎬Card

// #region 📌StatCard

/**
 * A metric card for the dashboard overview.
 *MUST display a label and a large value.
 **/
function StatCard({ label, value, sublabel }: { label: string; value: string | number; sublabel?: string }) {
  const { level, opensLevel } = useDockedPanelLevel();
  const body = (
    <div data-level={opensLevel ? level : undefined} className={opensLevel ? "rounded-lg border border-normal ui-surface p-4" : "rounded-lg border border-normal bg-transparent p-4"}>
      <div className="text-xs font-medium text-muted-foreground uppercase tracking-wider">{label}</div>
      <div className="mt-1 text-2xl font-bold text-foreground">{value}</div>
      {sublabel && <div className="mt-0.5 text-xs text-muted-foreground">{sublabel}</div>}
    </div>
  );
  return opensLevel ? <LevelProvider level={level}>{body}</LevelProvider> : body;
}

// #endregion 📌StatCard

// #region 🔤Button

/**
 * A styled button with variant support.
 *MUST support primary, secondary, and danger variants.
 **/
function Button({
  children,
  onClick,
  variant = "secondary",
  disabled = false,
  loading = false,
  className = "",
}: {
  children: React.ReactNode;
  onClick?: () => void;
  variant?: "primary" | "secondary" | "danger";
  disabled?: boolean;
  loading?: boolean;
  className?: string;
}) {
  const variants: Record<string, string> = {
    primary: "bg-active-base text-active-foreground hover:bg-hover-base disabled:opacity-50",
    secondary: "bg-element border border-normal text-foreground hover:bg-hover-interactive-fill disabled:opacity-50",
    danger: "bg-destructive-bg text-destructive-foreground border border-destructive-border hover:bg-hover-interactive-fill disabled:opacity-50",
  };
  return (
    <button onClick={onClick} disabled={disabled || loading} className={`inline-flex items-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium transition-colors cursor-pointer disabled:cursor-not-allowed ${variants[variant]} ${className}`}>
      {loading && (
        <svg className="w-3.5 h-3.5 animate-spin" viewBox="0 0 24 24" fill="none">
          <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
          <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
        </svg>
      )}
      {children}
    </button>
  );
}

// #endregion 🔤Button

// #region 🎹Spinner

/**
 * A centered loading spinner.
 *MUST display an animated spinning indicator.
 **/
function Spinner({ label }: { label?: string }) {
  const fallbackLabel = useLabel(codaKey("coda.common.loading"));
  const resolvedLabel = label ?? fallbackLabel ?? "Loading...";
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-12 text-muted-foreground">
      <svg className="w-6 h-6 animate-spin" viewBox="0 0 24 24" fill="none">
        <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
        <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
      </svg>
      <span className="text-sm">{resolvedLabel}</span>
    </div>
  );
}

// #endregion 🎹Spinner

// #region 🎊EmptyState

/**
 * An empty state placeholder.
 *MUST display a message and optional action.
 **/
function EmptyState({ message, action }: { message: string; action?: React.ReactNode }) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-12 text-muted-foreground">
      <p className="text-sm">{message}</p>
      {action}
    </div>
  );
}

// #endregion 🎊EmptyState

// #region 🖥️Collapsible

/**
 * A collapsible section with toggle.
 *MUST toggle visibility on header click.
 **/
function Collapsible({ title, children, defaultOpen = false, badge }: { title: string; children: React.ReactNode; defaultOpen?: boolean; badge?: React.ReactNode }) {
  const [open, setOpen] = reactHostPort.useState(defaultOpen);
  return (
    <div className="border border-normal bg-transparent rounded-md overflow-hidden">
      <button onClick={() => setOpen(!open)} className="flex w-full items-center gap-2 px-3 py-2 text-sm font-medium text-foreground hover:bg-hover-interactive-fill transition-colors cursor-pointer">
        {open ? <IconChevronDown className="w-3.5 h-3.5 text-muted-foreground" /> : <IconChevronRight className="w-3.5 h-3.5 text-muted-foreground" />}
        <span className="flex-1 text-left">{title}</span>
        {badge}
      </button>
      {open && <div className="border-t border-normal px-3 py-2">{children}</div>}
    </div>
  );
}

// #endregion 🖥️Collapsible

// #region 📸JsonViewer

/**
 * A formatted JSON viewer for translation data.
 *MUST display formatted JSON in a code block.
 **/
function JsonViewer({ data }: { data: unknown }) {
  const formatted = reactHostPort.useMemo(() => {
    try {
      return JSON.stringify(data, null, 2);
    } catch {
      return String(data);
    }
  }, [data]);
  return <pre className="overflow-auto max-h-96 rounded-md bg-transparent border border-normal p-3 text-xs font-mono text-muted-foreground whitespace-pre-wrap break-all">{formatted}</pre>;
}

// #endregion 📸JsonViewer

// #region 🧨OntologyTree
// Tree viewer for visualizing OWL class expression structure (schema-level, no instances).
// OntologyTree MUST render a collapsible tree of the class expression without truth values.

/**
 * Node kind in an ontology class expression tree.
 *Enumerates all OWL class expression constructs.
 **/
type OntologyNodeKind = "Class" | "And" | "Or" | "Not" | "SomeValuesFrom" | "AllValuesFrom" | "ExactCardinality" | "MinCardinality" | "MaxCardinality" | "DataSomeValuesFrom" | "DataAllValuesFrom" | "DataHasValue" | "DatatypeRestriction";

/**
 * A node in the ontology class expression tree (schema only, no instances).
 *MUST have id, kind, label, and children.
 **/
interface OntologyTreeNode {
  id: string;
  kind: OntologyNodeKind;
  label: string;
  fragment?: string;
  property?: string;
  className?: string;
  cardinality?: number;
  datatype?: string;
  restriction?: string;
  children: OntologyTreeNode[];
}

/**
 * Returns the icon label for an ontology node kind.
 **/
function ontologyNodeIcon(kind: OntologyNodeKind): string {
  switch (kind) {
    case "Class":
      return "C";
    case "And":
      return "∧";
    case "Or":
      return "∨";
    case "Not":
      return "¬";
    case "SomeValuesFrom":
      return "∃";
    case "AllValuesFrom":
      return "∀";
    case "ExactCardinality":
      return "=n";
    case "MinCardinality":
      return "≥n";
    case "MaxCardinality":
      return "≤n";
    case "DataSomeValuesFrom":
      return "∃d";
    case "DataAllValuesFrom":
      return "∀d";
    case "DataHasValue":
      return "v";
    case "DatatypeRestriction":
      return "D";
    default:
      return "?";
  }
}

function getOntologyNodeDescriptor(node: OntologyTreeNode): { icon: string; primaryText: string; secondaryText?: string } {
  return {
    icon: ontologyNodeIcon(node.kind),
    primaryText: node.label,
    secondaryText: node.fragment && node.fragment !== node.label ? node.fragment : undefined,
  };
}

/**
 * Renders a single ontology tree node with expand/collapse.
 *MUST display the node kind icon, label, and expandable children.
 **/
function OntologyTreeNodeView({ node, defaultExpanded = true }: { node: OntologyTreeNode; defaultExpanded?: boolean }) {
  const descriptor = getOntologyNodeDescriptor(node);

  return (
    <TreeItem
      id={node.id}
      defaultOpen={defaultExpanded}
      label={
        <span className="flex min-w-0 items-center gap-2 overflow-hidden">
          <span className="inline-flex items-center justify-center h-5 min-w-5 rounded bg-info-bg px-1 text-2xs font-bold text-info-foreground shrink-0" title={node.kind}>
            {descriptor.icon}
          </span>
          <span className="min-w-0 truncate text-sm">
            <span className="font-medium text-foreground">{descriptor.primaryText}</span>
            {descriptor.secondaryText ? (
              <>
                {" "}
                <span className="text-2xs leading-none text-muted-foreground">{descriptor.secondaryText}</span>
              </>
            ) : null}
          </span>
        </span>
      }
    >
      {node.children.map((child) => (
        <OntologyTreeNodeView key={child.id} node={child} defaultExpanded={defaultExpanded} />
      ))}
    </TreeItem>
  );
}

/**
 * Tree viewer that displays an OWL class expression as a collapsible tree.
 *MUST render the full ontology tree structure from root.
 **/
function OntologyTree({ root, title, defaultExpanded = true }: { root: OntologyTreeNode; title?: string; defaultExpanded?: boolean }) {
  const { level, opensLevel } = useDockedPanelLevel();
  const body = (
    <div data-level={opensLevel ? level : undefined} className={opensLevel ? "rounded-lg border border-normal ui-surface overflow-hidden" : "rounded-lg border border-normal bg-transparent overflow-hidden"}>
      {title && (
        <div className="border-b border-normal px-3 py-2">
          <h3 className="text-sm font-semibold text-foreground">{title}</h3>
        </div>
      )}
      <div className="p-2 overflow-x-auto">
        <Tree className="min-w-0" sections={[{ id: "ontology-root", label: null, content: <OntologyTreeNodeView node={root} defaultExpanded={defaultExpanded} /> }]} />
      </div>
    </div>
  );
  return opensLevel ? <LevelProvider level={level}>{body}</LevelProvider> : body;
}

// #endregion 🧨OntologyTree

// #region 🔔ValidationTree
// Tree viewer for visualizing validation results (data graph instances of the ontology).
// ValidationTree MUST render truth values, witnesses, data values, and cardinality info.

/**
 * Three-valued truth for validation nodes.
 *true = green, false = red, unknown = gray.
 **/
type TruthValue = "true" | "false" | "unknown";

/**
 * Node kind in a validation tree, extending ontology kinds with instance-level nodes.
 *Extends OntologyNodeKind with Witness and DataValue for instance data.
 **/
type ValidationNodeKind = OntologyNodeKind | "ClassAssertion" | "Witness" | "DataValue";

/**
 * A node in the validation result tree (instance-level with truth values).
 *MUST have id, kind, label, truth, and children.
 **/
interface ValidationTreeNode {
  id: string;
  kind: ValidationNodeKind;
  label: string;
  fragment?: string;
  truth: TruthValue;
  summary?: string;
  property?: string;
  className?: string;
  subject?: string;
  individual?: string;
  counted?: boolean;
  expectedCardinality?: number;
  matchingCount?: number;
  value?: number | string;
  datatype?: string;
  children: ValidationTreeNode[];
}

/**
 * A validation report for a specific instance.
 *MUST have instance, expression, truth, and tree.
 **/
interface ValidationReport {
  instance: string;
  expression: string;
  truth: TruthValue;
  tree: ValidationTreeNode;
}

/**
 * Maps truth value to color classes.
 **/
const truthColors: Record<TruthValue, { dot: string; text: string; bg: string }> = {
  true: { dot: "bg-success-border", text: "text-success-foreground", bg: "bg-success-bg" },
  false: { dot: "bg-destructive-border", text: "text-destructive-foreground", bg: "bg-destructive-bg" },
  unknown: { dot: "bg-info-border", text: "text-info-foreground", bg: "bg-info-bg" },
};

/**
 * Maps truth value to emoji indicator.
 **/
function truthEmoji(truth: TruthValue): string {
  switch (truth) {
    case "true":
      return "🟢";
    case "false":
      return "🔴";
    case "unknown":
      return "⚪";
  }
}

function hasValidationCardinalityBadge(kind: ValidationNodeKind): boolean {
  return kind === "ExactCardinality" || kind === "MinCardinality" || kind === "MaxCardinality";
}

function getValidationNodeDescriptor(node: ValidationTreeNode): {
  icon?: string;
  primaryText: string;
  secondaryText?: string;
  chips: string[];
  dimmed: boolean;
} {
  const isWitness = node.kind === "Witness";
  const isDataValue = node.kind === "DataValue";
  const chips: string[] = [];

  if (hasValidationCardinalityBadge(node.kind) && node.matchingCount !== undefined && node.expectedCardinality !== undefined) {
    chips.push(`${node.matchingCount}/${node.expectedCardinality}`);
  }
  if (node.counted === true) {
    chips.push("counted");
  }
  if (node.counted === false) {
    chips.push("not matching");
  }
  if (isDataValue && node.datatype) {
    chips.push(node.datatype);
  }

  return {
    icon: isWitness || isDataValue ? undefined : node.kind === "ClassAssertion" ? "∈" : ontologyNodeIcon(node.kind as OntologyNodeKind),
    primaryText: isDataValue ? String(node.value ?? node.label) : isWitness ? (node.individual ?? node.label) : node.label,
    secondaryText: !isDataValue && node.fragment && node.fragment !== node.label ? node.fragment : undefined,
    chips,
    dimmed: isWitness && node.counted === false,
  };
}

/**
 * Renders a single validation tree node with expand/collapse, truth badges, and witnesses.
 *MUST display truth indicator, node label, witness/value info, and expandable children.
 **/
function ValidationTreeNodeView({ node, defaultExpanded = true }: { node: ValidationTreeNode; defaultExpanded?: boolean }) {
  const colors = truthColors[node.truth];
  const descriptor = getValidationNodeDescriptor(node);

  // 🌿Separate witness children from non-witness children for alternative branches.
  const witnessChildren = node.children.filter((c) => c.kind === "Witness");
  const nonWitnessChildren = node.children.filter((c) => c.kind !== "Witness");
  const useAlternatives = witnessChildren.length > 1;
  const [activeWitnessIndex, setActiveWitnessIndex] = reactHostPort.useState(0);
  const clampedIndex = useAlternatives ? Math.min(activeWitnessIndex, witnessChildren.length - 1) : 0;

  return (
    <TreeItem
      id={node.id}
      defaultOpen={defaultExpanded}
      branchCount={useAlternatives ? witnessChildren.length : undefined}
      activeBranchIndex={useAlternatives ? clampedIndex : undefined}
      onBranchChange={useAlternatives ? setActiveWitnessIndex : undefined}
      label={
        <span className="flex min-w-0 flex-col gap-0.5 py-0.5" title={node.summary}>
          <span className="flex min-w-0 items-center gap-2">
            <span className={`inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-2xs font-medium shrink-0 ${colors.bg} ${colors.text}`}>
              <span>{truthEmoji(node.truth)}</span>
              <span>{node.truth}</span>
            </span>
            {descriptor.icon && (
              <span className={`inline-flex items-center justify-center h-5 min-w-5 rounded px-1 text-2xs font-bold shrink-0 ${colors.bg} ${colors.text}`} title={node.kind}>
                {descriptor.icon}
              </span>
            )}
            <span className={`min-w-0 truncate text-sm font-medium ${descriptor.dimmed ? "text-muted-foreground" : node.kind === "DataValue" ? "font-mono text-foreground" : "text-foreground"}`}>{descriptor.primaryText}</span>
            {descriptor.chips.map((chip) => (
              <span
                key={`${node.id}-${chip}`}
                className={`inline-flex items-center rounded px-1.5 py-0.5 text-2xs font-medium shrink-0 ${
                  chip === "counted" ? "bg-success-bg text-success-foreground" : chip === "not matching" ? "bg-info-bg text-info-foreground" : "bg-element text-muted-foreground border border-normal"
                }`}
              >
                {chip}
              </span>
            ))}
          </span>
          {(descriptor.secondaryText || node.summary) && <span className="pl-10 text-xs text-muted-foreground">{[descriptor.secondaryText, node.summary].filter(Boolean).join(" • ")}</span>}
        </span>
      }
    >
      {nonWitnessChildren.map((child) => (
        <ValidationTreeNodeView key={child.id} node={child} defaultExpanded={defaultExpanded} />
      ))}
      {useAlternatives ? (
        <ValidationTreeNodeView key={witnessChildren[clampedIndex].id} node={witnessChildren[clampedIndex]} defaultExpanded={defaultExpanded} />
      ) : (
        witnessChildren.map((child) => <ValidationTreeNodeView key={child.id} node={child} defaultExpanded={defaultExpanded} />)
      )}
    </TreeItem>
  );
}

/**
 * Tree viewer that displays a validation report as a collapsible tree with truth values.
 *MUST render instance header, expression, overall truth, and the expanded result tree.
 **/
function ValidationTree({ report, defaultExpanded = true }: { report: ValidationReport; defaultExpanded?: boolean }) {
  const { level, opensLevel } = useDockedPanelLevel();
  const body = (
    <div data-level={opensLevel ? level : undefined} className={opensLevel ? "rounded-lg border border-normal ui-surface overflow-hidden" : "rounded-lg border border-normal bg-transparent overflow-hidden"}>
      <div className="border-b border-normal px-3 py-2 space-y-1">
        <div className="flex items-center gap-2">
          <span className="text-sm font-semibold text-foreground">Instance: {report.instance}</span>
          <span
            className={`inline-flex items-center gap-1 rounded-full border px-2 py-0.5 text-xs font-medium ${
              report.truth === "true"
                ? "bg-success-bg text-success-foreground border-success-border"
                : report.truth === "false"
                  ? "bg-destructive-bg text-destructive-foreground border-destructive-border"
                  : "bg-info-bg text-info-foreground border-info-border"
            }`}
          >
            {truthEmoji(report.truth)} {report.truth}
          </span>
        </div>
        <div className="text-xs text-muted-foreground font-mono break-all">{report.expression}</div>
      </div>
      <div className="p-2 overflow-x-auto">
        <Tree className="min-w-0" sections={[{ id: "validation-root", label: null, content: <ValidationTreeNodeView node={report.tree} defaultExpanded={defaultExpanded} /> }]} />
      </div>
    </div>
  );
  return opensLevel ? <LevelProvider level={level}>{body}</LevelProvider> : body;
}

// #endregion 🔔ValidationTree

// #endregion 🎖️Components

// #region 🦀Hooks
// Custom React hooks for fetching coda MCP data.
// Hooks MUST handle loading, error, and data states.

/**
 * Fetches a coda MCP resource and returns parsed data.
 *MUST refetch when uri or refreshKey changes.
 **/
function useCodaResource<T>(uri: string, refreshKey: number = 0): { data: T | null; loading: boolean; error: string | null; refresh: () => void } {
  const [data, setData] = reactHostPort.useState<T | null>(null);
  const [loading, setLoading] = reactHostPort.useState(true);
  const [error, setError] = reactHostPort.useState<string | null>(null);
  const [localRefresh, setLocalRefresh] = reactHostPort.useState(0);

  reactHostPort.useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);
    window.coda
      .fetch(uri)
      .then((response) => {
        if (cancelled) return;
        const parsed = parseMcpResource<T>(response);
        if (parsed !== null) {
          const obj = parsed as Record<string, unknown>;
          if (obj && typeof obj === "object" && "error" in obj) {
            setError(obj.error as string);
            setData(null);
          } else {
            setData(parsed);
          }
        } else if (response.error) {
          setError(response.error.message);
        } else {
          setData(null);
        }
        setLoading(false);
      })
      .catch((err) => {
        if (cancelled) return;
        setError(err?.message ?? "Failed to fetch");
        setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [uri, refreshKey, localRefresh]);

  const refresh = reactHostPort.useCallback(() => setLocalRefresh((n) => n + 1), []);
  return { data, loading, error, refresh };
}

// #endregion 🦀Hooks

// #region 🎉Pages
// Page components for each view in the coda desktop application.
// Pages MUST use useCodaResource hooks to fetch and display data.

// #region 🕹️DashboardPage

/**
 * Dashboard overview showing project status, current run, iteration, and compliance summary.
 *MUST display stat cards for project, run, iteration, and breach counts.
 **/
function DashboardPage({ refreshKey }: { refreshKey: number }) {
  const { data: project, loading: projectLoading } = useCodaResource<Project>("coda://project", refreshKey);
  const { data: run, loading: runLoading } = useCodaResource<Run>("coda://current-run", refreshKey);
  const { data: iteration, loading: iterLoading } = useCodaResource<Iteration>("coda://current-iteration", refreshKey);
  const { data: report, loading: reportLoading } = useCodaResource<Report>("coda://report", refreshKey);
  const { data: properties } = useCodaResource<Property[]>("coda://properties", refreshKey);
  const { data: frameworks } = useCodaResource<Framework[]>("coda://frameworks", refreshKey);

  const loading = projectLoading || runLoading || iterLoading || reportLoading;

  const totalValidations = report?.validations?.length ?? 0;
  const violatedCount = reactHostPort.useMemo(() => {
    if (!report?.validations) return 0;
    return report.validations.filter((v) => v.truth === "false").length;
  }, [report]);

  const compliantCount = reactHostPort.useMemo(() => {
    if (!report?.validations) return 0;
    return report.validations.filter((v) => v.truth === "true").length;
  }, [report]);

  const totalPropertyCount = reactHostPort.useMemo(() => (properties ? countProperties(properties) : 0), [properties]);

  const loadingDashboardLabel = useLabel(codaKey("coda.loading.dashboard"));
  const dashboardTitle = useLabel(codaKey("coda.page.dashboard.title"));
  const latestValidationTitle = useLabel(codaKey("coda.card.latestValidation.title"));
  const generalConfigTitle = useLabel(codaKey("coda.section.generalConfig.title"));
  const propertiesTitle = useLabel(codaKey("coda.section.properties.title"));

  if (loading) return <Spinner label={loadingDashboardLabel} />;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">{dashboardTitle}</h2>
        <p className="text-sm text-muted-foreground mt-1">Overview of the coda compliance checking status.</p>
      </div>

      <div className="grid grid-cols-2 gap-4 lg:grid-cols-4">
        <StatCard label="Design" value={project?.design?.id ?? "—"} sublabel={project ? `${project.targets?.length ?? 0} targets` : undefined} />
        <StatCard label="Current Run" value={run?.id ?? run?.run_id ?? "—"} sublabel={run?.started ? `Started ${run.started}` : undefined} />
        <StatCard label="Iteration" value={iteration?.index ?? "—"} sublabel={iteration?.targets ? `${iteration.targets.length} targets` : undefined} />
        <StatCard label="Compliance" value={totalValidations > 0 ? `${compliantCount}/${totalValidations}` : "—"} sublabel={violatedCount > 0 ? `${violatedCount} violated` : totalValidations > 0 ? "All compliant" : undefined} />
      </div>

      {report?.validations && report.validations.length > 0 && (
        <Card title={latestValidationTitle}>
          <div className="space-y-4">
            {report.validations.map((validation) => (
              <ValidationTree key={validation.instance} report={validation} defaultExpanded={false} />
            ))}
          </div>
        </Card>
      )}

      <div className="space-y-4">
        <div className="flex items-center gap-2">
          <h3 className="text-sm font-semibold text-foreground">{generalConfigTitle}</h3>
          <span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">not project-scoped</span>
        </div>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          {properties && properties.length > 0 && (
            <Card title={`${propertiesTitle} (${totalPropertyCount})`}>
              <div className="space-y-1">
                {properties.map((prop) => (
                  <div key={prop.id} className="flex items-start gap-2 rounded px-2 py-1.5 text-sm hover:bg-hover-interactive-fill">
                    <IconConfig className="w-3.5 h-3.5 text-muted-foreground mt-0.5 shrink-0" />
                    <div>
                      <div className="flex items-center gap-2">
                        <span className="font-medium text-foreground">{prop.name ?? formatId(prop.id)}</span>
                        {prop.kind && <span className="text-xs bg-info-bg text-info-foreground px-1 py-0.5 rounded">{prop.kind}</span>}
                      </div>
                      {prop.description && <p className="text-xs text-muted-foreground">{prop.description}</p>}
                      {prop.measure_kinds && prop.measure_kinds.length > 0 && (
                        <div className="mt-0.5 flex flex-wrap gap-1">
                          {prop.measure_kinds.map((mk) => (
                            <span key={mk} className="text-xs bg-active-base text-active-foreground px-1 py-0.5 rounded font-mono">
                              {mk}
                            </span>
                          ))}
                        </div>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </Card>
          )}

          {frameworks && frameworks.length > 0 && (
            <Card title={`Frameworks (${frameworks.length})`}>
              <div className="space-y-1">
                {frameworks.map((fw) => (
                  <div key={fw.id} className="flex items-start gap-2 rounded px-2 py-1.5 text-sm hover:bg-hover-interactive-fill">
                    <IconReport className="w-3.5 h-3.5 text-muted-foreground mt-0.5 shrink-0" />
                    <div>
                      <span className="font-medium text-foreground">{formatId(fw.id)}</span>
                      <p className="text-xs text-muted-foreground">
                        {fw.properties?.length ?? 0} properties, {fw.rules?.length ?? 0} rules
                      </p>
                    </div>
                  </div>
                ))}
              </div>
            </Card>
          )}
        </div>
      </div>
    </div>
  );
}

// #endregion 🕹️DashboardPage

// #region 📣ConfigPage

/**
 * Counts all properties recursively in a property tree.
 *MUST count the property itself plus all nested properties and items.
 **/
function countProperties(props: Property[]): number {
  let count = 0;
  for (const p of props) {
    count += 1;
    if (p.properties) count += countProperties(p.properties);
    if (p.items?.properties) count += countProperties(p.items.properties);
  }
  return count;
}

/**
 * Renders a single property with its kind badge, measure_kinds, and nested children.
 *MUST display kind, measure_kinds, description, levels, nested properties, and items recursively.
 **/
function PropertyView({ prop, depth = 0 }: { prop: Property; depth?: number }) {
  const levelsTitle = useLabel(codaKey("coda.section.levels.title"));
  const itemsTitle = useLabel(codaKey("coda.section.items.title"));
  const propertiesTitle = useLabel(codaKey("coda.section.properties.title"));
  return (
    <div className={`rounded bg-element border border-normal p-2 ${depth > 0 ? "ml-3" : ""}`}>
      <div className="flex items-center gap-2 flex-wrap">
        <span className="text-sm font-medium text-foreground">{prop.name ?? formatId(prop.id)}</span>
        {prop.kind && <span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">{prop.kind}</span>}
        {prop.measure_kinds &&
          prop.measure_kinds.length > 0 &&
          prop.measure_kinds.map((mk) => (
            <span key={mk} className="text-xs bg-active-base text-active-foreground px-1.5 py-0.5 rounded font-mono">
              {mk}
            </span>
          ))}
      </div>
      {prop.description && <p className="text-xs text-muted-foreground mt-1">{prop.description}</p>}
      {prop.url && (
        <a className="text-xs text-active-base hover:underline mt-1 inline-block" href={prop.url} target="_blank" rel="noreferrer">
          Reference
        </a>
      )}
      {prop.values && prop.values.length > 0 && (
        <div className="mt-1.5 flex flex-wrap gap-1">
          <span className="text-xs text-muted-foreground mr-1">Values:</span>
          {prop.values.map((v) => {
            const isObj = typeof v === "object" && v !== null;
            const key = isObj ? v.id : v;
            const label = isObj ? (v.name ?? v.id) : v;
            return (
              <span key={key} className="text-xs bg-element border border-normal px-1 py-0.5 rounded font-mono" title={isObj ? v.description : undefined}>
                {label}
              </span>
            );
          })}
        </div>
      )}
      {prop.levels && prop.levels.length > 0 && (
        <div className="mt-2 space-y-2">
          <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
            {levelsTitle} ({prop.levels.length})
          </span>
          {prop.levels.map((level) => (
            <div key={level.value} className="rounded border border-normal p-2 space-y-1.5">
              <div className="flex items-start gap-2 text-xs">
                <span className="bg-active-base text-active-foreground px-1.5 py-0.5 rounded font-mono shrink-0">{level.value}</span>
                <div>
                  {level.name && <span className="font-medium text-foreground">{level.name}</span>}
                  {level.description && <p className="text-muted-foreground">{level.description}</p>}
                </div>
              </div>
              {level.measures && (
                <div className="space-y-1 pl-2 border-l-2 border-normal">
                  {level.measures.lower && level.measures.lower.length > 0 && (
                    <div>
                      <span className="text-xs font-semibold text-muted-foreground">↓ Lower measures:</span>
                      <div className="mt-0.5 space-y-0.5">
                        {level.measures.lower.map((lm) => (
                          <div key={lm.id} className="text-xs flex items-start gap-1">
                            <span className="bg-info-bg text-info-foreground px-1 py-0.5 rounded font-mono shrink-0">{lm.id}</span>
                            {lm.instruction && <span className="text-muted-foreground">{lm.instruction}</span>}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                  {level.measures.raise && level.measures.raise.length > 0 && (
                    <div>
                      <span className="text-xs font-semibold text-muted-foreground">↑ Raise measures:</span>
                      <div className="mt-0.5 space-y-0.5">
                        {level.measures.raise.map((hm) => (
                          <div key={hm.id} className="text-xs flex items-start gap-1">
                            <span className="bg-info-bg text-info-foreground px-1 py-0.5 rounded font-mono shrink-0">{hm.id}</span>
                            {hm.instruction && <span className="text-muted-foreground">{hm.instruction}</span>}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}
              {level.instructions && level.instructions.raise && level.instructions.raise.length > 0 && (
                <div className="space-y-1 pl-2 border-l-2 border-normal">
                  <span className="text-xs font-semibold text-muted-foreground">↑ Raise instructions:</span>
                  <div className="mt-0.5 space-y-0.5">
                    {level.instructions.raise.map((hi) => (
                      <div key={hi.id} className="text-xs flex items-start gap-1">
                        <span className="bg-info-bg text-info-foreground px-1 py-0.5 rounded font-mono shrink-0">{hi.id}</span>
                        {hi.instruction && <span className="text-muted-foreground">{hi.instruction}</span>}
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
      {prop.items && (
        <div className="mt-2">
          <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">{itemsTitle}</span>
          {prop.items.properties && prop.items.properties.length > 0 && (
            <div className="mt-1 space-y-1.5">
              {prop.items.properties.map((child) => (
                <PropertyView key={child.id} prop={child} depth={depth + 1} />
              ))}
            </div>
          )}
        </div>
      )}
      {prop.properties && prop.properties.length > 0 && (
        <div className="mt-2">
          <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
            {propertiesTitle} ({prop.properties.length})
          </span>
          <div className="mt-1 space-y-1.5">
            {prop.properties.map((child) => (
              <PropertyView key={child.id} prop={child} depth={depth + 1} />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

/**
 * Configuration page showing properties, frameworks (with properties and rules), and platforms.
 *MUST display all coda configuration in expandable sections.
 * Properties and frameworks are general (not project-scoped).
 **/
function ConfigPage({ refreshKey }: { refreshKey: number }) {
  const { data: propertyKinds, loading: propertyKindsLoading } = useCodaResource<PropertyKindMap>("coda://property-kinds", refreshKey);
  const { data: properties, loading: propertiesLoading } = useCodaResource<Property[]>("coda://properties", refreshKey);
  const { data: correlation, loading: correlationLoading } = useCodaResource<Correlation>("coda://correlation", refreshKey);
  const { data: frameworks, loading: frameworksLoading } = useCodaResource<Framework[]>("coda://frameworks", refreshKey);
  const { data: platforms, loading: platformsLoading } = useCodaResource<Platform[]>("coda://platforms", refreshKey);

  const loading = propertyKindsLoading || propertiesLoading || correlationLoading || frameworksLoading || platformsLoading;
  const loadingConfigLabel = useLabel(codaKey("coda.loading.config"));
  const configTitle = useLabel(codaKey("coda.page.config.title"));
  const propertiesTitle = useLabel(codaKey("coda.section.properties.title"));
  const rulesTitle = useLabel(codaKey("coda.section.rules.title"));
  if (loading) return <Spinner label={loadingConfigLabel} />;

  const totalPropertyCount = properties ? countProperties(properties) : 0;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">{configTitle}</h2>
        <p className="text-sm text-muted-foreground mt-1">Property kinds, properties, correlation, frameworks, and platforms from the coda configuration.</p>
      </div>

      <Card title={`Property Kinds (${propertyKinds ? Object.keys(propertyKinds).length : 0})`} action={<span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">general</span>}>
        {propertyKinds && Object.keys(propertyKinds).length > 0 ? (
          <div className="space-y-2">
            {Object.entries(propertyKinds).map(([kind, def]) => (
              <div key={kind} className="rounded bg-element border border-normal p-2">
                <span className="text-sm font-medium text-foreground font-mono">{kind}</span>
                {(def.measures ?? def.measure) && (
                  <div className="mt-1 flex flex-wrap gap-1">
                    <span className="text-xs text-muted-foreground mr-1">Measures:</span>
                    {(def.measures ?? def.measure ?? []).map((m) => (
                      <span key={m} className="text-xs bg-active-base text-active-foreground px-1.5 py-0.5 rounded font-mono">
                        {m}
                      </span>
                    ))}
                  </div>
                )}
              </div>
            ))}
          </div>
        ) : (
          <EmptyState message="No property kinds configured." />
        )}
      </Card>

      <Card title={`${propertiesTitle} (${totalPropertyCount})`} action={<span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">general</span>}>
        {properties && properties.length > 0 ? (
          <div className="space-y-2">
            {properties.map((prop) => (
              <PropertyView key={prop.id} prop={prop} />
            ))}
          </div>
        ) : (
          <EmptyState message="No properties configured." />
        )}
      </Card>

      {correlation && correlation.properties && correlation.properties.length > 0 && (
        <Card title={`Correlation Matrix (${correlation.properties.length} properties)`} action={<span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">general</span>}>
          <div className="overflow-x-auto">
            <table className="text-xs font-mono border-collapse">
              <thead>
                <tr>
                  <th className="p-1 text-left text-muted-foreground" />
                  {correlation.properties.map((p) => (
                    <th key={p} className="p-1 text-center text-muted-foreground whitespace-nowrap">
                      {formatId(p)}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {correlation.matrix.map((row, i) => (
                  <tr key={correlation.properties[i]}>
                    <td className="p-1 text-muted-foreground whitespace-nowrap">{formatId(correlation.properties[i])}</td>
                    {row.map((val, j) => (
                      <td key={j} className={`p-1 text-center ${i === j ? "text-foreground font-bold" : val > 0 ? "text-active-base" : val < 0 ? "text-destructive-foreground" : "text-muted-foreground"}`}>
                        {val.toFixed(2)}
                      </td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </Card>
      )}

      <Card title={`Frameworks (${frameworks?.length ?? 0})`} action={<span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">general</span>}>
        {frameworks && frameworks.length > 0 ? (
          <div className="space-y-3">
            {frameworks.map((fw) => (
              <Collapsible
                key={fw.id}
                title={formatId(fw.id)}
                badge={
                  <span className="text-xs text-muted-foreground">
                    {fw.properties?.length ?? 0} properties, {fw.rules?.length ?? 0} rules
                  </span>
                }
              >
                <div className="space-y-3">
                  {fw.properties && fw.properties.length > 0 && (
                    <div>
                      <h4 className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">{propertiesTitle}</h4>
                      <div className="space-y-1.5">
                        {fw.properties.map((prop) => (
                          <PropertyView key={prop.id} prop={prop} />
                        ))}
                      </div>
                    </div>
                  )}

                  {fw.rules && fw.rules.length > 0 && (
                    <div>
                      <h4 className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">{rulesTitle}</h4>
                      <div className="space-y-1.5">
                        {fw.rules.map((rule) => (
                          <div key={rule.id} className="rounded bg-element border border-normal p-2">
                            <div className="text-sm font-medium text-foreground">{formatId(rule.id)}</div>
                            {rule.description && <p className="text-xs text-muted-foreground mt-0.5">{rule.description}</p>}
                            {rule.clauses && rule.clauses.length > 0 && (
                              <div className="mt-2 pl-3 border-l-2 border-normal space-y-1">
                                {rule.clauses.map((clause) => (
                                  <div key={clause.id} className="text-xs">
                                    <span className="font-medium text-foreground">{formatId(clause.id)}</span>
                                    {clause.description && <span className="text-muted-foreground"> — {clause.description}</span>}
                                    {clause.properties && clause.properties.length > 0 && (
                                      <div className="mt-0.5 flex flex-wrap gap-1">
                                        {clause.properties.map((cp, idx) => (
                                          <span key={`${cp.id}-${cp.value}-${idx}`} className="bg-element border border-normal px-1 py-0.5 rounded font-mono text-muted-foreground">
                                            {cp.id}={cp.value}
                                          </span>
                                        ))}
                                      </div>
                                    )}
                                  </div>
                                ))}
                              </div>
                            )}
                            {rule.measures && rule.measures.length > 0 && (
                              <div className="mt-1.5 flex flex-wrap gap-1">
                                {rule.measures.map((m) => (
                                  <span key={m} className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded font-mono">
                                    {m}
                                  </span>
                                ))}
                              </div>
                            )}
                            {rule.data && Object.keys(rule.data).length > 0 && (
                              <div className="mt-1.5">
                                <span className="text-xs text-muted-foreground">Data schema:</span>
                                <div className="mt-0.5 flex flex-wrap gap-1">
                                  {Object.entries(rule.data).map(([k, v]) => (
                                    <span key={k} className="text-xs bg-element border border-normal px-1 py-0.5 rounded font-mono text-muted-foreground">
                                      {k}: {String(v)}
                                    </span>
                                  ))}
                                </div>
                              </div>
                            )}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              </Collapsible>
            ))}
          </div>
        ) : (
          <EmptyState message="No frameworks configured." />
        )}
      </Card>

      <Card title={`Platforms (${platforms?.length ?? 0})`}>
        {platforms && platforms.length > 0 ? (
          <div className="space-y-3">
            {platforms.map((platform) => (
              <Collapsible key={platform.id} title={formatId(platform.id)} badge={<span className="text-xs text-muted-foreground">{platform.properties?.length ?? 0} properties</span>}>
                {platform.properties && platform.properties.length > 0 ? (
                  <div className="space-y-2">
                    {platform.properties.map((pp) => {
                      const measureKindKeys = Object.keys(pp).filter((k) => k !== "id" && typeof pp[k] === "object");
                      return (
                        <div key={pp.id} className="rounded bg-element border border-normal p-2">
                          <div className="text-sm font-medium text-foreground font-mono">{pp.id}</div>
                          {measureKindKeys.length > 0 && (
                            <div className="mt-1.5 space-y-2">
                              {measureKindKeys.map((mk) => {
                                const mi = pp[mk] as PlatformMeasureInstruction;
                                return (
                                  <div key={mk} className="pl-2 border-l-2 border-active-base">
                                    <span className="text-xs bg-active-base text-active-foreground px-1.5 py-0.5 rounded font-mono">{mk}</span>
                                    {mi.instructions && <p className="text-xs text-muted-foreground mt-0.5">{mi.instructions}</p>}
                                    {mi.mcp?.resources && mi.mcp.resources.length > 0 && (
                                      <div className="mt-1">
                                        <span className="text-xs font-semibold text-muted-foreground">MCP Resources:</span>
                                        <div className="mt-0.5 space-y-0.5">
                                          {mi.mcp.resources.map((r) => (
                                            <div key={r.id} className="text-xs pl-2 border-l border-normal">
                                              <span className="font-mono text-active-base">{r.id}</span>
                                              {r.instruction && <span className="text-muted-foreground"> — {r.instruction}</span>}
                                            </div>
                                          ))}
                                        </div>
                                      </div>
                                    )}
                                    {mi.mcp?.tools && mi.mcp.tools.length > 0 && (
                                      <div className="mt-1">
                                        <span className="text-xs font-semibold text-muted-foreground">MCP Tools:</span>
                                        <div className="mt-0.5 space-y-0.5">
                                          {mi.mcp.tools.map((tool) => (
                                            <div key={tool.id} className="text-xs pl-2 border-l border-normal">
                                              <span className="font-mono text-active-base">{tool.id}</span>
                                              {tool.instruction && <span className="text-muted-foreground"> — {tool.instruction}</span>}
                                              {tool.parameters && tool.parameters.length > 0 && (
                                                <div className="mt-0.5 pl-2 space-y-0.5">
                                                  {tool.parameters.map((param) => (
                                                    <div key={param.id} className="text-xs text-muted-foreground">
                                                      <span className="font-mono">{param.id}</span>
                                                      {param.instruction && <span> — {param.instruction}</span>}
                                                    </div>
                                                  ))}
                                                </div>
                                              )}
                                            </div>
                                          ))}
                                        </div>
                                      </div>
                                    )}
                                  </div>
                                );
                              })}
                            </div>
                          )}
                        </div>
                      );
                    })}
                  </div>
                ) : (
                  <p className="text-xs text-muted-foreground">No property instructions.</p>
                )}
              </Collapsible>
            ))}
          </div>
        ) : (
          <EmptyState message="No platforms configured." />
        )}
      </Card>
    </div>
  );
}

// #endregion 📣ConfigPage

// #region 💡RunsPage

/**
 * Runs page showing current run, iterations list, and iteration details.
 *MUST display the current run, all iterations, and the current iteration detail.
 **/
function RunsPage({ refreshKey }: { refreshKey: number }) {
  const { data: run, loading: runLoading, error: runError } = useCodaResource<Run>("coda://current-run", refreshKey);
  const { data: iterations, loading: itersLoading } = useCodaResource<Array<{ index: string }>>("coda://iterations", refreshKey);
  const { data: iteration, loading: iterLoading } = useCodaResource<Iteration>("coda://current-iteration", refreshKey);

  const loading = runLoading || itersLoading || iterLoading;
  const loadingRunsLabel = useLabel(codaKey("coda.loading.runs"));
  const runsTitle = useLabel(codaKey("coda.page.runs.title"));
  const currentRunTitle = useLabel(codaKey("coda.card.currentRun.title"));
  const idColumn = useLabel(codaKey("coda.column.id"));
  const startedColumn = useLabel(codaKey("coda.column.started"));
  const noRunsMessage = useLabel(codaKey("coda.empty.noRuns"));
  const noIterationsMessage = useLabel(codaKey("coda.empty.noIterations"));
  const targetsTitle = useLabel(codaKey("coda.section.targets.title"));
  if (loading) return <Spinner label={loadingRunsLabel} />;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">{runsTitle}</h2>
        <p className="text-sm text-muted-foreground mt-1">Manage and inspect compliance checking runs.</p>
      </div>

      <Card title={currentRunTitle}>
        {runError ? (
          <EmptyState message={runError} />
        ) : run ? (
          <div className="space-y-2">
            <div className="flex items-center gap-3">
              <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider w-16">{idColumn}</span>
              <span className="text-sm font-mono text-foreground">{run.id ?? run.run_id ?? "—"}</span>
            </div>
            {run.started && (
              <div className="flex items-center gap-3">
                <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider w-16">{startedColumn}</span>
                <span className="text-sm text-foreground">{run.started}</span>
              </div>
            )}
          </div>
        ) : (
          <EmptyState message={noRunsMessage ?? ""} />
        )}
      </Card>

      <Card title={`Iterations (${iterations?.length ?? 0})`}>
        {iterations && iterations.length > 0 ? (
          <div className="space-y-1">
            {iterations.map((iter) => (
              <div key={iter.index} className={`flex items-center gap-3 rounded-md border px-3 py-2 text-sm ${String(iteration?.index) === iter.index ? "border-active-base bg-info-bg" : "border-normal hover:bg-hover-interactive-fill"}`}>
                <span className="font-mono font-bold text-active-base">#{iter.index}</span>
                {String(iteration?.index) === iter.index && <span className="text-xs bg-active-base text-active-foreground px-1.5 py-0.5 rounded">current</span>}
              </div>
            ))}
          </div>
        ) : (
          <EmptyState message={noIterationsMessage ?? ""} />
        )}
      </Card>

      {iteration && !iteration.error && (
        <Card title={`Current Iteration #${iteration.index}`}>
          <div className="space-y-2">
            {iteration.targets && iteration.targets.length > 0 && (
              <div>
                <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">{targetsTitle}</span>
                <div className="mt-1 flex flex-wrap gap-1.5">
                  {iteration.targets.map((tid) => (
                    <span key={tid} className="text-xs bg-element border border-normal px-2 py-1 rounded font-mono">
                      {tid}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>
        </Card>
      )}
    </div>
  );
}

// #endregion 💡RunsPage

// #region 📋ReportPage

/**
 * Report page showing compliance report with validation trees.
 *MUST display the full report with expandable validation trees showing truth values.
 **/
function ReportPage({ refreshKey }: { refreshKey: number }) {
  const { data: report, loading: reportLoading, error: reportError } = useCodaResource<Report>("coda://report", refreshKey);
  const loadingReportLabel = useLabel(codaKey("coda.loading.report"));
  const reportTitle = useLabel(codaKey("coda.page.report.title"));

  if (reportLoading) return <Spinner label={loadingReportLabel} />;

  const totalValidations = report?.validations?.length ?? 0;
  const violatedValidations = report?.validations?.filter((v) => v.truth === "false") ?? [];
  const compliantValidations = report?.validations?.filter((v) => v.truth === "true") ?? [];
  const unknownValidations = report?.validations?.filter((v) => v.truth === "unknown") ?? [];

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">{reportTitle}</h2>
        <p className="text-sm text-muted-foreground mt-1">Results from the latest validation iteration.</p>
      </div>

      {reportError ? (
        <Card>
          <EmptyState message={reportError} />
        </Card>
      ) : !report?.validations || report.validations.length === 0 ? (
        <Card>
          <EmptyState message="No report available. Run validation from the Actions page." />
        </Card>
      ) : (
        <>
          <div className="grid grid-cols-3 gap-4">
            <StatCard label="Total" value={totalValidations} />
            <StatCard label="Compliant" value={compliantValidations.length} sublabel={totalValidations > 0 ? `${Math.round((compliantValidations.length / totalValidations) * 100)}%` : undefined} />
            <StatCard label="Violated" value={violatedValidations.length} sublabel={totalValidations > 0 ? `${Math.round((violatedValidations.length / totalValidations) * 100)}%` : undefined} />
          </div>

          {violatedValidations.length > 0 && (
            <Card title={`Violations (${violatedValidations.length})`}>
              <div className="space-y-4">
                {violatedValidations.map((validation) => (
                  <ValidationTree key={validation.instance} report={validation} defaultExpanded={true} />
                ))}
              </div>
            </Card>
          )}

          {unknownValidations.length > 0 && (
            <Card title={`Unknown (${unknownValidations.length})`}>
              <div className="space-y-4">
                {unknownValidations.map((validation) => (
                  <ValidationTree key={validation.instance} report={validation} defaultExpanded={false} />
                ))}
              </div>
            </Card>
          )}

          {compliantValidations.length > 0 && (
            <Card title={`Compliant (${compliantValidations.length})`}>
              <div className="space-y-4">
                {compliantValidations.map((validation) => (
                  <ValidationTree key={validation.instance} report={validation} defaultExpanded={false} />
                ))}
              </div>
            </Card>
          )}
        </>
      )}
    </div>
  );
}

// #endregion 📋ReportPage

// #region 📊TranslationsPage

/**
 * Translations page showing translation outputs per target.
 *MUST display translation data for each project target.
 **/
function TranslationsPage({ refreshKey }: { refreshKey: number }) {
  const { data: project } = useCodaResource<Project>("coda://project", refreshKey);
  const targetIds = reactHostPort.useMemo(() => project?.targets?.map((t) => t.id) ?? [], [project]);
  const translationsTitle = useLabel(codaKey("coda.page.translations.title"));

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">{translationsTitle}</h2>
        <p className="text-sm text-muted-foreground mt-1">Translation outputs for each target in the current iteration.</p>
      </div>

      {targetIds.length === 0 ? (
        <Card>
          <EmptyState message="No project targets found." />
        </Card>
      ) : (
        targetIds.map((tid) => <TranslationCard key={tid} targetId={tid} refreshKey={refreshKey} />)
      )}
    </div>
  );
}

/**
 * A card that fetches and displays a single target translation.
 *MUST handle loading and error states for translation data.
 **/
function TranslationCard({ targetId, refreshKey }: { targetId: string; refreshKey: number }) {
  const { data, loading, error } = useCodaResource<Record<string, unknown>>(`coda://translation/${targetId}`, refreshKey);

  return (
    <Card title={formatId(targetId)}>{loading ? <Spinner label={`Loading ${targetId} translation...`} /> : error ? <EmptyState message={error} /> : data ? <JsonViewer data={data} /> : <EmptyState message="No translation data available." />}</Card>
  );
}

// #endregion 📊TranslationsPage

// #region 📡ActionsPage

/**
 * Actions page for invoking coda MCP tools.
 *MUST provide buttons for all coda tools and display results.
 **/
function ActionsPage({ refreshKey, onRefresh }: { refreshKey: number; onRefresh: () => void }) {
  const { data: project } = useCodaResource<Project>("coda://project", refreshKey);
  const targetIds = reactHostPort.useMemo(() => project?.targets?.map((t) => t.id) ?? [], [project]);

  const [actionLog, setActionLog] = reactHostPort.useState<Array<{ id: number; action: string; result: unknown; timestamp: string; success: boolean }>>([]);
  const [loading, setLoading] = reactHostPort.useState<string | null>(null);
  const actionsTitle = useLabel(codaKey("coda.page.actions.title"));
  const runManagementTitle = useLabel(codaKey("coda.card.runManagement.title"));
  const translationValidationTitle = useLabel(codaKey("coda.card.translationValidation.title"));
  const fixDesignTitle = useLabel(codaKey("coda.card.fixDesign.title"));
  const manualFixResultTitle = useLabel(codaKey("coda.card.manualFixResult.title"));
  const actionLogTitle = useLabel(codaKey("coda.card.actionLog.title"));

  const runTool = reactHostPort.useCallback(
    async (name: string, args: Record<string, unknown>, label: string) => {
      setLoading(label);
      try {
        const response = await window.coda.tool(name, args);
        const result = parseMcpTool(response);
        setActionLog((prev) => [{ id: Date.now(), action: label, result: result ?? response.error ?? "No response", timestamp: new Date().toLocaleTimeString(), success: !response.error }, ...prev]);
        onRefresh();
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : "Unknown error";
        setActionLog((prev) => [{ id: Date.now(), action: label, result: { error: message }, timestamp: new Date().toLocaleTimeString(), success: false }, ...prev]);
      } finally {
        setLoading(null);
      }
    },
    [onRefresh],
  );

  const runCall = reactHostPort.useCallback(
    async (method: string, params: Record<string, unknown>, label: string) => {
      setLoading(label);
      try {
        const response = await window.coda.call(method, params);
        const result = response.result ?? response.error ?? "No response";
        setActionLog((prev) => [{ id: Date.now(), action: label, result, timestamp: new Date().toLocaleTimeString(), success: !response.error }, ...prev]);
        onRefresh();
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : "Unknown error";
        setActionLog((prev) => [{ id: Date.now(), action: label, result: { error: message }, timestamp: new Date().toLocaleTimeString(), success: false }, ...prev]);
      } finally {
        setLoading(null);
      }
    },
    [onRefresh],
  );

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">{actionsTitle}</h2>
        <p className="text-sm text-muted-foreground mt-1">Invoke coda tools to run compliance checking workflows.</p>
      </div>

      <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
        <Card title={runManagementTitle}>
          <div className="space-y-3">
            <div className="flex items-start gap-3">
              <Button variant="primary" onClick={() => runTool("start_run", {}, "Start Run")} loading={loading === "Start Run"} disabled={loading !== null}>
                <IconPlay className="w-3.5 h-3.5" />
                Start Run
              </Button>
              <p className="text-xs text-muted-foreground pt-1">Create a new compliance checking run.</p>
            </div>
            <div className="flex items-start gap-3">
              <Button variant="primary" onClick={() => runTool("start_iteration", {}, "Start Iteration")} loading={loading === "Start Iteration"} disabled={loading !== null}>
                <IconRuns className="w-3.5 h-3.5" />
                Start Iteration
              </Button>
              <p className="text-xs text-muted-foreground pt-1">Begin a new iteration in the current run.</p>
            </div>
          </div>
        </Card>

        <Card title={translationValidationTitle}>
          <div className="space-y-3">{targetIds.length === 0 ? <EmptyState message="No project targets found." /> : targetIds.map((tid) => <TargetActionCard key={tid} targetId={tid} loading={loading} runTool={runTool} runCall={runCall} />)}</div>
        </Card>
      </div>

      <Card title={fixDesignTitle}>
        <FixAction loading={loading} onFix={(prompt) => runTool("fix", { prompt }, `Fix: ${prompt.slice(0, 30)}...`)} disabled={loading !== null} />
      </Card>

      <Card title={manualFixResultTitle}>
        <ManualFixInput loading={loading} onSubmit={(result) => runCall("save_report", { report_data: typeof result === "string" ? result : JSON.stringify(result) }, "Manual Save Report")} disabled={loading !== null} />
      </Card>

      {actionLog.length > 0 && (
        <Card
          title={actionLogTitle}
          action={
            <Button onClick={() => setActionLog([])} className="text-xs">
              Clear
            </Button>
          }
        >
          <div className="space-y-2 max-h-96 overflow-y-auto">
            {actionLog.map((entry) => (
              <Collapsible
                key={entry.id}
                title={entry.action}
                badge={
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-muted-foreground">{entry.timestamp}</span>
                    <span className={`w-2 h-2 rounded-full ${entry.success ? "bg-success-border" : "bg-destructive-border"}`} />
                  </div>
                }
              >
                <JsonViewer data={entry.result} />
              </Collapsible>
            ))}
          </div>
        </Card>
      )}
    </div>
  );
}

/**
 * Per-target action card with translate/validate buttons and manual input.
 *MUST offer tool invocation and manual result input for translate and validate.
 **/
function TargetActionCard({
  targetId,
  loading,
  runTool,
  runCall,
}: {
  targetId: string;
  loading: string | null;
  runTool: (name: string, args: Record<string, unknown>, label: string) => void;
  runCall: (method: string, params: Record<string, unknown>, label: string) => void;
}) {
  const [manualMode, setManualMode] = reactHostPort.useState<null | "translate" | "validate">(null);
  const [manualInput, setManualInput] = reactHostPort.useState("");

  const handleManualSubmit = reactHostPort.useCallback(() => {
    if (!manualInput.trim() || !manualMode) return;
    try {
      const parsed = JSON.parse(manualInput);
      if (manualMode === "translate") {
        runCall("save_translation", { target_id: targetId, data: typeof parsed === "string" ? parsed : JSON.stringify(parsed) }, `Manual Translate ${targetId}`);
      } else {
        runCall("save_validation", { target_id: targetId, data: typeof parsed === "string" ? parsed : JSON.stringify(parsed) }, `Manual Validate ${targetId}`);
      }
      setManualInput("");
      setManualMode(null);
    } catch {
      // Input is not valid JSON — send as plain text
      if (manualMode === "translate") {
        runCall("save_translation", { target_id: targetId, data: manualInput.trim() }, `Manual Translate ${targetId}`);
      } else {
        runCall("save_validation", { target_id: targetId, data: manualInput.trim() }, `Manual Validate ${targetId}`);
      }
      setManualInput("");
      setManualMode(null);
    }
  }, [manualInput, manualMode, targetId, runTool, runCall]);

  return (
    <div className="rounded border border-normal p-3 space-y-2">
      <div className="text-sm font-medium text-foreground font-mono">{targetId}</div>
      <div className="flex gap-2">
        <Button onClick={() => runTool("translate", { target_id: targetId }, `Translate ${targetId}`)} loading={loading === `Translate ${targetId}`} disabled={loading !== null}>
          <IconTranslations className="w-3.5 h-3.5" />
          Translate
        </Button>
        <Button onClick={() => runTool("validate", { target_id: targetId }, `Validate ${targetId}`)} loading={loading === `Validate ${targetId}`} disabled={loading !== null}>
          <IconCheck className="w-3.5 h-3.5" />
          Validate
        </Button>
        <Button onClick={() => setManualMode(manualMode ? null : "translate")} variant={manualMode ? "primary" : "secondary"} className="ml-auto text-xs">
          Manual
        </Button>
      </div>
      {manualMode && (
        <div className="space-y-2 pt-1">
          <div className="flex gap-2">
            <button
              onClick={() => {
                setManualMode("translate");
                setManualInput("");
              }}
              className={`text-xs px-2 py-1 rounded-full border cursor-pointer transition-colors ${
                manualMode === "translate" ? "bg-active-base text-active-foreground border-active-base" : "border-normal text-muted-foreground hover:bg-hover-interactive-fill"
              }`}
            >
              Translation
            </button>
            <button
              onClick={() => {
                setManualMode("validate");
                setManualInput("");
              }}
              className={`text-xs px-2 py-1 rounded-full border cursor-pointer transition-colors ${
                manualMode === "validate" ? "bg-active-base text-active-foreground border-active-base" : "border-normal text-muted-foreground hover:bg-hover-interactive-fill"
              }`}
            >
              Validation
            </button>
          </div>
          <textarea
            value={manualInput}
            onChange={(e) => setManualInput(e.target.value)}
            placeholder={`Paste ${manualMode} result JSON here...`}
            rows={5}
            className="w-full rounded-md border border-normal bg-element px-3 py-2 text-xs font-mono text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base resize-y"
          />
          <div className="flex gap-2 justify-end">
            <Button
              onClick={() => {
                setManualMode(null);
                setManualInput("");
              }}
              variant="secondary"
              className="text-xs"
            >
              Cancel
            </Button>
            <Button onClick={handleManualSubmit} variant="primary" disabled={!manualInput.trim() || loading !== null} className="text-xs">
              Save {manualMode === "translate" ? "Translation" : "Validation"}
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}

/**
 * Manual fix result input form.
 *MUST provide a textarea to paste fix results and submit them.
 **/
function ManualFixInput({ loading, onSubmit, disabled }: { loading: string | null; onSubmit: (result: unknown) => void; disabled: boolean }) {
  const [input, setInput] = reactHostPort.useState("");
  const fixResultJsonPlaceholder = useLabel(codaKey("coda.placeholder.fixResultJson"));
  const handleSubmit = () => {
    if (!input.trim()) return;
    try {
      const parsed = JSON.parse(input);
      onSubmit(parsed);
    } catch {
      onSubmit(input.trim());
    }
    setInput("");
  };
  return (
    <div className="space-y-3">
      <p className="text-xs text-muted-foreground">Paste the fix result (report JSON) from an agent to manually save it.</p>
      <textarea
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder={fixResultJsonPlaceholder}
        rows={4}
        className="w-full rounded-md border border-normal bg-element px-3 py-2 text-xs font-mono text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base resize-y"
      />
      <div className="flex justify-end">
        <Button onClick={handleSubmit} variant="primary" loading={loading === "Manual Save Report"} disabled={disabled || !input.trim()} className="text-xs">
          Save Report
        </Button>
      </div>
    </div>
  );
}

/**
 * Fix action form with prompt input.
 *MUST provide a text input for the fix prompt.
 **/
function FixAction({ loading, onFix, disabled }: { loading: string | null; onFix: (prompt: string) => void; disabled: boolean }) {
  const [prompt, setPrompt] = reactHostPort.useState("");
  const fixDescriptionExamplePlaceholder = useLabel(codaKey("coda.placeholder.fixDescriptionExample"));
  const handleSubmit = () => {
    if (prompt.trim()) {
      onFix(prompt.trim());
      setPrompt("");
    }
  };
  return (
    <div className="space-y-3">
      <p className="text-xs text-muted-foreground">Describe what should be fixed in the design to address compliance breaches.</p>
      <div className="flex gap-2">
        <input
          type="text"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSubmit()}
          placeholder={fixDescriptionExamplePlaceholder}
          className="flex-1 rounded-md border border-normal bg-element px-3 py-1.5 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base"
        />
        <Button variant="danger" onClick={handleSubmit} loading={loading?.startsWith("Fix:") ?? false} disabled={disabled || !prompt.trim()}>
          <IconWrench className="w-3.5 h-3.5" />
          Fix
        </Button>
      </div>
    </div>
  );
}

// #endregion 📡ActionsPage

// #region 🗃️EventsPage
// Events page showing real-time event stream from the coda sidecar process.
// EventsPage MUST display all events with timestamps, kind, and full data.
// EventsPage MUST allow clearing and filtering events.

/**
 * Events page showing the real-time event log from the sidecar.
 *MUST display all events in reverse chronological order.
 * MUST show event kind, timestamp, and full data payload.
 **/
function EventsPage({ events, onClear }: { events: CodaEvent[]; onClear: () => void }) {
  const [filter, setFilter] = reactHostPort.useState("");
  const eventsTitle = useLabel(codaKey("coda.page.events.title"));
  const filterEventsPlaceholder = useLabel(codaKey("coda.placeholder.filterEvents"));

  const filteredEvents = reactHostPort.useMemo(() => {
    if (!filter.trim()) return events;
    const lower = filter.toLowerCase();
    return events.filter((e) => e.event.toLowerCase().includes(lower) || JSON.stringify(e.data).toLowerCase().includes(lower));
  }, [events, filter]);

  const uniqueKinds = reactHostPort.useMemo(() => {
    const kinds = new Set(events.map((e) => e.event));
    return Array.from(kinds).sort();
  }, [events]);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-lg font-bold text-foreground">{eventsTitle}</h2>
          <p className="text-sm text-muted-foreground mt-1">Real-time event stream from the coda sidecar ({events.length} total).</p>
        </div>
        <div className="flex items-center gap-2">
          <Button onClick={onClear} variant="secondary" disabled={events.length === 0}>
            Clear
          </Button>
        </div>
      </div>

      <div className="flex items-center gap-3">
        <input
          type="text"
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          placeholder={filterEventsPlaceholder}
          className="flex-1 rounded-md border border-normal bg-element px-3 py-1.5 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base"
        />
        {uniqueKinds.length > 0 && (
          <div className="flex flex-wrap gap-1">
            {uniqueKinds.map((kind) => (
              <button
                key={kind}
                onClick={() => setFilter(filter === kind ? "" : kind)}
                className={`text-xs px-2 py-1 rounded-full border cursor-pointer transition-colors ${filter === kind ? "bg-active-base text-active-foreground border-active-base" : "border-normal text-muted-foreground hover:bg-hover-interactive-fill"}`}
              >
                {kind}
              </button>
            ))}
          </div>
        )}
      </div>

      {filteredEvents.length === 0 ? (
        <Card>
          <EmptyState message={events.length === 0 ? "No events received yet. Events will appear here as the sidecar processes requests." : "No events match the current filter."} />
        </Card>
      ) : (
        <div className="space-y-2">
          {filteredEvents.map((evt, idx) => {
            const ts = new Date(evt.timestamp * 1000);
            const timeStr = ts.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit", second: "2-digit", fractionalSecondDigits: 3 });
            return (
              <Collapsible key={`${evt.timestamp}-${idx}`} title={evt.event} badge={<span className="text-xs text-muted-foreground font-mono">{timeStr}</span>}>
                <JsonViewer data={evt.data} />
              </Collapsible>
            );
          })}
        </div>
      )}
    </div>
  );
}

// #endregion 🗃️EventsPage

// #endregion 🎉Pages

// #region 🧬Welcome
// Welcome screen shown on startup when no project is open.
// MUST offer two options: create a new project or open an existing one.

/**
 * Welcome screen with options to create or open a project.
 *MUST show create-new-project form and open-existing-project button.
 * MUST call onProjectReady with the resolved project path on success.
 **/
function WelcomePage({ onProjectReady, onMinimize, onMaximize, onClose }: { onProjectReady: (projectPath: string) => void; onMinimize: () => void; onMaximize: () => void; onClose: () => void }) {
  const [mode, setMode] = reactHostPort.useState<"choose" | "create" | "open">("choose");
  const [projectName, setProjectName] = reactHostPort.useState("");
  const [selectedFolder, setSelectedFolder] = reactHostPort.useState<string | null>(null);
  const [error, setError] = reactHostPort.useState<string | null>(null);
  const [loading, setLoading] = reactHostPort.useState(false);
  const titlebarSubtitle = useLabel(codaKey("coda.titlebar.subtitle"));
  const createProjectTitle = useLabel(codaKey("coda.welcome.createProject.title"));
  const openProjectTitle = useLabel(codaKey("coda.welcome.openProject.title"));
  const projectNameLabel = useLabel(codaKey("coda.welcome.projectName.label"));
  const projectFolderLabel = useLabel(codaKey("coda.welcome.projectFolder.label"));
  const projectNamePlaceholder = useLabel(codaKey("coda.placeholder.projectName"));
  const { level: boxLevel, opensLevel: boxOpensLevel } = useDockedPanelLevel();

  const handlePickFolder = reactHostPort.useCallback(async () => {
    const folder = await window.dialog.openFolder();
    if (folder) {
      setSelectedFolder(folder);
      setError(null);
    }
  }, []);

  const handleCreate = reactHostPort.useCallback(async () => {
    if (!projectName.trim()) {
      setError("Project name is required.");
      return;
    }
    if (!selectedFolder) {
      setError("Please select a folder.");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const result = await window.project.create(projectName.trim(), selectedFolder);
      if (result.success) {
        onProjectReady(selectedFolder);
      } else {
        setError(result.error ?? "Failed to create project.");
      }
    } finally {
      setLoading(false);
    }
  }, [projectName, selectedFolder, onProjectReady]);

  const handleOpen = reactHostPort.useCallback(async () => {
    const folder = await window.dialog.openFolder();
    if (!folder) return;
    setLoading(true);
    setError(null);
    try {
      const result = await window.project.open(folder);
      if (result.success) {
        onProjectReady(folder);
      } else {
        setError(result.error ?? "Failed to open project.");
        setMode("open");
      }
    } finally {
      setLoading(false);
    }
  }, [onProjectReady]);

  return (
    <LevelProvider level="window">
    <div data-level="window" className="flex h-screen w-screen flex-col ui-surface overflow-hidden">
      {/* Title Bar -- window-level chrome ribbon (GlassTier "ribbon" -> ui-glass-chrome @ window) */}
      <div className="flex h-9 items-center border-b border-normal ui-glass-chrome px-3 shrink-0" style={{ WebkitAppRegion: "drag" } as React.CSSProperties}>
        <div className="flex items-center gap-2 flex-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <span className="text-sm font-bold text-active-base">coda</span>
          <span className="text-xs text-muted-foreground">{titlebarSubtitle}</span>
        </div>
        <div className="flex items-center gap-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <button onClick={onMinimize} className="rounded p-1.5 text-muted-foreground hover:bg-hover-interactive-fill hover:text-foreground transition-colors cursor-pointer">
            <IconMinimize />
          </button>
          <button onClick={onMaximize} className="rounded p-1.5 text-muted-foreground hover:bg-hover-interactive-fill hover:text-foreground transition-colors cursor-pointer">
            <IconMaximize />
          </button>
          <button onClick={onClose} className="rounded p-1.5 text-muted-foreground hover:bg-destructive-bg hover:text-destructive-foreground transition-colors cursor-pointer">
            <IconClose />
          </button>
        </div>
      </div>

      {/* Welcome Content */}
      <div className="flex flex-1 items-center justify-center p-8">
        <div className="w-full max-w-2xl space-y-8">
          <div className="text-center">
            <h1 className="text-3xl font-bold text-active-base">coda</h1>
            <p className="mt-2 text-muted-foreground">{titlebarSubtitle}</p>
          </div>

          {mode === "choose" && (
            <div className="grid grid-cols-2 gap-6">
              {/* #region Create New Project Card */}
              <button
                onClick={() => {
                  setMode("create");
                  setError(null);
                }}
                data-level={boxOpensLevel ? boxLevel : undefined}
                className={
                  boxOpensLevel
                    ? "group flex flex-col items-center gap-4 rounded-xl border-2 border-normal ui-surface p-8 text-left transition-all hover:border-active-base hover:bg-info-bg cursor-pointer"
                    : "group flex flex-col items-center gap-4 rounded-xl border-2 border-normal bg-transparent p-8 text-left transition-all hover:border-active-base hover:bg-info-bg cursor-pointer"
                }
              >
                <div className="rounded-full bg-info-bg p-4 transition-colors group-hover:bg-hover-interactive-fill">
                  <svg className="w-8 h-8 text-active-base" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                    <polyline points="14 2 14 8 20 8" />
                    <line x1="12" y1="18" x2="12" y2="12" />
                    <line x1="9" y1="15" x2="15" y2="15" />
                  </svg>
                </div>
                <div className="text-center">
                  <div className="text-base font-semibold text-foreground">{createProjectTitle}</div>
                  <p className="mt-1 text-sm text-muted-foreground">Start fresh with a new coda project in a folder of your choice.</p>
                </div>
              </button>
              {/* #endregion */}

              {/* #region Open Existing Project Card */}
              <button
                onClick={handleOpen}
                disabled={loading}
                data-level={boxOpensLevel ? boxLevel : undefined}
                className={
                  boxOpensLevel
                    ? "group flex flex-col items-center gap-4 rounded-xl border-2 border-normal ui-surface p-8 text-left transition-all hover:border-active-base hover:bg-info-bg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                    : "group flex flex-col items-center gap-4 rounded-xl border-2 border-normal bg-transparent p-8 text-left transition-all hover:border-active-base hover:bg-info-bg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                }
              >
                <div className="rounded-full bg-info-bg p-4 transition-colors group-hover:bg-hover-interactive-fill">
                  <svg className="w-8 h-8 text-active-base" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
                  </svg>
                </div>
                <div className="text-center">
                  <div className="text-base font-semibold text-foreground">{openProjectTitle}</div>
                  <p className="mt-1 text-sm text-muted-foreground">Open a folder that already contains a coda project configuration.</p>
                </div>
              </button>
              {/* #endregion */}
            </div>
          )}

          {(mode === "create" || mode === "open") && (
            <div data-level={boxOpensLevel ? boxLevel : undefined} className={boxOpensLevel ? "rounded-xl border border-normal ui-surface p-6 space-y-5" : "rounded-xl border border-normal bg-transparent p-6 space-y-5"}>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => {
                    setMode("choose");
                    setError(null);
                  }}
                  className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                >
                  <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                    <polyline points="15 18 9 12 15 6" />
                  </svg>
                </button>
                <h2 className="text-base font-semibold text-foreground">{mode === "create" ? createProjectTitle : openProjectTitle}</h2>
              </div>

              {mode === "create" && (
                <div className="space-y-4">
                  <div className="space-y-1.5">
                    <label className="text-sm font-medium text-foreground">{projectNameLabel}</label>
                    <input
                      type="text"
                      value={projectName}
                      onChange={(e) => setProjectName(e.target.value)}
                      onKeyDown={(e) => e.key === "Enter" && handleCreate()}
                      placeholder={projectNamePlaceholder}
                      className="w-full rounded-md border border-normal bg-element px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base"
                      autoFocus
                    />
                  </div>

                  <div className="space-y-1.5">
                    <label className="text-sm font-medium text-foreground">{projectFolderLabel}</label>
                    <div className="flex gap-2">
                      <div className="flex-1 rounded-md border border-normal bg-element px-3 py-2 text-sm text-muted-foreground truncate">{selectedFolder ?? "No folder selected"}</div>
                      <Button onClick={handlePickFolder} variant="secondary">
                        Browse…
                      </Button>
                    </div>
                    <p className="text-xs text-muted-foreground">
                      A <code className="font-mono">.coda/project.json</code> will be created in this folder.
                    </p>
                  </div>
                </div>
              )}

              {mode === "open" && (
                <div className="space-y-4">
                  <div className="flex gap-2">
                    <div className="flex-1 rounded-md border border-normal bg-element px-3 py-2 text-sm text-muted-foreground truncate">{selectedFolder ?? "No folder selected"}</div>
                    <Button onClick={handlePickFolder} variant="secondary">
                      Browse…
                    </Button>
                  </div>
                  <p className="text-xs text-muted-foreground">
                    Select a folder that contains a <code className="font-mono">.coda/project.json</code> file.
                  </p>
                </div>
              )}

              {error && <div className="rounded-md border border-destructive-border bg-destructive-bg px-3 py-2 text-sm text-destructive-foreground">{error}</div>}

              <div className="flex justify-end gap-2 pt-1">
                <Button
                  onClick={() => {
                    setMode("choose");
                    setError(null);
                  }}
                  variant="secondary"
                >
                  Cancel
                </Button>
                {mode === "create" ? (
                  <Button onClick={handleCreate} variant="primary" loading={loading} disabled={!projectName.trim() || !selectedFolder}>
                    Create Project
                  </Button>
                ) : (
                  <Button onClick={handleOpen} variant="primary" loading={loading} disabled={loading}>
                    Open Project
                  </Button>
                )}
              </div>
            </div>
          )}

          {error && mode === "choose" && <div className="rounded-md border border-destructive-border bg-destructive-bg px-3 py-2 text-sm text-destructive-foreground text-center">{error}</div>}
        </div>
      </div>
    </div>
    </LevelProvider>
  );
}

// #endregion 🧬Welcome

/**
 * Navigation item configuration.
 *MUST define all navigable pages with icons and labels.
 **/
const navItems: Array<{ id: Page; icon: React.ComponentType<{ className?: string }> }> = [
  { id: "dashboard", icon: IconDashboard },
  { id: "config", icon: IconConfig },
  { id: "runs", icon: IconRuns },
  { id: "report", icon: IconReport },
  { id: "translations", icon: IconTranslations },
  { id: "actions", icon: IconActions },
  { id: "events", icon: IconEvents },
];

// #region 🪨CodaProductShell

const CODA_APP_ID = "coda";
const CODA_CONTROLLER_ID = "coda.shell";
const CODA_SURFACE_MAIN = "coda.surface.main/v1";
const CODA_BODY_MAIN = "coda.window.main";
const CODA_PANEL_DOCUMENT_BODY = "coda.panel.document";
const CODA_PANEL_CATALOGUE_BODY = "coda.panel.catalogue";
const CODA_PANEL_INSPECTION_BODY = "coda.panel.inspection";

interface CodaShellSelection {
  readonly id: string;
  readonly label: string;
  readonly kind: string;
  readonly propertyId?: string;
  readonly validationNode?: ValidationTreeNode;
}

interface CodaShellSnapshot {
  readonly currentPage: Page;
  readonly refreshKey: number;
  readonly project: Project | null;
  readonly run: Run | null;
  readonly iteration: Iteration | null;
  readonly frameworks: Framework[] | null;
  readonly properties: Property[] | null;
  readonly report: Report | null;
  readonly selection: CodaShellSelection | null;
}

const CODA_EMPTY_SHELL_SNAPSHOT: CodaShellSnapshot = {
  currentPage: "dashboard",
  refreshKey: 0,
  project: null,
  run: null,
  iteration: null,
  frameworks: null,
  properties: null,
  report: null,
  selection: null,
};

let codaPlatformSingleton: Platform | null = null;
let codaShellControllerSingleton: CodaShellController | null = null;
let codaBodiesRegistered = false;

const codaMainHostBridge = {
  refreshKey: 0,
  events: [] as CodaEvent[],
  onClearEvents: () => {},
  onRefresh: () => {},
};

function codaShellAct(action: string, args?: Record<string, unknown>): ActionDescriptor {
  return { controllerId: CODA_CONTROLLER_ID, action, args: args as never };
}

function getCodaShellController(): CodaShellController | null {
  return codaShellControllerSingleton;
}

function codaValidationTreeItems(nodes: readonly ValidationTreeNode[], prefix: string): UiTreeItemNode[] {
  return nodes.map((node) => ({
    id: `${prefix}:${node.id}`,
    label: `${truthEmoji(node.truth)} ${node.label}`,
    action: codaShellAct("setSelection", {
      id: `${prefix}:${node.id}`,
      label: node.label,
      kind: node.kind,
      validationNode: node,
    }),
    items: node.children.length ? codaValidationTreeItems(node.children, `${prefix}:${node.id}`) : undefined,
  }));
}

function buildCodaDocumentPanelBody(_ctx: WindowBodyViewContext): UiTreeNode {
  const snap = getCodaShellController()?.getSnapshot() ?? CODA_EMPTY_SHELL_SNAPSHOT;
  const sections: UiSectionNode[] = [
    {
      type: "section",
      id: "coda.document.project",
      label: "Project",
      children: [
        { type: "text", value: snap.project?.design?.id ?? "—" },
        { type: "text", value: `${snap.project?.targets?.length ?? 0} target(s)` },
      ],
    },
    {
      type: "section",
      id: "coda.document.run",
      label: "Run",
      children: [{ type: "text", value: snap.run?.id ?? snap.run?.run_id ?? "—" }],
    },
    {
      type: "section",
      id: "coda.document.iteration",
      label: "Iteration",
      children: [{ type: "text", value: snap.iteration?.index != null ? String(snap.iteration.index) : "—" }],
    },
  ];
  const validationItems: UiTreeItemNode[] = (snap.report?.validations ?? []).flatMap((validation, index) => [
    {
      id: `coda.document.validation.${index}`,
      label: validation.instance,
      action: codaShellAct("setSelection", {
        id: `coda.document.validation.${index}`,
        label: validation.instance,
        kind: "validation",
      }),
      items: codaValidationTreeItems([validation.tree], `coda.validation.${index}`),
    },
  ]);
  sections.push({
    type: "section",
    id: "coda.document.validations",
    label: "Validations",
    children: validationItems.length ? validationItems.map((item) => ({ type: "button", id: item.id, label: item.label, action: item.action })) : [{ type: "text", value: "No validation report loaded" }],
  });
  const tree = uiDeclarativeSectionsToTree(sections);
  return { ...tree, selectedIds: snap.selection ? [snap.selection.id] : [] };
}

function buildCodaCataloguePanelBody(_ctx: WindowBodyViewContext): UiTreeNode {
  const snap = getCodaShellController()?.getSnapshot() ?? CODA_EMPTY_SHELL_SNAPSHOT;
  const frameworkItems: UiTreeItemNode[] = (snap.frameworks ?? []).map((framework) => ({
    id: `coda.catalogue.framework.${framework.id}`,
    label: formatId(framework.id),
    action: codaShellAct("setSelection", { id: `coda.catalogue.framework.${framework.id}`, label: formatId(framework.id), kind: "framework" }),
  }));
  const propertyItems: UiTreeItemNode[] = (snap.properties ?? []).map((property) => ({
    id: `coda.catalogue.property.${property.id}`,
    label: property.name ?? formatId(property.id),
    action: codaShellAct("setSelection", {
      id: `coda.catalogue.property.${property.id}`,
      label: property.name ?? formatId(property.id),
      kind: "property",
      propertyId: property.id,
    }),
  }));
  return uiDeclarativeSectionsToTree([
    {
      type: "section",
      id: "coda.catalogue.frameworks",
      label: `Frameworks (${frameworkItems.length})`,
      children: frameworkItems.length ? frameworkItems.map((item) => ({ type: "button", id: item.id, label: item.label, action: item.action })) : [{ type: "text", value: "(none)" }],
    },
    {
      type: "section",
      id: "coda.catalogue.properties",
      label: `Properties (${propertyItems.length})`,
      children: propertyItems.length ? propertyItems.map((item) => ({ type: "button", id: item.id, label: item.label, action: item.action })) : [{ type: "text", value: "(none)" }],
    },
  ]);
}

function buildCodaInspectionPanelBody(_ctx: WindowBodyViewContext): UiTreeNode {
  const snap = getCodaShellController()?.getSnapshot() ?? CODA_EMPTY_SHELL_SNAPSHOT;
  const selection = snap.selection;
  if (!selection) {
    return uiDeclarativeSectionsToTree([
      {
        type: "section",
        id: "coda.inspection.empty",
        label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
        children: [{ type: "text", value: "Select a validation node, framework, or property in the document or catalogue." }],
      },
    ]);
  }
  const children: UiSectionNode["children"] = [
    { type: "text", value: selection.label },
    { type: "text", value: `Kind · ${selection.kind}` },
  ];
  if (selection.validationNode) {
    children.push({ type: "text", value: `Truth · ${selection.validationNode.truth}` });
    if (selection.validationNode.property) children.push({ type: "text", value: `Property · ${selection.validationNode.property}` });
    if (selection.validationNode.value != null) {
      children.push({
        type: "field",
        id: "coda.inspection.validation.value",
        label: "Value",
        child: {
          type: "input",
          id: "coda.inspection.validation.value.input",
          inputKind: "text",
          value: String(selection.validationNode.value),
          onChange: codaShellAct("invokeTool", { tool: "fix", args: { nodeId: selection.id, field: "value" } }),
        },
      });
    }
  }
  if (selection.propertyId) {
    const property = (snap.properties ?? []).find((row) => row.id === selection.propertyId);
    if (property?.description) children.push({ type: "text", value: property.description });
    children.push({
      type: "field",
      id: "coda.inspection.property.note",
      label: "Inspection note",
      child: {
        type: "input",
        id: "coda.inspection.property.note.input",
        inputKind: "text",
        value: "",
        placeholder: "Describe a measure or fix attempt",
        onChange: codaShellAct("invokeTool", { tool: "fix", args: { propertyId: selection.propertyId } }),
      },
    });
  }
  return uiDeclarativeSectionsToTree([{ type: "section", id: "coda.inspection.selection", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children }]);
}

/** @emoji 🎛 Coda shell controller: single selection, page routing, MCP tool bridge. */
class CodaShellController extends Controller {
  private snapshot: CodaShellSnapshot = CODA_EMPTY_SHELL_SNAPSHOT;

  constructor(actionBus: ActionBus, hostNotify: () => void) {
    super(CODA_CONTROLLER_ID, actionBus, hostNotify);
  }

  getSnapshot(): CodaShellSnapshot {
    return this.snapshot;
  }

  override run(action: string, args?: unknown): void {
    switch (action) {
      case "setShellData": {
        this.snapshot = { ...this.snapshot, ...(args as Partial<CodaShellSnapshot>) };
        break;
      }
      case "setPage": {
        const page = (args as { page?: Page }).page;
        if (page) this.snapshot = { ...this.snapshot, currentPage: page };
        break;
      }
      case "setSelection": {
        const payload = args as CodaShellSelection;
        this.snapshot = { ...this.snapshot, selection: payload.id ? payload : null };
        break;
      }
      case "invokeTool": {
        const payload = args as { tool?: string; args?: Record<string, unknown>; value?: unknown };
        if (typeof window !== "undefined" && payload.tool) {
          const toolArgs = { ...(payload.args ?? {}), ...(payload.value !== undefined ? { value: payload.value } : {}) };
          void window.coda.tool(payload.tool, toolArgs).catch((error) => {
            console.error("[DEBUG] coda invokeTool failed:", error);
          });
        }
        break;
      }
      default:
        break;
    }
    this.emit();
  }
}

function registerCodaShellBodies(): void {
  if (codaBodiesRegistered) return;
  codaBodiesRegistered = true;
  registerUiPanelSurfaceHost(CODA_SURFACE_MAIN, CodaMainSurfaceHost);
  registerWindowBody(CODA_BODY_MAIN, () => buildPanelWindowBody(CODA_SURFACE_MAIN, CODA_CONTROLLER_ID));
  registerCornerPanelBody(CODA_PANEL_DOCUMENT_BODY, buildCodaDocumentPanelBody);
  registerCornerPanelBody(CODA_PANEL_CATALOGUE_BODY, buildCodaCataloguePanelBody);
  registerCornerPanelBody(CODA_PANEL_INSPECTION_BODY, buildCodaInspectionPanelBody);
}

function buildCodaAppRuntime(controller: CodaShellController): AppRuntime {
  const app = new AppRuntime(CODA_APP_ID, "Coda", undefined, controller, createTabStackLayout(["main"], ["Main"]) as never, [new WindowKindRuntime("main", "Main", CODA_BODY_MAIN)]);
  app.defaultModeId = "explore";
  app.addMode(new ModeRuntime("explore", "Explore"));
  app.panelTabs = [
    { id: FRAMEWORK_PANEL_TAB_DOCUMENT_ID, iconId: FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID, panel: "workbench", order: 0, bodyKey: CODA_PANEL_DOCUMENT_BODY, label: FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL },
    { id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID, iconId: FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, panel: "workbench", order: 1, bodyKey: CODA_PANEL_CATALOGUE_BODY, label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL },
    { id: FRAMEWORK_PANEL_TAB_INSPECTION_ID, iconId: FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, panel: "details", order: 0, bodyKey: CODA_PANEL_INSPECTION_BODY, label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL },
  ];
  return app;
}

function ensureCodaPlatform(): Platform {
  if (codaPlatformSingleton) return codaPlatformSingleton;
  registerCodaShellBodies();
  const platform = new Platform({ initialPanelVisibility: { topLeft: false, topRight: false, bottomLeft: false, bottomRight: false } });
  const controller = new CodaShellController(platform.actionBus, () => platform.notify());
  codaShellControllerSingleton = controller;
  platform.addApp(buildCodaAppRuntime(controller));
  platform.activeAppId = CODA_APP_ID;
  codaPlatformSingleton = platform;
  return platform;
}

function CodaMainSurfaceHost(_props: { readonly node: UiPanelHostSurfaceNode }): React.ReactElement {
  const snap = getCodaShellController()?.getSnapshot() ?? CODA_EMPTY_SHELL_SNAPSHOT;
  const page = snap.currentPage;
  return (
    <main className="h-full overflow-y-auto p-6">
      {page === "dashboard" && <DashboardPage refreshKey={codaMainHostBridge.refreshKey} />}
      {page === "config" && <ConfigPage refreshKey={codaMainHostBridge.refreshKey} />}
      {page === "runs" && <RunsPage refreshKey={codaMainHostBridge.refreshKey} />}
      {page === "report" && <ReportPage refreshKey={codaMainHostBridge.refreshKey} />}
      {page === "translations" && <TranslationsPage refreshKey={codaMainHostBridge.refreshKey} />}
      {page === "actions" && <ActionsPage refreshKey={codaMainHostBridge.refreshKey} onRefresh={codaMainHostBridge.onRefresh} />}
      {page === "events" && <EventsPage events={codaMainHostBridge.events} onClear={codaMainHostBridge.onClearEvents} />}
    </main>
  );
}

function CodaNavButton({ item, active, onSelect }: { readonly item: (typeof navItems)[number]; readonly active: boolean; readonly onSelect: () => void }): React.ReactElement {
  const label = useLabel(codaKey(`coda.nav.${item.id}`));
  return (
    <button type="button" onClick={onSelect} className={`rounded px-2 py-1 text-xs whitespace-nowrap transition-colors ${active ? "bg-active-base text-active-foreground" : "text-muted-foreground hover:bg-hover-interactive-fill hover:text-foreground"}`}>
      {label}
    </button>
  );
}

function CodaPageNavbar({ currentPage, onPageChange }: { readonly currentPage: Page; readonly onPageChange: (page: Page) => void }): React.ReactElement {
  return (
    <div className="flex min-w-0 items-center gap-1 overflow-x-auto">
      {navItems.map((item) => (
        <CodaNavButton key={item.id} item={item} active={currentPage === item.id} onSelect={() => onPageChange(item.id)} />
      ))}
    </div>
  );
}

// #endregion 🪨CodaProductShell

function CodaShellDataSync({ refreshKey, currentPage }: { readonly refreshKey: number; readonly currentPage: Page }): null {
  const { data: project } = useCodaResource<Project>("coda://project", refreshKey);
  const { data: run } = useCodaResource<Run>("coda://current-run", refreshKey);
  const { data: iteration } = useCodaResource<Iteration>("coda://current-iteration", refreshKey);
  const { data: report } = useCodaResource<Report>("coda://report", refreshKey);
  const { data: properties } = useCodaResource<Property[]>("coda://properties", refreshKey);
  const { data: frameworks } = useCodaResource<Framework[]>("coda://frameworks", refreshKey);

  reactHostPort.useEffect(() => {
    const platform = ensureCodaPlatform();
    platform.actionBus.dispatch(CODA_CONTROLLER_ID, "setShellData", {
      currentPage,
      refreshKey,
      project: project ?? null,
      run: run ?? null,
      iteration: iteration ?? null,
      report: report ?? null,
      properties: properties ?? null,
      frameworks: frameworks ?? null,
    });
  }, [currentPage, refreshKey, project, run, iteration, report, properties, frameworks]);

  return null;
}

// #region 🖲️App
// Root application component with title bar and ProductShell layout.

/**
 * Root React component that renders the coda desktop app.
 *MUST show WelcomePage until a project is selected.
 * MUST show loading state until user ID is resolved.
 * MUST provide sidebar navigation and page content area.
 **/
function App() {
  const [userId, setUserId] = reactHostPort.useState<string>("");
  const [projectPath, setProjectPath] = reactHostPort.useState<string | null | undefined>(undefined);
  const [currentPage, setCurrentPage] = reactHostPort.useState<Page>("dashboard");
  const [refreshKey, setRefreshKey] = reactHostPort.useState(0);
  const [sidecarConnected, setSidecarConnected] = reactHostPort.useState(false);
  const [events, setEvents] = reactHostPort.useState<CodaEvent[]>([]);
  const platform = reactHostPort.useMemo(() => ensureCodaPlatform(), []);
  const titlebarSubtitle = useLabel(codaKey("coda.titlebar.subtitle"));
  const sidecarConnectedLabel = useLabel(codaKey("coda.titlebar.sidecarConnected"));
  const sidecarDisconnectedLabel = useLabel(codaKey("coda.titlebar.sidecarDisconnected"));
  const connectedLabel = useLabel(codaKey("coda.titlebar.connected"));
  const offlineLabel = useLabel(codaKey("coda.titlebar.offline"));
  const refreshLabel = useLabel(codaKey("coda.titlebar.refresh"));
  const loadingLabel = useLabel(codaKey("coda.common.loading"));

  reactHostPort.useEffect(() => {
    async function init() {
      try {
        const [id, path, connected] = await Promise.all([window.os.getUserId(), window.project.getPath(), window.coda.getConnectionStatus()]);
        setUserId(id);
        setProjectPath(path);
        setSidecarConnected(connected);
      } catch {
        setUserId("anonymous-user");
        setProjectPath(null);
        setSidecarConnected(false);
      }
    }
    init();
  }, []);

  reactHostPort.useEffect(() => {
    const refreshOn = new Set(["project_files_changed", "project_ready", "run_started", "iteration_started", "translation_saved", "report_saved", "validation_saved", "validation_completed", "translate_started"]);
    const unsubEvent = window.coda.onEvent((evt: CodaEvent) => {
      setEvents((prev) => [evt, ...prev]);
      if (refreshOn.has(evt.event)) setRefreshKey((k) => k + 1);
    });
    const unsubConnection = window.coda.onConnectionStatus((connected: boolean) => {
      setSidecarConnected(connected);
    });
    return () => {
      unsubEvent();
      unsubConnection();
    };
  }, []);

  const handleClearEvents = reactHostPort.useCallback(() => setEvents([]), []);

  const handleRefresh = reactHostPort.useCallback(() => setRefreshKey((n) => n + 1), []);

  const handleMinimize = reactHostPort.useCallback(() => {
    if (window.windowControls) window.windowControls.minimize();
  }, []);

  const handleMaximize = reactHostPort.useCallback(() => {
    if (window.windowControls) window.windowControls.maximize();
  }, []);

  const handleClose = reactHostPort.useCallback(() => {
    if (window.windowControls) window.windowControls.close();
  }, []);

  reactHostPort.useEffect(() => {
    codaMainHostBridge.refreshKey = refreshKey;
    codaMainHostBridge.events = events;
    codaMainHostBridge.onClearEvents = handleClearEvents;
    codaMainHostBridge.onRefresh = handleRefresh;
    platform.notify();
  }, [refreshKey, events, handleClearEvents, handleRefresh, platform]);

  reactHostPort.useEffect(() => {
    platform.actionBus.dispatch(CODA_CONTROLLER_ID, "setPage", { page: currentPage });
  }, [currentPage, platform]);

  useActionHotkey("ctrl+r,meta+r", handleRefresh, { preventDefault: true }, [handleRefresh]);

  if (projectPath === undefined) {
    return (
      <LevelProvider level="window">
        <div data-level="window" className="flex h-screen w-screen items-center justify-center ui-surface">
          <Spinner label={loadingLabel} />
        </div>
      </LevelProvider>
    );
  }

  if (!projectPath) {
    return <WelcomePage onProjectReady={(p) => setProjectPath(p)} onMinimize={handleMinimize} onMaximize={handleMaximize} onClose={handleClose} />;
  }

  const projectName = projectPath.split("/").pop() ?? projectPath;

  return (
    <LevelProvider level="window">
    <div data-level="window" className="flex h-screen w-screen flex-col ui-surface overflow-hidden">
      {/* #region Title Bar -- window-level chrome ribbon (GlassTier "ribbon" -> ui-glass-chrome @ window) */}
      <div className="flex h-9 items-center border-b border-normal ui-glass-chrome px-3 shrink-0" style={{ WebkitAppRegion: "drag" } as React.CSSProperties}>
        <div className="flex items-center gap-2 flex-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <span className="text-sm font-bold text-active-base">coda</span>
          <span className="text-xs text-muted-foreground">{titlebarSubtitle}</span>
          <span className="text-xs text-muted-foreground ml-1">|</span>
          <span className="text-xs text-muted-foreground ml-1 font-mono" title={projectPath}>
            {projectName}
          </span>
          {userId && (
            <>
              <span className="text-xs text-muted-foreground ml-1">·</span>
              <span className="text-xs text-muted-foreground ml-1">{userId}</span>
            </>
          )}
          <span className="text-xs text-muted-foreground ml-1">·</span>
          <span className={`ml-1 inline-flex items-center gap-1 text-xs ${sidecarConnected ? "text-success-foreground" : "text-destructive-foreground"}`} title={sidecarConnected ? sidecarConnectedLabel : sidecarDisconnectedLabel}>
            <span className={`w-1.5 h-1.5 rounded-full ${sidecarConnected ? "bg-success-border" : "bg-destructive-border"}`} />
            {sidecarConnected ? connectedLabel : offlineLabel}
          </span>
        </div>
        <div className="flex items-center gap-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <button onClick={handleRefresh} className="rounded p-1.5 text-muted-foreground hover:bg-hover-interactive-fill hover:text-foreground transition-colors cursor-pointer" title={refreshLabel}>
            <IconRefresh className="w-3.5 h-3.5" />
          </button>
          <button onClick={handleMinimize} className="rounded p-1.5 text-muted-foreground hover:bg-hover-interactive-fill hover:text-foreground transition-colors cursor-pointer">
            <IconMinimize />
          </button>
          <button onClick={handleMaximize} className="rounded p-1.5 text-muted-foreground hover:bg-hover-interactive-fill hover:text-foreground transition-colors cursor-pointer">
            <IconMaximize />
          </button>
          <button onClick={handleClose} className="rounded p-1.5 text-muted-foreground hover:bg-destructive-bg hover:text-destructive-foreground transition-colors cursor-pointer">
            <IconClose />
          </button>
        </div>
      </div>
      {/* #endregion Title Bar */}

      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <CodaShellDataSync refreshKey={refreshKey} currentPage={currentPage} />
        <PlatformView
          platform={platform}
          defaultAppId={CODA_APP_ID}
          initialPanelVisibility={PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY}
          className="min-h-0 flex-1"
          slotNavbarCenter={
            <CodaPageNavbar
              currentPage={currentPage}
              onPageChange={(page) => {
                setCurrentPage(page);
              }}
            />
          }
        />
      </div>
    </div>
    </LevelProvider>
  );
}

export default App;

export { getOntologyNodeDescriptor, getValidationNodeDescriptor, OntologyTree, ValidationTree };
export type { OntologyNodeKind, OntologyTreeNode, TruthValue, ValidationNodeKind, ValidationReport, ValidationTreeNode };

console.log("[DEBUG] renderer.tsx module body reached createRoot block");
if (typeof document !== "undefined") {
  const rootElement = document.getElementById("root");
  console.log("[DEBUG] rootElement:", rootElement ? "found" : "null");
  if (rootElement) {
    console.log("[DEBUG] calling createRoot().render()");
    createRoot(rootElement).render(
      <React.StrictMode>
        <App />
      </React.StrictMode>,
    );
  }
}

// #endregion 🖲️App

// #endregion ⛩️Renderer
