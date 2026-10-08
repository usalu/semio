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

## Wave 2

Changes in `📜️script.ts` (rule region only):
- Ruling "Ephemeral roots" (design.md): R13 and R14 now skip files with a `👥️presence` or `🫧️transient` directory anywhere in their path (`POLICY_DIFF_ONLY_EPHEMERAL_LANES`, documented in the constant and in the `policyDiffOnlyFileBreaches` docstring). Config, document and every other lane keep both rules. R8–R12, R15, R16 are unchanged in those lanes.
- Deleted paths: the inventory already drops them (`policyMutationLawInventory` keeps only paths where `lstatSync(...).isFile()`), so git-listed-but-deleted files are never read. The `↩️restore-n` hits in `🧰️framework/…/🏪️store/🧪️testing/🧬️mutations/*/🧬️mutations/↩️restore-n/🦀️.rs` seen in wave 1 were real: those files exist on disk (re-checked, mtime 02:30) and still return a restore variant. No code change needed.

Self-tests (`🧪️tests/🧪️diff-only-law-gate`, 28 pass, 0 fail, run from a non-root cwd as before): new fixture cases `ephemeral-presence-snapshot-inverse` (whole-state diff impl, `type Diff = <root>` and a `Snapshot` inverse in a `👥️presence` lane: admitted), `ephemeral-transient-nested-leaf` (exemption at depth, `Restore` inverse under `🫧️transient/🧬️schema/🧬️mutations/…/↩️inverse`: admitted), `config-lane-snapshot-inverse` (the same body under `🎚️config`: R13 at lines 1 and 5, R14 at line 7). The two older R13/R14 cases that used presence/transient paths were moved to `🎚️config`.

Gate run 3: `bun ./📜️script.ts verify mutation-outcome-law` foreground, 2:01 wall, exit 1, 149 breaches (all diff-only; the 12 outcome-code breaches are gone). Log: `T/🗑️generated/coord/gate-run-3.log`.

| Rule | wave 1 | wave 2 |
|---|---|---|
| R8 | 80 | 9 |
| R9 | 121 | 0 |
| R10 | 182 | 7 |
| R11 | 475 | 0 |
| R12 | 841 | 0 |
| R13 | 126 | 0 |
| R14 | 309 | 23 |
| R15 | 1905 | 109 |
| R16 | 101 | 1 |
| total | 4140 | 149 |

| plugin | R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 | R16 | total |
|---|---|---|---|---|---|---|---|---|---|---|
| 📕️norm | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 73 | 0 | 73 |
| 📸️remodel | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 36 | 0 | 39 |
| 🗄️stdio | 9 | 0 | 7 | 0 | 0 | 0 | 2 | 0 | 1 | 19 |
| 🧰️framework | 0 | 0 | 0 | 0 | 0 | 0 | 13 | 0 | 0 | 13 |
| 🧩️puzzle | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 3 |
| 🗒️note | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 |
| total | 9 | 0 | 7 | 0 | 0 | 0 | 23 | 109 | 1 | 149 |

## Wave 3 — blind spots G-1…G-10 closed

