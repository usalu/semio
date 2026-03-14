// #region 🔖Header
// [🔬coda🖱️desktop💻renderer](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the Electron renderer process mounting the coda React app.

// #endregion 🔖Header

// #region 🔖Renderer
// [🔬coda🖱️desktop💻renderer🔖renderer](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer)
// Electron renderer process that mounts the coda dashboard React app with window controls.
// MUST resolve the user identity before rendering the dashboard.
// MUST communicate with coda MCP server via the preload bridge.

import React, { useCallback, useEffect, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";

import "./globals.css";

// #region 🔖Types
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types)
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
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️codaevent](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/CodaEvent)
 *
 * MUST have event kind, data, and timestamp.
 **/
interface CodaEvent {
  event: string;
  data: Record<string, unknown>;
  timestamp: number;
}

/**
 * MCP JSON-RPC response wrapper.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️mcpresponse](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/McpResponse)
 *
 * MUST contain either result or error.
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
 * A design measure available for fixing breaches.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️measure](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Measure)
 *
 * MUST have id and description.
 **/
interface Measure {
  id: string;
  description: string;
}

/**
 * A platform with optional measure instructions.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️platform](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Platform)
 *
 * MUST have an id.
 **/
interface Platform {
  id: string;
  measures?: Array<{
    id: string;
    instructions?: string;
    mcp?: {
      resources?: Array<{ id: string; instruction: string }>;
      tools?: Array<{ id: string; instruction: string; parameters?: Array<{ id: string; instruction: string }> }>;
    };
  }>;
}

/**
 * A clause within a compliance rule.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️clause](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Clause)
 *
 * MUST have id and description.
 **/
interface Clause {
  id: string;
  description: string;
  status?: "compliant" | "violated" | "unknown";
  properties?: Array<{ id: string; value: string }>;
}

/**
 * A compliance rule belonging to a target.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️rule](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Rule)
 *
 * MUST have id and description.
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
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️levelmeasureref](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/LevelMeasureRef)
 *
 * MUST have id and optional instruction.
 **/
interface LevelMeasureRef {
  id: string;
  instruction?: string;
}

/**
 * A level within a property, optionally with measures and instructions for raising/lowering.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️level](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Level)
 *
 * MUST have a value. May have measures (lower/higher) and instructions (higher).
 **/
interface Level {
  value: string;
  name?: string;
  description?: string;
  measures?: { lower?: LevelMeasureRef[]; higher?: LevelMeasureRef[] };
  instructions?: { higher?: LevelMeasureRef[] };
}

/**
 * A property definition on a framework.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️property](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Property)
 *
 * MUST have id and name.
 **/
interface Property {
  id: string;
  name?: string;
  type?: string;
  description?: string;
  url?: string;
  levels?: Level[];
}

/**
 * A compliance framework with properties and rules. General (not project-scoped).
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️framework](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Framework)
 *
 * MUST have an id.
 **/
interface Framework {
  id: string;
  properties?: Property[];
  rules?: Rule[];
}

/**
 * Project configuration from .coda/project.json.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️project](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Project)
 *
 * MUST have design and targets.
 **/
interface Project {
  design?: { id: string; mcp?: Record<string, unknown> };
  targets?: Array<{ id: string; llm?: unknown[] }>;
  error?: string;
}

/**
 * Run metadata.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️run](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Run)
 *
 * MUST have an id.
 **/
interface Run {
  id?: string;
  started?: string;
  run_id?: string;
  error?: string;
}

/**
 * Iteration metadata.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️iteration](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Iteration)
 *
 * MUST have an index.
 **/
interface Iteration {
  index?: number | string;
  targets?: string[];
  error?: string;
}

/**
 * Compliance report from an iteration.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️report](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Report)
 *
 * MUST contain rules array.
 **/
interface Report {
  rules?: Rule[];
  breachs?: Rule[];
  error?: string;
}

/**
 * Navigation page identifiers.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️page](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Page)
 *
 * MUST enumerate all navigable pages.
 **/
type Page = "dashboard" | "config" | "runs" | "report" | "translations" | "actions" | "events";

// #endregion 🔖Types

// #region 🔖Helpers
// [🔬coda🖱️desktop💻renderer🔖renderer🔖helpers](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Helpers)
// Helper functions for parsing MCP responses and formatting data.
// Helpers MUST safely extract data from MCP JSON-RPC responses.

/**
 * Extract parsed JSON from an MCP resource response.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖helpers🛠️parsemcpresource](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Helpers/d/i/parseMcpResource)
 *
 * MUST return null when the response has no valid content.
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
// [🔬coda🖱️desktop💻renderer🔖renderer🔖helpers🛠️parsemcptool](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Helpers/d/i/parseMcpTool)
 *
 * MUST return null when the response has no valid content.
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
// [🔬coda🖱️desktop💻renderer🔖renderer🔖helpers🛠️formatid](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Helpers/d/i/formatId)
 *
 * MUST return a human-readable string.
 **/
function formatId(id: string | undefined): string {
  if (!id) return "—";
  return id.replace(/_/g, " ").replace(/-/g, " ");
}

// #endregion 🔖Helpers

// #region 🔖Icons
// [🔬coda🖱️desktop💻renderer🔖renderer🔖icons](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Icons)
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

// #endregion 🔖Icons

// #region 🔖Components
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components)
// Reusable UI components for the coda desktop application.
// Components MUST use Tailwind CSS classes for styling.

// #region 🔖StatusBadge
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖statusbadge](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/StatusBadge)

/**
 * Displays a colored badge for compliance status.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖statusbadge🛠️statusbadge](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/StatusBadge/d/i/StatusBadge)
 *
 * MUST render green for compliant, red for violated, gray for unknown.
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

// #endregion 🔖StatusBadge

// #region 🔖Card
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖card](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/Card)

/**
 * A card container for dashboard sections.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖card🛠️card](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/Card/d/i/Card)
 *
 * MUST render a bordered container with optional title.
 **/
function Card({ title, children, className = "", action }: { title?: string; children: React.ReactNode; className?: string; action?: React.ReactNode }) {
  return (
    <div className={`rounded-lg border border-border-window bg-window ${className}`}>
      {(title || action) && (
        <div className="flex items-center justify-between border-b border-border-window px-4 py-3">
          {title && <h3 className="text-sm font-semibold text-foreground">{title}</h3>}
          {action}
        </div>
      )}
      <div className="p-4">{children}</div>
    </div>
  );
}

// #endregion 🔖Card

// #region 🔖StatCard
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖statcard](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/StatCard)

/**
 * A metric card for the dashboard overview.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖statcard🛠️statcard](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/StatCard/d/i/StatCard)
 *
 * MUST display a label and a large value.
 **/
function StatCard({ label, value, sublabel }: { label: string; value: string | number; sublabel?: string }) {
  return (
    <div className="rounded-lg border border-border-window bg-window p-4">
      <div className="text-xs font-medium text-muted-foreground uppercase tracking-wider">{label}</div>
      <div className="mt-1 text-2xl font-bold text-foreground">{value}</div>
      {sublabel && <div className="mt-0.5 text-xs text-muted-foreground">{sublabel}</div>}
    </div>
  );
}

// #endregion 🔖StatCard

// #region 🔖Button
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖button](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/Button)

/**
 * A styled button with variant support.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖button🛠️button](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/Button/d/i/Button)
 *
 * MUST support primary, secondary, and danger variants.
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
    secondary: "bg-window border border-border-window text-foreground hover:bg-hover-window disabled:opacity-50",
    danger: "bg-destructive-bg text-destructive-foreground border border-destructive-border hover:bg-hover-window disabled:opacity-50",
  };
  return (
    <button
      onClick={onClick}
      disabled={disabled || loading}
      className={`inline-flex items-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium transition-colors cursor-pointer disabled:cursor-not-allowed ${variants[variant]} ${className}`}
    >
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

// #endregion 🔖Button

// #region 🔖Spinner
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖spinner](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/Spinner)

/**
 * A centered loading spinner.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖spinner🛠️spinner](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/Spinner/d/i/Spinner)
 *
 * MUST display an animated spinning indicator.
 **/
function Spinner({ label = "Loading..." }: { label?: string }) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-12 text-muted-foreground">
      <svg className="w-6 h-6 animate-spin" viewBox="0 0 24 24" fill="none">
        <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
        <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
      </svg>
      <span className="text-sm">{label}</span>
    </div>
  );
}

