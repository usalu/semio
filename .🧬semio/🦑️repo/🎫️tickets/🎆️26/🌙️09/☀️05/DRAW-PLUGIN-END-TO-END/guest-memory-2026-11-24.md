# Draw guest linear memory: who keeps it (2026-11-24)

Guest r37, playground 127.0.0.1:6064 untouched, full stock (2^31 per axis), headless Chromium, load 12 to 15. Probe: `probe-guest-memory.mjs` (samples every wasm memory of every worker once per 1.5 s, plus the actor's remaining stock from the bootstrap receipts: `cap` = remaining capacity MiB, `rel` = remaining release MiB; net retained = rel - cap). Three runs, one browser at a time. Logs were generated output and are deleted; the numbers are below.

## Measured

| Action | Guest memory (MiB) | Net retained by ledger (MiB) |
| --- | --- | --- |
| after boot (3 runs) | 230, 231, 230 | 6 to 7 |
| 6 selections of one row, 10 camera pans (wheel), 30 s idle (run 1; the adds of that run never produced a row, so it is selection/pan/idle only) | 231 flat | flat (charged and released equally) |
| add Rectangle (run 2) | 231 -> 231 | 7 -> 9 |
| 15 s and 30 s idle after an add | no change | no change, nothing is released |
| undo (run 2) | 231 -> 233 | 9 -> 10 |
| 2 x (add Rectangle, undo) (run 2) | 233 -> 238 | 10 -> 16 |
| idle 15 s after all that (run 2) | 238 flat | 16 flat |
| add Ellipse, then 3 x (add Ellipse, undo) (run 3) | 231 -> 237 | 7 -> 16 |
| delete step | not reached (no row was selected) | |

Reading:
- Memory grows only when a revision is published: add, undo and redo each add roughly 1 to 2 MiB with this small document (4 layers plus the emblem). Selection, camera pan and idle add nothing, and idle frees nothing (wasm memory cannot shrink, and the ledger shows nothing released).
- The growth follows the number of published revisions (edits and undo steps count the same), not the number of layers, and its size follows the size of the scene: in the failing run the scenes were larger and grew to about 25 MB per edit (308 MB charged, 156 MB released).
- Net retained (charged minus released) grows by the same edits; it is the retained-owner ledger, so the memory is live, not allocator fragmentation: a fragmentation or Vec-growth cause would also rise during selection/pan turns and would not match the ledger.
- No `zero progress` / out-of-memory console line in these short runs. The retirement ladder does get turns (the superseded candidate and the plan/producer cleanups are paid, rel keeps rising with cap); it is not starved. It simply has nothing to retire for the old visual.

## Owner (read in code, confidence: inferred from code plus measured correlation)

`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧵️mounted/🦀️.rs`

- Line 118 (the publish branch of `GeometryJob::advance`): when a new revision publishes, the previous visual's slot only gets `r.retiring[old]=true`; `i.visual` switches to the new slot. Nothing ever moves the old `State.visual` (a `PreparedScene`: `plan` with all segments, `geometry` with every flattened contour) into `State.discard`.
- Line 75 `State::begin_close` is the only place that does that (`visual.take()` -> `discard = PreparedSceneCloseJob`), and it is called only by `GeometryOwner::begin_close` (line 205, instance close).
- Line 197 `GeometryOwner::slot` only offers a shell for cleanup when `cleanup_pending()` (line 35), which is false for a shell whose only content is `visual`. Line 76 `State::empty` requires `visual.is_none()`, so the shell is never freed. `retiring` is read only by `prepare_source` (to avoid reusing the slot), so the flag has no consumer that retires.
- Consequence: every published revision keeps its whole scene until the window closes. Growth is linear in revisions times scene size, up to 256 shells (`SHELLS`), after which `prepare_source` returns false and the editor stops. Reuse of layer geometry (`adopt`, same file) moves the reused contours into the new scene and leaves placeholders, so it reduces but does not remove the retention. The file is being edited by others; line numbers shifted by one during this session.

Not the cause (measured or read): selection, pan, idle (flat), stock renewal (full stock), superseded candidates (cancelled and retired by `transfer(false)`), producer snapshot read and plan (`cleanup`, `raw_discard` are retired in the same ladder).
Not isolated: undo history and decoded snapshot leases could keep a smaller share; the runs cannot separate them, because the guest is a release component and no per-owner byte counters exist in it.

## Smallest fix (patch text; not applied: file is in the do-not-touch list)

In the publish branch (line 118), replace
`if let Some(old)=i.visual.filter(|old|*old!=self.slot){r.retiring[old]=true;}`
with
`if let Some(old)=i.visual.filter(|old|*old!=self.slot){r.retiring[old]=true;if let Ok(mut owner)=r.shells[old].try_borrow_mut(){if let Some(state)=owner.as_mut(){state.begin_close();}}}`

Why this is enough and sound: `begin_close` sets `cancelled`, retires nothing twice (`cache` and `producer` of a published state are already `None`), and moves `visual` into `discard`. `maintenance_pending(instance)` then reports pending, the existing ladder (`GeometryOwner::slot` -> `State::close_step` -> `PreparedSceneCloseJob`) retires it, and when `State::empty()` the shell is set to `None`, `retiring[slot]=false` and the slot is reusable. `i.visual` already points to the new slot, so no reader (`with_visual`, `with_query`, `visual_shell`) can reach the old one. The `try_borrow_mut` cannot fail inside the step (the old shell was borrowed and released by `adopt` earlier in the same step, and the job runs on the single guest thread); if it ever did, the old scene would stay as today. A unit test belongs in `🧵️mounted/🧪️tests/🔬️unit`: publish revision A, then revision B, run `maintenance` until `maintenance_pending` is false, assert the shell of A is `None` and `terminal_is_empty` for instance A's slot, and that ledger retained bytes return to the single-scene value.

## Verify after the next guest build

1. `PLAN=add,addundo,idle SHAPE="Add Ellipse" W=25000 IDLE=15000 PLAYWRIGHT_BROWSERS_PATH=$PWD/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright timeout 285 repo tools exec --light bun -- <T>/probe-guest-memory.mjs`.
2. Expect: memory rises with the first revisions and then plateaus (about one scene plus one candidate) instead of rising 1 to 2 MiB per edit and undo; net retained (rel - cap) returns to about 7 to 9 MiB after each idle of a few seconds; idle must now lower the net retained value (before: flat).
3. Failing scenario: add 12 shapes through the Artifact panel (Ellipse, Rectangle, Polygon, Line, repeating) with undo in between; the guest must stay well under 512 MiB and no `live cleanup zero progress` or out-of-memory line may appear.
