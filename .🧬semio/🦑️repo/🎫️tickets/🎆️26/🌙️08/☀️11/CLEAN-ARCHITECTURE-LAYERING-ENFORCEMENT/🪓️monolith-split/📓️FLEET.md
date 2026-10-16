# 🪓 Monolith Split and Dependency Direction Fleet

Sub-track of the parent ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`, opened 2026-10-15.
The repo MCP `ticket_open` and `ticket_reopen` both failed with a `structuredContent` schema error and created nothing, so this work is recorded here inside the parent ticket.

## Scope

- Split monolithic source files into child modules that follow the taxonomy (`<emoji>️<kebab>/🦀️.rs` wired via `#[path]`; TS siblings `<emoji>️<kebab>/🟦️.tsx`).
- Fix dependency edges that point the wrong way (framework→`✏️s`, General→Specific).
- Out of scope: the parent coordinator's Cargo/Actor/Store/Member/Media build-infra frontier, `🏪️store`, `🛢️db`, `🌊️flow`, and `◻️2d` (a peer is restructuring it live as of 01:35).

## Roles

- Coordinator: Opus 5.5 High (main chat).
- Executors: Sonnet 5.5 High, one per monolith, with disjoint file ownership.
- Auditors: Haiku 5.5 High, read-only seam maps and gate audits.

## Wave 0: audits (launched 01:3x)

| Agent | Deliverable |
| --- | --- |
| Shell wgpu seam map | SEAM-MAP-SHELL-WGPU.md |
| wgpu renderer + Scenes seam map | SEAM-MAP-WGPU-RENDERER-AND-SCENES.md |
| infinite world + board seam map | SEAM-MAP-INFINITE-WORLD-AND-BOARD.md |
| ShellHost/World3dHost/ShellHelpers TSX seam map | SEAM-MAP-SHELLHOST-TSX.md |
| plugin instance/host, manifest, hub bootstrap seam map | SEAM-MAP-PLUGIN-MANIFEST-HUB.md |
| Dependency-direction gates | DEPENDENCY-DIRECTION-AUDIT.md |

## Environment facts

- Raw cargo is refused. Use `bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --message-format=short`.
- At 01:3x the root workspace failed `cargo fetch --locked` because `◻️2d/📦️packages/🦀️rust/Cargo.toml` was missing during a peer move. It reappeared minutes later.
- Load average was about 22 at the start of the session.

## Wave 1: executors (Sonnet High)

