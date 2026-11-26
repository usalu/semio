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


## BRep guest CPU and empty reactor slots

Real circle requests needed four or five actor turns per compact 16-unit response. A second Chromium run profiled the matching unoptimized core. Its executable code section was compared by SHA-256 to the archived core carrying the Rust name section. The active worker's hottest sampled leaves were the response slot scanner (8,734 samples) and the retained extension invocation slot scanner (1,281). The other three workers were idle for their samples. Both paths moved every candidate using `Option::take` when searching 1,024 vacant or sparsely occupied slots.

Both paths now inspect occupancy and take only the selected owner. Polling a never-used table leaves it unallocated. The six language-neutral ordering cases passed the independent SQLite query. Two native Rust slot tests passed, including wraparound and a never-materialised table. In a focused V8 run of a standalone Rust Wasm build with 16 KiB payloads, the old scanner took 22.76–31.67 ms for sixteen calls. The new scanner took 0.47–0.66 ms for the same selections. Those timings cover the reproducer. Full component and editor acceptance require the newly built guest.

Cancellation at request ingress now prevents the first guest turn when already cancelled, and prevents a queued resume after the progress callback cancels. The independent three-case SQLite fixture and all 22 renderer inbound-request tests passed. In an isolated installation, a known Math component was transpiled, asked for its guest descriptor, staged with that descriptor, and then activated. The descriptor matched the published pack and JSON byte for byte. The development installer prepares files before switching directories and computes the activation digest from the staged bytes.

## Live schema admission and cancellation

The installed Math guest admitted arithmetic and schema-generated vector construction as immediate operators. The retained SDK rejected that admission. A direct Chromium call on that build did not return the sum before its deadline. The original BRep circle ran for 185 compact replies across 30 minutes. It passed input processing and output retirement, then remained in measurement when the probe ended. Neither run verified an end-user mesh.

The registry now compiles each generated schema into immutable field descriptors and defaults. Its live job advances through the original input, optional edits, validation, field outputs and instance publication. Partial maps and cancellation retain typed ownership. A native metadata replay compiled the current neural model and registry together. SQLite checks cover vector construction, decomposition, modification, invalid inputs and a composite schema. The separate arithmetic implementation has a 28-case mathjs result oracle.

The actor now checks cancellation before submitting each request turn. Both renderer bridges also check before activating an extension or serializing its request. The cancellation and loader suites passed 23 tests. Across the broader three suites, 77 tests passed. The browser geometry probe now asserts both 2 + 3 = 5 and a constructed vector of (0, 0, 6) before reporting Math success.

The native registry suite subsequently passed all 13 tests. It caught a success-path ownership bug on the first run. A dictionary-entry producer must finish a paid close even after it hands over its result. The corrected job preserves that producer until closure. Tests cancel during both input application and output publication. The composite-schema test compares text, nested dictionaries, and lists with the cold public registry.

The served Flow core now has SHA-256 `943d7fb905d0f1707bd6af8f97ed49cef4da6f497bcac4c7fb817c5cf1176363`. Its command, progress, status, selection and session-close operations completed with deadline-limited turns. The host and consumed-clock laws passed against the rebuilt bytes. The renderer graph-host suite that previously lacked this package passed all 26 tests.

BRep and its spatial session now consume the Flow geometry, mesh and extension crates directly. Full Flow host simulation is a spatial-session test dependency. Cargo's normal WASI dependency tree for the actual BRep component contains 632 distinct packages and includes the extension SDK, with zero instances of the editor facade or host-session crate.

The Flow ownership contract now checks the renderer test configuration's actual base directory and both relative runtime entries. The former assertion expected one joined source literal. The repository test passed for five owners, twenty consumers, forty-five build inputs and the SQLite and TypeScript checks.

Compiler writes exhausted local storage during parallel builds. After stopping obsolete ticket builds and retiring regenerable caches, the system compacted an idle generated compiler image and remounted it at its previous path. Its allocated storage dropped from 19,178,532,864 to 13,548,703,744 bytes. The mounted data directory was checked after reattachment.

## Math guest terminal delivery

Chromium produced a complete 289-byte Pack response for the rebuilt Math guest’s vector operation. Its decoded value was [0, 0, 6]. The original addition run failed on continuation round 13 after retiring the original retained owner. The extension SDK now keeps the terminal response in its original invocation while the close operation consumes its budget. A rebuilt candidate reproduced the vector result in 104.267 seconds.

The rebuilt guest completed addition at round 12 and returned a 125-byte Pack with sum 5 after 87.023 seconds. Both development renderers returned identical JavaScript and all three verified WASM cores over HTTP. Their installed package digest is `26a789546b0edb8207c22a1c0bcc6a16493f745a8ec3890bd1e1ff5590aa87fa`.

## BRep browser component

Both development hosts served the same seven verified artifacts, including the 136,317,728-byte executable core, its descriptor and the browser bridge. The package digest is `ebb09981f587244912a31bf565aa1a94b0c632baff9ac763c09212a899c176ea`. In Chromium the radius-two circle completed in 33.359 seconds at round 20, returned a curve handle, and supplied it directly to solid extrusion. The tessellation owner passed its two repository contract checks.

