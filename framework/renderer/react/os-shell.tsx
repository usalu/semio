import { Component, createContext, useCallback, useContext, useEffect, useMemo, useRef, useState, useSyncExternalStore, type ReactElement, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import Fuse, { type FuseResult } from "fuse.js";
import type { GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";
import {
  App,
  Button,
  ButtonGroup,
  ButtonGroupItem,
  ChromeAwareWindowScrollSurface,
  COMPOSE_WINDOW_TEMPLATE_MIME,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  Footer,
  Icon,
  Input,
  Layout,
  LevelProvider,
  Mode,
  Navbar,
  NavbarExampleSelect,
  Popover,
  PopoverAnchor,
  PopoverContent,
  Ribbon,
  SemioLogo,
  Slider,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Toggle,
  ToggleGroup,
  ToolbarDivider,
  ToolbarGroup,
  ToolbarItem,
  ToolbarZone,
  findPanelTabNode,
  findPanelTabPath,
  panelTabChildren,
  reconcileActivePath,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf,
  WindowMeasuresTree,
  bootstrapElementsSurfaceChromeDocument,
  cn,
  createEvenWindowLayout,
  iconRenderPort,
  getLevelBgClass,
  insertWindowAtDropZone,
  interactiveActiveFillClass,
  shellChromeTitleClassName,
  staticTreePanelDefinition,
  UiChromeLabelPolicyProvider,
  UI_MOBILE_MEDIA_QUERY,
  useElementsSurfaceChrome,
  useMediaQuery,
  useSidePanelChromeHotkeys,
  useActionHotkey,
  readStoredUiChromeCompact,
  readStoredUiChromeExpertise,
  readStoredUiChromeLayout,
  readStoredUiChromeLocale,
  readStoredUiChromeAppearance,
  readStoredUiChromeTerminology,
  readStoredUiChromeThemeId,
  readStoredUiChromeThemeSnapshot,
  readStoredUiCustomThemes,
  writeStoredUiChromeCompact,
  writeStoredUiChromeExpertise,
  writeStoredUiChromeLayout,
  writeStoredUiChromeLocale,
  writeStoredUiChromeAppearance,
  writeStoredUiChromeTerminology,
  writeStoredUiChromeThemeId,
  writeStoredUiChromeThemeSnapshot,
  writeStoredUiCustomThemes,
  activeUiTheme,
  builtinUiThemes,
  parseUiTheme,
  resolveThemeAppearancePalettes,
  semioTheme,
  serializeUiTheme,
  setActiveUiTheme,
  type ThemeAppearanceName,
  type ThemePaletteGroup,
  type UiTheme,
  windowTemplatePaletteTreeDragController,
  Expertise,
  resolveTranslationLabel,
  setUiLocale,
  uiI18n,
  UI_TERMINOLOGY_NATIVE,
  type ElementsSurfaceAppearance,
  type ElementsSurfaceDevice,
  type EngagementControl,
  type EngagementSpec,
  type ModeWindowDescriptor,
  type NavbarItem,
  type PanelTabNode,
  type RibbonDirection,
  type RibbonRow,
  type TreeDataItem,
  type TreePanelConfig,
  type UiChromeLayout,
  type UiChromeTerminologyId,
  type UiLocale,
  type UiTranslationKey,
  type WindowLayoutNode,
  type ModeCanvasDropTarget,
  type WindowTemplateDropPayload,
} from "@semio-tech/ui-react";
import { ICONS, type IconName } from "@semio-tech/ui-asset";
import { declarativeTreeDragController, interpretUiNode, uiTreeNodeToTreePanelConfig } from "./ui-interpreter.tsx";
import {
  DEFAULT_PLUGIN_REGISTRY,
  NamedLayoutStore,
  createBrowserStoragePort,
  createNamedLayout,
  loadPluginModule as loadCorePluginModule,
  loadPluginWasm as loadCorePluginWasm,
  buildContributionsJson,
  expandPluginRegistry,
  resolveExternalSlots,
  resolveLayoutForMode,
  resolvePlaygroundDefaultAppId,
  resolvePluginRegistryId,
  type ActionDefinition,
  type NamedLayout,
  type PluginAppLabelsOverlay,
  type PluginRegistryEntry,
  type PluginWasmHandle as CorePluginWasmHandle,
  type WindowLayout,
} from "@semio-tech/framework-core";
import {
  FRAMEWORK_SYNC_CONTROLLER_ID,
  buildFileBackboneUri,
  buildFolderBackboneUri,
  buildFrameworkSyncTools,
  buildRemoteBackboneUri,
  documentFromEnvelopeJson,
  readBackboneEnvelope,
  wrapDocumentEnvelope,
  writeBackboneEnvelope,
  type FrameworkSyncToolLeaf,
} from "@semio-tech/framework-os-core";

//#region ShellTypes
type LoadedPluginState = {
  readonly handle: PluginWasmHandle;
  readonly manifest: PluginManifest;
};

type ActiveSession = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly app: AppDefinition;
  readonly viewState: ViewState;
};

type StudioProgramEntry = {
  readonly pluginId: string;
  readonly programId: string;
  readonly appId: string;
  readonly label: string;
  readonly document: readonly string[];
  readonly yields: string;
};

type SpawnedAppEntry = {
  readonly id: string;
  readonly pluginId: string;
  readonly instanceId: number;
  readonly appId: string;
  readonly label: string;
  readonly document: readonly string[];
};

type StudioPanelState = {
  readonly activePanelTab: string;
  readonly programs: readonly StudioProgramEntry[];
  readonly spawnedApps: readonly SpawnedAppEntry[];
  readonly activeSpawnedId?: string;
};

export type FrameworkOsBootOptions = {
  readonly rootId?: string;
  readonly plugin?: string;
  readonly plugins?: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
};

type SyncCardKind = "file" | "folder" | "remote";

function syncDocumentId(session: ActiveSession, panel: StudioPanelState | null, studioMode: boolean): string {
  if (studioMode && panel?.activeSpawnedId) {
    const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (spawned) return `${spawned.pluginId}-${spawned.instanceId}`;
  }
  return `${session.pluginId}-${session.instanceId}`;
}

const S_HOME_APP_ID = "home";
const S_HOME_CONTROLLER_ID = "s-home";
const S_PLAY_APP_ID = "studio";
const S_PLAY_CONTROLLER_ID = "s-play";
const S_PLAY_CATALOGUE_TAB_ID = "s-play-catalogue";
const DEFAULT_LEFT_PANEL_SIZE = 280;
const DEFAULT_RIGHT_PANEL_SIZE = 320;

/** @emoji 🌳 Root category ids for the nested side-panel tab tree — the top row of {@link leftPanelTabs}/{@link rightPanelTabs}. */
const FRAMEWORK_CATEGORY_WORKBENCH_ID = "framework.category.workbench";
const FRAMEWORK_CATEGORY_DISPLAY_ID = "framework.category.display";
const FRAMEWORK_CATEGORY_DETAILS_ID = "framework.category.details";
const FRAMEWORK_CATEGORY_SETTINGS_ID = "framework.category.settings";
const APP_DOCUMENT_SEPARATOR = " · ";

const PRESENCE_CLIENT_STORAGE_KEY = "semio.presence.client";
const PRESENCE_HEARTBEAT_INTERVAL_MS = 5000;

function presenceClientIdentity(): { readonly clientId: string; readonly name: string } {
  if (typeof window === "undefined") return { clientId: "server", name: "Server" };
  const stored = window.sessionStorage.getItem(PRESENCE_CLIENT_STORAGE_KEY);
  if (stored) {
    try {
      const parsed = JSON.parse(stored) as { readonly clientId?: string; readonly name?: string };
      if (parsed.clientId && parsed.name) return { clientId: parsed.clientId, name: parsed.name };
    } catch {
      /* reseed identity */
    }
  }
  const clientId = `client-${Math.random().toString(36).slice(2, 10)}`;
  const identity = { clientId, name: `Guest ${clientId.slice(-4).toUpperCase()}` };
  window.sessionStorage.setItem(PRESENCE_CLIENT_STORAGE_KEY, JSON.stringify(identity));
  return identity;
}

type UIHistoryEntry = { readonly uri: string };
type UIHistory = { readonly entries: readonly UIHistoryEntry[]; readonly index: number };

function readBrowserUri(): string {
  if (typeof window === "undefined") return "/";
  return `${window.location.pathname}${window.location.search}` || "/";
}

function useUIHistory(initialUri = "/", syncBrowser = false) {
  const [history, setHistory] = useState<UIHistory>(() => ({
    entries: [{ uri: syncBrowser ? readBrowserUri() : initialUri }],
    index: 0,
  }));
  const uri = history.entries[history.index]?.uri ?? initialUri;
  const canGoBack = history.index > 0;
  const canGoForward = history.index < history.entries.length - 1;
  const segments = uri.split("/").filter(Boolean);
  const canGoUp = segments.length > 0;
  const parentUri = canGoUp ? `/${segments.slice(0, -1).join("/")}` : null;

  const goBack = useCallback(() => {
    setHistory((prev) => (prev.index > 0 ? { ...prev, index: prev.index - 1 } : prev));
  }, []);
  const goForward = useCallback(() => {
    setHistory((prev) => (prev.index < prev.entries.length - 1 ? { ...prev, index: prev.index + 1 } : prev));
  }, []);
  const goUp = useCallback(() => {
    if (!canGoUp || parentUri === null) return;
    setHistory((prev) => {
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: parentUri }], index: newEntries.length };
    });
  }, [canGoUp, parentUri]);
  const navigate = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);
  const syncUri = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const current = `${window.location.pathname}${window.location.search}`;
    if (current !== uri) window.history.pushState(null, "", uri);
  }, [syncBrowser, uri]);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const onPopState = () => syncUri(readBrowserUri());
    window.addEventListener("popstate", onPopState);
    return () => window.removeEventListener("popstate", onPopState);
  }, [syncBrowser, syncUri]);

  return { uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate, syncUri };
}

function downloadMediaExport(filename: string, mimeType: string, data: string, encoding?: string): void {
  if (typeof document === "undefined") return;
  const payload = encoding === "base64" ? Uint8Array.from(atob(data), (char) => char.charCodeAt(0)) : data;
  const blob = new Blob([payload], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

function downloadDataUrl(filename: string, dataUrl: string): void {
  if (typeof document === "undefined") return;
  const anchor = document.createElement("a");
  anchor.href = dataUrl;
  anchor.download = filename;
  anchor.click();
}

function requestFileOpen(accept: string, readAs?: string): Promise<{ contents: string; name: string } | null> {
  if (typeof document === "undefined") return Promise.resolve(null);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    input.onchange = async () => {
      const file = input.files?.[0];
      if (!file) {
        resolve(null);
        return;
      }
      if (readAs === "dataUrl") {
        const reader = new FileReader();
        reader.onload = () => resolve(typeof reader.result === "string" ? { contents: reader.result, name: file.name } : null);
        reader.onerror = () => resolve(null);
        reader.readAsDataURL(file);
        return;
      }
      resolve({ contents: await file.text(), name: file.name });
    };
    input.click();
  });
}
//#endregion ShellTypes

//#region ShellHelpers
function isStudioMode(pluginFilter?: string): boolean {
  return pluginFilter === "s";
}

export interface StudioShellPath {
  readonly studioId: string;
  readonly instanceId?: string;
}

/** @emoji 🧭 Parses `/studios/:id` and `/studios/:id/instances/:iid` shell paths; null for any other route (e.g. home). */
export function parseStudioShellPath(path: string): StudioShellPath | null {
  const match = /^\/studios\/([^/]+)(?:\/instances\/([^/]+))?$/.exec(path);
  if (!match) return null;
  return { studioId: match[1]!, instanceId: match[2] };
}

function buildStudioPrograms(loaded: readonly LoadedPluginState[]): readonly StudioProgramEntry[] {
  return loaded.flatMap((entry) =>
    entry.manifest.programs.map((program) => ({
      pluginId: entry.handle.pluginId,
      programId: program.programId,
      appId: program.appId,
      label: program.label,
      document: program.document,
      yields: program.yields,
    })),
  );
}

export function appDocumentLabel(document: readonly string[]): string {
  return document.join(APP_DOCUMENT_SEPARATOR);
}

export function appWindowDocumentLabel(app: AppDefinition, windowLabel: string): string {
  return windowLabel.trim() || app.label.trim();
}

function buildStudioPanelState(programs: readonly StudioProgramEntry[], spawnedApps: readonly SpawnedAppEntry[], activePanelTab = "s-play-catalogue", activeSpawnedId?: string): StudioPanelState {
  return { activePanelTab, programs, spawnedApps, activeSpawnedId };
}

function panelJsonFromState(state: StudioPanelState): string {
  return JSON.stringify(state);
}

function parsePanelState(viewState: ViewState): StudioPanelState | null {
  if (!viewState.panelJson) return null;
  try {
    return JSON.parse(viewState.panelJson) as StudioPanelState;
  } catch {
    return null;
  }
}

function panelSideForGroup(group: string): "left" | "right" {
  if (group === "workbench" || group === "document" || group === "display") return "left";
  return "right";
}

function convertFrameworkLayoutNodeToModeLayout(
  node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode,
  appLabelsOverlay: PluginAppLabelsOverlay,
): WindowLayoutNode {
  if (node.kind === "window") {
    return { kind: "window", id: node.windowKindId, title: resolveAppLabel(appLabelsOverlay, "windowKind", node.windowKindId, node.title ?? node.windowKindId) };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => ({
        kind: "window" as const,
        id: child.windowKindId,
        title: resolveAppLabel(appLabelsOverlay, "windowKind", child.windowKindId, child.title ?? child.windowKindId),
      })),
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToModeLayout(child, appLabelsOverlay)),
  };
}

/** @emoji 🗣️ Re-resolves every window's title from the current app-labels overlay in place, preserving the tree's structure/sizes/arrangement — used to react to a locale/terminology switch without discarding the user's live layout. */
function retitleWindowLayoutNode(node: WindowLayoutNode, appLabelsOverlay: PluginAppLabelsOverlay): WindowLayoutNode {
  if (node.kind === "window") {
    return { ...node, title: resolveAppLabel(appLabelsOverlay, "windowKind", node.id, node.title ?? node.id) };
  }
  return { ...node, children: node.children.map((child) => retitleWindowLayoutNode(child, appLabelsOverlay)) } as WindowLayoutNode;
}

function convertFrameworkLayoutToModeLayout(
  layout: WindowLayout | undefined,
  windowIds: readonly string[],
  appLabelsOverlay: PluginAppLabelsOverlay,
): WindowLayoutNode {
  if (!layout?.root) return createEvenWindowLayout(windowIds.length ? windowIds : ["main"]);
  return convertFrameworkLayoutNodeToModeLayout(layout.root, appLabelsOverlay);
}

function modeLayoutNodeToFramework(node: WindowLayoutNode): WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode {
  if (node.kind === "window") {
    return { kind: "window", windowKindId: node.id, ...(node.title ? { title: node.title } : {}) };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      ...(node.size !== undefined ? { size: node.size } : {}),
      children: node.children.map((child) => ({
        kind: "window" as const,
        windowKindId: child.id,
        ...(child.title ? { title: child.title } : {}),
      })),
    };
  }
  return {
    kind: node.kind,
    ...(node.size !== undefined ? { size: node.size } : {}),
    children: node.children.map((child) => modeLayoutNodeToFramework(child) as WindowLayoutStackNode | WindowLayoutAxisNode),
  };
}

function captureCurrentFrameworkLayout(shellLayout: WindowLayoutNode | null, fallback?: WindowLayout): WindowLayout | undefined {
  if (!shellLayout) return fallback;
  const root = modeLayoutNodeToFramework(shellLayout);
  if (root.kind === "window") return { root: { kind: "stack", children: [root] } };
  return { root };
}

function findDefaultActiveWindowKindId(layout: WindowLayout | undefined, windowKinds: readonly { readonly id: string }[]): string | null {
  const collectWindowIds = (node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode): string[] => {
    if (node.kind === "window") return [node.windowKindId];
    if (node.kind === "stack") return node.children.map((child) => child.windowKindId);
    return node.children.flatMap((child) => collectWindowIds(child));
  };
  const ordered = layout?.root ? collectWindowIds(layout.root) : windowKinds.map((kind) => kind.id);
  for (const id of ordered) {
    if (windowKinds.some((kind) => kind.id === id)) return id;
  }
  return windowKinds[0]?.id ?? null;
}

function windowEngagementControlToSpec(control: WindowEngagementControl | undefined, onAction: (action: ActionDescriptor) => void): EngagementControl | undefined {
  if (!control) return undefined;
  if (control.kind === "ring" || control.kind === "toggleGroup") {
    return {
      kind: control.kind,
      id: control.id,
      label: control.label,
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? (id: string) => onAction({ ...control.onSelect!, args: { ...(control.onSelect!.args as object | undefined), id } }) : undefined,
    };
  }
  if (control.kind === "select") {
    return {
      kind: "select",
      id: control.id,
      label: control.label,
      value: control.value,
      placeholder: control.placeholder,
      disabled: control.disabled,
      items: control.items.map((row) => ({ id: row.id, value: row.value, label: row.label })),
      onChange: control.onChange ? (value: string) => onAction({ ...control.onChange!, args: { ...(control.onChange!.args as object | undefined), value } }) : undefined,
    };
  }
  const dispatchNumeric = (action: ActionDescriptor | undefined, value: number) => {
    if (!action) return;
    onAction({ ...action, args: { ...(action.args as object | undefined), value } });
  };
  return {
    kind: control.kind,
    id: control.id,
    label: control.label,
    value: control.value,
    min: control.min,
    max: control.max,
    step: control.step,
    unit: control.unit,
    disabled: control.disabled,
    onChange: control.onChange ? (value: number) => dispatchNumeric(control.onChange, value) : undefined,
    onCommit: control.onCommit ? (value: number) => dispatchNumeric(control.onCommit, value) : undefined,
  };
}

const PLUGIN_LOAD_TIMEOUT_MS = 30_000;

async function loadPluginModuleResilient(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle | null> {
  try {
    return await Promise.race([
      loadPluginModule(pluginId, moduleUrl),
      new Promise<never>((_, reject) => {
        window.setTimeout(() => reject(new Error(`timeout loading ${pluginId}`)), PLUGIN_LOAD_TIMEOUT_MS);
      }),
    ]);
  } catch (error) {
    console.error("[DEBUG] plugin load failed", pluginId, error);
    return null;
  }
}

function isViewportSurface(surfaceKind: string | undefined): boolean {
  return surfaceKind === "world-3d" || surfaceKind === "node-graph" || surfaceKind === "canvas-2d";
}

function defaultViewportEngagement(): WindowEngagement {
  return {
    sessionActive: true,
    status: [{ id: "framework.viewport.status", text: shellLabel("ui.engagement.viewport") }],
  };
}

function resolveWindowEngagement(kind: AppDefinition["windowKinds"][number], byKind: Readonly<Record<string, WindowEngagement>>): WindowEngagement | undefined {
  const surfaceKind = (kind as { surfaceKind?: string }).surfaceKind;
  return byKind[kind.id] ?? kind.engagement ?? (isViewportSurface(surfaceKind) ? defaultViewportEngagement() : undefined);
}

function windowEngagementToSpec(engagement: WindowEngagement | undefined, onAction: (action: ActionDescriptor) => void): EngagementSpec | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    icon: option.iconId ? <Icon icon={option.iconId in ICONS ? (option.iconId as IconName) : "circle-dot"} size="small" /> : undefined,
    pressed: option.pressed,
    disabled: option.disabled,
    onPress: option.action ? () => onAction(option.action!) : undefined,
  }));
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder,
        disabled: engagement.input.disabled,
        onChange: engagement.input.onChange ? (value: string) => onAction({ ...engagement.input!.onChange!, args: { ...(engagement.input!.onChange!.args as object | undefined), value } }) : undefined,
        onSubmit: engagement.input.onSubmit ? (value: string) => onAction({ ...engagement.input!.onSubmit!, args: { ...(engagement.input!.onSubmit!.args as object | undefined), value } }) : undefined,
        onRepeatLast: engagement.input.onRepeatLast ? () => onAction(engagement.input!.onRepeatLast!) : undefined,
        onAbort: engagement.input.onAbort ? () => onAction(engagement.input!.onAbort!) : undefined,
      }
    : undefined;
  const status = engagement.status?.map((row) => ({ id: row.id, content: row.text }));
  const possibleEngagements = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    onSelect: row.action ? () => onAction(row.action!) : undefined,
  }));
  const control = windowEngagementControlToSpec(engagement.control, onAction);
  const controls = engagement.controls?.map((row) => windowEngagementControlToSpec(row, onAction)).filter((row): row is EngagementControl => row !== undefined);
  const hasContent = (options?.length ?? 0) > 0 || Boolean(input) || Boolean(control) || (controls?.length ?? 0) > 0 || (status?.length ?? 0) > 0 || (possibleEngagements?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, options, input, control, controls, status, possibleEngagements };
}

function panelTabIcon(tabId: string, group: string): React.FC<{ size?: number }> {
  if (tabId === S_PLAY_CATALOGUE_TAB_ID || group === "workbench") return shellTabIcon(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID);
  if (tabId.includes("parameters")) return shellTabIcon(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID);
  if (tabId.includes("inspector")) return shellTabIcon(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID);
  return shellTabIcon(tabId);
}

/** @emoji 🌳 Category-row icon: the first child's icon, or `fallback` when the category has no tabs yet. */
function categoryTabIcon(tabs: readonly PanelTabNode[], fallback: IconName): React.FC<{ size?: number }> {
  const FirstIcon = tabs[0]?.icon;
  return function CategoryTabIcon({ size = 16 }: { size?: number }) {
    return FirstIcon ? <FirstIcon size={size} /> : <Icon icon={fallback} size="small" />;
  };
}

