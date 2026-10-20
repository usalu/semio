# Audit of staged sets, 2026-10-20 (aud-staged)

Scope: every staged set listed in the brief, plus `staged-merge-tools/plan.json` and `plan.ts`. Read-only. Nothing was applied, no plan was regenerated, no set verification command was run. Only read-only checks and `--dry-run` / `check` modes were executed (each returns before any write; verified in source).

Runtime state at audit time: a dependency sync is running (`deps4-loop.sh`, pid 38018, `bun repo dependencies sync --all`). Every set's apply guard refuses with exit 2 until it exits; `--ignore-sync` was used only for dry-runs. The HARD FREEZE is assumed still in effect.

## Verdicts

| Set | Targets | Verdict | Blocking reason (short) |
|---|---|---|---|
| tax-stage | 168 ops (144 writes incl. creates, 24 deletes), 1 package.json op | FIX-FIRST | 2 peer-edited files conflict; plan stale; package.json op; compat re-export shims |
| tsc-stage | 23 patch entries (plan lists 22) | FIX-FIRST | Must run after tax-stage; silent retarget; applies onto a peer-edited file |
| walkperf | 9 (4 patch, 5 create) | FIX-FIRST | Depends on tsc-stage; `--rebase` fence bypass; diff3 conflict on shared file; gate:false |
| lease-fair | 12 (7 write, 5 create) | FIX-FIRST | No sync guard while sync loads `🔒️leases`; 3 `[DEBUG]` logs |
| cargo-pressure | 8 (4 patch, 4 create) | FIX-FIRST | Three fence relaxations need owner sign-off; weak idempotency |
| bzl-stage | 11 (5 write, 6 create) | FIX-FIRST | Provisional, no result report; fuzzy `patch` rebase; depends on tax-stage |
| env-pass2 | 41 files, 82 edits (36 in library) | FIX-FIRST | apply.py ignores `--dry-run`: drive's rehearsal writes files |
| fixups6 | 13 rows (all library) | NOT READY | Unregistered; apply.py ignores `--dry-run`; default root is the live repo; conflicts with tax-stage on the same test file |
| exports-stage | batch.json: 636 entries (508 apply, 128 blocked) | NOT YET | Freeze and deps-owner gated; last in order |
| bzl-real | 3 staged files | NOT READY | Scratch-host harness, not a repo change; never apply to library |
| law-sweep2 | 0 staged | NOT READY | No staged content, no apply entry |
| inputs-closure | 0 staged patch set | NOT READY | Census and oracle tools only; not registered |
| walkperf2 | none | NOT STARTED | Directory missing |

## Plan level (`staged-merge-tools/plan.ts`, `plan.json`, `sets.json`)

1. plan.json is stale. `bun plan.ts check` reports `inventory STALE, live STALE`. plan.json was generated 2026-10-19T23:46Z; tax-stage `staged/ops.json` was created 2026-10-20T00:35Z. plan.json also says exports-stage, env-pass2, bzl-real and law-sweep2 have "no apply entry point"; their entry files now exist (exports-stage `apply.ts`, env-pass2 `apply.sh`/`apply.py`). Regenerate only after the restage items below.
2. Order contradicts `sets.json`. `tsc-stage` declares `after: ["tax-stage"]` (sets.json), but `orderSets` treats `after` as soft (`plan.ts:516`). tax-stage is excluded (blocked), so plan.order runs tsc-stage first. tsc-stage then patches the large `📜️script.ts` bodies that tax-stage deletes or rewrites, and tax-stage can no longer apply. Make `after` hard, or refuse a set whose `after` predecessor is excluded.
3. Drift fence bypassed by planner. `attempt()` retries with `--rebase` whenever the plain run is refused (`plan.ts:287-288`). The chosen flag is stored in plan.json `applyArgs` and used by drive. walkperf (`["--rebase"]`) and bzl-stage (`["--rebase"]`) need it only because tsc-stage/tax-stage changed their anchors; that interplay should be fixed by restaging, not by skipping the sha check. `--rebase` skips the sha check in walkperf (`walkperf.py:265`, `patched_texts` strict flag at `:227-234`) and uses fuzzy `patch` in bzl-stage (`bzl-stage-tools/stage.py:90`).
4. Pair verdict ignores diff3. `pairVerdict` (`plan.ts:323-328`) returns `commute` from digests only. tsc-stage/walkperf is recorded as `commute` while diff3 on `💾️storage/🟦️.ts` is `conflict`. The verdict must downgrade when diff3 conflicts.
5. Planner sandbox writes live for fixups6. Sandbox runs inherit `process.env`. fixups6 `apply.py:2` defaults `ROOT` to the live repo (`APPLY_ROOT` unset), so a replay would write the live tree. Any tool registered in sets.json must root itself at its own sandbox.
6. `drive --from <unknown-id>` runs every step (`plan.ts:636`: `findIndex` returns -1, `Math.max(0,-1)=0`). Reject unknown ids as `--only` does.
7. Freeze guard. drive refuses without `--yes`, which is correct, but tax-stage's plan step has no `--skip-package-json` (see tax-stage below), so the step would edit a package.json inside the frozen zone once the freeze is lifted.

