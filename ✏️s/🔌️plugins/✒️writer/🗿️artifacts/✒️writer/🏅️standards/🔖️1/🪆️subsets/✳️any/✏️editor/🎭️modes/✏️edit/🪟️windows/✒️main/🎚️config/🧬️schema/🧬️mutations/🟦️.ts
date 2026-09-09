import type { WriterCamera, WriterEditorSettings, WriterMainWindowConfig } from "../🟦️";

export type WriterMainWindowConfigMutation =
  | { kind: "set-camera"; camera: WriterCamera }
  | { kind: "set-editor-settings"; settings: WriterEditorSettings };

export function applyWriterMainWindowConfigMutation(state: WriterMainWindowConfig, mutation: WriterMainWindowConfigMutation): WriterMainWindowConfig {
  return mutation.kind === "set-camera" ? { ...state, camera: mutation.camera } : { ...state, editorSettings: mutation.settings };
}
