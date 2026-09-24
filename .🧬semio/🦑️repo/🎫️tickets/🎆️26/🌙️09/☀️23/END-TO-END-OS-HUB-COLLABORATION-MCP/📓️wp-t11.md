# WP-T11: Editor Catalogs, dwg/jpg/tiff/txt Oracles, Semio Bridges, Inventory Drift, Lib-Test Debt

Slice: T11 (session 10). Captures: `.tmp-ticket/wp-t11/generated/`. Inputs: `.tmp-ticket/wp-t11/*`. Private cargo: `.tmp-ticket/wp-t11/target`.
Inherits: T5 §4/§5 (`📓️wp-t5.md`), T8 §5. Freeze: stdio/gis product source, kernel pack/store, framework plugin crate. Outcome fields belong to T10.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 0. Baseline contract | **667 rows**: outcome-mismatch 254, variant-mismatch 119, runtime-only 81, manifest-only 55, unregistered-vocabulary 48, production-fixture-dependency 31, justified-reference-gap 23, mutation-without-fixture 15, missing-external-oracle 13, no-oracle-covers-mutation 5, … | `contract-0.txt`, `breaches-0.json`, `inventory-rows-0.txt` |
| 1. 48 editor-layer vocabularies | **9 resolved at root, 39 designed + platform landed; content waits on the freeze and T10** (§1) | `editor-survey-0.json`, `editor-paths-0.json` |
| 2. dwgread oracle (4 dwg rows) | **done** (§2): `libredwg-dwg-preamble-cli` (third-party-cli, `executables: [dwgread]`) is registered for both dwg capabilities and the decisions are removed. The oracle role writes every document and requires `dwgread`'s reading to agree. Parity run 2: the ac1024 and ac1018 oracle roles pass 7/7 each. Where the command is missing, the platform records `oracle-unavailable`. `native:libredwg 0.13.3` is added to `🔒️dependencies.json` | `stdio-parity-2.txt` |
| 3. jpg, tiff, txt cases | **oracle roles green; the jpg/tiff baseline subject hosts are blocked by the freeze** (§3, §3c). Oracle roles: jpg baseline 20/20, tiff baseline 18/18. Round trips 1/1 each. The jpg document case went to 42/42 with parity 21/21 after the fixture URI and host import fixes. Stdio oracle crate lib tests: **393 pass / 0 fail** (was 341/48). txt: the oracle reads through the registered `bstr` reader; its subject host carries the kernel-trait debt (§3c) | `parity-*.txt`, `stdio-oracle-test-9.txt` |
| 4. semio mesh/brep bridges + inventories | **root cause fixed** (§4): the two subset bridges hand-mirrored the module tree (520/364 errors); removed, so both subsets resolve to the plugin-level stdio bridge, which lists `SemioMeshMutation`/`SemioBrepMutation` and builds (20 s). Inventories: pending the inventory run | `stdio-bridge-build-1.txt` |
| 5. gltf 119 / runtime-only 81 / manifest-only 55 | **in progress** (§5): gltf 119 fixed; sibling-subset ownership (las, bcf, avi, dxf, obj: 33 runtime-only + 33 manifest-only) fixed; stale `no-mutation` kind (9) removed; tiff baseline + dwg ac1018 bridge fixes landed, need the stdio bridge rebuild; pdf 1.7 (44), binary `splice` (2) open | |
| 6. 18 lib-test failures | **flow, deflate, html, mp4, pptx, zip, dwg: all green now. norm-en1994: fixed (test compared `LocalizedLabel` with `&str`). raster: 1 concurrency flake in a peer's saturation law. remodeling: 8 → 5 wall-clock 8 ms laws, 2 of them fail even serially** (§6) | `lib-*.txt` |
| 7. Contract rerun + delta | pending | |

## 5. Inventory drift (declare or remove, by root cause)