## Per-set findings

### tax-stage: FIX-FIRST

- Conflicts (`apply.py check`): 2. `🏗️build/🧪️tests/🕸️graph/🟦️.ts` (base `a50e942…`, live `3ece327…`, staged `9109382…`) and `🧫️fixtures/🕸️graph/🔣️.json`. Both were modified in the working tree at 02:43 and 02:45 today (`git status` M). This is a peer edit in progress, not something this set should overwrite. `apply.py` `classify` (`:39-47`) and `cmd_apply` (`:91-106`) refuse correctly. Decide the owner of that file before restaging.
- fixups6 is anchored on the same test file with base `3ece327…` (the current live hash). tax-stage and fixups6 therefore disagree about the same file.
- Package.json op: `🛠️tools/package.json` (ops.json `group: package-json`, blob `3955480281…`). Freeze forbids package.json edits. `apply` writes it unless `--skip-package-json` is passed (`apply.py:98`); plan.json applyArgs is `[]`. Either pass `--skip-package-json` in the plan or move the op into exports-stage.
- Compat re-export shims. Re-exports at old paths preserve old import surfaces, which AGENTS forbids (no compatibility layers or re-exports for compatibility):
  - `📦️packages/🟦️typescript/📦️artifacts/🟦️.ts`, staged blob `8c292e00…` lines 3-19 (barrel of 17 re-exports from `⚡️caching/📦️artifacts/...`).
  - `🔧️tools/🏗️bazel/🟦️.ts`, blob `88be2867…` line 21 (`prepareBazel` from caching bootstrap).
  - `📥️inputs/🧮️digest/🟦️.ts`, blob `39d31f4a…` line 7.
  - `📥️inputs/🔒️lock/🟦️.ts`, blob `8cb85f72…` line 5.
  Keep only re-exports a client explicitly needs; rewrite importers for the rest.
- Relocation fidelity (token balance across all 168 ops, removed side vs staged side): `throw` 940 vs 939, `Error(` 821 vs 820, `assert` 364 vs 364, `expect(` 3504 vs 3498, `if(` 1651 vs 1647, `return` 966 vs 961, `await` 3711 vs 3736, `import ` 1034 vs 1126, `export ` 239 vs 255, `[DEBUG]` 66 vs 66. The moves look behavior-preserving except the graph test, which loses 6 `expect(` and 1 `throw` relative to its base. Verify those assertions survive on the restaged file.
- Router skeletons (`📜️script.ts` replaced by `ScriptRouter` dispatch over owner modules, e.g. blob for `⚡️caching` with 39 lines) are consistent with AGENTS.
- Status: provisional (no `result-tax-stage.md`), not replayed by the planner (replay covered 5 sets only), plan.json entry excluded as blocked. The set's own verify (`root script builds`, `repo cli boots`, path-budget law) has not been run.
- Required before apply: restage with `bun stage.ts` after the graph-test owner decision; add `--skip-package-json` (or defer that op); resolve or drop the 4 re-export shims; write the result report; run verify.

### tsc-stage: FIX-FIRST

- Mechanics are sound: `stage.py apply` computes all patched texts before any write (`:199`), writes via tmp+`os.replace`, and refuses on the sync guard (`:193-197`). `--dry-run` returns before writes. Plain dry-run passes for all 23 entries (`--ignore-sync --dry-run`, rc 0).
- No base-sha check: `apply` never compares `baseSha256` (`patches.json`). Only anchor counts are checked. The graph test is reported `patchable (base drifted)` (`stage.py check`); tsc-stage would patch the peer-edited file.
- Silent retarget: `candidates()` (`stage.py:150-153`) and `patched()` (`:156-166`) retarget `📜️script.ts` patches to the sibling `🟦️.ts` whenever the anchors fail on the script. Once tax-stage rewrites the script, a patch aimed at the script lands in the owner module without any warning. Acceptable only if tax-stage runs first and the retarget is logged and asserted.
- Idempotency is looser than tax-stage's (`stage.py` `apply_ops`: `elif new in text: continue`), so a replacement that already occurs elsewhere counts as applied. Low risk; tighten to anchor-local check.
- Type changes: no widened types added. The one `: any` in `🕸️graph/📥️inference/🧪️tests/💾️cache/🟦️.ts` is pre-existing in the anchor text.
- Patch #22 (`💾️storage`) removes the `[DEBUG]` log, which walkperf requires (sets.json `after`). Correct.
- Order: must follow tax-stage (see plan item 2). Heavy `tsc` verify is gated (`heavy: true`).