| Slice | Files owned | Result file |
| --- | --- | --- |
| Infinite split | `♾️infinite/🌍️world`, `🎲️board/…/➕️normal`, `…/🕸️dag` | RESULT-INFINITE-SPLIT.md |
| wgpu renderer split | `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | RESULT-WGPU-RENDERER-SPLIT.md |
| Scenes split | `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | RESULT-WGPU-SCENES-SPLIT.md |
| Shell split | `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | RESULT-WGPU-SHELL-SPLIT.md |
| TSX split | `🛠️ShellHelpers/🟦️.tsx`, `🌐️World3dHost/🟦️.tsx` | RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md |
| Package 1: framework `s.` ids | io schema, kernel, os host, ShellHost, mcp | RESULT-FRAMEWORK-S-IDS.md |
| Packages 2 and 6: test-only cross edges | framework and os tests | RESULT-TEST-EDGE-RELOCATION.md |
| Packages 4 and 5: plugin→plugin edges | `✏️s` consumers and stdio | RESULT-PLUGIN-CROSS-EDGES.md |
| Package 9: UI cycles | `🖱️ui` | RESULT-UI-CYCLES.md |

Held for later waves: the ShellHost.tsx hooks refactor; plugin instance/host, manifest and hub bootstrap splits (seam map pending); packages 7 and 8 (SDK barrels), 10, 11, 12 (os cycles) and 13; and package 3 (repo-lib plugin roots, which overlaps the parent's repo-lib work).
| Plugin instance and host split | `🔌️plugin/🧵️instance`, `🔌️plugin/🖥️host` | RESULT-PLUGIN-INSTANCE-HOST-SPLIT.md |
| Manifest split | `🛂️manifest/🦀️.rs` | RESULT-MANIFEST-SPLIT.md |
| Hub bootstrap split | `🌎️hub/🏗️bootstrap` (routes moved to the hub lib) | RESULT-HUB-BOOTSTRAP-SPLIT.md |

## 02:40 pivot: the source-boundary gate

`⚡️caching/🔣️policy.json` `storage.sourceBoundary` (`maxActiveLines 4000`, `maxSourceFiles 32`, `maxSourceBytes 262144`) makes the compiler ingress refuse any crate whose first-party closure contains an oversized crate. `semio-framework-3d` (3.2 MB) and `semio-framework-job` are already refused, so nothing in our scope can be cargo-checked.

- All Rust split executors were told to verify statically: lossless line multiset, rustfmt parse, path resolution, conservative `pub(crate)` closure and emoji admission. Each child stays at or below about 2500 lines, ready to become its own crate.
- The wgpu renderer split is done statically: a 1,006-line parent plus 16 children. Compile is blocked by the gate.
- New audits: CRATE-SOURCE-GATE-CENSUS.md (which crates are oversized, in bottom-up order) and PRODUCER-CRATE-RECIPE.md (the peer fleet's canonical crate-carving recipe).
- Next wave: carve the leaf crates that are oversized but not peer-owned into small producer crates, so compilation unlocks bottom-up.

## Wave 1 outcomes (all verified statically; compile is blocked by the gate)

| Slice | Outcome |
| --- | --- |
| wgpu renderer | 19.8k lines → 1,006 plus 16 children |
| Scenes | 12.3k → 397 plus 15 |
| Shell | 35.6k → 966 plus 20 |
| infinite world / normal / dag | 17.4k / 13.8k / 5.4k → 2,581 / 2,669 / 1,649 plus 13 / 6 / 5. The audit found 144 private-item escapes; hardening is under way. |
| manifest | 8.1k → 589 plus 6. Wrong-direction vocabulary is grouped in `🧳️product-vocabulary`. |
| plugin instance / host | 10.9k / 7.3k → 1,256 / 838 plus 6 / 3 |
| hub bootstrap | 12k → 579 plus 22 route modules, still in the bin. The audit found the region markers dropped; restoration is under way. |
| ShellHelpers / World3dHost | 6.7k / 8.6k → 390 / 2,845. Vitest failure sets are identical before and after. |
| Package 1: `s.` ids | Neutral `framework.io.*` and `os.space-index` ids. The space plugin contributes the topic. |
| Packages 2 and 6: test edges | In progress |
| Packages 4 and 5: plugin edges | Cruiser edges 32 → 4. `QuestionDictionary` moved to the framework value crate; s-level `🏘️plugin-composition` test project added. Option 1 (moving the Semio standard into the framework) is queued. |
| Package 9: UI cycles | `🖱️ui` cycles 146 → 44. The react barrel shrank from 8.7k to 6.6k lines. |

The coordinator fixed one oracle line (World3dHost shading reads `🎨️mesh-render`) and one parent build-script literal (`framework.io.sqlite-snapshot`). It refreshed the root, layout and forms `Cargo.lock` files offline.

## Wave 2 (running)

- Hardening with the full-resolution oracle (`📓️RESOLUTION-ORACLE.md`): the wgpu trio, manifest + plugin + hub, and infinite.
- Splits: ShellHost hooks, puzzle 5d/2d editors, architect registers, bim mount re-homing.
- Seam maps: os Rust tier 2, TSX tier 2, hub/repo/surface.
- Source-text readers: one shared module-tree reader helper.
- Census of crates over the gate limits.

## Coordinator fixes (04:30–04:45)

- A peer sweep committed in `0c3e78944af` (2026-10-14 02:26) left invalid qualified paths of the form `<X as ArtifactPack + ::…::ArtifactStoreSnapshot>::decode_pack/encode_pack[_with]` in 10 places: hub inference runtime, gis/shooting/fem2d/fem3d/trinity editors, hub check-in and bin-unit tests, flow-vcs test and os space. The coordinator reduced each to `<X as …ArtifactPack>::…`, since every one of those methods lives on `ArtifactPack`. All touched files now parse with rustfmt, and `git grep` finds 0 remaining.
- Scratch cargo workspaces outside the repo were used by one agent (repo CLI/GraphQL) to type-check. Their target dirs were deleted, and the no-scratch-compile rule was added to `📓️EXECUTOR-RULES.md` and sent to every running Rust executor.

## Wave 2 results so far

- tiled-map: 277-line root plus 12 children. `image` runtime use was replaced by `semio_framework_pixels::decode_png`, with `image` kept as a dev-only oracle. GIS code is grouped in `🌍️gis-policy`.
- repo CLI: 35-line root plus 8 children, with the cycle broken. GraphQL: 61-line facade plus 6 children, and the new adapter crate `semio-framework-repo-graphql-fs` dropped 11 dependencies from the core.
- bim model: 7,817 → 127 lines through mount re-homing, with module-tree equality across 3,284 modules.
- hub directory: 4.4k lines → 156 active-line parent plus 8 children. Hub inference runtime: 4k → 94 active plus 11. The `artifact_authority` cycle is reduced to one edge.
- CAD renderer: 3 rules-of-hooks bugs fixed test-first. 6,859 → 424 lines plus 10 children.
- Puzzle editors: 5d 9.1k → 1,703 plus 8 children; 2d 4.1k → 1,357 plus 4.
- Source-text readers: shared `readModuleTreeSource` helper (`📚️compiler/📖️syntax/🌳️module-tree`) used by about 60 readers, plus `POLICY_SPLIT_SOURCE_ROOTS`.
- Infinite hardening: 51 struct widenings; the full-resolution differential is identical in 4 configurations.

## Cargo.lock state (05:0x)

- The root `Cargo.lock` working tree carries roughly 200 registry crates bumped to their latest patch versions compared with HEAD and with the coordinator's pre-session backup (for example tokio 1.52.3→1.53.3, wasmtime 47.0.3→47.0.4, serde 1.0.228→1.0.229). This came from someone's deliberate online update, not from this fleet's `update --workspace --offline` runs, which only added peer workspace members (composition-admission, pack-source, ui-contract-{budget,identity,text}, 2d sub-crates).
- As a result, `bun repo cargo … metadata --offline --locked` currently fails with `failed to download http-body-util v0.1.5`, because the new versions are not in the offline cache. This is left to the dependency-acquisition owner. The coordinator briefly pinned hyper and hyper-util back, then restored them to 1.12.0 and 0.1.21 so as not to fight the peer's upgrade.

## Wave 3: crate carving (launched after the census)

The census (`CRATE-SOURCE-GATE-CENSUS.md`) found 210 crates, 31 of them oversized. The tier-0 carvable leaves are compiler, pets, quiz, repo-graphql, replication and ui-contract; the last two are peer-active.

| Slice | Result file |
| --- | --- |
| compiler: five embedded fonts (1.47 MB) moved to a font port | RESULT-CARVE-COMPILER.md |
| pets: 11.3k lines carved into producers; shared module mounts get a single owner | RESULT-CARVE-PETS.md |
| quiz and repo-graphql: carve one cohesive module each | RESULT-CARVE-QUIZ-GRAPHQL.md |
| print assets out; server carved | RESULT-CARVE-PRINT-SERVER.md |

Rules are in `📓️CARVING-RULES.md`. At 10:xx the `bun repo cargo` launcher itself fails with "Live Original process issuance required" (peer infra), and `bun repo dependencies sync` fails the same way. So verification is the gate's own counter (`🗑️generated/census.ts`) plus static checks.

## Wave 2/3 completions (through ~13:00)

- ShellHost: 13,707 → 591 lines plus 9 module children and 21 ordered hooks; exports identical; 7 `?raw` readers re-pointed per child.
- NodeGraph: 4,005 → 627 lines plus 8 module children and 7 hooks; NodeGraph cycles 3 → 0 via the shell-neutral leaves `🌉️scene-host-contract` and `🖱️surface-menu`. A rewire to remove the compatibility re-exports is running.
- PluginRuntime: 4,582 → 369 lines plus 18 children; `loadPluginModule` became 14 phase functions; 0 cycles once `PluginManifest`/`ViewModel` moved to `🧷️handle-contract`.
- os.ts: 5,775 → 504 lines plus 5 children; os-root cycles 12 → 0. r3f: 4,601 → 495 lines plus 12 children. Region markers restored.
- CAD renderer, presentation dedupe and product split are done (presentation moved under `🎯️targets/⚛️react`, matching the UI layout).
- EngineCanvas: 205-line parent plus 9 children. Interpreter: 157-line parent plus 10 children. `element_contracts` holds the shared renderer contracts.
- os host: 8 inline modules became files; the `*_legacy` functions were renamed to accurate names (they are the live CAD composer route). mcp workspace: 12 children.
- architect registers: 107-line component plus kernel and 5 family children; new `registers` taxonomy kind.
- Test edges: 17 framework→s and 2 R3 edges cleared (94 → 0 cruiser edges from those files).
- Carving: compiler fits the gate (fonts go through `FontSet`/`CompilerFonts`, hosts install them; the first real `bun repo cargo check` succeeded); print fits (schema documents load at runtime); server is 5 crates plus a facade; quiz and repo-graphql each lost a `-schema` crate; pets is 7 producers plus a facade.
- Hardening with the patched oracle found and fixed 12 plugin-instance and 8 hub regressions and 51 infinite structs; wave-2 splits are clean.
- The region audit (`AUDIT-REGIONS-NAMES-SIZES.md`) found region markers dropped across many splits; a restore is running.

## Goal phase: end to end (started 14:30)

The user set the goal to keep untangling until everything runs end to end.

State at the start:
- 25 oversized crates (census rerun).
- The launcher fails with "Finite original maintenance authority required": the lease control has no deadline, and the error is thrown from `⚡️caching/🧹️pruning/🏪️budget/⚙️execution`.
- The Nx producer route fails on a missing `📚️layers/🟦️.ts` (peer mid-edit).

Wave 4 (14 agents) per `📓️WAVE4-BRIEF.md`: REPO-CLI, CANVAS, UI-RENDER, UI-SCENE, UI-RUNTIME, SURFACE (plus moving GIS policy into the GIS plugin and giving terrain a single owner), REPO-DASHBOARD, PACK, FRAMEWORK-FACADE (plus relocating product vocabulary), OS-INFINITE (plus relocating the puzzle2d knowledge), OS-MCP, OS-FLOW (plus the flow and workflow artifact crates), OS-HOST-NEURAL-DAG. Haiku audit: why the renderer-wgpu and db crates are refused, and the plugin and hub gate census.

## 16:05 coordination

- Compile attempts are refused with "Original selected owner registry changed", because the fleet's carves keep changing `📋️build-owners.json`, manifests and launch routes.
- Decision: launch no new carving waves until a quiet compile window has run. Once the wave-4 agents finish, freeze registry and manifest edits from this fleet, refresh locks, then run check and test bottom-up (census tier order) and fix errors.
- Load average is about 50.

## 16:30 compile blocker: image headroom

- Canonical compile: `SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --message-format=short` (RESULT-INFRA-UNBLOCK.md §7).
- It is refused with "Insufficient physical compiler initial headroom": the Rust image `⚡️cache/rust/fixed/m` has 4,293,222,400 bytes free and needs 4,294,967,296. The image holds 6.0 GB of cargo state and 5.3 GB of Nx SDK material.
- The sanctioned historical retirement fails with "Original funded graph publication namespace required" until the peer republishes the Nx SDK. This is not ours to bypass.
- Infra defects fixed by this fleet: the owner-contract validator now accepts `enum`/`uniqueItems`; `lsof` paths are decoded; the Repo Cargo launch entries carry a finite policy.
- A background watcher fires when the image has at least 4.4 GB free. Until then, carving continues, because registry churn no longer costs a compile window.

## 20:30 status

- Carved (static verification): canvas, ui-render (plus font and asset delivery), ui-scene, ui-runtime, surface, repo-dashboard, pack, the framework facade (21 crates), os host/neural/dag, os-mcp (22), os-flow (18), flow-flow, os-infinite world (7) and board (11), stdio-contract (11), trace testkit, quiz/graphql/pets/server/print/compiler/repo-cli.
- Infra: the launcher works for gate-compliant closures (renderer WP0 had 5 green `check -p`). The owner registry now covers every package (235 rows added). The counter's cfg bug is fixed.
- Plans: renderer (87 crates; WP0 done, WP1–WP5 running), os-kernel (48 crates; WP0 plus non-store packages running, store waits for peer batches), plugin (58 crates; WP0 running), replication (16 crates; executing).
- Plugin crates: wave-0 generators and the space-space pilot are done; S2b automation and the rollout to the 11 clean packages are running.
- Running: the compile pass 1 agent (real check and test on all compliant crates), db, hub.
- Regression fixed by the coordinator: the root script import of `INTERACTIVITY_AUDIT_WORLD3D_FILE`.
- Correction: the `icon-embeds` file mounts 250 files, which breaks the file budget, so WP2 moves IconRenderHost onto the runtime asset port.

## 21:20 usage-limit interruption

All 12 fleet agents and the relocation session stopped at the session usage limit, some mid-edit. After the reset they are resumed in batches to avoid another limit hit.
- Batch 1, which had mid-edit state: hub, db, kernel WP0, plugin WP0, renderer WP2, plugin generators. Each first verifies and repairs its own half-applied edits.
- Batch 2 part 1: replication, compile pass 1.
- Pending: renderer WP1, WP3, WP4, WP5 and the relocation session.

## 22:xx first real compile results

- Compile pass 1 (`--manifest-path 🧰️framework/Cargo.toml`, native lib): 129 of 137 gate-compliant framework crates pass `check` with warnings only, among them our ui-scene, ui-render and most surface carves. Two crates had one error each, fixed but not re-run: canvas-assets (a wasip2-only `estimate_only` was called on every target) and ui-wgpu-device (wgpu 29 `BufferView` has no lifetime). Six crates were not reached.
- Blocker: image headroom is 3,457,024 bytes short of the 4 GiB gate. The sanctioned recovery (`cargo-retirement-preview/retire` in the peer headroom script) fails with "Original funded graph publication namespace required" until the peer republishes its Nx SDK. Nothing is deleted or bypassed.
- Gate census: 204 of 557 crates pass. The biggest blockers are replication (332, now carved and pending a re-census) and ui-contract (203, which the peer is actively subdividing; see their `ui-contract-owner-subdivision-implementation-current.md`).
- db is carved into 41 crates and replication into 16 crates plus a facade.

- 2026-10-16: taxonomy sweep done (RESULT-TAXONOMY-SWEEP.md); slug-pattern follow-up dispatched to replace the 778+261 hand lists.

## 2026-10-16 05:00 wave 5 (new coordinator session)

The previous coordinator session ended at about 04:40. Its last executors (kernel engine WP5/WP6 in `okwp56-tools`, kernel sqlite-host in `oksh-tools`) were cut mid-edit; renderer integration 1 left two orphan full differentials running.
- Census at 04:50: 712 crates, 413 compilable, 6 oversized (plugin, renderer-wgpu, os-kernel, plugin-host, plugin-host-test-component, os-store-sync).
- Headroom is admitted. The framework lock was refreshed offline (+10 peer ui-contract crates). The launcher now fails at the peer's new `📐️profile` schema check (`Complete observed Cargo compiler profile required`).
- Wave 5 per `📓️WAVE5-BRIEF.md`:
  - Sonnet executors: INFRA-UNBLOCK-2, OS-KERNEL-ENGINE (resume), OS-KERNEL-SQLITE-HOST plus sync (resume), PLUGIN-INTEGRATION-1 (flip WP3–WP5), PLUGIN-WP7, RENDERER-INTEGRATION-2 (Shell tier-1, frame-core and WP5 flips), PLUGIN-HOST-CARVE.
  - Haiku audits: DEPENDENCY-DIRECTION-AUDIT-2, MONOLITH-CENSUS-2.
  - COMPILE-PASS-2 starts once the launcher reaches cargo.
- 05:02: INFRA-UNBLOCK-2 fixed the profile refusal. Cargo emits `trim_paths:["object"]`, so the schema now admits it as an optional field and the rlib adapter refuses non-empty values. The next blocker was 2 orphan old-format lease claims (dead pid 37808). Coordinator ruling: option 1. The claims were moved to `🗑️generated/coord-orphan-claims-backup/` and their 2 empty `.queue` dirs were removed; the unblock agent re-probes.
- Wave 5b, TS and source-file splits per `MONOLITH-CENSUS-2.md`:
  - ROOT-SCRIPT-SPLIT (`📜️script.ts`, 25.7k lines)
  - RENDERER-TS-TESTS-SPLIT (engine-contract 14.4k, plugin-runtime 3.8k, typedwire 3.2k)
  - REPO-TEST-VSCODE-SPLIT (repo test harness 7.7k; vscode client 3.6k with its schema-first codegen move)
  - MISPLACED-E2E-TESTS (gis service-composition 7.4k relocated to its owners; time-travel e2e made plugin-neutral)
  - INTERPRETER-KERNEL-TS (Interpreter.tsx host-registry inversion; TS kernel 4k)
  - COMPILE-PASS-2 waits for `LAUNCHER REACHES CARGO: yes` in RESULT-INFRA-UNBLOCK-2.md
- Ownership rulings: `🛢️db` stays with this fleet's earlier carve (RESULT-W4-OS-KERNEL-DB.md); its large files are not split further in wave 5. The `🖱️ui/…/🧊️wgpu` files stay with the peer's UI GPU cut.
- 05:04: LAUNCHER REACHES CARGO. `canvas-assets` and `ui-wgpu-device` (the compile-pass-1 fixes) now check with 0 errors. Every wave-5 slice now switches to real `bun repo cargo` evidence.
- 05:4x DEPENDENCY-DIRECTION-AUDIT-2 against the baseline:
  - production R1 framework→`✏️s` edges: 17 → 0
  - R2 cross-plugin edges: 32 → 4
  - literal-external: 2543 → 300
  - R4 general→specific got worse: renderer→specific 661, UI→framework packages 256
- Wave 5c edge fixers:
  - SDK-BOUNDARY: P1, 1234 s→framework-internal edges
  - SMALL-CROSS-EDGES: P8 layout→stdio, P9 hub→s, P10 test-only R1
  - CARGO-PLUGIN-DIRECTION: P6, 121 bridge→artifacts and stdio contract roles
  - UI-BARREL-IMPORTS: P3, excluding the peer's `🧬️contract` and the wgpu cut
- Deferred: P2 (renderer hosts, which waits for the renderer flips), P4 (the os/🔨️modules TS cycle), P5 and P7 (parent-owned build infra and Store). The Shell plugin-staging `#[path]` mounts went to RI2.
- 06:1x REPO-TEST-VSCODE-SPLIT done (RESULT-REPO-TEST-VSCODE-SPLIT.md):
  - The test harness (7.7k lines) became 32 acyclic children; tsc passes and the test failure set is unchanged.
  - The vscode client is now 153 lines plus 13 children, with GraphQL types generated from the served SDL.
  - The coordinator applied the library re-point hand-off (`rtv-tools/apply-library-repoints.mjs --write`, 3 anchored edits). The 2 library tests now load: 37 pass / 10 fail. All 10 failures are peer fixture drift (stdio TSV schema ENOENT, graph Cargo.toml ENOENT, gltf fixture path shapes) or library-internal readers; none involves the re-point. Log: `🗑️generated/coord-libtests.log`.
  - Open: library taxonomy rows and the generator-registry entry (`rtv-tools/generator-registry-entry.json`). `repo-vscode:generate-graphql-types` fails on `🛂️manifest/🟦️.ts` → `../🎠️kernel/🟦️.ts`, which INTERPRETER-KERNEL-TS is mid-splitting; recheck when it reports.
