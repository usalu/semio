STATUS: AUDIT REFRESH 18:18 (aud-matrix, single refresh as requested). Nothing is proven end to end. 0 of 8 panes has a passing smoke test. act14 exit 1 (03:00), act15 exit 1 (14:59), act16 still running (since 15:04, no exit at 18:17). No browser run against a server exists. Framework gate marker `framework-green` (07:05) is a gate-subset marker only.

# 1101 Audit Matrix (aud-matrix)

Read-only audit of `C/1101-result-*.md` and `C/🗑️generated/1101/` logs. Cell format: `<log> rc <n> (<mtime>)`, or `not run`, or `running` (no rc yet). Log paths are under `C/🗑️generated/1101/`. Newest completed log per pane is used; stale values are marked.

Pane-to-lane mapping: statik = fem3d and energie = energy come from the handover. The rest is inferred from pane behaviour and is not confirmed by a lane result: generator = gen, koordinator = cad, aggregator = flow, aussuchen = sourcing, bearbeiten = process, verfolgen = gis.

Corrections applied (coordinator, 18:17): `stdio-pdf-*` is owned by stdio2. `process3d-editor` was fixed by hubplug and is removed from the unowned list. `actinfra-graph.rc` = 1 is by design: collect mode publishes nothing and exits 1, and the log says "0 of 2815 projects failed", so the graph inference is green and the rc is not a failure.

## Matrix

| Pane (lane) | wasm check (newest completed) | native tests | smoke test (rtpanes) | act14 (exit) | act15 (exit) | act16 (status) | browser |
|---|---|---|---|---|---|---|---|
| generator (gen) | `gen-hub14.txt` rc 1 (18:02): red `stdio-pdf` | partial: `geninputs-tests.txt` rc 0 (04:37), funding only; generation3d not run | `rtpanes-generator.txt` rc 1 (10:15, stale) | exit 1 (03:00) | exit 1 (14:59) | running (15:04) | not run |
| koordinator (cad) | `cad-8.txt` rc 1 (12:47, newest completed): `stdio-pdf-schema-4`; `cad-t2` running (18:17) | not run | `rtpanes-koordinator` running (18:17, no rc; last rc 1 at 08:57, stale) | exit 1 (03:00) | exit 1 (14:59) | running (15:04) | not run |
| aggregator (flow) | `flowext-hub-flowplaybook-o.txt` rc 0 (14:06), 0 error lines | not run | `rtpanes-aggregator.txt` rc 1 (16:21): framework `os-flow-extensions` (private `work` field, missing `take_work` in `ExtensionResourceOwner`) | exit 1 (03:00) | exit 1 (14:59) | running (15:04) | not run |
| aussuchen (sourcing) | `hubplug-11.txt` rc 1 (12:37, older); `hubplug-t7` running (18:17) | not run | `rtpanes-aussuchen.txt` rc 1 (13:32): framework test crate `test-original-snapshot-owners` (9 errors, `expect` on `RetainedWorkOutcome`) | exit 1 (03:00) | exit 1 (14:59) | running (15:04) | not run |
| bearbeiten (process) | `extfix-process-concrete-r4.txt` rc 0 (13:15); earlier `robotic-r3`, `metal-r3`, `wood-r3` rc 0 | not run | `rtpanes-bearbeiten.txt` rc 1 (11:54, stale): cargo:test deadline 3559768 ms on cold build, no test ran | exit 1 (03:00) | exit 1 (14:59) | running (15:04) | not run |
| verfolgen (gis) | `gis-11.txt` rc 1 (16:37): framework `os-flow-extensions`, `stdio-pdf-schema-4` | not run | `rtpanes-verfolgen.txt` rc 1 (15:02): cargo:test deadline 3599502 ms, no test ran | exit 1 (03:00) | exit 1 (14:59) | running (15:04) | not run (map tiles need GPU) |
| energie (energy) | `energy-26.txt` rc 0 (16:59), 0 error lines, 729 `Checking` lines (energy model crates; manifest not confirmed in log head) | not run | `rtpanes-energie.txt` rc 1 (10:45, stale): `energy-model-snapshot` `RetireOwned` bound | exit 1 (03:00) | exit 1 (14:59) | running (15:04) | not run |
| statik (fem3d) | 2d/engine only: `fem-4.txt` rc 0 (07:29); 3d: `fem-24.txt` rc 1 (17:53): `stdio-pdf` | not run (3d snapshot tests UNVERIFIED) | `rtpanes-statik.txt` rc 1 (12:49, stale): `stdio-pdf-schema-4` E0432 | exit 1 (03:00) | exit 1 (14:59) | running (15:04) | not run |
| framework | wasm gate subset: `plugjob-29.txt` rc 0 (07:05); newest red: `gis-11` and `rtpanes-aggregator` (`os-flow-extensions`), `plugnext-hub-2.txt` rc 1 (08:13), `gaterp-62.txt` rc 1 (05:59), `testport-sc-6` running (18:17) | green subsets: `coretest-mesh3.txt` rc 0, `coretest-engine1.txt` rc 0, `coretest-budgeted4.txt` rc 0, `pixtest-run-full5.txt` rc 0 (87/87), `compute-d.txt` rc 0 (6), `uicust-3.txt` rc 0 (5/5), `mutpolicy-retained.txt` rc 0, `geo2d-bool3`/`trace4`/`stroke2` rc 0; red: `testport-run-1.txt` rc 1 (12:55) | 7 of 8 pane smokes rc 1, koordinator running | `act14.status` exit 1 (03:00, graph-plan failed); `act15.status` exit 1 (14:59): `bazel:build failed`, `native:owner-command failed`, red crates `semio-framework-os-store-replay`, `stdio-pdf-schema-5`; `act16.log` running (18:17, "still running after 11313 s") | not run |

