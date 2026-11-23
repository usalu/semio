# Framework native codec test suites (decoding, outcome, encoding)

Workspace: `C:\semio\🧰️framework` (root `Cargo.toml`). Packages (under `🔨️modules/🌱️value`):
`semio-framework-value-native-decoding` (`🛬️decode`), `semio-framework-value-native-outcome` (`🛬️decode/📬️outcome`), `semio-framework-value-native-encoding` (`🛫️encode`).

Run natively (repo selects no store here): `CARGO_TARGET_DIR=C:\t\fw`, `CARGO_BUILD_BUILD_DIR=C:\t\fwb`, `CARGO_INCREMENTAL=0`, debug info off, `-j 6`, `cargo check --all-targets --keep-going`, then `cargo test --no-fail-fast`. Logs: `🗑️generated/codec-tests-*.log`.

## Result

- Before: lib tests / `native-continuation` target of all three crates did not compile (14 errors).
- After: all three crates compile with `--all-targets` and pass: 95 passed, 0 failed, 1 ignored (pre-existing `#[ignore]` in `typed-retirement`), doctests 0.
  - decoding: 8 lib + 7 backing + 5 continuation + 1 view
  - encoding: 36 lib + 9 action-work + 5 effect-settlement + 8 source-effect + 5 typed-retirement (+1 ignored)
  - outcome: 5 lib + 1 accepted-construction-proof + 5 receiving-failure

## Test-side API drift ported (no compat layers, assertions kept or strengthened)

- `step_into` / `recover_step_into` take `body` first: `🛬️decode/📬️outcome/🛫️encoding/🎟️effect/🧪️tests/error-retirement.rs` (body 4096, as the demand quotes use).
- `RetirementStep::Progress` work is `u128`: `🛬️decode/♻️retirement/🎟️continuation/🦀️.rs`.
- `NativeDecodeContinuation::decode` takes a `source` (`None`): `🛬️decode/🫴️recipient/🔁️continuation/🧪️tests/{🦀️.rs,invalid.rs}`; refused-work law gets a `hop!` macro for the per-direction call shape.
- Encoder close is reply-shaped (`close_retirement_recipient_into`) and `close_step` has a `body`: `🛫️encode/🛂️allocation/🧪️tests/🦀️.rs`, continuation test macro. New shared fixtures `eff::close_reply` (both eff files) and `eff::invoked` (decoder eff; `close_step` reuses it). Decoder close now runs inside the original invocation.
- `new_recorded_source` takes the Source turn witness: `🛫️encode/🧭️span/🦀️.rs`.
- Original Work is lent through `OriginalWorkAuthority::with_loan` (no `OriginalWorkActionIssuer::loan`): `🛫️encode/🦀️.rs` test module.
- Test `Owner` implements `native_header_release` (terminal ticket lifecycle is explicit now).
- Outcome work-quote test follows production `settle_closure_work`: settle the partial only when performed work is pending (zero-work row has no partial), `record_performed_before_post(.., pending, ..)`.
- Forwarded charge law: `forwarded_owned_storage_requires_the_quoted_effect` asserted a refusal that the current `charge` no longer makes (forwarded `charge`/`copy_text` are admitted by the original port as `Admit` events and counted on the ledger). Replaced by `forwarded_owned_storage_is_admitted_by_the_original_port_before_it_is_owned` (exact admit counts and ledger bytes).
- Cleanup births are posted by the caller after reading the reply: encoder via `commit_closed_births`, decoder loan via `charge` (test macro posts both and asserts ledger == before + births).

## Framework fixes (root causes)

1. `🛫️encode/🦀️.rs`: `NativeEncodeControl::commit_closed_births` was private and never called (dead code), so births of a reply-shaped recipient close were never posted to the cumulative ledger. Made it `pub` (post step after the caller read the reply).
2. `🛬️decode/🦀️.rs` `validate_continuation`: now also refuses an unresolved live Work action (`self.work.has_live_actions()`), mirroring the encoder's `validate_continuation`; this is the context refusal `original_decode_envelope_keeps_result_and_context_refusal` and the `callEnvelope.context` fixture require. Not caused by the other agent's field/construction/admission changes. native-owned re-run after this change, uncapped: exit 0, all binaries ok, 0 failed.

None of the failures traced to today's peer edits (`🏭️construction`, `🧳️owned/🪡️field`, `📥️admission`); drift was older.

## Forwarded `charge` without an active quote (coordinator question)

Left as is; not made to refuse. Production callers of `NativeEncodeControl::charge` / `NativeDecodeControl::charge` outside tests, all of which run on forwarded controls and rely on charge being the admission step:
- the controls themselves: `allocate_vec`, `copy_text`, `copy_bytes`, text/byte copies and the retirement wrapper charge (`🛫️encode/🦀️.rs`, `🛬️decode/🦀️.rs`);
- codecs: `🔁️codec/🛫️controlled`, `🔁️codec/🛬️controlled`, `🔁️codec/📦️container-codecs`, `🏷️type/🛬️controlled`, `📦️paged/🦀️.rs` (`reserve_encoded_backing`), via the `NativeDecodeCloseControl` / retirement-close loan `charge`.
These obtain their grant from the original port: a forwarded `charge` outside an active quoted effect begins a non-physical `Admit` transaction on the original allocation port and only then counts the bytes on the ledger; inside an active quoted effect the effect's quote already covers it. So the grant does come from the port's verdict, not from the control. Making it refuse would break every owned-storage path above (they have no quoted effect around them); a refusal would belong to a redesign that moves them to `*_into` quoted effects. The replaced test asserts the exact `Admit` count and ledger bytes instead. The decoder loan's own `charge` already refuses on forwarded allocation ("requires actual receipt admission").
