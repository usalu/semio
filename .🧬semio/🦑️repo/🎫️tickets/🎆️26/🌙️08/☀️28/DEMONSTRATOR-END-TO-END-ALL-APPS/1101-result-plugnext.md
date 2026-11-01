STATUS: PARTIAL GREEN. `plugnext-3` rc 0 (wasm32-wasip2 --lib, hub manifest; time-travel-turns, tool-run-turns, normal-fill, flow-copy, instance-docs, instance-jobs, zero errors) and `plugnext-4` rc 0 (instance-render), logs in `C/🗑️generated/1101/`. No full hub run after these fixes: instance-typed-ops (lane plugturns) was still red in plugnext-hub-2, so 'no framework crate red' is UNVERIFIED.

## Done by hand
- normal-fill: payload writer replaced by `RetainedPayloadBuilder` (init, `append_original` from the encoder cursor, `seal`, `take_finished`), staged close via `close_step_granted`, `step` split into `step_original` + 3-arg `step`, close closures return `RetainedWorkOutcome` (work counted via out-param).
- flow-copy: `.result` on outcomes; `admit_borrowed` replaced by staged `RetainedCloneSourceAdmission` driven inside `from_read`; new trait `FlowSnapshotProjection` replaces the captured projection fn (new variant `FlowSnapshotCopyIntakeSource::Admission`).
- time-travel-turns: closures wrapped into `RetainedWorkOutcome`, `.result` on controlled retirements.
- tool-run-turns: job slot owns an `OriginalJobReturnOrigin` (+ returned/continuation holders); `ToolRunJobSlot::drive` admits the origin, calls `drive_step` with its receiver and receives the outcome; ack and close go through the origin.

## Not mine (peer actively editing, left alone)
- `instance-jobs` / `instance-docs` were ported by a peer between hub-1 and hub-2 (files touched 07:57-08:03).
- `instance-typed-ops` (80 errors in hub-2) is being edited by a peer (08:15).

## Scope note
After the coordinator message: time-travel-turns, tool-run-turns, typed-ops belong to plugturns and flow-copy to flowext; my hand edits there (listed above) were made before that note and are already green in plugnext-3. instance-render checked green in plugnext-4 with no edit by me.
