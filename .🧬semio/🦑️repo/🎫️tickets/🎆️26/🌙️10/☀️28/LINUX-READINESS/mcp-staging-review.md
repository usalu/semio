# OS-MCP Pre-Frontdoor Staging Independent Review

Ownership: Course Medium backend owns implementation. This reviewer made no OS-MCP launcher Source edit and started no additional OS-MCP gateway or native build. TUI coordination with Course coordinator thread 01a18330-85d7-7681-97b1-bbed4f0f08dd is restored.

## Verified Baseline

OS 🌉️mcp/🟦️.ts stageMcpBinaryAsync unconditionally calls registered startToolV1(repo, run @semio-tech/framework-os-mcp-rs:build --output=stream) for each call. serveWarmMcpStdio passes its gateway signal separately. mcpStagingEnvironment reads process.env and adds a per-process CARGO_BUILD_OWNER_PID; that PID is provenance, not product identity. The passed gateway env is not the env argument of stageMcpBinaryAsync. Source/product identity must match the actual environment/recipe the producer consumes.

The existing staging neutral tests currently fail before attesting actual tool ports: 1 pass / 3 fail / 11 assertions. They extract current functions with TypeScript AST but inject old spawn/spawnSync ports, leaving current startToolV1/runToolSyncV1 undefined. This stale-test failure is separate from duplicate producer behavior and was relayed to the implementation owner.

The handcrafted ticket input mcp-stage-coalescing-red.ts plus neutral JSON demonstrates the actual duplicate dispatch through recording ports: two concurrent identical-root calls to the extracted current staging function produce two registered-tool start attempts, versus one independently computed SQLite semantic-fixture group. Result: 0 pass / 1 fail / 4 assertions (expected1, observed2). No native process was launched. The capture/product labels in this corpus are neutral DATA, not issued Native authority; this red proves dispatch duplication only.

## Existing Authority Boundary

The process Native artifact cache producer interface is genuine cross-process exclusive writer custody, but it is tied to its captured control/Original. It is not an independent producer service or a storage reservation. Its 64-hex context validation proves shape and the lease proof proves writer ownership; neither alone proves actual Source capture/product semantics. The caller must derive the admitted identity and retain the genuine cache volume/Store and producer owner.

A process-local promise map cannot coalesce separate gateways. A coarse argv hash cannot distinguish changed Source/capture, profile, actual target triple/compiler configuration, product or namespace. A waiter signal cannot become the shared producer's lifetime when other waiters still need it. Likewise producer output pipes cannot depend only on a gateway that may disappear.

## Required Review Cases

The implementation's schema-derived neutral corpus and real acceptance should distinguish:
- Same admitted Source/capture/product/recipe under the same actual root and allocation joins exactly one producer before expensive frontdoor admission.
- Different Source capture, product/profile/recipe or actual root/allocation remains distinct even when argv matches.
- Publication and capture identity are rechecked after waiting; a stale Source/product cannot be returned merely because a prior record or command looked equivalent.
- Each waiter keeps its own real Original/control, progress and cancellation. Cancelling one waiter does not abort another's needed producer or transfer writer authority.
- All-waiter cancellation follows the declared independent producer policy and drains accepted cleanup under its actual owner; it does not leave invented leases or unsigned PID markers.
- Lost producer/Store custody, failed or stale publication and reboot loss refuse or reacquire through supported interfaces, without default grants, manual process signals, budget relaxation or fake Original objects.
- Sync staging callers and warm async gateways cannot silently create competing producers for the same product if both are supported entry routes.

No new coalescer API, schema or producer issuer has yet been received by this reviewer. Implementation acceptance remains pending; no repair green is claimed.

## Reboot Revalidation

At 2026-10-29T23:58:17Z boot11220615-7932-4930-ae1a-7d7fd56ccf52 was verified (uptime1542s); old watcher PID1649144 and resident PID1764191 were absent.

A durable receipt subsequently recovered proves the old watcher canonical actually exited1/null at23:19:00.073Z before reboot. Matching log shows graph inference463/515, anonymous-memory pressure reaching roughly3755–3806MB against1792MB admission threshold, then build:graph-plan SIGKILL and parent exit1. The signal cause is not proven by that log alone. No watcher target tests ran. Old resident canonical has no outcome receipt and remains unknown after reboot, not pass/fail.

All preboot terminal d428 runtime receipts remain preserved. New Root recovery check handles11846/11911/11977 remain Root-owned; this reviewer has not launched duplicate restored gates or MCP startups.

## Actual Cargo Plan Boundary Review

Read-only inspection of withCargoArtifactPlanV1 and withRepositoryCargoArtifactPlanV1 confirms a stronger existing identity boundary than staged mtimes or argv: actual compiler graph capture, Source-boundary admission, target recipe/mode/platform facts, derived identity, recapture/recheck, and completed compiler-observation Source digests. The repository wrapper also validates actual Native caller repositoryRoot and buildMode and retains genuine compiler-workspace storage.

