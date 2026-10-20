# result-vit3: vitest configs with the real install (generation 9e8ff88...)

Executor `vit3`, foreground, no sub-agents, no `bun test`, no git writes. Runner: `C/vit3-tools/runvt.py` (vit2 runner, output dir `vit3-out`, one config per `node vitest.mjs run --config <abs>` process, `SEMIO_SCRIPT_PROVIDERS` and invocation envelope issued by `provwire.ts` / `envwire.ts`); single config via `C/vit3-tools/one.sh <config> [cap]`. Helpers: `fails.py` (failure digest), `stripscan.mjs` (Node strip-only erasability scan), `ppfix.mjs` (parameter property codemod, TS AST), `introchain.mjs` (import chain of the introduction graph law). Chromium 1248 not installed and not downloaded.

## Result

60 configs: vit2 28 green -> first run today 37 green -> after fixes 42 green (full second pass `r2`, all 60 run once more after the last edit). The machine ran at load 60-190 (peer fleets), so timeouts (30 s/60 s test timeouts, 300 s caps) are noisy: `course-react` (3 timeout failures on the second pass, 1 stable failure) and `engine react` (cap hit on the first pass, finished at 398 s later) depend on load.

Note: the vit2 per-config table was swept with `vit2-out`; the "vit2" column below is reconstructed from `result-vit2.md` (green set = fem, hub testing, pets, server, framework root, shell and the other plain-green suites; red causes as listed there).

## Config -> vit2 -> now -> fix

