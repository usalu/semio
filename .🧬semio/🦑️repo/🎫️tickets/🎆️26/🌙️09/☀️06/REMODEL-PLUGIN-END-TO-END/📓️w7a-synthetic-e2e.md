# W7a — synthetic multi-view example + end-to-end reconstruction test

Closes `📓️explore-engines-io-realworld.md` §2.1/§4: every shipped remodeling document has zero
frames, so `run-reconstruction` short-circuits and the ~25.7k-LOC engine stack is never exercised by
a committed fixture. This lands the first document that drives it, plus the ground truth to score
the run against.

## 1. Files

| path (all under `✏️s/🔌️plugins/📸️remodel/`) | status |
|---|---|
| `🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🦀️.rs` | new — example module (`ID`, `label()` en/de, `PRIMARY_TEXT`, `GROUND_TRUTH_JSON`, `FRAMES` `include_bytes!` table, `source()`) |
| `…/📚️examples/🛰️synthetic-orbit/🧪️tests/🦀️.rs` | new — reference renderer, ground-truth loader, Sim(3) scorer, 5 tests + the regeneration routine |
| `…/📚️examples/🛰️synthetic-orbit/🟦️.ts` + `🧪️tests/🟦️.ts` | new — TS twin (see §7: `registry:check` hard-requires both leaves per example dir) |
| `…/📚️examples/🛰️synthetic-orbit/🖼️assets/🎞️frame-00…09.png` | new — 10 committed views |
| `…/📚️examples/🛰️synthetic-orbit/🖼️assets/🔮️ground-truth.json` | new — intrinsics, per-frame extrinsics, 140 world points |
| `…/📚️examples/🛰️synthetic-orbit/🖼️assets/🗣️.dsl.semio` | new — the remodeling document |
| `📦️packages/🦀️rust/🦀️.rs` | edited — ONE mount insertion (`subsets::any::examples::synthetic_orbit`) next to `demo`, plus a 1-line `#[cfg(test)]` fix on the `demo` test mount (§6a) |
| `…/✳️any/✏️editor/⚙️engine/🦀️.rs` | edited — ONE line deleted: an orphaned `#[cfg(test)]` that was cfg-gating `build_engine_params` out of every non-test build (§6a) |
| `📦️packages/🦀️rust/📜️script.ts` | edited — new `regenerate-example` command |
| `📦️packages/🦀️rust/📋️project.json` | edited — new `regenerate-example` nx target (project.json may only call `📜️script.ts`, so the target is a one-line passthrough) |
| ticket `🐍️synthetic-orbit-fixture.py` | new — stdlib-Python reference implementation (oracle + bootstrap) |

`📦️packages/🦀️rust/Cargo.toml` **unchanged**: the alignment maths uses the crate's own
`crate::lie::umeyama`/`Sim3`, and the third-party oracle role is already filled by the existing
`png 0.17.16` dev-dependency. No `nalgebra`/`approx` were added — no lockfile churn.

## 2. The fixture

Deterministic, single-seeded (`SEED = 0x5EED0B175CE9E000`, splitmix64), no external inputs:

- Scene: origin-centred 2 m textured cube, 6 faces × 22 seeded circular markers + 8 corners ⇒ **140
  ground-truth world points**. Non-planar by construction; the orbit phase is offset half a step so
  every view shows two faces meeting at an edge rather than one face head-on.
- Cameras: 10 on a jittered orbit, horizontal radius 2.6 m, elevation 2.2 m, looking at the origin.
- Optics: 320×240, `fx = fy = 0.85 · width = 272`, `cx/cy = 160/120`, Brown–Conrady `k1 = -0.02`,
  `k2 = 0.005` — recorded verbatim in `calibration.cameras` and in the ground truth.
- Renderer: analytic ray/box intersection through `unproject_ray` + the exact 8-step Newton
  undistortion of `📷️camera/🦀️.rs`, so rendered pixels are consistent with the plugin's own camera
  model rather than an approximation of it.

Naming: `🛰️synthetic-orbit` satisfies `taxonomy.exampleSlugPattern`, the only example-naming rule
any script enforces (`🔌️plugin/📇️registry/📜️script.ts:151`). Frame files use a descriptive `🎞️`
prefix rather than `exampleMediaKindPrefixes.image` (`🖼️`) — that map is declared but never enforced,
and existing examples already prefix by content.

**Sizes** — 64 KB total (budget was 1.5 MB): PNGs 1.9–3.2 KB each (~24 KB), ground truth 17 KB,
document 2.6 KB.

**Determinism proof** — the generator was run twice and the asset directory hashed each time:
`shasum -a 256 * | shasum -a 256` → `24cbc1bde0d2d12906ee4e83a23a70bd8fc61788ffc4f895eb3a9c0bee933503`
both times (identical bytes, second run after a full rewrite of all 12 files).

## 3. Two generators, on purpose