The browser extrusion then completed in 34.957 seconds. The first tessellation invocation returned 60 indexed triangles from 182 vertices. Measured bounds ran from [-1.9890438318252563, -1.9562952518463135, 0] to [1.9890438318252563, 2, 3]. The signed-triangle volume magnitude was 36.606298, against the analytic value 12π.

## Editor contribution delivery

The React shell reported a failed live extension command to the console and returned success to its publisher. The publisher then cached that undelivered package as installed. The shell now visits all eligible receivers, reports their individual errors and returns the first live delivery failure so a later refresh can retry. Guest retirement remains a normal end of delivery. The regression failed with the original shell and passed after the change. Twenty contribution-publisher and window-fault tests passed together.

Overlapping refreshes now await the same in-flight installation. Both callers observe a failed delivery and the next refresh can retry. Different packages are delivered to a given instance in request order, including after failure. The two suites passed all 22 tests with both overlap regressions.

A durable local installation was also verified using the repository’s artifact publisher and its normal cross-process publication lease. Publishing the current procedural package into an isolated development directory returned the same file-set digest before and after publication. The local preview server can serve that installed directory independently of the ticket build outputs.

## Authored Mushroom geometry

Chromium evaluated the authored regular hexagon and Math vector through both installed plugins. For heights 6 and 8 each solid yielded 36 vertices and 20 indexed triangles. The measured volumes were 3.897114247 and 5.196152329, respectively. Both runs preserved the footprint and ground plane. The detailed measurements and inspector traces are in the browser geometry report. Public editor acceptance still requires the requester to deliver those results into its own preview.

## Development host and browser bindings, November 26

The React renderer passed its full TypeScript check after the Rust shell registry regenerated its 28 type exports. React and WGPU native launchers now read the locally installed procedural component from their own development runtime at startup. An HTTP test exercised both launchers’ middleware with a valid WASM header, GET and HEAD responses, file inventory, unsupported methods and delegation of ordinary application requests.

The rebuilt spatial browser session passed its binding and retirement tests. Chromium created a box of volume 24 and released 159,812 bytes across 3,546 bounded retirement turns before closing the session. Its WASM SHA-256 was `f4de936ad40f1984afaed29f0b3e4669a4e028959a6306825db4774ebcaa82aa`.

A five-minute WGPU run painted the canvas and located the Add Generation control. The browser sent a physical click, and the worker accepted its input batch. The generation count did not advance in the following 50 seconds. A probe that moved the pointer every 200 ms repeatedly invalidated frames before they could appear. The lower-frequency run measured the renderer without that interference.

The native geometry service spent its first work turns constructing a failure-publication frame before any work had failed. On the single-item contract grant it then tried to retire that frame on every request. An idle failure check now preserves the caller’s budget, and the geometry field identifies its retained snapshot across turns. The native fixture rerun tracks the next actual inference result.

## Native geometry service follow-up

The four-widget native request initially consumed 100,000 caller turns without admission. The engine now inspects whether it actually owns a failure before allocating the failure handoff. The idle-service native regression passed. The field now validates its retained document across turns, so building a fresh borrowed geometry view around that document preserves the initiating source. The eight source-lifetime checks passed with 79 assertions.

A subsequent native run completed its scalar widget and then reached the first solid producer. The input constructor now funds native map insertion and retirement through the initiating source effect and charges the work it performs. Its repository contract passed. The native checkpoint test received an explicit unsupported-producer refusal on the pre-primitive object code. The native malformed-request test also passed, covering a foreign checkpoint and a budget below the cost of initial source admission. Partial publication now records measured retained work and waits for an actual widget result before exposing a checkpoint. The scalar-only terminal case and the full primitive run have separate native probes.

The browser journey runner now exits unsuccessfully when a requested control is missing. Earlier runs already recorded these rows as missing in JSON, but could exit with success if every other requested row passed.

## Geometry service and preview output

The native service now completes the four-widget graph. It evaluates a scalar, a box, a sphere, and a preview that receives the box's shape. The box and preview report the same solid content hash. An independent test also evaluates two slider connections that publish the same port and verifies that the later value reaches the preview. The equivalent Graphlib fixture passed with the other eight original-field checks. A scalar-only service call and its owner cleanup passed.

The native negative-radius example now finishes with a widget diagnostic at the radius port in English and German. The committed service fixture contains the observed request, a partial checkpoint, the complete result and the invalid result. Its seven independent schema and consistency tests passed with 31 assertions. Native coverage also passed for idle failure handling, foreign checkpoints and a caller that cannot fund admission. These observations are separate from the regular browser's draw and edit acceptance.

## Current requester component and native oracles

