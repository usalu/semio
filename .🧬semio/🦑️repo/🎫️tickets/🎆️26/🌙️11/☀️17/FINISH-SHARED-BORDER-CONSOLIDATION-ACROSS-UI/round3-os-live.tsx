import * as React from "react";
import { createRoot } from "react-dom/client";
import { useState, useEffect, type ComponentType } from "react";
import { createFrameworkSettingsPanelTab } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx";
import { createFrameworkDisplayPanelTabs } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🖥️display-panel/🟦️.tsx";
import { createFrameworkMarketplacePanelTab } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🏪️marketplace-panel/🟦️.tsx";
import { InterpretedUiNode } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx";
import { UiDocumentStore, builtNodeToSnapshot } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { NoticeStack, NoticeCell, FloatingPane } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧱️notice-bands/🟦️.tsx";
import { TransientNotice } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🍞️transient-notice/🟦️.tsx";
import { DocumentCloseDialog } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🚪️close-guard/🟦️.tsx";
import { UISearch, UIFind, UIFindProvider, useUIFind } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔎️ShellSearch/🟦️.tsx";
import { UiMobileProvider, UiI18nProvider, createShellI18nInstance, builtinUiThemes, builtinUiDrivers, DEFAULT_UI_DRIVER, Popover, PopoverContent, PopoverTrigger, Input, Tree, Search, SharedBorderRow, SharedBorderRowCell, Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter, App, Button, Canvas, ContextMenuController, Footer, HorizontalWindows, Layout, Mode, Navbar, PanelChromeTabBar, PanelTabBar, Window, createIconComponent, navbarFillItem, singleTreeLeaf, uiDataLabel, Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut, type ContextMenuItem, type PanelTabNode } from "@semio-tech/ui-react";

import { moreParts } from "./os-chrome-more.tsx";
import { r3Parts } from "./round3-os-parts.tsx";

const Pane = ({ label }: { label: string }) => <div className="flex h-full items-center justify-center bg-transparent"><span>{label}</span></div>;
const Home = createIconComponent("home");
const Info = createIconComponent("info");
const Layers = createIconComponent("layers");
const Settings = createIconComponent("settings");
const User = createIconComponent("user");
const leaf = (id: string, icon: ComponentType<{ size?: number }>, name: string, order: number, content: React.ReactNode) => singleTreeLeaf({ id, icon, name, order, tree: { sections: [{ id: `${id}.s`, label: "", items: [{ id: `${id}.i`, label: "", control: <div className="p-double">{content}</div> }] }] } });
const workbench = [{ kind: "branch" as const, id: "workbench", icon: Layers, name: "Workbench", children: [leaf("explorer", Layers, "Explorer", 0, "Explorer content"), leaf("search", Info, "Search", 1, "Search content")] }];