### walkperf: FIX-FIRST

- Depends on tsc-stage (sets.json `after`). With tsc-stage applied first, the plain run is refused and the planner adds `--rebase` (plan item 3). Restage walkperf after tsc-stage instead of rebasing: `walkperf.py:265` (`strict = "--rebase" not in args`) and `:227-234` (drift check skipped).
- diff3 conflict on `🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` against tsc-stage (plan.json pair `tsc-stage<->walkperf`, `diff3: conflict`, verdict `commute`). Resolve by hand before apply.
- Weak idempotency: `walkperf.py:208` `if text.count(replacement) >= 1: continue`. Use anchor-local check.
- Verify is `gate: false` (sets.json), so a failing walkperf suite still passes the apply. Make at least the `generation receipt` suite gating.
- Content is schema-first (`schema.json`) with a fixture (`fixtures.json`) and a test; sync guard present (`walkperf.py:264-270`). Everything is in library (frozen).

### lease-fair: FIX-FIRST (small)

- No sync guard. `apply.py:35` prints `check ok (no running-process check: stop the sync loop first)`, and the apply branch (`:36-47`) has no `ps` check. The dependency sync's storage path (`bootstrap/📦️dependencies/💾️storage/🟦️.ts`) imports the lease module, so changing it mid-sync risks mixed lease semantics across processes; the ticket format also changed (`queueTicketRow`, exact key set). Add the guard used by walkperf/stage.py.
- Lease-fair is outside the literal HARD FREEZE list (`🔨️modules/🏃️process/`), but the sync-path dependency above is why it needs the guard.
- `[DEBUG]` logs added (AGENTS: remove before close): `lease-fair-tools/staged/.../⚖️fairness/🧪️tests/🟦️.ts` lines 80 and 115; `.../📈️progress/🟦️.ts` line 77.
- Otherwise compliant: `progress.json` and `⏳️wait/🧬️schema/📜️policy.json` are schema changes with code; `fairness-fixture.json` and `fairness-schema.json` provide language-agnostic fixtures; the progress test cross-checks with Ajv (test-only; verify `ajv` is a devDependency: it is declared in root `package.json`). `project.json` adds `test-resource-lease-fairness` and only calls `bun ./📜️script.ts test resource-lease-fairness`; the `script.ts` test entry is added.
- Risk: the fairness test asserts CPU-time bounds ("bounded fraction of one core per waiter"). These are load-sensitive (see memory on timing laws under peer load); keep it gating only if it is stable on a quiet machine.
- Plan order is fine: lease-fair has no pair overlap with other ready sets.

### cargo-pressure: FIX-FIRST

- Three fence relaxations, each needs explicit owner sign-off:
  1. Optional recheck. `interface.new.txt` makes `recheck?` optional, and `apply.py:56-57` calls `port.recheck?.(...)`. Any port that omits `recheck` now silently skips storage rechecks. Make it required.
  2. Throttle. `port.new.txt` `storageRecheckGateV1` skips non-completion rechecks within 15 s. `feature.add.txt` says storage is "settled once before each unit", which the throttle contradicts unless the gate's clock is injected by the test. Owner must accept that custody is settled at most once per interval.
  3. First read unchecked. `apply.py:66-67` makes the recheck inside `read` run only when `offset>0`/`position>0`, relying on `cfg()` having settled storage. Correct only while `cfg` settles through the same gate.
- Idempotency: `apply.py:91` `if replacement in text: continue` (same weakness as walkperf).
- Gate: the admission suite is `gate: false` (documented pre-existing failure); the recheck law and read suite are gating.
- Sync guard present (`apply.py:138-143`); dry-run returns before writes (`:148`). Plain dry-run passes.
- Two creates are under library (frozen): `pressure.test.ts`, `pressure.json`.

### bzl-stage: FIX-FIRST

