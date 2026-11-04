# 📊️ Coverage Matrix: Well-Known (stdio) Artifact Editors

Single source of truth for the fleet goal. Maintained by the coordinator's recurring job (owner of this file: T20 haiku-documentation, updates from batch reports and logs). Last update: 2026-11-04 after batch B, C, D, F, G, H reports, registry addenda 2 to 4, data audit (T17e, T17f, T03 verdict), and the Steam Deck usability audit (T18d, T18e).

**Scope.** 38 stdio artifact definitions (39 directories under `🗿️artifacts` minus `📇️inventory`, which is a library). 36 ship `#editor` + `#viewer` for 88 editor/viewer pairs (`🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🧫️fixtures/✏️editor-catalog/🔣️.json`; shipped-fleet law `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs`). `idf` is definition-only; it opens in the energy model editor (batch F). `idml` now has a `#editor` and `#viewer` surface crate (batch B) but is not yet in the hub roster. `📇️inventory` is a library, not an artifact.

**Architecture.** `EditorApp<E>`/`ViewerApp<V>` wrap `ArtifactEditor`/`ArtifactViewer`. They are registered via `builder.editor/viewer` in 10 hub packages (core, binary, office, pdf, bim, media, image, semio, mesh, cad) plus playground rows. Opening goes through `os.open-artifact` → `OpeningResolver` (dialect + role). Editing uses six generic snapshot-edit actions plus per-artifact mutations. Natural-file open/save uses `natural_file_codec`.

**Columns.**
- NF = natural-file codec (open/save a real file from the app). `yes (uncompiled)` means code was written in this ticket and never compiled.
- Native = native pack receipt (durable store). `yes (uncompiled)` means a receipt was added in this ticket and the generators (`composition-prepare`, `native-codec-projection`) have not run.
- Edit path = `main-window` (the edit window of `✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main`), `Details` (generic snapshot-edit pane), or `none`. Batch C moved bmp and tiff to `main-window`, so 15 RO main windows remain on `Details`.

**Status values.**
- `gap`: a blocker is open and no work in this ticket closes it.
- `in progress (uncompiled)`: code exists (pre-existing or written in this ticket) but no compile or test result is recorded in a log of this ticket.
- `compiled`: a log shows a successful compile. None yet.
- `contract-verified`: the offline acceptance contract (`🧪️contract`, bun test) passes for the row: definition agreement, shipped editor and viewer in the family component and generated catalog, catalog activation, valid sample accepted by its third-party oracle, invalid input refused. **This is not editor e2e and not an editor unit test.** Source: `✅️validation.md` E1 (192 of 194 contract tests pass; only idml fails).
- `tests green`: a log or report shows the editor's cargo or vitest unit tests passing. None yet for an editor (the only passing cargo log, `t01-os-io-format-resolution.log`, is the registry law, not an editor; see Evidence).
- `e2e verified`: a browser journey log shows open, edit, save, reopen. None yet.

