// #region 🧲Header
/** @emoji 🛝 `@elements/playground/react` — Playground shell renderer: {@link PlaygroundView}, declarative tree panels, and surface hosts (depends only on `@elements/ui`). */
// #endregion 🧲Header

import * as React from "react";
import { createPortal } from "react-dom";
import { createRoot, type Root } from "react-dom/client";
import type { LucideIcon } from "lucide-react";
import { Folder, Info } from "lucide-react";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import {
	APP_TOOL_CATEGORY_ORDER,
	CommandBus,
	Controller,
	ProductRuntime,
	WindowKindRuntime,
	createWindowLayout,
	getSidePanelBodyFactory,
	getWindowBodyFactory,
	resolveAppState,
	type AppToolCategory,
	type AppTools,
	type CommandBus as PlaygroundCommandBus,
	type FooterItem as PlaygroundFooterItem,
	type ProductRuntime as PlaygroundProductRuntime,
	type ResolvedAppState,
	type SidePanelBodyViewContext,
	type SideTabSpec,
	type ToolItem,
	type UiBoardHostSurfaceNode,
	type UiNode,
	type UiScene3DHostSurfaceNode,
	type UiTableHostSurfaceNode,
	type WindowBodyViewContext,
	type WindowLayout,
	type WindowMeasure,
} from "../core.ts";
import {
	Button,
	ButtonGroup,
	ButtonGroupItem,
	ContextMenu,
	Footer,
	Layout,
	LevelProvider,
	Navbar,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Slider,
	Toggle,
	ToolbarDivider,
	ToolbarItem,
	ToolbarZone,
	Window,
	cn,
	staticTreePanelDefinition,
	useMediaQuery,
	type FooterItem,
	type NavbarItem,
	type SidePanelTabConfig,
	type SidePanelTabDefinition,
	type TreeDataItem,
	type TreeDataSection,
	type TreePanelConfig,
	type TreePanelDefinition,
	type TreePanelSource,
} from "@elements/ui";

export type {
	AppRuntime,
	AppTools,
	CommandBus,
	Controller,
	FooterItem as PlaygroundDeclarativeFooterItem,
	ModeRuntime,
	ProductRuntime,
	ResolvedAppState,
	SidePanelBodyViewContext,
	SideTabSpec,
	ToolItem,
	UiNode,
	WindowBodyViewContext,
	WindowKindRuntime,
	WindowLayout,
} from "../core.ts";

export {
	APP_TOOL_CATEGORY_ORDER,
	AppRuntime,
	buildScene3dWindowBody,
	CommandBus,
	createDefaultLayout,
	createStackLayout,
	createWindowLayout,
	getSidePanelBodyFactory,
	getWindowBodyFactory,
	ModeRuntime,
	PlaygroundController,
	ProductRuntime,
	registerSidePanelBody,
	registerWindowBody,
	resolveAppState,
	WindowKindRuntime,
} from "../index.ts";

function cnPlay(...inputs: ClassValue[]): string {
	return twMerge(clsx(inputs));
}

//#region 🔖TreePanels
/** @emoji 🌲 Enforces playground panels: each section needs `items` and/or `content` (no JSON-only fallbacks). */
export function enforcePlaygroundTreePanel(config: TreePanelConfig): void {
	if (!config.sections?.length) {
		throw new Error("Playground tree panel must declare at least one section.");
	}
	for (const section of config.sections) {
		const hasItems = Boolean(section.items?.length);
		const hasContent = section.content != null;
		if (!hasItems && !hasContent) {
			throw new Error(`Playground tree section "${section.id}" must declare items or content.`);
		}
	}
}

/** @emoji 📑 Abstract side-panel tab resolved to a {@link SidePanelTabConfig} tree. */
export abstract class PureSidePanelTabDefinition implements SidePanelTabDefinition {
	abstract resolveTab(): SidePanelTabConfig;
}

/** @emoji 🌲 Static tree panel: sections + items only. */
export class StaticTreePanelDefinition implements TreePanelDefinition {
	constructor(private readonly config: TreePanelConfig) {
		enforcePlaygroundTreePanel(config);
	}

	resolveTree(): TreePanelConfig {
		return this.config;
	}
}

/** @emoji 🌲 Factory for a static {@link StaticTreePanelDefinition}. */
export function playgroundStaticTreePanel(config: TreePanelConfig): StaticTreePanelDefinition {
	return new StaticTreePanelDefinition(config);
}

