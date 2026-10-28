# 🧪 audit-staged-1026g: staged gate-g batches (read-only)

Auditor aud-staged-g, 19:02. Read-only: only `check`/`--dry`/`--check` modes, sha256 base comparison and anchor counts; nothing applied, no builds, no git. Scripts `aud-staged-g-tools/{checks.sh,drift.py,overlap.py}`; raw outputs `🗑️generated/aud-staged-g/out/`.

Queue source: gate-g queue in `coordination.md` (12:22 numbered 1-8, 17:40 order, 18:53 restart) plus `STAGED`/`gate-g` announcements. The literal line `FOR gate-g: batch ready <path>` does not occur in coordination.md (only `FOR gate-g` at 13:53 and 14:27). Excluded as already landed per `result-gate-g.md`: zt-g, xplat-g, noop2+watch, phase-g, preadmit-g, keys2 (reduced), undeclared-g, ci-g, proof2 go-root-marker, e2eperf policy, rev3-g, keys3-g. derive-g is staged revert only.

## Per-batch state (17 staged units)

| # | Batch | Staged dir | Targets | Live state (check mode) | Verify | Shares files with | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | route2-g (q6) | `route2-g-tools/staged`, `staged-lib`, `staged-base.json` (56 keys) | 56 | `apply-staged.mjs --dry`: 42 would apply, 0 CHANGED. Manifest check: 44 base, 10 already live, **2 CHANGED**: `lib:⚡️caching/.../📡️session/🟦️.ts` (live 12:23) and `🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🔣️.json` (live 12:46). Both have no staged copy, so the apply tool skips them silently | `verify-staged.sh` | laws-6b (build suites json), testmem2 (course-inputs test) | **DRIFTED 2** (apply the other 54) |
| 2 | laws-g 6b (q6b) | `laws-g-tools/` (no staged dir) | 5 | `apply-build-tools.py --check`: 11 suites to change, clean. `route-unrouted.py --check`: **STALE**, AssertionError (caching `📋️project.json` no longer has `"test-lease-bootstrap": {`, file 13:28). `dedupe-tables --check`: nothing to do. `oracle-required --dry`: 0 files | none (`run-list.sh` is a runner) | route2-g (build suites json, must land after it) | **DRIFTED/STALE** (route-unrouted) |
| 3 | store-g (q7) | `store-g-tools/staged` (2 apply scripts, `managed/*.ts`, README) | 15 + 3 new test copies | `apply-foldsafe-preflight.py --check`: 14 anchors unique. `apply-managed-parent-guard.py --check`: 8 anchors ok | `ft.sh` (runner only) | setup-g (3 preflight files) | clean |
| 4 | self-g sealed flip (q8, LAST) | `self-g-tools/flip.py` (no staged dir) | 1 (`🏛️sealed/🔣️.json`) | anchor `"defaultMode": "live"` count 1, sealed 0: clean. flip.py takes a cwd-relative path, run from repo root | `post-flip.sh` | none | clean, last |
| 5 | gram2-g index-chunks | `gram2-g-tools/staged/index-chunks/edits.json` | 3 files (7 edits) | 7/7 anchors unique | `verify.sh` | rev3fix (3 generations files) | clean |
| 6 | gram2-g row-table | `gram2-g-tools/staged/row-table` (`batch.py status`) | 12 (10 edits in base, 12 new absent) | clean | `verify.sh` | none | clean |
| 7 | walk-g walks | `walk-g-tools/staged/walks/edits.json` | 7 files (29 edits) | 29/29 anchors unique | `roundtrip.sh` only | walk-footprint (README.md) | clean |
| 8 | walk-g footprint | `walk-g-tools/staged/footprint/edits.json` | 8 files (26 edits) | 26/26 anchors unique | `roundtrip.sh` only | rev3fix (2 host-capacity files), testmem2 + nxfix-core (`📓️build-guide.md`), walk-walks (README.md) | clean |
| 9 | nxfix-g core | `nxfix-g-tools/staged/core` (manifest 13) | 13 | `stage.py check --batch core`: 13 pending; by sha: 11 base + 2 new absent | `verify.sh` | nxlock (policy.json, leases test), testmem2 + walk-footprint (build-guide.md) | clean |
| 10 | nxfix-g nx-patch | `nxfix-g-tools/staged/nx-patch` (manifest 2) | 2 | 2 pending, base match | `verify.sh` | none | clean |
| 11 | nxlock-g | `nxlock-g-tools/staged/nxlock` | 12 | 10 pending, 2 `needs-core` (lands after nxfix core) | `verify.sh` | nxfix-core | clean, ordered |
| 12 | obsrows-g | `obsrows-g-tools/staged` (`🏗️build/👁️observation`, `🧪️testing/🌱️genuine-birth`) | 11 | `stage.py check`: 10 base, 1 new absent | **missing** (`run.sh`/`chain.sh` only) | none | clean |
| 13 | rev3fix-g | `rev3fix-g-tools/staged/rev3fix` (edits 51, newFiles 9, bases 18) | 27 | **DRIFTED 3**: base hashes of `generations/🟨️.mjs`, `generations/🟨️.d.mts`, `graph/🟦️.ts` changed 18:46 (rev3-g GROUP 2 apply). Anchors still 51/51 unique, new files absent. `batch.py check` reports "before" and never reads `bases.json`, so the drift is hidden | `verify.sh` | gram2 index-chunks (3 generations files), walk-footprint (2), conv-g (unit-cache `🏊️pool/🟦️.ts`) | **DRIFTED 3**, rebase before landing |
| 14 | setup-g | `setup-g-tools/staged` (`stage.py`, `build-overlay.py`, `base/`, `overlay/`, `index-edited.json`) | 12 | 12/12 base hashes; `build-overlay.py --check`: 12 anchors ok | **missing** (`stage.py check` only) | store-g (3 preflight files) | clean |
| 15 | testmem2-g | `testmem2-g-tools/staged` (`batch.json` 10, `stage.py`, `files/`) | 9 + 1 new | `stage.py check`: 9 base, 1 new absent | `verify.sh` (zsh) | route2 (course-inputs test), walk-footprint + nxfix-core (build-guide.md) | clean |
| 16 | xplat3-g | `xplat3-g-tools/staged/files`, `bases.tsv` (17) | 17 | 17/17 base sha256 | **missing** (`overlay.py` only) | none | clean |
| 17 | conv-g | `conv-g-tools` (`base/`, `new/`, `base-hashes.json` 40) | 40 | 40/40 base sha256 | **missing** (`build.py`, `ops.py`) | rev3fix (`🏊️pool/🟦️.ts`) | clean |