- 07:2x MISPLACED-E2E-TESTS done (RESULT-MISPLACED-E2E-TESTS.md):
  - Gis service composition: 100 of 129 tests were relocated to the os worker, Directory and config owners. The gis route went from 64/149 to 149/149 passing, with no new failures.
  - Time-travel e2e: a plugin-neutral framework harness, with the puzzle scenario moved to `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel` (`@semio-tech/puzzle-js:time-travel`). 0 framework files name a concrete plugin.
  - Not yet proven: a run against a live puzzle2d serve. That needs a dev serve from the main session.
- 07:3x INTERPRETER-KERNEL-TS done (RESULT-INTERPRETER-KERNEL-TS.md):
  - Interpreter.tsx is now a 150-line entry plus 18 children. It no longer imports any concrete host: the hosts register through `🗺️element-host-registry` from the composition root `🪢️ElementHosts`, with lazy reads that break the init cycle.
  - The TS kernel became 34 children (old file deleted); 56 plus 23 importers re-pointed by AST.
  - Renderer suite: 0 new failures. Kernel suite: 91/91.
  - Not yet proven: a browser boot of the React renderer with the new registry. Add it to the end-to-end boot list.
- 07:4x UI-BARREL-IMPORTS done (RESULT-UI-BARREL-IMPORTS.md):
  - `ui-no-framework-packages`: 256 → 244. 21 edges removed; a peer's new tests added 9.
  - Production edges outside peer areas: 42 → 25.
  - ui-react vitest: 1162/1162.