const quadLayout = { kind: "row", children: [{ kind: "column", size: 50, children: [{ kind: "stack", children: [{ kind: "window", id: "a" }], activeId: "a" }, { kind: "stack", children: [{ kind: "window", id: "b" }], activeId: "b" }] }, { kind: "column", size: 50, children: [{ kind: "stack", children: [{ kind: "window", id: "c" }], activeId: "c" }, { kind: "stack", children: [{ kind: "window", id: "d" }], activeId: "d" }] }] } as never;
const win = (id: string) => ({ id, title: uiDataLabel(id), iconId: "app-window", children: <Pane label={id} /> });
function ShellLayout({ bottom = false, many = false }: { bottom?: boolean; many?: boolean }) {
  const [tl, setTl] = useState(true);
  const [tlPath, setTlPath] = useState<readonly string[]>(["workbench", "explorer"]);
  const [br, setBr] = useState(bottom);
  const [brPath, setBrPath] = useState<readonly string[]>(["settings"]);
  const manyTabs = Array.from({ length: 14 }, (_, i) => leaf(`t${i}`, Layers, `Tab number ${i}`, i, `Content ${i}`));
  const tlTabs = many ? manyTabs : workbench;
  const brTabs = [leaf("settings", Settings, "Settings", 0, "Settings content"), leaf("log", Info, "Log", 1, "Log content")];
  return (
    <div style={{ height: "100vh" }}>
      <Layout
        navbar={<Navbar items={[{ content: <Home size={20} />, key: "home" }, { content: <span className="font-bold">Application</span>, key: "title" }, navbarFillItem("fill"), { key: "tl", content: <PanelChromeTabBar anchor="top-left" tabs={tlTabs} visible={tl} onVisibleChange={setTl} activeTabPath={many ? ["t0"] : tlPath} onActiveTabPathChange={setTlPath} /> }, { content: <User size={20} />, key: "user" }]} />}
        footer={<Footer items={[{ content: "Ready", key: "status" }, navbarFillItem("ffill"), { key: "br", content: <PanelChromeTabBar anchor="bottom-right" tabs={brTabs} visible={br} onVisibleChange={setBr} activeTabPath={brPath} onActiveTabPathChange={setBrPath} /> }]} />}
        panels={{
          "top-left": { visible: tl, onVisibleChange: setTl, activeTabPath: many ? ["t0"] : tlPath, onActiveTabPathChange: setTlPath, tabBarHost: "chrome", size: 250, onSizeChange: () => {}, tabs: tlTabs },
          "bottom-right": { visible: br, onVisibleChange: setBr, activeTabPath: brPath, onActiveTabPathChange: setBrPath, tabBarHost: "chrome", size: 300, onSizeChange: () => {}, tabs: brTabs },
          "top-middle": { visible: true, activeTabPath: ["console"], size: 360, onSizeChange: () => {}, tabs: [leaf("console", Info, "Console", 0, "Console output")] },
          "top-right": { visible: true, activeTabPath: ["props"], size: 300, onSizeChange: () => {}, tabs: [leaf("props", Settings, "Properties", 0, "Properties")] },
          ...(bottom ? { "bottom-left": { visible: true, activeTabPath: ["out"], size: 260, onSizeChange: () => {}, tabs: [leaf("out", Info, "Output", 0, "Output")] } } : {}),
        }}
        canvas={<Mode windows={["a", "b", "c", "d"].map(win)} layout={quadLayout} activeWindowId="a" />}
      />
    </div>
  );
}

const ctxItems: ContextMenuItem[] = [
  { id: "rename", label: uiDataLabel("Rename"), icon: "pencil", shortcut: "F2" },
  { id: "visible", label: uiDataLabel("Visible"), icon: "eye", checked: true },
  { id: "sep", label: uiDataLabel(""), separator: true },
  { id: "transform", label: uiDataLabel("Transform"), icon: "move", children: [{ id: "t.move", label: uiDataLabel("Move"), icon: "move" }, { id: "t.rot", label: uiDataLabel("Rotate"), icon: "rotate-cw" }, { id: "t.scale", label: uiDataLabel("Scale"), icon: "maximize-2", disabled: true }] },
  { id: "view", label: uiDataLabel("View"), icon: "eye", children: [{ id: "v.fit", label: uiDataLabel("Fit"), icon: "scan" }] },
  { id: "sep2", label: uiDataLabel(""), separator: true },
  { id: "delete", label: uiDataLabel("Delete"), icon: "trash-2", destructive: true, shortcut: "Del" },
];


