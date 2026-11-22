# Browser XLSX editor path audit

Read-only static audit of the browser-side XLSX path: recognition and routing in S, the editor and viewer panes, grid rendering, cell commit, save through the XLSX codec, and reopen. Covers the standalone pane (port 6272) and full S (port 6070).

Repo root: `/Users/ueli/Documents/semio`. Paths below are relative to that root unless stated. Nothing was built, served, or probed live; port state was not observed. Line numbers are from the files as they stood at audit time.

## 1. Build and serve targets

| Host | Project (name, file) | Target | Port source |
|---|---|---|---|
| Full S | `@semio-tech/framework-os-dev`, `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` | `dev-s-react-prototype` (not a literal key in that file; the name is generated, generator not located) | `S_OS_PORT`, set to `6070` only by `run-browser-io.ts:45` |
| Standalone XLSX | same project | `dev-stdio-xlsx-react-prototype` (generated, same caveat) | `S_OS_PORT`, set to `6272` only by `run-browser-io.ts:45` |
| Dev serve fallback | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:145` | Vite server | `Number(process.env.S_OS_PORT ?? 6066)` |
| Local hub launcher | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts:469` | spawns serve with `S_OS_PORT` | from request |
| WGPU serve | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts:28` | requires explicit port | `S_OS_PORT` or `--port` |
| S composition | `✏️s/🧑‍💻dev/📋️project.json` (`@semio-tech/s-dev-composition`) | `typecheck` depends on `generate-stdio-format-claims` | n/a |
| Hub I/O matrix | `🌎️hub/📦️packages/🦀️rust/📋️project.json:1580` (`os-hub`) | `io-matrix` | `--serve http://127.0.0.1:6070/` in `run-browser-io.ts` |
| XLSX artifact (Rust) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust/📋️project.json` (`@semio-tech/stdio-xlsx-rs`) | build | n/a |
| XLSX artifact (TS) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🟦️typescript/📋️project.json` (`@semio-tech/stdio-xlsx`) | build | n/a |
| Office publisher | `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/📦️packages/🦀️rust/📋️project.json` (`@semio-tech/stdio-office-plugin`) | component publication | n/a |
| Hub compositions | `🌎️hub/🧩️compositions/📋️project.json` (`@semio-tech/hub-compositions`) | composition | n/a |

Gap: no managed config declares 6070 or 6272. Both exist only in the ticket runner. Without `S_OS_PORT`, the React serve falls back to 6066.

## 2. Path, step by step

### Step 1: claims and routing data (complete, with a gap)

