# Wave U — Editor Correctness (2026-09-10)

Claim timestamp: **2026-09-10 00:25 CEST** (wave start). Repo MCP (`repo://goals`, ticket tools) is not
mounted in this Cursor session; work proceeds inside the existing ticket
`26/09/02/PUZZLE-3D-END-TO-END`.

Assignment: five editor-correctness defects in `semio-s-artifact-puzzle-3d`.

| # | Defect | Status |
|---|---|---|
| 1 | Gumball undo broken (`gumball_translate_drag_coalesces_into_one_edit`) | investigating |
| 2 | `relocateTargetVolume` / `worldRelocate` declared `ActionKind::View` while mutating | investigating |
| 3 | Outliner continuation row inert | investigating |
| 4 | Inspection `ids` list unbounded | investigating |
| 5 | Stale fixture id `setFillCountStep` | investigating |

Hard constraints: no modifying git; no edits to `✏️editor/⏳️precompute/🦀️.rs` or its tests (W-F);
no wasm rebuild; no touch of :6013; no kill of foreign processes; cargo envelope as in the brief.

---

## 0 Conditions

- Concurrent peers in the same tree. Foreign files modified within 10 minutes are waited out.
- Precompute owned by W-F — if a fix requires it, hand over and skip.
- Suite baseline claimed at hand-over: 613 passed / 9 failed. Defect 1's test is one of those 9.
- Generated artifacts go under `🗑️generated/` and are deleted when done.
