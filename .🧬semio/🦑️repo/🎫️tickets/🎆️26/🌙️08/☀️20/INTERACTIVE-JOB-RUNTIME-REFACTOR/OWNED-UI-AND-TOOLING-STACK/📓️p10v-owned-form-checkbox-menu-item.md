# Phase 10 Owned Form, Checkbox, and Menu Item

<!-- #region Outcome -->

## Outcome

Implemented the framework-owned UI primitive foundation required by the remaining raw-DOM migrations:

- `Form` is a ref-forwarding native form boundary and does not intercept submit, validation, or Enter-key behavior.
- `Checkbox` accepts `true`, `false`, and `"indeterminate"`; synchronizes the native `checked` and `indeterminate` properties; emits `aria-checked="true"`, `"false"`, or `"mixed"`; forwards its input ref and native change event; and retains native name/value form participation.
- `MenuItem` is a ref-forwarding native button row with default `type="button"`, `role="menuitem"`, disabled semantics, and the existing shared menu-row presentation without a `ButtonGroup` wrapper.
- Added focused Storybook matrices for native form submission, all checkbox states with enabled/disabled variants, and ready/active/selected/disabled menu rows.
- Added public barrel exports for all three components and their public prop/state contracts.
- Moved the shared floating-menu row class identity onto `menuItemClassName`; `floatingMenuItemClass` remains its public alias, avoiding divergent repeated presentation strings.
- Deleted only the proven stale CAD allowlist row. The two live demonstrator allowlist rows remain.
- Removed the stale direct `clsx` barrel re-export after the concurrently validated owned class-composition packet removed its manifest identity; the owned `cn` export is unchanged.

<!-- #endregion Outcome -->

<!-- #region Validation -->

## Validation

### Focused runtime matrices

```text
bunx vitest run --config 🧪️vitest.config.ts \
  ../../../../🧱️elements/☑️Checkbox/🧪️component.test.tsx \
  ../../../../🧱️elements/📋️MenuItem/🧪️component.test.tsx \
  ../../../../🧱️elements/🧾️Form/🧪️component.test.tsx

Test Files  3 passed (3)
Tests       7 passed (7)
```

The matrix covers all three checkbox states against DOM state, ARIA, and `FormData`; ref and native change forwarding; enabled/disabled menu-item semantics and activation; and native form ownership, uncancelled Enter, and `requestSubmit` delivery.

### TypeScript

```text
bun ./📜️script.ts typecheck
exit 0
```

The package typecheck passed. A second exact `tsc --noEmit` invocation over all nine new component, test, and story files with the workspace React/DOM/Vite/Node types also exited `0`.

### Static primitive policy

```text
bun ./📜️script.ts check-ui-primitives
framework/ui/js/react: found 12 UI primitive violation(s)
```

This is the expected intermediate result before packets 2–4 migrate their twelve live application call sites. The prior thirteenth stale-allowlist finding is absent, and no allowlist row was added.

### Held gates

The broad UI lint/build/test and repo-wide gates were intentionally not run while the serialized Puzzle/Cargo lane was active. An exact ESLint attempt against sibling element paths was ignored by the package-scoped ESLint base path and is not counted as a validation result.

<!-- #endregion Validation -->

<!-- #region Files -->

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/🧪️story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/🧪️story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🧪️story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts`

<!-- #endregion Files -->
