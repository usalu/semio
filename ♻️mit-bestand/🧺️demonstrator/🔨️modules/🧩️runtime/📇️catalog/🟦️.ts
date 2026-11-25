import catalog from "../🔣️.json" with { type: "json" };

export const DEMONSTRATOR_HOST = catalog.host;
export const DEMONSTRATOR_ASSETS_DIR = catalog.assetsDirectory;
export const DEMONSTRATOR_RUNTIME_PANES = catalog.panes;

/** 🧯️ Extensions this deployment neither ships nor offers, because their contributions stall the editors they extend. */
const withheldExtensions: ReadonlySet<string> = new Set(catalog.withheldExtensions);

/** 🚢️ Keeps the components this deployment ships. */
export function demonstratorShippedComponents<Row extends { readonly pluginId: string }>(rows: readonly Row[]): readonly Row[] {
  return rows.filter(row => !withheldExtensions.has(row.pluginId));
}

/** 🎛️ Selects a pane's authored runtime variant. */
export function demonstratorPaneRuntimeVariant(variant: string): string {
  const pane = catalog.panes.find(row => row.variant === variant);
  if (!pane) throw new Error(`Unknown demonstrator pane variant: ${variant}`);
  return pane.runtimeVariant;
}

