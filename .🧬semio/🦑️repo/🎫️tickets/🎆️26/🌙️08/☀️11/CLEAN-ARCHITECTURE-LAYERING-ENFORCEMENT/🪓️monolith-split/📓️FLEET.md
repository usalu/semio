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
