import { readFileSync, writeFileSync } from "node:fs";

const path = "elements/client/lib/react/index.tsx";
let s = readFileSync(path, "utf8");
const start = s.indexOf("export const App: React.FC<AppProps> = ({");
const end = s.indexOf("/**\n * Internal component that syncs app-level find items", start);
if (start < 0 || end < 0) throw new Error("patch markers not found");

const NEW = `export const WorkbenchView: React.FC<WorkbenchViewProps> = ({
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
	const panelTabs = withDefaultAppPanelTabs(activeApp, activeModeLabel);
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

	const workbenchIcon = workbenchTabs[0]?.icon ? React.createElement(workbenchTabs[0].icon, { size: 16 }) : <FolderIcon size={16} />;
	const detailsIcon = detailsTabs[0]?.icon ? React.createElement(detailsTabs[0].icon, { size: 16 }) : <InfoIcon size={16} />;
	const optionsIcon = optionsTabs[0]?.icon ? React.createElement(optionsTabs[0].icon, { size: 16 }) : <Settings2Icon size={16} />;
	const chatIcon = chatTabs[0]?.icon ? React.createElement(chatTabs[0].icon, { size: 16 }) : <MessageSquareIcon size={16} />;

	const navbarItems: NavbarItem[] = [];

	if (hasModeNav) {
		navbarItems.push({
			key: "modeNav",
			content: (
				<Select id={\`ui.mode.select.\${activeAppBase.id}\`} onValueChange={setActiveModeId} value={activeModeId ?? undefined}>
					<SelectTrigger className="h-medium w-30" id={\`ui.mode.select.\${activeAppBase.id}.trigger\`} size="sm">
						<SelectValue placeholder="Mode" />
					</SelectTrigger>
					<SelectContent>
						{activeAppBase.modes.map((mode) => (
							<SelectItem key={mode.id} id={\`ui.mode.select.\${activeAppBase.id}.\${mode.id}\`} value={mode.id}>
								<span className="flex items-center gap-single">
									{mode.iconId ? elementIconNodes.get(mode.iconId) ?? null : null}
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
					<NavigateBackIcon className="size-small" />
				</ButtonGroupItem>
			</ButtonGroup>
		),
	});
	navbarItems.push({
		key: "navForward",
		content: (
			<ButtonGroup id="ui.nav.forward">
				<ButtonGroupItem id="ui.nav.forward" onClick={onGoForward} className={cn(!canGoForwardProp && "opacity-30 pointer-events-none")}>
					<NavigateForwardIcon className="size-small" />
				</ButtonGroupItem>
			</ButtonGroup>
		),
	});
	navbarItems.push({
		key: "navUp",
		content: (
			<ButtonGroup id="ui.nav.up">
				<ButtonGroupItem id="ui.nav.up" onClick={onGoUp} className={cn(!canGoUpProp && "opacity-30 pointer-events-none")}>
					<NavigateUpIcon className="size-small" />
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
						<ButtonGroupItem key={app.id} id={\`ui.appNav.\${app.id}\`} className={cn(activeAppId === app.id && "bg-active-base")} onClick={() => setActiveAppId(app.id)}>
							{app.iconId ? elementIconNodes.get(app.iconId) ?? <span className="text-xs">{app.label}</span> : <span className="text-xs">{app.label}</span>}
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
		content: <Toggle kind="icon" id="ui.search.toggle" pressed={searchOpen} onPressedChange={setSearchOpen} icon={<SearchIcon size={16} />} />,
	});

	navbarItems.push({
		key: "find",
		content: <Toggle kind="icon" id="ui.find.toggle" pressed={findOpen} onPressedChange={setFindOpen} icon={<SearchIcon size={16} />} />,
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

	const mergedFooterItems = [...shellFooterToFooterItems(workbench.globalFooterItems, workbench.commandBus), ...shellFooterToFooterItems(activeApp.footerItems, workbench.commandBus)].sort(
		(a, b) => (a.order ?? 0) - (b.order ?? 0),
	);

	const searchItemsResolved = React.useMemo(
		() =>
			workbench.searchItems.map((row) => ({
				id: row.id,
				label: row.label,
				description: row.description,
				category: row.category,
				icon: row.iconId ? elementIconNodes.get(row.iconId) : undefined,
				onSelect: () => workbench.commandBus.dispatch(row.controllerId, row.command, row.args),
			})),
		[workbench, shellGen],
	);

	const goldenWindowKinds = React.useMemo(() => shellWindowKindsToGolden(activeApp.windowKinds), [activeApp.windowKinds, shellGen]);

	const toolbarElement = hasToolbarTools && mergedTools ? <UIToolbar tools={mergedTools} /> : undefined;

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
				<UIFindItemsSync findItems={activeApp.findItems} onFindSelect={activeApp.onFindSelect} />
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

export const App = WorkbenchView;
`;

writeFileSync(path, s.slice(0, start) + NEW + s.slice(end));
