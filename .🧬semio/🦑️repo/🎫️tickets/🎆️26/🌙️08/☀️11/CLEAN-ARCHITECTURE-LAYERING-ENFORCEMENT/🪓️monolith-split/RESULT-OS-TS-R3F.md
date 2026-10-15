# Result: os root barrel (D) and r3f world engine (G) split

Executed 2026-10-15 by the Sonnet executor. `O` = `🧰️framework/🛍️products/💻️os`, `R` = `O/🔨️modules/♾️infinite/🌍️world/🎨️r3f`, `B` = `O/🔨️modules/🔌️plugin/🌐️browser-bundle`.
Seam map used: `SEAM-MAP-TSX-TIER2.md` sections 2.D and 2.G. No Rust file was touched.

## 1. Outcome at a glance

| File | Before | After | Children |
|---|---|---|---|
| `O/🟦️.ts` | 5,775 lines | 504 lines (explicit named re-exports, the kept `export ... from` lines, the 12 in-source `import.meta.vitest` blocks) | 5 new |
| `R/🟦️.tsx` | 4,601 lines | 495 lines (re-exports, `WorldR3fTestDependencies`, the in-source test block) | 12 new |

Public API unchanged. Checked with the TypeScript checker (`getExportsOfModule`, name plus type/value kind), original text vs new tree: `O/🟦️.ts` 424 exports identical, `R/🟦️.tsx` 244 exports identical, 0 unresolved (`os-exports-{orig,new}.txt`, `r3f-exports-{orig,new}.txt`). Losslessness: every original top-level unit appears exactly once across parent plus children (`coverage2.mjs`): os root 410 units = 21 `export` statements kept in the parent + 388 exactly once + `AppChannelClient` (exactly once after the follow-up in section 9; the first cut had lost 8 marker lines); r3f 370 units = 1 kept `export { Canvas, ... }` (re-pointed at the adapters child) + 369 exactly once. No `export let` crosses a module boundary; the only `let` (`worldMeshBorderColorCache`) lives with both of its users in `🧷️r3f-foundation`.

## 2. Task D, step 1: the runtime cycle

The browser-bundle `action-handoff` modules imported the root barrel while the root imports `B/🎯️action-handoff/🟦️.ts`. Each now imports the concrete lower module (no barrel):

| File (under `B`) | Before | After |
|---|---|---|
| `🎯️action-handoff/🧭️intent/🟦️.ts` | `encodePackValue, packUInt, PackValue` from root | `🎁️packs` |
| `🎯️action-handoff/🎛️command/🟦️.ts` | `encodeAppCommand, encodePackValue, viewContextWireValue, PackValue` from root | `encodeAppCommand` from `🧾️app-channel`; the rest from `🎁️packs` |
| `🎯️action-handoff/📤️publication/🟦️.ts` | 7 names from root | `decodeAppFrame, encodeAppFrame, faultDisplayMessage` from `🧾️app-channel`; `decodeInvocationResultPacks, decodePackValue, encodePackValue, packValueToExactJson` from `🎁️packs` |
| `🎯️action-handoff/🟦️.ts` (in-source test) | dynamic `import("../../../../🟦️.ts")` | `decodeAppCommand` from `🧾️app-channel`, `decodePackValue` from `🎁️packs` |
| `🏗️materialization/🟦️.ts` | `APP_CHANNEL_VERSION, admitGuestChannelVersion` from root | `🧾️app-channel` |
| `🪟️view-context/🧪️tests/🪟️host-opening-context/🟦️.ts` | codec pair from root | `🧵️backbone` |

Test-only back edges the map listed were also retargeted: `O/🧪️tests/🧪️backbone-envelope-io/🟦️.ts` (20 root specifiers: `Pick<typeof import(root), ...>` became `Pick<typeof import(a) & typeof import(b), ...>` per owning module, type references and two dynamic imports re-pointed; module of each name resolved with the checker, so re-exports from `📇️directory` go to their origin) and `O/🧪️tests/🧊️mesh-pack-decode/🟦️.ts`. Verification script: `rootimporters.mjs` finds 0 importers of the root under `B`.
Honest note on the "runtime" nature: rollup sees no static cycle before or after (the root reaches only the `action-handoff` parent, whose only edge to its children is the `import.meta.vitest` dynamic block). The cycle existed for the dependency-cruiser gate (it counts dynamic imports) and under vitest; the real production gain is that command, intent and publication no longer pull the whole barrel.

