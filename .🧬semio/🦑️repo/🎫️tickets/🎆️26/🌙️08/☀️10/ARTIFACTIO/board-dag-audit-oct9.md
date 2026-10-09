# Current Board and DAG Receiving Audit

2026-10-09. Read-only source audit; no runtime tests executed, no native/kernel floor rerun. Applicable root, s, DAG, Flow, OS and Infinite instructions read. Existing ticket reused; repo MCP lifecycle unavailable to this auditor. Concurrent root hover edits are expressly in flight. The hashes below identify what was inspected, not a future completion snapshot.

## Concrete Findings

1. **Flow one-shot authority is not continuation authority.** `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:1609–1615` creates a new NativeDecodeControl/NativeEncodeControl each invocation, compares cumulative owned bytes to one poll byte_credit, and captures cancellation/interruption/time as immutable budget values. Domain operations 2515/2520/2523/2526/2529 call these synchronous wrappers; e.g. selection emission at line 1882. Work cannot yield inside the codec, cancellation cannot change inside the invocation, and ownership refusal maps to NoCredit then `finish_domain` (530–546) seals rather than retaining admitted progress for another grant. This is narrower than the retained selection cursor producer at IO selection `step` line 122; that retained producer does not justify these wrappers. This is a source finding, not a runtime failure claim.

2. **Unadmitted source projection before controlled emission.** Selection emission calls `domain.host.selection_domains()` inside the closure before the controlled encoder sees the resulting record (Wasm line 1882); channel emission does the analogous `selected_channels()` at line 1999. DAG pure host (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`) `selection_domains` at 2900 allocates channel vector, formatted handle strings, node IDs and edge IDs; `selected_channels` at 2981 collects owned refs. The encode control charges subsequent physical backing but does not authorize those semantic projection allocations. Owned facts alone are not accidental physical IO; this is specifically a host receiving ownership gap. Prefer borrowed source APIs or a controlled facts projection if these operations are to satisfy complete owner accounting.

3. **Hover physical methods still present in the inspected version; root repair pending.** DAG pure host (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`) lines 2994–3026 builds JSON via pack_json and format in `wire_type_refusal_json` and `hovered_channel_json`; Flow host line 1904 delegates physical text. This currently violates pure-host physical separation, but root is actively replacing it with typed DagHoverFacts. Do not report that pending change as fixed or duplicate the production edit. Additional physical pick/geometry projections exist in Flow host 1893–1900/1919 and DAG following 3035; determine whether those are explicitly outside this audit boundary before calling the entire host clean.

4. **Computing-progress receiver remains outside typed status admission.** Immediately before dag_input_decode, Wasm around 1590 admits JSON with serde_json::from_str, projects active via as_str and stale via filter_map, defaults malformed/missing stale to empty and mutates `set_computing_progress`. This is a concrete whole receiving hole adjacent to typed set_node_statuses; a malformed stale element is silently discarded. Selection/channel/status strict IO does not cover it.

## Narrow Positive Source Evidence

Board snapshot physical JSON is in IO snapshot line 4 with Reject member policy. Nested Rust snapshot records declare deny_unknown_fields and explicit default Option fields; TypeScript counterparts expose optional nullable fields, so optional null is intentional model admission rather than parser tolerance. Layout IO separately uses Defaulted<T> for missing defaulted non-null fields, preserving refusal of explicit null; centerX/centerY stay Option. Redraw admits both source and options before semantic pipeline. Snapshot/layout facts declarations inspected do not contain executable physical JSON calls (a layout docstring mentions an old call). This supports these exact local boundaries only.

DAG IO has complete closed-record checks: nodes/edges/handles all required; channel records exactly widgetId/port/direction; direction in/out only; status variants exactly their declared fields; all rows checked before typed model is returned. IDs/ports are nonempty. No evidence here proves host mutation rollback or fullhost availability. No fullhost result asserted: shared Kernel floor remains held according to parent coordination.

## Source Manifest

Paths below are repository relative. `BOARD` = `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/`; `FLOW` = `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/`.

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🧬️schema/🎯️dag-input/🦀️.rs`: SHA256 `e881a922aa54c61971f377341c5d81264fc5a344788f12ab4437d279bb159184` (2085 bytes).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🦀️.rs`: SHA256 `2489aa6275d406cdd15f8477167aecbbbfe157b9e03ce47017072a292a3d51e2` (3916 bytes).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/📤️selection/🦀️.rs`: SHA256 `7f526896eb3f6b297a945ec5d454fbb841f95e0b1dbe8c54fa185383c9555c5b` (9199 bytes).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`: SHA256 `03a78baa19a5796fcc702af197e8f3d0fd5431d85f43edfcca41c29707e99682` (313160 bytes).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/📸️snapshot/🦀️.rs`: SHA256 `716da2114f621d380fc57fa245dc893930d5770af04cfdc76d26df535d869d8a` (8040 bytes).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/📸️snapshot/🟦️.ts`: SHA256 `3d43bdb271d4d1a255b7f40af86d4d987a23a58b9beec6ddfb744c387bad5a87` (2016 bytes).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/📐️layout/🦀️.rs`: SHA256 `77498926e0e7aee451d8a55c07bf02c6cc15fad581bb1ef32bbca217a2802e4e` (8816 bytes).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`: SHA256 `4891cd246ac1fc8aef1c48b8e32b245499143ec495d802eb1eac871be974b46a` (281078 bytes).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs`: SHA256 `f08b7eadc1a7c057a2c0e663e64387146de6bff008572ab707b95ad53650c158` (287228 bytes).
