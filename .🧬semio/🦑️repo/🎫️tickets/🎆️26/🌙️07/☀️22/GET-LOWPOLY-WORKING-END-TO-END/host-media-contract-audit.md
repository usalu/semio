# Host Media Contract Audit

Read-only source review, 2026-11-05. No Native, browser, server, or setup execution. Export text delivery is consistent; React custom picker cancellation is a concrete defect.

## Export contract

Lowpoly owner: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️media/🦀️.rs`. All OBJ/PLY/STL serializers yield bytes converted through String::from_utf8, then DownloadMediaExport with encoding None. Names use the sanitized first-object stem (fallback lowpoly), and .obj/.ply/.stl; MIME is model/obj, application/x-ply, model/stl respectively.

Kernel `🧰️framework/🔨️modules/🎠️kernel/🎬️media-export/🟦️.ts` mediaExportBytes and Rust `🎞️media-jobs/🦀️.rs` media_export_bytes interpret absent/utf-8 as raw UTF8, base64 only when explicit. Therefore text beginning with data: is still ordinary export text with None. No extra base64/dataURL decoding occurs.

Renderer prefix below is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

React: ShellHost/host-effects/export-effects applyDownloadMediaExportEffect reads the wire option using mediaExportEncodingText, handles segmented markers separately, then calls ShellHelpers/plumbing/downloads-and-file-requests downloadMediaExport (line20). It kernel-decodes bytes, makes a Blob with exact MIME, mounts an anchor with exact filename, clicks, then removes anchor and revokes object URL after 10,000ms (lines26–39). Regular successful delivery has explicit disposal.

WGPU staging/session/theme-file-import-and-accessibility download_media_export kernel-decodes the identical data/encoding contract. Staging/doc-open/theme-file-import-and-accessibility present_media_export_bytes_cancellable (lines182–195) sends exact bytes+filename+MIME through host_io_call on wasm; native offers a save dialog using the filename extension, checks cancellation before and after selection, then std::fs::write(path, bytes). MIME supplies fallback extension only if filename lacks one. Browser host-io createWgpuPageHostIo routes download-media-export to downloadBytes (line133): copies exact bytes into Blob, mounts anchor, clicks inside try/finally, removes anchor, and schedules URL revocation. No format delivery contradiction found.

## Picker ingress and concrete defect

Lowpoly requests .obj,.ply,.stl with readAs=dataUrl; FileReader.readAsDataURL yields base64 dataURL. opened_file_bytes decodes explicit ;base64 dataURLs; direct raw input yields UTF8. Non-base64 percent-encoded dataURLs are not percent-decoded, but neither actual browser picker creates that representation.

React exported requestFileOpen at ShellHelpers/plumbing/downloads-and-file-requests line103 creates an input and installs only onchange. It does not install oncancel or a settled cleanup path. Browser cancel normally emits cancel rather than change; that leaves the promise pending, contrary to the nearby documented empty-on-cancel contract. Actual ShellHost/host-effects/document-load-effects applyRequestFileOpenEffect awaits this function before dispatching chunks, so the Lowpoly custom picker follows the affected path. WGPU host-io openFiles line155 explicitly handles onchange and oncancel with a settled guard and removes the mounted input. React natural-file requestFileSelectionV1 also has a separate correct cancel contract; its tests do not cover requestFileOpen.

## Smallest public regression seam

Extend renderer `🧪️tests/💾️media-export-and-natural-files/🟦️.ts`, which already imports public downloadMediaExport and requestFileSelectionV1. Import public requestFileOpen, supply a real input via document.createElement spy, stub click, start the promise, dispatch Event('cancel'), and assert it resolves [] and clears/removes picker resources. A matching selected FileReader dataURL case must assert exact original filename and contents; this tests the actual Lowpoly route rather than natural-file helper parity.

For export delivery, the existing test at line31 checks mounted anchor and non-immediate revocation, but does not inspect Blob bytes/MIME or eventual revocation. Extend that public helper test with all three neutral MIME/ext cases, intercepted createObjectURL Blob, undefined encoding, UTF8 including non-ASCII, exact download name, and fake timer disposal. WGPU equivalent public seam is createWgpuPageHostIo with download-media-export request and supplied Uint8Array; assert Blob/filename/disposal there. Native save dialog/fs sink needs its owned host interface test, not a browser mock.

Mounted Lowpoly public export tests prove effect contents using owning decoders once Native executes; they do not prove browser download completion. Independent Three oracle currently proves geometry fixture expectations only, not actual downloaded OBJ/PLY/STL bytes. This audit makes no browser/native pass claim.


## Follow-Up

Root corrected the React picker through existing requestFileSelectionV1 after actual public JSDOM red/green. Independent source review found no new defect; see react-picker-review.md for exact coverage limits and readiness snapshot. The initial defect section above describes the pre-correction state.
