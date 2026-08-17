/** ✏️ Fem2d editor — subset-level typed twin. Re-exports every window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️component.rs`'s `create_fem2d_app()` stitching every window/mode module together. */

export const FEM2D_EDITOR_DIALECT = { artifactKind: "s.fem.fem2d", standard: "1", subset: "*" } as const;

export const FEM2D_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Blanket `export *` is safe here: the model and results windows export distinct names
// (`Fem2dModelViewModel`/`FEM2D_MODEL_*` vs `Fem2dResultsViewModel`/`FEM2D_RESULTS_*`), unlike cad's
// four windows which share a `CadDislocateOptions` name and need namespacing.
export * from "./🎭️modes/✏️edit/🪟️windows/🧱️model/🟦️component";
export * from "./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️component";