| # | Batch | Artifact | Ed/Vw | Package | NF | Native | Edit path | Known gaps | Status |
|---|---|---|---|---|---|---|---|---|---|
| 1 | A | txt | 1/1 | core | yes | yes | main-window | compile not recorded (`a-txt-editor-test.log` still compiling) | contract-verified |
| 2 | A | md | 1/1 | core | yes | yes | main-window | DSL `wrap_text` trims BOM and leading whitespace (data audit) | contract-verified |
| 3 | A | json | 2/2 | core | yes | yes | main-window | definition `mutations: []` drift while i-json has mutations; definition directory fixed (T19) | contract-verified |
| 4 | A | html | 1/1 | core | yes | yes | main-window | root whitespace lossy; registry descriptor now generated (addendum 2, `stdio-format-claims.json`, not run through `repo`); composer/grammar/inference capabilities absent (F8) | contract-verified |
| 5 | A | xml | 2/2 | core | yes | yes | main-window | DSL `wrap_text` trim (data audit); preserves CDATA, comments, PI | contract-verified |
| 6 | B | csv | 1/1 | core | yes | yes | main-window | line endings normalized (lossy); DSL `wrap_text` trim; batch B: natural-file codec declared | contract-verified |
| 7 | B | tsv | 1/1 | core | yes | yes | main-window | registry descriptor now generated (addendum 2, uncompiled); byte-exact codec | contract-verified |
| 8 | B | xlsx | 3/3 | office | yes | yes | main-window (unverified) | window capability unverified (T16); unknown XML parts and binary OPC parts kept (batch B) | contract-verified |
| 9 | B | docx | 3/3 | office | yes | yes | main-window | raw rPr/pPr preserved; unknown XML parts kept (batch B) | contract-verified |
| 10 | B | pptx | 3/3 | office | yes | yes | main-window | XML parts preserved; custom-part fixture (batch B) | contract-verified |
| 11 | B | idml | 1/1 (new surface crate `semio-s-artifact-stdio-idml-any`) | — | yes (uncompiled, `.idml`) | no | main-window (document kit, `set-page`; story runs via `set-story-content`) | not in hub roster or composition (needs `composition-prepare`); story splice tests use `quick-xml` cross-check (uncompiled); no native receipt | in progress (uncompiled) |
| 12 | C | png | 1/1 | image | yes | yes | main-window | ancillary chunks (tEXt, eXIf, iCCP, unknown) retained as opaque chunks; retention test written (uncompiled, png crate cross-check) | contract-verified |
| 13 | C | jpg | 2/2 | image | yes | yes | Details | re-encoded scan and tables (lossy; APPn raw); region paint not added (lossy re-encode); details edit covers JFIF header and segments | contract-verified |
| 14 | C | bmp | 1/1 | image | yes | yes (uncompiled, `stdio.native.bmp.v1` added; generators not run) | main-window (paint-indexed-region, paint-direct-region as window actions) | palette edits fall to `replace-image` (no palette-entry mutation); native generators pending | contract-verified |
| 15 | C | gif | 2/2 | image | yes (uncompiled, 87a and 89a) | yes (uncompiled, `stdio.native.gif.87a.v1` and `.89a.v1`) | Details | dead `SetPixelRegion` variant not routed; natural-file and native receipts uncompiled; generators pending | contract-verified |
| 16 | C | tiff | 2/2 | image | yes (uncompiled, document and baseline) | yes | main-window (document editor; baseline has no paint) | paint-region is a window action on the document editor (uncompiled); baseline editor has no paint | contract-verified |
| 17 | D | svg | 3/3 | image | yes | yes | Details | main window read-only (T16); element rename, typed transform and view-box edits still open (recipe) | contract-verified |
| 18 | D | dxf | 1/1 | cad | yes (uncompiled, `s.stdio.dxf@r12`) | yes | Details | main window is a real 2D preview (uncompiled); viewer read-only; third-party cross-check with `dxf` 0.6 crate written (ezdxf absent); queued run `d-dxf-editor-shared.log` still compiling, no test result | contract-verified |
| 19 | D | dwg | 2/2 | cad | yes (uncompiled, `s.stdio.dwg@ac1018`, `@ac1024`) | yes | Details (whole-drawing `set-drawing`) | typed geometry rendered in both main windows (uncompiled); line-endpoint round-trip test decides whether the writer needs extension; layer and object edit kinds not started | contract-verified |
| 20 | D | pdf | 10/10 | pdf | 1.7 base yes; 1.7 subsets a, e, h, ua, vt, x yes (uncompiled); 1.4 base, a, x yes (uncompiled, lossless only); 1.4x no | yes (two factories) | main-window (1.7 base) | 1.4 path refuses any file it cannot reproduce byte for byte (loss decision); subset window capability unverified; `qpdf --check` tests written, not run | contract-verified |
| 21 | E | obj | 1/1 | mesh | yes (uncompiled) | yes | Details | transform `/transform` (uncompiled); real geometry in main window (uncompiled); `set-vertex` not wired; no third-party obj oracle; no editor package exists (the obj subset `✏️editor` has no Cargo.toml); run must use the obj artifact workspace (see Evidence) | contract-verified |
| 22 | E | stl | 1/1 | mesh | yes (uncompiled) | yes | Details | NF codec writes ASCII; real geometry (uncompiled); transform via Details | contract-verified |
| 23 | E | ply | 1/1 | mesh | yes (uncompiled) | yes | Details | NF codec keeps ascii or binary form (uncompiled); real geometry (uncompiled); transform via Details | contract-verified |
| 24 | E | gltf | 1/1 | mesh | yes (uncompiled) | yes | Details | saves `.gltf` only (`.glb` import only); transform only on scene-root nodes (shear refused); 3 mutations registered while definition says unimplemented; 3D window still placeholder | contract-verified |
| 25 | E | las | 1/1 | mesh | yes (uncompiled) | yes | Details | real geometry in main window (uncompiled); transform via Details | contract-verified |
| 26 | E | step | 7/7 | cad | yes (uncompiled) | yes | Details | 3D window still placeholder (brep tessellation missing); cc1..cc6 have codec but no transform gesture; `set-vertex` declared, not wired | contract-verified |
| 27 | F | ifc | 5/5 + 4x3 viewer (new) | bim | yes (uncompiled) for ifc 4, 2x3 base, cobie, cv20, sav, 4x3 | no (none declared) | Details | ifc 4 window read-only; 4x3 viewer and editor not regenerated into hub manifests (needs `composition-prepare`); cobie, cv20, sav domain mutations not wired (base vocabulary only); addendum 4 content sniff (`FILE_SCHEMA`) decides revision, IFC4X3 opens in IFC4 owner until its editor joins the catalog | contract-verified |
| 28 | F | bcf | 1/1 | bim | yes (uncompiled) | yes (ext "semio") | main-window | NF codec refuses archives without `bcf.version` (uncompiled); fixtures minimal, truncated and missing-version written by python (not our encoder); 0-byte demo placeholder replaced; jszip oracle not run | contract-verified |
| 29 | F | idf | — | energy model | yes via energy model `#editor` (uncompiled, `s.stdio.idf@25.2`) | no | Details (energy model editor, unverified) | definition-only artifact; `.idf` routes to `s.energy.model@1/*#editor`; import diagnostics refused by name; no standalone surface (coordinator decision) | contract-verified |
| 30 | F | epw | 1/1 | binary | yes (uncompiled) | no | main-window | independent oracle now present (Python stdlib `csv`, `natural_file_matches_the_independent_csv_oracle`, uncompiled); no native receipt | contract-verified |
| 31 | G | mp3 | 1/1 | media | yes | yes (ext "semio") | Details | shared `media_transport_view` (uncompiled); no frame-range trim (`patch-frames` not built; hand-authored multi-file change, see batch G); ffprobe tests uncompiled | contract-verified |
| 32 | G | wav | 1/1 | media | yes | yes (uncompiled, `stdio.native.wav.v1` added with PLACEHOLDER `pack_schema_hash`; must be refreshed before shipping) | main-window (table `edit-audio` and `cut-frames`) | placeholder hash in three JSON documents; `cut-frames` needs `composition-prepare`; ffprobe cut test uncompiled | contract-verified |
| 33 | G | mp4 | 1/1 | media | yes | yes (ext "semio") | Details | window "Unsupported" bug fixed in code and moved to shared media view (uncompiled); ffprobe trim test uncompiled; no trim | contract-verified |
| 34 | G | avi | 1/1 | media | yes | yes (ext "semio") | Details | same window fix (uncompiled); link to avi window unit module added; ffprobe trim test uncompiled | contract-verified |
| 35 | H | zip | 2/2 | binary | yes (uncompiled) | yes | main-window | NF codec written (uncompiled); unsafe-name guard (uncompiled); definition `mutations: []` drift; iso21320 surface has no codec; `h-zip-editor-test.log` failed to start (see Evidence) | contract-verified |
| 36 | H | deflate | 1/1 | binary | yes (uncompiled, `.zz`) | yes | main-window (header text only) | payload not editable; recompressed on save (lossy); docs name nonexistent `replace-text` action | contract-verified |
| 37 | H | binary | 1/1 | binary | yes (uncompiled, `.bin`) | no | main-window (hex splices) | no native receipt (needs `native-codec-projection`) | contract-verified |
| 38 | H | semio | 19/19 | semio | no (sniff dispatch needed) | no | main-window (mesh and brep subsets); Details (other subsets, unverified) | 19 subsets share `.semio`; a natural codec needs one registry-level sniff dispatch; no native receipt; about 15 subsets read-only and not started | contract-verified |

