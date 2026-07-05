import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
	App,
	ButtonGroup,
	ButtonGroupItem,
	ChromeAwareWindowScrollSurface,
	Footer,
	Icon,
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
	WindowMeasureTreeGroup,
	WindowMeasureTreeLeaf,
	WindowMeasuresTree,
	bootstrapElementsSurfaceChromeDocument,
	cn,
	createEvenWindowLayout,
	getLevelBgClass,
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
	Expertise,
	type ElementsSurfaceTheme,
	type EngagementControl,
	type EngagementSpec,
	type FooterItem,
	type ModeWindowDescriptor,
	type NavbarItem,
	type PanelToggleItem,
	type SidePanelTabConfig,
	type TreePanelConfig,
	type WindowLayoutNode,
} from "@semio-tech/ui-react";
import { ICONS, type IconName } from "@semio-tech/ui-asset";
import { DEFAULT_PLUGIN_REGISTRY, loadPluginModule, type PluginWasmHandle } from "./plugin-runtime.ts";
import {
	FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
	FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
	FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
	FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
	type AppDefinition,
	type CommandDescriptor,
	type PluginManifest,
	type UiNode,
	type UiTreeNode,
	type ViewState,
	type WindowEngagement,
	type WindowEngagementControl,
	type WindowLayout,
	type WindowLayoutAxisNode,
	type WindowLayoutStackNode,
	type WindowLayoutWindowNode,
	type WindowMeasure,
	type ToolNode,
} from "./types.ts";
import { interpretUiNode, uiTreeNodeToTreePanelConfig } from "./ui-interpreter.tsx";
import { ToolTree } from "./tool-tree.tsx";
import { UISearch, UIFind, UIFindProvider } from "./ui-search-find.tsx";
import {
	createFrameworkDisplayPanelTabs,
	createFrameworkSettingsPanelTab,
	useNamedLayoutHost,
	type SettingsHostApi,
} from "./os-chrome-panels.tsx";
import {
	NamedLayoutStore,
	createBrowserStoragePort,
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
	readonly yields: string;
};