// #endregion 🔖Spinner

// #region 🔖EmptyState
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖emptystate](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/EmptyState)

/**
 * An empty state placeholder.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖emptystate🛠️emptystate](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/EmptyState/d/i/EmptyState)
 *
 * MUST display a message and optional action.
 **/
function EmptyState({ message, action }: { message: string; action?: React.ReactNode }) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-12 text-muted-foreground">
      <p className="text-sm">{message}</p>
      {action}
    </div>
  );
}

// #endregion 🔖EmptyState

// #region 🔖Collapsible
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖collapsible](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/Collapsible)

/**
 * A collapsible section with toggle.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖collapsible🛠️collapsible](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/Collapsible/d/i/Collapsible)
 *
 * MUST toggle visibility on header click.
 **/
function Collapsible({ title, children, defaultOpen = false, badge }: { title: string; children: React.ReactNode; defaultOpen?: boolean; badge?: React.ReactNode }) {
  const [open, setOpen] = useState(defaultOpen);
  return (
    <div className="border border-border-window rounded-md overflow-hidden">
      <button onClick={() => setOpen(!open)} className="flex w-full items-center gap-2 px-3 py-2 text-sm font-medium text-foreground hover:bg-hover-window transition-colors cursor-pointer">
        {open ? <IconChevronDown className="w-3.5 h-3.5 text-muted-foreground" /> : <IconChevronRight className="w-3.5 h-3.5 text-muted-foreground" />}
        <span className="flex-1 text-left">{title}</span>
        {badge}
      </button>
      {open && <div className="border-t border-border-window px-3 py-2">{children}</div>}
    </div>
  );
}

// #endregion 🔖Collapsible

// #region 🔖JsonViewer
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖jsonviewer](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/JsonViewer)

/**
 * A formatted JSON viewer for translation data.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖jsonviewer🛠️jsonviewer](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/JsonViewer/d/i/JsonViewer)
 *
 * MUST display formatted JSON in a code block.
 **/
function JsonViewer({ data }: { data: unknown }) {
  const formatted = useMemo(() => {
    try {
      return JSON.stringify(data, null, 2);
    } catch {
      return String(data);
    }
  }, [data]);
  return (
    <pre className="overflow-auto max-h-96 rounded-md bg-panel border border-border-window p-3 text-xs font-mono text-muted-foreground whitespace-pre-wrap break-all">
      {formatted}
    </pre>
  );
}

// #endregion 🔖JsonViewer

// #region 🔖OntologyTree
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖ontologytree](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/OntologyTree)
// Tree viewer for visualizing OWL class expression structure (schema-level, no instances).
// OntologyTree MUST render a collapsible tree of the class expression without truth values.

/**
 * Node kind in an ontology class expression tree.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖ontologytree✂️ontologynodekind](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/OntologyTree/d/f/OntologyNodeKind)
 *
 * Enumerates all OWL class expression constructs.
 **/
type OntologyNodeKind =
  | "Class"
  | "And"
  | "Or"
  | "Not"
  | "SomeValuesFrom"
  | "AllValuesFrom"
  | "ExactCardinality"
  | "MinCardinality"
  | "MaxCardinality"
  | "DataSomeValuesFrom"
  | "DataAllValuesFrom"
  | "DataHasValue"
  | "DatatypeRestriction";

/**
 * A node in the ontology class expression tree (schema only, no instances).
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖ontologytree✂️ontologytreenode](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/OntologyTree/d/f/OntologyTreeNode)
 *
 * MUST have id, kind, label, and children.
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
    case "Class": return "C";
    case "And": return "∧";
    case "Or": return "∨";
    case "Not": return "¬";
    case "SomeValuesFrom": return "∃";
    case "AllValuesFrom": return "∀";
    case "ExactCardinality": return "=n";
    case "MinCardinality": return "≥n";
    case "MaxCardinality": return "≤n";
    case "DataSomeValuesFrom": return "∃d";
    case "DataAllValuesFrom": return "∀d";
    case "DataHasValue": return "v";
    case "DatatypeRestriction": return "D";
    default: return "?";
  }
}

/**
 * Renders a single ontology tree node with expand/collapse.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖ontologytree🛠️ontologytreenodeview](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/OntologyTree/d/i/OntologyTreeNodeView)
 *
 * MUST display the node kind icon, label, and expandable children.
 **/
function OntologyTreeNodeView({
  node,
  depth = 0,
  defaultExpanded = true,
}: {
  node: OntologyTreeNode;
  depth?: number;
  defaultExpanded?: boolean;
}) {
  const hasChildren = node.children.length > 0;
  const [expanded, setExpanded] = useState(defaultExpanded);

  return (
    <div className="select-none">
      <div
        className={`flex items-center gap-1.5 py-0.5 px-1 rounded hover:bg-hover-window transition-colors ${hasChildren ? "cursor-pointer" : ""}`}
        style={{ paddingLeft: `${depth * 16 + 4}px` }}
        onClick={() => hasChildren && setExpanded(!expanded)}
      >
        {hasChildren ? (
          expanded ? <IconChevronDown className="w-3 h-3 text-muted-foreground shrink-0" /> : <IconChevronRight className="w-3 h-3 text-muted-foreground shrink-0" />
        ) : (
          <span className="w-3 shrink-0" />
        )}
        <span className="inline-flex items-center justify-center w-5 h-5 rounded bg-info-bg text-info-foreground text-[10px] font-bold shrink-0" title={node.kind}>
          {ontologyNodeIcon(node.kind)}
        </span>
        <span className="text-sm font-medium text-foreground">{node.label}</span>
        {node.fragment && node.fragment !== node.label && (
          <span className="text-xs text-muted-foreground ml-1 truncate">{node.fragment}</span>
        )}
      </div>
      {expanded && hasChildren && (
        <div>
          {node.children.map((child) => (
            <OntologyTreeNodeView key={child.id} node={child} depth={depth + 1} defaultExpanded={defaultExpanded} />
          ))}
        </div>
      )}
    </div>
  );
}

/**
 * Tree viewer that displays an OWL class expression as a collapsible tree.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖ontologytree🛠️ontologytree](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/OntologyTree/d/i/OntologyTree)
 *
 * MUST render the full ontology tree structure from root.
 **/
function OntologyTree({
  root,
  title,
  defaultExpanded = true,
}: {
  root: OntologyTreeNode;
  title?: string;
  defaultExpanded?: boolean;
}) {
  return (
    <div className="rounded-lg border border-border-window bg-window overflow-hidden">
      {title && (
        <div className="border-b border-border-window px-3 py-2">
          <h3 className="text-sm font-semibold text-foreground">{title}</h3>
        </div>
      )}
      <div className="p-2 overflow-x-auto">
        <OntologyTreeNodeView node={root} defaultExpanded={defaultExpanded} />
      </div>
    </div>
  );
}

// #endregion 🔖OntologyTree

// #region 🔖ValidationTree
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖validationtree](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/ValidationTree)
// Tree viewer for visualizing validation results (data graph instances of the ontology).
// ValidationTree MUST render truth values, witnesses, data values, and cardinality info.

/**
 * Three-valued truth for validation nodes.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖validationtree✂️truthvalue](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/ValidationTree/d/f/TruthValue)
 *
 * true = green, false = red, unknown = gray.
 **/
type TruthValue = "true" | "false" | "unknown";

/**
 * Node kind in a validation tree, extending ontology kinds with instance-level nodes.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖validationtree✂️validationnodekind](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/ValidationTree/d/f/ValidationNodeKind)
 *
 * Extends OntologyNodeKind with Witness and DataValue for instance data.
 **/
type ValidationNodeKind = OntologyNodeKind | "ClassAssertion" | "Witness" | "DataValue";

/**
 * A node in the validation result tree (instance-level with truth values).
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖validationtree✂️validationtreenode](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/ValidationTree/d/f/ValidationTreeNode)
 *
 * MUST have id, kind, label, truth, and children.
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
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖validationtree✂️validationreport](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/ValidationTree/d/f/ValidationReport)
 *
 * MUST have instance, expression, truth, and tree.
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
    case "true": return "🟢";
    case "false": return "🔴";
    case "unknown": return "⚪";
  }
}

/**
 * Renders a single validation tree node with expand/collapse, truth badges, and witnesses.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖validationtree🛠️validationtreenodeview](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/ValidationTree/d/i/ValidationTreeNodeView)
 *
 * MUST display truth indicator, node label, witness/value info, and expandable children.
 **/
