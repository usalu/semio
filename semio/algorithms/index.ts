// #region 🧲Header
// 💻 semio/algorithms/index.ts
// Specs: Re-exports AlgorithmApp, native algorithm REST adapter helpers, and window kinds for Storybook.
// Summary: Algorithm bundle entry re-exporting AlgorithmApp from @semio/ui and nativeAlgorithmAdapter.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

export { AlgorithmApp, WindowKind, createIpoAlgorithmLayout, useAlgorithm, type AlgorithmAppProps, type AlgorithmContextValue, type AlgorithmWindowDef, type VecValue } from "@semio/ui";
export { nativeDeletePieces, nativeDragPieces, nativeFlatDesign, nativeFlattenDesign, nativeFlattenedDesign, nativeMovePieces, type NativeAlgorithmExecutePayload, type NativeAlgorithmLanguage, type NativeAlgorithmOperation } from "./nativeAlgorithmAdapter";
