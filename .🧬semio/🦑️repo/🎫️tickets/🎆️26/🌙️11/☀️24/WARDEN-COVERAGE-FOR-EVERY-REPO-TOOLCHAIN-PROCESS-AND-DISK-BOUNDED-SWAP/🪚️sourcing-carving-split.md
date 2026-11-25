# 🪚️ Sourcing carving split (2026-11-25)

## Landed (source only; see "Not measured")

1. Per-composition LTO (schema first). `PolicyRequest.composition` (optional package name) in the profile schema; `PolicyProtocol.leanCompositions` lists compositions built with the new `prototype-component-lean` row (`lto: off`, otherwise equal to `prototype-component`); every other component build keeps `prototype-component` (thin), GIS included. Listed: sourcing hub, beams, slabs, windows components (the hub draw component was added by a peer). The ingress derives the composition from the single `-p/--package` of the cargo invocation (`selectedCompositionV1`); several or no `-p` keep thin. Fixture rows: two new `rendering` rows (sourcing lean = off, GIS thin). Tests: profile 11 + consumption 26 pass.
2. Hub fleet split: new crates `semio-hub-sourcing-editor-app` / `-viewer-app` (`🧩️compositions/🪵️sourcing/🗃️apps/...`, the flow pattern), `SourcingApps` now routes to `SourcingEditorApps` / `SourcingViewerApps` with explicit `From` impls.
3. Curation tests: lib unit tests moved to one integration target `curation_unit` (`🧪️tests/🧪️unit-all`), the harness is a module file (`✏️editor/🧪️tests/🧰️harness`), the 857 line file split into `🧬️app` and `🌉️actions`; `[lib] test = false`. 108 tests passed (17+20+11+33+27) in the 5-target intermediate layout; the final one-target layout was not re-run to the end.
4. Admission carving list: added `curation_unit|host` 6687/7100, slabs 4196/4400, windows 4116/4300, `semio_plugin_tool_machine` 1600/1700, `semio_plugin_scrub_host` 1578/1700; fixture command of the unit-ceiling corpus moved to `semio_s_artifact_stdio_semio_brep_member`. Law test 11 pass. Entries for hub final / hub / curation lib are NOT removed (see below).

## Measured findings

- `semio_s_artifact_sourcing_curation` lib is already 0.45-0.8 GB (earlier carve by a peer); the 3.6-8.8 GB unit was its `--test` executable.
- Test executables are link dominated: an empty lib test unit 5254 MB / 518 s; split targets editor_app 5878, io 6023, actions 6099, windows 6228, commands 6785 MB (each 335-551 s); harness lib 3112 MB. Splitting by domain therefore made it worse (5 links), so it was folded back into one target (6687 MB, killed by the warden as unlisted, now listed). The real fix is link configuration (debuginfo / linker), not source carving.
- Hub component without LTO: 2523 MB final (earlier measurement), under 4096.

## Not measured

The hub fleet split and the lean hub/extension finals were never built to the end: three admitted attempts of `semio-hub-sourcing` were cancelled by the warden in unrelated units ("suspended for too long", host pressure). Hence `semio_hub_sourcing`, `semio_hub_sourcing_component` and `semio_s_artifact_sourcing_curation` entries remain; remove them after a measured run. A run of `curation_unit` (cur-tests5) was still compiling when this report was written. The three older integration targets (host_configuration, pane_smoke, persisted_receiving) are link units of the same class and unmeasured/unlisted.

## Changed files
See the reply.