Demonstrator Bazel targets (not per pane): `tsdemo2-generate.rc` 0, `tsdemo2-canonical-architecture.rc` 0, `tsdemo2-tsc-2.rc` 0; `tsdemo2-test.rc` 1, `tsdemo2-test-runtime-catalog.rc` 1, `tsdemo2-r2-test.rc` 1 (2 script.ts tests failed), `tsdemo2-r2-test-runtime-catalog.rc` 0.

Browser: the only browser runs are `1101-tools/e2eready/run.sh` dry runs against port 1 (`e2eready-dry/run.rc` 1; generator@en and statik@en BOOT FAIL, `ERR_UNSAFE_PORT`). That proves tooling, not panes. No server run exists for any pane.

## Crates still red in the newest log per pane

- generator: `gen-hub14.txt` (18:02): `semio-s-artifact-stdio-pdf`
- koordinator: `cad-8.txt` (12:47): `semio-s-artifact-stdio-pdf-schema-4`. Newer `cad-t2` still running.
- aggregator: `rtpanes-aggregator.txt` (16:21): `semio-framework-os-flow-extensions`
- aussuchen: `rtpanes-aussuchen.txt` (13:32): `semio-framework-test-original-snapshot-owners`
- bearbeiten: none named. Newest log `rtpanes-bearbeiten.txt` (11:54) hit a deadline; newest rc-bearing process log `extfix-process-concrete-r4.txt` (13:15) rc 0.
- verfolgen: `gis-11.txt` (16:37): `semio-framework-os-flow-extensions`, `semio-s-artifact-stdio-pdf-schema-4`
- energie: none named in `energy-26.txt` (16:59, rc 0). Last smoke log `rtpanes-energie.txt` (10:45) red on `semio-s-artifact-energy-model-snapshot`.
- statik: `fem-24.txt` (17:53): `semio-s-artifact-stdio-pdf`
- framework: `semio-framework-os-flow-extensions` (aggregator and gis-11), `semio-framework-os-store-replay` and `stdio-pdf-schema-5` (act15.log, 14:59), `semio-framework-plugin-instance-typed-ops`, `time-travel-turns`, `tool-run-turns` (plugnext-hub-2, 08:13, stale)