function resolveTreePanelSource(tree: TreePanelSource): TreePanelConfig {
	if (typeof (tree as TreePanelDefinition).resolveTree === "function") {
		const config = (tree as TreePanelDefinition).resolveTree();
		enforcePlaygroundTreePanel(config);
		return config;
	}
	enforcePlaygroundTreePanel(tree as TreePanelConfig);
	return tree as TreePanelConfig;
}

function resolveSidePanelTabSource(tab: SidePanelTabConfig | SidePanelTabDefinition): SidePanelTabConfig {
	if (typeof (tab as SidePanelTabDefinition).resolveTab === "function") {
		const resolved = (tab as SidePanelTabDefinition).resolveTab();
		resolveTreePanelSource(resolved.tree);
		return resolved;
	}
	const config = tab as SidePanelTabConfig;
	resolveTreePanelSource(config.tree);
	return config;
}
//#endregion 🔖TreePanels

//#region 🔖LayoutGolden
function convertWindowLayoutNodeToGoldenConfig(node: WindowLayout["root"]): Record<string, unknown> {
	if (node.kind === "stack") {
		return {
			type: "stack",
			...(node.size !== undefined ? { size: `${node.size}%` } : {}),
			content: node.children.map((child) => ({
				type: "component",
				componentName: child.windowKindId,
				title: child.title ?? child.windowKindId,
				componentState: {},
			})),
		};
	}
	return {
		type: node.kind,
		...(node.size !== undefined ? { size: `${node.size}%` } : {}),
		content: node.children.map((child) => convertWindowLayoutNodeToGoldenConfig(child)),
	};
}

function convertWindowLayoutToGoldenConfig(layout: WindowLayout): Record<string, unknown> {
	return { root: convertWindowLayoutNodeToGoldenConfig(layout.root) };
}

function findDefaultActiveWindowKindId(layout: WindowLayout | undefined, windowKinds: readonly { readonly id: string }[]): string | null {
	const allowed = new Set(windowKinds.map((windowKind) => windowKind.id));
	const visit = (node: WindowLayout["root"]): string | null => {
		if (node.kind === "stack") {
			for (const child of node.children) {
				if (allowed.has(child.windowKindId)) return child.windowKindId;
			}
			return null;
		}
		for (const child of node.children) {
			const match = visit(child);
			if (match) return match;
		}
		return null;
	};
	if (layout) {
		const match = visit(layout.root);
		if (match) return match;
	}
	return windowKinds[0]?.id ?? null;
}
//#endregion 🔖LayoutGolden

//#region 🔖UiRenderer
type Scene3DSurfaceHost = React.ComponentType<{ readonly node: UiScene3DHostSurfaceNode }>;
type BoardSurfaceHost = React.ComponentType<{ readonly node: UiBoardHostSurfaceNode }>;
type TableSurfaceHost = React.ComponentType<{ readonly node: UiTableHostSurfaceNode }>;

const scene3dSurfaceHosts = new Map<string, Scene3DSurfaceHost>();
const boardSurfaceHosts = new Map<string, BoardSurfaceHost>();
const tableSurfaceHosts = new Map<string, TableSurfaceHost>();

