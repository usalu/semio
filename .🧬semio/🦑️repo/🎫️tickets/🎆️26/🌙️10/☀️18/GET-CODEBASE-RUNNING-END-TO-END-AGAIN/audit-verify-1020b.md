# Audit verify 1020b (read-only, 2026-10-20 ~11:00-11:25)

Scope: live tree after today's applies, compared with the staged content of each set, then each `sets.json` verify command re-run one at a time. No repo file was edited; only files under `🗑️generated/audit-verify-1020b/` and this report were written.

## Method
- Live vs staged: `staged-merge-tools/plan.ts inventory` for target lists, plus a read-only checker (`🗑️generated/audit-verify-1020b/check-live.ts`). For write/create targets: sha256 of live vs staged (`exact-match`). For patch targets: sha256 when the ops carry it, else each replacement text must occur in the live file. Diffs in `🗑️generated/audit-verify-1020b/diffs/`.
- Each set's own `apply.sh check` was also run (`check-*.out`). Those reports are not conclusive for applied sets: they say "changed since staging" because the base has moved on.
- Verify commands: `🗑️generated/audit-verify-1020b/run-verify.ts` runs `sets.json` verify entries sequentially with `SEMIO_TEST_ARTIFACT_DIR=/private/tmp/aud-verify-art`. Outputs are in `verify/`, summary in `verify-summary.txt`.
- Caveat: the live tree was being restructured by other agents during the audit (new `🫧️transient`, `🛂️manifest`, `⏱️trace`, `🔏️hash`, `📁️filesystem`, `📣️publication` package.json files, and 145 exports-stage glue files, all with 11:04 mtimes).

## Headline
- Applied content matches staged content for every set except the drift listed below. Most drift is later edits (lift-fix repairs, peer edits at 06:12, 07:28 and 11:04), not failed applies.
- Most verify failures are environmental, not in the applied code: `node_modules` (generation `70825028305173c7f252`, built 05:15-05:37) has no workspace links for packages created or moved at 11:04 (`@semio-tech/framework-transient`, `-manifest`, `-trace`, `-hash`, `-filesystem`, `plugin-registry-publication`). Every module-resolution failure below traces to this. Not fixed here: needs `bun repo dependencies sync`, which is frozen.
- exports-stage: 148 of 379 glue creates are absent from the live tree (145 are staged-added in git and deleted in the worktree, all under `typescript/🕹️interaction`, `🖱️ui`, `🧪️testing`). The apply log reported success and the files were present at apply time. The deletion is not explained by anything this audit ran (see Method); 11:04 peer activity in the same directories is the only candidate found.
- Verify jobs run here: 22 (plus 1 skipped by design). 10 pass, 10 gate failures, 2 informational failures; details below.

## Per-set table
| set | live matches staged | verification (sets.json, one at a time) | issues |
|---|---|---|---|
| lease-fair (tree-bases, 12) | 12/12 exact sha | fairness law 11/11 PASS; progress vector law 4/4 PASS (with short `SEMIO_TEST_ARTIFACT_DIR`) | The suite refuses artifact paths over 256 chars. A ticket-folder artifact dir fails 1 test with "Materialized absolute path exceeds 256 characters". Verify runner needs a short temp artifact dir. |
| tax-stage (ops-json, 101) | 67 exact; 10 patch ops present (sha differs); 13 patch targets no longer contain the staged replacement; 7 writes/creates differ; 4 deletes absent (ok) | root build FAIL (unresolved `@semio-tech/framework-manifest/*`); repo CLI boots PASS (`bun repo --help`); path-budget law FAIL (missing `framework-transient`) | 13 misses are the nested `receiveScriptProcessInvocation` birth lines, rewritten by lift-fix (bare entry for test programs, `argv` pass-through for the rest). Sampled, all consistent with lift-fix. Differing creates: 2 lift-fix argv pass-through; 5 later peer edits (`with { type: "json" }`, `import.meta.dir`→`dirname`, import order, inline map). |
| tsc-stage (patches-json, 23) | 22/23 ops present; 1 superseded | path-budget FAIL (missing module); cache-probe tests PASS 3/3; repo tsc gate FAIL (rc 2): 413 errors, 237 are TS2307 module-not-found | The one superseded op (graph test `fixture-app` mapping) survived the fixups6 split into `🎯️source-demand/🟦️.ts`, which is correct. The remaining 176 TS errors are not triaged here; the result-post-apply pending list was not re-checked. |
| walkperf (py-module, 9) | 4 exact; 3 ops present; 1 target superseded; 1 differs | generation receipt gate PASS 12/12; neighbour suites (informational) rc 1, 12 pass, neutral readonly Bun failures | Test imports `🗂️workspaces/🟦️bun/🟨️.cjs` in staging; a peer converted the module to `🟦️.ts` (index shows `M`/`A`), so the walkperf patch to `.cjs` no longer applies. Superseded, not reverted. |
| bzl-stage (py-module, 11) | 10 exact; 1 differs | bazel suites PASS (11+2+9+4 pass, 0 fail, 1 skip each); rc 0 | Differing `script.ts` adds `with { type: "json" }` (peer edit 06:12). |
| cargo-pressure (py-module, 9) | 4 exact; 5 ops present | recheck law PASS 3/3; admission suite rc 1 (2 fail, staged note says 1 existing); read suite PASS 2/2; real-image heavy test SKIPPED by me | The admission suite has 2 failures, not the documented 1: "fills every actual compiler unit" (consumer boundary 9/8 files) and "progress vectors". Heavy test needs the cargo compiler image, which the brief forbids. |
| fixups6 (tree, 12) | 11 exact; 1 differs | aggregator bundle FAIL: unresolved `@semio-tech/plugin-registry-publication/build` and `framework-filesystem/observation/descriptor` | The differing admission test was changed at 11:04 to the new package name `@semio-tech/plugin-registry-publication`, which is a peer edit. |
| env-pass2 (edits-json, 41) | 41/41 replacements present | patch-set idempotence PASS ("ok 41 files") | none |
| own-crash (tree, 3) | all 3 files superseded by own-crash2 (same files) | admission law FAIL (missing `framework-transient`) | Expected supersession. |
| own-crash2 (edits-json, 4) | 4/4 ops present | admission law FAIL (missing `framework-transient`) | Failure is environmental. |
| exports-stage (batch-json, 379 creates, 21 extends, 19 deletes) | creates: 226 match, 148 absent, 5 differ; extends 77/78 ops present; 19 deletes absent (ok) | no `verify` entries in sets.json | 1 op not applied: world3d-scene-shading replacement in `⚛️react/📦️packages/🟦️typescript/📜️script.ts` (apply log warned "no occurrence"). The 5 differing creates: 3 `🔣️json/` relocations and 2 with different content. `bun repo dependencies lock` and `sync` were not run (frozen). |