## Crates with no owner

Method: every `could not compile` crate in the newest logs was matched against all `1101-result-*.md` files. Zero matches means no lane claims it. Corrections applied: `stdio-pdf-*` is stdio2's; `process3d-editor` is fixed by hubplug.

Unowned and red in a recent log:
- `semio-framework-os-flow-extensions` (rtpanes-aggregator 16:21, gis-11 16:37). Not named in any result. Framework crate, owner needed.
- `semio-framework-test-original-snapshot-owners` (rtpanes-aussuchen 13:32). Not named in any result.
- `semio-s-artifact-stdio-docx-base-model` (gis-9 13:07). Not named in any result.
- `semio-s-artifact-stdio-tiff-mutation-regressions` (gis-9 13:07). Not named in any result.
- `semio-framework-os-store-sync-native` (testport-run-1 12:55). Not named in any result.
- `semio-framework-plugin-reactor-patches` (rtpanes-verfolgen, 08:53, stale). The addendum assigns `reactor-jobs` to plugcore, not this crate.
- `semio-s-artifact-stdio-deflate-examples` (rtpanes-generator 10:15, stale). Energy calls it an example crate, unowned.
- `semio-s-artifact-energy-model-native-mutations-singletons` (stdio2-hub-1 12:21, stale). Energy by prefix; no result names it.
- `semio-s-fem-analyses` (replay-hub1 03:45, stale). Unowned.

Unowned and only in stale logs: `procedural-2d-mutations`, `procedural-2d-preview-eval`, `procedural-3d-geometry` (hub10 10:53), `stdio-semio-*-snapshot` family and `stdio-semio-*-model` / `animation-schema` (gis-7 09:45), `stdio-bmp-editor-2`, `stdio-wav-io-registry` (hub10 10:53, gis-7 09:45).

Owned by name: `stdio-pdf-*` (stdio2, per coordinator correction). `stdio-deflate-zlib` and `stdio-gltf-model-snapshot-root` (describe, `desc-w7.txt` rc 0, 12:30). `stdio-ifc-details-wire` (ifcsplit, by description).

## Lane claims contradicted by the logs

1. framework-green marker (07:05) rests on `plugjob-29` (`gate.sh` subset, rc 0). Later framework crates fail in `plugnext-hub-2` (08:13), `mutpolicy-fem2d` (08:16), `mutpolicy-wfc3d` (08:37), `rtpanes-aggregator` (16:21) and `gis-11` (16:37, `os-flow-extensions`). The marker is a gate-subset marker, not a whole-framework proof.
2. `actinfra`: the graph inference is green per the log text ("0 of 2815 projects failed"); `actinfra-graph.rc` = 1 is by design (collect mode publishes nothing and exits 1), so it is not a contradiction. `actinfra-showrepo.rc` = 0.
3. `1101-result-rtpanes.md` says the s2 pass ended with "all rc 101". The rc files say 1 (koordinator 08:57, verfolgen 08:53, aggregator 09:11, generator 10:15, energie 10:45). The result is stale.
4. `1101-result-e2eready.md` says "tsc rc 0". `e2eready-tsc.txt` has no rc marker (13:04). Claim unbacked. "Dry-load GREEN (run.sh --check 8/8)" has no `--check` log.
5. `1101-result-ifc4x3.md` (12:23) says no repo cargo check has run. `ifc4x3-2.txt` (12:59, rc 1) is an attempt that failed at argument parsing (`unexpected argument ' -p ...'`); no crate compiled. The claim holds, but the invocation is malformed.
6. `1101-result-tsdemo2.md` section 5 "Rerun after fixes" is empty. Logs show `tsdemo2-r2-test.rc` 1 and `tsdemo2-test-runtime-catalog.rc` 1.
7. Newly visible: `act15` names `stdio-pdf-schema-5` as red, while earlier logs name `stdio-pdf-schema-4`. Worth checking whether the crate was renamed or the lane's view is stale.
