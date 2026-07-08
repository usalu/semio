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
	PanelToggleGroup,
	SemioLogo,
	Slider,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Toggle,
	ToolbarDivider,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	WindowMeasureTreeGroup,
	WindowMeasureTreeLeaf,
	WindowMeasuresTree,
	bootstrapElementsSurfaceChromeDocument,
	cn,
	createEvenWindowLayout,
	getLevelBgClass,
	insertWindowAtDropZone,
	interactiveActiveFillClass,
	navbarFillItem,
	shellChromeTitleClassName,
	staticTreePanelDefinition,
	useMediaQuery,
	useSidePanelChromeHotkeys,
	useCommandHotkey,
	readStoredUiChromeCompact,
	readStoredUiChromeExpertise,
	readStoredUiChromeTheme,
	writeStoredUiChromeCompact,
	writeStoredUiChromeExpertise,
	writeStoredUiChromeTheme,
	windowTemplatePaletteTreeDragController,
	Expertise,
	type ElementsSurfaceTheme,
	type EngagementControl,
	type EngagementSpec,
	type FooterItem,
	type ModeWindowDescriptor,
	type NavbarItem,
	type PanelToggleItem,
	type SidePanelTabConfig,
	type TreeDataItem,
	type TreePanelConfig,
	type WindowLayoutNode,
	type ModeCanvasDropTarget,
	type WindowTemplateDropPayload,
} from "@semio-tech/ui-react";
import { ICONS, type IconName } from "@semio-tech/ui-asset";
import { interpretUiNode, uiTreeNodeToTreePanelConfig } from "./ui-interpreter.tsx";
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
	type NamedLayout,
	type PluginRegistryEntry,
	type PluginWasmHandle as CorePluginWasmHandle,
	type WindowLayout,
} from "@semio-tech/framework-core";

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

const S_HOME_APP_ID = "home";
const S_HOME_CONTROLLER_ID = "s-home";
const S_PLAY_APP_ID = "studio";
const S_PLAY_CONTROLLER_ID = "s-play";
const S_PLAY_CATALOGUE_TAB_ID = "s-play-catalogue";
const NAVBAR_NO_EXAMPLE_ID = "__no_example__";
const FRAMEWORK_SHELL_CHROME_THEME = "system" as const;
const DEFAULT_LEFT_PANEL_SIZE = 280;
const DEFAULT_RIGHT_PANEL_SIZE = 320;
const APP_DOCUMENT_SEPARATOR = " · ";

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

function downloadMediaExport(filename: string, mimeType: string, data: string): void {
	if (typeof document === "undefined") return;
	const blob = new Blob([data], { type: mimeType });
	const url = URL.createObjectURL(blob);
	const anchor = document.createElement("a");
	anchor.href = url;
	anchor.download = filename;
	anchor.click();
	URL.revokeObjectURL(url);
}
//#endregion ShellTypes

//#region ShellHelpers
function isStudioMode(pluginFilter?: string): boolean {
	return pluginFilter === "s";
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
	const normalizedWindow = windowLabel.trim().toLowerCase();
	const normalizedApp = app.label.trim().toLowerCase();
	const document = [...app.document];
	if (normalizedWindow && normalizedWindow !== normalizedApp && document.at(-1)?.toLowerCase() !== normalizedWindow) {
		document.push(normalizedWindow);
	}
	return appDocumentLabel(document);
}

function buildStudioPanelState(
	programs: readonly StudioProgramEntry[],
	spawnedApps: readonly SpawnedAppEntry[],
	activePanelTab = "s-play-catalogue",
	activeSpawnedId?: string,
): StudioPanelState {
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
): WindowLayoutNode {
	if (node.kind === "window") {
		return { kind: "window", id: node.windowKindId, title: node.title };
	}
	if (node.kind === "stack") {
		return {
			kind: "stack",
			size: node.size,
			children: node.children.map((child) => ({
				kind: "window" as const,
				id: child.windowKindId,
				title: child.title,
			})),
		};
	}
	return {
		kind: node.kind,
		size: node.size,
		children: node.children.map((child) => convertFrameworkLayoutNodeToModeLayout(child)),
	};
}

function convertFrameworkLayoutToModeLayout(layout: WindowLayout | undefined, windowIds: readonly string[]): WindowLayoutNode {
	if (!layout?.root) return createEvenWindowLayout(windowIds.length ? windowIds : ["main"]);
	return convertFrameworkLayoutNodeToModeLayout(layout.root);
}