| config | vit2 | now | fix / cause |
|---|---|---|---|
| mit-bestand demonstrator | red (missing `@dnd-kit`-class + fixtures) | GREEN 5 files / 31 tests | `demonstratorpanebranding` read `hub/compositions/<x>/🔣️.json` (moved to `🧩️component/🔣️.json`, 8 paths); `📜️script.ts` imported `runVitest` statically from the library testing barrel (loads `bun:sqlite` under Node) -> dynamic import inside `TestScript.run` |
| hub admin | red (`dnd-kit`, `bun:sqlite`) | GREEN 3 files / 22 tests | two React copies (hub workspace generation 02ad vs root 9e88, same version): config aliases `react`, `react/jsx-*`, `react-dom*`, `@testing-library/react` to the repo-root copies; `command-routing` imports `vitestRunArguments` from its owner instead of the `repo-lib/testing` barrel |
| os root | red (retry, empty-state, inbox source) | GREEN 36 files / 771 tests | `empty-state` test exported a function but never registered `it` -> registers it; three source-shape laws (`actorreturnresponseframing`, `heartbeat-and-liveness-watchdogs`) required `shardWorkerSource` to return one template, the split now returns `[...].join("")` -> accept that shape |
| kernel | red? (n/a) | GREEN 10 / 93 | `package-assets` extracted `spliceInstanceOpenAssets` from the old monolithic `shard-worker-source/🟦️.ts`; it now lives in `🔌️actor-loading-and-replies/🟦️.ts` |
| course (framework product) | red | GREEN 16 / 552 | `🎲️seeded-randomness` fixture: the quiz->course rename rewrote the hashed text `❓️quiz` to `📚️course`; hash recomputed to 1336824646 (independent Python FNV-1a and Node agree; the old value 2446261078 is FNV-1a of `❓️quiz`). Python oracle `🐍️.py` not run (no numpy) |
| teaching architecture course | red (tar-stream, realpath, ticket) | RED 2 failed / 11 passed | `/var` vs `/private/var` and "CDN staging ancestry cannot contain links": tests use `realpathSync(os.tmpdir())` now (local-stack, deploy). Left: `tar-stream` missing (see class A), `proctor-build` needs the original proctor ticket env |
| play | red (http-cache-semantics, fresh-build) | RED 1 file (56 tests pass) | `playfreshbuild` read the site `📜️script.ts`; the logic is in `📦️site/🟦️.ts` -> test + fixture caller re-pointed. Left: `http-cache-semantics` not linked in play's workspace node_modules (class A) |
| gis service-composition | red (guest fault) | RED 1 test | 4 inference-port tests used one `../` too few for the fixture (moved one level deeper); `document-open` called `createColdPairWorkerClass` before `openColdPairSession` defined `state/fields/documentId` (TDZ, split artifact) -> reordered. Left: `browser actor child ... reactor/poll: guest fault` then assertion `expected undefined to be null` in cold-pair test (wasm guest, class D) |
| gis services | red | RED (startup) | strip-only fixed (`FileObservationErrorV1`, `CurrentPhysicalOwnerV1`); now stops at JSON import attributes in native-build storage (class B) |
| gis inference bridge | red (startup `paths[0]`) | RED 1 file | config used Bun-only `import.meta.dir` and `mergeConfig` (arrays concatenate, so it re-included all os-mcp tests) -> Node-safe path + override of `include`. Left: `playgroundNativeHostArtifactV1` finds no declared native MCP host (needs the native build, class D) |
| ui-react | red | RED 3 failed / 43 passed (was 6/40) | `color` package is declared nowhere -> ShellScope contrast oracle uses declared `colord` + a11y plugin; `Tree` portal law reads `🌳️Tree/🧲️drag-drop`; Canvas storybook play module removed from vitest `include` (not a suite); introduction source-graph law: 66 `reactHostPort` imports re-pointed from the `Ports` barrel (scene/flow bindings) to `Ports/⚛️react/🟦️.ts` in ui elements/targets (introchain.mjs: graph clean). Left: `.storybook scope-resolution` (`bun:ffi` via native-build jobserver, class B), Chromium 1248 (Tree text-flow, Layout) |
| catalog (dev) | red | RED 3 failed | strip-only fixed; left: JSON attributes (B), generator stderr `MODULE_TYPELESS_PACKAGE_JSON` (needs `"type":"module"` in a `package.json`, frozen), `catalog source audit cancelled` |
| hub tests | red | RED 1 | strip-only fixed; JSON attributes (B) in `integration` |
| registry | red | RED 2 failed | strip-only fixed; JSON attributes (B) |
| registry composition | red | RED (startup) | strip-only fixed; JSON attributes (B) |
| os dev | red | RED 5 failed | JSON attributes (B), Chromium 1248 hot-update (C), `local-hub` status sequence under load (timing) |
| os mcp | red | RED 5 failed | JSON attributes (B), `semio-os-mcp` binary absent (D) |
| cad dev | red (brepjs) | RED 3 failed / 10 passed | `linear-prism` fixture path moved to `🧪️tests/🧫️fixtures` (fixed); left: `geometry.current-browser-binding-required` (session binding must be regenerated/qualified, D), `SemioGeometryCloseReceiptV1` missing `phase` (peer wire change), `rules of hooks` hook-count laws (renderer: counts differ between mount and rerender), brepjs oracle wasm `unreachable` |
| engine react | red (timeout) | RED 40 failed / 181 | wasm engine packages (`framework-os-node-graph-rs`, `surface-rs`, `editor-rs`, `flow-core`) are stubbed in the config; ~35 `X is not a function` come from names that only the built wasm-bindgen package exports; 12 Chromium 1248 |
| wgpu | red | RED 14 failed | Rust-source text laws over files peers are splitting (`UI_WORKER_RETIREMENT_POLICY`, native host views): peer-active |
| actor | red | RED 1 file | `wire-turn` lane vocabulary: TS says 14/15 lanes, Rust enum has 16 (peer-active) |
| 2d | red | RED 1 | Chromium 1248 |
| course react | red | RED 1 stable + 3 load timeouts | `pets-on-stage` cannot find `role=group name=Pets` (not investigated); `navigation`/`journey`/`deputy` time out at load 100+ |
| presentation, pets, pets react, server, async, replication, machine, styling, canvas react-renderer, 3d, schema, r3f, repo client mcp/sqlite, coordinator, framework root, shell, window-kits, store, fem, remodel, trinity lsp, cad extensions x4, puzzle x3, layout, gismap renderer, hub testing, stdio publication, presentation-react, kernel label fixture, root `tests` | green | GREEN | unchanged |

## Remaining blockers by class

- A. Workspace `node_modules` links lag the root generation (the root is 9e88; hub 02ad, play f0bd, teaching 49c6, cad 08a6/9521, ...): `tar-stream` (installed in 9e88 and 4506, not linked for teaching), `http-cache-semantics` (installed only in 9521, play links f0bd), `retry` (resolved since the os root run, no longer reported), `color` (declared nowhere). Dependency-sync/`package.json` owner (frozen).
- B. Node-native load of `native-build/💾️storage` (and `📥️inputs`, `💾️state/📋️layout`, `💾️cache/📋️model`): about 25 `import x from "./🧬️schema/🔣️.json"` without `with { type: "json" }` (`ERR_IMPORT_ATTRIBUTE_MISSING`) and static `bun:ffi` in `🎰️jobserver`. Blocks gis services, registry (2 configs), hub tests, catalog, mcp, os dev, ui-react scope-resolution. These are the storage/native-build sources the brief puts in the frozen library: recorded, not fixed. A full list of unattributed JSON imports under `🏃️process`/`📁️filesystem` was produced (non-test: `💾️storage/**`, `📥️inputs/🟦️.ts`, `💾️state/**`, `💾️cache/📋️model`, `📤️publication/📋️manifest`, `🧪️testing/🦀️cargo/🎯️exact/**`, `🧪️testing/🟨️bun/📁️files`).
- C. Chromium headless shell 1248 absent (1234 present): 2d, ui-react (Tree text-flow, Layout), os dev hot-update, engine react storybook hosts.
- D. Native/wasm builds not present: `semio-os-mcp` binary, gis playground native host, wasm-bindgen engine packages, wasm guest (`reactor/poll: guest fault`), cad browser binding.
- E. Peer-active source changes: actor `wire-turn` lanes, wgpu Rust-text laws, cad close-receipt `phase`.
- F. Environment: proctor-build needs the original proctor test ticket; catalog generators emit `MODULE_TYPELESS_PACKAGE_JSON` (frozen package.json); load-dependent timeouts.

