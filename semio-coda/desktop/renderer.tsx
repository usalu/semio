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
      fetch(uri: string): Promise<McpResponse>;
      tool(name: string, args: Record<string, unknown>): Promise<McpResponse>;
    };
  }
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
 * A property definition on a target.
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
  levels?: Array<{ value: string; name?: string; description?: string }>;
}

/**
 * A compliance target with properties and rules.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖types🛠️target](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Types/d/i/Target)
 *
 * MUST have an id.
 **/
interface Target {
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
type Page = "dashboard" | "config" | "runs" | "report" | "translations" | "actions";

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
    compliant: "bg-compliant/15 text-compliant border-compliant/30",
    violated: "bg-violated/15 text-violated border-violated/30",
    unknown: "bg-unknown/15 text-unknown border-unknown/30",
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
    <div className={`rounded-lg border border-border bg-surface ${className}`}>
      {(title || action) && (
        <div className="flex items-center justify-between border-b border-border px-4 py-3">
          {title && <h3 className="text-sm font-semibold text-text">{title}</h3>}
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
    <div className="rounded-lg border border-border bg-surface p-4">
      <div className="text-xs font-medium text-text-tertiary uppercase tracking-wider">{label}</div>
      <div className="mt-1 text-2xl font-bold text-text">{value}</div>
      {sublabel && <div className="mt-0.5 text-xs text-text-secondary">{sublabel}</div>}
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
    primary: "bg-coda-600 text-white hover:bg-coda-700 disabled:bg-coda-400",
    secondary: "bg-surface border border-border text-text hover:bg-surface-hover disabled:opacity-50",
    danger: "bg-violated/10 text-violated border border-violated/30 hover:bg-violated/20 disabled:opacity-50",
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
    <div className="flex flex-col items-center justify-center gap-3 py-12 text-text-tertiary">
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
    <div className="flex flex-col items-center justify-center gap-3 py-12 text-text-tertiary">
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
    <div className="border border-border rounded-md overflow-hidden">
      <button onClick={() => setOpen(!open)} className="flex w-full items-center gap-2 px-3 py-2 text-sm font-medium text-text hover:bg-surface-hover transition-colors cursor-pointer">
        {open ? <IconChevronDown className="w-3.5 h-3.5 text-text-tertiary" /> : <IconChevronRight className="w-3.5 h-3.5 text-text-tertiary" />}
        <span className="flex-1 text-left">{title}</span>
        {badge}
      </button>
      {open && <div className="border-t border-border px-3 py-2">{children}</div>}
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
    <pre className="overflow-auto max-h-96 rounded-md bg-surface-alt border border-border p-3 text-xs font-mono text-text-secondary whitespace-pre-wrap break-all">
      {formatted}
    </pre>
  );
}

// #endregion 🔖JsonViewer

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
  const { data: targets } = useCodaResource<Target[]>("coda://targets", refreshKey);

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
        <h2 className="text-lg font-bold text-text">Dashboard</h2>
        <p className="text-sm text-text-secondary mt-1">Overview of the coda compliance checking status.</p>
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
              <div key={rule.id} className="flex items-center justify-between rounded-md border border-border px-3 py-2">
                <div>
                  <span className="text-sm font-medium text-text">{formatId(rule.id)}</span>
                  {rule.description && <p className="text-xs text-text-secondary mt-0.5">{rule.description}</p>}
                </div>
                <StatusBadge status={rule.status} />
              </div>
            ))}
          </div>
        </Card>
      )}

      <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
        {measures && measures.length > 0 && (
          <Card title={`Measures (${measures.length})`}>
            <div className="space-y-1">
              {measures.map((m) => (
                <div key={m.id} className="flex items-start gap-2 rounded px-2 py-1.5 text-sm hover:bg-surface-hover">
                  <IconWrench className="w-3.5 h-3.5 text-text-tertiary mt-0.5 shrink-0" />
                  <div>
                    <span className="font-medium text-text">{formatId(m.id)}</span>
                    {m.description && <p className="text-xs text-text-secondary">{m.description}</p>}
                  </div>
                </div>
              ))}
            </div>
          </Card>
        )}

        {targets && targets.length > 0 && (
          <Card title={`Targets (${targets.length})`}>
            <div className="space-y-1">
              {targets.map((t) => (
                <div key={t.id} className="flex items-start gap-2 rounded px-2 py-1.5 text-sm hover:bg-surface-hover">
                  <IconReport className="w-3.5 h-3.5 text-text-tertiary mt-0.5 shrink-0" />
                  <div>
                    <span className="font-medium text-text">{formatId(t.id)}</span>
                    <p className="text-xs text-text-secondary">
                      {t.properties?.length ?? 0} properties, {t.rules?.length ?? 0} rules
                    </p>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        )}
      </div>
    </div>
  );
}

// #endregion 🔖DashboardPage

// #region 🔖ConfigPage
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖configpage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ConfigPage)