The committed bytes were produced by the ticket-folder Python script (stdlib `zlib` PNG encoder) so
the example directory is complete and the crate compiles before remodel has ever been built. The
**permanent** generator is the `#[test] #[ignore] regenerates_the_synthetic_orbit_example` in the
example's own `🧪️tests/🦀️.rs`, exposed as `bun ./📜️script.ts regenerate-example` /
`nx run @semio-tech/remodel-plugin:regenerate-example`; it re-renders with the plugin's own PNG
encoder and prints the document through the real `store::ArtifactDsl::print_dsl`. PNG *container*
bytes differ between the two encoders; the *decoded rasters* must not, and
`committed_frames_match_the_png_oracle_and_the_reference_renderer` asserts exactly that — which makes
the Python file a real cross-language oracle rather than a duplicate. Until the Rust generator runs
once, the committed `🗣️.dsl.semio` is Python's hand-printed mirror of `print_dsl`, and
`dsl_declares_the_ground_truth_camera_and_the_full_frame_table` is the check that it is faithful.

## 4. Tests

In `…/📚️examples/🛰️synthetic-orbit/🧪️tests/🦀️.rs`:

1. `primary_asset_is_nonempty` — id/text/frame-count smoke.
2. `dsl_declares_the_ground_truth_camera_and_the_full_frame_table` — parses the document; camera
   `fx/fy/cx/cy` and all five distortion slots must equal the ground truth's; stream kind, camera id,
   frame count, asset ids and timestamps must match `FRAMES`.
3. `committed_frames_match_the_png_oracle_and_the_reference_renderer` — every frame decoded by the
   `png` crate **and** by `remodeling_image::decode_png` must agree, and both must equal a
   from-scratch re-render.
4. `reconstructs_the_synthetic_orbit_against_ground_truth` — imports all ten frames through the real
   `import-frame-payload` handler, dispatches `run-reconstruction`, follows the `advanceReconstruction`
   continuation to a terminal phase inside the 200 000-tick ceiling. Asserts: every frame reached the
   document; progress never decreases; stage ≠ `Failed`; sparse cloud ≥ 25 % of 140 truth points;
   ≥ 3 trajectory poses; Sim(3)-aligned rotation error and translation RMSE within bound (§5); the
   placeholder mesh was replaced.
5. `cancel_requested_mid_run_terminates_the_synthetic_orbit_reconstruction` — same driver with
   `cancel-reconstruction` at tick 3; asserts continuations stop well inside the ceiling,
   `job.cancel_requested`, no committed sparse results, placeholder mesh untouched.
6. `regenerates_the_synthetic_orbit_example` — `#[ignore]`, the generator.

