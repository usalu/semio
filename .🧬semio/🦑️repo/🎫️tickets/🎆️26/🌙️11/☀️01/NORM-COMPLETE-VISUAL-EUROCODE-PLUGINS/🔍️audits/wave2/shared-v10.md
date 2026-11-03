# Shared v10 (wave 2)

Nothing compiled (no cargo, no rustc). "- ready for rebuild: shared v10" is in `🗑️generated/integrate/latest.md` and `build-en1993/latest.md`. All paths below are under `✏️s/🔌️plugins/📕️norm/`.

## 1. Token-valued choice handle
- Schema (`🎬️structural-scene/🧬️schema/🕹️handles`, rs + ts + proto + graphql twins): `ChoiceToken { token, label: LocalizedCopy }`, `Quantise::Token { tokens }` (the affine value is the index, rounded and clamped), `HandleKind::ChoiceStepper`, `ResolvedWrite.token`, `FieldBinding::{token, token_at}`, `Quantise::token_at`, validation (non-empty, unique tokens). Tests in the crate unit tests plus a feature line.
- Writes: `writes_for` (handle drag tool) and `DragPreview::at` emit `DslValue::String(token)` for a token binding. The dimension label shows the token label (EN/DE) instead of a number. Keyboard: `nudge` on a ChoiceStepper moves exactly one token and cycles past either end (the old ordinal grips nudged by span/100, so the keyboard barely worked); pointer/touch drag snaps with increment 1. Test `a_choice_stepper_cycles_by_keyboard_and_commits_the_enum_token`.
- Kit: `handle_kit::choice_handle(entity, tag, anchor, path, &[(token, en, de)], name)`.
- en1993 migrated (the only artifact with ordinal grips; EN 1998 MemberKind and EN 1995 have no enum grips yet, they use the kit helper): stiffener-shape and warping-support grips, `STIFFENER_SHAPE_UNIT`/`WARPING_SUPPORT_UNIT` deleted, ordinal branch of `resolve` in the edit rules deleted, ALL docs fixed, edit-rule test now token only, scene test checks the four tokens and `token_at(2.4) == FixedFork`.

## 2. Touch-menu Select all
`render_touch_menu(entries, controller, locale, domain)`. The Select all entry dispatches the framework verb `interactionSelect` with `{domainId, targets, merge: replace, method: pick}` over `select_all_targets(domain)` (what the chord dispatches). Limit: a UI text argument is 512 bytes, so a scene whose targets JSON exceeds that (about a dozen entities) falls back to the `norm.shortcut.select-all` action (a no-op on touch). Proper fix: norm implements `interaction_topology` so the framework `selectAll` verb needs no payload. Test `the_touch_menu_select_all_dispatches_interaction_select_with_the_chord_targets`.

## 3. Tree drag and drop
Router-level synthetic pointer test (`overlay-drag-drop-and-scroll-laws`): press on the third row, move past the threshold, hover marker (`drop_target`) on the first row, release gives exactly one `DropCommitted{source,target,payload}`. The remaining hops (`finish_tree_drag` -> `apply_tree_row_drop` -> `moveEntity` -> `MoveItem`) are covered by the v9 reconcile test and the entity-editing move tests. A test of the real wgpu shell tree (`finish_tree_drag` on a live shell) needs the shell fixtures and a build: not written.

## 4. Owner-decision lines
Nine lines annotated "shared v10" in requests.md. None is shared-scope and uncontroversial today: 500/967/1527 are field-adder/SCHEMA-CODEGEN, 585 waits for ndp xi rows, 770 and 791 need a green build, 938 is the repo guard (infra), 1019 touches every remedy constructor (do right after the first green build), 1298 needs verified Table A2.4. All stay flagged.

## 5. Check <-> entity link (coordinator extra)
New `🔗️check-entity-link` in app-surface (included by `🧾️panels`), pure and ephemeral:
- `check_owner`, `focus_of_check` (selection targets of the owner entity, camera by `fit_selection` over all entities with that base id, global check -> `CheckOwner::Global`, no camera change, global badge target when the scene has it), `focus_outcome` (interactionSelect-shaped `ShellKeyOutcome::Select`).
- `checks_of_owner` (rows of a selected owner through `plan_checks`, so status/query filters still apply; scroll target is the governing row), `owner_of_scene_id`, `entity_selected_event` -> `LaneEvent::ViewCheck`.
- 5 tests (owners by base id, check -> select+frame, global, entity -> narrowed rows and scroll, both directions agree).
Not wired: the live hooks (check row click -> `focus_outcome` + camera write, and the viewport's selection change -> `ViewCheck`) need the panels/shell event plumbing and a build; the pure core is ready to call.

## Risks (read, not compiled)
`Quantise::apply` clamps with `usize as f64`; `variants!` now holds `Vec<ChoiceToken>`; `touch-menu` uses `serde_json`, `ui_text`, `action_args_map`, `select_all_targets` via `crate::*`; `check_entity_link` assumes `base_id`/`GLOBAL_BADGE` are re-exported by the scene crate (they are `pub use` of projection-kit) and `Annotation::id()`; golden/hash twins of the handles schema (proto/graphql snapshot checks) may need re-bless.
