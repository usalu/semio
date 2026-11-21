# Canonical PDF archive installation

The 17-path receipt in `🗑️generated/archive-install/receipt.json` stages the shared format-1 receiver and the mounted PDF 1.7 Base and VT replay catalogs together. It retains the existing writer and registry. The active browser and native builds still have older capture inputs. The receipt has no apply mode and no product file has been changed.

## Adoption

1. Hold the receipt until the current PDF browser module publishes and the native retest settles. Recheck all current source preimages with `repo tools exec python -- <ticket>/archive-install-audit.py . --verify`. If a source moved, regenerate its insertion against the latest live content and repeat the receipt and closure checks. The previous VT editor candidate was stale after the live `set-font-size` tool appeared in its command roster and bounded tool list. The current receipt preserves both entries.
2. Once the owners coordinate a capture window, install the files under their verified paths in the receipt. The new Pack numeric source and test sources can be placed before their mounts. The aggregate binary mount and the aggregate schema removal of `original_pack_value_operation_receiving!` must land together before any new source capture, because they exchange the trait implementation for the same `PdfMutation`. Recheck postimage SHA-256 values before starting another build.
3. Run the registered base mutation/archive suite, the relocated Pack owner tests, and the actual committed VT archive suite:

   ```sh
   repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-pdf-testkit --test original_mutation_archive
   repo cargo --store=shared --configuration=prototype -- test -p semio-framework-pack-json --test original-json-owner
   repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-pdf-viewer --lib committed_vt_archive_restores_forward_and_inverse_with_registered_owners
   ```

   The first command captures the unchanged aggregate codec tests and the paid receiver tests alongside actual Base fixture apply and inverse. The third command authors and commits a real operation, consumes the resulting Pack and SPR in the registered VT viewer and compares both hydrated mutation directions with their archived bytes. These registered commands remain pending coordinated publication.

## Mounted path

A history-bearing load runs `PluginApp::begin_document_archive_load`, `poll_document_archive_load`, and `acknowledge_document_archive_load`. Existing editor and viewer store initializers route to `EditorDocumentStoreOwnerProducer` and `ViewerDocumentStoreOwnerProducer`. The receipt gives each Base and VT surface its source demand and the matching `cold_replay_document_store_owners::<PdfSnapshot, PdfMutation>` catalog constructor. The editor's existing `snapshot_details_editor_support!` catalog hook supplies admission alongside its routed edit factory. The viewer overrides its two catalog methods and uses the same registered initializer. The constructor installs the semantic replay and retirement issuers from the live `ColdMutationApplyFactory`. The bounded Original binary mutation hydration separately calls the shared `PdfMutation::admit_receiving` for both stored directions. No app registration manifest, wire manifest, new runtime dependency, or sibling framework replacement is needed.

Independent Base inverse verification lives in `archive-leaves-oracle.py --verify`. Pypdf independently opens the six committed before/after PDFs, applies each observed rotation, Info or language edit using its writer, applies the reverse value, and verifies the restored fields. Google CRC32C and Python SHA-256 reproduce each canonical frame. The focused Rust harness recompiles the installed exact receiving module and 17 tests passed. It also runs the actual three leaf inverse expressions against those observed Base values. The separate registered snapshot test stages the normal `decode_pdf`, production `mutation_inverse` and `apply_pdf_mutation` for both directions.

## Captured source overlap

Names in the first column are rooted at the Pack JSON folder or the PDF 1.7 subset indicated. The receipt lists all workspace-relative destinations and current SHA-256 values. Unit numbers below come from the completed source-admission observations of the active runs.

