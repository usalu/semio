/** ✏️ Equation editor — subset-level typed twin. Re-exports every window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest,
 * mirroring `🦀️.rs`'s `create_equation_app()` stitching every window/mode module
 * together. Namespaced (not `export *`): the graph and geometry windows each declare their own
 * `EquationCameraViewModel`-adjacent vocabulary and a blanket re-export risks a future
 * same-named-export collision as either window grows. */

export const EQUATION_EDITOR_DIALECT = { artifactKind: "s.mathematical.equation", standard: "1", subset: "*" } as const;

export const MATH_PLAY_MODE_EDIT = "edit" as const;

export * as geometryWindow from "./🎭️modes/✏️edit/🪟️windows/📐️geometry/🟦️";
export * as graphWindow from "./🎭️modes/✏️edit/🪟️windows/🕸️graph/🟦️";
