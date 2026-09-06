# W1 — de-async the remodel schema layer

Codemod: `🐍️w1-deasync.py` (this folder). **212 edits across 48 files, all of them exactly
`async fn` → `fn`** — verified by parsing `git diff -U0`: 212 added / 212 removed lines, and every
`(-,+)` pair differs only by that keyword (0 exceptions). No other churn, no reflow, no reordering.

## 1. Framework traits — read and confirmed sync before editing

| trait | location | methods (all plain `fn`, no `async`, no `Future` return) |
|---|---|---|
| `protocol::MutationKind<P,Op>` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs:215-236` | `diff` **221**, `inverse` **225**, `label` **227**, `target` **234** (the brief/explore note said 219/224/226/232 — the file has since shifted ~2 lines; these are re-verified) |
| `protocol::Mutation<P>` | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:145-153` | `descriptor` 151, `diff` 152, `inverse` 153 |
| `protocol::MutationDiff<P>` | same file `:98-114` | `apply` 99, `absorb` 114 |
| `protocol::OpText` | same file `:1252-1255` | `print_op`, `parse_op` |
| `protocol::OpBinary` | same file `:1266-1272` | `encode_op`, `decode_op` |
| `store::ArtifactDsl` | `…/🏪️store/🦀️.rs:4896-4906` | `EXTENSION`, `parse_dsl` 4900, `print_dsl` 4901, `envelope_id` 4903 |
| `store::ArtifactPack` | `…/🏪️store/🦀️.rs:9332-9357` | `encode_pack_with` 9333, `decode_pack_with` 9334, `record_spec` 9354 — **the whole trait is sync** |
| `store::InferredField<P>` | `…/💡️inference/🦀️.rs:83-110` | `reads`, `plan`, `dep_input`, `compute` |
| `protocol::Inference<P>` / `InferenceSpec<P>` | `…/📡️spr/🎮️command/🦀️.rs:33-34` / `:99-105` | `infer`; `inference_schema_id`, `schema_version`, `fields` |
| `ArtifactBuilder` | `…/🔌️plugin/🦀️.rs:923-933` | `empty`, `from_snapshot`, `from_text`, `from_binary`, `mutate`, `absorb`, `build` |
| `ArtifactAnalysis` / `ArtifactComposition` | `…/🔌️plugin/🦀️.rs:981-999` | `sniff`, `analyze`; `reads`, `compose` |
| `ArtifactEditor` | `…/🔌️plugin/🦀️.rs:26623+` | `initial_snapshot` :26805, `handle` :26818, `render` :26878, `window_measures` :26903, `app_schema` :26915, `io` :26918, `export_media` :26928, `import_media` :26943, `command_id` :26826, `command_from_action` :26829 — **all sync** |
| `ArtifactViewer` | `…/🔌️plugin/🦀️.rs:26972+` | `initial_snapshot` :27076, `handle` :27083, `render` :27100 — **all sync** |

Genuinely async, deliberately left alone: `ArtifactPack::pack_at_checkpoint` does **not exist** —
`pack_at_checkpoint` is a *VCS* trait method (`🏪️store/🦀️.rs:17708`), not in this subset at all.
`ArtifactApp::handle` (`🔌️plugin/🦀️.rs:11368`) **is** async — but remodel implements `ArtifactEditor`,
not `ArtifactApp`, and `ArtifactEditor::handle` is sync. `InferenceSpec::infer_cached` is async
(puzzle keeps it so); remodel has no impl of it.

Oracle: `🧩️puzzle/🧊️3d`'s `✳️any` subset has **zero** `async fn` in production code — 86 `async fn`
lines, every one either a `#[…async_test]` test or the wasm `Puzzle3dArtifactVcs::new`. Its
`🌍create-target-volume/🦀️.rs:29-38` is now byte-shape-identical to remodel's `🌱create-stream/🦀️.rs:29-38`.

## 2. Exact counts per method

**MutationKind (35 kind files, all 35 confirmed present in the changed set) — 123 edits**

| method | count | note |
|---|---|---|
| `diff` | 35 | one per kind |
| `inverse` | 35 | one per kind |
| `label` | 35 | one per kind |
| `target` | 18 | only the kinds that override the default |

The per-kind `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs` siblings were **already sync** free fns
(`pub fn diff(payload,base)`, `pub fn inverse(payload,base)`) — 0 edits needed, 70 files inspected.

**Everything else — 89 edits**

