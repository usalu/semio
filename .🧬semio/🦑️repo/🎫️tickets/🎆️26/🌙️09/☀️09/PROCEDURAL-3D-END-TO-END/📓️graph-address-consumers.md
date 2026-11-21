# Graph Address Consumers

The Generation3d editor now uses the directional graph codec for component selections, graph navigation, preview marks and the gumball's output address. Authored widget IDs stay raw. Authored output channel names are decoded at the operator-catalogue and wire boundaries. Component suffixes and analytic annotations are parsed after the instance scope. The retained component parser applies the same channel rules with a bounded scanner and keeps the canonical channel code borrowed.

Keyboard traversal and Edit preview consume the actual selection granularity, so a widget and a port with the same text project to different nodes. Edit preview summaries expose the current interaction selection in `PreviewInteractionMarks.selection_granularity` and `PreviewInteractionMarks.selected`; `graph_selection_ids()` projects those marks onto widget IDs. Untyped hover uses exact authored widget and wire IDs before trying a canonical handle or instance. The explicit gumball API still carries strings. It accepts canonical preview instances, gives a bare existing widget priority against a port-shaped alias, and rejects directional input targets with the existing output fault.

## Verification

- The registered Bun component-selection file passed 83 tests with 174 assertions, including shared graph-port rows, the authored delimiter fixture and Ajv component-schema tests.
- A focused Rust harness compiled the live model codec and the real cold component and gumball address functions. Both tests passed across the shared fixtures, including invalid components and input refusal. The temporary harness and executable were removed.
- Registered Rust cases cover the bounded parser, keyboard granularity, preview selection marks, and gumball channel resolution. Their full Cargo workspace execution was left to the owning build lane.
