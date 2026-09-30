# Wgpu Renderer React Parity Coordination

The active goal is full visual and behavioral parity with React. The coordinator uses GPT 6.1 Sol with extra-high reasoning, implementation agents use high reasoning, and read-only auditors use low reasoning, the available equivalent of the requested Light setting. Four concurrent slots include the coordinator; audit and execution waves keep the three worker slots occupied.

The repository MCP is not exposed as a callable tool in this chat. The configured `bun ./📜️script.ts dev mcp stdio codex` server was launched locally and accessed using its JSON-RPC protocol. `resources/read` for `repo://goals` succeeded. The existing ticket belongs to Running Framework / Running Products / Running OS / Running S. `ticket_reopen` returned `ticket is already open`, confirming that this ticket remains active; no duplicate ticket was created.

Initial responsibilities:

- React auditor: concrete component, layout, window, and interaction reference coverage.
- Wgpu auditor: effective runtime routing, parity harnesses, and remaining demonstrable defects.
- Window implementer: dock tree transformations, window identity, drag/drop, and maximize behavior.
- Coordinator: integration, acceptance coverage, runtime verification, and further assignments.

All agents preserve concurrent work and avoid modifying Git commands and worktrees. Generated output stays in this ticket's `🗑️generated` directory. Existing reports are reference evidence, not proof of the current checkout's behavior. Completion requires freshly executed tests and runtime parity evidence.

## Current Execution

The two initial source audits are complete and saved as `📓️react-current-parity-routing-audit.md` and `📓️sol-runtime-routing-audit-29.md`. They establish current routing and invalidate several historical missing-feature claims; they do not establish runtime parity.

Both vacated worker slots now execute browser media parity: one owns the shared media lifecycle, channel adapter, worker transport, and DOM host; the other owns the Rust accepted-slot collector, shell/snapshot publication, fixture validation, and native accessibility.

The window implementer captured a React fail-first result of eight failures and two passes before repairing drag destinations. Current fixture coverage also includes reachable tab-drag rejection of stale destinations, rather than relying only on whole-stack API tests.

The coordinator added a permanent `test-browser` Nx task and launch configuration so all WGPU Vitest laws can run independently of native Cargo compilation. The four shared browser media modules are registered in both sorted generator ownership arrays.

Fresh execution in progress:

- Native renderer library baseline: `@semio-tech/framework-renderer-wgpu:test-wgpu-unit`; dependency compilation remains in progress.
- Browser renderer baseline: `@semio-tech/framework-renderer-wgpu:test-browser`; generator prerequisites remain in progress.
- Independent React and WGPU listeners on ports 7300 and 7301 consume staged artifacts without launching duplicate activation graphs.

The in-app browser inspected the staged WGPU baseline on port 7301. It faults before the window system becomes available: `plugin-parse: plugin puzzle: manifest parse: missing field name at line 1 column 74646`. The page reports worker termination and refusal of input. This proves a boot failure for the staged renderer artifact, not a defect diagnosis of the current Rust source. A freshly compiled and published renderer is required before scoring interactive parity.

## First Executed Gates

The complete browser baseline ran 48 suites: 38 passed and 10 failed; 455 assertions passed and five failed. Six suites failed transformation while the media lifecycle move still had an old relative import; that import is now repaired. Two generator laws ran against in-flight media module writes. The three remaining failures expose a stale tree-row source guard (the row now also carries a leading virtualization offset), a stale document-terminal source guard (bounded paint opportunity checks now share one helper), and a timing-sensitive no-overrun assertion. The coordinator repaired these guards without removing their geometry/terminal requirements and made the fallback-state test use a deterministic clock; separate executing-time overrun tests retain real clocks.

The native baseline spent 8m59s compiling dependencies, then stopped before assertions because the newly mounted media-slot test file had not yet been written. That file now exists. Replacement native and browser runs were refused before assertions by a shared Nx graph failure. The window agent traced the primary failure to an existing co-located Ajv test import reached by the conservative browser input walker; it is correcting the source-input declaration, rather than relaxing graph enforcement. Follow-on missing-producer and test-host diagnostics will be rechecked after that repair.

