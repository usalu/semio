/** 🪟️ Playground viewer — main window: typed twin of `🦀️component.rs`'s `TextWindowKit` view-model.
 * Read-only mirror of the framework `TextView` payload `render()` produces. */

export interface PlaygroundViewMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: "playground";
  readOnly: true;
}

export const PLAYGROUND_VIEW_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const PLAYGROUND_VIEW_MAIN_BODY_KEY = "framework.window.text" as const;
