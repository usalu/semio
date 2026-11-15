# Kernel Rust Tests Ported to the Current Receiving and Retirement API

## Result

- `repo run @semio-tech/framework-rs:generate --skip-remote-cache`: exit 0.
- `repo run @semio-tech/framework-rs:test --skip-remote-cache -- --test kernel-event-receiving --test original_kernel_retirement`: nextest "2 tests run: 2 passed, 0 skipped". The command itself exits 1 only because the forwarded `--test` flags are also handed to the vitest half of that target ("Unknown option --test"); the cargo half is green.
- `repo run @teaching/architecture-course:test --skip-remote-cache`: exit 0, 19 test files, 849 tests passed.

## Recovery

The earlier port was recovered from subagent transcript `agent-a703278dda73f0886.jsonl` (Write of the events test, Python patch of the retirement test, Python patch of Cargo.toml and Cargo.lock). The dev-dependency and lock entry were gone, so both were re-applied. That port only compiled; it failed at runtime, so both tests were reworked below.

## Changes

- `🧰️framework/📦️packages/🦀️rust/Cargo.toml` and `🧰️framework/Cargo.lock`: dev-dependency `semio-framework-value-native-owned` re-added.
- `🧰️framework/🔨️modules/🎠️kernel/📥️events/🧪️tests/🦀️.rs`: uses `receiver-driver.rs` (`admit`, `lifecycle`), `EffectBank` with the law `stock` wallet; same law fixture, same serde-equivalence oracle, invalid rows must refuse, cancel cuts, and the wallet heap-delta balance assertion.
- `🧰️framework/🔨️modules/🎠️kernel/♻️retirement/🧪️tests/🦀️.rs`: `StepContext::new` replaced by `OriginalJobFixtureCaller` (it supplies the parent Source that `StepContext::new` lacked). Per owner and step: a zero-fuel caller must perform no work and no heap, denied capacity and release grants must progress nothing and not change the cumulative receipt, the funded step must have heap equal to receipt, and born equals released. The Source root is now closed by `caller.retire(stock)` (replacing the hand-rolled `CancelTokenRetirement` loop); the law grant is still the per-step grant.
- Product defect fixed: `🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/📄️command-page-behavior/🦀️.rs`: `FixedCommandPage::storage_demand` quoted `copy_bytes: 0` while its storage admission delegates to `Vec<u8>`, which needs `size_of::<Vec<u8>>()` copy bytes. A quote-sized grant therefore never admitted the page and the receiver stalled forever (found via the `commandIngressPage` law row). It now delegates to the `Vec<u8>` quote.

## Overwrite at 07:59

Not determined. Both files equal HEAD content and have mtime 07:59 with the directory. `/private/var/log/semio-fleet-replica.log` has no timestamps; it records one `overwritten` event (`Cargo.lock`, hlc about 05:15, origin g1-m04) and a full republish of 272537 files around that time. The log cannot be tied to 07:59, so the likely cause is a replica apply or archivist state from another machine restoring committed content (last-writer-wins). Unverified.

## Unverified

- The full `@semio-tech/framework-rs:test` suite (all packages plus vitest) was not run; other tests that assert `FixedCommandPage` demand quotes were not run individually.
- Temporary debug copy of the driver was removed; no `[DEBUG]` lines remain.