/** @emoji 🌳 Depth-first leaves of a recursive panel-tab tree — the nodes that actually carry a `bodyKey` to render. */
export function flattenPanelTabLeaves<T extends { readonly children?: readonly T[] }>(tabs: readonly T[]): T[] {
  return tabs.flatMap((tab) => (tab.children && tab.children.length > 0 ? flattenPanelTabLeaves(tab.children) : [tab]));
}

/** @emoji 🌳 Converts one plugin-declared {@link AppPanelTabDefinition} (recursively) into a {@link PanelTabNode}. */
function panelTabDefinitionToNode(
  tab: AppPanelTabDefinition,
  group: string,
  panelUiByKey: Readonly<Record<string, UiNode>>,
  onAction: (action: ActionDescriptor) => void,
  order: number,
  appLabelsOverlay: PluginAppLabelsOverlay,
): PanelTabNode {
  const label = resolveAppLabel(appLabelsOverlay, "panelTab", tab.id, tab.label);
  if (tab.children && tab.children.length > 0) {
    return {
      kind: "branch",
      id: tab.id,
      icon: panelTabIcon(tab.id, group),
      name: label,
      order,
      children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelUiByKey, onAction, childOrder, appLabelsOverlay)),
    };
  }
  return {
    kind: "leaf",
    id: tab.id,
    icon: panelTabIcon(tab.id, group),
    name: label,
    order,
    tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tab.id] ?? { type: "text", value: shellLabel("ui.common.loading") }, onAction)),
  };
}

function resolveCanvasBodyKey(app: AppDefinition): string {
  const windowKind = app.windowKinds[0];
  if (!windowKind) return "main";
  if (windowKind.bodyKey.includes("composite")) {
    const mediaGraph = app.windowKinds.find((kind) => kind.bodyKey.includes("media-graph"));
    return mediaGraph?.bodyKey ?? windowKind.bodyKey;
  }
  return windowKind.bodyKey;
}

/** @emoji 🎛 Picks dynamic plugin tools when present, otherwise static mode tools for a spawned app. */
export function selectSpawnedToolNodes(dynamicTools: readonly ToolNode[], app: Pick<AppDefinition, "modes" | "defaultModeId">, activeModeId?: string): readonly ToolNode[] {
  const modeId = activeModeId ?? app.defaultModeId ?? app.modes[0]?.id;
  const staticTools = app.modes.find((mode) => mode.id === modeId)?.tools ?? [];
  return dynamicTools.length > 0 ? dynamicTools : staticTools;
}

/** @emoji 💬 Builds spawned-window engagement and measures chrome for one window kind. */
export function spawnedWindowChromeForKind(
  kind: AppDefinition["windowKinds"][number],
  engagementsByKind: Readonly<Record<string, WindowEngagement>>,
  measuresByKind: Readonly<Record<string, readonly WindowMeasure[]>>,
  onAction: (action: ActionDescriptor) => void,
): { readonly engagement?: EngagementSpec; readonly measures: ReactNode } {
  return {
    engagement: windowEngagementToSpec(resolveWindowEngagement(kind, engagementsByKind), onAction),
    measures: windowMeasuresOverlay(measuresByKind[kind.id] ?? kind.measures, onAction),
  };
}

function isTreeNode(node: UiNode): node is UiTreeNode {
  return node.type === "tree";
}

export function uiNodeToTreePanelConfig(node: UiNode, onAction: (action: ActionDescriptor) => void): TreePanelConfig {
  if (isTreeNode(node)) return { ...uiTreeNodeToTreePanelConfig(node, onAction), dragAndDropController: declarativeTreeDragController(node, onAction) };
  return {
    sections: [
      {
        id: "panel.body",
        label: "",
        items: [
          {
            id: "panel.body.content",
            label: "",
            control: <ChromeAwareWindowScrollSurface className="min-h-0 flex-1">{interpretUiNode(node, { onAction })}</ChromeAwareWindowScrollSurface>,
          },
        ],
      },
    ],
  };
}

function shellTabIcon(iconId: string): React.FC<{ size?: number }> {
  return function ShellTabIcon({ size = 16 }: { size?: number }) {
    let iconName: IconName = "circle-dot";
    if (iconId === FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID) {
      iconName = "file-text";
    } else if (iconId in ICONS) {
      iconName = iconId as IconName;
    }
    return <Icon icon={iconName} size={size} />;
  };
}

/** @emoji 🌐 Resolves a chrome translation key outside hook context (tree builders run there). */
function shellLabel(key: UiTranslationKey): string {
  return resolveTranslationLabel(uiI18n.t(key)) ?? key;
}

/** @emoji 🗣️ Stable empty overlay reference so components depending on it don't re-render before the first `appLabels` fetch resolves. */
const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {} };

/** @emoji 🗣️ Resolves a window-kind/panel-tab/mode id's locale-aware label from the active app's overlay, falling back to the static manifest label. */
function resolveAppLabel(overlay: PluginAppLabelsOverlay, kind: "windowKind" | "panelTab" | "mode", id: string, fallback: string): string {
  const map = kind === "windowKind" ? overlay.windowKindLabels : kind === "panelTab" ? overlay.panelTabLabels : overlay.modeLabels;
  return map[id] ?? fallback;
}

/** @emoji 🗣️ Resolves a terminology id's display name; chrome-known ids get a translated label, app-declared ids fall back to their raw id. */
function shellTerminologyLabel(id: string): string {
  const isChromeKnown = id === "native" || id === "reuse";
  return isChromeKnown ? shellLabel(`ui.settings.terminology.${id as UiChromeTerminologyId}`) : id;
}

function renderWindowMeasure(measure: WindowMeasure, onAction: (action: ActionDescriptor) => void): ReactNode {
  if (measure.kind === "group") {
    return (
      <WindowMeasureTreeGroup key={measure.id} id={measure.id} label={measure.label} defaultOpen={measure.defaultOpen}>
        {measure.children.map((child) => renderWindowMeasure(child, onAction))}
      </WindowMeasureTreeGroup>
    );
  }
  if (measure.kind === "select") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <Select value={measure.value} onValueChange={(value) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } })}>
          <SelectTrigger id={measure.id} className="h-small w-full min-w-0" size="sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {measure.items.map((item) => (
              <SelectItem key={item.id} value={item.value}>
                {item.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "slider") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <Slider
          id={measure.id}
          value={[measure.value]}
          min={measure.min}
          max={measure.max}
          step={measure.step}
          onValueChange={(values) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value: values[0] ?? measure.value } })}
        />
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "toggle") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <Toggle
          id={measure.id}
          pressed={measure.pressed}
          text={measure.text}
          icon={<Icon icon={measure.iconId in ICONS ? (measure.iconId as IconName) : "circle-dot"} size="small" />}
          onPressedChange={(pressed) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), pressed } })}
        />
      </WindowMeasureTreeLeaf>
    );
  }
  return null;
}

function windowMeasuresOverlay(measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => void): ReactNode | undefined {
  if (!measures || measures.length === 0) return undefined;
  return <WindowMeasuresTree>{measures.map((measure) => renderWindowMeasure(measure, onAction))}</WindowMeasuresTree>;
}

function windowToolbarNode(tools: readonly ToolNode[] | undefined, windowId: string, onAction: (action: ActionDescriptor) => void): ReactNode {
  if (!tools?.length) return undefined;
  const grouped = groupToolNodesByCategory(tools, WINDOW_TOOL_CATEGORIES);
  if (!grouped.length) return undefined;
  return <ToolTree id={`ui.toolbar.${windowId}`} tools={grouped} onAction={onAction} direction="up" />;
}
//#endregion ShellHelpers

//#region Boot
export async function bootFrameworkOs(options: FrameworkOsBootOptions = {}): Promise<void> {
  const root = document.getElementById(options.rootId ?? "root");
  if (!root) throw new Error("missing #root");
  bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance());
  createRoot(root).render(<FrameworkOsShell pluginFilter={options.plugin} plugins={options.plugins ?? DEFAULT_PLUGIN_REGISTRY} />);
}
//#endregion Boot

//#region ErrorBoundary
class ShellRenderErrorBoundary extends Component<{ readonly children: ReactNode }, { readonly hasError: boolean; readonly message: string }> {
  constructor(props: { readonly children: ReactNode }) {
    super(props);
    this.state = { hasError: false, message: "" };
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, message: error.message };
  }

  render() {
    if (this.state.hasError) {
      return (
        <p className="p-4 text-sm text-destructive" role="alert">
          {shellLabel("ui.common.renderError")}: {this.state.message}
        </p>
      );
    }
    return this.props.children;
  }
}
//#endregion ErrorBoundary

