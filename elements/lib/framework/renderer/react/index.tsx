// #region 🧲Header
/** @emoji ⚛️ `@elements/framework-react` — React renderer for {@link @elements/framework}: declarative {@link UiNode} host (monolith). */
// #endregion 🧲Header

export type { Workbench } from "@elements/framework";

export type { Level } from "@elements/ui";
export {
	LevelProvider,
	useLevel,
	getLevelBgClass,
	getLevelHoverClass,
	getLevelActiveHoverClass,
	getLevelZClass,
	getLevelBorderElementClass,
	getLevelDivideElementClass,
} from "@elements/ui";

//#region 📦shell-chrome-types.tsx
import type * as React from "react";

/** @emoji 👣 Footer row rendered by the workbench shell. */
export interface FooterItem {
	readonly id: string;
	readonly icon?: React.ReactNode;
	readonly text?: string;
	readonly content?: React.ReactNode;
	readonly order?: number;
	readonly onClick?: () => void;
	readonly className?: string;
	readonly disabled?: boolean;
}

/** @emoji 🌲 Minimal tree panel payload for declarative side tabs. */
export interface ShellChromeTreePanelConfig {
	readonly sections: readonly { readonly id: string; readonly content: React.ReactNode }[];
}

/** @emoji 📑 Side panel tab registration consumed by {@link WorkbenchView}. */
export interface SidePanelTabConfig {
	readonly id: string;
	readonly icon: React.ComponentType<{ readonly size?: number }>;
	readonly order?: number;
	readonly tree: ShellChromeTreePanelConfig;
}

/** @emoji 📐 Floating window measure/control descriptors (golden-layout chrome). */
export type UIWindowMeasure =
	| { readonly kind: "display"; readonly id: string; readonly label?: string; readonly content: React.ReactNode }
	| { readonly kind: "reading"; readonly id: string; readonly label?: string; readonly text: string; readonly monospace?: boolean }
	| { readonly kind: "section"; readonly id: string; readonly title: string }
	| { readonly kind: "separator"; readonly id: string }
	| {
			readonly kind: "toggle";
			readonly id: string;
			readonly label?: string;
			readonly pressed?: boolean;
			readonly defaultPressed?: boolean;
			readonly icon?: React.ReactNode;
			readonly text?: string;
			readonly onPressedChange?: (pressed: boolean) => void;
	  }
	| {
			readonly kind: "select";
			readonly id: string;
			readonly label?: string;
			readonly value?: string;
			readonly defaultValue?: string;
			readonly items: readonly { readonly id: string; readonly value: string; readonly label: string }[];
			readonly onValueChange?: (value: string) => void;
	  }
	| {
			readonly kind: "combobox";
			readonly id: string;
			readonly label?: string;
			readonly value?: string;
			readonly placeholder?: string;
			readonly choices: readonly { readonly value: string; readonly label: string }[];
			readonly onValueChange?: (value: string) => void;
	  }
	| { readonly kind: "button"; readonly id: string; readonly label?: string; readonly text: string; readonly icon?: React.ReactNode; readonly onClick?: () => void }
	| {
			readonly kind: "buttonCycle";
			readonly id: string;
			readonly label?: string;
			readonly value?: string;
			readonly items: readonly { readonly value: string; readonly label: string; readonly icon?: React.ReactNode; readonly text?: string; readonly id?: string }[];
			readonly onValueChange?: (value: string) => void;
	  }
	| { readonly kind: "input"; readonly id: string; readonly label?: string; readonly value?: string; readonly placeholder?: string; readonly onLazyChange?: (value: string) => void }
	| { readonly kind: "textarea"; readonly id: string; readonly label?: string; readonly value?: string; readonly placeholder?: string; readonly rows?: number; readonly onLazyChange?: (value: string) => void }
	| { readonly kind: "checkbox"; readonly id: string; readonly label?: string; readonly checked?: boolean; readonly defaultChecked?: boolean; readonly onCheckedChange?: (checked: boolean) => void }
	| { readonly kind: "radio"; readonly id: string; readonly label?: string; readonly value: string; readonly items: readonly { readonly value: string; readonly label: string }[]; readonly onChange?: (value: string) => void }
	| { readonly kind: "slider"; readonly id: string; readonly label?: string; readonly value?: number; readonly min?: number; readonly max?: number; readonly step?: number; readonly onValueChange?: (value: number) => void }
	| { readonly kind: "number"; readonly id: string; readonly label?: string; readonly value?: number; readonly min?: number; readonly max?: number; readonly step?: number; readonly onChange?: (value: number) => void }
	| { readonly kind: "color"; readonly id: string; readonly label?: string; readonly value?: string; readonly onChange?: (value: string) => void };

/** @emoji 🪟 Golden-layout window kind registration. */
export interface UIWindowKindDefinition {
	readonly id: string;
	readonly label?: string;
	readonly icon?: React.ReactNode;
	readonly component: React.ComponentType;
	readonly measures?: readonly UIWindowMeasure[];
}

/** @emoji 🧰 Toolbar item for a single category slot. */
export interface UIToolbarItem {
	readonly id: string;
	readonly icon?: React.ReactNode;
	readonly label?: string;
	readonly text?: string;
	readonly onClick?: () => void;
	readonly kind?: "button" | "toggle" | "separator";
	readonly pressed?: boolean;
	readonly onPressedChange?: (pressed: boolean) => void;
	readonly order?: number;
}

/** @emoji 🧰 Toolbar category ids shared by framework shell tool maps. */
export type AppToolCategory = "history" | "hand" | "selection" | "lasso" | "filter" | "open" | "create" | "view" | "actions" | "settings";

/** @emoji 📋 Default toolbar category order. */
export const APP_TOOL_CATEGORY_ORDER: readonly AppToolCategory[] = [
	"history",
	"hand",
	"selection",
	"lasso",
	"filter",
	"open",
	"create",
	"view",
	"actions",
	"settings",
];

/** @emoji 🗂️ Per-category toolbar tools for the workbench shell. */
export type AppTools = Partial<Record<AppToolCategory, readonly UIToolbarItem[]>>;

//#endregion 📦shell-chrome-types.tsx

//#region 📦workbench-app-context.tsx
/** @emoji 🧭 Props for {@link WorkbenchView} (navbar, panels, golden-layout canvas). */
export interface WorkbenchViewProps {
	workbench: Workbench;
	defaultAppId?: string;
	uri?: string;
	onNavigate?: (uri: string) => void;
	canGoBack?: boolean;
	onGoBack?: () => void;
	canGoForward?: boolean;
	onGoForward?: () => void;
	canGoUp?: boolean;
	onGoUp?: () => void;
	mobile?: boolean;
	mobileQuery?: string;
	className?: string;
	resolvedWindowKindsOverride?: UIWindowKindDefinition[];
	slotToolbar?: React.ReactNode;
	extraFooterItems?: FooterItem[];
	augmentPanelTabs?: Partial<Record<"workbench" | "details", SidePanelTabConfig[]>>;
	initialPanelVisibility?: UIPanelVisibility;
}

/** @emoji 🧭 @deprecated Use {@link WorkbenchViewProps}. */
export type AppProps = WorkbenchViewProps;

export interface UIPanelVisibility {
	leftSidePanel: boolean;
	rightSidePanel: boolean;
}

