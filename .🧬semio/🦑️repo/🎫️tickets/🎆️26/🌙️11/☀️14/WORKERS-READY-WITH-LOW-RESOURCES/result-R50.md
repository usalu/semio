# R50: landing of the four prepared re-key packages (R29, R37, R39, R40), 2026-11-15 17:59 CET

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Landing entry: `T/landings.md` "Package R40 landing (2026-11-15 17:59)" plus my verification block under it. Originals for rollback: `T/🗑️generated/R40-landing-175904-73487` (`python3 -I T/🧰️scripts/R40/land.py --rollback <that dir>`). Scratch (outside the tree, 231 MB): `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R50/` (`rig.sh`, `runtests.sh`, `meas/` = measurement copies, `before-all.txt`, `after-all.txt`, `green.txt`).

## 1. Rebase
- The overlays are anchored edits (old text to new text, with expected occurrence counts) plus new files, not whole files. A "three-way merge" is therefore the anchor application on the live text, done by `land.py` inside the lock, re-reading each live file at that moment. Nothing overwrites a live file.
- Paths re-pointed (scripts only, no live source): `R39/edits.py` (default root), `R39/new/n03.ts` and `n06.ts` (the checkout candidate list of the two Rust-toolchain finders), `R29/R37/R39/R40 land.py` (root list now only the new root), `R37/payload_edits.py`, `R29/apply_edits.py`, `R29/payload.py`. The `lab/` measurement scripts keep the old path; I used patched copies in scratch.
- Against the current code: 15 of 16 live files equal their prepared baselines (R41, R42, R43, R45, R47 touched none of them); `script-inputs/🟨️.mjs` was edited by a peer after R29's baseline (14:23). All anchors still apply there, the merged file keeps the peer edit (diff vs live: 8 lines, only R29's own hunks). Inferrer stamp is still `script-inputs/4` live, so the combined window's `4 to 5` step is right (both rows now read `/5`).
- Dropped as superseded: nothing. Nothing of the four packages was already in the live tree. Not built by the packages themselves (so not landed): R40 part 1 (crate-granular dependency inputs: R40 proved it is already true in the records), manifest projection, the code tier of R39 (impossible with stock rustc), R37 groups c and d.

## 2. What landed (28 files: 14 edited, 1 merged, 13 new), one window
- R29: 26 tool rows, command lists name tools by reference (`provider-closures` json/schema/tests, `script-inputs/🟨️.mjs`, stamp rows, law test `🧮️blast-radius`).
- R37: `COMPACTION_HELPERS_V1` and `compactionHelperPathsV1` in `📥️inputs/🟨️.mjs` (+ `.d.mts`), law test `🧱️compaction-helpers` with its Python oracle.
- R39: interface identity in the unit-cache wrapper (`unit-cache/📜️script.ts`, new member `🔭️interface` with schema, tests, Python model, and `🧪️tests/🔭️cutoff`).
- R40: interface closure for `check` tasks (`📥️inputs/🔭️interface`, digest/read/inputs/native-manifests edits, granularity law test).
- The landing was done with `--no-tests` (the script runs about 35 test files twice in one go, over the 10-minute foreground limit at load 30 to 45); I ran the same lists by hand, one file at a time, before and after (below).

## 3. Tests
Staged-copy rig first (overlay of the 28 files over the live tree, bun preload redirect): R37 4 pass, R39 interface 28 pass and cutoff 4 pass, R40 interface 8 pass and granularity 1 pass, R29 blast-radius 3 pass (R29+R37 overlay alone, since in the full overlay the live walker cannot see the new unit-cache module; a rig artifact). The rig needed a MODULE.bazel, `📋️build-owners.json` and one `📋️project.json` copied into the overlay root.
Live, after landing, new tests: compaction-helpers 4 pass; unit-cache interface 28 pass; cutoff 4 pass; inputs interface 8 pass; blast-radius 3 pass; crate-granularity 1 pass (needs `SEMIO_TEST_CHECKOUT`, 99 s, compares 52 crates with `cargo metadata`; without that variable my loop showed it as 0 run, so it is skipped silently in that case).
Live, 31 existing files before and after the landing: identical failing sets, no new red. Red both before and after (not mine, not fixed): `📥️inputs/🧪️tests` 2 ("Cargo manifest inference projects the workspace sentinel...", "live inference bootstrap publishes the same closed project helper exports..."), `📥️inputs/📋️data/🧪️tests` 9, `unit-cache/🧪️tests/🌱️incremental` 1.

