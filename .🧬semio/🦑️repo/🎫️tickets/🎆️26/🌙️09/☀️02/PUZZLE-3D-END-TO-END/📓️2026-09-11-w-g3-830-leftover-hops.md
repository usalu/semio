# W-G3 leftover hops for rebuild #44

Guest hops only. No wasm rebuild, no serve kill, no git. Vite-live `1:window`, leftover Inspection refresh, gumball mode `move`, and W-AB `importFixture` were left in place.

Canonical fill-build note: `📓️2026-09-10-fill-build-host-tick.md` §8.30.

## Hop 1 — first-pick leftover `selectedIds`

Canvas pick of `seed-left-001` (object id on vortex domain) used to publish hover-only leftover with `selectedIds:[]`.

`dispatch_interaction_action` `INTERACTION_SELECT` now clones pick targets into leftover selection when `next_selection` returns empty ids. Hover leftover still restores a prior leftover selection. Vortex-uuid Inspection fall-through is unchanged.

Law: leftover `interactionSelect` of `seed-left-001` → leftover `selectedIds` contains it.

`cargo test -p semio-framework-plugin leftover_interaction_select_object_id` → **ok**.

## Hop 2 — leftover `translateSelection` mesh-mode pre-admit

Leftover gumball drag dispatched `{ids, mode: mesh}` and used to refuse with the old pre-admit string. Ingress already pre-admits a vacant residue-class slot before minting the op id. The new law requires leftover mesh-mode translate with explicit ids to move the object (no Notify refuse).

`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib leftover_translate_selection -- --test-threads=1` → **ok** (`leftover_translate_selection_mesh_mode_commits_pose_delta`, locked refuse).

Sibling leftover overlay translate + vortex-uuid Inspection fall-through also **ok**.

## Files

1. `framework/.../plugin/rs` — `INTERACTION_SELECT` keeps object-id pick targets when `next.ids` is empty.
2. `framework/.../plugin-runtime-plugin-builder-contract/rs` — object-id leftover `selectedIds` law.
3. `s/.../puzzle/3d/editor/tests/unit/rs` — mesh-mode leftover translate pose-delta law.

## Handoff

Ready for **#44**. Re-probe `--selection --clipboard --gumball --locked --port=6014` on vite-live after the wasm rebuild. Do not revert W-AB `importFixture`.
