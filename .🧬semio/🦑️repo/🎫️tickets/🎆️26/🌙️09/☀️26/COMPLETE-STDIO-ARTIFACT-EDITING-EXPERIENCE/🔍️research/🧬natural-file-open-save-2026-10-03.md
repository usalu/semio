# Natural File Open and Save — 2026-10-03

## Mounted route

The editor contract exposes exact `NaturalFileCodec` metadata and only publishes Open/Save when an editor supplies both a natural encoder and decoder. `PluginBuilder::editor_app` mounts the paired actions in the production composition path. The React shell saves from the focused owner and opens into a fresh owner; a canceled or failed import retires that new owner and preserves the existing document.

The browser ABI uses `artifact:native` with a binary descriptor whose `format_kind` equals the mounted codec. On import, `MediaArtifact.data` moves directly into `MediaPayload::Intrinsic { schema, value: DslValue::Bytes(data) }`. The artifact decoder borrows those owned bytes after an exact schema check. There is no document-pack substitution, base64 conversion, or JSON wrapper in this natural-file path.

## Current true codecs

The current source mounts 30 editors represented by 28 neutral capability rows across 19 artifact families: CSV, PNG, BMP, WAV, JPEG document/baseline, MP3, MP4, AVI, PDF 1.7 base, DOCX base/strict/transitional, Markdown, HTML, SVG base/basic/tiny, TSV, TXT, XML base/valid, JSON base/I-JSON, XLSX base/strict/transitional, and PPTX base/strict/transitional. Every mounted entry names its exact extension and media type and supplies both a natural encoder and decoder. Their Open path produces one event-sourced whole-document mutation.

The full inventory and remaining codec mounts are tracked in `🧬natural-file-open-save-coverage-2026-10-03.md`.

## Evidence

- `🗑️generated/natural-file-raw-picker-contract-3.log`: renderer neutral capability, isolated-owner, bounded browser read, progress, cancellation, read-failure, picker-cancel cleanup, and retry law, 1 passed with 699 excluded and Nx cache disabled.
- `🗑️generated/natural-file-raw-picker-typecheck-2.log`: fresh renderer aggregate typecheck after the raw picker mount.
- `🗑️generated/natural-file-plugin-native-4.log`: schema-first codec metadata, binary descriptor, and intrinsic-octet transport law, 1 passed with Nx cache disabled.
- `🗑️generated/natural-file-csv-native-4.log`: CSV edited bytes, independent parse, fresh-owner reopen, and one-mutation history law, 1 passed.
- `🗑️generated/natural-file-schema-typescript-7.log`: the DOCX retained public-facet TypeScript stage passed; the run then stopped at a concurrently repaired PPTX schema-closure registration. Root's later `pptx-canonical-schema-aggregate-1.log` is the fresh 36-artifact aggregate green receipt.
- `🗑️generated/natural-file-intrinsic-editor-route-contract-1.log`: the schema/renderer capability law, raw picker lifecycle, and exact registered-editor intrinsic-byte witness passed 1/1 with Nx cache disabled at the preceding 22-row checkpoint. A fresh fixture law is required for the current 28-row matrix.

## Remaining limits

The natural Open picker no longer creates a browser base64 string. It selects the raw `File`, admits a maximum 512 MiB contiguous owner, reads at most 256 KiB through one abortable `FileReader` turn, and reports exact byte progress. Picker cancel always settles and detaches the transient input handlers; read cancellation and failure leave the focused owner untouched. The exact focused owner, app, codec, and importer are captured before the picker await, so a selection change cannot redirect the import.

The native codec boundary and Save path still materialize a whole contiguous byte buffer. Segmented native decode/encode and a streaming download sink remain the scalability frontier. Browser acceptance still needs a real selected file, Tasks-window progress/cancel observation, fresh-owner publication, exported-byte inspection, and reopen. Format-specific native laws beyond current CSV remain in the coordinated serial queue; the exact status is in `🧬natural-file-open-save-coverage-2026-10-03.md`.
