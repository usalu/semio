# W7 — OS 3D-World Surface: `worldSelect`/`worldHover` → Framework Interaction Verbs

## The gap (from `📓️residue-sweep.md` §1b)

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs` (W3c) had already been converted to
emit the framework's `interactionSelect`/`interactionHover` verbs for the `world` domain. Its sibling
`♾️infinite/🌍️world/🦀️component.rs` — mounted separately into the same crate root
(`📦️packages/🦀️rust/📦️glue.rs`: `#[path="../../🦀️component.rs"] mod component;` **and**
`#[path="../../🌍️world/🦀️component.rs"] pub mod world;`, both `pub use ...::*`) — was left on the
byte-identical **pre-migration** copy, still dispatching the legacy `"worldSelect"`/`"worldHover"`
action strings at the five sites named in the ticket brief.

## What changed

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` — brought to parity with the
already-migrated `♾️infinite/🦀️component.rs`, using the exact same diff (confirmed via `diff` before/
after: the two files are byte-identical again except for the new test region's position within
`mod tests`):

- **Imports**: added `LocalizedLabel`, `GranularityDefinition`, `HierarchyProvider`, `HoverSpec`,
  `InteractionDefinition`, `MergeMode`, `SelectionMethod`, `SelectionMode`, `SelectionSpec`.
- **New `//#region 🔖️WorldInteractionDomain`**: `WORLD_INTERACTION_DOMAIN_ID = "world"`,
  `world_interaction_definition()` (granularities `surface`/`item`,
  `HierarchyProvider::PathDelimited{delimiter:"/"}`, `SelectionMethod::{Pick,Rectangle}`,
  `MergeMode::{Replace,Additive,Invertive}`), `world_item_target_id`/`world_item_id_for_surface`
  (join/split `surfaceId/id`), `merge_mode_wire_str`/`selection_method_wire_str`.
- **`apply_world_action_preview`**: deleted the `"worldSelect"`/`"worldHover"` arms; added
  `"interactionSelect"`/`"interactionHover"` arms gated on `args["domainId"] == "world"`, parsing the
  `targets` array and stripping this surface's own `surfaceId/` prefix back off each item id. Still
  optimistic-local-preview only — the os-kernel `next_selection`/`next_hover` machine is the source of
  truth once the round-trip settles (unchanged division of responsibility).
- **`pick_hover_action`** (line ~2882 pre-edit): now emits `interactionHover` with
  `{domainId, channel:"pointer", targets}`; empty `targets` is the clear signal.
- **`pick_select_action`** (line ~2937 pre-edit, the plain-object-pick fallback): now emits
  `interactionSelect` with `{domainId, targets, merge, method:"pick"}`. `merge` is now the canonical
  `MergeMode` wire label (`"replace"|"additive"|"invertive"`) instead of the old ad-hoc `"add"|"toggle"`.
- **`marquee_select_action`** (non-component branch): now batches ALL hit ids into **one**
  `interactionSelect` with `method:"rectangle"` — geometry (`screen_select_instances`) stays local to
  this file; no selection/merge algebra was added here, per the ticket's instruction that the algebra
  lives in the os-kernel `next_selection` machine.
- `merge_string_ids`/`merge_u32_ids` now also accept `"additive"` as a synonym of `"add"` (both feed
  `worldPick`'s unrelated, untouched component-level picking path, which still emits its own
  `"add"`/`"toggle"` vocabulary).
- `worldPick`/`setSelection`/`setHover` (component-level vertex/edge/face picking — the doc-comment-
  documented, explicitly out-of-scope separate mechanism) are untouched.
- **Deleted** the legacy `"worldSelect"`/`"worldHover"` match arms entirely — `grep -rn
  '"worldSelect"\|"worldHover"'` over `♾️infinite/` now returns nothing.
- **Tests**: extended the existing `mod tests` (new `//#region 🔖️WorldInteractionVerbs`) with the same
  five tests as the sibling file: `world_interaction_definition_declares_path_delimited_item_domain`,
  `pick_select_emits_batched_interaction_select_for_plain_object_pick`,
  `marquee_select_emits_batched_interaction_select_with_rectangle_method`,
  `pick_hover_emits_interaction_hover_and_clears_when_nothing_hit`,
  `apply_world_action_preview_applies_interaction_select_and_hover_for_world_domain`. No new test files
  — extended the file's existing `mod tests`.