| Existing path in receipt | Native capture | Browser capture |
|---|---|---|
| Pack JSON `🦀️.rs` and `🧮️parser-and-grammar-cursor/🦀️.rs` | Pack JSON units 167 and 170 | Units 228 and 231, exact current source blobs confirmed |
| Pack JSON `📥️decode/🧳️receiving/🦀️.rs` and `📦️packages/🦀️rust/Cargo.toml` | Same Pack JSON source owner and selected manifest | Same source owner and selected manifest, exact receiver blob confirmed |
| Base `🚪️io/💾️binary/🧬️mutations/🦀️.rs` and `🧬️schema/🧬️mutations/🦀️.rs` | Mutation family 8 unit 640 reads both actual roots | Unit 1005, exact current source blobs confirmed |
| Base `🧬️schema/🧪️testkit/📦️packages/🦀️rust/{Cargo.toml,🪞️mirror/🦀️.rs}` | Testkit dependency unit 658 and selected manifest | Unit 1025 and selected manifest, exact mirror blob confirmed |
| Base `✏️editor/🦀️.rs` | Schema 5 unit 653 | Schema 5 unit 1020, current source has moved since capture |
| Base `👁️viewer/🦀️.rs` and VT `✏️editor/🦀️.rs` | Snapshot-value unit 648 mounts both | Unit 1015, exact Base viewer blob and the earlier VT editor blob confirmed |
| VT `👁️viewer/🦀️.rs` | VT viewer is outside this native retest root | Viewer unit 1028, exact current source blob confirmed |
| VT `👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | Test-only outside this native retest root | Test-only outside the WASI library capture |

The new Pack scalar leaf and private-owner test driver, and the new Base archive integration Rust source and JSON corpus, have no captured preimage. They join those closures through the listed roots. The native capture is `pdf-editor-native-gate-media-handoff-retry.log` in lane `p69cdcb`. The browser observation is `pdf-browser-direct-components-factory-repair.log` in generation `wDIXcyY`, compiler lane `pc5147e`.

## Hard source limits

The repository's actual closure reader reports these figures on Linux, macOS, Windows and WASI. All stay within 32 files, 4,000 active lines and 262,144 bytes. The longest normalized absolute path is 185 UTF-16 units.

| Target | Files | Active lines | Bytes |
|---|---:|---:|---:|
| Pack JSON library | 17 | 3,131 | 252,435 |
| Pack JSON unit tests | 19 | 3,146 | 254,999 |
| Pack Original owner integration tests | 8 | 576 | 78,900 |
| PDF mutation family 8 library | 27 | 3,121 | 182,716 |
| PDF mutation family 8 unit tests | 32 | 3,466 | 202,714 |
| Base testkit library | 21 | 1,906 | 143,603 |
| Base testkit unit tests | 32 | 3,661 | 250,817 |
| Base Original archive integration tests | 9 | 832 | 63,954 |
| Base editor owner library | 21 | 3,849 | 251,501 |
| Base editor owner unit tests | 24 | 4,000 | 260,726 |
| VT editor and Base viewer owner library | 10 | 1,370 | 77,682 |
| VT editor and Base viewer owner unit tests | 31 | 3,432 | 211,138 |
| VT viewer library | 2 | 189 | 8,954 |
| VT viewer unit tests | 5 | 409 | 22,464 |

The binary owner keeps the receiver's typed lanes as lexical child modules in one source file. This preserves their production code while leaving the family 8 unit closure at 32 files. The Base aggregate laws move without editing their shared source into the dedicated archive integration target. The Pack test-only target compiles the exact authored Original receiver's source into its private test scope and preserves its existing private assertions. Pack's live test closure was already 266,417 bytes, and keeping those tests in the unit root would have made it 281,807 bytes after the numeric cursor. Pinned Rust parsed ten install candidates. Registration of the relocated Pack tests was source-checked. The focused local cache lacked the `semio-framework-value-testing-admission` test library for independently compiling their executable.

The paid leaf guarantee covers the measured Image Samples and ordinary rotation, Info, language and COS shapes. Complex image metadata, nested or large Info extras, and other mutation tags can still reach cold Bind. Mounted replay, full snapshot application and Pack test target linkage remain to be verified by the registered run after source publication.
