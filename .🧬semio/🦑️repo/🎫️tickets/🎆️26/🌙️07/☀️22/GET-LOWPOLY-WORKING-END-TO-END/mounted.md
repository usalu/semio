# Mounted Lowpoly Journeys

The retained editor integration tests still read mesh_content after geometry edits. Current edits correctly persist managed mesh_state independently from authored literal source, so those assertions can miss changes or fail for newly created primitives. Updated existing mounted journeys to inspect managed topology, preserve literal source checks, and compare the unselected object to its actual pre-edit snapshot.

Verification queued through repo cargo: semio-s-artifact-lowpoly-lowpoly-editor / lowpoly_editor_mounted. Machine capacity queue is shared with other active agents; no bypass used.

Added a language-neutral managed-edit journey contract: face pick, extrude, binary pack reload with fresh scratch, JSON reload through independent serde_json parser/writer, exact geometry undo and redo. The contract also requires literal source preservation. Updated existing gumball and face tests to assert current managed geometry.