export interface AppContextValue {
	workbench: Workbench;
	activeAppId: string;
	setActiveAppId: (id: string) => void;
	activeApp: ResolvedWorkbenchAppState;
	activeModeId: string | null;
	setActiveModeId: (id: string) => void;
	apps: WorkbenchApp[];
	panelVisibility: UIPanelVisibility;
	togglePanel: (panel: keyof UIPanelVisibility) => void;
	uri: string;
	navigate: (uri: string) => void;
	canGoBack: boolean;
	goBack: () => void;
	canGoForward: boolean;
	goForward: () => void;
	canGoUp: boolean;
	goUp: () => void;
}

export const AppContext = React.createContext<AppContextValue | undefined>(undefined);

/** @emoji 🪝 Returns the active {@link Workbench} shell context from the nearest {@link AppContext}. */
export function useApp(): AppContextValue {
	const ctx = React.useContext(AppContext);
	if (!ctx) throw new Error("useApp must be used within a WorkbenchView");
	return ctx;
}

//#endregion 📦workbench-app-context.tsx

//#region 📦workbench-history.tsx
import * as React from "react";

/** @emoji 🔖 Single URI stack entry. */
export interface UIHistoryEntry {
	readonly uri: string;
}

/** @emoji 🔖 URI navigation stack state. */
export interface UIHistory {
	readonly entries: readonly UIHistoryEntry[];
	readonly index: number;
}

/** @emoji 🧭 Manages URI history with back, forward, up, and navigate. */
export function useUIHistory(initialUri = "/"): {
	readonly history: UIHistory;
	readonly uri: string;
	readonly canGoBack: boolean;
	readonly canGoForward: boolean;
	readonly canGoUp: boolean;
	readonly parentUri: string | null;
	readonly goBack: () => void;
	readonly goForward: () => void;
	readonly goUp: () => void;
	readonly navigate: (uri: string) => void;
} {
	const [history, setHistory] = React.useState<UIHistory>({
		entries: [{ uri: initialUri }],
		index: 0,
	});
	const uri = history.entries[history.index]?.uri ?? initialUri;
	const canGoBack = history.index > 0;
	const canGoForward = history.index < history.entries.length - 1;
	const segments = uri.split("/").filter(Boolean);
	const canGoUp = segments.length > 0;
	const parentUri = canGoUp ? `/${segments.slice(0, -1).join("/")}` : null;

	const goBack = React.useCallback(() => {
		setHistory((prev) => (prev.index > 0 ? { ...prev, index: prev.index - 1 } : prev));
	}, []);
	const goForward = React.useCallback(() => {
		setHistory((prev) => (prev.index < prev.entries.length - 1 ? { ...prev, index: prev.index + 1 } : prev));
	}, []);
	const goUp = React.useCallback(() => {
		if (!canGoUp || parentUri === null) return;
		setHistory((prev) => {
			const newEntries = prev.entries.slice(0, prev.index + 1);
			return { entries: [...newEntries, { uri: parentUri }], index: newEntries.length };
		});
	}, [canGoUp, parentUri]);
	const navigate = React.useCallback((targetUri: string) => {
		setHistory((prev) => {
			const newEntries = prev.entries.slice(0, prev.index + 1);
			return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
		});
	}, []);

	return { history, uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate };
}

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("useUIHistory types", () => {
		it("exports history entry shape", () => {
			const entry: UIHistoryEntry = { uri: "/test" };
			expect(entry.uri).toBe("/test");
		});
	});
}
//#endregion 🧪Tests

//#endregion 📦workbench-history.tsx

//#region 📦ui-declarative-renderer.tsx
import { CommandBus, Controller } from "@elements/framework";
import { renderToStaticMarkup } from "react-dom/server";
import type {
	UiBoardHostSurfaceNode,
	UiButtonNode,
	UiNode,
	UiPanelHostSurfaceNode,
	UiTableHostSurfaceNode,
	UiScene3DHostSurfaceNode,
	UiSeparatorNode,
	UiStackNode,
	UiTextNode,
} from "@elements/framework";
import { clsx, type ClassValue } from "clsx";
import * as React from "react";
import { twMerge } from "tailwind-merge";

function cn(...inputs: ClassValue[]): string {
	return twMerge(clsx(inputs));
}

//#region 🔖Scene3DRegistry
type Scene3DSurfaceHost = React.ComponentType<{ readonly node: UiScene3DHostSurfaceNode }>;

const scene3dSurfaceHosts = new Map<string, Scene3DSurfaceHost>();

