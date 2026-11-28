# FEM 2D gumball rotate and uniform-scale rows (matrix v3)

Ticket `26/11/22/GET-FEM-PLUGIN-WORKING-END-TO-END`. Input: `🗑️generated/matrix-v3/2d/results.jsonl` (wasm of 19:52).

## Verdict: all three failures are harness defects, no product bug, no data loss

| Row | Root cause | Product or harness |
|---|---|---|
| `rotate-small`, `rotate-large`, `scale-uniform` (imageChanged=false / unreliable) | The checks selected ONE node (`ridge`). The gumball pivot is the selection centroid (`fem2d_transform_pivot`), so a rotation or uniform scale of a single node about itself maps it onto itself: the geometry cannot change, the Model image only differs by the animated gumball halo. `translate` and `scale-axis` passed only because a move, and a non-uniform scale about a pivot that does not matter for one point, change something visible or the gumball itself. | Harness (precondition). The product math is right. |
| `rotate-small` FAIL although `imageChanged=true`, `fatal=0`, `commands 5->17` | The pass term `n1.ms >= 0` waits only 15 s for the History command count to exceed the baseline. The command log grows per streamed tick while the guest re-solves in between (`translate` read `5->8`, `rotate` read `5->17`: the log was read mid-stream or late), so the wait expired once. History command counts are a weak, timing dependent oracle. | Harness |
| `rotate-large` "counts 36->0" | `x.counts()` returned `summaryCounts()` of an Inspection pane that already said `Schema` but whose count rows had not rendered yet, so it summed to 0. The final screenshot of the same row shows the full Artifact tree (12 nodes, 9 elements, 1 region, 4 supports, 2 load cases, 1 combination), Results `Converged · 36 / 512`, and the ridge still at `[4.00, 7.60]`. Nothing was lost; the Results re-solve still reported 36 entities. | Harness (readback race) |

Evidence for "no entity loss": the screenshot `2d.gumball.rotate-large.png`; the native law below shows `MoveSelection` never adds or removes an entity (ids, order, element references, supports, regions untouched; only the targeted node/region coordinates change).

Residual product defect (found here, FIXED in the follow-up below): rotating or uniformly scaling a lone node, or a zero-length drag, committed a transform command that moved nothing (an empty undo step).

## Follow-up: a gesture that moves nothing never enters history
- Root cause: `fem*_gumball_moves` only tested `MoveSelection::is_identity()`, never whether the transform carries any target elsewhere.
- Fix (shared tick contract, 2D and 3D guests): `fem2d_gumball_tick` / `fem3d_gumball_tick` now normalise a tick that moves no target of the document (nodes via `map`, regions/solids via their moved points; NaN still counts as moving so the degenerate-tick guards stay) to the IDENTITY tick over the same targets. The tool already turns an identity into no command: a one-shot yields nothing, a streamed gesture that ends on identity retracts its leaf and commits an empty transaction (no history entry); preview stream ticks stay. The normalisation must produce an identity tick (`Some`), not `None`, because the tool keeps the previous net for a missing tick.
- wgpu twin and React hosts: no change needed. They only send ticks relative to the press pose (`drag` sends nothing for an unchanged pose, `release` the identity when back on the press); they have no document, so the "moves nothing for this selection" decision belongs to the guest and now holds for every host at once.
- Laws: `an_identity_gesture_leaves_the_history_length_and_the_document_unchanged` in 2D and 3D (lone-node rotate and uniform scale, zero translate, zero angle, unit scale: tick is identity, one-shot commits nothing, a streamed run commits nothing and leaves no gesture open, a stream dragged back onto its press commits nothing; a real rotation still commits exactly one command). Existing laws that rotated or scaled a lone node (2D stale-tick law, 2D history replay law) now use two nodes. Results: 3D interaction lib 13 of 13 PASS; 2D interaction lib 10 of 11 PASS, the one failure is the known unrelated `a_gumball_move_edited_in_history_replays_its_downstream` store-retirement assertion (line 200, after all its replay assertions passed).
- Not run: the `🎮️commands` mounted-app gumball laws (`stream_ticks_in_flight_together_never_lose_the_gesture`) and the wgpu seam test; none builds a lone-node rotate or scale, but they were not re-executed here.

## Fixes (harness, `$T22/matrix-v3/`)
- `lib.mjs`: `counts()` polls until at least three count rows exist (null if never, which reads as an explicit readback failure, never as "0 entities"); new `nodePositions()` (parses `id · [x, y] Node` rows of the Artifact tree), `treeHeadCounts()` (group headings `NODES [12]` ...), `pickPair()` (pick A, Shift+click B, wait for two ids).
- `checks-core.mjs`: new `gumballGeometry()` used by `rotate-small`, `rotate-large`, `scale-uniform` (2D and 3D: same code path, the 3D twin selects `col_20_u` and `col_20_l`; if the 3D tree prints no parsable coordinates it degrades to the old image diff). It selects TWO nodes and asserts from the Artifact tree, after the commit settled (two identical consecutive readbacks):
  - rotate: centroid of the pair unchanged, pair distance unchanged, rotation angle above 0.5 degrees, every other node bit-identical, tree group counts and Inspection summary counts unchanged, Results re-solved, no fatal console;
  - uniform scale: centroid unchanged, direction unchanged (turn below 0.5 degrees), spacing factor differs from 1 by more than 1 percent, others untouched, same count and solve terms.
  `rotate-large` additionally requires no `failed` caption. The History command count is no longer a pass term.

