# Current Fixture and Test Repairs

Restored the Writer completion-rejection test's missing job owner and passed the current ViewModel through window/context-menu tests. The FEM3D route oracle now covers WindowTransient explicitly. Removed unreachable GIS and Sourcing helpers and unused Jack/Writer bindings.

Equation's six diff fixtures now contain exactly the four artifact fields defined by the current schema and owned codec. Updated the matching slot assertions and added independent Serde JSON shape checks for the applied coefficient and point mutations. Existing inverse, committed-state and codec assertions remain in place.

13 Rust files parsed; runtime and full compiler validation pending.

- ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs
- ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs
- ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/➗️equation/🧬️schema/🧬️mutations/🎚️change-coefficient/🧪️tests/🔬️t001/🔺️diff/🔣️.json
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/➗️equation/🧬️schema/🧬️mutations/🎚️change-coefficient/🧪️tests/🔬️t001/🦀️.rs
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/📍️seeds-the-empty-b1911b/🔺️diff/🔣️.json
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/📍️seeds-the-empty-b1911b/🦀️.rs
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧭️change-graph-directed/🧪️tests/➡️keeps-an-already-8d0f96/🔺️diff/🔣️.json
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧭️change-graph-directed/🧪️tests/➡️keeps-an-already-8d0f96/🦀️.rs
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🔄️replace-points/🧪️tests/🔄️replays-the-95870f/🔺️diff/🔣️.json
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🔄️replace-points/🧪️tests/🔄️replays-the-95870f/🦀️.rs
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🧪️tests/🕸️replays-the-61d5e6/🔺️diff/🔣️.json
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🧪️tests/🕸️replays-the-61d5e6/🦀️.rs
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🧪️tests/🔬️t004/🔺️diff/🔣️.json
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🧪️tests/🔬️t004/🦀️.rs
