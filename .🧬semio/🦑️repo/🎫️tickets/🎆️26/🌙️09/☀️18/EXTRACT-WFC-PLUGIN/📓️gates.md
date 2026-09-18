# 🚦️ Slice G — repo gates, plugin crate, assembly deletion

Logs: `🗑️generated/gates/`. Every cargo command ran in the FOREGROUND with `-j 4`; no command was
retried (no exit 137 occurred). `NX_DAEMON=false` on every nx invocation; `bun nx reset` ran once
first, so nothing below was read through the stale post-move project graph.

## 0. Result in one table

| gate | result |
|---|---|
| `bun nx reset` | ✅️ |
| `rm -rf …/🌀️procedural/🗿️artifacts/🧩️assembly` | ✅️ deleted; repo-wide referrer grep clean |
| `cargo metadata --format-version 1 -q` | ✅️ exit 0 (`Cargo.lock` refreshed, no hand edit) |
| `bun ./📜️script.ts schema generate` + `schema docs` | ✅️ 3 200 scopes; **0** `s.procedural.assembly*` keys left in either catalog; 77 `s.wfc.*` scopes |
| `cargo check -p semio-s-plugin-procedural --lib -j 4` | ✅️ |
| `loadTaxonomy(repoRoot)` after the new member rows | ✅️ |
| `bun nx run @semio-tech/plugin-registry:generate` | ✅️ (twice: before and after `describe`) |
| `bun nx run @semio-tech/plugin-registry:check` | 🔴️ repo-wide (2 283 findings); **wfc 78 → 54** |
| `cargo check -p semio-s-plugin-wfc --lib --tests -j 4` | ✅️ |
| `RUST_MIN_STACK=… cargo test -p semio-s-plugin-wfc --lib -j 4` | ✅️ **14 passed / 0 failed** |
| `--test boot_deadline` | ✅️ **1 passed** (was red — fixed, §4.1) |
| `--test idle_turns` | ✅️ **2 passed** (was red — fixed, §4.1) |
| `--test close_ladder` | 🔴️ **0 of 6** — see §5, the one blocking finding of this slice |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-wfc --target wasm32-wasip2 -j 4` | ✅️ |
| `bun nx run @semio-tech/wfc-plugin:describe` | ✅️ 10 apps, 2 window kinds per editor, `defaultLayout` on all 10, 13 manifest examples |
| `bun ./📜️script.ts verify dependencies write-baseline` | ✅️ 230 third-party deps |
| `bun ./📜️script.ts verify dependencies literal-external` | 🔴️ repo-wide (229 literal-external, target 0); **no wfc-specific row** |
| `bun ./📜️script.ts verify taxonomy report --scope "…/🀄️wfc"` | 🔴️ still aborts before any wfc path — §6 |
| `bun ./📜️script.ts policy` | ⛔️ cannot run at all — §7 |
| `NX_DAEMON=false bun nx run @semio-tech/wfc-js:test` | ✅️ **10 files / 148 tests** |

## 1. Assembly deletion

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly` is gone. The repo-wide grep for
`procedural_assembly|procedural-assembly|s\.assembly|data\.assembly|s\.procedural\.assembly|🧩️assembly|mutate-assembly`
(excluding `node_modules`, `target`, `dist`, `⚡️cache`, `🗑️generated`, tickets, `.git`) left six
stragglers, all of them prose, all fixed:

| file | what it said | what it says now |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts:90` | "including the one subset (🧩️assembly) that has no `🚪️io` yet" | "including any subset that has no `🚪️io` yet" |
| `📐️cad/…/🧪️tests/📐️mutate-cad-1/🦀️.rs:75` | listed `🧩️assembly` among the diffs that need `DIFF_ALIASES` rows | `🀄️wfc` |
| `🌀️procedural/…/🧊️mutate-procedural-3d-1/🦀️.rs` | same sentence | `🀄️wfc` |
| `🌀️procedural/…/🌀️mutate-procedural-2d-1/🦀️.rs` | same sentence | `🀄️wfc` |
| `💠️lowpoly/…/💠️mutate-lowpoly-1/🦀️.rs` | same sentence | `🀄️wfc` |
| `🧩️puzzle/…/🧊️mutate-puzzle-3d-1/🦀️.rs` and `◻️mutate-puzzle-2d-1/🦀️.rs` | same sentence | `🀄️wfc` |

The `🀄️wfc` substitution is the truthful one: I checked that the wfc graph subsets really do split
every collection into the `<name>Removed`/`<name>Upserted` pair those comments describe
(`◻️2d/…/🧬️schema/🔺️diff/` declares `slotsRemoved`/`slotsUpserted`/`edges…`/`tiles…`/`rules…`).

Taxonomy member `🧩️appends-slot-c-at-index-2`, which slice C kept for the A3/A5 port, is removed:
no wfc artifact uses that case name (the ported wfc2d/wfc3d cases are
`🧩️inserts-slot-d-in-canonical-order` / `🧩️inserts-room-c-at-the-sorted-position`).

Not touched, correctly: `♻️mit-bestand/🧺️demonstrator/dist/**` (build output, gitignored) and
`🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts:26`, whose assertion is
`expect(cargo).not.toContain("semio-s-artifact-procedural-assembly")` — a NEGATIVE assertion that the
deletion makes trivially true.

Historical prose inside the wfc tree that names assembly as its ancestor (`◻️2d/…/📚️examples/🦀️.rs`,
`🧩️mutate-wfc2d-1/{🥒️.feature,🐍️.py}`, `🧱️grid3d/…/🧩️outcome/🦀️.rs`) was left as written: those are
provenance notes by A3/A4, not live references.

## 2. Taxonomy member rows

`🔣️taxonomy.json` gained **155** member names, all of them wfc-unique strings:

| registry | + rows |
|---|---|
| `members-of-tests` | 67 (the five `mutate-*-1` ids, `🧩️mount-contract`, and every wfc mutation-fixture case name) |
| `members-of-engine` | 36 (the engine module directories, `🀄️tiled` … `🪞️symmetry`) |
| `members-of-schema` | 28 (the wfc mutation directories) |
| `members-of-examples` | 10 |
| `members-of-assets` | 6 |
| `members-of-fixtures` | 4 (`🎲️solver-contracts`, `🔀️topology-contracts`, `🔍️decoding-and-graphs`, `🧬️mutations`) |
| `members-of-windows` | 4 (`🔲️grid`, `🖼️input`, `🧊️preview`, `🧩️output`; `🕸️graph` and `👁️preview` were already registered) |

This covers everything `📓️wfc3d.md` §7.2 asks for except four GENERIC names it also listed —
`🔬️unit`, `🧩️example`, `🧩️outcome`, `🗄️store-fixture` — plus `🚪️close-ladder`, `😴️idle-turns`,
`🔬️surface`, `🔬️boot-deadline`, `🧩️suite`, `🪟️window`. Those are **deliberately not added**: the same
directory names exist unregistered under `🧩️puzzle`, `🗒️note`, `📸️remodel` and `🌀️procedural` today,
so `memberNames` is plainly not where the test taxonomy resolves them, and registering them would
change classification for every plugin in the repo, not just wfc. The other four artifact reports
(`📓️grid2d.md`, `📓️wfc2d.md`, `📓️grid3d.md` and the three `📓️audit-*.md`) list no taxonomy rows at all.

The scanner that produced the list is `📜️gates-taxonomy-scan.ts` (positional, mirrors the
`memberNames` read in `semanticDirectoryKindId`); raw output in
`🗑️generated/gates/member-scan-wfc.jsonl`, the accepted delta in
`🗑️generated/gates/taxonomy-rows-to-add.json`. Calibration: the same scan reports 328 unregistered
names for `🧩️puzzle`, 257 for `📸️remodel`, 223 for `🌀️procedural` and 81 for `🗒️note`, so a non-zero
count is the repo's normal state, not a wfc defect.

### 2.1 One more inadmissible emoji, like `▦️grid2d` before it
`loadTaxonomy()` threw on the first write: `members-of-examples has invalid exact member
"⬡️hex-ring"`. `⬡` (U+2B21 WHITE HEXAGON) is not `Extended_Pictographic`, so — exactly like A1's
`▦️` (U+25A6) — it can never pass `canonicalTaxonomyEmoji`. The example directory is renamed
`📚️examples/⬡️hex-ring` → `📚️examples/🔷️hex-ring` (`🔷` U+1F537, free among its four siblings), with
the crate-root `#[path]` mount in `◻️2d/🦀️.rs` and the docstring emoji inside the example's own
`🦀️.rs`/`🟦️.ts` repointed. `loadTaxonomy()` is green afterwards
(`members-of-tests` 1 311, `members-of-engine` 130, `members-of-examples` 67).

## 3. Structural registry fixes (`plugin-registry:check`, wfc 78 → 54)

1. **Mode and window facet folders (24 findings).** `🖼️bitmap`, `🔲️grid2d`, `◻️2d` and `🧱️grid3d` were
   missing `🎮️commands`/`🎚️config`/`👥️presence`/`🫧️transient` under both modes, and every window in the
   plugin was missing some of `🎬️actions`/`🪛️utilities`/`☑️options`/`🎚️config`/`👥️presence`/`🫧️transient`.
   154 folders created, each with a `📌️.empty.md` carrying the scaffolder's own marker text — the shape
   `🧊️3d` (A5) and `🧩️puzzle` already ship. I did **not** run `new surface --all`: `--all` walks every
   owned subset in the repo, and even the per-subset form
   (`bun …/📇️registry/📜️script.ts new surface wfc 2d 1 any editor --dry-run`) wants to create a
   spurious `🪟️windows/🪟️main` window that no wfc artifact has. The dry-run output is kept at
   `🗑️generated/gates/surface-dryrun.log`.
2. **Example `🟦️.ts` twins (6 findings).** `🪠️pipes-3d`, `🧱️blocks`, `🌸️flowers-24`, `🚪️rooms-16`,
   `🏜️terrain`, `🚰️pipes` got the thin metadata twin puzzle ships (`id`, `label`, `icon`, `seed`, plus
   `dslPath` where the example commits an asset), with the values read out of each example's `🦀️.rs`.
3. **`▦️grid` leftover.** An EMPTY `🔲️grid2d/…/🪟️windows/▦️grid` directory survived A2's rename and still
   carried the inadmissible `▦️`. Removed.
4. **wfc2d's emoji-less fixture directories.** `◻️2d/…/🧫️fixtures/🧬️mutations/` held fifteen BARE kebab
   directories (`change-seed`, `create-slot`, …) while its `🧬️schema/🧬️mutations/` siblings — and all
   four other wfc artifacts, and puzzle — use the emoji form. Each was renamed to its schema sibling's
   name (`🎲️change-seed`, `🧩️create-slot`, …) and the 16 files that named them (the mounted fixture
   tests' `include_str!`, the `🥒️.feature`, the `🐍️.py`, the TS suite) rewritten.
   `🐍️wfc2d-mutations.py` is fixed at the source: it wrote `FIX / semantic / case` and the matching
   `rel`, i.e. the bare kind, where it must write `spec["dirname"]`. Re-verified:
   `cargo test -p semio-s-artifact-wfc-2d --features component-app-assembly --lib` **182 passed / 0 failed**.

### 3.1 The 54 that remain, and why
All of them are categories the whole repo carries; the repo-wide count is in brackets.

| finding | wfc | repo | verdict |
|---|---|---|---|
| `window "X" has unexpected child "🧪️tests"` | 15 | 429 | Every plugin with mounted window unit tests has it (`✒️writer`, `🧩️puzzle`, `🗒️note`…). Moving the tests out would unmount them. Leave. |
| `surface is missing 👥️presence/🧬️schema/` | 10 | 251 | Needs a real `Presence` value model per surface (6 normative language leaves each). Artifact-author work, and 24 plugins are in the same state. |
| `surface is missing 🎚️config/🧬️schema/` (+ 12 itemised leaves) | 20 | 249 + 41 | Same. `◻️2d` has the directory but no leaves; the rest have neither. Note A3's remark holds regardless: the editor's `Wfc2dConfig`/`Wfc2dTransient` are REAL types and the `VcsArtifactApp<EditorApp<…>>` generic carries them — the plugin root needed no change for that, and `cargo check --lib --tests` proves it. |
| `example is missing 🖼️assets/` | 6 | 10 | `◻️2d`'s four are a DECLARED deviation (`◻️2d/…/📚️examples/🦀️.rs:6` says so explicitly: assembly/note commit a printed text asset and `include_str!` it, wfc2d builds in Rust instead). `🔲️grid2d`'s two (`🏜️terrain`, `🚰️pipes`) follow the same choice. Producing a byte-correct `🗣️.dsl.semio` needs each artifact's own printer — A2/A3 work, not the plugin root's. |
| `🧪️tests/🧩️mutate-{bitmap,wfc2d}-1/🦀️.rs is not reachable from Cargo manifest` | 2 | ~20 | These are the `semio_repo_test_host` adapters `bun ./📜️script.ts contract` drives, not cargo targets. No plugin in the repo mounts them; `✒️writer` carries the identical finding. `🔲️grid2d`/`🧱️grid3d`/`🧊️3d` have no adapter at all (wfc3d gap #1), which is the WORSE state, so I left the two that exist. |
| `plugin root is missing 🎮️commands/🦀️.rs` | 1 | 33 | `🧩️puzzle` and `🗒️note` ship `🎮️commands/📌️.empty.md` and nothing else, exactly like wfc. |

For scale: `🏗️fem` (5 artifacts) has 80 findings, `🀄️wfc` (5 artifacts) now has 54, `🧩️puzzle`
(3 artifacts) 27.

## 4. Plugin crate

`cargo check -p semio-s-plugin-wfc --lib --tests -j 4` is GREEN. Two plugin-root-owned defects were
in the way; neither needed an artifact API change.

1. **`examples::example_source_slice` does not exist on grid2d.** `🔲️grid2d` nests its example roster
   one level deeper than its four siblings (`examples::grid2d::example_source_slice()`, because its
   crate root wraps the module in `pub mod grid2d`). The plugin surface test was adapted TO the
   artifact, per contract: `🧪️tests/🔬️surface/🦀️.rs:61` now calls
   `semio_s_artifact_wfc_grid2d::examples::grid2d::example_source_slice()`.
2. **`every_wfc_inference_route_names_its_own_artifact_kind` asserted the wrong id.** It compared
   `metadata.artifact_kind` to `artifact_kind().id` (`2d.wfcbitmap`), but all five artifacts — and
   `📐️cad` (`s.cad.cad` vs kind `3d.cad`) and `🧩️puzzle` — put the DIALECT kind there, and
   `ArtifactInferenceServiceRegistry` keys its services by `(artifact_kind, inference_schema)` with the
   document's dialect kind. The law now asserts against each crate's `*_DOCUMENT_SCHEMA`, with the
   reasoning in its docstring. The artifacts were right; the plugin-root law was wrong.

### 4.1 Two lane defects, both in plugin-root test files, both fixed
`boot_deadline` and `idle_turns` both died on
`plugin.instance-metadata-capacity: fixed instance metadata authority is saturated or collided`, on
the SECOND editor. Both laws loop over all five editors, reuse ONE instance id, and deliberately leak
each runtime (`std::mem::forget`, because `NativeLifecycleRegistry`'s drop authority aborts the
process on an unwind) — so the fixed metadata authority still holds instance N when the next boot asks
for it. Fixed by giving each editor its own id (`BOOT_INSTANCES = [1..5]`,
`IDLE_INSTANCES = [3..7]` plus `UNACKNOWLEDGED_INSTANCE = 8`), with the reason in a docstring at each
constant. Both lanes are green afterwards; `bitmap` boots at
`open_us=10 051 describe_us=17 150 descriptor_bytes=173 386 settle_turns=1`, and the idle law measures
409 B over 512 turns (0 B/turn) against a 65 536 B ceiling.

`close_ladder`'s single looping law was split into five per-artifact `#[test]`s
(`bitmap_editor_instance_close_reaches_retired` …), because the fault below aborts the whole test
binary (`panic in a destructor during cleanup` → SIGABRT) and a loop therefore reports only whichever
artifact runs first while hiding the other four.

## 5. 🔴️ BLOCKING: the close ladder faults for ALL FIVE wfc editors

```
cargo test -p semio-s-plugin-wfc --test close_ladder -j 4 -- --test-threads=1
→ running 6 tests; 0 pass. Each aborts with SIGABRT.
```
Per-artifact logs: `🗑️generated/gates/close-ladder-{bitmap,grid2d,wfc2d,grid3d,wfc3d}.log`.

The fault is identical for every editor:
```
[DEBUG] close cleanup fault contained to its own retired lifetime: plugin.internal.prior-outcome
        runtime close cleanup faulted for instance 7: a prior step outcome faulted or was cancelled
        [prior-outcome] (elapsed 6us, ceiling 8000us)
wfc close-ladder turn: Fault { origin: Plugin, code: FaultCode("plugin.internal.prior-outcome"), … }
then: 🧰️framework/…/⚛️reactor/🚪️lifetime/🦀️.rs:489 "runtime lifetimes require terminal exact ACK
      before teardown" → panic in a destructor during cleanup → abort
```
`plugin.internal.prior-outcome` is raised at `🔌️plugin/🦀️.rs:35009` when
`RuntimeCloseCleanupJob`'s step returned `StepOutcome::Fault(_)` or `Cancelled` — that is, when the
APP's own `close_step` faulted inside the guest. The reactor's `more-work` trace shows the ladder
turning with `sources=["close_cleanup", "reconcile", "lifecycle"] closing=1` and the streak growing
until the fault (`🗑️generated/gates/close-ladder-wfc3d-diag.log`, produced with
`SEMIO_RUNTIME_DIAGNOSTICS=1`).

What this is NOT — each checked, not assumed:
- **Not the framework.** `cargo test -p semio-s-plugin-procedural --test close_ladder` is
  **3 passed / 0 failed** right now (`🗑️generated/gates/procedural-close-ladder-control.log`), through
  the identical `close_to_retired` helper and the identical `VcsArtifactApp<EditorApp<…>>` wrapper.
- **Not the test harness.** I temporarily reran the bitmap law as a plain
  `#[semio_framework_async_macros::async_test]` instead of the file's own
  `on_close_ladder_thread` + `block_on` (procedural's shape) — identical fault
  (`🗑️generated/gates/close-ladder-experiment.log`). The custom thread is back, since its
  256 MiB stack is a real requirement.
- **Not the instance-id collision of §4.1.** A single law, run alone on a fresh process with one boot,
  fails the same way, and the fault code is different.
- **Not the plugin root.** `🀄️wfc/🦀️.rs` matches `🌀️procedural/🦀️.rs` and `🧩️puzzle/🦀️.rs` line for line
  in every close-relevant respect (`dyn_enum_close!`, the wrapper types, `plugin_app_close_prelude`,
  `ExecutionMode::Isolated`, `plugin_exports!`).

**Therefore it is artifact-side, and uniform** — the five editors were authored in parallel from the
same assembly template, so a defect in that template is in all five. This is the fourth member of the
family A2/A3/A5 already documented (retained payloads not retired; `terminal_is_empty` gated on a
private `closing` flag; `CommitCandidate`'s second payload) — but those three were fixed in the
INFERENCE job, and this one is on the EDITOR INSTANCE's close path, which nothing before the
plugin-level lane exercised: every artifact's own `--lib` suite is green.

**Owner: A1–A5 (all five), W3.** Reproduce per artifact with
`RUST_MIN_STACK=33554432 cargo test -p semio-s-plugin-wfc --test close_ladder -j 4 -- --test-threads=1 <artifact>_editor_instance_close_reaches_retired`.
The cost law `wfc_close_cost_is_independent_of_the_retained_session` (fixture app
`s.wfc.grid2d@1/*#editor`) never reaches its ceiling/dilution assertions, so the numbers its
`lawNote` says to re-measure against wfc's own runs are still the inherited procedural ones.

## 6. `verify taxonomy` — still blocked, and why regeneration is not a cheap local step

The scoped run still exits 1 before classifying a single wfc path. The first of P's two recorded
faults is now GONE, the second is worse than it looked:

1. ~~`Current compiler input manifest compiler inputs are not path-sorted`~~ — fixed. The offending
   file, `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json`, is gitignored
   derived state (`.gitignore:15`, last written 2026-09-15), and exactly 10 of its 3 030 `inputs[]`
   rows were byte-unsorted. I sorted them in place with the law's own comparator
   (`Buffer.compare`, `🧹️normalization/🟦️.ts:2100`); backup at
   `🗑️generated/gates/manifest.json.backup`.
2. The NEXT staleness then surfaces: `Current compiler input manifest compiler input bytes differ:
   bun.lock`. I measured the whole file against disk: **1 281 of 3 030 rows point at paths that no
   longer exist, and 283 more have different bytes.** Only a real regeneration fixes that.
3. **The regeneration is broken, for an unrelated reason.**
   `bun nx run @semio-tech/framework-os-dev:generate-distribution` (the documented writer, `📋️project.json`
   → `bun ./📜️script.ts distribution generate`) fails in 2 s with
   `Missing or unsafe distribution static input:
   🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🏗️build-tooling.ts`.
   That path is declared in the TRACKED
   `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔗️inputs.json:11`, and the file has since
   moved to `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts` (which is what
   `🎨️styling/🏗️builder/🌐️vite/🟦️.ts:20` imports today). **Owner: whoever moved the react build tooling
   must repoint `🔗️inputs.json`; then `generate-distribution` will rewrite the manifest and the
   taxonomy gate unblocks for the whole fleet.** Log: `🗑️generated/gates/generate-distribution.log`.

The `📐️cad-draw-path-projection` frozen-digest fault P also recorded belongs to another owner and was
NOT touched, per the brief. It sits behind fault 2 in the same abort chain, so it has not been reached
again yet.

## 7. `bun ./📜️script.ts policy` — cannot run in this tree

```
error: [repo/cli] exit ?: ENOENT: no such file or directory, posix_spawn
       '…/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/client'
```
`runPolicyScript` resolves its entities through the repo CLI binary, which is not built. Building it
fails too: `bun nx run @semio-tech/repo-client:build` →
`stat …/💻️client/⌨️cli/cmd/repo: directory not found` (the Go `main` package the build script names
does not exist on disk; the module has `internal/` and the emoji leaf packages, no `cmd/`).
Log: `🗑️generated/gates/repo-client-build.log`. This is the same missing binary that made the `repo`
and `semio` MCP servers fail to connect for this session. **Owner: the repo-client module.**

Consequence: the three comment-inside-definition sites the brief named
(`🧱️grid3d/…/💡️inferences/🦀️.rs:505-508,551-556`, `📸️snapshot/💾️binary/🦀️.rs:90-91`, crate root
`🦀️.rs:507`) could not be confirmed as policy findings, so I did not rewrite them blind — a
`policy` run is the only thing that says which of them the rule actually flags.

## 8. Descriptor

`bun nx run @semio-tech/wfc-plugin:describe` (5 m 21 s) wrote `🀄️wfc/🔣️.json` (680 014 B) and
`🀄️wfc/🛂️.descriptor.semio` (173 587 B). All five apps and their viewers are present, each editor
carries exactly two window kinds, every app has a `defaultLayout`:

| app | window kinds |
|---|---|
| `s.wfc.bitmap@1/*#editor` | `wfc-bitmap-input`, `wfc-bitmap-output` |
| `s.wfc.grid2d@1/*#editor` | `wfc-grid2d-grid`, `wfc-grid2d-preview` |
| `s.wfc.wfc2d@1/*#editor` | `wfc-graph`, `wfc-2d-preview` |
| `s.wfc.grid3d@1/*#editor` | `wfc-grid3d-grid`, `wfc-grid3d-preview` |
| `s.wfc.wfc3d@1/*#editor` | `wfc-graph`, `wfc-3d-preview` |

`examples` is **13** and lives at the manifest TOP level, not per app — the same shape `🧩️puzzle`
ships (7 top-level, 0 per app), so the react example picker reads a non-empty roster.

## 9. Files this slice changed outside `🀄️wfc`

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (155 member rows added, 1 removed,
1 renamed), `…/📚️library/{🔣️schema-catalog.json,📓️schema-catalog.md}` (regenerated),
`…/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts` (one docstring),
`…/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json` (gitignored, inputs sorted), `Cargo.lock`
(refreshed by `cargo metadata`), the generated registry projections, six peer `mutate-*-1/🦀️.rs`
docstrings, and the deletion of `🌀️procedural/🗿️artifacts/🧩️assembly`.
