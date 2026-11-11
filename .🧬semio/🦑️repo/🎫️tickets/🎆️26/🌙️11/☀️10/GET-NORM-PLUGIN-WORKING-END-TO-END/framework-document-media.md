# semio-plugin-document-media migration

Result: `check -p semio-plugin-document-media --lib` from en1990 reports 0 errors (was 14 media errors when picked up; the envelope/store/archive files had already been migrated). The `framework-composition-publication.md` report did not exist.

## Changes (🔌️plugin/🎞️media)
- `💥️fault`: new `run_media_effect` / `close_media_effect` run an owner effect through `NativeEncodeControl::effect_allocated` (original allocation recipient, fresh effect state) and return the owner's own result and physical receipt; `media_failure` replaces the removed `NativeSnapshotFailure::into_value_error`. `close_media_projection` steps its `ControlledRetirement` with `(body, effect)` through that helper. `receive_media_output` / `receive_media_artifact_fault` use the new `receive`/`receive_nested` shape (output slot plus closure `slot, result, marker, unwind, native, wallet`).
- `🛫️intrinsic`: `close_cursor` takes the effect (`cursor.close(effect)`, `effect.performed(1)`), `close_step` no longer uses `plugin_unit_step`; the encode body drives `advance_intrinsic` and cursor `close` through `run_media_effect`; encode/copy results are published into the output slot.

## Not done
Tests (`--tests`, `test -p`) not run; the fixture `🧪️tests/🔬️app-owned-media-error` still uses the old owner/step API. Failure owners are converted to `ValueError` by message and kind (the old code did the same via `into_value_error`).

## Round 2 (tests)
- `check -p semio-plugin-document-media --tests` (repo root workspace): 0 errors. It first failed with a `#[global_allocator]` conflict: the dev-dependency job testkit pulls `semio-framework-value-testing-allocator`, so the lib-test heap witness in `🗃️document/🦀️.rs` was removed (the testing allocator is the one allocator).
- `test -p semio-plugin-document-media`: 2 lib tests pass (`original_archive_admission_laws`). They stalled in `OriginalJobFixtureCaller::retire` because the Source root close now demands 27481 copy bytes against 7159 left of the 65536 stock; the tests now call `fund_closure(65536, 0)` before each `retire` (🗄️document-archive-load/🦀️.rs).
- The `🧪️tests/🔬️app-owned-media-error` fixture is not a document-media target: it runs inside the `semio-plugin` facade tests (`🧷️companions/🍋️tests-04`, host `🪝️mounts/📜️wp2-declaration/🧪️hosts/🎞️owned-media-error`), which depend on the rest of the plugin stack; not yet compiled.
- `semio-plugin-surface-fixture`: the first run died in the repo source selector (`process.source.cfg-owned-item`) on `👷️job/🛬️receiving/🏗️steps/🦀️.rs`, where another agent had a trailing `#[cfg(test)] thread_local!{..}` (DBG); rerun was still compiling at report time.
