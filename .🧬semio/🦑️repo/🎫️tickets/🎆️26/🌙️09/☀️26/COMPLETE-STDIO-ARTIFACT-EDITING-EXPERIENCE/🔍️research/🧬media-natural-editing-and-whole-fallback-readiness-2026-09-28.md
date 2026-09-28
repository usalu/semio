# Media Natural Editing And Whole-Fallback Readiness

## WAV action and interaction checkpoint

The WAV natural editor has eight schema-native commands: set sample; append, insert, and remove frame; append, insert, and remove channel; and set sample rate. The current source gives each command exactly one action definition, tool-roster entry, retained factory key, artifact publication contract, command dispatcher branch, and rendered control whose argument map carries the current canonical revision. The authored unit law `every_audio_command_has_one_definition_and_a_revision_bound_surface_control` enforces the definition/control/revision join.

The rendered source wiring uses the host's native controls. Sample and sample-rate values are native inputs, and append/insert/remove operations are native buttons. Those component types support ordinary focus and input-commit or button Enter/Space semantics in the host, while the authored bindings already contain the revision. This slice has not completed browser/native runtime acceptance, so it does not claim that those controls dispatched successfully in a running WAV editor. All eight revision-bearing definitions remain absent from the raw command palette.

The six provisional app-global chords were not valid selected-row shortcuts. The WGPU host's `dispatch_app_keybinding` opens the staged action form whenever an action has arguments; it does not inherit the focused node's argument map. The WAV structural actions require the internal revision and, for insert/remove, a frame or channel coordinate. Those chords would therefore expose revision entry and could not address the selected row. They have been removed, and the window-definition law now requires every WAV action to have no global chord. Direct selected-frame/channel shortcuts remain a concrete host-selection integration gap; ordinary keyboard operation of the visible controls remains available.

The bounded execution envelope is explicit. Each data mutation carries at most 16,384 payload bytes, frame insertion/removal can span several patches at one stable logical index, a channel rewrite refuses any layout whose single frame exceeds that envelope, and the complete plan admits at most 128 mutations. A channel rewrite therefore admits at most 127 data pages plus its coherent `SetFmt`. `fmt.ext` is capped by the WAV schema at 65,535 bytes and copied through `RetainedBytesCopy` in 16,384-byte pages. Wide-file rendering is complete: up to eight channels use the frame matrix, and wider layouts use a windowed frame/channel/value coordinate projection plus independently windowed frame and channel control lists.

Focused TypeScript validation passed with 9 tests and 31 assertions. The focused native attempt stopped before compiling WAV because the shared `semio-framework-os-kernel` dependency currently has 30 `RetainedClone` trait/close errors; `🗑️generated/wav-native-current-8.log` is the exact captured result. The final chord-removal and complete action-roster Rust laws are therefore authored but unrun.

## Highest-value remaining natural media work

1. **WAV selection and transport.** Every sample and structural coordinate is reachable, but the editor has no persisted or ephemeral sample-range selection, range paste, gain/fade operation, waveform overview, playback position, or selected-row shortcut dispatch. Raw payloads intentionally remain read-only because their sample layout is not known. Oversized channel transformations are truthfully refused at the retained mutation ceiling rather than performed incrementally across several atomic publications.
2. **MP3, MP4, and AVI timelines.** All three primary windows still render `MediaWindowKit` with `duration_ms = 0` and `position_ms = 0`. MP3 and AVI still route reachable Details edits through whole `SetSnapshot`; MP4 has a compact `PatchSnapshot` fallback and several semantic mutations, but its primary surface still has no track/sample timeline, media-derived duration, transport state, selection, or structural track controls.
3. **GIF animation.** GIF 87a and 89a render a static `ImageWindowKit` preview and both still publish whole snapshots for reachable Details edits. The natural surface has no frame list, delay/disposal controls, frame insertion/removal/reorder, palette editing, animation transport, or bounded frame-payload replacement.
4. **Still-image canvases.** BMP, JPEG, and TIFF primary surfaces remain static image previews. JPEG document and TIFF document have compact Details fallbacks, while their baseline subsets and BMP remain on whole snapshots. PNG is the current natural region-edit pilot; the other image roots still need coordinate/region controls, zoom/pan, palette or tag controls where modeled, and bounded payload publication with native reopen oracles.
5. **Semio media subsets.** The 19-root Semio group includes audio, video, image, drawing, animation, presentation, and spatial models. Their uncovered Details paths remain in the whole-snapshot ledger. They need domain-specific canvases only after each aggregate can publish uncovered scalar and structural paths without retaining the complete document.

## Current 49-root whole-snapshot ledger

The scoped inventory contains 54 media, image, spatial, CAD, and Semio editor roots. The ticket's source-derived `🗑️generated/media-snapshot-edit-routes.tsv` contains exactly 54 rows. Five roots now use the compact native `PatchSnapshot` fallback: PNG 1.2 any, JPEG JFIF document, TIFF 6.0 document, MP4 ISOBMFF any, and WAV RIFF PCM any. The other 49 roots retain a reachable full-snapshot path. JPEG baseline and TIFF baseline first select semantic mutations for recognized paths and fall back to `SetSnapshot`; the other 47 use the full-snapshot route directly.

