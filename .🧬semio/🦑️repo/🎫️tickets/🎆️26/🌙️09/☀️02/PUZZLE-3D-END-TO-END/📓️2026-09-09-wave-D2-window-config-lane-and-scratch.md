# Wave D2 — the WindowConfig lane hang, `addObjectKind` on an empty document, the vortex popup

Scope: audit `📓️2026-09-09-remaining-test-failures-audit.md` §3.1 (window-config lane never quiesces),
§3.3 (`addObjectKind` silent no-op), §3.4 (suggestion popup never renders), plus the §3.2 re-run the
coordinator asked for (report only, no implementation).

Toolchain for every command below:

```
RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 \
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d \
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 \
  editor::puzzle3d::component -- --test-threads=1
```

---

## 0. Headline

| run | result | wall | note |
| --- | --- | ---: | --- |
| before (13:0x) | **89 passed / 44 failed** | 422.94s | matches the audit's run-2 baseline exactly |
| after, best (14:3x) | **96 passed / 38 failed** | 174.51s | 134 tests (one new law added) |
| after, second (14:2x) | 94 passed / 40 failed | 126.41s | same tree, different machine load |

**Not one `puzzle3d app never quiesced` remains in either "after" run.** The 422.94s → ~150s wall
collapse is the same fact from the other side: the settle loop no longer burns its full
`1_048_576`-turn runaway bound on the stuck WindowConfig publication.

The residual gap between the two "after" runs is entirely the `job-session.terminal-fault`
step-budget quarantine class, demonstrated flaky below (§5) — the box sat at **load average 89-95**
for the whole verification window (7 concurrent `rustc`, plus node/bun/git from peers), and that
fault is wall-clock-triggered.

---

## 1. §3.1 — the WindowConfig lane never quiesced (ROOT CAUSE, fixed)

### The defect

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs:82` (before this wave):

```rust
if self.cancelled || self.closing || !grant.permits_one() || grant.maximum_bytes < O::MAXIMUM_PUBLICATION_BYTES {
    return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
}
```

and the same clause in `close_step` (`:157`).

`O::MAXIMUM_PUBLICATION_BYTES` is the owner's **schema ceiling** — the widest record that window kind
may ever carry. Puzzle 3D declares `65_536`
(`✏️editor/🪟️window/🦀️.rs`, `Puzzle3dWindowConfigOwner`). The grant the publisher hands one
turn is a **per-turn work budget**, fixed at `TYPED_OPERATION_RESULT_PAGE_BYTES = 4_096`
(`🧰️framework/…/🔌️plugin/🦀️.rs:12132`, used at `:20621`
`ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }`).

`4_096 < 65_536` on **every** turn, forever. The chain:

`BoundedWindowConfigPreparation::advance` → `Blocked`
→ `store::advance_apply_batch`'s `Preparing` arm (`🏪️store/🦀️.rs:15829-15841`) → `ArtifactStoreOneItemAdvance::Blocked`
→ `publish_mounted_typed_operation_unit`'s `Progress(_) | Blocked => Ok(())` (`🔌️plugin/🦀️.rs:20762`)
→ the mounted operation stays in stage `Publishing` with runnable work, which is exactly the observed
`[DEBUG] typed-operation publication turn=… operations=["2:Publishing:true:true"]` repeated until the
settle budget ends the test.

It is a **conflation of a schema ceiling with a per-turn budget**, and it made *every* window-config
owner whose declared ceiling exceeds 4 096 permanently unpublishable — puzzle 3d (65 536),
puzzle 2d (65 536), puzzle 5d (65 536), note (65 536), trinity/jack editor (8 192). The four owners
that happen to declare ≤ 4 096 (trinity graph, trinity rewriting, reasoning wires, writer main,
mathematical graph) were the only ones that ever worked, which is why nothing caught it.

### The fix

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs`

