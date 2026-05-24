// #region 🧲Header
/** @emoji 🖥 Workbench shell bridge — {@link WorkbenchView} and mount helpers for `@elements/framework-react`. */
// #endregion 🧲Header

export { WorkbenchView, App } from "./workbench-view.tsx";
export { ReactUI, mountAsyncReactApp, mountReactApp } from "./workbench-mount.tsx";
export { getLevelBgClass, LevelProvider } from "@elements/ui";

export { useApp, type AppContextValue, type AppProps, type UIPanelVisibility, type WorkbenchViewProps } from "./workbench-app-context.tsx";
export { registerElementIcon } from "./shell-bridge.tsx";