## 3. Task D, step 2: the five children (`O/🔨️modules/<dir>/🟦️.ts`)

| Child | Lines | Holds | Imports (siblings) |
|---|---|---|---|
| `🎁️packs` | 968 | pack tags and integer carriers, symbol-table container, public encode/decode API, base64/exact-JSON projections, invocation-result decoding, scene and mesh pack decoders, `replicationPackCodec` | leaf |
| `🪢️wire-contracts` | 425 | backbone size ceilings, app command/frame value types and document-archive pack entries with their combinators, command-rejection codes and ack decoders | `🎁️packs` |
| `🧵️backbone` | 1,613 | artifact opening relay, backbone URIs/envelope transport, binary backbone messages, desktop window controls, persistence bindings, backbone worker wire protocol, workflow planner | `🎁️packs`, `🪢️wire-contracts`, `📞️directory-client` |
| `🧾️app-channel` | 1,873 | app command/frame codecs, channel version admission, request sequence, document-archive load host, `AppChannelClient` | `🎁️packs`, `🪢️wire-contracts` |
| `📞️directory-client` | 839 | stdio format kinds, hub reconnect policy, document link shortage, echo suppression, transient apply refusal, directory transport types/parsers, socket grants, `DirectoryClient` | leaf |

Acyclic (SCC check over the six files: 0 SCCs, 0 children importing the parent, `graph2.mjs`). The 12 `import.meta.vitest` blocks stay in the parent because they use `import.meta.dir`/`import.meta.url` for fixtures and `./🧪️tests/...` relative paths; the parent imports their names from the children.
Deviations from the map: (a) `codec-core` is `🪢️wire-contracts` because the `no-core-path` rule of the dependency gate bans path segments ending in `core` (the first cut with `codec-core` added 3 `no-core-path` violations; renamed before the final runs); (b) `wire-contracts` also holds the app command/frame value types and archive pack entries (not only combinators) to keep `app-channel` under 2,000 lines and to avoid a types-to-combinators cycle; (c) the whole CommandRejection region (not only `commandAckOutcomeOfValueV1`) moved there because `ArtifactEvent` and the ack decoder share its types; (d) `🧵️backbone` imports a type from `📞️directory-client` (`DirectoryEventPageAckV1`), which is a leaf, so no cycle.

## 4. Task G: twelve children (`R/<dir>/🟦️.ts[x]`)

