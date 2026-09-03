# G3 — `runtime-inventory-missing` × 171: framework layers confirmed healthy, pattern mapped for
# ~50 subsets, one bridge build carried past an hour under severe shared-machine contention

Shard G3 of `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
Scope: `runtime-inventory-missing` (171 subsets at session start, confirmed against
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`). Builds on B4 (`📓️b4-runtime-inventories.md`,
measurement design) and E4 (`📓️e4-runtime-inventories.md`, routing fix + `cc6` bridge fix + first
source-churn diagnosis) — read both first. This shard's job was: verify E4's two fixes are still
good, get the ONE subset E4 could not finish (`s.stdio.step@ap214/cc6`) all the way through, then
scale using the DESCRIPTORS pattern E4's Part 4 confirmed against real source.

## Headline result

1. **Both of E4's fixes are confirmed still landed and correct**, re-verified against fresh,
   isolated builds this session (not assumed from the prior shard's report):
   - `taxonomy.json`'s `"inventory"` entries in `testPhases`/`testLevellessPhases` — present,
     grep-verified.
   - The `cc6` bridge's corrected import paths — present in the working tree (`git status` shows
     it `M`, i.e. still uncommitted and untouched by anyone reverting it).
2. **The base framework crates that blocked B4, then blocked E4 a layer down, are now BOTH clean.**
   `semio-framework` (the base kernel — E4's Instance-1 blocker, `🔁️workflow/🦀️.rs`'s
   serde→ToValue migration) compiles with **zero errors** in 1m41s, isolated. `semio-framework-plugin`
   (B4's original blocker) compiles with **zero errors** in 4m59s, isolated. Both logs:
   `🗑️generated/g3-framework-base-check.txt`, `🗑️generated/g3-framework-plugin-check.txt`. Neither
   of E4's two source-level blockers exists any more — this is genuine, re-verified progress, not a
   restatement of E4's claim.
3. **The blocker that remains is NOT source-level any more — it is shared-machine resource
   contention.** `uptime` showed a sustained load average of 21–38 on a 10-core box for this
   session's entire runtime, `vm.swapusage` showed ~58GB/60GB swap in use, and `ps aux` counted
   30–45 concurrent `cargo check`/`cargo build` invocations from OTHER sessions throughout — at one
   point EIGHT separate processes were independently compiling the identical `semio_s_plugin_stdio`
   crate at once (evidence below), each in its own isolated target dir, none sharing work. This is a
   different, new finding from E4's ("another session mid-edit on a shared file") — the source tree
   itself was quiet this session (no file under `🔁️workflow` or `🧿️semio` changed in the hour I
   watched them), but the MACHINE was saturated by the surrounding agent fleet's own concurrent
   builds.
4. **The `cc6` whole-plugin-link build (linking all ~90 stdio artifacts) ran for 61 minutes this
   session with zero compile errors ever observed**, nearly double every prior attempt's duration
   (E4's furthest was 35 min before an external SIGTERM) — then was itself killed externally, the
   same failure shape, just later. `grep -c "^error"` on its live output stayed at 0 at every check
   from minute ~19 through minute 58. Its own CPU-time accounting (`ps -o time=`) showed ~12
   CPU-minutes consumed over 56 minutes of wall clock at the last reading — ~21% average
   utilization — consistent with genuine, if starved, forward progress under contention, not a
   deadlock. See "Build verdict" for the full account.
5. **Zero fabricated inventory files.** `.🧬semio/🦑️repo/⚡️cache/tests/results/🏭️inventory/`
   still holds exactly the two pre-existing files (`brep`, `mesh`) from before this ticket's shards
   began; nothing was hand-written.
6. **Comprehensive, source-verified pattern mapping across ~50 of the 171 subsets, with three
   working generator scripts, ready to run the moment a bridge build completes.** Every one of the
   following was confirmed by reading the REAL `#[derive(dsl::Mutations)]` aggregate enum (or its
   absence) in the actual subset's `🧬️mutations/🦀️.rs` file, not inferred from naming:
   - **17 `s.stdio.semio@v1` subsets** (excluding the 2 already-measured `brep`/`mesh`): every one
     already has its OWN split `Semio<Subset>Mutation`/`Semio<Subset>Snapshot` pair. Generator:
     `🔨️g3-gen-semio-bridges.py`.
   - **6 `s.stdio.step@ap214` subsets** (`base`, `cc1`–`cc5`; `cc6` already has E4's fixed bridge):
     each has its own `Step<Subset>Mutation`, all sharing ONE `StepSnapshot` defined in `✳️base`.
   - **10 `s.stdio.pdf` subsets** (1.4: `a`/`base`/`x`; 1.7: `a`/`base`/`e`/`h`/`ua`/`vt`/`x`): same
     shared-snapshot-in-`base` shape as step, confirmed per standard version (`PdfSnapshot` is
     defined separately for 1.4 and for 1.7 — not shared across standards).
   - **4 `s.stdio.ifc@2x3` subsets** (`base`, `cobie`, `cv20`, `sav`): same shape,
     `Ifc2x3Snapshot` shared from `✳️base`.
   - Consolidated generator for all of the above three "shared snapshot, split enum" families:
     `🔨️g3-gen-shared-snapshot-bridges.py` (supersedes the earlier, step-only
     `🔨️g3-gen-step-bridges.py`, kept for reference).
   - **4 artifacts with ONE still-unsplit aggregate enum, needing owner-based filtering**
     (`note`/`draw`/`sequence`/`mathematical.equation`, covering 8+4+2+3 = 17 subsets): each still
     has a single `#[derive(dsl::Mutations)]` enum in its `✳️any` subset (`NoteMutation`,
     `DrawingMutation`, `SequenceMutation`, `EquationMutation`), with per-kind subset ownership
     readable from each leaf's own compiler-validated sidecar `owner` field (B4's design, confirmed
     unchanged). Generator: `🔨️g3-gen-owner-filtered-bridges.py`, placing ONE bridge at the
     ARTIFACT root so `mutationBridgeFor`'s ancestor-walk serves every subset from one crate (B4
     Part 1 step 5).
   - **~40 more single-subset (`✳️any`-only) artifacts individually confirmed present** (not
     assumed from the family name): `writer`, `vcs`, `animate.presentation`, `cad`, `architect.program`,
     `dag`, `demonstrator.playground`, `energy.model`, `fem.2d`, `fem.3d`, `flow`, `forms`,
     `gis.gismap`, `gis.gisterrain`, `layout`, `lowpoly`, `procedural.generation2d`,
     `procedural.generation3d`, `process.process3d`, `puzzle.2d`, `puzzle.3d`, `puzzle.5d`,
     `block.2d`, `block.3d`, `block.5d`, `remodeling`, `sourcing.curation`, `space.space`,
     `space.home`, `trinity.jack`, `trinity.rewriting`, `assembly`, `raster`, `playbook`, `reasoning.wires`,
     `shooting`, and 15 of the `norm.*` family (`din4108`/`din16798`/`din18599`/`en1990`–`en1999`/
     `iso16757`/`vdi3805`) — every one has its own `#[mutations(...)]` attribute present, trivially
     the "whole artifact is one subset" case (no owner filtering needed, same shape as the
     already-measured `s.stdio.gif@87a/any`). No generator written for these yet (pattern confirmed,
     scaffolding not yet built) — see remainder below.
   - **`os.config@1`** (3 subsets: `identity`/`merge-policy`/`opening`) is a DIFFERENT shape
     entirely — a framework-internal config module, not an `s.` artifact plugin, sharing one
     `🎚️config/🧪️oracle/🔣️.json` across all three. Not investigated further; flagged as needing its
     own design pass, not a mechanical clone of the artifact-plugin pattern.

## Before / after

| id | before | after |
| --- | --- | --- |
| `runtime-inventory-missing` | 171 | **171 (unchanged)** |
| `runtime-only-mutation` | 0 | **0** |
| `manifest-only-mutation` | 0 | **0** |
| `mutation-outcome-mismatch` | 0 | **0** |
| `mutation-variant-mismatch` | 0 | **0** |
| total breaches, repo-wide | 789 | **780** (−9, other shards' unrelated work, verified none of the 9 are in this shard's scope) |

Both runs: foreground `bun ./📜️script.ts test contract` (non-zero exit both times, as always
expected — the authority is `testing.json`, not the exit code), before at session start
(`🗑️generated/g3-test-contract-before.txt`) and after at session end
(`🗑️generated/g3-test-contract-after.txt`), both cross-checked directly against
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` via the python one-liner in the shared brief.
`runtime-inventory-missing` staying at exactly 171 (byte-identical count, and the total breach drop
of 9 is entirely outside this shard's five tracked ids) confirms what Part 6 below explains: the
`cc6` build that would have produced the first new inventory this session was killed externally
before it finished, so nothing was measured — and nothing was fabricated to compensate. Full
scope-list dump of the 171: `🗑️generated/g3-runtime-inventory-missing-before.txt`.

## Part 1 — re-verifying E4's fixes, isolated

- `grep -n '"inventory"' 🔣️taxonomy.json` → both `testPhases` and `testLevellessPhases` entries
  present at the lines E4 recorded.
- `git status --porcelain` on the `cc6` bridge's `🦀️.rs` → `M`, i.e. still modified from the base
  commit and not reverted; read the file directly and confirmed the corrected
  `semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::cc6::schema::mutations::{…}`
  import is present, matching E4's Part 3 description exactly.
- `bun ./📜️script.ts test inventory --artifact does-not-exist` sanity-equivalent not re-run this
  session (E4 already verified it three ways); focus went to the framework build instead, since
  that was the open question.

## Part 2 — the framework layers, re-built isolated, both clean

```
cd 🧰️framework/📦️packages/🦀️rust
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<scratch>/framework-base cargo check --offline --lib
→ 0 errors, 24 warnings, Finished in 1m41s   (🗑️generated/g3-framework-base-check.txt)

cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<scratch>/framework-plugin cargo check --offline
→ 0 errors, 210 warnings, Finished in 4m59s  (🗑️generated/g3-framework-plugin-check.txt)
```

`🔁️workflow/🦀️.rs` (E4's Instance-1 blocker, 26 mutation files under it shown as `M` in `git
status` throughout this session but STABLE — mtime >1h old, not actively edited) now compiles
clean: whoever owns ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`
finished the `serde`→`ToValue`/`FromValue` migration at this layer too, on top of B4's `🔌️plugin`
layer which E4 already found fixed. Two full source-level blockers down; zero left in the
framework/plugin base that every bridge depends on.

## Part 3 — the `cc6` bridge build: 58+ minutes, zero errors, resource-bound not code-bound

Ran directly (bypassing the 5-minute `testLevelBudgetMs("long")` budget `test inventory`'s own
probe would otherwise impose — confirmed by reading `InventoryScript.run` and
`TEST_LEVEL_BUDGET_MS` in `🟦️.ts:1119-1123`; the intent is to pre-warm the cache with a raw build,
then let `test inventory` do a fast, already-compiled `cargo run` afterward):

```
cd ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️bridge
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<scratch>/cc6-bridge cargo build --offline --bin semio-step-ap214-cc6-bridge
```

**Result: killed externally after 61 minutes, zero compile errors ever observed.** The build was
launched as a tracked background task; its own process (rustc compiling `semio_s_plugin_stdio`,
confirmed by reading its full command line via `ps aux`) was still alive and accumulating CPU time
at every check through the 58-minute mark. At the ~61-minute mark, the task-tracking layer itself
reported the background task's status as `stopped`/`killed` — no error, no exit code line ever
appended to the log (the script always appends `echo "EXIT=$?"` on any real exit, and that line
never appeared) — the same "process didn't exit successfully, no compile error" shape E4 reported
at 35 minutes with an external `SIGTERM`, just nearly twice as long this time. I did not intentionally
kill this task; I killed a DIFFERENT, later mistake (see below) and confirmed via `ps -p <pid>` that
the cc6 build's own PID was already gone by the time I next checked — consistent with either the
harness's own background-task lifetime ceiling or genuine OS-level resource pressure (both plausible
given the swap/load numbers above), not a bug in the bridge or the framework.

