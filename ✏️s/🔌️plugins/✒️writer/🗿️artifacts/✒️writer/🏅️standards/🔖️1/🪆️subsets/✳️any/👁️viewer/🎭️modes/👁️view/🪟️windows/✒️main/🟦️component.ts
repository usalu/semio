/** ✒️ Writer viewer — Main window: typed twin of `🦀️component.rs`'s view-model. Read-only mirror of
 * the framework `TextWindowKit`'s `TextView` shape (`framework.window.text`) — text plus language,
 * `readOnly` always `true`. No selection/tokens/diagnostics/completions/camera fields: those are
 * editor-only chrome, never read from here. */

/** 👁️ The Main window's typed view-model — mirrors the Rust `render()` boundary's input, itself the
 * framework `TextView` shape. */
export interface WriterViewMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: string | null;
  readOnly: true;
}

export const WRITER_VIEW_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const WRITER_VIEW_MAIN_BODY_KEY = "framework.window.text" as const;
