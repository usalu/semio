# 📓️ exec-fw-gate — diff-only rules R8–R16 in `verify mutation-outcome-law`

Scope: root `📜️script.ts` region `//#region 🔧️PolicyRuleMutationOutcomeMergePolicy` only (plus the one-line test wiring).
Status: WRITTEN, SELF-TESTED, GATE RUN ONCE (breaches expected, no allowlists).

## Files touched

- `📜️script.ts`: nine new `export function policy…Breaches(repoRoot)` rules spread into `policyMutationOutcomeMergePolicyBreaches`, one pure per-file engine `policyDiffOnlyFileBreaches(relPath, content)` (R8–R14, R16), one pure tree rule `policyInverseSumLawUntestedLeafBreaches(files, read)` (R15). Stale `policyMutationTriadCompletenessBreaches` reference removed from the rule-1 docstring; "seven rules"/"7 rules" docs now say sixteen. `TestScript` route `test outcome-law-gate` now runs both law test files.
- `🧪️tests/🧪️diff-only-law-gate/🟦️.ts` (new) and `🧫️fixtures/🧫️diff-only-law-gate/🔣️.json` (new, schema-first vectors: 16 per-file cases, 1 leaf tree, rule→design-code table).
- No `launch.json`/seed/`project.json` change: the rules ride the existing `verify mutation-outcome-law` target and its `test-outcome-law-gate` dependency.

## Rules (summary format `file:line R<n>:<design-code> reason (\`token\`)`, kind `diff-only-mutation/<slug>`)

| Rule | slug | design code | What fires |
|---|---|---|---|
| R8 | leaf-mutable-base | V3-LEAF-APPLY | `&mut` in the signature or body of any `fn diff`/`fn inverse` of a leaf file |
| R9 | leaf-applies-diff | V3-LEAF-APPLY | `.apply(`, `::apply(`, `apply_diff(`, `ApplyCapability` (outside `use`) in `🔺️diff`/`↩️inverse`/`🦠️mutation` leaf files and inside `impl … MutationKind<`/`Mutation<` blocks; `impl … MutationDiff<`/`DiffAlgebra<` blocks exempt |
| R10 | leaf-between | V1-SNAPSHOT-DIFF | `between(` call (not `fn between`) in leaf files, outside diff-type impls |
| R11 | diff-derived-inverse | V2-DIFF-DERIVED-INVERSE | in an inverse body: `diff(`, `.diff(`, `*_inverse(…diff…/outcome)`, `inverse_from_diff(` |
| R12 | base-clone-diff | V1-SNAPSHOT-DIFF | in a diff body: `let mut x = <last param>….clone()/.to_owned();` (param name read from the signature) |
| R13 | whole-state-diff | V1-SNAPSHOT-DIFF | `impl MutationDiff<X> for X` in ANY non-test Rust file; `type Diff = Self/<P>/<implementing type>` in leaf `Mutation`/`MutationKind` impls |
| R14 | restore-inverse | V2-RESTORE-INVERSE | in an inverse body: `SetSnapshot`, `PatchSnapshot`, `ReplaceDocument`, `ReplaceSnapshot`, `Restore*`, `X::Snapshot` |
| R15 | inverse-sum-law-untested | V4-LAW-UNTESTED | leaf dir (below `🧬️mutations`, owns `🔺️diff`, no `🧩️plan`) whose OWN `🧪️tests/**/*.rs` never mentions `assert_mutation_inverse_sum_law` (shared `🧬️mutations/🧪️tests` does not count) |
| R16 | outcome-apply-to | V3-LEAF-APPLY | `.apply_to(` / `::apply_to(` in any Rust file, and `fn apply_to` inside `impl MutationOutcome<…>` |

Leaf file = non-test file under a `🧬️mutations` dir, or a file with an item-level `impl … MutationKind<`/`impl … Mutation<`. Comments, string/char/raw literals and `#[cfg(test)]`/`#[test]` items are blanked before any pattern runs. Files come from the memoized `policyMutationLawInventory` (git ls-files), never a walk; the per-file rules scan once per root (`policyDiffOnlyScan`).

