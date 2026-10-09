# Mounted Probe Physical Audit

Read-only source observation, 2026-10-09. No tests executed; no Native Probe qualification inferred. Paths below are relative to repository root. Concurrent caller migration and the known UTF8 enum correction are excluded.

## Remaining original-owner frontiers

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🚪️io/🔤️json/🦀️.rs:85–98`: synchronous parse/print own the actual recipient only as a stack local. A close error propagated by `?`, an oversized receipt refusal, or the finite loop exit returns while custody can remain pending. The successful `result` also leaves receiving-frame custody before this drain. Preserve the operation/recipient/result in an actual resumable owning object on denial, or establish and test a genuinely infallible close contract; another fresh grant/controller does not repair this boundary.
- `workspace/🪶️sqlite/🦀️.rs:23–26,40–72` under the same MCP prefix: semantic projection/reconstruction still creates row vectors, cloned strings, BTreeMap/BTreeSet nodes, sorted member vectors and output trees with ordinary allocation. These paths never call original SQL `admit_allocation_bytes`, `allocation_stage`, or `allocation_stage_native`. A caller with `max_allocation_bytes = 0` can therefore produce nonempty semantic objects despite the allocation ceiling. Native JSON hooks being controlled does not qualify these relational births or their partial error retirement.
- Same SQLite file `:9,59`: numeric TEXT goes through ordinary pack JSON `parse`, without `check_value_bytes(text.len())`. A physically supplied arbitrarily long numeric literal bypasses the numeric-column value ceiling and uses a separate parser owner. Apply the original column ceiling before lexical work and use a controlled exact-number producer with retained error custody. Projection numeric TEXT likewise lacks its value-byte check (`:23`).
- Same file `:61–66`: sorting all children is an uncheckpointed potentially expensive turn; object name-set insertion and recursive partial output are outside the original receiving frame. Cancellation checkpoints at node entry do not account these physical allocations or bound the sorting turn.

## Conserved behavior observed

- The direct JSON reader/writer records `normal_step_progress()` before propagating the cursor outcome (`json/🦀️.rs:44,70`), and preserves completed output in the installed tuple before validation/refusal. This is the proper direction for canceled performed prefixes.
- Current pack parser `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:949–956` now adds its mutable turn receipt before propagating `advance` failure; the earlier success-only receipt concern is superseded. Its Native error carries cumulative normal progress.
- SQL reconstruction enforces one root, exact row identity, existing parents, contiguous unique collection ordinals, array name absence, object name uniqueness, scalar payload shape, bounded depth and reachability (`sqlite/🦀️.rs:44–77`). Ordered objects are reconstructed by source ordinals rather than map-key order.
- Current Native test `sqlite/🧪️tests/🦀️.rs:6–32` pins neutral rows, exact Serde values, moved original object pointer, reconstruction and rejected raw wires. It currently uses default SQL limits and does not cover allocation denial, canceled relational partial owners, or synchronous close denial. Source runtime receipts supplied by Root remain separate evidence.

## Meaningful next laws

Use an original installed Native recipient and authored neutral five-axis policy: deny each close axis while preserving actual cursor/output identity, then resume the same owner; inject cancellation immediately after a performed parser/writer turn and verify its receipt. Independently deny SQL allocation before the first nonempty row/tree birth, test numeric-column value ceiling one-short, and cancel reconstruction after a real string/collection partial birth. These laws must exercise the production methods above, not just raw JSON registration.