Not in `sets.json` (checked by content, not re-run):
- lib-defects: staged only, not applied. The `unknown` state is correct; its ops are absent in live (12 patch targets).
- deps5 (live = staged except for `paths.test.ts`, which differs only in relative-import depth; all its relative imports resolve). The 5 other files are byte-identical. Its own run-tests results from result-deps5b were not re-run.
- inputs-closure: 6 of 6 edits present; new `adapter/closure/module.ts` differs only by `with { type: "json" }` (peer edit 06:12); other new files identical.
- profile-apply (`lift-fix-tools/profile-apply.py`): `.cargo/config.toml` `[profile.*]` removed (live equals index, `M`). In A067 `inventory/🧪️tests/🟦️.ts` the `expect(manifest.profile).toBeUndefined()` edit is present, but the type cast `as {profile:{prototype:{inherits:string}}}` in the same test is still stale. One fixture string `[profile.prototype] inherits="dev"` also remains (test data, not a live Cargo profile). A087 and A067 were not run.

## Root commands
- `bun build --target=bun ./📜️script.ts`: FAIL (rc 1), unresolved `@semio-tech/framework-manifest/*` and `framework-transient` (missing node_modules links).
- `bun repo --help`: PASS (rc 0).
- `bun ./📜️script.ts verify zero-touch`: FAIL, `Cannot find module '@semio-tech/framework-transient'`.
- `bun ./📜️script.ts verify build-owners`: FAIL, same error.

## Other checks
- `[DEBUG]`: no new `[DEBUG]` introduced by applied targets (live vs HEAD counts). Three files went from 1-4 to 0. Existing `console.log("[DEBUG] ...")` lines in tests (for example `🎛️dashboard-control`, 13 at HEAD) are unchanged and pre-existing.
- AGENTS (heuristic, not exhaustive):
  - Path length: 333 applied targets, longest 165 characters, none over 256 bytes.
  - Docstrings: all 145 TS/Rust/Python targets with a docstring start with a non-ASCII (emoji) character.
  - Compat keywords: 2 hits, both false positives (test fixture field `legacy` and identifiers). Not a compat layer.
  - Line comments: 16 applied targets contain `//` line comments (159 lines total), mostly `os/🧱️elements/.../🧪️tests`. Not compared against HEAD. Needs a human pass.

## Open items
1. Restore node_modules workspace links (`bun repo dependencies sync`, after the write pause lifts). Most verify failures disappear with it, and the root build, zero-touch and tsc gates can be re-run.
2. Investigate the 148 missing exports-stage glue files (145 staged-added in git, deleted from the worktree). Check peer activity at 11:04 before re-applying.
3. Re-run cargo-pressure admission suite and the repo tsc gate after (1). The cargo-pressure "2 fails vs documented 1" needs triage.
4. Apply the not-applied exports-stage op (world3d-scene-shading in the react package script).
5. Clean up the stale `as {profile:{prototype:{inherits:string}}}` cast in A067.

Raw outputs: `🗑️generated/audit-verify-1020b/` (`live-vs-staged.txt`, `inventory.txt`, `diffs/`, `verify/`, `verify-summary.txt`, `verify-rerun.txt`).
