/** 🌿️ VCS editor — subset-level typed twin. Re-exports every window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️component.rs`'s `create_vcs_app()` stitching every window/mode module together. */

export const VCS_EDITOR_DIALECT = { artifactKind: "s.vcs.vcs", standard: "1", subset: "*" } as const;

export const VCS_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): both windows are independent view-models with their own
// windowKindId/bodyKey literal constants — namespacing avoids collisions if a future window shares a
// field name.
export * as historyWindow from "./🎭️modes/✏️edit/🪟️windows/📜️history/🟦️component";
export * as editorWindow from "./🎭️modes/✏️edit/🪟️windows/📝️editor/🟦️component";
