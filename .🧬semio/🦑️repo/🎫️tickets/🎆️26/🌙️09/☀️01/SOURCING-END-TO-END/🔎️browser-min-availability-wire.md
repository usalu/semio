# Browser wire for Minimum availability `fill("25")` + `Enter` (2026-11-24)

Recorded with `record-min-availability-wire.ts` (this folder; `repo <file> [single|journey|quick|tick|baseline]`) against :6081 (older build, without the Defect 3 fix and with `[DEBUG]` probes). It hooks `JSON.stringify`/`postMessage` and capturing DOM listeners in the page. Raw output: `🗑️generated/min-availability-wire-<scenario>.json`.

## 1. Exact recorded sequence (scenario `single`, identical in `journey`)

| t (ms) | DOM / wire |
|---|---|
| 0 | `focusin` spinbutton (value `0`) |
| +20 | `beforeinput`, `input` (DOM value `25`) |
| +21 | WIRE tick: `{"address":{"pluginId":"sourcing","appId":"s.sourcing.curation@1/*#editor","modeId":"curation","windowKindId":"sourcing-pool","windowInstanceId":"sourcing-pool","actionId":"setFilterMinAvailability"},"arguments":{"value":25,"gesture":"sourcing-filter-min-availability:<ms>:1","commit":false,"windowId":"sourcing-pool"}}` |
| +570 | `keydown Enter` (DOM value already reverted to `0`), `beforeinput`, `keyup Enter`. NO wire message, nothing else. |
| (Pool rows stay 12 for as long as the field keeps focus; no console fault) | |
| +4650 | the next click elsewhere (sort header; in the journey the next step) fires `focusout` |
| +4652 | WIRE release: same address, `{"value":25,"gesture":"<same id>","commit":true,"windowId":"sourcing-pool"}` |
| then | no abort is ever sent; every later command is ignored (see section 4) |

Facts:
- `value` is a JSON number (`25`), never a string. `commit` is a boolean. The gesture id is `sourcing-filter-min-availability:<Date.now()>:<page serial>` (about 55 chars, not `input:1`). Arguments always carry an extra `"windowId":"sourcing-pool"`.
- Enter is NOT a commit. The only release is the blur (`focusout`). The press stays open (a held, provisional press) for the whole time between `fill` and the blur. The provisional tick is not visible in the Pool list (12 rows after Enter).
- Journey step `fill("0")` + Enter after `fill("25")` (scenario `journey`) sends NOTHING: the controlled field already shows `0` after the revert, so React sees no value change and fires no `onChange` (DOM `input` event with `value=0`, no wire). The later blur then releases the OLD value 25 (`last` of the lane). So the journey's "clear" cannot work through the keyboard path even when the filter applies.
- `abort` is never emitted by this control: `lane.abort` is wired only for sliders (`onBlur`/`onPointerCancel`, input-views `:408-410`) and as `lane.abort("retired")` on unmount (`action-dispatch :60`). For the number input blur means `lane.commit()` (release), not abort. So the native law's abort form (`{"gesture":..,"abort":"blur"}`) is not what the browser sends for this field (it would only be sent if the Pool window/instance of the control unmounts while the press is open: `abort:"retired"`).

## 2. Code paths (all under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/`)

