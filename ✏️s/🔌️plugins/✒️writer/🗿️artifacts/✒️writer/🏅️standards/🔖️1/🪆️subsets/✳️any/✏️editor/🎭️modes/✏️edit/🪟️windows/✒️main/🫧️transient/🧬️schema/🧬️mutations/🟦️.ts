import type { WriterEditorSelection, WriterMainWindowTransient } from "../🟦️";

export type WriterMainWindowTransientMutation =
  | { kind: "set-editor-selection"; selection: WriterEditorSelection | null }
  | { kind: "set-lint-generation"; value: number }
  | { kind: "set-engagement-input"; value: string };

export function applyWriterMainWindowTransientMutation(state: WriterMainWindowTransient, mutation: WriterMainWindowTransientMutation): WriterMainWindowTransient {
  switch (mutation.kind) {
    case "set-editor-selection": return { ...state, editorSelection: mutation.selection };
    case "set-lint-generation": return { ...state, lintGeneration: mutation.value };
    case "set-engagement-input": return { ...state, engagementInput: mutation.value };
  }
}
