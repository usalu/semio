# Artifact Space Collection `--lib`: Final State

Prefix `🧰️framework/🛍️products/💻️os/🔨️modules/`. Last measured run: 23 pass, 6 fail (was 22 / 7). Runs via `stale-cargo.sh cf*` (logs deleted).

## Fixed
- `builtin_creation_and_reload_publish_io_without_manual_registration` passes. Cause: `ArtifactStore::close_owned_unscheduled` (`🏪️store/⚙️engine/📥️lifted/🏪️store/🌱️runtime-seed-construction-and-owned-close`) built a fresh `AuthoringWallet` per close turn, so the captured shared original saw a new Work binding (same root cause as `store-close-ticket.md`). It now holds one resident wallet for the whole loop (`close_owned_step_resident`); `close_owned_step_cold` keeps its one-shot wallet.
- Cancel cause mapping: a native "canceled" error that is only the consequence of the settlement observer refusing is no longer the primary. `stage_failure` (`🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-retirement`) and the test mirror `staged` (`🪐️space/🧪️tests/🪶️sqlite/🫴️native-caller`) now report the settlement's own cause ("relational SQLite transfer cancelled") and keep the operation's retained progress.
- Collection receiving macro (`🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving`): settlement refusals now carry the body wallet / owner progress.

## Still failing (6)
1. `cumulative`, `complete`, `refusals`: heap (232202,198106) vs receipt (34617,521); constant unreceipted 197585 B requested and released in the observed window. A thread-local tap on the trace allocator saw only about 76 small allocations (about 41 KB), so the figure is not from the thread-local observer scope as read; not located. `refusals` also still shows zero progress on an error built outside the macro tail.
2. `boxed_custody` mode 2 (native cancel inside the body): the operation completes and publishes ("canceled boxed body cannot publish"). The sql callback sets the native flag false but no native checkpoint follows in the body. Hidden before behind the progress mismatch; not fixed.
3. `cancellation`: the post-failure probe step returns `Ok(Progress)` instead of a Canceled failure.
4. `public_borrowed_native_preflight_admits_actual_owner`: `retire_inputs` (`🔨️modules/🚪️io/⏱️control/🛫️snapshot`) refuses with "snapshot retirement exceeds original physical grant"; which grant axis is not measured.

## Notes
- Temporary `[DEBUG]` allocation hook in `🔨️modules/⏱️trace/🧮️memory` was added and fully removed.
- Concurrent edits by others (a `settling` flag in `NativeDecodeControl`, `invalid(cause)` in arc-custody factory) broke the build twice during this work; both resolved by their owners.
