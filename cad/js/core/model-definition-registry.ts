// #region 🧲Header
/** @emoji 📥 Model-definition asset module registry (populated by `assets.ts`). */
// #endregion 🧲Header

// #region 📥ModelDefinitionRegistry
/** @emoji 📥 Vite `import.meta.glob` modules registered by `@cad/js/core/assets`. */
export interface ModelDefinitionAssetModules {
  readonly typologies: Readonly<Record<string, unknown>>;
  readonly actions: Readonly<Record<string, unknown>>;
  readonly interactions: Readonly<Record<string, unknown>>;
  readonly manifests: Readonly<Record<string, unknown>>;
  readonly extensions: Readonly<Record<string, unknown>>;
  readonly attributes: Readonly<Record<string, unknown>>;
  readonly propertyDefinitions: Readonly<Record<string, unknown>>;
  readonly properties: Readonly<Record<string, unknown>>;
  readonly transformations: Readonly<Record<string, unknown>>;
}

const emptyModelDefinitionAssetModules = (): ModelDefinitionAssetModules => ({
  typologies: {},
  actions: {},
  interactions: {},
  manifests: {},
  extensions: {},
  attributes: {},
  propertyDefinitions: {},
  properties: {},
  transformations: {},
});

let modelDefinitionAssetModules: ModelDefinitionAssetModules = emptyModelDefinitionAssetModules();

let onModelDefinitionAssetsRegistered: (() => void) | null = null;

/** @emoji 📥 Hook invoked after each `registerModelDefinitionAssets` (core clears owner caches). */
export function setModelDefinitionAssetsRegisteredHook(fn: (() => void) | null): void {
  onModelDefinitionAssetsRegistered = fn;
}

/** @emoji 📥 Registers model-definition asset modules (typically from `assets.ts` at startup). */
export function registerModelDefinitionAssets(modules: ModelDefinitionAssetModules): void {
  modelDefinitionAssetModules = modules;
  onModelDefinitionAssetsRegistered?.();
}

export function modelDefinitionTypologyCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.typologies);
}

export function modelDefinitionActionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.actions);
}

export function modelDefinitionInteractionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.interactions);
}

export function modelDefinitionManifestCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionAssetModules.manifests), ...Object.values(modelDefinitionAssetModules.extensions)];
}

export function modelDefinitionAttributeCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.attributes);
}

export function modelDefinitionPropertyCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionAssetModules.propertyDefinitions), ...Object.values(modelDefinitionAssetModules.properties)];
}

export function modelDefinitionTransformationModules(): Readonly<Record<string, unknown>> {
  return modelDefinitionAssetModules.transformations;
}

export function modelDefinitionAssetModulesSnapshot(): ModelDefinitionAssetModules {
  return modelDefinitionAssetModules;
}
// #endregion 📥ModelDefinitionRegistry
