# Generation3d App-Action Census Triage

## Finding

The sole reported React engine-contract failure at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:11863-11875` is a stale descriptor assertion, not a Generation3d action removal or a runtime dispatch failure.

The test expects `setActiveExample` on all five Generation3d editor `windowKinds`. The current committed procedural descriptor has five editor windows, declares `setActiveExample` once in app `actions`, and declares it on zero window kinds. The same current shape applies to `addWidget` and `undo`.

## Authority trace

- `✏️s/🔌️plugins/🌀️procedural/🔣️.json:10257-10280` registers the Generation3d editor; its app roster declares `setActiveExample` at `:17820-17824`.
- The real editor still registers the command at `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:61-105`, parses it at `:1960-1965`, and defines it in `🎮️commands/🎨️set-active-example/🦀️.rs:35-70`.
- The source ownership list leaves app-wide actions such as `setActiveExample` out of `window_kind_action_refs` (`✏️editor/🦀️.rs:2491-2546`). Its nearby comment still says the older builder copied such actions; that prose is stale.
- The current shared manifest resolver makes every unclaimed app action available to each window **without serializing a copy**: `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3882-3902`. Its unit law explicitly verifies the roster is never copied into any window (`…/🧪️tests/🔬️app-label/🦀️.rs:452-470`).
- Runtime registries use that resolver (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12828-12835`). The React admission predicate also accepts an app-level action (`🧱️elements/🛠️ShellHelpers/🟦️.tsx:348-372`), and the navbar picker calls it with `appActions` (`:407-421`).

## Exact replacement law

Replace the assertion of serialized all-window copies with a descriptor-plus-resolution law:

1. `setActiveExample`, `addWidget`, and `undo` each appear exactly once in `s.procedural.generation3d@1/*#editor.actions`.
2. They appear in no individual `windowKinds[*].actions` entry.
3. For each of all five editor windows, `window_kind_actions(app, window)` resolves each unclaimed app action.
4. React `appSwitchesExamples(app.id, app.windowKinds, app.actions)` is true, and `undeclaredActionDiagnostic(..., "setActiveExample", ..., app.actions)` is null.

This preserves the real behavior the stale check sought—navbar switching and window action reachability—while protecting the descriptor de-duplication invariant. No production change is warranted. No test or build was run for this read-only triage.
