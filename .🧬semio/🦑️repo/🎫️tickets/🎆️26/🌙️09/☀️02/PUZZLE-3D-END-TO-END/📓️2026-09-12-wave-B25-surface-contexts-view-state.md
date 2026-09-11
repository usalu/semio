# Wave B25 — one `view_state` for every surface: the hop B23 named, now under law and fixed

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B25, 2026-09-12. Brief: build the missing testkit shape
for `plugin_mount_surface → SurfaceContexts → plugin_render_surface` INSIDE `semio-framework-plugin`,
quote its RED, then make `SurfaceContexts` keep one view state PER mounted surface.

No git write, ticket not opened/closed/reopened, `🗑️generated` written to and never deleted, **no wasm
build run**, every command foreground, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, no process killed. The one
temporary `[DEBUG] b25 presence …` tap was added, used and **removed** (`rg "DEBUG\] b25"` over the repo
outside `🗑️generated` → no match).

---

## 1 The route, with file:line

```
Event::SurfaceVisible                ⚛️reactor/🔄️turn/🦀️.rs:529-532
  → plugin_mount_surface             🔌️plugin/🦀️.rs:32344      (pub(crate))
      → SurfaceContexts::insert      ⚛️reactor/🪟️surfaces/🦀️.rs:32  (before this wave)
  → plugin_render_surface            🔌️plugin/🦀️.rs:32381
      → SurfaceContexts::get         ⚛️reactor/🪟️surfaces/🦀️.rs:52  (before this wave)
      → PluginApp::render(&context.body_key, None, &context.view_state)   🔌️plugin/🦀️.rs:32390
```

`SurfaceContexts` held **one** `view_state: Option<ViewModel>` plus one `section_view`, and every
`insert` overwrote it (`…/🪟️surfaces/🦀️.rs:48` before this wave). `get` then REBUILT a body's
`ViewModel` out of that single field — `view.for_window_instance(window)` for a window binding,
`view.for_panel()` for a panel. So the projection a body was rendered against came from **whichever
sibling surface mounted last**, not from the context the shell actually sent for it.

The shell has always sent a distinct context per surface, in this order
(`uiRefreshSurfaceEvents`, `🔌️PluginRuntime/🟦️.tsx:1611`): every window through
`windowViewContext`, then the leftover `1:window` alias bound to the LAST window
(`windowHostContextBindings`, `:1517`, `DEFAULT_LEFTOVER_WINDOW_SURFACE`, `:1504`), then every panel
through `panelViewContext`, then every reserved section through `sectionViewContext`
(`🛂️manifest/🟦️.ts:909/920/1235`). `panelViewContext` deliberately KEEPS `focusedWindowId` (B12 §5.1,
B15); `windowViewContext` stamps `windowId`/`activeWindowKindId`/`activeUtilityId`. Everything else —
`focusedWindowId`, `activeToolId`, `panelJson`, `contributionsJson`, `locale`, `terminology`,
`activeUtilityByWindowId`, `windowInstances` — was taken from the last mount, and surfaces are mounted
across refresh generations, not only within one. `plugin_handle_action` (`🔌️plugin/🦀️.rs:31969`) fed
the same single field from an ACTION's own (window-narrowed) view, so a pane's pick also re-pointed the
Inspection panel's projection.

Nothing else in the system conflated them. Every testkit render helper (`render_body`,
`render_panel_body`, `render_window_refresh`, `render_window`, `render_composite`) calls
`PluginApp::render` with a hand-built `ViewModel`, which is exactly why B23 §3 could name this hop but
no law could reach it.

---

## 2 The law that had never existed, and its RED

New, in its own tests tree beside `🔬️plugin-runtime-plugin-builder-contract`:

| file | what |
| --- | --- |
| `🔌️plugin/🧪️tests/🔬️surface-view-state-routing/🦀️.rs` | `every_mounted_surface_renders_against_its_own_view_state_while_one_pick_reaches_every_body` |
| `🔌️plugin/⚛️reactor/🪟️surfaces/🧫️fixtures/🪟️surface-view-state-routing/🔣️.json` | its language-neutral fixture: the four surfaces a split puzzle3d session mounts |