**Scoring method.** `terminal_sparse_chunk`/`camera_pose_preview` expose recovered poses in
*registration* order under a synthetic `cam-{i}` id — the originating frame is not recoverable (that
function's own docstring admits it), so the test searches the correspondence rather than assuming it:
every ordered triple of truth cameras seeds a 3-point `crate::lie::umeyama` fit, each seed is refined
by two greedy nearest-assignment + full-Umeyama rounds, lowest residual wins. Rotation error is the
geodesic angle between aligned and truth camera-to-world rotations; translation RMSE is normalised by
the widest truth camera separation.

## 5. Finding: document calibration never reaches the engine

`build_engine_params` (`✏️editor/⚙️engine/🦀️.rs`) maps all eight `ReconstructionParams` sub-structs
but leaves `EngineParams::assumed_focal_ratio` at its default `1.0` and takes no `CalibrationState`
argument at all — its own docstring calls this a documented simplification. So an app-level run
reconstructs this fixture through the engine's `fx = fy = max(w, h)` guess (320) instead of the true
`0.85 · width` (272): a 17.6 % focal error. `🏭️reconstruction/🦀️.rs:2038-2065` already records this
bug class from the inside — *"default `assumed_focal_ratio` of `1.0` against a `0.85` rendering camera
produced a reconstruction ~3× too large"* — and its own fixture pins `assumed_focal_ratio` to avoid it.

The fixture was **not** tuned around the gap; it renders at the realistic 0.85. Instead the app-level
pose assertions carry an explicit `UNCALIBRATED_GAUGE_SLACK = 12.0` multiplier on the strict 2° / 3 %
bounds, with a docstring saying to drop it to `1.0` once calibration is wired. The fix belongs to
whoever owns `🎮️commands/**` (W4) plus `⚙️engine/🦀️.rs`: pass `build_engine_params` the
`CalibrationState` and the first frame's dimensions, then set
`assumed_focal_ratio = cameras[0].fx / max(w, h)`. W7a did not make it — the call site is in
`🏗️run-reconstruction/🦀️.rs`, outside this packet's lease.

## 6. Build / run status

**RUN PENDING — the crate does not currently build.** The central
`cargo check -p semio-s-plugin-remodel --lib` (pid 47704) finished at 05:25 after 47 min with
`error: could not compile semio-s-plugin-remodel (lib) due to 128 previous errors; 39 warnings`.
The host then went to load 68 / swap 25.9 of 26.6 GB, i.e. below this lane's own run gate
(load < 40, swap < 16 GB), so W7a executed no `cargo` of its own. Everything reported as *measured*
(asset sizes, determinism hashes, rendered imagery) was measured through the Python generator; every
Rust test is written but **not yet compiled or run**.

Pre-existing error classes in that log, none of them from this packet's files: 61 × E0277,
15 × E0283, 13 × E0433, 11 × E0053, 7 × E0425, 5 × E0432, 3 × E0063, 2 × E0308, 2 × E0046, 1 each
E0599/E0080. By location: `✏️editor/📌️panels/**` 38, `⚙️engine/🥽️mesh` 14,
`🧬️mutations/{🏁commit-reconstruction,🧷create-asset}/🔺️diff` 14, `🚪️io` 3, `⚙️engine/🎥️video` 3 —
i.e. W4's panel/trait reshaping, W2b's diff work and W6's IO rewrite, all mid-flight.

### 6a. Two defects found in that log and fixed here

- **`build_engine_params` was cfg-gated out of every non-test build.** `⚙️engine/🦀️.rs:16` carried an
  orphaned `#[cfg(test)]` (left behind when W1's de-async codemod removed the `use` line beneath it);
  attributes bind to the next *item*, and `//#region 🔖️EngineMapping` is a comment, so it landed on
  `pub fn build_engine_params`. That produced `E0432: no build_engine_params in
  editor::remodeling::engine` at `🏗️run-reconstruction/🦀️.rs:11`, which takes the whole
  commands tree with it. One line deleted. A repo-wide scan of the plugin found three more
  `#[cfg(test)]`-before-a-region-comment sites (`🧬️schema/💡️inferences/**`); all three legitimately
  bind to a following `mod tests`, so they were left alone.
- **`📚️examples/🎬️demo`'s test mount is not `#[cfg(test)]`-gated**, so its `semio_framework_async_macros`
  (a dev-dependency) resolves in a plain `--lib` build and does not exist there: 3 × E0433. Added the
  missing attribute. The `synthetic-orbit` mount carried it from the start, which is why this packet's
  much larger test module adds nothing to the non-test build.

First-run risks, in order:

1. The 128 pre-existing errors above must clear first; none are in this packet's files, but they
   prevent any test from linking.
2. The hand-printed bootstrap DSL may not parse exactly as `print_dsl` would (test 2 is the detector;
   `regenerate-example` is the fix). Two printer conventions were reverse-engineered from the DSL
   module's own round-trip tests rather than guessed: a nested `TABLE` cell inside a table row prints
   inline as `[ {k=v k=v} {k=v} ]` (not as a header + braces block — `🗣️dsl/🧬️schema/🦀️.rs:2945`),
   and an `f32` document field prints its exact widened `f64` expansion (`-0.02f32` →
   `-0.019999999552965164`), so the committed camera row carries the expansions, not the literals.
3. Test 4 drives 10 frames of 320×240 through a `RECONSTRUCTION_STEP_BUDGET = 1` continuation at the
   **app** layer (one dispatch + snapshot per engine unit). The nearest existing test runs 48 frames
   at 128 px but drives the engine directly — test 4 may be slow enough to want a `long` classification.
4. `UNCALIBRATED_GAUGE_SLACK`, `MIN_SPARSE_POINTS_PER_TRUTH_POINT` and the tick ceiling are stated,
   not measured. The first green run should replace them with observed values.

## 7. Hand-offs

**W4 — example registry.** Add exactly this entry beside the `demo` one (the subset facet is mounted;
the top-level `pub mod examples` alias was deliberately left untouched to avoid colliding with the
registry work):

```rust
crate::artifacts::remodeling::standards::v1::subsets::any::examples::synthetic_orbit::source()
```

with `ID = "synthetic-orbit"`, label `"Synthetic Orbit"` / `"Synthetischer Orbit"`, icon `"camera"`.
Note that selecting it must also bind the media: dispatch one `import-frame-payload` per
`…::synthetic_orbit::FRAMES` row (payload = `data:image/png;base64,<bytes>`, `index` = row index),
otherwise the loaded document has a frame table pointing at absent assets.

**W3 — TypeScript twin + launch entry.** `📇️registry:check` emits a hard finding for any example
dir missing `🟦️.ts` (`🔌️plugin/📇️registry/📜️script.ts:1332`), so W7a created
`📚️examples/🛰️synthetic-orbit/🟦️.ts` and `🧪️tests/🟦️.ts` itself rather than land a knowingly-red
gate — they are new files in a directory only this packet touches, so there is nothing to merge, but
they are yours to reshape (the vitest twin checks PNG magic bytes and cross-checks the ground truth
against the declared frame table). `launch.json` is yours: please add a
`🛠️build🏺️remodel🛰️synthetic-orbit` (or equivalently-named) entry running
`nx run @semio-tech/remodel-plugin:regenerate-example`, following the existing grouping — the nx
target and the `📜️script.ts regenerate-example` command already exist.

**Coordinator.** First person with a compiling remodel crate should run
`cargo test -p semio-s-plugin-remodel --lib synthetic_orbit` and then
`bun ./📜️script.ts regenerate-example` (from `📦️packages/🦀️rust`) to make the Rust generator
authoritative over the Python bootstrap, and record the observed pose/sparse metrics back into §6.
