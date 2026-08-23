# 📊️ Migration status

## Ladder

`discovered → surveyed → contract-ready → oracle-green → subject-green → parity-green →
coverage-green → dependency-clean → legacy-removed → ci-enforced → complete`

Tracked machine-readably in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📇️registry/🔒️migration.json`.

| Owner | Status | Evidence |
| --- | --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` | `complete` | 3 scenarios × 5 implementations, 30/30 pairwise parity, 45 self-tests, contract/dependency/clean all green |
| `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition` | `legacy-removed` | 4/4 differential parity vs `clsx`; conflict half specified by vectors under a recorded no-oracle decision; legacy test deleted |
| `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️style-variants` | `legacy-removed` | 3/3 differential parity vs `class-variance-authority` across the full variant matrix; legacy test deleted |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf` | `oracle-green` | 5 oracle executions via `pdf-writer` + `lopdf`, projected by the independent reader |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png` | `oracle-green` | 3 oracle executions via `png` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif` | `oracle-green` | 2 oracle executions via `gif` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip` | `oracle-green` | 3 oracle executions via `zip`, including member removal |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate` | `oracle-green` | 3 oracle executions via `flate2`, round-trip mode |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav` | `oracle-green` | 3 oracle executions via `hound`, including sample-rate retune |
| every other non-`compose` owner | `discovered` / `surveyed` | 48 unmanaged legacy test files, shrink-only baseline |

All six artifact owners are `oracle-green` rather than `parity-green` for one reason: their
subject phase links `semio-s-plugin-stdio`, which does not compile while a concurrent session's
os-kernel refactor is in flight. Every subject handler is written; see `_blockedBy` in
`🔒️migration.json`.

## Repository dashboard, current values

| Metric | Value |
| --- | --- |
| Test cases discovered | 11 across 8 owners |
| Scenarios declared / executed at quick | 32 / 30 |
| Executions at quick | 49, all passing |
| Parity comparisons | 37 / 37 equal |
| Implementations with a native host | 5 of 5 |
| Implementation coverage | typescript 13/13 · rust 20/22 · dotnet 3/3 · go 3/3 · python 3/3 |
| Registered oracles | 10, every one classified `test-oracle` |
| Oracles carrying recorded production debt | 2 (`png`, `zip`), both Phase 7 targets |
| Recorded no-oracle decisions | 2, each naming its substitutes |
| Oracle coverage | 11/11 cases backed |
| Comparison profiles | 14, each self-tested for both what it ignores and what it keeps |
| External dependencies classified | 212 across 4 ecosystems |
| Production-reachable external dependencies | 152 |
| Dependency-clean coverage | 60/212 (28.3%) |
| Unmanaged legacy tests | 48, shrink-only |
| Orphan fixtures · committed generated wrappers · quarantine entries | 0 · 0 · 0 |

## What each remaining phase needs

* **Phase 6 — owner migration.** Every owner: move behaviour into `component.feature`, select or build
  the oracle, fill language gaps, delete the legacy tests in the same change, lower its count in
  `🔒️migration.json`. The contract phase already refuses to let any area's count grow.
* **Phase 7 — production dependency retirement.** Work leaf-first through the 154 production-reachable
  entries; the ratchet forbids new ones today.
* **Phase 9 — hard enforcement.** `.github/workflows/test.yml` runs taxonomy, contract, dependency
  purity, quick/long/exhaustive, coverage, production purity, clean safety and — in `test-exhaustive`
  — the non-aggregate metric gates (`test metrics --enforce`). That gate is red today for exactly one
  reason: the peer-blocked PDF subject phase leaves rust at 3/8 scenarios. It is reporting a real gap,
  which is what it is for.

## Deliberate deviations from the plan, and why

1. **No `🟦️component.ts` at the testing-domain root.** Shape V2 forbids leaf files at an owner root.
   The domain's TypeScript entry is `📦️packages/🟦️typescript/📦️index.ts`, which is what every caller
   imports.
2. **The oracle host is a Cargo *feature*, not a separate crate.** One crate with an `oracles` feature
   keeps a single protocol implementation and lets a subject-only host skip compiling the reference
   libraries entirely. No production target can enable it.
3. **The generated host crate's `Cargo.lock` is not committed.** The repository's `.gitignore` ignores
   `**/📦️packages/**/Cargo.lock`; the pinned constraint lives in the oracle registry entry and the
   host crate's `Cargo.toml` instead, matching the existing convention.
4. **`bun ./📜️script.ts test dsl` was removed** with the other manually enumerated suites. Nothing
   referenced it; the same projects are reachable through normal Nx targets.
5. **Storybook/Playwright stays a named suite.** It is the one non-`compose` behaviour that cannot yet
   be expressed as an owner test; removing it would have deleted coverage rather than relocated it.

## Migration playbook, as actually walked twice

Both UI owners were migrated by the same sequence, and it is the sequence the README prescribes:

1. Read the legacy test and ask what contract it is really asserting.
2. Split the contract where a third-party reference only covers part of it. `cn()` splits into a
   flattening half (`clsx` is a genuine oracle) and a conflict-resolution half over
   repository-owned utility families (no library knows them — recorded no-oracle decision).
3. Register the oracle with its rationale, license, capabilities and comparison profiles, and add it
   as a devDependency of the test platform package only.
4. Write the feature so the VECTORS live in it — doc-string JSON or a `| input | expected |` table —
   so the oracle and the subject are provably given the same input.
5. Make contract, then oracle, then subject, then parity green.
6. Run the dependency gate: the new package must appear as `test-oracle`, `productionReachable:false`,
   and the production-reachable count must not move.
7. Delete the legacy test AND its runner include entry in the same change; lower the owner's count in
   `🔒️migration.json`.

Scoping honestly is the part that carries the work. `tailwind-merge` was available and would have
"worked" as an oracle for `cn()` — and would have proven nothing, because it resolves against
Tailwind's default scale rather than this repository's families. Recording that as a no-oracle
decision with specification vectors is the stronger result.