//#region FrameworkOsShell
export function FrameworkOsShell({ pluginFilter, plugins }: { readonly pluginFilter?: string; readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[] }) {
  const studioMode = isStudioMode(pluginFilter);
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const [loadedPlugins, setLoadedPlugins] = useState<readonly LoadedPluginState[]>([]);
  const [session, setSession] = useState<ActiveSession | null>(null);
  const [windowUiByKind, setWindowUiByKind] = useState<Readonly<Record<string, UiNode>>>({});
  const [windowEngagementsByKind, setWindowEngagementsByKind] = useState<Readonly<Record<string, WindowEngagement>>>({});
  const [windowMeasuresByKind, setWindowMeasuresByKind] = useState<Readonly<Record<string, readonly WindowMeasure[]>>>({});
  const [panelUiByKey, setPanelUiByKey] = useState<Readonly<Record<string, UiNode>>>({});
  const [toolNodesByKind, setToolNodesByKind] = useState<Readonly<Record<string, readonly ToolNode[]>>>({});
  const [appLabelsOverlay, setAppLabelsOverlay] = useState<PluginAppLabelsOverlay>(EMPTY_APP_LABELS_OVERLAY);
  const [spawnedWindowUi, setSpawnedWindowUi] = useState<UiNode | null>(null);
  const [spawnedWindowEngagements, setSpawnedWindowEngagements] = useState<Readonly<Record<string, WindowEngagement>>>({});
  const [spawnedWindowMeasures, setSpawnedWindowMeasures] = useState<Readonly<Record<string, readonly WindowMeasure[]>>>({});
  const [spawnedToolNodes, setSpawnedToolNodes] = useState<readonly ToolNode[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [leftPanelVisible, setLeftPanelVisible] = useState(false);
  const [rightPanelVisible, setRightPanelVisible] = useState(false);
  const [leftPanelSize, setLeftPanelSize] = useState(DEFAULT_LEFT_PANEL_SIZE);
  const [rightPanelSize, setRightPanelSize] = useState(DEFAULT_RIGHT_PANEL_SIZE);
  const [activeWindowId, setActiveWindowId] = useState<string | null>(null);
  const [shellLayout, setShellLayout] = useState<WindowLayoutNode | null>(null);
  const [activeExampleId, setActiveExampleId] = useState("");
  const [searchOpen, setSearchOpen] = useState(false);
  const [findOpen, setFindOpen] = useState(false);
  const importStudioInputRef = useRef<HTMLInputElement>(null);
  const refreshGenerationRef = useRef(0);
  const spawnedRefreshGenerationRef = useRef(0);
  const contributorInstancesRef = useRef<Map<string, number>>(new Map());
  const layoutSeedKeyRef = useRef<string | null>(null);
  const noExampleResetInstanceIdRef = useRef<number | null>(null);
  const [mobilePanelPath, setMobilePanelPath] = useState<readonly string[]>([]);
  const [leftPanelPath, setLeftPanelPath] = useState<readonly string[]>([]);
  const [rightPanelPath, setRightPanelPath] = useState<readonly string[]>([]);
  const [extraWindowInstances, setExtraWindowInstances] = useState<readonly { readonly id: string; readonly windowKindId: string; readonly title: string }[]>([]);
  const extraWindowCounterRef = useRef(0);
  const openStudioIdRef = useRef<string | null>(null);
  const openInstanceIdRef = useRef<string | null>(null);
  const sessionRef = useRef<ActiveSession | null>(null);
  const [uiAppearance, setUiAppearance] = useState<ElementsSurfaceAppearance>(() => readStoredUiChromeAppearance());
  const [uiLayout, setUiLayout] = useState<UiChromeLayout>(() => readStoredUiChromeLayout());
  const uiDevice: ElementsSurfaceDevice = mobile ? "mobile" : uiLayout;
  const [uiCompact, setUiCompact] = useState(() => readStoredUiChromeCompact());
  const [uiExpertise, setUiExpertise] = useState(() => readStoredUiChromeExpertise());
  const [uiLocale, setUiLocaleState] = useState<UiLocale>(() => readStoredUiChromeLocale() ?? (uiI18n.resolvedLanguage?.toLowerCase().startsWith("de") ? "de" : "en"));
  const [uiTerminology, setUiTerminologyState] = useState<string>(() => readStoredUiChromeTerminology());
  const [uiThemeId, setUiThemeIdState] = useState<string>(() => readStoredUiChromeThemeId() ?? "semio");
  const [uiCustomThemes, setUiCustomThemesState] = useState<Record<string, UiTheme>>(() => readStoredUiCustomThemes());
  const [uiThemeDraft, setUiThemeDraft] = useState<UiTheme | null>(null);
  const uiTheme: UiTheme = useMemo(() => {
    if (uiThemeDraft) return uiThemeDraft;
    const found = builtinUiThemes().find((t) => t.id === uiThemeId) ?? uiCustomThemes[uiThemeId];
    return found ?? readStoredUiChromeThemeSnapshot() ?? semioTheme();
  }, [uiThemeId, uiCustomThemes, uiThemeDraft]);
  const [syncBackboneUri, setSyncBackboneUri] = useState<string | null>(null);
  const [syncCardKind, setSyncCardKind] = useState<SyncCardKind | null>(null);
  const [syncDraftPath, setSyncDraftPath] = useState("");
  const lastEnvelopeJsonRef = useRef<string | null>(null);
  const { uri: shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate: navigateHistory } = useUIHistory("/", studioMode);

  const namedLayoutStore = useMemo(() => new NamedLayoutStore(session?.app.id ?? "framework-os", createBrowserStoragePort()), [session?.app.id]);

  const registry = useMemo(() => {
    const expanded = expandPluginRegistry(plugins, pluginFilter || undefined, studioMode);
    if (studioMode) return expanded;
    return pluginFilter ? expanded : plugins;
  }, [pluginFilter, plugins, studioMode]);

  const panel = session ? parsePanelState(session.viewState) : null;
  const activeAppTitle = appDocumentLabel(panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId)?.document ?? session?.app.document ?? []);

  useEffect(() => {
    sessionRef.current = session;
  }, [session]);

  useEffect(() => {
    setSyncBackboneUri(null);
    setSyncCardKind(null);
    lastEnvelopeJsonRef.current = null;
  }, [panel?.activeSpawnedId, session, studioMode]);

  useEffect(() => {
    if (activeAppTitle) document.title = activeAppTitle;
  }, [activeAppTitle]);

  useEffect(() => {
    let cancelled = false;
    void (async () => {
      try {
        const settled = await Promise.allSettled(registry.map((entry) => loadPluginModuleResilient(entry.pluginId, entry.moduleUrl)));
        const loaded = settled.flatMap((result, index) => {
          if (result.status === "fulfilled" && result.value) return [result.value];
          if (result.status === "rejected") {
            console.error(`[DEBUG] plugin rejected: ${registry[index]?.pluginId}`, result.reason);
          }
          return [];
        });
        if (loaded.length === 0) throw new Error(shellLabel("ui.common.noPluginsLoaded"));
        if (cancelled) return;
        const loadedState = loaded.map((handle) => ({ handle, manifest: handle.manifest }));
        setLoadedPlugins(loadedState);

        if (studioMode) {
          const sPlugin = loadedState.find((entry) => entry.handle.pluginId === "s");
          const sApp = sPlugin?.manifest.apps.find((app) => app.id === S_HOME_APP_ID) ?? sPlugin?.manifest.apps[0];
          if (!sPlugin || !sApp) throw new Error("s studio plugin missing home app");
          const programs = buildStudioPrograms(loadedState);
          const panelState = buildStudioPanelState(programs, []);
          const instanceId = await sPlugin.handle.createApp(sApp.id);
          const viewState: ViewState = {
            activeModeId: sApp.defaultModeId ?? sApp.modes[0]?.id,
            activeWindowKindId: sApp.windowKinds[0]?.id,
            panelJson: panelJsonFromState(panelState),
          };
          setSession({ pluginId: sPlugin.handle.pluginId, instanceId, app: sApp, viewState });
          setActiveWindowId(sApp.windowKinds[0]?.id ?? null);
          return;
        }

        const registryPluginId = pluginFilter ? resolvePluginRegistryId(pluginFilter) : undefined;
        const primary = (registryPluginId ? loaded.find((entry) => entry.pluginId === registryPluginId) : undefined) ?? loaded[0];
        const defaultAppId = pluginFilter ? resolvePlaygroundDefaultAppId(pluginFilter) : undefined;
        const primaryApp = (defaultAppId ? primary?.manifest.apps.find((app) => app.id === defaultAppId) : undefined) ?? primary?.manifest.apps[0];
        if (primary && primaryApp) {
          const instanceId = await primary.createApp(primaryApp.id);
          setSession({
            pluginId: primary.pluginId,
            instanceId,
            app: primaryApp,
            viewState: {
              activeModeId: primaryApp.defaultModeId ?? primaryApp.modes[0]?.id,
              activeWindowKindId: primaryApp.windowKinds[0]?.id,
            },
          });
          setActiveWindowId(primaryApp.windowKinds[0]?.id ?? null);
        }
      } catch (bootError) {
        if (!cancelled) {
          console.error("[DEBUG] framework os boot failed", bootError);
          setError(bootError instanceof Error ? bootError.message : String(bootError));
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [registry, studioMode]);

  const findPluginForAction = useCallback(
    (action: ActionDescriptor) => {
      const byController = loadedPlugins.find((entry) => entry.manifest.apps.some((app) => app.controllerId === action.controllerId));
      if (byController) return byController;
      return loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId);
    },
    [loadedPlugins, session?.pluginId],
  );

  const refreshUi = useCallback(
    async (nextSession: ActiveSession) => {
      const generation = ++refreshGenerationRef.current;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId)?.handle;
      if (!plugin) return;
      const contributionsJson = buildContributionsJson(loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })));
      const viewState: ViewState = { ...nextSession.viewState, contributionsJson, locale: uiLocale, terminology: uiTerminology };
      const slotContext = {
        plugins: new Map(loadedPlugins.map((entry) => [entry.handle.pluginId, entry.handle])),
        contributorInstances: contributorInstancesRef.current,
        viewState,
      };
      const windowCount = nextSession.app.windowKinds.length;
      const panelTabLeaves = flattenPanelTabLeaves(nextSession.app.panelTabs);
      const rendered: unknown[] = [];
      for (const kind of nextSession.app.windowKinds) {
        rendered.push(await plugin.render(nextSession.instanceId, kind.bodyKey, viewState));
      }
      for (const tab of panelTabLeaves) {
        rendered.push(await plugin.render(nextSession.instanceId, tab.bodyKey!, viewState));
      }
      for (const kind of nextSession.app.windowKinds) {
        rendered.push(await plugin.tools(nextSession.instanceId, { ...viewState, activeWindowKindId: kind.id }));
      }
      rendered.push(await plugin.windowEngagements(nextSession.instanceId, viewState));
      rendered.push(await plugin.windowMeasures(nextSession.instanceId, viewState));
      const appLabelsOverlay = await plugin.appLabels(nextSession.instanceId, viewState);
      if (generation !== refreshGenerationRef.current) return;
      const windowNodes = await Promise.all(rendered.slice(0, windowCount).map((node) => resolveExternalSlots(node as UiNode, slotContext)));
      const panelNodes = await Promise.all(rendered.slice(windowCount, windowCount + panelTabLeaves.length).map((node) => resolveExternalSlots(node as UiNode, slotContext)));
      const toolsStart = windowCount + panelTabLeaves.length;
      const dynamicToolsByKind = rendered.slice(toolsStart, toolsStart + windowCount) as (readonly ToolNode[])[];
      const dynamicEngagements = rendered[rendered.length - 2] as Readonly<Record<string, WindowEngagement>>;
      const dynamicMeasures = rendered[rendered.length - 1] as Readonly<Record<string, readonly WindowMeasure[]>>;
      setWindowUiByKind(Object.fromEntries(nextSession.app.windowKinds.map((kind, index) => [kind.id, windowNodes[index]! as UiNode])));
      setWindowEngagementsByKind(dynamicEngagements);
      setWindowMeasuresByKind(dynamicMeasures);
      setAppLabelsOverlay(appLabelsOverlay);
      setPanelUiByKey(Object.fromEntries(panelTabLeaves.map((tab, index) => [tab.id, panelNodes[index]! as UiNode])));
      const activeModeId = viewState.activeModeId ?? nextSession.app.defaultModeId ?? nextSession.app.modes[0]?.id;
      const staticTools = nextSession.app.modes.find((mode) => mode.id === activeModeId)?.tools ?? [];
      setToolNodesByKind(
        Object.fromEntries(
          nextSession.app.windowKinds.map((kind, index) => {
            const dynamic = dynamicToolsByKind[index] ?? [];
            return [kind.id, dynamic.length > 0 ? dynamic : staticTools];
          }),
        ),
      );
      const windowIds = nextSession.app.windowKinds.map((kind) => kind.id);
      const layoutSeedKey = `${nextSession.pluginId}:${nextSession.app.id}:${nextSession.instanceId}`;
      if (layoutSeedKeyRef.current !== layoutSeedKey) {
        layoutSeedKeyRef.current = layoutSeedKey;
        setExtraWindowInstances([]);
        extraWindowCounterRef.current = 0;
        setShellLayout(convertFrameworkLayoutToModeLayout(nextSession.app.defaultLayout, windowIds, appLabelsOverlay));
        const defaultWindowId = findDefaultActiveWindowKindId(nextSession.app.defaultLayout, nextSession.app.windowKinds);
        if (defaultWindowId) setActiveWindowId(defaultWindowId);
        else if (windowIds[0]) setActiveWindowId(windowIds[0]);
      }
    },
    [loadedPlugins, uiLocale, uiTerminology],
  );

  /** @emoji 🗣️ Keeps already-built window titles (workbench layout, extra spawned windows) in sync with the app-labels overlay on every locale/terminology switch — `refreshUi` only rebuilds `shellLayout` from scratch on a session change, so an existing session's baked-in titles would otherwise go stale. */
  useEffect(() => {
    setShellLayout((current) => (current ? retitleWindowLayoutNode(current, appLabelsOverlay) : current));
    setExtraWindowInstances((current) =>
      current.map((entry) => ({ ...entry, title: resolveAppLabel(appLabelsOverlay, "windowKind", entry.windowKindId, entry.title) })),
    );
  }, [appLabelsOverlay]);

  const refreshSpawnedUi = useCallback(
    async (spawned: SpawnedAppEntry, viewState: ViewState) => {
      const generation = ++spawnedRefreshGenerationRef.current;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId);
      const plugin = pluginEntry?.handle;
      const app = pluginEntry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!plugin || !app) {
        console.warn("[os-shell] refreshSpawnedUi: plugin/app unavailable", { pluginId: spawned.pluginId, appId: spawned.appId });
        setSpawnedWindowUi({ type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}` } as UiNode);
        setSpawnedWindowEngagements({});
        setSpawnedWindowMeasures({});
        setSpawnedToolNodes([]);
        return;
      }
      const contributionsJson = buildContributionsJson(loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })));
      const fullViewState: ViewState = { ...viewState, contributionsJson, locale: uiLocale, terminology: uiTerminology };
      const bodyKey = resolveCanvasBodyKey(app);
      const [ui, dynamicTools, dynamicEngagements, dynamicMeasures] = await Promise.all([
        plugin.render(spawned.instanceId, bodyKey, fullViewState),
        plugin.tools(spawned.instanceId, fullViewState),
        plugin.windowEngagements(spawned.instanceId, fullViewState),
        plugin.windowMeasures(spawned.instanceId, fullViewState),
      ]);
      if (generation !== spawnedRefreshGenerationRef.current) return;
      setSpawnedWindowUi(ui as UiNode);
      setSpawnedWindowEngagements(dynamicEngagements);
      setSpawnedWindowMeasures(dynamicMeasures);
      setSpawnedToolNodes(selectSpawnedToolNodes(dynamicTools, app, fullViewState.activeModeId ?? app.defaultModeId ?? app.modes[0]?.id));
    },
    [loadedPlugins, uiLocale, uiTerminology],
  );

  useEffect(() => {
    if (!session) return;
    void refreshUi(session).catch((renderError) => {
      console.error("[DEBUG] render failed", renderError);
      setError(renderError instanceof Error ? renderError.message : String(renderError));
    });
  }, [loadedPlugins, refreshUi, session]);

  useEffect(() => {
    if (!studioMode || !session) {
      setSpawnedWindowUi(null);
      setSpawnedWindowEngagements({});
      setSpawnedWindowMeasures({});
      setSpawnedToolNodes([]);
      return;
    }
    const activeSpawned = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (!activeSpawned) {
      setSpawnedWindowUi(null);
      setSpawnedWindowEngagements({});
      setSpawnedWindowMeasures({});
      setSpawnedToolNodes([]);
      return;
    }
    void refreshSpawnedUi(activeSpawned, session.viewState).catch((renderError) => {
      console.error("[DEBUG] spawned render failed", renderError);
      setSpawnedWindowUi(null);
    });
  }, [loadedPlugins, panel, refreshSpawnedUi, session, studioMode]);

  const updateStudioPanel = useCallback((panelState: StudioPanelState) => {
    setSession((current) => {
      if (!current) return current;
      return { ...current, viewState: { ...current.viewState, panelJson: panelJsonFromState(panelState) } };
    });
  }, []);

  const switchToSApp = useCallback(
    async (appId: string, viewState?: ViewState): Promise<ActiveSession | null> => {
      const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === "s");
      const app = sPlugin?.manifest.apps.find((candidate) => candidate.id === appId);
      if (!sPlugin || !app) return null;
      if (session?.pluginId === sPlugin.handle.pluginId && session.app.id === appId) {
        if (!viewState) return session;
        const nextSession: ActiveSession = { ...session, viewState };
        setSession(nextSession);
        await refreshUi(nextSession);
        return nextSession;
      }
      const instanceId = await sPlugin.handle.createApp(app.id);
      const programs = buildStudioPrograms(loadedPlugins);
      const nextViewState: ViewState = viewState ?? {
        activeModeId: app.defaultModeId ?? app.modes[0]?.id,
        activeWindowKindId: app.windowKinds[0]?.id,
        panelJson: panelJsonFromState(buildStudioPanelState(programs, [])),
      };
      const nextSession: ActiveSession = { pluginId: sPlugin.handle.pluginId, instanceId, app, viewState: nextViewState };
      setSession(nextSession);
      setShellLayout(
        convertFrameworkLayoutToModeLayout(
          app.defaultLayout,
          app.windowKinds.map((kind) => kind.id),
          appLabelsOverlay,
        ),
      );
      setActiveWindowId(findDefaultActiveWindowKindId(app.defaultLayout, app.windowKinds) ?? app.windowKinds[0]?.id ?? null);
      if (appId === S_HOME_APP_ID) {
        openStudioIdRef.current = null;
        openInstanceIdRef.current = null;
      }
      await refreshUi(nextSession);
      return nextSession;
    },
    [loadedPlugins, refreshUi, session, appLabelsOverlay],
  );

  const syncSpawnedPluginDocument = useCallback(async (plugin: PluginWasmHandle, app: AppDefinition, pluginInstanceId: number, documentJson: string, viewState: ViewState) => {
    try {
      const document = JSON.parse(documentJson) as Record<string, unknown>;
      await plugin.handleAction(pluginInstanceId, JSON.stringify({ controllerId: app.controllerId, action: "setDocument", args: { document } }), viewState);
    } catch (syncError) {
      console.error("[DEBUG] spawned plugin document sync failed", syncError);
    }
  }, []);

  const ensureSpawnedPlugin = useCallback(
    async (program: StudioProgramEntry, label?: string, osInstanceId?: string, documentJson?: string) => {
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return;
      const app = pluginEntry.manifest.apps.find((candidate) => candidate.id === program.appId);
      const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
      const existing = osInstanceId ? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId) : currentPanel.spawnedApps.find((entry) => entry.appId === program.appId && entry.pluginId === program.pluginId);
      if (existing) {
        if (documentJson && app) {
          await syncSpawnedPluginDocument(pluginEntry.handle, app, existing.instanceId, documentJson, session.viewState);
        }
        updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, currentPanel.activePanelTab, existing.id));
        return;
      }
      const instanceId = await pluginEntry.handle.createApp(program.appId);
      if (documentJson && app) {
        await syncSpawnedPluginDocument(pluginEntry.handle, app, instanceId, documentJson, session.viewState);
      }
      const spawnedId = osInstanceId ?? `${program.pluginId}-${instanceId}`;
      updateStudioPanel(
        buildStudioPanelState(
          currentPanel.programs,
          [
            ...currentPanel.spawnedApps,
            {
              id: spawnedId,
              pluginId: program.pluginId,
              instanceId,
              appId: program.appId,
              label: label ?? program.label,
              document: program.document,
            },
          ],
          currentPanel.activePanelTab,
          spawnedId,
        ),
      );
    },
    [loadedPlugins, session, syncSpawnedPluginDocument, updateStudioPanel],
  );

  const processPluginOps = useCallback(
    async (ops: readonly string[], baseSession: ActiveSession) => {
      let nextViewState = baseSession.viewState;
      for (const opJson of ops) {
        const op = JSON.parse(opJson) as {
          op?: string;
          uri?: string;
          panel?: StudioPanelState;
          programId?: string;
          appId?: string;
          osInstanceId?: string;
          label?: string;
          documentJson?: string;
          filename?: string;
          mimeType?: string;
          data?: string;
          encoding?: string;
          accept?: string;
          importAction?: string;
          readAs?: string;
          items?: readonly { filename: string; request: unknown }[];
        };
        if (op.op === "setPanel" && op.panel) {
          nextViewState = { ...nextViewState, panelJson: panelJsonFromState(op.panel) };
        }
        if (op.op === "navigate" && typeof op.uri === "string") {
          navigateHistory(op.uri);
          continue;
        }
        if (op.op === "downloadMediaExport" && op.filename && op.mimeType && op.data) {
          downloadMediaExport(op.filename, op.mimeType, op.data, op.encoding);
        }
        if (op.op === "iconRenderExport" && op.items) {
          for (const item of op.items) {
            try {
              const result = await iconRenderPort.render(item.request as Parameters<typeof iconRenderPort.render>[0]);
              downloadDataUrl(item.filename, result.dataUrl);
            } catch (error) {
              console.error(`icon render export failed for ${item.filename}`, error);
            }
          }
        }
        if (op.op === "requestFileOpen" && op.importAction) {
          const opened = await requestFileOpen(op.accept ?? ".json,.spatial.json", op.readAs);
          if (opened) {
            const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
            if (pluginEntry) {
              const importOps = await pluginEntry.handle.handleAction(
                baseSession.instanceId,
                JSON.stringify({
                  controllerId: baseSession.app.controllerId,
                  action: op.importAction,
                  args: { payload: opened.contents, name: opened.name },
                }),
                baseSession.viewState,
              );
              await processPluginOps(importOps, baseSession);
            }
          }
        }
        if (op.op === "spawnPluginInstance" && op.programId && op.appId) {
          const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
          const program = currentPanel.programs.find((entry) => entry.programId === op.programId && entry.appId === op.appId) ?? currentPanel.programs.find((entry) => entry.programId === op.programId);
          if (program) await ensureSpawnedPlugin(program, op.label, op.osInstanceId, op.documentJson);
        }
        if (op.op === "openPluginInstance" && op.programId && op.appId) {
          const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
          const program = currentPanel.programs.find((entry) => entry.programId === op.programId && entry.appId === op.appId) ?? currentPanel.programs.find((entry) => entry.programId === op.programId);
          if (program) {
            await ensureSpawnedPlugin(program, op.label, op.osInstanceId, op.documentJson);
            if (op.osInstanceId && openStudioIdRef.current) {
              openInstanceIdRef.current = op.osInstanceId;
              navigateHistory(`/studios/${openStudioIdRef.current}/instances/${op.osInstanceId}`);
            }
          } else {
            console.warn(
              "[os-shell] openPluginInstance: no program matches",
              { programId: op.programId, appId: op.appId },
              "available:",
              currentPanel.programs.map((entry) => `${entry.programId}/${entry.appId}`),
            );
          }
        }
      }
      const nextSession = { ...baseSession, viewState: nextViewState };
      const isSpawnedPluginSession = studioMode && session && baseSession.pluginId !== session.pluginId;
      setSession((current) => {
        if (!current) return nextSession;
        if (isSpawnedPluginSession) return { ...current, viewState: nextViewState };
        if (current.instanceId !== nextSession.instanceId) return current;
        return { ...current, viewState: nextViewState };
      });
      if (isSpawnedPluginSession) {
        const spawned = parsePanelState(nextViewState)?.spawnedApps.find((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
        if (spawned) await refreshSpawnedUi(spawned, nextViewState);
      } else if (session?.instanceId === nextSession.instanceId || baseSession.instanceId === nextSession.instanceId) {
        await refreshUi(nextSession);
      }
      if (syncBackboneUri) {
        for (const opJson of ops) {
          const op = JSON.parse(opJson) as { op?: string; document?: unknown };
          if (op.op !== "setDocument" || op.document == null) continue;
          const docId = syncDocumentId(session, panel, studioMode);
          const envelopeJson = wrapDocumentEnvelope(op.document, docId, syncBackboneUri);
          lastEnvelopeJsonRef.current = envelopeJson;
          void writeBackboneEnvelope(syncBackboneUri, envelopeJson).catch(() => {
            /* backbone auto-sync is best-effort */
          });
        }
      }
    },
    [ensureSpawnedPlugin, loadedPlugins, navigateHistory, panel, refreshSpawnedUi, refreshUi, session, studioMode, syncBackboneUri],
  );

  const applyShellUri = useCallback(
    async (uri: string, preservedViewState?: ViewState) => {
      const currentSession = sessionRef.current;
      if (!studioMode || !currentSession || loadedPlugins.length === 0) return;
      const path = uri.split("?")[0] ?? "/";
      const studioPath = parseStudioShellPath(path);
      const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === "s")?.handle;
      if (!sPlugin) return;
      if (!studioPath) {
        openStudioIdRef.current = null;
        openInstanceIdRef.current = null;
        if (currentSession.app.id !== S_HOME_APP_ID) await switchToSApp(S_HOME_APP_ID, preservedViewState);
        return;
      }
      const { studioId, instanceId } = studioPath;
      const studioSession = currentSession.app.id === S_PLAY_APP_ID ? currentSession : await switchToSApp(S_PLAY_APP_ID, preservedViewState);
      if (!studioSession) return;
      if (openStudioIdRef.current !== studioId) {
        openStudioIdRef.current = studioId;
        openInstanceIdRef.current = null;
        await sPlugin.handleAction(studioSession.instanceId, JSON.stringify({ controllerId: S_PLAY_CONTROLLER_ID, action: "openStudio", args: { studioId } }), studioSession.viewState);
        await refreshUi(studioSession);
      }
      if (openInstanceIdRef.current === (instanceId ?? null)) return;
      openInstanceIdRef.current = instanceId ?? null;
      if (instanceId) {
        const ops = await sPlugin.handleAction(studioSession.instanceId, JSON.stringify({ controllerId: S_PLAY_CONTROLLER_ID, action: "openInstance", args: { instanceId } }), studioSession.viewState);
        await processPluginOps(ops, studioSession);
      } else {
        const ops = await sPlugin.handleAction(studioSession.instanceId, JSON.stringify({ controllerId: S_PLAY_CONTROLLER_ID, action: "closeFocusedInstance" }), studioSession.viewState);
        const currentPanel = parsePanelState(studioSession.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
        updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, currentPanel.activePanelTab, undefined));
        await processPluginOps(ops, studioSession);
      }
    },
    [loadedPlugins, processPluginOps, refreshUi, studioMode, switchToSApp, updateStudioPanel],
  );

  useEffect(() => {
    if (!studioMode || loadedPlugins.length === 0) return;
    void applyShellUri(shellUri).catch((uriError) => {
      console.error("[DEBUG] shell uri apply failed", uriError);
    });
  }, [applyShellUri, loadedPlugins.length, shellUri, studioMode]);

  const resolveSyncTargetSession = useCallback((): ActiveSession | null => {
    if (!session) return null;
    if (studioMode && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const app = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        if (app) return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
      }
    }
    return session;
  }, [loadedPlugins, panel, session, studioMode]);

  const attachSyncBackbone = useCallback(
    async (uri: string) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
      if (!plugin) return;
      const docId = syncDocumentId(targetSession, panel, studioMode);
      let envelopeJson = await readBackboneEnvelope(uri);
      if (!envelopeJson) {
        envelopeJson = wrapDocumentEnvelope({}, docId, uri);
        await writeBackboneEnvelope(uri, envelopeJson);
      }
      lastEnvelopeJsonRef.current = envelopeJson;
      const document = documentFromEnvelopeJson(envelopeJson);
      const ops = await plugin.handleAction(
        targetSession.instanceId,
        JSON.stringify({ controllerId: targetSession.app.controllerId, action: "setDocument", args: { document } }),
        targetSession.viewState,
      );
      setSyncBackboneUri(uri);
      setSyncCardKind(null);
      await processPluginOps(ops, targetSession);
    },
    [loadedPlugins, panel, processPluginOps, resolveSyncTargetSession, studioMode],
  );

  const detachSyncBackbone = useCallback(() => {
    setSyncBackboneUri(null);
    setSyncCardKind(null);
    lastEnvelopeJsonRef.current = null;
  }, []);

  const spawnProgram = useCallback(
    async (program: StudioProgramEntry) => {
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return;
      const instanceId = await pluginEntry.handle.createApp(program.appId);
      const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
      const spawnedId = `${program.pluginId}-${instanceId}`;
      updateStudioPanel(
        buildStudioPanelState(
          currentPanel.programs,
          [
            ...currentPanel.spawnedApps,
            {
              id: spawnedId,
              pluginId: program.pluginId,
              instanceId,
              appId: program.appId,
              label: program.label,
              document: program.document,
            },
          ],
          currentPanel.activePanelTab,
          spawnedId,
        ),
      );
    },
    [loadedPlugins, session, updateStudioPanel],
  );

  const onAction = useCallback(
    (action: ActionDescriptor) => {
      if (!session) return;

      if (action.controllerId === FRAMEWORK_SYNC_CONTROLLER_ID) {
        if (action.action === "selectFile") {
          setSyncCardKind("file");
          setSyncDraftPath(syncBackboneUri?.startsWith("file://") ? syncBackboneUri.slice("file://".length) : "");
          return;
        }
        if (action.action === "selectFolder") {
          setSyncCardKind("folder");
          setSyncDraftPath(syncBackboneUri?.startsWith("folder://") ? syncBackboneUri.slice("folder://".length) : "");
          return;
        }
        if (action.action === "selectRemote") {
          setSyncCardKind("remote");
          const remote = syncBackboneUri?.startsWith("remote://") ? syncBackboneUri.slice("remote://".length) : "";
          setSyncDraftPath(remote);
          return;
        }
        if (action.action === "attach") {
          const path = typeof action.args === "object" && action.args != null && "path" in action.args ? String((action.args as { path?: string }).path ?? "") : syncDraftPath;
          if (!path.trim()) return;
          const uri =
            action.args && typeof action.args === "object" && "kind" in action.args
              ? String((action.args as { kind?: string }).kind) === "remote"
                ? buildRemoteBackboneUri(path.split("/")[0] ?? "127.0.0.1:8787", path.split("/").slice(1).join("/") || syncDocumentId(session, panel, studioMode))
                : String((action.args as { kind?: string }).kind) === "folder"
                  ? buildFolderBackboneUri(path)
                  : buildFileBackboneUri(path)
              : buildFileBackboneUri(path);
          void attachSyncBackbone(uri);
          return;
        }
        if (action.action === "detach") {
          void detachSyncBackbone();
          return;
        }
        return;
      }

      if (studioMode && action.controllerId === S_HOME_CONTROLLER_ID && action.action === "importStudio") {
        importStudioInputRef.current?.click();
        return;
      }

      if (studioMode && action.action === "spawnApp" && action.controllerId !== S_PLAY_CONTROLLER_ID) {
        const programId = typeof action.args === "object" && action.args != null && "programId" in action.args ? String((action.args as { programId?: string }).programId ?? "") : "";
        const pluginId = typeof action.args === "object" && action.args != null && "pluginId" in action.args ? String((action.args as { pluginId?: string }).pluginId ?? "") : "";
        const currentPanel = parsePanelState(session.viewState);
        const program = currentPanel?.programs.find((entry) => entry.programId === programId && entry.pluginId === pluginId);
        if (program) void spawnProgram(program);
        return;
      }

      if (studioMode && action.controllerId === S_PLAY_CONTROLLER_ID && action.action === "setActivePanelTab") {
        const tabId = typeof action.args === "object" && action.args != null && "tabId" in action.args ? String((action.args as { tabId?: string }).tabId ?? "s-play-catalogue") : "s-play-catalogue";
        const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
        updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, tabId, currentPanel.activeSpawnedId));
        return;
      }

      const pluginEntry = findPluginForAction(action);
      const plugin = pluginEntry?.handle;
      if (!plugin) return;

      const targetSession =
        studioMode && action.controllerId !== session.app.controllerId
          ? (() => {
              const spawned = panel?.spawnedApps.find((entry) => {
                const app = loadedPlugins.find((p) => p.handle.pluginId === entry.pluginId)?.manifest.apps.find((a) => a.id === entry.appId);
                return app?.controllerId === action.controllerId;
              });
              if (!spawned) return session;
              const app = loadedPlugins.find((p) => p.handle.pluginId === spawned.pluginId)?.manifest.apps.find((a) => a.id === spawned.appId);
              if (!app) return session;
              return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
            })()
          : session;

      void plugin
        .handleAction(targetSession.instanceId, JSON.stringify(action), targetSession.viewState)
        .then(async (ops) => {
          if (studioMode && session.pluginId === "s" && panel?.activeSpawnedId && action.controllerId !== session.app.controllerId) {
            const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
            const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === "s")?.handle;
            if (spawned && sPlugin) {
              for (const opJson of ops) {
                const op = JSON.parse(opJson) as { op?: string; document?: unknown };
                if (op.op === "setDocument" && op.document != null) {
                  const patchOps = await sPlugin.handleAction(
                    session.instanceId,
                    JSON.stringify({
                      controllerId: S_PLAY_CONTROLLER_ID,
                      action: "patchAppSource",
                      args: { instanceId: spawned.id, inline: JSON.stringify(op.document) },
                    }),
                    session.viewState,
                  );
                  await processPluginOps(patchOps, session);
                }
              }
            }
          }
          await processPluginOps(ops, targetSession);
        })
        .catch((actionError) => {
          console.error("[DEBUG] action failed", actionError);
        });
    },
    [attachSyncBackbone, detachSyncBackbone, findPluginForAction, loadedPlugins, panel, processPluginOps, session, spawnProgram, studioMode, syncBackboneUri, syncDraftPath, updateStudioPanel],
  );

  const onActionRef = useRef(onAction);
  useEffect(() => {
    onActionRef.current = onAction;
  }, [onAction]);

  const studioSessionActive = studioMode && session?.app.id === S_PLAY_APP_ID;
  useEffect(() => {
    if (!studioSessionActive || typeof window === "undefined") return;
    const identity = presenceClientIdentity();
    const beat = () => onActionRef.current({ controllerId: S_PLAY_CONTROLLER_ID, action: "presenceHeartbeat", args: identity });
    const initial = window.setTimeout(beat, 1000);
    const timer = window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS);
    return () => {
      window.clearTimeout(initial);
      window.clearInterval(timer);
    };
  }, [studioSessionActive]);

  useSidePanelChromeHotkeys({
    onToggleLeft: () => setLeftPanelVisible((visible) => !visible),
    onToggleRight: () => setRightPanelVisible((visible) => !visible),
  });

  useElementsSurfaceChrome({ appearance: uiAppearance, device: uiDevice, expertise: uiExpertise, compact: uiCompact });

  useEffect(() => {
    writeStoredUiChromeAppearance(uiAppearance);
  }, [uiAppearance]);

  useEffect(() => {
    writeStoredUiChromeLayout(uiLayout);
  }, [uiLayout]);

  useEffect(() => {
    writeStoredUiChromeCompact(uiCompact);
  }, [uiCompact]);

  useEffect(() => {
    writeStoredUiChromeExpertise(uiExpertise);
  }, [uiExpertise]);

  useEffect(() => {
    writeStoredUiChromeLocale(uiLocale);
    void setUiLocale(uiLocale);
  }, [uiLocale]);

  useEffect(() => {
    writeStoredUiChromeTerminology(uiTerminology);
  }, [uiTerminology]);

  useEffect(() => {
    setActiveUiTheme(uiTheme);
    writeStoredUiChromeThemeSnapshot(uiTheme);
  }, [uiTheme]);

  useEffect(() => {
    writeStoredUiChromeThemeId(uiThemeId);
  }, [uiThemeId]);

  useEffect(() => {
    writeStoredUiCustomThemes(uiCustomThemes);
  }, [uiCustomThemes]);

  useActionHotkey(
    "mod+[",
    useCallback(() => {
      if (canGoBack) goBack();
    }, [canGoBack, goBack]),
  );
  useActionHotkey(
    "mod+]",
    useCallback(() => {
      if (canGoForward) goForward();
    }, [canGoForward, goForward]),
  );
  useActionHotkey(
    "mod+up",
    useCallback(() => {
      if (canGoUp) goUp();
    }, [canGoUp, goUp]),
  );
  useActionHotkey(
    "mod+p",
    useCallback(() => setSearchOpen((open) => !open), []),
  );
  useActionHotkey(
    "mod+f",
    useCallback(() => setFindOpen((open) => !open), []),
  );

  const applyNamedLayout = useCallback(
    (layout: WindowLayout) => {
      if (!session) return;
      const windowIds = session.app.windowKinds.map((kind) => kind.id);
      setExtraWindowInstances([]);
      extraWindowCounterRef.current = 0;
      setShellLayout(convertFrameworkLayoutToModeLayout(layout, windowIds, appLabelsOverlay));
      const defaultWindowId = findDefaultActiveWindowKindId(layout, session.app.windowKinds);
      if (defaultWindowId) setActiveWindowId(defaultWindowId);
    },
    [session, appLabelsOverlay],
  );

  const applyModeChange = useCallback((modeId: string) => {
    setSession((current) => {
      if (!current) return current;
      const layout = resolveLayoutForMode(current.app, modeId);
      if (layout) {
        setExtraWindowInstances([]);
        extraWindowCounterRef.current = 0;
        setShellLayout(
          convertFrameworkLayoutToModeLayout(
            layout,
            current.app.windowKinds.map((kind) => kind.id),
            appLabelsOverlay,
          ),
        );
        const defaultWindowId = findDefaultActiveWindowKindId(layout, current.app.windowKinds);
        if (defaultWindowId) setActiveWindowId(defaultWindowId);
      }
      return { ...current, viewState: { ...current.viewState, activeModeId: modeId } };
    });
  }, [appLabelsOverlay]);

  const handleTemplateDrop = useCallback(
    (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => {
      if (!session) return;
      const kind = session.app.windowKinds.find((entry) => entry.id === payload.windowKindId);
      if (!kind) return;
      extraWindowCounterRef.current += 1;
      const instanceId = `${payload.windowKindId}-${extraWindowCounterRef.current}`;
      setExtraWindowInstances((current) => [...current, { id: instanceId, windowKindId: payload.windowKindId, title: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label) }]);
      setShellLayout((current) => {
        const base =
          current ??
          convertFrameworkLayoutToModeLayout(
            session.app.defaultLayout,
            session.app.windowKinds.map((entry) => entry.id),
            appLabelsOverlay,
          );
        return insertWindowAtDropZone(base, instanceId, target);
      });
      setActiveWindowId(instanceId);
    },
    [appLabelsOverlay, session],
  );

  const displayHostRef = useRef<ReturnType<typeof useNamedLayoutHost> | null>(null);
  const displayHost = useNamedLayoutHost({
    appId: session?.app.id ?? "framework-os",
    windowKinds: session?.app.windowKinds.map((kind) => ({ ...kind, label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label) })) ?? [],
    builtinLayouts: session?.app.namedLayouts ?? [],
    currentLayout: captureCurrentFrameworkLayout(shellLayout, session?.app.defaultLayout),
    onApplyLayout: applyNamedLayout,
    namedLayoutStore,
  });
  displayHostRef.current = displayHost;

  //#region 🔖ThemeMutators
  const uiThemeBase = uiThemeDraft ?? uiTheme;
  const uiThemeDirty = uiThemeDraft !== null;
  const uiThemeList = useMemo((): readonly UiTheme[] => [...builtinUiThemes(), ...Object.values(uiCustomThemes)], [uiCustomThemes]);

  const draftThemePatch = useCallback(
    (patch: (next: UiTheme) => void) => {
      const next = structuredClone(uiThemeBase);
      patch(next);
      setUiThemeDraft(next);
    },
    [uiThemeBase],
  );

  const setThemeId = useCallback((id: string) => {
    setUiThemeDraft(null);
    setUiThemeIdState(id);
  }, []);

  const setThemeColor = useCallback((key: string, hex: string) => draftThemePatch((next) => { next.colors[key] = hex; }), [draftThemePatch]);
  const setThemeSpacing = useCallback((key: string, value: string) => draftThemePatch((next) => { next.spacing[key] = value; }), [draftThemePatch]);
  const setThemeFontStack = useCallback((key: string, value: string) => draftThemePatch((next) => { next.fontStacks[key] = value; }), [draftThemePatch]);
  const setThemeStroke = useCallback((key: string, value: number | number[]) => draftThemePatch((next) => { next.strokes[key] = value; }), [draftThemePatch]);
  const setThemeRadius = useCallback((key: string, value: number) => draftThemePatch((next) => { next.radii[key] = value; }), [draftThemePatch]);
  const setThemeOpacity = useCallback((key: string, value: number) => draftThemePatch((next) => { next.opacities[key] = value; }), [draftThemePatch]);
  const setThemeMetric = useCallback(
    (section: string, key: string, value: number | number[]) =>
      draftThemePatch((next) => {
        next.metrics[section] = { ...(next.metrics[section] ?? {}), [key]: value };
      }),
    [draftThemePatch],
  );
  const setThemeAppearancePaint = useCallback(
    (appearance: ThemeAppearanceName, group: ThemePaletteGroup, key: string, hex: string, alpha?: number) =>
      draftThemePatch((next) => {
        next.appearances[appearance][group][key] = alpha === undefined ? { hex } : { hex, alpha };
      }),
    [draftThemePatch],
  );

  const resetTheme = useCallback(() => {
    setUiThemeDraft(null);
    setUiThemeIdState("semio");
  }, []);

  const saveTheme = useCallback(
    (label: string) => {
      const trimmed = label.trim();
      if (!trimmed) return;
      const slug = trimmed
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, "-")
        .replace(/(^-+|-+$)/g, "");
      if (!slug) return;
      const id = `custom.${slug}`;
      const saved: UiTheme = { ...uiThemeBase, id, label: trimmed };
      setUiCustomThemesState((current) => ({ ...current, [id]: saved }));
      setUiThemeDraft(null);
      setUiThemeIdState(id);
    },
    [uiThemeBase],
  );

  const deleteTheme = useCallback(
    (id: string) => {
      if (!id.startsWith("custom.")) return;
      setUiCustomThemesState((current) => {
        const { [id]: _removed, ...rest } = current;
        return rest;
      });
      setUiThemeIdState((current) => (current === id ? "semio" : current));
      setUiThemeDraft(null);
    },
    [],
  );

  const exportTheme = useCallback(() => {
    downloadMediaExport(`${uiThemeBase.id}.theme.json`, "application/json", serializeUiTheme(uiThemeBase));
  }, [uiThemeBase]);

  const importTheme = useCallback(async () => {
    const opened = await requestFileOpen(".theme.json,application/json");
    if (!opened) return;
    try {
      const parsed = parseUiTheme(JSON.parse(opened.contents));
      saveTheme(parsed.label || parsed.id);
    } catch {
      /* invalid theme file, ignore */
    }
  }, [saveTheme]);
  //#endregion 🔖ThemeMutators

  const settingsHostRef = useRef<SettingsHostApi | null>(null);
  const settingsHost: SettingsHostApi = useMemo(
    () => ({
      appId: session?.app.id,
      appLabel: session ? appDocumentLabel(session.app.document) : undefined,
      controllerId: session?.app.controllerId,
      pluginId: session?.pluginId,
      compact: uiCompact,
      setCompact: setUiCompact,
      expertise: uiExpertise,
      setExpertise: setUiExpertise,
      appearance: uiAppearance,
      setAppearance: setUiAppearance,
      layout: uiLayout,
      setLayout: setUiLayout,
      mobileActive: mobile,
      locale: uiLocale,
      setLocale: setUiLocaleState,
      terminology: uiTerminology,
      setTerminology: setUiTerminologyState,
      terminologies: [UI_TERMINOLOGY_NATIVE, ...(session?.app.terminologies ?? [])],
      theme: uiThemeBase,
      themeId: uiThemeId,
      themeDirty: uiThemeDirty,
      themes: uiThemeList,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
    }),
    [
      session,
      uiCompact,
      uiExpertise,
      uiAppearance,
      uiLayout,
      mobile,
      uiLocale,
      uiTerminology,
      uiThemeBase,
      uiThemeId,
      uiThemeDirty,
      uiThemeList,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
    ],
  );
  settingsHostRef.current = settingsHost;

  const frameworkDisplayTabs = useMemo(() => createFrameworkDisplayPanelTabs(() => displayHostRef.current), [displayHost, uiLocale]);
  const frameworkSettingsTabs = useMemo(() => createFrameworkSettingsPanelTabs(() => settingsHostRef.current), [settingsHost]);

  useEffect(() => {
    if (!session?.app.keybindings.length) return;
    const parseKeys = (keys: string) =>
      keys
        .split(",")
        .map((key) => key.trim().toLowerCase())
        .filter(Boolean);
    const isEditableTarget = (target: EventTarget | null) => {
      if (!(target instanceof HTMLElement)) return false;
      const tag = target.tagName;
      if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
      if (target.isContentEditable) return true;
      return target.closest("[contenteditable='true'], [role='textbox']") != null;
    };
    const matches = (event: KeyboardEvent, binding: string) => {
      const parts = binding.split("+").map((part) => part.trim());
      const key = parts[parts.length - 1] ?? "";
      const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
      const needsShift = parts.includes("shift");
      const needsAlt = parts.includes("alt");
      const hasCtrl = event.ctrlKey || event.metaKey;
      if (needsCtrl !== hasCtrl) return false;
      if (needsShift !== event.shiftKey) return false;
      if (needsAlt !== event.altKey) return false;
      return event.key.toLowerCase() === key;
    };
    const onKeyDown = (event: KeyboardEvent) => {
      if (isEditableTarget(event.target)) return;
      for (const binding of session.app.keybindings) {
        for (const chord of parseKeys(binding.keys)) {
          if (!matches(event, chord)) continue;
          event.preventDefault();
          onAction(binding.action);
          return;
        }
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [onAction, session]);

  const activePanelTabId = panel?.activePanelTab ?? session?.app.panelTabs.find((tab) => panelSideForGroup(tab.group) === "right")?.id ?? session?.app.panelTabs[0]?.id;

  const workbenchLeftTabs = useMemo((): PanelTabNode[] => {
    if (!session) return [];
    const pluginLeftTabs = session.app.panelTabs
      .filter((tab) => panelSideForGroup(tab.group) === "left")
      .map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay));
    if (studioMode && session.app.id === S_PLAY_APP_ID && pluginLeftTabs.length > 0) return pluginLeftTabs;
    const hasPluginDocumentTab = pluginLeftTabs.some((tab) => tab.id === FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
    if (hasPluginDocumentTab) return pluginLeftTabs;
    const documentTab: PanelTabNode = {
      kind: "leaf",
      id: FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
      icon: shellTabIcon(FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID),
      name: shellLabel("ui.panel.document"),
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [
          {
            id: "document.root",
            label: shellLabel("ui.panel.document"),
            items: [{ id: "document.empty", label: studioMode ? `${panel?.spawnedApps.length ?? 0} ${shellLabel("ui.panel.spawnedAppsSuffix")}` : shellLabel("ui.panel.documentEmpty") }],
          },
        ],
      }),
    };
    return [documentTab, ...pluginLeftTabs];
  }, [appLabelsOverlay, onAction, panel?.spawnedApps.length, panelUiByKey, session, studioMode, uiLocale]);

  const detailsRightTabs = useMemo((): PanelTabNode[] => {
    if (!session) return [];
    return session.app.panelTabs
      .filter((tab) => panelSideForGroup(tab.group) === "right")
      .map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay));
  }, [appLabelsOverlay, onAction, panelUiByKey, session]);

  const settingsRightTabs = useMemo((): PanelTabNode[] => frameworkSettingsTabs, [frameworkSettingsTabs]);

  const leftPanelTabs = useMemo((): PanelTabNode[] => {
    const categories: PanelTabNode[] = [
      { kind: "branch", id: FRAMEWORK_CATEGORY_WORKBENCH_ID, icon: categoryTabIcon(workbenchLeftTabs, "folder"), name: shellLabel("ui.panelToggle.workbench"), order: 0, children: workbenchLeftTabs },
    ];
    if (frameworkDisplayTabs.length > 0) {
      categories.push({ kind: "branch", id: FRAMEWORK_CATEGORY_DISPLAY_ID, icon: categoryTabIcon(frameworkDisplayTabs, "layout-grid"), name: shellLabel("ui.panelToggle.display"), order: 1, children: frameworkDisplayTabs });
    }
    return categories;
  }, [frameworkDisplayTabs, uiLocale, workbenchLeftTabs]);

  const rightPanelTabs = useMemo(
    (): PanelTabNode[] => [
      { kind: "branch", id: FRAMEWORK_CATEGORY_DETAILS_ID, icon: categoryTabIcon(detailsRightTabs, "info"), name: shellLabel("ui.panelToggle.details"), order: 0, children: detailsRightTabs },
      { kind: "branch", id: FRAMEWORK_CATEGORY_SETTINGS_ID, icon: categoryTabIcon(settingsRightTabs, "settings-2"), name: shellLabel("ui.panelToggle.settings"), order: 1, children: settingsRightTabs },
    ],
    [detailsRightTabs, settingsRightTabs, uiLocale],
  );

  const leftPanelActivePath = useMemo((): readonly string[] => {
    if (studioMode && session?.app.id === S_PLAY_APP_ID && leftPanelPath[0] !== FRAMEWORK_CATEGORY_DISPLAY_ID) {
      const path = findPanelTabPath(leftPanelTabs, panel?.activePanelTab ?? S_PLAY_CATALOGUE_TAB_ID);
      if (path) return path;
    }
    return reconcileActivePath(leftPanelTabs, leftPanelPath, panelTabChildren);
  }, [leftPanelPath, leftPanelTabs, panel?.activePanelTab, session?.app.id, studioMode]);

  const rightPanelActivePath = useMemo((): readonly string[] => {
    if (rightPanelPath[0] !== FRAMEWORK_CATEGORY_SETTINGS_ID && panel?.activePanelTab) {
      const path = findPanelTabPath(rightPanelTabs, panel.activePanelTab);
      if (path) return path;
    }
    return reconcileActivePath(rightPanelTabs, rightPanelPath, panelTabChildren);
  }, [panel?.activePanelTab, rightPanelPath, rightPanelTabs]);

  const mobilePanelTabs = useMemo(() => [...leftPanelTabs, ...rightPanelTabs], [leftPanelTabs, rightPanelTabs]);

  const mobilePanel = useMemo(() => {
    if (mobilePanelTabs.length === 0) return undefined;
    return {
      visible: leftPanelVisible || rightPanelVisible,
      tabs: mobilePanelTabs,
      activeTabPath: mobilePanelPath,
      onActiveTabPathChange: (path: readonly string[]) => {
        setMobilePanelPath(path);
        const tabId = path[path.length - 1];
        if (tabId && studioMode && session?.app.id === S_PLAY_APP_ID) {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
      },
    };
  }, [leftPanelVisible, mobilePanelPath, mobilePanelTabs, onAction, rightPanelVisible, session, studioMode]);

  const activePluginManifest = useMemo(() => loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId)?.manifest, [loadedPlugins, session?.pluginId]);
  const exampleOptions = useMemo(() => {
    const appId = session?.app.id ?? "";
    if (!appId) return [];
    const seen = new Set<string>();
    return (activePluginManifest?.examples ?? [])
      .filter((example) => example.appId === appId)
      .filter((example) => {
        if (seen.has(example.id)) return false;
        seen.add(example.id);
        return true;
      })
      .map((example) => ({ id: example.id, label: example.label }));
  }, [activePluginManifest, session?.app.id]);

  useEffect(() => {
    if (exampleOptions.length === 0) return;
    setActiveExampleId((current) => (!current || exampleOptions.some((option) => option.id === current) ? current : ""));
  }, [exampleOptions, session?.app.id, session?.pluginId]);

  useEffect(() => {
    if (exampleOptions.length === 0 || activeExampleId || !session) return;
    if (noExampleResetInstanceIdRef.current === session.instanceId) return;
    noExampleResetInstanceIdRef.current = session.instanceId;
    onAction({ controllerId: session.app.controllerId, action: "setActiveExample", args: { exampleId: "" } });
  }, [activeExampleId, exampleOptions, onAction, session]);

  const activeModeId = session?.viewState.activeModeId ?? session?.app.modes[0]?.id ?? session?.app.id ?? "";

  const navbarItems = useMemo((): NavbarItem[] => {
    if (!session) return [];
    // Logo/title, example selector, and mode switcher render as one cluster, centered as a group in the navbar
    // (via `centered`) rather than left-anchored with fill spacers pushing the rest toward the trailing edge.
    const centerContent: ReactNode[] = [
      <div key="logoAndTitle" className="flex min-w-0 shrink-0 items-center gap-single">
        <SemioLogo className="size-workbench shrink-0" />
        <span data-slot="app-name" className={cn("px-single", shellChromeTitleClassName)}>
          {appDocumentLabel(session.app.document)}
        </span>
      </div>,
    ];
    if (exampleOptions.length > 0 && (!studioMode || session.app.id !== S_HOME_APP_ID)) {
      centerContent.push(
        <NavbarExampleSelect
          key="fixture"
          id="playground.navbar.fixture"
          value={activeExampleId}
          options={exampleOptions}
          onValueChange={(exampleId) => {
            setActiveExampleId(exampleId);
            onAction({ controllerId: session.app.controllerId, action: "setActiveExample", args: { exampleId } });
          }}
        />,
      );
    }
    if (session.app.modes.length > 1) {
      centerContent.push(
        <ButtonGroup key="modes" id="playground.navbar.modes">
          {session.app.modes.map((mode) => {
            const isActive = activeModeId === mode.id;
            return (
              <ButtonGroupItem
                key={mode.id}
                id={`playground.navbar.modes.${mode.id}`}
                className={cn(isActive && interactiveActiveFillClass)}
                data-state={isActive ? "on" : undefined}
                onClick={() => applyModeChange(mode.id)}
                icon={<span className="hidden" />}
                text={resolveAppLabel(appLabelsOverlay, "mode", mode.id, mode.label)}
              />
            );
          })}
        </ButtonGroup>,
      );
    }
    return [{ key: "center", centered: true, content: <div className="flex min-w-0 items-center gap-double">{centerContent}</div> }];
  }, [activeExampleId, activeModeId, appLabelsOverlay, applyModeChange, exampleOptions, onAction, session]);

  const searchItems = useMemo(() => {
    if (!session) return [];
    const items: UISearchItem[] = [];
    for (const tab of flattenPanelTabLeaves(session.app.panelTabs)) {
      items.push({
        id: `panel.${tab.id}`,
        label: resolveAppLabel(appLabelsOverlay, "panelTab", tab.id, tab.label),
        category: shellLabel("ui.search.category.panels"),
        icon: <Icon icon="panel-left" size="small" />,
        onSelect: () => onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId: tab.id } }),
      });
    }
    for (const kind of session.app.windowKinds) {
      items.push({
        id: `window.${kind.id}`,
        label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label),
        category: shellLabel("ui.search.category.windows"),
        icon: <Icon icon="app-window" size="small" />,
        onSelect: () => setActiveWindowId(kind.id),
      });
    }
    const keysByActionId = new Map(session.app.keybindings.map((binding) => [binding.action.action, binding.keys]));
    const declaredActionIds = new Set<string>();
    for (const action of session.app.actions ?? []) {
      if (!action.inPalette) continue;
      declaredActionIds.add(action.id);
      items.push({
        id: `action.${action.id}`,
        label: action.label,
        description: action.keys ?? keysByActionId.get(action.id),
        category: action.category ?? (action.kind === "history" ? shellLabel("ui.toolbar.parent.history") : shellLabel("ui.toolbar.parent.actions")),
        onSelect: () => onAction({ controllerId: session.app.controllerId, action: action.id }),
      });
    }
    for (const binding of session.app.keybindings) {
      if (declaredActionIds.has(binding.action.action)) continue;
      items.push({
        id: `keybinding.${binding.keys}`,
        label: binding.action.action,
        description: binding.keys,
        category: shellLabel("ui.toolbar.parent.actions"),
        onSelect: () => onAction(binding.action),
      });
    }
    if (studioMode && panel) {
      for (const program of panel.programs) {
        items.push({
          id: `spawn.${program.programId}`,
          label: `${shellLabel("ui.palette.spawnPrefix")} ${appDocumentLabel(program.document)}`,
          category: shellLabel("ui.search.category.catalogue"),
          onSelect: () => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "spawnApp", args: { programId: program.programId } }),
        });
      }
      items.push(
        {
          id: "studio.undo",
          label: shellLabel("ui.palette.undo"),
          category: shellLabel("ui.search.category.studio"),
          icon: <Icon icon="undo-2" size="small" />,
          onSelect: () => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "undo" }),
        },
        {
          id: "studio.redo",
          label: shellLabel("ui.palette.redo"),
          category: shellLabel("ui.search.category.studio"),
          icon: <Icon icon="redo-2" size="small" />,
          onSelect: () => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "redo" }),
        },
        {
          id: "studio.home",
          label: shellLabel("ui.palette.goHome"),
          category: shellLabel("ui.search.category.navigation"),
          onSelect: () => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "goHome" }),
        },
      );
    }
    return items;
  }, [appLabelsOverlay, onAction, panel, session, studioMode, uiLocale]);

  const footerItems = [];

  const footerModeToolTree = useMemo(() => {
    const deduped = dedupeToolNodesById([...Object.values(toolNodesByKind), spawnedToolNodes]);
    const grouped = groupToolNodesByCategory(deduped, FOOTER_TOOL_CATEGORIES);
    if (!grouped.length) return undefined;
    return <ToolTree id="ui.toolbar.footer" tools={grouped} onAction={onAction} />;
  }, [onAction, spawnedToolNodes, toolNodesByKind]);

  const footerToolbar = useMemo(() => {
    const syncTools = buildFrameworkSyncTools(syncBackboneUri) as readonly ToolNode[];
    const syncCard = syncTools.length ? (
      <SyncAttachCard
        activeUri={syncBackboneUri}
        cardKind={syncCardKind}
        draftPath={syncDraftPath}
        syncTools={syncTools}
        onAction={onAction}
        onDraftPathChange={setSyncDraftPath}
        onClose={() => setSyncCardKind(null)}
        onAttach={attachSyncBackbone}
        onDetach={detachSyncBackbone}
      />
    ) : undefined;
    if (!footerModeToolTree && !syncCard) return undefined;
    return (
      <>
        {footerModeToolTree}
        {syncCard}
      </>
    );
  }, [attachSyncBackbone, detachSyncBackbone, footerModeToolTree, onAction, syncBackboneUri, syncCardKind, syncDraftPath]);

  const modeWindows = useMemo((): ModeWindowDescriptor[] => {
    if (!session) return [];
    if (studioMode && spawnedWindowUi && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const spawnedApp = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        const windowKind = spawnedApp?.windowKinds[0];
        const chrome = windowKind ? spawnedWindowChromeForKind(windowKind, spawnedWindowEngagements, spawnedWindowMeasures, onAction) : undefined;
        return [
          {
            id: spawned.id,
            title: appDocumentLabel(spawned.document),
            fill: true,
            showControls: true,
            measures: chrome?.measures,
            engagement: chrome?.engagement,
            toolbar: windowToolbarNode(spawnedToolNodes, spawned.id, onAction),
            children: <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden">{interpretUiNode(spawnedWindowUi, { onAction })}</ChromeAwareWindowScrollSurface>,
          },
        ];
      }
    }
    if (Object.keys(windowUiByKind).length === 0) return [];
    const baseWindows = session.app.windowKinds.map((kind) => ({
      id: kind.id,
      title: appWindowDocumentLabel(session.app, resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label)),
      fill: true,
      showControls: true,
      measures: windowMeasuresOverlay(windowMeasuresByKind[kind.id] ?? kind.measures, onAction),
      engagement: windowEngagementToSpec(resolveWindowEngagement(kind, windowEngagementsByKind), onAction),
      toolbar: windowToolbarNode(toolNodesByKind[kind.id], kind.id, onAction),
      children: (
        <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" data-window-kind-id={kind.id}>
          {interpretUiNode(windowUiByKind[kind.id] ?? { type: "text", value: `${shellLabel("ui.common.missingWindow")}: ${kind.id}` }, { onAction })}
        </ChromeAwareWindowScrollSurface>
      ),
    }));
    const extraWindows = extraWindowInstances.flatMap((instance) => {
      const kind = session.app.windowKinds.find((entry) => entry.id === instance.windowKindId);
      if (!kind) return [];
      return [
        {
          id: instance.id,
          title: instance.title,
          fill: true,
          showControls: true,
          measures: windowMeasuresOverlay(windowMeasuresByKind[kind.id] ?? kind.measures, onAction),
          engagement: windowEngagementToSpec(resolveWindowEngagement(kind, windowEngagementsByKind), onAction),
          toolbar: windowToolbarNode(toolNodesByKind[kind.id], instance.id, onAction),
          children: (
            <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" data-window-kind-id={kind.id}>
              {interpretUiNode(windowUiByKind[kind.id] ?? { type: "text", value: `${shellLabel("ui.common.missingWindow")}: ${kind.id}` }, { onAction })}
            </ChromeAwareWindowScrollSurface>
          ),
        },
      ];
    });
    return [...baseWindows, ...extraWindows];
  }, [appLabelsOverlay, extraWindowInstances, loadedPlugins, onAction, panel, session, spawnedToolNodes, spawnedWindowEngagements, spawnedWindowMeasures, spawnedWindowUi, studioMode, toolNodesByKind, uiLocale, windowEngagementsByKind, windowMeasuresByKind, windowUiByKind]);

  const effectiveModeLayout = useMemo(
    () =>
      shellLayout ??
      (session ? convertFrameworkLayoutToModeLayout(session.app.defaultLayout, modeWindows.map((window) => window.id), appLabelsOverlay) : { kind: "stack" as const, children: [] }),
    [appLabelsOverlay, modeWindows, session, shellLayout],
  );

  const canvas = useMemo(() => {
    if (!session) return <p className="p-4 text-sm text-muted-foreground">{shellLabel("ui.common.loadingPlugins")}</p>;
    if (error)
      return (
        <p className="p-4 text-sm text-destructive" role="alert">
          {error}
        </p>
      );
    const modes = session.app.modes.length > 0 ? session.app.modes : [{ id: session.app.id, label: appDocumentLabel(session.app.document) }];
    const studioHomeBar =
      studioMode && session.app.id === S_PLAY_APP_ID && !panel?.activeSpawnedId ? (
        <button type="button" className="border-b border-border/60 px-3 py-2 text-left text-sm text-muted-foreground hover:bg-muted/40 hover:text-foreground" onClick={() => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "goHome" })}>
          ← {shellLabel("ui.common.home")}
        </button>
      ) : null;
    const focusedSpawned = panel?.activeSpawnedId ? panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId) : undefined;
    const focusedBar = focusedSpawned ? (
      <div className="flex items-center gap-2 border-b border-border/60 px-3 py-2 text-sm text-muted-foreground">
        <button type="button" className="hover:text-foreground" onClick={() => (openStudioIdRef.current ? navigateHistory(`/studios/${openStudioIdRef.current}`) : onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "closeFocusedInstance" }))}>
          ← {shellLabel("ui.common.backToMediaGraph")}
        </button>
        <span>·</span>
        <span>{appDocumentLabel(focusedSpawned.document)}</span>
      </div>
    ) : null;
    return (
      <div className="flex h-full min-h-0 flex-col overflow-hidden">
        {studioHomeBar}
        {focusedBar}
        <input
          ref={importStudioInputRef}
          type="file"
          accept="application/json,.json"
          className="hidden"
          onChange={(event) => {
            const file = event.target.files?.[0];
            if (!file) return;
            void file.text().then((json) => {
              onAction({ controllerId: S_HOME_CONTROLLER_ID, action: "importStudio", args: { json } });
              event.target.value = "";
            });
          }}
        />
        <div className="min-h-0 flex-1">
          <App modes={modes.map((mode) => ({ id: mode.id, label: resolveAppLabel(appLabelsOverlay, "mode", mode.id, mode.label), children: null }))} activeModeId={session.viewState.activeModeId ?? modes[0]?.id ?? session.app.id} onActiveModeChange={applyModeChange} chrome={false}>
            <Mode
              className="h-full w-full"
              mobile={mobile}
              windows={modeWindows}
              layout={effectiveModeLayout}
              activeWindowId={activeWindowId}
              onActiveWindowChange={setActiveWindowId}
              onLayoutChange={setShellLayout}
              onTemplateDrop={mobile ? undefined : handleTemplateDrop}
              onWindowClose={(windowId) => {
                if (studioMode && panel?.spawnedApps.some((entry) => entry.id === windowId)) {
                  const nextSpawned = panel.spawnedApps.filter((entry) => entry.id !== windowId);
                  updateStudioPanel(buildStudioPanelState(panel.programs, nextSpawned, panel.activePanelTab, nextSpawned[0]?.id));
                }
                setExtraWindowInstances((current) => current.filter((entry) => entry.id !== windowId));
                setShellLayout(
                  (current) =>
                    current ??
                    convertFrameworkLayoutToModeLayout(
                      session.app.defaultLayout,
                      modeWindows.map((window) => window.id),
                      appLabelsOverlay,
                    ),
                );
              }}
            />
          </App>
        </div>
      </div>
    );
  }, [activeWindowId, effectiveModeLayout, error, handleTemplateDrop, mobile, modeWindows, navigateHistory, onAction, panel, session, studioMode, uiLocale, updateStudioPanel]);

  return (
    <UIFindProvider>
      <LevelProvider level="window">
        <div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
          <Layout
            mobile={mobile}
            mobilePanel={mobilePanel}
            navbar={<Navbar items={navbarItems} showFullscreenToggle />}
            footer={<Footer items={footerItems} toolbar={footerToolbar} />}
            leftSidePanel={
              workbenchLeftTabs.length > 0 || frameworkDisplayTabs.length > 0
                ? {
                    position: "left",
                    visible: leftPanelVisible,
                    onVisibleChange: setLeftPanelVisible,
                    size: leftPanelSize,
                    onSizeChange: setLeftPanelSize,
                    tabs: leftPanelTabs,
                    activeTabPath: leftPanelActivePath,
                    onActiveTabPathChange: (path) => {
                      setLeftPanelPath(path);
                      const tabId = path[path.length - 1];
                      if (tabId && studioMode && session?.app.id === S_PLAY_APP_ID) {
                        onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
                      }
                    },
                  }
                : undefined
            }
            rightSidePanel={
              detailsRightTabs.length > 0 || settingsRightTabs.length > 0
                ? {
                    position: "right",
                    visible: rightPanelVisible,
                    onVisibleChange: setRightPanelVisible,
                    size: rightPanelSize,
                    onSizeChange: setRightPanelSize,
                    tabs: rightPanelTabs,
                    activeTabPath: rightPanelActivePath,
                    onActiveTabPathChange: (path) => {
                      setRightPanelPath(path);
                      const tabId = path[path.length - 1];
                      if (tabId && studioMode && session?.app.id === S_PLAY_APP_ID) {
                        onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
                      }
                    },
                  }
                : undefined
            }
            canvas={<ShellRenderErrorBoundary>{canvas}</ShellRenderErrorBoundary>}
          />
        </div>
        <UISearch items={searchItems} open={searchOpen} onOpenChange={setSearchOpen} />
        <UIFind open={findOpen} onOpenChange={setFindOpen} />
      </LevelProvider>
    </UIFindProvider>
  );
}
//#endregion FrameworkOsShell

//#region 🔖types
export type ActionDescriptor = {
  readonly controllerId: string;
  readonly action: string;
  readonly args?: unknown;
};

export type StyleSpec = {
  readonly variant?: string;
  readonly size?: string;
  readonly density?: string;
};

export type UiStackNode = {
  readonly type: "stack";
  readonly direction: string;
  readonly gap?: string;
  readonly padding?: string;
  readonly id?: string;
  readonly selected?: boolean;
  readonly activate?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
  readonly children: readonly UiNode[];
};

export type UiTextNode = {
  readonly type: "text";
  readonly value: string;
  readonly emphasize?: boolean;
  readonly dataAttributes?: Readonly<Record<string, string>>;
};

export type UiButtonNode = {
  readonly type: "button";
  readonly id?: string;
  readonly iconId: string;
  readonly label: string;
  readonly action: ActionDescriptor;
  readonly style?: StyleSpec;
  readonly disabled?: boolean;
};

export type UiSeparatorNode = { readonly type: "separator" };

export type UiImageNode = {
  readonly type: "image";
  readonly id: string;
  readonly src: string;
  readonly alt?: string;
};

export type UiInputNode = {
  readonly type: "input";
  readonly id: string;
  readonly inputKind: string;
  readonly value: string;
  readonly placeholder?: string;
  readonly commit?: string;
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
  readonly accept?: string;
  readonly onChange: ActionDescriptor;
};

export type UiSelectItem = {
  readonly value: string;
  readonly label: string;
};

export type UiSelectNode = {
  readonly type: "select";
  readonly id: string;
  readonly value: string;
  readonly items: readonly UiSelectItem[];
  readonly placeholder?: string;
  readonly onChange: ActionDescriptor;
};

export type UiToggleNode = {
  readonly type: "toggle";
  readonly id: string;
  readonly iconId: string;
  readonly pressed: boolean;
  readonly text?: string;
  readonly onChange: ActionDescriptor;
};

export type UiVec3Node = {
  readonly type: "vec3";
  readonly id: string;
  readonly value: readonly [number, number, number] | null;
  readonly onChange: ActionDescriptor;
};

export type UiKeyValueEntry = {
  readonly label: string;
  readonly value: string;
};

export type UiKeyValueNode = {
  readonly type: "keyValue";
  readonly entries: readonly UiKeyValueEntry[];
};

export type UiSliderNode = {
  readonly type: "slider";
  readonly id: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly unit?: string;
  readonly onChange: ActionDescriptor;
};

export type UiNumberStepperNode = {
  readonly type: "numberStepper";
  readonly id: string;
  readonly value: number;
  readonly step: number;
  readonly uniform: boolean;
  readonly onAbsolute: ActionDescriptor;
  readonly onDelta: ActionDescriptor;
};

export type UiRingNode = {
  readonly type: "ring";
  readonly id: string;
  readonly orbId: string;
  readonly t: number;
  readonly disabled?: boolean;
  readonly onChange: ActionDescriptor;
};

export type UiIconSelectNode = {
  readonly type: "iconSelect";
  readonly id: string;
  readonly value: string;
  readonly uniform: boolean;
  readonly classifierKind: string;
  readonly onChange: ActionDescriptor;
};

export type UiControlNode = UiInputNode | UiSelectNode | UiToggleNode | UiVec3Node | UiButtonNode | UiKeyValueNode | UiSliderNode | UiNumberStepperNode | UiRingNode | UiIconSelectNode;

export type UiFieldNode = {
  readonly type: "field";
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly required?: boolean;
  readonly error?: string;
  readonly child: UiNode;
};

export type UiSectionNode = {
  readonly type: "section";
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly children: readonly UiNode[];
};

export type UiTreeItemAction = {
  readonly iconId: string;
  readonly label?: string;
  readonly action: ActionDescriptor;
  readonly revealOnHover?: boolean;
};

export type UiTreeItemNode = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly iconId?: string;
  readonly selected?: boolean;
  readonly defaultOpen?: boolean;
  readonly action?: ActionDescriptor;
  readonly hoverAction?: ActionDescriptor;
  readonly unhoverAction?: ActionDescriptor;
  readonly actions?: readonly UiTreeItemAction[];
  readonly draggable?: boolean;
  readonly dragData?: Readonly<Record<string, string>>;
  readonly items?: readonly UiTreeItemNode[];
  readonly control?: UiControlNode;
  readonly isHidden?: boolean;
};

export type UiTreeSectionNode = {
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly items: readonly UiTreeItemNode[];
};

export type UiTreeNode = {
  readonly type: "tree";
  readonly sections: readonly UiTreeSectionNode[];
  readonly selectedIds?: readonly string[];
  readonly highlightedIds?: readonly string[];
  readonly selectionChange?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
};

export type UiInspectorFieldGroup = {
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly fields: readonly UiNode[];
};

export type Canvas2dScene = {
  readonly cameraX: number;
  readonly cameraY: number;
  readonly zoom: number;
  readonly layersJson: string;
};

export type World3dScene = {
  readonly cameraJson: string;
  readonly meshesJson: string;
  readonly instancesJson: string;
  readonly selectionJson: string;
  readonly vorticesJson?: string;
  readonly attractionsJson?: string;
  readonly targetVolumesJson?: string;
  readonly referencesJson?: string;
  readonly brushPreviewJson?: string;
  readonly interactionJson?: string;
  readonly engagementPreviewJson?: string;
  readonly lodJson?: string;
  readonly chunkingJson?: string;
  readonly contextMenuJson?: string;
  readonly environmentJson?: string;
  readonly frameJson?: string;
  readonly fitJson?: string;
};

export type NodeGraphScene = {
  readonly nodesJson: string;
  readonly edgesJson: string;
  readonly viewportJson: string;
  readonly editable?: boolean;
  readonly operatorsJson?: string;
  readonly contextMenuJson?: string;
  readonly findItemsJson?: string;
  readonly selectionJson?: string;
  readonly hoverJson?: string;
  readonly previewOffJson?: string;
  readonly lodJson?: string;
  readonly catalogueJson?: string;
  readonly controlsJson?: string;
  readonly clustersJson?: string;
  readonly computingJson?: string;
  readonly capabilitiesJson?: string;
  readonly fixtureJson?: string;
  readonly presencePeersJson?: string;
};

export type PresencePeer = {
  readonly clientId: string;
  readonly name: string;
  readonly selectionCount: number;
};

export type TextEditorScene = {
  readonly buffer: string;
  readonly language?: string;
  readonly selectionJson?: string;
  readonly tokensJson?: string;
  readonly diagnosticsJson?: string;
  readonly completionsJson?: string;
  readonly overlaysJson?: string;
  readonly occurrencesJson?: string;
  readonly placeholdersJson?: string;
  readonly extraCaretsJson?: string;
  readonly selectableSpansJson?: string;
  readonly settingsJson?: string;
  readonly cameraJson?: string;
  readonly hoverJson?: string;
  readonly newlineGatesJson?: string;
  readonly renameJson?: string;
};

export const nodeGraphActions = {
  select: "nodeGraphSelect",
  hover: "nodeGraphHover",
  edit: "nodeGraphEdit",
  viewport: "nodeGraphViewport",
  spotlightCommit: "spotlightCommit",
} as const;

export const textEditorActions = {
  edit: "textEdit",
  select: "textSelect",
  hover: "textHover",
  requestCompletions: "requestCompletions",
  commitRename: "commitRename",
  formatDocument: "formatDocument",
} as const;

export type TableScene = {
  readonly columnsJson: string;
  readonly rowsJson: string;
};

export type RasterScene = {
  readonly documentSyncJson: string;
  readonly assetsJson: string;
  readonly cameraJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeTool: string;
  readonly brushSize: number;
  readonly brushOpacity: number;
  readonly viewMode: string;
  readonly compositeViewportJson?: string;
};

export type IconRenderScene = {
  readonly requestJson: string;
  readonly footer?: string;
  readonly frameJson?: string;
};

export type VirtualFileSystemScene = {
  readonly schemaJson: string;
  readonly rowsJson: string;
  readonly selectedRowIdsJson?: string;
  readonly hoveredRowId?: string;
  readonly emptyMessage?: string;
  readonly dragDropEnabled?: boolean;
};

export type GisMapScene = {
  readonly mapFixtureJson: string;
  readonly cameraJson: string;
  readonly renderMode: string;
  readonly vectorStyle: string;
  readonly lodMode: string;
  readonly tileUrlTemplate: string;
  readonly vectorTileUrlTemplate: string;
  readonly layerVisibilityJson: string;
  readonly layerStrokeScaleJson: string;
  readonly selectionJson: string;
  readonly hoverJson: string;
  readonly selectionMethod: string;
  readonly selectionMode: string;
  readonly contextMenuJson?: string;
};

export type Puzzle2dBoardScene = {
  readonly fixtureJson: string;
  readonly cameraJson: string;
  readonly kindCatalogsJson: string;
  readonly selectionJson: string;
  readonly interactive: boolean;
  readonly hoveredId?: string;
  readonly activeTool?: string;
  readonly selectionMethod: string;
  readonly gridSnapEnabled: boolean;
  readonly gridFactor: number;
  readonly suggestionOffset: number;
  readonly brushKindWeightsJson: string;
  readonly kindCompatibilityJson: string;
  readonly lodMode: string;
};

export type NoteCanvasScene = {
  readonly documentJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeTool: string;
  readonly viewMode: string;
  readonly interactive: boolean;
};

/** @emoji 🗄️ A checkpoint ancestor-graph history view. `columnsJson` is a `HistoryColumn[]` array, newest checkpoint first. */
export type VcsHistoryScene = {
  readonly columnsJson: string;
};

export type UiExternalSlotNode = {
  readonly type: "externalSlot";
  readonly pluginId: string;
  readonly appId: string;
  readonly bodyKey: string;
  readonly paramsJson: string;
};

export type UiComponentSceneNode = {
  readonly type: "componentScene";
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly componentKind: string;
  readonly paneId?: string;
  readonly bindingId?: string;
  readonly canvas2d?: Canvas2dScene;
  readonly world3d?: World3dScene;
  readonly nodeGraph?: NodeGraphScene;
  readonly textEditor?: TextEditorScene;
  readonly table?: TableScene;
  readonly raster?: RasterScene;
  readonly virtualFileSystem?: VirtualFileSystemScene;
  readonly gisMap?: GisMapScene;
  readonly puzzle2dBoard?: Puzzle2dBoardScene;
  readonly iconRender?: IconRenderScene;
  readonly noteCanvas?: NoteCanvasScene;
  readonly vcsHistory?: VcsHistoryScene;
};

export type UiNode =
  | UiStackNode
  | UiTextNode
  | UiButtonNode
  | UiSeparatorNode
  | UiInputNode
  | UiSelectNode
  | UiToggleNode
  | UiVec3Node
  | UiKeyValueNode
  | UiSliderNode
  | UiNumberStepperNode
  | UiRingNode
  | UiIconSelectNode
  | UiFieldNode
  | UiSectionNode
  | UiTreeNode
  | UiImageNode
  | UiComponentSceneNode
  | UiExternalSlotNode;

export type WindowLayoutWindowNode = {
  readonly kind: "window";
  readonly windowKindId: string;
  readonly title?: string;
  readonly instanceId?: string;
  readonly templateId?: string;
};

export type WindowLayoutStackNode = {
  readonly kind: "stack";
  readonly size?: number;
  readonly children: readonly WindowLayoutWindowNode[];
};

export type WindowLayoutAxisNode = {
  readonly kind: "row" | "column";
  readonly size?: number;
  readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
};

export type WindowLayout = {
  readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
};

export type NamedLayout = {
  readonly id: string;
  readonly label: string;
  readonly iconId?: string;
  readonly layout: WindowLayout;
  readonly origin: "builtin" | "user";
  readonly groupPath?: readonly string[];
};

export type WindowEngagementOption = {
  readonly id: string;
  readonly label?: string;
  readonly iconId?: string;
  readonly pressed?: boolean;
  readonly disabled?: boolean;
  readonly action?: ActionDescriptor;
};

export type WindowEngagementInput = {
  readonly id?: string;
  readonly value?: string;
  readonly placeholder?: string;
  readonly disabled?: boolean;
  readonly onChange?: ActionDescriptor;
  readonly onSubmit?: ActionDescriptor;
  readonly onRepeatLast?: ActionDescriptor;
  readonly onAbort?: ActionDescriptor;
};

export type WindowEngagementStatus = {
  readonly id: string;
  readonly text: string;
};

export type WindowEngagementPossible = {
  readonly id: string;
  readonly label: string;
  readonly detail?: string;
  readonly action?: ActionDescriptor;
};

export type WindowEngagementRingOption = {
  readonly id: string;
  readonly label: string;
  readonly disabled?: boolean;
};

export type WindowEngagementToggleGroupOption = {
  readonly id: string;
  readonly label: string;
  readonly disabled?: boolean;
};

export type WindowEngagementSelectItem = {
  readonly id: string;
  readonly value: string;
  readonly label: string;
};

export type WindowEngagementControl =
  | {
      readonly kind: "slider";
      readonly id?: string;
      readonly label?: string;
      readonly value: number;
      readonly min: number;
      readonly max: number;
      readonly step?: number;
      readonly unit?: string;
      readonly disabled?: boolean;
      readonly onChange?: ActionDescriptor;
      readonly onCommit?: ActionDescriptor;
    }
  | {
      readonly kind: "stepper";
      readonly id?: string;
      readonly label?: string;
      readonly value: number;
      readonly min?: number;
      readonly max?: number;
      readonly step?: number;
      readonly unit?: string;
      readonly disabled?: boolean;
      readonly onChange?: ActionDescriptor;
      readonly onCommit?: ActionDescriptor;
    }
  | {
      readonly kind: "ring";
      readonly id?: string;
      readonly label?: string;
      readonly value?: string;
      readonly options: readonly WindowEngagementRingOption[];
      readonly disabled?: boolean;
      readonly onSelect?: ActionDescriptor;
    }
  | {
      readonly kind: "toggleGroup";
      readonly id?: string;
      readonly label?: string;
      readonly value?: string;
      readonly options: readonly WindowEngagementToggleGroupOption[];
      readonly disabled?: boolean;
      readonly onSelect?: ActionDescriptor;
    }
  | {
      readonly kind: "select";
      readonly id?: string;
      readonly label?: string;
      readonly value?: string;
      readonly placeholder?: string;
      readonly items: readonly WindowEngagementSelectItem[];
      readonly disabled?: boolean;
      readonly onChange?: ActionDescriptor;
    };

export type WindowEngagement = {
  readonly sessionActive?: boolean;
  readonly options?: readonly WindowEngagementOption[];
  readonly input?: WindowEngagementInput;
  readonly control?: WindowEngagementControl;
  readonly controls?: readonly WindowEngagementControl[];
  readonly status?: readonly WindowEngagementStatus[];
  readonly possibleEngagements?: readonly WindowEngagementPossible[];
};

export type WindowMeasure =
  | {
      readonly kind: "select";
      readonly id: string;
      readonly label?: string;
      readonly value: string;
      readonly items: readonly { readonly id: string; readonly value: string; readonly label: string }[];
      readonly onChange: ActionDescriptor;
    }
  | {
      readonly kind: "slider";
      readonly id: string;
      readonly label?: string;
      readonly value: number;
      readonly min: number;
      readonly max: number;
      readonly step?: number;
      readonly onChange: ActionDescriptor;
    }
  | {
      readonly kind: "toggle";
      readonly id: string;
      readonly iconId: string;
      readonly label?: string;
      readonly pressed: boolean;
      readonly text?: string;
      readonly onChange: ActionDescriptor;
    }
  | {
      readonly kind: "group";
      readonly id: string;
      readonly label: string;
      readonly defaultOpen?: boolean;
      readonly children: readonly WindowMeasure[];
    };

export type ViewState = {
  readonly activeModeId?: string;
  readonly activeWindowKindId?: string;
  readonly selectionJson?: string;
  readonly panelJson?: string;
  readonly contributionsJson?: string;
  readonly locale?: string;
  readonly terminology?: string;
};

/** @emoji 🌳 Mirrors Rust `PanelTabDefinition` — a leaf carries `bodyKey`, a branch carries `children`; `group` is only meaningful on root entries. */
export type AppPanelTabDefinition = {
  readonly id: string;
  readonly label: string;
  readonly group: string;
  readonly bodyKey?: string;
  readonly children?: readonly AppPanelTabDefinition[];
};

export type AppDefinition = {
  readonly id: string;
  readonly label: string;
  readonly document: readonly string[];
  readonly iconId?: string;
  readonly controllerId: string;
  readonly modes: readonly { readonly id: string; readonly label: string; readonly tools?: readonly ToolNode[] }[];
  readonly defaultModeId?: string;
  readonly windowKinds: readonly {
    readonly id: string;
    readonly label: string;
    readonly bodyKey: string;
    readonly iconId?: string;
    readonly measures?: readonly WindowMeasure[];
    readonly engagement?: WindowEngagement;
  }[];
  readonly panelTabs: readonly AppPanelTabDefinition[];
  readonly keybindings: readonly { readonly keys: string; readonly action: ActionDescriptor }[];
  readonly actions?: readonly ActionDefinition[];
  readonly namedLayouts?: readonly NamedLayout[];
  readonly defaultLayout?: WindowLayout;
  readonly terminologies?: readonly string[];
};

export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly AppDefinition[];
  readonly programs: readonly { readonly programId: string; readonly appId: string; readonly label: string; readonly document: readonly string[]; readonly yields: string }[];
  readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string; readonly appId: string }[];
  readonly contributions?: readonly {
    readonly kind: "formsQuestionKind";
    readonly appId: string;
    readonly questionKind: string;
    readonly label: string;
    readonly iconId: string;
    readonly defaultValueJson?: string;
    readonly paramsBodyKey: string;
    readonly previewBodyKey: string;
  }[];
};

export type PluginHotSwapEvent = {
  readonly pluginId: string;
  readonly version: string;
  readonly addedApps: readonly string[];
  readonly removedApps: readonly string[];
};

export enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

export type ToolCategory = "selection" | "tools" | "actions" | "history" | "sync";

export type ToolLeaf =
  | { readonly id: string; readonly kind: "separator"; readonly order?: number; readonly disabled?: boolean }
  | {
      readonly id: string;
      readonly kind: "button";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: ToolCategory;
      readonly controllerId?: string;
      readonly action?: string;
      readonly args?: unknown;
    }
  | {
      readonly id: string;
      readonly kind: "toggle";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly pressed?: boolean;
      readonly disabled?: boolean;
      readonly category?: ToolCategory;
      readonly controllerId?: string;
      readonly action?: string;
      readonly args?: unknown;
    };

export type ToolNode =
  | ToolLeaf
  | {
      readonly id: string;
      readonly kind: "collection";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: ToolCategory;
      readonly children: readonly ToolNode[];
    }
  | {
      readonly id: string;
      readonly kind: "button";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: ToolCategory;
      readonly onPress: ActionDescriptor;
    }
  | {
      readonly id: string;
      readonly kind: "toggle";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly pressed?: boolean;
      readonly disabled?: boolean;
      readonly category?: ToolCategory;
      readonly onChange: ActionDescriptor;
    };

/** @emoji 🌐 Locale-resolved mixed-value placeholder for this renderer layer; framework/core/js/index.ts keeps its own non-reactive low-level default. */
export const UI_INSPECTOR_MIXED_PLACEHOLDER = shellLabel("ui.common.mixedValues");

export const FRAMEWORK_PANEL_TAB_DOCUMENT_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL = "Document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL = "Catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL = "Inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ID = "framework.panel.parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL = "Parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID = "framework.panel.parameters";
//#endregion 🔖types

//#region 🔖plugin-runtime

export type PluginWasmHandle = {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleAction: (instanceId: number, actionJson: string, viewState: ViewState) => Promise<string[]>;
  readonly render: (instanceId: number, bodyKey: string, viewState: ViewState) => Promise<UiNode>;
  readonly renderWithDocument?: (instanceId: number, bodyKey: string, viewState: ViewState, documentJson: string) => Promise<UiNode>;
  readonly tools: (instanceId: number, viewState: ViewState) => Promise<readonly ToolNode[]>;
  readonly windowEngagements: (instanceId: number, viewState: ViewState) => Promise<Readonly<Record<string, WindowEngagement>>>;
  readonly windowMeasures: (instanceId: number, viewState: ViewState) => Promise<Readonly<Record<string, readonly WindowMeasure[]>>>;
  readonly appLabels: (instanceId: number, viewState: ViewState) => Promise<PluginAppLabelsOverlay>;
  readonly dispose: () => void;
};

export type { PluginRegistryEntry };
export { DEFAULT_PLUGIN_REGISTRY };

export async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  return adaptPluginHandle(await loadCorePluginModule(pluginId, moduleUrl));
}

export async function loadPluginWasm(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  return adaptPluginHandle(await loadCorePluginWasm(pluginId, moduleUrl));
}

function adaptPluginHandle(handle: CorePluginWasmHandle): PluginWasmHandle {
  return {
    pluginId: handle.pluginId,
    manifest: handle.manifest as unknown as PluginManifest,
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleAction: (instanceId, actionJson, viewState) => handle.handleAction(instanceId, actionJson, viewState),
    render: async (instanceId, bodyKey, viewState) => (await handle.render(instanceId, bodyKey, viewState)) as unknown as UiNode,
    renderWithDocument: handle.renderWithDocument ? async (instanceId, bodyKey, viewState, documentJson) => (await handle.renderWithDocument!(instanceId, bodyKey, viewState, documentJson)) as unknown as UiNode : undefined,
    tools: async (instanceId, viewState) => (await handle.tools(instanceId, viewState)) as unknown as ToolNode[],
    windowEngagements: async (instanceId, viewState) => (await handle.windowEngagements(instanceId, viewState)) as unknown as Readonly<Record<string, WindowEngagement>>,
    windowMeasures: async (instanceId, viewState) => (await handle.windowMeasures(instanceId, viewState)) as unknown as Readonly<Record<string, readonly WindowMeasure[]>>,
    appLabels: (instanceId, viewState) => handle.appLabels(instanceId, viewState),
    dispose: () => handle.dispose(),
  };
}
//#endregion 🔖plugin-runtime

//#region 🔖wasm-session-loader

//#region GraphSession
type GraphSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly GraphSession: new () => GraphWasmSession;
};

let graphSessionPromise: Promise<GraphSessionModule> | null = null;

export async function createGraphSession(): Promise<GraphWasmSession> {
  if (!graphSessionPromise) {
    graphSessionPromise = import("@semio-tech/framework-graph-rs/pkg/framework_graph.js").then(async (mod) => {
      await mod.default();
      return mod as GraphSessionModule;
    });
  }
  const mod = await graphSessionPromise;
  return new mod.GraphSession();
}
//#endregion GraphSession

//#region FlowSession
export type FlowWasmSession = GraphWasmSession & {
  loadFixtureJson(json: string): void;
  fixtureJson(): string;
  syncFromSceneJson?(json: string): void;
  setSelection(json: string): void;
  setPreviewOff(json: string): void;
  setCatalogueJson(json: string): void;
  setNeuronKindInfosJson(json: string): void;
  setComputingProgress(json: string): void;
  setAutomaticLod(enabled: boolean): void;
  setForcedDrawLodLabel(label: string): void;
  setCanvasThemeJson(json: string): void;
  setCamera(x: number, y: number, zoom: number): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean, pan: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
  labelOverlayPaintStateJson(): string;
  paramOverlayPaintStateJson(): string;
  stepperOverlayStateJson(): string;
  sliderOverlayStateJson(): string;
  selectionUnionBoundsScreenJson(): string;
  selectionPreviewPointsJson(): string;
  selectionPreviewCrossing(): boolean;
  selectedWidgetIds(): string;
  hoveredWidgetId(): string | undefined;
  hoveredChannelJson(): string;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  previewText(): string;
  preselectWidgetIdsJson(): string;
  previewOffWidgetIds(): string;
  alignSelection(mode: string): void;
  undo(): boolean;
  redo(): boolean;
  selectAll(): void;
  deleteSelection(): void;
  addWidget(descriptorJson: string, worldX: number, worldY: number): string;
  setGhostWidget(descriptorJson: string, worldX: number, worldY: number): void;
  clearGhostWidget(): void;
  worldFromScreen(sx: number, sy: number): string;
  evaluateSync(): string;
  noteInsertText(chunk: string): void;
  noteBackspace(): void;
  noteDeleteForward(): void;
  noteCommitEdit(): void;
  noteMoveCaret(direction: string, extend: boolean): void;
  setSliderValue(widgetId: string, value: number): void;
  setStepperFieldValue(widgetId: string, fieldKey: string, value: number): void;
  setNeuronParams(widgetId: string, paramsJson: string): void;
  setHover?(widgetId: string | null): void;
  setHoverChannel?(widgetId: string | null, port?: string | null): void;
  cameraJson?(): string;
};

type FlowSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly FlowSession: new () => FlowWasmSession;
};

let flowSessionPromise: Promise<FlowSessionModule> | null = null;

export async function createFlowSession(): Promise<FlowWasmSession> {
  if (!flowSessionPromise) {
    flowSessionPromise = import("@semio-tech/flow-core/pkg/flow_core.js").then(async (mod) => {
      await mod.default();
      return mod as FlowSessionModule;
    });
  }
  const mod = await flowSessionPromise;
  return new mod.FlowSession();
}
//#endregion FlowSession

//#region EditorSession
export type EditorWasmSession = GraphWasmSession & {
  syncFromSceneJson(json: string): void;
  setText(text: string): void;
  text(): string;
  caret(): number;
  anchor(): number;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number, buttons: number): void;
  pointerUpScreen(sx: number, sy: number, buttons: number): void;
  wheelScrollScreen(deltaY: number): void;
  insertText(text: string): void;
  backspace(): void;
  deleteForward(): void;
  selectAll(): void;
  replaceSelection(text: string): void;
  selectionText(): string;
  setCanvasThemeJson(json: string): void;
  hoverTokenRangeJson(): string;
  setHoverRange(start: number, end: number): void;
  cameraJson(): string;
  moveLeft(extend: boolean): void;
  moveRight(extend: boolean): void;
  moveUp(extend: boolean): void;
  moveDown(extend: boolean): void;
  moveLineStart(extend: boolean): void;
  moveLineEnd(extend: boolean): void;
  tabInsertText(): string;
  setSelectionRange(anchor: number, caret: number): void;
  selectSpanAt(offset: number): void;
  selectSpanAtScreen(sx: number, sy: number): void;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  caretWorldJson(): string;
  worldToScreenJson(wx: number, wy: number): string;
  setSelectionOccurrencesJson(json: string): void;
  setExtraCaretsJson(json: string): void;
  setCaretVisible(visible: boolean): void;
};

type EditorSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly EditorSession: new () => EditorWasmSession;
};

let editorSessionPromise: Promise<EditorSessionModule> | null = null;

export async function createEditorSession(): Promise<EditorWasmSession> {
  if (!editorSessionPromise) {
    editorSessionPromise = import("@semio-tech/framework-editor-rs/pkg/framework_editor.js").then(async (mod) => {
      await mod.default();
      return mod as EditorSessionModule;
    });
  }
  const mod = await editorSessionPromise;
  return new mod.EditorSession();
}
//#endregion EditorSession

//#region RasterSession
export type RasterWasmSession = {
  gpuReady(): boolean;
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  setCamera(x: number, y: number, zoom: number): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number): void;
  pointerUpScreen(sx: number, sy: number): void;
  syncDocumentJson(json: string): void;
  uploadLayerImage(layerId: string, bytes: Uint8Array): void;
  uploadRasterImageKey(key: string, bytes: Uint8Array): void;
  setActiveTool(tool: string): void;
  setBrushSize(size: number): void;
  setBrushOpacity(opacity: number): void;
  setHoveredIdSilent(id?: string | null): void;
  setSelectionIdsJson(json: string): void;
  setCanvasThemeJson(json: string): void;
  cameraJson(): string;
  setViewMode(mode: string, layerId?: string | null): void;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  marqueeHitsJson(queryJson: string): string;
  navigatorFitCameraJson(viewportW: number, viewportH: number): string;
  navigatorViewportOverlayJson(contentCameraJson: string, contentViewportJson: string): string;
  free(): void;
};

type RasterSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly RasterSession: new () => RasterWasmSession;
};

let rasterSessionPromise: Promise<RasterSessionModule> | null = null;

export async function createRasterSession(): Promise<RasterWasmSession> {
  if (!rasterSessionPromise) {
    rasterSessionPromise = import("@semio-tech/raster-rs/pkg/raster.js").then(async (mod) => {
      await mod.default();
      return mod as RasterSessionModule;
    });
  }
  const mod = await rasterSessionPromise;
  return new mod.RasterSession();
}
//#endregion RasterSession

//#region MapSession
export type MapWasmSession = {
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  setCamera(x: number, y: number, zoom: number): void;
  cameraJson(): string;
  cameraLimitsJson(): string;
  fitWorldCamera(): void;
  reclampCamera(): void;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number): void;
  pointerUpScreen(sx: number, sy: number): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  syncMapJson(json: string): void;
  uploadTile(z: number, x: number, y: number, bytes: Uint8Array): void;
  uploadVectorTile(z: number, x: number, y: number, bytes: Uint8Array): void;
  visibleTilesJson(): string;
  visibleVectorTilesJson(): string;
  setRenderMode(mode: string): void;
  setVectorStyle(style: string): void;
  setLodMode(mode: string): void;
  setLayerVisibilityJson(json: string): void;
  setLayerStrokeScaleJson(json: string): void;
  setSelectionJson(json: string): void;
  setHoverJson(json: string): void;
  featuresInRectJson(x0: number, y0: number, x1: number, y1: number, crossing: boolean): string;
  featuresInPolygonJson(pointsJson: string, crossing: boolean): string;
  hitTestFeatureJson(sx: number, sy: number): string;
  featureScreenJson(kind: string, id: string): string;
  positionScreenJson(id: string): string;
  currentLodJson(): string;
  setMapThemeJson(json: string): void;
  gpuReady(): boolean;
  free(): void;
};

type MapSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly MapSession: new () => MapWasmSession;
};

let mapSessionPromise: Promise<MapSessionModule> | null = null;

export async function createMapSession(): Promise<MapWasmSession> {
  if (!mapSessionPromise) {
    mapSessionPromise = import("@semio-tech/gis-2d-rs/pkg/gis_2d.js").then(async (mod) => {
      await mod.default();
      return mod as MapSessionModule;
    });
  }
  const mod = await mapSessionPromise;
  return new mod.MapSession();
}

export type Puzzle2dBoardWasmSession = {
  attach_canvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  parseFixtureJson(json: string): boolean;
  syncDescriptorJson(json: string): void;
  setKindCatalogsJson(json: string): void;
  setCamera(x: number, y: number, zoom: number): void;
  setSelectionIdsJson(json: string): void;
  setCanvasThemeJson(json: string): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  drainEventsJson(): string;
  cameraJson(): string;
  gpuReady(): boolean;
  setHoveredIdSilent?(id?: string | null): void;
  setActiveTool?(label: string): void;
  setSelectionOptions?(method: string, mode: string, selectNodes: boolean, selectEdges: boolean, selectHandles: boolean): void;
  setGridSnapEnabled?(enabled: boolean): void;
  setGridFactor?(v: number): void;
  setSuggestionOffset?(distance: number): void;
  setBrushKindWeights?(json: string): void;
  setHandleLinkCompatJson?(json: string): void;
  setAutomaticLod?(enabled: boolean): void;
  setForcedDrawLodLabel?(label: string): void;
  setSelectionIdsJsonSilent?(json: string): void;
  setCameraSilent?(x: number, y: number, zoom: number): void;
  pointerLeaveScreen?(alt: boolean): void;
  pickTargetsAtScreenJson?(sx: number, sy: number): string;
  deleteSelection?(): void;
  cancelAreaSelect?(): boolean;
  brushCycleCandidate?(forward: boolean): void;
  setFixtureDropPreviewJson?(json: string): void;
  clearFixtureDropPreview?(): void;
  defersDescriptorSyncFromJs?(): boolean;
  isDraggingAreaSelect?(): boolean;
  free(): void;
};

type Puzzle2dBoardSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly BoardSession: new () => Puzzle2dBoardWasmSession;
};

let puzzle2dBoardSessionPromise: Promise<Puzzle2dBoardSessionModule> | null = null;

export async function createPuzzle2dBoardSession(): Promise<Puzzle2dBoardWasmSession> {
  if (!puzzle2dBoardSessionPromise) {
    puzzle2dBoardSessionPromise = import("@semio-tech/puzzle-2d-rs/pkg/puzzle_2d.js").then(async (mod) => {
      await mod.default();
      return mod as Puzzle2dBoardSessionModule;
    });
  }
  const mod = await puzzle2dBoardSessionPromise;
  return new mod.BoardSession();
}
//#endregion MapSession

//#region SceneHelpers
export function isFlowGraphScene(capabilitiesJson?: string): boolean {
  if (!capabilitiesJson) return false;
  try {
    const caps = JSON.parse(capabilitiesJson) as { readonly engine?: string; readonly spotlight?: boolean; readonly noteEdit?: boolean };
    return caps.engine === "flow" || caps.spotlight === true || caps.noteEdit === true;
  } catch {
    return false;
  }
}
//#endregion SceneHelpers
//#endregion 🔖wasm-session-loader

//#region 🔖ui-search-find

//#region UISearch
export type UISearchItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly icon?: ReactNode;
  readonly category?: string;
  readonly onSelect: () => void;
};

export function UISearch({
  items,
  open,
  onOpenChange,
  placeholder = shellLabel("ui.search.placeholder"),
  emptyMessage = shellLabel("ui.search.empty"),
}: {
  readonly items: readonly UISearchItem[];
  readonly open: boolean;
  readonly onOpenChange: (open: boolean) => void;
  readonly placeholder?: string;
  readonly emptyMessage?: string;
}) {
  const [query, setQuery] = useState("");
  const fuse = useMemo(
    () =>
      new Fuse(items, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [items],
  );
  const results = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return items.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UISearchItem>);
  }, [fuse, items, query]);
  const grouped = useMemo(() => {
    const groups: Record<string, FuseResult<UISearchItem>[]> = {};
    for (const result of results) {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    }
    return groups;
  }, [results]);
  const handleSelect = useCallback(
    (item: UISearchItem) => {
      onOpenChange(false);
      setQuery("");
      item.onSelect();
    },
    [onOpenChange],
  );

  return (
    <CommandDialog title={shellLabel("ui.search.title")} description={shellLabel("ui.search.description")} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.search.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()} onSelect={() => handleSelect(result.item)}>
                <div className="flex items-center gap-single">
                  {result.item.icon}
                  <div className="flex flex-col">
                    <span>{result.item.label}</span>
                    {result.item.description ? <span className="text-xs text-muted-foreground">{result.item.description}</span> : null}
                  </div>
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
}
//#endregion UISearch

//#region UIFind
export type UIFindItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly category?: string;
};

export type UIFindContextValue = {
  readonly findItems: readonly UIFindItem[];
  readonly setFindItems: (items: readonly UIFindItem[]) => void;
  readonly setOnFindItem: (callback: ((itemId: string) => void) | undefined) => void;
  readonly triggerFindItem: (itemId: string) => void;
};

const UIFindContext = createContext<UIFindContextValue | null>(null);

function areFindItemsShallowEqual(previousItems: readonly UIFindItem[], nextItems: readonly UIFindItem[]): boolean {
  if (previousItems === nextItems) return true;
  if (previousItems.length !== nextItems.length) return false;
  for (let index = 0; index < nextItems.length; index += 1) {
    const previous = previousItems[index];
    const next = nextItems[index];
    if (!previous || !next || previous.id !== next.id || previous.label !== next.label || previous.description !== next.description || previous.category !== next.category) {
      return false;
    }
  }
  return true;
}

export function UIFindProvider({ children }: { readonly children: ReactNode }) {
  const [findItems, setFindItemsState] = useState<readonly UIFindItem[]>([]);
  const onFindItemCallbackRef = useRef<((itemId: string) => void) | undefined>(undefined);
  const setFindItems = useCallback((items: readonly UIFindItem[]) => {
    setFindItemsState((previousItems) => (areFindItemsShallowEqual(previousItems, items) ? previousItems : items));
  }, []);
  const setOnFindItem = useCallback((callback: ((itemId: string) => void) | undefined) => {
    onFindItemCallbackRef.current = callback;
  }, []);
  const triggerFindItem = useCallback((itemId: string) => {
    onFindItemCallbackRef.current?.(itemId);
  }, []);
  const contextValue = useMemo(() => ({ findItems, setFindItems, setOnFindItem, triggerFindItem }), [findItems, setFindItems, setOnFindItem, triggerFindItem]);
  return <UIFindContext.Provider value={contextValue}>{children}</UIFindContext.Provider>;
}

export function useUIFind(): UIFindContextValue {
  const context = useContext(UIFindContext);
  if (!context) throw new Error("useUIFind must be used within UIFindProvider");
  return context;
}

export function useUIFindSafe(): UIFindContextValue | null {
  return useContext(UIFindContext);
}

export function UIFind({ open, onOpenChange, placeholder = shellLabel("ui.find.placeholder"), emptyMessage = shellLabel("ui.find.empty") }: { readonly open: boolean; readonly onOpenChange: (open: boolean) => void; readonly placeholder?: string; readonly emptyMessage?: string }) {
  const [query, setQuery] = useState("");
  const findContext = useContext(UIFindContext);
  const findItems = findContext?.findItems ?? [];
  const triggerFindItem = findContext?.triggerFindItem;
  const fuse = useMemo(
    () =>
      new Fuse(findItems, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [findItems],
  );
  const results = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return findItems.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UIFindItem>);
  }, [findItems, fuse, query]);
  const grouped = useMemo(() => {
    const groups: Record<string, FuseResult<UIFindItem>[]> = {};
    for (const result of results) {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    }
    return groups;
  }, [results]);
  const handleSelect = useCallback(
    (item: UIFindItem) => {
      onOpenChange(false);
      setQuery("");
      triggerFindItem?.(item.id);
    },
    [onOpenChange, triggerFindItem],
  );

  if (!findContext) return null;

  return (
    <CommandDialog title={shellLabel("ui.find.title")} description={shellLabel("ui.find.description")} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.find.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()} onSelect={() => handleSelect(result.item)}>
                <div className="flex flex-col">
                  <span>{result.item.label}</span>
                  {result.item.description ? <span className="text-xs text-muted-foreground">{result.item.description}</span> : null}
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
}
//#endregion UIFind
//#endregion 🔖ui-search-find

//#region 🔖sync-attach-card

type SyncAttachCardProps = {
  readonly activeUri: string | null;
  readonly cardKind: SyncCardKind | null;
  readonly draftPath: string;
  readonly syncTools: readonly FrameworkSyncToolLeaf[];
  readonly onAction: (action: ActionDescriptor) => void;
  readonly onDraftPathChange: (value: string) => void;
  readonly onClose: () => void;
  readonly onAttach: (uri: string) => void;
  readonly onDetach: () => void;
};

function SyncAttachCard({ activeUri, cardKind, draftPath, syncTools, onAction, onDraftPathChange, onClose, onAttach, onDetach }: SyncAttachCardProps): ReactElement {
  const open = cardKind != null;
  const placeholder =
    cardKind === "remote" ? "127.0.0.1:8787/demo" : cardKind === "folder" ? "/absolute/project/folder" : "/absolute/document.json";

  const attachFromDraft = () => {
    if (!cardKind || !draftPath.trim()) return;
    if (cardKind === "remote") {
      const slash = draftPath.indexOf("/");
      const hostPort = slash > 0 ? draftPath.slice(0, slash) : draftPath;
      const documentId = slash > 0 ? draftPath.slice(slash + 1) : "document";
      onAttach(buildRemoteBackboneUri(hostPort, documentId));
      return;
    }
    onAttach(cardKind === "folder" ? buildFolderBackboneUri(draftPath) : buildFileBackboneUri(draftPath));
  };

  return (
    <Popover
      open={open}
      onOpenChange={(nextOpen) => {
        if (!nextOpen) onClose();
      }}
    >
      <PopoverAnchor asChild>
        <div>
          <ToolTree tools={syncTools as readonly ToolNode[]} onAction={onAction} />
        </div>
      </PopoverAnchor>
      {open ? (
        <PopoverContent side="top" align="center" className="w-80 space-y-3 p-3">
          <div className="space-y-1">
            <p className="text-sm font-medium capitalize">{cardKind} backbone</p>
            {activeUri ? <p className="break-all text-xs text-muted-foreground">{activeUri}</p> : null}
          </div>
          <Input value={draftPath} placeholder={placeholder} onChange={(event) => onDraftPathChange(event.target.value)} />
          <div className="flex items-center gap-2">
            <Button type="button" onClick={attachFromDraft}>
              Attach
            </Button>
            {activeUri ? (
              <Button type="button" onClick={onDetach}>
                Detach
              </Button>
            ) : null}
          </div>
        </PopoverContent>
      ) : null}
    </Popover>
  );
}
//#endregion 🔖sync-attach-card

//#region 🔖tool-tree

type ToolTreeProps = {
  readonly tools: readonly ToolNode[];
  readonly onAction: (action: ActionDescriptor) => void;
  readonly id?: string;
  /** @emoji 🎀 `up` stacks a new ribbon line above the base row per pressed collection (window toolbar); `inline` keeps the horizontal drill-down (footer). */
  readonly direction?: RibbonDirection;
};

function resolveLeafAction(node: ToolLeaf | Extract<ToolNode, { readonly kind: "button" | "toggle" }>): ActionDescriptor | null {
  if ("onPress" in node && node.onPress) return node.onPress;
  if ("onChange" in node && node.onChange) return node.onChange;
  if (node.kind === "button" || node.kind === "toggle") {
    if (!node.action || !node.controllerId) return null;
    return { controllerId: node.controllerId, action: node.action, args: node.args as Record<string, unknown> | undefined };
  }
  return null;
}

function toolIcon(iconId: string): IconName {
  return iconId in ICONS ? (iconId as IconName) : "circle";
}

/** @emoji 🔢 Sorts toolbar nodes by `order`. */
export function sortToolNodes(nodes: readonly ToolNode[]): ToolNode[] {
  return [...nodes].sort((left, right) => (left.order ?? 0) - (right.order ?? 0));
}

//#region 🗂️ToolCategoryGrouping

const TOOL_CATEGORY_ORDER: readonly ToolCategory[] = ["selection", "tools", "actions", "history", "sync"];

/** @emoji 🪟 Categories that are scoped to whatever window/pane the user is interacting with — selecting or editing content varies per window, so these live in each window's own bottom-left panel. */
const WINDOW_TOOL_CATEGORIES: readonly ToolCategory[] = ["selection", "tools"];

/** @emoji 🦶 Categories that apply regardless of which window has focus — undoing, running document-wide actions, or syncing are mode-wide, so they live once in the shared footer instead of being duplicated into every window. */
const FOOTER_TOOL_CATEGORIES: readonly ToolCategory[] = ["actions", "history"];

const TOOL_CATEGORY_ICON_ID: Readonly<Record<ToolCategory, string>> = {
  selection: "mouse-pointer",
  tools: "wrench",
  actions: "sparkles",
  history: "undo",
  sync: "cloud",
};

function toolNodeCategory(node: ToolNode): ToolCategory {
  if (node.kind === "separator") return "tools";
  if (node.category) return node.category;
  return node.kind === "button" ? "actions" : "tools";
}

/** @emoji 🗂️ Buckets top-level tool nodes into the given categories (default: all) so activating a category expands the panel with another line, matching {@link buildToolbarRibbonSegments}'s one-active-group-per-level picker. A category with a single already-meaningful collection is used as-is instead of being re-wrapped in a synthetic one, avoiding a redundant picker level with a duplicate-looking label (e.g. a lone "Selection" collection nested under a "Selection" category chip). Separators default to `tools` (mirrors Rust `ToolNode::category()`), so dividers between same-category runs survive; dividers that only separated different categories become redundant once those categories are separate picker lines. */
export function groupToolNodesByCategory(nodes: readonly ToolNode[], categories: readonly ToolCategory[] = TOOL_CATEGORY_ORDER): ToolNode[] {
  const buckets = new Map<ToolCategory, ToolNode[]>();
  for (const node of nodes) {
    const category = toolNodeCategory(node);
    if (!categories.includes(category)) continue;
    const bucket = buckets.get(category) ?? [];
    bucket.push(node);
    buckets.set(category, bucket);
  }
  return categories
    .filter((category) => hasInteractiveToolNodes(buckets.get(category)))
    .map((category, order) => {
      const bucket = buckets.get(category)!;
      if (bucket.length === 1 && bucket[0].kind === "collection") return { ...bucket[0], order };
      return { id: category, kind: "collection" as const, iconId: TOOL_CATEGORY_ICON_ID[category], text: category, order, category, children: bucket };
    });
}

/** @emoji 🦶 Deduplicates tool nodes by id across every window's tool set (mode-wide tools are attached identically to each window kind when a plugin doesn't differentiate per window), for a single shared footer entry per tool. */
export function dedupeToolNodesById(nodeLists: readonly (readonly ToolNode[])[]): ToolNode[] {
  const seen = new Map<string, ToolNode>();
  for (const nodes of nodeLists) {
    for (const node of nodes) {
      if (!seen.has(node.id)) seen.set(node.id, node);
    }
  }
  return [...seen.values()];
}

//#endregion 🗂️ToolCategoryGrouping

function isInteractiveToolNode(node: ToolNode): boolean {
  if (node.kind === "separator") return false;
  if (node.kind === "collection") return hasInteractiveToolNodes(node.children);
  return true;
}

function hasInteractiveToolNodes(nodes?: readonly ToolNode[]): boolean {
  return Boolean(nodes?.some((node) => isInteractiveToolNode(node)));
}

function hasInteractiveToolLeaves(items: readonly ToolLeaf[]): boolean {
  return items.some((node) => node.kind !== "separator");
}

type ToolCollectionNode = Extract<ToolNode, { readonly kind: "collection" }>;

export type ToolbarRibbonSegment =
  | { readonly kind: "picker"; readonly collections: readonly ToolCollectionNode[]; readonly depth: number }
  | { readonly kind: "tools"; readonly items: readonly ToolLeaf[]; readonly depth: number };

/** @emoji 🎀 Builds drill-down ribbon segments from a toolbar tree and active collection path; `depth` marks how many collections were drilled into to reach a segment. Collections never auto-activate: a level only recurses when `path[depth]` names one of its enabled collections, so at most one group per level is active and an unresolved level simply shows its picker. */
export function buildToolbarRibbonSegments(nodes: readonly ToolNode[], path: readonly string[], depth = 0): ToolbarRibbonSegment[] {
  const sorted = sortToolNodes(nodes);
  const collections = sorted.filter((node): node is ToolCollectionNode => node.kind === "collection" && !node.disabled);
  const looseLeaves = sorted.filter((node): node is ToolLeaf => node.kind !== "collection");
  const segments: ToolbarRibbonSegment[] = [];

  if (collections.length > 0) segments.push({ kind: "picker", collections, depth });
  if (hasInteractiveToolLeaves(looseLeaves)) segments.push({ kind: "tools", items: looseLeaves, depth });
  if (collections.length === 0) return segments;

  const activeId = path[depth];
  const active = activeId ? collections.find((node) => node.id === activeId) : undefined;
  if (!active) return segments;
  return [...segments, ...buildToolbarRibbonSegments(active.children, path, depth + 1)];
}

/** @emoji 🎀 Validates an active-group path against the current tool tree: keeps each entry only while it still names an enabled collection at that level, truncating at the first miss rather than substituting a default. */
export function reconcileToolPath(nodes: readonly ToolNode[], path: readonly string[]): readonly string[] {
  let current = nodes;
  const reconciled: string[] = [];

  for (const collectionId of path) {
    const collections = sortToolNodes(current).filter((node): node is ToolCollectionNode => node.kind === "collection" && !node.disabled);
    const active = collections.find((node) => node.id === collectionId);
    if (!active) break;
    reconciled.push(collectionId);
    current = active.children;
  }

  return reconciled;
}

function ToolToolbarItems({ items, onAction }: { readonly items: readonly ToolLeaf[]; readonly onAction: (action: ActionDescriptor) => void }): ReactElement {
  const sorted = useMemo(() => sortToolNodes(items) as ToolLeaf[], [items]);
  const nodes = useMemo(() => {
    const rendered: ReactElement[] = [];
    let buttonRun: ToolLeaf[] = [];
    let toggleRun: ToolLeaf[] = [];

    const flushButtons = () => {
      if (buttonRun.length === 0) return;
      const run = buttonRun;
      buttonRun = [];
      rendered.push(
        <ToolbarItem key={`buttons-${run.map((entry) => entry.id).join("-")}`}>
          <ButtonGroup>
            {run.map((entry) => {
              const action = resolveLeafAction(entry);
              if (!action) return null;
              return (
                <ButtonGroupItem
                  key={entry.id}
                  id={entry.id}
                  aria-label={entry.title ?? entry.label ?? entry.id}
                  title={entry.title ?? entry.label}
                  disabled={entry.disabled}
                  onClick={() => onAction(action)}
                  icon={<Icon icon={toolIcon(entry.iconId)} size="small" />}
                  text={entry.text ?? entry.label}
                />
              );
            })}
          </ButtonGroup>
        </ToolbarItem>,
      );
    };

    const flushToggles = () => {
      if (toggleRun.length === 0) return;
      const run = toggleRun;
      toggleRun = [];
      rendered.push(
        <ToolbarItem key={`toggles-${run.map((entry) => entry.id).join("-")}`}>
          <ToggleGroup
            kind="multiple"
            value={run.filter((entry) => entry.pressed).map((entry) => entry.id)}
            onValueChange={(values) => {
              for (const entry of run) {
                const action = resolveLeafAction(entry);
                if (!action) continue;
                const pressed = values.includes(entry.id);
                if ((entry.pressed ?? false) !== pressed) onAction(action);
              }
            }}
            items={run.map((entry) => ({
              value: entry.id,
              id: entry.id,
              icon: <Icon icon={toolIcon(entry.iconId)} size="small" />,
              text: entry.text ?? entry.label,
            }))}
          />
        </ToolbarItem>,
      );
    };

    const flushRuns = () => {
      flushButtons();
      flushToggles();
    };

    for (const item of sorted) {
      if (item.kind === "separator") {
        flushRuns();
        rendered.push(<ToolbarDivider key={item.id} />);
        continue;
      }
      if (item.kind === "toggle") {
        flushButtons();
        toggleRun.push(item);
        continue;
      }
      flushToggles();
      buttonRun.push(item);
    }
    flushRuns();
    return rendered;
  }, [onAction, sorted]);

  return <ToolbarGroup>{nodes}</ToolbarGroup>;
}

function toolbarRibbonSegmentKey(segment: ToolbarRibbonSegment, index: number): string {
  return segment.kind === "picker" ? `picker-${segment.depth}-${segment.collections.map((entry) => entry.id).join("-")}` : `tools-${index}-${segment.items.map((entry) => entry.id).join("-")}`;
}

export function ToolTree({ tools, onAction, id = "ui.toolbar", direction = "inline" }: ToolTreeProps): ReactElement | null {
  const [activePath, setActivePath] = useState<readonly string[]>([]);

  useEffect(() => {
    setActivePath((previousPath) => reconcileToolPath(tools, previousPath));
  }, [tools]);

  const segments = useMemo(() => buildToolbarRibbonSegments(tools, activePath), [tools, activePath]);

  if (!hasInteractiveToolNodes(tools)) return null;

  const renderSegment = (segment: ToolbarRibbonSegment): ReactNode =>
    segment.kind === "picker" ? (
      <ToolbarItem>
        <ToggleGroup
          kind="single"
          value={activePath[segment.depth] ?? ""}
          onValueChange={(value) => {
            setActivePath(value ? reconcileToolPath(tools, [...activePath.slice(0, segment.depth), value]) : activePath.slice(0, segment.depth));
          }}
          items={segment.collections.map((entry) => ({
            value: entry.id,
            id: `${id}.group.${entry.id}`,
            icon: <Icon icon={toolIcon(entry.iconId)} size="small" />,
            text: entry.text ?? entry.label,
          }))}
        />
      </ToolbarItem>
    ) : (
      <ToolToolbarItems items={segment.items} onAction={onAction} />
    );

  const rows: RibbonRow[] =
    direction === "inline"
      ? segments.map((segment, index) => ({ key: toolbarRibbonSegmentKey(segment, index), content: renderSegment(segment) }))
      : Array.from(
          segments.reduce((byDepth, segment, index) => {
            const zones = byDepth.get(segment.depth) ?? [];
            zones.push(<ToolbarZone key={toolbarRibbonSegmentKey(segment, index)}>{renderSegment(segment)}</ToolbarZone>);
            byDepth.set(segment.depth, zones);
            return byDepth;
          }, new Map<number, ReactElement[]>()),
        )
          .sort(([left], [right]) => left - right)
          .map(([depth, content]) => ({ key: `row-${depth}`, content }));

  return (
    <UiChromeLabelPolicyProvider policy="always">
      <Ribbon id={id} direction={direction} rows={rows} />
    </UiChromeLabelPolicyProvider>
  );
}
//#endregion 🔖tool-tree

//#region 🔖os-chrome-panels

//#region DisplayPanel
export type DisplayHostApi = {
  readonly windowKinds: readonly { readonly id: string; readonly label: string }[];
  readonly namedLayouts: readonly NamedLayout[];
  readonly userLayouts: readonly NamedLayout[];
  readonly saveCurrentLayout: (label: string) => void;
  readonly applyNamedLayout: (layoutId: string) => void;
  readonly deleteUserLayout: (layoutId: string) => void;
};

const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID = "framework.display.windows";
const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID = "framework.display.layout";
const FRAMEWORK_SETTINGS_GENERAL_TAB_ID = "framework.settings.general";
const FRAMEWORK_SETTINGS_THEME_TAB_ID = "framework.settings.theme";

let displayLayoutSaveLabel = "";

function groupNamedLayoutsToTreeItems(layouts: readonly NamedLayout[], onApply: (layoutId: string) => void, onDeleteUser?: (layoutId: string) => void): TreeDataItem[] {
  const root: TreeDataItem[] = [];
  const folderByKey = new Map<string, TreeDataItem>();
  const layoutLeaf = (entry: NamedLayout): TreeDataItem => ({
    id: `framework.display.layout.${entry.id}`,
    label: entry.label,
    onClick: () => onApply(entry.id),
    ...(entry.origin === "user" && onDeleteUser
      ? {
          actions: [
            {
              id: `framework.display.delete.${entry.id}`,
              icon: <Icon icon="trash-2" size="small" />,
              onClick: () => onDeleteUser(entry.id),
            },
          ],
        }
      : {}),
  });
  for (const entry of layouts) {
    if (!entry.groupPath?.length) {
      root.push(layoutLeaf(entry));
      continue;
    }
    let siblings = root;
    let pathKey = "";
    for (let index = 0; index < entry.groupPath.length; index += 1) {
      const segment = entry.groupPath[index]!;
      pathKey = pathKey ? `${pathKey}/${segment}` : segment;
      let folder = folderByKey.get(pathKey);
      if (!folder) {
        folder = { id: `framework.display.layout.group.${pathKey}`, label: segment, defaultOpen: false, items: [] };
        folderByKey.set(pathKey, folder);
        siblings.push(folder);
      }
      const folderItems = folder.items ?? (folder.items = []);
      if (index === entry.groupPath.length - 1) folder.items = [...folderItems, layoutLeaf(entry)];
      else siblings = folderItems;
    }
  }
  return root;
}

function buildDisplayWindowsTree(host: DisplayHostApi): TreePanelConfig {
  return {
    dragAndDropController: windowTemplatePaletteTreeDragController(),
    sections: host.windowKinds.length
      ? host.windowKinds.map((kind) => ({
          id: `framework.display.windows.${kind.id}`,
          label: kind.label,
          defaultOpen: false,
          items: [
            {
              id: `framework.display.windows.${kind.id}.kind`,
              label: kind.label,
              dragData: {
                [COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId: kind.id }),
              },
            },
          ],
        }))
      : [{ id: "framework.display.windows.empty", items: [{ id: "empty", label: "—" }] }],
  };
}

function buildDisplayLayoutTree(host: DisplayHostApi): TreePanelConfig {
  const builtinLayouts = host.namedLayouts.filter((entry) => entry.origin === "builtin");
  const userLayouts = host.userLayouts;
  const builtinItems = groupNamedLayoutsToTreeItems(builtinLayouts, (layoutId) => host.applyNamedLayout(layoutId));
  const userItems = userLayouts.length
    ? [
        {
          id: "framework.display.layout.group.saved",
          label: shellLabel("ui.display.saved"),
          defaultOpen: false,
          items: groupNamedLayoutsToTreeItems(
            userLayouts,
            (layoutId) => host.applyNamedLayout(layoutId),
            (layoutId) => host.deleteUserLayout(layoutId),
          ),
        },
      ]
    : [];
  return {
    sections: [
      {
        id: "framework.display.layout.save",
        label: shellLabel("ui.display.saveLayout"),
        defaultOpen: false,
        items: [
          {
            id: "framework.display.layout.save.label",
            label: shellLabel("ui.common.name"),
            control: (
              <Input
                id="framework.display.save-label"
                defaultValue={displayLayoutSaveLabel}
                onChange={(event) => {
                  displayLayoutSaveLabel = event.target.value;
                }}
                placeholder={shellLabel("ui.display.saveLayoutPlaceholder")}
              />
            ),
          },
          {
            id: "framework.display.layout.save.action",
            label: shellLabel("ui.common.save"),
            control: (
              <Button
                id="framework.display.save"
                size="sm"
                text={shellLabel("ui.display.saveCurrentLayout")}
                disabled={!displayLayoutSaveLabel.trim()}
                onClick={() => {
                  const label = displayLayoutSaveLabel.trim();
                  if (!label) return;
                  host.saveCurrentLayout(label);
                  displayLayoutSaveLabel = "";
                }}
              />
            ),
          },
        ],
      },
      {
        id: "framework.display.layout.list",
        label: shellLabel("ui.display.layouts"),
        defaultOpen: true,
        items: [...builtinItems, ...userItems],
      },
    ],
  };
}

export function createFrameworkDisplayPanelTabs(getHost: () => DisplayHostApi | null): PanelTabNode[] {
  return [
    {
      kind: "leaf",
      id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID,
      icon: shellTabIcon("framework.display.windows"),
      name: shellLabel("ui.display.tab.windows"),
      order: -100,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayWindowsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    },
    {
      kind: "leaf",
      id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID,
      icon: shellTabIcon("framework.display.layout"),
      name: shellLabel("ui.display.tab.layout"),
      order: -99,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayLayoutTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    },
  ];
}
//#endregion DisplayPanel

//#region SettingsPanel
export type SettingsHostApi = {
  readonly appId?: string;
  readonly appLabel?: string;
  readonly controllerId?: string;
  readonly pluginId?: string;
  readonly compact: boolean;
  readonly setCompact: (compact: boolean) => void;
  readonly expertise: string;
  readonly setExpertise: (expertise: string) => void;
  readonly appearance: string;
  readonly setAppearance: (appearance: string) => void;
  readonly layout: UiChromeLayout;
  readonly setLayout: (layout: UiChromeLayout) => void;
  readonly mobileActive: boolean;
  readonly locale: UiLocale;
  readonly setLocale: (locale: UiLocale) => void;
  readonly terminology: string;
  readonly setTerminology: (id: string) => void;
  readonly terminologies: readonly string[];
  readonly theme: UiTheme;
  readonly themeId: string;
  readonly themeDirty: boolean;
  readonly themes: readonly UiTheme[];
  readonly setThemeId: (id: string) => void;
  readonly setThemeColor: (key: string, hex: string) => void;
  readonly setThemeSpacing: (key: string, value: string) => void;
  readonly setThemeFontStack: (key: string, value: string) => void;
  readonly setThemeStroke: (key: string, value: number | number[]) => void;
  readonly setThemeRadius: (key: string, value: number) => void;
  readonly setThemeOpacity: (key: string, value: number) => void;
  readonly setThemeMetric: (section: string, key: string, value: number | number[]) => void;
  readonly setThemeAppearancePaint: (appearance: ThemeAppearanceName, group: ThemePaletteGroup, key: string, hex: string, alpha?: number) => void;
  readonly saveTheme: (label: string) => void;
  readonly deleteTheme: (id: string) => void;
  readonly resetTheme: () => void;
  readonly exportTheme: () => void;
  readonly importTheme: () => void;
};

function buildSettingsGeneralTree(host: SettingsHostApi): TreePanelConfig {
  return {
    sections: [
      ...(host.appId || host.appLabel || host.controllerId || host.pluginId
        ? [
            {
              id: "framework.settings.app",
              label: shellLabel("ui.settings.tab.app"),
              defaultOpen: true,
              items: [
                ...(host.appLabel ? [{ id: "framework.settings.app.label", label: `${shellLabel("ui.settings.app.name")}: ${host.appLabel}` }] : []),
                ...(host.appId ? [{ id: "framework.settings.app.id", label: `${shellLabel("ui.settings.app.id")}: ${host.appId}` }] : []),
                ...(host.controllerId ? [{ id: "framework.settings.app.controller", label: `${shellLabel("ui.settings.app.controller")}: ${host.controllerId}` }] : []),
                ...(host.pluginId ? [{ id: "framework.settings.app.plugin", label: `${shellLabel("ui.settings.app.plugin")}: ${host.pluginId}` }] : []),
              ],
            },
          ]
        : []),
      {
        id: "framework.settings.general",
        label: shellLabel("ui.settings.tab.general"),
        defaultOpen: true,
        items: [
          {
            id: "framework.settings.appearance",
            label: shellLabel("ui.settings.tab.appearance"),
            control: (
              <Select value={host.appearance} onValueChange={(value) => host.setAppearance(value)}>
                <SelectTrigger id="framework.settings.appearance" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="system">{shellLabel("ui.settings.appearance.system")}</SelectItem>
                  <SelectItem value="light">{shellLabel("ui.settings.appearance.light")}</SelectItem>
                  <SelectItem value="dark">{shellLabel("ui.settings.appearance.dark")}</SelectItem>
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.layout",
            label: shellLabel("ui.settings.tab.layout"),
            control: host.mobileActive ? (
              <span className="text-sm text-muted-foreground">{shellLabel("settings.layout.mobile")}</span>
            ) : (
              <Select value={host.layout} onValueChange={(value) => host.setLayout(value === "tablet" ? "tablet" : "desktop")}>
                <SelectTrigger id="framework.settings.layout" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="desktop">{shellLabel("settings.layout.desktop")}</SelectItem>
                  <SelectItem value="tablet">{shellLabel("settings.layout.tablet")}</SelectItem>
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.compact",
            label: shellLabel("settings.compact"),
            control: <input id="framework.settings.compact" type="checkbox" checked={host.compact} onChange={(event) => host.setCompact(event.target.checked)} />,
          },
          {
            id: "framework.settings.expertise",
            label: shellLabel("ui.settings.tab.expertise"),
            control: (
              <Select value={host.expertise} onValueChange={(value) => host.setExpertise(value)}>
                <SelectTrigger id="framework.settings.expertise" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="beginner">{shellLabel("settings.expertise.beginner")}</SelectItem>
                  <SelectItem value="normal">{shellLabel("settings.expertise.normal")}</SelectItem>
                  <SelectItem value="expert">{shellLabel("settings.expertise.expert")}</SelectItem>
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.language",
            label: shellLabel("ui.settings.tab.language"),
            control: (
              <Select value={host.locale} onValueChange={(value) => host.setLocale(value === "de" ? "de" : "en")}>
                <SelectTrigger id="framework.settings.language" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="en">{shellLabel("ui.settings.language.en")}</SelectItem>
                  <SelectItem value="de">{shellLabel("ui.settings.language.de")}</SelectItem>
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.terminology",
            label: shellLabel("ui.settings.tab.terminology"),
            control: (
              <Select value={host.terminology} onValueChange={(value) => host.setTerminology(value)}>
                <SelectTrigger id="framework.settings.terminology" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {host.terminologies.map((id) => (
                    <SelectItem key={id} value={id}>
                      {shellTerminologyLabel(id)}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          },
        ],
      },
    ],
  };
}

let themeSaveLabel = "";

function rgba8ToHex(rgba: readonly [number, number, number, number]): string {
  const [r, g, b] = rgba;
  return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
}

function themeColorInputRow(id: string, label: string, hex: string, onChange: (hex: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <input id={id} type="color" className="h-small w-16 shrink-0 rounded border border-border bg-background" value={hex} onChange={(event) => onChange(event.target.value)} />,
  };
}

function themeTextInputRow(id: string, label: string, value: string, onCommit: (value: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <Input id={id} defaultValue={value} onBlur={(event) => onCommit(event.target.value)} className="h-small w-32" />,
  };
}

function themeNumberInputRow(id: string, label: string, value: number | number[], onCommit: (value: number | number[]) => void): TreeDataItem {
  const text = Array.isArray(value) ? value.join(", ") : String(value);
  return {
    id,
    label,
    control: (
      <Input
        id={id}
        defaultValue={text}
        onBlur={(event) => {
          const raw = event.target.value.trim();
          if (raw.includes(",")) {
            const parts = raw
              .split(",")
              .map((part) => Number.parseFloat(part.trim()))
              .filter((n) => !Number.isNaN(n));
            if (parts.length) onCommit(parts);
            return;
          }
          const n = Number.parseFloat(raw);
          if (!Number.isNaN(n)) onCommit(n);
        }}
        className="h-small w-32"
      />
    ),
  };
}

const THEME_PALETTE_GROUP_LABEL_KEYS = {
  board: "ui.settings.theme.group.board",
  map: "ui.settings.theme.group.map",
  canvas: "ui.settings.theme.group.canvas",
  chrome: "ui.settings.theme.group.chrome",
} as const satisfies Record<ThemePaletteGroup, UiTranslationKey>;

function buildThemeAppearanceGroupItems(host: SettingsHostApi, appearance: ThemeAppearanceName, group: ThemePaletteGroup): TreeDataItem[] {
  const refs = host.theme.appearances[appearance][group];
  const resolved = resolveThemeAppearancePalettes(host.theme, appearance)[group];
  return Object.keys(refs)
    .sort()
    .map((paintKey) => {
      const rgba = resolved[paintKey] ?? [0, 0, 0, 255];
      const hex = rgba8ToHex(rgba);
      const alpha = rgba[3] / 255;
      return {
        id: `framework.settings.theme.appearances.${appearance}.${group}.${paintKey}`,
        label: paintKey,
        control: (
          <div className="flex w-full items-center gap-1">
            <input
              type="color"
              className="h-small w-10 shrink-0 rounded border border-border bg-background"
              value={hex}
              onChange={(event) => host.setThemeAppearancePaint(appearance, group, paintKey, event.target.value, alpha)}
            />
            <Input
              id={`framework.settings.theme.appearances.${appearance}.${group}.${paintKey}.alpha`}
              defaultValue={alpha.toFixed(2)}
              onBlur={(event) => {
                const nextAlpha = Number.parseFloat(event.target.value);
                if (!Number.isNaN(nextAlpha)) host.setThemeAppearancePaint(appearance, group, paintKey, hex, Math.min(1, Math.max(0, nextAlpha)));
              }}
              className="h-small w-14 shrink-0"
            />
          </div>
        ),
      } satisfies TreeDataItem;
    });
}

function buildSettingsThemeTree(host: SettingsHostApi): TreePanelConfig {
  const colorItems = Object.keys(host.theme.colors)
    .sort()
    .map((key) => themeColorInputRow(`framework.settings.theme.colors.${key}`, key, host.theme.colors[key]!, (hex) => host.setThemeColor(key, hex)));

  const spacingItems = Object.keys(host.theme.spacing)
    .sort()
    .map((key) => themeTextInputRow(`framework.settings.theme.spacing.${key}`, key, host.theme.spacing[key]!, (value) => host.setThemeSpacing(key, value)));

  const fontItems = Object.keys(host.theme.fontStacks)
    .sort()
    .map((key) => themeTextInputRow(`framework.settings.theme.fonts.${key}`, key, host.theme.fontStacks[key]!, (value) => host.setThemeFontStack(key, value)));

  const strokeItems = Object.keys(host.theme.strokes)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.strokes.${key}`, key, host.theme.strokes[key]!, (value) => host.setThemeStroke(key, value)));

  const radiusItems = Object.keys(host.theme.radii)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.radii.${key}`, key, host.theme.radii[key]!, (value) => host.setThemeRadius(key, typeof value === "number" ? value : value[0]!)));

  const opacityItems = Object.keys(host.theme.opacities)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.opacities.${key}`, key, host.theme.opacities[key]!, (value) => host.setThemeOpacity(key, typeof value === "number" ? value : value[0]!)));

  const metricSections = Object.keys(host.theme.metrics)
    .sort()
    .map(
      (section): TreeDataItem => ({
        id: `framework.settings.theme.metrics.${section}`,
        label: section,
        defaultOpen: false,
        items: Object.keys(host.theme.metrics[section]!)
          .sort()
          .map((key) => themeNumberInputRow(`framework.settings.theme.metrics.${section}.${key}`, key, host.theme.metrics[section]![key]!, (value) => host.setThemeMetric(section, key, value))),
      }),
    );

  const appearanceGroups: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome"];
  const appearanceItems: TreeDataItem[] = (["light", "dark"] as const).map((appearance) => ({
    id: `framework.settings.theme.appearances.${appearance}`,
    label: shellLabel(appearance === "light" ? "ui.settings.theme.appearance.light" : "ui.settings.theme.appearance.dark"),
    defaultOpen: false,
    items: appearanceGroups.map((group) => ({
      id: `framework.settings.theme.appearances.${appearance}.${group}`,
      label: shellLabel(THEME_PALETTE_GROUP_LABEL_KEYS[group]),
      defaultOpen: false,
      items: buildThemeAppearanceGroupItems(host, appearance, group),
    })),
  }));

  return {
    sections: [
      {
        id: "framework.settings.theme.select",
        label: `${shellLabel("ui.settings.theme.select")}${host.themeDirty ? ` (${shellLabel("ui.settings.theme.dirty")})` : ""}`,
        defaultOpen: true,
        items: [
          {
            id: "framework.settings.theme.select.picker",
            label: shellLabel("ui.settings.theme.select"),
            control: (
              <Select value={host.themeId} onValueChange={(value) => host.setThemeId(value)}>
                <SelectTrigger id="framework.settings.theme.select" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {host.themes.map((theme) => (
                    <SelectItem key={theme.id} value={theme.id}>
                      {theme.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.theme.save.label",
            label: shellLabel("ui.common.name"),
            control: <Input id="framework.settings.theme.save-label" defaultValue={themeSaveLabel} onChange={(event) => (themeSaveLabel = event.target.value)} placeholder={shellLabel("ui.settings.theme.savePlaceholder")} className="h-small w-32" />,
          },
          {
            id: "framework.settings.theme.save.action",
            label: shellLabel("ui.settings.theme.save"),
            control: (
              <Button
                id="framework.settings.theme.save"
                size="sm"
                text={shellLabel("ui.settings.theme.save")}
                disabled={!themeSaveLabel.trim()}
                onClick={() => {
                  const label = themeSaveLabel.trim();
                  if (!label) return;
                  host.saveTheme(label);
                  themeSaveLabel = "";
                }}
              />
            ),
          },
          {
            id: "framework.settings.theme.reset.action",
            label: shellLabel("ui.settings.theme.reset"),
            control: <Button id="framework.settings.theme.reset" size="sm" text={shellLabel("ui.settings.theme.reset")} disabled={!host.themeDirty && host.themeId === "semio"} onClick={() => host.resetTheme()} />,
          },
          {
            id: "framework.settings.theme.export.action",
            label: shellLabel("ui.settings.theme.export"),
            control: <Button id="framework.settings.theme.export" size="sm" text={shellLabel("ui.settings.theme.export")} onClick={() => host.exportTheme()} />,
          },
          {
            id: "framework.settings.theme.import.action",
            label: shellLabel("ui.settings.theme.import"),
            control: <Button id="framework.settings.theme.import" size="sm" text={shellLabel("ui.settings.theme.import")} onClick={() => host.importTheme()} />,
          },
          ...(host.themeId.startsWith("custom.")
            ? [
                {
                  id: "framework.settings.theme.delete.action",
                  label: shellLabel("ui.settings.theme.delete"),
                  control: <Button id="framework.settings.theme.delete" size="sm" text={shellLabel("ui.settings.theme.delete")} onClick={() => host.deleteTheme(host.themeId)} />,
                },
              ]
            : []),
        ],
      },
      { id: "framework.settings.theme.colors", label: shellLabel("ui.settings.theme.colors"), defaultOpen: false, items: colorItems },
      { id: "framework.settings.theme.spacing", label: shellLabel("ui.settings.theme.spacing"), defaultOpen: false, items: spacingItems },
      { id: "framework.settings.theme.fonts", label: shellLabel("ui.settings.theme.fonts"), defaultOpen: false, items: fontItems },
      { id: "framework.settings.theme.strokes", label: shellLabel("ui.settings.theme.strokes"), defaultOpen: false, items: strokeItems },
      { id: "framework.settings.theme.radii", label: shellLabel("ui.settings.theme.radii"), defaultOpen: false, items: radiusItems },
      { id: "framework.settings.theme.opacities", label: shellLabel("ui.settings.theme.opacities"), defaultOpen: false, items: opacityItems },
      { id: "framework.settings.theme.metrics", label: shellLabel("ui.settings.theme.metrics"), defaultOpen: false, items: metricSections },
      { id: "framework.settings.theme.appearances", label: shellLabel("ui.settings.theme.appearances"), defaultOpen: false, items: appearanceItems },
    ],
  };
}

export function createFrameworkSettingsPanelTabs(getHost: () => SettingsHostApi | null): PanelTabNode[] {
  return [
    {
      kind: "leaf",
      id: FRAMEWORK_SETTINGS_GENERAL_TAB_ID,
      icon: shellTabIcon("framework.settings.general"),
      name: shellLabel("ui.panelToggle.settings"),
      order: -98,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildSettingsGeneralTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
        },
      },
    },
    {
      kind: "leaf",
      id: FRAMEWORK_SETTINGS_THEME_TAB_ID,
      icon: shellTabIcon("paintbrush"),
      name: shellLabel("ui.settings.tab.theme"),
      order: -97,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildSettingsThemeTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
        },
      },
    },
  ];
}

