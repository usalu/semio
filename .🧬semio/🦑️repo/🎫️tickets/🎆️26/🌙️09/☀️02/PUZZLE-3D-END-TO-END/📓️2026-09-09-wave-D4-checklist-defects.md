# Wave D-4 — the user-feature checklist's source-read defects, closed with laws

Scope: the five defects `📓️2026-09-09-user-feature-checklist.md` found by reading current source and
listed in its own "Summary — priority order" as #7, #8, #9, #10 and #13. Every one is now fixed **and**
pinned by a test that asserts the user-visible behaviour, not the internal shape.

All eight laws were green at 18:03–18:05. As of 18:52 the three that need no actor turn still are; the
five that drive `dispatch` → `settle` are blocked by a peer wave's quiescence regression that landed at
18:10 and takes down every `dispatch`-driven test in the crate, W-D4's and everyone else's alike. Read
"Not verified" item 4 before treating any of that as this wave's breakage.

Paths below are relative to
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` unless a full path
is given. `EDITOR` = that folder's own `🦀️.rs`. Line numbers are as of 2026-09-09 18:40 on a tree
several waves are writing to concurrently — grep the named symbol if a number has drifted.

---

## 1. Checklist #7 — context menu "Zoom to Selection" dispatched an unregistered action

**Was:** the object / vortex / reference rows dispatched `"zoomToSelection"`. No such id exists —
`Puzzle3dCommand` / `TOOL_JOB_IDS` only declare `"focusSelection"`. `puzzle3d_context_menu_row`
constructs a `ContextMenuItemSpec` directly and never asks the registry, so the typo could not be
caught at build time; clicking the row faulted at `dispatch_action`.

**Now:** all three rows dispatch the registered `"focusSelection"`.

| what | where |
|---|---|
| object selection row | `EDITOR:2409` |
| vortex (single) row | `EDITOR:2430` |
| reference row | `EDITOR:2465` |

**Law:** `🧪️tests/🔬️unit/🦀️.rs:3212` `every_context_menu_row_dispatches_a_declared_action` — builds the
menu for **every** selection kind, flattens each row's `(row id, action id)`, and asserts each action
id is in the declared set (`ActionDefinition` ids from the app definition). It also asserts the three
`zoom` rows exist and each dispatches exactly `"focusSelection"`. This closes the class, not just the
instance: any future row that names an unregistered action now fails here instead of at a user's click.

---

## 2. Checklist #8 — outliner hide/lock rows never toggled back off

**Was:** `📌️panels/🗿️artifact/🦀️.rs`'s `flag_args` hardcoded `("value", ui_value_bool(true))`. The icon
and label alternated correctly ("Hide"/"Show", "Lock"/"Unlock") but every click sent
`setSelectionFlag{value:true}`, so "Show" on an already-hidden row re-applied `hidden:true` and the
object could never be un-hidden from the row that hid it. Same for Lock/Unlock, on object, reference
and target-volume rows.

**Now:** `flag_args` takes the requested `value` and `hide_lock_actions` passes `!hidden` / `!locked`,
the same negation the context menu (`!all_hidden`) and the inspection panel (`!pressed`) already used.

| what | where |
|---|---|
| `flag_args(entity, id, flag, value)` | `📌️panels/🗿️artifact/🦀️.rs:127` |
| `hide_lock_actions` passing `!hidden` / `!locked` | `📌️panels/🗿️artifact/🦀️.rs:131` |

**Laws:**

- `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:219`
  `outliner_hide_and_lock_rows_dispatch_the_inverse_of_the_current_flag` — renders a page carrying one
  object, one reference and one target volume, in **both** flag states, and asserts all six row actions
  ask for the inverse of the row's own state.
- `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:272`
  `an_outliner_flag_row_undoes_itself_on_the_second_click` (**added in this pass**) — the checklist's
  own QA closed as a loop: read the value the rendered row actually asks for, feed it to the reducer
  the row names (`apply_puzzle3d_selection_flag`, the explicit-`{entity, ids}` path
  `setSelectionFlag` takes for outliner rows), re-render, and assert the row now asks the OPPOSITE —
  for `hidden` and for `locked`, both directions. With the hardcoded `true` the second pass asked for
  `true` again and the object could never come back. Deliberately settle-free (render + pure reducer,
  no actor turn), so it stays runnable while the dispatch path is down — see "Not verified" item 4.
- `🧪️tests/🔬️unit/🦀️.rs:3356` `inspection_flag_rows_toggle_back_off` — the same round trip for the
  inspection panel's `patchInspector` flag rows, so the sibling surface cannot regress into the same
  shape.

---

## 3. Checklist #9 — the Add Object dialog's kind selector offered one static option

**Was:** both the "Add Object" dialog's arg schema and the standalone `addObjectKind` arg form declared
a single `ActionArgOption::new("Object", …)`. Unlike the Catalogue panel, which reads
`meta.kindCatalogs`, the dialog enumerated nothing — on either shipped example it could not add a single
real object kind.

**Now:** one shared select, built from the declared examples' own `meta.kindCatalogs` — exactly the rows
`📌️panels/🛍️catalogue` renders — deduplicated across examples in catalog order, with the default taken
from the first catalog row instead of a literal id no catalog declares.

| what | where |
|---|---|
| `PUZZLE3D_OBJECT_KIND_OPTIONS_MAX` (fixed ceiling, 64) | `EDITOR:7468` |
| `puzzle3d_object_kind_options()` | `EDITOR:7478` |
| `puzzle3d_default_object_kind()` | `EDITOR:7499` |
| `puzzle3d_object_kind_arg()` — the one select both forms build | `EDITOR:7505` |
| standalone `addObjectKind` arg form | `EDITOR:7619` |
| "Add Object" dialog arg form | `EDITOR:7710` |
| `catalog_entry_label` promoted to `pub(crate)` so both surfaces share one label rule | `📌️panels/🛍️catalogue/🦀️.rs:52` |

**Law:** `🧪️tests/🔬️unit/🦀️.rs:3253` `the_add_object_dialog_offers_every_object_kind_of_both_examples` —
computes the expected id set straight from both example fixtures' catalogs, asserts **both** surfaces
(dialog and standalone form) offer exactly it, that the set stays inside the fixed ceiling, that each
surface's own default is one of its own options, and that the dialog's select stays `required`. The
"built twice from the same catalog" invariant is asserted, so the two forms cannot drift apart again.

---

## 4. Checklist #10 — the engagement bar advertised verbs it silently dropped

**Was:** the placeholder was the literal string `"brush, fill <n>, zoom, clear, rectangle, lasso"` while
`engagement_submit` implemented only `fill <n>`, `brush` and `zoom`. `clear`, `rectangle` and `lasso`
were typed, accepted, and dropped by the `_ => {}` arm with no feedback.

**Now:** the advertised list is a constant the placeholder is derived from, and every verb on it has an
arm. `clear` empties the framework-owned `vortex` domain through the sanctioned reducer channel
(`Emit.interaction_writes`, expressed as a `Subtractive` write naming the current selection — an empty
`Replace` is a documented no-op in `protocol::next_selection`). `pick` / `rectangle` / `lasso` set the
marquee method the viewport sweeps with, which `world_selection_json` hands to
`World3dHost.selection.method`.

| what | where |
|---|---|
| `PUZZLE3D_ENGAGEMENT_VERBS` — the single source of truth | `🎮️commands/📨️engagement-submit/🦀️.rs:12` |
| the four new arms (`clear`, `pick`, `rectangle`, `lasso`) | `🎮️commands/📨️engagement-submit/🦀️.rs:33-38` |
| placeholder derived from the constant | `🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:536` |
| `world_selection_json` reads the live method (was a hardcoded `"pick"`) | `🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:442` |
| `Puzzle3dActionCtx::clear_selection` | `EDITOR:2278` |
| `selection_method` on the window config (projected, not scratch) | `🪟️window/🦀️.rs:27`, `:56`, `:256` |
| `selection_method` on the runtime + its default | `🎚️config/🦀️.rs:51`, `:158`, `:219` |
| schema-first, all four surfaces | `🪟️window/🧬️schema/{🔗️.graphql,🔣️.json,🛰️.proto,🟦️.ts}` |

`engagementSubmit` already publishes on the `WindowConfig` lane (`EDITOR:6465`), so the method change
is persisted, not dropped.

**Law:** `🧪️tests/🔬️unit/🦀️.rs:3286` `every_advertised_engagement_verb_is_implemented` — asserts the
placeholder advertises every verb in the constant, that a fresh window sweeps with `pick`, that typing
each of `pick`/`rectangle`/`lasso` moves the method the rendered composite hands the host, and that
typing `clear` takes a one-object selection to zero. An advertised verb with no arm now fails here.

**Docstring repair made in this pass:** `🎚️config/🦀️.rs:158` claimed the field was "ephemeral per-window
command-line state" that "`engagementAbort` resets". Both halves were false —
`🎮️commands/🛑️engagement-abort/🦀️.rs` deliberately leaves it alone, and `🪟️window/🦀️.rs:24` states the
opposite (and correct) design: it is a window option, so Escape never silently puts the marquee back to
a shape the user did not ask for. The docstring now says what the code does.

---

## 5. Checklist #13 — silent engine-failure swallowing

**Was:** `addBrushObject` and `acceptSuggestion` both ended in `if let Ok(Puzzle3dEngineOutcome::
Fixture(f)) = outcome`. A real engine refusal (collision / overlap budget), an absent target vortex, an
absent candidate, and a fixture the app model could not adopt all fell out of that `if let` with nothing
on screen: the click looked like it did nothing.

**Now:** every dead end speaks, through the shell's transient-notice channel, in the host's own declared
locale × terminology axes. At most one notice per action, so the effect list stays fixed-width; an
unauthored axis carries the app's `ui.localization.unsupported` code rather than an English sentence,
because this UI has no default language and a silent drop would restore the very defect being fixed.

| what | where |
|---|---|
| `Puzzle3dActionCtx::notice` (one notice per action, locale-resolved) | `EDITOR:2303` |
| `puzzle3d_notice_emit` — the retained-command equivalent | `EDITOR:3102` |
| `PUZZLE3D_LOCALIZATION_UNSUPPORTED` | `EDITOR:91` |
| retained `addBrushObject` work: unavailable / rejected / occupied | `EDITOR:5325`, `:5331`, `:5342`, `:5359`, `:5374`, `:5404` |
| retained `acceptSuggestion` work: unavailable / occupied | `EDITOR:5858`, `:5864`, `:5868`, `:5890`, `:5926`, `:5954` |
| `add_brush_object` reducer arm | `🎮️commands/🖌️add-brush-object/🦀️.rs:37` |
| `accept_suggestion` reducer arms | `🎮️commands/✅️accept-suggestion/🦀️.rs:34`, `:38`, `:56` |
| EN + DE prose, both terminology axes | `🗣️terminology/🦀️.rs:38-40` |
| shell end: `kernel::Effect::Notify` → `showTransientNotice` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4443-4447` (ref forwarded at `:1908`, assigned at `:6637`) |

