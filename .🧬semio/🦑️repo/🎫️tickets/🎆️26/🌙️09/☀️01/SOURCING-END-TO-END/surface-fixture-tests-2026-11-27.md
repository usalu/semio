# semio-plugin-surface-fixture lib tests (2026-11-27)

Crate: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🪟️surface-fixture/📦️packages/🦀️rust`. Plain cargo, `-j 2`, target `/Users/ueli/t/src-fixture` (deleted at the end). Logs: `🗑️generated/fixture-*.log`.

## Compile errors (7) and fixes
| Error | Fix |
|---|---|
| E0061 `begin_action` takes 3 args (`natural-file-reserved-job-laws` l.84) | pass the new `maximum_items` (`8`) between `args` and `cx` |
| E0433 x5 `crate::natural_file_import::publication_checkpoint_due` (same file, l.460-467) | the module now lives in `semio_plugin_decl_surface`; call `semio_plugin_decl_surface::natural_file_import::publication_checkpoint_due`; the fn was `pub(crate)`, so `📜️declaration/📥️natural-file/🦀️.rs:459` is now `pub` (module is already `pub`, `#[doc(hidden)]`) |
| E0425 `artifact_app_laws::dispatch_addressed_fixture_on_original` (`controller-and-envelope-laws` l.55) | it moved to `semio_plugin_testing_instance_laws`; call it there and add that path dependency to the fixture `Cargo.toml` |

## New test
`buffered_natural_decoder_charges_bounded_slices_then_decodes_once_like_the_format` compiled but panicked in the fixed-caller drop ("requires paid terminal closure"): the test never retired its `OriginalFixtureCaller`. Added `fuel_remaining`, `drop(original)`, `caller.retire(fuel)` at its end. Now passes (1 passed), so the buffered decoder itself (3 progress slices, then Complete, equal to the format oracle) is correct.

## Run results (RUST_MIN_STACK=128 MiB; the repo launcher sets a large stack, plain cargo's 2 MiB overflows `editor_fixture_still_mutates_normally`)
34 tests: 11 pass (incl. the new one), 21 fail, 2 hang (>60 s, skipped: `registered_editor_consumes_intrinsic_natural_bytes_through_the_real_media_route`, `registered_editor_natural_bytes_stay_untaken_while_unfunded_and_settle_once_funded`; killed my own test process).
Failing, not fixed (each is a behavioural drift in peers' in-flight framework work, not a compile/API-arity issue):
- `mounted natural-file codec owns its original reserved route` (4 tests, `natural-file-reserved-job-laws` l.57): sourcing/surface mounts no longer expose the reserved route; related to the missing "original controlled decoder" in `open-file-livelock-2026-11-27.md`.
- `birth.output_ready` / `birth.terminal_is_empty` / `original preparation admitted` / `birth requires no fabricated release budget` / `with_original_effect` assertions (6 tests): original birth/preparation semantics changed.
- `interactive-job.unknown-key 'increment'` (`editor_fixture_still_mutates_normally`), `'undo' must be rejected on a viewer`, `a diagnostic fault`, `one op lands one edit` fault, app-id envelope mismatches (editor and viewer), `close_owner Complete` (media owner context).
- retained-work laws: `observed_work`, `demand.copy_bytes <= 64 && depth <= 8`, job/allocator reconcile `left: 12152`, `Option::unwrap on None` in caller (child-emission/group-row tests).
Not investigated further: they are many distinct, unrelated peer-owned drifts; the requested compile fix and the new test are done.

## Files changed
- `…/🧪️tests/🧬️mutation-fixtures-surface/📄️natural-file-reserved-job-laws/🦀️.rs`
- `…/🧪️tests/🧬️mutation-fixtures-surface/🎫️controller-and-envelope-laws/🦀️.rs`
- `…/🧪️tests/🪟️surface-fixture/📦️packages/🦀️rust/Cargo.toml`
- `…/📜️declaration/📥️natural-file/🦀️.rs` (`publication_checkpoint_due` made `pub`)
