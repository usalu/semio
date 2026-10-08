# M-2: Declaration Gaps After Dissolving The Launch Files

Slice M-2 of round 2 (`r2-plan.md`). Written by the coordinator from the slice's outputs: the slice's own
report write was refused by its harness. Source of truth: `r2-m2-mapping.json` (machine-readable, keys
`claude`, `compounds`, `deviations`) and the appendix below (the 79 `.claude/launch.json` rows as a table). Helpers: `r2-m2-resolve.py`, `r2-m2-validate.ts`, `r2-m2-tables.py`, `r2-m2-mapping.py`.

## Round 2b: `continuous` On Server And Watcher Targets

Audit of every manifest target (outside tickets) that looks like a server or watcher: by name (`dev serve start watch preview
storybook activate`), by declared `ready`, and by command words (`serve dev watch start listen vite storybook`);
`r2-m2-continuous-scan.py`, `r2-m2-continuous-scan2.py`. 74 + 63 hits, read one by one against their scripts.

- Set `"continuous": true` (only that key, inserted after the target's `cache`, `r2-m2-continuous-edit.py`):
  `workspace:dev` (the generic dev launcher: storybook, playgrounds, mcp), `workspace:dev-mcp-engine` (the plain MCP inspector),
  `os-hub:scoped-presence-browser-serve` (Vite server held until SIGINT), `@semio-tech/framework-os-dev:serve-hold` (holds the
  shared serve fixture until stopped).
- Already long-running: every target with a declared `ready` (13 `dev-storybook*`, `dev-mcp`, `dev-mcp-repo`, hub, proctor,
  quiz, pets, demonstrator, play, admin) and the 50 others that were `continuous`; playground `dev-*` targets are inferred by the
  plugin and long-running by kind.
- Checked and left alone (finite): `*-preview-generated`, `preview-distribution`, `preview-playground-session` (print a generated
  tree), `activate-*`/`prepare-*` (receipts), `residency-watch`, `boot-watch` (bounded rounds), `dev-collaboration`,
  `dev-corpus`/`dev-latency` (tests), `support-dev`, `component-dev`, `native-scale` (benchmarks), `*-build-dev`.
- Verified with the debug binary built by A-2 at 04:26 (my own build was starved by shared lock waits): `semio run <id> --dry-run`
  shows `longRunning: true` for the four edited ids and for `workspace:dev-storybook`, `@teaching/architecture-quiz:dev`;
  `semio commands --check --refresh --root /c/git/semio`: 20409 commands, 0 problems.
- Not mine: the bare flag `--param steady` (without `=true`) is A-1's change.

## Result

- Every `.claude/launch.json` entry (79), every VS Code compound (4) and every M-1a deviation family (11) has a
  `semio run …` equivalent, a declaration added in this slice, or a recorded reason for dropping it.
- `.claude` entries: 69 exact, 9 declared now, 5 intentionally dropped (dead targets, missing scripts, closed or
  non-ticket folders, one attach port without a playground). Attach-only entries map to declared ready ports.
- Compounds: 4 of 4 exact (`compound:workspace/s-with-hub` and siblings; stop together).
- `semio commands --check --root /c/git/semio`: 0 problems (20,380 commands) after removing three dead
  `describe` groups that produced 14 problems at the start. The edited declaration files pass Ajv against
  `🎛️dashboard/🧬️schema/🎮️registry/🔣️.json`.

## Declared In This Slice

| Need | Declaration |
| --- | --- |
| Quiz without reload on others' edits | flag `steady` on `@teaching/architecture-quiz:dev` (`TEACHING_ARCHITECTURE_QUIZ_WATCH=off`) |
| Second quiz stack beside a running one | choice `stack=beside` on the same target: ports 6063 / 8793, own `PROCTOR_DATA`; ready port follows the choice |
| Static storybook with readiness | `tool:workspace/storybook-static`, ready port 6010 |
| stdio initial-record tests | `group:workspace/stdio-initial-record-tests` (jpg, tiff, pdf) |
| MCP inspector URL with session token | `ready.printed: true` on `workspace:dev-mcp`, `workspace:dev-mcp-repo`, `tool:workspace/mcp-inspector-os` (printed URL not run) |
| Open-ticket tools | `🎮️commands.json` for `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS` and `26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D` |

One edit outside a dashboard block: `TEACHING_ARCHITECTURE_QUIZ_PORT: "6061"` removed from `options.env` of the
quiz `dev` and `dev-site` targets, because Nx lets `options.env` override the client environment
(`running-tasks.js:516-531`); 6061 stays the default through `package.json` `semio.app.port.dev`.

## Requests (dispatched)

- A-1: expand `{workspace}` and other registry tokens inside `--env` values.
- Optional, not dispatched: a launch-time project subset for groups (two snapshot rows now run every project
  that declares the target).
- S-1: quiz README mentions `--param steady` and `--param stack=beside`.

## Not Verified

- Dry-runs used the `semio.exe` built by A-2 at 04:26 (newer than the last registry edit at 04:06); the slice's
  own build was cancelled because of shared-lock contention.
- No server was started.

## Appendix: `.claude/launch.json` Entries

| # | Original | Dashboard command | Status | Note |
| --- | --- | --- | --- | --- |
| 1 | `⚖️gate🎒️pack🫳️borrowed-preflight🦀️native` | `semio run @semio-tech/framework-pack-rs:test-borrowed-preflight-native --param cache=skip-local --param test-level=quick` | exact |  |
| 2 | `⚖️gate🎒️pack🫳️borrowed-preflight🟦️source` | `semio run @semio-tech/framework-pack-rs:test-borrowed-preflight-source --param cache=skip-local --param test-level=quick` | exact |  |
| 3 | `cad-react` | `semio run playground:cad --param renderer=react` | exact | renderer now explicit (`react`); the old command had no `SEMIO_RENDERER` and bound the wgpu port |
| 4 | `s-react` | `semio run playground:s --param renderer=react` | exact | renderer now explicit (`react`); the old command had no `SEMIO_RENDERER` and bound the wgpu port |
| 5 | `s-react-served` | `semio run playground:s --param renderer=react -- served` | exact |  |
| 6 | `procedural3d-react` | `semio run playground:generation3d --param renderer=react -- served` | exact |  |
| 7 | `procedural3d-wgpu` | `semio run playground:generation3d --param renderer=wgpu-wasm` | exact |  |
| 8 | `puzzle3d-react` | `semio run playground:puzzle3d --param renderer=react` | exact |  |
| 9 | `puzzle3d-wgpu` | `semio run playground:puzzle3d --param renderer=wgpu-wasm` | exact |  |
| 10 | `puzzle5d-react` | `semio run playground:puzzle5d --param renderer=react -- served` | exact |  |
| 11 | `puzzle5d-wgpu` | `semio run playground:puzzle5d --param renderer=wgpu-wasm` | exact |  |
| 12 | `puzzle5d-native` | `semio run playground:puzzle5d --param renderer=wgpu-native` | exact | resolves to `framework-os-dev:run-puzzle5d-native-dev`, the target `renderer-wgpu:native -- puzzle5d` reduces to; no URL, so no ready |
| 13 | `dag-react` | `semio run playground:dag --param renderer=react` | exact | renderer now explicit (`react`); the old command had no `SEMIO_RENDERER` and bound the wgpu port |
| 14 | `terra-jco-spike-static` | - | intentionally dropped | the script `🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/📜️script.ts` no longer exists (the testkit folder is gone) |
| 15 | `mit-bestand-demonstrator` | `semio run @semio-tech/mit-bestand-demonstrator:dev` | exact |  |
| 16 | `mit-bestand-demonstrator-supervised` | `semio run ticket:26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS/demonstrator-supervised` | declared now | ticket `DEMONSTRATOR-END-TO-END-ALL-APPS` is open: `🎮️commands.json` created, tool `demonstrator-supervised` |
| 17 | `map-harness` | - | intentionally dropped | ticket `GIS-MAP-END-TO-END` is closed and the entry's `📜️harnessscript.ts` does not exist |
| 18 | `storybook-framework-hosts` | `semio run workspace:dev-storybook-framework-hosts` | exact |  |
| 19 | `gis2d-wgpu` | `semio run playground:gis2d --param renderer=wgpu-wasm` | exact |  |
| 20 | `verify-gis2d-envelope-gates` | - | intentionally dropped | folder `26/09/16/GIS-2D-END-TO-END-BUILD` has no `🎫️ticket.json`, so it is no open ticket; the registry cannot index a `🎮️commands.json` there |
| 21 | `puzzle2d-react` | `semio run playground:puzzle2d --param renderer=react` | exact | renderer now explicit (`react`); the old command had no `SEMIO_RENDERER` and bound the wgpu port |
| 22 | `storybook-static` | `semio run tool:workspace/storybook-static` | declared now | was `workspace:dev -- storybook-static` without a ready declaration (`--wait-ready` would hang); tool with ready 6010 / `STORYBOOK_PORT` |
| 23 | `storybook-framework-os` | `semio run workspace:dev-storybook-framework-os` | exact | the entry's port 6011 never matched: the command binds 6010 (`STORYBOOK_PORT`), which is the declared ready port |
| 24 | `puzzle3d-react-attach` | no command: `semio tasks`, then `semio open playground:puzzle3d` (start with `semio run playground:puzzle3d --param renderer=react --detach --wait-ready`) | exact | port 6013 = ready port 6013 of `playground:puzzle3d` (react) |
| 25 | `puzzle3d-react-release-e2e-attach` | no command: `semio tasks`, then `semio open playground:puzzle5d` (start with `semio run playground:puzzle5d --param renderer=react --detach --wait-ready`) | exact | port 6014 = ready port 6014 of `playground:puzzle5d` (react) |
| 26 | `puzzle5d-react-attach` | no command: `semio tasks`, then `semio open playground:puzzle5d` (start with `semio run playground:puzzle5d --param renderer=react --detach --wait-ready`) | exact | port 6014 = ready port 6014 of `playground:puzzle5d` (react) |
| 27 | `puzzle5d-react-supervised` | `semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/serve-supervised --param variant=puzzle5d` | declared now | ticket is open: tool `serve-supervised` (choice `variant` puzzle5d/puzzle3d/puzzle2d carries variant and port) |
| 28 | `puzzle3d-react-supervised` | `semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/serve-supervised --param variant=puzzle3d` | declared now | same tool |
| 29 | `puzzle5d-battery` | `semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/puzzle5d-battery` | declared now | ticket is open: tool `puzzle5d-battery` |
| 30 | `puzzle5d-battery-explore` | `semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/puzzle5d-battery-explore` | declared now | ticket is open: tool `puzzle5d-battery-explore` |
| 31 | `puzzle2d-react-supervised` | `semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/serve-supervised --param variant=puzzle2d` | declared now | same tool |
| 32 | `procedural3d-react-attach` | no command: `semio tasks`, then `semio open playground:generation3d` (start with `semio run playground:generation3d --param renderer=react --detach --wait-ready`) | exact | port 6018 = ready port 6018 of `playground:generation3d` (react) |
| 33 | `process3d-react` | `semio run playground:process3d --param renderer=react -- served` | exact |  |
| 34 | `process3d-react-attach` | no command: `semio tasks`, then `semio open playground:process3d` (start with `semio run playground:process3d --param renderer=react --detach --wait-ready`) | exact | port 6022 = ready port 6022 of `playground:process3d` (react) |
| 35 | `process3d-react-lane-attach` | - | intentionally dropped | port 6222 belongs to no catalog playground (a ticket lane) |
| 36 | `fem2d-react` | `semio run playground:fem2d --param renderer=react -- served` | exact |  |
| 37 | `fem2d-react-attach` | no command: `semio tasks`, then `semio open playground:fem2d` (start with `semio run playground:fem2d --param renderer=react --detach --wait-ready`) | exact | port 6086 = ready port 6086 of `playground:fem2d` (react) |
| 38 | `fem3d-react` | `semio run playground:fem3d --param renderer=react -- served` | exact |  |
| 39 | `fem3d-react-attach` | no command: `semio tasks`, then `semio open playground:fem3d` (start with `semio run playground:fem3d --param renderer=react --detach --wait-ready`) | exact | port 6087 = ready port 6087 of `playground:fem3d` (react) |
| 40 | `energy-react` | `semio run playground:energy --param renderer=react -- served` | exact |  |
| 41 | `energy-react-attach` | no command: `semio tasks`, then `semio open playground:energy` (start with `semio run playground:energy --param renderer=react --detach --wait-ready`) | exact | port 6106 = ready port 6106 of `playground:energy` (react) |
| 42 | `forms-react-attach` | no command: `semio tasks`, then `semio open playground:forms` (start with `semio run playground:forms --param renderer=react --detach --wait-ready`) | exact | port 6058 = ready port 6058 of `playground:forms` (react) |
| 43 | `raster-react-attach` | no command: `semio tasks`, then `semio open playground:raster` (start with `semio run playground:raster --param renderer=react --detach --wait-ready`) | exact | port 6060 = ready port 6060 of `playground:raster` (react) |
| 44 | `shooting-react-attach` | no command: `semio tasks`, then `semio open playground:shooting` (start with `semio run playground:shooting --param renderer=react --detach --wait-ready`) | exact | port 6019 = ready port 6019 of `playground:shooting` (react) |
| 45 | `remodel-react-attach` | no command: `semio tasks`, then `semio open playground:remodel` (start with `semio run playground:remodel --param renderer=react --detach --wait-ready`) | exact | port 6063 = ready port 6063 of `playground:remodel` (react) |
| 46 | `layout-react-attach` | no command: `semio tasks`, then `semio open playground:layout` (start with `semio run playground:layout --param renderer=react --detach --wait-ready`) | exact | port 6079 = ready port 6079 of `playground:layout` (react) |
| 47 | `note-react-attach` | no command: `semio tasks`, then `semio open playground:note` (start with `semio run playground:note --param renderer=react --detach --wait-ready`) | exact | port 6080 = ready port 6080 of `playground:note` (react) |
| 48 | `wfc-bitmap-react-attach` | no command: `semio tasks`, then `semio open playground:bitmap` (start with `semio run playground:bitmap --param renderer=react --detach --wait-ready`) | exact | port 6041 = ready port 6041 of `playground:bitmap` (react) |
| 49 | `wfc-grid2d-react-attach` | no command: `semio tasks`, then `semio open playground:grid2d` (start with `semio run playground:grid2d --param renderer=react --detach --wait-ready`) | exact | port 6042 = ready port 6042 of `playground:grid2d` (react) |
| 50 | `wfc-wfc2d-react-attach` | no command: `semio tasks`, then `semio open playground:wfc2d` (start with `semio run playground:wfc2d --param renderer=react --detach --wait-ready`) | exact | port 6043 = ready port 6043 of `playground:wfc2d` (react) |
| 51 | `wfc-grid3d-react-attach` | no command: `semio tasks`, then `semio open playground:grid3d` (start with `semio run playground:grid3d --param renderer=react --detach --wait-ready`) | exact | port 6044 = ready port 6044 of `playground:grid3d` (react) |
| 52 | `wfc-wfc3d-react-attach` | no command: `semio tasks`, then `semio open playground:wfc3d` (start with `semio run playground:wfc3d --param renderer=react --detach --wait-ready`) | exact | port 6045 = ready port 6045 of `playground:wfc3d` (react) |
| 53 | `reasoning-react` | `semio run playground:reasoning-wires --param renderer=react -- served` | exact |  |
| 54 | `reasoning-react-attach` | no command: `semio tasks`, then `semio open playground:reasoning-wires` (start with `semio run playground:reasoning-wires --param renderer=react --detach --wait-ready`) | exact | port 6015 = ready port 6015 of `playground:reasoning-wires` (react) |
| 55 | `imperative-react` | `semio run playground:imperative --param renderer=react -- served` | exact |  |
| 56 | `imperative-react-attach` | no command: `semio tasks`, then `semio open playground:imperative` (start with `semio run playground:imperative --param renderer=react --detach --wait-ready`) | exact | port 6076 = ready port 6076 of `playground:imperative` (react) |
| 57 | `playbook-react` | `semio run playground:playbook --param renderer=react -- served` | exact |  |
| 58 | `playbook-react-attach` | no command: `semio tasks`, then `semio open playground:playbook` (start with `semio run playground:playbook --param renderer=react --detach --wait-ready`) | exact | port 6085 = ready port 6085 of `playground:playbook` (react) |
| 59 | `norm-react` | `semio run playground:din4108 --param renderer=react -- served` | exact |  |
| 60 | `norm-react-attach` | no command: `semio tasks`, then `semio open playground:din4108` (start with `semio run playground:din4108 --param renderer=react --detach --wait-ready`) | exact | port 6091 = ready port 6091 of `playground:din4108` (react) |
| 61 | `teaching-proctor` | `semio run @teaching/proctor:dev` | exact |  |
| 62 | `architecture-quiz` | `semio run @teaching/architecture-quiz:dev` | exact | `PROCTOR_PORT=8791` is the owner default |
| 63 | `host-count-component-sqlite` | `semio run @semio-tech/framework-plugin-host:count-component-check [--env SEMIO_TEST_ARTIFACT_DIR=<dir>]` | exact | the artifact directory is an output location, passed with `--env` when wanted |
| 64 | `architecture-quiz-site` | `semio run @teaching/architecture-quiz:dev-site` | exact | `PROCTOR_PORT=8791` is the owner default |
| 65 | `architektur-und-technologie-quizze` | `semio run @teaching/architecture-quiz:dev` | exact | `PROCTOR_PORT=8791` is the owner default |
| 66 | `architektur-und-technologie-quizze-steady` | `semio run @teaching/architecture-quiz:dev --param steady` | declared now | new flag `steady` = `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` |
| 67 | `architektur-und-technologie-quizze-beside` | `semio run @teaching/architecture-quiz:dev --param stack=beside` | declared now | new choice `stack=beside` = ports 6063 / 8793 and `PROCTOR_DATA={workspace}/.🧬semio/🎓️teaching/proctor-beside`; the Nx `options.env` pin of 6061 was removed so the choice can take effect |
| 68 | `framework-snapshot-sqlite` | `semio run @semio-tech/framework-rs:test-snapshot-sqlite` | exact |  |
| 69 | `framework-deflate-encoding` | `semio run @semio-tech/framework-rs:test-deflate-encoding` | exact |  |
| 70 | `framework-snapshot-sqlite-native` | `semio run @semio-tech/framework-rs:test-snapshot-sqlite-native` | exact |  |
| 71 | `framework-snapshot-sqlite-source` | `semio run @semio-tech/framework-rs:test-snapshot-sqlite-source` | exact |  |
| 72 | `framework-snapshot-sqlite-io` | - | intentionally dropped | target `@semio-tech/framework-rs:test-snapshot-sqlite-io` no longer exists |
| 73 | `value-borrowed-key-index-native` | `semio run @semio-tech/framework-os-kernel:test-borrowed-key-index-native --param cache=skip-local --param test-level=quick` | exact |  |
| 74 | `value-intrinsic-retirement-native` | `semio run @semio-tech/framework-os-kernel:test-intrinsic-retirement-native --param cache=skip-local --param test-level=quick` | exact |  |
| 75 | `value-controlled-construction` | `semio run @semio-tech/value-rs:test-controlled-construction` | exact |  |
| 76 | `value-controlled-construction-source` | `semio run @semio-tech/framework-value:test-controlled-construction-source` | exact |  |
| 77 | `pptx-outline-ownership-source` | `semio run @semio-tech/stdio-pptx-rs:test-outline-ownership --param cache=skip-local [--env SEMIO_TEST_ARTIFACT_DIR=<dir>]` | exact | the artifact directory is an output location, passed with `--env` when wanted |
| 78 | `pets-stories` | `semio run @semio-tech/pets-react:dev --param stories-port=6069` | exact |  |
| 79 | `architecture-pets-stories` | `semio run @semio-tech/pets-react:dev --param stories-port=6074 --param menagerie=🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts` | exact | second gallery: own port and menagerie |