`Effect::Notify` and its `{ readonly notify: { readonly message: string } }` arm already existed in
`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1177` — unchanged; only the ShellHost branch consuming it is new.

**Laws:**

- `🧪️tests/🔬️unit/🦀️.rs:3316` `a_refused_placement_surfaces_exactly_one_notice` — an accept with no
  target vortex and a brush placement onto an undeclared kind each raise exactly one `Effect::Notify`;
  each notice carries real prose (asserted `!=` the `ui.localization.unsupported` code, since the test
  host declares an authored axis) and is non-empty; and the happy path raises none.
- `🧪️tests/🔬️unit/🦀️.rs:1470` `accept_suggestion_closes_menu_even_when_placement_fails` — the popup
  still closes on every dead end, so speaking the failure did not turn a silent no-op into a stuck menu.

---

## Commands run, and their tails

Rust, foreground, from the repo root, with
`RUSTC_WRAPPER="" RUST_MIN_STACK=134217728
CARGO_TARGET_DIR=…/scratchpad/target-p3d` (private, seeded 18:04):

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --no-run
  → Finished `test` profile [unoptimized] target(s) in 2m 26s          (18:02, tree compiles)

cargo test … -j 4 <name> -- --test-threads=1            (each, 18:03–18:05, same build)
  every_context_menu_row_dispatches_a_declared_action ................ ok
  the_add_object_dialog_offers_every_object_kind_of_both_examples .... ok
  every_advertised_engagement_verb_is_implemented .................... ok
  a_refused_placement_surfaces_exactly_one_notice .................... ok
  inspection_flag_rows_toggle_back_off ............................... ok
  outliner_hide_and_lock_rows_dispatch_the_inverse_of_the_current_flag  ok
  accept_suggestion_closes_menu_even_when_placement_fails ............ ok
  → "test result: ok. 1 passed; 0 failed; … 594 filtered out" for each