/** @emoji 🧭 Binds a `surfaceId` from {@link UiScene3DHostSurfaceNode} to a host React canvas implementation. */
export function registerUiScene3DSurfaceHost(surfaceId: string, Component: Scene3DSurfaceHost): void {
	scene3dSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 🧹 Drops a surface binding (tests). */
export function unregisterUiScene3DSurfaceHost(surfaceId: string): void {
	scene3dSurfaceHosts.delete(surfaceId);
}
//#endregion 🔖Scene3DRegistry

//#region 🔖BoardRegistry
type BoardSurfaceHost = React.ComponentType<{ readonly node: UiBoardHostSurfaceNode }>;

const boardSurfaceHosts = new Map<string, BoardSurfaceHost>();

/** @emoji 📋 Binds `surfaceId` from {@link UiBoardHostSurfaceNode} to a host board canvas. */
export function registerUiBoardSurfaceHost(surfaceId: string, Component: BoardSurfaceHost): void {
	boardSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 🧹 Drops a board surface binding (tests). */
export function unregisterUiBoardSurfaceHost(surfaceId: string): void {
	boardSurfaceHosts.delete(surfaceId);
}
//#endregion 🔖BoardRegistry

//#region 🔖TableRegistry
type TableSurfaceHost = React.ComponentType<{ readonly node: UiTableHostSurfaceNode }>;

const tableSurfaceHosts = new Map<string, TableSurfaceHost>();

/** @emoji 📑 Binds `surfaceId` from {@link UiTableHostSurfaceNode} to a host table body. */
export function registerUiTableSurfaceHost(surfaceId: string, Component: TableSurfaceHost): void {
	tableSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 🧹 Drops a table surface binding (tests). */
export function unregisterUiTableSurfaceHost(surfaceId: string): void {
	tableSurfaceHosts.delete(surfaceId);
}
//#endregion 🔖TableRegistry

//#region 🔖PanelRegistry
type PanelSurfaceHost = React.ComponentType<{ readonly node: UiPanelHostSurfaceNode }>;

const panelSurfaceHosts = new Map<string, PanelSurfaceHost>();

/** @emoji 🧩 Binds `surfaceId` from {@link UiPanelHostSurfaceNode} to a host side-panel body. */
export function registerUiPanelSurfaceHost(surfaceId: string, Component: PanelSurfaceHost): void {
	panelSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 🧹 Drops a panel surface binding (tests). */
export function unregisterUiPanelSurfaceHost(surfaceId: string): void {
	panelSurfaceHosts.delete(surfaceId);
}
//#endregion 🔖PanelRegistry

//#region 🔖StackLayout
function stackClass(spec: UiStackNode): string {
	const dir = spec.direction === "horizontal" ? "flex-row" : "flex-col";
	const gap =
		spec.gap === "none"
			? "gap-0"
			: spec.gap === "tight"
				? "gap-1"
				: spec.gap === "relaxed"
					? "gap-4"
					: "gap-2";
	const pad = spec.padding === "none" ? "p-0" : "p-2";
	return cn("flex", dir, gap, pad, spec.direction === "vertical" ? "min-h-0 min-w-0" : "min-w-0");
}
//#endregion 🔖StackLayout

//#region 🔖Renderer
export interface UiRendererProps {
	readonly node: UiNode;
	readonly commandBus: CommandBus;
}

function renderText(node: UiTextNode): React.ReactElement {
	const dataProps = node.dataAttributes
		? Object.fromEntries(Object.entries(node.dataAttributes).map(([k, v]) => [`data-${k}`, v]))
		: {};
	return (
		<span
			className={cn(
				"text-muted-foreground px-1 text-xs",
				node.emphasize && "font-semibold uppercase tracking-wide",
			)}
			{...dataProps}
		>
			{node.value}
		</span>
	);
}

function renderButton(node: UiButtonNode, commandBus: CommandBus): React.ReactElement {
	const variant = node.style?.variant ?? "default";
	return (
		<button
			type="button"
			id={node.id}
			className={cn(
				"rounded-md border px-2 py-1 text-sm",
				variant === "danger" && "border-destructive text-destructive",
				variant === "success" && "border-green-600 text-green-700",
				variant === "subtle" && "border-transparent bg-muted/60",
				variant === "default" && "border-border bg-background",
			)}
			onClick={() => commandBus.dispatch(node.command.controllerId, node.command.command, node.command.args)}
		>
			{node.label}
		</button>
	);
}

function renderSeparator(_node: UiSeparatorNode, horizontalParent: boolean): React.ReactElement {
	return (
		<span
			role="separator"
			className={cn(
				"shrink-0 bg-border",
				horizontalParent ? "mx-1 h-4 w-px self-center" : "my-1 h-px w-full",
			)}
			aria-hidden
		/>
	);
}

function renderScene3d(node: UiScene3DHostSurfaceNode): React.ReactElement {
	const Host = scene3dSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported scene3d surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<Host node={node} />
		</div>
	);
}

function renderBoard(node: UiBoardHostSurfaceNode): React.ReactElement {
	const Host = boardSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported board surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<Host node={node} />
		</div>
	);
}

function renderTable(node: UiTableHostSurfaceNode): React.ReactElement {
	const Host = tableSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported table surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="relative min-h-0 min-w-0 flex-1 overflow-auto">
			<Host node={node} />
		</div>
	);
}

function renderPanel(node: UiPanelHostSurfaceNode): React.ReactElement {
	const Host = panelSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported panel surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="relative min-h-0 min-w-0 flex-1 overflow-auto">
			<Host node={node} />
		</div>
	);
}

function renderNode(node: UiNode, commandBus: CommandBus, horizontalParent: boolean): React.ReactElement {
	switch (node.type) {
		case "stack":
			return (
				<div className={cn(stackClass(node), node.direction === "vertical" && node.children.some((c) => c.type === "scene3d" || c.type === "board") && "relative min-h-0 flex-1")}>
					{node.children.map((child, index) => (
						<React.Fragment key={index}>{renderNode(child, commandBus, node.direction === "horizontal")}</React.Fragment>
					))}
				</div>
			);
		case "text":
			return renderText(node);
		case "button":
			return renderButton(node, commandBus);
		case "separator":
			return renderSeparator(node, horizontalParent);
		case "scene3d":
			return renderScene3d(node);
		case "board":
			return renderBoard(node);
		case "table":
			return renderTable(node);
		case "panel":
			return renderPanel(node);
		default:
			return (
				<div className="p-2 text-xs text-destructive">
					Unsupported UiNode {(node as { type?: string }).type ?? "unknown"}
				</div>
			);
	}
}

/** @emoji 🧩 Host entry: turns declarative {@link UiNode} trees into mounted React structure. */
export function UiRenderer({ node, commandBus }: UiRendererProps): React.ReactElement {
	return renderNode(node, commandBus, false);
}
//#endregion 🔖Renderer

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("UiRenderer", () => {
		it("renders text and dispatches button commands", () => {
			const bus = new CommandBus();
			let dispatched = "";
			class TCtrl extends Controller {
				constructor() {
					super("ctrl", bus, () => undefined);
				}
				override run(command: string): void {
					dispatched = command;
				}
			}
			new TCtrl();
			const markup = renderToStaticMarkup(
				<UiRenderer
					commandBus={bus}
					node={{
						type: "stack",
						direction: "vertical",
						children: [
							{ type: "text", value: "hello" },
							{
								type: "button",
								label: "Go",
								command: { controllerId: "ctrl", command: "go" },
							},
						],
					}}
				/>,
			);
			expect(markup).toContain("hello");
			expect(markup).toContain("Go");
			bus.dispatch("ctrl", "go");
			expect(dispatched).toBe("go");
		});
	});
}
//#endregion 🧪Tests

//#endregion 📦ui-declarative-renderer.tsx

//#region 📦shell-bridge.tsx
const elementIconNodes = new Map<string, React.ReactNode>();

/** @emoji 🖼 Registers a static icon node resolved by `iconId` for toolbars, footers, and tabs. */
export function registerElementIcon(iconId: string, node: React.ReactNode): void {
	elementIconNodes.set(iconId, node);
}

/** @emoji 🔍 Returns a registered element icon node for navbar/search rows. */
export function resolveElementIcon(iconId: string): React.ReactNode | undefined {
	return elementIconNodes.get(iconId);
}

const shellTabIcons = new Map<string, LucideIcon>();

/** @emoji 🖼 Registers a Lucide icon constructor for side-panel tab headers keyed by `iconId`. */
export function registerShellTabIcon(iconId: string, Icon: LucideIcon): void {
	shellTabIcons.set(iconId, Icon);
}

const windowBodyByKey = new Map<string, React.ComponentType<unknown>>();

/** @emoji 🪟 Binds a `bodyKey` from {@link WorkbenchWindowKind} to a React window body component. */
export function registerWindowBody(bodyKey: string, Component: React.ComponentType<unknown>): void {
	windowBodyByKey.set(bodyKey, Component);
}

const sidePanelBodyByKey = new Map<string, React.ComponentType<unknown>>();

/** @emoji 📑 Binds a `bodyKey` from {@link ShellSideTabSpec} to a React panel body component. */
export function registerSidePanelBody(bodyKey: string, Component: React.ComponentType<unknown>): void {
	sidePanelBodyByKey.set(bodyKey, Component);
}

const declarativeWindowBodyComponents = new Map<string, React.FC>();

