# 🎨️ terra / ts-react — verification report

## Scope
Owned paths only, per brief:
1. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/**`
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🧱️elements/**`

No edits were made in this pass — verification found the suite already at the recorded, non-regressed baseline (see below), so there was no real breakage to repair.

## Command run

```
cd "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react"
bun ./📜️script.ts test --reporter=verbose
```

(this is the exact consumer command — `bun ./📜️script.ts test` per `📋️project.json`'s `test` target, which wraps `runVitest` → `node_modules/vitest/vitest.mjs run --config 🧪️vitest.config.ts --passWithNoTests …`)

Full output saved to `terra-ts-react-vitest-run1.txt` (676 lines) in this ticket folder.

**Result: exit code 1.** Tail of output:
```
 Test Files  1 failed (1)
      Tests  11 failed | 325 passed (336)
     Errors  1 error
```

Confirmed by reading the summary line directly (not through a `| tail` pipe — rule 10/`cmd | tail; echo $?` trap avoided).

## `🧪️vitest.config.ts` — no filename-array hazard here (rule 13/18 check)

Read in full. `test.coverage.include` names `index.tsx` (coverage scope, not test collection) but there is no `include`/`includeSource` override at all — vitest's default glob applies, and there is exactly one test file (`🧪️index.test.ts`) in this package, so the "silently-orphaned file" and "double-counted via include+includeSource" traps from rules 13/18 do not apply. Not a config this packet needed to touch.

## Baseline comparison — BY NAME, not by count (rule 11)

Reference: the 15-name pre-existing baseline recorded in this ticket's `📓️terra-H1-vitest-final.txt`, reconfirmed in `📓️status.md` as "325/336, 11 remaining = exact subset of 15-name baseline."

Current 11 failures, all present verbatim in the 15-name baseline:
1. `declarative forms parity > renders selectable builder cards with selection ring`
2. `framework renderer hosts > interprets virtual file system component scenes`
3. `s workflow flow routing > isolates render faults in ShellFaultBoundary`
4. `window action panel — staging and single dispatch (P1/P2) > stages both args locally, dispatches nothing until Execute, then fires exactly one merged descriptor and keeps staged values`
5. `window action panel — staging and single dispatch (P1/P2) > gates Execute on required args, but a default-satisfied required arg counts without staging`
6. `window action panel — staging and single dispatch (P1/P2) > Reset restores defaults while keeping the form expanded`
7. `registry-derived utilities and activation (P5) > resolveWindowActions surfaces only panel-eligible definitions owned by the window`
8. `resolveCommands / commandCategories (footer command panel registry) > commandCategories orders and dedupes categories by first appearance`
9. `shell option locks (SEMIO_LOCKED_*) > ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND introduction is app-specific only after the general landing tour was split out`
10. `shell option locks (SEMIO_LOCKED_*) > mit-bestand/demonstrator footer credits render the funding/partner logos, links, and locale text`
11. `buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel) > buildCommandCategoryTabs builds one namespaced PanelTabLeaf per category, whose lazily-resolved tree only contains that category's commands`

The 4 baseline names NOT in the current failing set (i.e. now passing, a strict improvement over the 15-name baseline, already recorded by a prior H1 packet — not new work this pass):
- `auto-expands a singleton arg-carrying category into a flat form with section actions and no disclosure list`
- `an arg-carrying command row toggles expansion instead of executing, and a synthetic arg-form section only appears while expanded`
- `Execute is disabled until the required arg is staged, and calling it passes the effective (staged) args; Reset dispatches onResetArgs`
- `FrameworkOsShell portal layer is unconstrained by z-tutorial so portaled elements sit above elevated windows`

**Verdict: exact subset, zero regression.** Set arithmetic, not counts: 15 − 4 = 11, and the 11 are literally the same strings, confirmed by grepping the verbose reporter's `×` lines and the `FAIL` block test-name lines against each other for internal consistency, then against `📓️terra-H1-vitest-final.txt`.

## Pooled-actor runtime wiring sanity check

Brief called out `🧱️elements/PluginRuntime/🟦️component.tsx` → `ActivationRegistry` + `ShardClient` → `/plugin-modules/_shard/🟨️shard-worker.js` as the thing this renderer is wired to. Located it at
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx` (NOT under the top-level `🧱️elements/` — the react renderer keeps its own nested `🧱️elements/`, both of which are inside the owned path scope). Confirmed present and non-empty: `ActivationRegistry` (30 refs), `ShardClient` (50 refs), `shard-worker` (3 refs). No structural damage observed.

## Working-tree state in owned scope

`git status --porcelain` over both owned path roots shows only staged Rust changes (`🎠️runtime.rs`, `📦️glue.rs`, `Shell/🧊️component.rs`) from other sessions — none of them TypeScript/React, none of them touched by this pass. `Shell/🧊️component.rs` is explicitly registrar-shared per `📌️important.md`'s "Replace, never wrap" / registrar-only list note ("shared with live hover/selection tickets") — left untouched, as required.

## Conclusion

No repair needed: the ts-react suite is exactly at its documented, coordinator-verified baseline (325/336, 11-name exact subset of the 15-name pre-existing set). No regression. No edits made in owned paths this pass.