```

Re-run of all eight at **18:52**, after the peer wave described in "Not verified" item 4 landed:

| law | 18:03–18:05 | 18:52 | needs the actor turn? |
|---|---|---|---|
| `outliner_hide_and_lock_rows_dispatch_the_inverse_of_the_current_flag` | ok | **ok** | no |
| `an_outliner_flag_row_undoes_itself_on_the_second_click` | (written 18:50) | **ok** | no |
| `the_add_object_dialog_offers_every_object_kind_of_both_examples` | ok | **ok** | no |
| `every_context_menu_row_dispatches_a_declared_action` | ok | blocked | yes |
| `every_advertised_engagement_verb_is_implemented` | ok | blocked | yes |
| `a_refused_placement_surfaces_exactly_one_notice` | ok | blocked | yes |
| `inspection_flag_rows_toggle_back_off` | ok | blocked | yes |
| `accept_suggestion_closes_menu_even_when_placement_fails` | ok | blocked | yes |

"blocked" is always the identical panic, never an assertion of the law's own:

```
thread '…' panicked at 🧪️tests/🔬️testkit/🦀️.rs:316:5:
puzzle3d app never quiesced: pending typed operations outlived the settle budget
```

The three settle-free laws were re-run at 18:52 and print their own `[DEBUG]` witness, e.g.

```
test …::an_outliner_flag_row_undoes_itself_on_the_second_click ...
[DEBUG] outliner flag round trip flag=hidden asked=true  state=true
[DEBUG] outliner flag round trip flag=hidden asked=false state=false
[DEBUG] outliner flag round trip flag=locked asked=true  state=true
[DEBUG] outliner flag round trip flag=locked asked=false state=false
ok
test result: ok. 1 passed; 0 failed; … 604 filtered out; finished in 0.00s
```

TypeScript, from
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`:

```
SEMIO_TEST_LEVEL=long bun x vitest run "🔬️engine-contract"
  → Test Files 1 passed (1) | Tests 452 passed (452)     Duration 12.26s

SEMIO_TEST_LEVEL=long bun x vitest run
  → Test Files 1 failed | 18 passed (19) | Tests 712 passed (712)
  → the one failed SUITE is 🧪️tests/🧩️package-integration:
       ReferenceError: self is not defined
         ❯ 🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🔌️plugin-bridge.ts:159
    a jsdom global in a peer's wgpu plugin bridge — collection-time, zero tests ran in it,
    nothing in the W-D4 surface. Not caused by, and not fixed by, this wave.
```

`bun x vitest run --reporter=basic` fails before collection (`Failed to load url basic`) on this vitest
— use the default reporter.

---

## Not verified

1. **Nothing here was observed in a browser.** No server was started and no wasm was rebuilt, per this
   wave's constraints. Every claim above is `cargo test` / `vitest` / direct source reading.

2. **The ShellHost `notify` branch has no test.** `applyHostEffects` is a `useCallback` closure inside
   the `ShellHost` component with no harness that can invoke it; adding one means mounting the shell,
   which is W-A's surface. What IS proven: the plugin emits exactly one `Effect::Notify` carrying
   localized prose (`a_refused_placement_surfaces_exactly_one_notice`), the `notify` arm of the kernel's
   `Effect` union predates this wave, and the engine-contract suite (452 tests, ShellHost in its import
   graph) is green. The four lines that turn that effect into a visible toast are read-verified only.

3. **`pick` and `rectangle` are indistinguishable at the host today.** The plugin's `SelectionMethod`
   domain (`🧰️framework/🔨️modules/🕹️interaction/🟦️.ts:62`) has three members, but `World3dHost`'s marquee
   only branches on `"lasso"` (`🌐️World3dHost/🟦️.tsx:3148`); `SelectionMarqueeMethod` itself is
   `"lasso" | "rectangle"`. So typing `rectangle` after `pick` changes the value the plugin publishes —
   which is what the law asserts — but not, today, what the drag draws. This is the framework's own
   contract, not a puzzle3d defect; flagged rather than papered over. Pre-existing behaviour is
   preserved exactly: the old hardcode was `"pick"` and the new default is `"pick"`.

