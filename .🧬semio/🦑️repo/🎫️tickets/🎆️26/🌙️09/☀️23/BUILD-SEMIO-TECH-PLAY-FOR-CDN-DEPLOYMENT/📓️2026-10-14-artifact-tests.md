# 🧪️ Artifact Test Targets (2026-10-13 → 2026-10-14)

Lane: artifact-tests. Scope: the lib test targets of the plugin artifact crates under `✏️s/🔌️plugins/`, excluding stdio pdf/dwg/gltf/semio, architect and bim. The `3d` exclusion is `semio-framework-3d` (E4), so it does not remove any plugin crate. This report replaces the planned `📓️2026-10-13-artifact-tests.md`.

**Current state: STOPPED, blocked upstream.** Since 2026-10-14 20:40, `.cargo/config.toml` routes `build.rustc` through `…/🔒️enforcement/🏗️compiler/🚪️ingress/📜️script.ts`. That script always throws `Original compiler ingress requires the repository launcher and authentic enforcing store`. The only code that issues the compiler environment is `repositoryCompilerLaunchEnvironmentV1` (`🚪️ingress/🟦️.ts`), which needs a `compiler-workspace` native storage reservation. Only that lane's tests call it; there is no repository launcher for a plain `cargo test --manifest-path …`. So every bare cargo invocation in this lane fails at `rustc -vV`, including U1's 23:46 checks, which stop even earlier at workspace resolution. I did not get around the gate by setting `RUSTC`/`RUSTC_WRAPPER` myself: it is deliberate storage custody owned by the MODULAR-MONOREPO fleet. Before the gate, the 18:45–19:00 re-baseline was already blocked by in-flight framework cuts (`dsl-record` → `dsl-record-model`, `value` testing).

## Method

All scripts are in `artifact-tests/`.

- `🧪️sweep.py`: one `cargo test --lib --no-run` per crate. It adds `--features component-app-assembly` only where the manifest still declares that feature. Each cargo runs in the foreground through `🚦️cargo-slot.sh at-<lane>` with `⏱️bounded.sh`, and resumes on the next call. Private dirs are `⚡️cache/at/{t,b}`. Each result is classified as:
  - `ok`;
  - `fail`, meaning an error in the crate's own files;
  - `blocked`, meaning the first error is in a dependency (an absolute path);
  - `nolib`.

  After each crate the script keeps only the 3 newest unit dirs per package. A disk-full event on 10-13, at 1.7 GiB free, came from the fleet plus about 15 GB of my own; pruning freed 8.3 GB.
- `🧪️per-test.py`: runs one process per test with `RUST_MIN_STACK=67108864`, a 60–120 s timeout per test and 6 jobs. Each test is recorded as pass, ignored, fail or timeout. Once a package is fully recorded, its test unit is deleted.
- `🧮️buckets.py`: groups failures by normalized panic message.
- `🦀️cargo.sh`: a single cargo invocation with the same private dirs and slot.
- `📋️order.txt`: the 149 in-scope manifests, in priority order. Play editors (puzzle, shooting, trinity, procedural, block, draw, raster, writer, note, forms, layout, cad, …) come first, then the stdio formats, then generators, probes and oracles.

## Compile status (lib test target)

There are 149 in-scope manifests. The 10-13 sweep recorded 115 packages:

| Status | Count |
| --- | ---: |
| ok | 53 |
| nolib (generators, probes, codecs) | 20 |
| blocked | 25 |
| fail | 17 |

The remaining 34 manifests were not reached before the framework went red.

### Before and after, crates with own errors

