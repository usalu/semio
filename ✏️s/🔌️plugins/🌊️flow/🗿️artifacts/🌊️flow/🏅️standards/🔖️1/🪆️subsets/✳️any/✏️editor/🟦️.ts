/** ✏️ Flow editor — subset-level typed twin. Re-exports every window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_flow_app()` stitching every window/mode module together. */

export const FLOW_EDITOR_DIALECT = { artifactKind: "s.flow.flow", standard: "1", subset: "*" } as const;

export const FLOW_PLAY_MODE_EDIT = "edit" as const;
export const FLOW_PLAY_MODE_GENERATE = "generate" as const;

// 🪟️ Namespaced (not `export *`): five windows across two modes — namespacing keeps each window's
// constants/view-model addressable without relying on every future field staying collision-free.
export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🌊️main/🟦️component";
export * as compiledWindow from "./🎭️modes/✏️edit/🪟️windows/🗣️compiled/🟦️component";
export * as generationsWindow from "./🎭️modes/🧬️generate/🪟️windows/🗂️generations/🟦️component";
export * as generateFormWindow from "./🎭️modes/🧬️generate/🪟️windows/📝️form/🟦️component";
export * as generatePreviewWindow from "./🎭️modes/🧬️generate/🪟️windows/👁️preview/🟦️component";
