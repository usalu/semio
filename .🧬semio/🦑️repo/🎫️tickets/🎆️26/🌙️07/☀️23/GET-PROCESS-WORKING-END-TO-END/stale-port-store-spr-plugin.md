# Stale Port: store, spr, plugin, space and mcp test targets

Build: `stale-cargo.sh <label> ... --message-format short` (native `build-domain`, 3 jobs). Per-slice detail: `stale-port-codec-archive.md`, `stale-port-host-runtime.md`, `stale-port-collection-caller.md`.

## Shared test helper

`🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🪙️close/🦀️.rs` (mounted `#[cfg(test)]` from the space-history, codec, archive and mcp crate roots): `close_encode` (reply-slot close under an invoked `AuthoringWallet`), `close_decode` (birth-aware), `retire` (ControlledRetirement), `with_port`, `WorkStock`, `ConstructionStock`, `funded*`, and a `Mutex` backed `Cell` for Send authorities.

## Results (cargo check --tests clean for all nine crates)

| Crate | Before | After |
| --- | --- | --- |
| `🌉️mcp` | 3 (lib needs `SEMIO_BUILD_MODE`), then 30 | check clean; lib 116 pass, 10 fail before the fixture fix (9 fixed by syncing `🧪️tests/🧫️fixtures/host-operation-policy.json` with the canonical config, not rerun: the rerun is blocked by another agent's `store-initialization-replay` compile errors); `original_probe_pack_receiving...` still fails closing a cancelled owner (`original failure storage exceeds its retirement grant`) |
| `🏪️store/👥️presence/📸️snapshot` | 3 | check clean; 1 of 3 pass |
| `🏪️store/📜️space-history` | 11 | check clean; 4 of 9 lib pass |
| `🏪️store/📦️codec` | 16 | clean, 7/7 lib and 2/2 integration pass |
| `📡️spr/🧵️channel/📦️archive` | 27 | clean, 2 of 5 pass |
| `🔌️plugin/🖥️host` | 17 | clean, 23 of 25 pass |
| `🔌️plugin/🧠️interpreter/🏃️runtime` | 14 | clean, 8 failing (4 need an unbuilt wasm component) |
| `🪐️space/🗿️artifacts/🗂️collection` | 12 | clean, 15 of 29 pass |
| `🪐️space/🧪️tests/🪶️sqlite/🫴️native-caller` | 14 | clean (no tests of its own) |

## Product fixes

- `.cargo/config.toml`: `SEMIO_BUILD_MODE` defaults to `prototype` (`force = false`, launcher-issued values win), so a plain `cargo` compiles `env!("SEMIO_BUILD_MODE")` users zero-touch.
- Presence JSON `effect_step` (`🏪️store/👥️presence/📸️snapshot/🔤️json/🦀️.rs`): the JSON cursor step reported default progress while its effect marker held the real 392 B page; it now returns `cursor.normal_step_progress()`. This was the 'native physical marker differs from retained child capacity' failure (fixes the parse stage; the 392 B ledger gap the collection fork saw may be this or a sibling).
- `HeadlessWorkspace::has_open_probe` (mcp workspace) replaces a test reaching into the private `open_probes`.
- Host/runtime: `OriginalShardIdentity` returns its allocation authority on drop; boxed `PluginHostError::OriginalTurn` and `TurnFault::Guest` (see host-runtime note).

## Ports by file

- presence json unit tests: decode close `close_retirement_recipient(body, effect)`, encode close through reply slot (`close_encode`).
- space-history: `validation` (work authority, close macro parameter, misplaced `Recipient` impl moved into its test), `json-recovery` and `partial-projection` rewritten on `OriginalConstructionAuthority`/`OriginalWorkAuthority` loans and `native_snapshot_turn`.
- Removed as obsolete (their recovery protocol no longer exists: `allocation.unwind`, `recover_post`, `resolve_forwarded_allocation_callback`): `original_projection_recovers_allocation_after_funded_close_and_unwind` with fixture `🧫️fixtures/🪢️allocation-recovery`, and the wrong-authority recovery block of the JSON frame test (renamed `..._retains_its_cursor_through_a_receipt_callback_unwind_until_funded_close`).
- mcp: `fixture_text!` macro calls, `pack::` crate path, `serde` and `semio-os-spr/protocol-laws` dev-deps, `EntityIdentityAuthority` for `guest_apply_ops_binary`, `SharedUtf8`/`PagedBytes` envelope fields, probe pack receiving test on loans and output slots.

## Open (product side, not test shape)

- Decode close under a forwarded allocation port: `forwarded retirement allocation requires actual receipt admission` (`🌱️value/🛬️decode/♻️retirement/🦀️.rs` `charge`); forwarded encode `charge` outside a quoted effect. Archive (3), host activation (1), collection (2+2), codec forwarded-ledger laws.
- Presence: after the parse stage, `original projection must be driven or returned through its resident frame`; cancellation yields `UnsupportedOwner` where `Canceled` is asserted; a 2544 B heap residue per receive turn (invocation handoff) breaks the 'no leak' assertions.
- space-history: validation publication-refusal (`recipient.refused` never set), json receipts, partial projection (`armed` never set, `result.is_err()` fails), zero-allocation workspace: the receipt-refusal and cancel injection paths do not fire under the new construction loan.
- mcp probe pack receiving: cancelled-owner close refused with `original failure storage exceeds its retirement grant`.
