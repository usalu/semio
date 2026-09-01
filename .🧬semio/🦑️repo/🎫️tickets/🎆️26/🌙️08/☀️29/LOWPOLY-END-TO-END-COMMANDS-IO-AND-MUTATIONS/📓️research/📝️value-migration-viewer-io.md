# Value-Derive Migration — Viewer/IO Slice

Scope: `👁️viewer/`, `🚪️io/`, artifact root `🦀️component.rs`, plugin root `🦀️component.rs`,
`📦️packages/🦀️rust/` (glue.rs, Cargo.toml). Ran after the mechanical derive pass (427→159 error
lines) that added `value_derive::ToValue`/`FromValue` + `#[value(rename_all)]` across 96 sites.

## Cascades that cleared on their own (not touched)

- Target 1 (`👁️viewer/🦀️component.rs:41`, `Mutation<LowpolySnapshot>` bound) — the trait-bound part
  was a genuine schema-agent cascade and cleared once they landed `LowpolyMutation: Mutation<...>`.
- Target 2 (`dyn_enum_close!` for `LowpolyApps` in plugin root) — fully cleared on its own, zero
  errors there in every run.

## Genuinely mine — fixed

1. **Viewer `render` signature drift** (unrelated to value_derive, but blocking, in-scope files):
   `ArtifactViewer::render` now returns `UiAssemblyResult<ComponentTree>` (was `UiNode`) and window
   `render` fns now return `UiAssemblyResult<BuiltNode>`. Mirrored the landed stdio/bcf pattern
   (`main::render(...).map(built_to_component_tree)`, `built_text_to_component_tree` for the
   fallback arm). Fixed in:
   - `$A/👁️viewer/🦀️component.rs` — trait `render`, dropped now-unused `UiNode` import.
   - `$A/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/🦀️component.rs` — `render` return type,
     dropped `UiNode` import.
2. **Missing `.await` on two now-async trait methods** (framework's async convention, not opt-in):
   `ArtifactSchemaFields::field_states()` and `LowpolyArtifact::artifact_schema_id()` both changed to
   `async fn`/`-> impl Future<...>`. Fixed both call sites in the leaf test at the bottom of
   `$L/🗿️artifacts/💠️lowpoly/🦀️component.rs`.
3. **Test not gated by `#[cfg(test)]`**: that same leaf test function
   (`artifact_schema_descriptor_leaves_parse_and_field_states_match_snapshot_json`) sat *outside* the
   `#[cfg(test)] mod tests { ... }` block, so `cargo check --lib` compiled it unconditionally and hit
   an unresolved dev-only crate (`semio_framework_async_macros` is a `[dev-dependencies]` entry).
   Added `#[cfg(test)]` directly on the function — a real pre-existing bug, not migration fallout.
4. **`📦️glue.rs` module map**: one stale `#[path]` (`🧬️schema/🧬️mutations/🦀️component.rs`, renamed to
   `🦀️.rs` by the schema agent) — resolved by the time I re-ran after the schema agent's edit landed;
   verified programmatically (Python walk of every `#[path = "..."]` attr against disk) — 257 total
   path attrs, 0 unresolved now.

## Not touched (confirmed out of scope / untouched)

- `🚪️io/`: zero errors/warnings this whole session, made no edits — txt/json/obj/ply/png round-trip
  and the stl/gltf/dwg/las explicit-`Err` stubs are exactly as this ticket established them.
- `🧬️schema/`, `✏️editor/`: not edited; final run still shows 34 errors there (E0277 ToValue/FromValue
  on `PaintStrokeState`/`TransformState`/session BTreeMap, `render` signature drift same as above but
  in editor's own files, a `Box<LowpolyCommand>` vs `&LowpolyCommand` mismatch, one more missing
  `.await`) — all inside `✏️editor/` files, owned by the other agent. Flagging as handoff, not fixed.

## Before/after (my subtree only)

- First run after mechanical pass: 6 error sites in `👁️viewer/` + artifact-root leaf test (plus a
  transient glue.rs `#[path]` break surfaced once the schema rename landed).
- Final run (`cargo check -p semio-s-plugin-lowpoly --lib --message-format short`): **0** errors and
  **0** warnings anywhere under `👁️viewer/`, `🚪️io/`, the artifact root, the plugin root, or
  `📦️packages/🦀️rust/`. Crate-wide total is 34 errors, all confirmed (grep by path) to live under
  `✏️editor/`.

## Handoffs

- `✏️editor/` still needs: `ToValue`/`FromValue` on `PaintStrokeState`, `TransformState`, and a
  `BTreeMap<String,String>` used by-ref in `🖌️session/🦀️component.rs` (lines ~610/767/823/917/941);
  `LowpolyTransient: ToValue + FromValue`; the same `render` signature migration
  (`UiNode` → `UiAssemblyResult<ComponentTree>`) in `✏️editor/🦀️component.rs`; a
  `Box<LowpolyCommand>`/`&LowpolyCommand` mismatch and a missing `.await` on a `VcsError` future,
  both in `✏️editor/🦀️component.rs`.
