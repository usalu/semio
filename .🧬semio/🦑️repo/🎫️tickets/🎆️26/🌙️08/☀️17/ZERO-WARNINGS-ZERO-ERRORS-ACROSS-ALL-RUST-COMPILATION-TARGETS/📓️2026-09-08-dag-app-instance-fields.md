# DAG App Instance Fields

Native582 exposed invalid surface_contexts entries in AppInstance patterns and constructors, including Default::default() calls in patterns and fields following the rest pattern. DagNodeKind and DagNodeKindDsl both currently declare exactly instance_id, plugin_id, app_id, icon, inputs and outputs. No surface_contexts/surfaceContexts field appears in the artifact's other schema languages.

Pass600 removes 18 undeclared-field entries across nine files, including consumers and existing test constructors. Defined fields, port traversal, owned retirement, DSL conversions and assertions are preserved. No substitute default field or compatibility behavior was introduced. The initial broad read yielded before mutation; the bounded reread supplied the exact patch lines.

Files:

- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧪️tests/🔬️dag-direct/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧪️tests/🔬️dag-vcs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🦀️.rs`

Fresh parse, compiler and runtime checks remain pending.

Pass601 adds four existing runtime laws to the ticket runner: the DAG intrinsic Serde/selection oracle, DSL round trips for every node kind and the demo fixture, and the surface AppInstance conversion test. These directly exercise the corrected data paths. The runner now selects 322 laws across 56 groups. Execution remains pending.