function getDeclarativeWindowBodyComponent(windowKindId: string, bodyKey: string): React.FC {
	const cacheKey = `${bodyKey}\0${windowKindId}`;
	let component = declarativeWindowBodyComponents.get(cacheKey);
	if (!component) {
		component = function ShellDeclarativeWindowBody() {
			const { workbench, activeModeId } = useApp();
			const generation = React.useSyncExternalStore(
				(listener) => workbench.subscribe(listener),
				() => workbench.generation,
				() => 0,
			);
			const ctx: ShellWindowBodyViewContext = {
				workbench,
				windowKindId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getDeclarativeWindowBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative body "${bodyKey}"` };
			return <UiRenderer node={node} commandBus={workbench.commandBus} />;
		};
		declarativeWindowBodyComponents.set(cacheKey, component);
	}
	return component;
}

const declarativeSidePanelBodyComponents = new Map<string, React.FC>();

function getDeclarativeSidePanelBodyComponent(tabId: string, bodyKey: string): React.FC {
	const cacheKey = `${bodyKey}\0${tabId}`;
	let component = declarativeSidePanelBodyComponents.get(cacheKey);
	if (!component) {
		component = function ShellDeclarativeSidePanelBody() {
			const { workbench, activeModeId } = useApp();
			const generation = React.useSyncExternalStore(
				(listener) => workbench.subscribe(listener),
				() => workbench.generation,
				() => 0,
			);
			const ctx: ShellSidePanelBodyViewContext = {
				workbench,
				windowKindId: tabId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getDeclarativeSidePanelBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative panel "${bodyKey}"` };
			return <UiRenderer node={node} commandBus={workbench.commandBus} />;
		};
		declarativeSidePanelBodyComponents.set(cacheKey, component);
	}
	return component;
}

function shellMeasuresToGolden(measures: readonly ShellWindowMeasure[], bus: CommandBus): UIWindowMeasure[] | undefined {
	if (!measures.length) return undefined;
	return measures.map((measure) => {
		if (measure.kind === "select") {
			return {
				id: measure.id,
				kind: "select",
				label: measure.label,
				value: measure.value,
				items: measure.items.map((item) => ({ id: item.id, value: item.value, label: item.label })),
				onValueChange: (value: string) => bus.dispatch(measure.onChange.controllerId, measure.onChange.command, { ...(measure.onChange.args as object | undefined), value }),
			};
		}
		return { id: measure.id, kind: "display", content: null };
	});
}

/** @emoji 🪟 Converts framework window kinds into golden-layout window definitions. */
export function shellWindowKindsToGolden(windowKinds: readonly WorkbenchWindowKind[], bus: CommandBus): UIWindowKindDefinition[] {
	const goldenMeasures = (wk: WorkbenchWindowKind) => shellMeasuresToGolden(wk.measures, bus);
	return windowKinds.map((wk) => {
		const declarativeFactory = getDeclarativeWindowBodyFactory(wk.bodyKey);
		if (declarativeFactory) {
			return { id: wk.id, label: wk.label, component: getDeclarativeWindowBodyComponent(wk.id, wk.bodyKey), measures: goldenMeasures(wk) };
		}
		const Body =
			windowBodyByKey.get(wk.bodyKey) ??
			(() => (
				<div className="p-2 text-xs text-muted-foreground">
					Missing window body &quot;{wk.bodyKey}&quot;
				</div>
			));
		return { id: wk.id, label: wk.label, component: Body as React.ComponentType, measures: goldenMeasures(wk) };
	});
}

function shellTabIconComponent(iconId: string): React.ComponentType<{ size?: number }> {
	return function ShellResolvedTabIcon({ size = 16 }: { size?: number }) {
		const node = elementIconNodes.get(iconId);
		if (node) {
			return (
				<span className="inline-flex items-center justify-center" style={{ width: size, height: size }}>
					{node}
				</span>
			);
		}
		const Lucide = shellTabIcons.get(iconId);
		return Lucide ? <Lucide size={size} /> : <span style={{ display: "inline-block", width: size }} data-missing-icon={iconId} />;
	};
}

/** @emoji 📑 Converts framework side tabs into panel tab configs. */
export function shellSideTabsToPanelTabs(tabs: readonly ShellSideTabSpec[], bus: CommandBus): SidePanelTabConfig[] {
	void bus;
	return tabs.map((tab, orderIndex) => {
		const declarativeFactory = getDeclarativeSidePanelBodyFactory(tab.bodyKey);
		const Body = declarativeFactory
			? getDeclarativeSidePanelBodyComponent(tab.id, tab.bodyKey)
			: (sidePanelBodyByKey.get(tab.bodyKey) ?? (() => <div className="p-2 text-xs">Missing panel {tab.bodyKey}</div>));
		return {
			id: tab.id,
			icon: shellTabIconComponent(tab.iconId),
			order: tab.order ?? orderIndex,
			tree: { sections: [{ id: `${tab.id}.body`, content: <Body /> }] },
		};
	});
}

/** @emoji 👣 Converts framework footer items into React footer rows. */
export function shellFooterToFooterItems(items: readonly ShellFooterItem[], bus: CommandBus): FooterItem[] {
	return items.map((item) => ({
		id: item.id,
		text: item.text,
		order: item.order,
		className: item.className,
		disabled: item.disabled,
		icon: item.iconId ? elementIconNodes.get(item.iconId) : undefined,
		onClick: item.controllerId && item.command ? () => bus.dispatch(item.controllerId!, item.command!, item.args) : undefined,
	}));
}

function shellToolToToolbarItem(item: ShellToolItem, bus: CommandBus): UIToolbarItem {
	if (item.kind === "separator") {
		return { id: item.id, kind: "separator", order: item.order };
	}
	const iconNode = item.iconId ? elementIconNodes.get(item.iconId) : undefined;
	if (item.kind === "toggle") {
		return {
			id: item.id,
			kind: "toggle",
			icon: iconNode,
			label: item.label,
			text: item.text,
			order: item.order,
			pressed: item.pressed,
			onPressedChange: (pressed: boolean) => {
				if (item.controllerId && item.command) bus.dispatch(item.controllerId, item.command, { ...((item.args as object | undefined) ?? {}), pressed });
			},
		};
	}
	return {
		id: item.id,
		icon: iconNode,
		label: item.label,
		text: item.text,
		order: item.order,
		onClick: item.controllerId && item.command ? () => bus.dispatch(item.controllerId!, item.command!, item.args) : undefined,
	};
}

/** @emoji 🧰 Converts framework toolbar maps into React toolbar items. */
export function shellToolsToAppTools(tools: ShellAppTools | undefined, bus: CommandBus): AppTools | undefined {
	if (!tools) return undefined;
	const merged: AppTools = {};
	for (const category of APP_TOOL_CATEGORY_ORDER) {
		const list = tools[category];
		if (!list?.length) continue;
		merged[category] = list.map((entry) => shellToolToToolbarItem(entry, bus));
	}
	return Object.keys(merged).length > 0 ? merged : undefined;
}

/** @emoji 🔀 Merges config rows by `id` (extension overrides base). */
export function mergeConfigEntries<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
	if (!base?.length && !extension?.length) return undefined;
	const merged = new Map<string, T>();
	base?.forEach((entry) => merged.set(entry.id, entry));
	extension?.forEach((entry) => merged.set(entry.id, entry));
	return [...merged.values()];
}

//#endregion 📦shell-bridge.tsx

