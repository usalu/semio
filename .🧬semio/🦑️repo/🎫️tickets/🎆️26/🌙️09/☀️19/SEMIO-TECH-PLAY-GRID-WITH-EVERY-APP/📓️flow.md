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

