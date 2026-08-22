# Phase 10 UI Primitive Remediation Packets

<!-- #region Evidence -->

## Evidence

Reproduced only the static gate from `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react`:

```text
bun ./📜️script.ts check-ui-primitives
```

It exited `1` after 1.02 s with exactly 13 findings: 12 live raw-DOM primitives and one stale allowlist entry. There were no raw SVG or `component:` escape-hatch findings.

<!-- #endregion Evidence -->

<!-- #region Findings -->

## Findings

| File | Lines | Primitive | Owned replacement |
| --- | --- | --- | --- |
| `compose/client/ui/3dm/ui/js/index.tsx` | 283 | tree action `<button>` | `Button` with the already-imported `AddIcon` |
| `compose/client/ui/3dm/ui/js/index.tsx` | 366 | URL `<input>` | `Input` |
| `compose/client/ui/3dm/ui/js/index.tsx` | 374 | import `<button>` | `Button` with an explicit import/add icon |
| `compose/client/lib/sketchpad/js/boot.tsx` | 145 | feedback `<form>` | new owned `Form` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx` | 4022 | indeterminate master `<input type="checkbox">` | new owned tri-state `Checkbox` |
| same CAD renderer | 6158 | REPL suggestion `<button>` | new owned `MenuItem` |
| same CAD renderer | 6284, 6319, 6345, 6383 | four filter `<input type="checkbox">` controls | new owned `Checkbox` |
| same CAD renderer | 6545 | boolean field `<input type="checkbox">` | new owned `Checkbox` |
| `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️component.tsx` | 206 | token `<form>` | new owned `Form` |

The thirteenth finding is not a live primitive. Delete the stale CAD row at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts:68`; its target has zero current hits.

`Input` is not a valid checkbox replacement: it does not expose a consumer ref for the indeterminate master control and its text-field wrapper/styling is not checkbox semantics. `TreeCheckbox` is likewise insufficient because it is tree-specific and lacks indeterminate state. `Button` cannot preserve the interactive floating-menu row's menu-item semantics, so that case needs a dedicated owned `MenuItem` rather than a styled button-group control.

<!-- #endregion Findings -->

<!-- #region Packets -->

## File-Disjoint Implementation Packets

1. **Foundation (dependency for packets 2–4):** add public `Form`, `Checkbox`, and `MenuItem` elements plus focused matrices and barrel exports. `Checkbox` must support `true`, `false`, and `"indeterminate"`, keep native form participation, expose the correct `aria-checked` state, and own the indeterminate DOM property. `Form` must retain native submit and Enter-key behavior. `MenuItem` must own a button-like menu row without inheriting `ButtonGroup` layout. These files are framework-only.
2. **Compose 3DM:** change only `compose/client/ui/3dm/ui/js/index.tsx`; use the existing UI import boundary for `Button` and `Input`, retain the current handlers, disabled state, and visible styles.
3. **Sketchpad + Hub forms:** change only `compose/client/lib/sketchpad/js/boot.tsx` and `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️component.tsx`; migrate both to `Form`. The files are independent and can split into two agents after foundation exports land.
4. **CAD renderer:** change only `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx`; replace the five regular and one indeterminate checkbox usages with `Checkbox`, and the suggestion row with `MenuItem`. Preserve each existing label wrapping, `checked` callback, selection-prune side effect, menu key, and focus behavior.
5. **Policy cleanup:** delete only the stale allowlist row in `📜️script.ts:68`; do not add new allowlist entries.

Packets 2–5 do not overlap each other; packet 1 must land first because it owns the only new shared files and barrel exports. Run this same static gate after all five packets; no Cargo or full verification is required for this remediation gate.

<!-- #endregion Packets -->
