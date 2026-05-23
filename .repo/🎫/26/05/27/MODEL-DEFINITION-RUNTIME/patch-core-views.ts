#!/usr/bin/env bun
const path = "c:/git/semio/spatial/js/core/index.ts";
let s = await Bun.file(path).text();

s = s.replace(
  `/** @emoji 🏷️ Parsed extension manifest (\`spatial.extension/v1\`). */
export interface ExtensionManifest {
  readonly schema: "spatial.extension/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly kinds: readonly string[];
}

/** @emoji 🧾 Parses \`spatial.extension/v1\` JSON or returns \`null\`. */
export function parseExtensionManifest(raw: unknown): ExtensionManifest | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.extension/v1") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (!Array.isArray(r.kinds) || r.kinds.length === 0) return null;
  return {
    schema: "spatial.extension/v1",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    kinds: r.kinds as string[],
  };
}`,
  `/** @emoji 🏷️ Parsed model-definition manifest (\`spatial.extension/v1\` envelope on disk). */
export interface ModelDefinitionManifest {
  readonly schema: "spatial.extension/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly kinds: readonly string[];
}

/** @emoji 🧾 Parses a model-definition manifest JSON or returns \`null\`. */
export function parseModelDefinitionManifest(raw: unknown): ModelDefinitionManifest | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.extension/v1") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (!Array.isArray(r.kinds) || r.kinds.length === 0) return null;
  return {
    schema: "spatial.extension/v1",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    kinds: r.kinds as string[],
  };
}

/** @emoji 📚 Lists model-definition manifests under \`spatial/assets/modelDefinition/**\`. */
export function listModelDefinitionManifests(): readonly ModelDefinitionManifest[] {
  return modelDefinitionManifestCatalog()
    .map((raw) => parseModelDefinitionManifest(raw))
    .filter((m): m is ModelDefinitionManifest => m !== null);
}`,
);

const viewBlockStart = `/** @emoji 👁️ Extension view definition with readonly derived typology ids. */`;
const viewBlockEnd = `// #endregion 🧱Model`;
const start = s.indexOf(viewBlockStart);
const end = s.indexOf(viewBlockEnd);
if (start < 0 || end < 0) throw new Error("view block markers not found");
s = s.slice(0, start) + viewBlockEnd + s.slice(end + viewBlockEnd.length);

s = s.replaceAll("ExtensionViewService | null", "null");
s = s.replaceAll("ExtensionViewService | undefined", "undefined");
s = s.replaceAll("ExtensionViewService,", "null,");
s = s.replaceAll("ExtensionViewService;", "null;");
s = s.replaceAll("ExtensionViewService)", "null)");
s = s.replaceAll("ExtensionViewService ", "null ");
s = s.replaceAll("readonly views?: ExtensionViewService", "readonly views?: null");
s = s.replaceAll("views?: ExtensionViewService", "views?: null");

s = s.replace(
  `export function collectGeometrySelectionTargets(model: Model, kinds: readonly ModelEntityKind[], views?: ExtensionViewService | null, activeViewId?: string | null): SelectionTarget[] {`,
  `export function collectGeometrySelectionTargets(model: Model, kinds: readonly ModelEntityKind[], _views?: null, _activeViewId?: string | null): SelectionTarget[] {`,
);

s = s.replace(
  `      case "object":
        for (const o of views?.computeObjects(model, activeViewId ?? null) ?? []) push(kind, String(o.id), false);
        break;`,
  `      case "object":
        for (const id of Object.keys(model.objects)) push(kind, id, false);
        break;`,
);

s = s.replace(
  `export function applySelectionOperation(operation: SelectionApplyOperation, current: readonly SelectionTarget[], model: Model, kinds: readonly ModelEntityKind[], views?: ExtensionViewService | null, activeViewId?: string | null): SelectionTarget[] {
  if (operation === "deselectAll") return [];
  const scopeKinds = kinds.length > 0 ? kinds : [...ALL_MODEL_SELECTION_KINDS];
  const universe = collectGeometrySelectionTargets(model, scopeKinds, views, activeViewId);`,
  `export function applySelectionOperation(operation: SelectionApplyOperation, current: readonly SelectionTarget[], model: Model, kinds: readonly ModelEntityKind[], _views?: null, _activeViewId?: string | null): SelectionTarget[] {
  if (operation === "deselectAll") return [];
  const scopeKinds = kinds.length > 0 ? kinds : [...ALL_MODEL_SELECTION_KINDS];
  const universe = collectGeometrySelectionTargets(model, scopeKinds);`,
);

s = s.replace(
  `export function executeSelectionApply(params: SelectionApplyParams, ctx: { readonly model: Model; readonly views?: ExtensionViewService | null; readonly activeViewId?: string | null }): SelectionTarget[] {
  const seed = params.seedTargets ?? [];
  const kinds = params.operation === "selectKinds" ? [...(params.kinds ?? [])] : params.operation === "invert" || params.operation === "selectAll" ? [...ALL_MODEL_SELECTION_KINDS] : [];
  return applySelectionOperation(params.operation, seed, ctx.model, kinds, ctx.views ?? null, ctx.activeViewId ?? null);
}`,
  `export function executeSelectionApply(params: SelectionApplyParams, ctx: { readonly model: Model }): SelectionTarget[] {
  const seed = params.seedTargets ?? [];
  const kinds = params.operation === "selectKinds" ? [...(params.kinds ?? [])] : params.operation === "invert" || params.operation === "selectAll" ? [...ALL_MODEL_SELECTION_KINDS] : [];
  return applySelectionOperation(params.operation, seed, ctx.model, kinds);
}`,
);

s = s.replace(
  `/** @emoji 🪪 True when a selection command needs \`ExtensionViewService\` (\`object\` rows). */
export function selectionOperationUsesViewObjects(defn: Pick<SelectionOperationInteractionDef, "kinds">): boolean {
  return defn.kinds?.includes("object") ?? false;
}`,
  `/** @emoji 🪪 True when a selection command targets authored \`object\` rows on the model. */
export function selectionOperationUsesViewObjects(defn: Pick<SelectionOperationInteractionDef, "kinds">): boolean {
  return defn.kinds?.includes("object") ?? false;
}`,
);

s = s.replace(
  `  const views = opts.views ?? (selectionOperationUsesViewObjects(defn) ? ExtensionViewService.forKernel(opts.kernel) : undefined);`,
  `  const views = opts.views ?? undefined;`,
);

await Bun.write(path, s);
console.log("patched core views");
