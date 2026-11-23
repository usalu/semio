# Forwarded Allocation Ports

Prefix `🧰️framework/🔨️modules/🌱️value/`.

## Decision

Fix the framework, not the callers. A forwarded allocation port already distinguishes two cases through `NativeAllocationLoan::matches_transaction`: inside a quoted effect (an open `Quote`/`Post` transaction) the effect owns the admission and a `charge` is a no-op; outside one, a `charge` is one historical `Admit` event. The decode control always worked this way; the encode `charge` (`🛫️encode/🦀️.rs`) and the decode retirement `charge` (`🛬️decode/♻️retirement/🦀️.rs`) had been changed to refuse instead. Refusing forces every synchronous owner (`OriginalShardIdentity::issue`, sqlite encoders calling `allocate_vec`/`copy_text`, owners born during a decode close) to be rewritten into resumable quoted effects, although they are plain bounded admissions with an exact physical post reported independently (the allocation module contract: "cumulative admission records requested bytes; physical posts independently report actual capacity").

A committed law test `forwarded_owned_storage_requires_the_quoted_effect` (`🛫️encode/🛂️allocation/🧪️tests/🦀️.rs`, in HEAD since 749, red at HEAD because HEAD's `charge` still admitted) pinned the opposite for encode: unquoted `charge`/`copy_text` refused with no port calls. It is replaced, not weakened silently: product encoders over forwarded controls (`plugin/🚪️io/🛂️authority`, `store/🧩️composition/🚪️open`, `spr/📜️history`, `mcp` guest document codec, shard identity) all call `charge`/`allocate_vec`/`copy_text`, so refusing would force every one of them into resumable quoted effects with no gain in exactness (admission is requested bytes; physical posts report actual capacity separately). The replacement `forwarded_admission_refusal_leaves_no_owned_storage_and_no_port_residue` keeps the invariants that matter: a port that refuses admission leaves `owned_bytes() == 0` and no storage created. If the owners want quote-only encode, the callers listed above must move to `allocate_effect` first and the old law returns.

## Change

- `🎟️control/🧬️contract/🛂️allocation/🦀️.rs`: provided method `NativeAllocationLoan::admit_unquoted(request) -> Result<bool>`; `false` when a quoted transaction is open, `true` after exactly one `Admit` event, refusals map to the exact `ValueError`.
- `🛬️decode/🦀️.rs`: `NativeDecodeControl::admit` (shared) used by `charge`.
- `🛬️decode/♻️retirement/🦀️.rs`: `NativeRetirementDecodeControl::charge` no longer refuses a forwarded binding; callback, `admit`, ledger commit only when the port admitted.
- `🛫️encode/🦀️.rs`: `NativeEncodeControl::charge` forwards through `admit_unquoted` and commits the local ledger as before.

## Laws

- `🎟️control/🧬️contract/🛂️allocation/🧪️tests/🦀️.rs` `unquoted_admission_records_one_admit_and_defers_to_an_open_quoted_transaction`.
- `🛫️encode/🛂️allocation/🧪️tests/🦀️.rs` `forwarded_charge_outside_a_quoted_effect_admits_once_and_a_quoted_effect_owns_its_own_admission`.
- Codec decode/encode `🚪️control` tests run every row twice, local ledger and forwarded port whose own ledger must equal `owned_bytes` (`rows(forwarded)`).
- Archive decode tests close pending owners under a forwarded port and assert `control.owned_bytes() == port ledger` after each close.
- Encode `commit_closed_births` (births of a closed retirement owner) now also reaches a forwarded port through the same `admit`, so `control.owned_bytes()` equals the port ledger after every close (this was the ledger gap of the codec forwarded encode row).

## Test ports that followed

- `🪐️space/🧪️tests/🪶️sqlite/🫴️native-caller`: the macro port models the full event vocabulary (`Admit` advances, `Quote` asserts the prior total, `Post` sets `owned + actual`, `Cancel` ignored), the same ledger rule the archive laws use. The old port counted `Admit` only, which produced the "392 B" gap of `borrowed_semantic_gate`, `erased_both_directions`, `native_input_retained`, `complete_native_semantic_text_roles`, `independent_sql_edit`, `independent_surrogate_renumber`; they now pass.
- Archive `original_composed_carrier_reservations...`: the success path closes the recipient (`close_in`) and retires the returned archive inside the port loan before the recipient drops; the protobuf oracle spawns `repo tools exec bun --` (bare `bun` is not on PATH).

## Restored assertions

| Assertion | Where | State |
| --- | --- | --- |
| codec decode and encode `🚪️control` forwarded-ledger law | `rows(forwarded)` runs every row against a local ledger and a forwarded port ledger | passes |
| workspace-receiving `turn.work == 1` | `📥️lifted/🪆️workspace-receiving` single `funded_invoked` close, `Pending` is a failure | passes |
| runtime heap equals progress, `demand == heap.requested` | `🎛️operation/📥️input/🦀️.rs` both loops back to `assert_eq!` | passes |
| runtime zero-grant step | `original_close::denied` runs `owner.step` inside a plain (non-widened) `AuthoringWallet::run` of a zero grant: no progress, heap `(0,0)`, owner untouched | passes |
| `[DEBUG]` tags | codec encode/decode `🚪️control` tests; the stage-failure formatter is gone, the message is the row id | done |
| error layout budget | measured `PluginHostError` = 64 B (the earlier boxing of `OriginalTurn` was enough, no ValueError boxing needed), `TurnFault` = 144 B (`OriginalActorFixedCall` is 144 B and must stay inline: the refused call is returned allocation-free); fixture is `64` / `144` instead of `80` / `160` | passes |
| deleted host mock-backed routed-call tests | not restorable: `Host startup requires an implemented original phased backend` (`🏗️instance/🦀️.rs`), a `MockGuestRuntime` cannot serve a routed call; needs the built neutral component | open |

## Results (`--lib`, run `fw-b5`)

| Crate | Result |
| --- | --- |
| `📡️spr/🧵️channel/📦️archive` | 4 of 5; `undergrants` fails by 108 B (see below) |
| `🏪️store/📦️codec` | 8 of 9; `pack_receiving...paid_close` fails by 108 B |
| `🔌️plugin/🖥️host` | 24 of 25 (`neutral_activation_failures` now passes); `2550_record_scale_fixture_registry` reads a fixture that is a three-case generator list, not a 2550 record registry |
| `🔌️plugin/🧠️interpreter/🏃️runtime` | forwarded and input laws pass; 2 clock tests fail as before, one `[DEBUG]` startup test belongs to the receiving agent |
| `🪐️space/🗿️artifacts/🗂️collection` | 21 of 29 (from 15) |

## Still open (not forwarded ports)

- A stray 108 B allocation, unreleased and unreported in progress, during receiving and archive decode steps (archive `undergrants`, codec `pack_receiving`, collection `boxed_custody`/`cancellation`). It appeared between my first and second runs while other agents edited `🧵️job/🛬️receiving`, `🛬️decode/🧳️owned`, `🧬️retained-clone/📦️original-slot` (carries `[DEBUG]` prints); my changes only touch forwarded bindings and these tests are local ledgers. Needs the receiving agent.
- Collection receiving laws `heap == progress` (`complete`, `cumulative`, `refusals`, `registered_full_file`): transient sqlite working allocations are not in the progress receipt (cumulative: heap `(232310, 198106)` vs progress `(34617, 521)`).
- Collection `builtin_creation_and_reload...`: `captured shared original changed its Work or destination` (receiving/slot work being debugged by another agent).
- Collection preflight: `snapshot retirement exceeds original physical grant` while closing the decoded specimen with `owner.remaining_grant()`.

## Framework workspace laws

`cargo test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-value-control-contract -p semio-framework-value-native-encoding --lib`: contract 42 of 42, encode 37 of 37 (both new laws included). The host layout law (`host_error_layout_matches_language_neutral_budget`, budgets 64 and 144) passes in the root workspace.