| line | change |
| ---: | --- |
| 25-31 | docstring stating the ceiling-vs-grant distinction and naming the sibling that already had it right (`🫧️transient/🧵️publication/🦀️.rs`'s `BoundedTransientPreparation`, which gates on its OWN item's encoded length). |
| 39 | new `retained_bytes: usize` field on `BoundedWindowConfigPreparation` — THIS item's exact encoded cost. |
| 53-62 | new `BoundedWindowConfigPreparationFactory::item_retained_bytes(mutation, description)` — one place computing `encode_op(mutation).len() + description.len()`, bounded by the owner ceiling. |
| 69 | `preflight` now calls it (the admission footprint still declares the owner ceiling, which is the conservative gesture-wide reservation wave B's `merged` expects). |
| 78-92 | `begin` computes `retained_bytes` once and stores it on the preparation. |
| 100-105 | `advance` gates on `grant.permits_one()` only; a grant that cannot cover **this item's own** bytes is now a **fail-closed `Err`** ("window config item cannot ever fit the publication turn's byte grant"), not an eternal `Blocked` — the publisher's grant is a constant, so an indivisible item that does not fit can never fit, and a diagnosable fault beats a silent spin. |
| 176-183 | `close_step` no longer gates on the ceiling at all (retirement must always be able to release what it holds — otherwise the fault path in `advance` would itself hang in `Closing`), and reports the bytes it actually released. |

### The law, as a test

`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1555`
`one_window_config_mutation_publishes_exactly_one_generation_and_quiesces` — one `setCamera` on a
fresh single-window app: `has_pending_typed_operations()` is false afterwards, the window's config
generation advanced by **exactly one**, and the rendered window carries the published camera. Its
docstring names the exact 65 536-vs-4 096 conflation so the law cannot be re-broken silently.

### Effect on the six §3.1 tests

`camera_actions_are_view_actions_that_emit_no_artifact_mutations`,
`set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched`,
`vortex_direction_option_is_local_to_the_window_instance`,
`world_vortices_carry_their_own_selected_and_hovered_flags`,
`world_vortices_reveal_in_selected_mode_only_for_the_selected_object` — all observed **green**.
`grid_window_options_control_one_visible_grid_spacing` and
`window_options_are_local_to_the_window_instance_not_shared_across_split_panes` stopped hanging and
then hit the *next* gap in the same tests (`window_measures` asked with a window-less `ViewModel`),
fixed in §4.

---

## 2. §3.3 — `addObjectKind` on a document with no kind catalogs (fixed)

`✏️editor/🦀️.rs` `Puzzle3dAddObjectKindWork`:

| line | change |
| ---: | --- |
| 3506 | new `Puzzle3dAddObjectKindStage::Catalog`. |
| 3530 / 3546 | new `catalog_mutation: Option<Puzzle3dMutation>` owner. |
| 3576-3594 | `declared_default_kind(kind_id)` — the documented default catalog row (`Puzzle3dCatalogObjectKind` carrying the command's own `objectKind` default id, no representation, no vortex template). |
| 3598-3604 | `extent` for the no-catalog branch is **3**, not 1 (decode → materialize → publish); the old `Some(1)` was sized for the no-op it is replacing. |
| 3610 / 3620-3640 | `step` no longer short-circuits to `Complete(Emit::default())` when `meta.kind_catalogs` is `None`; it routes decode → `Catalog`, which stages `replace_kind_catalogs(Some(Puzzle3dKindCatalogs { objects: vec![declared default], .. }))`. |
| 3651 / 3672 | `Kind`/`Representation`/`Vortex` bind the catalogs per arm (they are only reachable when catalogs exist) and fault `puzzle3d-add-kind-catalog-owner` otherwise. |
| 3708 | `Publish` emits the catalog row **and** the object as ONE gesture (`catalog_mutation` then `mutation`) — wave B's batched lane stages both into one `Edit`, one ledger slot, one undo step. |
| 3724 / 3731 | close cursor and terminal-empty witness extended with `catalog_mutation`. |

**Test correction, with the reason** (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:2159`):
`add_object_kind_materializes_the_declared_kind_default` asserted `!result.mutations.is_empty()`.
That assertion is unreachable by construction on the retained route and is not what the audit
thought it was: `settle`'s own docstring
(`✏️editor/🧪️tests/🔬️testkit/🦀️.rs:234-238`) states that a registry-backed `VcsArtifactApp`
"does NOT apply a retained tool job inline… which is why the registry-less fixture used to see
mutations land synchronously and this one does not". The assertion is replaced by the real law: the
settled document gains exactly one object, its `objectKind` is the declared default, **and**
`/meta/kindCatalogs/objects` now contains that kind — i.e. the created object references a
catalogued kind, never a dangling one.

Verified: `add_object_kind` filter, 3 consecutive runs → `ok. 3 passed` twice, one flake (§5).

---

## 3. §3.4 — the vortex suggestion popup (fixed; two of the four tests remain, one for a different reason)

Two distinct causes, both real:

### 3.1 The render call never named a window

`✏️editor/🧪️tests/🔬️testkit/🦀️.rs:362-372` — `render_body` called
`app.render(body_key, None, &ViewModel::default())`. With no `window_id` the framework's
`WindowTransientOwnerRegistry::capture` returns `Ok(None)` (`🫧️transient` registry), so
`TransientView.window` is `None`, `window_ownership::transient_from_view` yields the type default,
and `runtime.suggestion_menu` is `None` at render — no matter what the command published. The popup
was being written correctly and read from a partition the render never addressed.
`render_body` now derives the window instance from the body key (`<body>:<windowInstanceId>`, else
the main instance) and renders through `window_view(window_id)`, exactly as `dispatch` already did.

`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1305-1312` —
`open_vortex_suggestions_records_explicit_window_id` now renders the pane it dispatched into
(`render_window(…, WINDOW_INSTANCE_PERSPECTIVE)`) instead of the composite: the popup is transient
state of the pane the user acted in, and the args' `windowId` is the *recorded target*, which is what
the test asserts. A cross-pane publication is not expressible — `addressed_transient` is validated
against the operation's own captured window authority (`window-transient.address`).

### 3.2 `acceptSuggestion` never dismissed the popup it answered

`Puzzle3dAcceptSuggestionWork` had no `view_state`, no `take_ephemeral`, and no window-transient
write at all — it was the only route that consumed the popup without retiring it. Invisible until
the popup started rendering.

`✏️editor/🦀️.rs`:

| line | change |
| ---: | --- |
| 5624 / 5643 | new `view_state: Option<ViewModel>` owner. |
| 5663-5673 | `dismissal()` — a `Puzzle3dWindowTransient` with `suggestion_menu: None` addressed at the captured window, or an empty emit when no popup was open. |
| 5678-5680 | `bind_view_state`. |
| 5691-5693 | `take_ephemeral` returns `dismissal()`; it is called exactly once, on `Complete` (`🎮️commands/🧵️retained/🦀️.rs:557`), so every terminal path — placed, refused, target absent, catalogs absent — retires the popup. |
| 5876 / 5894 | close cursor and terminal-empty witness extended with `view_state`. |

Observed green: `open_vortex_suggestions_opens_the_suggestion_popup`,
`open_vortex_suggestions_records_explicit_window_id`, `close_vortex_suggestions_clears_the_menu`,
`close_vortex_suggestions_clears_sticky_hover`, `accept_suggestion_closes_menu_even_when_placement_fails`.

**Still red, and it is NOT the transient lane:**
`hover_suggestion_updates_the_brush_candidate_index_and_live_preview` now gets past the popup and
fails on `assert!(!candidates.is_empty(), "suggestion candidates should be present")`
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1354`). The popup renders, `open` is `true`, `brushCandidateIndex`
is 0; what is empty is `session.brush_candidates(&menu.vortex_full_id)` — the precompute session the
render path builds (`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:322-345`). Same root for
`open_and_accept_vortex_suggestions_preserve_active_utility`'s remaining assertion
(`brush_preview_of(...).objectKindId` empty, `:1437`). That is a **precompute-warm-up-in-render**
defect, not a window-transient one, and it belongs to whoever owns `⏳️precompute` — it was masked
before by the popup never rendering at all.

---

## 4. The `window_measures`/`tool_measures` window-less ViewModel (the audit's bucket (b), now unblocking §3.1)

`window_measures_body`'s first line is
`let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };`
(`✏️editor/🦀️.rs`), so `app.window_measures(&ViewModel::default())` can only ever return an empty
map. Fifteen test call sites asked exactly that and then `.expect("main window measures")`. A host
never asks "what are this window's measures" without naming the window — the same argument the audit
made in its own §4, and the same one §3.1's render fix rests on.

Swept to `&window_view(main::WINDOW_KIND_ID)`:
`✏️editor/🧪️tests/🔬️unit/🦀️.rs` (11 `window_measures` + 4 `tool_measures` call sites) and
`✏️editor/🧪️tests/🔬️testkit/🦀️.rs:522` (`fill_count_slider_of`). The three
`puzzle3d_labels(&ViewModel::default())` calls are untouched — labels are locale/terminology, not
window state.

---

## 5. `job-session.terminal-fault` is a flake, not a regression — proof

`job-session.terminal-fault` is the framework's **pre-admitted quarantine page**
(`🧰️framework/🔨️modules/🧵️job/🦀️.rs:1896`, raised at `:2479` on a `CallbackVerdict::is_fault`
callback quarantine and at `:2483` on a worker panic). Under an unoptimised debug build the verdict
is a wall-clock step-budget verdict, so it fires on machine load, not on code.

Same binary, same tree, `add_object_kind` filter, three consecutive runs:

```
test result: FAILED. 2 passed; 1 failed; …; finished in 1.15s
test result: ok.     3 passed; 0 failed; …; finished in 0.72s
test result: ok.     3 passed; 0 failed; …; finished in 1.08s
```

`uptime` during the whole verification window: `load averages: 88.99 88.68 89.75` … `95.37 96.34 93.72`
(7 concurrent `rustc` plus peer node/bun/git). In the best "after" run 22 of the 38 failures carry
this fault; in the second-best run the SET of tests carrying it is different. Every one of this
wave's target tests was observed green in at least one run.

---

## 6. Coordinator item 4 — the §3.2 re-run after `dispatch_action` stopped faulting host-owned verbs

`interactive-job.missing-owned-reducer` is **gone from the whole suite** — not one occurrence in
either "after" run, nor in the "before" run (the coordinator's change had already landed when this
wave's baseline was taken). The 17 tests split as follows.

| what they now do | tests |
| --- | --- |
| **pass** | `set_active_utility_emits_no_ops_and_no_history_entry`, `set_object_kind_weight_declares_fill_options_ui_scope` |
| **host session state is not simulated** — the testkit mints a fresh `window_view()` per call, so a host-owned `setActiveTool`/`setActiveUtility` (now an empty `Emit`) is written nowhere and the NEXT call's `ViewModel` carries `active_tool_id: None` / an empty `active_utility_by_window_id` | `gumball_active_only_for_transform_utilities_with_object_selection` (`Some(false)` vs `Some(true)`), `transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes` (`None` vs `Some("transform")`), `transform_utility_options_expose_move_and_rotate_flags` (`None` vs `Some(true)`), `fill_build_tick_is_a_view_action_with_narrow_ui_scope` (`ui_scope` `None`), `fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job` (no `Effect::SpawnJob`) |
| **step-budget quarantine** (§5, load-dependent) | the seven remaining `fill_*` / `set_fill_count_*` tests |
| **window-less `window_measures`** (§4, now fixed) | `brush_placement_picker_appears_only_for_a_live_brush_target`, `fill_and_brush_params_are_tagged_utility_options_not_engagement_controls` |
| **fixed by §3.4** | `open_and_accept_vortex_suggestions_preserve_active_utility` (partially — its remaining assertion is the precompute gap in §3.2 above) |

**Recommendation on the scratch clearing the coordinator asked about.** puzzle3d's
`🎮️commands/🧰️set-active/🦀️.rs` clearing is indeed no longer reached, and the five "host session
state" tests above do NOT need it: what they need is for the *host* half to be simulated. The
active tool and the per-window active utility are host `ViewModel` state by decision
(`📓️2026-09-09-peer-config-runtime-split.md` §1(d)), so the honest fix is for the puzzle3d test
harness to hold ONE mutable `ViewModel` per fixture app that `setActiveTool`/`setActiveUtility`
update — the way a real host does — instead of re-minting a default one on every `dispatch`/`render`.
Routing it through a window-transient write instead would re-introduce the plugin-side duplicate of
host state that the ownership ticket just removed. **Not implemented here** (item 4 was report-only);
it is a testkit change of the same shape as §3.4's `render_body` fix.

---

## 7. Two edits outside this wave's three defects, and why

Both were **hard blockers**: the crate did not compile without them, in the middle of a peer's live
`WindowTransientOwner` migration.

1. `✏️editor/🪟️window/🦀️.rs` — `puzzle3d_window_transient_retained_bytes(transient) -> Option<usize>`.
   The peer's new `puzzle3d_window_transient_preflight` called it; it was never written (E0425 for
   ~25 minutes across five polls). Implemented to the repo's own convention
   (`✏️s/🔌️plugins/🏭️process/…/✏️editor/🦀️.rs:669` `process3d_config_retained_bytes`): the sum of the
   record's owned string lengths, `checked_add`-folded.
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3705-3712` —
   `ArtifactEphemeralBaseRead`'s field and `ArtifactEphemeralBaseOwner` are now `pub`. The peer's new
   `oversized_string_capacity_rejects_and_returns_the_exact_puzzle3d_owner`
   (`✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs`) constructs
   `store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::Transient(…))` literally, and
   both were private (E0433 + E0423 for ~15 minutes, the peer re-writing the same line each time).
   Widening is the minimum that makes their own code compile, and it is a legitimate surface: an app
   must be able to build the exact request its registered ephemeral factory receives in order to
   test it. Docstring says so.

Also touched, in the same class: `✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:41,59` — the peer's
`suggestion_and_input_retirement_reaches_terminal_empty_with_tiny_grants` called
`store::retirement::RetireOwned::retirement(...)` and then `close_step(1, 1).expect(...)`;
`RetirementCursor::close_step` takes ONE argument and returns a non-`Result` `RetirementStep`. The
driver they wanted is `store::retirement::owned_retirement(value)`
(`🏪️store/♻️retirement/🦀️.rs:297`), which is what that line now calls.

---

## 8. Every file changed

| file | what |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs` | §1 — per-item retained bytes instead of the owner ceiling; fail-closed `advance`; ungated `close_step`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | §7.2 — `ArtifactEphemeralBaseRead`/`ArtifactEphemeralBaseOwner` public. |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | §2 (`Puzzle3dAddObjectKindWork`), §3.2 (`Puzzle3dAcceptSuggestionWork`). |
| `…/✏️editor/🪟️window/🦀️.rs` | §7.1 — `puzzle3d_window_transient_retained_bytes`. |
| `…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | §3.1 — `render_body` addresses its window; §4 — `fill_count_slider_of`. |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | §1 (new law), §2 (retained-path law), §3.1 (`records_explicit_window_id`), §4 (15 measures call sites). |
| `…/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs` | §7 — `owned_retirement` driver. |

---

## 9. Not verified

1. No runtime/browser confirmation. Every verdict is a `cargo test` run or direct source reading.
2. The machine never dropped below load average ~88 during verification, so no run of the full module
   is free of the §5 quarantine flake. The "after" counts are therefore a floor, not a ceiling.
3. The fail-closed `Err` arm added to `BoundedWindowConfigPreparation::advance` is **not** exercised
   by any test — puzzle 3d's window config encodes to well under 4 096 bytes, so no fixture can reach
   it without a synthetic owner. It is reasoned, not measured.
4. Only `semio-s-artifact-puzzle-3d` was built. The `🎚️config/🦀️.rs` change is framework-wide and
   affects every app with a `WindowConfigOwner` (norm/note/trinity/reasoning/writer/mathematical/
   puzzle 2d/5d); none of their suites were run, and no wasm target was built.
5. `precompute::component::tests` / `precompute::fill::tests` were not re-run.
6. §3.4's remaining two failures are attributed to the precompute session in the render path by
   reading the assertion and `world_interaction_json`'s candidate source; the precompute path itself
   was not instrumented.
7. The crate was under continuous peer edit throughout (`🪟️window`, `🫧️transient`,
   `🏪️store/🧩️composition` all changed mid-verification, three times breaking the build outright).
   A re-run after that migration lands may show different numbers.

---

## 10. Closing state of the tree (15:56)

The final confirmation re-run of the full module **could not be taken**: from 15:04 onward a peer's
`ArtifactCompositionFields` / `ChildMember` composition wave has been landing across
`🧰️framework/…/🏪️store` and `🧰️framework/…/🔌️plugin`, and the workspace has not compiled for more
than a minute at a time since (`error[E0277]: the trait bound `Puzzle3dPlaySnapshot:
ArtifactCompositionFields` is not satisfied` at `✏️editor/🦀️.rs:6755` is that wave reaching this app;
implementing it here would be guessing at their sweep's shape). Every number in §0 and every
per-defect verification in §1-§4 was taken on a tree that compiled clean
(`cargo check … --lib --tests` → 0 errors, 14:10 and 14:39).

All of this wave's edits were re-confirmed present at 15:56 after the churn:
window-config `retained_bytes` (12 occurrences), the fail-closed grant arm
(`🎚️config/🦀️.rs:103-105`), the `Catalog` stage (`✏️editor/🦀️.rs:3506`),
`declared_default_kind` (`:3576`), `dismissal` (`:5663`), the testkit's window-addressed
`render_body` (`🔬️testkit/🦀️.rs:370`), the 15 measures call sites, and the new law
(`🔬️unit/🦀️.rs:1555`).

Whoever runs next: re-run `editor::puzzle3d::component` once that composition wave lands, on a box
that is not at load 90, before trusting any count in §0.