**Cross-cutting gaps.**
- No end-to-end open → edit → save → reopen test exists for any artifact. No stdio artifact has been opened by a test (entry audit D5).
- Editor unit tests: no editor-level cargo or vitest test has a passing result in this ticket. Every editor cargo run is queued, failed to start, or was cancelled (see Evidence). The offline contract passes for 37 of 38 rows (`contract-verified`); idml fails.
- Generators not run: `composition-prepare` and `native-codec-projection` are needed for bmp, gif, wav, ifc 4x3, idml and the new receipts. `stdio-format-claims.json` was generated by node only; the Rust parity test has not run.
- There is no generic fallback viewer (entry audit).
- Stdio default pins: the React `setDefaultApp`/`openArtifact` wire is implemented (`host.md` §5, D4), and wgpu reads `default_app_pins`. Pins are not persisted from wgpu to the OS config lane yet.
- Format catalog: `S_FORMAT_CATALOG_V1` is wired into the React and GIS entries (addendum 2). Shell-side content sniff for multi-revision formats (`revisionsOf`, `pickRevisionV1`) is specified in addendum 4, not yet called from the shell.
- Multi-revision owners (addendum 3): pdf 1.7, gif 89a, dwg ac1024, ifc 4. A `.ifc` with `FILE_SCHEMA(('IFC2X3'))` opens through the IFC2X3 revision only when the sniff is used.
- Dashboard startup bug (`dev-s-react-dev` target) is fixed in code and not run (`🛠️runtime.md`).