const noop = () => {};
const themes = builtinUiThemes();
const drivers = builtinUiDrivers();
const settingsHost = {
  appId: "s.demo", appLabel: "Demo", controllerId: "demo", pluginId: "s.demo", driverId: DEFAULT_UI_DRIVER.id, driver: DEFAULT_UI_DRIVER, driverDirty: false, drivers,
  setDriverId: noop, setDriverField: noop, saveDriver: noop, deleteDriver: noop, driverSaveLabel: "", setDriverSaveLabel: noop,
  appearance: "dark", setAppearance: noop, layout: "default", setLayout: noop, mobileActive: false, onResetDock: noop, locale: "en", setLocale: noop,
  terminology: "native", setTerminology: noop, terminologies: ["native", "architecture"],
  theme: themes[0], themeId: themes[0]!.id, themeDirty: false, themes, setThemeId: noop, setThemeColor: noop, setThemeSpacing: noop, setThemeFontStack: noop, setThemeStroke: noop, setThemeRadius: noop, setThemeOpacity: noop, setThemeMetric: noop, setThemeAppearancePaint: noop,
  saveTheme: noop, deleteTheme: noop, resetTheme: noop, exportTheme: noop, importTheme: noop, themeSaveLabel: "", setThemeSaveLabel: noop,
  controlKeybindings: new Map([["undo", "mod+z"], ["redo", "mod+shift+z"], ["ui.fullscreen.toggle", "mod+ctrl+f"]]), controlKeybindingLabel: () => null, keybindingCaptureControlId: null, setKeybindingCaptureControlId: noop, setKeybindingOverride: noop, resetKeybindingOverride: noop,
  locks: {}, mergePolicy: "Normal", setMergePolicy: noop,
} as never;
const defaultsHost = { rows: [{ kind: "text", label: "Text", current: "none", options: [{ value: "none", label: "None" }, { value: "a", label: "Editor" }] }], apps: [], onChange: noop, locale: "en" } as never;
const conflictsHost = { conflicts: [{ id: "c1", kind: { kind: "quarantined", envelopes: [] }, status: "open", messages: [], actors: [], timestamp: { actor: 1, physical_ms: 0, logical: 0 } }, { id: "c2", kind: { kind: "quarantined", envelopes: [] }, status: "open", messages: [], actors: [], timestamp: { actor: 1, physical_ms: 0, logical: 0 } }], locale: "en", selectedConflictId: "c1", kindLabel: () => "Quarantined", messageText: () => "A change was refused", onSelect: noop, onResolve: noop, currentDocumentText: "text" } as never;
const displayHost = { windowKinds: [{ id: "w1", label: "Puzzle 3D", iconId: "box", surfaceKind: "world-3d" }, { id: "w2", label: "Table", iconId: "table" }], namedLayouts: [{ id: "l1", label: "Default / Quad", origin: "builtin" }, { id: "l2", label: "Default / Split", origin: "builtin" }], userLayouts: [{ id: "u1", label: "Mine", origin: "user" }], saveCurrentLayout: noop, applyNamedLayout: noop, deleteUserLayout: noop, layoutSaveLabel: "", setLayoutSaveLabel: noop } as never;
const marketHost = { plugins: [{ pluginId: "s.cad", label: "CAD", version: "1.0", status: "ready", sourceId: "hub", canUninstall: true }, { pluginId: "s.flow", label: "Flow", version: "0.3", status: "available", sourceId: "hub", canUninstall: false }], extensions: [{ extensionId: "e1", label: "Ext", version: "1", extendsHost: "s.cad", enabled: true, status: "ready" }], installPlugin: noop, uninstallPlugin: noop, reloadPlugin: noop, installExtensionFromUrl: noop, installExtensionFromFile: noop, uninstallExtension: noop, setExtensionEnabled: noop } as never;

