# Fixture Oracles — 2026-11-04

Read the latest engine, surface and integration notes. Executed the retained `🐍️fixture-oracles.cjs` helper through `repo tools exec --light node -- <ticket>/🐍️fixture-oracles.cjs`; final exit code 0. Existing registered Node and already installed pngjs 7.0.0 / sharp 0.34.5 were used; no dependencies installed.

- Independently recomputed both valid raster sample arrays: valid-range [0, 0, 32768, 65535], constant [0, 0]. Constructed standard grayscale16 PNGs from these results and decoded with third-party pngjs, retaining full 16-bit precision. Both decode results match fixtures.
- Five malformed raster shapes agree with admission failure: empty width/height, missing/extra values and 2040-pixel row exceeding 2039-pixel durable allowance.
- pngjs rejects arbitrary bytes, truncated PNG signature, invalid-base64 decoded bytes and text payload. Node base64 decoding itself is permissive, so this does not establish production base64 rejection behavior.
- Verified zero-width PNG header has width 0, height 1. pngjs accepts this invalid shape; sharp raw decoding independently rejects it with `invalid image width`. The helper retains both observations and asserts sharp rejection.
- Four append fixtures independently match maximum reserved frame/asset index plus one, including orphan assets and u32 exhaustion.
- Cancellation fixture requires unchanged decisions, positive post-cancel turns and retired scene/engine. Its Rust test consumes those fields and asserts no further outcomes and empty observations/owed state after funded close.
- All three JSON fixtures have matching Rust include_str bindings. Absolute Windows fixture/test-source paths are at most 167 UTF-16 code units, below 256.

These are executable fixture, independent codec and source-binding checks. They do not execute the production Rust encoder, editor mutation paths or cancellation driver. Owned native build/test queue remains decisive for production behavior. No production files, AGENTS files or Git state were modified. No generated artifacts were retained by this validation.

## Latest Fixture and Declaration Revalidation

Extended retained oracle helper with Sharp JPEG encode/decode. Reran all earlier PNG/sample-shape/index/cancellation-fixture/binding/path assertions after the latest fixture changes: exit 0. Sharp 0.34.5 independently encodes the fixture's 2×3 RGBA white input at quality 90 as JPEG, decodes to grayscale, confirms dimensions 2×3 and exactly [255, 255, 255, 255, 255, 255] luma. Sharp also rejects the new truncated-JPEG bytes. No files are emitted by these in-memory codec checks.

Executed retained `🐍️typescript-declaration-check.cjs` through registered lightweight Node: exit 0. Its declaration fixture matches target execution metadata, source observation, runtime entries, resource closure and named-input evidence; Ajv accepts runtime/data schema values and micromatch confirms artifact coverage including ts, tsx, json, semio, png, absent and rs suffixes. This validates source declarations and cache-input patterns; it does not run TypeScript tests or infer fresh cached build correctness.

No production Rust execution or new compiler process was started for this revalidation.

## Compact Raster Fill Revalidation at 22:22 UTC

Updated only the retained oracle helper to expand the current neutral `fill: {count, value, valid}` descriptor. Explicitly verified the oversized-width case expands to 2040 values and 2040 validity entries, exactly width × height, before evaluating width > 2039. This isolates the row-width refusal from sample-count mismatch.

Reran fixture oracle and TypeScript declaration helper through registered lightweight Node; both exit 0. Both independent grayscale16 PNG cases, all malformed shape cases including the isolated oversized row, corrupt/zero-width PNG evidence, Sharp positive 2×3 JPEG exact luma and truncated JPEG rejection, four index cases, cancellation invariants, three source bindings, path lengths and Ajv/micromatch declaration checks still pass. No production Rust was executed.
