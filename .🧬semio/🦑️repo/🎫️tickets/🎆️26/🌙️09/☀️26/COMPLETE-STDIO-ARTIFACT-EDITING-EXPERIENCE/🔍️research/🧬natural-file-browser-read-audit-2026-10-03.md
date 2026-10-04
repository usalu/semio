# Natural File Browser Read Audit

The native media route now owns raw intrinsic bytes, but the browser import currently calls `requestFileOpen(..., "dataUrl")` and then `bytesOfDataUrlV1`. This performs a full FileReader data-URL/base64 conversion and a second byte materialization before the native import. Transfer tracking begins only after the picker/read promise returns, so the visible task cannot cancel or report progress during a large actual file read.

The shared string picker also has no `input.oncancel` handler. Cancelling selection can leave its Promise unresolved. Its async change handler does not settle file.text failures, and FileReader errors currently become omitted results.

The bounded ownership execution lane has been assigned a natural-file raw-byte picker/read path with bounded Blob slices, caller-owned AbortSignal, real byte progress, explicit error/cancel settlement and one-time cleanup. The captured document target must stay authoritative across selection/read waits. The existing native codec still consumes a whole byte owner; segmented native codec work is a separate remaining boundary.

## Implementation checkpoint — 2026-10-04

Natural Open now calls `requestFileSelectionV1` and reads the selected `File` with `readBlobBytesBoundedV1`. The picker settles an empty selection on `cancel`, guards duplicate terminal events, detaches both handlers and removes the transient input exactly once. A later attempt can select normally. The focused plugin/app owner, codec, and `importAppMedia` function are captured before the picker await.

The reader admits a maximum 512 MiB contiguous owner before reading and materializes at most 256 KiB per `Blob.slice` turn through abortable `FileReader.readAsArrayBuffer`. It reports `(completed,total)` in bytes after every slice, maps read/truncation faults to the natural-file read boundary, and honors the caller's `AbortSignal` during a slice and before the next turn. It produces raw octets directly; no data URL or base64 representation exists in this route.

The language-neutral fixture defines chunk, allocation, octet, progress, and cancellation boundaries. `natural-file-raw-picker-contract-3.log` records 1/1 focused green with cache disabled, covering exact bytes, maximum refusal, cancellation, read fault, picker cancel cleanup, retry, and fresh-owner retirement. `natural-file-raw-picker-typecheck-2.log` records a fresh renderer aggregate typecheck green.

This establishes the source/browser-host boundary, not a real picker gesture. Root still owns browser acceptance with an actual selected file and Tasks-window observation. The native codec and component bridge remain contiguous-owner operations; segmented decode/import is separate.
