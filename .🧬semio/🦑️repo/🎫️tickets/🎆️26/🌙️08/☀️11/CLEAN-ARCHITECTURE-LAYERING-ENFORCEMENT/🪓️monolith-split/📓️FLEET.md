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
