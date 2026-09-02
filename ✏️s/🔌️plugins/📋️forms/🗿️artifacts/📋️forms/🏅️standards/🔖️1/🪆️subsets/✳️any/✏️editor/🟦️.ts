/** ✏️ Forms editor — subset-level typed twin. Re-exports every window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_forms_app()` stitching every window/mode module together. */

export const FORMS_EDITOR_DIALECT = { artifactKind: "s.forms.forms", standard: "1", subset: "*" } as const;

export const FORMS_PLAY_MODE_BLUEPRINT = "blueprint" as const;

// 🪟️ Namespaced (not `export *`): keeps each window's typed view-model under its own name so a
// consumer importing both never risks a future name collision.
export * as builderWindow from "./🎭️modes/📝️blueprint/🪟️windows/🧱️builder/🟦️";
export * as tryWindow from "./🎭️modes/📝️blueprint/🪟️windows/▶️try/🟦️";
