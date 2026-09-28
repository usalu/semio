# Document-Space Alignment and Distribution

The selection command previously measured each selected layer as if it were a root, then added world displacement directly to local translation. A rotated, scaled or sheared ancestor therefore changed the meaning of alignment. It also rejected otherwise valid selections spanning different groups.

The planner now composes the complete ancestor chain, measures the actual transformed geometry, computes document-axis alignment or equal edge spacing, and converts each displacement through the inverse parent linear transform. Selected ancestors own descendant movement. Source transforms retain rotation, scale and shear; only translation changes. All mutations are prepared before publication, and locked/hidden or singular parents reject the operation atomically. Unchanged transforms produce no mutation. Arrangement supports different parents; existing sibling restrictions remain for structural commands such as grouping and reordering.

A schema-first geometry module has Rust and TypeScript implementations. Eleven neutral cases cover all eight operations, unsorted selection order, overlapping shapes/negative spacing, minimum counts and invalid sizes. Six neutral document cases cover rotation, shear, reflection, distinct parents, selected ancestor/descendant pairs, singular/locked parents and multiple ancestor levels. Three.js independently checks world boxes and inverse transforms. Native document tests apply the real mutation plan, and a registered app test verifies a cross-group alignment as one undo/redo history entry.

## Verification

- `tests-arrangement-red.txt`: missing geometry implementation reproduced, exit 1.
- `tests-arrangement-current.txt`: 187 tests / 19,008 assertions passed, exit 0.
- `tests-arrangement-nested.txt`: 188 tests / 19,058 assertions passed, exit 0, including five initial document fixtures and independent Three.js comparisons. The sixth grandparent fixture was added afterward and needs the final run recorded below.
- Each successful run also passed the 48-case field-patch oracle and 36-command publication audit.
- Native run 88159 (`tests-native-svg-gradients.txt`) is live. It started before arrangement edits; coverage must be confirmed at completion before claiming any current native pass.
- Component build 93519 remains live, so browser verification of the changed command is pending.

The existing synchronous selection discovery and recursive geometry bounds remain a large-document responsiveness limitation. The arrangement traversal has a 4,096-node capacity guard, but that is not fully resumable or cancellable planning. The full editing goal remains active and incomplete.

- Final `tests-arrangement-final.txt` passed **188 tests / 19,076 assertions / 26 files**, exit 0, including the grandparent fixture, 48 field-patch cases and 36-command audit. Native 88159 and component 93519 were polled afterward and remain live.