| Rows | Root cause | Fix |
|---|---|---|
| gltf 119 variant-mismatch | manifest `productionDispatch.variant` named the payload type (`GltfRequireExtensionPayload`) | set to the aggregate variant the runtime inventory measured (`gltf-variants.py`, only that field; 119/119) |
| las 6/6, bcf 5/5, avi 6/6, dxf 15/15, obj 2/2 (runtime-only / manifest-only pairs) | leaves live in the subset that dispatches them (header/markup/hdrl/geometry), the kinds are semantically owned by a sibling subset whose manifest measures no leaves | the gif 89a pattern (T5 §6): the dispatching subset's manifest declares each row with the schema's `subset` override; sibling `mutationManifests` removed; sibling catalogs, cases, fixtures unchanged; `LasMutation::X` variant spellings reduced to the aggregate variant; payloadSchema re-rooted (`adopt-sibling-rows.py`; every path checked to resolve) |
| `no-mutation` manifest-only ×9 (gif 87a, gif 89a base, bcf, avi, dxf, docx, obj, semio cad, semio document) | `NoMutation` was dropped from every aggregate (26/08/29); the manifests and catalogs kept it as a kind. It is the identity baseline, not a mutation | removed from manifest + catalog kinds; its fixtures became plain identity fixtures (no `mutation`/`outcome`); the outline rows became plain `@id-no-mutation-baseline-mutate/-inverse` scenarios (`baseline-scenarios.py`); adapters register those ids on the same handlers (`baseline-adapters.py` + hand edits for gif 89a and the two semio Rust/Python adapters); avi and docx subjects gained the `no-mutation` → `SetSnapshot(base)` arm they never had (their rows were unregistered before) |
| tiff baseline manifest-only 8 | `TiffBaselineMutation` is public and dispatched by the baseline editor, but the bridge generator's aggregate regex rejected comment lines between the derive and the enum | regex fixed in `wp-t5/bridges.py`; the aggregate row added by hand to `🗄️stdio/🏭️bridge/🦀️.rs` (no regeneration: T10 is patching every bridge) |
| dwg ac1018 manifest-only 2 | ac1018's vocabulary is a glob re-export of ac1024's `DwgMutation`; the bridge measured ac1018's own (dead, duplicated) leaf JSONs | bridge coordinate for ac1018 measures ac1024's leaves; generator gained `reexported_region`. Frozen follow-up: delete ac1018's duplicate leaf directories |
| binary raw 1/1 | leaf `replace-byte-range` vs wire/catalog/feature `splice` (`#[value(rename = "splice")]`) | frozen (stdio product source): one name, after the freeze |
| pdf 1.7 base runtime-only 44 | 60 leaves dispatched, 16 declared | open |
| gisterrain 3 | T10's owner debt (gis frozen) | T10 |

## 1. Editor-layer vocabularies (48)

- **9 were not vocabularies.**
  - 8 were empty directory trees with no files, not tracked by git: layout presence and config, writer presence, sequence presence and config, wires presence, jack transient and config. They were removed.
  - The 9th, gismap `🗺️map/🎚️config/🧫️fixtures/🧬️mutations`, is a fixture tree that mirrors the name. The gate now counts only a schema's mutations facet (`🧬️schema/🧬️mutations`, taxonomy `schemaScopeOwnerLevels.facetDirName`).
- **39 are real and dispatched** by their editors through the plugin's config, presence and transient stores. So the schema-first answer is "exposed, measured", not "editor-internal".
  - All 39 aggregates and snapshots resolve to public paths (`editor-paths-0.json`) except trinity's rewriting/jack window vocabularies.
  - Rewriting also uses the non-canonical `🪟️window` directory.
