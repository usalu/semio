# race_deadline usage audit

## Grep

`race_deadline` / `Race` appear only in:

- `plugin/host/⚡️effects/🦀️component.rs` — definition (`Race`, `race_deadline`)
- same file — sole call site `dispatch_storage` (~847)
- same file — unit tests `an_effects_deadline_is_enforced_and_the_loser_is_cancelled`, `race_deadline_returns_the_primary_result_when_it_finishes_first`, helper `futures_lite_block`
- ticket logs/reports (historical)

No `HttpPool` / `ComputePool` call sites use it. No re-exports elsewhere under `plugin/`.

## Conclusion

Safe to remove `race_deadline`/`Race` and match `StorageError::DeadlineExceeded` from `ticket.await_result()` directly.