## Native laws (Rust)
| Law | Where | Status |
|---|---|---|
| `a_large_rotate_or_uniform_scale_keeps_every_entity_and_maps_positions_exactly`: rotates by 1.0, 2.9, -2.0 and 7.0 rad and scales by 3 and 0.2 over four nodes of the demo; asserts counts of every entity kind, node ids and order, elements, supports and regions unchanged, each selected node at the exact rotation/scale about the centroid (1e-9), unselected nodes bit for bit, centroid fixed; plus a lone node maps onto itself | 2D `🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs` | PASS (`cargo test -p semio-s-artifact-fem-2d-editor-interaction --lib`, 10 tests: 9 pass) |
| same law in 3D with an independent Rodrigues reference (axes Z, X, (1,1,1), Y; angles 1.0, 2.9, -2.0, 7.0, 3.1), uniform scale 3, lone node | 3D same file | PASS |

Unrelated failure in the same 2D lib run: `a_gumball_move_edited_in_history_replays_its_downstream` ("the standalone store retires to its terminal-empty shell"), caused by the in-flight redo/retirement work of another agent (`retained-turn` debug edits); not touched.

Peer note: both cargo runs were first blocked by half-written framework edits (`BoundedJob::replay_input`, `debug_state`), fixed by their owners within minutes; nothing of mine.

## 3D twin (coordinator follow-up: `3d.gumball.rotate-large`, `3d.gumball.scale-uniform`)
Both were harness problems of my first harness version, not product:
- "tree gave no node coordinates" and `summary 47->?`: in 3D the Options layout closes the Artifact panel (it re-frames the camera), so the tree was not on screen; `counts()` right after a drag also raced the Inspection render. Fix: `readTree()` opens Artifact for the read and restores Inspection; the pass terms use the tree group counts (`NODES (16)` ...) and tree node rows; the summary count is compared only when both reads exist. Tree rows print `(x, y, z)` or `[x, y, z]` with U+2212 minus: the parsers accept both.
- 3D selection of a joint picks member elements, not nodes, so the check no longer assumes node ids: it takes the set of nodes whose tree coordinates changed and asserts rigidity (all pairwise distance ratios within 1 percent of 1, centroid fixed) for rotate and one common factor other than 1 for uniform scale.
- `solved=-1` of rotate-large: after the drag the Results caption stayed `Preparing analysis` past 150 s under load; the wait is now 240 s (the harness timeout 700 s). It is slow re-solve latency, not a failure caption. One run also hit a leftover `Loading document 1010 of 1011` replay (history carried by earlier 3D runs) that dropped the selection; it passed on rerun.
- The `n1.ms` command-count wait is no longer a pass term (final count must exceed baseline, 60 s plus 4 s settle).

## Browser verification (ports 6086 and 6087, wasm of 19:52 / 3D published wasm)
| Row | Result |
|---|---|
| `2d.gumball.translate`, `guide-cleared`, `undo-step`, `scale-axis`, `pointer-cancel` | PASS |
| `2d.gumball.rotate-small` | PASS: moved 2 nodes `[p0_l2,ridge]`, centroid shift 0.000, distance ratio 0.999 (display rounding), counts 12 nodes -> 12 |
| `2d.gumball.rotate-large` | PASS: 58.6 degrees, centroid 0.000, entities `NODES 12, ELEMENTS 9, ...` before and after, Results `Converged · 36 / 512` |
| `2d.gumball.scale-uniform` | PASS: factor 1.490, centroid 0.000, same counts |
| `3d.gumball.rotate-small` | PASS: 4 nodes moved, ratios 0.9997..1.0005, centroid shift 0.002 |
| `3d.gumball.rotate-large` | PASS (156 s): same rigidity, nodes 16 -> 16 |
| `3d.gumball.scale-uniform` | PASS: factor 6.67 on 4 nodes, centroid 0.003 |
The 2D entity counts never dropped to 0 in any run: confirms the `36->0` was the Inspection summary race.

## Files
- `$T22/matrix-v3/lib.mjs` (`counts`, `nodePositions`, `treeHeadCounts`, `pickPair`), `$T22/matrix-v3/checks-core.mjs` (`readTree`, `gumballGeometry`, `gumballDrag` pass term)
- `$T22/gb-law.sh` (private lane `C:\t\gb`, `-j 3`)
- Rust tests only: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs`
