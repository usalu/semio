# Note Point Eraser Verification

The editable Canvas previously tested only the stored point samples. A touch between the endpoints of a sparse segment had no effect, and fragments from a nested stroke were appended to the root.

The renderer now subtracts the open eraser circle from each segment in local stroke coordinates. It retains exact cut boundaries and joins uninterrupted outside portions across original vertices. The first outside run updates the existing block without changing its ID or sibling position. Later runs insert immediately after that block in the same parent, with unique IDs checked against the current document. A completely erased stroke produces a single remove event. A pure tangency leaves segment geometry unchanged. The eraser radius applies to the stroke's polyline centerline.

The neutral cases in `eraser.cases.json` cover sparse and dense samples, reversed direction, a diagonal cut, translated coordinates, tangency, apparent dots, full erasure, continuity, multiple crossings, and repeated erasure at the same location. Three.js ray–sphere intersections independently validate the expected clipping geometry. A separate event and tree check covers consecutive sibling edits, nesting, and an ID already used by a nested sibling.

The direct Bun contracts failed before the corresponding clipping, placement, and ID fixes and passed afterward. The mounted renderer's filtered InkCanvas suite passed 41 tests. No guest build was started for this change.
