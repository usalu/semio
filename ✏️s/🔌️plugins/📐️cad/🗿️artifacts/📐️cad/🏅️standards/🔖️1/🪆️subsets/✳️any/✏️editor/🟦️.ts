/** ✏️ CAD editor — subset-level typed twin. Re-exports every window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_cad_app()` stitching every window/mode module together. */

export const CAD_EDITOR_DIALECT = { artifactKind: "s.cad.cad", standard: "1", subset: "*" } as const;

export const CAD_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): all four windows independently export a same-named
// `CadDislocateOptions` interface, and a blanket `export *` from more than one of them would be an
// ambiguous re-export.
export * as shapeWindow from "./🎭️modes/✏️edit/🪟️windows/📐️shape/🟦️component";
export * as buildingWindow from "./🎭️modes/✏️edit/🪟️windows/🏢️building/🟦️component";
export * as energyWindow from "./🎭️modes/✏️edit/🪟️windows/🔥️energy/🟦️component";
export * as structureClassicWindow from "./🎭️modes/✏️edit/🪟️windows/🏛️structure-classic/🟦️component";