It mounts the browser's own shapes through `plugin_mount_surface` — `1:puzzle3d-main-perspective` and
`1:puzzle3d-main-top` (two instances of ONE window kind), the Inspection panel body
(`1:inspection`, no `windowId`), and the leftover `1:window` alias bound to the last pane — each with
the host view of the refresh generation that mounted it (differing `focusedWindowId`, `activeToolId`,
`panelJson`, `locale`, `terminology`). It dispatches one browser-shaped `interactionSelect`
(`domainId`/`targets`/`merge: replace`/`method: pick`) through `handle_action` + the reserved settle,
then drives `plugin_render_surface` for the Inspection body and for both panes' world lane, asserting
per surface that (a) the `ViewModel` the body was rendered against is byte-identical to that surface's
OWN projection, and (b) the pick reached that render (`PresenceUpdate { own.selected }` addressed at
that surface). Then it re-renders the picked pane and the Inspection panel to prove no later mount
clobbered an earlier context, asserts the alias resolves to the window it was bound to, and finally
pushes a fresh host view through `plugin_render` (→ `SurfaceContexts::update_view`) and re-renders
every surface to prove a host refresh rebinds each one through its own role.

**Deliberately NOT `interaction_registry`'s `HierarchyProvider::Topology` twin.** That fixture needs a
seeded document label, which today dies on a PEER's red
(`dispatch_typed(SetLabel)` → `interactive-job.missing-factory`, B23 §6). This law declares its one
domain `HierarchyProvider::Flat`, which `build_full_interaction_topology` (`🔌️plugin/🦀️.rs:22181`)
omits from the pruning map, so the pick survives `validate_state` without any typed command — the law
measures the surface route, not a factory proof.

**RED, against the exact code the fix covers** (`SurfaceContexts::get` restored to the shared-field
rebuild, everything else already in place):

```
thread '…::every_mounted_surface_renders_against_its_own_view_state_while_one_pick_reaches_every_body'
panicked at …/🔬️surface-view-state-routing/🦀️.rs:55:5:
assertion `left == right` failed: surface 1:inspection (body properties) was rendered against another
surface's view state
  left: Object {…, "focusedWindowId": String("puzzle3d-main-top"), "locale": String("de"),
                "panelJson": String("{\"world\":\"top\"}"), "terminology": String("reuse"), …}
 right: Object {…, "focusedWindowId": String("puzzle3d-main-perspective"), "locale": String("en"),
                "panelJson": String("{\"inspection\":true}"), "terminology": String("native"), …}
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 664 filtered out
```