//#region 📦workbench-view.tsx
const WorkbenchFindItemsSync: React.FC<{
	findItems?: UIFindItem[];
	onFindSelect?: (itemId: string) => void;
}> = ({ findItems, onFindSelect }) => {
	const { setFindItems, setOnFindItem } = useUIFind();
	const resolvedFindItems = findItems ?? [];
	React.useEffect(() => {
		setFindItems(resolvedFindItems);
		setOnFindItem(onFindSelect);
	}, [findItems, onFindSelect, resolvedFindItems, setFindItems, setOnFindItem]);
	return null;
};
const APP_WORKBENCH_TAB_ID = "workbench";
const APP_DETAILS_TAB_ID = "details";
const APP_OPTIONS_TAB_ID = "options";
const APP_CHAT_TAB_ID = "chat";
type AppPanelKind = "workbench" | "details" | "options" | "chat";

function hasAppPanelValue(value: unknown): boolean {
  if (value === null || value === undefined) return false;
  if (typeof value === "string") return value.trim().length > 0;
  if (Array.isArray(value)) return value.length > 0;
  if (typeof value === "object") return Object.keys(value as Record<string, unknown>).length > 0;
  return true;
}

const AppPanelStatePreview: React.FC<{
  emptyMessage: string;
  testId: string;
  value: unknown;
}> = ({ emptyMessage, testId, value }) => {
  if (!hasAppPanelValue(value)) {
    return <div data-testid={`${testId}.empty`} className="text-sm text-muted-foreground">{emptyMessage}</div>;
  }

  return (
    <pre data-testid={testId} className="text-xs leading-relaxed whitespace-pre-wrap break-words rounded-[3px] border bg-window p-small overflow-x-auto">
      {JSON.stringify(value, null, 2)}
    </pre>
  );
};

const AppWorkbenchPanel: React.FC<{
  activeModeLabel?: string | null;
  app: ResolvedWorkbenchAppState;
}> = ({ activeModeLabel, app }) => {
  return (
    <div data-testid="app-panel.workbench" className="flex min-h-0 flex-col gap-small text-sm">
      <div>
        <div className="font-medium">{app.label}</div>
        <div className="text-muted-foreground">{activeModeLabel ? `Mode: ${activeModeLabel}` : "Single-mode app"}</div>
      </div>
      <div className="grid gap-single text-muted-foreground">
        <div>{`Windows: ${app.windowKinds.length}`}</div>
        <div>{`Tools: ${countAppTools(app.tools)}`}</div>
        <div>{`Left tabs: ${app.leftPanelTabs?.length ?? 0}`}</div>
        <div>{`Right tabs: ${app.rightPanelTabs?.length ?? 0}`}</div>
      </div>
    </div>
  );
};

function createDefaultAppWorkbenchTabs(app: ResolvedWorkbenchAppState, activeModeLabel?: string | null): SidePanelTabConfig[] {
  return [
    new StaticSidePanelTabDefinition({
      id: APP_WORKBENCH_TAB_ID,
      icon: Folder,
      order: 0,
      tree: new StaticTreePanelDefinition({
        sections: [{ id: `${APP_WORKBENCH_TAB_ID}.summary`, content: <AppWorkbenchPanel activeModeLabel={activeModeLabel} app={app} /> }],
      }),
    }).resolveTab(),
  ];
}

function createDefaultAppDetailsTabs(app: ResolvedWorkbenchAppState): SidePanelTabConfig[] {
  return [
    new StaticSidePanelTabDefinition({
      id: APP_DETAILS_TAB_ID,
      icon: Info,
      order: 0,
      tree: new StaticTreePanelDefinition({
        sections: [
          {
            id: `${APP_DETAILS_TAB_ID}.state`,
            content: <AppPanelStatePreview emptyMessage="No detail state is available for this app." testId="app-panel.details" value={{ selection: app.selection ?? {}, hover: app.hover ?? {} }} />,
          },
        ],
      }),
    }).resolveTab(),
  ];
}

function createDefaultAppOptionsTabs(app: ResolvedWorkbenchAppState): SidePanelTabConfig[] {
  return [
    new StaticSidePanelTabDefinition({
      id: APP_OPTIONS_TAB_ID,
      icon: Settings2,
      order: 0,
      tree: new StaticTreePanelDefinition({
        sections: [{ id: `${APP_OPTIONS_TAB_ID}.state`, content: <AppPanelStatePreview emptyMessage="No options are available for this app." testId="app-panel.options" value={app.options ?? {}} /> }],
      }),
    }).resolveTab(),
  ];
}

function createDefaultAppChatTabs(app: ResolvedWorkbenchAppState): SidePanelTabConfig[] {
  return [
    new StaticSidePanelTabDefinition({
      id: APP_CHAT_TAB_ID,
      icon: MessageSquare,
      order: 0,
      tree: new StaticTreePanelDefinition({
        sections: [{ id: `${APP_CHAT_TAB_ID}.content`, content: <BasicChatPanel id={`app.chat.${app.id}`} title={app.label} /> }],
      }),
    }).resolveTab(),
  ];
}

function withDefaultAppPanelTabs(app: ResolvedWorkbenchAppState, bus: CommandBus, activeModeLabel?: string | null): Record<AppPanelKind, SidePanelTabConfig[]> {
	const defaultWorkbenchTabs = createDefaultAppWorkbenchTabs(app, activeModeLabel);
	const defaultDetailsTabs = createDefaultAppDetailsTabs(app);
	const defaultOptionsTabs = createDefaultAppOptionsTabs(app);
	const defaultChatTabs = createDefaultAppChatTabs(app);
	const shellLeft = shellSideTabsToPanelTabs(app.leftTabs, bus);
	const shellRight = shellSideTabsToPanelTabs(app.rightTabs, bus);
	return {
		workbench: mergeConfigEntries(defaultWorkbenchTabs, shellLeft.length ? shellLeft : undefined) ?? defaultWorkbenchTabs,
		details: mergeConfigEntries(defaultDetailsTabs, shellRight.length ? shellRight : undefined) ?? defaultDetailsTabs,
		options: defaultOptionsTabs,
		chat: defaultChatTabs,
	};
}

/**
 * Left panel toggle for the navbar.
 * Uses the first tab icon as the toggle icon.
 * Styled to match sketchpad: border border-element, h-medium.
 **/
const UIPanelToggleGroup: React.FC<{
  items: Array<{
    icon: React.ReactNode;
    id: string;
    onPressedChange: (pressed: boolean) => void;
    pressed: boolean;
  }>;
}> = ({ items }) => (
  <div data-slot="app-panel-toggle-group" className="flex items-stretch border border-element overflow-hidden h-medium">
    {items.map((item, index) => (
      <Toggle
        key={item.id}
        kind="icon"
        id={item.id}
        pressed={item.pressed}
        onPressedChange={item.onPressedChange}
        className={cn("border-0 rounded-none", index > 0 && "border-l")}
        icon={item.icon}
      />
    ))}
  </div>
);

/**
 * Domain-neutral composite component providing a full application shell.
 * The UI only has apps. An app has window kinds (rendered with golden-layout)
 * and registers left/right side panel tabs, footer items, toolbar items, and find items.
 * Every UI has: toolbar, search (Ctrl+P), panel toggles, back/forward/up navigation.
 * Every app has: find (Ctrl+F).
 * Every panel has: tree.
 * Fixed navbar layout: [mode (if >1 mode)] [back] [forward] [up] [app nav (if >1 app)] [uri (flex-1)] [search] [find] [panel toggles].
 **/