Engine rewrite of `policyDiffOnlyFileBreaches` (same region of `📜️script.ts`, no cargo). Scope model:
- **Every fn is scanned** (R8–R12, R14) in non-test files under `🧬️mutations`, `🧬️mutation-support`, and any `🔺️diff`/`↩️inverse` directory of a `🧬️schema` (G-1, G-3, G-9). Files elsewhere that hold an `impl … Mutation<`/`MutationKind<`/`MutationDiff<`/`DiffAlgebra<` are scanned in the fns inside those impls; plugin files under `🗿️artifacts/` (artifact roots) are scanned in fns named like a builder (`*diff*`, `*inverse*`, `negative`, `*_replacing`, `state_after`) (G-3, G-10).
- **Diff-type bodies are scanned** (G-2): `DiffAlgebra::inverse`, `MutationDiff::inverse` and every other diff-type fn. Exempt only: the diff type's `apply`/`absorb`/`retire_*`/`is_empty`, fns named `*between*` (sync), fns of non-mutation trait impls (codecs, `Default`, …), fns outside `🧬️mutations` that receive an `ApplyCapability`, and `*apply*` helpers in a file that implements a diff type outside `🧬️mutations`.
- **R9 (G-4)** forbids `apply_diff(`, `ApplyCapability` (outside `use`) and `.apply(`/`::apply(` in the whole file under any `🧬️schema`/`🧬️mutations`/`🧬️mutation-support` directory, whatever the fn is called, except the exemptions above.
- **R10 (G-5)** catches `*between*` (incl. `keyed_between::<T>(`, `value_diff_between(`, `between_indexed(`), `*_replacing(`, `*_state_diff(`, `state_after*(`, `value_diff*(`; `fn` definitions and bodies of `*between*`-named fns are not flagged.
- **R11 (G-8)** additionally catches `negative(`, `.negat…(`, `state_after*(`, `inverse_of(`, `value_diff*(` in every inverse-ish fn (name contains `inverse`/`negative`/`negation`); `diff(`/`*_inverse(…diff…)` stay mutation-side only because a diff type's own inverse legitimately reads its rows.
- **R12 (G-7)**: `let mut x = <ref param>….clone()/.to_owned()/.to_vec()/.iter().cloned().collect…` under any binding in any scanned fn; roots are the fn's `&T` parameters except payload-like names (`payload`, `mutation`, `op`, `edit`, `self`) and diff-typed parameters (`…Diff|Delta|Patch|Mutation`).
- **R14 (G-6)**: names `ReplaceConfig`, `ReplaceState`, `SetDocument`, `X::*Snapshot` (e.g. `HostSnapshot`) and whole-record fields `*_state|*_content|*_snapshot|*_document:` in an inverse-ish fn.
- **R8**: any `&mut` in a builder-named fn (`diff`, `*inverse*`, `*diff*`, `negative`, …); in every other scanned fn only a `&mut` typed `…Snapshot`/`…Document`/`…State`. A first run with `&mut` over all fns produced 1432 R8 hits, mostly codecs/parsers/retained-prepare state machines, so those are excluded: directories `🚪️io`, `🎮️prepare`, `📦️codec` and any `*-internals` are infrastructure (not "every fn", no R9 whole-file, no name-based scanning; their `impl Mutation`/diff-type fns are still judged).
- R13/R14 keep the presence/transient exemption; R15/R16 unchanged.

Self-tests (`🧪️tests/🧪️diff-only-law-gate`, 41 pass, 0 fail; run from a non-root cwd): new negative+compliant fixtures per pattern — `g1-helper-fns`, `g2-diff-type-inverse`, `g3-artifact-root-helper`, `g4-schema-helper-apply`, `g4-schema-operations-apply`, `g5-renamed-between`, `g6-whole-record-restore`, `g7-copy-bindings`, `g8-renamed-inverse`, and compliant controls `compliant-infrastructure`, `compliant-diff-type-inverse`, `compliant-engine-internals`, `compliant-engine-method`. Existing cases adjusted: `compliant-leaf` inverse no longer calls `between`; `r14-restore-inverse` now expects the inverse-named helper to fire.

Gate run 4: `bun ./📜️script.ts verify mutation-outcome-law`, foreground, 3:00 wall, exit 1, 449 breaches = 437 diff-only + 12 outcome-code (`mutation-migration/message-code`, back from a peer edit, not from these rules). Log `T/🗑️generated/coord/gate-run-4.log`; tally `T/🗑️generated/coord/tally-run-4.md`.

| Rule | wave 2 | wave 3 |
|---|---|---|
| R8 | 9 | 142 |
| R9 | 0 | 88 |
| R10 | 7 | 68 |
| R11 | 0 | 5 |
| R12 | 0 | 119 |
| R13 | 0 | 0 |
| R14 | 23 | 15 |
| R15 | 109 | 0 |
| R16 | 1 | 0 |
| total | 149 | 437 |

(R15/R16 drops are peer progress between runs, not rule changes.)

Per plugin (R8 R9 R10 R11 R12 R13 R14 R15 R16 = total):