- Hand-off to the parent (the policy lives in the held `📚️library/🕸️dependencies/🧭️direction`): decision T. Exempt `BOOTSTRAP_TOOLING_ENTRY_PATH` from `ui-no-framework-packages`, as the cross-technology rules already do (93 tooling edges).
- Follow-up queued: D1 left a UI-owned twin of the kernel ephemeral container (`🎨️styling/🫧️ephemeral-state`) next to `🎠️kernel/🫧️transient`. That is two owners of the same mechanism. Give it a single owner in a neutral leaf that both may import, and update the OS renderer pins.
- 07:5x SMALL-CROSS-EDGES done: P8/P9/P10 cruiser edges 23 → 0 (layout port `embeddedChildSlotV1`, 16 hub→s edges, Play publication tests relocated). Its residuals (layout Rust crates → stdio, the hub shipped-fleet `include_str!` of a stdio fixture, file-read couplings) went to the new slice RUST-CROSS-COUPLINGS (`rcc`).
- 08:1x PLUGIN-WP7 staged (RESULT-PLUGIN-WP7.md):
  - acyclic tier-2 order: render < dispatch (new crate) < {typed-ops, time-travel-turns, tool-run-turns} < commands < ladders < app;
  - all 406 methods are now borrowed-bundle `XCtx`/`XRef` methods;
  - differential clean.
  Not flipped. **Critical path: every plugin family crate depends on the oversized `os-kernel`.** Plugin compiles wait for OS-KERNEL-ENGINE and OS-KERNEL-SQLITE-HOST. The T10–T21 bins are computed but not applied.
- 07:00 compile headroom is the fleet bottleneck again.
  - The Rust image has about 4.0 GiB free against the 4 GiB gate. It holds cargo 5.9 GB (prototype 3.2, home 1.5) and peer Nx material 5.4 GB (graphs 2.1, tmp 1.5 from 10-15, native 1.0, artifacts 0.8).
  - The peer's sanctioned `cargo-retirement-historical-preview prototype` fails with `Live Original resource control issuance required` (log `🗑️generated/coord-retire-preview.log`).
  - Nothing was deleted. The fleet compile mutex (`🗑️generated/cargo-mutex.lock`, WAVE5-BRIEF) was broadcast to the 8 compiling executors, with the kernel slices first.
  - Escalated to the user: Nx material inside the Rust image, or the image size.
- 07:2x EPHEMERAL-SINGLE-OWNER done. The transient store has one owner at `🧰️framework/🔨️modules/🫧️transient/🟦️.ts`, and the UI twin is deleted. ui-react passes 1157/1157 and the kernel passes 91/91.
- Policy hand-offs to the parent: both live in `📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`, which is held by the parent.
  1. Admit `"^🧰️framework/🔨️modules/🫧️transient/"` in the `pathNot` list of `ui-no-framework-packages` (around line 428). This brings the rule back from 260 to 244.
  2. Decision T: exempt `BOOTSTRAP_TOOLING_ENTRY_PATH` from the same rule (93 tooling edges).
- 07:3x SDK-BOUNDARY done (RESULT-SDK-BOUNDARY.md):
  - `@semio-tech/framework` exports grew from 30 to 125 (95 facade files, explicit named re-exports). A codemod rewrote 807 plugin files.
  - Rule counts: `plugins-framework-sdk-only` 1763 → 725; `no-cross-technology-✏️s-to-🧰️framework` 1807 → 470.
  - Vite aliases now use exact-match regexes, which fixed the broken subpath resolution.
  - Policy edit (`dependencyTypesNot: ["aliased-workspace"]` in `crossTechnologyRules`): the coordinator reviewed it and accepts it. The rule's own comment limits it to relative imports, and alias edges stay covered by `declaredDependencyDirectionRules`.
  - The remaining 725 edges go to repo-lib (~520, plugin `📜️script.ts`) and os product (~250). Designs are in its §5. Next package: plugin scripts should use repo tooling through a tooling SDK, and os-product consumers should go through the os SDK.
- SDK-BOUNDARY-2 dispatched for buckets L, B4, B6 and B7 plus the 48 plugin-test source reads. Hand-offs to the parent: B1/B5 (repo-lib runners and the repo test kit, 519 edges: extract neutral runner kernels into `🔨️modules/🏃️process`, inject the product parts) and B2 (os store schemas `🪆️child`/`🔗️link`/`📦️blob` move next to `🧬️schema/🗿️artifact-reference`, 82 edges, needs the Store owner).
- 07:4x OS-KERNEL-SQLITE-HOST and WP7 sync done statically (RESULT-OS-KERNEL-SQLITE-HOST.md):
  - The SCC is gone, with 5 carve defects fixed.
  - Sync is split into a shared host core in `os-store-sync` and a platform `ArtifactHost` in each of `-sync-native` and `-sync-wasm`, all within budget.
  - Real compile is still refused: headroom was 4,262,895,616 B at 07:09, short by 32 MB.
  - The `dsl` path-root defect in the carved store crates (space-schema derives `dsl::Mutations` with no dsl dependency) went to OS-KERNEL-ENGINE.
- 07:5x CARGO-PLUGIN-DIRECTION done statically (RESULT-CARGO-PLUGIN-DIRECTION.md):
  - The real rule inventory shows 23 strict violations, not 121. The 98 bridge rows were standalone binaries the gate never sees.
  - 23 → 3: 12 stdio contract crates re-roled to `library`, and `🪐️space/🫀️core` now consumes `semio_framework_os`.
  - The 3 left are procedural core → os-product flow/playbook model crates. They need framework facade crates (hand-off).
  - Not compiled: headroom refused 31 attempts.