export const WorkbenchView: React.FC<WorkbenchViewProps> = ({
	workbench,
	defaultAppId,
	uri: uriProp = "/",
	onNavigate,
	canGoBack: canGoBackProp = false,
	onGoBack,
	canGoForward: canGoForwardProp = false,
	onGoForward,
	canGoUp: canGoUpProp = false,
	onGoUp,
	mobile,
	mobileQuery = "(max-width: 767px)",
	className,
	initialPanelVisibility,
	resolvedWindowKindsOverride,
	slotToolbar,
	extraFooterItems,
	augmentPanelTabs,
}) => {
	const shellGen = React.useSyncExternalStore(
		(onStoreChange) => workbench.subscribe(onStoreChange),
		() => workbench.generation,
		() => 0,
	);
	void shellGen;

	React.useEffect(() => {
		if (defaultAppId) {
			workbench.setActiveAppId(defaultAppId);
		}
	}, [defaultAppId, workbench]);

	React.useEffect(() => {
		workbench.uri = uriProp;
		workbench.onNavigate = onNavigate;
		workbench.onGoBack = onGoBack;
		workbench.onGoForward = onGoForward;
		workbench.onGoUp = onGoUp;
		workbench.canGoBack = canGoBackProp;
		workbench.canGoForward = canGoForwardProp;
		workbench.canGoUp = canGoUpProp;
		workbench.mobile = mobile;
		workbench.mobileQuery = mobileQuery;
		workbench.className = className ?? "";
		workbench.notify();
	}, [uriProp, onNavigate, onGoBack, onGoForward, onGoUp, canGoBackProp, canGoForwardProp, canGoUpProp, mobile, mobileQuery, className, workbench]);

	const [leftPanelSize, setLeftPanelSize] = React.useState(280);
	const [rightPanelSize, setRightPanelSize] = React.useState(300);
	const [panelVisibility, setPanelVisibility] = React.useState<UIPanelVisibility>(() => ({
		leftSidePanel: initialPanelVisibility?.leftSidePanel ?? false,
		rightSidePanel: initialPanelVisibility?.rightSidePanel ?? false,
	}));
	const [mobilePanelVisible, setMobilePanelVisible] = React.useState(false);
	const [activeDesktopRightPanelKind, setActiveDesktopRightPanelKind] = React.useState<Exclude<AppPanelKind, "workbench">>("details");
	const [activeMobilePanelKind, setActiveMobilePanelKind] = React.useState<AppPanelKind>("workbench");
	const [mobilePanelActiveTabId, setMobilePanelActiveTabId] = React.useState<string | undefined>(undefined);
	const [searchOpen, setSearchOpen] = React.useState(false);
	const [findOpen, setFindOpen] = React.useState(false);
	const detectedMobile = useMediaQuery(mobileQuery);
	const resolvedMobile = mobile ?? detectedMobile ?? workbench.mobile;

	useCommandHotkey(
		"ctrl+p,meta+p",
		() => {
			const activeEl = document.activeElement as HTMLElement | null;
			if (!searchOpen && activeEl && (activeEl.tagName === "INPUT" || activeEl.tagName === "TEXTAREA" || activeEl.isContentEditable)) {
				return;
			}
			setSearchOpen((previousValue) => !previousValue);
		},
		{ preventDefault: true, enableOnFormTags: true },
		[searchOpen],
	);
	useCommandHotkey(
		"ctrl+f,meta+f",
		() => {
			setFindOpen((previousValue) => !previousValue);
		},
		{ preventDefault: true, enableOnFormTags: true },
		[],
	);

	const togglePanel = React.useCallback((panel: keyof UIPanelVisibility) => {
		setPanelVisibility((prev) => ({ ...prev, [panel]: !prev[panel] }));
	}, []);

	const resolvedApps = workbench.apps;
	const activeAppId = workbench.activeAppId;
	const setActiveAppId = React.useCallback(
		(id: string) => {
			workbench.setActiveAppId(id);
		},
		[workbench],
	);

	const activeAppBase = workbench.getActiveApp();
	if (!activeAppBase) return null;

	const activeModeId = activeAppBase.getActiveModeId();
	const activeApp = activeAppBase.resolve(activeModeId);
	const activeModeLabel = activeAppBase.modes.find((mode) => mode.id === activeModeId)?.label ?? null;
	const panelTabsBase = withDefaultAppPanelTabs(activeApp, workbench.commandBus, activeModeLabel);
	const panelTabs = {
		...panelTabsBase,
		workbench: mergeConfigEntries(panelTabsBase.workbench, augmentPanelTabs?.workbench) ?? panelTabsBase.workbench,
		details: mergeConfigEntries(panelTabsBase.details, augmentPanelTabs?.details) ?? panelTabsBase.details,
	};
	const workbenchTabs = panelTabs.workbench;
	const detailsTabs = panelTabs.details;
	const optionsTabs = panelTabs.options;
	const chatTabs = panelTabs.chat;
	const activeDesktopRightPanelTabs = activeDesktopRightPanelKind === "details" ? detailsTabs : activeDesktopRightPanelKind === "options" ? optionsTabs : chatTabs;
	const activeMobilePanelTabs = activeMobilePanelKind === "workbench" ? workbenchTabs : activeMobilePanelKind === "details" ? detailsTabs : activeMobilePanelKind === "options" ? optionsTabs : chatTabs;

	const hasModeNav = activeAppBase.modes.length > 1;
	const setActiveModeId = (id: string) => {
		activeAppBase.setActiveModeId(id);
		workbench.notify();
	};

	const mergedTools = React.useMemo(
		() => mergeAppTools(shellToolsToAppTools(workbench.globalTools, workbench.commandBus), shellToolsToAppTools(activeApp.tools, workbench.commandBus)),
		[activeApp.tools, workbench, shellGen],
	);
	const hasToolbarTools = listPopulatedAppToolCategories(mergedTools).length > 0;

	const openDesktopWorkbench = React.useCallback((pressed: boolean) => {
		setPanelVisibility((prev) => ({ ...prev, leftSidePanel: pressed }));
	}, []);

	const openDesktopRightPanel = React.useCallback(
		(kind: Exclude<AppPanelKind, "workbench">, pressed: boolean) => {
			if (pressed) {
				setActiveDesktopRightPanelKind(kind);
				setPanelVisibility((prev) => ({ ...prev, rightSidePanel: true }));
				return;
			}
			setPanelVisibility((prev) => ({ ...prev, rightSidePanel: kind === activeDesktopRightPanelKind ? false : prev.rightSidePanel }));
		},
		[activeDesktopRightPanelKind],
	);

	const openMobilePanel = React.useCallback(
		(kind: AppPanelKind, pressed: boolean) => {
			if (pressed) {
				setActiveMobilePanelKind(kind);
				setMobilePanelVisible(true);
				return;
			}
			if (activeMobilePanelKind === kind) {
				setMobilePanelVisible(false);
			}
		},
		[activeMobilePanelKind],
	);

	const workbenchIcon = workbenchTabs[0]?.icon ? React.createElement(workbenchTabs[0].icon, { size: 16 }) : <Folder size={16} />;
	const detailsIcon = detailsTabs[0]?.icon ? React.createElement(detailsTabs[0].icon, { size: 16 }) : <Info size={16} />;
	const optionsIcon = optionsTabs[0]?.icon ? React.createElement(optionsTabs[0].icon, { size: 16 }) : <Settings2 size={16} />;
	const chatIcon = chatTabs[0]?.icon ? React.createElement(chatTabs[0].icon, { size: 16 }) : <MessageSquare size={16} />;

	const navbarItems: NavbarItem[] = [];

	if (hasModeNav) {
		navbarItems.push({
			key: "modeNav",
			content: (
				<Select id={`ui.mode.select.${activeAppBase.id}`} onValueChange={setActiveModeId} value={activeModeId ?? undefined}>
					<SelectTrigger className="h-medium w-30" id={`ui.mode.select.${activeAppBase.id}.trigger`} size="sm">
						<SelectValue placeholder="Mode" />
					</SelectTrigger>
					<SelectContent>
						{activeAppBase.modes.map((mode) => (
							<SelectItem key={mode.id} id={`ui.mode.select.${activeAppBase.id}.${mode.id}`} value={mode.id}>
								<span className="flex items-center gap-single">
									{mode.iconId ? resolveElementIcon(mode.iconId) ?? null : null}
									<span>{mode.label}</span>
								</span>
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			),
		});
	}

	navbarItems.push({
		key: "navBack",
		content: (
			<ButtonGroup id="ui.nav.back">
				<ButtonGroupItem id="ui.nav.back" onClick={onGoBack} className={cn(!canGoBackProp && "opacity-30 pointer-events-none")}>
					<ArrowLeft className="size-small" />
				</ButtonGroupItem>
			</ButtonGroup>
		),
	});
	navbarItems.push({
		key: "navForward",
		content: (
			<ButtonGroup id="ui.nav.forward">
				<ButtonGroupItem id="ui.nav.forward" onClick={onGoForward} className={cn(!canGoForwardProp && "opacity-30 pointer-events-none")}>
					<ArrowRight className="size-small" />
				</ButtonGroupItem>
			</ButtonGroup>
		),
	});
	navbarItems.push({
		key: "navUp",
		content: (
			<ButtonGroup id="ui.nav.up">
				<ButtonGroupItem id="ui.nav.up" onClick={onGoUp} className={cn(!canGoUpProp && "opacity-30 pointer-events-none")}>
					<ArrowUp className="size-small" />
				</ButtonGroupItem>
			</ButtonGroup>
		),
	});

	if (resolvedApps.length > 1) {
		navbarItems.push({
			key: "appNav",
			content: (
				<ButtonGroup id="ui.appNav">
					{resolvedApps.map((app) => (
						<ButtonGroupItem key={app.id} id={`ui.appNav.${app.id}`} className={cn(activeAppId === app.id && "bg-active-base")} onClick={() => setActiveAppId(app.id)}>
							{app.iconId ? resolveElementIcon(app.iconId) ?? <span className="text-xs">{app.label}</span> : <span className="text-xs">{app.label}</span>}
						</ButtonGroupItem>
					))}
				</ButtonGroup>
			),
		});
	}

	navbarItems.push({
		key: "uri",
		className: "flex-1 min-w-0",
		content: <span className="text-sm text-muted-foreground truncate px-single select-all">{uriProp}</span>,
	});

	navbarItems.push({
		key: "search",
		content: <Toggle kind="icon" id="ui.search.toggle" pressed={searchOpen} onPressedChange={setSearchOpen} icon={<Search size={16} />} />,
	});

	navbarItems.push({
		key: "find",
		content: <Toggle kind="icon" id="ui.find.toggle" pressed={findOpen} onPressedChange={setFindOpen} icon={<Search size={16} />} />,
	});

	navbarItems.push({
		key: "panelToggles",
		content: (
			<UIPanelToggleGroup
				items={
					resolvedMobile
						? [
								{ id: "ui.panelToggle.workbench", icon: workbenchIcon, pressed: mobilePanelVisible && activeMobilePanelKind === "workbench", onPressedChange: (pressed) => openMobilePanel("workbench", pressed) },
								{ id: "ui.panelToggle.details", icon: detailsIcon, pressed: mobilePanelVisible && activeMobilePanelKind === "details", onPressedChange: (pressed) => openMobilePanel("details", pressed) },
								{ id: "ui.panelToggle.options", icon: optionsIcon, pressed: mobilePanelVisible && activeMobilePanelKind === "options", onPressedChange: (pressed) => openMobilePanel("options", pressed) },
								{ id: "ui.panelToggle.chat", icon: chatIcon, pressed: mobilePanelVisible && activeMobilePanelKind === "chat", onPressedChange: (pressed) => openMobilePanel("chat", pressed) },
						  ]
						: [
								{ id: "ui.panelToggle.workbench", icon: workbenchIcon, pressed: panelVisibility.leftSidePanel, onPressedChange: openDesktopWorkbench },
								{ id: "ui.panelToggle.details", icon: detailsIcon, pressed: panelVisibility.rightSidePanel && activeDesktopRightPanelKind === "details", onPressedChange: (pressed) => openDesktopRightPanel("details", pressed) },
								{ id: "ui.panelToggle.options", icon: optionsIcon, pressed: panelVisibility.rightSidePanel && activeDesktopRightPanelKind === "options", onPressedChange: (pressed) => openDesktopRightPanel("options", pressed) },
								{ id: "ui.panelToggle.chat", icon: chatIcon, pressed: panelVisibility.rightSidePanel && activeDesktopRightPanelKind === "chat", onPressedChange: (pressed) => openDesktopRightPanel("chat", pressed) },
						  ]
				}
			/>
		),
	});

	const mergedFooterItems = [
		...shellFooterToFooterItems(workbench.globalFooterItems, workbench.commandBus),
		...shellFooterToFooterItems(activeApp.footerItems, workbench.commandBus),
		...(extraFooterItems ?? []),
	].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));

	const searchItemsResolved = React.useMemo(
		() =>
			workbench.searchItems.map((row) => ({
				id: row.id,
				label: row.label,
				description: row.description,
				category: row.category,
				icon: row.iconId ? resolveElementIcon(row.iconId) : undefined,
				onSelect: () => workbench.commandBus.dispatch(row.controllerId, row.command, row.args),
			})),
		[workbench, shellGen],
	);

	const goldenWindowKinds = React.useMemo(
		() => resolvedWindowKindsOverride ?? shellWindowKindsToGolden(activeApp.windowKinds, workbench.commandBus),
		[activeApp.windowKinds, resolvedWindowKindsOverride, workbench.commandBus],
	);

	const toolbarElement = slotToolbar ?? (hasToolbarTools && mergedTools ? <UIToolbar tools={mergedTools} /> : undefined);

	return (
		<AppContext.Provider
			value={{
				workbench,
				activeAppId,
				setActiveAppId,
				activeApp,
				activeModeId,
				setActiveModeId,
				apps: resolvedApps,
				panelVisibility,
				togglePanel,
				uri: uriProp,
				navigate: onNavigate ?? (() => {}),
				canGoBack: canGoBackProp,
				goBack: onGoBack ?? (() => {}),
				canGoForward: canGoForwardProp,
				goForward: onGoForward ?? (() => {}),
				canGoUp: canGoUpProp,
				goUp: onGoUp ?? (() => {}),
			}}
		>
			<UIFindProvider>
				<WorkbenchFindItemsSync findItems={activeApp.findItems} onFindSelect={activeApp.onFindSelect} />
				<Layout
					className={className}
					mobile={resolvedMobile}
					navbar={<Navbar items={navbarItems} />}
					footer={mergedFooterItems.length > 0 ? <Footer items={mergedFooterItems} /> : undefined}
					toolbar={toolbarElement}
					mobilePanel={
						resolvedMobile
							? {
									visible: mobilePanelVisible,
									activeTabId: mobilePanelActiveTabId,
									onActiveTabChange: setMobilePanelActiveTabId,
									tabs: activeMobilePanelTabs,
							  }
							: undefined
					}
					leftSidePanel={
						!resolvedMobile
							? {
									position: "left" as const,
									visible: panelVisibility.leftSidePanel,
									size: leftPanelSize,
									onSizeChange: setLeftPanelSize,
									tabs: workbenchTabs,
							  }
							: undefined
					}
					rightSidePanel={
						!resolvedMobile
							? {
									position: "right" as const,
									visible: panelVisibility.rightSidePanel,
									size: rightPanelSize,
									onSizeChange: setRightPanelSize,
									tabs: activeDesktopRightPanelTabs,
							  }
							: undefined
					}
					canvas={
						<UICanvas
							windowKinds={goldenWindowKinds}
							defaultLayout={
								resolvedMobile
									? createTabStackLayout(
											goldenWindowKinds.map((windowKind) => windowKind.id),
											goldenWindowKinds.map((windowKind) => windowKind.label ?? windowKind.id),
									  )
									: (activeApp.defaultLayout as UIWindowLayout)
							}
							onActiveWindowChange={activeApp.onActiveWindowChange}
						/>
					}
				/>
				{searchItemsResolved.length > 0 && <UISearch items={searchItemsResolved} open={searchOpen} onOpenChange={setSearchOpen} />}
				<UIFind open={findOpen} onOpenChange={setFindOpen} />
			</UIFindProvider>
		</AppContext.Provider>
	);
};

/** @emoji 🧭 @deprecated Alias for {@link WorkbenchView}. */
export const App = WorkbenchView;

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("WorkbenchView", () => {
		it("synthesizes default panel toggles for a single-app workbench", () => {
			const wb = new Workbench();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new WorkbenchApp("test", "Test", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WorkbenchWindowKind("main", "Main", "test.workbench-view.main"),
			]);
			registerWindowBody("test.workbench-view.main", () => <div>Main</div>);
			wb.addApp(app);
			const markup = renderToStaticMarkup(<WorkbenchView workbench={wb} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />);

			expect(markup).toContain('data-panel="leftSidePanel"');
			expect(markup).toContain('id="ui.panelToggle.workbench"');
			expect(markup).toContain('id="ui.panelToggle.details"');
		});

		it("merges appwide tools, selection, options, and window kinds with the active mode", () => {
			const wb = new Workbench();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new WorkbenchApp("app", "App", undefined, new TCtrl(), createTabStackLayout(["base"], ["Base"]), [
				new WorkbenchWindowKind("base", "Base", "test.workbench-view.base"),
			]);
			app.tools = { selection: [{ id: "base-tool", kind: "button", label: "Base", controllerId: "tctrl", command: "x" }] };
			app.selection = { base: true };
			app.options = { snap: true };
			const inspect = new WorkbenchMode("inspect", "Inspect", undefined);
			inspect.tools = { actions: [{ id: "mode-tool", kind: "button", label: "Mode", controllerId: "tctrl", command: "y" }] };
			inspect.selection = { mode: true };
			inspect.options = { isolate: true };
			inspect.windowKinds = [new WorkbenchWindowKind("mode", "Mode", "test.workbench-view.mode")];
			app.addMode(inspect);
			app.defaultModeId = "inspect";
			const resolved = app.resolve("inspect");

			expect(resolved.activeModeId).toBe("inspect");
			expect(resolved.tools?.selection?.map((tool) => tool.id)).toEqual(["base-tool"]);
			expect(resolved.tools?.actions?.map((tool) => tool.id)).toEqual(["mode-tool"]);
			expect(resolved.selection).toEqual({ base: true, mode: true });
			expect(resolved.options).toEqual({ snap: true, isolate: true });
			expect(resolved.windowKinds.map((windowKind) => windowKind.id)).toEqual(["base", "mode"]);
		});

		it("renders a leading mode dropdown when an app has multiple modes", () => {
			const wb = new Workbench();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new WorkbenchApp("app", "App", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WorkbenchWindowKind("main", "Main", "test.workbench-view.mm.main"),
			]);
			registerWindowBody("test.workbench-view.mm.main", () => <div>Main</div>);
			app.addMode(new WorkbenchMode("inspect", "Inspect", undefined));
			app.addMode(new WorkbenchMode("edit", "Edit", undefined));
			wb.addApp(app);
			const markup = renderToStaticMarkup(<WorkbenchView workbench={wb} />);

			expect(markup).toContain('id="ui.mode.select.app.trigger"');
			expect(markup).not.toContain("ui.modeNav.app");
		});
	});
}
//#endregion 🧪Tests