- **Ownership (coordinator-approved design)**:
  - An editor vocabulary cannot share the artifact's document coordinate. T10 confirmed this on gisterrain and deleted that manifest. The gate `capability-without-manifest` still requires a manifest per catalog capability.
  - Landed now (TS + schema only): `MutationManifest.surface` and `RuntimeMutationInventory.surface`.
    - `surfaceProblem` validates the value as the owner's own path below its subset, spelled with the taxonomy's `subsetSurfaceDirs`, `modesDirName`/`windowsDirName` and the lanes `surfaceChildDirs ∩ modeChildDirs ∩ windowChildDirs`.
    - The inventory cache key, the coordinates and duplicate-owner keys carry `#<surface>`.
    - `test inventory` passes the surface as a 5th bridge argument and refuses an inventory that does not echo it.
    - Typecheck: 0 errors.
- **Order, agreed with T10**:
  1. T10 lands the outcome switch (after the freeze).
  2. T11 patches every bridge: list editor aggregates, answer a surface by owner prefix, keep the document answer free of editor/viewer owners.
  3. T11 authors the 39 manifests (outcomes from `wp-t10/leaf-outcomes.json`), catalogs, surveyed decisions, per-kind before/after fixtures from the implementation, and one outline-registered case per vocabulary.
  4. The case adapters call `store::test_support::mutation_report_json::<M, S>`. It lands in os-kernel after the freeze (coordinator-approved), with LIFO inverse replay, a law test, and the 4 non-gis hand copies deleted.

## 2. dwg: LibreDWG `dwgread` as a CLI oracle

- **Platform** (schema-first):
  - `OracleRegistryEntry` gains `executables` (the commands a `third-party-cli` runs) and `hostImplementation` (the adapter language that drives a `native` reference).
  - `oracleDecision` maps a native entry to that adapter and looks every executable up with `Bun.which`.
  - When one is missing, `runPhases` records `oracle-unavailable <case> (<oracle> needs <cmd> on PATH)`, skips that case's oracle role and parity, and counts the case in `RunSummary.oracleUnavailable` and the summary line. The skip is visible, never a pass. TS typecheck of the changed files: 0 errors.
- **Reader** (`🔟ac1024/…/🔮️oracles/🦀️.rs`):
  - `dwgread()` runs `dwgread -v3` and reads its file-header trace: the version code, `maint_version` and `codepage`, or `dwg too small: N bytes` for a preamble-only document.
  - `dwgread_agrees()` writes a document and requires that reading to match the projection.
  - Both adapters call it for mutate, inverse and identity.
  - Measured by hand on the real drawing (AC1024/2/30), AC1032/7/29 and a 22-byte stub.
- **Registry**:
  - `libredwg-dwg-preamble-cli` (GPL-3.0-or-later, run as a separate process and never linked) covers `dwg-ac1024-mutate` and `dwg-ac1018-mutate`.
  - Both no-oracle decisions are deleted, and the manifests require `third-party-cli`.
  - Features are retagged and their prose rewritten.
- **Provisioning**: macOS via Homebrew `libredwg`. Ubuntu 24.04 (devcontainer) and Windows have no package (packages.ubuntu.com shows none for noble), so on those platforms the case is recorded as `oracle-unavailable`.

## 3. txt: the registered reader now runs

- `bstr_split` (`🔤️txt/…/🔮️oracles/🦀️.rs`): the document is read with `bstr::ByteSlice::lines_with_terminator`, so line boundaries, LF/CRLF and the final terminator are bstr's reading. A CRLF document that also carries a bare LF is refused: this format reads that LF as line content, bstr as a boundary.
- `oracle_apply_mutation`/`oracle_inverse_spec` and the case's round-trip oracle read through it. The `@id-spec-vector` rows pin the format's own split rule, so they stay on the hand-written re-derivation.
- `bstr` 1.13 added to the stdio test-oracle crate (`oracles` feature). The registry entry gained `exact-bytes-v1` (the case compares bytes). The `txt-utf-8-line-structure` decision is deleted and the feature is tagged `@oracle-bstr-txt-utf-8-mutate-reader`. Stale "no-oracle" prose was rewritten in the feature, the adapter and the unit tests.
- **Found on the way:** the terminal-log fixture `🔤️.txt` has had no CRLF since it was committed, because `* text=auto eol=lf` normalized the two captured CRLFs. The test that asserted them failed. `.gitattributes` now marks `**/🧫️fixtures/** -text` (no EOL conversion for evidence). The test was rewritten to what the committed file proves (bstr and the rule agree line for line).

