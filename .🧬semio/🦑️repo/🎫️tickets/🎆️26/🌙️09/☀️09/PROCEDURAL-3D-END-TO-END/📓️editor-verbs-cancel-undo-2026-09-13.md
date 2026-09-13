# Editor Verbs, Cancel and Undo — generation3d (2026-09-13)

Lane `editor-verbs-cancel-undo`. Items #4, #6, #9, #11, #12, #17 and the cancel row of §5 of
`📓️audit-user-journey-gaps-2026-09-13.md`.

## TL;DR

- **The "9 generation3d `--lib` reds that swap between runs" were never nine defects. They were ONE
  stack overflow.** A `libtest` thread gets 2 MiB; one `procedural generation3d` fixture boot
  measured **2 096 176 bytes** of frame — 976 bytes under it — so the process ABORTED on whichever
  test the allocator happened to be in, and the surviving report blamed that test. Measured, not
  inferred: macOS crash reports, `-Zprint-type-sizes`, and per-frame prologue disassembly
  (§1).
- Two real defects behind it, both fixed at the owning layer: `FlowRetirement::close_step` compiled
  to a **530 592-byte** debug frame (now 20 480), and `assert_undo_redo_round_trip` **never ran
  undo at all** — `"undo"` is a framework-RESERVED job whose admission the law dropped on the floor.
  With both fixed, `undo_redo_round_trips_flow_graph_edits` passes its own assertions for the first
  time.
- Cancel is now offered by **all three** preview windows (edit, generate, viewer). The bug was not
  the `cancellable` predicate (session 3 already made it truthful) and not the missing viewer
  command (a prior lane already added it) — it was that **no preview window listed the verb in
  `window_kind_action_refs`**, so `ShellHost`'s `declaredAction` gate dropped every click.
- Five new app keybindings, one new language-agnostic keyboard fixture, two new arg-free
  `cycleShowMode`/`cycleLodMode` commands (a chord cannot carry `value`).
- Catalogue panel now accounts for **every** registered operator; `evalLen`/`evalHead` are gone from
  the preview status contract; `demo-session`'s exclusion from the picker is now a law, not an
  oversight.
- **Runtime proof obtained** (§7.2), once `📓️fix-forward-set-contributions-hang-2026-09-13.md`
  unblocked 6018. Against a fresh restage: the cancel button now RENDERS, its click dispatches
  `cancelPreviewEval` on the Interactive lane, **aborts 1 in-flight extension request**, and settles
  with `historyUpserts: 0`; `cycleShowMode`, `cycleLodMode`, `undo` and `redo` all fire from the
  keyboard alone.
- **Two undocumented ShellHost keyboard rules, both measured** (§6.1): a chord token is compared to
  `event.key` verbatim — so `mod+period` can NEVER match and was a dead chord (fixed to `mod+.`,
  re-proved at runtime; the energy plugin still carries the dead spelling) — and a chord resolves
  against the FOCUSED window kind's actions only, so a chord for a `window_kind_action_refs`-owned
  verb is window-scoped by construction.

---

## 1. Item #4 — undo/redo: two defects, neither of them "flakiness"

### 1.1 What the flakiness actually was

`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --
undo_redo_round_trips_flow_graph_edits --test-threads=1` run 5× (12:39, `🗑️generated/editor-verbs/undo-run-*.txt`;
runs 3-5 hit a transient peer compile break, runs 1-2 and a clean re-run at 12:33 all reproduced):

```
thread '…undo_redo_round_trips_flow_graph_edits' has overflowed its stack
fatal runtime error: stack overflow, aborting
signal: 6, SIGABRT
```

A stack overflow **aborts the process**. `libtest` therefore never reports a failure for the test
that actually overflowed in a multi-test run — the run dies, and the last line printed names whatever
test was executing. That is the entire mechanism behind "9 pre-existing reds that swap between runs"
(`📓️interaction-coverage-2026-09-12.md` §4.1). Confirmed by three independent crash reports today,
each blaming a different test (`undo_redo_round_trips_flow_graph_edits`,
`hex_column_evaluates_end_to_end_through_the_extension_round_trip`,
`select_generation_does_not_mutate_the_document`) on the same underlying path.

