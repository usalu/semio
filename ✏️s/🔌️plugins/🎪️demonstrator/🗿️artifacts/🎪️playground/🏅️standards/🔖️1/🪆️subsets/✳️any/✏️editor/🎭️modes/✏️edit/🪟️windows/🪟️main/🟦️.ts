/** 🪟️ Playground editor — main window: typed twin of `🦀️.rs`'s `TextWindowKit` view-model.
 * Editable mirror of the framework `TextView` payload `render()` produces. */

export interface PlaygroundEditMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: "playground";
  /** ✍️ The kit's explicit-draft policy: edited locally, ONE `textEdit` on Apply. */
  commit: "explicit";
}

export const PLAYGROUND_EDIT_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const PLAYGROUND_EDIT_MAIN_BODY_KEY = "framework.window.text" as const;
