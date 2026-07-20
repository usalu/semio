// #region Header
// 2026 Ueli Saluz <ueli@semio-tech.de>
// Specs: MCP App for coda; uses @semio-tech/ui-react and @modelcontextprotocol/ext-apps/react like semio engine pattern.
// Summary: Host iframe renders workspace payload from show_coda_workspace with panel tabs.
// #endregion Header

// #region 🔌Adapters
import { Card, CardGrid, reactHostPort, registerUiTranslationBundles, useLabel } from "@semio-tech/ui-react";
import "@semio-tech/ui-react/globals.css";
import type { App as McpApp } from "@modelcontextprotocol/ext-apps";
import { useApp, useDocumentTheme } from "@modelcontextprotocol/ext-apps/react";
import React from "react";
import { createRoot } from "react-dom/client";
// #endregion 🔌Adapters

const PANELS = ["dashboard", "config", "runs", "report", "translations", "actions", "events"] as const;

//#region 🪁McpAppI18n
type McpAppTranslationKey = `mcpApp.panel.${(typeof PANELS)[number]}` | `mcpApp.card.${string}`;

function mcpAppTranslationBundle(entries: Readonly<Record<string, string>>): Readonly<Record<McpAppTranslationKey, { readonly label: { readonly normal: string; readonly beginner: string } }>> {
  const resolved: Record<string, { readonly label: { readonly normal: string; readonly beginner: string } }> = {};
  for (const [key, value] of Object.entries(entries)) resolved[key] = { label: { normal: value, beginner: value } };
  return resolved as Readonly<Record<McpAppTranslationKey, { readonly label: { readonly normal: string; readonly beginner: string } }>>;
}

registerUiTranslationBundles({
  en: {
    translation: mcpAppTranslationBundle({
      "mcpApp.panel.dashboard": "Dashboard",
      "mcpApp.panel.config": "Config",
      "mcpApp.panel.runs": "Runs",
      "mcpApp.panel.report": "Report",
      "mcpApp.panel.translations": "Translations",
      "mcpApp.panel.actions": "Actions",
      "mcpApp.panel.events": "Events",
      "mcpApp.card.propertyKinds": "Property kinds",
      "mcpApp.card.correlation": "Correlation",
      "mcpApp.card.properties": "Properties",
      "mcpApp.card.frameworks": "Frameworks (targets)",
      "mcpApp.card.platforms": "Platforms",
      "mcpApp.card.session": "Session",
      "mcpApp.card.run": "Run",
      "mcpApp.card.iteration": "Iteration",
      "mcpApp.card.translationsIndex": "Translations index",
      "mcpApp.card.reportSummary": "Report summary",
      "mcpApp.card.breachsShallow": "Breachs (shallow)",
      "mcpApp.card.translationFiles": "Per-target translation files",
      "mcpApp.card.actionsSession": "Session (tools: start_run, start_iteration, translate, validate, save_*)",
      "mcpApp.card.eventsNotice": "Use coda desktop Events page for live log; this panel shows static snapshot",
      "mcpApp.card.project": "Project",
      "mcpApp.card.measures": "Measures",
      "mcpApp.card.runIteration": "Run / iteration",
    }),
  },
  de: {
    translation: mcpAppTranslationBundle({
      "mcpApp.panel.dashboard": "Uebersicht",
      "mcpApp.panel.config": "Konfiguration",
      "mcpApp.panel.runs": "Durchlaeufe",
      "mcpApp.panel.report": "Bericht",
      "mcpApp.panel.translations": "Uebersetzungen",
      "mcpApp.panel.actions": "Aktionen",
      "mcpApp.panel.events": "Ereignisse",
      "mcpApp.card.propertyKinds": "Eigenschaftsarten",
      "mcpApp.card.correlation": "Korrelation",
      "mcpApp.card.properties": "Eigenschaften",
      "mcpApp.card.frameworks": "Frameworks (Ziele)",
      "mcpApp.card.platforms": "Plattformen",
      "mcpApp.card.session": "Sitzung",
      "mcpApp.card.run": "Durchlauf",
      "mcpApp.card.iteration": "Iteration",
      "mcpApp.card.translationsIndex": "Übersetzungsindex",
      "mcpApp.card.reportSummary": "Berichtszusammenfassung",
      "mcpApp.card.breachsShallow": "Verstöße (oberflächlich)",
      "mcpApp.card.translationFiles": "Übersetzungsdateien pro Ziel",
      "mcpApp.card.actionsSession": "Sitzung (Werkzeuge: start_run, start_iteration, translate, validate, save_*)",
      "mcpApp.card.eventsNotice": "Für das Live-Protokoll die coda-Desktop-Ereignisseite verwenden; dieses Panel zeigt eine statische Momentaufnahme",
      "mcpApp.card.project": "Projekt",
      "mcpApp.card.measures": "Massnahmen",
      "mcpApp.card.runIteration": "Durchlauf / Iteration",
    }),
  },
});
//#endregion 🪁McpAppI18n

function JsonBlock({ value }: { value: unknown }) {
  return <pre className="text-xs overflow-auto max-h-64 rounded border border-border p-2 bg-panel font-mono whitespace-pre-wrap">{JSON.stringify(value, null, 2)}</pre>;
}