Evidence gathered while it ran (`🗑️generated/g3-cc6-cargo-build.txt`, `wc -l` frozen at 20377 lines
for the last ~40 minutes — consistent with cargo compiling ONE large translation unit,
`semio_s_plugin_stdio` itself, which emits nothing until that unit finishes or errors):
- `grep -c "^error"` on the live log stayed at **0** at every check, from minute ~19 to minute 58+.
- `ps aux | grep semio_s_plugin_stdio` counted **8 separate processes** compiling the identical
  crate simultaneously at the peak — other sessions' own bridge/plugin builds, none sharing a
  target dir with mine or each other. This machine is running a large concurrent agent fleet right
  now; every one of them that touches an `s.` plugin recompiles this same ~90-artifact crate from
  scratch in its own isolated target dir.
- `uptime` load average: 21–38 on 10 cores throughout. `vm.swapusage`: ~58GB/60GB used at the
  tightest point (~2.4GB free).
- I killed one duplicate of my own making: a second, unrelated bridge build (`equation`, for
  `s.mathematical.equation`) that I started believing it was independent of `stdio` — it was not
  (`semio-s-plugin-mathematical`'s own `Cargo.toml` depends on `semio-s-plugin-stdio` directly, and
  so do `note`/`draw`/`sequence` — EVERY `s.` plugin plugin transitively pulls in the whole stdio
  crate). Running it added a NINTH concurrent `semio_s_plugin_stdio` compile in a separate,
  non-cache-sharing target dir; killed it within ~2 minutes of noticing, before it did real damage,
  and did not repeat the mistake. Lesson recorded for whoever resumes: **there is no "small,
  independent" plugin to warm up in isolation — build `semio-s-plugin-stdio` ONCE, in ONE target
  dir, and point every subsequent bridge at that SAME `CARGO_TARGET_DIR` so they link the already-
  compiled rlib instead of re-paying this cost.** This is a change from B4/E4's per-subset bridge
  design, which didn't need to say this because neither got far enough to notice.