## Cross-cutting blockers

Owner names follow the routing in the data audit and the usability audit, and the coordinator's assignment of 07:20 (sonnet-host T02 owns the shell items A1, A2, A3, A5, A6, A7, A8; batch E owns mesh keyboard A4; batch C owns Paint2dHost keyboard A10; batch G owns the media host A11; batch D owns PDF/UA text A12; T01 owns schema-level accessible-name enforcement A13; T19 owns the duplicate Tools labels A14). No item has runtime evidence yet; all are static findings.

### Critical: save status and close (data loss)

Source: `🔍️data-audit.md` (T17e "Save Status to UI", T17f "Stale-Async Protection", T03 verdict table, Top Risks 2 and 3).

| ID | Item | Evidence (static) | Owner | Status |
|---|---|---|---|---|
| C1 | "Saved" shown before acknowledgement | `pollFolderOnce` sets `persisted: true` on any successful read (`🏪️store/👷️worker/📁️folder/🟦️.ts:103`); check-in pill ignores `persisted` (`transfer-checkin-tutorial/🟦️.ts:397-401`). T03 reports the write path fixed (F7, `persistFolderArchive`); the poll path was not re-checked. | T03 (worker), T02 (pill) | gap (re-check needed) |
| C2 | No dirty marker on windows or tabs | Only the text-draft marker exists (`✏️TextEditor/🟦️.tsx:546`), and it clears on admission, not on acknowledgement | T02 | gap |
| C3 | Close discards unsaved work; no navigation guard | `closeDocument` clears pending (`close-document/🟦️.ts:20-94,148-157`); worker teardown discards `pendingMutations` and the unsaved archive (`🛑️session-teardown/🟦️.ts:110-128`); Rust close never flushes `unsaved` (`🎬️…/🦀️.rs:181-208`); no `beforeunload` | T03 (close), T02 (guard) | gap |
| C4 | In-flight save after close | TS: aborted by `docAbort` (`✍️write/🟦️.ts:29-33,65`); Rust: write completes but `unsaved` dropped | T03 | gap |
| C5 | Archive written under the wrong document | Folder persistence closure reads archive by instance (`🔸️open-document/🟦️.ts:46-53`); if the instance switched to B, A's persistence writes B's archive under A's id | T03 | gap (critical) |
| C6 | Corrupt archive can be overwritten | Decode failure is treated as absent (T03 verdict: "partial"; setup read error fixed) | T03 | gap (critical) |
| C7 | Write failure reason discarded | No error field on status (T03 verdict: partial) | T03 | gap |
| C8 | Stale rollback after failed write | Retry clobbers external divergence; rollback still runs when `unsaved` is none (T03 verdict: partial) | T03 | gap |
| C9 | `append_ops` not atomic; TS dev plugin still uses `documents.db` | T03 verdict | T03 | gap |
| C10 | Natural-file import has no single-flight guard; panel rebuilt from stale state | `🔶️scope/🟦️.tsx:141-187`, `update-space-panel:63-65` | T02 | gap |
| C11 | Runtime key takes first plugin and instance match | `🔸️document-transfer-ref/🟦️.tsx:156-161` | T02 | gap |