- Control selection: sourcing declares a plain `ui::input(InputKind::Number)` with `Trigger::Change` (`✏️s/🔌️plugins/🪵️sourcing/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs:128-130`), no `commit` mode, so `uiInputCommitsOnBlur(undefined)` is false (`🔨️modules/🖱️ui/🧬️contract/🧩️component/🟦️.ts:236`) and `continuous = kind==="number" && !commitOnBlur` (`✍️input-views/🟦️.tsx:206`). It is the only continuous number field in sourcing.
- Tick: `<Input onChange>` (`✍️input-views/🟦️.tsx:318`) -> `commitValue` (`:210-232`, `Number(raw)` via `typedNumber`, `:175`) -> `lane.offer(value)` (`:229`) -> `createContinuousGestureLane.offer` (`🔨️modules/🖱️ui/🎬️scene/🖐️gestures/🟦️.ts:113`, phase `live`) -> `useContinuousTriggerLane.send` (`⚡️action-dispatch/🟦️.tsx:45-50`: mints the press id with `continuousPressIdentity` `gestures/🟦️.ts:~60`, dispatches trigger `change` with `{value, gesture, commit:false}`) -> `dispatchControlTrigger` (`🎛️control-port/🟦️.ts:85`) -> `controls.dispatch` -> intent -> shell funnel (adds `windowId`, `origin:"gesture"`: `🔌️surface-bridge/🟦️.tsx:168-182`) -> wire.
- Enter: `onKeyDown` (`✍️input-views/🟦️.tsx:319`) calls only `lawKey` (arrows/page/home/end) and `draftKey` only when `commitOnBlur`; for the continuous field Enter has no handler at all.
- Release: `<Input onBlur>` (`:320`, `continuous ? () => lane.commit()`) -> `lane.commit()` (`gestures/🟦️.ts:120`, no value: takes `owed ?? last` = 25, phase `commit`) -> `send` with `commit:true`, gesture ref reset to null (`action-dispatch :47`). A commit on a closed lane is a no-op.
- The lane keeps at most one send in flight and one owed value: the release is sent only after the tick's promise settles (the tick/release pair never overlaps, but the release is queued behind a tick whose turn may not have finished).

## 3. Gaps against the native law `the_min_availability_gesture_forms_settle_after_every_tick_release_and_abort`

