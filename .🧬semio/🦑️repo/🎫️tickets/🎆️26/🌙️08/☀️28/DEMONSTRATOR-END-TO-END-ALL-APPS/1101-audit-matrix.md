STATUS: AUDIT 13:08 (aud-matrix). Nothing is proven end to end. 0 of 8 panes has a passing smoke test, activation is red (act14 rc 1) with act15 still running, and no browser run against a server exists. Framework gate marker `framework-green` (07:05) rests on one gate run (`plugjob-29`, rc 0) and is contradicted by later framework red logs. Refresh at 13:21 (rolling); next refresh due 13:36. At 13:21: `act15` still running (log at 13:20 in `bazel:source-snapshot`, no exit line), `rtpanes-aussuchen` still running (no rc file), `gis-9` rc 1 (13:07).

# 1101 Audit Matrix (aud-matrix)

Read-only audit of `C/1101-result-*.md` and `C/🗑️generated/1101/` logs. Cell format: `<log> rc <n> (<mtime>)`, or `not run`, or `red`/`partial` with the citing log. Log paths are under `C/🗑️generated/1101/`.

Pane-to-lane mapping: the handover (`C/1028-HANDOVER.md`) names statik = fem3d and energie = energy. The other pane-to-lane links are inferred from pane behaviour in `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e` (for example verfolgen = map = gis, aussuchen = stock sourcing) and are NOT confirmed by any lane result. Treat them as a working assumption.

## Matrix

| Pane (lane, inferred) | wasm check | native tests | smoke test (rtpanes) | activation | browser |
|---|---|---|---|---|---|
| generator (gen, geninputs) | not proven: `gen-hub13.txt` rc 1 (12:10), `gen-hub12.txt` rc 1 (12:08), `gen-art1.txt` rc 1 (13:03); red only in foreign `stdio-pdf-schema-4`, `stdio-png-tests` | partial: `geninputs-tests.txt` rc 0 (04:37), funding test only; generation3d native tests not run | red: `rtpanes-generator.txt` rc 1 (10:15), 10 errors in `stdio-deflate-examples` | not proven: `act14.status` exit 1 (03:00); `act15` running | not run |
| koordinator (cad, inferred) | not proven: `cad-8.txt` rc 1 (12:47), 7 errors, blocked by `stdio-pdf-schema-4`; `cad-presence` has no log | not run | red: `rtpanes-koordinator.txt` rc 1 (08:57), source refused in `plugin-testing-acceptance` | not proven (as above) | not run |
| aggregator (flow, inferred) | not proven: `flowext-hub-flowplaybook-m.txt` rc 1 (12:52), `flow-inferences` red; `flowext-flow-model-c.txt` rc 0 is one crate only | not run | red: `rtpanes-aggregator.txt` rc 1 (09:11), framework `os-flow-host-core` mismatched types | not proven | not run |
| aussuchen (sourcing, inferred) | partial: `hubplug-11.txt` rc 1 (12:37), sourcing crates reported clean, hub red only in `stdio-pdf-schema-4` | not run | in progress: `rtpanes-aussuchen.txt` (pass s3, pid 76975 alive, no rc yet, 13:06) | not proven | not run |
| bearbeiten (process, inferred) | partial: `extfix-process-metal-r3.txt` rc 0, `robotic-r3` rc 0, `wood-r3` rc 0, `concrete-r3` rc 1; `stdio2-hub-1.txt` rc 1 (12:21) lists `process3d-editor` red | not run | red: `rtpanes-bearbeiten.txt` rc 1 (11:54), cargo:test hit the 3559768 ms deadline on a cold build, no test ran | not proven | not run |
| verfolgen (gis, inferred) | not proven: `gis-8.txt` rc 1 (10:11); `gis-9.txt` rc 1 (13:07), errors only in `stdio-pdf-schema-4`, `stdio-tiff-mutation-regressions`, `stdio-docx-base-model`; no gis crate reached | not run | red: `rtpanes-verfolgen.txt` rc 1 (08:53), `plugin-reactor-patches` missing `semio_framework_plugin_testing_caller` | not proven | not run (map tiles need GPU, per e2e note) |
| energie (energy, handover) | red: `energy-14.txt` rc 1 (11:15), 142 error lines, `energy-model-session` and `native-mutations` red; `energy-4.txt` rc 1 (07:22) | not run | red: `rtpanes-energie.txt` rc 1 (10:45), `energy-model-snapshot` `RetireOwned` bound | not proven: `act14` exit 1 | not run |
| statik (fem3d, handover) | 2d/engine only: `fem-4.txt` rc 0 (07:29); 3d not proven: `fem-20.txt` rc 1 (12:56), blocked by `stdio-pdf-schema-4` | not run (3d snapshot tests UNVERIFIED per `1101-result-fem.md`) | red: `rtpanes-statik.txt` rc 1 (12:49), `stdio-pdf-schema-4` E0432 | not proven | not run |
| framework | wasm gate: `plugjob-29.txt` rc 0 (07:05), `gate.sh` subset only; later red: `plugnext-hub-2.txt` rc 1 (08:13), `mutpolicy-fem2d.txt` rc 1 (08:16), `mutpolicy-wfc3d.txt` rc 1 (08:37) | green subsets: `coretest-mesh3.txt` rc 0, `coretest-engine1.txt` rc 0, `coretest-budgeted4.txt` rc 0, `pixtest-run-full5.txt` rc 0 (87/87), `compute-d.txt` rc 0 (6), `uicust-3.txt` rc 0 (5/5), `mutpolicy-retained.txt` rc 0, `geo2d-bool3`/`trace4`/`stroke2` rc 0; red: `testport-run-1.txt` rc 1 (12:55), store-sync-native; host closure `compute-b` still running | `rtpanes-*` rc 1 for all panes (see above); framework smoke not proven | `act14.status` exit 1 (03:00, graph-plan failed); `act15` running (log 13:06, "verifying input closures 999 of 1050"). `actinfra-graph.rc` = 1 (05:06), `actinfra-showrepo.rc` = 0 | not run |

