// #region Header
// 2026 Ueli Saluz <ueli@semio-tech.de>
// Specs: MCP App for coda; uses @elements/ui and @modelcontextprotocol/ext-apps/react like semio engine pattern.
// Summary: Host iframe renders workspace payload from show_coda_workspace with panel tabs.
// #endregion Header

import { Card, CardGrid, i18next, initReactI18next } from "@elements/ui";
import "@elements/ui/globals.css";
import type { App as McpApp } from "@modelcontextprotocol/ext-apps";
import { useApp, useDocumentTheme } from "@modelcontextprotocol/ext-apps/react";
import React, { useMemo, useState } from "react";
import { createRoot } from "react-dom/client";

i18next.use(initReactI18next).init({
  lng: "en",
  fallbackLng: "en",
  interpolation: { escapeValue: false },
  resources: { en: { translation: {} } },
});

const PANELS = ["dashboard", "config", "runs", "report", "translations", "actions", "events"] as const;

function JsonBlock({ value }: { value: unknown }) {
  return (
    <pre className="text-xs overflow-auto max-h-64 rounded border border-border p-2 bg-panel font-mono whitespace-pre-wrap">
      {JSON.stringify(value, null, 2)}
    </pre>
  );
}

function WorkspaceBody({ payload, activePanel }: { payload: Record<string, unknown>; activePanel: string }) {
  const panel = activePanel || (payload.panel as string) || "dashboard";
  if (panel === "config") {
    return (
      <CardGrid>
        <Card title="Property kinds" icon="">
          <JsonBlock value={payload.property_kinds} />
        </Card>
        <Card title="Correlation" icon="">
          <JsonBlock value={payload.correlation} />
        </Card>
        <Card title="Properties" icon="">
          <JsonBlock value={payload.properties} />
        </Card>
        <Card title="Frameworks (targets)" icon="">
          <JsonBlock value={payload.frameworks} />
        </Card>
        <Card title="Platforms" icon="">
          <JsonBlock value={payload.platforms} />
        </Card>
      </CardGrid>
    );
  }
  if (panel === "runs") {
    return (
      <CardGrid>
        <Card title="Session" icon="">
          <JsonBlock value={payload.session} />
        </Card>
        <Card title="Run" icon="">
          <JsonBlock value={payload.run} />
        </Card>
        <Card title="Iteration" icon="">
          <JsonBlock value={payload.iteration} />
        </Card>
        <Card title="Translations index" icon="">
          <JsonBlock value={payload.translations} />
        </Card>
      </CardGrid>
    );
  }
  if (panel === "report") {
    return (
      <CardGrid>
        <Card title="Report summary" icon="">
          <JsonBlock value={payload.report} />
        </Card>
        <Card title="Breachs (shallow)" icon="">
          <JsonBlock value={payload.breachs_shallow} />
        </Card>
      </CardGrid>
    );
  }
  if (panel === "translations") {
    return (
      <Card title="Per-target translation files" icon="">
        <JsonBlock value={payload.translations} />
      </Card>
    );
  }
  if (panel === "actions") {
    return (
      <Card title="Session (tools: start_run, start_iteration, translate, validate, save_*)" icon="">
        <JsonBlock value={payload.session} />
      </Card>
    );
  }
  if (panel === "events") {
    return (
      <Card title="Use coda desktop Events page for live log; this panel shows static snapshot" icon="">
        <JsonBlock value={payload.session} />
      </Card>
    );
  }
  return (
    <CardGrid>
      <Card title="Project" icon="">
        <JsonBlock value={payload.project} />
      </Card>
      <Card title="Measures" icon="">
        <JsonBlock value={payload.measures} />
      </Card>
      <Card title="Run / iteration" icon="">
        <JsonBlock value={{ run: payload.run, iteration: payload.iteration }} />
      </Card>
      <Card title="Report summary" icon="">
        <JsonBlock value={payload.report} />
      </Card>
    </CardGrid>
  );
}

function McpShell() {
  useDocumentTheme();
  const app = useApp() as McpApp | null;
  const [panel, setPanel] = useState<string>(() => {
    const root = document.getElementById("root");
    return root?.getAttribute("data-coda-panel") ?? "dashboard";
  });

  const payload = useMemo(() => {
    const tool = app?.toolResponse;
    const sc = tool?.structuredContent as Record<string, unknown> | undefined;
    if (sc && sc.kind === "coda-workspace") return sc;
    return {
      kind: "coda-workspace",
      panel,
      error: "Open this app via show_coda_workspace tool or load a tool result in the host.",
    };
  }, [app?.toolResponse, panel]);

  return (
    <div className="min-h-full flex flex-col gap-3 p-3 bg-window text-foreground">
      <div className="flex flex-wrap gap-1 border-b border-border pb-2">
        {PANELS.map((p) => (
          <button
            key={p}
            type="button"
            className={`rounded px-2 py-1 text-xs capitalize ${panel === p ? "bg-primary text-primary-foreground" : "bg-panel hover:bg-muted"}`}
            onClick={() => setPanel(p)}
          >
            {p}
          </button>
        ))}
      </div>
      <WorkspaceBody payload={payload} activePanel={panel} />
    </div>
  );
}

const rootEl = document.getElementById("root");
if (!rootEl) throw new Error("Missing #root");
createRoot(rootEl).render(
  <React.StrictMode>
    <McpShell />
  </React.StrictMode>,
);