Verified fixed in code, not runtime-tested: torn tail, atomic folder writes, watcher path (Rust), sticky `persisted` in the Rust actor (T03 verdict). No repro log exists for any row.

### High: accessibility and Steam Deck usability

Source: `🎮️steamdeck-usability-audit.md` (T18d "High" items 1 to 7; T18e "High" items). Owner: the usability audit routes no owners; "proposed" means this matrix suggests one.

| ID | Item | Evidence (static) | Owner | Status |
|---|---|---|---|---|
| A1 | Panels float over the editor; no max clamp, no collapse on shrink; reopen on tool select | `ui/🖼️Panel/🟦️.tsx:170,385-386,436-440`; `sh/🏗️dock-assembly/🪟️panel-reveal-and-tool-tabs/🟦️.ts:322-323` | sonnet-host (T02) | gap |
| A2 | Text editor cannot be selected by touch (no `touch-action: none`) | `sh/✏️TextEditor/🟦️.tsx:50-94` | sonnet-host (T02) | gap |
| A3 | Text editor and shell menus need right click; no visible button | `TextEditor:104`, `build-shell-context:56-67`, `ContextMenu:449`, table row menus | sonnet-host (T02) | gap |
| A4 | Mesh suggestions need Alt + right click; no touch or keyboard route | World3d `instance-selection/🟦️.ts:569-591` | batch E (T11), mesh keyboard | gap |
| A5 | Opening an artifact does not move focus | `🔶️catalog/🟦️.ts:199-214`; `🔸️open-artifact-app/🟦️.ts:70-107` | sonnet-host (T02) | gap |
| A6 | Ctrl+Z swallowed in Details fields; editor history unreachable with field focus | `⌨️keyboard-chords/🟦️.ts:16-17`; `document-transfer-ref:54` | sonnet-host (T02) | gap |
| A7 | TextEditor traps Tab; Escape does not leave (WCAG 2.1.2) | `✏️TextEditor/🟦️.tsx:388-393,295-298` (T18e High) | sonnet-host (T02) | gap |
| A8 | Details tree has no arrow-key navigation (no tabIndex, no key handler) | `🖱️ui/🌳️Tree/🍃️item/🟦️.tsx:208-211` (T18e High) | sonnet-host (T02) | gap |
| A9 | No gamepad layer (Steam Input emulation only) | documented limitation | none (limitation, not a defect) | accepted limitation |
| A10 | Paint2dHost has no keyboard route for paint | T18f (image a11y) | batch C (T09) | gap |
| A11 | Media host: trim sliders not wired, host stuck in loading after cancel, raw milliseconds | `🎬️MediaTransportHost/🟦️.tsx:114,149,173,177-178` (T18e Medium) | batch G (T13) | gap |
| A12 | PDF/UA text layer and accessible text (PDF 1.4 page kit not traced) | T18f | batch D (T10) | gap |
| A13 | Schema-level accessible-name enforcement (labels required at schema level for en and de) | T18f | T01 | gap |
| A14 | Duplicate "Tools" labels | T18f | T19 | gap |