## Changes (files)

Erasable-syntax (parameter properties -> explicit fields, TS AST codemod `ppfix.mjs`, behaviour identical): `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts`, `.../🧾️observation/📁️current/🟦️.ts`, `.../📁️filesystem/📷️snapshot/🟦️.ts`, `.../🏃️process/💾️state/🎟️lifetime/🟦️.ts`, `.../native-build/📥️inputs/🤲️continuations/🟦️.ts`, `.../native-build/🧩️component-pass/📥️planning/🟦️.ts`, `.../native-build/📥️inputs/🧪️tests/📥️consumers/🟦️.ts`, `.../🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`. Caveat: the last five were found by the scan, not by a failing config, and four sit in `native-build` / `🏃️process`, which the dependency sync loads; they are outside the literal freeze paths, but if the sync restarted around 00:00 on 10-20 these are the suspects (revert is a mechanical re-inline of the constructor).

Tests/configs/fixtures: `♻️mit-bestand/🧺️demonstrator/📜️script.ts`, `.../🧪️demonstratorpanebranding/🟦️.ts`; `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🎚️config/🟦️.ts`, `.../🧪️command-routing/🟦️.ts`; `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/💡️inference-port/{🔑️broker-session-authority,🔒️sealed-owner-retention,🪪️receipt-and-lease-admission,🔌️broker-port-retirement-and-queue}/🟦️.ts`, `.../🗺️document-open/🟦️.ts`; `.../💡️services/🧪️tests/🎚️config/🧵️gis-inference-bridge/🟦️.ts`; `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧠️semio/🧪️tests/🧪️semio-tech-cad-js-spatial-kernel-semio/🟦️.ts`; `🎓️teaching/🏛️architecture/📚️course/🧪️tests/{🧪️deploy,🧱️local-stack}/🟦️.ts`; os plugin tests `🚫️empty-state`, `🧪️actorreturnresponseframing-...`, `🧪️shardclient-reserved-response-settlement/💓️heartbeat-and-liveness-watchdogs`; `🎠️kernel/🧪️tests/📦️package-assets/🟦️.ts`; `📚️course/🧫️fixtures/🎲️seeded-randomness/🔣️.json`; play `🧪️playfreshbuild/{🟦️.ts,🧫️fixtures/🔣️.json}`; ui: `🐚️ShellScope/🧪️tests/🧩️component/🟦️.tsx`, `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, `📐️introduction-geometry`, `🧭️introduction-resolve`, `🎬️introduction-demo`, `🎓️introduction` and 66 element/target files whose only import was `{ reactHostPort }` from the Ports barrel (now `🔌️Ports/⚛️react/🟦️.ts`).

Nothing edited under `🦑️repo/🔨️modules/📚️library/`, no `package.json`/`bun.lock`/`🔣️taxonomy.json`, no root `📜️script.ts`.

## Verification

- `python3 C/vit3-tools/runvt.py C/vit3-tools/list.txt C/🗑️generated/vit3-out/r1.jsonl|r2.jsonl <budget> <cap>`: r1 = 37/60 green, r2 (after all fixes) = 42/60 green; per-config re-runs with `one.sh` after each fix (hub admin 14 -> 0 failures, os root 3 -> 0, mit-bestand 2 -> 0, kernel 1 -> 0, course 1 -> 0, teaching 4 -> 2, play 2 -> 1, gis service-composition 9 -> 1, ui-react 6 -> 3).
- `bun build --target=bun 📜️script.ts --outfile=/dev/null`: 2,465 modules bundled (CLI closure intact after the filesystem/process edits).
- `node C/vit3-tools/introchain.mjs`: introduction graph no longer reaches the Ports barrel or the react target barrel (92 -> 91 modules).
- Not run: Python seeded-randomness oracle (numpy missing), Rust twins of the course fixture, Chromium-mode configs, cargo, any `bun test`.
- Generated output (`🗑️generated/vit3-out`) deleted; no `[DEBUG]` lines added.
