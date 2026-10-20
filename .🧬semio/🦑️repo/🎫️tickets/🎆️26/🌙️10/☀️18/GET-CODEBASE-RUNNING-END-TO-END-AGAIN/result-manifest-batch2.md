# Result manifest-batch2 (executor manifest-batch2, 2026-10-20 11:00 - 12:45)

`manifest-batch2.READY` written 12:42. Lock is green; sync NOT run by me.

## 1. What was applied (live tree)

| step | outcome |
| --- | --- |
| cycle-break delta (471 manifest ops, 16 new files, 1144 edits, 243 file ops) | guard pass refused first: 5 sources changed since staging, all by the json-attribute codemod (`with { type: "json" }` added at 06:12). `manifest-batch2-tools/resha.ts` re-pinned the 5 sha256 (guarded: live text minus the attribute must equal the staged hash). Guard clean, `--write` applied 11:04, 846 files. Backup `🗑️generated/cycle-break-x/applied-delta-2026-10-20T09-04-20-576Z`. Original delta kept as `🗑️generated/manifest-batch2/cycle-break-delta.orig.json`. |
| lay17 delta (13 SDK export entries, 15 glue files, 2 glue patches, 39 files rewritten) | After cycle-break one rewrite was superseded (fem shared-authority `color-input` now `@semio-tech/ui-contract/testing/color-input` from cycle-break). Dropped entry E0165 from `lay17-tools/staged/batch-delta.json` (original in `🗑️generated/manifest-batch2/batch-delta.orig.json`). Dry run 0 problems, applied. No key collision with the carved packages. |
| lay17 follow-ups | `policy-renderer-hosts.py --apply`, `script-removal.py --apply`, `vitest-owners.ts --write` (owners 55 to 53). |
| defects found while merging | (a) `puzzle-5d-ts` had no workspace member entry: `✏️s/package.json` depended on it but did not list the path. Added `🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🟦️typescript` with the planner's sorted insertion (`manifest-batch2-tools/add-member.ts`). (b) One dynamic importer of a removed key was missed by the carve: `📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts` imported `@semio-tech/framework/products/repo/modules/dashboard/installation`; now relative to the moved module under `⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation`. |

## 2. Re-validation of the merged manifests (live tree, `manifest-batch2-tools/validate.ts`, `exports-exist.ts`)

- 318 workspace members over the 6 roots, 318 unique names, every member has a manifest.
- Targets: all `workspace:*` edges resolve to a member (0 unresolved after fix (a)).
- No alias or non-registry specs (`npm:`, `git`, `file`, `link`, `http`); the delta has only `workspace:*` additions, external specs of the 3 new manifests with externals (`repo-server-library`, `repo-product`, `ui*`) all exist in the product locks.
- Production-graph cycles: exactly one pair (`print`, `print-viz-inference`), as documented in `result-cycle-break.md` 2.1.
- Export targets: 2204 checked, 94 missing, all `dist/` or `bindings/` build outputs (pre-existing, generated).
- Versions come from the lock; the delta adds no external version.

## 3. Lock

`bun repo dependencies lock --all` under `perl -e 'alarm 590'`: runs 1-4 (11:18 - 12:04) hit SIGALRM in the ownership walk (13.4 M operations; load average 45-66 from the cargo fleet); run 5 detached with `alarm 3000` (12:04), finished `rc=0` 12:37 ("Store dependency lock complete", `completedLocks":6`). It waited ~9 min on `native-fixed-image .cache/b/d` behind a queued shared request of pid 65212 (`repo run repo-mcp:build --skip-cache`, a peer). Logs `🗑️generated/manifest-batch2/lock1..5.log`.

