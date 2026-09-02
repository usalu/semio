/** 👁️ Forms viewer — subset-level typed twin. Re-exports the Try window's typed view-model binding
 * so a host-side TS consumer has one import surface for the whole viewer manifest, mirroring
 * `🦀️.rs`'s `create_forms_viewer()` stitching the mode/window together. */

export const FORMS_VIEWER_DIALECT = { artifactKind: "s.forms.forms", standard: "1", subset: "*" } as const;

export const FORMS_VIEW_MODE_VIEW = "view" as const;

export * as tryWindow from "./🎭️modes/👁️view/🪟️windows/▶️try/🟦️component";
