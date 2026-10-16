# Shared CRC and Print PDF Source Checkpoint

2026-10-16. This checkpoint covers Source, pure byte algorithms, ordinary in-memory SQLite, genuine issued control with delegated worker event ports, and strict compilation. No PDF renderer, native canvas allocation, worker thread, Tectonic, archive acquisition, Native cache image, SDK publication, or network request ran.

## Shared checksum ownership

Core `process/📦️artifacts/🔢️checksum/🔁️crc32` now owns the one CRC32 table and incremental algorithm. Its closed schema fixes unsigned finalized seed and at most 65,536 input bytes. The caller owns progress/cancellation between chunks. The original archive CRC wrapper delegates to this primitive, preserving its actual control checks and progress interval. ZIP framing and SDK tar behavior remain in their existing owners.

The missing-owner test first failed in `print-shared-crc-red1.log`. The neutral corpus then matches independent crc-32, all fragmentation splits, and SQLite. Print PNG chunks use the same primitive and are independently checked against pngjs and crc-32, with Noble SHA256 on raster DATA. Corrupt CRC, incomplete footer, trailing bytes, and valid empty IDAT framing are covered.

## Genuine correction

`print-shared-crc-prototype1.log` caught Buffer.slice aliasing: mutating the input changed the supposedly owned PNG copy. The actual capture now creates an independent Uint8Array for both PDF input and PNG output. Its unchanged regression passes in successor2. This was an actual byte-copy defect, not renderer qualification.

## Collected terminal evidence

| Route | Prototype | Release |
|---|---|---|
| Existing vite-environment | successor2 exit0 | release1 exit0 |
| Core CRC + archive | 2 tests / 128 assertions | same counts, exit0 |
| SDK archive | 6 / 40 | same counts, exit0 |
| Print | 41 / 558 | same counts, exit0 |
| Shared Source cohort | 120 / 1993 | same counts, exit0 |
| Default Cargo Source cohort | 4 / 301 | same counts, exit0 |
| Expanded vite-environment-types | prototype1 exit0 | release1 exit0 |

Worker port tests hold termination until its real delegated gate joins and retain Error and undefined acknowledgement failure. No actual worker was spawned. The pipeline consumes bounded first-party snapshot DATA; PDF.js/canvas implementation types stay within the worker Script. Complete PNG CRC validation occurs under caller checkpoints before accepted raster use.

## Remaining runtime and caller frontiers

The new PDF snapshot/raster worker is Source-qualified only. Native worker memory enforcement, renderer termination latency and OS descendant behavior remain unqualified. Actual pinned downloads/materialization, Tectonic compilation, failed-child inspector execution, native Rust emission and full pipeline publication remain held. Inferred Print targets still declare old checkout dist outputs and are the next actual metadata correction. This checkpoint does not claim full Print runtime success or cache-hit correctness.

## Exact Source epoch

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🔢️checksum/🔁️crc32/🟦️.ts` SHA256 `b65d44b4d85ef3bd0cc1613b4110817eb72e9bd2d5243e7f07ecc6e58d848cb7`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🟦️.ts` SHA256 `c07ae16b87de8458950697f8cfbf69bddb672a09020f065adba070d407da294e`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/📥️pdf/🟦️.ts` SHA256 `d8b30b6e0c624cec0873df43773f1ab3d24b81bdc82c80fbc771a2f63f59141b`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/📥️pdf/📜️script.ts` SHA256 `388d867db64dd2e862063ab8cffbe9b7d69c8a6d9ef3e594c01a44e35a848985`
- `🧰️framework/🛍️products/📓️print/📦️artifacts/🧵️worker/🟦️.ts` SHA256 `25e40c0d59f2fc50f0c90e14443442c084afe8007f092d00df6a2c994289f7fe`