## 3b. tiff and jpg baseline: registered readers wired into the cases

- **tiff baseline**:
  - The new oracle module `🖼️tiff/…/🧱️baseline/🔮️oracles/🦀️.rs` reads IFD 0's five Baseline axes with `tiff` 0.11 (`find_tag`, `read_image`, IFD walk).
  - It applies each kind as TIFF 6.0 defines the field and reads the verdict off the spec's tables. It never touches this repository's codec.
  - The case registers oracle handlers and is tagged `@oracle-tiff-tiff-6-0-baseline-mutate-reader` (profile `ordered-json-v1` added to the entry).
- **jpg baseline**:
  - The new oracle module reads the SOFn code, component sampling, DHT class/id and DAC with `djpeg -v -v`, and the precision with `rdjpgcom -verbose`. It applies T.81 semantics and tables.
  - The Pillow entry is replaced by `libjpeg-jpg-jfif-1-01-baseline-marker-cli`: Pillow's JPEG plugin skips DHT and DAC, so it could never judge those kinds.
  - The devcontainer now installs `libjpeg-turbo-progs`, so this is zero-touch there.
- **Round trips**:
  - For both subsets the byte round trip is a claim about this repository's encoder, which no third party can reproduce.
  - It moved into `🔁️round-trip-{tiff-6-0,jpg-jfif-1-01}-baseline` cases, each with an honest `*-round-trip-normalization` decision on its own capability.
  - The old contradictory decisions are gone.
- **Unit tests**: 4/4 new (real scan read inside the class; every kind moves its axis and raises the table's code).

## 6a. stdio test-oracle crate lib tests: 341 pass / 48 fail → 391 / 0 (test-6), plus 4 new baseline-oracle tests

Root causes fixed:
- **Moved fixtures**, read through `CARGO_MANIFEST_DIR`-relative or absolute paths that no longer existed: bachelor-thesis PDF ×32, reuse-marketplaces workbook ×3, the bus-shelter R12 DXF ×2. They now use `include_bytes!` relative to the test file.
- **pdf 1.4 vector paths**: `🧫️fixtures/🧬️mutations/<m>/🔄️round-trips-the-concrete-inverse`.
- **`crate::standards` paths** in pdf a/x and step tests: now crate-rooted `artifacts::…` paths.
- **14 oracle modules mounted as `any` for non-`any` subsets** (avi hdrl, bcf markup, docx/gif/json/pdf ×2/svg/xml base, dxf/las header, jpg/tiff document, obj geometry): renamed to their subset (`oracle-module-names.py`). Every adapter import follows, which also fixed 7 adapters that imported `…::base`/`…::document` and never compiled.
- **`no-mutation` still in 9 oracle `KINDS`** (dwg, pptx, mp3, step cc1–6) after the catalogs dropped it: removed, and the count assertions updated.
- **note's dxf smoke test** read the projection key `kind`; the projection writes `entityKind`.
- **txt terminal log**: see §3.

## 4. semio mesh/brep bridges

- Both were subset-level bridges that re-declared the production module tree with `#[path]` mounts. Every refactor of the tree broke them.
- The plugin-level bridge (`🗄️stdio/🏭️bridge`) already links `semio-s-artifact-stdio-semio` and lists both aggregates with their coordinates. The inventory lookup takes the nearest bridge at the owner or an ancestor, so deleting the two subset bridges (archived in `generated/semio-subset-bridges-removed.tar`) makes both subsets use it.
- Same pattern, still compiling, not touched: `✉️base/🏭️bridge` and step `6️⃣cc6/🏭️bridge`.

## 6b. Registration by Scenario Outline (coordinator directive)

- **Root cause**: adapters registered each expanded row id (`mutate-<kind>`) from a hand-kept `KINDS` list. The list drifted from the catalogs (`no-mutation` stayed in 8 KINDS after the catalogs dropped it), and 50 stdio features' `no-mutation-baseline-*` scenarios had no registration at all (13 of them reached it through `law::scenario_id`).
- **Protocol** (all five hosts: Rust, Python, Go, .NET, TypeScript):
  - a handler registered under a Scenario Outline's base id (`mutate`) serves every row the feature expands (the plan's existing `outlineOf`); exact ids win;
  - a new `row()` accessor returns the row id;
  - TS `validateRegistration` accepts outline ids.
  - Parity case `🖥️host-protocol-parity` gained an outline scenario: **20/20 subject, parity 40/40** across the five hosts.
