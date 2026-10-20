import catalog from "../🔣️.json";

export const DEMONSTRATOR_HOST = catalog.host;
export const DEMONSTRATOR_ASSETS_DIR = catalog.assetsDirectory;
export const DEMONSTRATOR_RUNTIME_PANES = catalog.panes;

/** 🎛️ Selects a pane's authored runtime variant. */
export function demonstratorPaneRuntimeVariant(variant: string): string {
  const pane = catalog.panes.find(row => row.variant === variant);
  if (!pane) throw new Error(`Unknown demonstrator pane variant: ${variant}`);
  return pane.runtimeVariant;
}

