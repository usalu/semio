// #region 🧲Header
/** @emoji 🔁 Re-exports dependency-boundary lint from `@repo/lib/js` index after monolith consolidation. */
// #endregion 🧲Header

export {
  isAdapterBoundaryFile,
  shouldSkipDependencyBoundaryFile,
  loadThirdPartyDeps,
  parseTsImportSpecs,
  dependencyBoundaryBreachesForFile,
} from "./index.ts";
