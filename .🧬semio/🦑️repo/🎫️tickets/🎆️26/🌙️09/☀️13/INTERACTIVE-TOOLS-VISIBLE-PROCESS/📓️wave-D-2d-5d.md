# Wave D — puzzle 2d / 5d fill parity

Ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`. Scope: every fill-related file under
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/**` and `…/🖐️5d/**`. No file under `🗿️artifacts/🧊️3d/**` or
`🧰️framework/**` was touched.

Roots below are abbreviated:
`E2 = ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
`E5 = ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`

---

## 1. Deliverable 1 — the count pins are gone, the default is 100

| What | Where | Change |
|---|---|---|
| `PUZZLE2D_FILL_COUNT_MAX` | `E2/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:11` (was) | **deleted**; replaced by `PUZZLE2D_DEFAULT_FILL_COUNT: u32 = 100` (`…/🪣️fill/🦀️.rs:16`) |
| begin-search refusal | `E2/🎮️commands/🧮️set-fill-count/🦀️.rs:794` (was) | **deleted** — `begin_search` now assigns `maximum_count` unconditionally (`…/🦀️.rs:793`) |
| preflight refusal | `E2/🎮️commands/🧮️set-fill-count/🦀️.rs:1489` (was) | **deleted** — `extent()` declares the same per-stage chunk ceilings for any count |
| `requested_fill_count()` helper | `E2/🎮️commands/🧮️set-fill-count/🦀️.rs:1610` (was) | **deleted** — its only caller was the preflight refusal |
| control-verb clamp | `E2/🎮️commands/🧮️set-fill-count/🦀️.rs:1664` (was) → `…:1641` | ceiling replaced by a plain finite/non-negative/`u32::MAX` domain check; the `puzzle2d-fill-count-capacity` code is **gone from the whole repo** (grep: 0 hits) |
| session-begin refusal | `E2/🎮️commands/🏁️fill-session-begin/🦀️.rs:16` (was) | **deleted**; `use …tools::fill;` import dropped with it |
| `PUZZLE5D_FILL_COUNT_MAX` | `E5/🦀️.rs:62` (was) | **deleted**; replaced by `PUZZLE5D_DEFAULT_FILL_COUNT: u32 = 100` (`E5/🦀️.rs:65`) |
| 5d clamp | `E5/🎮️commands/🧮️set-fill-count/🦀️.rs:10` | `.min(PUZZLE5D_FILL_COUNT_MAX)` → `.clamp(0.0, f64::from(u32::MAX))` (a domain guard, not a product ceiling) |

Defaults, both artifacts (Rust default + JSON-schema default; neither artifact has a TS guard on
`fillCount` — the TS mirrors are plain `fillCount: number` interfaces, `E2/🪟️window/🧬️schema/🟦️.ts:6`
and `E5/🪟️window/🧬️schema/🟦️.ts:23`):

- `E2/🎚️config/🦀️.rs:36` new `default_fill_count()`, applied at `:178` (`#[value(default = …)]`) and `:223` (`Default` impl).
- `E2/🪟️window/🦀️.rs:32` — `Puzzle2dWindowConfig::default().fill_count = PUZZLE2D_DEFAULT_FILL_COUNT`.
- `E2/🪟️window/🧬️schema/🔣️.json:14` — `"default": 100`.
- `E5/🎚️config/🦀️.rs:32` new `default_fill_count()`, applied at `:74` and `:104`.
- `E5/🪟️window/🦀️.rs:25` — `Puzzle5dWindowConfig::default().fill_count` (`Puzzle5dBoardWindowConfig` inherits it at `:55`).
- `E5/🪟️window/🧬️schema/🔣️.json:44` — `"default": 100`.

## 2. Deliverable 2 — the count controls are unbounded `WindowMeasure::Number`

Both are `Number { min: Some(0.0), max: None, step: Some(1.0), … }`, per master plan §2.4:

- `E2/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:53-70` `count_measure()` — `ready` is the accepted count while a
  session runs, `loading`/`waiting` keep the old lifecycle split.
- `E5/🎭️modes/✏️edit/☑️options/🪣️fill/🦀️.rs:30-45` `fill_count_measure()` — `ready` is the wrapped 3d
  planner's `applied_count`, `loading` is `!done` (previously `ready: None, loading: None, waiting: None`).

Wave F's variant and `MeasureProgressStep`/`MeasureProgressStepKind` were already exported through
`semio_framework_plugin` when these crates were compiled — no fallback was needed.

## 3. Deliverable 4 — 2d's `Progress: accepted/count` readout moved onto `WindowMeasure::Progress`

`E2/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:91-109` `progress_measure()`:

- published for every lifecycle except `Idle`/`Discarded`;
- `stage` from `Puzzle2dFillLifecycle` via the new `puzzle2d_fill_stage_label()`
  (`E2/🗣️terminology/🦀️.rs:98-116`), EN + DE, with the fault code appended verbatim rather than swallowed;