- **Adapters** (`outline-registration.py`, `outline-registration-py.py`, `unfactory-handlers.py`, `drop-dead-kinds.py`, `registration-docs.py`): every stdio Rust, Python and TypeScript mutation adapter registers `mutate`/`inverse`/`spec-vector` plus the exact baseline ids its feature declares. Factories that took `kind` read it from the doc string or `ctx.row()`. Every KINDS mirror is deleted, as are `law::scenario_id` and the keep-alive statements. Python semio KINDS stay where they are the wire-tag vocabulary.
- ~150 adapters outside stdio still register per kind. They are not broken, and the same scripts apply.

## 3c. jpg/tiff/txt runtime, and the stdio case-host compile debt

The runs are per case: `parity exhaustive --case <id>`, captured as `parity-<case>.txt`.

| Case | Oracle role | Subject host |
|---|---|---|
| `🛡️mutate-jpg-jfif-1-01-baseline` | 20/20 | E0451: every leaf payload field is `pub(crate)` (frozen product) |
| `🧱️mutate-tiff-6-0-baseline` | 18/18 | E0451, same cause |
| `🔁️round-trip-jpg-jfif-1-01-baseline` | 1/1 | — |
| `🔁️round-trip-tiff-6-0-baseline` | 1/1 | — |
| `📸️mutate-jpg-jfif-1-01` (document) | 42/42, **parity 21/21** | compiles |

Fixes on the way:
- The jpg `SCAN`/`INPUT` URIs named `🧪️abbau…`, but the fixture dir and the features use `🏘️abbau…`. Fixed in the three jpg adapters.
- `use crate::JpgSnapshot` became the artifact crate.
- The tiff baseline `remove_*` modules are now imported.

Stdio parity run 2 had **54 case hosts** that did not compile. All of them are committed adapter debt, not T11 edits:
- **~30 hosts: `use crate::X`.** An adapter compiles as its own host crate, so `crate::` names the host. Some sweep rewrote the artifact-crate paths to `crate::`. `adapter-crate-paths.py` rewrote them back to the crate that exports `X` at its root. The script only touches dirs that carry a `🥒️.feature` (the `#[path]`-mounted `mutation-regressions` modules really are in-crate) and only names the artifact root exports and the adapter does not define. That is 34 lines in 30 adapters. The rerun of those 30 cases is in `rerun-summary.txt`.
- **13 hosts need frozen product API.** Either leaf payload fields are `pub(crate)` (366 fields across 211 stdio leaf files), or the adapter needs the kernel `Mutation` trait (`protocol::Mutation::inverse`), which the host cannot name: its only deps are the artifact crate and the oracle crate. Root fix, post-freeze: leaf payload fields become `pub` (the schema is the public contract), and each artifact crate exposes `inverse_<x>_mutation` the way jpg/tiff baseline already do. txt is in this class.
- **The rest are singles:**
  - svg and the semio table/value hosts use a sibling artifact crate (`xml`, `csv`, `json`) that the host generator does not add.
  - docx/gltf/obj/binary adapters use stale `subsets::any` module paths.
  - pdf `create-minimal-pdf` builds a `PdfPage { text }` field that no longer exists.
  - `mutate-gltf-2-0-material`: argument drift.

