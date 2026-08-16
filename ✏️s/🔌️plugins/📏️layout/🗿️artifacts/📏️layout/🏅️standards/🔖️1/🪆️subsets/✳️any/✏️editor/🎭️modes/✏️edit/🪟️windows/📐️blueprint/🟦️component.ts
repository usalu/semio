/** 📐️ Layout editor — Blueprint window: typed twin of `🦀️component.rs`'s view-model. Mirrors the
 * pane's `render(engine: &mut LayoutEngine, doc: &LayoutSnapshot, config: &LayoutConfig)` boundary —
 * the host canvas-2d layer payload plus the authoring surface's own ephemeral camera pose, absent
 * entirely from the viewer's read-only twin (see `👁️viewer/…/👁️preview/🟦️component.ts`). */

/** ✏️ Ephemeral per-surface camera pose — mirrors the Rust `LayoutCamera`. */
export interface LayoutCameraViewModel {
  x: number;
  y: number;
  zoom: number;
}

/** ✏️ The Blueprint window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface LayoutBlueprintViewModel {
  windowKindId: "layout-blueprint";
  bodyKey: "layout.play.blueprint";
  surfaceId: "layout.play.blueprint";
  camera: LayoutCameraViewModel;
}

export const LAYOUT_PLAY_WINDOW_BLUEPRINT = "layout-blueprint" as const;
export const LAYOUT_PLAY_BODY_BLUEPRINT = "layout.play.blueprint" as const;
export const LAYOUT_PLAY_SURFACE_BLUEPRINT = "layout.play.blueprint" as const;