All six `bun.lock` (root, `✏️s`, `♻️mit-bestand`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`) contain every package created at 11:04: plugin-registry-publication, repo-server-library, repo-product, framework-hash, -trace, -interaction, -transient, -filesystem, -onboarding, -manifest, -action-argument-resolution, -effect, ui-contract, puzzle-5d-ts (the last only in the `✏️s`-containing locks). node_modules links appear with the sync (root build, `verify zero-touch`, `verify build-owners` fail until then, as expected).

## 4. Coordinator item: "148 glue files missing"

Not a sweep. 145 of the 148 are the cycle-break delta's deliberate deletions: exports-stage entries whose key moved to a carved package (list: `cycle-break-supersedes.json`, 145 entries / 145 glue, every old key is gone from `@semio-tech/framework` exports, every new key exists in the carved package, no glue file remains, no dangling export target). Do NOT re-run the exports-stage creates for them (that would recreate dead glue for removed keys); after the sync run `exports-stage-tools/stage.ts` for the residue (supersede note in the json). The other 3 are the three never-applied "blocked" entries of `result-cycle-break.md` 3.3 (`energy-js ./extensions/epw/testing/unit`, repo-product coverage oracle glue under `📦️packages/…/🎛️dashboard/🧪️testing`, `repo-lib ./build/script`): their keys are not in any live manifest, so nothing dangles. Nothing treats glue as removable.

## 5. Scan and cruise after apply

`manifest-batch2-tools/scan.ts` = lay17 scan with package discovery from the workspace manifests (so unlinked packages resolve) plus an unresolved-`@semio-tech` report; policy dumped from the live library policy. 16,667 files.

| rule | lay17 end (mirror, no cycle-break) | live now |
| --- | ---: | ---: |
| `ui-no-framework-packages`, `ui-foundation-closure` | 10 + 3 | **0 + 0** (family absent from the scan) |
| `plugins-framework-sdk-only` | 0 | **37** |
| `renderer-hosts-only-ui` | 0 | **21** |
| `plugin-no-extension-or-artifact` | 4 | 4 |
| `no-cross-technology` / `framework-no-products` | 21 / 13 | 19 / 8 |
| `no-core-path` (new in the policy dump, peers' `🟣️shared` test folders in ui Select and pets; not from this batch) | n/a | 51 |

Unresolved `@semio-tech` specifiers: 14, all generated outputs (storybook wasm pkg, `dist` of artifact packages, `…-rs/pkg`); none from this batch.

The per-area dependency-cruiser cruise was NOT run: it resolves through `node_modules/@semio-tech`, where the 14 new packages are not linked until the sync, so it would report false unresolved edges. The scan is the stand-in; run the cruise after the sync.

### 5.1 CONFLICT between the two deltas (needs a decision)

All 58 `plugins-framework-sdk-only` + `renderer-hosts-only-ui` violations are introduced by cycle-break specifier rewrites (verified: every violating import is an edited specifier of the delta). The carve moved SDK keys (`@semio-tech/framework/{manifest,hash,interaction,trace,ui/targets/react,ui/styling,ui/scene,…}`) to module-root packages and re-pointed plugin and renderer-host importers to the carved packages, which the rules forbid (plugins: only the SDK package; hosts: only ui, framework-core protocol types). The manifest batch itself is consistent and unaffected; this is policy versus carve. Options:

1. Policy (recommended, no manifest change, matches the carve's no-glue design): admit the carved neutral packages that superseded SDK keys. Derive it from data, e.g. `publicApi` gets a list of SDK foundation packages (or the allowlist is the export targets of the 145 supersede entries for plugins, and trace/interaction/manifest/hash/ui-react/ui-styling/ui for hosts via the existing `ui-foundation` owner paths). Needs the closed `publicApi` check (line 7 of the construction `🟨️.cjs`), its schema and fixture, and a test.
2. Glue: re-add the SDK keys in `@semio-tech/framework` as glue to the carved packages (reverses the carve for those keys, adds framework to carved package dependencies, a lock refresh).

I did not change policy: it is a library edit that the sync loads, and the lock needed a stable tree. Do it before the sync starts, or accept one more restart.

## 6. Tools and kept inputs

`C/manifest-batch2-tools/`: `resha.ts`, `add-member.ts`, `validate.ts`, `exports-exist.ts`, `scan.ts`. Generated (delete with the folder): `🗑️generated/manifest-batch2/` (policy dump, scans, lock logs, original deltas), `🗑️generated/cycle-break-x/` backup (keep until the sync/tests are green).

Files touched outside C: everything of the two deltas, `✏️s/package.json` (member), `📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts`, plus the lay17 policy/vitest-owners/script-removal sets, and the six `bun.lock`.

## 7. Not run

Sync; dependency-cruiser cruise; tsc over the applied edits (`cycle-break-tools/tsconfig.d2.json`, `d6.json`, needs node_modules links for the new packages); `bun build 📜️script.ts`; the law suites (resolved-boundary, workspace-exports, repo-source-ownership, manifestless-source-closure, `vitest-configuration-ownership`). All follow the sync.
