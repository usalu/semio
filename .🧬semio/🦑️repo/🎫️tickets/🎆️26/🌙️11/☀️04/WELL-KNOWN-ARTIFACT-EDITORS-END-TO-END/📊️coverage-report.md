# 📊️ Coverage Report: Well-Known Artifact Editors End to End

Final delivery skeleton. Owner: T20 (haiku-documentation). Status: **in progress; no editor has a passing test or an end-to-end run yet.** Every section is filled from what exists in the ticket folder on 2026-11-04. Unknowns are marked **[unknown]**, and unverified claims are marked **[U]**. The coordinator will ask for a refresh later; each section names its source file so the refresh can be checked against it.

## 1. Scope and terminology

- **Fleet goal.** Every well-known artifact editor (stdio format artifacts under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*`) works end to end in the real OS app `🧰️framework/🛍️products/💻️os`.
- **Artifact.** One stdio artifact definition (`📜️artifact-definition.json`). **38** artifacts. The 39th directory, `📇️inventory`, is a library and is excluded.
- **Editor / viewer surface.** A per-subset `#editor` or `#viewer` app, published by a hub family plugin. **88 editor/viewer pairs** across 36 shipping artifacts; `idf` opens in the energy model editor; `idml` has a new surface crate but is not yet in the roster.
- **Dialect coordinate.** `<kind>@<standard>/<subset>#<role>`, for example `s.stdio.json@rfc8259/*`. The `*` subset is the owner/base subset.
- **NF (natural file).** The editor's open and save of a real file (`natural_file_codec`).
- **Native receipt.** The durable pack receipt in the store (`stdio.native.<x>.v1`).
- **Edit path.** `main-window` (the editable window) or `Details` (the generic snapshot-edit pane; the only edit path for read-only main windows).
- **Status values.** `gap`, `in progress (uncompiled)`, `compiled`, `contract-verified`, `tests green`, `e2e verified`. `contract-verified` means the offline acceptance contract (bun, `🧪️contract`) passes for the row; it is not editor e2e and not an editor unit test. Only `tests green` and `e2e verified` require a log with a passing result under `🗑️generated/`; passing results cited from logs that will be deleted are transcribed in `✅️validation.md` (E1 to E6) so the citation survives.
- **Heavy job.** Any cargo, vite, browser or full build. These go through the shared capacity gate (`repo capacity`), one at a time.
- **Owners.** T-numbers are the fleet workers from `🗂️board.md`. Names such as sonnet-host (T02) and batch C (T09) are the coordinator's assignments of 07:20.

## 2. Coverage matrix

Full table: `📊️coverage-matrix.md` (38 rows; cross-cutting blockers and evidence are in the same file).

| Status | Rows | Notes |
| --- | --- | --- |
| `gap` | 0 | (semio moved to `contract-verified`; its open gaps are listed in its row) |
| `in progress (uncompiled)` | 1 | idml: fails 2 of 2 contract tests (`✅️validation.md` E1) |
| `contract-verified` | 37 | offline contract passes (192 of 194 contract tests; idml is the only artifact failing) |
| `compiled` | 0 | no log shows a successful compile |
| `tests green` | 0 | no log shows a passing editor unit test |
| `e2e verified` | 0 | no browser journey has run |

| Edit path | Rows | Notes |
| --- | --- | --- |
| `main-window` | 22 | includes bmp and tiff (batch C) |
| `Details` | 16 | 15 read-only main windows (las, gif, mp4, svg, mp3, avi, jpg, stl, ply, dxf, dwg, obj, gltf, step, ifc 4) and idf through the energy model editor |
| `none` | 0 | |

Natural-file coverage (from the matrix): NF written in this ticket for dxf, dwg (ac1018 and ac1024), pdf 1.7 subsets and 1.4 base/a/x, zip, deflate, binary, gif (87a and 89a), tiff, epw, bcf, ifc (4, 2x3, cobie, cv20, sav, 4x3), energy model (idf), idml, png/jpg kept, wav kept. All uncompiled.

**Coverage counts are per row and are not test coverage.** No row has runtime evidence.

## 3. Fixes and shared contract changes

Grouped. Every item is uncompiled unless noted; "[host tests]" marks the only passing results, which are reported in `🖥️host.md` and have no log under `🗑️generated/`.

### 3.1 Registry and resolution (T01, addenda 2 to 4 in `📇️registry.md`)

- Resolver contract (frozen): explicit key, then longest extension suffix (case-insensitive), then exact MIME; a disagreeing MIME never wins; no fallback format. Rust `🚪️io/🧭️format-resolution` and the TypeScript twin share the fixture `🚪️io/🧪️tests/🧫️fixtures/🧭️format-resolution/🔣️.json`.
- Registry no longer sorts `extensions`; declared order is export precedence. html and tsv publish a `FormatDescriptor` (previously none). Aliases `.jpeg .tif .htm .markdown .stp` claimed.
- Owner rule: `representations[].owner` marks the most complete revision. Declared owners: pdf 1.7, gif 89a, dwg ac1024, ifc 4.
- Content sniff: `representations[].sniff` for gif, dwg, ifc (pdf has none; every `%PDF-1.x` goes to 1.7).
- Generated shell claims: `stdio-format-claims.json` (37 formats) from node; parity test `the_generated_shell_format_claims_equal_the_published_formats` not run.
- Accessible-name enforcement at schema level: assigned to T01, not started [U].

### 3.2 Host and opening (T02, `🖥️host.md`; T01 wiring, addendum 2)

- F1: `os.open-artifact` refuses with the localized `open-rejected` notice on any failure (React and wgpu D1 and D3).
- F3: the session publishes only after `openDocument` succeeds (`releasePreparedArtifactOpening`).
- D2: no viewer-to-editor fallback in wgpu.
- D4: pins. React `setDefaultApp`/`openArtifact` wire is implemented; wgpu reads `default_app_pins` through `pinned_app_for`. Pins are not yet persisted from wgpu to the OS config lane.
- E6: Open file routes a picked file through `resolvePickedFileFormatV1` and `routePickedFileV1` (en/de refusals); `formatCatalog` port is supplied by the React and GIS entries via `S_FORMAT_CATALOG_V1`.
- Dashboard target fix: `playground_target` now emits `-prototype` (was `-dev`); hub secure suite passes `serve s react prototype`. Not run.
- [host tests] Vitest: `🚪️replay-open-artifact` and `🛰️framework-plugin-runtime`, 2 files, 29 tests passed; plus `📂️picked-file-opening`, 3 files, 55 tests passed; after phase 3, reported as 7 files, 94 tests (`🖥️host.md` §7). Re-run with a log on the current tree: 8 of 9 files pass (96 tests); one suite fails to transform (owner T02), so the 94 count is not reproduced (`✅️validation.md` E3). Run with the temporary `vitest.host.config.mjs`.

### 3.3 Persistence (T03, `💾️persistence.md`; `🔍️data-audit.md` T03 verdict)

- F1 torn-tail truncation and fsync on the folder event log; fixture `folder-event-log-torn-tail`.
- F2 write failures no longer swallowed; `unsaved` archive held and retried with backoff.
- F3 `persisted` true only after the folder or hub acknowledges the newest archive.
- F4 watcher watches `.semio/events.semio`.
- F5 atomic `FolderTextStorage` writes (own `atomic_replace`).
- F6 setup read error is a recoverable `folderReadFailed` conflict.
- F7 TypeScript worker `persistFolderArchive`: one write in flight, submission order, mutations leave `pendingMutations` only when covered.
- Fixtures: `folder-write-acknowledgment`, `folder-event-log-torn-tail`, `folder-write-sequencing` (Rust corpus and vitest).
- Open critical items (C1 to C11) are in section 8.

### 3.4 Formats (T04, `🔄️formats.md`)

- Natural-file contract: one route per editor (`natural_file_codec`, `encode_natural_file`, `decode_natural_file`), mounted as Open File and Save File actions.
- Export file stem from the document name, sanitized (Windows device names safe).
- Export MIME and extension from the format descriptor.
- Envelope registry keyed by version.
- `wrap_text` preserves BOM and leading whitespace; space import refuses to overwrite by id; file-space port writes only the chosen file.
- Status: uncompiled in the Rust crates; `t04_files.txt` in the generated folder records the file list [U as to test results].
- Before this ticket, `🎟️capability-audit.md` found 19 editors with a declared natural-file codec. The additions made in this ticket are listed per batch in section 3.5 and in the matrix NF column.

### 3.5 Per batch

| Batch | Worker | Changes (uncompiled unless noted) | Open |
| --- | --- | --- | --- |
| A (txt, md, json, html, xml) | T07 | no batch-A report in ticket; registry descriptor for html; json directory fixed (T19) | batch A report missing [unknown] |
| B (csv, tsv, xlsx, docx, pptx, idml) | T08 | idml surface crate `semio-s-artifact-stdio-idml-any` (`#editor`, `#viewer`, `.idml` natural codec); story content splice with quick-xml cross-check; csv/tsv/xlsx/docx/pptx natural codecs declared | not in hub roster; no native receipt |
| C (png, jpg, bmp, gif, tiff) | T09 | png ancillary chunk retention test; bmp native `stdio.native.bmp.v1` and paint window actions; gif 87a and 89a natural codecs and native receipts; tiff document and baseline natural codecs; paint-region window action | generators pending; palette mutations absent; `SetPixelRegion` unrouted |
| D (svg, dxf, dwg, pdf) | T10 | dxf natural codec and 2D preview (editor and viewer); dwg ac1018 and ac1024 natural codecs and typed-geometry preview; pdf 1.7 subsets (six) natural codecs with subset format kinds; pdf 1.4 base/a/x lossless-only open (refuses files it cannot reproduce) | dwg entity edits are whole-drawing `set-drawing`; layer and object kinds not started; svg element rename and typed transform not started |
| E (obj, stl, ply, gltf, las, step) | T11 | natural codecs in all six artifacts (base and cc1..cc6); shared transform gesture `/transform` (cross-checked against three.js r182 fixture); real geometry in obj, stl, ply, las main windows | gltf and step 3D windows still placeholders; step cc1..cc6 no transform; no obj editor package exists |
| F (ifc, bcf, idf, epw, energy) | T12 | ifc 4, 2x3 base, cobie, cv20, sav, 4x3 natural codecs; ifc 4x3 viewer and registration rows; energy model `.idf` natural codec; bcf natural codec and fixtures (minimal, truncated, missing-version, written by python); epw natural codec with independent CSV oracle (Python stdlib) | cobie, cv20, sav domain enums not wired; 4x3 manifests not regenerated; idf routed to energy model |
| G (mp3, wav, mp4, avi) | T13 | shared `media_transport_view` (mp3, mp4, avi); mp4 and avi window "Unsupported" fix; wav `edit-audio` and `cut-frames`; wav native codec `stdio.native.wav.v1` with placeholder hash; ffprobe cross-checks with hard fail when absent | mp3 frame trim (`patch-frames`) not built; wav hash placeholder must be refreshed; media host trim and cancel (A11) |
| H (zip, deflate, binary, semio) | T14 | zip unsafe-name guard (`..`, absolute, drive, backslash, NUL) and natural codec; deflate `.zz` natural codec and cross-check with `miniz_oxide`; binary `.bin` natural codec; deflate doc fix | native receipts for binary and semio; semio sniff dispatch and per-subset edit paths not started |
| Runtime (T05) | T05 | `repo-mcp:install` fixed (launcher link replaced atomically); MCP initialize answered with 55 tools; dashboard `-dev` target fixed | bun not on PATH; dev server not yet started |
| Validation (T06) | T06 | heavy-queue owner; no result files yet | `🗑️generated` logs show waits and failures (section 7) |
| Audits | T15, T16, T17, T18, T19 | inventory, capability, data, entry, registration, fixture manifest, Steam Deck audit | a11y owners set in matrix section "Cross-cutting blockers" |

Cross-batch shared contract: `stdio` contract `media_transport_view` (`📇️registry/🧬️contract/🪟️window-kit`), `mesh_scene` (`🪟️window-kit/🧊️mesh-scene`), and the snapshot-edit `/transform` gesture (`📐️transform`). The OS fixture `📄️natural-file-lifecycle` is shared by all batches (row count disputed; section 8).

## 4. Startup, build and test commands and URLs

Source: `📘️commands.md` (general, and "Running stdio crate tests"), `🛠️runtime.md`, `🚪️entry-audit.md` Follow-up. Cargo runs use `--store=shared --configuration=prototype` only. Each command below is **[U]** until it has a passing log in `🗑️generated/`.

**OS app (React `s` shell)**

```bash
cd /home/deck/semio
repo run @semio-tech/framework-os-dev:activate-s-react-prototype   # once; currently stuck in dependency sync (section 6)
repo run @semio-tech/framework-os-dev:dev-s-react-prototype        # Vite on http://127.0.0.1:6070/
# alternative, from the package directory:
cd "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
repo ./📜️script.ts serve s react prototype
repo ./📜️script.ts serve-hold --serve http://127.0.0.1:6070 --variant s
```

- Optional env: `S_DATA_DIR=<repo>/.🧬semio/🔗space/s-dev`, `S_LOCAL_ONLY=1`, `S_HUB_URL=http://127.0.0.1:8787`.
- Hub: `repo run os-hub:dev` (URL `http://127.0.0.1:8787/`, admin at `/admin`).
- wgpu: `playground:s renderer=wgpu-wasm` (6066) through the dashboard is documentation only; the CLI equivalent is `dev-s-wgpu-dev` [U].
- Storybook OS scope: `repo run workspace:dev-storybook-framework-os` (port 6010).
- MCP inspector: `repo run workspace:dev-mcp` (port 6274).

**Browser e2e (Playwright, Chromium headless)**

```bash
repo run workspace:deps-browsers
cd "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
repo ./📜️script.ts verify e2e                          # S_OS_PORT (6070) or S_STUDIO_URL
repo ./📜️script.ts verify catalog --out <ticket>/🗑️generated/catalog-smoke
repo ./📜️script.ts verify home --serve http://127.0.0.1:6070 --locale en
```

`verify` with no route runs the full heavy chain; do not run it outside the queue.

**Stdio plugin**

```bash
repo run workspace:stdio-quick
repo run @semio-tech/stdio-plugin:test
repo run @semio-tech/stdio-plugin:native-codec-projection   # regenerates native catalog (needed for bmp, gif, wav)
repo run @semio-tech/stdio-plugin:composition-prepare       # regenerates hub roster (needed for bmp, gif, wav, idml, ifc 4x3)
```

**Crate tests (per owner workspace)**

```bash
cd "<owner workspace from 📦️crate-names.md>"
repo cargo --store=shared --configuration=prototype -- test -p <package> --lib <filter>
```

Owner examples: zip editor from `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip`, obj root from `…/🗽️obj`. Bridge crates need `--manifest-path` (commands.md). Do not run `-p` from the repo root for artifact-workspace crates.

**Catalog generation**

- `repo run @semio-tech/plugin-registry:generate` (part of `repo run workspace:prepare`).
- `node "🧰️…/📇️registry/🧬️contract/🧭️format-claims/📜️generate.ts" <repo-root>` writes `stdio-format-claims.json`; project targets `generate-stdio-format-claims` and `check-stdio-format-claims` exist in `✏️s/🧑‍💻dev/📋️project.json` [U: not run through `repo`].
- `repo run workspace:components describe <plugin-id> --profile prototype` (heavy; needed before `os run`).

## 5. Fixtures and output evidence

- **Acceptance samples (current).** `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧫️fixtures/📇️samples.json` (checked in, schema `🧬️schema/📇️samples.json`) replaces the top-level manifest as the sample source. The top-level `🧫️fixture-manifest.json` is superseded and marked for deletion at close (coordinator decision); `build-fixture-manifest.py` is kept as an input. Description of the superseded manifest: schema `semio.stdio.fixture-manifest/v1`, 38 artifacts. Lacking valid fixture: idml. Lacking surface: idf, idml. Invalid fixtures: 54 files across 34 artifacts (truncated and wrong-magic, plus ifc truncated under a path outside the walker root). Artifacts still without invalid fixtures: idf, idml.
- **Language-agnostic fixtures shared by Rust and TypeScript.** Format resolution (`🧭️format-resolution/🔣️.json`, sniff cases included), folder write acknowledgment, folder event log torn tail, natural-open fixtures for bcf (`📂️natural-open/`), epw CSV oracle (`📖️natural-open/🔣️.json`), dxf and dwg preview fixtures, transform oracle (`📐️transform/🔣️.json`), wav cut-frames, zip unsafe names, story content (idml).
- **Natural-file lifecycle.** `🧰…/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json`. Row count reported as 33, 48, 51, 55 and 67 by different batches. Reconciliation owner: T06 (board). Count assertions were changed to set-based checks by T10.
- **Generated outputs.** Logs and reports in `🗑️generated/` (about 36 entries; deleted at close after transcription). Results that the report relies on are transcribed in `✅️validation.md` (E1 to E6) and the per-batch files. No editor unit test passes in any log. The passing logs are `[DEBUG]contract.txt` (contract, 192 of 194) and `t01-os-io-format-resolution.log` (registry law, 5 of 5).
- **Generated OS dev output.** `🧰…/🧑‍💻dev/🤖️generated/` contains `🗿️artifacts`, `🦀️catalog.json` and `🩺️diagnostics.json` (from earlier runs; freshness [unknown]).
- **Stray directory to delete.** `.🧬️semio` at the repo root (variation-selector typo created by a batch C command) [U: still present; batch C asked to delete it after the run].

## 6. Local configuration required

- **bun is not on PATH.** `repo` provisions its own bun at `~/.cache/semio/runtimes/bun/…`. Direct `bun` calls fail. Until T05 delivers bun on PATH, host vitest runs use the ticket config `vitest.host.config.mjs` with `HOST_TEST_ROOT` and `HOST_TEST_ENV` (jsdom for shell tests). Delete the temporary config when the project runner is in use.
- **Hub home e2e with `--hub`** needs `OS_HUB_PROBE_EMAIL` and `OS_HUB_PROBE_PASSWORD`; otherwise the run is reported `blocked`.
- **Shared store only.** `repo cargo --store=shared`. Two workers used isolated stores and forced framework rebuilds (board). Do not set `CARGO_TARGET_DIR` by hand.
- **Capacity gate.** Every cargo, vite and browser job waits for `repo capacity`. Expect long waits when load is above 20 (observed 19.9 to 31).
- **Tools present:** `ffprobe`, `qpdf`, `pdfinfo`, Python 3 standard library, `node` (24.x), `cargo` (rust toolchain pinned to `nightly-2026-07-20` in `rust-toolchain.toml`). **Absent:** `ezdxf` (dxf oracle uses the Rust `dxf` 0.6 crate instead); `jszip` oracle not run.
- **Playwright browsers** via `PLAYWRIGHT_BROWSERS_PATH` (repo cache `tools/ms-playwright`), set by `deps-browsers`.
- **Rust toolchain** installs on the first cargo call in the repo (README); no image-pinned toolchain.
- **Cargo.lock refresh** expected in the dxf, dwg, pdf, energy and hub workspaces (new path dependencies) [U: not yet performed; bcf log shows a refresh in progress].

## 7. Verification results

Only logs that contain results are cited as results. Logs with no result are listed as state.

**Passed (with a log or transcription).**
- Offline acceptance contract, `repo tools exec bun -- test …/🧪️contract/🟦️.ts`: **192 pass, 2 fail** of 194 (381 expect calls). Log `🗑️generated/[DEBUG]contract.txt`, transcribed in `✅️validation.md` E1. Failures: `B/idml` (2 tests). Exit code not recorded [U]. `contract-verified` for 37 rows; not editor e2e.
- Registry law, `semio-framework-os-io` `format_resolution`: **5 pass, 0 fail**, `EXIT 0`. Log `t01-os-io-format-resolution.log`, transcribed in `✅️validation.md` E2.
- Acceptance contract re-run, `contract-rerun.log`: **rc=1**, 192 pass, 2 fail (both idml, **owner batch B, T08**). E1 of `✅️validation.md`.
- Host vitest, `host-vitest.log` (`HOST_TEST_ROOT=/home/deck/semio HOST_TEST_ENV=jsdom`): **rc=1**. 8 files pass with **96 tests**; `🛰️framework-plugin-runtime` **fails to transform** (esbuild syntax error at `🔶️scope/🟦️.tsx:120:349`, **owner T02, sonnet-host**). The earlier 94 count cannot be reproduced while that error stands. E3.
- Registry vitest, `registry-vitest.log`: **rc=0**, 2 files, **4 tests**. E4.
- Media vitest, `media-vitest.log`: **rc=0**, **20 tests**; media React, `media-react-vitest.log`: **rc=0**, **10 tests** (owner batch G, T13). E7.

**Passed (reported, no log under `🗑️generated/`; transcribed in `🖥️host.md` and `✅️validation.md` E3, E4).**

**Failed to start (root workspace selection; result is a failure, not a pass; transcribed in `✅️validation.md` E5).**
- `🗑️generated/h-zip-editor-test.log`: `semio-s-artifact-stdio-zip-editor` did not match any packages (exit status 1).
- `🗑️generated/e-obj-editor-test.log`: `semio-s-artifact-stdio-obj` did not match any packages (exit status 1).
- `🗑️generated/e-semio-s-artifact-stdio-obj-editor::obj.log`: `semio-s-artifact-stdio-obj` did not match any packages (exit status 1).
- `🗑️generated/e-transform-test.log`: `semio-s-artifact-stdio-contract-editing` did not match any packages (exit status 1).

**Compile errors.** None in the compiled-error sense. `🗑️generated/d-dxf-editor-shared.log` shows about 300 `Compiling` lines and warnings only; the compile is still running.

**Still waiting or running (no result).**
- `🗑️generated/a-txt-editor-test.log` (compiling)
- `🗑️generated/d-dxf-editor-shared.log` (compiling)
- `🗑️generated/t01-os-io-format-resolution.log` (compiling; warnings)
- `🗑️generated/f-batch.log` (bcf, ifc, epw; waiting for capacity)
- `🗑️generated/h-zip-editor-test2.log` (waiting)
- `🗑️generated/e-semio-s-artifact-stdio-obj-editor::obj.log` (waiting)
- `🗑️generated/f-bcf-lib.log`, `f-bcf-natural.log`, `f-epw.log`, `f-ifc4-natural.log`, `f-ifc2x3-natural.log` (waiting)
- `🗑️generated/show-stdio-plugin.txt` (waiting for capacity)
- `🗑️generated/activate-s.log` (dev server activation running for more than 12 minutes in dependency sync; `🛠️runtime.md`)

**Cancelled.**
- `🗑️generated/d-dxf-editor-baseline.log` (SIGTERM)
- `🗑️generated/t13-wav-codec.log` (SIGTERM while waiting for capacity)

**Not started.** Browser journeys (`verify e2e`, `verify catalog`, `verify home`), `stdio-format-claims` parity test, `natural-file-lifecycle` test, every batch's targeted test run.

**Totals.** Contract-verified rows: **37** (offline contract, not e2e). Editor unit tests green: **0**. Editors e2e verified: **0**. Editors compiled: **0**.

## 8. Remaining blockers

Each blocker is reproducible from the command or file named. Owners follow the coordinator's assignment.

**8.1 Runs to queue (heavy; T06).**
1. Zip editor: `cd "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip" && repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-zip-editor --lib`.
2. Obj root: `cd "…/🗽️obj" && repo cargo … -- test -p semio-s-artifact-stdio-obj --lib`.
3. Transform tests in `semio-s-artifact-stdio-contract-editing`: owner directory to be taken from `📦️crate-names.md` before queueing.
4. Per batch, the test filters in each batch report (for example batch C: `-p semio-s-artifact-stdio-bmp-editor-2 --lib editor::bmp`).

**8.2 Generators (T06).**
5. `repo run @semio-tech/stdio-plugin:composition-prepare` and `repo run @semio-tech/stdio-plugin:native-codec-projection`: needed for bmp, gif, wav, binary, semio, idml and the ifc 4x3 registration.
6. `native-codec-projection refresh stdio.native.wav.v1`: replaces the **placeholder** `pack_schema_hash` (`0000…0001`) in `🔊️wav/📜️artifact-definition.json`, `📜️native-codec-factories.json` and `📜️native-catalog.json`. Must not ship as-is.
7. `node …/📜️generate.ts` and the Rust parity test `the_generated_shell_format_claims_equal_the_published_formats`.

**8.3 Save status and close (critical; owners T03 and T02).** C1 to C11 in `📊️coverage-matrix.md`, "Cross-cutting blockers". Highest priority: C5 (archive written under the wrong document id), C6 (corrupt archive overwritten), C3 (close drops unsaved work), C1 ("Saved" shown before acknowledgment; re-check needed).

**8.4 Accessibility and usability (High).** A1 to A8 (sonnet-host, T02; A4 batch E), A10 (batch C), A11 (batch G), A12 (batch D), A13 (T01), A14 (T19). Matrix section "Cross-cutting blockers".

**8.5 Editor gaps (uncompiled work plus missing features).**
- semio: sniff dispatch, native receipt, per-subset edit paths (about 15 read-only subsets).
- mp3: frame-range trim (`patch-frames`), a hand-authored multi-file change needing a compile lane.
- gltf and step: 3D windows still placeholders; brep tessellation missing.
- dwg: layer and object edit kinds; line-endpoint round-trip test decides if the writer needs extension.
- ifc: cobie, cv20, sav domain enums; 4x3 registration not regenerated.
- obj: no editor package; `set-vertex` not wired.
- idml: not in hub roster.
- pdf 1.4: lossless-only open; other 1.4 subsets refused by design.

**8.6 Environment.**
- bun not on PATH (section 6).
- Dev server: `activate-s-react-prototype` stuck in dependency sync (`🛠️runtime.md`; `🗑️generated/activate-s.log`).
- Capacity: queue load (section 9).
- ezdxf absent (Rust `dxf` crate used instead).

**8.7 Missing reports and conflicts.**
- No batch-A report; `✅️validation.md` missing.
- Natural-file lifecycle row count conflict (33, 48, 51, 55, 67). Reconciliation owner T06.
- Stray `.🧬️semio` directory at the repo root.
- The `🗑️generated` stdio shows `show-projects.txt` (partial queue log only).

**8.8 Stdio gates (shallow).** `stdio runtime`, `fuzz` and `cross-platform` run only the quick structural gate (`🎛️commands/🗄️stdio/🟦️.ts`).

**8.9 Bridge crates.** `semio-s-artifact-energy-model-snapshot-bridge` and `semio-semio-v1-base-bridge` have no owner workspace. Possible `--manifest-path` failure (commands.md, "Bridge crates"). Coordinator decision needed on membership.

## 9. Fleet, model and concurrency record

Source: `🗂️board.md` (concurrency log and task table), 2026-11-04.

**Fleet.** 20 worker profiles: 14 Sonnet (editors, host, persistence, formats, runtime, validation) and 6 Haiku (inventory, capabilities, data, entry, registration, documentation). **[unknown]** The runtime model of each worker cannot be verified from ListAgents, which shows profile names only. Coordinator: Opus (medium).

**Observed concurrency.**
- Start: ListAgents showed 20 of 20 subagents running at once.
- 06:51: load 20.3 on 12 cores, 7.4 GB free, capacity gate 4 running and 7 waiting.
- 07:03: load 30.8, gate 5 running and 13 waiting, 6.8 GB free. Queue rule: at most one queued cargo invocation per worker, multi-package `-p` runs.
- 07:06: load 19.9, swap full, gate 4 running and 8 waiting. Rule: each stdio artifact is its own Cargo workspace, so runs are per artifact directory and chained.
- 07:20: `repo capacity run -- repo run @semio-tech/framework-os-dev:activate-s-react-prototype` running for 12 min 47 s (dependency workspace-edge resolution). T05 and T06 blocked inside it, not stalled.

**Hand-backs and waves.** Waves 1 to 7 are listed in the board log (audits T15 to T19, host T02, persistence T03, formats T04, batches A to H, runtime T05, validation T06, documentation T20). Refills and reassignments: a11y items moved (shell items to T02; media host to T13; Paint2dHost to T09; schema enforcement to T01).

**Decisions recorded on the board.** `--store=shared` only; pdf 1.4 cross-crate dependency approved; DWG entity editing in scope; dxf viewer must render; Details split layout default for read-only main windows; subset-qualified format kinds for pdf 1.7 and ifc 2x3; NF extension aliases approved; idf routed to the energy model; pdf owner is 1.7; mp3 kinds via schema generators; duplicate TypeScript resolver removed in favour of T01's.

**Messages undelivered.** T05 and T06 status messages were queued but undelivered for a time (board, wave 5 and 6); both were blocked in long tool calls.

**Commands shown on the board for T20.** `commands.md` (startup, crate tests, bridge crates); `crate-names.md`; `coverage-matrix.md` refresh; this report.
