# WP-P6 — Plugin Native Test Residue: Playbook, Norm, Generation3d, Timing Laws, Quick Sweep

Slice P6, session 10. Native only. Captures: `.tmp-ticket/wp-p6/generated/`. Private cargo: `.tmp-ticket/wp-p6/target`.
Inherits the "not mine" residue of `📓️wp-p4.md` and `📓️wp-r1.md`.

## Status

| # | Item | Before | After | Evidence |
|---|------|--------|-------|----------|
| 1 | playbook artifact crate | 142/158, 16 fail | 158/158 | `wp-p6/generated/playbook-before.txt`, `playbook-after.txt` |
| 2 | norm en1990 / din18599 (+5 sibling norm crates with the same stale law) | 155/156, 171/173 | 156/156, 173/173; din16798 523, en1991 320, en1992 323, en1993 214, din4108 234 all pass | `norm-*-before.txt`, `norm-*-after.txt` |
| 3 | generation3d test target compile, then its reds | lib test + `example-geometry` do not compile | compiles; nextest quick 538/560 (21 fail, 1 timeout) → 558/560 | `generation3d-before.txt`, `generation3d-nextest{,-2,-3,-4}.txt` |
| 4 | timing laws under load (lowpoly, mathematical) | equation law fails 4-in-a-row at load 37 (1951 turns, 1.49 s); lowpoly held to a debug-only 32 ms ceiling | equation 392/392 x3 (3611 turns, 190 ms); lowpoly 299/299 x3 at load 31–34 with the exact 8 ms ceiling | `equation-timing-{before,after}.txt`, `equation-after{,-2,-3}.txt`, `lowpoly-timing-before.txt`, `lowpoly-after-{1,2,3}.txt` |
| 5 | test-quick sweep of 34 plugin packages (`nextest --profile quick -- --skip long:: --skip exhaustive::`) | block did not compile (peer rename) | 34/34 packages build; 301 laws, 280 pass, 1 skipped; the only reds are `descriptor_is_fresh` in 21 packages (w1) | `sweep-2.txt` (33 pkgs), `sweep-block.txt` |
| +a | generation2d/3d mounted pack sessions: decoded limits = compressed byte count | latent (p5) | fixed at the root in both, same rule as `RetainedTypedPackSession` | generation3d mounted laws in `generation3d-targeted-2.txt` |
| +b | schema-hash laws on wasm32 | not run (kernel test build pulls native-only CLI bins) | 4/4 on `wasm32-wasip2` under wasmtime, via the carrier crate `wp-p6/schema-hash-wasm32/` | `kernel-schema-hash-wasm32.txt` |
| +d | generation2d (touched by +a and the flow schema rename) | 269/274 | 274/274 | `generation2d-nextest{,-2,-3,-4}.txt` |
| +c | writer `mutate-writer-1` narrative | described the retired hex carrier and a `stdio.json` header | describes the derived record spelling and the real header | files below |

## Findings

### 1. Playbook (16 failures, 5 root causes)

- **`updatePlaybook` was hard-dead in the UI** (`interactive-job.not-ui-safe`, BatchOnlyPendingRewrite). The rationale on
  `PLAYBOOK_RETAINED_TOOL_IDS` ("`Emit::amend` has no truthful lane") was wrong: the retained Artifact lane carries the
  coalesce key onto the store publication (`publication.set_coalesce_key`, and writer's retained `TextEdit` already emits
  `Emit::amend` this way). The title field is now a retained Migrated tool (Artifact lane), so the title edit is one undo step per typing burst.