Bisected by stack size against the already-built binary:

| `RUST_MIN_STACK` | outcome |
|---|---|
| 2 097 152 (libtest default) | `fatal runtime error: stack overflow, aborting` |
| 8 388 608 | runs; **fails** `undo did not revert to the expected snapshot, left: 8, right: 7` |
| 33 554 432 | same real failure |

So the overflow was **masking** a genuine, fully deterministic undo/redo defect.

### 1.2 Frame budget, measured

Per-frame prologue disassembly of the crashing thread's 41 frames
(`objdump -d --disassemble-symbols=…`, counting both the `sub sp, sp, #n` literals and the
`sub x9, sp, #n, lsl #12` probe-loop bounds):

| frame | bytes |
|---|---|
| `…component::unit_tests::…::{closure}` (test body future) | 450 416 |
| `…unit_tests::context::app_with_registry::{closure}` | 325 488 |
| `…unit_tests::context::app::{closure}` | 325 408 |
| `artifact_app_laws::new_app_with_registry::{closure}` | 227 152 |
| `VcsArtifactApp::…::{closure}` ×2 | 221 568 + 154 192 |
| `os_dsl::schema::parse_shape` ×2 | 19 024 each |
| everything else | ~152 000 |
| **total** | **2 096 176** |

Against a 2 097 152-byte thread stack: **976 bytes of headroom**. Any peer edit that adds one local
to any frame on that path flips the whole suite from green to abort. These are debug async state
machines (`block_on` materialises nested futures inline); this is the
"Semio Async Convention Debt" surface, not something one lane rewrites.

