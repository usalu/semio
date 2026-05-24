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

export {
	registerElementIcon,
	registerShellTabIcon,
	registerWindowBody,
	registerSidePanelBody,
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
	useApp,
	registerElementIcon,
	getLevelBgClass,
	LevelProvider,
	ReactUI,
	type WorkbenchViewProps,
	type AppProps,
	type AppContextValue,
	type UIPanelVisibility,
} from "./workbench-bridge.tsx";
