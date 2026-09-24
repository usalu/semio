# WP-P7 — Brep Fuse Floor, Slider Hang, Mounted Sessions onto the Framework, Kernel CLI Bins off wasm32

Slice P7, session 10. Follows `📓️wp-p6.md` (Open) and `📓️wp-p5.md`. Captures: `.tmp-ticket/wp-p7/generated/`.
Private cargo: `.tmp-ticket/wp-p7/target`.

## Status

| # | Item | Before | After | Evidence |
|---|------|--------|-------|----------|
| 1a | generation3d `delivery_sphere_box_fuse` triangle floor | 428 < 440 | floor 428, justified by OCCT (see §1a); kernel law pins OCCT's face count | `kernel-booleans-1.txt`, `kernel-booleans-2.txt`, `occt-sphere-box-fuse.txt` |
| 1b | `slider_values` hang on `sphere-cut-with-torus` | 900 s timeout | root cause: budgeted evaluator livelock on an uncached failing node (fixed in the neural engine) + kernel cavity-winding defect (fixed) | `slider-torus-2.txt`, `slider-all-{1,2,3}.txt`, `neural-budgeted.txt` |
| 2 | generation2d/3d mounted pack sessions onto `RetainedTypedPackSession` | bespoke (~560 lines each) | deleted; both are `RetainedTypedPackSession<…TypedSnapshotOwner>` aliases + one constructor fn | `generation3d-nextest-2.txt`, `generation2d-nextest-1.txt` |
| 3 | kernel `pack`/`spr`/`semio` CLI bins off wasm32; schema-hash laws in the kernel on wasm32 | carrier crate `wp-p6/schema-hash-wasm32/` | bins behind `native-bin`; kernel's own `--test pack_schema_hash` 4/4 on wasm32-wasip2 (wasmtime); carrier crate deleted | `kernel-schema-hash-wasm32.txt` |
| 4 | generation3d / generation2d / kernel suites | 558/560 (1 fail, 1 timeout), 274/274 | generation3d **560/560** (slider law 99.6 s), generation2d **274/274**, kernel `--lib` + `--test pack_schema_hash` **1136/1136**, kernel schema-hash on wasm32 **4/4** | `generation3d-nextest-2.txt`, `generation2d-nextest-1.txt`, `kernel-lib-nextest.txt`, `kernel-schema-hash-wasm32.txt` |

## Findings

### 1a. `sphere-box-fuse`: no lost faces, the floor was a measurement of an older boolean result

