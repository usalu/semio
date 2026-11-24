# PDF value ownership: sqlite snapshot encode/decode and test callers

Builds go through `sh native-cargo.sh` (rustup nightly fallback, 2 lanes). `bun` for the sqlite oracles: `~/Library/Caches/semio/runtimes/bun/<hash>/`.

## Counts (`semio-s-artifact-stdio-pdf --lib`)

| | pass | fail |
|---|---|---|
| before | 22 | 16 |
| after the compile fixes (derives), stale stdio helpers | 30 | 8 |
| after `with_encoding`/`with_decoding` delegation | 33 | 5 |
| after `io_run`/`with_sqlite_recipient` delegation | 38 | 0 |

## Contract of the framework (current)

The value encoder and decoder no longer own an allocation or construction payer of their own. The *original caller* lends two authorities to every native control for the whole run:
- the Work recipient (`OriginalWorkAuthority`, `with_work_recipient` / `install_work_recipient`),
- the physical construction recipient (`OriginalConstructionAuthority`, `with_construction_recipient` / `install_construction_recipient`).

A control without them fails with `native encoder requires its original construction recipient` (encode) or `native construction requires its original recipient` (decode). The old texts `forwarded native storage requires its quoted allocation effect` no longer exist; `charge()` now admits through `admit()` / `admit_unquoted` on the forwarded port.

The framework's own owners helper crate `semio-framework-test-original-snapshot-owners` (`🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners`) is the one place that lends both authorities: `with_decoding`, `with_encoding`, `io_run`, `with_sqlite_recipient`, `export_sqlite`, `import_sqlite`, `encode_native`, `decode_native`.

## Root cause

`semio-s-artifact-stdio-contract-testing` kept its own copies of `with_decoding`, `with_encoding`, `with_encoding_async` and `io_run`. They installed only the retirement recipient, never the Work and construction loans, so every sqlite snapshot test that went through them (XLSX included, which only passed while the framework did not demand the loans) failed in the value layer. The PDF snapshot code (`encode_sqlite_snapshot_native` / `decode_sqlite_snapshot_native`) was correct.

## Fix

- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧰️testing`: `🧲️native-owners` and `🔀️routed-io` are now re-exports of the framework helpers (no stale variant remains); `export_sqlite` and `import_sqlite` are re-exported too; `📦️packages/🦀️rust/Cargo.toml` depends on the framework helper crate.
- Framework helper crate: new `with_encoding_async`. The loan API takes synchronous closures, so the future is polled to completion on the calling thread (no-op waker, `yield_now`) inside the loans. Exercised by the page-editor tests (`with_encoding_async` callers in the 1.7 editor tests).
- PDF 1.7 schema: `NativeReceive` derive on `PdfGraphIdentity`, `PdfGraphPath`, `PdfStreamRoleValue`, `PdfAdmittedStreamRole` (stream roles), `PdfPathSegment`, `PdfPageBox`. `PdfIndexedDiff<T>` stays with the history agent: the derive refuses generic records.
- XLSX sqlite laws: test callers moved to `import_sqlite` / `export_sqlite` / `encode_native` / `decode_native`; `XlsxSnapshot` and `XlsxWorkbook` imports; `serde_json` dev-dependency; XLSX `demo_diff_cases`, `snapshot_a`, `snapshot_b` exposed as `pub` (as BCF and STL do) and the `cfg(test)` imports they need dropped, so the mirrored integration targets see them.

## Page-editor tests (`semio-s-artifact-stdio-pdf-page-editor-tests`)

65 of 65 pass (the target had 62 at the time of the `codec-dct-ycck` fix; the JPX smoothing and German boundary-box tests are included). Before: 61 of 62, failing `paint_fidelity::canvas_paint_matches_mupdf_probes`, case `codec-dct-ycck`.

Root cause (found with the repo's own oracle recipe, `uv run --offline --no-project --with pymupdf==1.26.7 python -I`, no download): the probes of `codec-dct-ycck` and `codec-dct-ycck-inverted` reproduce exactly. Two independent differences from the previous preview code:
- MuPDF applies neither the Adobe YCCK transform nor `/ColorTransform` to a four-component DCT stream; its CMYK pixmap at (1,1) is the raw decoded planes (149,152,169,243), and the `/Decode [1 0 ...]` variant is 255 minus them.
- MuPDF converts DeviceCMYK image samples through its default CMYK profile, not through the 16-corner interpolation it uses for fill colours (magenta 128: image 244,154,193 against corners 245,127,197).

Fix: `🔆️image-preview` keeps four-component DCT planes as stored; `🧮️function-colour` gained `image_rgb`, which interpolates a 17-node grid sampled from MuPDF 1.26.7 (`🌈️cmyk-image-clut.bin`, generated and checked by `📜️cmyk-image-clut.py`, 250 KB; maximum error against MuPDF on 3,000 random points 1.9 per channel). Fill colours still use the corner table. It is not the new smoothing: the 16x12 image is upscaled exactly 4.0x and `SMOOTH_UPSCALE_LIMIT` is exclusive.

## XLSX (`semio-s-artifact-stdio-xlsx`)

| target | pass | fail |
|---|---|---|
| `model_laws` | 28 | 0 |
| `sqlite_laws` | 19 | 1 (`..._deep_erased_native_input_output_are_interior_cancellable`, skipped in the counts above) |

Both targets did not compile before: the sqlite test callers used the old `codec.import` / `codec.export` / `decode_sqlite_snapshot_native` arities and `RetainedWorkOutcome` unwraps; the model-laws mirrors needed `pub` demo cases and the `ColdDiffAlgebra` / `DiffAlgebra` imports; one test needed the XML snapshot schema in its resolver.

### The deep-XML overflow at 262144 bytes

Not fixable by making a pass iterative; measured, not guessed:
- The test passes at 4 MiB and 16 MiB and overflows at 256 KiB and 1 MiB.
- Stack address at the XML emitter entry is identical for the Binary and Text encodings and flat while 8,192 nodes are popped (the emitter uses an explicit pending stack). Reconstruction and retirement of the 8,192-deep tree are fine at 256 KiB.
- The cost is before the emitter and fixed: thread start to the plain `encode_native` emitter is about 780 KB; through `import_sqlite` about 1.8 MB. Of the 780 KB: 49 KB in the owner helper, 57 KB in `encode_typed`'s own frame, about 650 KB between `encode_typed` entry and the receive closure.
- A backtrace at that closure has 50 frames in total and no frame repeats with depth. The chain is `encode_typed` -> `NativeSnapshotEncodeOwner::scoped_native` -> `scoped_maximum` -> `scoped_observer` -> `snapshot::receive` (several generic instances plus `catch_unwind` wrappers) -> `with_retirement_owner`. A disassembly scan finds no single frame above about 20 KB: it is some 25 generic frames of 7 to 20 KB each in a debug build.
- Consequence: a 256 KiB stack cannot hold this owner chain in a debug build regardless of document depth. It needs either an optimised build, or fewer or smaller frames in `receive` / `encode_typed` / `scoped_*`. Not changed; the fixture stays 262144.

Counts after all changes: PDF lib 38 / 0, page-editor 65 / 0, XLSX 28 + 19 pass, 1 deep test known failing for the reason above.
