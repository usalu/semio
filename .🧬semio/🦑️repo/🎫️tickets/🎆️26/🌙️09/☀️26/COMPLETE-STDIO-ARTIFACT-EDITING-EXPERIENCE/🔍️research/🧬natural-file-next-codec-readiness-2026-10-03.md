# Natural File Next Codec Readiness — 2026-10-03

This is a source audit of the next four natural-file mounts. It does not claim runtime acceptance. Native execution remains queued behind the coordinated CSV and PNG jobs.

| Family | Editors | Existing natural codec mount | Authoritative codec | Full-snapshot publication | Readiness and required witness |
|---|---:|---|---|---|---|
| BMP v3 | 1 | Mounted in current source | `decode_bmp` / `encode_bmp` in the v3 any-subset IO | `SetSnapshot` exists and is registered | Exact metadata, canonical byte preservation, independent image decode/RGBA projection, and fresh `SetSnapshot` witness are authored. Native execution is queued. |
| TIFF 6.0 | 2 (document, baseline) | None | Document IO exposes `decode_tiff`, `encode_tiff`, and controlled variants | `SetSnapshot` exists | Codec exists, but the baseline editor needs explicit subset admission rather than blindly sharing the document codec. Existing tests establish canonical regeneration rather than input-byte identity. Office owns current TIFF UI work, so this lane made no source edit. |
| PDF 1.4 / 1.7 | 10 | 1.7 base neutral capability and native witness staged by root | Separate 1.4 and 1.7 `decode_pdf` / `encode_pdf` implementations; 1.7 also has password-aware decode and encode options | Snapshot mutations exist across profiles | Root owns the production 1.7 base mount and independent `lopdf` validation. The other declared profiles still need exact gates, including typed password/encryption behavior. |
| SVG 1.1 | 3 (base, basic, tiny) | Mounted in current source | `SvgSnapshot::import_utf8` / `export_utf8`, backed by `parse_svg_xml` / `write_svg_xml` | `SetSnapshot` exists | Base canonical XML serialization and Basic/Tiny hard conformance checks now gate both import and save. Independent quick-XML reopen and fresh `SetSnapshot` witnesses are authored; native execution is queued. |

## Evidence boundaries

- A codec's existence does not make its editor action available: the plugin declaration must mount paired import and export methods with exact extension and media type.
- Regenerating codecs may preserve typed semantics while changing bytes. Their acceptance laws must compare semantic axes through independent readers.
- Open must create a fresh owner, reject invalid/subset-incompatible input before publication, expose progress/cancellation, and retire the candidate owner on failure.
- Save must serialize the selected document captured before asynchronous transfer; later selection changes must not redirect output.
- The browser picker now reads bounded raw `Blob` slices with cancellation and progress, settles chooser cancellation, and retires failed candidate owners. Browser acceptance still requires each newly mounted codec's independent native witness and an end-to-end Open/Save run.
