/** 👁️ FEM 3D viewer — subset-level typed twin. Read-only counterpart of the editor surface's own
 * `🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports
 * (no command payload types, no config schema beyond the framework's own empty config). */

export const FEM3D_VIEWER_DIALECT = { artifactKind: "s.fem.fem3d", standard: "1", subset: "*" } as const;

export const FEM3D_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🧱️model/🟦️component";