Demonstrator-level Bazel targets (not per pane): `tsdemo2-generate.rc` 0, `tsdemo2-canonical-architecture.rc` 0, `tsdemo2-tsc-2.rc` 0; `tsdemo2-test.rc` 1, `tsdemo2-test-runtime-catalog.rc` 1, `tsdemo2-r2-test.rc` 1 (2 script.ts tests failed), `tsdemo2-r2-test-runtime-catalog.rc` 0.

Browser: the only browser runs are `1101-tools/e2eready/run.sh` dry runs against port 1 (`e2eready-dry/run.rc` 1, `generator@en` and `statik@en` BOOT FAIL by `ERR_UNSAFE_PORT`). That proves tooling, not panes. No server run exists for any pane.

## Crates with no owner

Method: every `could not compile` crate in the latest logs (`gen-hub13`, `hubplug-11`, `energy-14`, `fem-20`, `stdio2-hub-1`, `cad-8`, `gis-9`, `hub10`, `plugnext-hub-2`, `mutpolicy-*`, `replay-hub1`, `testport-run-1`, `gatepf-6`, `rtpanes-*`) was matched against the stem in all 45 `1101-result-*.md` files. Zero matches means no lane claims it.

Unowned and currently red (latest log in brackets):
- `semio-s-artifact-stdio-pdf-schema-4` [gen-art1 13:03, fem-20 12:56, cad-8 12:47, hubplug-11 12:37, gis-9 13:07]. Blocks 5 lanes; 4 results name it as blocker, no lane owns it. Highest priority.
- `semio-s-artifact-stdio-png-tests` [gen-hub13 12:10]
- `semio-s-artifact-stdio-docx-base-model` [gis-9 13:07]
- `semio-s-artifact-stdio-tiff-mutation-regressions` [gis-9 13:07]
- `semio-s-artifact-process-process3d-editor` [stdio2-hub-1 12:21]
- `semio-framework-os-store-sync-native` [testport-run-1 12:55]
- `semio-framework-plugin-reactor-patches` [rtpanes-verfolgen 08:53]; addendum 07:20 assigns `reactor-jobs` to plugcore, not this crate
- `semio-s-artifact-stdio-deflate-examples` [rtpanes-generator 10:15]; energy lane calls it an example crate, unowned
- `semio-s-artifact-energy-model-native-mutations-singletons` [stdio2-hub-1 12:21]; energy by prefix, no result names it
- `semio-s-artifact-flow-flow-schema` [hub10 10:53 only]; not in any later log
- `semio-s-fem-analyses` [replay-hub1 03:45]; stale, unowned