type SpawnedAppEntry = {
	readonly id: string;
	readonly pluginId: string;
	readonly instanceId: number;
	readonly appId: string;
	readonly label: string;
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
			yields: program.yields,
		})),
	);
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
	if (group === "workbench" || group === "hierarchy" || group === "display") return "left";
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
		const iconName = iconId in ICONS ? (iconId as IconName) : "circle-dot";
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
	if (!measures?.length) return undefined;
	return (
		<WindowMeasuresTree>
			{measures.map((measure) => renderWindowMeasure(measure, onCommand))}
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
		if (studioMode) return plugins;
		return pluginFilter ? plugins.filter((entry) => entry.pluginId === pluginFilter) : plugins;
	}, [pluginFilter, plugins, studioMode]);

	const panel = session ? parsePanelState(session.viewState) : null;

	useEffect(() => {
		sessionRef.current = session;
	}, [session]);

	useEffect(() => {
		let cancelled = false;
		void (async () => {
			try {
				const loaded = await Promise.all(registry.map((entry) => loadPluginModule(entry.pluginId, entry.moduleUrl)));
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
			const windowCount = nextSession.app.windowKinds.length;
			const rendered = await Promise.all([
				...nextSession.app.windowKinds.map((kind) =>
					plugin.render(nextSession.instanceId, kind.bodyKey, nextSession.viewState),
				),
				...nextSession.app.panelTabs.map((tab) => plugin.render(nextSession.instanceId, tab.bodyKey, nextSession.viewState)),
				plugin.tools(nextSession.instanceId, nextSession.viewState),
			]);
			if (generation !== refreshGenerationRef.current) return;
			const windowNodes = rendered.slice(0, windowCount);
			const panelNodes = rendered.slice(windowCount, windowCount + nextSession.app.panelTabs.length);
			const dynamicTools = rendered[rendered.length - 1] as readonly ToolNode[];
			setWindowUiByKind(
				Object.fromEntries(
					nextSession.app.windowKinds.map((kind, index) => [kind.id, windowNodes[index]!]),
				),
			);
			setPanelUiByKey(Object.fromEntries(nextSession.app.panelTabs.map((tab, index) => [tab.id, panelNodes[index]!])));
			const activeModeId = nextSession.viewState.activeModeId ?? nextSession.app.defaultModeId ?? nextSession.app.modes[0]?.id;
			const staticTools = nextSession.app.modes.find((mode) => mode.id === activeModeId)?.tools ?? [];
			setActiveToolNodes(dynamicTools.length > 0 ? dynamicTools : staticTools);
			const windowIds = nextSession.app.windowKinds.map((kind) => kind.id);
			setShellLayout(convertFrameworkLayoutToModeLayout(nextSession.app.defaultLayout, windowIds));
			const defaultWindowId = findDefaultActiveWindowKindId(nextSession.app.defaultLayout, nextSession.app.windowKinds);
			if (defaultWindowId) setActiveWindowId(defaultWindowId);
			else if (!activeWindowId && windowIds[0]) setActiveWindowId(windowIds[0]);
		},
		[activeWindowId, loadedPlugins],
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
						{ id: spawnedId, pluginId: program.pluginId, instanceId, appId: program.appId, label: label ?? program.label },
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
					const program = currentPanel.programs.find((entry) => entry.programId === op.programId)
						?? { pluginId: "", programId: op.programId, appId: op.appId, label: op.label ?? op.programId, yields: "" };
					if (program.pluginId) {
						await ensureSpawnedPlugin(program, op.label, op.osInstanceId, op.documentJson);
					}
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
						{ id: spawnedId, pluginId: program.pluginId, instanceId, appId: program.appId, label: program.label },
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
			setShellLayout(convertFrameworkLayoutToModeLayout(layout, windowIds));
			const defaultWindowId = findDefaultActiveWindowKindId(layout, session.app.windowKinds);
			if (defaultWindowId) setActiveWindowId(defaultWindowId);
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
			compact: uiCompact,
			setCompact: setUiCompact,
			expertise: uiExpertise,
			setExpertise: setUiExpertise,
			theme: uiTheme,
			setTheme: setUiTheme,
		}),
		[uiCompact, uiExpertise, uiTheme],
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
		const matches = (event: KeyboardEvent, binding: string) => {
			const parts = binding.split("+").map((part) => part.trim());
			const key = parts[parts.length - 1] ?? "";
			const needsCtrl = parts.includes("ctrl") || parts.includes("meta");
			const needsShift = parts.includes("shift");
			const needsAlt = parts.includes("alt");
			if (needsCtrl && !(event.ctrlKey || event.metaKey)) return false;
			if (needsShift && !event.shiftKey) return false;
			if (needsAlt && !event.altKey) return false;
			return event.key.toLowerCase() === key;
		};
		const onKeyDown = (event: KeyboardEvent) => {
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
		const hasPluginHierarchyTab = pluginLeftTabs.some((tab) => tab.id === FRAMEWORK_PANEL_TAB_HIERARCHY_ID);
		if (hasPluginHierarchyTab) return pluginLeftTabs;
		const hierarchyTab: SidePanelTabConfig = {
			id: FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
			icon: shellTabIcon(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
			order: 0,
			tree: staticTreePanelDefinition({
				sections: [{ id: "hierarchy.root", label: "Scene", items: [{ id: "hierarchy.empty", label: studioMode ? `${panel?.spawnedApps.length ?? 0} spawned app(s)` : "—" }] }],
			}),
		};
		return [hierarchyTab, ...pluginLeftTabs];
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
		if (activeLeftPanelKind === "display") return frameworkDisplayTabs[0]?.id ?? FRAMEWORK_PANEL_TAB_HIERARCHY_ID;
		if (studioMode && session?.app.id === S_PLAY_APP_ID) return panel?.activePanelTab ?? S_PLAY_CATALOGUE_TAB_ID;
		return workbenchLeftTabs[0]?.id ?? FRAMEWORK_PANEL_TAB_HIERARCHY_ID;
	}, [activeLeftPanelKind, frameworkDisplayTabs, panel?.activePanelTab, session?.app.id, studioMode, workbenchLeftTabs]);

	const activeRightPanelTabId = useMemo(() => {
		if (activeRightPanelKind === "settings") return settingsRightTabs[0]?.id;
		if (panel?.activePanelTab && detailsRightTabs.some((tab) => tab.id === panel.activePanelTab)) return panel.activePanelTab;
		return detailsRightTabs[0]?.id ?? activePanelTabId;
	}, [activePanelTabId, activeRightPanelKind, detailsRightTabs, panel?.activePanelTab, settingsRightTabs]);

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
						<span data-slot="app-name" className={cn("px-single", shellChromeTitleClassName)}>{session.app.label}</span>
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
		const items: import("./ui-search-find.tsx").UISearchItem[] = [];
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
					label: `Spawn ${program.label}`,
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
		const items: FooterItem[] = [
			{
				id: "framework.footer.app",
				text: session.app.label,
				icon: <Icon icon={session.app.iconId && session.app.iconId in ICONS ? (session.app.iconId as IconName) : "app-window"} size="small" />,
			},
		];
		if (studioMode && session.app.controllerId === S_PLAY_CONTROLLER_ID) {
			items.push(
				{
					id: "framework.footer.undo",
					text: "Undo",
					icon: <Icon icon="undo-2" size="small" />,
					onClick: () => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "undo" }),
				},
				{
					id: "framework.footer.redo",
					text: "Redo",
					icon: <Icon icon="redo-2" size="small" />,
					onClick: () => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "redo" }),
				},
				{
					id: "framework.footer.checkpoint",
					text: "Checkpoint",
					icon: <Icon icon="git-commit-horizontal" size="small" />,
					onClick: () => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "commitCheckpoint" }),
				},
			);
		}
		return items;
	}, [onCommand, session, studioMode]);

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
						title: spawned.label,
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
		return session.app.windowKinds.map((kind) => ({
			id: kind.id,
			title: kind.label,
			fill: true,
			showControls: true,
			measures: windowMeasuresOverlay(kind.measures, onCommand),
			engagement: windowEngagementToSpec(kind.engagement, onCommand),
			children: (
				<ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" data-window-kind-id={kind.id}>
					{interpretUiNode(windowUiByKind[kind.id] ?? { type: "text", value: `Missing window: ${kind.id}` }, { onCommand })}
				</ChromeAwareWindowScrollSurface>
			),
		}));
	}, [onCommand, panel, session, spawnedWindowUi, studioMode, windowUiByKind]);

	const canvas = useMemo(() => {
		if (!session) return <p className="p-4 text-sm text-muted-foreground">Loading plugins…</p>;
		if (error) return <p className="p-4 text-sm text-destructive" role="alert">{error}</p>;
		const modes = session.app.modes.length > 0 ? session.app.modes : [{ id: session.app.id, label: session.app.label }];
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
				<span>{focusedSpawned.label}</span>
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
							onWindowClose={(windowId) => {
								if (studioMode && panel?.spawnedApps.some((entry) => entry.id === windowId)) {
									const nextSpawned = panel.spawnedApps.filter((entry) => entry.id !== windowId);
									updateStudioPanel(buildStudioPanelState(panel.programs, nextSpawned, panel.activePanelTab, nextSpawned[0]?.id));
								}
								setShellLayout((current) => current ?? convertFrameworkLayoutToModeLayout(session.app.defaultLayout, modeWindows.map((window) => window.id)));
							}}
						/>
					</App>
				</div>
			</div>
		);
	}, [activeWindowId, error, modeWindows, onCommand, panel, session, shellLayout, studioMode, updateStudioPanel]);

	return (
		<UIFindProvider>
			<LevelProvider level="window">
				<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
					<Layout
						mobile={mobile}
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
										activeTabId: activeLeftPanelTabId,
										onActiveTabChange: (tabId) => {
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
										activeTabId: activeRightPanelTabId,
										onActiveTabChange: (tabId) => {
											if (studioMode && session?.app.id === S_PLAY_APP_ID) {
												onCommand({ controllerId: session.app.controllerId, command: "setActivePanelTab", args: { tabId } });
											}
										},
									}
								: undefined
						}
						canvas={canvas}
					/>
				</div>
				<UISearch items={searchItems} open={searchOpen} onOpenChange={setSearchOpen} />
				<UIFind open={findOpen} onOpenChange={setFindOpen} />
			</LevelProvider>
		</UIFindProvider>
	);
}
//#endregion FrameworkOsShell