- 07:2x RUST-CROSS-COUPLINGS done (RESULT-RUST-CROSS-COUPLINGS.md): cross-technology Rust includes went from 12 to 7.
  - The hub shipped-fleet test reads `SHIPPED_FLEET_SQLITE_OWNERS` from the stdio test-support crate. Not compiled yet.
  - The gis io-boundary duplicate was folded into the s dev ownership test.
  - The Play/demonstrator `project.json` reads moved into their own bun tests.
  - The hub composition reads of plugin artifacts are justified as sanctioned composition roots.
  - Deferred to the compile window: the layout→stdio drawing Rust coupling (30 files, 11 crates). Its staged design and the alternative (host the drawing standard in the framework) are in §2.
- 07:5x COMPILE-PASS-2 stopped partial (RESULT-COMPILE-PASS-2.md):
  - Framework `--lib`: batch 0 (60 crates) passes. Batches 1–2 had 5 fleet-carve errors (replication-interaction, replication-mutation-messages, pack-format-catalog, replication facade, pack-record-retained), all fixed in source but not recompiled, plus 1 foreign error (peer ui-contract-presentation).
  - Batch 3 was hit by machine-wide ENFILE at 05:40.
  - Since 06:05 every launcher call has been refused by headroom (4,162,984 KiB free vs 4,194,304 KiB required).
  - Remaining after headroom returns: framework batches 1–3 recheck, root `--lib` (99 crates), `--tests`, wasm, `test`, and the hub/teaching/plugin workspaces. The sequence is in its file.
- 10:4x OS-KERNEL-ENGINE done statically (RESULT-OS-KERNEL-ENGINE.md):
  - 18 crates carved; the store root went from 11,416 to 1,345 lines.
  - **The kernel census row is `overLimit=false`**: 5 files / 1,624 lines, down from 56 / 26,162.
  - The `DslArtifact` derive root now emits `::semio_framework_dsl_record`, and space-schema names `semio_framework_os_dsl`.
  - No compile ran (headroom). The first command to run once headroom returns: `python3 okeng-tools/waitcc.py lib1 okeng-tools/b2.crates`.
- 10:4x PLUGIN-HOST-CARVE done statically: 14 producers plus a facade, worst closure 3,429 lines / 6 files / 218,501 B; differential 370/370 clean; 4 seeded regressions caught. Not compiled (headroom). The batched command is in its last section.
- 11:xx RENDERER-INTEGRATION-2 done statically (RESULT-RENDERER-INTEGRATION-2.md). Not compiled (SQLITE_FULL, then headroom).
  - All 15 Shell tier-1, 8 WP5 and 9 frame-core crates are flipped. **The renderer top crate went from 336 files / 53,368 lines to 37 / 1,847.**
  - 148 free items were homed, 18 crates got `testkit`, and 23 of the 55 Shell tests moved into 7 bins.
  - The 16 `🧲️plugins` mounts turned out to be shell-plugins staging, not plugin source. They are gone from the parent.
  - The ri1 placeholders are filled.
  - A union oracle wrote through a symlink into `🌓️theme/📐️geometry/🦀️.rs`. The file is restored and the tool is guarded.
  - Stray identifier `S_SPACE_INDEX_DOCUMENT_SCHEMA` in shell-actions predates this pass; it belongs to the Package 1 `s.` ids follow-up.
- 11:xx RENDERER-TS-TESTS-SPLIT done: engine-contract (14.4k lines), plugin-runtime and typedwire became 53 suites (largest 829 lines). Test titles are identical: 713/715, 137/137, 114/217, the same failures as at baseline. The hub JSON path is now env-supplied (3 tests now pass). The ui fixtures go through a new public entry `🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts`. Left: 48 ui corpus paths and 34 other fixture paths still read internals.
- 11:xx ROOT-SCRIPT-SPLIT done (RESULT-ROOT-SCRIPT-SPLIT.md):
  - **`📜️script.ts`: 25,739 → 166 lines.** The logic now lives in 56 modules under `📚️library/🎛️commands` and `⚖️laws`, with no barrels.
  - The 59 importers are re-pointed, and the self-test lanes are injected through a `SelfTestLanes` port.
  - Parity: 31/31 CLI commands match, and 79/82 policy rules return identical rows.
  - Open: taxonomy registration of the new library folders (they join the unresolved class). This edit landed inside the parent's held `📚️library`; it was flagged to the parent through this file.
- 12:xx SDK-BOUNDARY-2 done (RESULT-SDK-BOUNDARY-2.md):
  - Rule counts: `plugins-framework-sdk-only` 725 → 619; `no-cross-technology-✏️s-to-🧰️framework` 471 → 372.
  - Layout is fully on SDK entries.
  - The time-travel kit moved to `⏪️time-travel/🧪️testing`, and scenarios load through `--scenario <module>`.
  - New SDK entry `@semio-tech/framework/test/source-text` replaces 101 literal framework-path reads in plugin tests.
  - Hand-offs: B6 has 12 edges left in one cluster, gis → os document-service worker; its move design is in the result file. B1/B2/B3 as before.
- 12:xx TAXONOMY-SWEEP-2 done: the slug-pattern rule (`memberSlugPattern`) has landed. 5412 of 5414 wave-born folders already resolve, and the last 2 are fixed by the row `semio-tech-module-children`. Whole-repo unresolved findings: 3657 → 3642; ambiguous: 0; kind-id changes: 0.
- 12:xx AUDIT-WAVE5-RUST:
  - **Census: 727 crates, 627 compilable (413 at 04:50), 3 oversized** (plugin; renderer-wgpu at 37 files over the 32-file limit; plugin-host-test-component at 41 files).
  - Findings: guest-contract double mount; abi double mount; 117 orphan stdio artifact manifests (auto-commit `1bae6115bc5`); 116 files shared across plugin crates; a space-schema dev-dep back edge; the kernel test mount of store-testing; 107 `[DEBUG]` lines; ~183 `pub use X as Y` aliases; 11 paths over 240 bytes.
  - The stray `🏪️store/🦀️.rs.orig` (868 KB backup) was moved to `🗑️generated/coord-store-root.rs.orig`.
- Wave 5d fix slices:
  - SINGLE-MOUNT-AND-GATE (`smg`)
  - PLUGIN-CRATE-HYGIENE (`pch`)
  - DEBUG-AND-ALIAS-CLEANUP (`dac`)
- 12:xx AUDIT-WAVE5-TS: entry sizes are confirmed and there are no compatibility barrels. Findings went to TS-AUDIT-FIXES (`taf`): one broken launch entry, 44 + 10 test-time cycles (in-source blocks import their moved tests), 2 dangling fixture rows, 3 modules over 1,500 lines, repeated 🧭 docstrings in the root split, stale kernel comments, and docstring placement.
- 13:xx DEBUG-AND-ALIAS-CLEANUP done statically: 8 temporary `[DEBUG]` lines removed, 94 deliberate diagnostics kept with reasons. 58 of 183 alias lines removed, with 665 `test_allocation` uses re-pointed; 82 facade-API and 9 macro/path-root lines kept with justification. Hand-off: re-pointing facade consumers needs new manifest dependencies and lock updates. Long paths: none added by wave 5, and the max is 207 code points (within the AGENTS limit). Differential: 72/72 runs clean.
- 13:xx TS-AUDIT-FIXES done (RESULT-TS-AUDIT-FIXES.md): tests import code and code never imports its tests. In-tree cycles: worker 45 → 1, kernel 10 → 1 (both remaining ones predate the split). Launch entry and fixture rows re-pointed. The 3 oversized modules were split to ≤1,400 lines. The root-split docstrings now have 60 distinct emoji. Test sets identical.
- 13:xx SINGLE-MOUNT-AND-GATE done statically (RESULT-SINGLE-MOUNT-AND-GATE.md):
  - guest-contract depends on plugin-abi, and the context vocabulary moved into the contract.
  - abi is a real dependency of ui-host and webgpu.
  - renderer-wgpu went from 37 to 13 files.
  - The test component went from 41 to 16 files, plus the new `plugin-host-test-component-refusal` crate.
  - **Census: 1 oversized crate (`plugin`).**
  - Duplicate mounts: 116 → 45.
  - D5 (kernel dev-dep on the fixtures package) is not done; its blueprint is in the file.
  - Plugin workspace locks are stale in about 150 workspaces until an offline update runs.