## 6. T5's 18 lib-test failures

Each crate was rerun with `cargo test -p <crate> --lib`, captured as `lib-<crate>.txt`.

| Crate | T5 | Now | Cause / fix |
|---|---|---|---|
| flow-flow | 1 fail | 255/0 | fixed upstream since T5 |
| stdio-deflate | 2 | 50/0 | fixed upstream |
| stdio-html | 1 | 42/0 | fixed by T5 |
| stdio-mp4 | 4 | 46/0 | fixed upstream |
| stdio-pptx | 1 | 76/0 | fixed upstream |
| stdio-zip | 1 | 55/0 | fixed upstream |
| stdio-dwg | compile (`include_bytes!` temp dwg) | 70/0 | fixed upstream |
| norm-en1994 | compile | **226/0** | T11: `labels_are_human_readable` compared `LocalizedLabel` with `&str`; it now compares with `LocalizedLabel::native("Change span to 12", "Spannweite auf 12 ändern")`, both locales |
| raster-raster | compile | 227/1 | see below |
| remodel-remodeling | 8 | 1296/5 | see below |

**raster.** `raster_standalone_control_max_plus_one_…` passes alone (2/2 runs) and fails in the full binary. The law saturates the process-wide standalone control pool and then asserts that the refused probe still holds no credit after `close_step`. Sibling tests that construct retirements without `RASTER_STANDALONE_RETIREMENT_TEST_LOCK` return credits in between, so `close_step` legitimately re-acquires one. The root fix belongs to the raster ticket (its 09-22 note already measured this race), not to T11: either every retirement-constructing test takes the lock, or the pool becomes injectable per test.

**remodeling.** All 5 failures are wall-clock `< 8 ms` step laws in a debug build: PNG scanline ×2, texture bake, TSDF envelope rejection, and a feature-detect microstep (14.4 ms). Load average was 14–22. Serially, the texture, TSDF and feature laws pass, but both PNG laws still fail: they time the unoptimized third-party `png` decoder. The root fix belongs to the remodel owner: bound these steps by work units (fuel or scanline counts, as the fem runtime laws do), not by debug-build wall time.

## Dependencies

`bun ./📜️script.ts verify dependencies`:
- None of the 9 NEW entries it lists are T11's (they are @types/*, picomatch, pngjs, hayro, naga).
- T11 edited `🔒️dependencies.json` by hand, changing only its own rows:
  - `native:libredwg 0.13.3`: new (`libredwg-dwg-preamble-cli`, capabilities dwg-ac1018/ac1024-mutate).
  - `native:libjpeg-turbo`: gains `libjpeg-jpg-jfif-1-01-baseline-marker-cli` / `jpg-jfif-1-01-baseline-mutate`.
  - `python:Pillow`: loses the removed `pillow-jpg-jfif-1-01-baseline-mutate-reader`.
- `bstr`/`tiff` were already recorded. `write-baseline` was **not** run, because it would also approve the 9 foreign entries.
- `literal-external` is at 243 / 19 oracle conflicts repo-wide. The `rust:tiff` conflict predates T11: the tiff document generator codec declares it.

## Processes (pids)

- contract-0: 5536 (exited)
- stdio bridge build: 24099 (exited, 20 s)
- first stdio verification chain: 46266/61616 (stopped by me after the oracle phase so the oracle-crate fixes could land; oracle phase: 124 cases, 1904 executed, 1723 pass / 61 fail / 120 errored, 10 host builds failed, all of them the stale imports fixed in §6a)
- stdio parity run 2: 74122

## Files changed (T11)