| Crate | Before (own errors) | After | What was stale |
| --- | --- | --- | --- |
| puzzle-5d | 67 (corrections #62) | **ok**, 0 | Already migrated by the time of this sweep, 10-13 |
| puzzle-3d | 32 | **ok**, 0 | Four leaf tests and one panel test still imported `schema::mutations::apply_puzzle3d_mutation`; it is now `crate::apply_puzzle3d_mutation` (one duplicate import removed). Other fixes:<br>• `crate::Puzzle3dPlaySnapshot`.<br>• `with_registry` now takes `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` and `&mut identity` through `with_authoring_identity!`.<br>• `Puzzle3dConfigMutation::Snapshot` became `SetActiveExampleId(..)`.<br>• `JobTurn::Fault` now carries `Vec<u8>`, so `fault.detail.single_page()` became `fault.as_slice()`.<br>• `MutationDiff::apply` became `protocol::apply_diff`.<br>• The new `drive_step(job, &mut cx, site, stage, verdict)` uses the shared `puzzle_job::testing::owned_turn`.<br>• `from_dsl_value(&…)`, `as_array` → `<[_]>::len`, `cold_insert`/`cold_remove` → `insert`/`remove`, and `HistoryInputDrafts`. |
| stdio-html | 2 | **ok**, 0 | The warning-flood inliner (#76b) had dropped `let mut borrowed` / `let mut interrupted = SqliteSnapshotControl::new(..)` in the cohort test; the bindings are restored. |
| stdio-bcf | 1 | **ok**, 0 | The kinds test imported the private `print_bcf_mutation`, which lives inside `mod mutations_codec`. The test now uses `protocol::OpText::print_op`; product visibility is unchanged. |
| stdio-binary | 10 (all upstream) | **ok**, 0 | Nothing in the crate; the kernel was mid-cut. |
| trinity-jack | stray `}` at `✏️editor/🧪️tests/🔬️unit/🦀️.rs:70` | removed, not compiled | Blocked upstream at re-check (3 kernel errors). |
| stdio-tiff, -png, -bmp | stale camel-case `tiffAnyEditor_*` / `pngEditor_*` / `bmpEditor_*` | renamed to the snake-case functions (#warning-flood) | bmp compiled ok after the rename (10-13). tiff and png were not re-compiled because they were blocked upstream. |

The other 10-13 `fail` rows were classified before the dependency classifier existed:

- tsv, xlsx, docx, md, xml, png, jpg, avi, pptx, txt, stl, dxf, tiff, deflate, obj, ply, json;
- also the original csv and step rows.

Every log I opened (binary, step, md, txt, tiff, json, avi, stl, docx) showed errors only in kernel or plugin files that were being cut at that moment. Treat these rows as blocked, not as crate debt.

The blocked rows break down as follows:

| Rows | First error |
| --- | --- |
| norm ×15, generation2d/3d, rewriting, curation, playground | plugin crate (E2b) |
| puzzle-2d | `semio-framework-3d` (E4) and gltf (E3b) |
| shooting | stdio-semio drawing export |

The generation3d "missing test file" was not reproduced: a scan of every top-level `#[path]` in the 149 crates found no missing target.

### 10-14 re-baseline (18:45–19:00, before the gate)

37 rows were recorded: 34 blocked, 1 nolib, and 2 `fail` that are actually the ingress refusal (html, html-oracle, 23:46). The blockers were:

- `semio-framework-dsl-record` (unresolved `semio_framework_dsl_record_model`, private `precedence`/`symbol`);
- `semio-framework-value` `🧪️testing/🧾️wire` (`RetainedCloneGrant` not in scope).

Both are owned by the MODULAR-MONOREPO fleet or U1.

## Tests run (10-13 binaries, one process per test)

| Crate | Pass | Fail | Timeout | Ignored |
| --- | ---: | ---: | ---: | ---: |
| puzzle-5d (837) | 769 | 47 | 17 | 4 |
| puzzle-3d (partial: 61 of the list, cut by the disk event) | 29 | 8 | 24 | 0 |
| stdio-ifc | 232 | 12 | 0 | 0 |
| stdio-svg | 148 | 7 | 0 | 0 |
| stdio-gif | 150 | 2 | 0 | 0 |
| stdio-zip | 130 | 10 | 0 | 0 |
| stdio-wav | 79 | 2 | 0 | 0 |
| stdio-html | 73 | 8 | 0 | 0 |
| stdio-bcf | 66 | 3 | 0 | 0 |
| stdio-las | 66 | 2 | 0 | 0 |
| stdio-mp4 | 65 | 6 | 1 | 0 |
| stdio-bmp | 53 | 5 | 0 | 0 |
| stdio-epw | 51 | 2 | 0 | 0 |
| stdio-mp3 | 46 | 8 | 0 | 0 |
| mp3 test oracle | 7 | 1 | 0 | 0 |
| mp3 metadata laws | 6 | 0 | 0 | 0 |
| jpg oracle | 2 | 0 | 0 | 0 |
| mp4 oracle | 1 | 0 | 0 | 0 |

The remaining oracle crates list 0 tests in their lib target.

### Failure buckets

None of these was fixed: the tree went red before a fix could be verified.

1. **Framework `history_edit_acceptance_law` fails for every stdio editor** (wav, gif, las, svg, zip, ifc, mp3, mp4, epw). The messages are:
   - `authoring requires an installed canonical operation wire or an original domain codec` on the example document;
   - `none of the 0 committed and N derived editable case(s) … exercises a history edit`;
   - `batch original canonical digest authority is absent` (epw `set-cell`).

   The owners are plugin and store (E1b/E2b); the hub `editor_catalog` was 177/177 before the cuts.
2. **Leaf payload schemas without labels** (mp3 `id3v2/frames` and `id3v1/artist`, also svg, mp4 and bmp). The message is `InputSchemaError LabelMissing`. This is artifact schema debt, fixable in the crate's `🔣️.json` `x-semio-ui.label`.
3. **puzzle-5d window-config verbs: `interactive-job.publication-stalled`** (15 tests: setCamera, setGridVisible, setLodMode, toggleSun, focusSelection, …). The message is "advanced 4096 publication units without moving its ladder". This is the framework mounted window-config lane (#74).
4. **puzzle-5d real product bug, not yet fixed.** `editor::Puzzle5dPart.part_kind: String` (required) projects the typed `Puzzle5dPart.part_kind: Option<String>`. A valid snapshot with a kindless part therefore panics in `puzzle5d_editor_projection` with `expect("admitted typed snapshot has a host projection")` (2 gesture-kind tests). The root fix is `#[value(default, rename = "partKind", skip_serializing_if = "String::is_empty")]` on the host field, which round-trips `None`. It is not applied because it cannot be compiled now.
5. **puzzle-5d test debt:**
   - 4 source-scan hostile laws read moved editor code;
   - 2 committed diff fixtures drifted (rename-puzzle5d, replace-kind-catalogs), plus the canonical-regeneration law (`SEMIO_PUZZLE5D_WRITE_FIXTURES=1`);
   - 4 `inverse_sums_to_the_negative_diff` / `committed_diff_applies_to_after`;
   - 5 `store did not reach its terminal-empty shell within its own declared close turns`;
   - 5 `original payload append requires its same operation ledger` (fill/job);
   - 17 timeouts (clipboard, kit import, context menu, brush).
6. **Smaller stdio items:**
   - ifc: two committed fixture paths are missing (`NotFound`), and v2x3 `op tag unknown`;
   - svg: diff grammar round trip;
   - mp3: `SetId3v1` variant missing from the catalog;
   - zip: a valid UTF-8 comment is rejected as ambiguous.

## Files touched

- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs/🦀️.rs`: in the `cfg(test)` `testing` module, a new `owned_turn`; `drive` uses it.
- puzzle 3d, under `🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`:
  - `✏️editor/🦀️.rs`: `republished` binding, `cfg_attr(not(test), allow(unused_variables))`;
  - `✏️editor/🧪️tests/🔬️unit/🦀️.rs`;
  - `✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`;
  - `✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`;
  - `✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`;
  - `🧬️schema/🧬️mutations/🧪️tests/{🔬️unit,🧪️selection-time-travel}/🦀️.rs`;
  - four leaf tests under `🧬️schema/🧬️mutations/{✋️drag-selection,🔄️rotate-selection,🔍️scale-selection}/🧪️tests/…`.
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🚦️cohort/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🧬️mutations/🧪️tests/🔬️kinds/🦀️.rs`.
- The `✏️editor/🧪️tests/🔬️unit/🦀️.rs` of tiff (`🔖️6.0/🪆️subsets/🧾️document`), png (`🔖️1.2/🪆️subsets/✳️any`) and bmp (`🔖️v3/🪆️subsets/✳️any`).
- `artifact-tests/{🧪️sweep.py,🧪️per-test.py,🧮️buckets.py,🦀️cargo.sh,📋️order.txt}`, this report, and corrections entry 97.
- The private dirs `⚡️cache/at/` have been deleted.

## To resume

Resume once a repository launcher can issue the compiler environment for an arbitrary manifest, or the coordinator allows a sanctioned `RUSTC=rustc RUSTC_WRAPPER=` with a disk guard, and once U1 reports sdk/kernel/hub green. Then:

1. `🧪️sweep.py artifact-tests/📋️order.txt <results> --retry`, two lanes.
2. `🧪️per-test.py`.
3. Fix bucket 4 (5d `part_kind`) and bucket 2 (labels) first; both are owned by the artifact crates. Then the test-debt buckets.
