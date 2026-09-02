/** ✏️ Playbook editor — subset-level typed twin. Re-exports the Builder window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_playbook_play_app()` stitching every window/mode module together. */

export const PLAYBOOK_EDITOR_DIALECT = { artifactKind: "s.playbook.playbook", standard: "1", subset: "*" } as const;

export const PLAYBOOK_PLAY_MODE_BUILDER = "builder" as const;

export * from "./🎭️modes/🏗️builder/🪟️windows/🏗️builder/🟦️component";
