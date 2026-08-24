# Terra Fresh P5d Independent Source/Static Audit

Date: 2026-08-24

Verdict: **GREEN for the scoped P5d source/static contract; Phase 5 runtime acceptance remains open.**

## Source Verdict

The mounted route is present in the six audited production files: retained prepared input and
generation-qualified permits, `PreparedRenderJob`, capacity-one mailbox/packet, shared-session
mounting, retained GPU presenter, acknowledgement/abort, and five abandonment-drain phases. The
P5d verifier extracts those same production-only boundaries and rejects dynamic backing, late
admission, bulk fuel/loops, whole GPU render/composition, stale publication, missing Drop recovery,
and restoration of `FrameEngine::build_frame`/`Scene::finish` outside `cfg(test)`.

No concrete P5d source counterexample was found in this independent read. Textured-scene behavior
is intentionally a measured/retained no-op because no production textured pipeline exists; it does
not claim a synthetic renderer.

## Mounted Authority Trace

The audited live route is `OsHost::redraw_core` through `AppFrameTransaction` and
`AppFrameBuild::into_preparation` in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`,
then `PreparedRenderInput::try_new` / `PreparedRenderJob::try_new` in
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs`, the shared
`BatchJobSession` at fuel one and budget one millisecond, `PreparedRenderReceiver`,
`AppPresenter::present_step`, and `PreparedGpuPresentCursor` in
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️gpu.rs`.

Fixed metadata, command directories/pages, and the 64-slot generation-qualified process permit
are source-checked before producer transfer; page/backing growth is checked-CAS and its release
is separately cursor-retired. The job and GPU cursor each advance one retained scalar/page/command
or platform phase per grant, with cancellation, deadline, and generation checks around opaque work.
`prepared.rs` carries explicit rejected input/job, mailbox/receiver, packet, and Drop-abandonment
authorities; GPU close/Drop returns interrupted cursors to the mounted abandonment drain. The glue
drains GPU, input, job, receiver, and packet authorities before new frame text work.

The static P5d predicate and 39 mutations bind these exact live callees: dynamic metadata/command
substitution, process permit omission/wrapping, post-transfer capacity checking, bulk worker fuel,
loop/recursive preparation, stale packet publication, whole GPU composition, missing ACK/abort,
missing abandonment drain, and non-test duplicate builders are rejected. Independent source probes
also confirmed that `FrameEngine::build_frame`, `Scene::finish`, and complete scene composition
helpers are excluded by production-source extraction (`cfg(test)`), while the live textured command
path is a truthful retained no-op rather than a hidden complete textured renderer.

| Authority | Refusal/interruption owner | Mounted recovery |
| --- | --- | --- |
| Input/job | exact `PreparedRenderInputRejected` / `PreparedRenderJobRejected` | input/job abandonment close cursor |
| Mailbox/receiver | capacity-one generation-qualified packet | receiver take/close then packet drain |
| Packet | stale/cancel/fault keeps candidate separate from last valid | packet close releases nested pages/backings/permit dimensions |
| GPU cursor | surface/device/stale/Drop aborts candidate | GPU abandonment cursor before ACK |
| Presenter | ACK only after matching completed cursor | abort leaves last-valid gate unchanged |

## Executed Gates

```text
bun ./📜️script.ts verify interactivity p5d
[verify interactivity p5d] live-source and hostile mutations clean.
```

The P5d verifier executed all 39 faithful mutations. The six-file
`rustfmt --edition 2021 --config skip_children=true --check` and scoped diff check were clean.

The exported preservation functions were rerun separately to completion:

```text
interactivityMountedFrameTransactionSelfTests(process.cwd())  -> p5a GREEN
interactivityLiveReconcileSelfTests(process.cwd())             -> p5b GREEN
interactivityMountedLayoutTextSelfTests(process.cwd())         -> p5c GREEN
```

## Aggregate P5a/P5b/P5c Status

Each requested aggregate invocation (`p5a`, `p5b`, and `p5c`) is RED before its own preservation
checks because the unrelated global Puzzle Fill envelope baseline fails:

```text
Puzzle FillBuilder still materializes a whole preview/result envelope inside one worker grant
```

This is separately recorded as unrelated aggregate RED, not a P5d blocker. No source, verifier, or
existing report was changed. Cargo, Nx, Wasm, browser/native runtime, timing, network, and broad
build gates were not run.