- 13:3x Wave 6. The user asked for more saturation without hitting the limit early. 10 new Sonnet executors were dispatched; pi1 and pch were still running. Slices:
  - OS-KERNEL-WP8 (`okwp8`)
  - DUPLICATE-MOUNTS-2 (`dm2`)
  - FRAMEWORK-RUST-FILE-SPLITS (`frs`: number, async, dsl grammar, spr channel)
  - REPO-HUB-RUST-FILE-SPLITS (`rhs`: repo model, tickets, the 3 hub directory backends, server gateway)
  - TS-SPLITS-3 (`ts3`)
  - PLUGIN-FILE-SPLITS-A (`pfa`: bim, process, raster, norm)
  - PLUGIN-FILE-SPLITS-B (`pfb`: energy, fem)
  - RENDERER-HOSTS-UI (`rhu`: P2)
  - OS-TS-CYCLES (`otc`: P4)
  - DIRECTION-LEFTOVERS (`dlo`: procedural core, gis worker)
  - One Haiku explorer prepares WAVE7-CANDIDATES.md.
- 13:5x WAVE7-CANDIDATES.md is ready. Dispatched WP-A OS-SERVICES-MCP-SPLITS (`osm`), WP-B SCHEMA-VALUE-MATH-SPLITS (`svm`) and WP-C REPO-CRATE-ROOT-SPLITS (`rcr`); 15 executors are running. Queued: WP-F (tool-run, which needs pi1), WP-H, WP-D (minus the ts3 pets test), WP-E (minus the ts3 presentation test), H1 Shell bins (compile window), WP-G BREP (ask the user), WP-I1 UI (peer).
- 14:xx REPO-CRATE-ROOT-SPLITS done statically: 6 repo crate roots (1.7k–2.4k lines) are now 54–84-line roots plus 40 children. Differential identical, nothing lost. Its 40 new folders are queued for TAXONOMY-SWEEP-3 at the end of the wave.
- 14:xx REPO-HUB-RUST-FILE-SPLITS done statically:
  - The repo model, tickets, gateway and the 3 hub directory backends became 63 files (largest 839 lines).
  - The backends share a 12-section layout; neutral helpers are deduplicated into `📇️directory/🛟️backend-support`.
  - Follow-up: split the `HubDirectory` port into capability traits.
  - Foreign: `🌎️hub/📦️packages/🦀️rust/📜️script.ts:93` still imports `startLocalSessionBroker`, which a peer renamed to `withLocalSessionBrokerV1` (their test asserts the old export is gone). The root script itself loads fine. Left to the peer.
- 14:xx Dispatched PRODUCT-FILE-SPLITS (`pfs`, WP-D pets plus WP-E print/presentation/quiz/teaching; excludes the two ts3 files) and OS-RUN-VCS-IO-SPLITS (`orv`, WP-H).
- 14:xx PLUGIN-FILE-SPLITS-B done statically:
  - Energy Rust and Python twin tests, sim and fem visual-job are split, all files ≤1,301 lines.
  - Evidence: Python hashes identical over 1,189 scenarios; the 594 vector tables are identical.
  - Open issues, all pre-existing (the counts are identical with the originals restored):
    - `--check-applied` exits 1 for the fem3d (39/41) and energy-model (2/2) packages;
    - 7 missing fixture folders in the energy Python table;
    - the `sut` feature `.expect` on `impl Fn`.
  - Follow-up: one shared 594-row manifest instead of three indexes (Rust, Python, feature), to do in the compile window.
- 14:xx Dispatched TOOL-GRAPH-MACHINE-SPLITS (`tgm`, WP-F; framework modules only, never `🔌️plugin/**`).
- 14:xx OS-SERVICES-MCP-SPLITS done statically: 36 children, the services root is now a 105-line facade, and the differential is identical in 24/24 configurations. The services lib+test closure is over the gate (5,591 lines), as it was before the split. Dispatched the Haiku TEST-CLOSURE-CENSUS, because `census.ts` counts lib closures only.
- 15:xx DIRECTION-LEFTOVERS done: gis B6 12 edges → 0 (6 new SDK entries plus a worker-host port; gis service composition 149/149). Procedural core Cargo violations 5 → 0 through the new facade crates `os-flow-contract` and `os-playbook-contract`. Rust is static only.
- 15:xx Dispatched HUB-SCRIPT-SPLIT (`hss`, the hub `📜️script.ts`, 17,555 lines; the peer edits it actively, so cuts are anchored and one family at a time).
- 15:xx FRAMEWORK-RUST-FILE-SPLITS done statically:
  - number, async, dsl grammar and spr channel became 38 children; parents are 52–222 lines.
  - The differential is identical across 12 configurations.
  - **New oracle capability:** a sandbox-only stub layer for external crates lets rustc reach typeck. It catches E0603, E0616 and E0624, confirmed by mutation tests (`frs-tools/`).
  - Dispatched PRIVACY-ORACLE-SWEEP (`pos`) to run it over every finished wave-5/6 Rust carve and fix the carve defects it finds.
- 15:xx DUPLICATE-MOUNTS-2 done statically: 25 of 42 shared files now have one owner (13 new single-owner crates, 31 `[[test]]` targets moved to their owners). 17 were handed off to pi1, okwp8 and pch. 180 `Cargo.lock` files were edited by hand, so the compile window must start with an offline lock update everywhere. The plugin-host carve crates are missing from every lock.
- Dispatched OS-PACK-DEDUPE (`opd`): os-pack duplicates the pack-record-schema types, and its test build references a missing `test_allocation`.
- 15:xx TEST-CLOSURE-CENSUS (TEST-CLOSURE-CENSUS.md, tool `tcc-tools/tcc-census.ts`):
  - **28 native test units in 27 crates exceed the gate** (82 across labels). The worst are kernel lib-test (12× over, owned by okwp8), print (1.6 MB embedded schema), os-config (223 files) and pixels.
  - 11 native units cannot be measured because inputs are missing.
  - Oversized dev-deps: `ui` (through `wgpu-engine`, 80 renderer test crates) and `plugin`.
  - The lib census CSV is stale for 135 rows.
- Dispatched:
  - OS-TEST-COMPANIONS (`otcp`)
  - FRAMEWORK-TEST-COMPANIONS (`ftc`)
  - MISSING-TEST-INPUTS (`mti`)
- Queued: value / tool-run / tool-machine / time-travel / publication-mounted companions once svm, tgm and pi1 finish; the ui `wgpu-engine` dev-dependency diet for the renderer test crates.
- 16:xx OS-TS-CYCLES done: `no-circular` for os/🔨️modules (without the renderer) 51 → 4 (only the excluded store/media/vite rows remain), os/🎚️config 15 → 0, repo modules 1 → 0. In-source test registrations became self-registering tests. Fixing the barrel cycle exposed 31 tests that were hidden before (they show as failures now). tsc unchanged.
- 16:xx SCHEMA-VALUE-MATH-SPLITS done statically: 6 files → 19 children; the 59 derive expansion items are byte-identical, with a corpus harness ready for the compile window (`svm-tools/derive-corpus/`). P11: 0 edges fixed, because all importers sit outside its modules (246 edges; per-rule counts are in the file). The `value` test companion went to `ftc`.
- 16:xx TOOL-GRAPH-MACHINE-SPLITS done statically: 6 roots became 34 children, and the differential is identical across all configurations. The coordinator re-pointed the plugin tool-run operation test to `readModuleTreeSource`; `bun test` gives 2 pass / 0 fail.
- 16:xx MISSING-TEST-INPUTS done: about 55 path literals fixed in 21 files, nothing fabricated. Census error units went from 32 to 2.
  - Still refused: `os-renderer-wgpu` lib-test. It reads the draw pipeline files that the peer's ui draw carve split; handed off to that peer.
  - **New: the `plugin` lib-test is 31.2× over (571 files / 8.19 MB / 81,723 lines).**
