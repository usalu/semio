# instance-jobs staging (reserved, capture, declarations)

Baseline (ijobs-a-1.log): 35 errors in my three areas (others' files had none reached yet).

## Changes
- reserved: decoder `(NativeAllocationIdentity,u64)`; `cache_roots` is `CacheRoots<A>=(ArcLease<Snapshot>,ArcLease<Config>,ArcLease<HistoryView>)` with `snapshot_root()` / `take_cache_roots()` accessors (render `reserved-jobs` consumers updated at two lines); action wire advance/return via `funded`, wire close via `retired`, final lane close via `retired_poll` (`step_granted(body,effect)`/`begin_granted(..,effect)`); slot-close outcome pinned to `ValueError`.
- capture: owner alias return, registry and catalog close threaded through effects (`close_original_retained_effect_step`/`close_original_effect_step`); `units` array resized to 2.
- declarations: `OriginalDeclaredTools::close_step(effect)` and registration trait close via `close_original_retained_effect_step`.

## Verification
ijobs-a-2/-3 logs: `semio-plugin-retained-command` (other agent, `full-operation/producer:22`, `ResidentConstruction<ErasedToolJob>` passed to `build_tool_job`) fails first, so instance-jobs was not reached; my edits are unverified by the compiler. Rerun once retained-command compiles.