function modeLayoutNodeToFramework(
	node: WindowLayoutNode,
): WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode {
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

function windowEngagementControlToSpec(
	control: WindowEngagementControl | undefined,
	onCommand: (command: CommandDescriptor) => void,
): EngagementControl | undefined {
	if (!control) return undefined;
	if (control.kind === "ring" || control.kind === "toggleGroup") {
		return {
			kind: control.kind,
			id: control.id,
			label: control.label,
			value: control.value,
			disabled: control.disabled,
			options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
			onSelect: control.onSelect
				? (id: string) => onCommand({ ...control.onSelect!, args: { ...(control.onSelect!.args as object | undefined), id } })
				: undefined,
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
			onChange: control.onChange
				? (value: string) => onCommand({ ...control.onChange!, args: { ...(control.onChange!.args as object | undefined), value } })
				: undefined,
		};
	}
	const dispatchNumeric = (cmd: CommandDescriptor | undefined, value: number) => {
		if (!cmd) return;
		onCommand({ ...cmd, args: { ...(cmd.args as object | undefined), value } });
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
		status: [{ id: "framework.viewport.status", text: "Viewport" }],
	};
}

function resolveWindowEngagement(
	kind: AppDefinition["windowKinds"][number],
	byKind: Readonly<Record<string, WindowEngagement>>,
): WindowEngagement | undefined {
	const surfaceKind = (kind as { surfaceKind?: string }).surfaceKind;
	return byKind[kind.id] ?? kind.engagement ?? (isViewportSurface(surfaceKind) ? defaultViewportEngagement() : undefined);
}

function windowEngagementToSpec(engagement: WindowEngagement | undefined, onCommand: (command: CommandDescriptor) => void): EngagementSpec | undefined {
	if (!engagement) return undefined;
	const options = engagement.options?.map((option) => ({
		id: option.id,
		label: option.label,
		icon: option.iconId ? <Icon icon={option.iconId in ICONS ? (option.iconId as IconName) : "circle-dot"} size="small" /> : undefined,
		pressed: option.pressed,
		disabled: option.disabled,
		onPress: option.command ? () => onCommand(option.command!) : undefined,
	}));
	const input = engagement.input
		? {
				id: engagement.input.id,
				value: engagement.input.value,
				placeholder: engagement.input.placeholder,
				disabled: engagement.input.disabled,
				onChange: engagement.input.onChange
					? (value: string) => onCommand({ ...engagement.input!.onChange!, args: { ...(engagement.input!.onChange!.args as object | undefined), value } })
					: undefined,
				onSubmit: engagement.input.onSubmit
					? (value: string) => onCommand({ ...engagement.input!.onSubmit!, args: { ...(engagement.input!.onSubmit!.args as object | undefined), value } })
					: undefined,
				onRepeatLast: engagement.input.onRepeatLast ? () => onCommand(engagement.input!.onRepeatLast!) : undefined,
				onAbort: engagement.input.onAbort ? () => onCommand(engagement.input!.onAbort!) : undefined,
			}
		: undefined;
	const status = engagement.status?.map((row) => ({ id: row.id, content: row.text }));
	const possibleEngagements = engagement.possibleEngagements?.map((row) => ({
		id: row.id,
		label: row.label,
		detail: row.detail,
		onSelect: row.command ? () => onCommand(row.command!) : undefined,
	}));
	const control = windowEngagementControlToSpec(engagement.control, onCommand);
	const controls = engagement.controls?.map((row) => windowEngagementControlToSpec(row, onCommand)).filter((row): row is EngagementControl => row !== undefined);
	const hasContent =
		(options?.length ?? 0) > 0 || Boolean(input) || Boolean(control) || (controls?.length ?? 0) > 0 || (status?.length ?? 0) > 0 || (possibleEngagements?.length ?? 0) > 0;
	if (!hasContent) return undefined;
	return { sessionActive: engagement.sessionActive, options, input, control, controls, status, possibleEngagements };
}

function panelTabIcon(tabId: string, group: string): React.FC<{ size?: number }> {
	if (tabId === S_PLAY_CATALOGUE_TAB_ID || group === "workbench") return shellTabIcon(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID);
	if (tabId.includes("parameters")) return shellTabIcon(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID);
	if (tabId.includes("inspector")) return shellTabIcon(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID);
	return shellTabIcon(tabId);
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

function isTreeNode(node: UiNode): node is UiTreeNode {
	return node.type === "tree";
}

function uiNodeToTreePanelConfig(node: UiNode, onCommand: (command: CommandDescriptor) => void): TreePanelConfig {
	if (isTreeNode(node)) return uiTreeNodeToTreePanelConfig(node, onCommand);
	return {
		sections: [
			{
				id: "panel.body",
				label: "",
				items: [
					{
						id: "panel.body.content",
						label: "",
						control: (
							<ChromeAwareWindowScrollSurface className="min-h-0 flex-1">
								{interpretUiNode(node, { onCommand })}
							</ChromeAwareWindowScrollSurface>
						),
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

function renderWindowMeasure(measure: WindowMeasure, onCommand: (command: CommandDescriptor) => void): ReactNode {
	if (measure.kind === "group") {
		return (
			<WindowMeasureTreeGroup key={measure.id} id={measure.id} label={measure.label} defaultOpen={measure.defaultOpen}>
				{measure.children.map((child) => renderWindowMeasure(child, onCommand))}
			</WindowMeasureTreeGroup>
		);
	}
	if (measure.kind === "select") {
		return (
			<WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
				<Select
					value={measure.value}
					onValueChange={(value) => onCommand({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } })}
				>
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
					onValueChange={(values) => onCommand({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value: values[0] ?? measure.value } })}
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
					onPressedChange={(pressed) => onCommand({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), pressed } })}
				/>
			</WindowMeasureTreeLeaf>
		);
	}
	return null;
}

function windowMeasuresOverlay(measures: readonly WindowMeasure[] | undefined, onCommand: (command: CommandDescriptor) => void): ReactNode {
	return (
		<WindowMeasuresTree>
			{(measures ?? []).map((measure) => renderWindowMeasure(measure, onCommand))}
		</WindowMeasuresTree>
	);
}
//#endregion ShellHelpers

//#region Boot
export async function bootFrameworkOs(options: FrameworkOsBootOptions = {}): Promise<void> {
	const root = document.getElementById(options.rootId ?? "root");
	if (!root) throw new Error("missing #root");
	bootstrapElementsSurfaceChromeDocument(FRAMEWORK_SHELL_CHROME_THEME);
	createRoot(root).render(
		<FrameworkOsShell pluginFilter={options.plugin} plugins={options.plugins ?? DEFAULT_PLUGIN_REGISTRY} />,
	);
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
					Render error: {this.state.message}
				</p>
			);
		}
		return this.props.children;
	}
}
//#endregion ErrorBoundary

//#region FrameworkOsShell
export function FrameworkOsShell({
	pluginFilter,
	plugins,
}: {
	readonly pluginFilter?: string;
	readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
}) {
	const studioMode = isStudioMode(pluginFilter);
	const mobile = useMediaQuery("(max-width: 767px)");
	const [loadedPlugins, setLoadedPlugins] = useState<readonly LoadedPluginState[]>([]);
	const [session, setSession] = useState<ActiveSession | null>(null);
	const [windowUiByKind, setWindowUiByKind] = useState<Readonly<Record<string, UiNode>>>({});
	const [windowEngagementsByKind, setWindowEngagementsByKind] = useState<
		Readonly<Record<string, WindowEngagement>>
	>({});
	const [windowMeasuresByKind, setWindowMeasuresByKind] = useState<
		Readonly<Record<string, readonly WindowMeasure[]>>
	>({});
	const [panelUiByKey, setPanelUiByKey] = useState<Readonly<Record<string, UiNode>>>({});
	const [activeToolNodes, setActiveToolNodes] = useState<readonly ToolNode[]>([]);
	const [spawnedWindowUi, setSpawnedWindowUi] = useState<UiNode | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [leftPanelVisible, setLeftPanelVisible] = useState(true);
	const [rightPanelVisible, setRightPanelVisible] = useState(true);
	const [activeLeftPanelKind, setActiveLeftPanelKind] = useState<"workbench" | "display">("workbench");
	const [activeRightPanelKind, setActiveRightPanelKind] = useState<"details" | "settings">("details");
	const [leftPanelSize, setLeftPanelSize] = useState(DEFAULT_LEFT_PANEL_SIZE);
	const [rightPanelSize, setRightPanelSize] = useState(DEFAULT_RIGHT_PANEL_SIZE);
	const [activeWindowId, setActiveWindowId] = useState<string | null>(null);
	const [shellLayout, setShellLayout] = useState<WindowLayoutNode | null>(null);
	const [activeExampleId, setActiveExampleId] = useState("");
	const [searchOpen, setSearchOpen] = useState(false);
	const [findOpen, setFindOpen] = useState(false);
	const importStudioInputRef = useRef<HTMLInputElement>(null);
	const refreshGenerationRef = useRef(0);
	const contributorInstancesRef = useRef<Map<string, number>>(new Map());
	const layoutSeedKeyRef = useRef<string | null>(null);
	const [mobileActiveTabId, setMobileActiveTabId] = useState<string | undefined>(undefined);
	const [leftPanelTabId, setLeftPanelTabId] = useState<string | undefined>(undefined);
	const [rightPanelTabId, setRightPanelTabId] = useState<string | undefined>(undefined);
	const [extraWindowInstances, setExtraWindowInstances] = useState<readonly { readonly id: string; readonly windowKindId: string; readonly title: string }[]>([]);
	const extraWindowCounterRef = useRef(0);
	const openStudioIdRef = useRef<string | null>(null);
	const sessionRef = useRef<ActiveSession | null>(null);
	const [uiTheme, setUiTheme] = useState<ElementsSurfaceTheme>(() => readStoredUiChromeTheme());
	const [uiCompact, setUiCompact] = useState(() => readStoredUiChromeCompact());
	const [uiExpertise, setUiExpertise] = useState(() => readStoredUiChromeExpertise());
	const { uri: shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate: navigateHistory } = useUIHistory("/", studioMode);

	const namedLayoutStore = useMemo(
		() => new NamedLayoutStore(session?.app.id ?? "framework-os", createBrowserStoragePort()),
		[session?.app.id],
	);

	const registry = useMemo(() => {
		const expanded = expandPluginRegistry(plugins, pluginFilter || undefined, studioMode);
		if (studioMode) return expanded;
		return pluginFilter ? expanded : plugins;
	}, [pluginFilter, plugins, studioMode]);

	const panel = session ? parsePanelState(session.viewState) : null;
	const activeAppTitle = appDocumentLabel(
		panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId)?.document
		?? session?.app.document
		?? [],
	);

	useEffect(() => {
		sessionRef.current = session;
	}, [session]);

	useEffect(() => {
		if (activeAppTitle) document.title = activeAppTitle;
	}, [activeAppTitle]);

	useEffect(() => {
		let cancelled = false;
		void (async () => {
			try {
				const settled = await Promise.allSettled(
					registry.map((entry) => loadPluginModuleResilient(entry.pluginId, entry.moduleUrl)),
				);
				const loaded = settled.flatMap((result, index) => {
					if (result.status === "fulfilled" && result.value) return [result.value];
					if (result.status === "rejected") {
						console.error(`[DEBUG] plugin rejected: ${registry[index]?.pluginId}`, result.reason);
					}
					return [];
				});
				if (loaded.length === 0) throw new Error("No plugins loaded");
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

				const first = loaded[0];
				const firstApp = first?.manifest.apps[0];
				if (first && firstApp) {
					const instanceId = await first.createApp(firstApp.id);
					setSession({
						pluginId: first.pluginId,
						instanceId,
						app: firstApp,
						viewState: {
							activeModeId: firstApp.defaultModeId ?? firstApp.modes[0]?.id,
							activeWindowKindId: firstApp.windowKinds[0]?.id,
						},
					});
					setActiveWindowId(firstApp.windowKinds[0]?.id ?? null);
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

	const findPluginForCommand = useCallback(
		(command: CommandDescriptor) => {
			const byController = loadedPlugins.find((entry) => entry.manifest.apps.some((app) => app.controllerId === command.controllerId));
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
			const contributionsJson = buildContributionsJson(
				loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })),
			);
			const viewState: ViewState = { ...nextSession.viewState, contributionsJson };
			const slotContext = {
				plugins: new Map(loadedPlugins.map((entry) => [entry.handle.pluginId, entry.handle])),
				contributorInstances: contributorInstancesRef.current,
				viewState,
			};
			const windowCount = nextSession.app.windowKinds.length;
			const rendered = await Promise.all([
				...nextSession.app.windowKinds.map((kind) =>
					plugin.render(nextSession.instanceId, kind.bodyKey, viewState),
				),
				...nextSession.app.panelTabs.map((tab) => plugin.render(nextSession.instanceId, tab.bodyKey, viewState)),
				plugin.tools(nextSession.instanceId, viewState),
				plugin.windowEngagements(nextSession.instanceId, viewState),
				plugin.windowMeasures(nextSession.instanceId, viewState),
			]);
			if (generation !== refreshGenerationRef.current) return;
			const windowNodes = await Promise.all(
				rendered.slice(0, windowCount).map((node) => resolveExternalSlots(node as UiNode, slotContext)),
			);
			const panelNodes = await Promise.all(
				rendered
					.slice(windowCount, windowCount + nextSession.app.panelTabs.length)
					.map((node) => resolveExternalSlots(node as UiNode, slotContext)),
			);
			const dynamicTools = rendered[windowCount + nextSession.app.panelTabs.length] as readonly ToolNode[];
			const dynamicEngagements = rendered[rendered.length - 2] as Readonly<Record<string, WindowEngagement>>;
			const dynamicMeasures = rendered[rendered.length - 1] as Readonly<Record<string, readonly WindowMeasure[]>>;
			setWindowUiByKind(
				Object.fromEntries(
					nextSession.app.windowKinds.map((kind, index) => [kind.id, windowNodes[index]! as UiNode]),
				),
			);
			setWindowEngagementsByKind(dynamicEngagements);
			setWindowMeasuresByKind(dynamicMeasures);
			setPanelUiByKey(
				Object.fromEntries(
					nextSession.app.panelTabs.map((tab, index) => [tab.id, panelNodes[index]! as UiNode]),
				),
			);
			const activeModeId = viewState.activeModeId ?? nextSession.app.defaultModeId ?? nextSession.app.modes[0]?.id;
			const staticTools = nextSession.app.modes.find((mode) => mode.id === activeModeId)?.tools ?? [];
			setActiveToolNodes(dynamicTools.length > 0 ? dynamicTools : staticTools);
			const windowIds = nextSession.app.windowKinds.map((kind) => kind.id);
			const layoutSeedKey = `${nextSession.pluginId}:${nextSession.app.id}:${nextSession.instanceId}`;
			if (layoutSeedKeyRef.current !== layoutSeedKey) {
				layoutSeedKeyRef.current = layoutSeedKey;
				setExtraWindowInstances([]);
				extraWindowCounterRef.current = 0;
				setShellLayout(convertFrameworkLayoutToModeLayout(nextSession.app.defaultLayout, windowIds));
				const defaultWindowId = findDefaultActiveWindowKindId(nextSession.app.defaultLayout, nextSession.app.windowKinds);
				if (defaultWindowId) setActiveWindowId(defaultWindowId);
				else if (windowIds[0]) setActiveWindowId(windowIds[0]);
			}
		},
		[loadedPlugins],
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
			return;
		}
		const activeSpawned = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
		if (!activeSpawned) {
			setSpawnedWindowUi(null);
			return;
		}
		const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === activeSpawned.pluginId)?.handle;
		const app = loadedPlugins.find((entry) => entry.handle.pluginId === activeSpawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === activeSpawned.appId);
		if (!plugin || !app) {
			setSpawnedWindowUi(null);
			return;
		}
		const bodyKey = resolveCanvasBodyKey(app);
		void plugin
			.render(activeSpawned.instanceId, bodyKey, {
				activeModeId: app.defaultModeId ?? app.modes[0]?.id,
				activeWindowKindId: app.windowKinds[0]?.id,
			})
			.then(setSpawnedWindowUi)
			.catch(() => setSpawnedWindowUi(null));
	}, [loadedPlugins, panel, session, studioMode]);

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
			setShellLayout(convertFrameworkLayoutToModeLayout(app.defaultLayout, app.windowKinds.map((kind) => kind.id)));
			setActiveWindowId(findDefaultActiveWindowKindId(app.defaultLayout, app.windowKinds) ?? app.windowKinds[0]?.id ?? null);
			if (appId === S_HOME_APP_ID) openStudioIdRef.current = null;
			await refreshUi(nextSession);
			return nextSession;
		},
		[loadedPlugins, refreshUi, session],
	);

	const applyShellUri = useCallback(
		async (uri: string, preservedViewState?: ViewState) => {
			const currentSession = sessionRef.current;
			if (!studioMode || !currentSession || loadedPlugins.length === 0) return;
			const path = uri.split("?")[0] ?? "/";
			const studioMatch = /^\/studios\/([^/]+)$/.exec(path);
			const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === "s")?.handle;
			if (!sPlugin) return;
			if (!studioMatch) {
				openStudioIdRef.current = null;
				if (currentSession.app.id !== S_HOME_APP_ID) await switchToSApp(S_HOME_APP_ID, preservedViewState);
				return;
			}
			const studioId = studioMatch[1]!;
			const studioSession =
				currentSession.app.id === S_PLAY_APP_ID
					? currentSession
					: await switchToSApp(S_PLAY_APP_ID, preservedViewState);
			if (!studioSession) return;
			if (openStudioIdRef.current === studioId) return;
			openStudioIdRef.current = studioId;
			await sPlugin.handleCommand(
				studioSession.instanceId,
				JSON.stringify({ controllerId: S_PLAY_CONTROLLER_ID, command: "openStudio", args: { studioId } }),
				studioSession.viewState,
			);
			await refreshUi(studioSession);
		},
		[loadedPlugins, refreshUi, studioMode, switchToSApp],
	);

	useEffect(() => {
		if (!studioMode || loadedPlugins.length === 0) return;
		void applyShellUri(shellUri).catch((uriError) => {
			console.error("[DEBUG] shell uri apply failed", uriError);
		});
	}, [applyShellUri, loadedPlugins.length, shellUri, studioMode]);

	const syncSpawnedPluginDocument = useCallback(
		async (
			plugin: PluginWasmHandle,
			app: AppDefinition,
			pluginInstanceId: number,
			documentJson: string,
			viewState: ViewState,
		) => {
			try {
				const document = JSON.parse(documentJson) as Record<string, unknown>;
				await plugin.handleCommand(
					pluginInstanceId,
					JSON.stringify({ controllerId: app.controllerId, command: "setDocument", args: { document } }),
					viewState,
				);
			} catch (syncError) {
				console.error("[DEBUG] spawned plugin document sync failed", syncError);
			}
		},
		[],
	);

	const ensureSpawnedPlugin = useCallback(
		async (program: StudioProgramEntry, label?: string, osInstanceId?: string, documentJson?: string) => {
			const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
			if (!pluginEntry || !session) return;
			const app = pluginEntry.manifest.apps.find((candidate) => candidate.id === program.appId);
			const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
			const existing = osInstanceId
				? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId)
				: currentPanel.spawnedApps.find(
						(entry) => entry.appId === program.appId && entry.pluginId === program.pluginId,
					);
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
				};
				if (op.op === "setPanel" && op.panel) {
					nextViewState = { ...nextViewState, panelJson: panelJsonFromState(op.panel) };
				}
				if (op.op === "navigate" && typeof op.uri === "string") {
					navigateHistory(op.uri);
					continue;
				}
				if (op.op === "downloadMediaExport" && op.filename && op.mimeType && op.data) {
					downloadMediaExport(op.filename, op.mimeType, op.data);
				}
				if (op.op === "spawnPluginInstance" && op.programId && op.appId) {
					const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
					const program = currentPanel.programs.find((entry) => entry.programId === op.programId && entry.appId === op.appId)
						?? currentPanel.programs.find((entry) => entry.programId === op.programId);
					if (program) await ensureSpawnedPlugin(program, op.label, op.osInstanceId, op.documentJson);
				}
				if (op.op === "openPluginInstance" && op.programId && op.appId) {
					const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
					const program = currentPanel.programs.find((entry) => entry.programId === op.programId && entry.appId === op.appId);
					if (program) await ensureSpawnedPlugin(program, op.label, op.osInstanceId, op.documentJson);
				}
			}
			const nextSession = { ...baseSession, viewState: nextViewState };
			setSession((current) => {
				if (!current) return nextSession;
				if (current.instanceId !== nextSession.instanceId) return current;
				return { ...current, viewState: nextViewState };
			});
			if (session?.instanceId === nextSession.instanceId || baseSession.instanceId === nextSession.instanceId) {
				await refreshUi(nextSession);
			}
		},
		[ensureSpawnedPlugin, loadedPlugins, navigateHistory, refreshUi, session?.instanceId],
	);

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

	const onCommand = useCallback(
		(command: CommandDescriptor) => {
			if (!session) return;

			if (studioMode && command.controllerId === S_HOME_CONTROLLER_ID && command.command === "importStudio") {
				importStudioInputRef.current?.click();
				return;
			}

			if (studioMode && command.command === "spawnApp" && command.controllerId !== S_PLAY_CONTROLLER_ID) {
				const programId = typeof command.args === "object" && command.args != null && "programId" in command.args
					? String((command.args as { programId?: string }).programId ?? "")
					: "";
				const pluginId = typeof command.args === "object" && command.args != null && "pluginId" in command.args
					? String((command.args as { pluginId?: string }).pluginId ?? "")
					: "";
				const currentPanel = parsePanelState(session.viewState);
				const program = currentPanel?.programs.find((entry) => entry.programId === programId && entry.pluginId === pluginId);
				if (program) void spawnProgram(program);
				return;
			}

			if (studioMode && command.controllerId === S_PLAY_CONTROLLER_ID && command.command === "setActivePanelTab") {
				const tabId = typeof command.args === "object" && command.args != null && "tabId" in command.args
					? String((command.args as { tabId?: string }).tabId ?? "s-play-catalogue")
					: "s-play-catalogue";
				const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
				updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, tabId, currentPanel.activeSpawnedId));
				return;
			}

			const pluginEntry = findPluginForCommand(command);
			const plugin = pluginEntry?.handle;
			if (!plugin) return;

			const targetSession =
				studioMode && command.controllerId !== session.app.controllerId
					? (() => {
							const spawned = panel?.spawnedApps.find((entry) => {
								const app = loadedPlugins.find((p) => p.handle.pluginId === entry.pluginId)?.manifest.apps.find((a) => a.id === entry.appId);
								return app?.controllerId === command.controllerId;
							});
							if (!spawned) return session;
							const app = loadedPlugins.find((p) => p.handle.pluginId === spawned.pluginId)?.manifest.apps.find((a) => a.id === spawned.appId);
							if (!app) return session;
							return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
						})()
					: session;

			void plugin
				.handleCommand(targetSession.instanceId, JSON.stringify(command), targetSession.viewState)
				.then(async (ops) => {
					if (
						studioMode &&
						session.pluginId === "s" &&
						panel?.activeSpawnedId &&
						command.controllerId !== session.app.controllerId
					) {
						const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
						const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === "s")?.handle;
						if (spawned && sPlugin) {
							for (const opJson of ops) {
								const op = JSON.parse(opJson) as { op?: string; document?: unknown };
								if (op.op === "setDocument" && op.document != null) {
									const patchOps = await sPlugin.handleCommand(
										session.instanceId,
										JSON.stringify({
											controllerId: S_PLAY_CONTROLLER_ID,
											command: "patchAppSource",
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
				.catch((commandError) => {
					console.error("[DEBUG] command failed", commandError);
				});
		},
		[findPluginForCommand, loadedPlugins, panel, processPluginOps, session, spawnProgram, studioMode, updateStudioPanel],
	);

	useSidePanelChromeHotkeys({
		onToggleLeft: () => setLeftPanelVisible((visible) => !visible),
		onToggleRight: () => setRightPanelVisible((visible) => !visible),
	});

	useEffect(() => {
		bootstrapElementsSurfaceChromeDocument(uiTheme);
		writeStoredUiChromeTheme(uiTheme);
	}, [uiTheme]);

	useEffect(() => {
		writeStoredUiChromeCompact(uiCompact);
		document.documentElement.toggleAttribute("data-ui-compact", uiCompact);
	}, [uiCompact]);

	useEffect(() => {
		writeStoredUiChromeExpertise(uiExpertise);
	}, [uiExpertise]);

	useCommandHotkey(
		"mod+[",
		useCallback(() => {
			if (canGoBack) goBack();
		}, [canGoBack, goBack]),
	);
	useCommandHotkey(
		"mod+]",
		useCallback(() => {
			if (canGoForward) goForward();
		}, [canGoForward, goForward]),
	);
	useCommandHotkey(
		"mod+up",
		useCallback(() => {
			if (canGoUp) goUp();
		}, [canGoUp, goUp]),
	);
	useCommandHotkey("mod+p", useCallback(() => setSearchOpen((open) => !open), []));
	useCommandHotkey("mod+f", useCallback(() => setFindOpen((open) => !open), []));

	const applyNamedLayout = useCallback(
		(layout: WindowLayout) => {
			if (!session) return;
			const windowIds = session.app.windowKinds.map((kind) => kind.id);
			setExtraWindowInstances([]);
			extraWindowCounterRef.current = 0;
			setShellLayout(convertFrameworkLayoutToModeLayout(layout, windowIds));
			const defaultWindowId = findDefaultActiveWindowKindId(layout, session.app.windowKinds);
			if (defaultWindowId) setActiveWindowId(defaultWindowId);
		},
		[session],
	);

	const handleTemplateDrop = useCallback(
		(payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => {
			if (!session) return;
			const kind = session.app.windowKinds.find((entry) => entry.id === payload.windowKindId);
			if (!kind) return;
			extraWindowCounterRef.current += 1;
			const instanceId = `${payload.windowKindId}-${extraWindowCounterRef.current}`;
			setExtraWindowInstances((current) => [
				...current,
				{ id: instanceId, windowKindId: payload.windowKindId, title: kind.label },
			]);
			setShellLayout((current) => {
				const base =
					current ??
					convertFrameworkLayoutToModeLayout(
						session.app.defaultLayout,
						session.app.windowKinds.map((entry) => entry.id),
					);
				return insertWindowAtDropZone(base, instanceId, target);
			});
			setActiveWindowId(instanceId);
		},
		[session],
	);

	const displayHostRef = useRef<ReturnType<typeof useNamedLayoutHost> | null>(null);
	const displayHost = useNamedLayoutHost({
		appId: session?.app.id ?? "framework-os",
		windowKinds: session?.app.windowKinds ?? [],
		builtinLayouts: session?.app.namedLayouts ?? [],
		currentLayout: captureCurrentFrameworkLayout(shellLayout, session?.app.defaultLayout),
		onApplyLayout: applyNamedLayout,
		namedLayoutStore,
	});
	displayHostRef.current = displayHost;

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
			theme: uiTheme,
			setTheme: setUiTheme,
		}),
		[session, uiCompact, uiExpertise, uiTheme],
	);
	settingsHostRef.current = settingsHost;

	const frameworkDisplayTabs = useMemo(
		() => createFrameworkDisplayPanelTabs(() => displayHostRef.current),
		[displayHost],
	);
	const frameworkSettingsTab = useMemo(
		() => createFrameworkSettingsPanelTab(() => settingsHostRef.current),
		[settingsHost],
	);

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
					onCommand(binding.command);
					return;
				}
			}
		};
		window.addEventListener("keydown", onKeyDown);
		return () => window.removeEventListener("keydown", onKeyDown);
	}, [onCommand, session]);

	const activePanelTabId = panel?.activePanelTab ?? session?.app.panelTabs.find((tab) => panelSideForGroup(tab.group) === "right")?.id ?? session?.app.panelTabs[0]?.id;

	const workbenchLeftTabs = useMemo((): SidePanelTabConfig[] => {
		if (!session) return [];
		const pluginLeftTabs = session.app.panelTabs
			.filter((tab) => panelSideForGroup(tab.group) === "left")
			.map((tab, order) => ({
				id: tab.id,
				icon: panelTabIcon(tab.id, tab.group),
				name: tab.label,
				order,
				tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tab.id] ?? { type: "text", value: "Loading…" }, onCommand)),
			}));
		if (studioMode && session.app.id === S_PLAY_APP_ID && pluginLeftTabs.length > 0) return pluginLeftTabs;
		const hasPluginDocumentTab = pluginLeftTabs.some((tab) => tab.id === FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
		if (hasPluginDocumentTab) return pluginLeftTabs;
		const documentTab: SidePanelTabConfig = {
			id: FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
			icon: shellTabIcon(FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
			order: 0,
			tree: staticTreePanelDefinition({
				sections: [{ id: "document.root", label: "Document", items: [{ id: "document.empty", label: studioMode ? `${panel?.spawnedApps.length ?? 0} spawned app(s)` : "—" }] }],
			}),
		};
		return [documentTab, ...pluginLeftTabs];
	}, [onCommand, panel?.spawnedApps.length, panelUiByKey, session, studioMode]);

	const detailsRightTabs = useMemo((): SidePanelTabConfig[] => {
		if (!session) return [];
		return session.app.panelTabs
			.filter((tab) => panelSideForGroup(tab.group) === "right")
			.map((tab, order) => ({
				id: tab.id,
				icon: panelTabIcon(tab.id, tab.group),
				name: tab.label,
				order,
				tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tab.id] ?? { type: "text", value: "Loading…" }, onCommand)),
			}));
	}, [onCommand, panelUiByKey, session]);

	const settingsRightTabs = useMemo((): SidePanelTabConfig[] => [frameworkSettingsTab], [frameworkSettingsTab]);

	const leftPanelTabs = useMemo(
		(): SidePanelTabConfig[] => (activeLeftPanelKind === "display" ? frameworkDisplayTabs : workbenchLeftTabs),
		[activeLeftPanelKind, frameworkDisplayTabs, workbenchLeftTabs],
	);

	const rightPanelTabs = useMemo(
		(): SidePanelTabConfig[] => (activeRightPanelKind === "settings" ? settingsRightTabs : detailsRightTabs),
		[activeRightPanelKind, detailsRightTabs, settingsRightTabs],
	);

	const activeLeftPanelTabId = useMemo(() => {
		if (activeLeftPanelKind === "display") return frameworkDisplayTabs[0]?.id ?? FRAMEWORK_PANEL_TAB_DOCUMENT_ID;
		if (studioMode && session?.app.id === S_PLAY_APP_ID) return panel?.activePanelTab ?? S_PLAY_CATALOGUE_TAB_ID;
		return workbenchLeftTabs[0]?.id ?? FRAMEWORK_PANEL_TAB_DOCUMENT_ID;
	}, [activeLeftPanelKind, frameworkDisplayTabs, panel?.activePanelTab, session?.app.id, studioMode, workbenchLeftTabs]);

	const activeRightPanelTabId = useMemo(() => {
		if (activeRightPanelKind === "settings") return settingsRightTabs[0]?.id;
		if (panel?.activePanelTab && detailsRightTabs.some((tab) => tab.id === panel.activePanelTab)) return panel.activePanelTab;
		return detailsRightTabs[0]?.id ?? activePanelTabId;
	}, [activePanelTabId, activeRightPanelKind, detailsRightTabs, panel?.activePanelTab, settingsRightTabs]);

	useEffect(() => {
		setLeftPanelTabId(undefined);
	}, [activeLeftPanelKind]);

	useEffect(() => {
		setRightPanelTabId(undefined);
	}, [activeRightPanelKind]);

	const mobilePanelTabs = useMemo(() => [...leftPanelTabs, ...rightPanelTabs], [leftPanelTabs, rightPanelTabs]);

	const mobilePanel = useMemo(() => {
		if (mobilePanelTabs.length === 0) return undefined;
		return {
			visible: leftPanelVisible || rightPanelVisible,
			tabs: mobilePanelTabs,
			activeTabId: mobileActiveTabId ?? mobilePanelTabs[0]?.id,
			onActiveTabChange: (tabId: string) => {
				setMobileActiveTabId(tabId);
				if (studioMode && session?.app.id === S_PLAY_APP_ID) {
					onCommand({ controllerId: session.app.controllerId, command: "setActivePanelTab", args: { tabId } });
				}
			},
		};
	}, [leftPanelVisible, mobileActiveTabId, mobilePanelTabs, onCommand, rightPanelVisible, session, studioMode]);

	const workbenchIcon = useMemo(() => {
		const TabIcon = workbenchLeftTabs[0]?.icon;
		return TabIcon ? <TabIcon size={16} /> : <Icon icon="folder" size="small" />;
	}, [workbenchLeftTabs]);

	const detailsIcon = useMemo(() => {
		const TabIcon = detailsRightTabs[0]?.icon;
		return TabIcon ? <TabIcon size={16} /> : <Icon icon="info" size="small" />;
	}, [detailsRightTabs]);

	const displayIcon = useMemo(() => {
		const TabIcon = frameworkDisplayTabs[0]?.icon;
		return TabIcon ? <TabIcon size={16} /> : <Icon icon="layout-grid" size="small" />;
	}, [frameworkDisplayTabs]);

	const settingsIcon = useMemo(() => <Icon icon="settings-2" size="small" />, []);

	const panelToggles = useMemo((): PanelToggleItem[] => {
		const items: PanelToggleItem[] = [];
		if (frameworkDisplayTabs.length > 0) {
			items.push({
				id: "ui.panelToggle.display",
				icon: displayIcon,
				pressed: leftPanelVisible && activeLeftPanelKind === "display",
				onPressedChange: (pressed) => {
					if (pressed) setActiveLeftPanelKind("display");
					setLeftPanelVisible((visible) => pressed || (activeLeftPanelKind === "workbench" && visible));
				},
			});
		}
		items.push({
			id: "ui.panelToggle.workbench",
			icon: workbenchIcon,
			pressed: leftPanelVisible && activeLeftPanelKind === "workbench",
			onPressedChange: (pressed) => {
				if (pressed) setActiveLeftPanelKind("workbench");
				setLeftPanelVisible((visible) => pressed || (activeLeftPanelKind === "display" && visible));
			},
		});
		items.push({
			id: "ui.panelToggle.details",
			icon: detailsIcon,
			pressed: rightPanelVisible && activeRightPanelKind === "details",
			onPressedChange: (pressed) => {
				if (pressed) setActiveRightPanelKind("details");
				setRightPanelVisible((visible) => pressed || (activeRightPanelKind === "settings" && visible));
			},
		});
		items.push({
			id: "ui.panelToggle.settings",
			icon: settingsIcon,
			pressed: rightPanelVisible && activeRightPanelKind === "settings",
			onPressedChange: (pressed) => {
				if (pressed) setActiveRightPanelKind("settings");
				setRightPanelVisible((visible) => pressed || (activeRightPanelKind === "details" && visible));
			},
		});
		return items;
	}, [
		activeLeftPanelKind,
		activeRightPanelKind,
		detailsIcon,
		displayIcon,
		frameworkDisplayTabs.length,
		leftPanelVisible,
		rightPanelVisible,
		settingsIcon,
		workbenchIcon,
	]);

	const activePluginManifest = useMemo(
		() => loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId)?.manifest,
		[loadedPlugins, session?.pluginId],
	);
	const exampleOptions = useMemo(
		() => (activePluginManifest?.examples ?? []).map((example) => ({ id: example.id, label: example.label })),
		[activePluginManifest],
	);

	useEffect(() => {
		if (exampleOptions.length === 0) return;
		setActiveExampleId((current) => (exampleOptions.some((option) => option.id === current) ? current : exampleOptions[0]!.id));
	}, [exampleOptions, session?.app.id, session?.pluginId]);

	const activeModeId = session?.viewState.activeModeId ?? session?.app.modes[0]?.id ?? session?.app.id ?? "";

	const navbarItems = useMemo((): NavbarItem[] => {
		if (!session) return [];
		const items: NavbarItem[] = [
			{
				key: "logoAndTitle",
				className: "min-w-0 shrink-0 flex items-center gap-single",
				content: (
					<div className="flex items-center gap-single">
						<SemioLogo className="size-workbench shrink-0" />
						<span data-slot="app-name" className={cn("px-single", shellChromeTitleClassName)}>
							{appDocumentLabel(session.app.document)}
						</span>
					</div>
				),
			},
		];
		if (exampleOptions.length > 0 && (!studioMode || session.app.id !== S_HOME_APP_ID)) {
			items.push({
				key: "fixture",
				content: (
					<NavbarExampleSelect
						id="playground.navbar.fixture"
						value={activeExampleId}
						options={exampleOptions}
						onValueChange={(exampleId) => {
							setActiveExampleId(exampleId);
							onCommand({ controllerId: session.app.controllerId, command: "setActiveExample", args: { exampleId } });
						}}
					/>
				),
			});
			items.push(navbarFillItem());
		} else {
			items.push(navbarFillItem());
		}
		items.push({ key: "panelToggles", content: <PanelToggleGroup items={panelToggles} /> });
		if (session.app.modes.length > 1) {
			items.push({
				key: "modes",
				content: (
					<ButtonGroup id="playground.navbar.modes">
						{session.app.modes.map((mode) => {
							const isActive = activeModeId === mode.id;
							return (
								<ButtonGroupItem
									key={mode.id}
									id={`playground.navbar.modes.${mode.id}`}
									className={cn(isActive && interactiveActiveFillClass)}
									data-state={isActive ? "on" : undefined}
									onClick={() => {
										setSession((current) => (current ? { ...current, viewState: { ...current.viewState, activeModeId: mode.id } } : current));
									}}
									icon={<span className="hidden" />}
									text={mode.label}
								/>
							);
						})}
					</ButtonGroup>
				),
			});
		}
		return items;
	}, [activeExampleId, activeModeId, exampleOptions, onCommand, panelToggles, session]);

	const searchItems = useMemo(() => {
		if (!session) return [];
		const items: UISearchItem[] = [];
		for (const tab of session.app.panelTabs) {
			items.push({
				id: `panel.${tab.id}`,
				label: tab.label,
				category: "Panels",
				icon: <Icon icon="panel-left" size="small" />,
				onSelect: () => onCommand({ controllerId: session.app.controllerId, command: "setActivePanelTab", args: { tabId: tab.id } }),
			});
		}
		for (const kind of session.app.windowKinds) {
			items.push({
				id: `window.${kind.id}`,
				label: kind.label,
				category: "Windows",
				icon: <Icon icon="app-window" size="small" />,
				onSelect: () => setActiveWindowId(kind.id),
			});
		}
		for (const binding of session.app.keybindings) {
			items.push({
				id: `keybinding.${binding.keys}`,
				label: binding.command.command,
				description: binding.keys,
				category: "Commands",
				onSelect: () => onCommand(binding.command),
			});
		}
		if (studioMode && panel) {
			for (const program of panel.programs) {
				items.push({
					id: `spawn.${program.programId}`,
					label: `Spawn ${appDocumentLabel(program.document)}`,
					category: "Catalogue",
					onSelect: () => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "spawnApp", args: { programId: program.programId } }),
				});
			}
			items.push(
				{
					id: "studio.undo",
					label: "Undo",
					category: "Studio",
					icon: <Icon icon="undo-2" size="small" />,
					onSelect: () => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "undo" }),
				},
				{
					id: "studio.redo",
					label: "Redo",
					category: "Studio",
					icon: <Icon icon="redo-2" size="small" />,
					onSelect: () => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "redo" }),
				},
				{
					id: "studio.home",
					label: "Go Home",
					category: "Navigation",
					onSelect: () => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "goHome" }),
				},
			);
		}
		if (studioMode && session.app.id === S_HOME_APP_ID) {
			items.push({
				id: "home.createStudio",
				label: "Create Studio",
				category: "Home",
				onSelect: () => onCommand({ controllerId: S_HOME_CONTROLLER_ID, command: "createStudio" }),
			});
		}
		return items;
	}, [onCommand, panel, session, studioMode]);

	const footerItems = useMemo((): FooterItem[] => {
		if (!session) return [];
		return [
			{
				id: "framework.footer.app",
				text: appDocumentLabel(session.app.document),
				icon: <Icon icon={session.app.iconId && session.app.iconId in ICONS ? (session.app.iconId as IconName) : "app-window"} size="small" />,
			},
		];
	}, [session]);

	const footerToolbar = useMemo(() => {
		if (!activeToolNodes.length) return undefined;
		return <ToolTree tools={activeToolNodes} onCommand={onCommand} />;
	}, [activeToolNodes, onCommand]);

	const modeWindows = useMemo((): ModeWindowDescriptor[] => {
		if (!session) return [];
		if (studioMode && spawnedWindowUi && panel?.activeSpawnedId) {
			const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
			if (spawned) {
				return [
					{
						id: spawned.id,
						title: appDocumentLabel(spawned.document),
						fill: true,
						showControls: true,
						children: (
							<ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
								{interpretUiNode(spawnedWindowUi, { onCommand })}
							</ChromeAwareWindowScrollSurface>
						),
					},
				];
			}
		}
		if (Object.keys(windowUiByKind).length === 0) return [];
		const baseWindows = session.app.windowKinds.map((kind) => ({
			id: kind.id,
			title: appWindowDocumentLabel(session.app, kind.label),
			fill: true,
			showControls: true,
			measures: windowMeasuresOverlay(windowMeasuresByKind[kind.id] ?? kind.measures, onCommand),
			engagement: windowEngagementToSpec(resolveWindowEngagement(kind, windowEngagementsByKind), onCommand),
			children: (
				<ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" data-window-kind-id={kind.id}>
					{interpretUiNode(windowUiByKind[kind.id] ?? { type: "text", value: `Missing window: ${kind.id}` }, { onCommand })}
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
					measures: windowMeasuresOverlay(windowMeasuresByKind[kind.id] ?? kind.measures, onCommand),
					engagement: windowEngagementToSpec(resolveWindowEngagement(kind, windowEngagementsByKind), onCommand),
					children: (
						<ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" data-window-kind-id={kind.id}>
							{interpretUiNode(windowUiByKind[kind.id] ?? { type: "text", value: `Missing window: ${kind.id}` }, { onCommand })}
						</ChromeAwareWindowScrollSurface>
					),
				},
			];
		});
		return [...baseWindows, ...extraWindows];
	}, [extraWindowInstances, onCommand, panel, session, spawnedWindowUi, studioMode, windowEngagementsByKind, windowUiByKind]);

	const canvas = useMemo(() => {
		if (!session) return <p className="p-4 text-sm text-muted-foreground">Loading plugins…</p>;
		if (error) return <p className="p-4 text-sm text-destructive" role="alert">{error}</p>;
		const modes = session.app.modes.length > 0
			? session.app.modes
			: [{ id: session.app.id, label: appDocumentLabel(session.app.document) }];
		const studioHomeBar =
			studioMode && session.app.id === S_PLAY_APP_ID && !panel?.activeSpawnedId ? (
				<button
					type="button"
					className="border-b border-border/60 px-3 py-2 text-left text-sm text-muted-foreground hover:bg-muted/40 hover:text-foreground"
					onClick={() => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "goHome" })}
				>
					← Home
				</button>
			) : null;
		const focusedSpawned = panel?.activeSpawnedId ? panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId) : undefined;
		const focusedBar = focusedSpawned ? (
			<div className="flex items-center gap-2 border-b border-border/60 px-3 py-2 text-sm text-muted-foreground">
				<button
					type="button"
					className="hover:text-foreground"
					onClick={() => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "closeFocusedInstance" })}
				>
					← Back to Media Graph
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
							onCommand({ controllerId: S_HOME_CONTROLLER_ID, command: "importStudio", args: { json } });
							event.target.value = "";
						});
					}}
				/>
				<div className="min-h-0 flex-1">
					<App
						modes={modes.map((mode) => ({ id: mode.id, label: mode.label, children: null }))}
						activeModeId={session.viewState.activeModeId ?? modes[0]?.id ?? session.app.id}
						onActiveModeChange={(modeId) => {
							setSession((current) => (current ? { ...current, viewState: { ...current.viewState, activeModeId: modeId } } : current));
						}}
						chrome={false}
					>
						<Mode
							className="h-full w-full"
							windows={modeWindows}
							layout={shellLayout ?? convertFrameworkLayoutToModeLayout(session.app.defaultLayout, modeWindows.map((window) => window.id))}
							activeWindowId={activeWindowId}
							onActiveWindowChange={setActiveWindowId}
							onLayoutChange={setShellLayout}
							onTemplateDrop={handleTemplateDrop}
							onWindowClose={(windowId) => {
								if (studioMode && panel?.spawnedApps.some((entry) => entry.id === windowId)) {
									const nextSpawned = panel.spawnedApps.filter((entry) => entry.id !== windowId);
									updateStudioPanel(buildStudioPanelState(panel.programs, nextSpawned, panel.activePanelTab, nextSpawned[0]?.id));
								}
								setExtraWindowInstances((current) => current.filter((entry) => entry.id !== windowId));
								setShellLayout((current) => current ?? convertFrameworkLayoutToModeLayout(session.app.defaultLayout, modeWindows.map((window) => window.id)));
							}}
						/>
					</App>
				</div>
			</div>
		);
	}, [activeWindowId, error, handleTemplateDrop, modeWindows, onCommand, panel, session, shellLayout, studioMode, updateStudioPanel]);

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
							leftPanelTabs.length > 0
								? {
										position: "left",
										visible: leftPanelVisible,
										size: leftPanelSize,
										onSizeChange: setLeftPanelSize,
										tabs: leftPanelTabs,
										activeTabId: leftPanelTabId ?? activeLeftPanelTabId,
										onActiveTabChange: (tabId) => {
											setLeftPanelTabId(tabId);
											if (studioMode && session?.app.id === S_PLAY_APP_ID) {
												onCommand({ controllerId: session.app.controllerId, command: "setActivePanelTab", args: { tabId } });
											}
										},
									}
								: undefined
						}
						rightSidePanel={
							rightPanelTabs.length > 0
								? {
										position: "right",
										visible: rightPanelVisible,
										size: rightPanelSize,
										onSizeChange: setRightPanelSize,
										tabs: rightPanelTabs,
										activeTabId: rightPanelTabId ?? activeRightPanelTabId,
										onActiveTabChange: (tabId) => {
											setRightPanelTabId(tabId);
											if (studioMode && session?.app.id === S_PLAY_APP_ID) {
												onCommand({ controllerId: session.app.controllerId, command: "setActivePanelTab", args: { tabId } });
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
export type CommandDescriptor = {
	readonly controllerId: string;
	readonly command: string;
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
	readonly command: CommandDescriptor;
	readonly style?: StyleSpec;
};

export type UiSeparatorNode = { readonly type: "separator" };

export type UiInputNode = {
	readonly type: "input";
	readonly id: string;
	readonly inputKind: string;
	readonly value: string;
	readonly placeholder?: string;
	readonly commit?: string;
	readonly onChange: CommandDescriptor;
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
	readonly onChange: CommandDescriptor;
};

export type UiToggleNode = {
	readonly type: "toggle";
	readonly id: string;
	readonly iconId: string;
	readonly pressed: boolean;
	readonly text?: string;
	readonly onChange: CommandDescriptor;
};

export type UiVec3Node = {
	readonly type: "vec3";
	readonly id: string;
	readonly value: readonly [number, number, number] | null;
	readonly onChange: CommandDescriptor;
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
	readonly onChange: CommandDescriptor;
};

export type UiNumberStepperNode = {
	readonly type: "numberStepper";
	readonly id: string;
	readonly value: number;
	readonly step: number;
	readonly uniform: boolean;
	readonly onAbsolute: CommandDescriptor;
	readonly onDelta: CommandDescriptor;
};

export type UiRingNode = {
	readonly type: "ring";
	readonly id: string;
	readonly orbId: string;
	readonly t: number;
	readonly disabled?: boolean;
	readonly onChange: CommandDescriptor;
};

export type UiIconSelectNode = {
	readonly type: "iconSelect";
	readonly id: string;
	readonly value: string;
	readonly uniform: boolean;
	readonly classifierKind: string;
	readonly onChange: CommandDescriptor;
};

export type UiControlNode =
	| UiInputNode
	| UiSelectNode
	| UiToggleNode
	| UiVec3Node
	| UiButtonNode
	| UiKeyValueNode
	| UiSliderNode
	| UiNumberStepperNode
	| UiRingNode
	| UiIconSelectNode;

export type UiFieldNode = {
	readonly type: "field";
	readonly id: string;
	readonly label: string;
	readonly child: UiControlNode;
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
	readonly command: CommandDescriptor;
	readonly revealOnHover?: boolean;
};

export type UiTreeItemNode = {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	readonly iconId?: string;
	readonly selected?: boolean;
	readonly defaultOpen?: boolean;
	readonly command?: CommandDescriptor;
	readonly hoverCommand?: CommandDescriptor;
	readonly unhoverCommand?: CommandDescriptor;
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
	readonly selectionChange?: CommandDescriptor;
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
};

export const nodeGraphCommands = {
	select: "nodeGraphSelect",
	hover: "nodeGraphHover",
	edit: "nodeGraphEdit",
	viewport: "nodeGraphViewport",
	spotlightCommit: "spotlightCommit",
} as const;

export const textEditorCommands = {
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
	readonly width: number;
	readonly height: number;
	readonly pixelsBase64: string;
};

export type VirtualFileSystemScene = {
	readonly schemaJson: string;
	readonly rowsJson: string;
	readonly selectedRowIdsJson?: string;
	readonly hoveredRowId?: string;
	readonly emptyMessage?: string;
	readonly dragDropEnabled?: boolean;
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
	readonly command?: CommandDescriptor;
};

export type WindowEngagementInput = {
	readonly id?: string;
	readonly value?: string;
	readonly placeholder?: string;
	readonly disabled?: boolean;
	readonly onChange?: CommandDescriptor;
	readonly onSubmit?: CommandDescriptor;
	readonly onRepeatLast?: CommandDescriptor;
	readonly onAbort?: CommandDescriptor;
};

export type WindowEngagementStatus = {
	readonly id: string;
	readonly text: string;
};

export type WindowEngagementPossible = {
	readonly id: string;
	readonly label: string;
	readonly detail?: string;
	readonly command?: CommandDescriptor;
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
			readonly onChange?: CommandDescriptor;
			readonly onCommit?: CommandDescriptor;
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
			readonly onChange?: CommandDescriptor;
			readonly onCommit?: CommandDescriptor;
	  }
	| {
			readonly kind: "ring";
			readonly id?: string;
			readonly label?: string;
			readonly value?: string;
			readonly options: readonly WindowEngagementRingOption[];
			readonly disabled?: boolean;
			readonly onSelect?: CommandDescriptor;
	  }
	| {
			readonly kind: "toggleGroup";
			readonly id?: string;
			readonly label?: string;
			readonly value?: string;
			readonly options: readonly WindowEngagementToggleGroupOption[];
			readonly disabled?: boolean;
			readonly onSelect?: CommandDescriptor;
	  }
	| {
			readonly kind: "select";
			readonly id?: string;
			readonly label?: string;
			readonly value?: string;
			readonly placeholder?: string;
			readonly items: readonly WindowEngagementSelectItem[];
			readonly disabled?: boolean;
			readonly onChange?: CommandDescriptor;
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
			readonly onChange: CommandDescriptor;
	  }
	| {
			readonly kind: "slider";
			readonly id: string;
			readonly label?: string;
			readonly value: number;
			readonly min: number;
			readonly max: number;
			readonly step?: number;
			readonly onChange: CommandDescriptor;
	  }
	| {
			readonly kind: "toggle";
			readonly id: string;
			readonly iconId: string;
			readonly label?: string;
			readonly pressed: boolean;
			readonly text?: string;
			readonly onChange: CommandDescriptor;
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
	readonly panelTabs: readonly { readonly id: string; readonly label: string; readonly group: string; readonly bodyKey: string }[];
	readonly keybindings: readonly { readonly keys: string; readonly command: CommandDescriptor }[];
	readonly namedLayouts?: readonly NamedLayout[];
	readonly defaultLayout?: WindowLayout;
};

export type PluginManifest = {
	readonly pluginId: string;
	readonly label: string;
	readonly version: string;
	readonly apps: readonly AppDefinition[];
	readonly programs: readonly { readonly programId: string; readonly appId: string; readonly label: string; readonly document: readonly string[]; readonly yields: string }[];
	readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string }[];
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
			readonly controllerId?: string;
			readonly command?: string;
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
			readonly controllerId?: string;
			readonly command?: string;
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
			readonly onPress: CommandDescriptor;
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
			readonly onChange: CommandDescriptor;
	  };

export const UI_INSPECTOR_MIXED_PLACEHOLDER = "Mixed";

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
	readonly handleCommand: (instanceId: number, commandJson: string, viewState: ViewState) => Promise<string[]>;
	readonly render: (instanceId: number, bodyKey: string, viewState: ViewState) => Promise<UiNode>;
	readonly renderWithDocument?: (
		instanceId: number,
		bodyKey: string,
		viewState: ViewState,
		documentJson: string,
	) => Promise<UiNode>;
	readonly tools: (instanceId: number, viewState: ViewState) => Promise<readonly ToolNode[]>;
	readonly windowEngagements: (
		instanceId: number,
		viewState: ViewState,
	) => Promise<Readonly<Record<string, WindowEngagement>>>;
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
		handleCommand: (instanceId, commandJson, viewState) =>
			handle.handleCommand(instanceId, commandJson, viewState),
		render: async (instanceId, bodyKey, viewState) =>
			(await handle.render(instanceId, bodyKey, viewState)) as unknown as UiNode,
		renderWithDocument: handle.renderWithDocument
			? async (instanceId, bodyKey, viewState, documentJson) =>
					(await handle.renderWithDocument!(instanceId, bodyKey, viewState, documentJson)) as unknown as UiNode
			: undefined,
		tools: async (instanceId, viewState) =>
			(await handle.tools(instanceId, viewState)) as unknown as ToolNode[],
		windowEngagements: async (instanceId, viewState) =>
			(await handle.windowEngagements(instanceId, viewState)) as unknown as Readonly<
				Record<string, WindowEngagement>
			>,
		windowMeasures: async (instanceId, viewState) =>
			(await handle.windowMeasures(instanceId, viewState)) as unknown as Readonly<
				Record<string, readonly WindowMeasure[]>
			>,
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
	placeholder = "Search commands…",
	emptyMessage = "No results.",
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
		<CommandDialog title="Search" description="Global command palette" open={open} onOpenChange={onOpenChange} shouldFilter={false}>
			<CommandInput id="ui.search.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
			<CommandList>
				<CommandEmpty>{emptyMessage}</CommandEmpty>
				{Object.entries(grouped).map(([category, categoryResults]) => (
					<CommandGroup key={category || "__default"} heading={category || undefined}>
						{categoryResults.map((result, idx) => (
							<CommandItem
								key={`${result.item.id}-${idx}`}
								value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()}
								onSelect={() => handleSelect(result.item)}
							>
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
		if (
			!previous ||
			!next ||
			previous.id !== next.id ||
			previous.label !== next.label ||
			previous.description !== next.description ||
			previous.category !== next.category
		) {
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
	const contextValue = useMemo(
		() => ({ findItems, setFindItems, setOnFindItem, triggerFindItem }),
		[findItems, setFindItems, setOnFindItem, triggerFindItem],
	);
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

export function UIFind({
	open,
	onOpenChange,
	placeholder = "Find in window…",
	emptyMessage = "No results.",
}: {
	readonly open: boolean;
	readonly onOpenChange: (open: boolean) => void;
	readonly placeholder?: string;
	readonly emptyMessage?: string;
}) {
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
		<CommandDialog title="Find" description="Find in active window" open={open} onOpenChange={onOpenChange} shouldFilter={false}>
			<CommandInput id="ui.find.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
			<CommandList>
				<CommandEmpty>{emptyMessage}</CommandEmpty>
				{Object.entries(grouped).map(([category, categoryResults]) => (
					<CommandGroup key={category || "__default"} heading={category || undefined}>
						{categoryResults.map((result, idx) => (
							<CommandItem
								key={`${result.item.id}-${idx}`}
								value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()}
								onSelect={() => handleSelect(result.item)}
							>
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

//#region 🔖tool-tree

type ToolTreeProps = {
	readonly tools: readonly ToolNode[];
	readonly onCommand: (command: CommandDescriptor) => void;
};

function resolveLeafCommand(
	node: ToolLeaf | Extract<ToolNode, { readonly kind: "button" | "toggle" }>,
): CommandDescriptor | null {
	if ("onPress" in node && node.onPress) return node.onPress;
	if ("onChange" in node && node.onChange) return node.onChange;
	if (node.kind === "button" || node.kind === "toggle") {
		if (!node.command || !node.controllerId) return null;
		return { controllerId: node.controllerId, command: node.command, args: node.args as Record<string, unknown> | undefined };
	}
	return null;
}

function toolIcon(iconId: string): IconName {
	return iconId in ICONS ? (iconId as IconName) : "circle";
}

function renderToolLeaf(node: ToolNode, onCommand: (command: CommandDescriptor) => void): ReactElement | null {
	if (node.kind === "separator") return <ToolbarDivider key={node.id} />;
	if (node.kind === "button") {
		const command = resolveLeafCommand(node);
		if (!command) return null;
		return (
			<ToolbarItem key={node.id}>
				<ButtonGroupItem
					aria-label={node.title ?? node.label ?? node.id}
					title={node.title ?? node.label}
					disabled={node.disabled}
					onClick={() => onCommand(command)}
				>
					<Icon icon={toolIcon(node.iconId)} size="small" />
				</ButtonGroupItem>
			</ToolbarItem>
		);
	}
	if (node.kind === "toggle") {
		const command = resolveLeafCommand(node);
		if (!command) return null;
		return (
			<ToolbarItem key={node.id}>
				<Toggle
					aria-label={node.title ?? node.label ?? node.id}
					title={node.title ?? node.label}
					icon={<Icon icon={toolIcon(node.iconId)} size="small" />}
					pressed={node.pressed ?? false}
					disabled={node.disabled}
					onPressedChange={() => onCommand(command)}
				/>
			</ToolbarItem>
		);
	}
	return null;
}

function ToolCollection({
	node,
	onCommand,
}: {
	readonly node: Extract<ToolNode, { readonly kind: "collection" }>;
	readonly onCommand: (command: CommandDescriptor) => void;
}): ReactElement {
	const [open, setOpen] = useState(false);
	const leaves = node.children.filter((child) => child.kind !== "collection");
	return (
		<ToolbarGroup key={node.id}>
			<ToolbarItem>
				<Toggle
					aria-label={node.title ?? node.label ?? node.id}
					title={node.title ?? node.label}
					icon={<Icon icon={toolIcon(node.iconId)} size="small" />}
					pressed={open}
					disabled={node.disabled}
					onPressedChange={setOpen}
				/>
			</ToolbarItem>
			{open
				? leaves.map((child) => {
						if (child.kind === "separator") return <ToolbarDivider key={child.id} />;
						if (child.kind === "button") return renderToolLeaf(child, onCommand);
						if (child.kind === "toggle") return renderToolLeaf(child, onCommand);
						return null;
					})
				: null}
		</ToolbarGroup>
	);
}

export function ToolTree({ tools, onCommand }: ToolTreeProps): ReactElement | null {
	const content = useMemo(() => {
		if (!tools.length) return null;
		return (
			<ToolbarZone>
				<ButtonGroup>
					{tools.map((node) => {
						if (node.kind === "collection") {
							return <ToolCollection key={node.id} node={node} onCommand={onCommand} />;
						}
						return renderToolLeaf(node, onCommand);
					})}
				</ButtonGroup>
			</ToolbarZone>
		);
	}, [onCommand, tools]);
	return content;
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

let displayLayoutSaveLabel = "";

function groupNamedLayoutsToTreeItems(
	layouts: readonly NamedLayout[],
	onApply: (layoutId: string) => void,
	onDeleteUser?: (layoutId: string) => void,
): TreeDataItem[] {
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
					label: "Saved",
					defaultOpen: false,
					items: groupNamedLayoutsToTreeItems(userLayouts, (layoutId) => host.applyNamedLayout(layoutId), (layoutId) => host.deleteUserLayout(layoutId)),
				},
			]
		: [];
	return {
		sections: [
			{
				id: "framework.display.layout.save",
				label: "Save layout",
				defaultOpen: false,
				items: [
					{
						id: "framework.display.layout.save.label",
						label: "Name",
						control: (
							<Input
								id="framework.display.save-label"
								defaultValue={displayLayoutSaveLabel}
								onChange={(event) => {
									displayLayoutSaveLabel = event.target.value;
								}}
								placeholder="Layout name"
							/>
						),
					},
					{
						id: "framework.display.layout.save.action",
						label: "Save",
						control: (
							<Button
								id="framework.display.save"
								size="sm"
								text="Save current layout"
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
				label: "Layouts",
				defaultOpen: true,
				items: [...builtinItems, ...userItems],
			},
		],
	};
}

export function createFrameworkDisplayPanelTabs(getHost: () => DisplayHostApi | null): SidePanelTabConfig[] {
	return [
		{
			id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID,
			icon: shellTabIcon("framework.display.windows"),
			name: "Windows",
			order: -100,
			tree: {
				resolveTree: () => {
					const host = getHost();
					return host ? buildDisplayWindowsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: "Display unavailable" }] }] };
				},
			},
		},
		{
			id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID,
			icon: shellTabIcon("framework.display.layout"),
			name: "Layout",
			order: -99,
			tree: {
				resolveTree: () => {
					const host = getHost();
					return host ? buildDisplayLayoutTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: "Display unavailable" }] }] };
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
	readonly theme: string;
	readonly setTheme: (theme: string) => void;
};

function buildSettingsGeneralTree(host: SettingsHostApi): TreePanelConfig {
	return {
		sections: [
			...(host.appId || host.appLabel || host.controllerId || host.pluginId
				? [
						{
							id: "framework.settings.app",
							label: "App",
							defaultOpen: true,
							items: [
								...(host.appLabel
									? [{ id: "framework.settings.app.label", label: `Name: ${host.appLabel}` }]
									: []),
								...(host.appId ? [{ id: "framework.settings.app.id", label: `App id: ${host.appId}` }] : []),
								...(host.controllerId
									? [{ id: "framework.settings.app.controller", label: `Controller: ${host.controllerId}` }]
									: []),
								...(host.pluginId
									? [{ id: "framework.settings.app.plugin", label: `Plugin: ${host.pluginId}` }]
									: []),
							],
						},
					]
				: []),
			{
				id: "framework.settings.general",
				label: "General",
				defaultOpen: true,
				items: [
					{
						id: "framework.settings.theme",
						label: "Theme",
						control: (
							<select
								id="framework.settings.theme"
								className="h-small w-full rounded border border-border bg-background px-2 text-sm"
								value={host.theme}
								onChange={(event) => host.setTheme(event.target.value)}
							>
								<option value="system">System</option>
								<option value="light">Light</option>
								<option value="dark">Dark</option>
							</select>
						),
					},
					{
						id: "framework.settings.compact",
						label: "Compact UI",
						control: (
							<input
								id="framework.settings.compact"
								type="checkbox"
								checked={host.compact}
								onChange={(event) => host.setCompact(event.target.checked)}
							/>
						),
					},
					{
						id: "framework.settings.expertise",
						label: "Expertise",
						control: (
							<select
								id="framework.settings.expertise"
								className="h-small w-full rounded border border-border bg-background px-2 text-sm"
								value={host.expertise}
								onChange={(event) => host.setExpertise(event.target.value)}
							>
								<option value="beginner">Beginner</option>
								<option value="normal">Normal</option>
								<option value="expert">Expert</option>
							</select>
						),
					},
				],
			},
		],
	};
}

export function createFrameworkSettingsPanelTab(getHost: () => SettingsHostApi | null): SidePanelTabConfig {
	return {
		id: FRAMEWORK_SETTINGS_GENERAL_TAB_ID,
		icon: shellTabIcon("framework.settings.general"),
		name: "Settings",
		order: -98,
		tree: {
			resolveTree: () => {
				const host = getHost();
				return host ? buildSettingsGeneralTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: "Settings unavailable" }] }] };
			},
		},
	};
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
