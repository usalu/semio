/** ✏️ FEM 3D editor — subset-level typed twin. Re-exports both windows' typed view-model bindings so
 * a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_fem3d_app()` stitching every window/mode module together. */

export const FEM3D_EDITOR_DIALECT = { artifactKind: "s.fem.fem3d", standard: "1", subset: "*" } as const;

export const FEM3D_EDIT_MODE_EDIT = "edit" as const;

export * as modelWindow from "./🎭️modes/✏️edit/🪟️windows/🧱️model/🟦️";
export * as resultsWindow from "./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️";