- **Data loss in persistence.** Since the composition refactor, a playbook's steps lived only in the `flow` handle's
  ephemeral local owner. Neither the text DSL nor the pack carried them, and the framework only asks the parent to *derive*
  child packs (`genesis_child_pack`) from its own state. So every save/load/example load produced an empty playbook. The committed demo
  (`🖼️assets/🎬️demo/🗣️.dsl.semio`) had also been reduced to an empty document ("facade-generator" lost its 3 steps).
  Fix (same pattern P4 used for imperative/wires/raster): the text codec gets a `steps=` line (hex of the steps' JSON), and pack
  encodes a private `PlaybookPackRecord` with a `steps` JSON-text field (block `default`/`params` are order-significant
  `DslValue`s, pack sorts map keys). Both attach the steps to the exact decoded `flow` handle. `record_spec()` = the pack record.
  The demo is regenerated from the last pre-composition source (`6f693a1d06~1`, saved as
  `wp-p6/facade-generator-pre-composition.dsl.semio`) via a one-off test that was removed afterwards. The text grammar
  (`📖️.grammar.semio`, `🅰️.g4`, `🔤️.ebnf`) gains `steps-line`.
- Stale test harness (buckets 2/3/8 of the plugin unit-test debt playbook): unbound instance (`interactive-job.live-instance`),
  no settle after dispatch, store dropped without close witness, registry-less member roster for the convergence pair,
  and a bare `ArtifactStore::new` without owners (`edit history insertion requires its exact mutation retirement factory`).
- Fixture canon: the `replace-block` mutation fixture wrote f64 `min`/`max` as integers. The interactive-job catalog fixture
  was pretty-printed (the owned line parser found 0 rows), was missing `setActiveExample`, and named a factory that no longer exists.

### 2. Norm

- `retained_command_dispositions_match_the_language_neutral_oracle` read `window.actions` directly. After R1/P3 removed
  the app-roster fan-out, that list is empty for app-level actions. It now reads `semio_framework::window_kind_actions`,
  which is the fix a peer had already applied to 8 of the 15 norm crates. I applied it to the remaining 7 (en1990, din18599, din16798,
  en1991, en1992, en1993, din4108).
- din18599 `update-climate/refuses-a-negative-january-irradiance` hard-coded the target `"din18599-climate"`. Every fixture
  names `din18599-climate-fixture-zone2`, and target == child_id is required by `ChildRestoreProjection`. The law now
  compares against `before().climate.target`, which is what its message claims to check.

### 4. Timing laws

- **Equation: a real product defect.** `EquationRetainedCommandWork::step` called `equation_command_extent` on every
  microturn. That call deep-clones the whole working scene (`crate::equation_scene`) and, for `nodeGraphEdit`, re-parses
  the whole edit JSON. The job has one turn per node, edge, point and JSON byte, so every turn cost O(document) and the operation
  was quadratic. The framework's retained driver already measures `ArtifactCommandWork::extent` exactly once, in its
  `Preflight` phase, over the job-owned command and snapshot, so the per-turn re-check was redundant. It was removed. The
  cursor/extent overflow guard stays. Measured on the maximum document (debug, load 37): before, 1951 turns in 1.49 s with
  4 consecutive >8 ms turns. After, 3611 turns in 190 ms, mean 53 µs per turn, and one 92 ms turn from descheduling.
- **The equation law now applies the runtime's own rule** (`StepOverrunLedger`): a session is quarantined only after
  `SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive turns past `INTERACTIVE_STEP_CEILING_US`. An isolated descheduled
  turn is forgiven exactly as production forgives it. A turn whose own cost exceeds 8 ms overruns every time and is
  convicted, and the old code fails this law on every run. This measures the product budget. It is not a looser threshold.
- **Lowpoly:** measured every migrated command on the unit box (debug, load 33). The slowest is `paintFill`/`fillBucket` at 5.3 ms,
  then `unwrapActive` 3.0 ms, and everything else is under 2.1 ms. The debug-only ceiling of 4×8 = 32 ms a peer had added was a blind
  loosening, and I removed it. The law now uses the exact 8 ms ceiling in every build and measures intrinsic cost: the fastest of
  up to 16 attempts, stopping at the first one under the ceiling. At load 26–41 on 10 cores, a 5 ms turn takes 15–40 ms of wall
  time, so best-of-3 could not tell load apart from cost. A command whose own cost is above 8 ms fails all 16 attempts.
- Open (product, not law): on the 195-face default document, whole-mesh kernel ops are single monolithic turns
  (`mirror` 42 ms debug, per the law's own doc). The runtime records each one as an admitted overrun. The real fix is
  kernel ops chunked into resumable steps. That is a kernel slice, and I did not do it here.
- Peer note: process3d has the same per-step extent re-derivation (`process3d-retained-work-extent-drift`, editor line
  ~495). I told p5, who owns process3d.

### 3. generation3d (compile, then 21 reds)

- Compile: the `example-geometry` `[[test]]` used the feature-gated editor and `semio_framework_ui`, but it did not declare
  `required-features`. The package runner always tests with `component-app-assembly`. `frame_orbit_to_bounds` now takes
  (width, height) in pixels. `CommandView.label` is a `LocalizedLabel`. The serial-lock law that names the gated editor and viewer doors
  needs the same gate.
- Window action roster (8 laws, editor, viewer and generate-interactions): same stale read as norm. App-level actions live
  in `AppDefinition::actions` and reach windows through `window_kind_actions`.
- Framing (4 `delivery_*` laws): passing (aspect, 1.0) as (width, height) made the parallel-projection branch think the
  viewport was 1 px tall. The law now passes the real pixel viewport (900·aspect × 900).
- Stale schema id `flow.fixture` in the 8 generation3d example assets and the generation2d demo. The flow host
  snapshot schema is `flow.host_snapshot`, and the loose `||` in a generation2d law was removed.
- Catalogue windowing (2 laws): the host keys a nested window by its full path (`section` + `TREE_WINDOW_PATH_SEPARATOR`
  + `group`). The laws sent the bare group key, so they were never seated.
- `vcs_artifact_app_non_empty_retained_maintenance_swap…`, 3 real product defects:
  1. **Neither the generation3d editor nor the viewer declared `child_restore_projection`**, so every archive load and every
     maintenance swap faulted with "editor did not declare a loaded-parent child projection". Both now declare the empty
     projection (the snapshot has no child slot).
  2. The law fed the semio-container `encode` into the mounted ingress, which requires the `P3D3` stream. It now uses
     `encode_mounted`.
  3. It dropped the rejected candidate snapshot without retiring it (neural `Dictionary` ownership panic).
- Mounted pack session limits (scope item a): `max_file_len`/`max_segment_len`/`max_total_alloc` and the catalog symbol
  byte bounds were the canonical (compressed) byte count, so a deflated segment larger than its compressed size was refused
  as `segment-malformed`. In both generation2d and generation3d they are now bounded by `ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES`,
  the same rule as p5's framework `RetainedTypedPackSession`. The source cursor still enforces the exact byte count.
  Collapsing both sessions onto the framework session is the cleaner end state, and I left it as an open item.
- Generation preview (1 law): commit 611 deliberately moved the generation preview to the per-window evaluation
  publication. No route writes `SetGenerationPreview` any more, but 6 publication contracts still declared a
  `Transient` lane, and the generate-preview renderer's parameter still carried the old name. The contracts now declare what
  the tools write, the parameter is `preview_eval_text`, and the law asserts `(artifact 1, config 1, transient 0)` plus that the
  two windows render one evaluation. Open: `Generation3dTransient.generation_preview_text` and its mutation have no writer
  and no reader. Deleting them needs `Transient = NoTransient` plus 5 schema assets and a fixture.
- `hex_column_boot…` ledger: the publication gate decides per publication target, so `considered == ticks × attached
  previews` (2), not `== ticks`. The tick-cost part of that law had the same load problem as lowpoly (3 ticks per boot; the best
  was 9.9 ms at load 30). It now measures up to 8 fresh boots and stops at the first tick under 8 ms. The ceiling is unchanged.
- Still red: `delivery_sphere_box_fuse` delivers 428 triangles against a pinned floor of 440 (volume and union laws
  pass). The floor was pinned at commit 625, and the brep tessellation changed since. I did not lower it without the kernel
  owner's reason. `slider_values::every_moved_slider_converges…` **hangs**. Run alone under the long profile, it timed out at
  900 s (`generation3d-slider.txt`). The in-process oracle table for every fixture value completes, and the hexagonal and rectangle rows
  converge on all 5 gestures each. `sphere-cut-with-torus` boots correctly, and then its first slider gesture's preview run never returns.
  It is the same brep boolean family as the fuse triangle drift. Open, for the brep/extension owner. I did not investigate further inside P6.

### 5. Sweep

- Script: `wp-p6/sweep.sh` (package list `wp-p6/plugin-packages.txt`, `SWEEP_EXCLUDE` for one package).
- The only non-descriptor red was a compile break. t2's block-5d kind-dir rename (`update-part-2d` → `update-part2d` and 4
  siblings) left 20 `#[path]` attributes in `🧱️block/🗿️artifacts/🖐️5d/🦀️.rs` on the old names. I repointed them, and t2 confirmed.
- `descriptor_is_fresh` fails in 21 packages: animate, block, cad, demonstrator, imperative, layout, lowpoly, mathematical,
  playbook, procedural, process, puzzle, raster, reasoning, sequence, shooting, sourcing, space, stdio, trinity,
  writer. These belong to w1 (requests in `wp-w1/requests/p6.txt` for playbook and procedural).

### generation2d (+d)

- The same missing `child_restore_projection` as generation3d, on both the editor and the viewer.
- **Product defect:** `Generation2dRetainedMutationRetirement::close_step` bare-dropped retired history mutations, and a
  `create-widget`/`replace-widget` owns a `Widget` whose neural `Dictionary` aborts the process on drop. That happens on every store
  close with widget history. New `Generation2dMutation::retire_cold` (the generation3d twin), and the retirement routes through it.
- Stale laws: `setActiveExample` (now a bounded tool) was missing from `every_command`, from the keyword table (`active-example`) and
  from 3 roster counts. The German catalogue law looked for the retired "Quellen" section, where the catalogue section is now
  "Komponenten". The `generation2d_labels.sources` label no longer has a product reader, and I left it open. Two laws dropped
  a live `FlowEvalSession` and an `initial_snapshot` without closing or retiring them.

## Changes

- playbook `✏️editor/🦀️.rs`: `updatePlaybook` added to retained ids, contracts (Artifact), extent, proofs, manifest Migrated; doc fixed.
- playbook `🧬️schema/📸️snapshot/🦀️.rs`: `PlaybookPackRecord`, text `steps=` line, strict line parser; `DslRecord` derive removed from the snapshot.
- playbook `📝️text/{📖️.grammar.semio,🅰️.g4,🔤️.ebnf}`, `🖼️assets/🎬️demo/🗣️.dsl.semio` (regenerated).
- playbook tests: editor harness (self-closing bound `PlaybookApp`, settle, LoadDocument, `history_verb`, retired render),
  add-step/add-block/builder-window tests, binary envelope test (owners + close), pack law now round-trips the demo's steps,
  catalog law also pins the retained roster. Fixtures: `🧫️interactive-jobs/🔣️.json`, `replace-block` mutation floats.
- norm: 7 editor unit test files (window roster), din18599 climate law.
- equation `✏️editor/🦀️.rs` (`step` no longer re-derives the extent; doc) and law `retained_maximum_microturns_stay_below_eight_milliseconds`.
- lowpoly law `retained_migrated_turns_stay_below_eight_milliseconds` (exact ceiling, intrinsic-cost attempts).
- generation3d: `Cargo.toml` `example-geometry` gets `required-features = ["component-app-assembly"]` (it uses the
  gated editor + `semio_framework_ui`, and the package runner always tests with that feature); `frame_orbit_to_bounds`
  call gets width/height (aspect, 1.0); `CommandView.label` is a `LocalizedLabel` (en/de); the serial-lock law that names
  the gated editor/viewer doors is gated on the same feature.
- generation3d/generation2d: see §3 (Cargo.toml, tests, examples, editor/viewer `child_restore_projection`, contracts,
  generate-preview param, mounted-session limits in both).
- writer `mutate-writer-1`: `🥒️.feature`, `🐍️.py`, `🔮️oracles/🔣️.json` (text only; JSON and Python validated).
- New `wp-p6/schema-hash-wasm32/` (standalone `[workspace]` carrier crate that mounts the kernel's `pack_schema_hash`
  test file). I tried letting the kernel's pack/spr CLI modules build on WASI and reverted it: they need `FilePackSource`, `write_atomic` and
  `compact`, which are also native-only.
- W1 request: `wp-w1/requests/p6.txt` (playbook descriptor regen: classification + example changed).

## Open

- generation3d `delivery_sphere_box_fuse` (428 < 440 triangles) and `slider_values::every_moved_slider_converges…`
  (hang after the `sphere-cut-with-torus` boot). Both need the brep/extension owner.
- The generation2d/3d mounted sessions are fixed in place. The end state is to collapse them onto `store::mounted_pack_rt::RetainedTypedPackSession`.
- Dead state: `Generation3dTransient.generation_preview_text` and `SetGenerationPreview` (no writer, no reader), and the
  `generation2d_labels.sources` label.
- Kernel `pack`/`spr` CLI bins still keep the kernel's own `--test pack_schema_hash` off wasm32. The carrier crate measures it
  instead. The root fix is the kernel owner's (bins behind `required-features`, or the CLI file layer made WASI-capable).
- Descriptors (w1): playbook (classification and example) and procedural (examples), plus every package listed in §5.

## Not Run

- wasm32 `cargo check` of the touched plugin crates. I queued it behind w1 (holding since 08:43) and h4, then withdrew it (my pid
  66523, killed; queue entry removed). The edits are target-neutral Rust. W1's describe chain rebuilds these guests anyway.

## Processes

- None running. All detached nextest runs (pids 65974, 77918, 99350, 28340, 34799, 49194, 50474) have exited. The wasm-mutex wrapper 66523 was killed by me.