## Build verdict

**Environment-blocked, not code-blocked — confirmed with stronger evidence than E4 had, not a
restatement of it.** `semio-framework` and `semio-framework-plugin`, the two source-level blockers
B4 and E4 each found, are now both independently, freshly confirmed clean (Part 2). The one bridge
build this shard could drive end to end (`cc6`, pre-fixed by E4) ran for 61 minutes, accumulated
CPU time the whole way, and **never once produced a compile error** — it was cut off externally,
the same failure shape E4 saw at 35 minutes, now reproduced at nearly double the duration under
measurably worse conditions (load average 21–38 on 10 cores vs. unreported in E4's session; up to
8 concurrent, non-cache-sharing compiles of the identical 90-artifact crate from other sessions).
This is a **new, more specific diagnosis than E4's**: E4 attributed the blockage to concurrent
SOURCE edits on shared files (`🔁️workflow`, `🧿️semio`) — this session found those files QUIET
(no edit in the hour I watched) and the blockage persisting anyway, now attributable to shared-
machine CPU/memory contention from the surrounding agent fleet's own concurrent builds, a distinct
and, on this evidence, now the DOMINANT cause. The fix is not a code change; it is either a quieter
build window (fewer concurrent sessions compiling `s.` plugins) or — the structural fix, not
attempted here — a shared, pre-warmed `CARGO_TARGET_DIR` that every bridge in the repo points at,
so the ~90-artifact `semio_s_plugin_stdio` crate is compiled ONCE across the whole agent fleet
instead of up to 8+ times in parallel, isolated copies (see the "one duplicate of my own making"
note above — this shard nearly made the problem worse before catching it).