/** @emoji 🧭 Binds a `surfaceId` from {@link UiScene3DHostSurfaceNode} to a host React canvas implementation. */
export function registerUiScene3DSurfaceHost(surfaceId: string, Component: Scene3DSurfaceHost): void {
	scene3dSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 📋 Binds `surfaceId` from {@link UiBoardHostSurfaceNode} to a host board canvas. */
export function registerUiBoardSurfaceHost(surfaceId: string, Component: BoardSurfaceHost): void {
	boardSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 📊 Binds `surfaceId` from {@link UiTableHostSurfaceNode} to a host table body. */
export function registerUiTableSurfaceHost(surfaceId: string, Component: TableSurfaceHost): void {
	tableSurfaceHosts.set(surfaceId, Component);
}

function stackClass(spec: { direction: "horizontal" | "vertical"; gap?: string; padding?: string }): string {
	const dir = spec.direction === "horizontal" ? "flex-row" : "flex-col";
	const gap = spec.gap === "none" ? "gap-0" : spec.gap === "tight" ? "gap-1" : spec.gap === "relaxed" ? "gap-4" : "gap-2";
	const pad = spec.padding === "none" ? "p-0" : "p-2";
	return cnPlay("flex", dir, gap, pad, spec.direction === "vertical" ? "min-h-0 min-w-0" : "min-w-0");
}

export function UiRenderer({ node, commandBus }: { readonly node: UiNode; readonly commandBus: CommandBus }): React.ReactElement {
	switch (node.type) {
		case "stack":
			return (
				<div className={cnPlay(stackClass(node), node.direction === "vertical" && node.children.some((c) => c.type === "scene3d" || c.type === "board") && "relative min-h-0 flex-1")}>
					{node.children.map((child, index) => (
						<UiRenderer key={index} node={child} commandBus={commandBus} />
					))}
				</div>
			);
		case "text":
			return <span className="text-muted-foreground px-1 text-xs">{node.value}</span>;
		case "button":
			return (
				<button
					type="button"
					className="rounded-md border border-border bg-background px-2 py-1 text-sm"
					onClick={() => commandBus.dispatch(node.command.controllerId, node.command.command, node.command.args)}
				>
					{node.label}
				</button>
			);
		case "separator":
			return <span role="separator" className="bg-border my-1 h-px w-full shrink-0" aria-hidden />;
		case "scene3d": {
			const Host = scene3dSurfaceHosts.get(node.surfaceId);
			if (!Host) {
				return <div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">Unsupported scene3d surface &quot;{node.surfaceId}&quot;</div>;
			}
			return (
				<div className="absolute inset-0 min-h-0 min-w-0">
					<Host node={node} />
				</div>
			);
		}
		case "board": {
			const Host = boardSurfaceHosts.get(node.surfaceId);
			if (!Host) {
				return <div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">Unsupported board surface &quot;{node.surfaceId}&quot;</div>;
			}
			return (
				<div className="absolute inset-0 min-h-0 min-w-0">
					<Host node={node} />
				</div>
			);
		}
		case "table": {
			const Host = tableSurfaceHosts.get(node.surfaceId);
			if (!Host) {
				return <div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">Unsupported table surface &quot;{node.surfaceId}&quot;</div>;
			}
			return (
				<div className="relative min-h-0 min-w-0 flex-1 overflow-auto">
					<Host node={node} />
				</div>
			);
		}
		default:
			return <div className="p-2 text-xs text-destructive">Unsupported UiNode</div>;
	}
}
//#endregion 🔖UiRenderer

//#region 🔖DeclarativeHosts
const shellTabIcons = new Map<string, LucideIcon>();

/** @emoji 🖼 Registers a Lucide icon constructor for side-panel tab headers keyed by `iconId`. */
export function registerTabIcon(iconId: string, Icon: LucideIcon): void {
	shellTabIcons.set(iconId, Icon);
}

function shellTabIconComponent(iconId: string): React.ComponentType<{ size?: number }> {
	return function ShellResolvedTabIcon({ size = 16 }: { size?: number }) {
		const Lucide = shellTabIcons.get(iconId);
		return Lucide ? <Lucide size={size} /> : <span style={{ display: "inline-block", width: size }} data-missing-icon={iconId} />;
	};
}

const declarativeWindowBodyComponents = new Map<string, React.FC>();

function getDeclarativeWindowBodyComponent(windowKindId: string, bodyKey: string): React.FC {
	const cacheKey = `${bodyKey}\0${windowKindId}`;
	let component = declarativeWindowBodyComponents.get(cacheKey);
	if (!component) {
		component = function ShellDeclarativeWindowBody() {
			const { runtime, activeModeId } = useApp();
			const generation = React.useSyncExternalStore(
				(listener) => runtime.subscribe(listener),
				() => runtime.generation,
				() => 0,
			);
			const ctx: WindowBodyViewContext = {
				runtime,
				windowKindId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getWindowBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative body "${bodyKey}"` };
			return <UiRenderer node={node} commandBus={runtime.commandBus} />;
		};
		declarativeWindowBodyComponents.set(cacheKey, component);
	}
	return component;
}

export interface UIWindowKindDefinition {
	id: string;
	label?: string;
	component: React.ComponentType;
	measures?: React.ReactNode;
}

function windowMeasureShell(measureId: string, label: string | undefined, children: React.ReactNode): React.ReactNode {
	return (
		<div data-slot="window-measure-float" data-measure-id={measureId} className="border-element/80 bg-window/90 max-w-[11rem] min-w-0 rounded-md border px-single py-half shadow-md backdrop-blur-sm">
			{label ? <span className="text-muted-foreground mb-half block max-w-full truncate text-[10px] font-semibold uppercase tracking-wide">{label}</span> : null}
			<div className="min-w-0 w-full">{children}</div>
		</div>
	);
}

function windowMeasuresToGolden(measures: readonly WindowMeasure[], bus: CommandBus): React.ReactNode {
	if (!measures.length) return undefined;
	return (
		<div data-slot="window-measures-stack-inner" className="pointer-events-auto flex flex-col items-end gap-half p-single">
			{measures.map((measure) => {
				if (measure.kind === "select") {
					return (
						<React.Fragment key={measure.id}>
							{windowMeasureShell(
								measure.id,
								measure.label,
								<Select id={measure.id} value={measure.value} onValueChange={(value) => bus.dispatch(measure.onChange.controllerId, measure.onChange.command, { value })}>
									<SelectTrigger id={measure.id} className="h-medium w-full min-w-0 max-w-[9.5rem]" size="sm">
										<SelectValue placeholder={measure.label} />
									</SelectTrigger>
									<SelectContent>
										{measure.items.map((item) => (
											<SelectItem key={item.id} value={item.value}>
												{item.label}
											</SelectItem>
										))}
									</SelectContent>
								</Select>,
							)}
						</React.Fragment>
					);
				}
				if (measure.kind === "slider") {
					return (
						<React.Fragment key={measure.id}>
							{windowMeasureShell(
								measure.id,
								measure.label,
								<Slider
									id={measure.id}
									value={[measure.value]}
									min={measure.min}
									max={measure.max}
									step={measure.step}
									onValueChange={(vals) => bus.dispatch(measure.onChange.controllerId, measure.onChange.command, { value: vals[0] ?? measure.min })}
								/>,
							)}
						</React.Fragment>
					);
				}
				if (measure.kind === "toggle") {
					return (
						<React.Fragment key={measure.id}>
							{windowMeasureShell(
								measure.id,
								measure.label,
								<Toggle
									id={measure.id}
									pressed={measure.pressed}
									text={measure.text}
									onPressedChange={(pressed) => bus.dispatch(measure.onChange.controllerId, measure.onChange.command, { pressed })}
								/>,
							)}
						</React.Fragment>
					);
				}
				return null;
			})}
		</div>
	);
}

export function windowKindsToGolden(windowKinds: readonly WindowKindRuntime[], bus: CommandBus): UIWindowKindDefinition[] {
	return windowKinds.map((wk) => ({
		id: wk.id,
		label: wk.label,
		component: getDeclarativeWindowBodyComponent(wk.id, wk.bodyKey),
		measures: windowMeasuresToGolden(wk.measures, bus),
	}));
}

/** @emoji 📑 Converts playground side tabs into enforced tree panel configs (sections with items). */
export function sideTabsToPlaygroundPanelTabs(tabs: readonly SideTabSpec[], bus: CommandBus): SidePanelTabConfig[] {
	void bus;
	return tabs.map((tab, orderIndex) => {
		const declarativeFactory = getSidePanelBodyFactory(tab.bodyKey);
		const Body = declarativeFactory
			? getDeclarativeSidePanelBodyComponent(tab.id, tab.bodyKey)
			: () => <div className="p-2 text-xs">Missing panel {tab.bodyKey}</div>;
		return resolveSidePanelTabSource({
			id: tab.id,
			icon: shellTabIconComponent(tab.iconId),
			order: tab.order ?? orderIndex,
			tree: staticTreePanelDefinition({
				sections: [
					{
						id: `${tab.id}.host`,
						label: tab.id,
						defaultOpen: true,
						items: [{ id: `${tab.id}.body`, label: tab.id, description: <Body /> }],
					},
				],
			}).resolveTab().tree,
		});
	});
}

const declarativeSidePanelBodyComponents = new Map<string, React.FC>();

function getDeclarativeSidePanelBodyComponent(tabId: string, bodyKey: string): React.FC {
	const cacheKey = `${bodyKey}\0${tabId}`;
	let component = declarativeSidePanelBodyComponents.get(cacheKey);
	if (!component) {
		component = function ShellDeclarativeSidePanelBody() {
			const { runtime, activeModeId } = useApp();
			const generation = React.useSyncExternalStore(
				(listener) => runtime.subscribe(listener),
				() => runtime.generation,
				() => 0,
			);
			const ctx: SidePanelBodyViewContext = {
				runtime,
				windowKindId: tabId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getSidePanelBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative panel "${bodyKey}"` };
			return <UiRenderer node={node} commandBus={runtime.commandBus} />;
		};
		declarativeSidePanelBodyComponents.set(cacheKey, component);
	}
	return component;
}
//#endregion 🔖DeclarativeHosts

//#region 🔖UICanvas
interface UICanvasPortal {
	key: string;
	element: HTMLElement;
	windowKind: UIWindowKindDefinition;
}

const UICanvas: React.FC<{
	windowKinds: UIWindowKindDefinition[];
	defaultLayout: WindowLayout;
	onActiveWindowChange?: (windowId: string) => void;
}> = ({ windowKinds, defaultLayout, onActiveWindowChange }) => {
	const containerRef = React.useRef<HTMLDivElement>(null);
	const layoutRef = React.useRef<{ destroy: () => void; updateSize: () => void } | null>(null);
	const [portals, setPortals] = React.useState<UICanvasPortal[]>([]);
	const onActiveWindowChangeRef = React.useRef(onActiveWindowChange);
	onActiveWindowChangeRef.current = onActiveWindowChange;
	const windowKindRegistryKey = React.useMemo(() => windowKinds.map((wk) => wk.id).join("\0"), [windowKinds]);

	React.useEffect(() => {
		if (!layoutRef.current) return;
		setPortals((prev) =>
			prev.map((portal) => {
				const next = windowKinds.find((wk) => wk.id === portal.windowKind.id);
				return next ? { ...portal, windowKind: next } : portal;
			}),
		);
	}, [windowKinds]);

	React.useEffect(() => {
		if (!containerRef.current || layoutRef.current) return;
		let disposed = false;
		const loadGoldenLayout = async () => {
			try {
				const goldenLayoutModule = await import("golden-layout");
				if (disposed) return;
				const GoldenLayout = (goldenLayoutModule as { GoldenLayout?: new (config: unknown, el: HTMLElement) => { init: () => void; destroy: () => void; updateSize: () => void; registerComponent: (name: string, fn: (c: GoldenContainer) => void) => void; on: (ev: string, fn: (...args: unknown[]) => void) => void } }).GoldenLayout;
				if (!GoldenLayout) return;
				const config = convertWindowLayoutToGoldenConfig(defaultLayout);
				const layout = new GoldenLayout(config, containerRef.current!);
				let portalCounter = 0;
				type GoldenContainer = {
					getElement: () => HTMLElement | HTMLElement[];
					on: (ev: string, fn: () => void) => void;
				};
				windowKinds.forEach((windowKind) => {
					layout.registerComponent(windowKind.id, (container: GoldenContainer) => {
						if (disposed) return;
						const element = container.getElement();
						const domElement = Array.isArray(element) ? element[0] : element;
						if (!(domElement instanceof HTMLElement)) return;
						const portalKey = `${windowKind.id}-${portalCounter++}`;
						const portal: UICanvasPortal = { key: portalKey, element: domElement, windowKind };
						setPortals((prev) => [...prev, portal]);
						container.on("destroy", () => {
							setPortals((prev) => prev.filter((p) => p.key !== portalKey));
						});
					});
				});
				layout.on("tab", (tab: { _header?: { on: (ev: string, fn: () => void) => void }; _contentItem?: { config?: { componentName?: string } } }) => {
					tab._header?.on("click", () => {
						const componentName = tab._contentItem?.config?.componentName;
						if (componentName && onActiveWindowChangeRef.current) onActiveWindowChangeRef.current(componentName);
					});
				});
				layout.init();
				layoutRef.current = layout;
				const handleResize = () => layout.updateSize();
				window.addEventListener("resize", handleResize);
				return () => {
					window.removeEventListener("resize", handleResize);
				};
			} catch (error) {
				console.error("[PlaygroundUICanvas] Failed to load GoldenLayout:", error);
			}
		};
		const cleanupPromise = loadGoldenLayout();
		return () => {
			disposed = true;
			void cleanupPromise;
			setPortals([]);
			try {
				layoutRef.current?.destroy();
			} catch {}
			layoutRef.current = null;
		};
	}, [windowKindRegistryKey, defaultLayout, windowKinds]);

	return (
		<>
			<div ref={containerRef} className="h-full w-full" />
			{portals.map((portal) => {
				const WindowComponent = portal.windowKind.component;
				const clickGoldenLayoutControl = (selector: string) => {
					const stackElement = portal.element.closest(".lm_item.lm_stack") as HTMLElement | null;
					stackElement?.querySelector<HTMLElement>(selector)?.click();
				};
				return createPortal(
					<Window
						key={portal.key}
						id={portal.windowKind.id}
						isVisible
						showControls
						onOpenInNewWindow={() => clickGoldenLayoutControl(".lm_popout")}
						onMaximize={() => clickGoldenLayoutControl(".lm_maximise")}
						onMinimize={() => clickGoldenLayoutControl(".lm_maximise")}
						onClose={() => clickGoldenLayoutControl(".lm_close")}
						measures={portal.windowKind.measures}
					>
						<div className="flex min-h-0 min-w-0 flex-1 flex-col">
							<WindowComponent />
						</div>
					</Window>,
					portal.element,
				);
			})}
		</>
	);
};
//#endregion 🔖UICanvas

//#region 🔖Toolbar
type UIToolbarItem = {
	id: string;
	kind?: "separator" | "toggle";
	icon?: React.ReactNode;
	text?: string;
	label?: string;
	title?: string;
	order?: number;
	pressed?: boolean;
	onPressedChange?: (pressed: boolean) => void;
	onClick?: () => void;
};

function declareToolsToViewTools(tools: AppTools | undefined, bus: CommandBus): Partial<Record<AppToolCategory, UIToolbarItem[]>> | undefined {
	if (!tools) return undefined;
	const merged: Partial<Record<AppToolCategory, UIToolbarItem[]>> = {};
	for (const category of APP_TOOL_CATEGORY_ORDER) {
		const list = tools[category];
		if (!list?.length) continue;
		merged[category] = list.map((item) => {
			if (item.kind === "separator") return { id: item.id, kind: "separator", order: item.order };
			if (item.kind === "toggle") {
				return {
					id: item.id,
					kind: "toggle",
					text: item.text,
					label: item.label,
					title: item.title,
					order: item.order,
					pressed: item.pressed,
					onPressedChange: (pressed: boolean) => {
						if (item.controllerId && item.command) bus.dispatch(item.controllerId, item.command, { ...(item.args as object | undefined), pressed });
					},
				};
			}
			return {
				id: item.id,
				text: item.text,
				label: item.label,
				title: item.title,
				order: item.order,
				onClick: item.controllerId && item.command ? () => bus.dispatch(item.controllerId!, item.command!, item.args) : undefined,
			};
		});
	}
	return Object.keys(merged).length > 0 ? merged : undefined;
}

const PlaygroundToolbar: React.FC<{ tools: Partial<Record<AppToolCategory, UIToolbarItem[]>> }> = ({ tools }) => (
	<div className="flex min-w-0 flex-1 items-center gap-single overflow-x-auto px-single">
		{APP_TOOL_CATEGORY_ORDER.map((category) => {
			const items = tools[category];
			if (!items?.length) return null;
			const sorted = [...items].sort((left, right) => (left.order ?? 0) - (right.order ?? 0));
			return (
				<ToolbarZone key={category} id={`playground.toolbar.${category}`}>
					{sorted.map((item) => {
						const tooltip = item.title ?? item.label ?? item.text;
						if (item.kind === "separator") {
							return <ToolbarDivider key={item.id} id={item.id} />;
						}
						if (item.kind === "toggle") {
							return (
								<ToolbarItem key={item.id}>
									<Toggle
										id={item.id}
										title={tooltip}
										text={item.text ?? item.label}
										pressed={item.pressed ?? false}
										onPressedChange={(pressed) => item.onPressedChange?.(pressed)}
									/>
								</ToolbarItem>
							);
						}
						return (
							<ToolbarItem key={item.id}>
								<button
									type="button"
									id={item.id}
									title={tooltip}
									onClick={item.onClick}
									className="flex cursor-pointer items-center gap-single rounded px-single py-tiny text-sm hover:bg-hover-panel"
								>
									{item.icon}
									{(item.text ?? item.label) ? <span>{item.text ?? item.label}</span> : null}
								</button>
							</ToolbarItem>
						);
					})}
				</ToolbarZone>
			);
		})}
	</div>
);
//#endregion 🔖Toolbar

//#region 🔖PlaygroundView
export interface PlaygroundPanelVisibility {
	leftSidePanel: boolean;
	rightSidePanel: boolean;
}

export interface PlaygroundContextValue {
	runtime: ProductRuntime;
	activeAppId: string;
	activeApp: ResolvedAppState;
	activeModeId: string | null;
}

export const PlaygroundContext = React.createContext<PlaygroundContextValue | undefined>(undefined);

/** @emoji 🪝 Returns the active {@link ProductRuntime} from the nearest {@link PlaygroundView}. */
export function useApp(): PlaygroundContextValue {
	const ctx = React.useContext(PlaygroundContext);
	if (!ctx) throw new Error("useApp must be used within PlaygroundView");
	return ctx;
}

export interface PlaygroundViewProps {
	readonly runtime: ProductRuntime;
	readonly defaultAppId?: string;
	readonly className?: string;
	readonly mobile?: boolean;
	readonly mobileQuery?: string;
	readonly initialPanelVisibility?: PlaygroundPanelVisibility;
	readonly slotToolbar?: React.ReactNode;
	readonly extraFooterItems?: readonly FooterItem[];
	readonly augmentPanelTabs?: Partial<Record<"workbench" | "details", readonly (SidePanelTabConfig | SidePanelTabDefinition)[]>>;
	readonly onActiveWindowChange?: (windowKindId: string) => void;
}

function mergePanelTabs(base: SidePanelTabConfig[] | undefined, extension: readonly (SidePanelTabConfig | SidePanelTabDefinition)[] | undefined): SidePanelTabConfig[] {
	if (!extension?.length) return base ?? [];
	const merged = new Map<string, SidePanelTabConfig>();
	base?.forEach((tab) => merged.set(tab.id, resolveSidePanelTabSource(tab)));
	extension.forEach((tab) => merged.set(resolveSidePanelTabSource(tab).id, resolveSidePanelTabSource(tab)));
	return [...merged.values()];
}

/** @emoji 🛝 Playground application shell: tree-only side panels, no JSON fallback details tab. */
export const PlaygroundView: React.FC<PlaygroundViewProps> = ({
	runtime,
	defaultAppId,
	className,
	mobile,
	mobileQuery = "(max-width: 767px)",
	initialPanelVisibility,
	slotToolbar,
	extraFooterItems,
	augmentPanelTabs,
	onActiveWindowChange,
}) => {
	React.useSyncExternalStore(
		(onStoreChange) => runtime.subscribe(onStoreChange),
		() => runtime.generation,
		() => 0,
	);

	React.useEffect(() => {
		if (defaultAppId) runtime.setActiveAppId(defaultAppId);
	}, [defaultAppId, runtime]);

	const [leftPanelSize, setLeftPanelSize] = React.useState(280);
	const [rightPanelSize, setRightPanelSize] = React.useState(300);
	const [panelVisibility, setPanelVisibility] = React.useState<PlaygroundPanelVisibility>(() => ({
		leftSidePanel: initialPanelVisibility?.leftSidePanel ?? false,
		rightSidePanel: initialPanelVisibility?.rightSidePanel ?? false,
	}));
	const detectedMobile = useMediaQuery(mobileQuery);
	const resolvedMobile = mobile ?? detectedMobile ?? runtime.mobile;

	const activeAppBase = runtime.getActiveApp();
	if (!activeAppBase) return null;

	const activeModeId = activeAppBase.getActiveModeId();
	const activeApp = activeAppBase.resolve(activeModeId);
	const bus = runtime.commandBus;

	const workbenchTabs = mergePanelTabs(sideTabsToPlaygroundPanelTabs(activeApp.leftTabs, bus), augmentPanelTabs?.workbench);
	const detailsTabs = mergePanelTabs(undefined, augmentPanelTabs?.details);

	const mergedTools = declareToolsToViewTools(activeApp.tools, bus);
	const hasToolbarTools = mergedTools && APP_TOOL_CATEGORY_ORDER.some((c) => mergedTools[c]?.some((i) => i.kind !== "separator"));

	const [activeWindowKindId, setActiveWindowKindId] = React.useState<string | null>(() => findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds));

	React.useEffect(() => {
		setActiveWindowKindId((previous) => {
			if (previous && activeApp.windowKinds.some((wk) => wk.id === previous)) return previous;
			return findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds);
		});
	}, [activeApp.defaultLayout, activeApp.windowKinds]);

	const goldenWindowKinds = React.useMemo(() => windowKindsToGolden(activeApp.windowKinds, bus), [activeApp.windowKinds, bus, runtime.generation]);

	const footerItems: FooterItem[] = [
		...(activeApp.footerItems.map((item) => ({
			id: item.id,
			text: item.text,
			order: item.order,
			className: item.className,
			disabled: item.disabled,
			onClick: item.controllerId && item.command ? () => bus.dispatch(item.controllerId!, item.command!, item.args) : undefined,
		})) as FooterItem[]),
		...(extraFooterItems ?? []),
	].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));

	const workbenchIcon = workbenchTabs[0]?.icon ? React.createElement(workbenchTabs[0].icon, { size: 16 }) : <Folder size={16} />;
	const detailsIcon = detailsTabs[0]?.icon ? React.createElement(detailsTabs[0].icon, { size: 16 }) : <Info size={16} />;

	const navbarItems: NavbarItem[] = [
		{
			key: "title",
			className: "flex-1 min-w-0",
			content: <span className="truncate px-single text-sm font-medium">{activeApp.label}</span>,
		},
		{
			key: "panelToggles",
			content: (
				<div className="flex items-stretch overflow-hidden border border-element h-medium">
					<Toggle kind="icon" id="playground.panel.workbench" pressed={panelVisibility.leftSidePanel} onPressedChange={(pressed) => setPanelVisibility((p) => ({ ...p, leftSidePanel: pressed }))} icon={workbenchIcon} className="rounded-none border-0" />
					<Toggle kind="icon" id="playground.panel.details" pressed={panelVisibility.rightSidePanel} onPressedChange={(pressed) => setPanelVisibility((p) => ({ ...p, rightSidePanel: pressed }))} icon={detailsIcon} className="rounded-none border-0 border-l" />
				</div>
			),
		},
	];

	const toolbarElement = slotToolbar ?? (hasToolbarTools && mergedTools ? <PlaygroundToolbar tools={mergedTools} /> : undefined);

	return (
		<PlaygroundContext.Provider
			value={{
				runtime,
				activeAppId: runtime.activeAppId,
				activeApp,
				activeModeId,
			}}
		>
			<Layout
				className={className}
				mobile={resolvedMobile}
				navbar={<Navbar items={navbarItems} />}
				footer={footerItems.length > 0 ? <Footer items={footerItems} /> : undefined}
				toolbar={toolbarElement}
				leftSidePanel={
					!resolvedMobile && workbenchTabs.length > 0
						? {
								position: "left",
								visible: panelVisibility.leftSidePanel,
								size: leftPanelSize,
								onSizeChange: setLeftPanelSize,
								tabs: workbenchTabs,
							}
						: undefined
				}
				rightSidePanel={
					!resolvedMobile && detailsTabs.length > 0
						? {
								position: "right",
								visible: panelVisibility.rightSidePanel,
								size: rightPanelSize,
								onSizeChange: setRightPanelSize,
								tabs: detailsTabs,
							}
						: undefined
				}
				canvas={
					<UICanvas
						windowKinds={goldenWindowKinds}
						defaultLayout={activeApp.defaultLayout}
						onActiveWindowChange={(windowKindId) => {
							setActiveWindowKindId(windowKindId);
							onActiveWindowChange?.(windowKindId);
						}}
					/>
				}
			/>
		</PlaygroundContext.Provider>
	);
};
//#endregion 🔖PlaygroundView

