# Open File into the Empty example (2026-11-27)

## Root cause
- Guest stderr does reach the browser console (`console.error`, see `🌐️wasi/🟦️.ts` `classifyGuestLogLine`). The 11:47 hub simply predated the `[DEBUG]` prints.
- With the prints (hub 15:08) the import is NOT a livelock: `NaturalFileImportJob` job size 8304 B fits the 1 MiB grant (ceiling 2^31 items); the job is created, then the second step fails with `UnsupportedOwner: natural-file format has no original controlled decoder`. The sourcing editor declares `natural_file_codec` (json) but leaves `SINGLE_TURN_NATURAL_DECODE=false` and had no controlled decoder; the trait default `admit_natural_file_decode_owner` refuses. The earlier endless `more-work` loop on the 11:47 hub was this refusal being retried while the kernel retirement/receipt accounting (since fixed by peers, see `boot-retirement-invariant-2026-11-27.md`) could not close the failed job.
- A single-turn synchronous decode would not do: the file (4.6 KB) exceeds `max_work_units_per_step` (4096).

## Fix
- Framework (`🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs`, re-exported by `semio_plugin`): generic `BufferedNaturalDecodeCursor`, `build_buffered_natural_decode`, `admit_buffered_natural_decode`. For parsers needing the whole input: charges the bytes as slices of at most `max_work_units_per_step` (positive progress per turn, yields only when fuel is 0), runs the format's one-shot decode on the last slice, closes through the owned retirement of its byte buffer.
- Sourcing editor (`✏️editor/🧩️sourcing-app/🦀️.rs`): overrides `build_natural_file_decode_owner` / `admit_natural_file_decode_owner` with the buffered decoder over `decode_natural_file`.
- Test: `buffered_natural_decoder_charges_bounded_slices_then_decodes_once_like_the_format` (`🧪️tests/🧬️mutation-fixtures-surface/📄️natural-file-reserved-job-laws/🦀️.rs`): 1000 bytes at 256/slice gives 3 progress slices, then Complete; snapshot equals an independent iterator-sum oracle; completed decoder refuses further advance. Not executed: the `semio-plugin-surface-fixture` lib-test crate currently fails to compile for unrelated peer reasons (`begin_action` arity at line 84 and others); the framework and sourcing code compiled in the hub wasm build.
- All `[DEBUG] natural-*` prints removed (both).

## Verification
- Hub wasm 17:29 (includes the fix): `probe-open-file.ts` against 6081: no faults, no `file-open-failed`; the Empty example now shows the saved document (Curated: KVH C24 100x200 x1, Steel IPE 200 x1, Pool curated counts 1/1; screenshot `🗑️generated/journey/probe-en-probe.png`). Coordination plan 18:16: Open File no longer fails in the full journey.

## Notes
- The decode itself is still one-shot on the last slice (the sourcing JSON parser is not incremental); a truly incremental JSON cursor remains future work.
- I removed a stale `🗑️generated/build.lock` (no holder, 1.5 h old) at 13:3x.
- Language-agnostic fixture row for the buffered decoder was not added (the shared natural-file fixture JSON has other-language consumers).