function PanelShell({ tabs, path, anchor = "top-right" }: { tabs: PanelTabNode[]; path: readonly string[]; anchor?: "top-left" | "top-right" | "bottom-right" | "bottom-left" }) {
  const [visible, setVisible] = useState(true);
  const [p, setP] = useState<readonly string[]>(path);
  const vertical = anchor.startsWith("top") ? "top" : "bottom";
  const Bar = <PanelChromeTabBar anchor={anchor} tabs={tabs} visible={visible} onVisibleChange={setVisible} activeTabPath={p} onActiveTabPathChange={setP} />;
  const bar = [navbarFillItem("fill"), { key: "bar", content: Bar }];
  return (
    <div style={{ height: "100vh" }}>
      <Layout
        navbar={<Navbar items={vertical === "top" ? [{ content: <Home size={20} />, key: "home" }, ...bar] : [{ content: <Home size={20} />, key: "home" }]} />}
        footer={<Footer items={vertical === "bottom" ? [{ content: "Ready", key: "s" }, ...bar] : [{ content: "Ready", key: "s" }]} />}
        panels={{ [anchor]: { visible, onVisibleChange: setVisible, activeTabPath: p, onActiveTabPathChange: setP, tabBarHost: "chrome", size: 420, onSizeChange: noop, tabs } }}
        canvas={<Mode windows={["a", "b"].map(win)} layout={{ kind: "row", children: [{ kind: "stack", children: [{ kind: "window", id: "a" }], activeId: "a" }, { kind: "stack", children: [{ kind: "window", id: "b" }], activeId: "b" }] } as never} activeWindowId="a" />}
      />
    </div>
  );
}
const settingsTab = createFrameworkSettingsPanelTab(() => settingsHost, () => defaultsHost, () => conflictsHost);
type Open = { measures?: boolean; utility?: boolean; actions?: boolean };
const richWindow = (id: string, open: Open) => ({
  id, title: uiDataLabel(id), iconId: "app-window", children: <Pane label={id} />,
  measures: <div className="p-double">Measures</div>, measuresFolded: !open.measures, utilityBar: <div className="p-double">Utilities</div>, utilityBarFolded: !open.utility,
  actionPane: <div className="p-double">Actions tree</div>, actionsFolded: !open.actions,
  engagement: { sessionActive: true, options: [{ id: "e1", label: "Draw", icon: "pencil" }, { id: "e2", label: "Move", icon: "move", pressed: true }] },
  search: { sessionActive: false, input: { value: "ca", placeholder: uiDataLabel("Search"), onChange: noop }, possibles: [{ id: "p1", label: "Capsule", detail: "Box" }, { id: "p2", label: "Cabinet", detail: "Furniture" }, { id: "p3", label: "Cap", detail: "Part" }] },
}) as never;
function Rich({ open, many }: { open: Open; many?: boolean }) {
  return <div style={{ height: 600 }}><Mode mobile={innerWidth < 768} windows={(many ? ["a", "b"] : ["a"]).map(id => richWindow(id, open))} layout={many ? { kind: "row", children: [{ kind: "stack", children: [{ kind: "window", id: "a" }], activeId: "a" }, { kind: "stack", children: [{ kind: "window", id: "b" }], activeId: "b" }] } as never : undefined} activeWindowId="a" /></div>;
}
function CtxSub({ pick }: { pick: string }) {
  useEffect(() => {
    const t = setTimeout(() => {
      const item = [...document.querySelectorAll<HTMLElement>('[role="menuitem"]')].find(e => e.textContent?.includes(pick));
      item?.focus();
      item?.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowRight", bubbles: true }));
      item?.dispatchEvent(new PointerEvent("pointermove", { bubbles: true }));
    }, 400);
    return () => clearTimeout(t);
  }, [pick]);
  return <ContextMenuController title={uiDataLabel("Actions")} open position={{ x: 200, y: 120 }} items={ctxItems} onOpenChange={noop} />;
}
const searchItems = [{ id: "i1", label: "Open file", description: "Open a document", category: "File", onSelect: noop }, { id: "i2", label: "Save", description: "Save the document", category: "File", onSelect: noop }, { id: "i3", label: "Toggle theme", category: "View", onSelect: noop }];
function FindSeed() { const f = useUIFind(); useEffect(() => { f.setFindItems(searchItems); }, []); return null; }


