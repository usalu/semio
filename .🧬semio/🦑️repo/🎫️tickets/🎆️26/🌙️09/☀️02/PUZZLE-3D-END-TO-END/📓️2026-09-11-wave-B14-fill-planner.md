# Wave B14 — Fill planner: leftover `select` no longer starves the Fill tab

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B14 fill planner, 2026-09-11.
Audit C1 §12: Fill tab arms, `fill-wait-ready` count stays **0**, `fill-history-entry` FAIL.
User must open Fill, see count rise above 0, see objects, and cancel.

No git. No `:6013`. No wasm rebuild. Peers live on `✏️editor/🦀️.rs` — B9/B11/B12/B13 not reverted.
There is a sibling `📓️2026-09-11-wave-B14-settle-semantics.md` (different wave, same numeral).

---

## 0 Verdict

Fill is a **mode-level tool** (`setActiveTool` / `activeToolId === "fill"`), not a window utility.
The host leftover overlay only carried `activeUtility`. After Fill armed, leftover stayed at
`select` and `mergeWorldInteractionWithLeftoverV1` overwrote the guest world lane back to
`select`. `worldFillBuildShouldTick` already accepted a third `activeToolId`, but the
`World3dHost` call site passed two arguments, so `fillBuildTick` never started and the planner
count stayed 0.

| lane | result |
|---|---|
| cargo `fill_tool_wins_the_world_lane_over_a_select_window_utility` | **1 passed**, 701 filtered. Tail in `🗑️generated/b14-cargo.txt` |
| vitest `long` leftover/fillBuildTick | **4 passed**, 880 skipped (24 files). Tail in `🗑️generated/b14-vitest.txt` |
| wasm `component-release` | **not started** — no `wasm-release` running; guest planner already in tree |
| `:6013` | **not touched** (bun pid 6648 still listening) |

```
test editor::puzzle3d::component::tests::fill_tool_wins_the_world_lane_over_a_select_window_utility ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 701 filtered out
```

```
Test Files  1 passed | 23 skipped (24)
     Tests  4 passed | 880 skipped (884)
```


---

## 1 Root cause (file:line)

| symbol | file | meaning |
|---|---|---|
| `puzzle3d_fill_tool_active` | `✏️editor/🦀️.rs` | `config.active_tool_id == "fill"` |
| `puzzle3d_scene_active_utility` | `✏️editor/🦀️.rs` | fill **wins** over the window utility |
| `fill_build_tick` | `🎮️commands/fill-build-tick/🦀️.rs` | no-ops unless the fill tool is active; then polls/enqueues the isolated fill job |
| `worldFillBuildShouldTick` | `World3dHost` | ticks while `activeUtility === "fill"` **or** `activeToolId === "fill"` and the plan is unstarted (`done && count === 0`) |
| leftover overlay | `World3dHost` | host-owned armament; was utility-only |
| `SET_ACTIVE_TOOL` | `ShellHost` | armed the tool in session/view, **did not** publish leftover |

Chicken-and-egg: leftover `select` kept the world lane on select → the 120 ms interval never
dispatched `fillBuildTick` → guest `fill_progress_summary().count` stayed 0 → slider `ready`
stayed 0 → `setFillCount` had nothing planned to apply.

Guest already had the job + `setFillCount` retained command. The missing hop was host leftover
+ the third tick argument.

---

## 2 Fix (vite-live host + one guest law)

1. `LeftoverWorldSelectionOverlayV1.activeToolId` — leftover now carries the mode tool.
2. `leftoverOverlayCarryingUtilityV1` also carries `activeToolId` across InteractionView republish
   (same rule as utility: omit = keep prior; explicit value wins).
3. `leftoverOverlayArmedUtilityV1` — leftover `activeToolId === "fill"` wins over leftover
   `select`, matching `puzzle3d_scene_active_utility`.
4. `mergeWorldInteractionWithLeftoverV1` uses that armed utility so `fillMode` and the tick gate
   see `fill` the moment the tab is clicked.
5. `SET_ACTIVE_TOOL` publishes leftover `{ activeToolId, activeUtility: fill|select }`.
6. `SET_ACTIVE_UTILITY` clears leftover `activeToolId` when a real utility claims the pointer
   (mutual exclusion; B9 utility leftover unchanged).
7. `World3dHost` calls `worldFillBuildShouldTick(activeUtility, fillBuild, leftover.activeToolId)`.
8. `[DEBUG] fillBuildTick hop` / `[DEBUG] setActiveTool hop leftover` for later probe tails.

B9 `puzzle3d_addressed_window_id` / per-window utility publication is untouched.
B11 editor actions untouched. B12 WindowConfig untouched. B13 leftover selection-carry untouched
(`leftoverOverlayCarryingSelectionV1` still wraps the utility/tool carry).

---

## 3 Laws

### cargo

`editor::puzzle3d::component::tests::fill_tool_wins_the_world_lane_over_a_select_window_utility`

Runtime with `active_tool_id = fill` and a window `select` utility must publish `fill` on the
world lane. Clearing the tool returns `select`. Existing fill job laws
(`fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job`,
`fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances`) already prove the
guest planner once ticks arrive.

### vitest

`leftover fill tool overlays guest select so fillBuildTick still arms`

Leftover `{ activeUtility: "select", activeToolId: "fill" }` overlays guest select to fill,
carries the tool across a hover leftover, and `worldFillBuildShouldTick("select", {done:true,count:0}, "fill")`
stays true (the helper already had that third-arg case; the call site now passes it).

---

## 4 User path this unblocks

1. Open Fill tab → `SET_ACTIVE_TOOL fill` → leftover `{ tool: fill, utility: fill }`.
2. Host interval dispatches `fillBuildTick`.
3. Guest `fill_build_tick` sees `puzzle3d_fill_tool_active`, enqueues the isolated fill job.
4. Slider `ready` climbs with `fill_progress_summary().count`.
5. `setFillCount` applies planned pieces (`reveal` key `puzzle3d-fill`).
6. `cancelFillBuild` still identity-scoped to `(job, operation, generation)`.

Browser `--only=fill-tab,fill-apply-max` still needs a wasm that already contains the guest
planner (present since wave J / F5) plus this vite-live leftover. Not run here (`:6013` reserved).
