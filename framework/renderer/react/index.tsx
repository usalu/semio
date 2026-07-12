// #region 🧱Header
/** @emoji 🎨 `@semio-tech/framework-renderer-react` — trusted React renderer for declarative Rust plugin UI trees. */
// #endregion 🧱Header

export type { ActionDescriptor, UiNode } from "@semio-tech/framework-core";
export type { PluginManifest, ViewState } from "./os-shell.tsx";
export { bootFrameworkOs, FrameworkOsShell } from "./os-shell.tsx";
export { loadPluginWasm, type PluginWasmHandle } from "./os-shell.tsx";
export { interpretUiNode, renderUiControl, uiTreeNodeToTreePanelConfig } from "./ui-interpreter.tsx";