- `completed` = `fill_job_accepted_count`, `total` = `fill_count`;
- `steps` (`…/🪣️fill/🦀️.rs:76-88`): accepted (Success), tested (Info), fault code (Danger) — a counter at
  zero contributes no line;
- `cancel` = the old `brushFillSessionCancel` toggle's action, generation args unchanged, present only
  while the session is running;
- the retry `Toggle` is kept unchanged for `Faulted`/`Cancelled` (`…/🪣️fill/🦀️.rs:115-117`).

New 2d labels (`E2/🗣️terminology/🦀️.rs:52-60`), all four locale×terminology cells:
`fill_cancelled`, `fill_stage_{capturing,queued,searching,applying,closing,done}`, `fill_tested`,
`fill_accepted`. The now-dead `fill_result` label was removed (the Progress row's `completed/total`
expresses the result numerically).

## 4. Deliverable 3 — 5d is wired to the wrapped 3d session

`E5/🧠️precompute/🦀️.rs`:

- `Puzzle5dFillProgress` (`:17-37`) — a 5d-owned projection of the 3d `FillProgressSummary`
  (`count/applied_count/requested_count/done/tested/rejected/collisions/stage/stall_reason`). Declared
  locally rather than re-exported so 5d's public surface names only 5d types.
- `fill_progress()` (`:85`), `fill_requested_count()` (`:101`), `set_fill_requested_count()` (`:109`),
  `fill_job_identity()` (`:116`), `cancel_fill_job_for()` (`:121`) — passthroughs to wave B1's session API.
- `E5/🎮️commands/🧮️set-fill-count/🦀️.rs:13` now calls `set_fill_requested_count(count)` before
  `apply_fill_count_rust(count)`: the 5d session used to only *project* a count onto a plan the planner
  was still holding to its own target.

`E5/🎭️modes/✏️edit/☑️options/🪣️fill/🦀️.rs:69-85` `fill_progress_measure()` — `WindowMeasure::Progress`
with `completed = applied_count`, `total = requested_count`, a localized stage caption via the new
`puzzle5d_fill_stage_label()` / `puzzle5d_fill_stall_reason_label()` (`E5/🗣️terminology/🦀️.rs`, mapping the
3d planner's own machine tokens onto 5d's part/grip vocabulary, EN + DE), the four verdict counters as
`MeasureProgressStep`s, and a `cancelFillBuild` cancel carrying the 3d identity args.

Both windows now pass the session into the group:
`E5/🎭️modes/✏️edit/🪟️windows/◻️2d/🦀️.rs:52` and `…/🧊️3d/🦀️.rs:68` → `mode_options::fill::measure(envelope, precompute, labels)`.

**New 5d action `cancelFillBuild`** (5d had none): `E5/🎮️commands/🛑️cancel-fill-build/🦀️.rs` (new file),
module at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs:2006-2007`, variant at `E5/🦀️.rs:3874`, dispatch at
`E5/🦀️.rs:4268`, manifest action at `E5/🦀️.rs:8475`, interactive-job classification at `E5/🦀️.rs:8546`.
It cancels the identified background job when one exists **and** pins `runtime.fill_count` to the locked
count — that pin is what actually stops 5d's synchronous planner, because 5d never enqueues the 3d fill
job (`fill_job_identity()` is `None` there today). The row is therefore gated on `!progress.done` rather
than on the job identity; when no job exists the cancel args are `{}` and only the pin happens.

## 5. Deliverable 5 — tests

### 2d — `E2/🎭️modes/✏️edit/🛠️tools/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` (rewritten)

| Test | What it pins |
|---|---|
| `fill_count_entry_is_a_tool_measure` | unchanged contract: tool measure, not a window group, not an engagement control |
| `fill_count_entry_is_unbounded` | `Number` with `max: None`, `min: Some(0.0)`, a 5000 count carried verbatim |
| `fill_count_defaults_to_one_hundred` | `PUZZLE2D_DEFAULT_FILL_COUNT`, `Puzzle2dPlayRuntime::default()`, `Puzzle2dWindowConfig::default()` and the rendered value all = 100 |
| `live_fill_publishes_a_progress_row_with_cancel` | no row when idle; stage/completed/total/steps/cancel-args when running |
| `fill_progress_localizes_stage_fault_and_retry` | DE stage captions, fault code as a Danger step, no cancel on a stopped run, retry toggle |

### 2d — `E2/🎮️commands/🧮️set-fill-count/🧪️tests/🔬️unit/🦀️.rs`

- `fill_control_verbs_are_pure_runtime_transitions`: the old `…MAX + 1 → Err(capacity)` assertion is
  replaced by `count 5000 → Ok(Some((5000, 1)))` plus `runtime.fill_count == 5000`, and a `-1` case still
  refuses with `puzzle2d-fill-count`.
- `fill_session_extent_is_the_enforced_budget`: the old `over → None` assertion is replaced by
  `count 5000 → Some(search_budget)`.

### 5d — `E5/🧪️tests/🔬️unit/🦀️.rs`

| Test | What it pins |
|---|---|
| `fill_count_entry_is_unbounded_and_defaults_to_one_hundred` | constant, runtime default, window-config default, rendered `Number` (`max: None`) in both windows |
| `set_fill_count_carries_a_large_count_and_retargets_the_planner` | a 5000-count dispatch is not refused; `set_fill_requested_count`/`fill_requested_count` round-trip |
| `fill_progress_row_reflects_the_wrapped_session_summary` | the row's `completed`/`total`/`stage`/`cancel`/`loading` equal the wrapped session's own summary, and the row exists exactly when the session is not `done` |
| `cancel_fill_build_pins_the_count_to_what_is_locked` | the new `cancelFillBuild` verb dispatches with stale identity args without faulting |

Helper churn: `has_measure_slider` (dead once the fill control stopped being a slider) replaced by
`find_measure_number` / `find_measure_progress`; the existing
`fill_and_brush_params_are_tagged_utility_options_not_engagement_controls` now asserts the `Number` leaf.

## 6. Verification — commands run and results

```
cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly -j 4 --message-format short
  Finished `dev` profile [unoptimized] target(s) in 4m 49s
  → 0 errors. 4 warnings, all pre-existing `unnecessary qualification` on
    E2/🪟️window/🦀️.rs:234/237/245/250 (std::mem::size_of qualifications, untouched by this wave).

CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly -j 4 --message-format short
  Finished `dev` profile [unoptimized] target(s) in 2m 58s
  → 0 errors.
```

Test runs (`CARGO_INCREMENTAL=0 RUST_MIN_STACK=134217728 cargo test -p <crate> --features component-app-assembly -j 4 … fill`):

<!-- TEST-RESULTS -->

### Disk incident during verification

The first `cargo test -p semio-s-artifact-puzzle-2d` run died on `No space left on device`
(`/System/Volumes/Data` at 100 %, 464 MiB free; `⚡️cache` alone held 379 G, of which
`cargo/build/debug/incremental` was 36 G). `cargo/build/debug/incremental` was pruned — a pure
rebuild cache, nothing derived from it is lost — which restored 69 GiB. Every later build in this wave
ran with `CARGO_INCREMENTAL=0` so it does not regrow. This is workspace-wide, not wave-D-specific:
other waves building concurrently hit the same wall.

## 7. Open issues / cross-wave dependencies

1. **`take_fill_locked_chunk` is `pub(crate)` in 3d** (`…/🧊️3d/…/⏳️precompute/🦀️.rs:3325`), so 5d cannot
   adapt to it. 5d still commits through the `ApplyFillCount` engine command + `merge_engine_fixture`.
   If wave B1 intends the locked-chunk path to be the only commit route, that method has to become `pub`
   (or the 3d crate has to expose an equivalent), and `E5/🎮️commands/🧮️set-fill-count/🦀️.rs` +
   `E5/🧠️precompute/🦀️.rs` follow in one edit.
2. **5d never enqueues a background fill job**, so `fill_job_identity()` is always `None` there and the
   `cancelFillBuild` identity args are empty in practice. The verb still does real work (it pins the
   requested count to the locked count). If 5d is later moved onto the 3d job path, nothing in this
   wave's UI has to change.
3. **`✏️s/🔌️plugins/🧩️puzzle/🔣️.json` is stale** — it is a `describe`-generated output
   (`…/🧩️puzzle/📦️packages/🦀️rust/📋️project.json:83`) and still carries the old
   `{"kind": "slider", "max": 1000}` shape for `puzzle5d-fill-count` (lines 22877, 26924). Whoever runs
   the deploy chain should run `bun nx run …:describe` after waves A/B/C/D/F have all landed; running it
   now would bake in a half-migrated 3d manifest.
4. **Retained-jobs fixture**: `cancelFillBuild` was deliberately *not* added to
   `E5/…/🧫️fixtures/🗄️retained-jobs/🔣️.json` `toolIds`/`evidenceToolIds` — it is not a retained tool
   (`PUZZLE5D_RETAINED_TOOL_IDS` does not list it, exactly as `setFillCount` is not listed). If wave E's
   policy predicates require every manifest action to carry retained evidence, that fixture needs an entry.
5. The framework `WindowMeasure::Progress` carries no accessible name for its cancel control; 2d's
   `fill_cancel` label is consequently unused by the measure tree. Left in place for wave F to consume if
   the renderer grows a cancel label field.