| Child | Lines | Holds |
|---|---|---|
| `🔌️adapters` | 57 | fiber/drei ports and the destructured three constructors with instance types (extra child, not in the map: every other child needs it and the map's "parent keeps adapters" would make children import the parent) |
| `🔭️scene-types` | 44 | projection vocabulary types plus `ProjectionGizmoHit` |
| `🧷️r3f-foundation` | 294 | vec types, camera state, navigation gestures, entity flags/render modes, event binding controller, CAD to three precision, mesh borders, `worldRaycastNone` (the map's `r3f-core`, renamed for the `no-core-path` rule) |
| `🧱️collision` | 209 | collision bodies and overlap tests |
| `📷️camera-view` | 548 | orbit view ids/directions/templates/layouts, camera accessors, snap gate context, far-plane constants, camera view rig, matched ortho/perspective math |
| `🧮️chunking-lod` | 567 | chunk keys/visibility, pools, view radius layer, LOD context/bridge/grid helper |
| `🗂️layers` | 30 | world layer stack |
| `🖼️reference` | 399 | reference planes |
| `🧊️volume` | 264 | volumes |
| `📐️projection` | 1,158 | projection defaults, framing, poses, matrices, rig/applier/curvilinear pass, template ids, labels, switch panes |
| `🧭️gizmo-viewport` | 538 | gizmo hit/palette/viewport, snap driver, view controls |
| `🎬️canvas` | 645 | orbit mouse bindings, gated orbit controls, canvas |

DAG (0 SCCs, 0 children importing the parent): adapters, scene-types, layers are leaves; foundation to adapters+scene-types; collision, reference, volume to adapters+foundation; camera-view to adapters+scene-types+foundation; chunking-lod to camera-view; projection to camera-view+reference+scene-types; gizmo-viewport to projection; canvas to camera-view+layers. Deviations from the map: unit-level cuts moved `WORLD_ORBIT_CAMERA_MIN_FAR`/`..._FAR_DISTANCE_FACTOR`/`adaptiveOrbitCameraFar` into `camera-view` (they would otherwise force camera-view to import chunking-lod, which imports camera-view), the projection switch panes (map: gizmo region) sit in `projection`, the camera rig and orbit templates sit in `camera-view`, mouse bindings joined `canvas`. The in-source test block and `WorldR3fTestDependencies` stay in the parent (the vitest config includes only the parent file).
Module-level mutable state: one owner each (`worldMeshBorderColorCache` and the scratch objects with their users).

## 5. Registration and project metadata

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: `memberNames` appended (textual edit, nothing reformatted) to `members-of-modules`: `🪢️wire-contracts 🎁️packs 🧵️backbone 🧾️app-channel 📞️directory-client`; and to `members-of-members-of-members-of-members-of-modules` (the kind of children of `🎨️r3f`): `🔌️adapters 🔭️scene-types 🧷️r3f-foundation 🧱️collision 📷️camera-view 🧮️chunking-lod 🗂️layers 🖼️reference 🧊️volume 📐️projection 🧭️gizmo-viewport 🎬️canvas`. No sibling emoji repeats; no banned stem (`core common util utils helper helpers misc shared base lib impl`).
- Nx inputs (the old `default` input lists named only the monolith file, so edits to children would not invalidate the cache): `O/📦️packages/🟦️typescript/📋️project.json` (5 child files), `R/📦️packages/🟦️typescript/📋️project.json` (`R/**/*`, which also fixes the pre-existing gap that the component source was not an input), and `O/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` (5 child files next to the root entry).
- Source-text readers of `O/🟦️.ts` re-pointed at `readModuleTreeSource` (the repo's split-source reader, already used for ShellHost): `O/📦️packages/🟦️typescript/📜️script.ts` (`protocol`), `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts` (`osTs`), `O/📦️packages/🦀️rust/📜️script.ts` (`typescript` plus `sliceModuleTreeSource` for the `eventPage` method; new import), `O/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` (`wire`; new import). Evidence: the `candidateProtocol` clauses (6) and `wire` clauses (2) give identical results on the original text and the new tree (`clauses.mjs`, 0 differences); the oracle functions `persistenceDataClassOracle` (checks=11) and `directoryEventPageClientOracle` (checks=11) pass when extracted and run after the change (`h-space.ts`, `h-rust.ts`). `proveDocumentOpeningAttempt` fails with "production correlation is incomplete" on the ORIGINAL text as well (other files' clauses), and `scopedPresenceOracle` stops earlier on "presence schema module publishes no export ScopedPresenceCloseV1"; both are pre-existing and unrelated.

## 6. Verification, baseline vs after (all foreground, logs and tools in `🗑️generated/os-ts-r3f/`)

Dependency-cruiser (`dc.sh`, the repo config, roots = `O/🟦️.ts`, `B`, `R`; `dcsum.mjs`):

| | Baseline (`dc-baseline.json`) | After (`dc-after2.json`) |
|---|---|---|
| modules / deps cruised | 768 / 2,970 | 816 / 3,215 (18 new files plus peer changes) |
| `no-circular` violations, whole cruised closure | 147 | 139 |
| cycles through a touched path (root, `B`, `R`) | 22 | 12 |
| cycles through the os root | 12 | 0 |
| `no-core-path` | 11 | 12 (the +1 is `util`/`zlib` in files I did not touch; none from my files) |

Remaining 12 touched-path cycles are all test-registration or pre-existing: parent to its own `🧪️tests` module (r3f chunkkey, `patch-handoff`, `🧵️child/🧬️schema`), the `action-handoff` parent dynamic-importing its `command`/`intent`/`requests` children (which import the parent's parsers), `browser-bundle/📜️script.ts` pairs, and 2 library/store chains. The non-circular rule deltas on my files are the same cross-package-relative edges moved from root to children (added 25 / removed 16, all `no-cross-package-relative-*`, dominated by the browser-bundle files now naming two children instead of one root).

TypeScript (`tsconfig.json` in the generated folder extends `O/tsconfig.json`, files = both parents, the five `action-handoff` modules and the r3f test; program 2,059 files after): baseline 0 errors, after 0 errors (`tsc-baseline.log`, `tsc-after3.log`, after all edits including the test retarget). With `--noUnusedLocals --noUnusedParameters` no unused import exists in any new file; the 7 hits in new files are declarations that were already unused in the monolith (`_collisionRay`, `_worldVolumeMatrix`, `directoryPublicDocument`, `directoryMember`, `directoryDocument`, `postJson`, `actor`), and 2 more are in the untouched chunkkey test (`tsc-unused-after.log`).

Vitest (config of each package; baseline = original file contents injected through a Vite `load` hook that serves `orig-abs/*` with relative specifiers made absolute and `import.meta.dir/url` made literal, confirmed active by a `[DEBUG]` line; `vt.sh`, `vt-*.config.ts`):

| Suite | Baseline | After |
|---|---|---|
| `@semio-tech/framework-os` (9 files, includes the root in-source suite of 258 tests and the action-handoff in-source test) | 2 files failed, 7 passed; 110 tests failed, 399 passed (509) | identical counts, and again identical after the test retarget (437 s run); the 110 failing test names are the same set (`vt-os-*.fails.txt`): 107 in `actor-transport`, 3 in the root suite (fixture-path failures) |
| `@semio-tech/infinite-world-r3f` (run with `bun --bun vitest`; node cannot load the config) | 103 passed / 103 | 103 passed / 103 |
| `@semio-tech/framework-renderer-react` full config (184 files, the suite that contains World3dHost, ShellHost, window-scope, projection-render-parity, engine-contract) | 32 files failed, 152 passed; 41 tests failed, 2,980 passed (3,021) | 29 files failed, 155 passed; 38 tests failed, 2,983 passed (3,021) |

React detail: every failure after is also a failure in the baseline run (47 sorted failure lines after, 50 before). The 3 only in the baseline are timing-sensitive tests (`wgpu-intake-budget`, `embedded-mount`, `Paint2dHost selection-focus`) that ran while my Vite bundle jobs shared the CPU; I did not re-run them. Caveat for the baseline mode: the already-edited browser-bundle files import the new children while the root is the injected original, so the baseline is not a pure pre-change tree; the identical outcomes still show the new tree loses nothing. Not run: `bun repo run ...:test` (earlier siblings found it broken in this session; direct vitest with the package configs instead).

Taxonomy (`bun ./📜️script.ts verify taxonomy report --scope <dir>`): the five os child dirs `clean=true errors=0`; scope `R`: `errors=1`, `fixed-source-disposition-unresolved` on `R/📦️packages/🟦️typescript/📜️script.ts` (a file I did not create or edit); no finding on any of the 12 new dirs.

Browser bundle: the play site `build` target compiles every wasm component through Cargo, far beyond 15 minutes and unsafe next to peers, so I did not run it. Instead `vite-bundle.ts` runs the real Vite/rollup bundler with the repo's workspace resolve config (`createWorkspaceViteResolveConfig`, `import.meta.vitest` defined away as in the site build) on the two package entries: after: 4 chunks, 273 modules, 6,181,332 bytes, 25 warnings of which 23 `CIRCULAR_DEPENDENCY`; original root/r3f injected (and, in a second variant, the three browser-bundle files with their old root imports): 259 modules, 6,181,571 bytes, the same 23 cycle warnings, none involving an os or r3f module (they are retained-UI/replication/d3 chains). Handoff-only entries (command, intent, publication): 27 modules / 105,817 bytes after, 26 / 105,127 bytes before, 8 circular warnings both (none os). The bundler resolves every new specifier; there is no runtime evidence from a browser page.

Path length: longest new file path is 130 UTF-8 bytes below the repo root (limit 240).

## 7. Follow-ups and limits

1. Remaining non-test importers of the os root barrel (not in any cycle): `📦️packages/🟦️typescript/🟦️.ts` (glue), `📇️directory/🪪️session-refresh/🪪️session-port`, `🎬️media/📡️channel`, `🛂️SpaceAdministration`, `⚡️effect-backbone`, `✅️catalog-verification`, `🧑‍💻dev/🔎️verification/🧾️publication`, plus about 20 tests. Pointing them at the children is mechanical (`rootimporters.mjs` lists them).
2. Superseded: the static input lists were updated, see section 9.2.
3. Superseded: the splitter dropped every top-level `//#region` marker, not only the 8 inside `AppChannelClient`; all were restored, see section 9.
4. The 4 oracles named in section 5 were not run through `bun repo run`; their function bodies were extracted and run with the same reader code.
5. Tools kept for traceability in `🗑️generated/os-ts-r3f/` (not repo tooling): `tools/{analyze,split,write,units,groupcheck,cfg-os,cfg-r3f,exports,coverage2,graph2,rootimporters,retarget-tests,clauses,oracle-harness,mkabs}.mjs`, `dc.sh dcsum.mjs dcdiff*.mjs`, `vt.sh vt-*.config.ts`, `vite-bundle.ts`, `tax.sh`, copies of the originals (`orig-os-root.ts`, `orig-r3f.tsx`, `orig-test-*.ts`), `scratch-os/` and `scratch-r3f/` (pre-write renders). The `🗑️generated` folder was left in place.

## 8. Files

Created (18): `O/🔨️modules/{🎁️packs,🪢️wire-contracts,🧵️backbone,🧾️app-channel,📞️directory-client}/🟦️.ts` and `R/{🔌️adapters,🔭️scene-types,🧷️r3f-foundation,🧱️collision,📷️camera-view,🧮️chunking-lod,🗂️layers,🖼️reference,🧊️volume,📐️projection,🧭️gizmo-viewport,🎬️canvas}/🟦️.ts[x]`.
Edited: `O/🟦️.ts`, `R/🟦️.tsx`; `B/🎯️action-handoff/{🟦️,🎛️command/🟦️,📤️publication/🟦️,🧭️intent/🟦️}.ts`, `B/🏗️materialization/🟦️.ts`, `B/🪟️view-context/🧪️tests/🪟️host-opening-context/🟦️.ts`; `O/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`, `O/🧪️tests/🧊️mesh-pack-decode/🟦️.ts`; the four reader scripts in section 5; the three `📋️project.json` files in section 5; `taxonomy.json`.

## 9. Follow-up requested by the coordinator (region markers and inventories)

### 9.1 Region markers

- The 8 markers inside `AppChannelClient` (`🏠️LocalInteractionQuery`, `🔖️Merge`, `🔖️Presence`, `🔖️Transaction`, each begin and end, two-space indent) are back in `O/🔨️modules/🧾️app-channel/🟦️.ts` at the positions of the original, with the original text. The class body, compared line by line with the original lines 4035-4698 (`difflib`), now has 0 differences; this also restored one blank line the first cut had collapsed and removed one it had added before `//#endregion 🔖️Transaction`. `diff` of the indented `//#` lines of the original class vs the new class: identical.
- Finding while verifying the counts: the splitter had dropped every top-level `//#region` marker too (os root 81 markers in the original vs 10 in the tree, r3f 40 vs 2; the first result file wrongly said only the class markers were affected). All were restored by `restore-regions.mjs` (original line text verbatim, `Header` regions were never touched): a region whose members now live in one file wraps its first and last member there; a region whose members were split across children is repeated, balanced, in each file that holds members (so cross-file regions such as `🔖️Backbone`, `🔖️Types`, `🔖️AppChannelCodec`, `🔖️Directory`, `🔖️HubBinding`, `📶️Lod`, `📐️WorldProjection` appear once per holding file); regions that contained only the relocated `export ... from` statements of the parent (`🔖️HubSignIn`, `🔖️HubFirstRun`) wrap them in the parent; the one unpaired `//#endregion 🔖️EnvelopeCodec` of the original (it has no begin there) is kept at its original place in `🧵️backbone`.
- Counts, original vs tree (`grep -c '^[ \t]*//[ ]*#\(end\)\?region'`): os root 81 vs 115 (81 = 2 Header + 8 class + 70 for 35 pairs + 1 unpaired; the extra 34 are the repeated begin/end pairs of the 17 cross-file regions), r3f 40 vs 58 (20 names = 19 region pairs + Header; the extra 18 are 9 repeated pairs). Distinct region names: os 41 vs 41, r3f 20 vs 20. Per file every begin has its end, nesting is proper (`regbal.mjs`: 0 unclosed, 0 mismatched in all 19 files); the counts therefore cannot be equal where a region spans children, and I did not collapse a region into a single file.
- tsc after the restore: 0 errors (`tsc-after4.log`). Exports still identical (424 / 244). Losslessness: every os root unit now appears exactly once, including `AppChannelClient` (389 + 21 kept `export` statements = 410; r3f 369 + 1 = 370).
- Vitest after the restore: `@semio-tech/framework-os` 2 files failed / 7 passed, 110 tests failed / 399 passed (509), identical to the baseline and to the previous run, and the same 110 failing test names (`vt-os-new3.log`); `@semio-tech/infinite-world-r3f` 103 / 103 passed (`vt-r3f-new2.log`).

### 9.2 Inventories that named only the old root files

Edited (each JSON re-parsed after the edit):
- `✏️s/🔌️plugins/{🏗️fem/🗿️artifacts/◻️2d,🏗️fem/🗿️artifacts/🧊️3d,🗄️stdio/🗿️artifacts/🗽️obj,🗄️stdio/🗿️artifacts/🧊️gltf,🗄️stdio/🗿️artifacts/🧿️semio}/.../🔮️oracles/🔣️.json` (6 `productionDebt.reachableFrom` lists for `three`): the r3f parent is followed by the 11 children that reach `three` (every child except `🔭️scene-types`, which only holds types; computed from the dependency-cruiser closure of `dc-after2.json`, `reach.mjs`, and checked on the baseline where only the old parent reached it). The schema allows plain strings only, so no globs; note the list is described as shrink-only, these are relocations of the same code.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/⚙️config-graph.json`: the five new os children added to `deny` next to the root (the Vite config graph must not pull any of them).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧫️fixtures/🧬️generator-ownership/🔣️.json`: a new `browserTransport.imports` row `📞️directory-client` -> `session-authority-v1` (browser scope); the existing root rows stay valid because the parent still imports and re-exports those targets. I ran the body of `testBrowserTransportOwnership` against the real files (`transport-rows.ts`): 11 of 11 rows pass.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🖥️os-source-topology/🔣️.json`: the `BackboneWorkerRequest` row now lists the parent and `🧵️backbone` as consumers (the test only requires the consumers to exist). The `EffectBackbone` row is unchanged: nothing in the tree mentions that anchor.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: the five os children added, in byte order, to `generatorContracts.wgpu-frame-worker.packageGeneration.browserProfile.sourceModulePaths`. This is the list that `testWgpuGeneratorOwnership` compares with Bun's observed module set; I ran the same `Bun.build` observation (`wgpu-observed.ts`): the five children were observed and missing, now recorded. The observed set also differs from the recorded list by 37 other observed paths (actor, replication, plugin registry, ...) and 1 stale entry; that drift predates this work, so that test's exact comparison does not pass today and I did not touch those paths. The `frameWorkerSources` named input those children need is the wgpu `📋️project.json` edit of section 5 (checked: all five present).

Unchanged on purpose: `🧰️framework/🛍️products/📓️print/🔮️oracles/🔣️.json` (ajv `reachableFrom` lists the os root; ajv is reached only through the dynamic test imports kept in the parent, none of the five children imports ajv or a validator), the `EffectBackbone` row, and the prose pointer "TypeScript twin `💻️os/🟦️.ts` region `🚫️CommandRejection`" in `🏪️store/🔄️sync/🧬️schema/🔣️command-rejection/🔣️.json` (editing it could invalidate digests in `🗣️dsl/✨️derive/🗂️schema-document-index.json`; the region now exists in the parent and in `🪢️wire-contracts`).

Additional files edited in this follow-up: `O/🔨️modules/🧾️app-channel/🟦️.ts`, `O/🔨️modules/{🎁️packs,🪢️wire-contracts,🧵️backbone,📞️directory-client}/🟦️.ts`, `O/🟦️.ts`, `R/🟦️.tsx` and the 12 r3f children (markers only), plus the six inventory files above.
