/** 👁️ Layout editor — Preview window: typed twin of `🦀️.rs`'s view-model. The authoring
 * app's own unchromed read-only pane (separate from `👁️viewer/…/👁️preview`, which is a genuinely
 * independent surface — this window still renders through the editor's `LayoutEngine`/full glyph
 * layout, the viewer's does not, see that file's own doc). Mirrors the pane's
 * `render(engine: &mut LayoutEngine, doc: &LayoutSnapshot, config: &LayoutConfig)` boundary. */

import type { LayoutCameraViewModel } from "../📐️blueprint/🟦️component";

/** 👁️ The Preview window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface LayoutPreviewViewModel {
  windowKindId: "layout-preview";
  bodyKey: "layout.play.preview";
  surfaceId: "layout.play.preview";
  camera: LayoutCameraViewModel;
}

export const LAYOUT_PLAY_WINDOW_PREVIEW = "layout-preview" as const;
export const LAYOUT_PLAY_BODY_PREVIEW = "layout.play.preview" as const;
export const LAYOUT_PLAY_SURFACE_PREVIEW = "layout.play.preview" as const;