| Family | Whole-snapshot roots | Migration unit |
| --- | ---: | --- |
| Semio v1 | 19 | One leaf per distinct mutation aggregate and concrete-dialect editor wiring |
| STEP AP214 | 7 | One shared STEP leaf, seven concrete editor routes and dialect proofs |
| IFC | 5 | IFC4 leaf plus shared IFC2x3 leaf, five concrete dialect proofs |
| SVG 1.1 | 3 | Base/basic/tiny aggregate and derived-dialect coverage |
| DWG | 2 | Version-specific AC1018 and AC1024 aggregate coverage |
| GIF | 2 | Shared GIF mutation coverage, two format roots |
| AVI | 1 | AVI compact leaf plus retained large-chunk proof |
| BMP | 1 | BMP compact leaf plus oversized bitmap sibling proof |
| DXF | 1 | DXF header leaf and tagged-value proof |
| glTF | 1 | glTF leaf plus external/binary buffer sibling proof |
| JPEG baseline | 1 | Baseline aggregate leaf; keep existing semantic header mutations first |
| LAS | 1 | LAS leaf plus point-data sibling proof |
| MP3 | 1 | MP3 leaf plus compressed-frame/tag sibling proof |
| OBJ | 1 | OBJ leaf plus geometry sibling proof |
| PLY | 1 | PLY leaf plus vertex/face sibling proof |
| STL | 1 | STL leaf plus facet sibling proof |
| TIFF baseline | 1 | Baseline aggregate leaf; keep existing tag mutations first |
| **Total** | **49** | **38 remaining aggregate wrappers across 49 concrete roots** |

The 38-wrapper figure follows the established 43-aggregate inventory minus the five mounted pilots. Shared aggregates still require one protocol tag and codec leaf, while every concrete standard/subset editor retains its own controller identity, dialect validation, and reopen/undo proof.

## Migration-ready execution order

1. Convert JPEG baseline and TIFF baseline first. Their document siblings already exercise the shared patch carrier and native codecs, while the baseline mutation aggregates still need independent leaves and must preserve their existing semantic-first dispatch.
2. Convert MP3, AVI, GIF, and BMP next. These are the highest-value natural media roots still retaining complete compressed audio/video/frame/bitmap owners for small Details edits. Each needs one neutral fixture with a payload larger than the store item limit, exact forward/inverse codec bounds, cancel-before-publication, undo/redo, and an independent native reopen oracle.
3. Convert glTF, OBJ, PLY, STL, LAS, DXF, and DWG with storage-shape-specific large-sibling fixtures. A compact scalar patch is insufficient evidence for buffer, vertex, facet, point, tagged-value, or opaque-section owners.
4. Convert SVG, IFC, STEP, and the 19 Semio roots by shared aggregate, then execute one concrete-dialect test per editor root. Derived roots must not inherit a base controller identity or skip their own schema validation.
5. Remove whole-snapshot admission only after the registry proves that every reachable Details operation resolves to a semantic leaf, compact `PatchSnapshot`, or explicitly size-refused whole-source operation. Large subtree/source replacement still needs the separate atomic multi-patch retained plan described in `📓️media-compact-patch-rollout.md`.

Native rollout should resume only after the shared retained-clone foundation compiles coherently. The present source inventory is migration-ready, but the current dependency failure means none of the newly authored WAV Rust interaction laws or the repaired ZIP/OPC native laws may be reported as passing.

## Source file ledger for this WAV interaction checkpoint

The natural-audio checkpoint is confined to these WAV-local source groups and the two ticket reports. Earlier WAV serialization/schema and ZIP metadata changes are documented separately in `📓️wav-serialization-state-and-zip-preparation-2026-09-27.md`.

| Path | Responsibility |
| --- | --- |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎮️commands/🔊️edit-audio/🧬️schema/🔣️.json` | Language-neutral command schema |
| `…/edit-audio/🦀️.rs` | Retained planner, validation, mutation paging, inverse-safe close, Rust laws |
| `…/edit-audio/🟦️.ts` | TypeScript command contract and independent mutation model |
| `…/edit-audio/🧪️tests/🟦️.test.ts` | TypeScript fixture/oracle/bounds tests |
| `…/edit-audio/🧫️fixtures/🎚️natural-edit/🔣️.json` | Neutral PCM16/edit/format fixture |
| `…/✏️editor/🦀️.rs` | Command roster, retained factory, publication contracts, routing |
| `…/✏️editor/🟦️.ts` | TypeScript public command exports |
| `…/✏️editor/🎭️modes/✏️edit/🦀️.rs` | Mode/window mount |
| `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs` | Bounded frame/coordinate/channel/format surfaces and localized controls |
| `…/main/🟦️.ts` | TypeScript window declaration |
| `…/main/🧪️tests/🔬️unit/🦀️.rs` | Complete action roster, late-channel projection, control binding, localization laws |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🟦️typescript/📜️script.ts` | Existing Nx test entry extended to the natural-audio test |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/Cargo.toml` | Existing internal test-only oracle dependency wiring |
| `🔍️research/🧬wav-natural-sample-selection-editing-audit-2026-09-28.md` | Capability, bounds, and validation record |
| `🔍️research/🧬media-natural-editing-and-whole-fallback-readiness-2026-09-28.md` | Action audit, remaining media gaps, and 49-root migration ledger |

## Root Follow-Up: Timing and Transport Integration

A narrow source review confirmed duration inference already exists for MP3, MP4, and AVI under each snapshot schema’s `💡️inferences/⏱️duration` leaf. Reimplementing these calculations in each primary window would duplicate authority. MP3 and MP4 currently fold all frames/samples synchronously; AVI reads scalar header fields. MP4’s inference also uses decode duration rather than the retained movie/edit-list presentation timing, so it must be validated against that model before it becomes the transport clock.

`MediaWindowKit` is currently a `KeyValueList` of English-only Duration/Position/Kind fields and explicitly has no playback engine. Its nominal seek action is classified as an artifact mutation even though playback position should be ephemeral local interaction state. Simply replacing zero duration constants would therefore not deliver a usable transport. The next media slice should define a shared localized transport/selection contract and an incremental, revision-bound timing projection, then mount that contract in MP3/MP4/AVI and add visible play/seek/range controls through the host’s media capability. Root has not edited those media production files in this follow-up.
