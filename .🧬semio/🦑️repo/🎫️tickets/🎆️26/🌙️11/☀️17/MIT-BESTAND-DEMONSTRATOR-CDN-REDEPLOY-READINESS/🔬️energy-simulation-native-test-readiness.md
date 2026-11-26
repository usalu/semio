# Energie Simulation Native Test Readiness

The previous native sim library test stopped at seven compile errors caused by stale fixture use of receiving and close APIs and a removed wire writer field. The current test-only retirement fixture uses the issued native receiving work scope, verifies its original outcome, posts the returned construction-step receipt and uses the current retained close effect API. The page-birth fixture now waits for the actual retained payload builder to initialize before asserting how a prior page grant affects birth.

The existing portable numerical handoff test passed in the previous managed native build. Its receipt log records one passing test, and the owner has since reclaimed that test executable. The updated sim library fixtures still need their actual native execution.

The first locked native planning attempt failed because the Energie workspace lock predated two geometry manifest dependencies. An offline Cargo alternate-lock plan confirmed only `semio-framework-value` additions to geometry and geometry solids. The Energie workspace lock now records both. The pinned `--offline --locked --unit-graph` query resolves 299 units for the sim library and standalone numerical handoff test together. The Hub release lock was outside this change.

The exact affected paths are:

- `✏️s/🔌️plugins/🔋️energy/Cargo.lock`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️close/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧺️batch/🧪️tests/🦀️.rs`

No native test was launched while the priority release pair held the shared disk lane. The bounded managed command for the next available compiler slot is:

```sh
SEMIO_OFFLOAD=never repo cargo --store=shared --configuration=prototype --parallelism=1 -- test \
  --offline --locked --no-fail-fast --manifest-path '✏️s/🔌️plugins/🔋️energy/Cargo.toml' \
  -p semio-s-plugin-energy-engine-sim --lib --test numerical-admission-handoff
```