//#endregion 📦workbench-view.tsx

//#region 📦workbench-mount.tsx
type ElementsDomRoot = HTMLElement & { __elementsReactRoot?: Root };

function getElementById<T extends HTMLElement = HTMLElement>(id: string): T | null {
	return document.getElementById(id) as T | null;
}

/** @emoji ⚛️ Imperative React root helpers for workbench shells. */
export class ReactUI {
	private static mountedRoot: Root | null = null;

	/** @emoji 🖥️ Mounts a {@link Workbench} shell into `#root` (or `rootId`) with {@link WorkbenchView}. */
	static mount(workbench: Workbench, rootId = "root"): void {
		if (typeof document === "undefined") return;
		const rootElement = getElementById<ElementsDomRoot>(rootId);
		if (!rootElement) {
			throw new Error(`React root #${rootId} missing.`);
		}
		rootElement.__elementsReactRoot ??= createRoot(rootElement);
		ReactUI.mountedRoot = rootElement.__elementsReactRoot;
		rootElement.__elementsReactRoot.render(<WorkbenchView workbench={workbench} />);
	}

	static unmount(rootId = "root"): void {
		const rootElement = getElementById<ElementsDomRoot>(rootId);
		rootElement?.__elementsReactRoot?.unmount();
		if (rootElement) {
			delete rootElement.__elementsReactRoot;
		}
		ReactUI.mountedRoot = null;
	}
}

/** @emoji 🖥️ Mounts an arbitrary React tree into `#root` (or `rootId`). */
export function mountReactApp(element: React.ReactElement, rootId = "root"): void {
	if (typeof document === "undefined") return;
	const rootElement = getElementById<ElementsDomRoot>(rootId);
	if (!rootElement) {
		throw new Error(`React root #${rootId} missing.`);
	}
	rootElement.__elementsReactRoot ??= createRoot(rootElement);
	rootElement.__elementsReactRoot.render(element);
}

/** @emoji 🖥️ Loads a {@link Workbench} asynchronously then mounts {@link WorkbenchView}. */
export async function mountAsyncReactApp(loadWorkbench: () => Promise<Workbench>, rootId = "root"): Promise<void> {
	ReactUI.mount(await loadWorkbench(), rootId);
}

//#endregion 📦workbench-mount.tsx
