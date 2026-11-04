# Batch H (T14): zip, deflate, binary, semio

## Stdio reference editor pattern (zip), verified by reading

Surfaces (host contract in 🖥️host.md): `s.stdio.zip@2.0/*#editor` (+ `iso21320`), `s.stdio.deflate@rfc1950/*#editor`, `s.stdio.binary@raw/*#editor`, `s.stdio.semio@v1/{*,animation,...,video}#editor` are all published (manifest ids verified) by `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/{🔢️binary,🧿️semio}/🧩️component/🔣️.json` (binary family carries binary, deflate, zip, zip iso21320).

Zip surface layers (artifact root `🎒️zip`):
- Snapshot `ZipSnapshot { entries: [ZipEntry{name,data,metadata}], comment, commentUtf8 }`; metadata holds compression method, full local/central header fields (timestamps, attrs, extra fields, descriptor flags), so unedited entries re-encode byte-faithfully.
- Mutations (`🧬️schema/🧬️mutations`): set-archive-comment, add-entry(before), remove-entry, rename-entry, set-entry-data. Each is a leaf with sparse diff + concrete inverse (history/undo).
- Edit rules (`🧭️edit-rules`): Details-tree pointer edits (`/entries/*/name`, `/entries/*/data`, `/comment`, remove `/entries/*`, insert via `snapshot_edit_special`) resolve to those mutations.
- Native command `set-node` (comment / entry rename draft) with `text_revision` BLAKE3 draft-conflict guard, retained cursor `🧵️retained` (96-byte checkpoint `ZAT1`) for bounded, resumable, cancellable work.
- Codec in `🚪️io` (decode/encode, CRC, CP437/UTF-8, zip64 read, data descriptors); `📦️opc` builds OPC (docx/xlsx/pptx/idml/bcf containers) on the same snapshot.
- Reusable for batches B/F: container artifacts reuse `ZipSnapshot`/`encode_zip`/`decode_zip` and the zip mutations; the unsafe-name guard (below) applies to every container.

## Changes (phase 2, zip)
- New `ZipError::UnsafeEntryName` + `check_entry_name` (rejects `..` segments, absolute, drive-qualified, backslash, NUL) enforced on decode and encode: `🚪️io/🧮️byte-primitives-and-text/🦀️.rs`, `🔍️inspect-decode-and-sniff`, `📮️encode`.
- Editor refuses unsafe names before history: `require_safe_names` in `🎒️zip/✏️editor/🦀️.rs`, wired into `edit_node` and the snapshot-edit path of base and iso21320 surfaces (`stdio.zip.name-unsafe`).
- Language-agnostic fixture `🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/🛡️unsafe-names/🔣️.json`; tests: codec test cross-checked with the third-party `zip` crate (`enclosed_name`), corrupt/truncated archive test, editor test for rename/insert.

## Natural-file codecs added (open/save real files from the app)
- zip: `.zip`, `application/zip`, format_kind `s.stdio.zip@2.0` via `encode_zip`/`decode_zip` in `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs` (iso21320 surface not given a codec: same extension would collide; open resolves to the base surface).
- deflate: `.zz`, `application/zlib`, `s.stdio.deflate@rfc1950` via `encode_deflate_snapshot`/`decode_deflate_snapshot` in `🗜️deflate/.../✳️any/✏️editor/🦀️.rs`.
- binary: `.bin`, `application/octet-stream`, `s.stdio.binary@raw`, identity bytes, in `💾️binary/.../✏️editor/🦀️.rs`.
- Fixture rows + count (33) in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/{🧫️fixtures/📄️natural-file-lifecycle/🔣️.json,📄️natural-file-lifecycle/🦀️.rs}`.
- Tests added (not yet executed, see below): zip natural round trip + edit + third-party `zip` reader; deflate cross-check with `miniz_oxide` (incl. foreign stream, corrupt adler, truncation); binary byte-exact round trip.

## Open (not done)
- Native receipts for binary and semio, and zip/deflate definition `mutations: []` honesty: need the generator run through T06 (`native-codec-projection`, `composition-prepare`).
- semio: 19 subset editors share extension `.semio`; each subset has its own sniffer in `🚪️io/🦀️.rs`. A natural-file codec needs one registry-level dispatch by sniff (T01/T04 envelope version fix), not 19 colliding `NaturalFileCodec`s. Per-subset meaningful edit paths (about 15 read-only) not started.
- deflate docs name nonexistent `replace-text` action: not fixed.

## Transcribed evidence (T20, before the `🗑️generated` deletion)

Full text in `✅️validation.md` E5 and E6.
- `h-zip-editor-test.log`: `cargo test -p semio-s-artifact-stdio-zip-editor …` from the repo root; exit status 1; `error: package ID specification `semio-s-artifact-stdio-zip-editor` did not match any packages`. Failure (root selection). No pass/fail counts.
- `h-zip-editor-test2.log`: capacity-wait lines only; no result.
- Chain `🧪️batch-h-chain.sh`: the zip, deflate, binary and semio runs write to `🗑️generated/h-*.log`; no such log exists in `🗑️generated` at this snapshot, so those runs have not produced output.
- Contract (offline) for zip, deflate, binary, semio: passing in E1 (`✅️validation.md`).