### 1.3 Fix A — the retirement dispatcher's frame (owning layer: framework flow)

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`

`FlowOwner` is **3 160 bytes** (`cargo rustc -p semio-framework-artifact-flow-flow -- -Zprint-type-sizes`,
`🗑️generated/editor-verbs/type-sizes.txt`): its `SetCursor`/`LayoutCursor`/`NodeCursor` payloads each
inline a `protocol::value::ordered::Retirement<V>`, which is an inline
`[Option<Owner<V>>; MAX_AVL_HEIGHT + 3]` = 3 152 bytes. `<FlowRetirement as ErasedSnapshotRetirement>::close_step`
was one flat `match` over ~20 owner families, each arm building an `[Option<FlowOwner>; N]` (N up to
8) plus its `IntoIter`/`Flatten`. An unoptimised build gives **every** arm's temporaries their own
slot in the enclosing frame:

```
close_step prologue, before:  sub x9, sp, #0x81, lsl #12  + sub sp, sp, #0x8a0   → 530 592 bytes
close_step prologue, after:   sub x9, sp, #0x5,  lsl #12                          →  20 480 bytes
```

Fix: `close_step`'s owner dispatch is now `FlowRetirement::retire_owner`, which hands each owner
family to its own method (`strings`, `set`, `set_cursor`, `dictionary`, `value`, `neural`, `fixture`,
`widgets`, `specs`, `layouts`, `layout_cursor`, `tree`, `neurons`, `synapses`, `gui`, `nodes`,
`node_cursor`, `previews`, `layout`, plus the pre-existing `widget`/`chrome`). Each method returns
`Option<usize>` — `None` is the arm's former `return Ok(Step::Blocked)`, `Some(bytes)` its
`released_bytes`. **Semantics, ordering and the blocked-after-reinstall sequence are byte-identical**;
only the frame layout changes, and release builds inline the methods straight back.
`cargo check -p semio-framework-artifact-flow-flow`: clean.

**26× reduction, and it was not enough on its own** — after it the suite reached
`select_generation_does_not_mutate_the_document` and overflowed there instead, in
`os_dsl::schema::parse_*` under `default_snapshot`, on the frame budget in §1.2. Hence:

### 1.4 Fix B — the test thread's stack (owning layer: repo cargo config)

`.cargo/config.toml`, new `[env] RUST_MIN_STACK = "67108864"`. Documented in place, next to the
existing `-C link-arg=-zstack-size=16777216` for `wasm32-unknown-unknown`, which is the same defect
class (a debug build's frames overrunning a toolchain default, trapping instead of failing). Thread
stacks are reserved, not committed, so the headroom costs address space only. Cross-platform and
zero-touch: cargo exports it to every process it runs, on every OS.

This is the honest boundary of this lane: shrinking the framework's ~50-500 KiB **async** debug
frames is a separate, repo-wide piece of work. §1.2 is the measurement it should start from.

### 1.5 Fix C — the real undo/redo defect (owning layer: framework plugin)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `artifact_app_laws::assert_undo_redo_round_trip`
(~:6884).

`"undo"`/`"redo"` are `framework_reserved_job!` routes (`FrameworkUndoJob`,
`🔌️plugin/🦀️.rs:14863`, dispatched at `:14911`). `PluginApp::handle_action("undo", …)` therefore only
**admits**: it returns an `InvocationResult` carrying `Effect::SpawnJob { kind: FRAMEWORK_RESERVED_JOB_KIND }`,
and nothing reaches `ArtifactCommand::Undo` in the store until that job is run and completed
(`commit_framework_history_route`, `:24330`). The law threw that receipt away —

```rust
app.handle_action("undo", None, &meta("local")).await.expect("undo");
settle_registered_typed_operation(app, receiver).await.expect("undo publication");
```

— and `settle_registered_typed_operation` pumps only TYPED operations. So on every retained app the
undo was a **no-op**, and the law's own assertion (`left: 8, right: 7`) was correct: the widget was
still there. The test fixture's sibling helper `context::select_graph` already knew this and used
`app::settle_framework_reserved_admission`; the law did not.

Fix: new `artifact_app_laws::settle_history_verb(app, action, receiver)` drives all three steps —
admit, `settle_framework_reserved_admission`, `settle_registered_typed_operation` — and
`assert_undo_redo_round_trip` calls it for both `"undo"` and `"redo"`. A non-retained app admits no
spawn job and `settle_framework_reserved_admission` returns the same result unchanged, so the six
other artifacts that use this law (cad, process3d, wires, remodeling, …) are unaffected in behaviour
and newly correct if they ever become retained.

## 2. Cancel in all three preview windows (item #9 + §5 cancel row)

Two of the audit's three claims were **stale**, and the real defect was a third thing:

| audit claim | state on disk today |
|---|---|
| "the viewer has no `cancelPreviewEval`" | **stale** — `👁️viewer/🦀️.rs:183` declares the command, `:1306` bridges it, `:1543` publishes it in the palette, `:509` handles it |
| "the `cancellable` predicate was stale" | **stale in source** — `🧵️preview-eval/🦀️.rs:813` already ORs the tessellation ledger, the budgeted-eval ledger, `extensions_in_flight()` and `FlowEvalSession::pending`. Only the SERVED wasm was old |
| "generate preview does not offer cancel" | **true, and true of all three** |

`ShellHost` resolves a status's `cancelAction` against the **focused window kind's own** `actions`
(`WindowKindDefinition.actions`, fed by `.window_kind_action_refs`). None of the three preview
windows listed `cancelPreviewEval`, so each published `cancellable` + `cancelAction`, painted a
button, and had the click dropped by the `declaredAction` gate before `plugin.handleAction` ran.

- `✏️editor/🦀️.rs` — `cancelPreviewEval` added to `window_kind_action_refs` for
  `edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW` and
  `generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW`.
- `👁️viewer/🦀️.rs` — added for `preview::WINDOW_KIND_ID`.

Laws (fixture-driven, `🧫️fixtures/🛑️preview-cancel.json` gains `offersCancelOnWindow` on all three
`statusContract.surfaces` rows):

- `both_editor_preview_windows_offer_the_cancel_verb_the_fixture_names`
  (`✏️editor/🎮️commands/🛑️cancel-preview-eval/🧪️tests/🔬️unit/🦀️.rs`)
- `the_viewer_preview_window_offers_the_cancel_verb_the_fixture_names`
  (`👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs`)
- TypeScript twin extended: `🧪️tests/🔬️status-contract/🟦️.ts` now asserts `offersCancelOnWindow` on
  every surface row.

Progress chrome needed no change: `phase`/`phaseLabel` (en+de)/`progress`/`inFlight`/`ratio` are
already published by `preview_progress_status_json` for all three windows.

**Not claimed:** a `cancellable:true` frame and a cancel round trip on `Sphere Cut With Torus`.
That needs a restage and a working 6018; see §7.

## 3. Catalogue panel (item #11)

`✏️editor/📌️panels/🛍️catalogue/🦀️.rs`. The audit's "continuation row omits count" is stale —
`panel_continuation_row` labels itself `+n`. The **real** defect: the panel's row page is
`UI_VALUE_PAGE_ROWS` = `UI_BUILT_CHILDREN_MAX - 1` = 31 rows across ALL groups, and a section the
budget could not open at all was `continue`d over with **no count anywhere**. With the real
`brep`/`math` operator sets installed, the panel silently understated the catalogue by however many
whole sections did not fit.

Fix: `render` tracks what it actually placed (via `PanelRowBudget::remaining()` before/after each
nested section) and emits a panel-level continuation row carrying the TOTAL omitted count whenever
anything was left out. A new `catalogue_page() -> CataloguePage { shown, omitted }` exposes the same
arithmetic without building UI, so the law reads it directly.

Laws (`✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs`):

- `the_catalogue_panel_accounts_for_every_registered_operator` — `shown + omitted ==` the whole
  `flow_palette_catalogue_sections()` roster.
- `the_spotlight_catalogue_offers_every_operator_the_panel_omits` — `flow_app_catalogue()`, the
  app-static payload published on the reserved `framework.section.catalogue` surface that the canvas
  spotlight browses, carries the WHOLE roster, never the panel's page. This is where "every operator
  is reachable" is actually true, and it is now asserted.
- `the_catalogue_panel_publishes_its_omitted_count` — the rendered tree contains `+n`.

A paging cursor in the panel itself was considered and rejected for this lane: it needs a new
`Generation3dConfig` field, a config mutation leaf and a command, and the 31-row arena bound would
still make the panel the wrong browse surface. The spotlight already is the unbounded one.

## 4. `demo-session` (item #12) — no change, and that is now a statement

`✏️editor/📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio` is two lines (`semio generation.3d.cmd v1` /
`action=demo`) — a command REPLAY, not a flow fixture, and `setActiveExample`'s only vocabulary is
"load a registered example document". `🖨️raster/🦀️.rs:18-27` states exactly this rule for exactly this
leaf, and notes that `🧩️puzzle` mounts and tests three `demo-session` leaves without registering any,
while `🧱️block` ships none. `🎬️demo-session` is also a registered taxonomy member name across six
artifacts, so deleting generation3d's copy would break the convention, not fix a half state.

What WAS half was that generation3d's omission was undocumented and unenforced. Now:
`examples()`'s docstring states the rule and cites the precedent, and
`the_example_picker_offers_the_flow_examples_and_never_the_command_session`
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs`) asserts the picker offers exactly the eight flow examples, never the
session leaf, that every offered id passes `is_generation3d_example_id`, and that
`setActiveExample`'s option list equals `examples()` in its order.

