# WGPU Browser Input Ordering

The browser frame transport now submits one bounded FIFO sequence. A pointer-down, pointer-up, cancel, leave, key event, wheel sample, or other ordered input seals the current coalescing segment. Within a segment, each pointer keeps its latest move and resize keeps its latest metrics. Each wheel input retains its own coordinates, modifiers and deltas. A move after release stays after release, even when an acknowledgment or a wire page boundary arrives in between. [Ordered wheel verification](./wgpu-ordered-wheel.md)

The worker validates each page before applying it. Browser pointer samples keep their positions in the Rust host's ordered queue when they are drained. The native window callback continues using the existing pointer coalescer. Browser input still uses the worker's latest viewport metrics for hit testing, while the coordinates captured with each pointer event remain in logical CSS pixels.

The browser and worker enforce bounded item counts. The browser also budgets exact UTF-8 bytes for each outgoing JSON event and splits text streams within that limit. Hub document status retains its independent, per-document coalescing.

## Verification

- All 122 tests passed across the browser transport, wire, and accessibility suites. The shared JSON cases cover two pointer identities, a move during drag, hover after release, leave and return, and ordering across worker credits.
- The latest native Node/Vitest rerun passed all 122 with the test policy's 30-second budget. The default five-second deadline had expired in a boot-brand test during the parallel Rust build.
- All 18 native Rust tests passed in the isolated verifier. It mounts the repository's EventQueue, allocation observer, browser wire decoder, and hub status contract, and uses the interaction library that provides the renderer's dispatch types. It checks the same fixture through serde and confirms ordered movement across two host drain pages, including refusal before the queue generation changes.
- The focused strict TypeScript check for the WGPU transport and wire fixture passed. A separate check that included the worker traversed existing errors in the Diagram node attributes and worker state plugin manifest types.
- The Rust verifier covers native wire decoding and queue admission. The live Chromium route remained blocked by the browser and server-bind sandbox restrictions documented in the browser audit.

## Transport timing

`measure-wgpu-input-transport.ts` uses the production JavaScript transport and the existing fake worker. It measures pointer admission, coalescing, batch preparation, and acknowledgement. Each profile records 5,000 frames after 1,000 warmup frames.

| Samples per frame | Pointers | Median | 95th percentile | 99th percentile |
| --- | ---: | ---: | ---: | ---: |
| 8 moves, press, release | 1 | 4 µs | 7 µs | 14 µs |
| 32 moves, press, release | 2 | 5 µs | 12 µs | 21 µs |

This measurement excludes worker scheduling, WASM dispatch, document changes, and pixels. Browser input-to-frame latency remains unmeasured.
