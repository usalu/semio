// #region Header

// store.tsx

// Type editor-specific state management.
// This file re-exports all TypeEditor-related types, hooks, and commands from the main store.
// The implementation remains in the main store due to tight coupling with Y.js infrastructure,
// but this provides a clean, decentralized API for the type editor.

// #endregion

export type {
  TypeEditorCommandContext,
  TypeEditorCommandResult,
  TypeEditorDiff,
  TypeEditorEdit,
  TypeEditorHover,
  TypeEditorId,
  TypeEditorPresence,
  TypeEditorPresenceOther,
  TypeEditorSelection,
  TypeEditorSelectionDiff,
  TypeEditorSelectionPortsDiff,
  TypeEditorSelectionRepresentationsDiff,
  TypeEditorState,
  TypeEditorStep,
} from "../../store";

export { TypeEditorFullscreenWindow } from "../../store";

export { useTypeEditor, useTypeEditorCamera, useTypeEditorCommands, useTypeEditorOthers, useTypeEditorPanelVisibility, useTypeEditorSelection } from "../../store";
