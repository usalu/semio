# Actual Generic JSON Heap Corroboration

Six sequential fresh Node v24.15.0 processes ran through `repo --tool=nx capacity run --memory=1536 --cpu=1`, with `--max-old-space-size=1536`. Each naturally exited 0 with no signal. No managed cache payload was read. The retained neutral fixture, schema and probe input are at the ticket root; complete command output and structured measurements are generated artifacts.

| Case | Actual outcome | Peak RSS (MiB) | Parse and scan (ms) |
| --- | --- | ---: | ---: |
| ascii-padding | accepted | 898.28 | 490 |
| unicode-padding | accepted | 1401.33 | 1436 |
| wide-node-boundary | accepted | 1334.67 | 3156 |
| wide-keys-boundary | accepted | 1504.20 | 6686 |
| reference-boundary | accepted | 1112.58 | 991 |
| node-overflow | bounded-node-refusal | 597.14 | 2297 |

All five valid cases scanned the exact 256 MiB wire and retained every expected reference. The overflow case used 4,194,305 key/value nodes and refused through the existing `Bounded closure reference DATA required` guard. The legal flat boundary is 4,194,303 because the actual traversal charges root plus key and value for every edge; the unchanged cap is 4,194,304. Each process checked actual reference results against an independent small TypeScript JSON AST oracle, with Ajv validating the closed neutral case contract.

Actual parse/reference function SHA-256 remained `b624a6f636647101635b9dac579dcca75846635906663bd8650406f10a41867e` for all six processes. Worker file SHA changed as the separately coordinated immutable Source adapter was saved; extracted functions were identical. The peak RSS is the OS process high water mark, including TypeScript/Ajv harness modules and transient parse memory.

The wide object reached 1504.20 MiB, leaving a narrow margin under the 1,536 MiB reservation. These six shapes do not establish a universal or full parent-plus-worker bound. No OOM or cap violation was demonstrated, so no production parser change, cap change, or DFS rewrite is justified by this probe.
