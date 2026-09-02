/** ✏️ Sequence editor — subset-level typed twin. Re-exports every window's typed view-model binding
 * so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_sequence_app()` stitching every window/mode module together. */

export const SEQUENCE_EDITOR_DIALECT = { artifactKind: "s.sequence.sequence", standard: "1", subset: "*" } as const;

export const SEQUENCE_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): each window declares its own view-model interface; namespacing
// keeps every window's exports addressable without relying on their names never colliding.
export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/📽️main/🟦️component";
export * as scriptWindow from "./🎭️modes/✏️edit/🪟️windows/📜️script/🟦️component";
export * as compiledWindow from "./🎭️modes/✏️edit/🪟️windows/🧬️compiled/🟦️component";
