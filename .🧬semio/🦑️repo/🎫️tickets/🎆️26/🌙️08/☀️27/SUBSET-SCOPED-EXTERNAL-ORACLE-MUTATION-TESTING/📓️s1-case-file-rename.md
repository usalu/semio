# 📓 S1 — Test-case filename migration (component.X → emoji-only)

## Scope actually renamed

Candidate case directories: **168** — a directory qualifies only if it is a *direct* child of a
`🧪️tests` directory **and** directly contains a feature file (`component.feature` or the already-migrated
`🥒️.feature`). Verified zero candidates contain a `🧬️mutations` path segment (script `--list-cases` output,
saved during the run; none flagged in `contains_mutations_segment`).

This is one less than the 169 measured in the task brief: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/host-protocol-parity`
is a real, qualifying case dir but sits under the coordinator-owned test-platform module
(`🔨️modules/🧪️test/**`), which the hard rules forbid touching. It was **deliberately skipped** — its
`component.feature`, `🐹️component.go` and `🔷️component.cs` (the sole go/cs instances in the whole repo)
and its `🦀️component.rs`/`🟦️component.ts` were all left exactly as-is.

Files renamed, per kind (394 total):

| Old name | New name | Renamed |
| --- | --- | --- |
| `component.feature` | `🥒️.feature` | 167 |
| `🦀️component.rs` | `🦀️.rs` | 161 |
| `🐍️component.py` | `🐍️.py` | 60 |
| `🟦️component.ts` | `🟦️.ts` | 6 |
| `🐹️component.go` | `🐹️.go` | 0 (sole instance is the forbidden case above) |
| `🔷️component.cs` | `🔷️.cs` | 0 (sole instance is the forbidden case above) |

394 = 167+161+60+6. `git status --porcelain` shows 396 `R`/`RM` lines under `🧪️tests/`; the extra 2 are a
different agent's earlier, unrelated `🔣️component.json → 🔣️.json` renames under `🔨️modules/🧪️test/📇️registry`
and `.../🧬️schema`, already in the tree before this ticket started.

**Deliberately not renamed** (confirmed by inspection, not by assumption):
- `🔨️modules/🧪️test/🧪️tests/host-protocol-parity/*` — coordinator-owned module, hard rule.
- Any `🧪️oracle/🔣️.json` — owned by another agent, hard rule; never opened for writing.
- Directories that are direct children of some `🧪️tests` but have **no feature file**, e.g.
  `.../✨️derive/🧪️tests/📤️macro-exports/` (a Rust-macro-only fixture) and
  `.../🖼️bmp/.../✳️any/🧪️tests/🧬️mutation-regressions/` (a regression-only Rust dir). Neither is a BDD test
  case in this taxonomy, so neither was renamed and neither's references were rewritten.
- Any file under a `🧬️mutations` directory (physical mutation-vector scenario bundles, a different
  contract — `mutationVectorRegistryBreaches`). None of the 168 case dirs sit inside one.

## Reference rewrite

Approach: build the exact whitelist of the 168 renamed case-directory **basenames** (all unique — no
collisions), then only rewrite a `🧪️tests/<name>/<old-filename>` occurrence when `<name>` is in that
whitelist. This replaced an earlier, broken word-boundary/blacklist approach that over-matched (it
initially caught 410 files including physical `🧬️mutations` bundles and `🧪️oracle/🔣️.json` catalog files —
neither of which were renamed, so rewriting those references would have broken them). The whitelist
approach is exact: it can't touch a reference to a directory that wasn't actually renamed, regardless of
how the reference is expressed (relative `#[path=...]`, doc-comment `@see`, etc.).

Also excluded from the reference-rewrite file scan: `.🧬semio/**` (ticket-management data — other
sessions' historical, dated audit reports; rewriting those would revise history, not fix a stale
reference) and `.nx/**` (generated build cache, regenerated on its own).

**Result: 32 files, 34 lines changed.** Full list:
- 26 `🦀️component.rs` files (mostly `🧪️oracle/🦀️component.rs` Rust oracle sources and 3
  `🧬️schema/🧬️mutations/🦀️component.rs` mutation-kind definitions) carrying `#[path=...]` includes or
  `@see` doc-comments that point at a case dir that was renamed.
- 6 files that were *also* renamed and contain a self-referential path comment inside themselves
  (`mutate-xml-1-0/🥒️.feature`, `mutate-bmp-v3/🦀️.rs`, `mutate-png-1-2/🦀️.rs`,
  `mutate-procedural-2d-1/🐍️.py`, `mutate-procedural-3d-1/🐍️.py`, `mutate-forms-1/🐍️.py`) — these show as
  `RM` in git status (renamed + content-modified).

Spot-checked one: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧬️schema/🧬️mutations/🦀️component.rs`
lives *inside* a `🧬️mutations` bundle but its doc-comment references (via `../../../../../../`) the real,
separate case dir `🧪️tests/mutate-svg-1-1-basic/` two directories up — that case dir was renamed and does
exist, so the doc-comment fix is correct; the mutations bundle itself was untouched.

## `discoverTestCases` verification

```
bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts discover | tail -3
```
**Before: 4** (task-brief baseline, pre-rename). **After: `[discover] 168 test case(s)`** — matches the
168 qualifying case dirs exactly (169 minus the one forbidden case).

`find . -name "component.feature"` (excluding `node_modules`/`.git`): exactly **1** result, the forbidden
`host-protocol-parity/component.feature` — expected and correct, not a miss.

## Contract verification

```
bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts contract
```
completed in one run (exit 1 — breaches present, as expected) in a few minutes with a generous timeout;
did not need to be scoped down to `--owner`.

**After (measured just now):** 238 high-priority breach(es) across 6 rules:

| Rule | Count |
| --- | --- |
| `testing/contract` | 206 |
| `testing/fixture` | 25 |
| `testing/discovery` | 4 |
| `testing/oracle` | 1 |
| `testing/taxonomy` | 1 |
| `testing/dependency` | 1 |

**Before: not directly comparable, reported honestly rather than guessed.** The task brief cited "~147
spurious `mutation-catalog-unclaimed` breaches"; that rule id **does not exist** in the current contract
output at all — the coordinator changed the test library itself while this ticket was in flight (per
their own message), and the rule taxonomy is now `testing/contract` / `testing/fixture` /
`testing/discovery` / `testing/oracle` / `testing/taxonomy` / `testing/dependency`. There is no
apples-to-apples "before" number I can give for this exact metric without re-running contract against a
pre-rename tree, which would mean reverting live, shared files — not attempted.

What I *can* say with confidence, checked directly against the full 238-line breach list:
- Zero breaches mention "unclaimed", "no case", "missing case", or `component.feature` (other than the
  deliberately-untouched forbidden file) — i.e. nothing in the current breach set is attributable to
  test-case discovery/naming, which is what this rename targeted.
- The remaining 238 breaches are a mix of: pre-existing `no-mutation`/mutation-vocabulary issues already
  documented in this ticket's own `📓️h5-no-mutation-blast-radius.md` and `📓️w0-baseline.md`; breaches
  inside **other tickets'** archived run fixtures under `.🧬semio/.../🧾️runs/...` (e.g. the `☀️12` ticket's
  `workflow-registry-integration-32` test fixtures, and the `☀️17` ticket's `energy-support-acceptance`
  fixtures) that predate this session; and `testing/discovery` counts (`59`/`16`/`9`/`7` files outside the
  canonical owner-root tree) that are unrelated to filenames inside case directories. None of these trace
  to my rename or reference rewrite — confirmed by grepping the full breach list for every renamed
  filename and finding no hits.

## Files touched (summary)

- **Script (kept, per ticket rules):**
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📜️rename-case-files.py`
- **394 test-case files renamed** across 161 case directories (see table above) — full list is
  reproducible via `git status --porcelain | grep 🧪️tests/`.
- **32 files had 34 stale-reference lines rewritten** (list above).
- **Nothing else was modified.** No `🧪️oracle/🔣️.json`, no `🔨️modules/🧪️test/**`, no `.🧬semio/**` ticket
  docs (other than this report and the script), no `.nx/**` cache, no directory renamed, no file outside
  a `🧪️tests/<case>/` directory renamed.

## Tool-generated scratch output (deleted per ticket rules)

Intermediate dry-run JSON/logs were written to the session scratchpad
(`/private/tmp/.../scratchpad/`), not this ticket folder, so there is nothing to clean up here beyond the
kept script and this report.
