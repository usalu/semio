/** ✏️ Writer editor — subset-level typed twin. Re-exports the (single) window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️component.rs`'s `create_writer_app()` stitching every window/mode module together. */

export const WRITER_EDITOR_DIALECT = { artifactKind: "s.writer.writer", standard: "1", subset: "*" } as const;

export const WRITER_PLAY_MODE_EDIT = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/✒️main/🟦️component";