## Point 2 — "is the domain bound to a window kind?" — verified, confirmed still unbound, NOT fixed here

`grep -rn "world_interaction_definition()"` over the whole repo still returns only the definition itself
and its own unit tests (now two copies, one per mounted file — both dead the same way). **Zero**
`AppDefinition` anywhere calls `.interaction(world_interaction_definition())`; the 9 world3d apps (cad,
gis-3d, procedural-3d, puzzle-3d, block-3d, remodel, process-3d, shooting, space) each only register
their own domain (e.g. `cad_interaction_definition()`) — confirmed by reading
`✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs:1207`, the only `.interaction(` call in that file.
There is no shared, framework-level place that mounts a "world3d window kind" — every
`WindowKindDefinition`/`AppDefinition.interactions` wiring is per-app, inside each plugin app crate
under `✏️s/🔌️plugins/**`.

Per the ticket's own concurrency warning ("Do NOT edit any file under `🔌️plugin/`... do NOT chase
plugin-crate errors" — every plugin crate is mid-refactor by another session right now) and per W3c's
own precedent (its summary explicitly deferred this exact wiring as "per-app (wave 4)", out of its named
file scope), wiring `.interaction(world_interaction_definition())` onto each of the 9 apps'
`AppDefinition`s was **not** attempted here — it requires editing 9 separate files under
`✏️s/🔌️plugins/**`, all currently subject to the concurrent Command/Action churn. This is flagged, not
silently dropped: the domain is declared and (as of this task) actually emitted/consumed correctly by
both `♾️infinite` mount points, but still needs one `.interaction(world_interaction_definition())` call
added per world3d app as a follow-up once the plugin-crate churn settles.

## Acceptance

- `cargo check -p semio-framework-os-infinite` — **0 errors** (69 pre-existing warnings, unrelated to
  this change — unnecessary-qualification lints in `🎲️board/🔌️ports/…` and one `future-incompat` notice
  for the `block` crate). Real output saved: `w7-world-cargo-check.txt`.
- `cargo test -p semio-framework-os-infinite --lib` — **fails to compile**, but for a confirmed
  pre-existing, cross-cutting regression unrelated to this task: `DslValue` no longer implements the
  indexing operator (`error[E0608]: cannot index into a value of type 'DslValue'`) used by the
  `args["key"]` idiom throughout this file's pre-existing tests (the same idiom my 5 new tests follow, to
  stay consistent with the file). Proof this is not something this task introduced: the exact same error
  count (15) at the exact same relative test bodies occurs in the **sibling** `♾️infinite/🦀️component.rs`
  — a file this task did not touch, migrated by a prior (W3c) task, and confirmed clean under `cargo
  check` — plus 2 identical "couldn't read `🧊️capsule_J.glb`" missing-asset errors, one per mount point.
  32 total errors, 100% attributable to this pre-existing regression (documented before this task began
  in `w3c-infinite-preexisting-dslvalue-index-break.txt`, itself attributed to the concurrent
  `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` ticket). Real full output saved:
  `w7-world-cargo-test-full.txt`.

## Outstanding

1. **Wiring `world_interaction_definition()` onto the 9 world3d apps' `AppDefinition`s** — see "Point 2"
   above. Needs one `.interaction(world_interaction_definition())` call each in
   `✏️s/🔌️plugins/{📐️cad,🌍️gis,🌀️procedural,🧩️puzzle,🧱️block,📸️remodel,🏭️process,🎥️shooting,🪐️space}/**`,
   plus the corresponding `WindowKindDefinition.interactions` entry. Not attempted — blocked by the
   concurrent, unrelated Command/Action refactor churn across every plugin crate, per the ticket's own
   instruction not to touch `🔌️plugin/`-adjacent, currently-broken plugin code.
2. **`World3dHost/🟦️component.tsx:4102,4519`** (TS side) — still calls `dispatch("worldSelect", ...)`.
   Flagged by W3c already as outside the Rust-only scope of these `ActionDescriptor`-emitting files; still
   not this task's file list (not one of the five named sites). `handle_world3d_input` in
   `Shell/🧊️component.rs` forwards whatever action name the Rust side returns generically, so once (1)
   above lands, only the TS dispatch call sites need the rename — no further Rust-side change needed.
3. **`cargo test --lib` for `semio-framework-os-infinite`** stays red until the repo-wide `DslValue`
   `Index` regression (unrelated ticket) is fixed — not something this task can or should work around.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs`