function MobileLayout({ settings }: { settings?: boolean }) {
  const [visible, setVisible] = useState(true);
  const mobileTabs = settings ? [settingsTab] : [...workbench, leaf("settings", Settings, "Settings", 1, "Settings content")];
  const [path, setPath] = useState<readonly string[]>(settings ? ["framework.settings", "framework.settings.general"] : ["workbench", "explorer"]);
  return (
    <div style={{ height: "100vh" }}>
      <Layout
        mobile
        mobilePanel={{ visible, tabs: mobileTabs as never, activeTabPath: path, onActiveTabPathChange: setPath }}
        navbar={<Navbar items={[{ content: <Home size={20} />, key: "home" }, { content: <span className="font-bold">App</span>, key: "title" }, navbarFillItem("fill"), { key: "t", content: <button onClick={() => setVisible(v => !v)}>Panel</button> }]} showFullscreenToggle={false} />}
        footer={<Footer items={[{ content: "Ready", key: "s" }]} />}
        canvas={<Mode windows={["a", "b"].map(win)} layout={{ kind: "stack", activeId: "a", children: [{ kind: "window", id: "a" }, { kind: "window", id: "b" }] } as never} activeWindowId="a" />}
      />
    </div>
  );
}

const CID = "plugin-chrome";
const LEAF = { kind: "leaf", width: "hug", height: "hug" } as never;
const COLUMN = { kind: "stack", axis: "vertical", gap: "sm", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false } as never;
const ROW = { kind: "stack", axis: "horizontal", gap: "sm", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false } as never;
const STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" } as never;
const A11Y = { label: null, description: null, live: "off", shortcut: null, hidden: false } as never;
const bind = (trigger: string, name: string) => ({ trigger, action: { scope: CID, name, version: 1 }, args: null, capability: null }) as never;
const N = (key: string, component: unknown, o: { layout?: unknown; bindings?: unknown[]; children?: unknown[]; disabled?: boolean } = {}): never => ({ key, component, layout: o.layout ?? LEAF, style: STYLE, activity: "idle", disabled: o.disabled ?? false, accessibility: A11Y, bindings: o.bindings ?? [], menu: null, children: o.children ?? [] }) as never;
const box = (role: string, label: string | null, key: string, children: unknown[], layout: unknown = COLUMN) => N(key, { type: "container", role, label, description: null, required: null, error: null, defaultOpen: true, dropOverlay: null }, { layout, children });
const btn = (key: string, icon: string, label: string) => N(key, { type: "button", icon, label }, { bindings: [bind("activate", key)] });
const tgl = (key: string, icon: string, text: string, on: boolean) => N(key, { type: "toggle", appearance: "button", on, icon, text }, { bindings: [bind("change", key)] });
const field = (key: string, label: string, child: unknown) => box("field", label, key, [child]);
const input = (key: string, value: string) => N(key, { type: "input", kind: "text", value, placeholder: null, commit: "blur", min: null, max: null, step: null, accept: null, precision: null, snaps: [] }, { bindings: [bind("commit", key)] });
const stepper = (key: string, value: number) => N(key, { type: "numberStepper", value, min: 0, max: 100, step: 1, unit: "mm", precision: 1 }, { bindings: [bind("change", key)] });
const sel = (key: string, value: string) => N(key, { type: "select", value, items: [{ value: "a", label: "Union" }, { value: "b", label: "Cut" }, { value: "c", label: "Intersect" }], placeholder: null }, { bindings: [bind("change", key)] });
const sld = (key: string, value: number) => N(key, { type: "slider", value, min: 0, max: 1, step: 0.05, unit: "", snaps: [] }, { bindings: [bind("change", key)] });
const kv = (key: string) => N(key, { type: "keyValueList", entries: [{ label: "Volume", value: "1.2 m3" }, { label: "Faces", value: "42" }, { label: "Edges", value: "96" }] });
const tItem = (key: string, label: string, icon: string, children: unknown[] = []) => N(key, { type: "treeItem", label, description: null, icon, defaultOpen: true, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, rowActions: [], target: null, inlineToolbar: null, detail: null }, { bindings: [bind("activate", key)], children });
const tSec = (key: string, label: string, children: unknown[]) => N(key, { type: "treeSection", label, defaultOpen: true, headerToolbar: null, window: null }, { children });
const tree = (key: string, children: unknown[]) => N(key, { type: "tree", presentation: "standard", interactionDomain: null }, { layout: COLUMN, children });
const prog = (key: string) => N(key, { type: "progress", value: 0.4, label: "Solving" });
const tblRow = (key: string, cells: string[]) => N(key, { type: "tableRow", cells, rowActions: [], target: null });
const tbl = (key: string) => N(key, { type: "table", label: "Results", columns: ["Node", "Time"], rowLabel: null, columnLabel: null, actionsLabel: null, window: null, columnWindow: null }, { children: [tblRow(`${key}-r1`, ["Load", "12 ms"]), tblRow(`${key}-r2`, ["Solve", "340 ms"])] });
const toolbar = (prefix: string) => box("toolbar", null, `${prefix}-toolbar`, [btn(`${prefix}-new`, "plus", "New"), btn(`${prefix}-open`, "folder-open", "Open"), tgl(`${prefix}-snap`, "magnet", "Snap", true), tgl(`${prefix}-grid`, "grid-3x3", "Grid", false), btn(`${prefix}-run`, "play", "Run")], ROW);
const cadPanel = box("plain", null, "cad-panel", [toolbar("cad"), tree("cad-tree", [tSec("cad-features", "Features", [tItem("f1", "Sketch 1", "pencil"), tItem("f2", "Extrude 1", "box", [tItem("f2a", "Profile", "square")]), tItem("f3", "Fillet 1", "circle")])]), box("section", "Properties", "cad-props", [field("cad-name", "Name", input("cad-name-i", "Extrude 1")), field("cad-op", "Operation", sel("cad-op-s", "a")), field("cad-depth", "Depth", stepper("cad-depth-s", 12)), sld("cad-op-sl", 0.6)]), box("section", "Inspector", "cad-insp", [kv("cad-kv")])]);
const flowPanel = box("plain", null, "flow-panel", [toolbar("flow"), tree("flow-tree", [tSec("flow-cat", "Neurons", [tItem("n1", "math.add", "plus"), tItem("n2", "math.mul", "x"), tItem("n3", "slider", "sliders-horizontal")])]), box("group", "Selected", "flow-sel", [field("fl-a", "a", input("fl-a-i", "3")), field("fl-b", "b", stepper("fl-b-s", 4)), sld("fl-sl", 0.3)])]);
const dagPanel = box("plain", null, "dag-panel", [toolbar("dag"), box("section", "Run", "dag-run", [prog("dag-prog"), btn("dag-cancel", "x", "Cancel")]), box("section", "Results", "dag-res", [tbl("dag-tbl")]), box("form", "Node", "dag-node", [field("dn-n", "Label", input("dn-n-i", "Composite")), field("dn-k", "Kind", sel("dn-k-s", "b"))])]);
function Plugin({ root, id, canvas }: { root: never; id: string; canvas?: boolean }) {
  const store = React.useMemo(() => { const st = new UiDocumentStore(id); st.loadSnapshot(builtNodeToSnapshot(id, root)); return st; }, [id, root]);
  const mobile = innerWidth < 768;
  const tabs = [singleTreeLeaf({ id: "p", icon: Settings, name: "Plugin", order: 0, tree: { sections: [{ id: "p.s", label: "", items: [{ id: "p.i", label: "", control: <InterpretedUiNode store={store} onAction={noop} onIntent={noop} /> }] }] } })];
  const canvasNode = <Mode mobile={mobile} windows={[{ id: "a", title: uiDataLabel("Canvas"), iconId: "app-window", children: canvas ? <div data-border-probe-art="" className="h-full w-full" style={{ background: "repeating-linear-gradient(45deg,#234,#234 10px,#345 10px,#345 20px)" }} /> : <Pane label="Canvas" /> }]} activeWindowId="a" />;
  return (
    <div style={{ height: "100vh" }}>
      <Layout
        mobile={mobile}
        mobilePanel={mobile ? { visible: true, tabs: tabs as never, activeTabPath: ["p"], onActiveTabPathChange: noop } : undefined}
        navbar={<Navbar items={[{ content: <Home size={20} />, key: "home" }]} showFullscreenToggle={!mobile} />}
        footer={<Footer items={[{ content: "Ready", key: "s" }]} />}
        panels={mobile ? undefined : { "top-right": { visible: true, activeTabPath: ["p"], size: 360, onSizeChange: noop, tabs } }}
        canvas={canvasNode}
      />
    </div>
  );
}

