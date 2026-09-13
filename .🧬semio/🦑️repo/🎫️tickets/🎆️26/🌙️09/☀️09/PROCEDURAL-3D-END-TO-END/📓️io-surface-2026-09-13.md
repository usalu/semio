# 🚪️ generation3d IO — from nine unreachable codecs to a real user surface (2026-09-13)

Lane `io-surface` (Opus). Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.
Repo/semio MCP both failed to connect all session (`repo`: -32602 invalid initialize params;
`semio`: CONNECTION_CLOSED) — no ticket opened/closed/reopened by this lane, no `📓️status.md` or
`🎫️ticket.json` edit, no modifying git command, no dev server started or stopped.

## TL;DR

`📓️audit-user-journey-gaps-2026-09-13.md` §6 / P0 #1: nine round-trip-tested codecs in
`generation3d/🚪️io/`, and **zero references to them from any command, menu, button or keybinding** —
a user could not import or export anything. That is closed, and **proven in the browser on 6018**:

- **Export**: the flow window's Actions pane offers `Export Document…`; picking a format and pressing
  Execute produces a real download. Measured: `generation3d.stl`, 5 080 B, decoding to
  `solid generation3d-preview / facet normal 0 0 -1 / vertex 0.5 0 0 …` — the hexagonal mushroom
  column's **evaluated** preview as genuine ASCII STL. `generation3d.txt` came back as 1 572 B of the
  document's own `semio procedural.generation3d.dsl v1` text, byte-correct.
- **Import**: `Import Document…` opens the **real file picker** with the accept filter this artifact
  derives from its own formats (`.stl,.obj,.ply,.gltf,.dwg,.json,.txt`); choosing a real `.stl`
  replaces the graph with the three-widget import fixture
  (`imported-source`/`imported-geometry`/`imported-preview`).
- **Laws**: 45 (io-round-trip, incl. the new document-surface lane with `parry3d` as the third-party
  oracle) + 14 (editor/viewer document-io) + 43 (stdio-stl) + 54 (stdio-txt) + 6 + 6
  (stdio plugin registry) — all foreground, all green. TypeScript twin green and `tsc --strict` clean.

Three defects were found **by running it**, two fixed here, one reported (§7).

---

## 1. What was actually wrong

The codecs were never the problem — `📓️io-codecs-2026-09-09.md` had already made all nine real. The
problem was that nothing named them. `Generation3dCommand` had no IO row; the viewer's eight commands
were all view-only; `grep -rln "🚪️io" ✏️editor 👁️viewer` returned nothing. Library-complete IO with
no door.

## 2. The established pattern this follows

Found before designing anything, by grepping the framework and every plugin:

- `process3d` (`✏️s/🔌️plugins/🏭️process/…/✏️editor/🎮️commands/📤️media/🦀️.rs`) is the exact analogue —
  a 3D artifact with a `🚪️io` module and a **triple**: a picker verb emitting
  `Effect::RequestFileOpen`, an internal verb receiving the picked file, and an export verb emitting
  `Effect::DownloadMediaExport`. `cad` (`importCadFile`), `space` (`importSpace`) and `remodeling`
  are the same shape.
- The chunked inbound lane is `puzzle3d`'s (`🎮️commands/📥️import-fixture/🦀️.rs`), whose host half is
  `dispatchOpenedFiles`/`importPayloadChunks` (`🛠️ShellHelpers/🟦️.tsx:785-832`): the shell sends one
  invocation per chunk, in order, awaiting each.
- Format facts come from each `s.stdio.<format>` artifact's own `formats()`.

So the names are `importDocumentRequest` / `importDocument` / `exportDocument` — the coordinator's
`importDocument`/`exportDocument` shape in the repo's own `<verb><DomainNoun>[Request]` vocabulary.

## 3. Design

### 3.1 One composition point, no restated format facts

`🚪️io/🦀️.rs:168` `document_io` is the whole user-facing half. A roster row
(`EXPORT_FORMATS` `:193`, seven; `IMPORT_FORMATS` `:209`, seven) carries the id the user picks, the
en/de labels, and the **owning artifact's own representation id** — extension, MIME and binary-ness
are read from that artifact's `formats()` (`descriptor_of`), never restated here. So the accept
filter (`:248`), the download filename and the base64 decision (`document_export_envelope` `:294`)
all follow from the format's owner.