Known textual-loophole precision limits (by design of the brief, no allowlist):
- R9 flags every `.apply(`/`::apply(` in scope, including domain operations that are not `MutationDiff::apply` (e.g. `payload.splice().apply(` in writer `✂️splice-text` diff/inverse, `TextSplice::apply`): rename the domain method.
- R16 flags every `.apply_to(`, including `ItemPatch::apply_to` (`🔱️trinity` rewriting diff, `🪵️sourcing` operations): rename or restructure.
- R11's helper clause needs `diff`/`outcome` in the arguments or `diff` in the callee name; an arbitrary local variable holding the forward diff passed to `x_inverse(base, o)` is not seen.

## Tests

Run (isolated cwd, because `bun test` with cwd = repo root segfaults in this environment — see issues):
`cd <scratchpad> && bun test "/Users/ueli/Documents/semio/🧪️tests/🧪️diff-only-law-gate/🟦️.ts"` → 25 pass, 0 fail, 77 expects.
Covers: every rule R8–R16 fires exactly on the planted lines (negatives), three compliant controls stay silent (sparse leaf with prose/strings/`cfg(test)` naming every token, non-leaf file, test file), design codes match the fixture table, the TypeScript scanner (`typescript` 5.9.3) re-derives R8/R9/R10/R16 lines independently, the real inventory-driven entry points report the same `path:line` sets over a scratch `git init` repository, and R15 runs both pure and through the inventory.

## Gate run

`bun ./📜️script.ts verify mutation-outcome-law` (foreground; wall 16:07 under load ~64, 107 s user CPU), exit 1 with 4152 breaches: 4140 diff-only + 12 pre-existing rule-2 message-code breaches. Raw output: `T/🗑️generated/fw-gate/gate.log`.

| Rule | breaches |
|---|---|
| R8 leaf-mutable-base | 80 |
| R9 leaf-applies-diff | 121 |
| R10 leaf-between | 182 |
| R11 diff-derived-inverse | 475 |
| R12 base-clone-diff | 841 |
| R13 whole-state-diff | 126 |
| R14 restore-inverse | 309 |
| R15 inverse-sum-law-untested | 1905 |
| R16 outcome-apply-to | 101 |
| total | 4140 |

### Per top-level dir under `✏️s/🔌️plugins/` (🧰️framework = framework files)