The plan is live and control-bound. Its recheck/produce closures refuse after its scope settles; facts/identity DATA is not transferable producer authority. The normal repository wrapper also holds a compiler-store lane through the plan callback. Awaiting the full build frontdoor for the same crate inside that callback could retain the very lane the delegated build needs. A real implementation must demonstrate no lock cycle, directly use its declared producer path, or close that plan/lane before independent production and freshly revalidate identity. It must not reuse an expired plan or forge a producer token.

The gateway protocol session interface retains its real channel and per-session Original/control, but does not itself acquire a product Store. requireMcpBinary only verifies a path/file and executable mode. Whether staged publication needs a retained mapped product backing must be proven by the actual publication/use contract; success status is not automatically use custody. CARGO_BUILD_OWNER_PID currently names the gateway process, so an independent shared producer's actual ownership must be resolved rather than copied from an arbitrary waiter.

A direct TUI send to the backend v2 sub-agent was rejected by the app server (direct sub-agent input unsupported, -32600); findings were relayed through the authorized Course coordinator. This is an API routing limitation, not an approval denial.

Native fixture inventory for later work: the authored process-crash service case issues only lifecycle/status requests and never dispatches a build/test task. The fixture server nevertheless constructs an execution port with a joined faketool path, and the current helper gates the whole scenario on a manual native directory. No tracked Rust/Cargo faketool implementation was found in the scoped/global Source searches. A schema-declared lifecycle-only fixture with an actual qualified pinned watcher could avoid inventing authority for an unused executable; Root has been notified for ownership guidance. No fixture Source was edited while MCP review has priority.

At2026-10-30 00:44UTC, actual OS-MCP launcher Source still SHA38d2a02c20ec240557bed2ec36462e02334a58dedc2497a7bf6e61273facf6b6 (mtime2026-10-28T13:13:06.387Z). The independent baseline duplicate-start red remains applicable; no implementation closure or physical coalescing acceptance is claimed. Course Medium retains implementation ownership; Root watcher does not patch that seam.

Current read-only physical census 2026-10-30T00:46:45.247Z observes 6 distinct live normal build frontdoors for the same actual MCP target and --output=stream argv, all owned by existing client gateways. This confirms duplicate ingress is physically present; shared product/capture authority is not inferred from argv. No new gateways or builds were launched for this census; no custody or cancellation controls were mutated. PIDs:10196(parent10109),10442(parent10412),10554(parent10503),11308(parent11276),11499(parent11469),11572(parent11532).

Verified all six observed frontdoor parents actually have root argv dev mcp stdio os --catalog ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated --folder .🧬semio/🔗space/os-mcp. Parent command role is observed, rather than inferred from PID ancestry alone. No proof of shared captured product identity is claimed from these requested arguments.

Current independent authenticated issuer Source review (2026-10-30T00:53:49.342Z, native-build SHA8c11c08e2fabb404698cf636fd48d4559cfbb63f89b68e093b696ba3425a8b60): withCargoArtifactPlanV1 requires actual Native compiler-workspace allocation and issued control; admits actual captured graph Source; computes recipe/platform facts; recheck recaptures graph and checks completed Cargo provenance against admitted Source; frozen plan tracks bounded pending operations and refuses operations after callback closure. This supports the plan issuer boundary, not a completed launcher coalescer. Actual positive two-independent-Original same-product identity remains necessary because lane/path/environment/PID data can otherwise accidentally split products. Producer custody/control must be independent of any departing waiter, and a plan whose scope ended cannot be retained as authority. Relayed to Course coordinator; no Source edits by reviewer.

Read the owner's primary checkpoint mcp-owned-artifact-plan-2026-10-30.md in TEACHING-ARCHITECTURE-DEPLOY-READY. It identifies the actual next authority seam: ordinary DevScript/MCP Original has workspaceRoot only, and the nested canonical build currently supplies Native artifact authority. A real closed declared-product acquisition child must therefore be factored through that existing canonical issuer before cold staging can use the plan. Cargo plan authenticity does not authorize direct use by the Dev Original. No fabricated Original, expanded Dev capabilities, cross-issuer argv/PID lease or retained compiler lane around delegated same-crate build is acceptable. The owner explicitly records actual scoped Native receipt binding, mutation/lease-wait cases, drainage and physical multiprocess coalescing as pending, separate from neutral/control tests. Independent reviewer agrees with these qualifications; implementation remains Course-owned.

2026-10-30T03:02 read-only recheck: actual OS/modules/🌉mcp/TS still SHA38d2a02c20ec240557bed2ec36462e02334a58dedc2497a7bf6e61273facf6b6 (mtime2026-10-28T13:13:06.387Z). stageMcpBinaryAsync still unconditionally startToolV1(repo run MCP_BINARY_TARGET --output=stream) per gateway. No coalescing implementation or authentic independent-Original physical acceptance observed at this seam; Course owner retains implementation. No competing Source edits or builds launched by this audit.

Read-only current launcher Source checkpoint: {"at":"2026-10-30T06:50:47.582Z","sha256":"38d2a02c20ec240557bed2ec36462e02334a58dedc2497a7bf6e61273facf6b6","mtime":"2026-10-28T13:13:06.387Z"}. No gateway/build/process or Source mutation was made by this audit.