`IMPORT_ONLY_IN_REGISTRY` (`:221`) names the two leaves the picker deliberately withholds — `las`
(export-only) and `png` (no recoverable graph) — and a law drives their `deserialize_bytes` to prove
they really cannot, so a leaf that later gains a decoder fails the law instead of staying hidden.

`export_format_options()` (`:242`) builds the picker options once; the editor and the viewer both
call it, so the two surfaces cannot drift.

### 3.2 Import is event-sourced, not a document replace

`importDocument` builds an ordered batch of real `Generation3dMutation`s via
`generation3d_fixture_operations` — exactly what `setActiveExample` does — so one `mod+z` puts the
previous graph back. An `Effect::LoadDocument` would have been a CRUD write with no inverse. The
camera rides the Config lane through `config_after_document_load`, which was
`config_after_example_load`, renamed and shared rather than copied.

### 3.3 Progress and cancellation are the chain that already owns them

Two halves:

- **Inbound**: `importDocument` carries `{name, payload, chunk, chunkCount}`. Staging lives in the app
  **instance's** retained owner (not a process-global `OnceLock`, unlike puzzle3d's) so two users
  importing into two documents never see each other's bytes. `Staged { next_chunk, chunk_count }` is
  the progress a surface reports and costs no document edit and no history row; an abandoned run is
  cancellation and is swept. Bounds are derived, never literals:
  `GENERATION3D_IMPORT_CHUNK_BYTES = PUBLIC_INVOCATION_STRING_BYTES` (the cap
  `validate_public_json_envelope` applies **before** any tool contract is consulted, so no contract
  can widen it) and `GENERATION3D_IMPORT_TOTAL_BYTES = GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES`
  (an import is planted as an `InputNote`'s text, so the run must fit one Artifact-lane edit).
- **Outbound**: an export reads the **retained** `FlowEvalSession`'s already-tessellated preview
  (`export_mesh_from_session`, `✏️editor/🦀️.rs:2813`) and evaluates nothing. The expensive work is the
  existing `flowEvalTick` chain with its existing status pill and `cancelPreviewEval` button. This is
  not a preference — see §7.2, it is a measured defect fix.

### 3.4 Its own route

`GENERATION3D_DOCUMENT_IO_TOOL_IDS` (`✏️editor/🦀️.rs:311`) with its own factory, contract and proofs
(`:878`, `:885`, `:953`, `:1016`) — the same reason the preview chain has one: an import chunk is
nothing like an 8 KiB gesture, and widening the gesture quota would widen 25 unrelated routes.

### 3.5 Viewer: export only

`GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS` (`👁️viewer/🦀️.rs:230`) owns `exportDocument` alone, on
`HostOnly` — which is *why* a read-only surface may have it. Import replaces a document and stays on
the editor; `no_viewer_tool_publishes_on_the_artifact_lane` now covers the io route too.

## 4. Files

| File | What |
|---|---|
| `…/✳️any/🚪️io/🦀️.rs:168-390` | `document_io`: roster, descriptors, accept filter, download envelope, export/import entry points |
| `…/✳️any/✏️editor/🎮️commands/📂️import-document-request/🦀️.rs` | picker verb, `RequestFileOpen` with the derived accept filter |
| `…/✳️any/✏️editor/🎮️commands/📥️import-document/🦀️.rs` | chunk envelope, instance-scoped staging ledger, event-sourced document replace |
| `…/✳️any/✏️editor/🎮️commands/📤️export-document/🦀️.rs` | export verb + `retained_preview` |
| `…/✳️any/✏️editor/🦀️.rs` | enum rows `:94-96`, route `:311`/`:878`-`:1016`, actions `:2203-2204`, args, chords `:2392-2393`, context menu `:2143`, `export_mesh_from_session` `:2813` |
| `…/✳️any/👁️viewer/🦀️.rs` | route `:230`/`:933`, action `:1546`, chord, **fix-forward** `:1545` |
| `…/✳️any/👁️viewer/🎮️commands/📤️export-document/🦀️.rs` | viewer export + `retained_preview` |
| `…/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs` | `config_after_example_load` → `pub config_after_document_load`, shared with import |
| `…/✳️any/🧫️fixtures/🚪️io/📄️document-surface.json` | **the language-agnostic law** |
| `…/✳️any/🚪️io/🧪️tests/📄️document-surface/🦀️.rs` | Rust laws (mounted into the `io-round-trip` lane) |
| `…/✳️any/🚪️io/🧪️tests/📄️document-surface/🟦️.ts` | TypeScript twin |
| `…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `📄️DocumentIoSurface` laws + roster fix-forward |
| `…/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | viewer export/no-import law, io route in the four-table and artifact-lane laws |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/…/🚪️io/🦀️.rs:152-161` | **fix**: checked arithmetic in `decode_stl_auto` (§7.1) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/…/🚪️io/🧪️tests/🔬️unit/🦀️.rs` | law for it |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📜️artifact-definition.json` | **fix**: `s.stdio.txt` declared no representation capability at all, so it had no format descriptor anywhere in the repo |
| `📜️script.ts`, `📋️project.json`, `.vscode/launch.json` | `generation3d-document-io` + `-native` lane |
| `T/🐍️io-surface-probe.mjs` | the browser proof |

## 5. Laws (all run in the foreground)

| Lane | Result |
|---|---|
| `cargo test -p semio-s-artifact-procedural-generation3d --test io-round-trip` | **45 passed** (9 codec lanes + 11 new document-surface laws) |
| `… --features component-app-assembly --lib` (document-io + roster + viewer) | **14 passed** |
| `cargo test -p semio-s-artifact-stdio-stl` | **43 passed** (+1 new overflow law) |
| `cargo test -p semio-s-artifact-stdio-txt` | **54 passed** |
| `cargo test -p semio-s-plugin-stdio` | **6 + 6 passed** (catalogue/collision gates, after the txt capability) |
| `bun nx run workspace:generation3d-document-io` | green — TS twin + `tsc --noEmit --strict` |

Raw output: `T/🗑️generated/io-surface/all-laws.txt`.

The **third-party oracle** is the one this surface's sibling already uses: every geometry format's
export goes back in through its own import leaf and is handed to `parry3d`'s
`MassProperties::from_trimesh`, which must agree on volume 1.0 and `[0,0,0]..[1,1,1]` for the
committed unit cube (`every_geometry_export_format_moves_the_committed_cube_past_the_oracle`). LAS is
a point cloud and is asserted on positions instead.

The **TypeScript twin** is independent where it counts: where Rust asks `document_io` for a format's
extension/MIME/binary-ness, the twin reads each `s.stdio.*` artifact's own
`📜️artifact-definition.json` off disk. It earned its keep immediately — it caught that the fixture's
`retransmission-is-acknowledged` case collided with the restart-at-zero rule, which the Rust half
would have silently agreed with. The fixture now states both rules and both halves drive them.

## 6. Runtime proof (6018, React, restaged 18:20)

`T/🐍️io-surface-probe.mjs`; evidence in `T/🗑️generated/io-surface/` (screenshots, `console.txt`,
`results.json`, the downloaded files).

| Step | Result |
|---|---|
| boot | converged, 3 meshes, `phase: idle` |
| published | `#action.importDocumentRequest` → "Import Document…", `#action.exportDocument` → "Export Document…" |
| export | `#format` combobox seeded with **"STL Mesh"** (this artifact's own label + default); Execute → download `generation3d.stl`, 5 080 B, real ASCII STL of the evaluated preview |
| export (`txt`) | `generation3d.txt`, 1 572 B, the document's own DSL text |
| import | picker opened with `accept=.stl,.obj,.ply,.gltf,.dwg,.json,.txt`, `importAction=importDocument`, `req=131`; after choosing a real `.stl` the graph became `["imported-source","imported-geometry","imported-preview"]` |

## 7. Defects found by running it

### 7.1 FIXED — `s.stdio.stl`'s auto-detect panics the guest on ordinary ASCII STL

`decode_stl_auto` disambiguated a `solid`-prefixed file with `84 + count * 50`, where `count` is four
arbitrary bytes of ASCII text. `usize` is **32 bits on wasm32**, so that multiply overflows and the
arithmetic panic takes the whole plugin actor down. Measured live: a 209-byte hand-written ASCII
triangle trapped the instance with `attempt to multiply with overflow` mid-import. Now checked
(`…/🔺️stl/…/🚪️io/🦀️.rs:159`), with a law that is meaningful on both targets. This is squarely the
"native cargo misses wasm-gated code" class — no native test could have seen it.

### 7.2 FIXED — geometry export re-evaluated instead of reading the retained session

`export_mesh_from_document` builds a **fresh** `FlowHost` and calls `host.evaluate()` synchronously.
In a guest the brep/math operators are host-contributed and reached only through the asynchronous
extension chain, so that evaluation resolves nothing: "Export Document" on a fully painted 3-mesh
preview faulted with `no preview geometry (no positions)`. Both surfaces now read the retained
session (`export_mesh_from_session`, `export_document_with_preview`); the in-process fallback stays
for the native lanes, which really can evaluate.

### 7.3 OPEN, NOT MINE — `Effect::DownloadMediaExport.encoding` is dropped on the guest→shell hop

**Every plugin with a binary export is affected**, including `process3d`'s `glb`.

Measured at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5415`
with a temporary `[DEBUG]` line (since removed): for a `dwg` export the guest sends
`encoding: Some("base64")` and the shell receives `raw=undefined type=undefined`, while `filename`,
`mimeType` and `data` all arrive intact. `mediaExportEncodingText` therefore answers `undefined`,
`downloadMediaExport` skips its `atob`, and the user saves a file whose bytes are base64 **text**
under a binary name: `generation3d.dwg` began `QUMxMDE1…` (= `AC1015…`). `txt`, which declares no
encoding, is byte-correct — so the non-encoded path is fine and only the `Option` is lost. The WIT
declares `encoding: option<string>` (`🔌️plugin/🧬️schema/📜️.wit:433`) and the jco `.d.ts` renders it
`encoding?: string`, so the loss is in the effect hop, not the contract.

Not fixed here: it is a framework-wide effect-wire defect with its own blast radius and needs its own
lane and its own laws. It does not affect the textual formats (`txt`/`obj`/`ply`/`gltf`), which are
the ones this surface's laws drive.

### 7.4 OPEN, NOT MINE — the host slices imports at 32 KiB, the guest envelope admits ~4 KiB

`IMPORT_CHUNK_BYTES = GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2` (32 KiB,
`🛠️ShellHelpers/🟦️.tsx:785`), but `validate_public_json_envelope`
(`🔌️plugin/🦀️.rs:33383`) refuses any invocation string above `PUBLIC_INVOCATION_STRING_BYTES`
(4 KiB) **before** the addressed tool's contract is read. So a picked file above ~4 KiB is refused by
the host envelope for **any** plugin — `puzzle3d`'s own chunked import included. This lane's guest
bound is derived from the bound that actually binds, so generation3d is correct on its side; the
owning-layer fix is the shell constant, tied to `puzzle3d`'s by an engine-contract law, which is why
it is not changed here. The browser proof therefore used a single-chunk file.

## 8. Fix-forward on peers' work (noted, minimal)

- `👁️viewer/🦀️.rs:1545`: a peer added `cancelPreviewEval` to the preview window's
  `window_kind_action_refs` while it was declared only as a `CommandDefinition`, so
  `build_definition` rejected the **whole viewer** and every app-fixture law in the crate died at
  construction. Fixed with `.view_action(…)` — the identical fix the sibling editor already carries
  for the identical mistake.
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs`: the peer's new `cycleShowMode`/`cycleLodMode` rows reached
  `every_command()` but not the wire-keyword roster. Two keywords added in enum order.

## 9. Follow-ups

1. §7.3 — the `encoding` wire drop. Highest value: it silently corrupts every binary export in the repo.
2. §7.4 — the import chunk-size mismatch; needs the shell constant and `puzzle3d`'s twin moved together.
3. `s.stdio.stl@ascii` declares `is_binary: true` for a representation whose only encoder is
   `encode_stl_ascii` (text). Truthful value is `false`; left alone because `is_binary` has other
   consumers and changing it while §7.3 is open would mask rather than fix.
4. The chunk-staging ledger here and `puzzle3d`'s are the same idea twice. Once §7.4 forces the shell
   half to move, lift the run/gap/capacity ledger into the framework beside its TS twin and have both
   plugins use it.