1. Shape: the law sends `{"value":6,"gesture":"input:1","commit":false}` with a 7-char id and no `windowId`; the browser id is ~55 chars and carries `windowId`. Values are equal in tick/release (as in the browser), good.
2. Sequence: the law runs tick, release, ANOTHER-gesture release-only (`input:2`, `commit:true` without a preceding tick; the browser never sends a release without a tick of the same press), then an abort for a press that was never opened (`input:3` abort without a tick; the browser's `lane.abort` early-returns when not open). The one sequence the browser really sends (tick(v) ... release(v), then a different command) is only half covered: tick-then-release of the same press followed by a command OTHER than `setFilterQuery`.
3. Follow-up command: the law ends with `setFilterQuery("timber")`; the browser's next command is `sortTable` (journey step 15) or any other editor command, and the stall is observed there ("sortTable has held the lane for 5000 ms, guest status more-work, ingress command-pending").
4. THE MAIN GAP, the success criterion: `wire_forms_settle` only checks that the dispatch returns Ok and that `drive_live_cleanup` does not return Err. `drive_live_cleanup` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:403-415`) drops the returned `PluginLifecycleStep`, so it never notices a ladder that makes zero progress forever. The browser's actor applies `runtime_live_cleanup_nonterminal_status` (`⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs:20-37`): 256 consecutive `Progress(default)`/`Blocked` steps (`RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT = 256`, `🧹️cleanup-vocabulary/🦀️.rs:200`) fault the instance with `plugin.internal.zero-progress`, after which everything is refused. The law runs exactly 256 units per form and never checks `Complete`. That is why it passes while the browser wedges.
   Evidence in the browser (scenario `quick`: `fill("25")` then immediately a click elsewhere, no Enter, no waiting): after the release, console shows `ladder zero blocks=false … cursor=26 ran ToolOverlay -> Progress(0)`, `live cleanup zero progress 256`, then `[actor] original-turn.semantic plugin.internal.zero-progress: runtime live cleanup faulted for instance 1 … (elapsed 0us, ceiling 8000us)` and `input #5 setFilterMinAvailability refused: dispatch-failed`. With a 4 s hold (`single`) the release is delivered but the next command (`sortTable`) stalls on `more-work` and no filter/sort ever applies (rows and sort order unchanged, second sort click too; `baseline` without the filter edit sorts fine: `8,10,12,… ` then reversed).
5. Hold time/turn boundaries: in the browser the open press lives across many host turns and maintenance turns (idle seconds) between tick and release; in the law everything is settled in one fixture turn with `settle`. Not proven to matter (the `quick` scenario wedges without a hold) but not covered.
6. UI-side: Enter does nothing and the second `fill("0")` emits nothing (section 1), so even with a healthy native side the journey step cannot see the filter after `Enter` (the provisional tick does not filter the Pool list; only the blur release does) and cannot clear it. The journey (or the control) must release: either press Tab/blur after `fill`, or make the continuous number field commit on Enter (`onKeyDown` Enter -> `lane.commit()`), plus `Escape` -> `lane.abort("blur")`.

## 4. Proposed native law reproducing the browser (not applied; edit target is the unit test file above)

```rust
/// 🎚️ LAW: the browser's number input (tick, then release on blur, then another command) drains the cleanup ladder to idle and keeps the instance alive.
#[semio_framework_async_macros::async_test]
async fn the_min_availability_browser_press_drains_its_cleanup_ladder_and_keeps_the_next_command_alive() {
    use crate::editor_unit_tests::context::dispatch_wire;
    let stock = semio_framework_value::RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 520 * 1024, maximum_capacity_bytes: 16 * 1024 * 1024, maximum_release_bytes: 1024 * 1024, maximum_depth: 64 };
    let mut app = new_app().await;
    let _browser = semio_plugin::artifact_app_laws::arm_host_turn_grant(stock);
    let press = "sourcing-filter-min-availability:1795524369502:1";
    for arguments in [
        format!(r#"{{"value":25,"gesture":"{press}","commit":false,"windowId":"sourcing-pool"}}"#),
        format!(r#"{{"value":25,"gesture":"{press}","commit":true,"windowId":"sourcing-pool"}}"#),
    ] {
        dispatch_wire(&mut app, "setFilterMinAvailability", &arguments).await.unwrap_or_else(|fault| panic!("{arguments}: {fault:?}"));
        drain_live_cleanup_to_idle(&mut app, 256);
    }
    dispatch(&mut app, SourcingCurationCommand::SortTable(/* availability column, ascending: the browser's next command */)).await;
    drain_live_cleanup_to_idle(&mut app, 256);
}
```

with the new helper replacing the blind `drive_live_cleanup` for this law:

```rust
/// 🧹️ Drives live cleanup like the actor does and fails on what the actor faults on: `limit` consecutive zero-progress steps without reaching `Complete`.
fn drain_live_cleanup_to_idle(app: &mut SourcingTestApp, limit: usize) {
    let mut stalled = 0usize;
    loop {
        /* admit the MountedOwnerTurnV1 exactly as drive_live_cleanup does, then: */
        let step = app.0.mounted_owner_maintenance_step(&mut turn).expect("a live-cleanup maintenance unit never faults");
        match step {
            PluginLifecycleStep::Complete(_) => return,
            PluginLifecycleStep::Progress(receipt) if receipt != RetainedCloneProgress::default() => stalled = 0,
            PluginLifecycleStep::AwaitingInput { .. } => return,
            _ => { stalled += 1; assert!(stalled < limit, "cleanup ladder made no progress for {limit} steps (browser: plugin.internal.zero-progress)"); }
        }
    }
}
```

Also add the abort variant the browser really sends: tick, then `{"gesture":"<press>","abort":"retired"}` (unmount while open), asserting the same drain. The existing `blur` abort stays as is for the slider.

Expected: it fails natively today with the ladder stuck at `ToolOverlay` (the stage the browser trace names, `cursor=26`) after the release, reproducing the browser wedge.

## 5. Caveats
- The served build has `[DEBUG]` probes and lacks the Defect 3 fix; the recorded wedge (zero-progress at `ToolOverlay`) may be this build's state. The wire and DOM sequence do not depend on it. Re-run `repo record-min-availability-wire.ts quick` against a fresh build to confirm.
- No repo source was edited; only the recorder script was added here.