| group | methods × impls | edits |
|---|---|---|
| `Mutation<P>` (`RemodelingConfigMutation`, `RemodelingPresenceMutation`) | `diff`,`inverse` × 2 | 4 |
| `ArtifactDsl` (`RemodelingSnapshot`, `RemodelingConfig`, `RemodelingPresence`) | `envelope_id`,`parse_dsl`,`print_dsl` × 3 | 9 |
| free `parse_dsl`/`print_dsl` in `🧬️schema/📸️snapshot/📝️text/🦀️.rs:16,21` (callees of the above) | 2 | 2 |
| `ArtifactPack` (same 3 types) | `encode_pack_with`,`decode_pack_with`,`record_spec` × 3 | 9 |
| `OpText` (`RemodelingMutation`, `…ConfigMutation`, `…PresenceMutation`) | `parse_op`,`print_op` × 3 | 6 |
| `OpBinary` (same 3) | `encode_op`,`decode_op` × 3 | 6 |
| `MutationDiff` (`RemodelingDiff`, `RemodelingPresence`) + inherent `RemodelingDiff::apply_to_artifact` | `apply` 2, `absorb` 2, `apply_to_artifact` 1 | 5 |
| `InferredField` (`RemodelingRelativeCameraPose`) | `reads`,`plan`,`dep_input`,`compute` | 4 |
| `Inference` + `InferenceSpec` (`RemodelingInference`) | `infer`,`inference_schema_id`,`schema_version`,`fields` | 4 |
| `ArtifactBuilder` (`RemodelingBuilderConstruction`) | `empty`,`from_snapshot`,`from_text`,`from_binary`,`mutate`,`absorb`,`build` | 7 |
| `ArtifactAnalysis` (`RemodelingAnalyzerAnalysis`) | `sniff`,`analyze` | 2 |
| descriptor builders (`remodeling_artifact_schema_descriptor`, `…_inference_descriptor`, `app_schema_descriptor`) | 3 | 3 |
| `📶️signal-internals` pure-math free fns (fft/ifft/fft2/ifft2/hann/hamming/blackman/cosine_window/welch_psd/cross_spectrum/averaged_segment_spectra/xcorr_normalized/subsample_peak/savitzky_golay/gaussian_smooth_1d/moving_average/convolve_mirrored/mirror_index/find_peaks/next_pow2) | 20 | 20 |
| inference callees `compute_remodeling_bounds`, `se3_from_preview`, `trajectory_poses` | 3 | 3 |
| `#[cfg(test)] mod` **helpers** (no test attribute; their own test bodies already call them without `.await`): `populated_scene_fixture` ×2, `triangle_snapshot`, `two_pose_snapshot`, `seeded_noise` | 5 | 5 |

Files touched: 48 (35 mutation kinds + `🧬️mutations/🦀️.rs`, `🧬️mutations/📝️text`, `🧬️mutations/💾️binary`,
`🧬️schema/🦀️.rs`, `📸️snapshot/🦀️.rs`, `📸️snapshot/📝️text`, `🔺️diff/📝️text`, `📶️signal-internals`,
`💡️inferences/🦀️.rs`, `💡️inferences/📦bounds`, `💡️inferences/🔄relative-pose`,
`✏️editor/🎚️config/🦀️.rs`, `✏️editor/🎚️config/🧬️schema/🦀️.rs`, `✏️editor/👥️presence/🦀️.rs`).

## 3. Proofs (static — no compile)

- `async fn (diff|inverse|label|target|parse_dsl|print_dsl)` under `🧬️schema/` non-test: **0**.
- Same in the three named `ArtifactDsl` files (`📸️snapshot`, `🎚️config`, `👥️presence`): **0**.
- `.await` anywhere in the edited scope: **0** (was 0 before too — the codemod that added `async`
  never added a single `.await`, which is why de-asyncing is a pure signature revert).
- Remaining `async fn` in the edited scope: **51, every one directly preceded by a
  `#[semio_framework_async_macros::async_test]`-style attribute** (0 exceptions, script-verified).
  Kept async deliberately: puzzle's test fns are async too.
- Not touched, per brief: any `🧪️tests/` directory, `🧪️tests/📸️mutate-remodeling-1` (W2),
  `📦️packages/🦀️rust/🦀️.rs`.

## 4. Compile status

**Compile pending: host gate.** Checked twice — at start `swap used 65825M / load 139 / 33 rustc`,
at end `swap used 63128M / load 210 / 56 rustc`. Both far past the gate (swap < 45000M, load < 60),
so no `cargo check` was run and `🗑️generated/w1-check-native.txt` was not produced. Nothing was killed.

## 5. Still-broken async left OUTSIDE W1's scope — needs an owner

The blanket codemod hit the whole subset, not just the schema layer. **199 prod `async fn` remain**,
and they are not merely "unoptimized": the framework calls them synchronously, so they are hard type
errors. Someone must take these before the crate compiles.

| area | free `async fn` | trait impls still async | why it is a compile error |
|---|---|---|---|
| `✏️editor/🦀️.rs` (+ `🎭️modes`, `📌️panels`, `📚️examples`, `🗣️terminology`) | 44 | 10 (`ArtifactEditor`: `app_schema`, `initial_snapshot`, `io`, `export_media`, `import_media`, `command_id`, `command_from_action`, `handle`, `render`, `window_measures`) | trait is sync (§1); and `render`'s match arms call `model::windows::model::render(scene, config)` with no `.await` |
| `✏️editor/🎮️commands/**` | 54 (42 of them `pub async fn handle`) | 0 (they are free fns, not trait impls) | **the brief's premise does not hold**: `app_commands!` generates `pub fn dispatch(…)` (sync) whose arms are `$module::handle(payload, doc, cfg, ctx)` with **no `.await`** (`🔌️plugin/🦀️.rs:10751-10753`). Puzzle's command modules are sync (`pub fn apply`). Left untouched as instructed, but they are broken, not legitimate. |
| `👁️viewer/**` | 7 | 5 (`ArtifactViewer`: `initial_snapshot`, `handle`, `render`; `OpBinary`: `encode_op`, `decode_op`) | traits sync (§1) |
| `🚪️io/**` (importers/exporters, 9 formats × 2 directions) | 75 | 2 (`ArtifactComposition`: `reads`, `compose`) | trait sync (§1) |
| `📚️examples/🎬️demo` | 2 | 0 | called synchronously from the editor |

All 39 `.await` occurrences in the entire subset live in one `#[cfg(test)]` module
(`✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs:1070-1466`), so this remaining wave is also a pure
keyword-strip — `🐍️w1-deasync.py` handles it unchanged by widening `TARGET_DIRS`/`TARGET_FILES`.
