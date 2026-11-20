# Test drift in native-owned, native-decoding and derive

Draft only: nothing in the repo was edited, nothing was built. Apply with `python3 apply-test-drift-fixes.py` (idempotent, `--dry-run` supported; verified on a mirror copy and dry-run against the live tree, all anchors match). Then run the three suites through `repo`.

## 1. native-owned: source-size budget (library was right, structure was wrong)

- Mechanism: `[package.metadata.semio.rust-source-boundary]` in the crate Cargo.toml (`maxSourceBytes=262144`) is enforced by the native-build source admission (`📥️source/🎟️admission/🟦️.ts`), which sums the bytes of the whole `mod`/`#[path]`/`include_str!` closure of each compile unit.
- Measured offline: the library unit (`📦️packages/🦀️rust/🦀️.rs`, cfg(test)) is 265,222 bytes, over by 3,078; `receiver-driver.rs` is the last file read, hence the 13,557 left.
- Fix, no limit raised:
  - `receiver-driver.rs` becomes a facade over co-located `receiver-driver/turns.rs` (everything the library tests use) and `receiver-driver/scenario.rs` (`bank`, `settle`, `receive_faulted`, `lifecycle`); all other includers keep the same API.
  - New `🧪️tests/📥️admission/unit.rs` exposes only the turns to the library unit; `🦀️.rs` there points at it.
  - Inline wire-schema test (plus its 1,472 byte JSON) moves from `⚠️refusal/🦀️.rs` into `⚠️refusal/🧪️tests/🦀️.rs` as a receiver-driven law; ordinal and out-of-range checks of the private `variants` are dropped.
  - Driver indentation normalised to the repo's one-space style.
- Result: 260,467 bytes, headroom 1,677. Other units that include the facade grow by about 260 bytes; none was measured near the limit.

## 2. native-decoding: tests were stale

- `eff.rs`: `ControlledRetirement::step(body,effect)`; added `BODY=4096` and a reply-shaped `settle`. Same stale call fixed in `🏭️construction/📦️backing/🧪️tests`.
- `native-continuation`: decode control `close_retirement_recipient(body,effect)`; the encode control only has `close_retirement_recipient_into`, so the encode arm goes through `eff::settle`. Both `decode(...)` calls and `invalid.rs` gain the `None` Source witness argument.
- `♻️retirement/🎟️continuation`: `RetirementStep` work is u128, so compare with `u128::from(...)`.

## 3. derive: tests were stale

- Clone expansion is wrapped in `const _: () = {...}`; the oracle now descends into that block.
- Projection ports are `demands`/`advance_work_demand` (4 inputs, `maximum_work_units`) and `advance_into` (6 inputs, `maximum`); test updated.

## Unverified / follow-up

- No compiler ran: encode-arm denial behaviour (`Pending` vs `Complete`) and later errors hidden behind the first ones are unknown.
- Same stale `step(effect)`/`close_retirement_recipient(effect)` pattern remains in `🛫️encode/🛂️allocation/🧪️tests`, `🛫️encode/🧪️tests/♻️close`, `🛬️decode/🏭️construction/📦️effect/🧪️tests/original-effect.rs`, `🛬️decode/🛂️allocation/🧪️tests`, `♻️retirement/🧪️tests` and `🫴️recipient/🎟️work/🧪️tests`; not patched because their receiver types were not confirmed.

## Flow catalogue ownership

The catalogue ownership law from the handoff is now verified independently. The compiled test initially failed because a reusable callback was declared `Fn` even though it advances a mutable catalogue owner. The fixture now accepts `FnMut` and pays for the fixture caller's terminal retirement. Pinned Cargo compiled the three native dependencies for the actual law. The pinned compiler linked that law against their exact Cargo fingerprints, and the executable passed 1/1. This result covers catalogue derivation and closure under the turn's original Source. The larger Flow host law suite has a separate build history.