The canonical media descriptor schema now lives under WGPU `🎬️media-slots/🧬️contract/🔣️.json`; the generator input authority uses that shared owner. Slot bounds, explicit locale, resource identity, cancellation, and native capability presentation are covered by newly authored neutral fixtures; execution results remain pending.

## Integration Graph Repair and Current Rebuild

The first integrated commands stopped during Nx graph construction. The browser source-input walker conservatively visits colocated `import.meta.vitest` imports, including the existing media window-kit Ajv oracle. That existing test dependency is now explicitly declared by the browser build contract. A second graph fault came from the browser authority assertion importing the channel adapter and thereby the whole Os runtime. The shared authority assertion now lives in the neutral lifecycle module; browser code uses it without reaching the channel codec. Workspace runtime aliases are not being declared external to hide that dependency.

Generator ownership now includes all four neutral media runtime modules, the segmented-download wrapper and canonical media window-kit source. The presented media descriptor schema is a generator input, not a runtime module. Generation, the full browser gate, native WGPU unit gate and a fresh Wasm build have been started through registered Nx targets after those changes; no passing outcome is claimed until each completes.

The spawned-app audit found a remaining owner-routing defect: the literal `spawned` dock surface can render through the active session controller, and its existing host entry does not carry a trusted parent-document identity. Execution agents are fixing its app routing and adding a lightweight trusted document-identity protocol, with fixture coverage, instead of accepting resource-provided authority or decoding a whole document archive each frame.

## Executed Integration Progress

The repaired graph constructed successfully. Browser boot and frame-worker generation succeeded through Nx after adding the newly transitive text-splice module to its schema-owned source closure. The full browser gate progressed to 49 passing files and 530 passing assertions; one real Chromium media journey failed because a play/pause interruption disposed the player. The media agent is fixing that real runtime race in both hosts.

The exact React dock destination gate passed fourteen tests. Its neutral fixture covers thirteen destinations; the companion spawned-app oracle passed fifteen tests with eight owner vectors. The new native spawned routing law is intentionally still red until its selected native execution proves the baseline failure.

The channel-version census passed with pin 20, thirty registered consumers and zero findings after ten derived/dependent fixtures were reconciled. Details and exact digests are retained in the channel V20 fixture report. Source fixture gates are running separately; the census does not validate binary component freshness.

The first current native and Wasm builds stopped on source snapshots taken during the identity protocol edit and a wrong test-only tree import; those are repaired and fresh builds have started. No native assertion pass is claimed yet. The React reference booted in the in-app browser on port 7300, presented Top and Perspective windows, and the tour Skip button dismissed the overlay; its error/warning console was empty at that checkpoint. The WGPU staged baseline remains faulted until fresh Wasm/component activation.

A parity review found transient occlusion must preserve the media player: dropping slots during a menu would cancel playback and reload exported bytes. The collector and page host are being changed to retain owned descriptors with explicit occlusion, hide covered controls, and preserve playback through hide/show.

## Current Integrated Gate Results

The complete browser gate passed fifty files and 536 tests in the current integrated snapshot. The new Wasm compile preflight succeeded through Cargo and Nx; this confirms source compilation, not a freshly activated browser binary. The nine-vector spawned routing oracle passed fifteen assertions; the genuine native red baseline previously executed and failed before the routing repair.

The full native green attempt compiled and ran 552 of 1571 selected tests: 550 passed and two media-slot assertions failed, after which nextest stopped the remaining tests. Those failures concern numeric fixture representation and oversized descriptor publication and are assigned to the media-slot owner; the full native gate is still pending. The trusted document identity kernel law executed successfully once; newly added malformed-payload cases still need a fresh run.

A real media-app acceptance command is now registered in the WGPU package router, Nx project, package scripts and editor launch configuration. Its current producer fixture deliberately checks the same unsupported media capability emitted by both React and WGPU app sources. Real WAV/MP4 playback is separately verified against the production browser host, without claiming the app producer already exports playback. A custom theme metric divergence is being fixed from a neutral schema and shared vectors by the React/theme and native-shell owners.
