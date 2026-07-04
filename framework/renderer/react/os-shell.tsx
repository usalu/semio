import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
	App,
	Breadcrumb,
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
	createEvenWindowLayout,
	getLevelBgClass,
	navbarFillItem,
	staticSidePanelTabDefinition,
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
} from "./types.ts";
import { interpretUiNode, uiTreeNodeToTreePanelConfig } from "./ui-interpreter.tsx";

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

function useUIHistory(initialUri = "/") {
	const [history, setHistory] = useState<UIHistory>({ entries: [{ uri: initialUri }], index: 0 });
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

	return { uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate };
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

function uriToBreadcrumbItems(uri: string, onNavigate: (href: string) => void) {
	if (uri === "/" || uri === "") {
		return [{ id: "breadcrumb.root", content: "Home", onNavigate: () => onNavigate("/") }];
	}
	const segments = uri.split("/").filter(Boolean);
	const items: { id: string; content: string; onNavigate: () => void }[] = [{ id: "breadcrumb.root", content: "Home", onNavigate: () => onNavigate("/") }];
	let path = "";
	for (const segment of segments) {
		path += `/${segment}`;
		const href = path;
		items.push({ id: `breadcrumb.${href}`, content: segment === "studios" ? "Studios" : segment, onNavigate: () => onNavigate(href) });
	}
	return items;
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
	const [spawnedWindowUi, setSpawnedWindowUi] = useState<UiNode | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [leftPanelVisible, setLeftPanelVisible] = useState(true);
	const [rightPanelVisible, setRightPanelVisible] = useState(true);
	const [leftPanelSize, setLeftPanelSize] = useState(DEFAULT_LEFT_PANEL_SIZE);
	const [rightPanelSize, setRightPanelSize] = useState(DEFAULT_RIGHT_PANEL_SIZE);
	const [activeWindowId, setActiveWindowId] = useState<string | null>(null);
	const [shellLayout, setShellLayout] = useState<WindowLayoutNode | null>(null);
	const [activeExampleId, setActiveExampleId] = useState("demo");
	const importStudioInputRef = useRef<HTMLInputElement>(null);
	const [uiTheme, setUiTheme] = useState<ElementsSurfaceTheme>(() => readStoredUiChromeTheme());
	const [uiCompact, setUiCompact] = useState(() => readStoredUiChromeCompact());
	const [uiExpertise, setUiExpertise] = useState(() => readStoredUiChromeExpertise());
	const { uri: shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate: navigateHistory } = useUIHistory("/");

	const registry = useMemo(() => {
		if (studioMode) return plugins;
		return pluginFilter ? plugins.filter((entry) => entry.pluginId === pluginFilter) : plugins;
	}, [pluginFilter, plugins, studioMode]);

	const panel = session ? parsePanelState(session.viewState) : null;

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
			const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId)?.handle;
			if (!plugin) return;
			const [windowNodes, ...panelNodes] = await Promise.all([
				...nextSession.app.windowKinds.map((kind) =>
					plugin.render(nextSession.instanceId, kind.bodyKey, nextSession.viewState),
				),
				...nextSession.app.panelTabs.map((tab) => plugin.render(nextSession.instanceId, tab.bodyKey, nextSession.viewState)),
			]);
			setWindowUiByKind(
				Object.fromEntries(
					nextSession.app.windowKinds.map((kind, index) => [kind.id, windowNodes[index]!]),
				),
			);
			setPanelUiByKey(Object.fromEntries(nextSession.app.panelTabs.map((tab, index) => [tab.id, panelNodes[index]!])));
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
			await refreshUi(nextSession);
			return nextSession;
		},
		[loadedPlugins, refreshUi],
	);

	const ensureSpawnedPlugin = useCallback(
		async (program: StudioProgramEntry, label?: string, osInstanceId?: string) => {
			const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
			if (!pluginEntry || !session) return;
			const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
			const existing = currentPanel.spawnedApps.find(
				(entry) => entry.appId === program.appId && entry.pluginId === program.pluginId,
			);
			if (existing) {
				updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, currentPanel.activePanelTab, existing.id));
				return;
			}
			const instanceId = await pluginEntry.handle.createApp(program.appId);
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
		[loadedPlugins, session, updateStudioPanel],
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
					filename?: string;
					mimeType?: string;
					data?: string;
				};
				if (op.op === "setPanel" && op.panel) {
					nextViewState = { ...nextViewState, panelJson: panelJsonFromState(op.panel) };
				}
				if (op.op === "navigate" && typeof op.uri === "string") {
					navigateHistory(op.uri);
					if (op.uri === "/" || op.uri === "") {
						await switchToSApp(S_HOME_APP_ID, nextViewState);
						return;
					}
					if (op.uri.startsWith("/studios/")) {
						const studioId = op.uri.split("/")[2];
						const studioSession = await switchToSApp(S_PLAY_APP_ID, nextViewState);
						if (studioId && studioSession) {
							const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === "s")?.handle;
							if (plugin) {
								const openOps = await plugin.handleCommand(
									studioSession.instanceId,
									JSON.stringify({ controllerId: S_PLAY_CONTROLLER_ID, command: "openStudio", args: { studioId } }),
									studioSession.viewState,
								);
								const documentOps = openOps.filter((row) => {
									const parsed = JSON.parse(row) as { op?: string };
									return parsed.op !== "navigate";
								});
								if (documentOps.length > 0) await processPluginOps(documentOps, studioSession);
								else await refreshUi(studioSession);
							}
						}
						return;
					}
				}
				if (op.op === "downloadMediaExport" && op.filename && op.mimeType && op.data) {
					downloadMediaExport(op.filename, op.mimeType, op.data);
				}
				if (op.op === "spawnPluginInstance" && op.programId && op.appId) {
					const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
					const program = currentPanel.programs.find((entry) => entry.programId === op.programId && entry.appId === op.appId)
						?? currentPanel.programs.find((entry) => entry.programId === op.programId);
					if (program) await ensureSpawnedPlugin(program, op.label, op.osInstanceId);
				}
				if (op.op === "openPluginInstance" && op.programId && op.appId) {
					const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
					const program = currentPanel.programs.find((entry) => entry.programId === op.programId)
						?? { pluginId: "", programId: op.programId, appId: op.appId, label: op.label ?? op.programId, yields: "" };
					if (program.pluginId) {
						await ensureSpawnedPlugin(program, op.label, op.osInstanceId);
					}
				}
			}
			const nextSession = { ...baseSession, viewState: nextViewState };
			if (nextSession.pluginId === session?.pluginId) {
				setSession((current) => (current ? { ...current, viewState: nextViewState } : current));
			}
			await refreshUi(nextSession);
		},
		[ensureSpawnedPlugin, loadedPlugins, navigateHistory, refreshUi, session?.pluginId, switchToSApp],
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

			if (studioMode && command.command === "spawnApp") {
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
				.then((ops) => processPluginOps(ops, targetSession))
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

	const activePanelTabId = panel?.activePanelTab ?? session?.app.panelTabs.find((tab) => panelSideForGroup(tab.group) === "left")?.id ?? session?.app.panelTabs[0]?.id;
	const activeLeftPanelTabId = studioMode && session?.app.id === S_PLAY_APP_ID ? (panel?.activePanelTab ?? S_PLAY_CATALOGUE_TAB_ID) : FRAMEWORK_PANEL_TAB_HIERARCHY_ID;

	const leftPanelTabs = useMemo((): SidePanelTabConfig[] => {
		if (!session) return [];
		const pluginLeftTabs = session.app.panelTabs
			.filter((tab) => panelSideForGroup(tab.group) === "left")
			.map((tab, order) =>
				staticSidePanelTabDefinition({
					id: tab.id,
					icon: panelTabIcon(tab.id, tab.group),
					name: tab.label,
					order,
					tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tab.id] ?? { type: "text", value: "Loading…" }, onCommand)),
				}),
			);
		if (studioMode && session.app.id === S_PLAY_APP_ID && pluginLeftTabs.length > 0) return pluginLeftTabs;
		const hierarchyTab: SidePanelTabConfig = staticSidePanelTabDefinition({
			id: FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
			icon: shellTabIcon(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
			order: 0,
			tree: staticTreePanelDefinition({
				sections: [{ id: "hierarchy.root", label: "Scene", items: [{ id: "hierarchy.empty", label: studioMode ? `${panel?.spawnedApps.length ?? 0} spawned app(s)` : "—" }] }],
			}),
		});
		return [hierarchyTab, ...pluginLeftTabs];
	}, [onCommand, panel?.spawnedApps.length, panelUiByKey, session, studioMode]);

	const rightPanelTabs = useMemo((): SidePanelTabConfig[] => {
		if (!session) return [];
		return session.app.panelTabs
			.filter((tab) => panelSideForGroup(tab.group) === "right")
			.map((tab, order) =>
				staticSidePanelTabDefinition({
					id: tab.id,
					icon: panelTabIcon(tab.id, tab.group),
					name: tab.label,
					order,
					tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tab.id] ?? { type: "text", value: "Loading…" }, onCommand)),
				}),
			);
	}, [onCommand, panelUiByKey, session]);

	const panelToggles = useMemo((): PanelToggleItem[] => {
		const items: PanelToggleItem[] = [];
		if (leftPanelTabs.length > 0) {
			items.push({
				id: "framework.panel.left",
				icon: <Icon icon="panel-left" size="small" />,
				pressed: leftPanelVisible,
				onPressedChange: setLeftPanelVisible,
			});
		}
		if (rightPanelTabs.length > 0) {
			items.push({
				id: "framework.panel.right",
				icon: <Icon icon="panel-right" size="small" />,
				pressed: rightPanelVisible,
				onPressedChange: setRightPanelVisible,
			});
		}
		return items;
	}, [leftPanelTabs.length, leftPanelVisible, rightPanelTabs.length, rightPanelVisible]);

	const sPluginManifest = useMemo(() => loadedPlugins.find((entry) => entry.handle.pluginId === "s")?.manifest, [loadedPlugins]);
	const exampleOptions = useMemo(
		() => (sPluginManifest?.examples ?? []).map((example) => ({ id: example.id, label: example.label })),
		[sPluginManifest],
	);

	const navigateFromBreadcrumb = useCallback(
		(href: string) => {
			navigateHistory(href);
			if (href === "/" || href === "") void switchToSApp(S_HOME_APP_ID);
			else if (href.startsWith("/studios/")) {
				const studioId = href.split("/")[2];
				void switchToSApp(S_PLAY_APP_ID).then((studioSession) => {
					if (studioId && studioSession) {
						onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "openStudio", args: { studioId } });
					}
				});
			}
		},
		[navigateHistory, onCommand, switchToSApp],
	);

	const navbarItems = useMemo((): NavbarItem[] => {
		const items: NavbarItem[] = [
			{
				key: "logo",
				content: (
					<div className="flex items-center gap-single">
						<SemioLogo className="size-workbench shrink-0" />
						<span data-slot="app-name" className="px-single text-sm font-semibold">{session?.app.label ?? (studioMode ? "S Studio" : "semio os")}</span>
					</div>
				),
			},
			{
				key: "navHistory",
				content: (
					<ButtonGroup id="ui.nav">
						<ButtonGroupItem id="ui.nav.back" onClick={goBack} className={canGoBack ? "" : "pointer-events-none opacity-30"} icon={<Icon icon="arrow-left" size="small" />} />
						<ButtonGroupItem id="ui.nav.forward" onClick={goForward} className={canGoForward ? "" : "pointer-events-none opacity-30"} icon={<Icon icon="arrow-right" size="small" />} />
						<ButtonGroupItem id="ui.nav.up" onClick={goUp} className={canGoUp ? "" : "pointer-events-none opacity-30"} icon={<Icon icon="arrow-up" size="small" />} />
					</ButtonGroup>
				),
			},
			{
				key: "breadcrumb",
				className: navbarFillItem().className,
				content: <Breadcrumb className="min-w-0" items={uriToBreadcrumbItems(shellUri, navigateFromBreadcrumb)} />,
			},
		];
		if (studioMode && session?.app.id === S_PLAY_APP_ID && exampleOptions.length > 0) {
			items.push({
				key: "example",
				content: (
					<NavbarExampleSelect
						id="s.navbar.example"
						value={activeExampleId}
						options={exampleOptions}
						onValueChange={(exampleId) => {
							setActiveExampleId(exampleId);
							onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "setActiveExample", args: { exampleId } });
						}}
					/>
				),
			});
		}
		items.push(navbarFillItem());
		if (panelToggles.length > 0) {
			items.push({ key: "panel-toggles", content: <PanelToggleGroup items={panelToggles} /> });
		}
		items.push({
			key: "theme",
			content: (
				<Select value={uiTheme} onValueChange={(value) => setUiTheme(value as ElementsSurfaceTheme)}>
					<SelectTrigger className="h-medium w-[7rem]">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="system">System</SelectItem>
						<SelectItem value="light">Light</SelectItem>
						<SelectItem value="dark">Dark</SelectItem>
					</SelectContent>
				</Select>
			),
		});
		items.push({
			key: "compact",
			content: <Toggle id="ui-compact" text="Compact" pressed={uiCompact} onPressedChange={setUiCompact} />,
		});
		items.push({
			key: "expertise",
			content: (
				<Select value={uiExpertise} onValueChange={(value) => setUiExpertise(value as Expertise)}>
					<SelectTrigger className="h-medium w-[7rem]">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value={Expertise.BEGINNER}>Beginner</SelectItem>
						<SelectItem value={Expertise.NORMAL}>Normal</SelectItem>
						<SelectItem value={Expertise.EXPERT}>Expert</SelectItem>
					</SelectContent>
				</Select>
			),
		});
		if (studioMode) {
			items.push({
				key: "uri",
				content: (
					<span className="text-muted-foreground max-w-[12rem] truncate text-xs" title={shellUri}>
						{shellUri}
					</span>
				),
			});
		}
		return items;
	}, [activeExampleId, canGoBack, canGoForward, canGoUp, exampleOptions, goBack, goForward, goUp, navigateFromBreadcrumb, onCommand, panelToggles, session?.app.id, session?.app.label, shellUri, studioMode, uiCompact, uiExpertise, uiTheme]);

	const footerItems = useMemo((): FooterItem[] => {
		if (!session) return [];
		return [
			{
				id: "framework.footer.app",
				text: session.app.label,
				icon: <Icon icon={session.app.iconId && session.app.iconId in ICONS ? (session.app.iconId as IconName) : "app-window"} size="small" />,
			},
		];
	}, [session]);

	const modeWindows = useMemo((): ModeWindowDescriptor[] => {
		if (!session || Object.keys(windowUiByKind).length === 0) return [];
		const windows: ModeWindowDescriptor[] = session.app.windowKinds.map((kind) => ({
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
		if (studioMode && spawnedWindowUi && panel?.activeSpawnedId) {
			const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
			if (spawned) {
				windows.push({
					id: spawned.id,
					title: spawned.label,
					fill: true,
					showControls: true,
					children: (
						<ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
							{interpretUiNode(spawnedWindowUi, { onCommand })}
						</ChromeAwareWindowScrollSurface>
					),
				});
			}
		}
		return windows;
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
				<button type="button" className="hover:text-foreground" onClick={() => updateStudioPanel(buildStudioPanelState(panel.programs, panel.spawnedApps, panel.activePanelTab, undefined))}>
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
						chrome={modes.length > 1}
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
		<LevelProvider level="window">
			<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
				<Layout
					mobile={mobile}
					navbar={<Navbar items={navbarItems} showFullscreenToggle />}
					footer={<Footer items={footerItems} />}
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
											onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "setActivePanelTab", args: { tabId } });
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
									activeTabId: activePanelTabId,
									onActiveTabChange: (tabId) => onCommand({ controllerId: S_PLAY_CONTROLLER_ID, command: "setActivePanelTab", args: { tabId } }),
								}
							: undefined
					}
					canvas={canvas}
				/>
			</div>
		</LevelProvider>
	);
}
//#endregion FrameworkOsShell
