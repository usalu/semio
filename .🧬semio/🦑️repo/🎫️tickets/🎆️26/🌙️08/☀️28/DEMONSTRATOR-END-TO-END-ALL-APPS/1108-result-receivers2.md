# 1108 lane receivers2: NativeReceiver port outside the value owned crates

## Design rule used (composites)
- Children are closed only in the parent close path, never inside `advance_local` (its reply is `OriginalReceivingPrimary`, a close reply is `OriginalRetirementFailure`).
- Close path forwards to the single open child with `reply.progress()`: not-closing child -> `begin_close_into(effect, reply.progress())`, closing -> `close_into(body, &mut effect.nested(child_grant(g,0)), reply.progress())` (or `close_original_into` when `requires_original_control`); recovery routes by state to `recover_*`.
- `advance_pending` = any child pending; close start is the separately paid flag (`native_receiver_close_start!`).
- Where a record must close a child mid-advance (derive matcher/source/field slots, paged map child, named root child) the reply-shaped close is settled locally with `native_close_settle(effect, |effect, reply| owner.close_into(body, effect, reply))`.
- New shared helpers (protocol crate, re-exported through `native_decoding::owned`): `native_close_settle`, `native_reply_poll`, `native_admit_default`, `native_receiver_close_start`, `native_close_nested`.

## Ported (EDITED, unverified: owned/protocol chain is red, W-receivers2-2/3 show only errors inside the owned crate)
- graph manifest receive, ui text receive (+TextRetirement), ui values receive (+UiNativeRetirement, Phase::CloseChild removed), value paged receive text (+defaults), paged map receive (cleanup uses native_close_nested; child close start via native_close_settle), named root receive, derive receive (+enum), stdio part21, os expanded paths, os store child, os neural.
- 14 outside `NativeDefaultValue` impls moved to the reply form via `native_admit_default`: generation3d editor/viewer config, energy editor/viewer camera, draw attributes, raster layer-model, stdio base, kernel ui turn patches, ui patch ops, ui fixed list/map/bytes, viewport2d, flow camera/ui.
- derive proc macro crate compiles (W-receivers2-1 rc 0); every edited file passes a rustfmt parse.

## Open / debts
- Raster coverage receiver (plugin raster artifacts selection/coverage `receiving`, `builder`, `owned`) still on the pre-732 API (`advance(original,control,body,grant)`, `begin_close_admitted`, grant-shaped builder). Needs a builder port, not a trait port: NOT touched.
- Test drivers still call the old receiver protocol: ui presentation receive tests, expanded paths `work_tests`, graph manifest `work_tests`, part21 tests, paged `retained_work_tests`: need a testkit driver rewrite.
- close_original_step on derive record/enum falls back to close_step (no original-control forwarding), same as before.
- recover_close_step for composite owners without a recoverable child returns `reply.pending()`.
- Paged text receiver wraps the paged append cursor's sync `close_step(effect)` (cursor itself is the paged owner's).

## Rerun (once owned compiles)
`"$C/1105b-tools/chk.sh" W-receivers2-<n> <manifest> native|wasm` for: graph/🛂️manifest, ui/🧬️contract/🔤️text, ui/🧬️contract/🌱️values, value paged (via value crate), derive-using crates.

## Verify round (owned is green)
- W-receivers2-6/7/8 (graph manifest, ui values, ui text) stop in the semio-framework-value facade (shared-utf8 clone, publication root retirement, retained-clone paged, BoundedOrd, type): owned by instance-b/toolrun, none of my files has an error yet. Fixed paged append cursor body params (instance-b file, logged).
- Raster coverage ported blind: builder (effect-shaped push/finish/advance/take, reply close_into), NativeReceiver receiver, OwnedFieldReceiving owned, builder tests (cx.with_original_effect driver). Receiver/owned tests use the shared OriginalWorkFixture harness, left as is. Old-protocol drivers ported blind onto one shared driver `value/🧪️testing/📥️admission/receiver-driver.rs` (EffectBank + settle/post; receive/close): graph manifest, expanded paths, part21, ui text, ui presentation `run`, paged (denied begin_close_into). Denial/cancel-cut matrices were dropped (old per-axis assertions); sources and outputs are `mem::forget`-ed (no source close). Dev-deps testing-admission/allocator added to 5 manifests: LOCK NEEDED.
- W-receivers2-9 (graph manifest): the semio-framework-value facade is now refused by the Source byte budget before compiler; not raised.

STATUS: BLOCKED value facade refused by source budget (instance-b/toolrun); receivers2 crates unverified