| plugin | R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 | R16 | total |
|---|---|---|---|---|---|---|---|---|---|---|
| 🗄️stdio | 98 | 36 | 65 | 1 | 91 | 0 | 2 | 0 | 0 | 293 |
| 🧰️framework | 13 | 3 | 0 | 4 | 5 | 0 | 0 | 0 | 0 | 25 |
| 📕️norm | 0 | 15 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 16 |
| 🀄️wfc | 8 | 0 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 12 |
| 🧩️puzzle | 3 | 3 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 9 |
| 📋️forms | 1 | 2 | 0 | 0 | 3 | 0 | 1 | 0 | 0 | 7 |
| 🖍️draw | 3 | 3 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 7 |
| 🖨️raster | 4 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 |
| 🔋️energy | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 |
| 💠️lowpoly | 0 | 0 | 0 | 0 | 4 | 0 | 2 | 0 | 0 | 6 |
| 🌀️procedural | 3 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 |
| 📏️layout | 0 | 1 | 3 | 0 | 0 | 0 | 1 | 0 | 0 | 5 |
| 📖️playbook | 1 | 2 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 4 |
| 🔱️trinity | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 |
| 🧱️block | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 |
| 🏛️architect | 0 | 2 | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 4 |
| 🌍️gis | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 |
| 🪵️sourcing | 0 | 1 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 3 |
| ➗️mathematical | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 3 |
| 🏗️fem | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| ✒️writer | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| 🌿️vcs | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| 🎞️animate | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 2 |
| 🏭️process | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 2 |
| 🪐️space | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 2 |
| 🎪️demonstrator | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| 📜️imperative | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 |
| 🕸️dag | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 |
| total | 142 | 88 | 68 | 5 | 119 | 0 | 15 | 0 | 0 | 437 |

Residual precision limits (no allowlists): R9 flags row-level `.apply(` of unrelated patch types under `🧬️schema` (e.g. `ItemPatch`) and `apply_*_mutation(snapshot: &mut …)` hand-written appliers (the latter are true positives); R10 flags snapshot-differencing helpers not named `*between*` (`diff_document`, `page_changes`, `diff_stream`): rename them `*between*` if they are sync-only, or delete; R12 flags any non-payload `&T` parameter copied into a `let mut`, including legitimate scratch copies of non-base data. stdio dominates (293): codec-adjacent schema modules plus the gltf/pdf/json diff helpers.

## Wave 3b — ephemeral exemption by lane type

R13/R14 now also exempt impl blocks of an ephemeral lane TYPE wherever the file lives (`POLICY_DIFF_ONLY_EPHEMERAL_TYPE_RE = /(?:Transient|Presence)(?:Mutation)?$/` against the implementing type and the trait's type arguments): `impl MutationDiff<X> for X` (R13), `type Diff = …` of a `Mutation` impl (R13) and the `inverse` bodies inside such an impl (R14). So a `Puzzle{2,3,5}dWindowTransientMutation` / `…PresenceMutation` aggregate in a `🪟️window/🦀️.rs` file is admitted; `*WindowConfig*`/document types never are. The directory exemption (`👥️presence`, `🫧️transient`) is unchanged.

Self-tests (44 pass, 0 fail): `ephemeral-window-transient-type` and `ephemeral-window-presence-type` (admitted: whole-state diff impl, `type Diff = <root>`, `Snapshot` inverse), `window-config-type-still-fires` (same body with `Puzzle5dWindowConfig`: R13 lines 1 and 5, R14 line 7). The older R13/R14 fixtures that used `Presence`/`Transient` type names were renamed to `Settings…` so they keep testing the persisted-lane behaviour.

Gate run 5: `bun ./📜️script.ts verify mutation-outcome-law` foreground, 4:05 wall, exit 1, 70 breaches (no outcome-code breaches this time). Log `T/🗑️generated/coord/gate-run-5.log`, tally `tally-run-5.md`.

| Rule | run 4 | run 5 |
|---|---|---|
| R8 | 142 | 33 |
| R9 | 88 | 19 |
| R10 | 68 | 9 |
| R11 | 5 | 0 |
| R12 | 119 | 5 |
| R13 | 0 | 0 |
| R14 | 15 | 4 |
| R15 | 0 | 0 |
| R16 | 0 | 0 |
| total | 437 | 70 |

(Most of the drop is peer progress between runs; R14 15 → 4 includes the lane-type exemption.)

Per plugin (R8 R9 R10 R11 R12 R13 R14 R15 R16 = total):

| plugin | R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 | R16 | total |
|---|---|---|---|---|---|---|---|---|---|---|
| 🗄️stdio | 20 | 2 | 6 | 0 | 2 | 0 | 0 | 0 | 0 | 30 |
| 🀄️wfc | 8 | 0 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 12 |
| 📕️norm | 0 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 |
| 🔋️energy | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 |
| 📏️layout | 0 | 1 | 3 | 0 | 0 | 0 | 1 | 0 | 0 | 5 |
| 🧱️block | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 |
| 🏛️architect | 0 | 2 | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 4 |
| 🏗️fem | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| total | 33 | 19 | 9 | 0 | 5 | 0 | 4 | 0 | 0 | 70 |
