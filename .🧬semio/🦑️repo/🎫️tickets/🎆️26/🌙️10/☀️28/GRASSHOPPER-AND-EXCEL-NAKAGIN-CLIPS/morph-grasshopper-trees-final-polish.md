# Morph Final Polish: grasshopper / trees (`Beat5_DataTrees`)

Status: code done, frame verification pending (see "External dependency").

## What changed (`G/🌳️trees/🐍️.py`)

1. `recap`: each legend term flies out of a copy of the live object it names, then its row (label, mini tree, result) fades in at the landing point. Sources: Tree and Flatten from the diagram, Simplify from the diagram trunk lines, Branch from the colour Panel, Index from its first Panel row, Path from the running `#21` chip, Item from capsule 21, Graft and Path Mapper from their components, Flip Matrix from the tower. Rows start staggered (`LaggedStart`, 0.18). The old diagram and pipeline fade out in parallel (the former shrink-into-row fold was dropped because fliers replace it); duration 2.6 to 3.6 s. Sources are kept in `self._anchors` (set in the `fix` section).
2. `fix`: new `_fan`: after the grafted colour Panel and its swatches appear, a swatch copy flies to every one of the 140 leaves of the grafted diagram and carries its item's colour (teal or orange from `paint["cp2"]`); then the diagram swaps to the per-item coloured leaves. Stage order: Succession(texts and second part fade in, fan, diagram swap).
3. `why`: unchanged; graft is only applied in `fix`, so the fan-out lives there.

Captions and narration unchanged (en and de).

## Tests

- Morph toolkit: `python -m unittest 🎓️teaching/🏛️architecture/🔀️morph/🧪️tests/🐍️.py`: 37 tests OK (23.7 s).
- Grasshopper clip tests: not run (they build towers, see below).

## External dependency: occlusion culling slowdown

`🏢️nakagin/🎨️visuals/🐍️.py` (other session, mtime 22:19, ~800 added lines uncommitted) made `tower_axon` very slow (`_opaque_visible` polygon clipping). The scene builds about 25 towers before the first clause, so a capture takes far longer than before: the first tower alone took over 5 minutes CPU. Two earlier en captures were killed while still in construction (not a hang: faulthandler dump shows `_tower` to `tower_axon` to `_opaque_visible`). Not touched per instruction. Rerun `recap fix` en (pid 56980) is waiting on it.

## Capture attempts (en, `recap fix`)

1. Killed while still building the first tower (slow culling, see above).
2. Restarted after the visuals file changed (22:19:19): exit 139 (segmentation fault inside numpy `roll`) before any frame was saved.
3. Rerun with the final visuals file: construction finished, playback reached animation 7 (the 140-capsule `tree` Swarm), 75 s per sub-step (5 steps, 6 min 19 s), then exit 139 again. No `recap`/`fix` frame was written. The capture plays every earlier clause even when only `recap fix` is saved, so the crash and the slowness happen long before the clauses of this task.

## Frames checked

None. Blocked by the external dependency (slow `tower_axon` culling and the segfault while rendering the `tree` swarm). German capture not started (must follow English). Next: rerun en then de `recap fix` when the Nakagin visuals work settles; check recap mid frame (fliers leaving diagram, Panel, chip, capsule 21, components) and fix mid frame (swatch copies fanning to leaves).