## 4. Measured re-key counts
Method: the R40 scripts (`dump`, `prune`, `oracle_meta`, `survey2`, `keysets`, `top10`, plus my `manifest.py` in scratch) on generation 237 (4 453 Rust owners, closure leaves of `build-input-closures`), Cargo's `cargo metadata` as the independent dependency oracle (4 452 packages, 1 workspace fails to load as in R40). The numbers reproduce R40's report exactly (top 10 table identical). "Before" = today's persisted nodes. "After" = the landed rule applied to the same persisted token membership (a model: see section 5).

| edit | check before | check after | build before/after | test before/after |
|---|---|---|---|---|
| body-only edit of the lib entry of `semio-framework-value-source-turn` (3 896 Cargo dependents) | 3 897 | **4** (the crate, `framework-rs`, `framework-schema`, `ui-contract-rs`) | 3 853 / 3 853 | 4 010 / 4 010 |
| same, crate with median fan-out (`semio-s-artifact-puzzle-5d-mut-any-5`) | 22 | **1** | 21 / 21 | 24 / 24 |
| same, 90th percentile fan-out (`2d-text-rs`) | 2 897 | **1** | 2 853 / 2 853 | 2 969 / 2 969 |
| same, crate without dependents (`2d-text-font-testing-rs`) | 4 | **1** | 3 / 3 | 10 / 10 |
| `Cargo.toml` edit of `value-source-turn` | 3 897 | 3 897 (unchanged) | 3 853 / 3 853 | 4 010 / 4 010 |
| `Cargo.toml` edit, median / p90 / no-dependents crate | 22 / 2 897 / 4 | same | same | same |
| interface edit of a `.rs` file | unchanged by design | | | |

Reading: only `check` of production dependents benefits, and only for body-only `.rs` edits. A manifest edit re-keys exactly what it re-keys today (the interface digest hashes every non-`.rs` file by bytes). `build` and `test` are unchanged by design (rustc code tier impossible, R39). R40 measured on 14 days of history that only 3.2 percent of crate edits are body-only, which gives -2.6 percent of check owner-stales; I did not repeat that replay.
For R39's unit-cache layer (a dependent unit of a `check` chain is a cache hit with rebased metadata) the proof is the real-Cargo law tests above (28 + 4 pass), not a count of owners.
R29 and R37 claims (nodes embed 123 434 to 15 992 file names; helper edit 5 043 to 0 owners) were NOT re-measured: they need nodes inferred by the landed code.

## 5. Unverified
- No graph refresh has run with the landed code: `inferProjectInputs` with the new `nativeInterface` group, the unchunked interface closure records, and Nx with `{input:"nativeInterface"}` are untested outside unit tests (R40 report items 1, 2). The "after" column above is a model on the old nodes, not a hash of a refreshed task.
- The inferrer stamp is now `/5`: the next graph read re-infers every project once and every Rust `check` key changes once (cold check pass). A read (runner 91760) started 18:14, after my landing, and is the first one on the new code; I did not look at its result.
- Design rule R40 needs the owner to accept: a cached `check` of X attests X against dependency interfaces, so a body-broken dependency leaves X's cached check green until X's key changes. Switch-off: remove `"check"` from `targets.names` in `📥️inputs/🔭️interface/🧬️schema/🔣️.json`.
- Graph-read survival: read pids 31945/31946 were alive before the landing, right after it, and still at 18:10. The record then changed (new read 91760 at 18:14, reason `request`); the old read's log is overwritten and I could not establish why it ended (R49 took the lock at about the same time). So: not shown to be killed by the landing, not shown to be unaffected either after 18:10.
- Workflow rule 3 of `design.md` ("one real `repo` build command through the changed path") was not run by me (no graph command allowed by the brief).
- Linux, Windows, release mode / opt-level above 0 for the interface check digest, remote-cache movement of the new closure leaves (R40 items 6, 7) are untested.
- The three pre-existing red test groups (section 3) are unexplained; they were red before this landing.
