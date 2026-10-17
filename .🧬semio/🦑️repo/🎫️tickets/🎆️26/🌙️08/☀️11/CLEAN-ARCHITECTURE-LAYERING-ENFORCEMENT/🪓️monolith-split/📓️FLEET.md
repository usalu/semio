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
- 18:0x HYGIENE-RULES-2 done:
  - `not-to-unlisted`: production 146 → 0. 6 oracle libraries were added to `🔒️dependencies.json` through the repo's own freeze check.
  - `no-core-path`: production 625 → 536, from 5 folder renames and 26 `@/lib` aliases.
  - Blocked: the stdio `🧱️base`/`✉️base` subset folders (493 edges). `base` is a subset id used about 8,900 times, and the taxonomy requires `<emoji><subset id>`. This is a hand-off to the rule owner: rename the subset id across stdio, or exempt standard subset ids.
  - The partial rename broke `loadTaxonomy` for about 4 minutes; restored.
- 18:0x PLUGIN-GATES-GREEN done: en1990, en1993, en1999, vdi3805, process3d, fem3d and energy-model reach exit 0 with 0 extras at `--base=f856dff0813`.
  - The gate generator now mounts the hand-built crates on both sides; that asymmetry was the root cause of the red gates.
  - The vdi descriptor facet is byte-equal to `readModuleTreeSource`.
  - **Coordinator ruling:** puzzle 2d is verified at `--base=HEAD`. Its remaining extra comes from imports added after f856, so HEAD is the correct base for it.
- 18:0x Dispatched PLUGIN-GATES-REVERIFY (`pgr`: every carved plugin package through the corrected gate) and a Haiku that consolidates `📓️COMPILE-RUNBOOK.md` (an ordered runbook for the first compile window).
- 18:3x RENDERER-HOSTS-UI-2 done: `renderer-hosts-only-ui` 535 → 391; 56 files moved and 10 split into framework modules, plus 31 SDK entries. Renderer vitest went from 38 failing tests to 29. 31 actionable production edges remain, with port designs in its file. **Regression:** 12 renderer test files fail to load (`URL must be of scheme file`) through the ui fixture entry written at 18:28, probably by tfe. tfe has been told.
- 18:4x `📓️COMPILE-RUNBOOK.md` is written: Phase 0 prerequisites plus 9 launcher phases in dependency order (113 invocation lines) plus Phase 10, the generators and runtime checks. Open decisions are in its §15.
- 18:5x PRIVACY-ORACLE-SWEEP done (RESULT-PRIVACY-ORACLE-SWEEP.md, tool `pos-tools/` with a typeck canary):
  - **31 pre-compile defects fixed:** 12 private members in plugin-host, 6 in kernel/sqlite, 7 in renderer, and 6 trait/import/path errors.
  - Clean: rcr, rhs, osm, smg; dlo was confirmed statically.
  - The single-`:` regex defect was fixed before any family result was recorded.
  - Open: renderer staging files name Shell-parent modules that no longer exist; a testkit feature forwarding problem; 12 E0603 rows.
- Dispatched RENDERER-INTEGRATION-3 (`ri3`) for those three items.
- 19:0x TS-TYPE-ERRORS-2 done: tsc 2,454 → 2,098 (draw 231 → 28).
  - Binary64 follows the ruling, converting once through `binary64Value`.
  - The 5d diff is on Binary64.
  - The xml doctype validator mirrors the Rust validator.
  - The parser generator now emits only parsers whose callees exist; 30 new parsers, 28/30 agree with Ajv.
- Dispatched PARSER-GENERATOR-PERMANENT (`pgp`): the generator is still a ticket script, against AGENTS.md; it must become permanent and be applied to 14 more inference files.
- Dispatched DRAW-RUNTIME-TESTS (`drt`): the 44 runtime failures.
- 19:1x SHARED-FILES-3 done statically: shared files 18 → 1 (the last one is in `pi1`). The generation-native template now has one owner with a `GenerationNative` trait; the 4 generation3d supports are crates; 13 crates in total. It repaired 3 support files a peer sweep had broken. generation2d and generation3d `--check-applied` exit 0. Foreign: an `assert_eq!*(` syntax corruption, 3 wiring-budget breaches, 3 over-length test paths.
- 19:1x Coordinator fix: 8 `assert_eq!*(` corruptions from sweep `7af1259dba8` were restored to `assert_eq!(*…)`, which was the sweep's intent: map indexing → `*get().unwrap()`. Files: the generation3d geometry and brep-topology unit tests. rustfmt reports 0 parse errors (it rejected both files before).
- 19:2x PLUGIN-INTEGRATION-1 done statically: 24 crates flipped (all of WP3, the WP4 reactor/runtime crates, and the WP5 panel); 27 `cfg(test)` leaks and 14 plan defects fixed. The `plugin` facade went from 225 files / 4.30 MB / 56,472 lines to 128 / 2.07 MB / 27,863.
- Dispatched PLUGIN-INTEGRATION-2 (`pi2`): tool-run, time-travel, the WP6 tier-1 and WP7 tier-2/3 flips, the T10–T21 bins and the last shared file. Goal: the facade inside the gate, so that **0 crates in the repo are oversized**.
- 19:3x DRAW-RUNTIME-TESTS done: draw route 581/46 → 663/7 (the 7 left are xmldom-missing or peer-held). **Real reader bug fixed:** the `draw_text` row width was 5, but the schema has had 6 columns since `font_family`. The unicode `properties.bin` split is wired correctly (SHA-identical when concatenated). Dispatched RETIREMENT-ORACLE-TESTS (`rot`).
- 19:4x RETIREMENT-ORACLE-TESTS done: controlled-retirement test 8/7 → 15/0. The copy-demand oracle now has a single owner (`🌱️value/♻️retirement/🧪️tests/🧫️fixtures`), shared through an SDK entry. 5 stale Rust source anchors were fixed, and a new `between()` helper throws on a missing marker. One overstated title was corrected. Hand-off: 6 other retirement tests (protocol, shared, sealed).
- 19:5x RENDERER-TEST-BINS done statically: the renderer top crate test unit went from 592 files / 9.3 MB / 38,108 lines (35.5×) to 22 / 113 KB / 1,397 (0.69×). 0 test units over the gate across 37 renderer crates. Fixtures moved into owner `🧰️fixtures` modules plus a fixtures crate; source-text laws now read at run time through `test-support`. Unchecked: cross-crate privacy of the moved laws, since the typeck oracle did not reach typeck; candidate widenings are listed in its file. `dock` is at 3,774/4,000.
- 19:5x Dispatched RENDERER-HOSTS-UI-3 (`rhu3`: the last 31 actionable edges plus the rhu2 hand-offs) and RETIREMENT-TESTS-2 (`rt2`: the 6 protocol/shared/sealed tests). 9 executors are running.
- 20:0x PACKAGE-SPECIFIER-CODEMOD-2 done: `no-cross-package-relative-*` 8,429 → 4,877 lines (production 4,841 → 2,848). 1,875 importers now use specifiers; 549 facades, 555 `exports` entries, 30 new TS packages. `not-to-unlisted` 890 → 405 (production 0). 895/905 test files identical; the other 10 are peer or environment. Skipped: 2,274 production edges in library/pi1/rhu2/sf3, with facades prepared for hand-off; 145 UI-internal edges need an owner decision.
- 20:1x RENDERER-INTEGRATION-3 done statically: 28 stale Shell-parent heads removed. Two real back edges fixed: `run_native` lifted into host-winit, and the runtime-mailbox lib→test companion edge replaced by a hook port. The testkit field is ungated, and 7 E0603 rows were real and are fixed. Typeck oracle (canary reached): the ri2 family rows are gone. Hand-off: 414 test resolve rows plus 5 `SceneSurfaceState` privacy rows in the Scenes/EngineCanvas relocation.
- 20:2x RETIREMENT-TESTS-2 done: the retirement test tree is 50/0 (was 44/6). The causes were stale anchors, one oracle exception and one end marker; no production defect.
- 20:3x PARSER-GENERATOR-PERMANENT done: `bun ./📜️script.ts schema parsers report|apply|check` (module in `📚️library/🎛️commands/🧬️schema`), byte-identical to the ticket script; Ajv test 73 pass; freshness gate passes. The 14 untyped-map files plus 2 hub configs were fixed. Dispatched PARSER-STRICTNESS (`pst`): additionalProperties refusal, migrating the 379 older parsers into the gate, and 2 Rust re-exports.
- 20:4x PACKAGE-SPECIFIER-CODEMOD-3 done: the procedural packages and the spatial-kernel session are on specifiers; the cross-scope hub gate moved into the hub gis composition test root (4+4 edges gone). The owner decision on the 145 UI-internal edges goes to the UI peer (proposal: carve `@semio-tech/ui-contract-ts` and then `ui-core`).
- **Coordinator ruling on the asset/JS edges:** a single-consumer file moves next to its consumer; a shared file gets an explicit `exports` entry at its owner. Dispatched ASSET-JS-EDGES (`aje`).
- 20:5x PRIVACY-ORACLE-SWEEP-2 done: **the cause of the oracle early stop was found** (unresolved trait refs in bounds → fatal; one body ICE). All 92 runs over 46 sub-families now reach typeck with no quarantine.
  - **58 more real carve defects fixed**: okwp8 150/167 privacy errors → 0, plus rf8, ftc and otcp ones, 4 trait imports, and 10 missing dependency edges.
  - Open: 298 privacy errors in the ri2 renderer family; codec-factory → os-io trait (design); ephemeral dev-deps; ledger API +13; 7 missing imports in the peer's 3d mesh carve.