4. **THE WHOLE `dispatch`-DRIVEN COHORT IS RED RIGHT NOW, FROM A PEER'S IN-FLIGHT WAVE.** Timeline,
   measured, same private target dir:

   - 18:02 build finishes; 18:03–18:05 the seven laws above all pass individually.
   - 18:09 `⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` and 18:10 `⏳️precompute/🦀️.rs` are rewritten by the mesh
     upload wave (`PagedBrushMeshUploads`: `stage_brush_mesh_page`, `brush_mesh_digest`,
     `adopt_shared_mesh(url, digest)`, `retire_abandoned_brush_mesh_uploads`). No other `.rs` under the
     puzzle3d tree changed after 17:34.
   - From the next rebuild on, **every** test that goes through the testkit's `dispatch` → `settle`
     stalls, including long-standing ones that predate W-D4 entirely:
     `duplicate_selection_reselects_the_created_clones`, `add_object_kind_honors_drop_origin`,
     `camera_actions_are_view_actions_that_emit_no_artifact_mutations`,
     `context_menu_at_selects_object_groups_flags_and_keeps_delete_last`,
     `close_vortex_suggestions_clears_the_menu`, … and the W-D4 laws with them.
   - The failure is always the same: `settle` panics with *"puzzle3d app never quiesced: pending typed
     operations outlived the settle budget"* (`🧪️tests/🔬️testkit/🦀️.rs:316`), and the coordinator's
     trace shows one operation frozen as `2:Worker:true:true` from turn 1 to turn 1 048 576 —
     `result_page_presented = true` with the page never taken or acknowledged. That is a receiver /
     result-page-retirement break, upstream of anything in this wave's files.
   - **This is not W-D4's.** The only edits this pass made after the green run were a docstring
     (`🎚️config/🦀️.rs:158`) and an added test — neither can stall an unrelated operation, and the
     failing set includes tests that never touch any W-D4 code. Re-run the five blocked laws once the
     precompute wave settles; they were green at 18:03–18:05 on a build of this exact source.
   - **What this pass did about it, rather than shipping an unrun test.** The outliner round trip was
     first written as a component-level law driving `dispatch("setSelectionFlag", …)`. It could not be
     run — its very first turn hit this stall — so it was **deleted**, not left in the tree with an
     unverified claim attached to it. It was replaced by
     `an_outliner_flag_row_undoes_itself_on_the_second_click`, which asserts the same user-visible loop
     (row asks X → apply → row now asks !X → apply → back) through render + the pure reducer, with no
     actor turn, and which is green. The end-to-end arg-parsing hop through `setSelectionFlag`'s JSON
     args stays covered by the pre-existing
     `context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden` — itself blocked
     by the same stall today.

   - **A candidate, offered NOT asserted — with its own counter-evidence.** The same diff drops the
     `resume_candidate_index > 0` guard from the brush re-queue in two places, `⏳️precompute/🦀️.rs:1677`
     (`if result.unknown_pending && !self.brush_queue.iter().any(…)`) and `:1896`
     (`let needs_resume = result.unknown_pending;`), deliberately, to cure a livelock where a target
     that could not clear its broad phase in one 500 µs slice was never revisited. Without the guard a
     target that stays `unknown_pending` and makes **no** progress re-queues itself every pass, so that
     lane always reports more work — the shape that starves a bounded maintenance budget. **But** the
     same diff's new `eprintln!("[DEBUG] free_until …")` (`⏳️precompute/🦀️.rs:1746`, `:1763`, `:1774`)
     never prints during a stall, so the brush lane is not visibly the one spinning. Treat this as a
     lead, not a diagnosis. Independent of the cause, those five unguarded hot-loop `eprintln!` at
     `⏳️precompute/🦀️.rs:1746`, `:1763`, `:1774`, `:1950`, `:1974` should not ship.

5. **The whole-crate number is not a W-D4 result.** A full
   `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 -- --test-threads=1`
   at 18:17 reported `516 passed; 85 failed` — every one of those 85 is the `settle` stall of item 4,
   on a tree that changed under the run. Do not read it as a W-D4 baseline; re-measure after the
   precompute wave settles.
