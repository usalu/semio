# 📓️ Lease request — T1: regenerate `semio-framework-actor` TS bindings

**Requesting**: sol/registrar runs the typegen regen for `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
so `ActorMetricsSample`/`ShardMetricsSample`/`RuntimeMetricsSnapshot` (new in packet T1) land in
`🤖️generated/🟦️actor.ts`, which `🎭️actor/🟦️component.ts` re-exports (`export * from
"./🤖️generated/🟦️actor.js"`).

**Why not done directly**: `🤖️generated/**` is registrar-only per `📌️important.md`.

**Pre-existing gap, not new**: `🤖️generated/🟦️actor.ts` does not exist at all yet — `ls
🧰️framework/🔨️modules/🎭️actor/🤖️generated/` is empty. Typegen for this crate has apparently never
been run. This lease covers running it for the FIRST time, which will emit bindings for every
`#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]` type in the crate (all pre-existing types plus
this packet's 3 new ones), not just T1's additions.

**Command** (from `🎭️actor/🟦️component.ts`'s own doc comment):
```
bun nx run @semio-tech/framework-actor-rs:typegen
```

**Verification once run**: `ActorMetricsSample.export()`/`ShardMetricsSample.export()`/
`RuntimeMetricsSnapshot.export()` are already added to `component.rs`'s
`exports_typescript_bindings` test (region `🔖️Typegen`) — that test currently only runs under
`--features typegen`, deliberately never part of the plain `cargo test` path (see the feature's own
doc comment in `Cargo.toml`).

**Until this lands**: `🧵️shard-client.ts` and `🎠️kernel/🟦️component.ts`'s new T1 code use hand-
authored "stand-in" interfaces (`ShardMetrics`/`ShardMetricsSample` in `shard-client.ts`,
`RuntimeMetricsActorRow`/`RuntimeMetricsSnapshot` in `kernel/🟦️component.ts`) — the SAME pattern
`shard-client.ts` already used for `ShardBudget`/`ShardJobBudget` before typegen for THIS crate ever
landed. No consumer is blocked; this lease is a follow-up quality/parity item, not a hard dependency.
