# Current Direction Provider and Retirement Contract

Read-only fresh source audit; no builds.

## Canonical signatures now current

Store main12157 export takes `(schema, dialect, &IoPayload, &mut SqliteSnapshotControl, &mut NativeSnapshotDecodeOwner) -> IoResult<SqliteDatabase>`. Import12158 takes `(schema, dialect, &mut Option<SqliteDatabase>, encoding, &mut SqliteSnapshotControl, &mut NativeSnapshotEncodeOwner) -> IoResult<IoPayload>`. These are direction owners, not IoRunControl, in the actual declaration now read. Direct native trait decode12091 takes payload, SQL control and decode owner; encode12097 takes encoding, SQL control and encode owner. Receiving reconstruction/projection likewise take matching direction owners.

CSV/TSV snapshot implementations16–19 now again implement `retire_sqlite_snapshot(self)` by `FromValue::retire_decoded(self)`; Store trait12086 defaults to drop(self). Neither method accepts a grant, recipient or cumulative receipt. Restored compilation compatibility does not make this paid controlled retirement. Replace actual corpus use with controlled ownership laws, rather than adding another compatibility helper.

## Honest test-local function shapes

Use generic test-local direction scopes: `with_original_decode<R>(law, limits, sql_callback, operation: impl FnOnce(&mut SqliteSnapshotControl<'_>, &mut NativeSnapshotDecodeOwner<'_, '_>) -> R)` and symmetric encode scope. Explicit authored law supplies original body grant and native maximum. Named native callback, forwarded allocation port, original retirement recipient and control live outside owner loan. Execute assertions while actual returned R remains owned, then close returned owners through actual controlled retirement. Returning R out of this helper means caller assumes explicit controlled cleanup; the helper must not silently claim all output was closed.

For returned values use actual `admit_typed_controlled_retirement(value, closeGrant)` or original Option-preserving factory admission where appropriate; retain `(error,value)` on refused admission. Original recipient pending failures close through the same Native control `close_retirement_recipient(closeGrant)`, after direction owner is dropped. Account admission birth and actual close progress, including cursor backing; an uncharged `value.retirement()` allocation is not an equivalent replacement. Output admission/close and producer recipient close are distinct ownership obligations. Never reconstruct a grant from demands or payload size.

## Existing finite policy scope

Both receiving fixture policies explicitly author native maximum16MiB, body items65536/copy1MiB/capacity4MiB/release4MiB/depth128, close items4096/copy1MiB/capacity4MiB/release4MiB/depth256, maximum65536 close turns and zero denied grant. Large authored UTF8 field98304 bytes is smaller than declared copy ceiling, but arithmetic alone cannot qualify complete trees, native spec/record/envelope or retirement cursor birth. Existing tests check individual close step fits and terminal turn bound. These limits are suitable declared candidates for corpus adaptation; adequacy for every native corpus case remains actual execution evidence, not an audit-certified guarantee. Do not raise ceilings automatically from data.

## Additional current default frontier

Trait default native decode/encode/preflight12091–12108 still construct `ValueError::new` UnsupportedOwner and ignore the native owner argument. Existing default calls therefore allocate refusal prose outside original zero-capacity control, unlike explicit literal receiving defaults. This is a concrete separate fixed-refusal ownership frontier.
