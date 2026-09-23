# 📓️ Flow — topic report (`🌊️flow` plugin, ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP)

Agent topic `flow`, 2026-09-21 13:55 → 16:15. Scope amended at 14:22: peer slice **S10 owns the GUEST code**
of `✏️s/🔌️plugins/🌊️flow`, so this topic diagnoses to the exact root cause, fixes **test-side / harness**
defects only, and hands every production change over as a proposed diff below. No `activate-*-react-dev`
was run; no wasm32 cargo command was run (none was needed — no production wasm-gated code was changed).

## 1. Crates and runs

Crate list via `cargo metadata --no-deps` filtered to `✏️s/🔌️plugins/🌊️flow` — **11 crates, none declares
`component-app-assembly`** (the extensions declare `component-guest`/`default` only), so every run used
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools
cargo test -p … --lib --tests --no-fail-fast -- --test-threads=4`, all 11 crates batched into ONE invocation.
The coordinator baseline (`🗑️generated/baseline/summary.tsv`) never reached a flow crate, so these are my own runs.

| crate | first run (`🗑️generated/flow/first-run.txt`) | final (`🗑️generated/flow/run3.txt`) |
|---|---|---|
| `semio-s-artifact-flow-flow` | FAIL 195 ok / **58 failed** | FAIL 248 ok / **5 failed** |
| `semio-s-plugin-flow` | FAIL 3 ok / **1 failed** | FAIL 3 ok / **1 failed** |
| `semio-s-plugin-flow-extension-brep` | FAIL 24 ok / **6 failed** | **ok 30/30** |
| `semio-s-plugin-flow-extension-text` | FAIL 4 ok / **1 failed** | **ok 5/5** |
| `…-bim` / `-dictionary` / `-draw` / `-list` / `-logic` / `-math` / `-primitive` | ok (10/7/42/10/5/10/6) | ok (unchanged) |

Logs: `🗑️generated/flow/first-run.txt`, `run2.txt`, `run2-serial.txt`, `run2-noprobe.txt`, `run3.txt`, `run4.txt`,
backtrace `bt-addwidget.txt`, browser probe `probe/flow.txt`.

## 2. The two commissioned questions

### 2.1 "The flow test app never finishes closing" (teardown hang)

**There is no unbounded hang in the native suites any more.** The whole 11-crate batch runs in ~5 s of test
time (`run3.txt`) and the 10-minute watchdog never fired in any of my four runs. The 09-19 symptom decomposes
into three distinct, separately-proven causes:

1. **A poisoned `Once`, not a teardown.** 54 of the 58 first-run failures were
   `Once instance has previously been poisoned`. One panic inside
   `install_first_party_light_flow_extensions_for_tests`'s `Once` poisoned it for every later test. Backtrace
   (`🗑️generated/flow/bt-addwidget.txt`): the hand-built `flow::FlowExtensionManifest` was **dropped**, and
   `ChannelSpec::number_default` puts a `Value::Dictionary` in `default`, whose fail-closed `Drop` aborts with
   `final Dictionary ownership must be explicitly retired or owned by a cold boundary` (bucket **XCUT-DICT**).
   **Peer slice S10 landed this fix at 14:14:59** (two `retire_cold()` loops after the encode); my first run had
   compiled the pre-fix source. Confirmed fixed in run 2.
2. **A test-owned settle loop that could never terminate** — this is the actual "never finishes" shape.
   `flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows` drove its own `drain()` loop that
   (a) asked for result pages of receiver **`1`** while the runtime binds instance **`71`**
   (`take_typed_operation_result_page` filters on `operation.meta.instance_id == receiver`, so no page was ever
   presented or ACKed) and (b) never drained `take_typed_operation_completion()` — stale-test **bucket 6**.
   `has_pending_typed_operations()` counts both, so the loop spun to its own 30 s deadline and failed with
   *"Flow retained publications timed out"*. **Fixed test-side**: the law now settles through
   `artifact_app_laws::settle_registered_typed_operation(app, 71)`, the framework's own ladder.
3. **A real close-ladder cost defect in the framework, still open.** `flow_actual_surface_factories_close_all_
   owners_under_neutral_grants` (`semio-s-plugin-flow`) closes the REAL assembled `s.flow.flow@1/*#editor`
   surface at `items=1, bytes=1`. I instrumented the assertion; run 3 reports:

   > `s.flow.flow@1/*#editor bytes=1 after 100000 close turns: 2051 items / 1389 bytes released, 96588 of them releasing nothing`

   It is **not** a stall on a single owner and **not** a byte-size problem: the ladder releases about one owner
   per **26** turns because `PluginApp::maintenance_step` rotates a fixed `MAINTENANCE_STAGES = 26` stage cursor
   and answers `Pending { released_items: 0, released_bytes: 0 }` on every empty stage
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29418`, `:30391`). 3 412 productive turns ×26 ≈ the
   100 000 the fixture allows. A fresh real editor surface therefore cannot finish closing inside its committed
   bound, and in a live guest — where the host hands out one small page per turn — the same 26× rotation tax is
   what a user sees as "the flow app never finishes closing".
   **Not fixed**: `🔌️plugin/🦀️.rs` is on slice S10's do-not-edit list. Proposed fix in §5.
   `close_registered_fixture_app` itself already papers over this (`if released_items == 0 && released_bytes == 0
   { std::thread::yield_now(); }` inside a 30 s wall deadline).

### 2.2 "The rename and patch-widget paths still drop live hosts"

**Not reproducible on the current tree — those two paths are clean.** Reading every `to_host_snapshot()` /
`host_from_snapshot()` call site in `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow`:

* `🎮️commands/🏷️rename-flow-widget/🦀️.rs::renamed_fixture` retires the live projection on its no-op early
  return (`fixture.retire_cold()`) and **transfers** it on the success path into
  `FlowSnapshot::from_host_snapshot(fixture)`, which destructures the `FlowHostSnapshot` and moves
  widgets/synapses/layout into the content child's `FlowWorkingScene` local owner — and `FlowWorkingScene`'s own
  `Drop` retires the `OrderedMap` layout root (`🗿️artifacts/🌊️flow/🦀️.rs:261`).
* `🎮️commands/🩹️patch-flow-widgets/🦀️.rs::patched_widgets_fixture` has no early return and transfers the same way.
* `crate::schema::mutations::snapshot_operations` retires **both** projections it builds.
* `evaluate_generation_preview` retires `live` on the parse-OK branch, transfers it on the parse-error branch,
  and retires the host in both.
* Every render/probe goes through `with_host_from_snapshot` / `with_live_host_snapshot`;
  `fallible_host_operations` retires the host on its error, no-change and success branches.

Evidence, not inspection alone: `booting_renders_and_evaluates_without_dropping_a_live_flow_owner`,
`rename_rejects_blank_unchanged_and_taken_ids` and
`patch_flow_widgets_parses_the_raw_value_string_into_the_slider` all pass in run 3 (they were red in run 1 only
through the poisoned `Once`), and the live `#flow` pane boots with **zero** console errors and no
`ordered-map root must be explicitly retired before drop` / `runtime instance authority is busy` (§4).

One test-side defect was found here instead: `booting_renders_and_evaluates_without_dropping_a_live_flow_owner`
asserted `armed.effects.is_empty()` "the starter graph contains operators unavailable to this bare fixture".
The flow extension registry is **process-global**, and the sibling `math` fixture module makes the starter
graph's `math.add` servable, so the claim held only when that test happened to run first — **proven**: the same
binary passes it alone (`--test-threads=1`, one filter) and fails it in the suite. Restated as the
order-independent anti-spin law it was really about: a second probe of the same unchanged snapshot must arm
nothing (the `arm_window_tick` latch).

## 3. Everything else that was fixed (all test-side, all `#[cfg(test)]`)

* **Stale channel names, 7 tests.** Commit `39fbe1b9bf` (2026-09-14, *"graph navigation … distinct input/output
  port identities"*) renamed every produced channel to `<name>Out` across the flow extensions. brep's
  `xform.translate`/`rotate`/`rotateAbout` now emit `geometryOut`, `eval.curveClosestParameter` /
  `eval.surfaceClosestUv` emit `pointOut`, text's `text.upper` emits `textOut`; the list/logic/math/dictionary
  tests were updated back then, brep's six and text's one were not. Tests updated to the current contract
  (production untouched) → brep 30/30 and text 5/5.
* **A process-global UI arena race, 6 tests.** `flow_render_fixture_projection_retires_populated_and_rejected_
  pages` deliberately SATURATES the process-global built-node page pool (`UI_BUILT_CHILD_RETIRE_SLOTS`) and then
  asserts terminal emptiness. Under `--test-threads=4` that made three catalogue renders and two document-panel
  builds fail with `ui.fixed-capacity: fixed UI admission failed at panel-tree.root-sections` and let foreign
  pages leak into its own terminal probe. **Proven, not guessed**: the same binary is 247/6 at
  `--test-threads=1` (`run2-serial.txt`) and 246/6 at `--test-threads=4` with only that one probe skipped
  (`run2-noprobe.txt`) — identical failure set both times. Fixed with a crate-local `RwLock` arena guard
  (ordinary builds share it, the saturating probe takes it exclusively), the same shape brep already uses for
  its shared kernel. 12 failures → 5.

## 4. Play pane `http://127.0.0.1:6033/#flow`

Headless chromium from `node_modules/playwright` with
`PLAYWRIGHT_BROWSERS_PATH=…/⚡️cache/tools/ms-playwright` and `--use-angle=metal`, via the ticket's own
`🧪️probe-console.mjs`. Result (`🗑️generated/flow/probe/flow.txt`, 64 lines): reaches its shell outcome (no
`[probe] shell outcome timeout`), **0 page errors, 0 console errors, 0 refused inputs**. The only warnings are
the frozen dev server's repo-wide staleness banner. The pane renders: `[DEBUG] flow surface created
surface=1 966x807 dpr=1 present=webgpu` and `[DEBUG] dag draw lod=normal zoom=1.000 icon=false label=Name`.

**Catalog `example` for the flow pane: leave it absent.** `flow` declares NO examples. `create_flow_app`
(`…/✏️editor/🦀️.rs:2904`) documents that `.example_source(crate::examples::art_flow_demo::source())` is
*dropped, not ported* — `EditorBuilder` has no example carrier — and `plugin()`
(`✏️s/🔌️plugins/🌊️flow/🦀️.rs`) registers the editor without examples, so `App.examples` is empty. The artifact
does own two example sources on disk (`📚️examples/🎬️demo`, `ID = "demo"`, and `✏️editor/📚️examples/🎬️demo-session`,
`ID = "demo-session"`); `"demo"` is the one that would belong in
`🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json` — but only AFTER a framework carrier exists, otherwise play's
example gate rejects an undeclared example. That carrier lives in `🧰️framework/🔨️modules/🛂️manifest`
(`ExampleDefinition`/`ExampleSource`), which is on slice S10's do-not-edit list. **Catalog not edited.**


## 5. Still failing, with the exact reason (nothing was weakened, deleted or ignored)

### 5.1 `flow_actual_surface_factories_close_all_owners_under_neutral_grants` (`semio-s-plugin-flow`)
The close-ladder cost defect of §2.1.3, measured:
`bytes=1 after 100000 close turns: 2051 items / 1389 bytes released, 96588 of them releasing nothing`.
Root cause is `PluginApp::maintenance_step`'s fixed 26-stage rotation answering `Pending { 0, 0 }` on every
empty stage, in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — **routed to the peer's FP7 slice and
explicitly out of bounds for me, so this test stays red.** Independently corroborated by the fem3d finding
("the reactor spends ONE maintenance step per turn over a 26-stage rotation → ~1 owner per 26 turns drained").
Proposed fix for FP7: while `close_started`, advance the stage cursor to the next stage that has work instead
of spending a turn on an empty one (or skip the rotation entirely once every maintenance queue is terminal).

### 5.2 `flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows` (`semio-s-artifact-flow-flow`)
Now fails on an assertion that had **never been reached before** (the law used to die 30 s earlier in its own
drain): `Flow generation transient survived same-byte document reload`. After
`PluginApp::load_document_pack(&document_before)` with byte-identical content,
`app.window_transient_generation(&generation)` still answers `Some`. `load_document_pack` does call
`prepare_document_window_reset` / `commit_document_window_reset`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs`), which
replaces the whole `window_transient_store` with a fresh registry at `generation + 1`, so the surviving answer
comes from framework window-transient lifecycle, not from flow's owner registration. Adding a full settle after
the reload does not change it (measured, `diag3.txt`). Framework-side, `🔌️plugin/**` — **not fixed, out of bounds.**

**Separate live production defect this law now proves** (recorded here, not fixed — it needs the retained
window-config route, which is shared framework/flow surface): dispatching two retained window-config commands
for the **same** window back to back, without settling in between, **silently drops the second** — no result
page, no fault lane, its amend lost. Measured before the per-dispatch settle was added: lanes `(2, 1)` instead
of `(4, 1)`, left grid stuck at the default `(true, 10.0)` instead of `(false, 10.0)`, right at `10.0` instead
of `20.0`, while both cameras (the FIRST command of each pair) landed correctly. Neither command declares a
`coalesce_key`, so this is not latest-wins coalescing.

### 5.3 `two_instances_converge_on_disjoint_edits` (`semio-s-artifact-flow-flow`)
Two registered instances joined by a `MemoryBackbone` do not converge on the composed `content` child: A holds
its own `inputNote (40, 41)` and not B's widget, B holds its own `inputSlider (300, 301)` and not A's —
`left: [("inputNote", 40, 41), ("inputSlider", 0, 0), ("neuron", 0, 0), ("outputPreview", 0, 0)]` vs
`right: [("inputSlider", 0, 0), ("inputSlider", 300, 301), ("neuron", 0, 0), ("outputPreview", 0, 0)]`. The
parent document converges (the law gets that far); the CHILD lane's edits are not replicated across the
backbone. That is child-store replication in `🏪️store` / `🔌️plugin`, not flow's own reducers — **not fixed.**

### 5.4 Proposed, not applied (framework, repo-wide rebuild cost — for the coordinator to schedule)
`ArtifactStoreBatchPublication::progress()`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3920`) reports only the CURRENT item's checkpoint and
answers `ArtifactStoreOneItemCheckpoint::default()` (all zeros) as soon as an item's preparation is released,
so a batch's progress snaps back to 0 between items. Measured in my first run:
`case=delete-widget grant=1 cancel=None step=22213 preparation progress changed from 22013 to 0`. `fold_batch_item`
already accumulates `stage.completed_items` / `stage.completed_bytes`, so the fix is to fold the released item's
checkpoint into a publication-level accumulator at the point where `publication.preparation = None` (NOT at fold
time, which would double-count while the item is closing) and return `accumulator + current item checkpoint`.
Not applied: `🏪️store/🦀️.rs` is the most widely depended-on file in the repo and this would have forced a
repo-wide rebuild while twelve peer cargos were already queued on the shared artifact lock. Note the symptom no
longer reproduced in my later runs after a peer's own store change; the `progress()` body is unchanged, so the
defect is still there and only the flow preparation test's path around it moved.

---

## 6. Applied — 2026-09-21 16:15–16:55 (peer approval to edit `✏️s/🔌️plugins/🌊️flow/**`)

Constraints honoured: only files under `✏️s/🔌️plugins/🌊️flow/**`; **nothing** in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (an earlier exploratory edit of
`🧰️framework/…/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs` was reverted and the file is byte-identical to HEAD); every file
re-read immediately before editing; no peer edit reverted except as noted in §6.3. No `cfg(target_arch =
"wasm32")` code was touched, so **no wasm32 check was run** — the guest build of these crates is therefore
unverified by me and needs no mutex slot.

### 6.1 Final numbers (`🗑️generated/flow/final.txt`, private `CARGO_TARGET_DIR="$T/🗑️generated/flow/target"`,
`--lib --tests --no-fail-fast -- --test-threads=4`)

| crate | first run | final |
|---|---|---|
| `semio-s-artifact-flow-flow` | 195 ok / **58 failed** | **251 ok / 2 failed** |
| `semio-s-plugin-flow` | 3 ok / **1 failed** | 3 ok / **1 failed** |
| `semio-s-plugin-flow-extension-brep` | 24 ok / **6 failed** | **30 / 0** (`run3.txt`) |
| `semio-s-plugin-flow-extension-text` | 4 ok / **1 failed** | **5 / 0** (`run3.txt`) |
| `-bim -dictionary -draw -list -logic -math -primitive` | ok | ok (`run3.txt`) |

The private `CARGO_TARGET_DIR` (intermediates stay in the shared `build-dir`, per `.cargo/config.toml`) cut the
turnaround from 45–60 min of artifact-lock starvation to ~2 min per run.

### 6.2 Files changed (absolute, all mine, all `#[cfg(test)]` except 6.3)

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️evaluate-budget/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/🧪️tests/🔬️unit/🦀️.rs`
  — stale produced-channel names → `geometryOut` / `pointOut` / `textOut` (commit `39fbe1b9bf`, §3).
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  — process-global built-node arena guard (`ui_arena_shared` / `ui_arena_exclusive`), taken by `render_with_view`
  and exclusively by the saturating projection probe.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`
  — the four document-panel `render(&document, …)` sites take the shared arena guard.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️interactive-job/🦀️.rs`
  — the boot law restated order-independently (second probe arms nothing), §2.2.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs`
  — settles through `artifact_app_laws::settle_registered_typed_operation` at the app's own instance id (71,
  now a named const) instead of a hand-rolled loop on receiver `1` that never drained
  `take_typed_operation_completion()`; one settle per dispatched command; the lane count is checked AFTER the
  content laws so it can no longer mask a camera/settings regression.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🧪️tests/🔬️surface/🦀️.rs`
  — the close-ladder assertion now reports turns / items / bytes / idle turns, which is what produced the
  measurement in §2.1.3.

### 6.3 One production line touched, and one peer value changed

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs:141`
  — slice S10's in-flight rewrite (16:07:59) that routes `nodeGraphEdit` through the CHILD lane did not compile
  (`sync_host_selection(host, …)`, `expected &mut FlowHost, found FlowHost`). Waited ~11 min, re-read, applied
  the compiler's own one-token repair `&mut host`. **That rewrite is what fixed four of the five failures that
  were open at 16:00** (`batched_delete_selection_clears_the_node_selection_on_the_scene`,
  `spotlight_commit_shares_the_node_graph_edit_vocabulary`,
  `flow_document_text_round_trips_store_with_applied_operation`, and the `Flow recipe target is missing` /
  `Flow mutation requires its explicit batch-only recipe` class) — credit to the peer, not to this topic.
- The same window-ownership law: a peer set the expected lane count to `(2, 1)` at 16:27:22 to match the
  behaviour of two back-to-back config dispatches. I set it back to **`(4, 1)`** because that is what the
  runtime actually publishes once each command is settled — measured, `diag2.txt`: four separate
  `[WindowConfig, Ui, Terminal]` settles plus one `[WindowTransient, Ui, Terminal]`, and with them **every**
  camera and grid assertion passes. `(2, 1)` was the symptom of the defect in §5.2, not the contract.


---

## 7. Session 6 successor — 2026-09-22 17:30 →

Scope this session: peer released `✏️s/🔌️plugins/🌊️flow/**` to this topic (guest edits allowed). Framework
`🔌️plugin/🦀️.rs`, `🏪️store`, `👷️worker`, ShellHost, browser-bundle, hub, mcp stay peer-owned → proposed diff at
`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/proposed-flow-retained.diff.md`.
Durable scratch moved to `$G = .🧬semio/🦑️repo/⚡️cache/play-fleet/flow` (the 16:05 workspace cleanup deleted
`$T/🗑️generated/flow/` — every 09-21 log referenced in §1–§6 above is gone; the findings stand, the logs do not).

### 7.1 Play pane `http://127.0.0.1:6033/#flow` — still green (17:40)
`🧪️probe-console.mjs` against the 15:47 activation / 16:58 serve: shell outcome reached, **0 page errors, 0
console errors, 0 refused inputs**; renders `[DEBUG] flow surface created surface=1 966x807 dpr=1 present=webgpu`
and `[DEBUG] dag draw lod=normal zoom=1.000 icon=false label=Name`. The only warning is the host's repo-wide
staleness banner, and **flow is not in its stale list** (7 stale modules: cad, playbook, process, puzzle,
reasoning, sequence, writer) — the served flow guest matches its source. Log `$G/probe/flow.txt`.
Consistent with `📓️acceptance-runs.md` 17:00 (70/70, `✓ 5 boots Flow (flow) (7.5s)`).

### 7.2 Native run queued
One mutex invocation for all 11 flow crates (pid 41720, queued 17:34 at rank 9 of 9), log `$G/run1.txt`,
`CARGO_TARGET_DIR=$G/target`, `CARGO_BUILD_JOBS=2`, `RUST_BACKTRACE=1`, `--lib --tests -- --test-threads=4`.

### 7.3 run1 (18:12) — a 66-red regression with ONE cause, and it was flow-side

`$G/run1.txt`, 11 crates, one mutex invocation, `--lib --tests --no-fail-fast -- --test-threads=4`:

| crate | 09-21 final | run1 18:12 |
|---|---|---|
| `semio-s-artifact-flow-flow` | 251 / **2** | 188 / **66** |
| `semio-s-plugin-flow` | 3 / **1** | 2 / **2** |
| the 9 `…-extension-*` | green | green (10/30/7/42/10/5/10/6/5) |

Every one of the 66 is the same defect. Panic split: **51** at the framework harness
`artifact_app_laws::close_registered_fixture_app` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:7584`)
with *"registered fixture did not reach its exact terminal-empty witness, last pending close authority: document
store close awaits a retained reader or owner"*, **15** at flow-side sites, one of them the `unreachable!` at
`…/✏️editor/🧵️retained/🦀️.rs:104` — *"internal error: entered unreachable code: positive Flow retirement grant"*.

**Root cause.** The peer's FL3 slice rewrote `FlowRetirement` in the FRAMEWORK crate
`semio-framework-artifact-flow-flow` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`)
from a draw-down frontier into an ATOMIC one: `release_root_backing` (:539) frees an owner's backing whole or not
at all and answers `SnapshotRetirementStep::Blocked` whenever `maximum_bytes` is below the demand the frontier now
publishes through `next_close_byte_demand()` (:223). The old `root_backing_credit` field and the
`owner_backing_payload` helper were deleted. **Every driver must now read the demand before it grants** — which is
exactly what the peer's own `CopyCursor::close_step` does (`🧵️retained/📑️copy/🦀️.rs:439`,
`let step = state.retirement.close_page(1, maximum_bytes.max(demand))`).

Four drivers in `✏️s/🔌️plugins/🌊️flow` forwarded their caller's fixed page raw and therefore blocked forever on any
owner whose backing is bigger than that page — which is every Flow document with more than a page of text, and
hence every registered fixture close (`close_registered_fixture_app` grants
`store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` = 4 096). **The 51 framework-harness panics were flow-side after
all**: the harness only reported the `Blocked` that `crate::retirement`'s own owners were answering.

### 7.4 Fix applied (flow production code, ours since today)

New `pub(crate) fn close_frontier_page(domain: &mut FlowRetirement, debt: &mut usize, maximum_bytes: usize)` in
`/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/♻️retirement/🦀️.rs`: it reads the frontier's
published demand, grants `maximum_bytes.max(demand)` out of the driver's own admission, and hands the freed bytes
back to the caller in page-sized instalments through a `debt` counter — so the caller's grant is never exceeded
AND the total reported over the close still equals the total physically freed (the accounting is amortized, the
free is not delayed). Applied to all four drivers; each grew a `debt` field that its `terminal_is_empty`/`Drop`
now includes:

- `…/♻️retirement/🦀️.rs` — `RootRetirement<T>` (scene + mutation owners, `store_owners()`).
- `…/♻️retirement/📸️snapshot/🦀️.rs` — `SnapshotRetirement` (the document lane; this one caused the 51).
- `…/✳️any/✏️editor/👥️presence/♻️retirement/🦀️.rs` — `FlowPresenceRetirement`.
- `…/✳️any/✏️editor/🧵️retained/🦀️.rs` — `Retirement`, whose `unreachable!("positive Flow retirement grant")` is now
  a `Blocked` propagation that pushes the owner back intact, plus a published
  `next_close_byte_demand()` (inherent + `ErasedSnapshotRetirement`). **That `unreachable!` was a live production
  abort**: under the new frontier any guest page smaller than an owner's backing aborted the plugin, not a test.

`🗑️generated/activate.request/{flow,demonstrator}` touched — guest production code changed, so the served lane is
now behind its source until the coordinator re-activates.

### 7.5 run2 (19:20) — proof

`$G/run2.txt`, same command:

| crate | run1 | run2 |
|---|---|---|
| `semio-s-artifact-flow-flow` | 188 / **66** | **250 / 4** |
| `semio-s-plugin-flow` | 2 / **2** | **3 / 1** |
| `…-extension-bim` | 10 / 0 | **10 / 0** |
| `…-extension-brep` | 30 / 0 | **30 / 0** |
| `…-extension-dictionary` | 7 / 0 | **7 / 0** |
| `…-extension-draw` | 42 / 0 | **42 / 0** |
| `…-extension-list` | 10 / 0 | **10 / 0** |
| `…-extension-logic` | 5 / 0 | **5 / 0** |
| `…-extension-math` | 10 / 0 | **10 / 0** |
| `…-extension-primitive` | 6 / 0 | **6 / 0** |
| `…-extension-text` | 5 / 0 | **5 / 0** |

**The three reds routed to the peer on 09-21 (§5.1, §5.2, §5.3) are GREEN**, and so is the back-to-back
window-config command drop of §5.2:
`flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows` ✓,
`two_instances_converge_on_disjoint_edits` ✓, `flow_viewer_never_mutates` ✓, and the new
`flow_two_window_config_commands_in_one_turn_both_land` ✓ — two retained window-config commands dispatched in one
turn both land now, so the silent second-command drop is fixed. `semio-s-plugin-flow`'s close-ladder law reached
2 051 items / **35 781** bytes released (it was 1 389 bytes on 09-21) after the peer's `maintenance_step` rotation
fix (`🔌️plugin/🦀️.rs:31018`, which now scans past empty stages inside one call and cites this topic's 09-21
measurement).

### 7.6 The 5 remaining reds — one framework accounting defect, routed to FL3

All five are the SAME assertion failure and none of them is a flow defect: the frontier now reports **allocation**
bytes as **released payload** bytes.

| law | expected payload | reported |
|---|---|---|
| `retained::artifact::snapshot::tests::child_typed_handoff_preserves_mismatched_owner_then_retires_exact_scene` | 16 388 | 31 868 |
| `presence::component::retirement::tests::flow_presence_store_owners_preserve_readers_and_retire_neutral_byte_grants` | 16 388 | 37 908 |
| `retained::artifact::recipe::tests::recipe_cancellation_retires_every_partial_frontier_without_losing_original_root` | 4 849 | 39 361 |
| `retained::artifact::tests::sixteen_kib_authored_label_copies_and_retires_at_actual_grants` | 14 (a 7-byte scene, twice) | 30 974 |
| `semio-s-plugin-flow` `plugin::surface_tests::flow_actual_surface_factories_close_all_owners_under_neutral_grants` | complete in 100 000 turns at `bytes=1` | 2 051 items / 35 781 bytes / 62 226 idle turns, not complete |

**Crate / symbol / line.** `semio-framework-artifact-flow-flow`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`:
`release_root_backing` (:539) returns `RootBackingRelease::Released(owner_backing_bytes(owner))` and
`ErasedSnapshotRetirement::close_step` (:614) reports that as `Pending { released_bytes }`.
`owner_backing_bytes` (:60) is `capacity * size_of::<T>()`, and `owner_waits_for_backing_release` (:100) makes it
apply to every drained element vector (`Strings`/`Widgets`/`Specs`/`Neurons`/`Synapses`/`Previews`/`Layout`), not
just `Bytes`. The same commit DELETED `owner_backing_payload`, whose own doc comment forbade exactly this: *"its
remaining `capacity` is an allocation, not payload, and `capacity * size_of::<T>()` is machine-width dependent, so
it can never be charged against a caller's byte grant (ticket 26/09/09/PROCEDURAL-3D-END-TO-END)"*.

Three consequences, all measured above: retiring a **7-byte** scene now reports **30 974** released bytes; every
exact-payload byte law in flow (and in any other plugin that owns a `FlowRetirement`) is now unassertable and
machine-width dependent; and a driver that pages at 1 byte needs ~36 000 accounting turns for one real editor
surface, which is what keeps the surface law short of its 100 000-turn bound (62 226 of those turns still release
nothing, i.e. the remaining idle share is the framework close ladder, not this frontier).

Note this contradicts FL3's OWN design elsewhere: `CopyCursor::close_step` deliberately keeps allocation bytes out
of the caller's page via `FlowCopyAllocationBudget::charge_release`, and their new test asserts
`cursor.allocation().returned_bytes() > charged`. `release_root_backing` should do the same — release the
allocation atomically, but report only the payload (`len`, not `capacity`) to the caller. **I did not restate the
five laws**: their expected numbers are the payload contract, they passed on 09-21 against the previous frontier,
and rewriting them to accept a `size_of`-dependent number would enshrine a non-portable quantity.

### 7.7 Files changed (absolute) and pane state

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/♻️retirement/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/♻️retirement/📸️snapshot/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/♻️retirement/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs`

No test was deleted, ignored, weakened or hand-matched; no framework file was touched; no wasm32 / describe /
activate command was run (the fix is not `cfg(target_arch = "wasm32")` code, so the guest build is unverified by
me — the activation request covers it). Pane `#flow` on :6033 measured green at 17:47 (§7.1) against the 15:47
activation, i.e. BEFORE this fix; it needs the requested re-activation to be served.

---

## 8. Session 7 successor — 2026-09-23 01:55 →

### 8.1 Baseline run4 (02:02, `⚡️cache/play-fleet/flow/run4.txt`, before any edit of mine)
FL3's payload-only accounting correction is in: every one of the five §7.6 accounting reds is GREEN
(run3b 23:36 already showed `semio-s-artifact-flow-flow` 254/0). run4 then shows NEW reds from another session's
in-flight `setActiveExample` rollout (staged 01:26–01:33, 27 plugins): `semio-s-artifact-flow-flow` 251/**4**
(fixture/route/wire-keyword/host-wire-ordinal drift), `semio-s-plugin-flow` 2/**2** (`descriptor_is_fresh` + the
surface law), 9 extensions green (10/30/7/42/10/5/10/6/5).

### 8.2 The surface-law red: it was never the 26-stage rotation
`flow_actual_surface_factories_close_all_owners_under_neutral_grants` reports EXACTLY the 09-21 numbers
(`2051 items / 1389 bytes, 96588 of 100000 turns idle` at `bytes=1`) although the peer's `maintenance_step`
rotation fix has landed — `PluginApp::close_step` never calls `maintenance_step`, so §2.1.3/§5.1's diagnosis was
wrong. Measured with lldb on the copied test binary (debug info is off; breakpoint hit counts + lane string in x0):
after the document store (656 turns) every remaining turn lands in
`drive_artifact_owned_disposer("config-store", ArtifactStore<NoConfig, NoConfigMutation>)` (19 345 hits by turn
~20 000, still climbing). Root cause, 🏪️store/🦀️.rs `BoundedArtifactValueRetirement::close_step`:
`if maximum_items == 0 || maximum_bytes < ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES { return Pending { 0, 0 } }` — a
permanent idle answer to every sub-page grant while `next_close_byte_demand` publishes the default 1, and
`ArtifactStoreCursorDisposer` never reads demand anyway. `no_config_store_owners()` (documented as "exact store
owners for the zero-payload configuration lane") delegated to exactly these page-charged owners, and
`ViewerApp::build_draft_store_owners` (🔌️plugin/🦀️.rs:34637) installs them for every viewer's NoDraft lane.

### 8.3 Fixes (framework-general + flow; the flow framework crate and 🔌️plugin/🦀️.rs untouched)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` `BoundedArtifactValueRetirement`: frees the value on the
  first POSITIVE grant and carries the rest of its one-page charge as `debt`, reported in grant-sized instalments
  (the §7 `close_frontier_page` shape): no turn exceeds its grant, the total is still exactly one page per value,
  `terminal_is_empty`/`Drop` include `debt == 0`. Identical behaviour for grants >= 4096. New law
  `bounded_value_retirement_is_live_and_conserves_one_page_under_every_grant` + fixture
  `🏪️store/🧫️fixtures/♻️bounded-value-retirement/🔣️.json` (grants 1/64/4096/10000).
- `🔌️plugin/📝️draft/🚫️none/♻️retirement/🦀️.rs`: the NoDraft retirement becomes the shared
  `zero_payload_store_owners::<P, M>()` (one owner, zero bytes, any positive grant);
  `🔌️plugin/🎚️config/🚫️none/♻️retirement/🦀️.rs`: `no_config_store_owners()` now uses it — NoConfig holds no payload.
- flow viewer (`…/✳️any/👁️viewer/🦀️.rs`): `no_config_store_owners()`/`no_config_store_disposer()` instead of the
  page-charged `bounded_config_store_*::<NoConfig, …>`.
- Completed the in-flight flow `setActiveExample` wiring (kept the author's mid-table ordinal, which they already
  restated in the SetGridVisible bytes law): wire keyword `set-active-example` (the kebab-of-id law), interactive-job
  fixture no longer lists it as a direct-store tool (the code routes it through the graph-operation factory), the
  graph-operation probe law probes it (`demo`), host-wire fixture ordinals +1 after ordinal 13.
  `🛂️.descriptor.semio` is stale by construction → `describe.request/flow` touched (describe is forbidden to me).

### 8.4 Pane `#flow` on :6033 (03:46, activation 03:23) — boots, but the canvas is EMPTY
One page, `🧪️probe-console.mjs` + `⚡️cache/play-fleet/flow/probe/probe-flow-404.mjs` (logs failing URLs, shell
state, screenshot `probe/s7/flow.png`): `data-shell-ready` reached, chrome shows the curated example label
**Demo**, 0 page errors, 0 refused inputs, `[DEBUG] flow surface created … webgpu`, `dag draw lod=normal`.
ONE console error: HTTP 404 on `🔌️plugin-modules/🪞️vendor/🔤️guestslim-typst-fonts.bin` (a staged vendor asset
missing from the activation — not flow code; routed to the coordinator). The screenshot shows only the grid: no
widget, no wire, empty DSL window.
**Cause (flow-side):** the other session's `setActiveExample` rollout made `demo` the curated default
(`plugin().editor_with_examples(…, [demo::source()])`), but `🖼️assets/🎬️demo/🗣️.dsl.semio` is a 2026-09-09 JSON
document that holds only a composed `content` REFERENCE (`flow-content-877ad0c8ad49fb9b`) and no scene. The
JSON parse path of `FlowSnapshot::parse_dsl` caches no working scene, so `flow_genesis_content_pack` answers `None`
and the `content` child opens empty. The rollout's own law passed only because an empty graph "changes the widget
count". **Fix:** the demo asset is now WRITTEN (never hand-edited) by `zzz_write_demo_example_asset` from
`demo_host_snapshot()` = the default slider → add → preview graph with layout (40/120/240, −40, the same layout the
generation2d demo paints with) in the host grammar that `parse_dsl` caches as the child's genesis scene. New laws:
`demo_example_ships_the_laid_out_default_graph_as_its_content_genesis` (asset == writer output, 3 widgets,
2 resolvable synapses, layout for all, genesis pack `Some`) and the restated
`set_active_example_demo_loads_the_published_demo_graph` (live document == demo graph, incl. layout).

### 8.5 run6 (04:46, `⚡️cache/play-fleet/flow/run6.txt`) — proof of §8.3/§8.4
Writer `zzz_write_demo_example_asset` wrote `🖼️assets/🎬️demo/🗣️.dsl.semio` (host grammar: `slider`/`add`/`preview`,
`s1 slider@number->add@a`, `s2 add@sum->preview`, layout 40/120/240 × −40). Then the suite:

| crate | run4 (baseline) | run6 |
|---|---|---|
| `semio-s-artifact-flow-flow` | 251 / **4** | **256 / 0** (1 ignored = the writer) |
| `semio-s-plugin-flow` | 2 / **2** | 2 / **2** (`descriptor_is_fresh`, surface law) |
| 9 `…-extension-*` | green | **green** (10/30/7/42/10/5/10/6/5) |
| kernel `bounded_value_retirement_is_live_and_conserves_one_page_under_every_grant` | — | **1 / 0** |

The surface law now gets past the config lane (2 153 items / 1 483 bytes vs 2 051 / 1 389) and stalls at a
SECOND gate: a `[DEBUG]` probe after the stall shows 64 turns at 2 or 4 bytes release nothing, 64 turns at
4 096 bytes release 64 items / 1 214 bytes — so it is neither a UTF-8-scalar gate nor a livelock, it is another
stage that refuses grants below some constant in (4, 4096]. lldb is no longer usable (Developer mode is disabled;
every new attach now waits on a GUI authorization), so run7 bisects the threshold from inside the law.
`descriptor_is_fresh` needs `describe` (requested, forbidden to me).

### 8.6 The last surface-law red: `AppActionRegistry::close_step` — framework, 🔌️plugin/🦀️.rs (routed, not mine)
run7 (`⚡️cache/play-fleet/flow/run7.txt`, bisection from inside the law, `[DEBUG]` probe since removed): after the
stall at `bytes=1`, grants 8…64 release nothing; the first productive grant is **128**; the next 12 steps at 128
each release ONE item of 4/15/9/18/14/16/16/10/13/3/14/16 bytes — one catalogue key per turn. The owner is
`AppActionRegistry::close_step` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` ≈ :13304–:13412, reached
from `close_retained_fields_step` via `self.registry.close_step(maximum_items.min(1), maximum_bytes)`): every branch
(`actions`, `window_actions` rows/owners, `mode_commands` rows/owners, `app_commands`, `tool_runs`, `interactions`,
`window_body_keys`, `interaction_window_bodies`, `controller_id`) does
`if bytes > maximum_bytes { return Pending { 0, 0 } }` — a key longer than the grant is never released. Every app
has catalogue keys longer than 1 byte, so NO app can finish a neutral 1-byte close, and window-action keys
(owner + row, 65–128 B here) block even 64-byte closes. `🔌️plugin/🦀️.rs` is on the peer's do-not-edit list, so this
is a PROPOSED DIFF for the peer, in the same `debt` shape as §7.4 / `📓️flow-driver-debt-shape.md` /
§8.3's `BoundedArtifactValueRetirement`:

```rust
pub struct AppActionRegistry {
    …
    /// 🎟️ Key bytes already removed but not yet reported to a sub-key grant.
    close_debt: usize,
}

pub(crate) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
    if maximum_items == 0 || maximum_bytes == 0 {
        return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
    }
    if self.close_debt > 0 {
        let paid = self.close_debt.min(maximum_bytes);
        self.close_debt -= paid;
        return PluginCloseStep::Pending { released_items: 0, released_bytes: paid };
    }
    // every branch: remove the entry unconditionally, then
    //     return self.charge(bytes);
    …
}

fn charge(&mut self, bytes: usize, maximum_bytes: usize) -> PluginCloseStep {
    let paid = bytes.min(maximum_bytes);
    self.close_debt = bytes - paid;
    PluginCloseStep::Pending { released_items: 1, released_bytes: paid }
}

pub(crate) fn terminal_is_empty(&self) -> bool { … && self.close_debt == 0 }
```

Delete the nine `if bytes > maximum_bytes { return Pending { 0, 0 } }` guards. Grants ≥ the key length behave
exactly as today (debt stays 0). Law to add next to it: close a registry holding a 100-byte window-action key at
`bytes=1` → completes, `Σ released_bytes ==` total key bytes, no `Pending { 0, 0 }` turn.
The same "atomic item priced against one grant" shape still exists (not exercised by this law) at
`🔌️plugin/🦀️.rs` :15203 (`BoundedConfigValueRetirement`, a verbatim duplicate of 🏪️store's
`BoundedArtifactValueRetirement` that should delegate to it) and :15883 (bounded store initializer close), and in
🏪️store/🦀️.rs :8924/:9040/:9102 (envelope decode record retirement below a page).

### 8.7 Final proof run8 (05:00, `⚡️cache/play-fleet/flow/run8.txt`, `[DEBUG]` probe removed)

| crate | run4 baseline | run8 final |
|---|---|---|
| `semio-s-artifact-flow-flow` | 251 / 4 | **256 / 0** (+1 ignored writer) |
| `semio-s-plugin-flow` | 2 / 2 | **3 / 1** (`descriptor_is_fresh` green after the coordinator's 04:52 describe) |
| 9 `…-extension-*` | green | **green** (bim 10, brep 30, dictionary 7, draw 42, list 10, logic 5, math 10, primitive 6, text 5) |
| kernel bounded-value law | — | **1 / 0** |

Only red: `flow_actual_surface_factories_close_all_owners_under_neutral_grants` = §8.6 (`AppActionRegistry`,
🔌️plugin/🦀️.rs, peer). Pane: `activate.request/flow` + `describe.request/flow` re-touched 04:57 (the demo asset
changed at 04:43, after the 03:23 activation) — until that activation lands `#flow` boots green but paints an
empty canvas (§8.4). No process of others was touched; lldb copies of the test binary deleted.
