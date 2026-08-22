# Phase 10 CAD UI Primitives

<!-- #region Outcome -->

## Outcome

Migrated all seven remaining CAD static-policy findings:

- The shared master toggle now renders the owned Checkbox. The all, none, and partial states map to true, false, and indeterminate through the tested spatialToggleCheckboxState boundary, removing the renderer's manual ref/effect DOM synchronization.
- Primitive show/filter and typology show/selection leaves use the owned Checkbox while retaining their label wrapping, controlled values, toggle-map updates, selection-menu clearing, hover clearing, and primitive/typology prune side effects.
- The boolean attribute editor uses the owned Checkbox while retaining its controlled value and field update.
- REPL suggestions use the owned MenuItem, retaining their stable compound key, click handler, content/detail layout, and the same shared floating-menu presentation. The menu host now exposes role=menu and its existing localized suggestions label.

The CAD renderer now has zero raw button or input elements.

<!-- #endregion Outcome -->

<!-- #region Validation -->

## Validation

### Focused CAD Renderer Matrix

    bunx vitest run --config 🧪️vitest.config.ts \
      ../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx \
      -t "spatialToggleGroupState reports all, none, and partial chrome groups"

    Test Files  1 passed (1)
    Tests       1 passed | 68 skipped (69)

The focused matrix asserts all three aggregate-to-checkbox mappings in addition to the existing group-state/fill behavior.

An initial filtered package collection also compiled and passed this renderer test but reached an unrelated existing defaultModelDefinitionIdCache initialization failure in the artifact suite. Isolating the renderer source produced the clean result above.

### Owned DOM Matrices

    bunx vitest run --config 🧪️vitest.config.ts \
      ../../../../🧱️elements/☑️Checkbox/🧪️component.test.tsx \
      ../../../../🧱️elements/📋️MenuItem/🧪️component.test.tsx

    Test Files  2 passed (2)
    Tests       6 passed (6)

These matrices cover controlled true/false/indeterminate DOM and ARIA state, native form participation, ref/change forwarding, and enabled/disabled menu-item semantics.

### TypeScript

The CAD package bunx tsc --noEmit -p tsconfig.json was attempted and remains non-green across existing generated/action/artifact/world-renderer inconsistencies. Filtering its diagnostics to the CAD renderer confirms no diagnostic at any changed import, state mapping, checkbox, menu, handler, or focused-test line. The pre-existing renderer diagnostics begin at unrelated lines such as the missing WINDOW_SEARCH_USER barrel member and legacy model/Three/UI contracts.

### Exact Static Primitive Audit

    bun ./📜️script.ts check-ui-primitives
    framework/ui/js/react: no UI primitive violations (2 allowlisted files)

This is the requested zero-finding result. The remaining two allowlisted demonstrator files both still have live hits; no allowlist entry was added.

Broad lint/build/test and repo-wide gates remained held as requested.

<!-- #endregion Validation -->

<!-- #region Files -->

## Files

- ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/OWNED-UI-AND-TOOLING-STACK/📓️p10y-cad-ui-primitives.md

<!-- #endregion Files -->
