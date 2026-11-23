# Stale Test Port: Encode, IO and Stdio Crates

Detail per crate lives in `stale-port-schema-sqlite-stdio.md`, `stale-port-control-snapshot-failure.md` and `stale-port-control-tests-crate.md`. Build: `stale-cargo.sh`; final compile sweep: `stale-final-check.sh`.

| Crate | Before | After |
| --- | --- | --- |
| value/encode | 9 errors | compiles, all 54 tests pass |
| io/control | 67 errors | lib tests compile, 30 of 35 pass; `typed-settlement` does not compile |
| io/control/tests | 60 errors | compiles, 24 of 56 pass |
| io/schema | 6 errors | compiles, 5 of 5 pass |
| io/sqlite-snapshot | 7 errors | compiles, 5 of 8 pass |
| os/io | 2 errors | compiles, all pass |
| stdio txt | 47 errors | partly ported, lib tests do not compile |
| ifc any | 3 errors | not ported (blocked by in-flight edits elsewhere) |
| binary raw | 2 errors | compiles, 6 of 6 pass |

## Encode (done by me)

- `🧪️tests/🎟️eff/🦀️.rs`: new `close_with` (original invocation, grant copy bytes as body hint) and `close_encode` (local reply slot around `close_retirement_recipient_into`).
- `🛂️allocation/🧪️tests/🦀️.rs`: `close_step(body, effect)` arity, `close_with`/`close_encode` in place of `close_retirement_recipient`, `native_header_release` on the fixture owner.
- `🧭️span/🦀️.rs`: `new_recorded_source` takes the witness.
- `🦀️.rs` test: `OriginalWorkAuthority::with_loan` replaces the removed issuer `loan`.
- Product: `NativeEncodeControl::charge` on an idle forwarded control refused as documented (`UnsupportedOwner`) instead of calling the port and committing.

## IO control (octets by me, rest by forks)

- `📦️octets/🧪️tests/🦀️.rs`: recipients are `OriginalWorkRecipient`/`OriginalConstructionRecipient` behind `OriginalWorkAuthority`/`OriginalConstructionAuthority::with_loan`; the decode test reuses the encode fixtures; `close` runs `ControlledRetirement::step(body, effect)` inside `EffectBank::run_outcome`.
- Products fixed by forks: `NativeSnapshotFailure` fields inline again (was one unreceipted 2.5 to 3.6 KB box per conversion), encode allocation `reset_completed`, `begin_original_settlement` maximum work, `FaultScope` retirement cursor field-wise (no unreceipted 224 B sequence), `io_registry::register` restored.

## Open

- control: five lib failures (dual-control heap +108 B, canceled nested decode close, `first-copy-receipt` receipt never reaching the wallet, header denials fixture law), `typed-settlement` needs a work-recording API on `NativeSnapshotEncodeSettlement`.
- control tests crate: 32 failures (forwarded direct storage, unreceipted nested allocations, removed settlement flows, unfunded drive scope).
- sqlite-snapshot: refused-receipt and pre-work-denial settlement not released, blob stage work 1 vs 4.
- txt/ifc: need the control lib to build and the txt laws to be ported.
