// #region 🧱Header
/** @emoji 🎨 `@semio-tech/framework-renderer-react` — trusted React renderer for declarative Rust plugin UI trees. */
// #endregion 🧱Header

export type { CommandDescriptor, PluginManifest, UiNode, ViewState } from "./types.ts";
export { bootFrameworkOs, FrameworkOsShell } from "./os-shell.tsx";
export { loadPluginWasm, type PluginWasmHandle } from "./plugin-runtime.ts";
export { interpretUiNode, renderUiControl, uiTreeNodeToTreePanelConfig } from "./ui-interpreter.tsx";
