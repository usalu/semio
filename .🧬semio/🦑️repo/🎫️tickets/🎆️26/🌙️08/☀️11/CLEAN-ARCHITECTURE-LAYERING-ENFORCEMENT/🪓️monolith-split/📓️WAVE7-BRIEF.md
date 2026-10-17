# 🔂️ Wave 7 Brief: Recover, Finish, Compile

Wave 7 starts 2026-10-17 07:15, after the third Claude Code process restart (about 07:05). Every agent from the earlier sessions is gone. Their edits, tool folders and RESULT files are on disk.

## Read first

`/Users/ueli/Documents/semio/AGENTS.md`, `📓️EXECUTOR-RULES.md` (all of it), `📓️WAVE5-BRIEF.md` (rules 1–6, the compile mutex, wave 6), `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`, then your own RESULT file and tool folder.

## Rules

1. **Recovery first.** You continue a cut executor. Before any edit: read your RESULT file, your `<prefix>-tools/` folder (state files, journals, `orig/` snapshots, staged trees) and your `🗑️generated/<prefix>-*` logs; compare the live files with the snapshots (`git diff HEAD --stat -- <files>` plus your snapshot hashes). Scratchpad paths named in your RESULT file are gone (the scratchpad is wiped on restart); rebuild staging from the tools folder. Write a "found state (wave 7)" section into your RESULT file first. Finish or roll back only your own half-applied batch.
2. **Finish, then report.** Bring the slice to DONE with static evidence (losslessness, rustfmt, the scoped patched differential, the census, region markers, struct and impl visibility, source-text readers, no compatibility re-exports). Re-run each long oracle at most twice.
3. **Real compile.** The compile mutex in `📓️WAVE5-BRIEF.md` still applies to every `bun repo cargo` call. The coordinator probes the launcher at 07:15; if it admits, the coordinator announces it in `📓️FLEET.md` (line starting `COMPILE ADMITTED`). Check that line before you finish. If it is there, run your slice's real check (one batched call under the mutex); otherwise record the exact command for the compile queue.
4. **Deliverable.** Rewrite section 0 (verdict) of your RESULT file with DONE or BLOCKED and the reason. Confirm the file with `ls -l` before you end your turn.
5. Foreground only, no sub-agents, no ticket tool, no modifying git command, never delete anything in a `🗑️generated` folder, logs only in `🗑️generated/<prefix>-*`.

## Crate rollout (09:00 priority change)

`AUDIT-WAVE7-CRATE-GATE.md` (all 287 workspaces, 2,419 crates): 58 crates are oversized in their lib closure, and 43 more only in their test closure. Each oversized crate refuses every crate above it (the stdio codec crates each block about 300). The crate gate, not file length, is what stops the compile. The plugin-crate generator claim in `📓️FLEET-CLAIMS.md` is this fleet's, so the fleet owns the rollout.

Rollout slices follow `📓️PLUGIN-ROLLOUT-BRIEF.md` and `RESULT-PLUGIN-WAVE0-GENERATORS.md` §16 (§16.6a is the per-package sequence: `new crates <pkg> --verify --oracle --sandbox=<dir>`, apply, `--check-applied` exit 0 with 0 extras, owners scan, taxonomy, `lib-wiring-budget`, launch JSONs parse). Work bottom-up within each slice (dependencies first). Afterwards the census row for every crate you produced must be inside the gate, for lib and test. Re-measure with `🗑️generated/cgc7-census.ts`. When a package is also being file-split by a running `c4p*` slice, wait for that slice's RESULT to say DONE before carving it.

## Carve rule: no cross-crate `cfg(test)` items (11:30)

An upstream crate's `#[cfg(test)]` items exist only when that crate itself is the test target. The union oracle still resolves them across the new crate boundaries, but a real build does not. When a carve puts a `cfg(test)` helper (fixtures, demo cases, demo snapshots) in one crate and its users in another, move the helper into the owner's fixtures/test-support crate (a dev-dependency of the users), or behind a non-default `testkit` feature that the users enable from `[dev-dependencies]`. Never use `cfg(any(test, …))` hacks or compatibility re-exports. CFG-TEST-CROSS-CRATE (`ctc`) fixes the crates that are already finished.

## Oracle blind spot: kept root file modules (12:10)

The resolution oracle's unlinked-cascade filter hides unresolved `use` lines that go through a root file module the carve keeps (`pub mod x;` at the artifact root). After every carve, check by hand that no `crate::<kept-root-mod>::<item>` path points at an item that moved to another crate (R8C found 8 such files in equation).

## Canonical testkit idiom (14:10, corrects the 11:30 rule)

The single canonical form is the owner's `#[cfg(any(test, feature = "testkit"))]` on the helper, `testkit = []` (plus any optional dependencies) in the owner's `[features]`, and each consumer crate enabling `features = ["testkit"]` on the owner in its `[dev-dependencies]`. Do not use a self dev-dependency (`path = "."`): it builds the lib twice in test builds, which duplicates types. Plain `#[cfg(feature = "testkit")]` combined with a self dev-dependency gets converted to the canonical form. The earlier "no `cfg(any(test, …))`" wording only ever meant hacks that compile test code into production builds without a feature.
