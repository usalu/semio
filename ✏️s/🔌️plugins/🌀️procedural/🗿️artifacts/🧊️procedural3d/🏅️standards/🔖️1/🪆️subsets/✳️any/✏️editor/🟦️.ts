/** ✏️ Procedural3d editor — subset-level typed twin. Re-exports every window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_procedural3d_app()` stitching every window/mode module together. */

export const PROCEDURAL3D_EDITOR_DIALECT = { artifactKind: "s.procedural.procedural3d", standard: "1", subset: "*" } as const;

export const PROCEDURAL3D_PLAY_MODE_EDIT = "edit" as const;
export const PROCEDURAL3D_PLAY_MODE_GENERATE = "generate" as const;

// 🪟️ Namespaced (not `export *`): every window independently exports a same-named
// `<Window>ViewModel` interface, and a blanket `export *` from more than one of them would be an
// ambiguous re-export.
export * as flowWindow from "./🎭️modes/✏️edit/🪟️windows/🕸️flow/🟦️component";
export * as previewWindow from "./🎭️modes/✏️edit/🪟️windows/👁️preview/🟦️component";
export * as generationsWindow from "./🎭️modes/🧬️generate/🪟️windows/🗂️generations/🟦️component";
export * as generateFormWindow from "./🎭️modes/🧬️generate/🪟️windows/📝️form/🟦️component";
export * as generatePreviewWindow from "./🎭️modes/🧬️generate/🪟️windows/👁️preview/🟦️component";
