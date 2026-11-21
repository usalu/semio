# Rewrite engine

The rule planner produces ordered Jack graph effects against an immutable graph. The existing graph application entry point applies those effects atomically, and the Puzzle integration can translate them into native mutations.

The right-hand side deletes, creates, merges, then applies property assignments. This lets assignments address variables created or merged by the same rule. The planner passes owned property values to Jack, so structured literals and bound strings retain their values. Only the predicate is parsed from text, and it must produce one WHERE clause.

The preview and planner share the right-hand side clause order. Jack accepts the preview's escaped double-quoted strings and quoted kind names such as `"edge.link"`. The standalone Jack parser supports scalar literals. Structured right-hand-side values execute through the typed planner.

The JSON application result now carries a self-contained Jack scene with nodes and edges, which can be read through `Graph::load_scene_json`.

## Workflow delivery

Host negotiation uses the port media types and resolves the shared invocation kind to an intrinsic wire with the same named schema. The headless runner uses the app's port IDs for media commands and the workflow's scoped port IDs for delivery records and fingerprints.

Connected, required and explicitly bound outputs are exported. The dry plan and runner both reconsider cached nodes when their selected output set changes. The implicit artifact input can use the node's pinned document or genesis. The implicit artifact output keeps the complete document archive and uses its declared port type when the wire contains only the schema.

## Verification

- The language-neutral rule vector passed an independent SQLite oracle for the created node, value and edge endpoints.
- The ten Rust sources edited for the engine and runner passed the nightly Rust syntax check.
- The semantic Rust suite covers preview replay, dotted kinds, typed assignment, merge idempotence, JSON scene reload, and rejection of partial patterns or a predicate containing another clause.
- The runner regression `workflow_transfers_selected_media_using_guest_port_names` checks delivery of the intrinsic invocation and distinguishes connected, unconnected and explicitly bound outputs. The existing archive carrier law covers the document output's declared media type.
- The targeted repository Cargo test waited for disk headroom and was cancelled in favor of the consolidated native build.