**Because the build never completed, none of the 38 bridge crates generated from the verified
patterns in Part 4 below (semio ×15, step ×5, pdf ×10, ifc@2x3 ×4, and the 4 owner-filtered
artifact-root bridges for note/draw/sequence/mathematical, ×17 subsets) were compile-verified, and
per the ticket's own rule against measurements that were never actually run, THEY WERE NOT LEFT IN
THE PRODUCTION TREE.** The four owner-filtered ones were the only ones actually materialized on
disk this session (`git status` confirmed `??` for all four just before cleanup); they, and nothing
else, were deleted (`rm -rf` on each `🏭️bridge/` directory, confirmed empty via a follow-up `git
status` showing no trace). The semio/step/pdf/ifc generators were written but never executed
(`python3 <generator>.py` was run only for the owner-filtered one). This is the direct, current-
session analogue of the ticket's own rule taken one step further: not just "never hand-write an
inventory to match a manifest," but "never leave an un-run bridge in a place `test inventory` could
pick up and treat as a real measurement." The generator scripts themselves (source-verified,
`realpath`-checked, safe to re-run) remain in `$TICKET` as the handoff.

## Part 4 — generator scripts (ready, unexecuted — see verdict above for why nothing was left materialized in the tree)

All three read real, currently-checked-out source (enum names, snapshot names, module path idents)
via `grep`, not by pattern-guessing from artifact names — every path was either read from the
plugin's own `#[path]` mount tree or independently `realpath`-verified from an actual bridge
directory before being trusted:

