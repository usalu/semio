# Owned Auxiliary Input Research

The actual owned process backend uses node:child_process.spawn and already preserves ordinary stdin unless explicit input is provided. A snapshot on ordinary stdin would therefore change existing repo exec behavior. The selected first-party auxiliary input port must preserve stdin and live output, retain issued control/cancellation, bound exact UTF-8 bytes and join publication/terminal errors. The carrier conveys DATA, not authority. No temporary Source store, environment Source blob or extra runtime library is selected.

[Node24.15 stdio documentation](https://nodejs.org/download/release/v24.15.0/docs/api/child_process.html#optionsstdio) defines extra stdio indices as additional pipes exposed through child.stdio. It separately documents Windows overlapped mode and warns that pipe descriptor pseudo-files are unsuitable. Use the actual inherited descriptor/stream interface, not /dev/fd paths.

[Pinned Bun1.3.14 child_process implementation](https://raw.githubusercontent.com/oven-sh/bun/bun-v1.3.14/src/js/node/child_process.ts) routes extra pipe slots through the native spawn handle and node:net. This supports designing against the existing system API but does not establish Windows correctness. The [upstream historical extra-descriptor issue](https://github.com/oven-sh/bun/issues/11044) was closed with a reported Linux verification; that report is not a Windows pass or our product test.

[Bun spawn API](https://bun.com/reference/bun/spawn) documents extra descriptors and caller ownership details. Production stays behind the first-party owned process interface. Actual large-payload, ordinary-stdin, streaming, early-close, cancellation/deadline and primary-error tests are required. No subprocess or product code was executed for this research.


## Captured Command Progress Routing

The existing runOwnedCommand live implementation sends its own ten-second heartbeat through onProgress when supplied, otherwise console.log. stream-without-prefixes controls forwarded child line formatting and does not alone prevent parent heartbeat text on stdout. The new captured-entry caller must preserve its actual stdout contract with explicit semantic/stderr progress routing where needed. High is checking its exact callers; no protocol contamination or runtime failure has been observed in this slice.
