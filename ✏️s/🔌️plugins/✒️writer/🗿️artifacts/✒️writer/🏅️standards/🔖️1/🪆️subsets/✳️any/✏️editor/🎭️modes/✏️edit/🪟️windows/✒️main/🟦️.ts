/** ✒️ Writer editor — Main window: typed twin of `🦀️.rs`'s view-model. Mirrors the window's
 * `render(document: &WriterSnapshot, config: &WriterConfig) -> UiNode` boundary's inputs (the
 * `TextEditorScene` payload every field below projects onto) plus the window-kind id/body-key/
 * surface-id constants the Rust side declares as bare `&str` consts. */

/** ✏️ Raw caret/range selection, editor-intrinsic (not the `ast` interaction domain). */
export interface WriterEditorSelection {
  start: number;
  end: number;
}

/** ✏️ Editor chrome settings — mirrors Rust `WriterEditorSettings`. */
export interface WriterEditorSettings {
  showLineNumbers: boolean;
  fontPx: number;
  lineHeight: number;
  tabSize: number;
}

/** ✏️ Editor viewport pan/zoom — session-only, never a document field. */
export interface WriterCamera {
  x: number;
  y: number;
  zoom: number;
}

/** ✏️ The Main window's typed view-model — mirrors the Rust `render()` boundary's inputs, itself the
 * `TextEditorScene` shape `build_text_editor_scene` emits (every field JSON-encoded, matching the
 * Rust struct's `Option<String>` fields one-for-one). */
export interface WriterMainViewModel {
  windowKindId: "writer-main";
  bodyKey: "writer.play.main";
  surfaceId: "writer.play";
  buffer: string;
  language: string | null;
  selectionJson: string | null;
  tokensJson: string | null;
  diagnosticsJson: string | null;
  completionsJson: string | null;
  occurrencesJson: string | null;
  overlaysJson: string | null;
  placeholdersJson: string | null;
  extraCaretsJson: string | null;
  selectableSpansJson: string | null;
  settingsJson: string | null;
  cameraJson: string | null;
  hoverJson: string | null;
  newlineGatesJson: string | null;
  renameJson: string | null;
}

export const WRITER_PLAY_WINDOW_KIND = "writer-main" as const;
export const WRITER_PLAY_BODY_MAIN = "writer.play.main" as const;
export const WRITER_PLAY_SURFACE_MAIN = "writer.play" as const;