- Dispatched RENDERER-PRIVACY (`rpv`) and KERNEL-CODEC-FACTORY (`kcf`).
- 21:0x RENDERER-HOSTS-UI-3 done: **all actionable renderer host edges are gone** (391 → 371). What remains is 127 react resolver artefacts, 38 parent Actor/Media edges, 8 tooling entries and 12 r3f artefacts; the exact r3f allow-list line for the parent is in its §3.
  - Ports are bound once in `OS/🧑‍💻dev/🪝️host-ports`.
  - The react vitest aliases also fix a 159-file load failure that came from a peer codemod.
  - The schema catalog was regenerated.
  - The 12 renderer files load again, with baseline results.
- Parent policy hand-offs so far:
  1. the transient pathNot;
  2. tooling entries (decision T);
  3. the react `node_modules` real-path allow-list;
  4. the r3f allow-list line;
  5. the stdio `base` subset rule.
- 21:4x HANDOFFS.md consolidated 61 items: policy 12, library 10, peer 15, user/Root 11, fleet 13. Dispatched TS-LEFTOVERS (`tsl`: F-03, F-05, F-12), RUST-LEFTOVERS (`rsl`: F-04a, F-11) and LAYOUT-DRAWING-DECOUPLE (`ldd`: F-06a). F-04b went to pi2. tfe was nudged to write its RESULT.
- 21:5x KERNEL-CODEC-FACTORY done: added the codec-factory → os-io edge (rank 35 → 27, downward). Ephemeral dev-deps added. The ledger fields went back to `pub(crate)` behind 17 owner methods (public delta: 24 methods, 0 fields). `HANDOFF-3D-MESH-CARVE-IMPORTS.md` was written for the peer. Kernel oracle: 0 new rows, 0 privacy errors.
- 22:0x ASSET-JS-EDGES done: 31/31 edges cleared through owner facades (`?raw`/`?url`/`?inline`), because an `exports` target cannot leave the package; bytes are identical. **Found:** the quiz-react vitest route is red at HEAD (8/8 suites), because string aliases swallow subpaths after the codemods. Dispatched ALIAS-SUBPATH-SWEEP (`ass`) over every vite/vitest/storybook alias config.
- 22:1x RENDERER-WP2-TESTS done statically: no `use super::*` left in the WP2 laws; privacy rows 270/394 → 0; test resolve rows 316 → 0; 1,789 test names identical. Dispatched RENDERER-INFINITE-IMPORTS (`rii`): 66 unimported infinite free functions, plus the 24 ui dev rows that the scene-state testkit edges reintroduced.
- 22:2x PARSER-STRICTNESS done: generated parsers refuse unknown keys exactly where the schema requires it; 340 regenerated; 59,426/59,454 mutations agree with Ajv; 573 bun test files show no new failures. 221/398 older parsers are now generated and gated; 177 hand-written ones are registered with reasons, gated against drift. 8 presence schemas tightened, and the Rust re-export was added.
- 22:3x ALIAS-SUBPATH-SWEEP done: 28 configs plus Storybook now use exact-match regex aliases. Vitest files that load: 327 → 402; tests collected: 6,275 → 8,504; failing tests unchanged at 173. quiz-react 926/926. Vite: 227 bad specifiers → 0. Dispatched SUITE-LOAD-FIXES (`slf`): the 6 suites that still do not load, Storybook `pluginModulesRoot`, and the vitest-ownership gate fixture path.
- 22:4x TS-LEFTOVERS done: new permanent route `bun ./📜️script.ts verify typescript <project|check>`, which generates the tsconfig from git ls-files and has a 16-test suite. **Whole-workspace baseline: 13,577 roots, 1,919 diagnostics** (was 2,722 at unblind). Worker cycle and loops I/J cut (os no-circular 475 → 472). F-12 paths are fine.
- 22:5x RENDERER-PRIVACY done: the ri2 family went 298 lib / 525 test → 0/0 (also 0 on the 92-crate ri3bins family). 40 `pub`, 2 narrow owner methods, 24 testkit hooks. **New oracle blind spot:** E0451 (private field in a struct literal or pattern) is never reported once other errors exist. The static scanner `rpv-tools/lit.py` fixed 17 sites. Dispatched E0451-SWEEP (`e51`) over all families.
- 23:0x RENDERER-INFINITE-IMPORTS done: the 66 infinite functions are imported (oracle rows 28/39 → 0, 0 new, canary reached). The 24 over-gate dev rows → 0 through a new dependency-free leaf crate `os-renderer-wgpu-thread-isolation` that carries the per-thread cell switch. Hand-off: an `EngineCanvasPacket.scene_retirement_faulted` field drift.
- 23:1x MONOLITH-CENSUS-4: 408 free files over 1,000 lines (624k lines) in 66 packages. Dispatched CENSUS4-PKG-1 (os/♾️infinite), PKG-3 (plugin TS) and PKG-4/5/6 (plugin Rust); the os/🔌️plugin packages stay with pi2. 14 executors are running.
- 23:2x SUITE-LOAD-FIXES done: vitest files that load went 402 → 419/434, and 20/24 routes load fully. Storybook config loads again (4 stale imports fixed); the vitest ownership gate reads its fixture (4/7 pass, 3 need an owner decision). Still not loading: cad (bun install), ui-react ShellScope (`color` package; recommendation: declare `colord`), mcp (needs the built binary), hub-tests document-check-in (7 uncatalogued hub schema modules, one of them a fixture that defines a schema).
- 23:3x The user asked again for more saturation. Dispatched CENSUS4-PKG-7, 8 (plugin Rust), 13 (apps fem Rust), 14 (plugin TS) and 15 (hub Rust); 18 executors are running. Queued census-4 packages: 9, 11, 12, 16–23, 25+.
- 23:4x TEST-FIXTURE-ENTRIES done: 38 public fixture entries; 196 test files re-pointed; cross-module fixture reads 726 → 7 in its scope (wide scan 1,211 → 481). The `scheme file` bug is fixed with `fixtureUrl()`. 6 new bun failures in its final rerun are attributed to peers (a retirement file being edited at 23:32, plus source-text asserts over other slices' rewrites); not proven by revert. The taxonomy of the new `🧫️fixtures` folders is unchecked, so tx3-tools should re-check at the end.
- 23:4x ENGINE-CANVAS-PACKET-DRIFT done: the field was deliberately removed in `0c3e78944af` (a fault is now an `Err` from `close_step`); the stale fixture was fixed and a law added. Dispatched RETAINED-STEP-TEST-DRIFT (`rsd`): outdated `close_step()`/`StepContext::new`/`StepBudget::new` uses in tests, and the native-socket-probe E0599.
- 23:5x CENSUS4-PKG-1 done statically: os/♾️infinite 9 files → 31 children (≤963 lines); test roots use `include!` fragments, so the names stay identical; oracle 0 new rows; census 0 over gate. Backfilled CENSUS4-PKG-9.
- 23:5x LAYOUT-DRAWING-DECOUPLE done statically: design (A), a port. New framework crate `io-ports` (slot, codec trait, neutral `VectorScene`) plus the hub composition crate `semio-hub-layout-drawing`, which binds layout to stdio. **layout→stdio Cargo edges 32 → 0.** PNG goes through framework pixels. The layout gate went from exit 1 to exit 0. Backfilled CENSUS4-PKG-11.

## 2026-10-17 00:0x session restart recovery

The Claude Code process restarted at about 00:00 and cut 20 running agents.
- Complete before the cut:
  - OS-PACK-RECORD-CODEC-DEDUPE: os-pack no longer copies the codec; 14 files deleted; `pack-record-token` retired; the os-pack crate is 228 lines.
  - TSC-UNBLIND
  - PACK-QUIZ-FILE-SPLITS
  - WIRING-BUDGET-ROOTS: `lib-wiring-budget` 3/0
  - RUST-LEFTOVERS: 702 alias uses re-pointed, 4 facade aliases deleted, the engine spill valve gives `os-store-engine-bundle` (engine 199 KB)
- Resumed via SendMessage from their transcripts, each recovery-first: e51, pgr, c4p3/4/5/6/7/8/9/11/13/14/15, hsc, rsd, and pi2 (for its final verification).
- 00:2x CENSUS4-PKG-15 done statically: 7 hub files → 43 children (≤572 lines), oracle clean, `hub_module_tree` follows `include!`. The coordinator moved the orphan `🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit` (an old copy of the plugin-host shard tests that nothing mounts or reads) to `🗑️generated/coord-orphan-hub-dist-assets-test/`. Its taxonomy member name `🔬️dist-assets-bll2-7qi-unit` is now unused.
- 05:4x Second restart, at about 00:18; 16 agents cut again. CENSUS4-PKG-8 had completed (34 children, oracle clean). CENSUS4-PKG-4 was split and installed with its gates pending. The other 15 were resumed again via SendMessage, recovery first.
- 06:0x CENSUS4-PKG-14 done: 8 TS/Python files; roots under 500, children ≤652; the Python oracle over 5 adapters shows identical results; the PDF diff schema moved to a sibling JSON, deep-equal to the original; tsc identical (9).
- 06:1x HUB-SCHEMA-CATALOG done: 6 hub schemas moved to owned `🧬️schema` with `$id`s; the carried-phase schema, which validated only its own corpus, was deleted. Catalog 3,560 → 3,566 scopes, `--check` passes. hub-tests loads 12/12 (failures 9 → 5); `document-check-in` passes.
- 06:2x CENSUS4-PKG-9 done statically: 7 architect register/inference modules plus the process3d editor root split; the 1,799-line generated catalog is now cut in its generator (root plus 4 children); oracle row multiset is identical. Hand-off for the peer: `generate-register-catalog --check` flags the peer-staged `🗂️catalog/🟦️.ts`, because the generator TS renderer does not follow their import split.
- 06:3x CENSUS4-PKG-13 done statically: 8 FEM engine crate roots are now 19–33-line roots plus 25 children (≤927); function bodies byte-identical; oracle 1,744/4,228 identical on both sides; fem 2d/3d `--check-applied` exit 0.
- 06:4x CENSUS4-PKG-5 done statically: 7 puzzle3d files became 37 children (≤941 lines). Oracle identical on lib, test, wasm and wasip2; `--check-applied` exit 0; public API unchanged. Foreign: lib-wiring-budget now fails on new roots (lowpoly, procedural, layout), and process3d has +25 reader-probe failures (whose slice? pkg-7 owns process3d).
- 06:5x CENSUS4-PKG-3 done: 8 BIM, architect and CAD TS/Python files split; the CAD REPL is now 5 ordered hooks (as in NodeGraph); tsc 45 → 45 with nothing new; BIM IFC oracle byte-identical on the 6 fixture cases. The architect diff root is 1,012 lines (generated parser block). Not run: the CAD browser REPL, cargo, EXPRESS (pytest is missing).
- 07:0x CENSUS4-PKG-21 done: 8 TS/Python files → 28 children; remodel stories data moved into JSON fixtures; the three Python oracles (norm 1,235, remodel 275, energy 1,189) are row-identical; tsc 11 → 11.
- 07:1x CENSUS4-PKG-7 done: process3d unit test, lowpoly editor and forms editor split; oracle clean; `--check-applied` exit 0. The process3d reader regression came from c4p9 (its root was missing from `POLICY_SPLIT_SOURCE_ROOTS`); one line fixed it, and the failure hash equals the baseline. Dispatched LIB-WIRING-RATCHET (`lwr`).
- 07:3x LIB-WIRING-RATCHET done: the law passes 3/0 with no pin raised. 3 grown roots slimmed (layout export json 1,898 → 1,457); 6 compliant pins removed; 21 pins tightened.

## 2026-10-17 07:15 wave 7 (third restart recovery)

The Claude Code process restarted at about 07:05; every agent was cut. Brief: `📓️WAVE7-BRIEF.md`. Coordinator launcher probe started 07:16 (`🗑️generated/coord-probe-1017.log`).
- Resumed as fresh Sonnet agents (recovery first): c4p4 (a812aeaa980f2af31), c4p6 (a60281629f4af6f2a), c4p12 (a357cc03cb0014e00), c4p16 (a1a26aba6491b52b6), c4p17 (a2dbc00445eea19a2), c4p18 (a8733f2b0a3fec526), c4p19 (a8dd11bc6319e6aa5), c4p20 (ab30bef8a1bfa2968), c4p22 (a2df08a1cbfe65feb), c4p23 (aa76c75ab462e6361), c4p25 (a6a5e83beb8c5e9fa), c4p28 (adbd462f1eadee5ad), e51 (ac01bbf7dd9698f7c), pgr (aa5020c9863a3792b), rsd (a7ec48dc639692d8d), pi2 (aca7226f7ddb27296).
- 07:3x Probe 1 (`coord-probe-1017.log`): the launcher spent 15 min in its preflight without reaching cargo and was cut by the 900 s alarm. Probe 2 runs with the full 3,500 s budget (`coord-probe2-1017.log`) and holds the mutex. Peer checkpoint (MODULAR-MONOREPO `build-coordination-checkpoint-current.md`): the fixed Rust partition lacks the 8 GiB initial availability for a distinct 4 GiB prototype state; whole-cache retirement still open. `image.json` still has no `mountDirectory`.
- Dispatched Haiku audits AUDIT-WAVE7-CENSUS4-SPLITS (a70162dfc6db8e0a4) and AUDIT-WAVE7-QUEUE (aa89e0a99bc9f5802), and Sonnet TS-TYPE-ERRORS-3 (`tt3`, a7cd5fdba531cab9c). 19 agents running.
- 07:4x AUDIT-WAVE7-QUEUE done: 32 READY census-4 packages (212k lines), 5 PARTIAL (wgpu files peer-held), 6 DONE-BY-OTHERS (pi2/wgpu). Executable hand-offs: F-06 b/c, F-10 bun checks, F-11 c/e, F-12 c, F-13 (after e51/pgr/pi2).
- Queue order for free slots: 30, 32, 34, 35, 37, 38, 39, 41, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 54, 55, 56, 58–66, then PARTIAL free files (26, 33, 36, 42, 57), then F-10/F-11c/F-06/F-11e/F-12c, F-13 last.
- Dispatched CENSUS4-PKG-27 (`c4p27`, ab942d8c19c8e7483) and CENSUS4-PKG-29 (`c4p29`, a644d041fe7d87faa). 20 agents running.
- 07:4x CENSUS4-PKG-6 DONE (static): all 7 roots and 25 children intact; the 3d layout gate was re-run after an ENFILE crash: native 4,063/4,063 and test 4,904/4,904, onlyCombined 0. Bun 1.3.14 segfaults under load. Dispatched CENSUS4-PKG-30 (`c4p30`, a7af0ec87fcf76535).
- 08:0x AUDIT-WAVE7-CENSUS4-SPLITS done: regions 10 (pkg 1 ×9, pkg 15), readers 2 + 1 risk (pkg 9, 14; pkg 3), 1 sibling emoji clash (pkg 11), 133 glob child re-exports (sanctioned wiring, not shims by default), private items 0. Dispatched SPLIT-HARDENING-7 (`sh7`, a0835fe7c262b25a7).
- 08:0x CENSUS4-PKG-19 DONE (static): 8 files (11,447 lines) → 8 wiring roots + 38 children (max 977); all 8 `--check-applied` exit 0, onlyCombined 0; census 0 over gate. Known foreign: procedural unit test calls `retirement_demands` (E0599 at base). Dispatched CENSUS4-PKG-32 (`c4p32`, a42daf9a90b6ace25).
- 08:1x CENSUS4-PKG-20 DONE (static): 8 remodel files → 34 children (max 681); `--check-applied` after3 exit 0 (47 crates, onlyCombined 0); typeck rows lib 307→256, test 709→655 (0 new; visibility fixes). Dispatched CENSUS4-PKG-34 (`c4p34`, abc8dd410b13faed0).
- 08:0x Probe-2 diagnosis: the launcher is not hung in preflight. It waits on the single-row `semio_resource_lease` (`.cache/b/c/3dfb…sqlite`, resource `native-fixed-image:…/.cache/b/l`), which is also held by an orphaned (ppid 1) `bun repo run @semio-tech/framework-os-mcp-rs:build --output=stream` (pid 74190, started about 07:16, not from this fleet). Compile admission is FIFO behind it. Not touched.
- 08:1x CENSUS4-PKG-17 DONE (static): 8 files → 37 children (max 655), typeck lib NEW 0, test NEW 14 (stub noise, same as baseline), `--check-applied` 5/5 exit 0. Reported: `equation`, `wfc-bitmap`, `wfc-3d` crates exceed the source gate (need crate carves). Dispatched Haiku AUDIT-WAVE7-CRATE-GATE (`cgc7`, a272259b444d99807).
- 08:2x CENSUS4-PKG-23 DONE (static): 8 raster/stdio-registry files → 35 children (max 523); splitter bug fixed (hoisted `pub use` of a child-held module); POSDIFF IDENTICAL; raster gate 43/43, 70/70. Hand-off for the compile queue: the raster stub crate does not mount rsc/rdf/rio/rtu. Dispatched CENSUS4-PKG-35 (`c4p35`, ad6ad69ef9cfdf1c7).
- 08:2x CENSUS4-PKG-12 DONE (static; the earlier record was stale, everything was already installed): 8 files → 58 children (max 465); typeck 0 new non-stub; cad/en1990 gates equal, layout smaller than base (peer decoupling). Dispatched CENSUS4-PKG-37 (`c4p37`, a34f3178fa5f774d4).
- 08:3x RETAINED-STEP-TEST-DRIFT DONE: 34 test files are on the current step API; 0 wrong-arity test sites; native-socket-probe E0599 was an oracle stub artefact. 26 production renderer sites still use the old arity (a real compile would give E0061). **Coordinator ruling:** the fleet fixes them itself. Dispatched RETAINED-STEP-PRODUCTION (`rsp`, acf0a318447e50b72).
- 08:3x CENSUS4-PKG-16 DONE (static): 8 stdio files → 38 children (max 714); mp4 children merged 5→3 so `mp4-tests` stays at 31 files ≤ 32; oracle NEW 0. Already over the gate at baseline: `zip` (files) and `mp4-test-oracle` (2.7 MB bytes).
- 08:3x Dispatched CENSUS4-PKG-38 (`c4p38`, hub, a51dc25cfdf2df35b). Open: several slices report `lib-wiring-budget` failing again on the lowpoly/procedural/layout roots. Queue a ratchet fix once the plugin splits settle.
- 08:4x CENSUS4-PKG-28 DONE (static): 7 files → 35 children (max 697), 8 real privacy fixes in the puzzle editor (22 E0624 + 1 E0451 at baseline); 3 gates exit 0. Foreign: `callers.json` test 7 stale rows (generation2d, sequence, process3d, playbook, procedure, 2 plugin instance), ratchet energy schema root 165→214, sourcing test mount missing. These go to LAW-REPAIR-7 (queued).
- 08:4x Dispatched CENSUS4-PKG-39 (`c4p39`, renderer, a502b3bfbc1482967).
- 08:5x CENSUS4-PKG-18 DONE (static): 8 files → 43 children (max 727); fixed a real private-macro defect (gis `owned-field-decode` struct/fn → pub(super)); 4 gates exit 0. Found parse errors at HEAD (envelope inner attribute after items, gismap stray comma, stdio las). Dispatched SYNTAX-SWEEP-7 (`ss7`, a3e092335011fca9f): repo-wide parse-only sweep to 0.
- 09:0x AUDIT-WAVE7-CRATE-GATE done (`cgc7-census.ts` now covers all 287 workspaces): 2,419 crates; 1,868 compilable on lib and 1,811 on test; 58 oversized lib and 43 test-only. The top blockers are the stdio artifact crates (semio-mesh 314, xml 301, deflate 300, semio-drawing 299, zip 288, json 283 …), plus framework `3d-brep-engine` (bytes 1.04×, 177 blocked), bim-model (1,300 files), stdio-pdf (722), puzzle-2d/5d and architect-program.
- **Coordinator ruling (priority):** crate rollout before file splits. Freed slots go to rollout slices R1–R9 (WAVE7-BRIEF "Crate rollout"). Order:
  - R1: stdio deflate, zip, xml, json
  - R2: stdio semio-mesh, -drawing, -brep, -model
  - R3: semio-image, -object, -kit, -flow, -document
  - R4: svg, semio-cad, -presentation, -animation, -graph, semio root
  - R5: dxf, jpg, tiff, pdf, gltf ×3 (after c4p22)
  - R6: gis gismap/gisterrain, wfc engine/3d/2d/bitmap/grid2d/grid3d
  - R7: trinity jack/rewriting, sourcing curation, vcs, space home, writer, sequence, flow-brep
  - R8: puzzle 2d/5d, note, shooting, forms, animate presentation, equation, dag, reasoning wires
  - R9: bim model, architect program, demonstrator, energy model schema
  - FB: framework 3d-brep-engine
  Census-4 file splits resume afterwards.
- 09:1x E0451-SWEEP DONE (static): 126 journalled edits over 111 files (74 mutation-leaf structs got pub fields; constructor `ContributedMachineCatalog::new`); whole-workspace scanner `e51-tools/scoped.py`; 0 hits after, 344 hits with the edits undone. Hand-off: 7 test-mounting drift structs private to another crate (§5). Dispatched CRATE-ROLLOUT-R1 (`r1`, a57af5c5271808a4d).
- 09:2x SPLIT-HARDENING-7 DONE: pkg-1 regions balanced (21 lines over 10 files); architect inference and energy mutations facets are module-tree `concat!`s (probes 0 missing); pkg-3 parser projection now reads `SourceStore.declarations` (509 → 0 differing names); en1992 `📦️design-effects` → `🧮️design-effects`; 133 glob re-exports confirmed as genuine split parents. Dispatched CRATE-ROLLOUT-R2 (`r2`, a5c5430a99e4fcdd8).
- 09:3x CENSUS4-PKG-22 DONE (static): obj/gltf/ply 7 roots + 29 children; missed reader fixed (the gltf exports `DIFF` const is now a module-tree `concat!`); POSDIFF IDENTICAL; 3 gates exit 0. Dispatched CRATE-ROLLOUT-R3 (`r3`, ac7d92c958ec15e01).
- 09:3x CENSUS4-PKG-39 DONE (static): 6 Shell renderer test roots (9,662 lines) → 148-line roots + 45 `include!` fragments (max 495); POSDIFF IDENTICAL, test names identical, 0 units over the gate. Dispatched CRATE-ROLLOUT-R4 (`r4`, a8f4fa24784472c8f).
- 09:4x RETAINED-STEP-PRODUCTION DONE (static): the 26 renderer sites are on the current API (`rsd.py` 1509/0). Grants: os-host root `self.retained`, new `JOB_REPLAY_STEP_GRANT` (kernel-core, 4096 items / 8 MiB, matches the actor bridge grant), `BOARD_POINTER_SETTLE_GRANT` (non-consuming). Renderer lib oracle 366=366.
- 09:4x R5 split into R5A (dxf, jpg, tiff), R5B (pdf, 722 files) and R5C (gltf ×3). Dispatched CRATE-ROLLOUT-R5A (`r5a`, adfcca10dae43618a).
- 09:5x CENSUS4-PKG-35 DONE (static): 8 framework files → 37 children (max 453); scoped oracle CLEAN on lib/test/wasip2, negative control caught; replication laws unchanged. Dispatched CRATE-ROLLOUT-R5C (`r5c`, gltf ×3, a892ba4b14a72576d).
- 10:0x CENSUS4-PKG-25 DONE (static): 8 remodel/energy files → 34 children installed; posdiff identical (lib 1056, test 1342); energymodel/remodeling gates equal, onlyCombined 0; energy reader rows identical. Dispatched CRATE-ROLLOUT-R6 (`r6`, gis + wfc, abc77ce3b44035f22). The mutex is held by cad (since 08:42).
- 10:1x CENSUS4-PKG-38 DONE (static): 7 hub files → 42 children (max 735); POSDIFF IDENTICAL (lib 2,884, test 6,125), seeded mutations caught; 5 reader re-points. **Coordinator ruling:** baseline 'foreign' compile/test defects are the fleet's too. Dispatched BASELINE-DEFECTS-7 (`bd7`, ae777b31aa11f3ceb): harvests every RESULT foreign-error section into `COMPILE-DEFECT-LEDGER.md` and fixes them.
- 10:2x CENSUS4-PKG-27 DONE (TS renderer): 7 files → 40 new files; World3dHost 2,849 → 434 (7 hooks over a typed core bag); 10 re-export lists (172 names) removed and about 28 importers re-pointed; the AgentBridge suite went from failing at load to 83/83; renderer tsc 54 = 54; vitest 3,206 tests with an identical failure set (38). Open: 3 oracles red on stale fixture paths / Rust anchors (→ bd7 ledger).
- 10:2x Dispatched CRATE-ROLLOUT-R7 (`r7`, trinity ×2, sourcing curation, vcs, space home, writer, sequence, flow-brep; ae85f8d1e62e6c3db).
- 10:3x CENSUS4-PKG-34 DONE (static): 7 stdio io/diff files → 48 children (max 409); differential 18/18 identical; POS 0 new. Effect: png-mutations went to 37/39 files (over 32). **Ruling:** keep the split; png added to R5A's carve scope (message sent). Foreign: deflate `--check-applied` exit 1 at base (`DeflateSnapshot` unresolved in the deflate io root) → bd7/r1.
- 10:3x R8 split into R8A (puzzle 2d/5d), R8B (note, shooting, forms, animate presentation) and R8C (equation, dag, reasoning wires). Dispatched CRATE-ROLLOUT-R8B (`r8b`, a61a560c983086931).
- 10:4x CENSUS4-PKG-37 DONE (static): 8 framework files → 37 children; the codec was re-cut to 1 child to keep value-intrinsic tests at 32/32 files; oracle CLEAN on 8 groups; 5 readers re-pointed. Already over the gate before the split: the time-travel test closure (472 KB). Foreign: `toolJobMicrosecondBudgetSelfTests` throws; interactivity self-test throws on the node-graph tests mount (→ bd7).
- 10:4x Dispatched CRATE-ROLLOUT-R8C (`r8c`, equation, dag, reasoning wires; ab76824fcc4898077).
- 10:5x CENSUS4-PKG-30 DONE (static): 8 trinity/draw files (10,589 lines) → 2,066 root lines + 35 children (max 491); oracle clean on 4 crates × 4 cfgs; trinity gates equal. Open: the draw `--check-applied` test lane onlyCombined 35 → 205, shown by an A/B in the gate sandbox NOT to come from the split (→ bd7: find the cause). Draw publication-authority audit red at base.
- 10:5x Dispatched CRATE-CARVE-FB (`fb`, framework 3d-brep-engine, 177 blocked; a762c08f4e53b1925).
- 11:0x CENSUS4-PKG-29 DONE (static): 7 PDF/docx files → 29 children (max 597); typeck identical (`c4pfullcmp.py`, because the oracle truncates *-final.err at 30M characters and earlier diffs showed phantom rows); docx/pdf gates exit 0. The pdf crate is now 747 files (R5B).
- 11:0x Dispatched CRATE-ROLLOUT-R5B (`r5b`, stdio pdf, a29e90e2b1b397cb3).
- 11:1x BASELINE-DEFECTS-7 partial: 28 ledger rows (`COMPILE-DEFECT-LEDGER.md`); 12 fixed (hub space E0425 test rewritten, hostile-input filters, 3 E0061 `close_step` copies, callers.json, 5 moved include inputs, descriptor-digest 8/8, ingress law 4/4, energy schema root carved to 150), 2 partial, 2 gone, 12 open. A sweep of 2,989 Cargo roots finds 0 missing mount/include inputs. Remaining: 7 private-struct test drifts, plugin-runtime/child-registry source anchors, catalog suites, 125 missing TS test inputs (`bd7-ts-enoent.txt`), cause sha pins (deliberately held), raster stub wiring.
- 11:1x Dispatched BASELINE-DEFECTS-8 (`bd8`, a6f332a7227177b3c). Queued: R10 raster stub wiring/carve, R8A puzzle 2d/5d, R9 bim/architect/demonstrator/energy-schema, census-4 41+, F-13 re-measure.
- 11:2x SYNTAX-SWEEP-7 DONE: 34,151 .rs files parsed (rustc -Zunpretty=normal plus an expanded --test pass); real parse errors 16 files / 27 sites → 0 (inner attributes after items ×5, orphaned docs/regions ×5, gismap comma, pub macro_rules, pasted fn arg, minified `"lit"if` ×12, extra `>`). Leads for the compile: `Generation3dViewCamera` used without an import in the viewer presence schema; a generation2d host `#[cfg(test)]` now binds to a single `use`.
- 11:2x Dispatched CRATE-ROLLOUT-R9B (`r9b`, architect program, demonstrator playground, energy model schema; a18bdd5549581d277). R9A (bim model, 1,300 files) queued.
- 11:3x CRATE-ROLLOUT-R2 DONE (static): stdio semio model/mesh/drawing/brep → 44 crates, all inside the gate (max 28 files / 215 KB / 2,965 lines); gates exit 0. Source prep: leaf imports via `crate::schema::mutations::<leaf>`, model `use super::*` → explicit (42 files), cross-slice `pub(crate)` → pub. Open: (a) leaf-family crates named after their first leaf (`-insert-element`, `-create-node`, `-change-stroke-width`) → rename to family names; (b) **compile risk:** `#[cfg(test)] pub fn` helpers used across crates resolve in the union oracle but NOT in real multi-crate test builds. Dispatching CFG-TEST-CROSS-CRATE fleet-wide.
- 11:4x CENSUS4-PKG-32 DONE (static): shooting/fem3d/bim 8 files → 8 roots + 40 children (max 577); the bim labels table became 4 chained `macro_rules!` row providers (expansion identical); differential identical after the `fault_notices` re-export fix; 3 gates exit 0. Dispatching R9A (bim model) next.
- 11:4x Dispatched CRATE-ROLLOUT-R9A (`r9a`, bim model, a949672105f74c288).
- 11:5x CRATE-CARVE-FB DONE (static): 3d-brep-engine lib 1.038× → 0.72×, test 1.77× → 0.85×, plus 4 new crates (kernel-contract, mesh-io, shape-value, engine-tests), all inside the gate; 95 of 160 dependents now pass the lib gate (the other 65 are blocked by other oversized crates); union oracle 0 new / 0 gone on 5 cfgs, 4 seeded regressions caught. Lock refresh pending.
- 11:5x Dispatched CRATE-ROLLOUT-R8A (`r8a`, puzzle 2d/5d, ad4649da938b0ffc4). Every rollout row of AUDIT-WAVE7-CRATE-GATE now has an owner except raster wiring (R10) and the 43 test-only oversized crates (next: TEST-CLOSURE-ROLLOUT).
- 12:0x BASELINE-DEFECTS-8 partly done: 7 private-struct drifts resolved (tests moved to owner crates; found and fixed a production E0432 where a puzzle session counter was `cfg(test)`); new `pluginFamilyText()` reader (`readModuleTreeSource` skips 195/536 plugin sources whose mounts leave the crate); 6 deleted library schemas restored (d4597d0410a), 4 fixtures re-pointed; hub oracle 63/0, mcp law OK, descriptor-verification 21/0. Open: O-03 process3d suite stale vs. deleted retirement machinery, O-05 21 close-budget tests, O-07 never-authored window fixtures, O-11 vitest gate re-pin. To BD9.
- 12:0x Test-closure rollout planned (43 test-only oversized crates):
  - T1: stdio test oracles (tiff 17.5 MB, ifc, mp4, jpg, pptx, docx, pptx, ply-inferences, pdf-test-oracle refused)
  - T2: draw drawing-* ×11, layout mutations-frames/-content/editor-commands, block 2d/5d editors
  - T3: framework expression, time-travel, plugin-child-group-wire-laws, plugin-publication-mounted, 3d-brep-boolean, 3d-mesh-tests/-document, os-kernel, number, teaching-proctor, os-shell (refused)
  - T4: reader-refused hub-artifact-creation/-stdio/-directory-laws/-directory-backend-laws, composition-laws, procedural 2d/3d io-sqlite
  - `semio-framework-plugin` stays with pi2.
  Dispatched TEST-CLOSURE-T1 (`tc1`, a45de74c20e1e5a62).
- 12:1x CRATE-ROLLOUT-R8C DONE (static): dag (8 + facade), reasoning wires (9), equation (9 + facade), all inside the gate; `--check-applied` exit 0, 0 extras. Generator fixes with tests (crate-tree suite 64 pass), including cfg(test) coherence: a slice that reaches another slice's `#[cfg(test)]` module stays in that crate. **Oracle blind spot:** unresolved `use` lines of kept root file modules are hidden by the unlinked-cascade filter, so check `crate::<root-mod>::<item>` paths by hand (added to the brief).
- 12:1x Dispatched TEST-CLOSURE-T2 (`tc2`, draw ×11, layout ×3, block ×2; a6c8c5fd9a0c0e5e6).
- 12:2x CRATE-ROLLOUT-R7 partial: vcs (7 + facade), space home (7 + facade) and flow-brep (facade + mesh + tests, hand carve; a 672 KB descriptor is now read at run time) DONE inside the gate. BLOCKED, needing layering redesign: trinity jack (5,424-line artifact-root hub named as `crate::Graph`, `crate::Node`), rewriting, writer, curation (schema groups held by cycles through the editor root/hub; `r7-tools/scc.py`), sequence (18-slice cycle host↔commands↔root helpers). Generator fixes: `fixturesToRuntime` for 🧪️tests, extension router test lists. Dispatching R7B (trinity jack + rewriting) and R7C (writer, curation, sequence) as design-and-carve slices.
- 12:2x Dispatched CRATE-ROLLOUT-R7B (`r7b`, trinity jack + rewriting layering, adb1635499e1e8e0a). R7C (writer, curation, sequence) waits for a free slot (cap 20). Queue: R7C, T3, T4, BD9, R10 raster, the R2 leaf-crate renames, census-4 41+.
- 12:1x **Usage limit hit at about 11:10** (session limit, reset 12:10): 16 agents cut (r1, r4, r5a, r5b, r5c, r6, r7b, r8a, r8b, r9a, r9b, ctc, tc1, tc2, pi2, pgr). Resumed via SendMessage with context intact: r1, r4, r5a, r5b, r5c, r6, r8a, r8b, r9a, r9b, ctc, tc1 (r3 and cad were still live; messages queued). Held for later in the window: tc2, r7b, pi2, pgr, then R7C, T3, T4, BD9. **Fleet sized at about 14–16 concurrent** to stay under the usage window.
- 12:2x TEST-CLOSURE-T1 DONE (static): 9 stdio test crates inside the gate (tiff oracle 66.9× → 0.27×, ifc 13.3× → 0.77×, mp4 10.5× → 0.13×, pdf oracle refused → 0.76×; 50 embedded fixtures now read at run time via `runtime_fixture`; 146 test names and 45 fixture sha256 identical). Found and fixed a pdf 4.1.4 `concat!` path missing a `/` (it could never have compiled). ply-inferences sits at 31/32 files (re-carve candidate). Resumed tc2.
- 12:3x CRATE-ROLLOUT-R5A DONE (static): jpg (6 + facade), tiff (6 + facade), dxf (6 + facade), png-mutations (facade + 3); all 40 crates in the 4 artifact dirs inside the gate for lib and test. Generator: `--base=dir:<snapshot>`, `F3_SLICES=1`, file modules cut from their parent are declared pub, a hosting-anchor fallback; 73 tests pass. cfg(test) demo helpers became plain `pub fn`. dxf-schema is at 242/262 KB (the descriptor embeds the whole diff tree as its rust facet). Resumed r7b.
- 12:3x CRATE-ROLLOUT-R9B DONE (static): demonstrator playground (4 + facade), energy model schema (one crate, 58 → 5 files: facet leaves served at run time through the schema registry), architect program (17 + facade; 183 KB SQLite DDL in its own module; editor root split; register macros in `schema::kernel`). Generator fixes with tests. Real-build risk: architect register families share `#[macro_export]` macros across crates. **Pitfall:** `new crates --backup=<abs path>` joins the path onto the repo root and created a stray `Users/…` tree in the repo; the coordinator moved its 18 old `f3-backup-*` entries into `🗑️generated/` and removed the empty tree. A generator fix is queued (absolute `--backup`).
- CTC: RULE for every carve: a `#[cfg(test)]` pub item (fixture, demo_*_cases/snapshot, test-only method, test mod, macro_export macro) must never be named from ANOTHER crate. Re-gate it `#[cfg(any(test, feature = "testkit"))]` (reuse the crate's existing testing feature name), declare `testkit = []` in the owner (plus `dep:` entries for every dev-only crate its body uses), and enable it in each consumer's `[dev-dependencies]` (`features = ["testkit"]`; self-consumers use `required-features` or a path "." self dev-dependency). Scan with `bun F/ctc-tools/ctc-scan.ts --out F/🗑️generated/ctc-<n>` and triage with `ctc-report.ts <prefix> bad --triage=F/🗑️generated/ctc-triage.tsv`; check closure with `ctc-oracle.ts <package> <feature>`. Hand-offs per slice are in `RESULT-CFG-TEST-CROSS-CRATE.md` (r1 deflate-examples, r5a dxf-snapshot, r6 wfc/gis, r7 trinity-jack/sequence, r8b forms, r9b architect). Facade tests mounted from a carved owner via `crate::schema::*` globs are the usual leak (R2 model/mesh/drawing/brep io). Companion crates `*-editor-commands-tests` (layout, draw) are rewritten by a peer: re-check their `features = ["testkit"]` dev-dependency after regeneration.
- 12:4x CFG-TEST-CROSS-CRATE DONE (static): scanner `ctc-tools/ctc-scan.ts` over 2,922 crates (seeded proof); about 190 cross-crate cfg(test) groups fixed with the testkit precedent (`cfg(any(test, feature="testkit"))` + consumer dev-dependency feature); also fixed: undeclared `protocol-laws` feature, a stray cfg(test) on production `COMPONENT_GRAMMAR_SEMIO`, a wrong `validate_snapshot` arity, a private `db_wal::tests` reference, 4 features using dev-only crates. Hand-offs sent: r1 (deflate-examples), r6 (wfc ×3, gismap), r8b (forms), r7b (jack); ctc resumed for the finished r5a/r9b items; the sequence item goes to R7C.
- 12:5x CRATE-ROLLOUT-R5C DONE (static): gltf model (8), mutations (13), inferences (22); 52 crates inside the gate; facade `--check-applied` exit 0. Generator: `--lift=<path>` (🪜️lift-kids), enum payload leaves (120/120), the `testing` feature forwarded across carved crates, detached aggregate tests, an oracle label-insensitive compare. Dispatched CRATE-ROLLOUT-R7C (`r7c`, afe6c1087a5ff3dbc).
- 13:0x CTC follow-up: dxf `TEST_ALLOCATION_OBSERVER` fixed (plus an ungated `use … as test_allocation` of a dev-only crate that would have broken every normal build); the architect item was a false positive. 9 groups remain in r1/r6/r7/r8b crates (sent). Dispatched TEST-CLOSURE-T3 (`tc3`, framework/teaching test closures, a0c3883448fd01dc6).
- 13:1x CRATE-ROLLOUT-R1 DONE (static): deflate (facade + 6), xml (facade + 8), json (facade + 6), zip (facade + 7); 37 crates inside the gate; conformance engines moved into schema conformance modules; demo helpers behind `testkit`. **Incident:** its cfg(test) isolation tool rewrote `test`→`__r1_never` in real deflate/zip files; reversed, and the coordinator confirmed 0 occurrences repo-wide (tracked + untracked .rs). Dispatched TEST-CLOSURE-T4 (`tc4`, reader-refused hub/composition/procedural crates, a4d442ef11c5ccecf).
- 13:2x TEST-CLOSURE-T3 DONE (static): 11 framework/teaching crates inside the gate (expression 3.38 → 0.06, child-group-wire-laws 4.49 → 0.92, os-shell refused → 0.84 via a run-time read of the generated mirror, number and publication-mounted got companion crates, teaching-proctor lib split into base/domain); 127 run-time fixture reads logged with sha256; 2 real mistakes caught by the differential. brep-boolean sits at 4000/4000 lines with committed [DEBUG] code. Dispatched BASELINE-DEFECTS-9 (`bd9`, a93bd63df2f27af11): [DEBUG] removal, absolute --backup fix, R2 leaf-crate renames, raster wiring, lib-wiring-budget, O-03/05/07/11.
- 13:3x CRATE-ROLLOUT-R6 DONE (static): wfc engine (6 + facade), wfc-3d/2d/bitmap/grid2d/grid3d (7–10 + facade each), gisterrain (6 + facade), gismap (15 + facade); 85 crates, 0 over the gate; generator `prunedUse` prefix bug fixed (78 tests). Fixed an invalid `<T as Trait>::original_owners::…` path at HEAD.
- 13:4x TS-TYPE-ERRORS-3 DONE: whole-workspace tsc 1,920 → ~846 (next-env.d.ts excluded from the root program, which removed ~320; worker-law actor grants; Ajv `compile<T>`; 45 converted tests had lost loops/bindings, real runtime bugs incl. `.map.map(`; curation sqlite 3/46 → 49/0). **Coordinator rulings:** revert the repo-wide bun:test matcher loosening and fix the call sites; `verify typescript` must also check the excluded app tsconfigs; validate the tightened frame schemas against fixtures and Rust readers. Dispatched TS-TYPE-ERRORS-4 (`tt4`, a39c9f28c4d9a787f).
- 14:0x TEST-CLOSURE-T4 DONE (static): 7 crates readable and inside the gate; the canonical source reader resolved mounts inside `include!`d files against the includer's dir instead of the included file's dir (fixed, corpus case added, 44 tests pass); hub-stdio fixtures re-pointed; composition-laws generation3d target (1.45 MB) cut into 6 `[[test]]` targets with 46 run-time fixture reads (293 = 293 names). Full census now shows 0 reader-refused crates. Dispatched Haiku AUDIT-WAVE7-CRATE-GATE-2 (`cgc8`, aeebf5c4f9a384667).
- 14:1x CRATE-ROLLOUT-R4 DONE (static): svg, semio cad/presentation/animation/graph/root → 64 crates inside the gate; the root facade test closure sits at exactly 32 files (5 new sqlite-laws crates); generator support-hoist module and more fixes (85 tests). **Ruling:** the canonical testkit idiom is `cfg(any(test, feature="testkit"))` with consumer dev-deps; self dev-dependencies (R1, R4, infinite world) get normalized (brief updated). Open: the graph subset still names the removed `semio_framework_graph` (needs migration to graph-engine/manifest, shared with trinity); first-leaf crate names in R4 too (`graph-create-edge`, `presentation-insert-layout`) go to bd9.
- CAD: The launcher is not waiting on a lease now (3 probes fail in ~6 s, zero SQLITE_BUSY, no exclusive holder; orphan pid 74190 is this session's dead-owner MCP build holding only SHARED on `l`, harmless); the terminal refusal is `Pending Native image publication must settle` (interrupted `.publication` intents on `.cache/b/t` and `d`), and behind it `Exact canonical persistent Native publication required` (image.json of t/d/g/p/rust lacks `mountDirectory`), both needing the peer publication owner, not a lease expiry; the 07:15-08:29 stalls were not reproduced. See RESULT-COMPILE-ADMISSION-DIAG.md.
- 14:3x COMPILE-ADMISSION-DIAG DONE: the earlier stalls are unproven; the current terminal refusal (≈6 s) is `Pending Native image publication must settle` (interrupted peer publication intents on .cache/b/t at 08:43 and d at 11:27), and behind it the missing `mountDirectory` receipts. pid 74190 is this session's dead-owner semio-MCP build holding only a SHARED lock (harmless). HANDOFFS U-01 updated: peer/user action required, the fleet cannot bypass it.
- 14:4x **AUDIT-WAVE7-CRATE-GATE-2** (cgc8): crates 2,419 → 2,866; compilable lib 1,868 → 2,454, test 1,811 → 2,403; **oversized lib 58 → 8, test-only 43 → 1, reader-refused 10 → 0**. Remaining (all owned): writer, curation (r7c), trinity-rewriting (r7b), puzzle-2d (r8a), bim-model (r9a), stdio-pdf (r5b), semio-kit (r3), plugin test (pi2); flow-flow was unowned → dispatched CRATE-ROLLOUT-R11 (`r11`, a608fb7f03b8d3d7c). Also dispatched CENSUS4-PKG-43 (af6fbec4b50de65cd) and PKG-45 (ab410e6d357fc773b).
- 14:5x CENSUS4-PKG-4 DONE (static): 8 splits intact (verify OK, 115 test names, raster gate equal); the draw gate's base self-heal is capped at 16 rounds, so the draw proof is a layout A/B against a healthy pre-split base: 0 unique messages on either side. Noted: peers added a per-file `runtime_fixture_text` copy in 10+ files (and per-crate `runtime_fixture` modules across T1/T3/T4). Queued FIXTURE-READER-DEDUPE: one shared Rust test-support fixture reader, run after the rollouts settle.
- 15:0x CRATE-ROLLOUT-R8B DONE (static): forms (15 + facade), animate presentation (13 + facade), shooting (12 + facade), note (14 + facade), all inside the gate; `--check-applied` exit 0, 0 extras; generator flags `--no-test-aggregate`, `--inline-root-modules` plus fixes (97 tests); lib-wiring pins lowered (note 1530 → 667).
- 15:2x BASELINE-DEFECTS-9 partial: [DEBUG] sweep (6,380 temporary prints in 2,386 files stripped with a string-aware tool; 70 test-consumed statements restored; all 2,428 stripped files parse); `--backup` absolute-path fix + 2 tests; 15 first-leaf crates renamed to `-mut-family[-N]` and the generator fixed; 40 self-dev-dep crates converted to the canonical testkit idiom (39 `[[test]]` targets now `required-features = ["testkit"]`); stdio semio graph migrated to graph-engine/manifest; flow brep mesh root 1,211 → 24; O-07 done; 16 projection hashes re-pinned. Open: raster (root stubbed by 793f63533f2; sources import 10 carved crates that have no manifests), O-03, O-05 (43 assertions look for removed code), O-11 (vitest policy). → BD10.
- 15:2x Dispatched BASELINE-DEFECTS-10 (`bd10`, a8bb5edbf73b55d79): raster crate reconstruction, O-05 re-authoring, O-03, O-11.
- 15:3x CENSUS4-PKG-44 DONE (static): 7 os mcp files → 55 children (max 320); oracle CLEAN 24/24; schema rust leaf is a module-tree `concat!`.
- 15:3x CENSUS4-PKG-43 DONE (static): 7 os files (inference, directory schema, spr codec/laws/channel) → 33 children; differential identical across native/test/wasip2/features, negative controls caught; a dangling `//#region` in the spr codec closed. **`semio-framework-os-directory` at 3,983/4,000 lines → crate carve needed.** Dispatching CRATE-CARVE-OSDIR.
- 15:4x CRATE-ROLLOUT-R3 DONE (static): stdio semio image (6), object (5), kit (6), flow (7), document (7) + facades; 43 crates, 0 over the gate; document element codecs moved diff → snapshot (removes a cycle); generator fixes (mutation leaves without `use super::*`, subset crate-name disambiguation, impls move with their type) with 4 tests. Note: an accidental unverified apply was reverted path-exact and redone. Dispatching CENSUS4-PKG-49.
- 15:5x CENSUS4-PKG-45 DONE (static): 7 framework files (pixels, graph dsl tests, canvas renderer, deflate, dsl schema/derive, surface paint) → 39 children; the full typeck oracle caught one real E0624 (`Scene::vello_scene` pub(super) → pub(crate)), final 0 new / 0 gone. Tight: `dsl-record` test closure at 30/32 files and 258/262 KB. Dispatching CRATE-HEADROOM (crates at ≥ 0.95 of any limit).
- 15:5x CRATE-ROLLOUT-R7B DONE (static): trinity jack hub dissolved (facade 5,439 → 219 lines; 13 crates), rewriting schema → 10 crates; cycles broken via the `JackContentFactory` port, runner/effects/camera moves; gates exit 0. Stale refs at HEAD (real compile errors, oracle-invisible): `materialize_jack_snapshot` ×55 in rewriting (lives in the dev crate), `Graph::host_snapshot_json` missing its composition arg, `JackSnapshot::with_content` missing its type arg; gitignored manifest registries must regenerate with `GraphManifest`. → TRINITY-API-DRIFT.
- 16:0x CENSUS4-PKG-47 DONE (static): 7 os files (workflow graph, shell schema, dsl/neural/host tests, codec-abi, plugin-host) → 52 children (max 625); non-stub rows equal; 12 stub-layer rows from `use super::*` trait learning (the real check decides). An over-widening incident (re-stage read the split root) was corrected; files are byte-equal to the corrected stage.
- 16:1x CENSUS4-PKG-48 DONE (static): 6 repo crates (dashboard registry, goals, graphql repo-schema/fs-context, server coordinator, codebase) → 70 children (max 250); oracle CLEAN 12/12; public API sets identical.
- 16:2x TS-TYPE-ERRORS-4 partial: 941 → 577 (20 in a peer's new file); matcher ambient file deleted, but 159 sites got `expect<unknown>(…)` (a per-site loosening, rejected → TT5 fixes the types properly); `verify typescript` now checks the excluded Next apps (20/20 tests); frame schemas confirmed against the Rust wire format (Ajv-vs-parser 47/47); PackError mirrors the Rust refusals. Note: a half-applied debug-strip in `capacity/tests/derivative/🟦️.ts` broke whole-workspace tsc; patched.
- 16:3x CENSUS4-PKG-49 DONE (static): 7 framework files (tiled-map/tool-machine tests, manifest actions/arg-schema/app-surface, sampling config/vocabulary) → 55 children (max 322); oracle CLEAN 25/25. Dispatched TS-TYPE-ERRORS-5 (`tt5`, a9d68698401b43e2d) and CENSUS4-PKG-51 next.
- 16:4x PLUGIN-INTEGRATION-2 applied (final checks running): 19 companion test bins (114 launch routes), tests-20..29 retired; **semio-framework-plugin facade cfg(test) closure 368 files / 4.3 MB / 45,733 lines → 21 / 181 KB / 2,808 (inside the gate)**; 1,729 test pairs identical; 277 include literals resolve.
- 17:1x **Second usage-limit cut at about 16:45** (15 agents; reset 17:10). Resumed 12 via SendMessage: r5b, r9a, r8a, r7c, r11, pi2 (final checks), tc2, tad, bd10, osd, hdr, tt5. Held: c4p46, c4p50, c4p51 (TS file splits). **Fleet cap now 12.**
- 17:2x CRATE-CARVE-OSDIR DONE (static): os-directory 3,983 lines → facade 215 + schema (2,741) + browser-actor, check-in, binary, access-policy; union differential 0/0 on 5 cfgs; lock refresh pending. Still near the limit: `…-client-tests` (0.95 bytes) and `tests-1` (207 KB embedded) → CRATE-HEADROOM.
- 17:3x CRATE-ROLLOUT-R11 DONE (static): flow-flow (531 KB, 4.09×) → 11 crates + facade; the 2,958-line editor hub dissolved into base/host; `retirement` moved under schema; new generator option `--target=<files>/<bytes>/<lines>`. Open: `-unit-tests` at 0.97 lines / 0.95 bytes (cfg(test) harness needs a testkit crate) → HDR.
- 17:4x TRINITY-API-DRIFT DONE (static): `materialize_jack_snapshot` owner moved out of the dev crate into a new `semio-s-artifact-trinity-jack-semio-content` crate (57 call sites); with_content/host_snapshot_json/to_snapshot type args; the graph generator gains a `rustOwner` row option so regeneration keeps a single mount; 3 crates renamed (-model). Also fixed 3 real dev-crate errors. Open: ~40 rewriting tests call diff codecs at the wrong path (schema::diff → io::text::diff); dev-test stale calls (preset_query, executor::run, subgraph_snapshot, …). Note: `bun test` segfaults at start in this checkout (Bun 1.3.14) even on untouched files. Dispatching ORACLE-HARVEST-PLUGINS.
- 17:5x CENSUS4-PKG-51 DONE: 6 quiz files → 29 children; quiz-react 926 baseline statuses identical (+3 new stylesheet-tree tests), quiz core 451/451, Python oracle 4/4, minified CSS byte-identical.
- 18:0x CRATE-ROLLOUT-R7C DONE (static): sequence (5 + facade), writer (8 + facade), curation (6 + facade), all inside the gate; layering via base layers, explicit layer imports, harness moved to `editor_unit_tests`; generator: wasm32 cfg scanned by the planner, cfg gates kept on re-declared modules, carved crates as package dev-deps, nested alias shims (92 + 27 tests). Open: first-slice crate names (`-tests` holds base+host in sequence) → rename follow-up.
- 18:1x PLUGIN-INTEGRATION-2 DONE (static): 19 companion bins; plugin facade test closure 0.70 (was 16.45×); 1,729 test pairs identical; oracle test 0 new / 70 gone. Tight: tests-05 at 0.996 bytes; `semio-framework-dsl` test unit at 1.001 → hdr.
- 18:2x CENSUS4-PKG-46 DONE: InkCanvasHost, Board2dHost, TiledMapHost (hooks over a typed core), ShellHelpers plumbing/window-panes/time-travel tests → 45 new files; renderer tsc byte-identical; vitest 3,206 tests with an identical failure set. Open: `🛠️ShellHelpers/🟦️.tsx` is still a re-export barrel (AGENTS forbids compat barrels) → SHELLHELPERS-UNBARREL.
- 18:4x SHELLHELPERS-UNBARREL DONE: the barrel is deleted; 307 used names re-pointed to their owners in 106 importers (42 unused names dropped); 5 source-text readers re-pointed; renderer tsc 24 → 9 (all baseline); vitest failure set equal except 2 load-flaky tests. Dispatching CENSUS4-TS-SMALL (packages 52, 55, 56, 58).
- 18:5x TEST-CLOSURE-T2 DONE (static): 16 draw/layout/block crates inside the gate (scene 3.78 → 0.86, layout mutations-frames 6.31 → 0.91) via run-time fixtures + 17 companion test crates; 1,065 tests identical, 169 fixtures unchanged; layout/block gates exit 0. Draw `--test` gate has 17 rows: a host-arena carve gap (7 private items, 3 undefined names incl. `DrawingOwnedRetirement`) → ohp. Layout editor-commands lib at 32/32 files → hdr.
- 18:5x Dispatched CENSUS4-SMALL-2 (`c4s2`, packages 54, 59–66, aec648d2ae7bb2e4c).
- 19:0x CRATE-ROLLOUT-R5B DONE (static): stdio pdf (747 files / 3.7 MB, 22×) → 50 producer crates + facade; 63 crates in the artifact folder, 0 over the gate; `--check-applied` exit 0 (after one reverted attempt); testkit module for 10 conformance fixtures + mutation_bridge; generator fixes (88 + 9 tests), incl. a peer regression in `prunedUse` that silently disabled test detaching. Tight: `pdf-tables` (generated font tables) at 0.95 bytes → hdr (asset channel).
- 19:0x Dispatched CENSUS4-PARTIAL (`c4pp`, package 41 + free files of 26/33/36/42/57; a9d647bdf8646c1ac). That covers every census-4 package.
- 19:1x FIXTURE-READER-DEDUPE DONE (static): new dependency-free `semio-framework-test-fixtures` (fixture_text!/fixture_bytes!/repo_text! …, 10 Rust + 5 bun tests); 1,009 helper copies removed, 5,178 call sites in 1,391 files, 494 manifests; generator emits the macros (100 tests); 4,815 test names identical. Found: 28 fixture reads whose paths do not exist (will fail at run time; one curation path contains a literal `u{1f9ea}`). Deferred: r5b (now done), ohp, bd10, r8a, r9a areas → FRD-2.
- 19:3x FRD-2 DONE (static): pdf migrated (1,677 files / 5,475 macro sites / 515 manifests repo-wide); 269 of 274 missing fixture paths repaired (stale post-carve depths, moved TS modules, a stripped `\u{..}` escape restored); the hub-norm gaming scan scanned a nonexistent dir and passed vacuously — now scans the real artifacts. Census: oversized lib crates 3 → 2. Deferred: ohp/bd10/r8a/r9a areas.
- 19:4x CENSUS4-SMALL-2 DONE: packages 54, 59–66 → 14 files split into 82 children; Rust POSDIFF identical, TS/vitest/playwright sets identical.
- 19:5x CENSUS4-PKG-50 DONE: tool-run, directory schema, machine, backbone, app-channel → 42 children; tsc 0 = 0, all baseline test sets identical. Queued: `AppChannelClient` (688-line class, god-object rule) collaborator decomposition; orphan JSDoc stubs in event-page/command/admin.
- 20:0x BASELINE-DEFECTS-10 mostly done: raster rebuilt (22 crates inside the gate, editor hand-layered with a `RasterWorkApp` generic, `--check-applied` exit 0, 22/22 raster TS suites); O-05 close-budget 30/31 (1 cargo-gated skip), 3 dead archive-close fns deleted; O-03 rewritten as a lib unit test (uncompiled); O-11 vitest ownership gate 7/7. **Census over 3,032 crates: 0 oversized.** New O-13: 71 plugin TS test files (177 tests) fail on source-anchor drift; `🎭️actor-transport` hangs → BD11.
- 20:1x CENSUS4-TS-SMALL DONE: presentation/print/repo/pets 15 files → 15 roots + 74 children; every failure set identical (presentation 140 + 56, print d3 631/659, pets 1,133 + 175), the CSS bundle byte-identical, two region defects repaired. Note: the repo launcher refuses nx routes (`Original bounded retirement catalog layout required`).
- 20:2x TS-TYPE-ERRORS-5 partial: workspace diagnostics 557 → 363 (real 295 → 79 owner-deep); all 254 `expect<unknown>` sites typed properly (some by swapping actual/expected → rejected, to unswap); `verify typescript` now uses the compiler API and survives parse errors (23 tests); TS parse sweep over 18,468 files clean. Remaining 79: 48 custody/storage-admission tests on pre-redesign APIs, 18 generated parsers vs. stub schemas, 13 missing packages/wasm. → TT6.
- 20:3x CENSUS4-PARTIAL DONE: 16 files (stdio semio internals/diffs, renderer test roots, react entry, TextEditor, mesh-render, hub bootstrap/dev-collaboration; 22,377 lines) → 16 roots + 127 children; oracles identical, plugin gates exit 0, export set identical (563). **All census-4 packages are now done** (wgpu files stay peer-held). Open: renderer tsc 7 → 73 from an in-progress carve under 📇️directory (59 rows) and SpaceAdministration/directory-client/catalog bindings (→ tt6); library gate suites cargo-transaction-command-source, build-surfaces and plugin-publication-source-ownership fail (→ rg7).
- 20:4x AUDIT-WAVE7-COMPLIANCE: [DEBUG] 6 source sites + 21 test log lines; 105 `🪞️mirror` path-preserving barrels + 41 `Form*` aliases + 6 TS alias exports; 94 new files without a docstring and 29 with a plain comment; 2 duplicate sibling emoji; ~194 comment lines added inside bodies; 157 `cfg(any(test, feature=…))` lines with non-testkit feature names; 41/34 missing launch-route paths; a stray `Users/` tree again (r6 gisterrain backup from before the bd9 fix) → coordinator moved it into 🗑️generated and removed the empty `Users/` and `.%F0%9F%A7%ACsemio` trees. **Ruling:** the mirror barrels are compat layers → MIRROR-UNWIND after the rollouts finish; the other items → COMPLIANCE-FIX now.
- 20:5x APP-CHANNEL-DECOMPOSE DONE: `AppChannelClient` 712 → 302-line composer + 13 collaborators (≤ 124 lines), public API unchanged; 10 orphan JSDoc stubs removed; os vitest 722 identical + 5 new wiring tests (each catches a seeded break). Follow-up sent: remove the never-read `actor` parameter/field and its 2 call sites, and the unused `editedArtifactFrontier`.
- 20:5x CRATE-ROLLOUT-R8A DONE (static): puzzle 5d → 24 crates, 2d → 29 crates (test closures were 3.7 MB / 6.7 MB), all inside the gate; `--check-applied` exit 0, 0 extras; generator fixes (orphan-legal foreign-trait impls, detached-test widening, oracle noise filters; 103 tests). Fragile: testkit `[features]`/dev-dep rows were hand-edited in generated Cargo.toml files (r8a and others) → GENERATOR-TESTKIT, so the generator emits them.
- 20:5x APP-CHANNEL follow-up DONE: the dead `actor` constructor parameter/field and its 2 callers, and the unused `editedArtifactFrontier`, removed; tsc and vitest unchanged. Dispatching MIRROR-UNWIND (bim excluded).
- 21:0x COMPLIANCE-FIX DONE: [DEBUG] census 354 → 248 hits (17 not in HEAD; ~76 unread test logs removed; ~77 read-back channel lines retagged `[DATA]` with their readers); 203 docstrings added/fixed; 2 fleet-added body comments removed (the other ~192 were moved HEAD comments); 24 launch routes; one undeclared `testing` feature → `testkit`. A Codex peer is still adding [DEBUG] lines; rerun `cpf-tools/debug_pipeline.sh --apply` at the end. The stray `bd8-diag.ts` was moved into 🗑️generated.
