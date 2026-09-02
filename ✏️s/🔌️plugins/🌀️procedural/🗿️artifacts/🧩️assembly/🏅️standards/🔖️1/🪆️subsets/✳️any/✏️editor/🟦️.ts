/** ✏️ Assembly editor — subset-level typed twin. Re-exports the `structure` window's typed
 * view-model bindings so a host-side TS consumer has one import surface for the whole editor
 * manifest, mirroring `🦀️.rs`'s `create_assembly_editor()` stitching the window/mode module
 * together. Namespaced (not `export *`) for parity with every other migrated surface's convention,
 * even with a single window today — a second window later stays a pure addition, never a rename. */

export const ASSEMBLY_EDITOR_DIALECT = { artifactKind: "s.assembly", standard: "1", subset: "*" } as const;

export const ASSEMBLY_EDIT_MODE_ID = "edit" as const;

export * as structureWindow from "./🎭️modes/✏️edit/🪟️windows/🌳️structure/🟦️";