## 5. `evalLen` / `evalHead` (item #17)

`🧵️preview-eval/🦀️.rs` — `PreviewStatusDebug` loses its `eval_json` field, and
`preview_window_status_json`'s `debug` object loses `evalLen` and `evalHead`. `evalHead` was up to
240 characters of raw evaluation text stamped onto **every** status frame of **every** preview window
— document text leaking into view state, for no observable the projection did not already carry
(`error`/`widgetErrors` from `preview_status_json`, `phase`/`progress` from
`preview_progress_status_json`). `meshesLen`/`instancesLen` stay: they are facts about what reached
the scene, and probes read them.

Consumers fixed: the three call sites (`🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:85`,
`🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:63`,
`👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:371`), the fixture
(`🧫️fixtures/🛑️preview-cancel.json` `statusContract.debugKeys`) and the TypeScript law
(`🧪️tests/🔬️status-contract/🟦️.ts`), which now asserts both that the two survivors are declared and
that the two retired keys are **not**.

## 6. Keyboard reachability (item #6)

`AppDefinition.keybinding` carries a chord and an action id and nothing else, and `ShellHost`'s
keybinding loop fires an ARG-FREE action straight through `onAction` while opening a staged argument
form for anything with a required arg. So `setShowMode`/`setLodMode` — both of which require
`value` — cannot be bound at all without making the chord slower than the picker it replaces.