//#region 🔖Mount
type ElementsDomRoot = HTMLElement & { __elementsPlaygroundRoot?: Root };

/** @emoji 🚀 Mounts an arbitrary React tree into `#root` (or `rootId`). */
export function mountPlaygroundApp(element: React.ReactElement, rootId = "root"): void {
	if (typeof document === "undefined") return;
	const rootElement = document.getElementById(rootId) as ElementsDomRoot | null;
	if (!rootElement) throw new Error(`React root #${rootId} missing.`);
	rootElement.__elementsPlaygroundRoot ??= createRoot(rootElement);
	rootElement.__elementsPlaygroundRoot.render(element);
}

/** @emoji 🚀 Alias for {@link mountPlaygroundApp}. */
export const mountReactApp = mountPlaygroundApp;
//#endregion 🔖Mount

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("enforcePlaygroundTreePanel", () => {
		it("rejects sections without items or content", () => {
			expect(() =>
				enforcePlaygroundTreePanel({
					sections: [{ id: "a" }],
				}),
			).toThrow(/items or content/);
		});

		it("accepts content-only sections", () => {
			expect(() =>
				enforcePlaygroundTreePanel({
					sections: [{ id: "a", content: "panel body" }],
				}),
			).not.toThrow();
		});

		it("accepts sections with items", () => {
			expect(() =>
				enforcePlaygroundTreePanel({
					sections: [{ id: "a", items: [{ id: "i", label: "Item" }] }],
				}),
			).not.toThrow();
		});
	});
}
//#endregion 🧪Tests