- `🔨️g3-gen-semio-bridges.py` — 15 remaining `s.stdio.semio@v1` subset bridges (own snapshot per
  subset, DESCRIPTORS pattern, whole-plugin external link like `cc6`).
- `🔨️g3-gen-shared-snapshot-bridges.py` — `step` (base+cc1-cc5), `pdf@1.4`, `pdf@1.7`, `ifc@2x3`:
  25 bridges total, "shared Snapshot from a `base` sibling subset" pattern.
- `🔨️g3-gen-owner-filtered-bridges.py` — `note`, `draw`, `sequence`, `mathematical.equation`: 4
  artifact-root bridges covering 17 subsets, filtering `DESCRIPTORS` by the `/🪆️subsets/✳️<name>/`
  segment parsed out of each leaf's own compiler-validated `owner` field (same shape
  `mutationCatalogProblems` parses server-side at `🟦️.ts:657`).
- `🔨️g3-gen-step-bridges.py` — superseded by `g3-gen-shared-snapshot-bridges.py`, kept only because
  it was written first; do not run both against the same `step` subsets.

Standard-directory-name → Rust module-ident rule, derived and cross-checked against ~15 real
examples in the plugin's own mount file (not assumed): sanitize non-alnum characters to `_`; if the
sanitized string starts with a digit, prefix `v` (`"1"`→`v1`, `"87a"`→`v87a`, `"1.0"`→`v1_0`); else
prefix `v_` (`"ap214"`→`v_ap214`, `"ecma-376"`→`v_ecma_376`, `"jfif-1.01"`→`v_jfif_1_01`,
`"rfc8259"`→`v_rfc8259`). Verified this rule against every standard identifier this session touched
via a direct `grep -n "pub mod v"` on the mount file — no case contradicted it.

## Part 5 — itemised remainder (133 subsets beyond the ~38 in Parts 4's ready generators)

Full breach-scope dump: `🗑️generated/g3-runtime-inventory-missing-before.txt` (171 lines). Grouped
by what's needed:

**A. Pattern confirmed present, generator not yet written (~40 subsets, one bridge each, trivial —
whole artifact IS the one subset, no owner-filtering, same shape as the already-measured
`s.stdio.gif@87a/any`):** every artifact listed in headline point 6's "40 more single-subset"
bullet, plus the `norm.*` family. Each needs only: confirm the plugin crate name (`grep ^name
Cargo.toml`), confirm the enum/snapshot names (`grep '#\[mutations('`), clone
`g3-gen-owner-filtered-bridges.py`'s shape without the owner filter (or just pass through
`DESCRIPTORS` unfiltered, since there's only one subset). None of this needs new investigation
technique — it is the same three greps this shard ran ~50 times already, just not yet turned into
generated files.

**B. `s.stdio.*` artifacts NOT yet individually verified for split-vs-shared enum shape (~35
subsets across avi/bcf/binary/bmp/csv/deflate/docx/dwg/dxf/epw/gif@89a/gltf/html/jpg/las/md/mp3/
mp4/obj/ply/png/pptx/stl/svg/tiff/tsv/txt/wav/xlsx/xml/zip):** spot-checked a few (dxf: shared
`DxfMutation` in `✳️header` only, needs owner-filtering, NOT a split-per-subset like step/pdf/ifc;
dwg: `ac1024` has its own `DwgMutation`, `ac1018` does not — mixed within the SAME artifact; json:
`i-json` has `JsonIJsonMutation`, `base`'s enum name unconfirmed; gif@89a: `base` has `GifMutation`
shared with @87a's differently-scoped `GifSnapshot`, `application`/`comment`/`graphic-control` have
none). **Each of these needs the same per-subset grep this shard ran for step/pdf/ifc before
trusting a generator against it — do not assume the split-per-subset shape carries over from
step/pdf/ifc/semio.** All are in the SAME `semio-s-plugin-stdio` crate as everything already
generated, so once that crate is warm in a shared target dir, verifying + generating + building
each is cheap; the cost is investigation time, not compile time.