The Inspection panel was rendered against the `1:window` alias's projection of the TOP pane: the wrong
focused pane (B15's field), the wrong locale, the wrong terminology and the wrong `panelJson`.
Full capture: `🗑️generated/b25-red-surface-view-state.txt`.

---

## 3 The fix

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs` — rewritten around ONE
idea: a binding owns its projection.

- **`SurfaceRole { Window(String), Panel, Section }`** — what a surface IS, fixed at mount. `project`
  is the one place a host view becomes a surface's view: `for_window_instance(id)` / `for_panel()` /
  the full unnarrowed clone for a reserved section.
- **`SurfaceBinding.view_state: ViewModel`** — each mounted surface carries its own projected view.
  `insert` projects ONCE at mount and stores it; `get` returns it verbatim, so a later mount of any
  other surface cannot change what an earlier one renders against. The `1:window` alias is just
  another binding with `SurfaceRole::Window(last)`, so it resolves to the window it was bound to
  without touching that window's own context.
- **`update_view` replays** one fresh host view onto every live binding THROUGH that binding's own
  role, instead of replacing a shared field — so a locale switch, a rearmed tool or an action's view
  still reaches every surface (the `surface-context-lifecycle` law's own statement), and no surface
  borrows a sibling's window identity while doing it. Reserved sections stay untouched, which is what
  `reserved_section_surfaces_keep_the_unnarrowed_view_and_outlive_their_windows` requires.
- **`section_view` is gone.** It existed only to shield sections from the shared field every mount
  overwrote; per-surface state makes it redundant. Its law now states the same property directly
  through a `#[cfg(test)] is_section` accessor instead of reaching into the removed field.
- `prune_windows` and the capacity/slot-reuse policy are unchanged (they read `role.window_id()`).

**A fault class disappears with it.** Old `get` returned `None` — and `plugin_render_surface` then
raised `surface {surface} has no host context`, replacing the body with a fault card — whenever the
LAST-mounted view's `windowInstances` did not carry an earlier pane's instance. A window surface's
context no longer depends on any other surface's roster.

**One hygiene fix in the law itself, not the product.** The law's reserved pick parks a worker-session
retirement PROCESS-globally (maintenance stage 23); left behind it polluted the heap
`a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` measures (`per_turn=82 B` against a
64 B ceiling → assert → panic-in-destructor → SIGABRT for the whole binary). The law now drains the
parked retirements the way `close_fixture_app_to_terminal_emptiness` (`🔌️plugin/🦀️.rs:17719`)
prescribes. Measured before/after in §4.

---

## 4 Verification — every command foreground, tails quoted

| command | result |
| --- | --- |
| `RUST_MIN_STACK=… cargo test -p semio-framework-plugin --lib -- --test-threads=1 every_mounted_surface_renders_against_its_own_view_state` | `test … ok` / `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 664 filtered out; finished in 0.03s` |
| `… --test-threads=1 every_mounted_surface_renders_against_its_own_view_state surface_context reserved_undo reserved_section typed_operation_slot release_their_exact_slot typed_operation_ingress_pre_admits never_parks_a_turn status_only_host_call a_nakagin_scale_mixed_surface_turn microsecond_registered_factory_dispatch retained_latest_wins_registered_dispatch retained_operation_continues_after_command_admission` (B8/B16/B21 law names) | `test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 637 filtered out; finished in 1.45s` — includes all four `surface_context::tests` lifecycle laws, `default_window_surface_has_host_context` on `7:window`, `reserved_section_surfaces_*`, `surface_context_reaches_real_app_render_and_rejects_hidden_surfaces`, `surface_context_presence_targets_each_concrete_surface`, `surface_context_refresh_projects_panels_from_focused_window_state` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: semio-s-artifact-puzzle-3d (lib) generated 88 warnings` / `Finished dev profile … in 2.48s` — **0 errors**, same 88 warnings B23 recorded (the warnings are the proof expansion ran) |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Checking semio-s-plugin-puzzle v0.1.0` / `Finished dev profile … in 22.69s` — **0 errors** |
| `RUST_MIN_STACK=… cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 selection leftover inspection lock a_browser_shaped_pick` | `test result: FAILED. 64 passed; 1 failed; 652 filtered out` — the ONE red is a PEER's, see below |
| `RUST_MIN_STACK=… cargo test -p semio-s-artifact-puzzle-3d … a_browser_shaped_pick` | `test editor::puzzle3d::component::tests::a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches ... ok` — B23's law still green |

### The whole `semio-framework-plugin --lib` suite, failure NAMES diffed against a baseline

| run | totals |
| --- | --- |
| with this wave's law and fix | `test result: FAILED. 520 passed; 145 failed; 0 ignored; 0 measured; 0 filtered out; finished in 34.56s` |
| same tree, `--skip every_mounted_surface_renders_against_its_own_view_state` (baseline) | `test result: FAILED. 518 passed; 146 failed; 0 ignored; 0 measured; 1 filtered out; finished in 57.95s` |

Name-by-name (`comm` over the sorted `failures:` blocks, `🗑️generated/b25-fail-{before,after}.txt`):

- **new reds: NONE.**
- one baseline red now passes:
  `retained_operation_continues_after_command_admission_until_publication_and_retirement` (B21's law) —
  it is order-dependent on the same process-global parked retirements §3 describes, and this wave's
  drain clears them earlier in the run.

The 145 remaining reds are the peer's typed-publication wave (B23 §6 measured the same family:
every one fails at its first `dispatch_typed` with
`interactive-job.missing-factory: typed command '…' has no exact controller/owner/factory/tool/schema
proof`). Not this wave's, not reverted.

### The one puzzle3d red is the peer rewriting the editor

`gumball_active_only_for_transform_utilities_with_object_selection` fails at
`✏️editor/🧪️tests/🔬️unit/🦀️.rs:3997` — `an unattached gumball must never render: left Some(true), right
Some(false)`. Both `✏️editor/🦀️.rs` and `✏️editor/🧪️tests/🔬️unit/🦀️.rs` carry mtime **23:37:00**, two
minutes before that run: the editor peer is live in those files. This wave touched neither, and nothing
in the puzzle3d testkit reaches `SurfaceContexts` (its helpers call `PluginApp::render`/`handle_action`
directly, never `plugin_mount_surface`), so it cannot be reached from here. Flagged, not repaired, not
reverted.

### The SIGABRT, before and after the law's drain

```
before: [DEBUG] settled reactor turn retention: settled=86648252 after=86669364 per_turn=82 B
        a settled reactor turn retains 82 B — 21112 B over 256 turns  → panic in a destructor
        during cleanup → thread caused non-unwinding panic. aborting.  (signal: 6, SIGABRT)
after:  test …::a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford ... ok
```

---

## 5 A hypothesis killed on the way

**`interaction_ui_topology` is NOT puzzle3d's problem.** That map (`🔌️plugin/🦀️.rs:19122`) is the other
"one shared thing every surface overwrites" in this file — it is keyed by DOMAIN only, and
`stamp_and_cache_interaction_ui` (`:22496`) rewrites a domain's entry from whichever body rendered last,
so a pick could in principle be pruned against a sibling body's node set. It cannot happen here:
puzzle3d declares `hierarchy: HierarchyProvider::Topology` (`✏️editor/🦀️.rs`,
`puzzle3d_interaction_definition`), so `resolve_domain_topology` (`:22156`) serves it from
`ArtifactApp::interaction_topology` and never reads the UiTree cache. Only the catalogue
(`📌️panels/🛍️catalogue/🦀️.rs:147`) and the outliner (`📌️panels/🗿️artifact/🦀️.rs:320`) bind the vortex
domain into a `Tree` at all, and the Inspection body binds none.

---

## 6 What this should flip on `#50`, stated by mechanism

This wave repairs **which `ViewModel` each body is rendered against**. It does NOT touch interaction
state, so a verdict that reds purely because `interaction.selected` is empty at render time is not
addressed by it. Ranked honestly:

| verdict | expected on `#50` | mechanism |
| --- | --- | --- |
| `gumball-scene-delta`, `brush-preview-place`, `volume-brush-add-target-volume` | **flip likely** | these read `activeToolId`/`activeUtilityId` through `puzzle3d_fill_tool_active` and `puzzle3d_scene_active_utility` (`✏️editor/🦀️.rs:917`, `:995`). A pane rendered against a sibling's context carried the sibling's armed tool — `fill` wins over the pane's own utility by declaration — so the gumball/brush lanes were computed for the wrong arming. Each pane now renders against its own. |
| `relocate-pose-delta`, `delete-selection`, `outliner-hide-applies` | **flip possible** | all resolve their target window through `puzzle3d_addressed_window_id` (`:967`), whose PANEL hop is `ViewModel::focused_window_id` (B15). The Inspection/outliner panels were carrying the LAST-mounted pane's `focusedWindowId`, which is the exact field B15 added to stop a panel retuning the roster's first entry. |
| `inspection-object-fields`, `inspection-locked-flag-row`, `locked-flag-row`, `locked-refusal-notice` | **flip only if the panel's own `panelJson`/focused pane was the cause** | the Inspection body's paged ids come from `panel_pages` off `panelJson` (`window_ownership::config_from_view`), and the panel was rendering against a WINDOW surface's `panelJson`. But `.empty` requires `interaction.selected` empty at that render, and that is app state, not view state — B23 §2.2 measured the guest rendering one body WITH the live interaction (vortex lane 2 → 2 640 B) while Inspection published the empty summary. If `#50` still reds these, §7's read is the next step, not another view-state wave. |

## 7 Handover

1. `#50` must carry BOTH `🔌️plugin/🦀️.rs` (B23's `interaction_selection_loss_v1` report, still guest
   Rust) and this wave's `⚛️reactor/🪟️surfaces/🦀️.rs`. Both are guest-side; neither is host-live.
2. On the next `--only=selection-surfaces` run, B23 §8's two-attribute read
   (`data-guest-selection-json` vs `data-selection-json`) now has a THIRD discriminator available for
   free: if `interaction selection lost reason=…` appears at all, the pick is dying in
   `revalidate_and_persist_interaction_state`, which this wave does not touch; if it never appears and
   the Inspection body is still `.empty` while the world lane paints the marker, the remaining
   suspect is the per-body render input, and the first thing to print is `view_state.panelJson` /
   `focusedWindowId` on the Inspection render (now, for the first time, guaranteed to be the panel's
   own).
3. The peer's 145 `interactive-job.missing-factory` reds keep
   `interaction_select_replace_persists_through_the_interaction_store` and every other
   `interaction_registry` law dark. Any wave needing a seeded document label in
   `semio-framework-plugin` is blocked behind that fixture until the peer lands.
