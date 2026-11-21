# Graph Interaction Namespaces

Generation3d now publishes distinct semantic node, handle and edge IDs through the existing canvas interaction prefixes. Both Edit and View use the same topology writer, including parent references, so the framework's string-keyed hover traversal can preserve all three marks when their authored text is identical. Artifact tree row keys carry the same semantic IDs, and Inspection hover bindings encode the target independently of the row key.

Scene restoration and authoring continue to use authored widget and wire IDs and the canonical port address. The Edit, Generate and View object previews publish the semantic port identifier while keeping each visual instance identifier as before. Preview marks route by kind, including wire endpoint and upstream inlet expansion. The Flow window retains the first framework hover target and sends its explicit node, port or edge fields into scene restoration. Keyboard, delete and transform commands translate graph targets before operating on the authored graph.

## Verification

- The registered Rust fixtures cover an actual pair of widgets, a handle and a wire with equal authored text. They exercise shared topology closure, distinct world marks and wire endpoints, typed visual scene restoration, source-window metadata and hover clearing.
- The focused native directional/graph witness passed six tests, including the exact same-text target case. The paired Edit/View preview witness passed four tests. The focused scripts use live Rust source sections and minimal host stubs.
- The focused browser action and SQLite fixture suite passed 21 tests with 97 assertions. The Inspection JSDOM fixture passed one test with 18 assertions.
- The pinned Rust formatter parsed the modified sources, and Git's whitespace check passed. Full Cargo and mounted renderer verification remain with the owning build lane. Generated witness files were removed.
