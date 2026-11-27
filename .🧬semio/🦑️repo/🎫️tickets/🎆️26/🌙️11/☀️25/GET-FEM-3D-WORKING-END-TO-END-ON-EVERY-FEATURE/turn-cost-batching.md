# FEM 3D solve turn cost: batching at the root

## Result (settle law `a_default_slab_and_the_demo_settle_within_an_interactive_turn_budget`, package `semio-s-artifact-fem-3d-editor-visual-job --lib`)

| case | turns before | turns after | dominant stage after |
| --- | --- | --- | --- |
| slab | 37,025 (BuildCsr 24,978, Assembly 5,645, Pcg 4,360, SolidMesh 1,905) | 232 | BuildCsr 99, PreparePcg 59, PrepareAssembly 48 |
| demo | 4,960 (BuildCsr 2,576, Pcg 1,142, SolidMesh 295) | 43 | BuildCsr 10, PreparePcg 15 |
| demo-modal | 28,115 (Subspace 17,275, Ldlt 5,833) | 900 | Subspace 777, Ldlt 33 |

The law now gates 400 / 100 / 1,500 turns per case (settle budget argument), so any stage that falls back to one unit, one payload scalar or one retained item per turn fails it. The whole test also dropped from 138 s to 49 s.

## What actually bounded the turns (found by measuring, the "one unit per run" lead was only part of it)

A `run_turn` probe (stage, runs, remaining fuel, remaining grant after a turn) showed three independent per-turn ceilings, none of them the unit loop:

1. **Payload publication writes one scalar per turn.** `write_staged_source_funded` / `append_original` refuse a second page action in a turn (`payload_page_granted`), and `write_numerical_entry` writes one 8-byte scalar per call. PCG preview and complete (`fem.pcg.preview-page` / `complete-page`), LDLT output (`fem.ldlt.output-entry`) and the mesh publications spent 100 percent of their turns on this: 4,096 spinning `run` calls with 1 item and 8 bytes of grant consumed. A mounted child never needs the payload (its owner reads the vectors, the factor, the modes and the mesh in place through `visual_scalar`, `take_factor`, `completed_point`).
2. **One retained item per micro step.** The host grants 64 `maximum_items` per turn (`MountedOwnerPolicyV1::INTERACTIVE_COMPONENT`, shipped fleet 32); every LDLT column micro step (`admit_work`), Subspace step and CSR build step charged `copied_items: 1`, so 64 micro steps were all a turn could do (LDLT contributor entries and Subspace were at 0 items left after every turn, `run_turn` then spun 4,096 times for nothing).
3. **Assembly yielded after every element** in the owned and mounted modes (the preview and checkpoint flags returned `Yield` although nothing is ever published there).

Raising the unit cap alone (4,096 to 65,536) made the same turn counts 16 times more expensive in CPU (spinning against the exhausted grant), which is why the first experiment ran more than 8 minutes instead of 138 s.

## Changes

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🔶️csr-build-and-run/🦀️.rs`: `AssemblyJob::run` ignores the preview and checkpoint flags in the owned and mounted modes (nothing is published there) and keeps looping until `should_yield()`, cancellation or the end of the stage; the flags are only honoured by the borrowed mode that publishes them. A flag consumed by a turn that is out of fuel still yields (keeps the language-agnostic step-grant fixture).
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧮️triplet-csr-build/🦀️.rs`: `step_one_in` runs up to `CSR_UNITS_PER_STEP` (256) in-place units (`InitializeRows`, `Sort`, `CountUnique`, `Merge`, `Indptr`) per quoted step, each unit re-quoted against the same grant (a denied axis ends the batch), one item and the largest unit copy quote charged per step. Reserve stages stay single, allocating steps. 24,978 BuildCsr turns became 99.
- PCG (`🔁️pcg`), LDLT (`🔺️ldlt`), Subspace (`🎵️subspace`), mesh (`🕸️mesh`): a `publishes` flag with a `without_publication()` builder (PCG: `new_with_mounted_rhs` sets it; LDLT, Subspace and mesh are set by `Fem3dNumericalChild`). Without it a job skips previews, checkpoints and the terminal payload and completes with the desk's output marker. 2D (`◻️2d/.../🎼️eigen-stages`) still builds `LdltJob::new` with payloads (not touched, same fix applies).
- LDLT `admit_work`, Subspace `job-core`/`run-and-publish`/`iteration-steps` micro-step charges: `copied_items: 1` removed, the real `copied_bytes` stay charged, so the 1 MiB copy axis (not the 64 item axis) bounds a turn to thousands of steps. Real copies (transfers, doublings, closes) keep their item.
- `✏️s/🔨️modules/🏗️fem/⚙️engine/📮️payload-turns/🦀️.rs`: shared `NUMERICAL_UNITS_PER_TURN = 65_536` (the session's `UNITS_PER_STEP`), used by `run_turn`, the staged step loop (`🍓️staged-step`) and `MeshJob::units_per_turn`.
- `🤝️numerical-protocol`: `run_turn` stops after `NUMERICAL_IDLE_RUNS_PER_TURN` (16) consecutive yields that neither advanced the retained progress nor spent more than one fuel, so a turn whose grant is exhausted no longer spins.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/.../🧵️session/⚙️mounted-job/🦀️.rs`: `MountedJob::step` no longer breaks after one unit when `default_now_us()` is `None`; it is bounded by the turn's fuel, `should_yield()` and `UNITS_PER_STEP`. (Natively the clock exists, so this was not the cause there.)
- Tests: settle law tightened with per-case budgets and an explanation of what it guards; mesh unit tests step one unit per `step_plain` through `job.units_per_turn = 1` (their stage-by-stage probes had been made stale by the committed mesh loop).
- Not changed: `Fem3dNumericalChild::step`, the host grants (64 items; shipped fleet 32). Per host turn a round still spends the 64 items shared by all child steps of that round, which is why the app-level pump needs rounds in the hundreds, not the thousands.

## Laws (native, private target dirs, all deleted at the end)

See the final section appended by the verification runs.