- 16:xx OS-RUN-VCS-IO-SPLITS done statically: run, vcs, pack-value, os-io and neural-model became 34 children; the differential is identical in 30/30 configurations. `os-pack` was already over the gate before this split (295,282 B / 4,093 lines).
- 16:xx Dispatched TOOL-TEST-COMPANIONS (`ttc`) and HUB-DIRECTORY-CAPABILITIES (`hdc`: split the `HubDirectory` port into capability traits and drop the forwarding layer). 16 executors are running. A fresh lib census is running (`🗑️generated/coord-census-run-2.log`).
- 17:xx RENDERER-HOSTS-UI done (RESULT-RENDERER-HOSTS-UI.md):
  - `renderer-hosts-only-ui`: 707 → 535. Real production edges: 341 → 200 (38 of them are parent Actor/Media edges).
  - The `.cache/b/d/m` edges are a resolver artefact: `node_modules` is a symlink into the dependency image. Parent policy hand-off: add the real path to the react allow-list.
  - 102 renderer files now use 21 SDK entries; `app-channel` and `wire-contracts` were lowered into the framework.
  - About 160 actionable edges remain. They need os wire types lowered first; the order of work is in its file.
- 17:xx TS-SPLITS-3 done: spatial geometry (201 lines plus 14 children), the deck test, the backbone test, the pets stage test, and the os `📜️script.ts` (2,921 lines → 123-line router plus 9 command modules). Source modules no longer import their tests. Test titles and failure sets are identical; `no-circular` 5 → 2 (geometry) and 1 → 0 (presentation).
- Correction (14:51): the coordinator entries labelled "15:xx" to "17:xx" above were written between 14:00 and 14:51.
- 14:51 fresh lib census: **740 crates, 638 compilable, 1 oversized (`plugin`, 128 files / 2,073,206 B / 27,863 lines, now a carvable leaf).**
- PLUGIN-INTEGRATION-1 status: groups G1–G8 are applied (all of WP3 and most of WP4); G9 and later are verifying in its differentials.
- 15:0x PRODUCT-FILE-SPLITS done: 9 pets/print/presentation/quiz/teaching files split into children ≤961 lines; test-title multisets identical; Rust differential identical. It also fixed the presentation-core vitest `include`, which had been matching zero files. Its 56 new folders go to tx3, which picks them up because it scans everything born since 10-15.
- 15:xx HUB-SCRIPT-SPLIT done: **the hub `📜️script.ts` went from 17,556 to 80 lines** (36 modules, largest 1,074 lines). 79/79 parity runs identical, tsc clean, 442/442 declarations lossless, no cycles (the old dev-owner↔script cycle is removed). Final SHA-1 `642aab0f26c8`; a peer re-saving a stale buffer would resurrect the monolith. New folders go to tx3.
- 15:1x PLUGIN-CRATE-HYGIENE done statically (RESULT-PLUGIN-CRATE-HYGIENE.md):
  - The 117 "orphan" manifests came from finished carves. The real defect: every artifact workspace kept `**/🏅️standards/**` in `exclude-patterns`, so repo member discovery never saw the carved crates. The generator is fixed and 45 workspaces are repaired; 510 crates were hidden in total.
  - Shared files: 116 → 22.
  - `--check-applied` now takes `--base=<ref>` (with HEAD it was vacuous after auto-commits) and also fails on hidden members and on dual owners.
  - About 30 more locks were hand-patched. The framework lock is missing the space artifacts and the cells crate.
  - Open: the generation-native template sharing needs a design change (move the shared DSL records, abstract the snapshot behind a trait).
- 15:2x Dispatched SHARED-FILES-3 (`sf3`): the generation template design, the generation3d test supports, the framework test helpers, and the pch/okwp8 hand-offs.
- 15:2x TAXONOMY-SWEEP-3: nothing to register. All 6,235 folders born since 10-15 resolve (0 unresolved, 0 ambiguous). The 1,141 unresolved folders outside dot paths are older debt. Re-check tool: `tx3-tools/` (~20 s).
- 15:3x OS-KERNEL-WP8 done statically (RESULT-OS-KERNEL-WP8.md):
  - The 12.6k-line store unit file is gone. Its tests now live in 14 test crates, with 2 helper crates (testing-fixtures and testing-doubles).
  - The test multiset is identical: 661 → 661.
  - D5 is done: the kernel takes the fixtures crate as a dev-dependency.
- New launcher refusal: `bun repo cargo` now fails at its own start with `Exact canonical persistent Native publication required` (a peer launcher change). Every compile is still blocked, now by two separate gates.
- 15:4x OS-PACK-DEDUPE done statically: `pack-record-schema` is now the single owner of the schema graph, `schema_hash` and the wire vocabulary; the `test_allocation` wiring is fixed; the os-pack test closure is back inside the gate (16 / 261,382 / 3,637).
- Dispatched OS-PACK-RECORD-CODEC-DEDUPE (`orc`): os-pack's copy of the whole record codec (~130 names) and the framework `TAG_*` constants defined twice.
- Dispatched INFRA-UNBLOCK-3 (`iu3`): the launcher publication refusal, plus a preview-only diagnosis of the peer's retirement script failure.
- 15:1x INFRA-UNBLOCK-3 (RESULT-INFRA-UNBLOCK-3.md): LAUNCHER REACHES CARGO: no.
  - Refusal 1: the peer's uncommitted Native storage contract work (15:02–15:08) made the `ImageReceipt` schema require a `mountDirectory` claim. The live receipt `⚡️cache/rust/fixed/image.json` (from 10-14) lacks it. Republishing it is a physical creator or retire step that belongs to the peer or Root.
  - Fixed: the headroom script's `withNxToolingUseV1` call was using the old signature (one line, authority checks intact), so the preview now gets past `Live Original resource control issuance required` and stops at the receipt refusal.
  - Refusal 2: headroom is still 32,124,928 B short.
  - Both blockers belong to the peer, Root or the user.
- 15:2x PLUGIN-FILE-SPLITS-A done statically:
  - The bim, process3d, raster and three norm files became 33 children, all ≤1,270 lines, with the norm seams aligned. Differential identical.
  - The bim patches split happens in its generator, which still produces byte-identical output.
  - `--check-applied` exits 1 for en1999, en1993, vdi3805 and process3d with the same numbers when the split is reverted, so the failure predates it. en1990 fails too and was not touched.
  - Follow-up: the vdi inference descriptor's embedded Rust facet now holds only the parent file. Re-point it with `readModuleTreeSource`.
- 15:3x MONOLITH-CENSUS-3 against census 2:
  - files over 1,500 lines: 334 → 267;
  - lines in those files: 912,154 → 642,759;
  - files over 1,000 lines: 702 → 658;
  - 10 of 11 spot-checks confirmed.
- Dispatched:
  - PLUGIN-GATES-GREEN (`pgg`)
  - RUST-FILE-SPLITS-8 (`rf8`): repo statutes/cli verbs/events/metrics/move, the generation3d composition tests, the spatial-kernel session
  - TS-FILE-SPLITS-8 (`tf8`): test-harness orchestration and test-platform, the spatial-kernel preview