- The fuse result has **7 faces: one sphere (6 coedges) and six planes** (3 trimmed squares with 5 coedges, 3 untouched
  with 4). OCCT (brepjs 18.119.8 over brepjs-opencascade, script `wp-p7/📜️occt-sphere-box-fuse.ts`) fuses the same
  operands (`sphere(1.2)` at the origin + `box(1.5³)` from the origin) into **7 faces: SPHERE + 6 PLANE**, volume
  `9.70845078963702` (exactly the fixture's analytic value). Nothing is lost.
- At the delivery tolerance 0.05 our tessellation is 428 triangles (sphere 407, planes 5+2+2+2+5+5), and OCCT meshes the
  identical fuse at the same linear deflection with **312 triangles** (204 vertices; 4262 at 0.0025). Our count is
  above the third-party count at the same tolerance.
- Geometric fidelity measured per triangle (sag of each sphere triangle's centroid below the analytic sphere): worst
  0.062 at tol 0.05 (at the +z pole fan), 0.0034 at tol 0.0025. parry3d volumes: 9.4354 at 0.05, 9.6928 at 0.0025, against 9.7085.
- The tessellator has not changed since 2026-09-10. The floor was pinned 2026-09-12 (`8add1df147`), and the boolean was
  rewritten after that (`6f33e313da` 09-15: deep copies of both operands, scoped orphan sweep; `5a0b1ace28` 09-16). The
  12-triangle drop comes from the different result topology/boundary sampling of that rewrite, not from a missing piece.
- Floor now 428 (the measured count, still a regression guard ≥ OCCT's 312). Once the triangle assertion passed, the same law's
  delivered-extent row (pinned 2026-09-15 09:46, before both boolean commits) failed too: the delivered min corner is now
  `(-1.1817693, -1.1817693, -1.2)`, x/y-symmetric like the operands, where the old pin had `(-1.1862875, -1.1817694, -1.2)`. It stays inside the
  analytic extent (`expect.boundingBox*`, which the same law checks), and the fixture row is re-pinned. The kernel law
  `sphere_box_fuse_example_is_a_closed_oriented_solid` now also pins OCCT's face count (7), so a lost boolean piece fails
  at the kernel level instead of as a triangle-count drift.
- Side observation (not changed): `TessellationReport.max_chordal` reads 0.61 at tol 0.05 for a PLAIN sphere too
  (0.15 at 0.0025), while no triangle's actual sag exceeds 0.062. The certificate is wrong near the poles
  (pole-fan triangles), not the mesh. Open for the tessellation owner.

### 1b. The slider "hang" is an evaluator livelock, exposed by a kernel refusal

- Kernel probe over every radius the `sphere-cut-with-torus` rows touch (torus 2.0/0.5): nothing hangs. r=2.5 (sphere tangent to
  the torus's outer equator) is refused in 4.7 ms (`imprint segment midpoint not found`). r ≥ 2.8 is answered by the containment fast
  path in < 1 ms, r=1.0/1.5/2.2 run the general job in 51–346 ms (debug).
- Run alone (`SEMIO_SLIDER_EXAMPLES=sphere-cut-with-torus`), the law finished in 8.6 s. Run with the full roster it hung. Turn
  tracing in `drive_preview_run` plus `evaluate_tick` shows why: after the boot, the host evaluated the graph IN-PROCESS
  (`registry.dispatch`, `parked=0`, `answered=0`) and on every `flowEvalTick`:
  `brep_bool_cut_5: error (imprint failed … face-278-2)`, then `face-406-0`, `face-697-0` … and `brep_measure_volume_2: computing`, `more=true`.
- Root cause (`🧠️neural/⚙️engine` `evaluate_channels_budgeted`): a successful dispatch is seeded into the `NeuralCache`, a
  FAILED one was not. The tick deadline expires after the expensive refused boolean, the walk stops before
  `brep.measure.volume`, and the next tick re-dispatches the same failing boolean first. It is a livelock. It also leaks one set of
  imprinted working copies into the kernel body per tick, since the face ids keep climbing. The extension path already caches an error
  answer (`resolve_eval` → `seed_node_cache`), which is why the torus-only run (served through the extension) converged.
  Fix: the budgeted walk seeds the failure dictionary like a success. New law
  `evaluate_channels_budgeted_serves_a_failed_dispatch_from_the_cache_on_resume` (budget 1: tick 1 fails `a`, tick 2 serves `a`
  from the cache and converges on `b`; 2 dispatches total).
- Kernel defect found on the way (it made every `inside`/`max-end`/`burst` torus value unpaintable, and oracle and chain agreed on it,
  so the law could not see it): the containment fast path's `Cut` answers a cavity but left the tool's faces wound OUTWARD, so
  every r > 2.5 failed validation with `void-shell-not-inverted`. The inner shell's faces are now flipped. New kernel law
  `sphere_cut_with_torus_past_the_tube_is_a_ball_with_a_torus_cavity` (r = 2.8, 3, 3.5, 6, 8, 10: validate clean, χ = 2,
  volume = 4/3·π·r³ − 2π²·R·r² within 5e-3, parry3d agrees and is positive). OCCT agrees with the closed form: its cut at r = 3 has
  2 faces and volume 103.22773112814316 (closed form 103.22773112814318), `wp-p7/📜️occt-sphere-cut-torus-tangent.ts`. At the
  tangent r = 2.5, OCCT's own cut is degenerate as well (3 faces, volume −9.8696), so our named refusal there stands.
- Progress + cancellation of expensive booleans: the served route (extension `evaluate` with `budget`/`nodeHash`) runs every
  `brep.bool.*` as `BrepBooleanOperatorJob` (`step_plan`), with `OperatorProgress` and `cancel` (`evaluateCancel` on release).
  The in-process `registry.dispatch` route that the full-roster harness fell onto calls `evaluate` → `boolean_sync` in one go.
  Why the full-roster process ends up with in-process brep operators, while the torus-only one reaches the extension, is not
  investigated here (see Open).

### 3. Kernel CLI bins

- `pack`, `spr` and `semio` are `[[bin]]`s of `semio-framework-os-kernel`, and their `cli` modules are mounted only off wasm32. Cargo builds every bin
  for an integration test target, so `--target wasm32-wasip2 --test pack_schema_hash` failed before any test ran. All three
  now carry `required-features = ["native-bin"]` (the renderer's existing `native-bin` precedent). Callers: root `📜️script.ts`
  `SemioScript` now runs `-p semio-framework-os-kernel --features native-bin --bin semio`. It named the non-existent crate
  `semio-framework-os-kernel-semio`, so it was broken. `rustWarningTargetScope` passes `--features native-bin` to the
  kernel's native `--all-targets` check, so the bins stay linted.
- The laws live in the kernel's own integration test `[[test]] pack_schema_hash`. The kernel `--lib` test target still does not
  build for wasm32 (peers' native-only directory tests, `📓️wp-p5.md`), so the lib target was not the carrier.

## Changes

- `🧠️neural/⚙️engine/🦀️.rs`: budgeted walk seeds a failed dispatch into the cache; doc. `🧪️tests/🔬️unit/🦀️.rs`: new law.
- brep kernel `🔺️diff/🔀️boolean/🦀️.rs`: containment-cut cavity faces flipped; doc on `trivial_topology_fast_path`.
- brep kernel `🧪️tests/🧲️procedural-example-booleans/🦀️.rs`: cavity law; OCCT face-count assertion on the fuse law.
- generation3d `📚️examples/🧲️sphere-box-fuse/🧫️fixtures/🧩️example/🔣️.json`: `delivery.minTriangles` 440 → 428.
- generation3d `🧬️schema/📸️snapshot/💾️binary/🦀️.rs`: bespoke `Generation3dMountedPackSession`/`…CloseStep`/`…Phase` deleted;
  `Generation3dMountedTypedSnapshotOwner` implements `RetainedTypedPackOwner`; `pub type Generation3dMountedPackSession =
  RetainedTypedPackSession<…>` + `generation3d_mounted_pack_session(expected, items)` (header `P3D3`, depth 64).
  `🧬️mutations/💾️binary/🦀️.rs` and the mounted laws use it; the test-only FNV ingress ledger assertion is gone (the exact
  round trip already proves the stream reaches the cursors unchanged).
- generation2d: the same, header `P2D2` (`🧬️schema/📸️snapshot/💾️binary/🦀️.rs`, `🧬️mutations/💾️binary/🦀️.rs`, both law files).
- kernel `📦️packages/🦀️rust/Cargo.toml`: `native-bin` feature; `required-features` on `pack`, `spr`, `semio`.
- root `📜️script.ts`: `SemioScript` crate/feature, `rustWarningTargetScope` kernel `native-bin`.
- Deleted: `.tmp-ticket/wp-p6/schema-hash-wasm32/` (P6's carrier crate).
- generation3d/generation2d `✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs`: the static verifier required the five retained cursor
  names inside the artifact's mounted region. They now live in the framework session, so it requires the framework session alias, the
  owner trait impl and the artifact header instead. The whole-decoder forbidden list is unchanged.
- generation3d `🧲️sphere-box-fuse` fixture: `delivery.boundingBoxMin` re-pinned (§1a).
- Ticket inputs kept: `wp-p7/📜️occt-sphere-box-fuse.ts`, `wp-p7/📜️occt-sphere-cut-torus-tangent.ts` (OCCT oracles).

## Test Results

| crate | command | result | capture |
|---|---|---|---|
| semio-framework-os-kernel | `cargo test --target wasm32-wasip2 --test pack_schema_hash` (wasm mutex, `CARGO_TARGET_WASM32_WASIP2_RUNNER=wasmtime`) | 4/4 passed | `kernel-schema-hash-wasm32.txt` |
| semio-framework-os-kernel-neural-engine | `cargo test --lib evaluate_channels_budgeted` | 8/8 passed (incl. the new law) | `neural-budgeted.txt` |
| semio-s-artifact-stdio-semio | `cargo test --test brep_procedural_example_booleans` | 4/4 passed (cavity law incl.; plus the probe, since removed) | `kernel-booleans-1.txt` |
| generation3d | slider law, torus only, before fixes | passed in 8.6 s (oracle and chain agree on `void-shell-not-inverted`) | `slider-torus-2.txt` |
| generation3d | slider law, full roster, before the engine fix | reproduced: >2000 `flowEvalTick` turns after the torus boot | `slider-all-1.txt`..`-3.txt` |
| generation3d | `cargo nextest run --features component-app-assembly --profile quick` (after all fixes) | 560/560 | `generation3d-nextest-2.txt` |
| generation3d | same, before the fixture/verifier updates | 558/560: fuse bbox (see §1a), static verifier (see Changes) | `generation3d-nextest-1.txt` |
| generation2d | `cargo nextest run --features component-app-assembly --profile quick` | 274/274 | `generation2d-nextest-1.txt` |
| semio-framework-os-kernel | `cargo nextest run --lib --test pack_schema_hash` | 1136/1136 | `kernel-lib-nextest.txt` |
| semio-framework-os-kernel | `cargo check --features native-bin --bins` | Finished | `kernel-bins-check.txt` |
| semio-framework-os-kernel-neural-engine | `cargo nextest run` | 57/57 | `neural-nextest.txt` |
| semio-s-artifact-stdio-semio | `nextest -E 'binary_id(/brep/) \| test(/brep/)' --profile quick` | 650/651. The one fail is `fillet_of_all_twelve_box_edges_stays_inside_the_interactive_budget` (356 ms > 200 ms at load 28–32, no boolean involved); it passed alone | `stdio-semio-brep-nextest.txt`, `blend-budget-rerun.txt` |
| OCCT | `bun wp-p7/📜️occt-sphere-cut-torus-tangent.ts` | r=3: 2 faces, V 103.2277 = closed form; r=2.5: degenerate (V −9.87) | `occt-sphere-cut-torus-tangent.txt` |
| OCCT | `bun wp-p7/📜️occt-sphere-box-fuse.ts` | 7 faces, 312 tris @0.05, 4262 @0.0025, V 9.70845 | `occt-sphere-box-fuse.txt` |

## Open

- Why the full-roster slider process evaluates `brep.*` IN-PROCESS (`registry.dispatch`, a synchronous `boolean_sync` with no
  progress/cancel), while the torus-only process reaches the extension (`evaluate` + `BrepBooleanOperatorJob` with progress and
  cancel). Some earlier example or oracle path leaves native brep operators in the process-wide flow registry. The served wasm
  product always takes the extension route. It only matters for native harnesses, and the evaluator livelock that turned it into a
  hang is fixed.
- Tangent sphere/torus cut (r = 2.5) is refused by the kernel (`imprint segment midpoint not found`); OCCT is degenerate there too.
- `TessellationReport.max_chordal` over-reports near sphere poles (0.61 for a plain sphere at tol 0.05, while the worst triangle sag is 0.062).
- generation2d and generation3d `…MountedTypedSnapshotOwner` are still two near-identical ~800-line owners over the same flow
  host snapshot. They could become one shared owner generic over the snapshot type. Out of scope here.
- The kernel `--lib` test target still does not build for wasm32 (peers' native-only directory tests). The schema-hash laws run in the
  kernel's own integration test target on wasm32 instead.
- P6's `wp-p6/target-wasm-carrier` (build output of the deleted carrier crate) is left for P6/the coordinator.

## Not Run

- wasm32 `cargo check` of generation3d, generation2d, the neural engine and stdio-semio. I queued it behind w1, which had held the
  wasm mutex since 10:14, and h4. I withdrew it at about 11:35 after 45 min in the queue: my wrapper pid 46926 was killed and its queue
  entry is gone. The edits are target-neutral Rust. The framework session they now use already runs on wasm32 inside process3d
  (`📓️wp-p5.md`, `wasm-check-process.txt`), and w1's describe chain rebuilds these guests anyway.

## Processes

- None running. My detached cargo runs (92249, 95059, 97869, 811, 10875, 22130, 35810) have all exited or been stopped by me. I also stopped the
  wasm-mutex wrapper 46926. The wasm32 kernel test ran under the mutex via wrapper 95568, which exited normally.
