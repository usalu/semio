# Media Pilot Repairs

## Implemented repairs

MP4's mutation text grammar now uses the current grammar dialect and lists all ten aggregate operations, including `patch-snapshot`. The text and binary TypeScript facets re-export the aggregate `Mp4Mutation` instead of importing themselves. Native laws now include `PatchSnapshot` in text/binary round trips and require the grammar recognizer to accept the aggregate printer's patch output.

The focused MP4 payload test had selected `s.stdio.mp4` through `s.stdio.mp4@isobmff/*` without assembling the production plugin that normally registers the descriptor. Its two direct guarded-edit tests now register `mp4_artifact_schema_descriptor()` during test setup. The validator remains strict; the test setup mirrors production assembly registration.

WAV's mutation grammar now recognizes recursive compact JSON values instead of a marker line. Its Rust text facet exposes the grammar source, and its aggregate TypeScript declaration now mirrors the native camel-case tagged union for snapshots, formats, sample data, RIFF chunks, `PatchSnapshot`, and every existing mutation variant. Text and binary TypeScript facets re-export that aggregate. The native grammar law feeds every aggregate printer variant to the recognizer.

The MP4 and WAV TypeScript package runners now reject selectors before invoking the generic router, type-check aggregate/text/binary mutation facets plus their contract tests under strict TypeScript, and execute those Bun tests. Their Nx inputs include the complete artifact taxonomy, exclude generated package `dist` trees, and include the shared build libraries and lockfile.

PNG's interactive raster admission remains bounded and increases from 16 MiB to 32 MiB, which admits a 3840 × 2160 RGBA8 raster at 33,177,600 bytes. The native UHD case edits one bottom-right pixel, checks the encoded mutation remains below the store item limit, and verifies all unrelated pixels. The full-build compile repairs import `InteractiveJobCloseStep` from `semio_framework_job` and expose the command module crate-wide for the separately mounted window module.

## Executed validation

- `bun nx test @semio-tech/stdio-png --skip-nx-cache` passed: two tests, six assertions, including `pngjs` bitblt and encode/decode comparison. Nx reported cache skipped.
- `bun nx test @semio-tech/stdio-mp4 --skip-nx-cache` passed after fixing readonly expectation typing: one Bun test, thirteen assertions. The runner also completed strict facet type-checking and generic package export validation. Nx reported cache skipped.
- `bun nx test @semio-tech/stdio-wav --skip-nx-cache` passed after the same test-only readonly correction: one Bun test, nine assertions. The runner also completed strict facet type-checking and generic package export validation. Nx reported cache skipped.
- Explicit uncached Nx invocations with `forbidden-selector` for MP4 and WAV both exited with status 1 at the package guard before the generic router, with the expected complete-suite-only error.
- `git diff --check` over the MP4, WAV, and PNG artifact trees passed.

The first MP4 and WAV Nx executions reached strict TypeScript and failed only because Bun's mutable `toEqual` overload rejected readonly expected arrays. The expectations now materialize mutable arrays; both complete uncached reruns passed as recorded above.

## Evidence still pending

No Cargo command was launched by this worker. The earlier focused native MP4 payload test failed after 114 minutes because its direct unit-test path had not registered the selected schema descriptor. The registration repair is source-only until the warmed native build reruns it. The new MP4/WAV grammar laws and PNG UHD native case are also unexecuted.

Root full native build run 7 was active as process/session `7970` when this report was written. Its result is not claimed here. Earlier actual language-neutral oracle evidence supplied to this worker remains 77 passing tests and 232 assertions, and the PNG TypeScript suite result above is the fresh uncached execution.

The PNG 32 MiB ceiling and retained row preparation establish bounded admission and bounded preparation records. They do not establish bounded end-to-end store work because inverse, diff, and apply still meet an atomic shared-store seam. The core cooperative hook must land and the native UHD test must pass before claiming end-to-end bounded 4K editing.

## Next compact rollout order

After the five pilots and cooperative publication hook are native-green, the smallest textual aggregates should prove the repeatable mount before payload-heavy formats: SVG base/basic/tiny first, then the compact Semio value/text roots, followed by DXF and STL scalar or textual paths. Each batch must preserve the exact concrete dialect controller even when an aggregate is shared. GIF, BMP, MP3, AVI, glTF, PLY, LAS, and other large byte/vertex/point carriers should follow only with one payload-past-1-MiB retained publication test per storage shape. IFC, STEP, DWG, and the remaining Semio subsets come last because their derived-root controller matrix needs the broadest registration proof.