| plugin | R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 | R16 | total |
|---|---|---|---|---|---|---|---|---|---|---|
| 🗄️stdio | 14 | 119 | 182 | 453 | 49 | 1 | 226 | 144 | 53 | 1241 |
| 📕️norm | 34 | 0 | 0 | 0 | 349 | 1 | 0 | 554 | 0 | 938 |
| 🔋️energy | 0 | 0 | 0 | 0 | 293 | 0 | 0 | 297 | 2 | 592 |
| 🏛️architect | 0 | 0 | 0 | 0 | 3 | 6 | 1 | 266 | 1 | 277 |
| 🧩️puzzle | 3 | 0 | 0 | 10 | 10 | 18 | 12 | 113 | 0 | 166 |
| 🧱️block | 5 | 0 | 0 | 0 | 5 | 10 | 6 | 104 | 3 | 133 |
| 🀄️wfc | 0 | 0 | 0 | 0 | 7 | 7 | 4 | 73 | 0 | 91 |
| 🏗️fem | 2 | 0 | 0 | 4 | 1 | 5 | 4 | 60 | 6 | 82 |
| 🌀️procedural | 2 | 0 | 0 | 0 | 20 | 14 | 6 | 38 | 0 | 80 |
| 📸️remodel | 0 | 0 | 0 | 0 | 37 | 3 | 3 | 36 | 0 | 79 |
| 🗒️note | 15 | 0 | 0 | 0 | 0 | 4 | 2 | 33 | 1 | 55 |
| 🎥️shooting | 0 | 0 | 0 | 0 | 12 | 1 | 0 | 31 | 0 | 44 |
| ➗️mathematical | 0 | 0 | 0 | 0 | 16 | 0 | 0 | 18 | 2 | 36 |
| 📐️cad | 5 | 0 | 0 | 0 | 3 | 4 | 3 | 19 | 0 | 34 |
| 🏭️process | 0 | 0 | 0 | 0 | 13 | 3 | 1 | 15 | 2 | 34 |
| 💠️lowpoly | 0 | 0 | 0 | 3 | 1 | 3 | 2 | 21 | 0 | 30 |
| 🔱️trinity | 0 | 0 | 0 | 2 | 6 | 2 | 0 | 10 | 10 | 30 |
| 🧰️framework | 0 | 0 | 0 | 0 | 0 | 7 | 17 | 0 | 4 | 28 |
| 📋️forms | 0 | 0 | 0 | 0 | 2 | 1 | 1 | 13 | 5 | 22 |
| 🌍️gis | 0 | 0 | 0 | 0 | 0 | 2 | 1 | 14 | 4 | 21 |
| 🪐️space | 0 | 0 | 0 | 0 | 1 | 5 | 4 | 5 | 4 | 19 |
| 🖨️raster | 0 | 0 | 0 | 0 | 1 | 3 | 2 | 12 | 0 | 18 |
| ✒️writer | 0 | 2 | 0 | 0 | 5 | 1 | 0 | 5 | 1 | 14 |
| 🪵️sourcing | 0 | 0 | 0 | 0 | 1 | 5 | 3 | 3 | 1 | 13 |
| 🖍️draw | 0 | 0 | 0 | 0 | 0 | 4 | 3 | 5 | 0 | 12 |
| 🎞️animate | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 9 | 0 | 11 |
| 🌿️vcs | 0 | 0 | 0 | 0 | 0 | 3 | 1 | 6 | 1 | 11 |
| 🌎️hub | 0 | 0 | 0 | 0 | 1 | 3 | 2 | 0 | 0 | 6 |
| 📏️layout | 0 | 0 | 0 | 3 | 0 | 1 | 1 | 0 | 0 | 5 |
| 🌊️flow | 0 | 0 | 0 | 0 | 0 | 3 | 2 | 0 | 0 | 5 |
| 🎬️sequence | 0 | 0 | 0 | 0 | 0 | 3 | 2 | 0 | 0 | 5 |
| 📖️playbook | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 1 | 3 |
| 💡️reasoning | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 2 |
| 📜️imperative | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 2 |
| 🕸️dag | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| total | 80 | 121 | 182 | 475 | 841 | 126 | 309 | 1905 | 101 | 4140 |

Observations: 🏛️architect/🗒️note/🧩️puzzle/🧱️block etc. show R15 only because no leaf test calls the sum-law helper yet (it is pending from the framework executor); 🧰️framework R13/R14 hits are framework-side whole-state/Restore variants worth the framework executor's eye; 🌎️hub rows are non-plugin top-level dir.

## Open issues

- `bun test` with cwd = repo root segfaults deterministically in this environment (Bun 1.3.14, RSS 2 GB, "Segmentation fault at address 0x8033"), for the pre-existing `🧪️tests/🧪️outcome-law-gate` test as well; the same files pass when cwd is another directory. `test outcome-law-gate` (cwd = root) is therefore unverified through the script route; the route edit is a plain second path argument.
- The gate was not type-checked with `tsc` (script.ts is run by Bun); the code follows the existing `match.index`/`matchAll` conventions of the file.
- R15 is intentionally strict (leaf's own `🧪️tests`); plugins whose unit tests live in the shared `🧬️mutations/🧪️tests/🔬️unit` keep firing until each leaf gets its own sum-law test.