function ValidationTreeNodeView({
  node,
  depth = 0,
  defaultExpanded = true,
}: {
  node: ValidationTreeNode;
  depth?: number;
  defaultExpanded?: boolean;
}) {
  const hasChildren = node.children.length > 0;
  const [expanded, setExpanded] = useState(defaultExpanded);
  const colors = truthColors[node.truth];
  const isWitness = node.kind === "Witness";
  const isDataValue = node.kind === "DataValue";

  return (
    <div className="select-none">
      <div
        className={`flex items-center gap-1.5 py-0.5 px-1 rounded hover:bg-hover-window transition-colors ${hasChildren ? "cursor-pointer" : ""}`}
        style={{ paddingLeft: `${depth * 16 + 4}px` }}
        onClick={() => hasChildren && setExpanded(!expanded)}
        title={node.summary}
      >
        {hasChildren ? (
          expanded ? <IconChevronDown className="w-3 h-3 text-muted-foreground shrink-0" /> : <IconChevronRight className="w-3 h-3 text-muted-foreground shrink-0" />
        ) : (
          <span className="w-3 shrink-0" />
        )}
        {/* Truth indicator */}
        <span className="text-xs shrink-0" title={`${node.truth}`}>{truthEmoji(node.truth)}</span>
        {/* Node content */}
        {isWitness ? (
          <>
            <span className={`text-sm font-medium ${node.counted === false ? "text-muted-foreground" : "text-foreground"}`}>
              {node.individual ?? node.label}
            </span>
            {node.counted === true && (
              <span className="text-[10px] font-medium text-success-foreground bg-success-bg px-1 py-0.5 rounded">counted</span>
            )}
            {node.counted === false && (
              <span className="text-[10px] font-medium text-info-foreground bg-info-bg px-1 py-0.5 rounded">not matching</span>
            )}
          </>
        ) : isDataValue ? (
          <>
            <span className="text-sm font-mono text-foreground">{String(node.value ?? node.label)}</span>
            {node.datatype && (
              <span className="text-xs text-muted-foreground">{node.datatype}</span>
            )}
          </>
        ) : (
          <>
            <span className={`inline-flex items-center justify-center w-5 h-5 rounded ${colors.bg} ${colors.text} text-[10px] font-bold shrink-0`} title={node.kind}>
              {node.kind === "ClassAssertion" ? "∈" : ontologyNodeIcon(node.kind as OntologyNodeKind)}
            </span>
            <span className="text-sm font-medium text-foreground">{node.label}</span>
            {node.kind === "ExactCardinality" || node.kind === "MinCardinality" || node.kind === "MaxCardinality" ? (
              node.matchingCount !== undefined && node.expectedCardinality !== undefined && (
                <span className="text-xs text-muted-foreground ml-1">
                  [{node.matchingCount}/{node.expectedCardinality}]
                </span>
              )
            ) : null}
            {node.fragment && node.fragment !== node.label && (
              <span className="text-xs text-muted-foreground ml-1 truncate">{node.fragment}</span>
            )}
          </>
        )}
        {/* Summary tooltip as text */}
        {node.summary && !isWitness && !isDataValue && (
          <span className={`text-xs ${colors.text} ml-auto shrink-0`}>{node.truth}</span>
        )}
      </div>
      {expanded && hasChildren && (
        <div>
          {node.children.map((child) => (
            <ValidationTreeNodeView key={child.id} node={child} depth={depth + 1} defaultExpanded={defaultExpanded} />
          ))}
        </div>
      )}
    </div>
  );
}

/**
 * Tree viewer that displays a validation report as a collapsible tree with truth values.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖components🔖validationtree🛠️validationtree](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Components/s/ValidationTree/d/i/ValidationTree)
 *
 * MUST render instance header, expression, overall truth, and the expanded result tree.
 **/
function ValidationTree({
  report,
  defaultExpanded = true,
}: {
  report: ValidationReport;
  defaultExpanded?: boolean;
}) {
  return (
    <div className="rounded-lg border border-border-window bg-window overflow-hidden">
      <div className="border-b border-border-window px-3 py-2 space-y-1">
        <div className="flex items-center gap-2">
          <span className="text-sm font-semibold text-foreground">Instance: {report.instance}</span>
          <span className={`inline-flex items-center gap-1 rounded-full border px-2 py-0.5 text-xs font-medium ${report.truth === "true"
            ? "bg-success-bg text-success-foreground border-success-border"
            : report.truth === "false"
              ? "bg-destructive-bg text-destructive-foreground border-destructive-border"
              : "bg-info-bg text-info-foreground border-info-border"
            }`}>
            {truthEmoji(report.truth)} {report.truth}
          </span>
        </div>
        <div className="text-xs text-muted-foreground font-mono break-all">{report.expression}</div>
      </div>
      <div className="p-2 overflow-x-auto">
        <ValidationTreeNodeView node={report.tree} defaultExpanded={defaultExpanded} />
      </div>
    </div>
  );
}

// #endregion 🔖ValidationTree

// #endregion 🔖Components

// #region 🔖Hooks
// [🔬coda🖱️desktop💻renderer🔖renderer🔖hooks](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Hooks)
// Custom React hooks for fetching coda MCP data.
// Hooks MUST handle loading, error, and data states.

/**
 * Fetches a coda MCP resource and returns parsed data.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖hooks🛠️usecodaResource](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Hooks/d/i/useCodaResource)
 *
 * MUST refetch when uri or refreshKey changes.
 **/
function useCodaResource<T>(uri: string, refreshKey: number = 0): { data: T | null; loading: boolean; error: string | null; refresh: () => void } {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [localRefresh, setLocalRefresh] = useState(0);

  useEffect(() => {
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

  const refresh = useCallback(() => setLocalRefresh((n) => n + 1), []);
  return { data, loading, error, refresh };
}

// #endregion 🔖Hooks

// #region 🔖Pages
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages)
// Page components for each view in the coda desktop application.
// Pages MUST use useCodaResource hooks to fetch and display data.

// #region 🔖DashboardPage
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖dashboardpage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/DashboardPage)

/**
 * Dashboard overview showing project status, current run, iteration, and compliance summary.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖dashboardpage🛠️dashboardpage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/DashboardPage/d/i/DashboardPage)
 *
 * MUST display stat cards for project, run, iteration, and breach counts.
 **/
