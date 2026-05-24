// #region 🧲Header
/** @emoji 🖥 Workbench shell bridge — re-exports React workbench chrome until {@link WorkbenchView} moves here fully. */
// #endregion 🧲Header

export {
	WorkbenchView,
	mountReactApp,
	mountAsyncReactApp,
	getLevelBgClass,
	LevelProvider,
	ReactUI,
} from "@elements/ui";

export { useApp, type AppContextValue, type AppProps, type UIPanelVisibility, type WorkbenchViewProps } from "./workbench-app-context.tsx";
export { registerElementIcon } from "./shell-bridge.tsx";