The current requester reached the pinned WASI component encoder after all editor and viewer applications compiled. The debug-linked core contained 1,531,412 defined functions and the encoder rejected it at its one-million-function ceiling. The complete input was 1,042,791,572 bytes. Two passes through the pinned native WASM parser and encoder folded byte-identical functions with identical signatures, remapping direct calls. The transform preserves the identities of functions exposed through exports, tables and reference instructions. An independent WebAssembly runtime test checked return values and the distinct table references of identical bodies before and after folding.

The compact core has 945,784 defined functions, occupies 324,634,192 bytes and has SHA-256 `04a23725facb9cc4380435e34706dcf3c9534421ad93069cc2ced5bc4827a6cd`. Its six component-interface section names and contents and all 106 exported names and kinds matched the unoptimized input. The original pinned component encoder accepted these bytes and produced a 324,650,559-byte component with SHA-256 `168b42e1bbca11af063e8c4b185c2232f335f07a8ef8f6b0b1b101b77c0e866e`.

The native Parry comparison passed for circle, extrusion, the authored mushroom, mesh box and mesh extrusion, including single-unit progression. An expanded retirement observation measured 96,286 funded close turns. The primitive owner also passed its independent cancellation, physical-allocation receipt, indexed bounds and volume tests.

## Browser request intake and native geometry on November 26

A clean React run on the freshly linked procedural requester advanced Mushroom to four of seven widgets. The first installed Math call returned the original 87-byte progress Pack. Chromium delivered request 1, then the run ended after 738 seconds with one outstanding invocation and zero vertices. A second Chromium run sampled the actual reactor worker across that delivery. Following receipt, the existing response owner repeatedly validated its original loan for roughly 100 seconds. The worker never sampled the Completed body decoder or response-action dispatcher. The WIT bridge was passing a local optional event to the kernel. On first admission the kernel reserved the response slot and left the original event in that option, which the exported poll then discarded. The bridge now retains that exact event between crossings, drains queued arrivals in host order, and keeps the actor runnable while an input remains. Two native ownership tests and an independent six-crossing FIFO replay passed. A new application WASI build is in progress.

A separate native replay decoded the captured browser Pack, including the original 64-bit node hash 17647890481751977919, and drove the actual Flow receiving owner. After 104 single-item turns, its stored progress was 256 of 257, phase computing, and the addressed window had zero outstanding requests. The receiver, source decoder, caller and session retired successfully. The native service reproduced the full committed success, partial and bilingual failure fixtures. Changing a saved generation changed the connected box and its preview while preserving the independent sphere. A full service sweep also identified four outstanding tests covering bounded steps, cancellation and inference caching. The original field driver currently bypasses its optional cache.

The Math native suite passed all 12 tests, including all 28 operator fixtures, cancellation and the installed capability's complete Pack. Chromium ran the current compiled component through vector construction, sparse sum, empty sum and sparse variadic addition. The outputs were [0,0,6], 3, 0 and 3. React and WGPU both serve the same 96,533,927-byte stripped Math core, SHA-256 749664be71b250d7a6c51ec292dfb2f25c898626c47f9d3a4e94d5e88af9724b. A native probe of 25,000 sparse reactor scans fell from 2,904 milliseconds to 33 milliseconds after changing traversal to use the existing occupancy bitmap.

The prototype profile now selects size optimization for the procedural browser component. Its regression fixture failed before the change and passed afterward. All 27 Cargo profile consumption tests passed, including both TOML parsers and 1,599 assertions. Sampling the older fleet build found Cargo blocked writing to the driver's diagnostic stream for several hours with four unreaped compiler wrapper children. The private native fallback uses the repository profile renderer's current artifact selectors and regular output files. Its Cargo command selects size optimization, 16 codegen units, disabled incremental compilation and the configured prototype assertions. A separate warm build has already compiled the repaired WIT bridge and context for wasm32-wasip2.

The geometry service now distinguishes a new request for the same document from a resumed published checkpoint. It returns a named cancellation error to an attempted continuation, and a new request can restart from its original caller. On the pre-cache build, 11 of 13 service tests passed. The two remaining cases both measured zero real incremental hits. The geometry owner now quotes and drives its request, pending compute, values, snapshot, failure and original caller closure. Native tests exercised those quotes with single-item grants and checked the effects against the actual receipts on every turn. Cache final closure and the browser editor verification continue on the current source.

All 332 tests across the 46 Generation3D TypeScript test modules now pass, with 5,765 assertions. The native typed inference cache separately passed cold, warm, edited and repeated graph runs with one retained item per crossing, exact allocation receipts and funded eviction. Python graphlib independently reproduced the four counts and evaluation sequences.

The Chromium SetContributions profile charged 6.1 seconds of self time to borrowed JSON character decoding. The decoder read the first source byte twice, including every ASCII byte. It now returns ASCII directly and fetches only continuations for multibyte text. A debug native replay of 800,000 ASCII characters reduced source reads from 1,600,000 to 800,000, and elapsed time from 103 ms to 41 ms. Both versions decoded all 1,112,064 Unicode scalars. These measurements cover the decoder, not the complete browser command.