function DashboardPage({ refreshKey }: { refreshKey: number }) {
  const { data: project, loading: projectLoading } = useCodaResource<Project>("coda://project", refreshKey);
  const { data: run, loading: runLoading } = useCodaResource<Run>("coda://current-run", refreshKey);
  const { data: iteration, loading: iterLoading } = useCodaResource<Iteration>("coda://current-iteration", refreshKey);
  const { data: report, loading: reportLoading } = useCodaResource<Report>("coda://report", refreshKey);
  const { data: measures } = useCodaResource<Measure[]>("coda://measures", refreshKey);
  const { data: frameworks } = useCodaResource<Framework[]>("coda://frameworks", refreshKey);

  const loading = projectLoading || runLoading || iterLoading || reportLoading;

  const breachCount = useMemo(() => {
    if (!report?.rules) return 0;
    return report.rules.filter((r) => r.status === "violated").length;
  }, [report]);

  const compliantCount = useMemo(() => {
    if (!report?.rules) return 0;
    return report.rules.filter((r) => r.status === "compliant").length;
  }, [report]);

  const totalRules = report?.rules?.length ?? 0;

  if (loading) return <Spinner label="Loading dashboard..." />;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">Dashboard</h2>
        <p className="text-sm text-muted-foreground mt-1">Overview of the coda compliance checking status.</p>
      </div>

      <div className="grid grid-cols-2 gap-4 lg:grid-cols-4">
        <StatCard label="Design" value={project?.design?.id ?? "—"} sublabel={project ? `${project.targets?.length ?? 0} targets` : undefined} />
        <StatCard label="Current Run" value={run?.id ?? run?.run_id ?? "—"} sublabel={run?.started ? `Started ${run.started}` : undefined} />
        <StatCard label="Iteration" value={iteration?.index ?? "—"} sublabel={iteration?.targets ? `${iteration.targets.length} targets` : undefined} />
        <StatCard label="Compliance" value={totalRules > 0 ? `${compliantCount}/${totalRules}` : "—"} sublabel={breachCount > 0 ? `${breachCount} violated` : totalRules > 0 ? "All compliant" : undefined} />
      </div>

      {report?.rules && report.rules.length > 0 && (
        <Card title="Compliance Report">
          <div className="space-y-2">
            {report.rules.map((rule) => (
              <div key={rule.id} className="flex items-center justify-between rounded-md border border-border-window px-3 py-2">
                <div>
                  <span className="text-sm font-medium text-foreground">{formatId(rule.id)}</span>
                  {rule.description && <p className="text-xs text-muted-foreground mt-0.5">{rule.description}</p>}
                </div>
                <StatusBadge status={rule.status} />
              </div>
            ))}
          </div>
        </Card>
      )}

      <div className="space-y-4">
        <div className="flex items-center gap-2">
          <h3 className="text-sm font-semibold text-foreground">General Configuration</h3>
          <span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">not project-scoped</span>
        </div>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          {measures && measures.length > 0 && (
            <Card title={`Measures (${measures.length})`}>
              <div className="space-y-1">
                {measures.map((m) => (
                  <div key={m.id} className="flex items-start gap-2 rounded px-2 py-1.5 text-sm hover:bg-hover-window">
                    <IconWrench className="w-3.5 h-3.5 text-muted-foreground mt-0.5 shrink-0" />
                    <div>
                      <span className="font-medium text-foreground">{formatId(m.id)}</span>
                      {m.description && <p className="text-xs text-muted-foreground">{m.description}</p>}
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
                  <div key={fw.id} className="flex items-start gap-2 rounded px-2 py-1.5 text-sm hover:bg-hover-window">
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

// #endregion 🔖DashboardPage

// #region 🔖ConfigPage
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖configpage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ConfigPage)

/**
 * Configuration page showing measures, frameworks (with properties and rules), and platforms.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖configpage🛠️configpage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ConfigPage/d/i/ConfigPage)
 *
 * MUST display all coda configuration in expandable sections.
 * Measures and frameworks are general (not project-scoped).
 **/
function ConfigPage({ refreshKey }: { refreshKey: number }) {
  const { data: measures, loading: measuresLoading } = useCodaResource<Measure[]>("coda://measures", refreshKey);
  const { data: frameworks, loading: frameworksLoading } = useCodaResource<Framework[]>("coda://frameworks", refreshKey);
  const { data: platforms, loading: platformsLoading } = useCodaResource<Platform[]>("coda://platforms", refreshKey);

  const loading = measuresLoading || frameworksLoading || platformsLoading;
  if (loading) return <Spinner label="Loading configuration..." />;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">Configuration</h2>
        <p className="text-sm text-muted-foreground mt-1">Measures, frameworks, and platforms from the coda configuration. These are general and not project-scoped.</p>
      </div>

      <Card title={`Measures (${measures?.length ?? 0})`} action={<span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">general</span>}>
        {measures && measures.length > 0 ? (
          <div className="divide-y divide-border-window">
            {measures.map((m) => (
              <div key={m.id} className="py-2 first:pt-0 last:pb-0">
                <div className="text-sm font-medium text-foreground font-mono">{m.id}</div>
                {m.description && <p className="text-xs text-muted-foreground mt-0.5">{m.description}</p>}
              </div>
            ))}
          </div>
        ) : (
          <EmptyState message="No measures configured." />
        )}
      </Card>

      <Card title={`Frameworks (${frameworks?.length ?? 0})`} action={<span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">general</span>}>
        {frameworks && frameworks.length > 0 ? (
          <div className="space-y-3">
            {frameworks.map((fw) => (
              <Collapsible key={fw.id} title={formatId(fw.id)} badge={<span className="text-xs text-muted-foreground">{fw.properties?.length ?? 0} properties, {fw.rules?.length ?? 0} rules</span>}>
                <div className="space-y-3">
                  {fw.properties && fw.properties.length > 0 && (
                    <div>
                      <h4 className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">Properties</h4>
                      <div className="space-y-1.5">
                        {fw.properties.map((prop) => (
                          <div key={prop.id} className="rounded bg-panel border border-border-window p-2">
                            <div className="flex items-center gap-2">
                              <span className="text-sm font-medium text-foreground">{prop.name ?? formatId(prop.id)}</span>
                              {prop.type && <span className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded">{prop.type}</span>}
                            </div>
                            {prop.description && <p className="text-xs text-muted-foreground mt-1">{prop.description}</p>}
                            {prop.url && (
                              <a className="text-xs text-active-base hover:underline mt-1 inline-block" href={prop.url} target="_blank" rel="noreferrer">
                                Reference
                              </a>
                            )}
                            {prop.levels && prop.levels.length > 0 && (
                              <div className="mt-2 space-y-2">
                                <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Levels ({prop.levels.length})</span>
                                {prop.levels.map((level) => (
                                  <div key={level.value} className="rounded border border-border-window p-2 space-y-1.5">
                                    <div className="flex items-start gap-2 text-xs">
                                      <span className="bg-active-base text-active-foreground px-1.5 py-0.5 rounded font-mono shrink-0">{level.value}</span>
                                      <div>
                                        {level.name && <span className="font-medium text-foreground">{level.name}</span>}
                                        {level.description && <p className="text-muted-foreground">{level.description}</p>}
                                      </div>
                                    </div>
                                    {level.measures && (
                                      <div className="space-y-1 pl-2 border-l-2 border-border-window">
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
                                        {level.measures.higher && level.measures.higher.length > 0 && (
                                          <div>
                                            <span className="text-xs font-semibold text-muted-foreground">↑ Higher measures:</span>
                                            <div className="mt-0.5 space-y-0.5">
                                              {level.measures.higher.map((hm) => (
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
                                    {level.instructions && level.instructions.higher && level.instructions.higher.length > 0 && (
                                      <div className="space-y-1 pl-2 border-l-2 border-border-window">
                                        <span className="text-xs font-semibold text-muted-foreground">↑ Higher instructions:</span>
                                        <div className="mt-0.5 space-y-0.5">
                                          {level.instructions.higher.map((hi) => (
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
                          </div>
                        ))}
                      </div>
                    </div>
                  )}

                  {fw.rules && fw.rules.length > 0 && (
                    <div>
                      <h4 className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">Rules</h4>
                      <div className="space-y-1.5">
                        {fw.rules.map((rule) => (
                          <div key={rule.id} className="rounded bg-panel border border-border-window p-2">
                            <div className="text-sm font-medium text-foreground">{formatId(rule.id)}</div>
                            {rule.description && <p className="text-xs text-muted-foreground mt-0.5">{rule.description}</p>}
                            {rule.clauses && rule.clauses.length > 0 && (
                              <div className="mt-2 pl-3 border-l-2 border-border-window space-y-1">
                                {rule.clauses.map((clause) => (
                                  <div key={clause.id} className="text-xs">
                                    <span className="font-medium text-foreground">{formatId(clause.id)}</span>
                                    {clause.description && <span className="text-muted-foreground"> — {clause.description}</span>}
                                    {clause.properties && clause.properties.length > 0 && (
                                      <div className="mt-0.5 flex flex-wrap gap-1">
                                        {clause.properties.map((cp, idx) => (
                                          <span key={`${cp.id}-${cp.value}-${idx}`} className="bg-window border border-border-window px-1 py-0.5 rounded font-mono text-muted-foreground">
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
                                    <span key={k} className="text-xs bg-window border border-border-window px-1 py-0.5 rounded font-mono text-muted-foreground">
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
              <Collapsible
                key={platform.id}
                title={formatId(platform.id)}
                badge={<span className="text-xs text-muted-foreground">{platform.measures?.length ?? 0} measures</span>}
              >
                {platform.measures && platform.measures.length > 0 ? (
                  <div className="space-y-2">
                    {platform.measures.map((pm) => (
                      <div key={pm.id} className="rounded bg-panel border border-border-window p-2">
                        <div className="text-sm font-medium text-foreground font-mono">{pm.id}</div>
                        {pm.instructions && <p className="text-xs text-muted-foreground mt-0.5">{pm.instructions}</p>}
                        {pm.mcp?.tools && pm.mcp.tools.length > 0 && (
                          <div className="mt-2">
                            <span className="text-xs font-semibold text-muted-foreground">MCP Tools:</span>
                            <div className="mt-1 space-y-1">
                              {pm.mcp.tools.map((tool) => (
                                <div key={tool.id} className="text-xs pl-2 border-l border-border-window">
                                  <span className="font-mono text-active-base">{tool.id}</span>
                                  {tool.instruction && <span className="text-muted-foreground"> — {tool.instruction}</span>}
                                </div>
                              ))}
                            </div>
                          </div>
                        )}
                      </div>
                    ))}
                  </div>
                ) : (
                  <p className="text-xs text-muted-foreground">No measure instructions.</p>
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

// #endregion 🔖ConfigPage

// #region 🔖RunsPage
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖runspage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/RunsPage)

/**
 * Runs page showing current run, iterations list, and iteration details.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖runspage🛠️runspage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/RunsPage/d/i/RunsPage)
 *
 * MUST display the current run, all iterations, and the current iteration detail.
 **/
function RunsPage({ refreshKey }: { refreshKey: number }) {
  const { data: run, loading: runLoading, error: runError } = useCodaResource<Run>("coda://current-run", refreshKey);
  const { data: iterations, loading: itersLoading } = useCodaResource<Array<{ index: string }>>("coda://iterations", refreshKey);
  const { data: iteration, loading: iterLoading } = useCodaResource<Iteration>("coda://current-iteration", refreshKey);

  const loading = runLoading || itersLoading || iterLoading;
  if (loading) return <Spinner label="Loading runs..." />;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">Runs & Iterations</h2>
        <p className="text-sm text-muted-foreground mt-1">Manage and inspect compliance checking runs.</p>
      </div>

      <Card title="Current Run">
        {runError ? (
          <EmptyState message={runError} />
        ) : run ? (
          <div className="space-y-2">
            <div className="flex items-center gap-3">
              <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider w-16">ID</span>
              <span className="text-sm font-mono text-foreground">{run.id ?? run.run_id ?? "—"}</span>
            </div>
            {run.started && (
              <div className="flex items-center gap-3">
                <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider w-16">Started</span>
                <span className="text-sm text-foreground">{run.started}</span>
              </div>
            )}
          </div>
        ) : (
          <EmptyState message="No runs yet. Start a run from the Actions page." />
        )}
      </Card>

      <Card title={`Iterations (${iterations?.length ?? 0})`}>
        {iterations && iterations.length > 0 ? (
          <div className="space-y-1">
            {iterations.map((iter) => (
              <div
                key={iter.index}
                className={`flex items-center gap-3 rounded-md border px-3 py-2 text-sm ${String(iteration?.index) === iter.index ? "border-active-base bg-info-bg" : "border-border-window hover:bg-hover-window"
                  }`}
              >
                <span className="font-mono font-bold text-active-base">#{iter.index}</span>
                {String(iteration?.index) === iter.index && <span className="text-xs bg-active-base text-active-foreground px-1.5 py-0.5 rounded">current</span>}
              </div>
            ))}
          </div>
        ) : (
          <EmptyState message="No iterations yet." />
        )}
      </Card>

      {iteration && !iteration.error && (
        <Card title={`Current Iteration #${iteration.index}`}>
          <div className="space-y-2">
            {iteration.targets && iteration.targets.length > 0 && (
              <div>
                <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Targets</span>
                <div className="mt-1 flex flex-wrap gap-1.5">
                  {iteration.targets.map((tid) => (
                    <span key={tid} className="text-xs bg-panel border border-border-window px-2 py-1 rounded font-mono">
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

// #endregion 🔖RunsPage

// #region 🔖ReportPage
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖reportpage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ReportPage)

/**
 * Report page showing compliance report with rules and breaches.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖reportpage🛠️reportpage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ReportPage/d/i/ReportPage)
 *
 * MUST display the full report with expandable rules showing clause details.
 **/
function ReportPage({ refreshKey }: { refreshKey: number }) {
  const { data: report, loading: reportLoading, error: reportError } = useCodaResource<Report>("coda://report", refreshKey);
  const { data: breachs, loading: breachsLoading } = useCodaResource<Rule[]>("coda://breachs", refreshKey);

  const loading = reportLoading || breachsLoading;
  if (loading) return <Spinner label="Loading report..." />;

  const totalRules = report?.rules?.length ?? 0;
  const violatedRules = report?.rules?.filter((r) => r.status === "violated") ?? [];
  const compliantRules = report?.rules?.filter((r) => r.status === "compliant") ?? [];

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">Compliance Report</h2>
        <p className="text-sm text-muted-foreground mt-1">Results from the latest validation iteration.</p>
      </div>

      {reportError ? (
        <Card>
          <EmptyState message={reportError} />
        </Card>
      ) : !report?.rules || report.rules.length === 0 ? (
        <Card>
          <EmptyState message="No report available. Run validation from the Actions page." />
        </Card>
      ) : (
        <>
          <div className="grid grid-cols-3 gap-4">
            <StatCard label="Total Rules" value={totalRules} />
            <StatCard label="Compliant" value={compliantRules.length} sublabel={totalRules > 0 ? `${Math.round((compliantRules.length / totalRules) * 100)}%` : undefined} />
            <StatCard label="Violated" value={violatedRules.length} sublabel={totalRules > 0 ? `${Math.round((violatedRules.length / totalRules) * 100)}%` : undefined} />
          </div>

          {violatedRules.length > 0 && (
            <Card title={`Violations (${violatedRules.length})`}>
              <div className="space-y-2">
                {violatedRules.map((rule) => (
                  <Collapsible key={rule.id} title={formatId(rule.id)} defaultOpen badge={<StatusBadge status="violated" />}>
                    <div className="space-y-2">
                      {rule.description && <p className="text-sm text-muted-foreground">{rule.description}</p>}
                      {rule.clauses && rule.clauses.length > 0 && (
                        <div className="space-y-1.5">
                          {rule.clauses.map((clause) => (
                            <div key={clause.id} className="flex items-start gap-2 rounded bg-panel border border-border-window p-2">
                              <StatusBadge status={clause.status} />
                              <div>
                                <span className="text-sm font-medium text-foreground">{formatId(clause.id)}</span>
                                {clause.description && <p className="text-xs text-muted-foreground">{clause.description}</p>}
                              </div>
                            </div>
                          ))}
                        </div>
                      )}
                      {rule.measures && rule.measures.length > 0 && (
                        <div className="flex flex-wrap gap-1">
                          <span className="text-xs text-muted-foreground mr-1">Measures:</span>
                          {rule.measures.map((m) => (
                            <span key={m} className="text-xs bg-info-bg text-info-foreground px-1.5 py-0.5 rounded font-mono">
                              {m}
                            </span>
                          ))}
                        </div>
                      )}
                      {rule.data && Object.keys(rule.data).length > 0 && (
                        <div>
                          <span className="text-xs text-muted-foreground">Data:</span>
                          <JsonViewer data={rule.data} />
                        </div>
                      )}
                    </div>
                  </Collapsible>
                ))}
              </div>
            </Card>
          )}

          {compliantRules.length > 0 && (
            <Card title={`Compliant (${compliantRules.length})`}>
              <div className="space-y-1">
                {compliantRules.map((rule) => (
                  <div key={rule.id} className="flex items-center justify-between rounded-md border border-border-window px-3 py-2">
                    <div>
                      <span className="text-sm font-medium text-foreground">{formatId(rule.id)}</span>
                      {rule.description && <p className="text-xs text-muted-foreground mt-0.5">{rule.description}</p>}
                    </div>
                    <StatusBadge status="compliant" />
                  </div>
                ))}
              </div>
            </Card>
          )}

          {breachs && breachs.length > 0 && (
            <Card title={`Breachs (${breachs.length})`}>
              <JsonViewer data={breachs} />
            </Card>
          )}
        </>
      )}
    </div>
  );
}

// #endregion 🔖ReportPage

// #region 🔖TranslationsPage
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖translationspage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/TranslationsPage)

/**
 * Translations page showing translation outputs per target.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖translationspage🛠️translationspage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/TranslationsPage/d/i/TranslationsPage)
 *
 * MUST display translation data for each project target.
 **/
function TranslationsPage({ refreshKey }: { refreshKey: number }) {
  const { data: project } = useCodaResource<Project>("coda://project", refreshKey);
  const targetIds = useMemo(() => project?.targets?.map((t) => t.id) ?? [], [project]);

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">Translations</h2>
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
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖translationspage🛠️translationcard](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/TranslationsPage/d/i/TranslationCard)
 *
 * MUST handle loading and error states for translation data.
 **/
function TranslationCard({ targetId, refreshKey }: { targetId: string; refreshKey: number }) {
  const { data, loading, error } = useCodaResource<Record<string, unknown>>(`coda://translation/${targetId}`, refreshKey);

  return (
    <Card title={formatId(targetId)}>
      {loading ? (
        <Spinner label={`Loading ${targetId} translation...`} />
      ) : error ? (
        <EmptyState message={error} />
      ) : data ? (
        <JsonViewer data={data} />
      ) : (
        <EmptyState message="No translation data available." />
      )}
    </Card>
  );
}

// #endregion 🔖TranslationsPage

// #region 🔖ActionsPage
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖actionspage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ActionsPage)

/**
 * Actions page for invoking coda MCP tools.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖actionspage🛠️actionspage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ActionsPage/d/i/ActionsPage)
 *
 * MUST provide buttons for all coda tools and display results.
 **/
function ActionsPage({ refreshKey, onRefresh }: { refreshKey: number; onRefresh: () => void }) {
  const { data: project } = useCodaResource<Project>("coda://project", refreshKey);
  const targetIds = useMemo(() => project?.targets?.map((t) => t.id) ?? [], [project]);

  const [actionLog, setActionLog] = useState<Array<{ id: number; action: string; result: unknown; timestamp: string; success: boolean }>>([]);
  const [loading, setLoading] = useState<string | null>(null);

  const runTool = useCallback(async (name: string, args: Record<string, unknown>, label: string) => {
    setLoading(label);
    try {
      const response = await window.coda.tool(name, args);
      const result = parseMcpTool(response);
      setActionLog((prev) => [
        { id: Date.now(), action: label, result: result ?? response.error ?? "No response", timestamp: new Date().toLocaleTimeString(), success: !response.error },
        ...prev,
      ]);
      onRefresh();
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : "Unknown error";
      setActionLog((prev) => [{ id: Date.now(), action: label, result: { error: message }, timestamp: new Date().toLocaleTimeString(), success: false }, ...prev]);
    } finally {
      setLoading(null);
    }
  }, [onRefresh]);

  const runCall = useCallback(async (method: string, params: Record<string, unknown>, label: string) => {
    setLoading(label);
    try {
      const response = await window.coda.call(method, params);
      const result = response.result ?? response.error ?? "No response";
      setActionLog((prev) => [
        { id: Date.now(), action: label, result, timestamp: new Date().toLocaleTimeString(), success: !response.error },
        ...prev,
      ]);
      onRefresh();
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : "Unknown error";
      setActionLog((prev) => [{ id: Date.now(), action: label, result: { error: message }, timestamp: new Date().toLocaleTimeString(), success: false }, ...prev]);
    } finally {
      setLoading(null);
    }
  }, [onRefresh]);

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-foreground">Actions</h2>
        <p className="text-sm text-muted-foreground mt-1">Invoke coda tools to run compliance checking workflows.</p>
      </div>

      <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
        <Card title="Run Management">
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

        <Card title="Translation & Validation">
          <div className="space-y-3">
            {targetIds.length === 0 ? (
              <EmptyState message="No project targets found." />
            ) : (
              targetIds.map((tid) => (
                <TargetActionCard key={tid} targetId={tid} loading={loading} runTool={runTool} runCall={runCall} />
              ))
            )}
          </div>
        </Card>
      </div>

      <Card title="Fix Design">
        <FixAction loading={loading} onFix={(prompt) => runTool("fix", { prompt }, `Fix: ${prompt.slice(0, 30)}...`)} disabled={loading !== null} />
      </Card>

      <Card title="Manual Fix Result">
        <ManualFixInput loading={loading} onSubmit={(result) => runCall("save_report", { report_data: typeof result === "string" ? result : JSON.stringify(result) }, "Manual Save Report")} disabled={loading !== null} />
      </Card>

      {actionLog.length > 0 && (
        <Card title="Action Log" action={<Button onClick={() => setActionLog([])} className="text-xs">Clear</Button>}>
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
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖actionspage🛠️targetactioncard](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ActionsPage/d/i/TargetActionCard)
 *
 * MUST offer tool invocation and manual result input for translate and validate.
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
  const [manualMode, setManualMode] = useState<null | "translate" | "validate">(null);
  const [manualInput, setManualInput] = useState("");

  const handleManualSubmit = useCallback(() => {
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
    <div className="rounded border border-border-window p-3 space-y-2">
      <div className="text-sm font-medium text-foreground font-mono">{targetId}</div>
      <div className="flex gap-2">
        <Button
          onClick={() => runTool("translate", { target_id: targetId }, `Translate ${targetId}`)}
          loading={loading === `Translate ${targetId}`}
          disabled={loading !== null}
        >
          <IconTranslations className="w-3.5 h-3.5" />
          Translate
        </Button>
        <Button
          onClick={() => runTool("validate", { target_id: targetId }, `Validate ${targetId}`)}
          loading={loading === `Validate ${targetId}`}
          disabled={loading !== null}
        >
          <IconCheck className="w-3.5 h-3.5" />
          Validate
        </Button>
        <Button
          onClick={() => setManualMode(manualMode ? null : "translate")}
          variant={manualMode ? "primary" : "secondary"}
          className="ml-auto text-xs"
        >
          Manual
        </Button>
      </div>
      {manualMode && (
        <div className="space-y-2 pt-1">
          <div className="flex gap-2">
            <button
              onClick={() => { setManualMode("translate"); setManualInput(""); }}
              className={`text-xs px-2 py-1 rounded-full border cursor-pointer transition-colors ${manualMode === "translate" ? "bg-active-base text-active-foreground border-active-base" : "border-border-window text-muted-foreground hover:bg-hover-window"
                }`}
            >
              Translation
            </button>
            <button
              onClick={() => { setManualMode("validate"); setManualInput(""); }}
              className={`text-xs px-2 py-1 rounded-full border cursor-pointer transition-colors ${manualMode === "validate" ? "bg-active-base text-active-foreground border-active-base" : "border-border-window text-muted-foreground hover:bg-hover-window"
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
            className="w-full rounded-md border border-border-window bg-window px-3 py-2 text-xs font-mono text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base resize-y"
          />
          <div className="flex gap-2 justify-end">
            <Button onClick={() => { setManualMode(null); setManualInput(""); }} variant="secondary" className="text-xs">
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
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖actionspage🛠️manualfixinput](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ActionsPage/d/i/ManualFixInput)
 *
 * MUST provide a textarea to paste fix results and submit them.
 **/
function ManualFixInput({ loading, onSubmit, disabled }: { loading: string | null; onSubmit: (result: unknown) => void; disabled: boolean }) {
  const [input, setInput] = useState("");
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
        placeholder="Paste fix result JSON here..."
        rows={4}
        className="w-full rounded-md border border-border-window bg-window px-3 py-2 text-xs font-mono text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base resize-y"
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
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖actionspage🛠️fixaction](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ActionsPage/d/i/FixAction)
 *
 * MUST provide a text input for the fix prompt.
 **/
function FixAction({ loading, onFix, disabled }: { loading: string | null; onFix: (prompt: string) => void; disabled: boolean }) {
  const [prompt, setPrompt] = useState("");
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
          placeholder="e.g., Increase gross floor area to meet room program requirements"
          className="flex-1 rounded-md border border-border-window bg-window px-3 py-1.5 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base"
        />
        <Button variant="danger" onClick={handleSubmit} loading={loading?.startsWith("Fix:") ?? false} disabled={disabled || !prompt.trim()}>
          <IconWrench className="w-3.5 h-3.5" />
          Fix
        </Button>
      </div>
    </div>
  );
}

// #endregion 🔖ActionsPage

// #region 🔖EventsPage
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖eventspage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/EventsPage)
// Events page showing real-time event stream from the coda sidecar process.
// EventsPage MUST display all events with timestamps, kind, and full data.
// EventsPage MUST allow clearing and filtering events.

/**
 * Events page showing the real-time event log from the sidecar.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖eventspage🛠️eventspage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/EventsPage/d/i/EventsPage)
 *
 * MUST display all events in reverse chronological order.
 * MUST show event kind, timestamp, and full data payload.
 **/
function EventsPage({ events, onClear }: { events: CodaEvent[]; onClear: () => void }) {
  const [filter, setFilter] = useState("");

  const filteredEvents = useMemo(() => {
    if (!filter.trim()) return events;
    const lower = filter.toLowerCase();
    return events.filter((e) => e.event.toLowerCase().includes(lower) || JSON.stringify(e.data).toLowerCase().includes(lower));
  }, [events, filter]);

  const uniqueKinds = useMemo(() => {
    const kinds = new Set(events.map((e) => e.event));
    return Array.from(kinds).sort();
  }, [events]);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-lg font-bold text-foreground">Events</h2>
          <p className="text-sm text-muted-foreground mt-1">
            Real-time event stream from the coda sidecar ({events.length} total).
          </p>
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
          placeholder="Filter events by kind or content..."
          className="flex-1 rounded-md border border-border-window bg-window px-3 py-1.5 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base"
        />
        {uniqueKinds.length > 0 && (
          <div className="flex flex-wrap gap-1">
            {uniqueKinds.map((kind) => (
              <button
                key={kind}
                onClick={() => setFilter(filter === kind ? "" : kind)}
                className={`text-xs px-2 py-1 rounded-full border cursor-pointer transition-colors ${filter === kind ? "bg-active-base text-active-foreground border-active-base" : "border-border-window text-muted-foreground hover:bg-hover-window"
                  }`}
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
              <Collapsible
                key={`${evt.timestamp}-${idx}`}
                title={evt.event}
                badge={<span className="text-xs text-muted-foreground font-mono">{timeStr}</span>}
              >
                <JsonViewer data={evt.data} />
              </Collapsible>
            );
          })}
        </div>
      )}
    </div>
  );
}

// #endregion 🔖EventsPage

// #endregion 🔖Pages

// #region 🔖Welcome
// [🔬coda🖱️desktop💻renderer🔖renderer🔖welcome](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Welcome)
// Welcome screen shown on startup when no project is open.
// MUST offer two options: create a new project or open an existing one.

/**
 * Welcome screen with options to create or open a project.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖welcome🛠️welcomepage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Welcome/d/i/WelcomePage)
 *
 * MUST show create-new-project form and open-existing-project button.
 * MUST call onProjectReady with the resolved project path on success.
 **/
function WelcomePage({
  onProjectReady,
  onMinimize,
  onMaximize,
  onClose,
}: {
  onProjectReady: (projectPath: string) => void;
  onMinimize: () => void;
  onMaximize: () => void;
  onClose: () => void;
}) {
  const [mode, setMode] = useState<"choose" | "create" | "open">("choose");
  const [projectName, setProjectName] = useState("");
  const [selectedFolder, setSelectedFolder] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handlePickFolder = useCallback(async () => {
    const folder = await window.dialog.openFolder();
    if (folder) {
      setSelectedFolder(folder);
      setError(null);
    }
  }, []);

  const handleCreate = useCallback(async () => {
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

  const handleOpen = useCallback(async () => {
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
    <div className="flex h-screen w-screen flex-col bg-window overflow-hidden">
      {/* Title Bar */}
      <div className="flex h-9 items-center border-b border-border-window bg-panel px-3 shrink-0" style={{ WebkitAppRegion: "drag" } as React.CSSProperties}>
        <div className="flex items-center gap-2 flex-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <span className="text-sm font-bold text-active-base">coda</span>
          <span className="text-xs text-muted-foreground">ACC Design Assistant</span>
        </div>
        <div className="flex items-center gap-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <button onClick={onMinimize} className="rounded p-1.5 text-muted-foreground hover:bg-hover-window hover:text-foreground transition-colors cursor-pointer">
            <IconMinimize />
          </button>
          <button onClick={onMaximize} className="rounded p-1.5 text-muted-foreground hover:bg-hover-window hover:text-foreground transition-colors cursor-pointer">
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
            <p className="mt-2 text-muted-foreground">ACC Design Assistant</p>
          </div>

          {mode === "choose" && (
            <div className="grid grid-cols-2 gap-6">
              {/* #region Create New Project Card */}
              <button
                onClick={() => { setMode("create"); setError(null); }}
                className="group flex flex-col items-center gap-4 rounded-xl border-2 border-border-window bg-window p-8 text-left transition-all hover:border-active-base hover:bg-info-bg cursor-pointer"
              >
                <div className="rounded-full bg-info-bg p-4 transition-colors group-hover:bg-hover-window">
                  <svg className="w-8 h-8 text-active-base" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                    <polyline points="14 2 14 8 20 8" />
                    <line x1="12" y1="18" x2="12" y2="12" />
                    <line x1="9" y1="15" x2="15" y2="15" />
                  </svg>
                </div>
                <div className="text-center">
                  <div className="text-base font-semibold text-foreground">Create New Project</div>
                  <p className="mt-1 text-sm text-muted-foreground">Start fresh with a new coda project in a folder of your choice.</p>
                </div>
              </button>
              {/* #endregion */}

              {/* #region Open Existing Project Card */}
              <button
                onClick={handleOpen}
                disabled={loading}
                className="group flex flex-col items-center gap-4 rounded-xl border-2 border-border-window bg-window p-8 text-left transition-all hover:border-active-base hover:bg-info-bg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <div className="rounded-full bg-info-bg p-4 transition-colors group-hover:bg-hover-window">
                  <svg className="w-8 h-8 text-active-base" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
                  </svg>
                </div>
                <div className="text-center">
                  <div className="text-base font-semibold text-foreground">Open Existing Project</div>
                  <p className="mt-1 text-sm text-muted-foreground">Open a folder that already contains a coda project configuration.</p>
                </div>
              </button>
              {/* #endregion */}
            </div>
          )}

          {(mode === "create" || mode === "open") && (
            <div className="rounded-xl border border-border-window bg-window p-6 space-y-5">
              <div className="flex items-center gap-2">
                <button onClick={() => { setMode("choose"); setError(null); }} className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer">
                  <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                    <polyline points="15 18 9 12 15 6" />
                  </svg>
                </button>
                <h2 className="text-base font-semibold text-foreground">
                  {mode === "create" ? "Create New Project" : "Open Existing Project"}
                </h2>
              </div>

              {mode === "create" && (
                <div className="space-y-4">
                  <div className="space-y-1.5">
                    <label className="text-sm font-medium text-foreground">Project Name</label>
                    <input
                      type="text"
                      value={projectName}
                      onChange={(e) => setProjectName(e.target.value)}
                      onKeyDown={(e) => e.key === "Enter" && handleCreate()}
                      placeholder="My Project"
                      className="w-full rounded-md border border-border-window bg-window px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-active-base focus:border-active-base"
                      autoFocus
                    />
                  </div>

                  <div className="space-y-1.5">
                    <label className="text-sm font-medium text-foreground">Project Folder</label>
                    <div className="flex gap-2">
                      <div className="flex-1 rounded-md border border-border-window bg-window px-3 py-2 text-sm text-muted-foreground truncate">
                        {selectedFolder ?? "No folder selected"}
                      </div>
                      <Button onClick={handlePickFolder} variant="secondary">
                        Browse…
                      </Button>
                    </div>
                    <p className="text-xs text-muted-foreground">A <code className="font-mono">.coda/project.json</code> will be created in this folder.</p>
                  </div>
                </div>
              )}

              {mode === "open" && (
                <div className="space-y-4">
                  <div className="flex gap-2">
                    <div className="flex-1 rounded-md border border-border-window bg-window px-3 py-2 text-sm text-muted-foreground truncate">
                      {selectedFolder ?? "No folder selected"}
                    </div>
                    <Button onClick={handlePickFolder} variant="secondary">
                      Browse…
                    </Button>
                  </div>
                  <p className="text-xs text-muted-foreground">Select a folder that contains a <code className="font-mono">.coda/project.json</code> file.</p>
                </div>
              )}

              {error && (
                <div className="rounded-md border border-destructive-border bg-destructive-bg px-3 py-2 text-sm text-destructive-foreground">
                  {error}
                </div>
              )}

              <div className="flex justify-end gap-2 pt-1">
                <Button onClick={() => { setMode("choose"); setError(null); }} variant="secondary">
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

          {error && mode === "choose" && (
            <div className="rounded-md border border-destructive-border bg-destructive-bg px-3 py-2 text-sm text-destructive-foreground text-center">
              {error}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

// #endregion 🔖Welcome

// #region 🔖App
// [🔬coda🖱️desktop💻renderer🔖renderer🔖app](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/App)
// Root application component with sidebar navigation, title bar, and page routing.
// App MUST render the frameless window with custom title bar and navigation.

/**
 * Navigation item configuration.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖app🪨navitems](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/App/d/i/navItems)
 *
 * MUST define all navigable pages with icons and labels.
 **/
const navItems: Array<{ id: Page; label: string; icon: React.ComponentType<{ className?: string }> }> = [
  { id: "dashboard", label: "Dashboard", icon: IconDashboard },
  { id: "config", label: "Config", icon: IconConfig },
  { id: "runs", label: "Runs", icon: IconRuns },
  { id: "report", label: "Report", icon: IconReport },
  { id: "translations", label: "Translations", icon: IconTranslations },
  { id: "actions", label: "Actions", icon: IconActions },
  { id: "events", label: "Events", icon: IconEvents },
];

/**
 * Root React component that renders the coda desktop app.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖app🛠️app](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/App/d/i/App)
 *
 * MUST show WelcomePage until a project is selected.
 * MUST show loading state until user ID is resolved.
 * MUST provide sidebar navigation and page content area.
 **/
function App() {
  const [userId, setUserId] = useState<string>("");
  const [projectPath, setProjectPath] = useState<string | null | undefined>(undefined);
  const [currentPage, setCurrentPage] = useState<Page>("dashboard");
  const [refreshKey, setRefreshKey] = useState(0);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
  const [sidecarConnected, setSidecarConnected] = useState(false);
  const [events, setEvents] = useState<CodaEvent[]>([]);

  useEffect(() => {
    async function init() {
      try {
        const [id, path, connected] = await Promise.all([
          window.os.getUserId(),
          window.project.getPath(),
          window.coda.getConnectionStatus(),
        ]);
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

  useEffect(() => {
    const unsubEvent = window.coda.onEvent((evt: CodaEvent) => {
      setEvents((prev) => [evt, ...prev]);
    });
    const unsubConnection = window.coda.onConnectionStatus((connected: boolean) => {
      setSidecarConnected(connected);
    });
    return () => {
      unsubEvent();
      unsubConnection();
    };
  }, []);

  const handleClearEvents = useCallback(() => setEvents([]), []);

  const handleRefresh = useCallback(() => setRefreshKey((n) => n + 1), []);

  const handleMinimize = useCallback(() => {
    if (window.windowControls) window.windowControls.minimize();
  }, []);

  const handleMaximize = useCallback(() => {
    if (window.windowControls) window.windowControls.maximize();
  }, []);

  const handleClose = useCallback(() => {
    if (window.windowControls) window.windowControls.close();
  }, []);

  if (projectPath === undefined) {
    return (
      <div className="flex h-screen w-screen items-center justify-center bg-window">
        <Spinner label="Loading..." />
      </div>
    );
  }

  if (!projectPath) {
    return (
      <WelcomePage
        onProjectReady={(p) => setProjectPath(p)}
        onMinimize={handleMinimize}
        onMaximize={handleMaximize}
        onClose={handleClose}
      />
    );
  }

  const projectName = projectPath.split("/").pop() ?? projectPath;

  return (
    <div className="flex h-screen w-screen flex-col bg-window overflow-hidden">
      {/* #region Title Bar */}
      <div className="flex h-9 items-center border-b border-border-window bg-panel px-3 shrink-0" style={{ WebkitAppRegion: "drag" } as React.CSSProperties}>
        <div className="flex items-center gap-2 flex-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <span className="text-sm font-bold text-active-base">coda</span>
          <span className="text-xs text-muted-foreground">ACC Design Assistant</span>
          <span className="text-xs text-muted-foreground ml-1">|</span>
          <span className="text-xs text-muted-foreground ml-1 font-mono" title={projectPath}>{projectName}</span>
          {userId && <><span className="text-xs text-muted-foreground ml-1">·</span><span className="text-xs text-muted-foreground ml-1">{userId}</span></>}
          <span className="text-xs text-muted-foreground ml-1">·</span>
          <span className={`ml-1 inline-flex items-center gap-1 text-xs ${sidecarConnected ? "text-success-foreground" : "text-destructive-foreground"}`} title={sidecarConnected ? "Sidecar connected" : "Sidecar disconnected (offline mode)"}>
            <span className={`w-1.5 h-1.5 rounded-full ${sidecarConnected ? "bg-success-border" : "bg-destructive-border"}`} />
            {sidecarConnected ? "Connected" : "Offline"}
          </span>
        </div>
        <div className="flex items-center gap-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <button onClick={handleRefresh} className="rounded p-1.5 text-muted-foreground hover:bg-hover-window hover:text-foreground transition-colors cursor-pointer" title="Refresh data">
            <IconRefresh className="w-3.5 h-3.5" />
          </button>
          <button onClick={handleMinimize} className="rounded p-1.5 text-muted-foreground hover:bg-hover-window hover:text-foreground transition-colors cursor-pointer">
            <IconMinimize />
          </button>
          <button onClick={handleMaximize} className="rounded p-1.5 text-muted-foreground hover:bg-hover-window hover:text-foreground transition-colors cursor-pointer">
            <IconMaximize />
          </button>
          <button onClick={handleClose} className="rounded p-1.5 text-muted-foreground hover:bg-destructive-bg hover:text-destructive-foreground transition-colors cursor-pointer">
            <IconClose />
          </button>
        </div>
      </div>
      {/* #endregion Title Bar */}

      <div className="flex flex-1 overflow-hidden">
        {/* #region Sidebar */}
        <nav className={`flex flex-col border-r border-border-window bg-panel shrink-0 transition-all duration-200 ${sidebarCollapsed ? "w-12" : "w-48"}`}>
          <div className="flex-1 py-2">
            {navItems.map((item) => {
              const Icon = item.icon;
              const active = currentPage === item.id;
              return (
                <button
                  key={item.id}
                  onClick={() => setCurrentPage(item.id)}
                  className={`flex w-full items-center gap-2.5 px-3 py-2 text-sm transition-colors cursor-pointer ${active ? "bg-info-bg text-active-base border-r-2 border-active-base" : "text-muted-foreground hover:bg-hover-window hover:text-foreground"
                    }`}
                  title={item.label}
                >
                  <Icon className="w-4 h-4 shrink-0" />
                  {!sidebarCollapsed && <span className="truncate">{item.label}</span>}
                </button>
              );
            })}
          </div>
          <button
            onClick={() => setSidebarCollapsed(!sidebarCollapsed)}
            className="border-t border-border-window p-2 text-muted-foreground hover:text-foreground hover:bg-hover-window transition-colors cursor-pointer"
            title={sidebarCollapsed ? "Expand sidebar" : "Collapse sidebar"}
          >
            <IconChevronRight className={`w-4 h-4 mx-auto transition-transform ${sidebarCollapsed ? "" : "rotate-180"}`} />
          </button>
        </nav>
        {/* #endregion Sidebar */}

        {/* #region Content */}
        <main className="flex-1 overflow-y-auto p-6">
          {currentPage === "dashboard" && <DashboardPage refreshKey={refreshKey} />}
          {currentPage === "config" && <ConfigPage refreshKey={refreshKey} />}
          {currentPage === "runs" && <RunsPage refreshKey={refreshKey} />}
          {currentPage === "report" && <ReportPage refreshKey={refreshKey} />}
          {currentPage === "translations" && <TranslationsPage refreshKey={refreshKey} />}
          {currentPage === "actions" && <ActionsPage refreshKey={refreshKey} onRefresh={handleRefresh} />}
          {currentPage === "events" && <EventsPage events={events} onClear={handleClearEvents} />}
        </main>
        {/* #endregion Content */}
      </div>
    </div>
  );
}

export default App;

export { OntologyTree, ValidationTree };
export type { OntologyTreeNode, OntologyNodeKind, ValidationTreeNode, ValidationNodeKind, TruthValue, ValidationReport };

const rootElement = document.getElementById("root");
if (rootElement) {
  createRoot(rootElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
}

// #endregion 🔖App

// #endregion 🔖Renderer
