# 🔬️ Sweep for more bugs of the same class

Having found that puzzle 2d's brush and fill were dead because data died crossing a contract boundary
with the error swallowed, the obvious question is: **where else?** This is the sweep, run in parallel
across the puzzle plugin. Every claim below was re-verified by hand rather than taken from the
exploring agent's report — two of the reported "findings" did not survive that check and are recorded
as dismissed, because knowing what is *not* broken is worth as much as knowing what is.

## Confirmed: puzzle5d has the identical defect

`🗿️artifacts/🖐️5d/…/🎭️modes/✏️edit/🪟️windows/◻2d/🦀️component.rs:107`

```rust
pub fn board_kind_catalogs_value(document: &Puzzle5dDocument) -> Value {
    let catalogs = document.kind_catalogs.clone().unwrap_or(json!({}));
    json!({
        "nodes":   catalogs.get("parts").cloned().unwrap_or(json!([])),
        "handles": catalogs.get("grips").cloned().unwrap_or(json!([])),
        "edges":   catalogs.get("fasteners").cloned().unwrap_or(json!([])),
        "wires":   catalogs.get("ropes").cloned().unwrap_or(json!([])),
    })
}
```

Used at line 152 as `glyph_catalogs_json`. This renames 5d's own slice names onto the **document**
naming — it never reaches the **engine** naming (`nodeKinds`/`handleKinds`/…), and it does not project
rows, so `Puzzle5dCatalogPartKind`'s mandatory `label` rides along and the engine rejects the row.
Same all-or-nothing push, same empty `node_kinds`, same dead brush.

5d also has **no** production manifest→engine builder to fall back on (its only `manifest_by_id` use
reads `kind_compatibility` only), and one of its three examples ships without `kind-catalogs` at all.

**Not fixed here.** This ticket is puzzle 2d, and 5d is being actively worked in
`26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`. The fix is now cheap for whoever owns it: reuse
`crate::editor::puzzle2d::board_kind_catalogs_json`'s row-projection approach, or call it directly
after the parts→nodes rename.

## Confirmed clean: puzzle3d

No production call to `set_board_kind_catalogs_from_json` or `setKindCatalogsJson` anywhere in the 3d
tree — 3d has no board-engine integration to break. Its only `manifest_by_id` use is inside
`#[cfg(test)] mod precompute_model_tests`.

## Confirmed clean: the board-event round trip

The engine emits 17 event names; `apply_board_events` handles 9. The 8 unhandled ones —
`preselect`, `preselectCancel`, `hover`, `brushPreview`, `linkCompatibleNodes`, `linkTargetRing`,
`indirectConnect`, `proximityConnect` — are all preview/transient feedback that must **not** reach the
document. **Every document-mutating event is handled.** No silent drops. (`brushPlace` is handled at
`🎮️commands/🎲️apply-board-events/🦀️component.rs:112`; it was only the *Storybook* mirror that
dropped it, which this ticket fixed.)

## Dismissed after checking: `setBrushNodeSize` is not a broken user setting

Reported as a "dead setting" because `set_brush_node_size` writes only the host while its sibling
`set_suggestion_offset` writes both `ctx.scene.runtime.suggestion_offset` and the host. The asymmetry
is real, but the conclusion was not: **no UI measure dispatches `setBrushNodeSize` at all.** Its only
references are the enum variant (`🦀️component.rs:883`), the dispatch arm (1843), the action
declaration (2067), one test (2765), and the wasm export (`🌉️wasm/🦀️component.rs:370`). There is no
slider to lose a value from — it is an action exposed for programmatic/canvas callers with no
control bound to it, and `Puzzle2dPlayRuntime` has no `brush_node_size` field by design.

Not a defect. Worth revisiting only if a node-size control is ever added to the brush options.

## Dismissed after checking: the freshly-constructed host in engagement

`🦀️component.rs:1943` passes `&puzzle_board_host()` — a brand-new, empty host — into
`puzzle2d_engagement`. That looks alarming next to the catalog bug. It is not: engagement reads the
host for exactly one thing,

```rust
let overlay: Value = serde_json::from_str(&host.overlay_paint_state_json()).unwrap_or(Value::Null);
let lod = overlay.get("lod").and_then(Value::as_str).unwrap_or(/* pane's configured mode */);
```

and node/edge counts come from the envelope, not the host. An empty host means the status line shows
the pane's *configured* LOD instead of the engine's *computed* one — a cosmetic fallback that is
already the intended default. No brush or fill behaviour depends on it.

## Conclusion

Within puzzle 2d, the kind-catalog defect was **the** bug of this class; the sweep found no second
instance. The one genuine sibling is in puzzle 5d, flagged above for its owner.
