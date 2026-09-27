# Stdio Artifact Editor UX and Architecture Audit

## Scope and evidence

This audit covers `✏️s/🔌️plugins/🗄️stdio` and the directly used framework window kits. It is based on the source tree as inspected on 2026-09-26; no product files were changed.

Stdio currently declares 36 artifact kinds and 88 standard/subset editor roots. The artifact kinds are LAS, HTML, EPW, ZIP, GIF, MP4, SVG, MP3, IFC, BCF, binary, CSV, STEP, TSV, XLSX, PDF, DOCX, Markdown, XML, PNG, JPEG, AVI, PPTX, WAV, text, STL, DWG, DXF, TIFF, deflate, OBJ, glTF, PLY, JSON, semio, and BMP. Each root follows the same `✏️editor → 🎭️modes → 🪟️windows → 🪟️main` structure.

The shared authoring surface is implemented in [the framework plugin module](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs). Its seven reusable kits are:

| Family | Shared kit | Current write verb |
|---|---|---|
| Source text | `TextWindowKit` | `replace-text` |
| Tables | `TableWindowKit` | `set-cell` |
| Trees | `TreeWindowKit` | `set-node` |
| Raster/vector image | `ImageWindowKit` | `set-pixel-region` |
| Mesh/CAD | `MeshWindowKit` | `set-vertex` |
| Pages/documents | `DocumentWindowKit` | `set-page` |
| Audio/video | `MediaWindowKit` | `seek-media` |

The kits are intentionally narrow: their editable actions start at [line 33611](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33611), and the image, mesh, document, and media actions are declared without the arguments their commands require. They cannot be a complete editing contract.

## Current end-user gaps

The current UI is a preview/viewer plus one generic mutation affordance, not an editing application.

- The text kit supplies only a buffer and `readOnly` setting to `TextEditorScene`; it sends no selections, syntax tokens, diagnostics, completions, occurrences, placeholders, rename state, overlays, or newline gates. `replace-text` replaces the complete document.
- The table kit renders `Vec<String>` columns and cells and exposes only `row`, `column`, and `value`. It has no row or column insertion, removal, movement, rename, typing, formula, selection, range paste, sheet, filter, sort, validation, or metadata model.
- The JSON base editor renders a navigable tree but `set-node` emits `JsonMutation::SetScalar` as a `JsonValue::String`; it cannot preserve or choose `null`, boolean, number, array, or object values. It cannot insert/remove/move array entries or add/remove/rename object members despite the schema containing those mutations. See [the editor reducer](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs:197) and [the `handle` limitation](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs:417).
- Image surfaces render a static data URI. `set-pixel-region` has no action arguments and is repurposed as a whole-snapshot replacement for SVG; the UI has no crop, canvas, layer, palette, alpha, metadata, EXIF, frame, or region model.
- Mesh surfaces display a `World3d` scene. The sole `set-vertex` verb is unparameterized in the kit; several editors document that their implementation returns a no-op or uses a whole-snapshot substitution because no matching per-vertex mutation is wired.
- Page surfaces render a vertical stack of plain text rather than page geometry and expose only `set-page`. They cannot create, remove, reorder, restyle, lay out, or edit document/presentation structures.
- The media kit emits a three-row key/value list. MP3 demonstrates the failure clearly: it renders duration and position as zero and declares `seek-media`, but only registers the example-selection tool and reduces seek to `Emit::default()`. See [MP3 editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/✏️editor/🦀️.rs:16) and [its main window](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:13).
- Inconsistency is structural. The artifacts contain 224 generated `SetSnapshot` schema references, but only six editor roots directly use such a mutation. Some editors completely route their kit action through an app-owned retained job; many register only the `setActiveExample` route. An editable window can therefore visibly declare a verb that cannot reach a persisted edit.
- Generic schema infrastructure exists (`ArtifactSchema`, `ArtifactSchemaFields`, generated JSON schema, and `ToValue`/`FromValue` derives), but there is no reusable schema-to-property-form/editor component. The framework has primitive inputs, selects, steppers, tree/table widgets, and the action pane, but no reflective property editor that can walk a snapshot and publish a typed artifact mutation.

## Existing event-sourced route to reuse

Do not add a browser-only mutable editing session. The correct integration pattern already exists in the JSON and CSV editors:

1. A visible action is declared on the app/window definition.
2. `ArtifactEditor::command_from_action` converts the host action and arguments into a typed command.
3. The editor's command id is listed in `OpBinary::TOOL_JOB_IDS`, with an `ArtifactToolPublicationContract` authorizing the `Artifact` lane.
4. An `ArtifactOwnedToolJobFactory` is registered. It produces `ArtifactRetainedCommandJob<EditorApp<E>>` and a bounded `ToolExecutionContract`.
5. `BoundedArtifactCommandWork` calls the reducer with the persisted snapshot; the reducer emits the format's typed mutation(s). The document store appends those events, then the editor re-renders from the new snapshot.

[The JSON editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs:110) is the precise working template: retained-tool roster, publication contracts, bounded work, factory registration, `build_tool_job`, and `command_from_action` are all present. Any new common editor operation has to be added through every one of those joins. Registering only a window action recreates the current dead-action defect.

## Recommended architecture

Build one schema-first **Stdio snapshot editor protocol** and use it in all 88 roots. Keep the format-specific semantic visualizer as the primary canvas, but add an additional split **Details** window beside it. The Details window owns the universal property tree and its Source tab; it never replaces the format canvas and it reaches every persisted snapshot field.

### 1. Common snapshot edit protocol

Define one generated, language-neutral operation schema in stdio's shared artifact contract. It needs stable field paths and typed values, not string-only values:

- `set-value { path, value }`
- `insert { collection-path, index/key, value }`
- `remove { path }`
- `move { collection-path, from, to }`
- `rename { path, key }`
- `replace-source { source-format, source }`
- `replace-bytes { range, bytes }` for binary payloads, with paged transfer/progress/cancellation metadata.

Expose each operation as an immutable command/event. The shared reducer applies it to the current typed snapshot using the generated `ToValue`/`FromValue`/schema representation, validates the resulting snapshot against the selected standard/subset, and emits that artifact's own generated mutation. Prefer a native granular mutation when its schema declares one; otherwise emit the artifact's existing generated `SetSnapshot`. This remains event sourced, keeps undo/history and multi-user replay correct, and gives every format a complete escape hatch without inventing 88 disconnected editors.

The common protocol should own the retained routing boilerplate in a stdio Rust shared module or macro: tool-id roster, publication contract, payload schema, bounded job factory, proof macro, registration, and `build_tool_job`. Each artifact root supplies only a typed adapter: dialect, snapshot/mutation types, validation, serialization, and a mapping from common operation to the native mutation or `SetSnapshot`. This replaces per-root hand-copied factories and closes the existing registration gaps.

### 2. Reflective Details editor

Add a reusable framework `PropertyEditorWindowKit`, rather than adding fields to the current seven kits. It should render a virtualized, schema-driven tree/form from the snapshot's generated schema and value representation. Each scalar chooses a typed control (string, integer, decimal, boolean, enum, date/time, bytes, reference); objects and lists provide explicit add/remove/rename/reorder controls; validation diagnostics stay adjacent to the field; selection synchronizes with the format canvas.

The SDK already has the raw pieces: `ArtifactSchema`/`ArtifactSchemaFields` in [the schema module](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs:136), action argument controls, virtualized tree windows, and event-sourced artifact commands. It does **not** currently have this composition, so it must be implemented once in the framework, then called from stdio. It must expose localized labels and descriptions from schema annotations, with neither locale selected as a default.

### 3. Source editor as the complete fallback

Every format must also provide an explicit Source tab that serializes the *current* snapshot with its native `ArtifactDsl` printer and commits through `replace-source`. Parse and validate before publishing, report exact diagnostics and a diff preview, and preserve the user's draft locally only while editing. On Apply, publish one immutable command that reduces from the stored snapshot. This makes every represented detail editable on day one, including details for formats whose visual family editor is still improving.

For large source/binary payloads, use a paged operation payload with progress and cancellation, never an unbounded action argument. The renderer already has paged text carriers for large image data, and the retained job system already bounds and checkpoints command work. The UI should show parse/validation progress, allow cancellation before event publication, and never freeze on a connection interruption.

### 4. Family canvases use the same protocol

Make family experiences pleasant by translating their direct interactions into the same common operations:

| Family | Required direct experience |
|---|---|
| Text/markup | source plus rendered preview, find/replace, diagnostics, outline, typed attributes |
| JSON/XML/BCF/archive | typed tree, add/remove/rename/move, value-kind chooser, attributes, comments, order |
| CSV/TSV/EPW/XLSX | sheet/table selection, cells, rows, columns, ranges, paste, types, formulas, headers, metadata |
| Image/GIF/TIFF/JPEG/PNG/SVG | canvas, zoom/pan, crop, frames/layers when modeled, palettes/metadata, region edits |
| Mesh/CAD/OBJ/STL/PLY/glTF/IFC/DWG/DXF/STEP | hierarchy, inspector, transforms/materials/attributes/topology, selection-linked property edits |
| PDF/DOCX/PPTX | page/slide navigator, content/metadata/structure inspector, order/insert/remove and source fallback |
| MP3/WAV/MP4/AVI | actual decoded transport, timeline/track/metadata editor, source fallback |
| ZIP/deflate/binary/LAS | member/segment tree, byte-range hex view, decoded header/property inspector, replacement/import controls |

No family canvas gets a private write model. It dispatches shared operations and reads the persisted snapshot after every accepted event.

## Delivery order

1. Add the common protocol, retained-route helper, source window, parser/validator diagnostics, and property editor kit. Wire `replace-source`/typed `set-value` through every stdio root, using generated `SetSnapshot` where no granular mutation exists. This gives all 88 roots complete edit reach.
2. Replace the seven one-action kit contracts with rich family contracts, preserving the source/details fallback. First correct JSON/XML, CSV/TSV/XLSX, archive/binary, and media because their current declared actions are missing or nonfunctional.
3. Add format family adapters and selection synchronization. Do not duplicate command factories or mutate a client-only document copy.

## Required verification

Use a generated 88-root matrix. For each root, open the editor, enumerate the declared actions, verify every action resolves through `command_from_action`, has a retained factory/proof/publication contract, emits an artifact event, persists, reloads, and restores the identical snapshot. Exercise source edits for every scalar, object/list operation where represented, invalid source diagnostics, cancellation, a large paged payload, undo/redo, and a short reconnect.

For each property operation, compare the resulting serialized artifact with a third-party format parser where one is already available in the repository's test dependencies. Pair the language-agnostic fixture with the Rust editor test so the UI event, event log, codec, and emitted artifact agree.

Existing interactive launch entries cover only Markdown, CSV, HTML, JSON, TSV, text, and XML (including i-JSON and XML valid) in React and WGPU. They begin at [launch.json:7008](/Users/ueli/Documents/semio/.vscode/launch.json:7008). Add an ordered React/WGPU launch pair for every remaining format family and root used for acceptance testing. Existing broad checks are `@semio-tech/stdio-js:build`, `:check`, `:test`, `:package-contract`, and `:package-graph` at [launch.json:5202](/Users/ueli/Documents/semio/.vscode/launch.json:5202); they are necessary but do not prove user-visible editing.