- 15:4x PACKAGE-SPECIFIER-CODEMOD done: framework-schema 102 → 11, schema-validator 239 → 132, value 146 → 17; production edges 269 → 111. 277 importers and 36 package.json files rewritten, a new package `@semio-tech/schema-ts`, 39 facade entries. bun.lock was transplanted by hand: confirm with the `workspaces-check` route. The codemod was resumed for the ts3 os-script edges.
- 15:5x HUB-DIRECTORY-CAPABILITIES done statically: `HubDirectory` became 15 capability traits plus a `PrincipalAuthority` extension trait. The 263 forwarders and 17 dead default bodies are gone. About 24 consumers lacked trait imports, left over from the wave-4 carve; fixed. Typeck oracle over 19 configurations: 0 new errors; 8/8 mutations caught.
- 16:0x RENDERER-TEST-DEPS done statically: crates with an over-limit `ui` dev row 66 → 4 (63 dev edges replaced by 84 edges on 21 producers; 599 test paths re-pointed; production untouched). 588 test uses of aggregate-only items remain (`input`/`events` 496); these are peer ui-aggregate modules with no producer. Foreign: a renderer async-boundary test embeds a deleted ui file; 40 renderer lock packages are stale.
- 16:1x TOOL-TEST-COMPANIONS done statically: tool-run and tool-machine have 0 over-limit test units. The companions `-tests` mount the existing files, all 28 fixture reads happen at run time, and the test multisets are identical.
- **Oracle defect:** the union-oracle regex left path heads after a single `:` unresolved, which hides privacy errors in dense one-line code. Fixed in `ttc-tools/`. Earlier "NEW 0" results from forks of that oracle may be blind there; sent to PRIVACY-ORACLE-SWEEP to re-run.
- Compile window: regenerate `🔒️dependencies.json` (`verify dependencies write-baseline`) for all new manifests (checklist item 2).
- 16:2x TAXONOMY-DEBT done: non-dot unresolved findings 1,141 → 28. 1,049 fixed by JSON rows; 64 folders renamed for emoji rule violations, with all references updated; 7 stdio tests got their fixture paths fixed. 5 folders changed kind id (4 cascades from newly resolved CAD parents, 1 peer edit). Left: 9 generated/vendored trees, which need an engine edit to the exclusion list pinned at 3 ids, and 18 in running slices or addressed by key.
- 16:3x TS-FILE-SPLITS-8 done: test orchestration 2,609 lines → 66-line parent plus 9 children; test-platform became 4 topic tests (110/15 identical); spatial preview became 225 lines plus 7 children (1,672-call differential, 0 mismatches). tsc and the failure sets are unchanged.
- 16:3x `verify dependencies` (read-only) is clean: 303 third-party dependencies against baseline 303; literal-external 300; oracle-conflicts 4. No `write-baseline` is needed (log `🗑️generated/coord-verify-deps.log`).
- 16:3x Dispatched PACKAGE-SPECIFIER-CODEMOD-2 (`psc2`: the cross-package relative family, ~5,299 production edges), HYGIENE-RULES-2 (`hr2`: no-core-path ~607 and not-to-unlisted ~144) and TEST-FIXTURE-ENTRIES (`tfe`: cross-module fixture reads). 14 executors are running.
- 16:4x OS-PACK-RECORD-CODEC-DEDUPE done statically: the os-pack record codec copy is deleted (14 files, 31 consumers re-pointed); duplicate definitions 134 → 9. The stale copy had a latent `Shape::Ref` encode bug, fixed by using the owner. The `pack-record-token` crate (an older copy) is deleted; `pack-record-schema` is the single owner, guarded by the new law `🪪️single-owner`. os-pack is down to 2 files / 228 lines. Differential: 0 unexplained errors across 18 configurations.
- 16:4x TSC-UNBLIND done (RESULT-TSC-UNBLIND.md):
  - The 905 tsc syntax errors came from a sweep (`251fa724c4a`) that corrupted template strings in the puzzle 3d/5d `🔺️diff` files (13 spans restored), plus one invalid cast in a layout test.
  - Type checking now runs and shows 2,722 type errors in 880 files.
  - Families:
    - repo library: 668, parent-held
    - half-done Binary64 migration: about 270
    - workspace packages missing from `node_modules`: 153 TS2307 + 67 TS7016, which cascade
    - stdio brep: about 50
  - Dispatched TS-TYPE-ERRORS-1 (`tte`).
  - **The user must decide:** a `bun install` to link the missing workspace packages. It downloads third-party packages, so it needs permission.
- 16:5x RUST-FILE-SPLITS-8 done statically: 5 repo crates (statutes, cli verbs, events, metrics, move) and the spatial-kernel session became 55 children, largest 816 lines. The generation3d tests use `include!` fragments, because the law fixture pins exact test paths. Differential identical. Foreign: 6 suites in the composition fixture have stale sha pins.
- 17:0x OS-TEST-COMPANIONS done statically:
  - 9 crates now have 12 companion test crates, plus a new `os-kernel-dsl-derive-expand` crate (a proc-macro crate cannot export test hooks).
  - 0 over-limit test units on native, wasm32-unknown-unknown and wasm32-wasip2.
  - Test multisets identical.
- Coordinator fix: the `os-channel` manifest lacked `semio-framework-io-control`, which its `📦️archive/🛬️decode` production code uses. One line added; the path is verified. Its `Cargo.lock` entry still needs the offline lock update.
- 17:1x SMALL-FIXES-1 done:
  - Composition sha pins repinned (`sfx-tools/repin.ts`), and `testCompositionOwnership` passes.
  - The renderer async-boundary test now reads the carved draw files, so the census no longer refuses the renderer lib-test. It is measurable now, and **35.5× over native**.
  - The stdio testing root went from 152 to 14 lines.
  - The mount-emitters law message was reworded.
- Dispatched RENDERER-TEST-BINS (`rtb`) and WIRING-BUDGET-ROOTS (`wbr`: energy viewer, simulation-session, remodel vectors).
- 17:2x FRAMEWORK-TEST-COMPANIONS done statically: 12 crates (print, pixels, artifact-space-collection, async, pack-json, mesh-engine, quiz, server, io-base64, ui-host, artifact-workflow-run, value) got 11 companions. All 23 are inside the gate, the test multisets are identical and the fixtures byte-identical. The ui-host manifest lacked `job-worker` and `job-worker-authority` for its window tests (fixed as companion dev-dependencies). `print-tests` reaches the oversized `plugin` closure, as it already did at HEAD.
- 17:3x PACKAGE-SPECIFIER-CODEMOD (resumed) done: the os `🎛️commands` and 4 straggler files now use `@semio-tech/schema-ts`. framework-schema: 7 left, all renderer tests. schema-validator rose to 137 from new peer `📚️library` edges.
- 17:4x PACK-QUIZ-FILE-SPLITS done: pack value, pack catalog and quiz schema are now 50–57-line parents plus children (differential identical, typeck-level stubs). Quiz home-grid, pet-companions and session split (923/923 tests). Left: the `QuizSession` class (857 lines, 40 mutually calling private methods) needs a design change, a collaborator split.
- 17:5x TS-TYPE-ERRORS-1 done: tsc 2,722 → 2,454.
  - The 3d diff is finished to the Binary64 end state.
  - 50 diff-JSON `$ref`s were missing their `#`.
  - The diff parsers now refuse unknown fields.
  - **167 generated `📝️text` parsers rejected every string. Fixed**, with a new Ajv-oracle test (205 pass).
- **Coordinator ruling (Binary64):** Binary64 words are the model and wire representation (schemas, snapshots, diffs, mutations). They convert to `number` exactly once, at the computational boundary, through one framework helper.
- Dispatched TS-TYPE-ERRORS-2 (`tt2`): draw, the 5d diff, the xml doctype validator, and the generated diffs' missing parse calls.
- 17:5x QUIZ-SESSION-DECOMPOSE done: `QuizSession` is now a 254-line composer plus 12 collaborators (≤160 lines each), with the public API unchanged; 926/926 tests, including a new playthrough test. **Alert 17:54:** `loadTaxonomy` throws on `subsetDirectoryOverrides…["base"]`, most likely the HR2 stdio base rename half-applied; `taxonomy.json` has mode 600. HR2 has been told to fix it atomically.
- 17:5x Taxonomy loader healthy again at 17:54 (file mode back to 644). WIRING-BUDGET-ROOTS done: `lib-wiring-budget` 3/3 passing (simulation-session root 1,620 → 319 lines). `--check-applied` exit 0 before and after.
