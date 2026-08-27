# Owned Renderer Graph Validation

The owned graph cursor captures the exact immutable node index before beginning. Every lookup retains a node handle until its value has been read, then retires that handle while the captured index remains the lifetime anchor. Cancellation clears explicit graph frames, sibling-key owners, visited/violation tables, active node/read owners and finally the source index. A returned violation table is an owned result, not publication authority.

The existing retained patch cursor and the new owned validator now call one graph traversal implementation. It preserves depth-first child order, cycle/section/numeric/depth checks, orphan and duplicate-key reporting, and insertion-ordered dangling-root reporting. Hash collisions use exact incremental string comparisons; there is no native string Set admission in the retained validator.

Strict language-neutral fixtures contain eight cases: valid safe-53-bit IDs, duplicate Unicode key, orphan, cycle, depth quota, missing root, node quota and empty graph. Tests compare exact ordered violations against the existing reference validator with an Immer-produced Map snapshot. The valid graph is cancelled at every observed step (more than 100 prefixes); every close is checked against one item/4096 bytes. A captured surface byte view survives source and validator retirement until its independent node reader closes.

Actual test history:

- R1: wrong Nx project spelling; no test ran.
- R2: expected missing owned-validation module during collection; four failed suites, no behavioral test executed.
- R3: one executable test failed because the test-only Immer MapSet plugin was not enabled. The oracle now uses Immer on the entry array, then constructs the reference Map; production code was unchanged for that repair.
- R4: canonical `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedValidation'` passes 1 test, 530 skipped, 531 total, five files, 24.37 seconds total and 3.95 seconds test time.
- Full R1: canonical same target with `--args='--run'` passes all 531 tests across five files; 48.74 seconds total and 51.32 seconds aggregate test time. This includes the previous retained semantic/rejection corpus after extracting the shared validator.
- Strict typecheck R1: eight diagnostics, comprising seven known tutorial producer joins and one newly authored fixture missing nullable text defaults. The fixture was fixed; a fresh strict rerun is not yet recorded here.
- Strict typecheck R2: exactly seven known tutorial producer joins; no owned validation, numeric, operation or fixture diagnostics remain.
- Targeted `git diff --check` passes for numeric, retained UI and UiDocumentStore paths.

All complete outputs are retained as `🧪️renderer-owned-validation-{red-r1,red-r2,r3,r4,full-r1,typecheck-r1}-2026-08-27.txt`. These are private preparation/read-lifetime tests. The production React and wgpu wire/ACK paths are not yet mounted on this owned pipeline; tree/hash, notification and per-instance aggregate close remain required. Logical work grants are not browser allocation/GC or outer-poll latency certification.
