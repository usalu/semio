// #region 🧲Header
/** @emoji 🧭 {@link WorkbenchView} — framework workbench shell (navbar, panels, golden-layout canvas). */
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	Workbench,
	WorkbenchApp,
	WorkbenchMode,
	WorkbenchWindowKind,
	createTabStackLayout,
	type ResolvedWorkbenchAppState,
} from "@elements/framework";
import {
	ArrowLeft,
	ArrowRight,
	ArrowUp,
	Folder,
	Info,
	MessageSquare,
	Search,
	Settings2,
} from "lucide-react";
import * as React from "react";
import { renderToStaticMarkup } from "react-dom/server";

import {
	BasicChatPanel,
	ButtonGroup,
	ButtonGroupItem,
	Footer,
	Layout,
	Navbar,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	StaticSidePanelTabDefinition,
	StaticTreePanelDefinition,
	Toggle,
	UICanvas,
	UIFind,
	UIFindProvider,
	UISearch,
	UIToolbar,
	cn,
	countAppTools,
	listPopulatedAppToolCategories,
	mergeAppTools,
	useCommandHotkey,
	useMediaQuery,
	useUIFind,
	type FooterItem,
	type NavbarItem,
	type SidePanelTabConfig,
	type UIFindItem,
	type UIWindowLayout,
} from "@elements/ui/chrome";

import {
	mergeConfigEntries,
	registerWindowBody,
	resolveElementIcon,
	shellFooterToFooterItems,
	shellSideTabsToPanelTabs,
	shellToolsToAppTools,
	shellWindowKindsToGolden,
} from "./shell-bridge.tsx";
import { AppContext, type UIPanelVisibility, type WorkbenchViewProps } from "./workbench-app-context.tsx";

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