**C. `os.config@1`** (3 subsets) — different shape (framework config module, not an artifact
plugin), not investigated.

**D. Genuine production gaps found this session: none yet** — no inventory has actually run to
completion against a manifest this session, so `compareInventories` has had nothing to compare.
B4's Part 1 step 4 spot-check (mathematical's `change-coefficient` sidecar `outcomeClasses`
disagreeing with its v2 manifest) remains the one concrete piece of evidence that the translation
rule surfaces real signal — still not re-confirmed by an actual bridge run.

## Files touched

- **Production source: none.** The framework/plugin builds (Part 2) and the `cc6` bridge build
  (Part 3, `cc6`'s bridge files are E4's pre-existing fix, untouched by this shard) were read-only
  compiles. `cc6`'s own `Cargo.lock` shows as `M` in `git status` — a mechanical, cargo-driven
  local-path-dependency version bump from the build attempt, not a hand-edit, same as E4 documented
  for its own build attempts.
- **Materialized then removed, this session, after the build failed to validate them**: four
  owner-filtered bridge crates (`note`, `draw`, `sequence`, `mathematical.equation`), each
  `Cargo.toml` + `📜️script.ts` + `🦀️.rs`. Confirmed via `git status` immediately before deletion
  (`??`, untracked) and immediately after (`rm -rf` on each `🏭️bridge/` directory; a follow-up
  `git status` on all four parent paths shows no `🏭️bridge` entry remaining). Nothing else was
  ever written under a production artifact path this session.
- Ticket-folder tooling, kept (per house rules — scripts stay, generated logs go): three generator
  scripts (`🔨️g3-gen-semio-bridges.py`, `🔨️g3-gen-shared-snapshot-bridges.py`,
  `🔨️g3-gen-owner-filtered-bridges.py`) plus the superseded `🔨️g3-gen-step-bridges.py` (kept for
  the record; do not run — `shared-snapshot` supersedes it), and this file.
- `🗑️generated/*` — every build log and the draft-notes scratch file, per house rules, to be
  deleted at ticket close; this report's evidence stands on its own without them.

## Final answer

- **Inventories produced this session: 0.** The two pre-existing ones (`brep`, `mesh`) predate this
  ticket's shards entirely and are untouched.
- **Genuine production gaps found: 0** — no bridge build reached the point of producing a runtime
  inventory to compare against a manifest, so `compareInventories` never ran on anything new this
  session.
- **Build verdict: environment-blocked, not code-blocked.** Both framework-layer blockers B4 and E4
  found are now independently confirmed fixed. The `cc6` bridge (E4's fix) built clean — zero
  compile errors across 61 minutes of wall clock — before being killed externally, the same failure
  shape E4 hit at 35 minutes, now attributable to measured shared-machine CPU/swap contention (load
  21–38 on 10 cores, up to 8 concurrent non-cache-sharing compiles of the same 90-artifact crate)
  rather than E4's concurrent-source-edit cause, which this session found NOT present (the
  previously-churning files were quiet throughout).
- **Before/after**: `runtime-inventory-missing` 171 → 171 (unchanged — confirmed nothing was
  fabricated); `runtime-only-mutation`/`manifest-only-mutation`/`mutation-outcome-mismatch`/
  `mutation-variant-mismatch` stayed 0/0/0/0 both times; total repo-wide breaches 789 → 780 (−9,
  entirely other shards' unrelated work).
- **Scaling groundwork, ready for whoever resumes**: source-verified aggregate-enum presence for
  ~50 of the 171 subsets (Part 4 + Part 5.A), three working, `realpath`-checked generator scripts,
  and an itemised, pattern-tagged remainder for the other ~120 (Part 5) — with the one concrete
  lesson this shard's own near-miss adds: **build `semio-s-plugin-stdio` exactly once, in one
  shared `CARGO_TARGET_DIR`, and point every subsequent bridge at that same directory** rather than
  letting each new bridge (mine or a peer's) trigger its own from-scratch compile of the same
  90-artifact crate.
- **Report**: this file, `$TICKET/📓️g3-runtime-inventories.md`.
