# Extension pack projection fault (2026-11-27)

## Verdict

The `flowEvaluation.pack` fault ("original projection must be driven or returned through its resident frame") was NOT a source defect. The Nov-25 cursor fix (`OwnedProjectedPackWireCursor::advance_one` routes `projectionStart` to `project_owned`, commit 757) is sufficient. The served Math and BRep modules were stale.

## Where the message comes from

`RetainedCloneSourceAdmission::advance` (`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🎟️admission/🦀️.rs`) refuses the phases `projectionStart|projection|projectionFailure|projectionFailed`. The evaluate reply is encoded by `OwnedProjectedPackWireCursor<EvaluationEnvelopeSource>` (`🎒️pack/🌱️value/🛫️encode/🫳️borrowed/📨️wire/🦀️.rs`, driven from `🌊️flow/🧩️extensions/🕸️wasm/🗃️evaluation-output-and-retained-evaluation`). The current cursor, the JSON wire and the JSON decoders all route `projectionStart` to `project_owned`. The existing law `original-owned-wire` already drives a real source through that cursor (it exercises the owned projection path), so no new native law was added: it could not fail before the fix because the source was already fixed.

## Root cause: package does not rebuild the browser modules

`repo run @semio-tech/flow-extension-{math,brep}-rust:package` publishes only the `model` facet (`.cache/b/p/m/browser/prototype/components/*/📦️component.wasm`, `packages/*/📦️extension.sxt`). The served files (`modules/🧮️flow-extension-math`, `modules/🧊️flow-extension-brep`, copied by `activate-generation3d-react-prototype` into `runtime/react/generation3d/extensions`) are the `browser` facet (compile, describe, jco transpile, bridge). They are produced only by the component target of the component crate. The activation log said "Activated extension ..." while the served core was still the 43,604,537-byte Nov-25 build (identical to `stages/transpile/8a15...`).

Correct rebuild command (project name is the component crate):

```
repo run semio-s-plugin-flow-math-component:component -c prototype --exclude-dependencies
repo run semio-s-plugin-flow-brep-component:component -c prototype --exclude-dependencies
repo run @semio-tech/framework-os-dev:activate-generation3d-react-prototype --exclude-dependencies
```

`T/extension-browser-build.sh <math|brep>` wraps the first two; `T/activate-and-probe.sh <OUT> [ONLY]` activates, restarts `screen -S g3d` and runs the probe.

Operational findings on the way: (1) the brep component sat for 20+ minutes at "prototype-03 waits for a local slot: it needs 4318 MB ... 4096 MB floor" through the CLI; the MCP `run` tool (own admission) completed it. (2) ENOSPC on `.cache/b/r/m/cargo/prototype/captures` once; `repo cache gc` freed space (it also dropped registry crates which were refetched).

## Result after the rebuild

Math browser module 57,391,485 B (10:20), BRep 73,952,682 B (14:17). Probe `react-journey-fix` rows 1 and 4 (`🗑️generated/journey-react-journey-fix.log`):

- `EXT_INVOKE math req 1` at 251.7 s, `EXT_ANSWER ... bytes 87` at 253.1 s, `EXT_DELIVERED`.
- `EXT_INVOKE brep req 2`, `EXT_ANSWER ... bytes 86` at 255.7 s.
- No `EXT_FAULT`. The `flowEvaluation.pack` fault is gone.
- Preview then stays `computing 4/7, inFlight 2, meshes 0`, because of the next fault below. Row 1 fails (boot, meshes 0).

## Next fault (requester, not the extensions)

Immediately after the brep answer the procedural requester wasm panics (`console.txt` line 300):

```
[semio-plugin panic] panicked at .../⚛️reactor/🪟️surfaces/🦀️.rs:123:9:
original surface capture holds the registry mutation lease
```

then `wasm trap: unreachable` in `typed-operation progress refresh`, `refresh-owed host effects`, `flowEvalTick` dispatch and both `extension ... evaluate failed after its dispatch`, which leaves the two calls in flight.

Analysis: `SurfaceContexts::insert` (also `remove`, `update_view`) asserts `cached_capture.is_none()`. A cached view read (`🪟️surfaces/read.rs`, `advance_selected`, phase 0 sets `cached_capture`, released only at its close) spans several turns. `Event::SurfaceVisible` in `⚛️reactor/🔄️turn/📨️event-dispatch/🦀️.rs` line 72 calls `plugin_mount_surface` (`🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs`) unconditionally, so a mount that arrives while a render capture is open aborts the guest. Reads already return `Ok(false)` (try later) when the lease is taken; the mutating entry points have no such path. Fix direction: the dispatcher must defer `SurfaceVisible`/`SurfaceHidden`/view updates while `SurfaceContexts` reports a held capture (add a `capture_held()` predicate and requeue the event), with a law that mounts during an open capture. Not attempted here: it changes the reactor event ordering and each requester rebuild/probe cycle costs more than an hour on this host.

## Files

- New: `T/extension-browser-build.sh`, `T/activate-and-probe.sh`, this note.
- No framework or extension source changed.

## Tests

None new; existing law `original-owned-wire` (pack crate) already covers the cursor. The native run could not start (`semio-framework-pack` is not a workspace member for plain `cargo test`; needs the repo test route).
