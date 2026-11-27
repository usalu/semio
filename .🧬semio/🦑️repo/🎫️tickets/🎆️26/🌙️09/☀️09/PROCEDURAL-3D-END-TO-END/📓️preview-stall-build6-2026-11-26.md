# Preview stall after build 6 (2026-11-26)

## Verdict

The "evaluation is never reissued after `flow.extension-not-contributed`" reading in the build 6 evidence was a mislead. The evaluation IS reissued, but the tick that was owed lands minutes later than the probe's boot window and then dies in the extensions:

| Step | Observation (builds 7 to 12, `SEMIO_PROBE_ONLY=1` and `=4` with `SEMIO_PROBE_BASELINE_FIRST=1`) |
| --- | --- |
| `setContributions` | 425,209 JSON characters, 39.9 s to 45 s after page load (build 6 needed 158 s on the old requester). Result `effects: []`, `uiScope: none` (a live run was woken, nothing to carry). |
| `toolRunStart previewEval` | 45.2 s, host result `uiScope: full`. |
| `flowEvalTick` for `procedural-preview` and `generation3d-generate-preview` | both dispatched, `current: true`. |
| Preview status | `faulted / flow.extension-not-contributed` clears at 60 s, then `computing 0/1`, `inFlight 0` until the first extension request leaves the guest (about 90 s after the tick, 140 s to 157 s of page time). |
| `EXT_INVOKE flow-extension-math.evaluate req 1` | 140 s to 157 s. |
| Math answer | NOT an 87-byte pack. A fault: `origin framework`, `code flowEvaluation.pack`, `message "original projection must be driven or returned through its resident frame"`, `domain.source pack`, `refusal.kind invalidValue`. |
| `EXT_INVOKE flow-extension-brep.evaluate req 2` | 2.5 s later, same fault. |
| Requester | the fault page is larger than the 480-byte bounded result page (`ArtifactBoundedToolFault::from_fault`), so the host only sees `interactive-job.fault-capacity: the typed operation fault exceeded its bounded result page` ("failed after its dispatch" in the console). After both faults the preview stays at `4/7` with `inFlight 2` forever, meshes 0. |

So the visible stall is a requester that issued the extension calls and got faults back, with the fault delivery then leaving the two calls "in flight" (see Open items).

## Root cause 1: stale extension components

The fault is produced INSIDE the extension components while they encode the evaluation reply envelope (`OwnedProjectedPackWireCursor::advance_one`, `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/📨️wire/🦀️.rs`). That cursor now routes the admission phase `projectionStart` to `RetainedCloneSourceAdmission::project_owned`. The older cursor sent every phase but `ready` to the generic `advance`, which refuses `projectionStart` with exactly this message (`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🎟️admission/🦀️.rs`, `advance`).

Evidence the served extensions pre-date that fix:

- The cursor file was last changed 2026-11-25 19:00 (commit 757).
- The staged math core `.cache/b/p/m/browser/prototype/modules/🧮️flow-extension-math/semio_s_plugin_flow_math_component.core.wasm` is dated 2026-11-25 14:36, the brep module 2026-11-25 17:07. The requester was rebuilt 2026-11-26 21:50.
- `activate-generation3d-react-prototype` only re-stages/installs what is already in `modules/`; it does not rebuild the extensions. "Owned and fresh" therefore meant ownership of stale bytes.

Fix: rebuild both extension components with the requester's current framework sources, then re-run the activation and restart the server:

```
repo run @semio-tech/flow-extension-math-rust:package -c prototype --exclude-dependencies
repo run @semio-tech/flow-extension-brep-rust:package -c prototype --exclude-dependencies
repo run @semio-tech/framework-os-dev:activate-generation3d-react-prototype --exclude-dependencies
screen -S g3d -X quit; screen -dmS g3d "<T>/serve-generation3d-native.sh"
```

## Probe tooling fixes

`react-journey-probe.mjs` `SEMIO_PROBE_EXTENSION_TRACE=1` was stale: its `EXT_FAULT` patch matched `encodePackValue(fault)` (the source now says `encodeFaultPack(fault)`), so a faulted extension call left no trace line. The patch now matches the current source and prints the whole fault (BigInt-safe). That is what exposed the `flowEvaluation.pack` fault above.

## Build blockers hit on the rebuild (not source defects)

1. First math package run (13 min): `compiler broker admission failed: compiler needs an estimated 2163 MiB but its parent memory grant allows 1638 MiB` for `semio_framework_artifact_flow_flow_model`. Memory pressure from other agents' runs.
2. Retries of math and brep (23:00 to 23:16): `Internal error: Exact canonical persistent Native publication required: .cache/b/d {"filesystemBytes":[38653657088,34358689792],"maximumPhysicalBytes":[38654705664,34359738368]}`. An uncommitted edit to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` (another agent) lowered the `d` image from 36 GiB to 32 GiB (`capacityBytes 38654705664 -> 34359738368`) while the existing `.cache/b/d/fixed.dmg` is still 36 GiB, so every build now refuses to publish. The image has to be re-provisioned (or the policy edit finished/reverted) by its owner; it was not touched here.

## Root cause 2: native-return refusals (analysis only)

`original-turn.native-return-refused: semantic: UnsupportedOwner: original tool port final effects require native retirement` comes from `OriginalToolRunJobPortRetirement::demands` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/⏯️tool-run/🦀️.rs`, line 172). When the predecessor instance of a `switchToPluginApp` is retired while its `previewEval` run job's `ToolRunJobPort` still has queued host effects (an undelivered `flowEvalTick` or `invokeExtension`), the final native stage sees a non-empty `VecDeque<Effect>` and refuses instead of retiring the effects one by one. The port-level law (`⏯️tool-run/📮️port/♻️retirement/🧪️tests/🦀️.rs`) pins the refusal for a non-empty final queue (`assert!(port.original_close_demands().is_err())`), so the code states "not implemented" rather than regressing. `Effect` derives `RetireOwned`, so a paid per-effect retirement inside the port cursor is the missing piece; it is a change in the retirement ladder's demand accounting and was not attempted without a green rebuild to validate it. Later errors in the build 6 console (`batched publication original generation or revision is stale`, `typed command requires its complete original source and receiving recipient`) are consequences of the instance that failed to retire.
`🔨️return-origin` refactor (`StepContextReturnOrigin`) was not implicated by any trace line.

## Open items

- Rebuild extensions once the storage policy mismatch is resolved, then re-run `SEMIO_PROBE_ONLY=1` and `=4` (with `SEMIO_PROBE_EXTENSION_TRACE=1`, `SEMIO_PROBE_BASELINE_FIRST=1`, `SEMIO_PROBE_BASELINE_MS=400000`).
- After the faults the requester keeps two calls "in flight" instead of settling them as failed nodes. The fault completion arrives as a `flowEvaluation.pack` fault page that exceeds 480 bytes (`encode_fault_bytes_bounded` drops causes and scope but keeps `domain`, which here carries the whole nested refusal). Making the bounded encoder also drop `domain` (keeping `code`, `message`, `retainedProgress`) would keep the real code visible and let the requester settle the node. Not changed yet.
- Rows 5 to 19 need a green baseline (meshes > 0) first.

## Files changed

- `react-journey-probe.mjs` (ticket): extension fault trace.
- This note.