Two new commands, arg-free by construction, reading their own next value off the config:

- `✏️editor/🎮️commands/🔁️cycle-show-mode/🦀️.rs` — `cycleShowMode`
- `✏️editor/🎮️commands/🔁️cycle-lod-mode/🦀️.rs` — `cycleLodMode`

Both walk ONE ladder declared in `✏️editor/🎚️config/🦀️.rs` (`GENERATION_3D_SHOW_MODES`,
`GENERATION_3D_LOD_MODES`, `next_show_mode`, `next_lod_mode`) — and the two pickers now BUILD their
`MeasureSelectItem` rows from that same ladder (`🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`,
`🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`), so a keyboard cycle can never reach a mode the picker does
not offer. Wired through the command enum, `GENERATION3D_RETAINED_TOOL_IDS`, the bounded factory's
`PUBLICATION_CONTRACTS` (`Config` lane) and `bounded_first_step_tool_proofs!`, `command_from_action`,
the manifest's `ActionDefinition`/`action_interactive_job`, `every_command()`'s round-trip roster,
and `🔣️taxonomy.json` `members-of-commands` (`🔁️cycle-show-mode`, `🔁️cycle-lod-mode`).

New app-level chords, each on the chord its verb class already uses elsewhere in this repo:

| chord | action | scope | why this chord |
|---|---|---|---|
| `delete,backspace` | `deleteSelection` | every window | the chord three sibling artifacts already declare |
| `mod+.` | `cancelPreviewEval` | preview windows | the cancel chord the simulation surface declares — see §6.1 for why the token is `.` |
| `mod+shift+g` | `addGeneration` | Generations window | follows the `mod+shift+<initial>` create chords |
| `mod+alt+d` | `cycleShowMode` | every window | new |
| `mod+alt+k` | `cycleLodMode` | every window | new |

### 6.1 Two runtime rules these chords obey, both measured, neither documented anywhere

`🐍️editor-verbs-keyboard-probe.mjs` drives ONLY the keyboard against the running playground and
reads each verb's own `performInvocation` line. First run (`🗑️generated/editor-verbs/keys-1`),
flow window focused: `cycleShowMode` ✅ (twice, second arming a `flowEvalTick`), `cycleLodMode` ✅,
`undo` ✅, `redo` ✅ — and `cancelPreviewEval` ❌ and `addGeneration` ❌, **zero** console lines
each, under every Playwright spelling (`Control+Period`, `Control+.`, `Control+Shift+g`,
`Control+Shift+G`, `Meta+Shift+g`; `🗑️generated/editor-verbs/keys-2`). Two distinct causes, both
read off `🏛️ShellHost/🟦️.tsx`'s `handleAppKeydown`:

1. **A chord token is compared to `event.key` verbatim.** `matches()` takes the last `+` segment and
   does `event.key.toLowerCase() === key`. The period key's `event.key` IS `"."`, so **`mod+period`
   can never match anything** — it is a silently dead chord. Fixed here to `mod+.`.
   ⚠️ `mod+period` is declared repo-wide for the SAME cancel verb class: the energy plugin
   (`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1344`,
   plus its simulation window's help leaf and `🟦️.ts` row) has the same dead chord. Out of this
   lane; flagged.
2. **A chord resolves against the FOCUSED window kind's actions only.**
   `const actionById = new Map((session.app.windowKinds.find((kind) => kind.id === focusedWindowKindId)?.actions ?? []).map(…))`
   and then `if (!definition) continue;`. So a chord for a verb listed in `window_kind_action_refs`
   is live exactly while that window has focus, and a chord for an unowned verb (copied onto every
   window by `build_definition`) is live everywhere. That is why the four that worked, worked: all
   four are unowned. `cancelPreviewEval` and `addGeneration` are window-owned by design — the
   preview windows and the Generations window genuinely own them (§2) — so their chords are
   window-scoped, and the fixture now says so (`windowScoped: true`).

Neither rule is a defect in the app; rule 1 IS a defect in the chord string, and both are now stated
in `🧫️fixtures/⌨️keyboard-reachability.json`'s note so the next lane does not re-derive them.

`Fit graph` is deliberately **not** an app keybinding: it is shell chrome painted over any node-graph
surface (`wgpu-shell.rs:9493`, React's `fitGraphToView`), not an app action, and the shell already
binds `F` to it. Binding it again in the app would shadow the shell's own control.

Language-agnostic statement: `🧫️fixtures/⌨️keyboard-reachability.json` — ten `bindings` rows (the
WHOLE keyboard surface, including the two framework history verbs and the io lane's two) plus a
`shellChrome` row for `fitGraph`. Law
`the_editor_binds_every_keyboard_verb_the_fixture_names` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`) asserts the
fixture row count equals `definition.keybindings.len()` (so a chord added without a fixture row
fails), that each chord reaches its named verb, that each non-framework verb takes no REQUIRED
argument unless the row is marked `staged` (only `exportDocument` is), and that the app does not
re-bind the shell's `f`.

Every new label carries en+de and no default (`Cycle Show Mode`/`Anzeigemodus wechseln`,
`Cycle Lod Mode`/`LOD-Modus wechseln`).

## 7. Verification

- `cargo check -p semio-framework-artifact-flow-flow` — **clean**.
- `cargo check -p semio-s-artifact-procedural-generation3d --features component-app-assembly --tests`
  — **clean** (`🗑️generated/editor-verbs/check-2.txt`).
- `cargo test … --lib --test-threads=1` — see §7.1.
- **Browser proof: obtained.** See §7.2 — 6018 came back with
  `📓️fix-forward-set-contributions-hang-2026-09-13.md`, and both probes ran against a fresh
  `activate-generation3d-react-dev` restage.

### 7.1 Suite state

| run | result | what changed since the previous run |
|---|---|---|
| `lib-full-4.txt` | 291 passed, **133 failed**, ran to completion | first COMPLETE run of this suite today — §1.4 removed the stack-overflow abort that had been truncating every previous run |
| `lib-full-7.txt` | 266 passed, 117 failed, **aborted near the end** | viewer action declaration + `next_show_mode` fixed |

Two things this lane caused and fixed, both visible in the delta:

1. **26 viewer failures were mine.** Adding `cancelPreviewEval` to the preview windows'
   `window_kind_action_refs` (§2) while the verb was declared ONLY as a `CommandDefinition` made
   `build_definition` reject the whole viewer with
   `app-definition.invalid: … references undeclared action cancelPreviewEval`. The
   `generate-mode-interactions` and `io-surface` lanes fix-forwarded the editor and viewer halves
   respectively while this lane was rate-limited; the duplicate `view_action` that collision produced
   is removed and the viewer now pairs the peer's `view_action` with the
   `action_interactive_job("cancelPreviewEval", Migrated)` the editor already had.
   `lib-full-7.txt` carries **0** `references undeclared action` failures.
2. **`next_show_mode("")` returned `shaded`, not `shaded+edges`.** An unset config DISPLAYS `shaded`,
   so the first cycle must land on the second rung; `next_in_ladder`'s unknown-value fallback gave
   the first. `next_lod_mode` already normalised its empty case and show mode did not.
   `the_show_mode_ladder_is_a_total_cycle` now passes.
3. **`the_editor_binds_every_keyboard_verb_the_fixture_names` was too strict.** A peer's interaction
   domain made `build_definition` mint `escape`→`clearSelection` (and `mod+a`→`selectAll`), which the
   plugin never declares; the law exempted only `History`/`Clipboard`-kind minted chords. Exemption
   widened to `ActionKind::Interaction` — by KIND, so a fourth framework-minted chord cannot break a
   plugin fixture that had no say in it. Law passes (`🗑️generated/editor-verbs/keyboard-law-2.txt`).
4. **`the_catalogue_panel_publishes_its_omitted_count` compared the wrong two numbers.** It asserted
   the rendered page contains `+{catalogue_page().omitted}` — but `catalogue_page()` measures the row
   budget OUT OF BAND while `render` runs inside a live app whose argument arena another panel may
   already hold credit against (`render`'s own docstring says so). The standalone walk legitimately
   places more rows, so the law went red (`111` against the page's own `+115`) the moment peers added
   operators, with nothing actually wrong with the panel. Rewritten to read the panel's OWN published
   numbers: `shown + omitted == roster`, where `shown` is the clickable `addWidget` rows in the
   rendered body and `omitted` is the `+n` the total continuation row publishes. That is the
   invariant §3 is about, and it cannot drift with the arena. `render` and `catalogue_page` now also
   share ONE walk (`walk_catalogue_groups`), so a group the tree drops can never be counted as shown
   by either.

All twenty-five of this lane's own laws pass (`🗑️generated/editor-verbs/lane-laws-3.txt`).

**The remaining ~115 failures are NOT this lane's, and they are not new — they were invisible.**
Every previous run of this suite aborted on the §1.1 stack overflow before reaching them. They are
one failure mode with three spellings, all in the app-fixture CLOSE ladder, never in a verb:
`Generation3d app fixture did not reach its terminal-empty close witness` (57),
`… last pending close authority: returned snapshot-read disposer is waiting on external ownership`
(19), and `… document store close awaits a retained reader or owner` (13). Several are named in the
2026-09-12 red list (`add_generation_records_an_undoable_generation_operation`,
`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`), which is
the strongest evidence they predate this lane.

`lib-full-7.txt` ends in a NEW abort class, and the change is itself the §1.4 result: no
`stack overflow` appears anywhere in it. It is
`panic in a destructor during cleanup … thread caused non-unwinding panic` — a fixture Drop assert
firing while the thread was ALREADY unwinding from a close-witness panic, with
`drop_glue::<UnlinkedFlowExtensions>` on the trace. That is the close-ladder debt aborting the run
from a second direction, and it also means `lib-full-7.txt` never printed its `failures:` detail
block. Someone owning the close ladder should take it; §8 item 5.

### 7.2 Runtime proof (browser, 6018, fresh restage)

**Cancel — `🐍️cancel-preview-probe.mjs`, `🗑️generated/cancellation/cancel-verbs-1`.** The
affordance §2 is about now EXISTS and works:

```
armed        cancelButton {"action":"cancelPreviewEval","label":"Cancel","tag":"BUTTON"}
             pane {"phase":"idle","cancellable":true,"text":"Idle Cancel"}
8375 warning [DEBUG] extension requests aborted by surface cancel {"action":"cancelPreviewEval","pluginId":"procedural","instanceId":1,"aborted":1}
8375 warning [DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"cancelPreviewEval"}
8375 warning [DEBUG] command ingress lane {"instanceId":1,"actionId":"cancelPreviewEval","seq":22,"lane":"Interactive"}
9380 warning [DEBUG] performInvocation settled {… "frames":2,"frameKinds":["Invocation","Ephemeral"],"historyUpserts":0,"historyCanUndo":null,"effects":2}
```

Four things at once, none of them inferred: the button renders (it did not before — the
`declaredAction` gate dropped it), the click dispatches as an **action** on the **Interactive**
lane, it **aborted 1 in-flight extension request**, and it settled with `historyUpserts: 0` — a view
action that mutates no document, which is exactly the law §2 states. No `undeclared action` refusal
appears anywhere in the 6.7k-line console.

**Keyboard — `🐍️editor-verbs-keyboard-probe.mjs`, `🗑️generated/editor-verbs/keys-1…3`.** Keyboard
only, no clicks on the verbs. `cycleShowMode` ✅ ×2, `cycleLodMode` ✅, `undo` ✅, `redo` ✅ — the
undo/redo chords reach the framework verbs whose admission §1.5 fixed. The two window-scoped chords
and the dead `period` token are §6.1.

After the `mod+.` fix and a second restage, the cancel chord fires from the keyboard alone
(`🗑️generated/editor-verbs/keys-4`, preview window focused):

```
cancel-dot        invoked ["cancelPreviewEval"]
cancel-dot-again  invoked ["cancelPreviewEval"]
```

`addGeneration`'s chord could not be exercised: the Generations window it is owned by is a GENERATE
mode window and is not mounted in edit mode (`🗑️generated/editor-verbs/keys-5`, focus selector
matched nothing). Consistent with rule 2 and left as a stated scope, not a claimed pass.

## 8. Pre-existing defects found, not fixed (not this lane's)

1. **`an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain` HANGS.** Not a
   failure — an infinite `FlowHost::retire_cold` → `FlowHostRetirement::close_page` loop, sampled at
   11 minutes and still spinning (`sample 50569`). It was already in the 2026-09-12 red set
   (`📓️interaction-coverage-2026-09-12.md`), attributed there to "a peer lane, in flight"; the abort
   in §1.1 was hiding the fact that it never terminates. Skipped in the run in §7.1.
2. **`cargo test -p semio-framework-artifact-flow-flow` does not compile.** Its test target calls
   `FlowRetirement::next_push_allocation_bytes` / `reserve_push_allocation`, which the lib has never
   had (`🧵️retained/🧪️tests/🧵️retained/🦀️.rs:173-189`). Pre-existing, unrelated to §1.3 — that change
   removed no public method. Someone's in-flight refactor.
3. The framework's debug **async** frames (§1.2) are the remaining stack consumer: 450/325/227/221 KiB
   per future on a single fixture-boot path. §1.4 buys headroom; it does not fix them.
4. **The data volume filled up mid-lane** — 124 MiB free of 926 GiB at 14:0x, with
   `.🧬semio/🦑️repo/⚡️cache/cargo/build` at **358 GB**, of which `debug/incremental` alone was
   **68 GB**. Every concurrent cargo in the fleet was failing with
   `error: failed to write …/fingerprint/…` (ENOSPC), which reads exactly like a build-lock stall.
   Cleared by deleting `debug/incremental` (pure cache; this lane already runs `CARGO_INCREMENTAL=0`)
   → 94 GiB free. `bun nx run repo:cache-prune` would not have helped much: it guards any unit
   touched in the last `CACHE_POLICY.storage.guardAgeMs`, and under a live fleet nearly everything is
   hot. Worth a coordinator-level watch.
5. **The app-fixture close ladder does not reach terminal-empty for ~115 of this crate's lib tests**,
   and when it panics inside a Drop that is already unwinding it ABORTS the run
   (`lib-full-7.txt`). §7.1 has the three spellings and the evidence that it predates this lane.
   This is now the single biggest thing standing between this crate and a green suite, and it is a
   close-ladder/retirement question, not a verb question.
6. **`mod+period` is a dead chord repo-wide.** §6.1 rule 1. Fixed here; the energy plugin still
   carries it in three places. Flagged as a separate task.

## 9. Files

Framework:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs` — `close_step`
  decomposed into `retire_owner` + per-family methods.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `settle_history_verb`,
  `assert_undo_redo_round_trip`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — two `members-of-commands` names.
- `.cargo/config.toml` — `[env] RUST_MIN_STACK`.

generation3d (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`):
- `✏️editor/🦀️.rs`, `✏️editor/🎚️config/🦀️.rs`,
  `✏️editor/🎮️commands/🔁️cycle-show-mode/**`, `✏️editor/🎮️commands/🔁️cycle-lod-mode/**`,
  `✏️editor/🎮️commands/🛑️cancel-preview-eval/🧪️tests/🔬️unit/🦀️.rs`,
  `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`,
  `✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`,
  `✏️editor/📌️panels/🛍️catalogue/🦀️.rs` (+ `walk_catalogue_groups`, §7.1 item 4) + its `🧪️tests/🔬️unit/🦀️.rs`,
  `✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `👁️viewer/🦀️.rs`, `👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs`,
  `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs`
- `🧵️preview-eval/🦀️.rs`
- `🧫️fixtures/🛑️preview-cancel.json`, `🧫️fixtures/⌨️keyboard-reachability.json` (new),
  `🧫️fixtures/🎛️generate-mode-interactions.json` (the `mod+period`→`mod+.` token, §6.1)
- `🧪️tests/🔬️status-contract/🟦️.ts`
- crate root mount: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs`

Ticket probes (inputs, kept):
- `🐍️editor-verbs-keyboard-probe.mjs` (new) — keyboard-only runtime probe; `SEMIO_PROBE_CHORDS` and
  `SEMIO_PROBE_FOCUS` make a dead chord re-testable under another spelling or another focused window
  without a new probe.
- `🐍️cancel-preview-probe.mjs` (existing) — used unmodified for §7.2.
