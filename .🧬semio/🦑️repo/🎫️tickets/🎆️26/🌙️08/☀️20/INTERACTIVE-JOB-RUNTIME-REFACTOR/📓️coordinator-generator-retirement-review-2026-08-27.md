# Generator Shared Ownership Review

## Executed DOM Rerun

The coordinator executed `NX_DAEMON=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='🧪️index.test.ts --run -t "graph parameter|graph slider"'`: **7 passed, 0 failed, 347 skipped (354 total)** in 21.87 s (1.05 s test runtime). This covers the mounted FlowGraphCanvasHost small parameter payload, three app contexts, keyboard/drag/end/cancel behavior, stale overlay rejection, focus/labels/scoped IDs, and prior slider accessibility laws. The Wasm task boundary is mocked; all three backend registrations, authoritative edits, undo, and live Wasm remain separate gates. Log: `🧪️coordinator-graph-parameter-dom-2026-08-27.txt`.

The coordinator read the revised immutable generation wrapper and retirement cursor, plus the shared Flow retirement implementation and its authored tests. These are source reviews; their native tests remain queued behind the real CAD constructor build.

The generation wrapper keeps the existing serialized GenerationPlayState shape and shares the exact immutable Arc allocation. Cold mutation requires unique ownership through Arc::get_mut. The revised Drop only releases a shared alias or a provably empty final state; a nonempty final owner must enter explicit retirement. Cursor ownership is held in ManuallyDrop, so a terminal-guard violation does not recursively destroy nested JSON or cause a second panic during unwinding. Retirement walks owned arrays/maps and releases names, IDs, keys, and values by byte grants.

Shared Flow retirement covers all nine current Widget variants, all NodeChrome variants, fixture strings/widgets/synapses/layout, nested neural trees and dictionaries, GUI node/previews, and long text. The coordinator compared the match arms against the current owning type definitions; heap fields are transferred into the frontier, not discarded through a wildcard. Native laws author worker transfer, exact released-byte totals, zero grants, and live-owner guard/unwind behavior.

OrderedMap adoption remains separate. Current map/set retirement uses pop-first ownership without whole-key comparison; constructing destination maps is the remaining bounded-construction problem. The shared renderer parameter action affects Flow, Procedural2d, and Procedural3d, and all three need exact registered factories and runtime tests before this UI feature is complete.