Medium items from both audits (double-click only, Enter blurs drafts, Escape overlap, no minimum window size, hover-only states) are not listed here; see the audit files. Media trim and cancel items are in A11.

## Evidence

Logs in `🗑️generated/` (all paths under the ticket folder). No row has a passing result.

| Log | Scope | Result |
|---|---|---|
| `🗑️generated/a-txt-editor-test.log` | txt tests | compiling; no test result |
| `🗑️generated/d-dxf-editor-baseline.log` | dxf | cancelled by SIGTERM; no result |
| `🗑️generated/d-dxf-editor-shared.log` | dxf (`semio-s-artifact-stdio-dxf-tests`, `-mutations`) | compiling (about 300 `Compiling` lines), warnings only; no test result |
| `🗑️generated/e-obj-editor-test.log` | obj | failed to start: package spec `semio-s-artifact-stdio-obj` did not match any packages (root selection; the obj artifact workspace owns the crate) |
| `🗑️generated/e-semio-s-artifact-stdio-obj-editor::obj.log` | obj | waiting for capacity; no result |
| `🗑️generated/e-transform-test.log` | batch E transform | failed to start: `semio-s-artifact-stdio-contract-editing` did not match any packages (root selection) |
| `🗑️generated/f-bcf-lib.log`, `f-bcf-natural.log`, `f-epw.log`, `f-ifc4-natural.log`, `f-ifc2x3-natural.log` | bcf, epw, ifc | waiting for capacity or Cargo.lock refresh; no result |
| `🗑️generated/f-batch.log` | bcf, ifc (base, cobie, cv20, sav, 4x3), epw natural-file batch | waiting for capacity; no result |
| `🗑️generated/h-zip-editor-test.log` | zip | failed to start: `semio-s-artifact-stdio-zip-editor` did not match any packages (root selection; run from the zip artifact workspace) |
| `🗑️generated/h-zip-editor-test2.log` | zip | waiting for capacity; no result |
| `🗑️generated/t01-os-io-format-resolution.log` | registry format resolution (os-io) | compiling (warnings), no test result |
| `🗑️generated/t03-rust-sync-native.log`, `t03-rust-sync-native2.log` | persistence native actor | no test result |
| `🗑️generated/t13-wav-codec.log` | wav codec | cancelled by SIGTERM while waiting for capacity; no result |
| `🗑️generated/show-stdio-plugin.txt` | `repo show project @semio-tech/stdio-plugin` | waiting for capacity; no result |
| `bmp` run (`c-bmp-editor.txt` stray path) | batch C bmp | not produced: a stray directory name from a variation-selector typo; no result. Batch C's queued command `repo cargo ... -p semio-s-artifact-stdio-bmp-editor-2 --lib editor::bmp` is still waiting (`🗑️generated` batch chain logs) |

Passing results (not editor unit tests; all logs in `🗑️generated/`, transcribed in `✅️validation.md`): acceptance contract re-run `contract-rerun.log`, rc=1, 192 pass, 2 fail (idml, owner batch B) (E1); registry format-resolution law `t01-os-io-format-resolution.log`, 5 of 5, `EXIT 0` (E2); registry vitest `registry-vitest.log`, rc=0, 4 pass (E4); media vitest `media-vitest.log`, rc=0, 20 pass, and media React vitest `media-react-vitest.log`, rc=0, 10 pass (E7); host vitest `host-vitest.log`, rc=1, 96 tests pass in 8 files, one suite (`🛰️framework-plugin-runtime`) fails to transform, owner T02 (E3). None of these is an editor unit test, so no row moves to `tests green`.

Next actions (heavy queue, T06; all logs are deleted at close after transcription in `✅️validation.md`): re-queue every failed run from the owning artifact workspace using the names in `📦️crate-names.md` (see `📘️commands.md`, section "Running stdio crate tests"). Then queue the batch test commands listed in each batch report, starting with the zip editor, obj root and contract-editing transform tests.