const parts: Record<string, () => React.ReactElement> = {
  layout: () => <ShellLayout />,
  "layout-bottom": () => <ShellLayout bottom />,
  "plugin-cad": () => <Plugin root={cadPanel} id="s.cad" canvas />,
  "plugin-flow": () => <Plugin root={flowPanel} id="s.flow" canvas />,
  "plugin-dag": () => <Plugin root={dagPanel} id="s.dag" canvas />,
  "layout-mobile": () => <MobileLayout />,
  "layout-mobile-settings": () => <MobileLayout settings />,
  "layout-many": () => <ShellLayout many />,
  quad: () => <div style={{ height: 500 }}><Mode windows={["a", "b", "c", "d"].map(win)} layout={quadLayout} activeWindowId="a" /></div>,
  "mode-stack": () => <div style={{ height: 400 }}><Mode windows={["design", "review", "notes"].map(win)} layout={{ kind: "stack", activeId: "design", children: ["design", "review", "notes"].map(id => ({ kind: "window", id })) } as never} activeWindowId="design" /></div>,
  "mode-mixed": () => <div style={{ height: 500 }}><Mode windows={["a", "b", "c", "d"].map(win)} layout={{ kind: "row", children: [{ kind: "stack", size: 35, activeId: "a", children: [{ kind: "window", id: "a" }, { kind: "window", id: "b" }] }, { kind: "column", size: 65, children: [{ kind: "stack", size: 70, activeId: "c", children: [{ kind: "window", id: "c" }] }, { kind: "stack", size: 30, activeId: "d", children: [{ kind: "window", id: "d" }] }] }] } as never} activeWindowId="a" /></div>,
  app: () => <div style={{ height: 400 }}><App modes={[{ id: "design", label: uiDataLabel("Design"), children: <Mode windows={["l", "r"].map(win)} activeWindowId="l" /> }, { id: "review", label: uiDataLabel("Review"), children: <Mode windows={["p"].map(win)} activeWindowId="p" /> }]} activeModeId="design" onActiveModeChange={() => {}} /></div>,
  context: () => <ContextMenuController title={uiDataLabel("Actions")} open position={{ x: 200, y: 120 }} items={ctxItems} onOpenChange={() => {}} />,
  command: () => <div style={{ padding: 40 }}><Command className="border w-96"><CommandInput placeholder="Search types..." /><CommandList><CommandEmpty>None</CommandEmpty><CommandGroup heading="Types"><CommandItem value="capsule"><span>Capsule</span></CommandItem><CommandItem value="base"><span>Base</span></CommandItem></CommandGroup><CommandGroup heading="Settings"><CommandItem value="profile"><span>Profile</span><CommandShortcut>P</CommandShortcut></CommandItem></CommandGroup></CommandList></Command></div>,

  "settings-general": () => <PanelShell tabs={[settingsTab]} path={["framework.settings", "framework.settings.general"]} />,
  "settings-theme": () => <PanelShell tabs={[settingsTab]} path={["framework.settings", "framework.settings.theme"]} />,
  "settings-keys": () => <PanelShell tabs={[settingsTab]} path={["framework.settings", "framework.settings.keybindings"]} />,
  "settings-defaults": () => <PanelShell tabs={[settingsTab]} path={["framework.settings", "framework.settings.defaults"]} />,
  "settings-conflicts": () => <PanelShell tabs={[settingsTab]} path={["framework.settings", "framework.settings.conflicts"]} />,
  display: () => <PanelShell tabs={createFrameworkDisplayPanelTabs(() => displayHost)} path={["framework.display.windows"]} />,
  marketplace: () => <PanelShell tabs={[createFrameworkMarketplacePanelTab(() => marketHost)]} path={["framework.marketplace"]} />,
  "panel-bottom": () => <PanelShell tabs={[settingsTab]} path={["framework.settings", "framework.settings.general"]} anchor="bottom-right" />,
  "window-rich-folded": () => <Rich open={{}} />,
  "window-rich-measures": () => <Rich open={{ measures: true }} />,
  "window-rich-utility": () => <Rich open={{ utility: true }} />,
  "window-rich-actions": () => <Rich open={{ actions: true }} />,
  "window-rich-open": () => <Rich open={{ measures: true, utility: true, actions: true }} />,
  "window-rich-many": () => <Rich open={{ measures: true, utility: true, actions: true }} many />,
  "context-sub": () => <CtxSub pick="Transform" />,
  "notice-stack": () => <div style={{ height: 500, position: "relative" }}><NoticeStack className="left-0 top-0 w-80"><NoticeCell>Background sync paused</NoticeCell><NoticeCell>Plugin install: 40%</NoticeCell></NoticeStack></div>,
  "notice-toast": () => <div style={{ height: 500, position: "relative" }}><TransientNotice notice={{ message: "Saved", kind: "info" }} closeLabel="Close" onClose={noop} /></div>,
  "notice-toast-error": () => <div style={{ height: 500, position: "relative" }}><TransientNotice notice={{ message: "Save failed", kind: "error", code: "x" }} closeLabel="Close" onClose={noop} /></div>,
  "notice-pane": () => <div style={{ height: 500, position: "relative" }}><FloatingPane className="right-0 top-0 w-96 h-72"><div className="p-double">Workspace pane</div></FloatingPane></div>,
  "dialog-close": () => <DocumentCloseDialog name="Notes" locale="en" failure={null} onKeep={noop} onDiscard={noop} />,
  "dialog-close-failed": () => <DocumentCloseDialog name="Notes" locale="en" failure={{ code: "disk" }} onKeep={noop} onDiscard={noop} />,
  palette: () => <UISearch items={searchItems} open onOpenChange={noop} placeholder="Search commands" emptyMessage="None" />,
  find: () => <UIFindProvider><FindSeed /><UIFind open onOpenChange={noop} placeholder="Find" emptyMessage="None" /></UIFindProvider>,
  "window-search": () => <div style={{ height: 400, position: "relative" }}><Search sessionActive={false} input={{ value: "ca", placeholder: uiDataLabel("Search"), onChange: noop }} possibles={[{ id: "p1", label: "Capsule", detail: "Box" }, { id: "p2", label: "Cabinet", detail: "Furniture" }]} active /></div>,
  popover: () => <div style={{ padding: 40 }}><Popover defaultOpen><PopoverTrigger>Open</PopoverTrigger><PopoverContent><Input aria-label="n" /><Button icon="check" text="Apply" /></PopoverContent></Popover></div>,
  tabbar: () => <div style={{ width: 360 }}><PanelTabBar variant="panel" tabs={workbench as PanelTabNode[]} activePath={[]} onActivePathChange={() => {}} direction="down" /></div>,
};

Object.assign(parts, moreParts, r3Parts);
const i18n = createShellI18nInstance("en");
const g = window as unknown as { __ocl?: { root: ReturnType<typeof createRoot>; host: HTMLDivElement } };
export function show(part: string) {
  g.__ocl?.root.unmount(); g.__ocl?.host.remove();
  for (const el of document.body.children) if (el instanceof HTMLElement && el.id === "root") el.style.display = "none";
  const host = document.createElement("div"); host.id = "os-chrome-live"; host.style.cssText = "position:fixed;inset:0;overflow:auto;background:var(--background,#fff);z-index:0";
  document.body.append(host);
  const root = createRoot(host);
  g.__ocl = { root, host };
  const P = parts[part];
  if (!P) throw new Error("unknown part " + part);
  root.render(<UiI18nProvider instance={i18n}><UiMobileProvider mobile={innerWidth < 768}><P /></UiMobileProvider></UiI18nProvider>);
}
export const partNames = Object.keys(parts);
