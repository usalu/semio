/** ✏️ Animate editor — subset-level typed twin. Re-exports the window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_animate_presentation_app()` stitching mode/window together. */

export const ANIMATE_EDITOR_DIALECT = { artifactKind: "s.animate.presentation", standard: "1", subset: "*" } as const;

export const ANIMATE_PLAY_MODE_MAIN = "main" as const;

export * from "./🎭️modes/🖊️main/🪟️windows/🖼️tile-editor/🟦️";