/** @emoji 🪁 Resolves every card label unconditionally so hook order stays stable across the panel-dependent early returns below. */
function useMcpAppCardLabels() {
  return {
    propertyKinds: useLabel("mcpApp.card.propertyKinds"),
    correlation: useLabel("mcpApp.card.correlation"),
    properties: useLabel("mcpApp.card.properties"),
    frameworks: useLabel("mcpApp.card.frameworks"),
    platforms: useLabel("mcpApp.card.platforms"),
    session: useLabel("mcpApp.card.session"),
    run: useLabel("mcpApp.card.run"),
    iteration: useLabel("mcpApp.card.iteration"),
    translationsIndex: useLabel("mcpApp.card.translationsIndex"),
    reportSummary: useLabel("mcpApp.card.reportSummary"),
    breachsShallow: useLabel("mcpApp.card.breachsShallow"),
    translationFiles: useLabel("mcpApp.card.translationFiles"),
    actionsSession: useLabel("mcpApp.card.actionsSession"),
    eventsNotice: useLabel("mcpApp.card.eventsNotice"),
    project: useLabel("mcpApp.card.project"),
    measures: useLabel("mcpApp.card.measures"),
    runIteration: useLabel("mcpApp.card.runIteration"),
  };
}

function WorkspaceBody({ payload, activePanel }: { payload: Record<string, unknown>; activePanel: string }) {
  const panel = activePanel || (payload.panel as string) || "dashboard";
  const l = useMcpAppCardLabels();
  if (panel === "config") {
    return (
      <CardGrid>
        <Card title={l.propertyKinds ?? ""} icon="">
          <JsonBlock value={payload.property_kinds} />
        </Card>
        <Card title={l.correlation ?? ""} icon="">
          <JsonBlock value={payload.correlation} />
        </Card>
        <Card title={l.properties ?? ""} icon="">
          <JsonBlock value={payload.properties} />
        </Card>
        <Card title={l.frameworks ?? ""} icon="">
          <JsonBlock value={payload.frameworks} />
        </Card>
        <Card title={l.platforms ?? ""} icon="">
          <JsonBlock value={payload.platforms} />
        </Card>
      </CardGrid>
    );
  }
  if (panel === "runs") {
    return (
      <CardGrid>
        <Card title={l.session ?? ""} icon="">
          <JsonBlock value={payload.session} />
        </Card>
        <Card title={l.run ?? ""} icon="">
          <JsonBlock value={payload.run} />
        </Card>
        <Card title={l.iteration ?? ""} icon="">
          <JsonBlock value={payload.iteration} />
        </Card>
        <Card title={l.translationsIndex ?? ""} icon="">
          <JsonBlock value={payload.translations} />
        </Card>
      </CardGrid>
    );
  }
  if (panel === "report") {
    return (
      <CardGrid>
        <Card title={l.reportSummary ?? ""} icon="">
          <JsonBlock value={payload.report} />
        </Card>
        <Card title={l.breachsShallow ?? ""} icon="">
          <JsonBlock value={payload.breachs_shallow} />
        </Card>
      </CardGrid>
    );
  }
  if (panel === "translations") {
    return (
      <Card title={l.translationFiles ?? ""} icon="">
        <JsonBlock value={payload.translations} />
      </Card>
    );
  }
  if (panel === "actions") {
    return (
      <Card title={l.actionsSession ?? ""} icon="">
        <JsonBlock value={payload.session} />
      </Card>
    );
  }
  if (panel === "events") {
    return (
      <Card title={l.eventsNotice ?? ""} icon="">
        <JsonBlock value={payload.session} />
      </Card>
    );
  }
  return (
    <CardGrid>
      <Card title={l.project ?? ""} icon="">
        <JsonBlock value={payload.project} />
      </Card>
      <Card title={l.measures ?? ""} icon="">
        <JsonBlock value={payload.measures} />
      </Card>
      <Card title={l.runIteration ?? ""} icon="">
        <JsonBlock value={{ run: payload.run, iteration: payload.iteration }} />
      </Card>
      <Card title={l.reportSummary ?? ""} icon="">
        <JsonBlock value={payload.report} />
      </Card>
    </CardGrid>
  );
}

function McpPanelButton({ panelId, active, onSelect }: { readonly panelId: (typeof PANELS)[number]; readonly active: boolean; readonly onSelect: () => void }) {
  const label = useLabel(`mcpApp.panel.${panelId}` as McpAppTranslationKey);
  return (
    <button type="button" className={`rounded px-2 py-1 text-xs capitalize ${active ? "bg-primary text-primary-foreground" : "bg-panel hover:bg-muted"}`} onClick={onSelect}>
      {label}
    </button>
  );
}

function McpShell() {
  useDocumentTheme();
  const app = useApp() as McpApp | null;
  const [panel, setPanel] = reactHostPort.useState<string>(() => {
    const root = document.getElementById("root");
    return root?.getAttribute("data-coda-panel") ?? "dashboard";
  });

  const payload = reactHostPort.useMemo(() => {
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
          <McpPanelButton key={p} panelId={p} active={panel === p} onSelect={() => setPanel(p)} />
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
