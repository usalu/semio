// #region Header

// store.tsx

// Kit editor-specific state management.
// This file re-exports all KitEditor-related types, hooks, and commands from the main store.
// The implementation remains in the main store due to tight coupling with Y.js infrastructure,
// but this provides a clean, decentralized API for the kit editor.

// #endregion

export type {
  KitEditorCommandContext,
  KitEditorCommandResult,
  KitEditorDiff,
  KitEditorEdit,
  KitEditorHover,
  KitEditorId,
  KitEditorPresence,
  KitEditorPresenceOther,
  KitEditorSelection,
  KitEditorSelectionDesignsDiff,
  KitEditorSelectionDiff,
  KitEditorSelectionTypesDiff,
  KitEditorSortColumn,
  KitEditorSortDirection,
  KitEditorState,
  KitEditorStep,
} from "../../store";

export { KitEditorFullscreenWindow } from "../../store";

export { useKitEditor, useKitEditorCommands, useKitEditorFullscreen, useKitEditorOthers, useKitEditorSelection } from "../../store";
