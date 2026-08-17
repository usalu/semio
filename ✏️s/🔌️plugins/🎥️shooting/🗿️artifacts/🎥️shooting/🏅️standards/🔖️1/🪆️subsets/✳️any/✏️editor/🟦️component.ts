/** ✏️ Shooting editor — subset-level typed twin. Re-exports every window's typed view-model binding so
 * a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️component.rs`'s `create_shooting_app()` stitching every window/mode module together. */

export const SHOOTING_EDITOR_DIALECT = { artifactKind: "s.shooting.shooting", standard: "1", subset: "*" } as const;

export const SHOOTING_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): both windows independently declare their own typed view-model
// interface names, so a namespaced re-export keeps the import surface unambiguous even if either
// window later grows a same-named export (matches the cad pilot's own precedent).
export * as sceneWindow from "./🎭️modes/✏️edit/🪟️windows/🎥️scene/🟦️component";
export * as iconWindow from "./🎭️modes/✏️edit/🪟️windows/🖼️icon/🟦️component";
