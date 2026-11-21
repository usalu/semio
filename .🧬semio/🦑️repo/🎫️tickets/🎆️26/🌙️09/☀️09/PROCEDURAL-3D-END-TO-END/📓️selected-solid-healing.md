# Selected solid healing

The existing Heal kernel compared and rewrote every vertex in the body. It could snap a neighboring solid to the selected one and leave all vertex edits live when validation refused the result. The repaired path builds its snap plan only from vertices reachable from the selected solid, checks for shared vertices before applying changes, and restores the original positions on validation failure. Successful changes alone enter the history and invalidate the selected solid's primitive provenance. A valid box remains unchanged.

The portable fixture contains two closely spaced distinct box profiles and a selected-edge collapse tolerance. Three.js independently confirms the geometric distance relationships. A compact native witness ran the existing and current healing algorithms in a controlled topology model. The old algorithm moved the unselected solid. The current algorithm passed both isolation and rollback, including unchanged history after refusal. A registered native test exercises the same cases with real BRep boxes and awaits the linked Cargo lane.

The Flow Heal operator still needs retained admission and funded publication. Wider repairs such as wire-gap closure and removal of collapsed edges need dedicated topology algorithms.