Unowned and only in stale logs (not in any recent log, likely resolved or no longer reached): `procedural-2d-mutations`, `procedural-2d-preview-eval`, `procedural-3d-geometry` (hub10 10:53; the addendum gives gen only the editor and viewer transients), `stdio-semio-*-snapshot` family and `stdio-semio-animation/audio/text/value/video-model` (gis-7 09:45), `stdio-bmp-editor-2` and `stdio-wav-io-registry` (hub10 10:53, gis-7 09:45).

Owned by name: `stdio-deflate-zlib` (describe, `desc-w7.txt` rc 0, 12:30, after the 12:21 red), `stdio-gltf-model-snapshot-root` (describe, same batch), `stdio-ifc-details-wire` (ifcsplit by description: "the wire crate").

## Lane claims contradicted by the logs

1. framework-green marker (`framework-green`, 07:05): written on `plugjob-29` alone (`gate.sh`, rc 0). After 07:05, framework crates still fail: `plugnext-hub-2.txt` rc 1 (08:13) `tool-run-turns`, `time-travel-turns`, `instance-typed-ops`; `mutpolicy-fem2d.txt` rc 1 (08:16) `tool-run-turns`, `instance-commands`; `mutpolicy-wfc3d.txt` rc 1 (08:37) `instance-initialization`; `rtpanes-aggregator.txt` rc 1 (09:11) framework `os-flow-host-core`. The marker is not a whole-framework proof. Coordinator should treat it as a gate-subset marker only.
2. `1101-result-actinfra.md` says GREEN for the graph ("0 failures of 2815"). `actinfra-graph.rc` = 1 (05:06), and the log's last line throws `BUILD_COLLECTED_ERRORS` while stating "0 of 2815 projects failed". The graph inference is not green by rc. Only `actinfra-showrepo.rc` = 0 is green.
3. `1101-result-rtpanes.md` says the s2 pass ended with "all rc 101". The rc files say 1: koordinator 08:57, verfolgen 08:53, aggregator 09:11, generator 10:15, energie 10:45. The result is stale.
4. `1101-result-e2eready.md` (13:05) says "tsc rc 0". `e2eready-tsc.txt` (13:04) has no rc marker and its tail is still `[repo] still working` and `[capacity] admitted command`. The claim is unbacked. Also "Dry-load GREEN (run.sh --check 8/8)" has no `--check` log in `🗑️generated/1101`.
5. `1101-result-ifc4x3.md` (12:23) says no repo cargo check has run. `ifc4x3-2.txt` (12:59, rc 1) shows an attempt that failed at argument parsing (`unexpected argument ' -p semio-...'`), so no crate compiled. The claim holds, but the invocation is malformed and should be fixed before the next run.
6. `1101-result-tsdemo2.md` section 5 "Rerun after fixes" is empty. The logs show `tsdemo2-r2-test.rc` 1 and `tsdemo2-test-runtime-catalog.rc` 1, so the demonstrator `test` target is not green.
7. `1101-result-gen.md`, `-energy.md`, `-fem.md` are consistent with their logs. Their UNVERIFIED flags match the rc values.

## Refresh plan

Re-collect rc values for `rtpanes-aussuchen`, `gis-9`, `act15`, `compute-b`, `energy-21`, `fem-21`, `cad-3`, `ifc4x3-3`, `stdio2-3`, `hubplug-t1` (all running at 13:06-13:07) and recompute this file every 15 minutes while lanes run.
