# Phase 10 Owned Hotkey Listener

<!-- #region Outcome -->

## Outcome

Replaced `react-hotkeys-hook` at the framework UI boundary with one owned React listener:

- comma-separated chords compile once per key/platform change;
- `mod` resolves to Meta on Apple platforms and Control elsewhere;
- Control, Meta, Alt, Shift, key names, and common key aliases match strictly;
- input, textarea, select, and editable targets remain protected unless `enableOnFormTags` is set;
- disabled or empty bindings attach no listener;
- `preventDefault` runs only after a chord matches;
- callbacks stay current through an owned ref and every effect removes its window listener deterministically.

The UI barrel now imports and exports the owned hook, and the UI package plus lockfile contain no `react-hotkeys-hook` row.

<!-- #endregion Outcome -->

<!-- #region Validation -->

## Validation

| Gate | Outcome |
| --- | --- |
| `bun install` | PASS; 2,011 installs across 2,063 packages, lockfile saved |
| `bun nx run @semio-tech/ui-react:test-quick -- -t 'owned hotkeys'` | PASS; 5 focused tests |
| `bun nx run @semio-tech/ui-react:test-quick` | PASS; 2 files, 543 tests |
| `bun nx run @semio-tech/ui-react:typecheck` | PASS |
| `bun nx run @semio-tech/ui-react:lint` | PASS; zero ESLint warnings |
| `bun ./📜️script.ts verify dependencies` | PASS; 163 current identities from the 238 baseline, 75 removed, no additions |
| `bun ./📜️script.ts verify dependencies parity js` | PASS; 83 manifests, 286 external rows, 137 evidenced, 149 unowned, 0 undeclared imports |
| focused UI source/manifest/lockfile census | PASS; no live `react-hotkeys-hook` reference |

The focused runtime tests cover Apple/non-Apple `mod`, explicit Control and Meta alternatives, comma parsing, matched-only default prevention, protected and opted-in form fields, disabled listeners, and post-unmount cleanup.

<!-- #endregion Validation -->

<!-- #region Scope -->

## Scope

The committed dependency-freeze baseline intentionally retains the removed identity for ratchet comparison. A historical renderer Rust comment also names the former library while documenting a behavior shared by both renderer implementations; neither is a live JS dependency or import.

No Cargo or build command was run by this packet.

<!-- #endregion Scope -->

<!-- #region Files -->

## Files

- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/⌨️control-keybinding-context/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/⌨️control-keybinding-context/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `bun.lock`

<!-- #endregion Files -->
