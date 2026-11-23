# Value test targets port: five stale `semio-framework-value` integration targets

Build recipe: these targets are only reachable through the framework workspace (`🧰️framework/Cargo.toml`), because `semio-framework-value` is not a member of the root workspace. Command: `stale-cargo.sh <label> test --offline --manifest-path 🧰️framework/Cargo.toml -p semio-framework-value --test <target>`.

## Dev-dependencies added (`🌱️value/📦️packages/🦀️rust/Cargo.toml`)
`semio-framework-value-clone-source-testing`, `semio-framework-value-testing-effect-bank`, `semio-framework-value-testing-admission`.

## Per target
- `value-paged-record`: only needed the fixture crate (`FixtureSourceBirth`, `FixtureSource`) as a dev-dependency.
- `value-utf8`: `RetainedUtf8Buffer::close_step` takes `&mut RetainedAllocationEffect`; the helper runs it through the shared fixture effect (`♻️retirement/📦️owner/🧪️work/effect.rs`, included by path), work is `u128`.
- `value-paged-append-work`: imports of `PagedUtf8`, `PagedUtf8AppendCursor`, `PAGED_UTF8_CHUNK_BYTES`; `Bank` runs close turns through the fixture effect (`Bank::run`/`Bank::settle`), fuel is `u128`; `close_step(body, effect)`. Product: `PagedUtf8AppendCursor::{source,destination,position}` are now `#[doc(hidden)] pub` like `complete` (an integration test cannot observe the header otherwise).
- `value-publication`: `FactoryPayloadRetirement` fixture updated (`prepare_close_state_step`/`close_state_step` take `body, effect` and return polls, native work quotes, `close_state_terminal_native_release_safe`); `ControlledRetirement::step(body, effect)` and `FactoryAuthority::step` run through the fixture effect; denied/cached work asserts use `RetainedWorkOutcome::work` (`u128`).
- `value-paged-receiving` (+ `maps`): rewritten on the shared receiver driver (`🧪️testing/📥️admission/receiver-driver.rs`, `default-receiver.rs`): every case is denied on each axis, cut at 0/1/4/40/complete, then closed with `driver::lifecycle`; the finite stock comes from the fixture `receiveLaw.stock`.

## Product bugs found by the port (fixed in `📦️paged/🛬️receive/🗂️map/🦀️.rs`)
1. `advance_local` delegated to the value child for every phase once a binding existed, so the early phases (child not yet created) returned `invalid()` outside the reply and the begin-close stage (child already taken) reached a finished child. Delegation is now `phase==5` only.
2. `relay_close` stepped the child under `effect.nested(child_grant(g,0))` while its quote (`child.close_demands(body)`, and `close_step` itself) lends exactly `g`; the receive stalled for 1024 funded turns. It now lends `effect.nested(g)`.
3. The child turns (`advance_local`, `recover_advance_local`, `take`) ran on the parent's un-nested grant, so a denied parent turn still worked (3 and 1 work units observed). They now run under `effect.nested(child_grant(g,0))`, matching the quote `nested(child.demands(..),0)` and `relay_close`.

## Result
All five targets compile and pass: `value-paged-record` 1, `value-utf8` 4, `value-paged-append-work` 3, `value-publication` 3, `value-paged-receiving` 3. The whole `semio-framework-value` + `semio-framework-value-derive` package run (the command the neutral-owner twin executes in its reduced workspace) is green: 20 result blocks, 0 failures.

## TS twins
- `🏷️type/🧪️tests/🚮️absence`: 1 pass (4 nested type-law tests pass).
- `🧪️tests/🧩️neutral-owner`: 16 pass, 1 fail ("actual value derives build and execute with product trees absent"). The failure is environmental, not Rust: the twin copies the module tree to a random path and runs `cargo clean` + `cargo test` there inside a hard 120 s test timeout; with a shared, lock-serialised build dir that always times out (each copy forces a full rebuild). With the artifact dir under the repo tree it instead fails earlier because the repo `.cargo/config.toml` routes `rustc -vV` through the compiler-ingress script. The equivalent real Rust run above is green.

## Rust tests of the `effect.nested(grant)` swap crates (run in the framework workspace)
- Green: `semio-framework-value-native-receiver-protocol`.
- Not caused by the swap (stale tests, not touched here): `semio-framework-value-retirement` 7 failures all `ControlledRetirement::new` allocating 108 bytes where the test expects 0, or source fixture heap drift; `semio-framework-value-clone-source`, `-factory`, `-shared-issuance` lib/integration tests do not compile (old `close_step`/`FactoryPayloadRetirement` signatures); `semio-framework-job` `original-work-grant` missing `OriginalJobFixtureCaller`; `semio-framework-value-native-owned` 6 failures from a missing `🔗️sealed-shared-receiver/🧪️tests/🧫️fixtures/📐️.json` (fixture/schema swap); `semio-framework-pack-format` 2 failures (compressed/chunked round-trip).
- Not run (not in the framework workspace or no cheap target): the os plugin shard event, window epoch, icon paint cache, replication canonical seal, fem material patch.
