# Lane command result (semio-plugin-command)

## State
- semio-plugin-command lib: wasm32-wasip2 via hub manifest `W-command-3` rc0 (58 errors -> 0), native `W-command-4` rc0, `--tests` native `W-command-6` rc0. No warnings in the four ported files.
- Unit test `original_latest_key_admission_encoding_cancellation_preserve_funded_turns` (rewritten for the funded API): `W-command-8` ok.
- Upstream one-line fix I claimed (factoryticket half edit): `value/retirement/aliases/🦀️.rs:54` `crate::close_owner` -> `semio_framework_value_retirement::close_owner`.

## Files ported
- `command/🗝️latest/🦀️.rs`: `ToolLatestWinsKeyCopy` is cx-driven. `admit(&mut Option<Self>,instance,parts,cx)` (birth paid with exact capacity), `demands(parts)`/`advance(parts,cx)` (<=64 source bytes per turn, 3 copied bytes each), `take_key(&mut Option<SharedOwner<String>>,cx)` (`SharedOwner::admit_from` on the turn effect), `close_demands()`/`close_step(cx)` (`forgotten`). Denied turns leave state untouched.
- `command/📥️admission/🦀️.rs`:
  - `ToolLatestWinsScope` derives `RetireOwned` (key alias, operation, token, claim). Private `Slot<T>` = original + `ControlledRetirement` frame; every owner (removed scope, previous/discarded map, update cursor, key alias, unadmitted scope source/shared) retires through `plugin_typed_owner_original`.
  - `ToolLatestWinsRegistry` cx API: `begin(op,&key,&lease,cx)->bool` (funded units: alias key -> build scope -> `SharedOwner::admit_from` -> alias key -> `OrderedMap::admit_update_cursor`; `true` once the update cursor is admitted; if the registry is busy it advances the busy frontier instead and returns false), `advance(cx)` (one funded unit: retire removed / previous / update cursor, `advance_native`, `take_result_into`, settle candidate, scan one rank, removal alias + update), `cancel(op)`, `take_outcome(op)`, `can_begin()`, `begin_close()`, `close_step(cx)->bool`, `terminal_is_empty()`. Removed: `advance_demands`, grant `advance`, `retirement_step`, `retired_*` fields.
  - Laws preserved: new op replaces and cancels the key's old scope (when operation differs); candidate over `ARTIFACT_LIVE_OUTPUT_SLOTS` is rejected (outcome false, candidate retired); a full map is scanned (64 ranks, finished claims removed) before insertion, `admission_scan` reset after each settled update; cancel while pending gives outcome false and retires update + every unadmitted begin owner; finished-claim removal is a scan-driven removal update; `can_begin` needs the whole frontier empty and no pending outcome.
  - `PendingLatestWinsCommand`: `record_original_receipt`, `receipt`, `retired_key`, `permits`, grant `close_step` removed. `original_decoder` is `(NativeAllocationIdentity,u64)`, `context_view` is `ViewLease`, `fault_retirement` and new `key_retirement` are `ControlledRetirement`. New: `close_step(cx)->bool` (true when command owners are empty), `close_key_step(cx)->bool`, `retirement_demands(body)`, `prepare_context_metadata`/`close_context_metadata`/`close_metadata` on `retired`/`begin`/`acted`.
- `command/🎟️typed/📨️wire/🦀️.rs`: `born` birth (`admit(command,source,maximum,cx,admission,failure,output)`), `advance`/`close_step` through `forward`/`retired` with nested effects for the codec, wire and command retirement through `RetainedCloneClose`.
- `command/🎟️typed/🏃️runtime/🦀️.rs`: host event command is born with `produced` (a quote that yields no command is an invariant error); test helper `retire` moved to `retired`/`begin`, stale `#[global_allocator]` removed.
- `command/🗝️latest/🧪️tests/🦀️.rs`: rewritten for the new API (fixture + independent TS oracle unchanged).

## Caller impact (instance lanes)
- `typed-operations` `advance_latest_wins_admission_unit`: replace `begin(...)+record_original_receipt` with `registry.begin(op,key,lease,cx)` (set `lookup_started` from its bool), `registry.advance(cx)`, `pending.close_step(cx)`/`close_key_step(cx)`, `ToolLatestWinsKeyCopy::{admit,advance,take_key}`; no receipt handback any more.
- `jobs/components`: nothing changes except `ToolLatestWinsRegistry::new()` stays.
- Scope no longer has a `fn new`; only `begin` builds it.

## Debts
- `command/🎟️typed/🏃️runtime` unit tests (6) fail at `producer birth fits the fixture grant`: they drive `semio_plugin_decl_app` `admit_op_binary_producer` (decl-job op-binary, not this lane) with a one-turn birth helper; the producer birth now takes several turns. Needs a harness loop; not caused by this port.
- `command/🎟️typed/📨️wire/🧪️tests` and `🎟️typed/🧪️tests` are not registered as targets; not compiled or run.
- Latest-wins registry has no focused test yet (the instance `tool-job-latest-wins` harness covers it once callers are ported).

## Rerun
`"$C/1105b-tools/chk.sh" W-command-<n> 🌎️hub/Cargo.toml wasm -p semio-plugin-command`; `"$C/1105b-tools/chk.sh" W-command-<n> Cargo.toml native --tests -p semio-plugin-command`; `"$C/1105b-tools/runtime/tst.sh" W-command-<n> Cargo.toml native -p semio-plugin-command --lib -- original_latest_key`

STATUS: FIXED-verified W-command-3 (wasm hub) and W-command-4 (native)