- The XLSX format claim is generated in `✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🤖️generated/stdio-format-claims.json:217-226`. It names `.xlsx` and the SpreadsheetML MIME, and points `artifactRef` at `s.stdio.xlsx@ecma-376/*` (line 226).
- No `revisions` entry exists for xlsx in that file. Other formats (gif, ifc, pdf, dwg) have them.
- The editor definition is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📜️artifact-definition.json`. The codec is declared `implemented` (`.codecs`), but several other statuses remain `unimplemented` or `unverified` (see gap G3).

Gap G1 (routing): Strict and Transitional are never reached by extension or MIME. The claim names only `@ecma-376/*` (claims line 226). The base analyzer always reports dialect `*` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🦀️.rs:181` and `:212`). The Strict and Transitional editors are reachable only by explicit key or alias, for example `s.stdio.xlsx@ecma-376/strict#editor` and `stdio xlsx strict` in `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🧩️component/📦️packages/🦀️rust/Cargo.toml:68-76`.

### Step 2: recognizing an opened file (complete for Open, gap for drop)

- Open file: `openNaturalFile` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx:449-496`. The picker `accept` comes from the catalog descriptors (line 450). The picker itself is `requestFileSelectionV1` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts:136-161`.
- Format resolution: the picker calls `routePickedFileV1` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📂️picked-file-opening/🟦️.ts:21-34`). It calls `resolveFormatV1` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧭️format-resolution/🟦️.ts:38-60`). Precedence is explicit key, then longest extension, then MIME. This matches the Rust resolver (`🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧭️format-resolution/🦀️.rs:81-105`). A `.xlsx` name always resolves by extension before MIME.
- Revision sniff: `routePickedFileV1` reads file heads only when a format has revisions (`picked-file-opening/🟦️.ts:30-31`). xlsx has none, so no content check happens before routing.
- Refusal text: `picked-file-opening/🔣️.json` has en and de for all four refusal reasons. Complete.
- Drop: no handler accepts OS file drops onto S. Searches for `types.includes("Files")`, `dataTransfer.files`, and file-drop handling found nothing outside tests. The only UI drop handler filters on `application/x-semio-` MIME types (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📋️receipts-drop-and-clipboard/🦀️.rs:177`).

Gap G2 (drop): dragging a `.xlsx` from the OS onto S is not recognized. Only the Open file picker works.

Gap G4 (no content check): `.xlsx` routing trusts the extension. A non-SpreadsheetML zip named `.xlsx` routes to the editor. Failure happens later in `importInto`, which creates the app instance before importing (scope `:467-483`). The user gets a generic failure notice, not a refusal.

### Step 3: owner resolution and import (complete)

- `resolveOwner` calls `resolveArtifactOpeningWithActivationRef("os.open-artifact", {artifactRef, role: "editor"})` (scope `:461-466`). The viewer is never a target. The viewer trait has no natural-file hooks (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/👁️artifact-viewer-trait/🦀️.rs:11`), so a viewer cannot open a workbook. This is by design, noted as info.
- `naturalFileFormatV1` requires matching Open and Save action args (`🧾️transfer-checkin-tutorial/🟦️.ts:69-91`). Actions are mounted from the codec declaration (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs:79-117`). Labels are en and de: "Open File…"/"Datei öffnen…" and "Save File"/"Datei speichern" (lines 95-96).
- Import: `importInto` (scope `:467-483`) reads bytes with a bound (`readBlobBytesBoundedV1`), then `openNaturalFileOwnerV1` creates, imports, and retires on failure (`transfer-checkin-tutorial/🟦️.ts:122-`).
- Editor side: `import_media` delegates to `import_media_as_load` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:201-204`). That function decodes via the codec and emits a whole-document load (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🪟️window-kit/🦀️.rs:494-520`). Any other port returns `NotImplemented` (line 517). Intentional.
- Decode: `decode_xlsx` (`…/🚪️io/📥️import/🧩️deserializers/🦀️.rs:20-37`) runs OPC decode, XML parse, and `validate_authority`. Complete.

### Step 4: editor and viewer panes (complete, with i18n gaps)

- Editor: `XlsxEditor` (`…/✏️editor/🦀️.rs:166-290`) with `create_xlsx_editor` (`:381-391`). Mode and layout: `…/✏️editor/🎭️modes/✏️edit/🦀️.rs:17-32`. Viewer: `…/👁️viewer/🦀️.rs:81-124`.
- Grid: `render_sheet_grid` (`…/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:56-102`). One tab per worksheet (`:113-118`). Revision-bound cells, with `set-cell` as the action and the `sheetName`, `row`, `column`, `revision`, `value` args (`:97`).
- Grid labels: "Row"/"Column" are chosen by a `match locale` (`:70-73`) rather than a catalog key. Both languages are present, but this bypasses the catalog (G7).

Gap G5 (English-only layout titles): `title: Some("Worksheets")` in the edit layout (`…/✏️editor/🎭️modes/✏️edit/🦀️.rs:29`, also strict and transitional at the same line). `title: Some("Cells")` in the view layout (`…/👁️viewer/🎭️modes/👁️view/🦀️.rs:30`, also strict and transitional). No German string.

Gap G6 (English-only fallback): `Label::data(format!("Unknown body: {body_key}"))` (`…/✏️editor/🦀️.rs:287`, `…/👁️viewer/🦀️.rs:114`). Not localized. Low severity.

### Step 5: cell commit (complete, with a German number gap)

- UI: cells are inputs with `aria-label` set from `record.accessibility.label` (`🧑‍🎨engine/🧱️elements/🗣️Interpreter/✍️input-views/🟦️.tsx:278-279`). Enter commits (`:209-244`, `:290`), and blur commits when `commitOnBlur` is set (`:291`). The draft-dirty flag is `data-draft-dirty` (`:278`).
- Action bridge: `command_from_action` maps `set-cell` (`…/✏️editor/🦀️.rs:240-250`) through `window_kit_stable_cell_edit` (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🪟️window-kit/🦀️.rs:399-408`). Required args are enforced. Complete.
- Emit: `xlsx_set_cell_emit_fields` (`…/✏️editor/🦀️.rs:306-347`). Revision checks at `:331-333` (cell) and `:340-342` (vacancy). Stale address faults at `:330` and `:339`. Writes `SetCell` or `InsertCell`. Complete.
- Agent path: `bounded_native_editing_editor!` (`…/✏️editor/🦀️.rs:366-376`) supplies the same reducer for tool calls. `agent_target_revision` (`:237-239`, `:292-299`) fills an omitted revision from the current cell. Complete.
- Value parsing: `parse_xlsx_cell_value` (`…/✏️editor/🦀️.rs:76-94`). Accepts `'` for literals, `=` for formulas, lowercase `true`/`false`, and numbers via `str::parse::<f64>`.

Gap G8 (German decimal input): `text.parse::<f64>()` at `…/✏️editor/🦀️.rs:89-92` rejects a German decimal comma. Typing `1,5` in a German session stores text, not a number. Display is `format!("{n}")` (`:61`, viewer `:44`), which always uses `.`, so the number format is not localized.

### Step 6: save through the XLSX codec (complete)

- Editor side: `encode_natural_file` calls `encode_xlsx` (`…/✏️editor/🦀️.rs:215-218`). `encode_xlsx` (`…/🚪️io/📤️export/🧵️serializers/🦀️.rs:9-17`) validates authority, serializes XML parts to text, and encodes OPC with package order.
- S side: `saveNaturalFile` (scope `:332-`) builds the name with `naturalFileNameV1` (`transfer-checkin-tutorial/🟦️.ts:60-63`), verifies the exported archive matches the expected revision (`verifyDelivery`, scope `:~350`), then delivers with `downloadDeclaredMediaFileV1`. Complete.

Gap G9 (save name): the file name is `xlsx-<ISO timestamp>.xlsx` (`naturalFileNameV1`, `:60-63`). The original file name is discarded. Low severity.

### Step 7: reopen (complete)

- Native reopen: `openKeptFile` (scope, starts near `:382`) reopens a kept archive and checks identity. It reuses the same document UUID. Complete.
- Import of a saved file uses the same path as Step 2 and Step 3.
- The Hub I/O matrix row `stdio-office/xlsx` (`🌎️hub/🧪️tests/🧫️fixtures/🚪️io-matrix.json:149-151`) declares `saveArtifactFile`, `openArtifactFile`, and `editedCell: "A1"`. Complete.

### Step 8: standalone pane on 6272 (gap)

- Port 6272 appears in source nowhere. It is set only by `run-browser-io.ts:45`. There is no managed default, so the pane has no reproducible launch outside the ticket runner (G10).
- The probe checks `http://127.0.0.1:6272/` (`probe-xlsx-pane.ts:9`). Its undo and redo clicks use `action.undo` and `action.redo` ids through `[data-slot="window-action-pane"]`. The ids are defined by the framework's document-command list (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/📥️ingress/🦀️.rs:28`), not by the XLSX editor.

### Step 9: full S on 6070 (gap, runtime)

- The full S host reaches the XLSX editor through the same `openNaturalFile` path (Steps 2 to 6). The Office publisher is `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🦀️.rs:23-63`. It registers the base, Strict, and Transitional editor and viewer pairs.
- Runtime readiness depends on the shared graph and the Office browser product, which were blocked at audit time per the ticket notes. Not verified here.

## 3. Concrete gaps

| ID | Severity | Where | What |
|---|---|---|---|
| G1 | high | `✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🤖️generated/stdio-format-claims.json:226`; `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🧩️component/📦️packages/🦀️rust/Cargo.toml:68-76` | Strict and Transitional editors cannot be reached by Open or drop. Only `@ecma-376/*` is claimed. Base analyzer always stamps `*` (`…/🧱️base/🚪️io/🦀️.rs:181,212`). |
| G2 | high | `🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📋️receipts-drop-and-clipboard/🦀️.rs:177` | No OS file drop onto S. The drop handler accepts only `application/x-semio-` MIME types. |
| G3 | medium | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📜️artifact-definition.json:19,27,36,73-74,79,101,285-291` | Definition metadata says `unverified`/`unimplemented` for the profile, source dialect, resource policy, conformance suite, and read/write/lossless/canonical support, with `mutations` and `inferences` empty, while the codec is `implemented`. Stale metadata. |
| G4 | medium | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📂️picked-file-opening/🟦️.ts:30-31`; scope `:467-483` | No content check before routing for xlsx. A bad file fails after an instance is created, with a generic failure, not a refusal. |
| G5 | medium | `…/✏️editor/🎭️modes/✏️edit/🦀️.rs:29`; `…/👁️viewer/🎭️modes/👁️view/🦀️.rs:30` (and strict and transitional equivalents) | English-only layout titles "Worksheets" and "Cells". No German. |
| G6 | low | `…/✏️editor/🦀️.rs:287`; `…/👁️viewer/🦀️.rs:114` | English-only "Unknown body" fallback label, not localized. |
| G7 | low | `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:70-73` | Grid "Row"/"Column" hardcoded per locale instead of a catalog key. |
| G8 | medium | `…/✏️editor/🦀️.rs:89-92` (parse), `:61` and viewer `:44` (display) | German decimal comma not parsed. Numbers displayed with `.` regardless of locale. |
| G9 | low | `🧰️framework/🛍️products/💻️os/🔨️modules/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts:60-63` | Saved file name drops the original name. |
| G10 | medium | `run-browser-io.ts:45` (ticket folder); `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:145` | Ports 6070 and 6272 exist only in the ticket runner. Without `S_OS_PORT`, React serve uses 6066. |
| G11 | medium | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs:80` | Natural-file hidden argument labels are `LocalizedLabel::native(id, id)`: the raw id in both languages. Hidden (`:81`), so low visibility. |
| G12 | info | `🧰️framework/.../🪟️surface/👁️artifact-viewer-trait/🦀️.rs:11` | Viewer cannot import a natural file. By design. |

No `TODO`, `FIXME`, `todo!()`, `unimplemented!()`, or stub was found in the XLSX artifact tree. The only `unreachable!()` hits are structural match guards (`…/✏️editor/🦀️.rs:268`, and similar in `🧬️schema`).

## 4. Ticket scripts

- `run-browser-io.ts`, `probe-xlsx-pane.ts`, and `inspect-browser-run.ts` reference only files that exist: `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts`, `🌎️hub/🧪️tests/🚪️io-matrix/🟦️.ts`, `🌎️hub/🧪️tests/🧫️fixtures/🚪️io-matrix.json`, `…/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts`, and `…/🧮️program-matrix/🟦️.ts`.
- The demo workbook `…/📚️examples/🎬️demo/🖼️assets/📕️example.xlsx` exists and is 2,196 bytes, matching the ticket note.
- The `io-matrix` target exists in `🌎️hub/📦️packages/🦀️rust/📋️project.json:1580`.

## 5. Not verified

- Live ports 6070 and 6272 (not probed).
- The generator of `dev-s-react-prototype` and `dev-stdio-xlsx-react-prototype`.
- The grid's ARIA table role and row or column header semantics.
- Whether the Office runtime exposes the same undo and redo ids as the framework list at `…/📥️ingress/🦀️.rs:28`.
- Runtime behaviour of the Office browser product and the shared graph.
