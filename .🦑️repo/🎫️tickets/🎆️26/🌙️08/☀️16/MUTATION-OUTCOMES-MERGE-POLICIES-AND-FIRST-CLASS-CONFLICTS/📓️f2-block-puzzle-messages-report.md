# F2 — Block + Puzzle gate remediation report

## Gate
`bun ./📜️script.ts verify mutation-outcome-law … | grep -cE "block|puzzle"` → **0** (was 32 block + 8 puzzle).
All 40 leaves got real verb-family detection per `📋️contract-freeze.md`, wired through the actual snapshot
types (read, not assumed) — logs in `🧪️f2-verify-full.txt`, `🧪️f2-breaches-raw.txt`.

## Key deviation from the brief's literal `*-kind-*` rule (validated against the real snapshot)
Block's `node_kind`/`part_kind`/`object_kind` (`BlockKindIdentity`) and `presentation`/`part_2d`/`part_3d`
are **document-root singletons**, not catalog rows addressed by id — the mutation payloads carry no id to
look up (e.g. `RenameNodeKind{ new_name }`, no `id`). So for the 26 block `*-kind-*` / `update-*` /
`change-meta-description` leaves: **no `mutation.target-missing`** (structurally unreachable — the field
always exists), **no Fatal `duplicate-id`** on rename (nothing to collide with), only Warning `mutation.no-op`
on an unchanged value. Same reasoning applied to puzzle's `change-manifest-id`, `change-domain`,
`rename-puzzle5d`, `change-description`, and `replace-kind-catalogs` (all live on the root `meta` singleton
or root fields). Puzzle5d's `replace-kind-catalogs` is content-addressed (`kind_catalogs_child_handle` hashes
content into `child_id`), so its no-op check compares minted-handle-id + overflow-extra against `base` —
deterministic, no reliance on the documented-stale scratch cache.
Camera leaves (`move-camera2d/3d`, `scale-camera2d/3d`) are also root singletons: Fatal `mutation.invariant`
on non-finite x/y/position/target or non-finite/non-positive zoom, Warning `mutation.no-op` on an identical
transform — no target-missing per the brief.
No 8th code introduced; no leaf added to the total-kind allowlist (all 40 got a real check).

## Blockers found and fixed (outside the 40 flagged leaves, inside the lease, required for `cargo check`/`test`)
Pre-existing/concurrent breakage unrelated to message codes, needed to get the crates green:
- `🧊️3d/…/🌱create-vortex-kind` and `❌delete-vortex-kind` `🔺️diff`: referenced dead field `base.vortex_kinds`
  (snapshot moved to composed `catalog`+`vortex_kind_extra`) → routed through `vortex_kinds_of(base)`.
- `🧊️3d/…/🧬️mutations/🦀️component.rs:348` (test): same dead-field bug → `set_vortex_kinds(&mut base, …)`.
- `◻2d/🖐️5d/🧊️3d …/💾️binary/🦀️component.rs` tests: `*Store::new(…)` now returns `Result` (C6) → added
  `.expect("valid initial state")` at 3 call sites.
- Puzzle: 8 leaves (`create-node/part/object/target-volume/reference`, `replace-node-handle/part-grip/
  object-vortex`) had a misplaced `protocol::MutationOutcome::new(` paren wrapping an `if let`/`for`
  statement instead of the trailing struct literal (syntax error) — moved the wrap to the tail expression.
- Puzzle: 8 more leaves (`add/remove-node-handle`, `add/remove-part-grip`, `add/remove-object-vortex`, plus
  the 3 `replace-*` above) had a bare `return XDiff::default();` where the signature requires
  `MutationOutcome<XDiff>` — filled in per the same verb-family table: "already exists" ⇒ Warning `no-op`,
  "not found" (replace/remove) ⇒ Error `target-missing`.

## Left alone (out of scope, confirmed concurrent/foreign)
`✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs:44,48,52` — `.editor_mutation_roster::<Puzzle{2d,3d,5d}PlayApp>()`
fails `SemanticMutation<Puzzle*PlaySnapshot>` (an editor/viewer `PlaySnapshot` split, unrelated to
`MutationOutcome`). `git log --date=iso` shows this file's last touch (`c8a29e41c5`) landed **after** this
lane's start commit (`5a1367dfcc`) — a peer session is actively working it (matches the
ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET ticket in git status). Not touched; blocks `semio-s-plugin-puzzle`
compilation and therefore its `cargo test`.

## Verify (real numbers)
1. Gate: **0** block/puzzle breaches (`🧪️f2-verify-full.txt`).
2. `cargo check -p semio-s-plugin-block`: **0 errors** (131 pre-existing warnings) — `🧪️f2-cargo-check-block2.txt`.
3. `cargo check -p semio-s-plugin-puzzle`: **3 errors**, all the foreign `SemanticMutation<PlaySnapshot>`
   ones above — `🧪️f2-cargo-check-puzzle4.txt`.
4. `cargo test -p semio-s-plugin-block --lib`: **185 passed, 14 failed** — `🧪️f2-cargo-test-block2.txt`.
   Failures confirmed unrelated to the 40 fixed leaves (none call any touched mutation):
   `move_handle_diff_absorb_law` / `move_vortex_diff_absorb_law` / `move_grip_2d_diff_absorb_law` (pre-existing,
   different mutations — `move-handle`/`move-vortex`/`move-grip-2d`, never touched by this lane) plus 11
   editor/viewer fixture/registry tests (`export_media_catalog_out_wraps_*`, `set_active_example_loads_*`,
   `the_manifest_stitches_every_taxonomy_node`, `command_from_action_covers_every_declared_action…`,
   `declares_the_vortex_interaction_domain…`, `view_actions_never_emit_artifact_mutations…`,
   `create_block3d_viewer_builds_a_definition…`) — all in editor/viewer surfaces this lane's lease does not
   include logic for and did not edit.
5. `cargo test -p semio-s-plugin-puzzle --lib`: blocked — crate does not compile (item 3 above, foreign/concurrent).

## Files touched
32 block `🔺️diff` leaves + 8 puzzle `🔺️diff` leaves (the flagged breaches) + 2 block3d vortex-kind diff
leaves + 1 block3d test fixture + 3 block `💾️binary` test files + 16 puzzle diff leaves (8 syntax fix, 8
return-type fix) — all under `✏️s/🔌️plugins/🧱️block/**` and `✏️s/🔌️plugins/🧩️puzzle/**`, all within lease.
Logs: `🧪️f2-breaches-raw.txt`, `🧪️f2-verify-full.txt`, `🧪️f2-cargo-check-block2.txt`,
`🧪️f2-cargo-check-puzzle4.txt`, `🧪️f2-cargo-test-block2.txt`.
