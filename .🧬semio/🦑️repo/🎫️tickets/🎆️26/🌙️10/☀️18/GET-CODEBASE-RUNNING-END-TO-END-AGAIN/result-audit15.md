# Result audit15 (read-only apply-readiness audit of backlog group E1, Nx)

Lane audit15, wave 1027b, started 23:49 on 10/27, after G49 part 2 and during the G49 rest chain `g49r` (pid 76177, still running at my last check). Live root `/Users/ueli/Documents/semio`. Nothing live was edited, no `stage.py apply|revert` ran, no `land.py apply|revert` ran, no git modifying command ran, no mirror was created, no sub-agent ran.

Method: `stage.py check` (read-only) for the three code batches and the two docs batches; `backlog-tools/mergeland.py <id> check` (read-only, same as `land.py <id> check` for these ids); a sha scan of every staged manifest (267 manifests, 2193 overlap pairs) against live in `C/🗑️generated/audit15/analyze.py` -> `report.json`; replays of the mergeland 3-way merges in `C/🗑️generated/audit15/sim/` (loader `sim/mlib.py` execs `mergeland.py` without its CLI tail). Raw check output: `C/🗑️generated/audit15/check-*.txt`, `mergeland-*.txt`.

## FOR coordinator

- E1 is NOT landed. No `@live` backup exists for `nxfix-core`, `nxfix-nx-patch` or `nxlock` (only `@b2work` mirror runs), and live equals the base for 10 + 1 + 8 pending rows.
- The only path that works is mergeland (`land.py`), never `stage.py apply`. `stage.py apply` refuses all three code batches (base-changed 3, 1, 4), and re-running `check` will not change that.
- Hard blocker 1: `nxlock` must not land before `nxfix-g core`. Right now `mergeland check` reports `nxlock` MERGE-CLEAN, but that merge is wrong without core. Replayed without core, the merged `policy.json` drops core's `nx:put-gate:` and `nx:put-slot:` prefixes and the new description, so those resources become unclassified. The merged `order/tests` drops nxlock's own test "publishing into the shared Nx cache image follows the artifacts it publishes" (15 test blocks instead of 16). Replayed core-first, both are present. Evidence: `C/🗑️generated/audit15/sim/nx-now-*.txt` against `nx-after-core-*.txt`.
- Hard blocker 2: do not land any E1 batch while `g49r` runs. E1 changes the lease-order policy and the Nx startup lease; a landing mid-gate would contaminate the G49 verdict.
- Stale manifest: `nxlock-g-tools/staged/nxlock/manifest.json` (written 10-26 18:46) still carries the pre-recut `new` hashes for `policy.json`, the order tests and `startup/tests/🟦️.ts`. `recut_nxlock.py` (backlog2-tools, 10-27 11:37) rewrote `files/f00`, `f01`, `f09` after that. `stage.py check` is therefore wrong for those rows even after landing. Use mergeland, which reads `files/`, and do not use `stage.py` for `nxlock` again.
- After E1 lands, `stage.py check` will report base-changed for the code rows, because the merged live bytes are not the staged `new` bytes (live has leaseord content that core does not). This is expected. Do not re-apply through `stage.py`.
- Pending batches that overlap E1: `langpick/copies` (`order/🟦️.ts`) and `germanqa/process` (`policy.json`). Both are cut from today's live bytes; both merge cleanly onto the post-E1 files in replay, but both need a re-cut after E1 for exact-state tools to work again. `germanqa/docs` (guide, base sha 7b0e, not live) has no docs3 blocks in its new bytes, so it must be re-cut after `docs3-nxfix-core` and `docs3-nxlock`, or it will erase them.
- No path overlap between E1 and `nolegacy/lang`, `nolegacy/nobaseline`, `schemafirst/*`, `langpick` (other than copies), or `incfacts/libfacts`.
- Doc drift: `docs3-g-tools/staged/nxlock/README.md` and `BASE.txt` say 4 edits; `edits.json` has 2 (bullet en/de), which is what `check` reads. Cosmetic.

## 1. Stage-level state (read-only)

Columns: rows; `stage.py check` pending / applied / base-changed; `stage.py apply` result; `mergeland check` result; verdict.

