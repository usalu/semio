/** 🖼️ Shooting editor — Icon window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * window's `render(snapshot: &ShootingSnapshot, cfg: &ShootingConfig)` boundary — the icon-render
 * request payload plus the sticky default shot/asset/format chrome a mutation-capable surface carries. */

/** ✏️ The Icon window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface ShootingIconViewModel {
  windowKindId: "shooting-icon";
  bodyKey: "shooting.play.icon";
  surfaceId: "shooting.play.icon";
  defaultShotFormat: "svg" | "png";
  defaultShotShape: "rectangle" | "ellipse";
  defaultAssetFormat: "glb";
}

export const SHOOTING_PLAY_ICON_WINDOW_KIND_ID = "shooting-icon" as const;
export const SHOOTING_PLAY_ICON_BODY_KEY = "shooting.play.icon" as const;
export const SHOOTING_PLAY_ICON_SURFACE_ID = "shooting.play.icon" as const;
