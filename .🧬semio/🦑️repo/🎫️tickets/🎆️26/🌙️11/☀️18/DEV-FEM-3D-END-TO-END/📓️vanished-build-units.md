# Vanished build unit `proc-macro2/ddca0b4202affb52`

## Verdict
Deleter = suspect (1), the cargo unit GC (`collectUnitsV1`), not retention and not cargo.

## Evidence
- `prototype/build/proc-macro2` mtime is 2026-11-19 00:01:34; nine other crate dirs (anyhow, prettyplease, quote, serde, serde_core, serde_json, wit-bindgen-rust, wit-bindgen-rust-macro, zmij) changed within the same second: one sweep removing about 10-12 units. The build ran 23:26-00:48, so the sweep hit mid-build.
- `u/gc.log` shows a collection with `units:12` (and `superseded:96`); `fill.jsonl` shows the 30 min collector cadence (23:57, 00:27). The log has no per-path ledger or timestamps.
- Cargo does not delete old hash dirs; retention does not touch `build/<crate>/<hash>` of a lane.
- Why it was allowed: compilers hold the profile build lease per compilation unit (`compilerLockGranularity: cargo-compilation-unit`), so between units the profile is free, the collector takes it exclusively and any unit idle over 30 min whose `.lock` is free goes. Fresh (reused) units are not touched by cargo, so they look idle. The lane live marker (`u/l/<lane>/v/<pid>-*`) was never consulted by `collectUnitsV1`.

## Fix
- `🧱️units/🟦️.ts`: new `profileLivePortV1` (lane of a profile has a live holder); `measureUnitsV1` marks all units of a live lane `held`; `collectUnitsV1` skips live profiles at planning and again at removal; optional `live` port for injection.
- `🧵️lane/🟦️.ts` `laneLiveV1`: a holder with a live pid stays live however stale the heartbeat (a long sync snapshot would otherwise drop the marker and expose the lane). Matches the existing committed test.
- Test: units tests, "a lane with a live holder loses no unit ..." (13 pass).

## Provenance snapshot
Not changed. The vanished file belonged to the build's own reported units, so the exit 75 restart was a correct detection of a real violation; ignoring disappearances would mask races. The fix removes the cause.

## Open
- `🧪️tests/🫀️live-holder` still fails its `listLanesV1` assertion that reserved lane `d` is not listed (pre-existing, separate).
- `🧪️tests/🟦️.ts` unit-volume test fails on Windows (`os.statvfs` oracle), pre-existing.