| Batch | Rows | check: pending / applied / base-changed (rows) | stage.py apply | mergeland check | Verdict |
|---|---|---|---|---|---|
| nxfix-g `core` | 13 (incl. guide) | 10 / 0 / 3: `leases/order/policy.json`, `leases/order/tests/🟦️.ts`, `build-guide.md` | REFUSED (all-or-nothing) | 0 conflicts: MERGE-CLEAN 2, NEW 2, UNCHANGED-BASE 8, guide skipped by design | READY via mergeland, first |
| nxfix-g `nx-patch` | 2 | 1 / 0 / 1: `startup/lifetime/tests/🟦️.ts` (cachinga landed) | REFUSED | 0 conflicts: MERGE-CLEAN 1, UNCHANGED-BASE 1 (patch file) | READY via mergeland; needs package install step |
| nxlock-g `nxlock` | 12 | 8 / 0 / 4: `policy.json`, order `tests/🟦️.ts`, `order/🟦️.ts`, order `schema/🔣️.json` | REFUSED (base-changed; the `needs-core` state is not reached) | 0 conflicts, but MERGE-CLEAN is only valid after core (see FOR, blocker 1); NEW 3, UNCHANGED 5 | HOLD until core is live |
| docs3 `nxfix-core` (guide, en+de) | 2 edits | PENDING 2 (guide sha 38412f96 -> 7a73f1dd; both anchors found once) | apply is anchored, not exact | n/a | READY (any order vs core; the guide entry in core's manifest is skipped by mergeland) |
| docs3 `nxlock` (guide bullet, en+de) | 2 edits | PENDING 2 (same guide, both anchors found) | apply is anchored | n/a | READY after `nxlock` code |

No `stage.py apply` or `revert` was run. `check` exit codes were 0 for every batch, with the states above.

## 2. Overlaps with live landings (G45 to G49 and the earlier leaseord, cachinga, progress and nolegacy batches)

All E1 rows checked against all 267 staged manifests:

- `leases/order/policy.json` and `leases/order/tests/🟦️.ts`: landed by `leaseord-tools/staged3` (G29 era). Core was cut from the leaseord `staged` base (sha 4d31745b), so this is a BRANCH. Replay of core on top of live: clean. Policy: 14 classes, live's `pin`, `bazel-picker`, `descriptor-custody` and `graph` edits kept, core's `image` edit kept. Order-test expected map: core's `nx:put-gate` and `nx:put-slot` entries and live's `build-graph` and `rust-code` entries all present, no marker lines.
- `build-guide.md`: changed by G49 `nolegacy/lease` (applied). docs3 anchors still found, so docs3 is PENDING, not conflicting. `germanqa/docs` guide base is 7b0e, which is neither live nor new.
- `startup/lifetime/tests/🟦️.ts` (nx-patch): G49 `isolate/cachinga` changed the same file from the same base. mergeland clean. Not replayed semantically.
- `leases/order/🟦️.ts` and `order/schema/🔣️.json` (nxlock): `progress/status` landed; `progress/umlaut` overlaps the schema json and is neither base nor new on live (it was superseded by the landed status batch). mergeland clean, not replayed semantically.
- No other E1 row overlaps a landed batch.

## 3. Pending batches against E1

| Group | Rows (pending / other) | E1 overlap | Action |
|---|---|---|---|
| `nolegacy/lang` (reverted 23:41) | 37 / 0 (all base) | none by path | independent; re-cut is the lang lane's job |
| `nolegacy/nobaseline` (reverted 23:41) | 17 / 0 (9 deletions) | none by path | independent; land with lang, after the lang lane |
| `schemafirst/*` (15 batches) | all base = live, 0 other | none by path | independent; re-check with `check` before each run |
| `langpick/*`: core 25/2, copies 26/7, dashboard 16/0, imports 31/4, launcher 6/1, launcher2 0/1, routing 4/5 | per batch | copies `leases/order/🟦️.ts` only; base = live now, nxlock base is older; replay onto post-nxlock clean | re-cut `copies` after nxlock; the other langpick drifts are outside E1 |
| `germanqa/*`: docs 5/2, libbuild 54/8, libcache 18/3, libother 7/0, mcp 3/1, modules 14/0, process 18/5 | per batch | `process` `policy.json`: base = live now; replay onto post-E1 policy clean; `docs` guide: base 7b0e, drifted | re-cut `process` after E1; re-cut `docs` after docs3 |
| `incfacts/libfacts` (5 pending / 4 other) | none by path | none | independent; keep last, per result-incfacts |

The "other" counts are drifts from unrelated landings. They are listed for completeness only.

## 4. Removals (base to new) in the E1 batches

Lines removed / added by difflib. Base is the live base for core rows, and core's new for the nxlock layered rows. Nothing deletes a whole file.

- core `policy.json`, -2 / +10: description widened for put-gate and put-slot, two prefixes added. Intended.
- core `order/tests/🟦️.ts`, -1 / +15: expected map line rewritten, new test. Intended.
- core `failure-tail/🟦️.ts`, -5 / +16: import removed, push and decoder rewritten to the keep-tail rule. Intended per the batch text, not verified.
- core `refusals/🟦️.ts`, -4 / +10, and `refusals.json` schema, -2 / +28 (two `"messages": []` lines removed): refusal tables are filled in. Intended, reviewed by sample only.
- core `failure-tail` schema, -3 / +6: `maximumTailCharacters` replaced by the 256 KiB keep. Intended.
- nx-patch `nx@23.2.0.patch`, -15 / +16: regenerated hunks. Changes the Nx toolkit digest, so the next Nx run re-keys every task once (verify.sh says so).
- nxlock `policy.json`, -12 / +41: `nx:daemon-startup:`, `nx:daemon-slot:` and `build-graph:` prefixes leave the graph class; the new classes `daemon-slot` and `daemon-startup` take them, and live's `pin` class takes `build-graph:`. Consistent with live. Intended.
- nxlock `order/🟦️.ts`, -5 / +6: drops the `if(wanted.rank>0){` branch of the wait check. Behaviour change. Flagged for owner review; the order suite (75 cases) in verify.sh covers it.
- nxlock `startup/🟦️.ts`, -6 / +23; `startup/service/🟦️.ts`, -3 / +4; phase fixture, -2 / +2 (duplicate `actualPhase` entries). Intended per batch text, not verified.
- nxlock `component-driver` test, -2 / +2: cargo-custody reworded. Intended.

No unintended removals found. The one behaviour change (the rank branch) needs an owner read.

## 5. Rebuild scripts

- nxfix-g: none. `stage.py build --mirror M` regenerates staged bytes from a fresh mirror; not needed unless the base moves again.
- nxlock-g: `backlog2-tools/recut_nxlock.py` is the one-shot re-cut (keeps `*.pre-backlog2`). Nothing refreshes the manifest, so the manifest is stale for three rows (see FOR). Do not hand-edit the manifest; mergeland reads `files/` and is the path to use.
- docs3: none. The edits are anchored and idempotent.
- Landing and revert for code batches: `python3 -I backlog-tools/land.py <id> check|apply|revert`. Backups go to `backlog-tools/merge/backup/<id>@live`.

## 6. Apply order

Re-run `analyze.py` and `mergeland.py <id> check` immediately before each step.

- Step 0, wait: `g49r` (pid 76177) must finish GREEN first.
- Step 1: `python3 -I backlog-tools/land.py nxfix-core apply`, then `sh nxfix-g-tools/staged/core/verify.sh` (5 single-file suites).
- Step 2: `python3 -I docs3-g-tools/staged/nxfix-core/stage.py apply --root <live>`, then confirm the two guide blocks are present.
- Step 3: `python3 -I backlog-tools/land.py nxfix-nx-patch apply`, then the package install step (the new toolkit is provisioned on the next Nx run), then `sh nxfix-g-tools/staged/nx-patch/verify.sh`.
- Step 4: after Step 1 only, re-run `land.py nxlock check` and confirm the merged `policy.json` contains the `nx:put-gate:` prefix and `order/tests` has 16 `test(` blocks, both on a mirror. Then `land.py nxlock apply`, then `sh nxlock-g-tools/staged/nxlock/verify.sh`.
- Step 5: `python3 -I docs3-g-tools/staged/nxlock/stage.py apply --root <live>`.
- Step 6, product verdict: `course:build`, `repo-dashboard-rs:build`, one Nx daemon cold start (no `[lease-order]` hold-and-wait for `nx:daemon-startup:`), plus the mirror law list from result-backlog2 (263 files) before the next gate.
- Step 7, dependents: re-cut `langpick/copies` and `germanqa/process` on the post-E1 bytes. Re-cut `germanqa/docs` on the post-docs3 guide.

Revert, in reverse order: `docs3 nxlock` (stage.py revert), `nxlock` (land.py revert), `docs3 nxfix-core` (stage.py revert), `nx-patch` (land.py revert), `core` (land.py revert). Do not use `stage.py revert` on the code batches after Step 1; the exact-state check will not match the merged live bytes.

## 7. Limits

- Merges are textual (git merge-file plus mergeland's line rules). Semantic replay was done for `policy.json` and `order/tests` only (core and nxlock). `lifetime`, `startup`, `order/🟦️.ts` and the refusal tables were not replayed.
- `verify.sh` was not run (no mirror was created; the lane rules forbid a live run). The verdicts above do not replace it.
- `analyze.py` compares sha only. Removal counts use difflib line counts; base bytes were found by sha in the staged `base`, `files`, `backup` and `base0` folders.
- `docs3` checks are anchor searches only; nothing was applied.
- The germanqa guide content and the G49 chain outcome are out of scope; only their overlap with E1 was checked.
- `mergeland check` for `nxfix-core` and `nxlock` was run directly with `python3 -I` (the same code path `land.py` uses for these ids). It writes nothing in check mode.
