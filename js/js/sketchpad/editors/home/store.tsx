// #region Header

// store.tsx

// Home editor-specific state management.
// This file re-exports all Home-related types, hooks, and commands from the main store.
// The implementation remains in the main store due to tight coupling with Y.js infrastructure,
// but this provides a clean, decentralized API for the home editor.

// #endregion

export type { HomeCommandContext, HomeCommandResult, HomeDiff, HomeSelection, HomeSelectionDiff, HomeSortColumn, HomeSortDirection, HomeState } from "../../store";

export { useHome, useHomeCommands, useHomePanelVisibility } from "../../store";
