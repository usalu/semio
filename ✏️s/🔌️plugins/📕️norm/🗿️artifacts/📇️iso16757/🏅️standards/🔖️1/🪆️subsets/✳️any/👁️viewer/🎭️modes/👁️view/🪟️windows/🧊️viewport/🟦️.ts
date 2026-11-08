/** 🧊️ ISO 16757 viewer — viewport window: typed twin of `🦀️.rs`, the projected masonry scene on the `world-3d` surface. */

export interface Iso16757ViewportViewModel {
  windowKindId: "norm-iso16757-viewport";
  bodyKey: "norm.iso16757.view.viewport";
}

export const ISO16757_VIEW_VIEWPORT_WINDOW_KIND_ID = "norm-iso16757-viewport" as const;
export const ISO16757_VIEW_VIEWPORT_BODY_KEY = "norm.iso16757.view.viewport" as const;