- Provisional (no `result-bzl-stage.md`), so drive skips it unless `--include-provisional`.
- `rebased()` uses `patch -s` with the default fuzz (`bzl-stage-tools/stage.py:90`). Fuzzy hunks can land at the wrong place without failing. Use zero fuzz (and assert the hunk count) before allowing `--rebase`.
- Shares `🔧️tools/🏗️bazel/📜️script.ts` with tax-stage (plan pair conflict); restage after tax-stage.
- Plain dry-run passes now; `--rebase` is needed only after tax-stage/tsc-stage interplay.
- New test files use `as any` (11 lines in `sources-test.ts` and `workspace-test.ts`). Not an AGENTS rule, but weakens types.
- Verify gate: `gate: false`.

### env-pass2: FIX-FIRST (apply.py broken for the planner)

- `apply.py:44` `only = set(sys.argv[3:])` and `:61` `elif mode == "apply"`: the mode is `argv[1]` and `--dry-run` is never honored. `apply --dry-run` (the form drive and the sandbox use) therefore writes every staged file. `apply --ignore-sync --dry-run` writes nothing and prints success, because `only` filters everything out. Nothing is checked for drift; writes are non-atomic (`open(...).write`).
- Entry is now present (`apply.sh` -> `apply.py`), so a re-plan would register it. Fix before any plan.
- 41 files, 36 in library (frozen); 82 edits; anchors have counts (strict). No package.json.
- Leftover tool outputs in the tool directory (`summary*.tsv`, batch scripts) must be deleted at close.

### fixups6: NOT READY

- Not in `sets.json`, so not planned. `apply.py:13` `dry=mode in ("--dry-run","dry-run")` gets `argv[1]`, so `apply --dry-run` writes. Its default `ROOT` is the live repo (`:2`).
- 13 rows, all in library. Its BASE row is the same test file as tax-stage's conflict (base `3ece327…` equals the current live file). The coordinator must choose one owner for `🕸️graph/🟦️.ts` and its fixture before either set is planned.

### exports-stage: NOT YET (last)

- `apply.ts` guards are strong: `--freeze-lifted` required (`:134`), process and sync guards, baseline and target checks, backups, tmp+rename writes, remove-key and round-trip checks. `check` mode writes nothing.
- Staged state (`batch.json`, `stagedAt 2026-10-20T00:36Z`): 636 entries, 508 apply, 128 blocked, 34 manifests, 379 glue creates, 21 extends, 19 deletes, 25 importer rewrites. Note: the manifest check covers a snapshot from before the peer edits above.
- Verification runs after writes (`apply.ts:159-162`): failures exit 3 with the tree already modified. Consider verifying before writing the second half.
- Requires dependency lock and sync afterwards; deps-owner gated.

### bzl-real: NOT READY

- `stage.ts` writes `staged/adapter.ts`, `staged/store-receipt.mjs`, `staged/store-receipt.test.ts` as a scratch-host variant of the Bazel adapter. It is a harness, not a repo change. Keep out of plan.json; never apply to library.

### law-sweep2, inputs-closure, walkperf2

- law-sweep2: no staged content, no apply entry, `verify: []`.
- inputs-closure: census and oracle tools only, not in sets.json.
- walkperf2: directory missing.

## Recommended global order

Consistent with `sets.json` `after` edges and the planner, once the freeze is lifted and the sync has exited:

1. Coordinator decides owner of `🕸️graph/🟦️.ts` and its fixture (tax-stage vs fixups6).
2. lease-fair: add sync guard, remove 3 `[DEBUG]` logs, then apply (outside the literal freeze, but it is loaded by the sync).
3. tax-stage: restage (`bun stage.ts`), `--skip-package-json`, resolve the re-export shims, result report, verify.
4. tsc-stage: restage after tax-stage; no `--rebase`.
5. walkperf: restage after tsc-stage; resolve the storage diff3 conflict; gating verify.
6. bzl-stage: restage after tax-stage; zero-fuzz rebase; result report.
7. cargo-pressure: owner sign-off on the three relaxations, then apply (library test file frozen until lift).
8. fixups6, then env-pass2: fix `apply.py` argument handling and `ROOT` before registration.
9. exports-stage: last; then `bun repo dependencies lock`, `bun repo dependencies sync`, `simulate.ts`.

Not ordered: walkperf2 (not started), bzl-real (harness), law-sweep2 and inputs-closure (no content).

Then regenerate plan.json (`bun plan.ts plan`) and confirm `check` reports current inventory and live hashes before any `apply --yes`.

## Limits

- No test or verify command was executed. Only dry-runs and `check` modes.
- The freeze status and sync state were read from `ps` and the brief; not confirmed with the coordinator.
- Token-balance and docstring/comment scans are heuristics over added text; they do not replace reading each owner module.
