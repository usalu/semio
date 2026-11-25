# Procedural 3D editor and Flow bridge

## Browser host

A legal 1,048,576-byte catalogue request overflowed the JavaScript call stack before the host could send it. The frame writer now accumulates byte chunks and copies them into a typed array once. The focused test compared the full UTF-8 payload and its two little-endian lengths against Node Buffer. The regular host suite also exercised the maximum-size request and rejected the oversized request before consuming request credit.

The host suite failed before its first assertion because three fixture paths pointed outside the test tree. After correcting those paths it exposed three missing schema definitions. Restored the named contracts against their current fixtures, including late observation of a rejected session open. The mock-bridge host tests passed, covering cancellation, surface lifecycle, event acknowledgements, startup, two independent sessions and terminal cleanup.

The compiled-session and clock tests now open the browser package selected by the build. The initial compiled test on the previously published package failed with DeadlineExceeded. A separate bridge probe opened a session, attached a canvas and acknowledged its creation, then reproduced a failure on setSelection.

## Product verification

The React editor and WGPU test results are recorded separately in this ticket. The preview is still under investigation. Full-size request framing and mock-host tests alone do not establish a working preview.

## Scene synchronization and retirement

The original native suite reproduced four failures. Opening an input’s retirement frame required 55,192 copied bytes on this target, above the browser’s previous 4,096-byte grant. Using compiled WebAssembly and one retained item per poll, 16 KiB and 64 KiB allowed the input to answer but left session retirement stalled. The schema and browser now use the existing 1 MiB request bound for the retained copy currency. The transport still spends at most 4,096 work units per poll. All four native canvas laws passed, including a deliberately underfunded caller that observes NoCredit, increases its retirement grant and receives the original refusal. The JavaScript host suite also checks the actual grant against the ABI schema.

A real wall clock exposed a second failure at setSelection. A poll could admit work, exhaust its eight milliseconds and terminate the entire operation. The observer now yields at the boundary between work units. Cancellation and interruption still reach the native cursor, and a budget already expired upon admission still fails. Verification of the rebuilt WebAssembly is pending.

The host’s local regenerated build caches from the previous day and an unused November 12 search index were removed to provide native compiler headroom. No project sources were included in that cleanup.

## Node graph interaction

The React renderer test configuration pointed at a generated Flow initializer in the source tree. The node graph suite failed during module resolution before collecting a test. Its Vitest configuration now uses the same Flow source-delivery plugin as the development application. This selects the authored JavaScript host for injected bridges and keeps compiled-Wasm verification in the publication suite.

The recovered suite exposed a handle whose node and port names both support the at sign. The native pick producer now uses the directed graph’s existing endpoint encoder, and the React decoder reverses the percent and at-sign escapes only on the port side. The fixture covers namespace-qualified node IDs and escaped port IDs. The graph parameter test session now implements the real asynchronous close contract. All 26 renderer host tests passed, including slider keyboard and pointer edits, hover, graph picking and session cleanup.

The staged shard worker matched the current generator output byte for byte, including its per-actor drive interruption logic.

The Flow extension WebAssembly TypeScript suite passed all 34 tests against the current shared source. This rerun supersedes the earlier 24/34 result for that file.

## Procedural contract suite

A direct run across 46 Generation3D TypeScript test files found 318 passing tests and 11 failures. The graph hover DOM oracle treated a graph handle prefix as part of the node name and could not express an edge. It now checks all three identities against the same fixture the Rust projection uses. Three other checks referenced retired Rust implementation details for map insertion, shared evaluation custody and translated mesh retirement. Their updated assertions track the current paid insertion, borrowed key and owned clone paths. All 12 tests in those four files passed on rerun.

The seven remaining failures all read the geometry service's committed three-byte empty JSON object. The Rust contract now compares the exact typed request and all three actual service responses, including the stepped and faulted cases it already computed. Regeneration and verification of the shared fixture await that native run.

## React requester continuation

A live extension reply left the procedural actor reporting more work after the bounded interactive settle. The host returned frames at that point without transferring pending work to the existing per-instance operation driver. A two-case integration fixture exercises an idle and a still-working requester. The working case failed before the change, while the idle case passed. After the handoff, all 32 extension invocation tests passed. The browser still needs to verify solid geometry with the rebuilt requester and Flow component.

The React settle loop could also return while its own deferred ingress was still draining. The new 131-event fixture simulates an idle actor consuming one queued event per crossing and publishing its receipt on the last. The previous implementation returned without the receipt after 128 quiet crossings. Settlement now counts a shrinking queue as progress, yields between batches, and waits for the queue before handing off. Genuine stalls still reach the original guard. All 147 PluginRuntime suite tests passed.

The latest procedural dependency graph includes the concurrently introduced Flow authoring reader. The earlier native Cargo invocation held the old Flow host unit and lacked a lock for its new dependency. The replacement invocation uses the current graph. Another Flow Rust compiler received SIGTERM. Its cached build resumed with a single compiler job. The ticket compiler retry helper preserves ordinary compiler failures and retries a child SIGTERM at most twice while its original parent remains live.

A combined rerun passed all 179 React runtime and extension invocation tests. The deferred count now follows the newest accepted receipt in constant time. The compiler-input fallback captured 167,047 workspace paths into a private copy outside the ticket. Cargo resolved the frozen procedural workspace, including the Flow authoring reader and the intrinsic Pack body receiver. The fleet scheduler reported 14 connected workers, including Darwin ARM64 workers with headroom. A distributed Cargo request is running alongside the local build.

## Interactive queue and compiled BRep timing

The standing React operation drain submitted its worker polls in the user-visible lane while occupying the interactive ingress lane. A test with a busy actor established that a queued drain ran ahead of a waiting edit. The drain now uses the user-visible lane at both boundaries, so the queued edit runs first. The combined React suites passed all 180 tests.

An isolated Chromium capability probe used the installed BRep module, real shard client and actual Wasm response bytes. The circle advanced through input parsing, ordering and canonicalization, returning its 38th progress envelope at 357 seconds. It had not published a curve at the six-minute probe deadline. This establishes progression and the cost of externally resuming every 16 work units. Builds of the revised SDK cadence are running for both BRep and Math. This probe does not establish a visible preview.

The isolated browser packaging path was tested on the published 197,451,281-byte procedural WASI component. The regenerated 289,649-byte semantic descriptor matched the published descriptor byte for byte, as did its JSON projection. All three component bridge and worker-loading tests passed (37 assertions). The preview uses the descriptor’s component table and probes the actual guest before serving. The new requester build is measured separately.