/**
 * Configuration page showing measures, targets (with properties and rules), and platforms.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖pages🔖configpage🛠️configpage](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/Pages/s/ConfigPage/d/i/ConfigPage)
 *
 * MUST display all coda configuration in expandable sections.
 **/
function ConfigPage({ refreshKey }: { refreshKey: number }) {
  const { data: measures, loading: measuresLoading } = useCodaResource<Measure[]>("coda://measures", refreshKey);
  const { data: targets, loading: targetsLoading } = useCodaResource<Target[]>("coda://targets", refreshKey);
  const { data: platforms, loading: platformsLoading } = useCodaResource<Platform[]>("coda://platforms", refreshKey);

  const loading = measuresLoading || targetsLoading || platformsLoading;
  if (loading) return <Spinner label="Loading configuration..." />;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-text">Configuration</h2>
        <p className="text-sm text-text-secondary mt-1">Measures, targets, and platforms from the coda configuration.</p>
      </div>

      <Card title={`Measures (${measures?.length ?? 0})`}>
        {measures && measures.length > 0 ? (
          <div className="divide-y divide-border">
            {measures.map((m) => (
              <div key={m.id} className="py-2 first:pt-0 last:pb-0">
                <div className="text-sm font-medium text-text font-mono">{m.id}</div>
                {m.description && <p className="text-xs text-text-secondary mt-0.5">{m.description}</p>}
              </div>
            ))}
          </div>
        ) : (
          <EmptyState message="No measures configured." />
        )}
      </Card>

      <Card title={`Targets (${targets?.length ?? 0})`}>
        {targets && targets.length > 0 ? (
          <div className="space-y-3">
            {targets.map((target) => (
              <Collapsible key={target.id} title={formatId(target.id)} badge={<span className="text-xs text-text-tertiary">{target.rules?.length ?? 0} rules</span>}>
                <div className="space-y-3">
                  {target.properties && target.properties.length > 0 && (
                    <div>
                      <h4 className="text-xs font-semibold text-text-tertiary uppercase tracking-wider mb-2">Properties</h4>
                      <div className="space-y-1.5">
                        {target.properties.map((prop) => (
                          <div key={prop.id} className="rounded bg-surface-alt border border-border p-2">
                            <div className="flex items-center gap-2">
                              <span className="text-sm font-medium text-text">{prop.name ?? formatId(prop.id)}</span>
                              {prop.type && <span className="text-xs bg-coda-100 text-coda-700 px-1.5 py-0.5 rounded">{prop.type}</span>}
                            </div>
                            {prop.description && <p className="text-xs text-text-secondary mt-1">{prop.description}</p>}
                            {prop.url && (
                              <a className="text-xs text-coda-600 hover:underline mt-1 inline-block" href={prop.url} target="_blank" rel="noreferrer">
                                Reference
                              </a>
                            )}
                            {prop.levels && prop.levels.length > 0 && (
                              <div className="mt-2 space-y-1">
                                {prop.levels.map((level) => (
                                  <div key={level.value} className="flex items-start gap-2 text-xs">
                                    <span className="bg-coda-600 text-white px-1.5 py-0.5 rounded font-mono shrink-0">{level.value}</span>
                                    <div>
                                      {level.name && <span className="font-medium text-text">{level.name}</span>}
                                      {level.description && <p className="text-text-secondary">{level.description}</p>}
                                    </div>
                                  </div>
                                ))}
                              </div>
                            )}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}

                  {target.rules && target.rules.length > 0 && (
                    <div>
                      <h4 className="text-xs font-semibold text-text-tertiary uppercase tracking-wider mb-2">Rules</h4>
                      <div className="space-y-1.5">
                        {target.rules.map((rule) => (
                          <div key={rule.id} className="rounded bg-surface-alt border border-border p-2">
                            <div className="text-sm font-medium text-text">{formatId(rule.id)}</div>
                            {rule.description && <p className="text-xs text-text-secondary mt-0.5">{rule.description}</p>}
                            {rule.clauses && rule.clauses.length > 0 && (
                              <div className="mt-2 pl-3 border-l-2 border-border space-y-1">
                                {rule.clauses.map((clause) => (
                                  <div key={clause.id} className="text-xs">
                                    <span className="font-medium text-text">{formatId(clause.id)}</span>
                                    {clause.description && <span className="text-text-secondary"> — {clause.description}</span>}
                                  </div>
                                ))}
                              </div>
                            )}
                            {rule.measures && rule.measures.length > 0 && (
                              <div className="mt-1.5 flex flex-wrap gap-1">
                                {rule.measures.map((m) => (
                                  <span key={m} className="text-xs bg-coda-100 text-coda-700 px-1.5 py-0.5 rounded font-mono">
                                    {m}
                                  </span>
                                ))}
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
          <EmptyState message="No targets configured." />
        )}
      </Card>

      <Card title={`Platforms (${platforms?.length ?? 0})`}>
        {platforms && platforms.length > 0 ? (
          <div className="space-y-3">
            {platforms.map((platform) => (
              <Collapsible
                key={platform.id}
                title={formatId(platform.id)}
                badge={<span className="text-xs text-text-tertiary">{platform.measures?.length ?? 0} measures</span>}
              >
                {platform.measures && platform.measures.length > 0 ? (
                  <div className="space-y-2">
                    {platform.measures.map((pm) => (
                      <div key={pm.id} className="rounded bg-surface-alt border border-border p-2">
                        <div className="text-sm font-medium text-text font-mono">{pm.id}</div>
                        {pm.instructions && <p className="text-xs text-text-secondary mt-0.5">{pm.instructions}</p>}
                        {pm.mcp?.tools && pm.mcp.tools.length > 0 && (
                          <div className="mt-2">
                            <span className="text-xs font-semibold text-text-tertiary">MCP Tools:</span>
                            <div className="mt-1 space-y-1">
                              {pm.mcp.tools.map((tool) => (
                                <div key={tool.id} className="text-xs pl-2 border-l border-border">
                                  <span className="font-mono text-coda-600">{tool.id}</span>
                                  {tool.instruction && <span className="text-text-secondary"> — {tool.instruction}</span>}
                                </div>
                              ))}
                            </div>
                          </div>
                        )}
                      </div>
                    ))}
                  </div>
                ) : (
                  <p className="text-xs text-text-tertiary">No measure instructions.</p>
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
        <h2 className="text-lg font-bold text-text">Runs & Iterations</h2>
        <p className="text-sm text-text-secondary mt-1">Manage and inspect compliance checking runs.</p>
      </div>

      <Card title="Current Run">
        {runError ? (
          <EmptyState message={runError} />
        ) : run ? (
          <div className="space-y-2">
            <div className="flex items-center gap-3">
              <span className="text-xs font-semibold text-text-tertiary uppercase tracking-wider w-16">ID</span>
              <span className="text-sm font-mono text-text">{run.id ?? run.run_id ?? "—"}</span>
            </div>
            {run.started && (
              <div className="flex items-center gap-3">
                <span className="text-xs font-semibold text-text-tertiary uppercase tracking-wider w-16">Started</span>
                <span className="text-sm text-text">{run.started}</span>
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
                className={`flex items-center gap-3 rounded-md border px-3 py-2 text-sm ${
                  String(iteration?.index) === iter.index ? "border-coda-400 bg-coda-50" : "border-border hover:bg-surface-hover"
                }`}
              >
                <span className="font-mono font-bold text-coda-600">#{iter.index}</span>
                {String(iteration?.index) === iter.index && <span className="text-xs bg-coda-600 text-white px-1.5 py-0.5 rounded">current</span>}
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
                <span className="text-xs font-semibold text-text-tertiary uppercase tracking-wider">Targets</span>
                <div className="mt-1 flex flex-wrap gap-1.5">
                  {iteration.targets.map((tid) => (
                    <span key={tid} className="text-xs bg-surface-alt border border-border px-2 py-1 rounded font-mono">
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
        <h2 className="text-lg font-bold text-text">Compliance Report</h2>
        <p className="text-sm text-text-secondary mt-1">Results from the latest validation iteration.</p>
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
                      {rule.description && <p className="text-sm text-text-secondary">{rule.description}</p>}
                      {rule.clauses && rule.clauses.length > 0 && (
                        <div className="space-y-1.5">
                          {rule.clauses.map((clause) => (
                            <div key={clause.id} className="flex items-start gap-2 rounded bg-surface-alt border border-border p-2">
                              <StatusBadge status={clause.status} />
                              <div>
                                <span className="text-sm font-medium text-text">{formatId(clause.id)}</span>
                                {clause.description && <p className="text-xs text-text-secondary">{clause.description}</p>}
                              </div>
                            </div>
                          ))}
                        </div>
                      )}
                      {rule.measures && rule.measures.length > 0 && (
                        <div className="flex flex-wrap gap-1">
                          <span className="text-xs text-text-tertiary mr-1">Measures:</span>
                          {rule.measures.map((m) => (
                            <span key={m} className="text-xs bg-coda-100 text-coda-700 px-1.5 py-0.5 rounded font-mono">
                              {m}
                            </span>
                          ))}
                        </div>
                      )}
                      {rule.data && Object.keys(rule.data).length > 0 && (
                        <div>
                          <span className="text-xs text-text-tertiary">Data:</span>
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
                  <div key={rule.id} className="flex items-center justify-between rounded-md border border-border px-3 py-2">
                    <div>
                      <span className="text-sm font-medium text-text">{formatId(rule.id)}</span>
                      {rule.description && <p className="text-xs text-text-secondary mt-0.5">{rule.description}</p>}
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
        <h2 className="text-lg font-bold text-text">Translations</h2>
        <p className="text-sm text-text-secondary mt-1">Translation outputs for each target in the current iteration.</p>
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

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-bold text-text">Actions</h2>
        <p className="text-sm text-text-secondary mt-1">Invoke coda tools to run compliance checking workflows.</p>
      </div>

      <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
        <Card title="Run Management">
          <div className="space-y-3">
            <div className="flex items-start gap-3">
              <Button variant="primary" onClick={() => runTool("start_run", {}, "Start Run")} loading={loading === "Start Run"} disabled={loading !== null}>
                <IconPlay className="w-3.5 h-3.5" />
                Start Run
              </Button>
              <p className="text-xs text-text-secondary pt-1">Create a new compliance checking run.</p>
            </div>
            <div className="flex items-start gap-3">
              <Button variant="primary" onClick={() => runTool("start_iteration", {}, "Start Iteration")} loading={loading === "Start Iteration"} disabled={loading !== null}>
                <IconRuns className="w-3.5 h-3.5" />
                Start Iteration
              </Button>
              <p className="text-xs text-text-secondary pt-1">Begin a new iteration in the current run.</p>
            </div>
          </div>
        </Card>

        <Card title="Translation & Validation">
          <div className="space-y-3">
            {targetIds.length === 0 ? (
              <EmptyState message="No project targets found." />
            ) : (
              targetIds.map((tid) => (
                <div key={tid} className="rounded border border-border p-3 space-y-2">
                  <div className="text-sm font-medium text-text font-mono">{tid}</div>
                  <div className="flex gap-2">
                    <Button
                      onClick={() => runTool("translate", { target_id: tid }, `Translate ${tid}`)}
                      loading={loading === `Translate ${tid}`}
                      disabled={loading !== null}
                    >
                      <IconTranslations className="w-3.5 h-3.5" />
                      Translate
                    </Button>
                    <Button
                      onClick={() => runTool("validate", { target_id: tid }, `Validate ${tid}`)}
                      loading={loading === `Validate ${tid}`}
                      disabled={loading !== null}
                    >
                      <IconCheck className="w-3.5 h-3.5" />
                      Validate
                    </Button>
                  </div>
                </div>
              ))
            )}
          </div>
        </Card>
      </div>

      <Card title="Fix Design">
        <FixAction loading={loading} onFix={(prompt) => runTool("fix", { prompt }, `Fix: ${prompt.slice(0, 30)}...`)} disabled={loading !== null} />
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
                    <span className="text-xs text-text-tertiary">{entry.timestamp}</span>
                    <span className={`w-2 h-2 rounded-full ${entry.success ? "bg-compliant" : "bg-violated"}`} />
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
      <p className="text-xs text-text-secondary">Describe what should be fixed in the design to address compliance breaches.</p>
      <div className="flex gap-2">
        <input
          type="text"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSubmit()}
          placeholder="e.g., Increase gross floor area to meet room program requirements"
          className="flex-1 rounded-md border border-border bg-surface px-3 py-1.5 text-sm text-text placeholder:text-text-tertiary focus:outline-none focus:ring-2 focus:ring-coda-400 focus:border-coda-400"
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

// #endregion 🔖Pages

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
];

/**
 * Root React component that renders the coda desktop app.
// [🔬coda🖱️desktop💻renderer🔖renderer🔖app🛠️app](semiorepo://p/r/coda/b/u/desktop/f/renderer.tsx/s/Renderer/s/App/d/i/App)
 *
 * MUST show loading state until user ID is resolved.
 * MUST provide sidebar navigation and page content area.
 **/
function App() {
  const [userId, setUserId] = useState<string>("");
  const [currentPage, setCurrentPage] = useState<Page>("dashboard");
  const [refreshKey, setRefreshKey] = useState(0);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  useEffect(() => {
    async function fetchUserId() {
      try {
        const id = await window.os.getUserId();
        setUserId(id);
      } catch {
        setUserId("anonymous-user");
      }
    }
    fetchUserId();
  }, []);

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

  if (!userId) {
    return (
      <div className="flex h-screen w-screen items-center justify-center bg-surface">
        <Spinner label="Loading user data..." />
      </div>
    );
  }

  return (
    <div className="flex h-screen w-screen flex-col bg-surface overflow-hidden">
      {/* #region Title Bar */}
      <div className="flex h-9 items-center border-b border-border bg-surface-alt px-3 shrink-0" style={{ WebkitAppRegion: "drag" } as React.CSSProperties}>
        <div className="flex items-center gap-2 flex-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <span className="text-sm font-bold text-coda-600">coda</span>
          <span className="text-xs text-text-tertiary">ACC Design Assistant</span>
          <span className="text-xs text-text-tertiary ml-1">|</span>
          <span className="text-xs text-text-tertiary ml-1">{userId}</span>
        </div>
        <div className="flex items-center gap-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <button onClick={handleRefresh} className="rounded p-1.5 text-text-secondary hover:bg-surface-hover hover:text-text transition-colors cursor-pointer" title="Refresh data">
            <IconRefresh className="w-3.5 h-3.5" />
          </button>
          <button onClick={handleMinimize} className="rounded p-1.5 text-text-secondary hover:bg-surface-hover hover:text-text transition-colors cursor-pointer">
            <IconMinimize />
          </button>
          <button onClick={handleMaximize} className="rounded p-1.5 text-text-secondary hover:bg-surface-hover hover:text-text transition-colors cursor-pointer">
            <IconMaximize />
          </button>
          <button onClick={handleClose} className="rounded p-1.5 text-text-secondary hover:bg-violated/20 hover:text-violated transition-colors cursor-pointer">
            <IconClose />
          </button>
        </div>
      </div>
      {/* #endregion Title Bar */}

      <div className="flex flex-1 overflow-hidden">
        {/* #region Sidebar */}
        <nav className={`flex flex-col border-r border-border bg-surface-alt shrink-0 transition-all duration-200 ${sidebarCollapsed ? "w-12" : "w-48"}`}>
          <div className="flex-1 py-2">
            {navItems.map((item) => {
              const Icon = item.icon;
              const active = currentPage === item.id;
              return (
                <button
                  key={item.id}
                  onClick={() => setCurrentPage(item.id)}
                  className={`flex w-full items-center gap-2.5 px-3 py-2 text-sm transition-colors cursor-pointer ${
                    active ? "bg-coda-50 text-coda-700 border-r-2 border-coda-600" : "text-text-secondary hover:bg-surface-hover hover:text-text"
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
            className="border-t border-border p-2 text-text-tertiary hover:text-text hover:bg-surface-hover transition-colors cursor-pointer"
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
        </main>
        {/* #endregion Content */}
      </div>
    </div>
  );
}

export default App;

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

// #endregion 🔖App

// #endregion 🔖Renderer