Summary: 14 clean (nxlock is clean but waits on nxfix core), 3 drifted or stale (route2-g 2 files, laws-g route-unrouted stale, rev3fix 3 base hashes). Every check mode read-only: verified by reading the code paths (`setup build-overlay` exits before any write, `apply-build-tools`/`route-unrouted`/`dedupe` write only when not `--check`, `oracle-required` only without `--dry`).

## Verify scripts

- Missing: obsrows-g, setup-g, xplat3-g, conv-g.
- Runner or partial only (no verifier that checks results): laws-g (`run-list.sh`), store-g (`ft.sh`, README steps), walk-g (`roundtrip.sh`).
- Present: route2-g, gram2-g (both), nxfix-g (core, nx-patch), nxlock-g, rev3fix-g, testmem2-g, self-g (`post-flip.sh`).

## Overlaps and order

| Pair | Shared files | Required order |
|---|---|---|
| route2-g x laws-6b | build `🗂️suites/🔣️.json` | route2 before laws |
| route2-g x testmem2 | build `🎓️course-inputs/🟦️.ts` | route2 before testmem2 |
| setup-g x store-g | build `🩺️preflight/🟦️.ts`, `🧪️tests/🧮️matrix/🔣️.json`, `🧬️schema/🔣️.json` | setup before store (setup's base hashes break after store) |
| nxfix-core x nxlock | `🧭️order/🔣️policy.json`, `🧭️order/🧪️tests/🟦️.ts` | nxfix core before nxlock |
| gram2 index-chunks x rev3fix | 3 generations files | gram2 before rev3fix (rev3fix re-based) |
| walk-walks x walk-footprint | `README.md` | walks before footprint |
| walk-footprint x rev3fix | 2 host-capacity files | walk-footprint before rev3fix |
| conv-g x rev3fix | `🏊️pool/🟦️.ts` | conv-g before rev3fix |
| nxfix-core, walk-footprint, testmem2 | `📓️build-guide.md` | text anchors, serial landing |

## Disjoint groups proposal (max 3 per group)

- G1: obsrows-g, xplat3-g, gram2 row-table (no shared files)
- G2: nxfix-core, nxfix-nx-patch, conv-g
- G3: route2-g (without its 2 CHANGED entries), gram2 index-chunks, walk-g walks
- G4: laws-g 6b (route-unrouted re-staged), setup-g, walk-g footprint
- G5: store-g (after setup-g), nxlock (after nxfix core), testmem2 (after route2, footprint)
- G6: rev3fix (singleton; rebase the 3 drifted bases first; after gram2, footprint, conv-g)
- G7: self-g sealed flip alone, last

## Actions for gate-g

1. route2-g: drop the 2 manifest entries without staged copies, or re-stage them; fix `apply-staged.mjs` to fail on manifest entries with no staged file (today it skips them silently).
2. laws-g route-unrouted: re-stage against the current caching `📋️project.json` (anchor gone). apply-build-tools can land after route2.
3. rev3fix-g: re-base the 3 files changed at 18:46 (rev3-g GROUP 2) and make batch.py check compare `bases.json`.
4. Add verify scripts for obsrows-g, setup-g, xplat3-g, conv-g before landing.
5. Unannounced staged dirs found but not audited: `jobsrv-g-tools/staged` (18:43, 4 entries), `gram3-g-tools/staged` (18:46, 1 entry). `obs-g-tools` has tools only.
