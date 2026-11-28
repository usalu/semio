# History preview cleanup issuers (2026-11-27)

## Check
`selected preview retains its original base cleanup issuers` in `original_retirement_step_with_control` (`🧰️framework/.../🔌️plugin/⏪️time-travel/♻️retirement/🦀️.rs`): with `preview_cancelled` and `preview_base` set, `snapshot_issuers` must still hold the snapshot and mutation retirement factories that `admit_original_snapshot_alias` needs.

## Root cause
`step_preview` stores `snapshot_issuers` first and `preview_base` right after; the base then lives across many turns of a selected (not yet cancelled) preview. The ordinary retirement drive (`retire_step`, run for pending work such as aliases of earlier snapshots) reached the unguarded branch `snapshot_issuers.take()` as soon as no alias was left, and moved the issuers into `snapshot_issuer_close` while the preview base was still live. A later Discard (or accept/cancel) set `preview_cancelled`, found `preview_base` Some and `snapshot_issuers` None, and refused with UnsupportedOwner. It depends on whether an alias retirement turn lands between preview base creation and the discard, hence the 1-in-4 flakiness (and the boot-time ordering of the latest rebuild).

## Fix
- `♻️retirement/🦀️.rs`: the issuer hand-off (`snapshot_issuers.take()` into `snapshot_issuer_close`) and its demand quote only apply while `preview_base.is_none()`; the base's own cancelled-branch consumes the issuers first, then the hand-off runs.
- `🧱️store-owners-and-member-visitors/🦀️.rs`: `has_pending_work` no longer counts `snapshot_issuers` while a preview base holds them (no idle spin; `terminal_is_empty` unchanged).
- Test: `♻️retirement/🧪️tests/🟦️.ts` "original snapshot issuers stay with a live selected preview base until it is retired" (passes; 7 other tests in that file already failed on stale source-text expectations, untouched). A native Rust test needs a full store/preview fixture and `semio-plugin-tests-14-2` does not compile today (peers' in-flight modules), so none added.

## Verification
Rebuild `NATIVE=1 sh rebuild-sourcing-retry.sh semio-hub-sourcing-component 2` (attempt 0 killed by SIGTERM on rustc, attempt 1 ok, staged; the script restarted 6081). History journey runs on the rebuilt hub (logs `🗑️generated/pc-hist-<lang>-<n>.txt`): see result below.

Result: 5 consecutive history journey runs each in en and de on the rebuilt hub, all exit 0, 0 FAIL lines, 0 'retains its original' refusals (history edit accept/discard passed in all 10). No target dir was created by me (no native cargo build was run), nothing to delete.
