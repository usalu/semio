// #region 🧲Header
/** @emoji ⚛️ `@elements/framework-react` — React renderer for {@link @elements/framework}: declarative {@link UiNode} host. */
// #endregion 🧲Header

export {
	UiRenderer,
	registerUiBoardSurfaceHost,
	registerUiTableSurfaceHost,
	registerUiScene3DSurfaceHost,
	unregisterUiBoardSurfaceHost,
	unregisterUiTableSurfaceHost,
	unregisterUiScene3DSurfaceHost,
	type UiRendererProps,
} from "./ui-declarative-renderer.tsx";

export type { Workbench } from "@elements/framework";

export type {
	AppToolCategory,
	AppTools,
	FooterItem,
	ShellChromeTreePanelConfig,
	SidePanelTabConfig,
	UIWindowKindDefinition,
	UIWindowMeasure,
	UIToolbarItem,
} from "./shell-chrome-types.tsx";

export { APP_TOOL_CATEGORY_ORDER } from "./shell-chrome-types.tsx";

export {
	registerElementIcon,
	registerShellTabIcon,
	registerWindowBody,
	registerSidePanelBody,
	resolveElementIcon,
	shellWindowKindsToGolden,
	shellSideTabsToPanelTabs,
	shellFooterToFooterItems,
	shellToolsToAppTools,
	mergeConfigEntries,
} from "./shell-bridge.tsx";

export {
	AppContext,
	useApp,
	type AppContextValue,
	type AppProps,
	type UIPanelVisibility,
	type WorkbenchViewProps,
} from "./workbench-app-context.tsx";

export {
	WorkbenchView,
	mountReactApp,
	mountAsyncReactApp,
	getLevelBgClass,
	LevelProvider,
	ReactUI,
} from "./workbench-bridge.tsx";