export function useNamedLayoutHost(options: {
  readonly appId: string;
  readonly windowKinds: readonly { readonly id: string; readonly label: string }[];
  readonly builtinLayouts: readonly NamedLayout[];
  readonly currentLayout: WindowLayout | undefined;
  readonly onApplyLayout: (layout: WindowLayout) => void;
  readonly namedLayoutStore: { getSnapshot: () => readonly NamedLayout[]; save: (layout: NamedLayout) => void; remove: (layoutId: string) => void; subscribe: (listener: () => void) => () => void };
}): DisplayHostApi {
  const userLayouts = useSyncExternalStore(
    (listener) => options.namedLayoutStore.subscribe(listener),
    () => options.namedLayoutStore.getSnapshot(),
    () => options.namedLayoutStore.getSnapshot(),
  );
  return useMemo(
    (): DisplayHostApi => ({
      windowKinds: options.windowKinds,
      namedLayouts: options.builtinLayouts,
      userLayouts,
      saveCurrentLayout: (label) => {
        if (!options.currentLayout) return;
        const id = `user-${Date.now()}`;
        options.namedLayoutStore.save(createNamedLayout(id, label, options.currentLayout, "user"));
      },
      applyNamedLayout: (layoutId) => {
        const layout = [...options.builtinLayouts, ...userLayouts].find((entry) => entry.id === layoutId);
        if (layout) options.onApplyLayout(layout.layout);
      },
      deleteUserLayout: (layoutId) => options.namedLayoutStore.remove(layoutId),
    }),
    [options, userLayouts],
  );
}
//#endregion SettingsPanel
//#endregion 🔖os-chrome-panels
