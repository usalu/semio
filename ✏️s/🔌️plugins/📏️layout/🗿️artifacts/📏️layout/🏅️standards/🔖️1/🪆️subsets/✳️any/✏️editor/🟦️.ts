/** ✏️ Layout editor — subset-level typed twin. Re-exports every window's typed view-model binding so
 * a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_layout_app()` stitching every window/mode module together. */

export const LAYOUT_EDITOR_DIALECT = { artifactKind: "s.layout.layout", standard: "1", subset: "*" } as const;

export const LAYOUT_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): the Preview window's `LayoutPreviewViewModel` imports the
// Blueprint window's `LayoutCameraViewModel`, and namespacing keeps every window's re-export surface
// independently addressable as this ticket's other W2 packets already established.
export * as blueprintWindow from "./🎭️modes/✏️edit/🪟️windows/📐️blueprint/🟦️";
export * as previewWindow from "./🎭️modes/✏️edit/🪟️windows/👁️preview/🟦️";
