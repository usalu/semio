/** ✏️ Procedural2d editor — subset-level typed twin. Re-exports every window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_procedural2d_app()` stitching every window/mode module together. */

export const PROCEDURAL2D_EDITOR_DIALECT = { artifactKind: "s.procedural.procedural2d", standard: "1", subset: "*" } as const;

export const PROCEDURAL2D_PLAY_MODE_EDIT = "edit" as const;
export const PROCEDURAL2D_PLAY_MODE_GENERATE = "generate" as const;

// 🪟️ Namespaced (not `export *`): keeps each window's view-model/constant names scoped under its own
// module even though none currently collide, so a future window can reuse a common name (e.g.
// `ViewModel`) without silently becoming an ambiguous re-export here.
export * as flowWindow from "./🎭️modes/✏️edit/🪟️windows/🕸️flow/🟦️";
export * as editPreviewWindow from "./🎭️modes/✏️edit/🪟️windows/👁️preview/🟦️";
export * as generatePreviewWindow from "./🎭️modes/🧬️generate/🪟️windows/👁️preview/🟦️";
export * as generateFormWindow from "./🎭️modes/🧬️generate/🪟️windows/📝️form/🟦️";
export * as generationsWindow from "./🎭️modes/🧬️generate/🪟️windows/🗂️generations/🟦️";
