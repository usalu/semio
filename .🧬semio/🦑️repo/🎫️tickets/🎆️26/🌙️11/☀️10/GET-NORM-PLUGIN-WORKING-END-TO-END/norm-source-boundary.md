# Window-Config Source Boundary

## Symptom

The `norm` component build was refused during source admission:

```
refused process.source.read-bytes-exceeded {"file":".../🪟️window/🎚️config/🧭️owner-registry/🌱️registration/🦀️.rs","observedBytes":"7508","maximumBytes":7185}
```

## Cause

- **Where the limit comes from.** Source admission (`🏗️native-build/📥️source/🎟️admission`) reads each crate's cfg-active source closure up to `min(policy, crate boundary)`. The global policy (`⚡️caching/🔣️policy.json`) is 262144 bytes. A crate's `rust-source-boundary` can only lower that limit, never raise it.
- **Which crate failed.** The failing unit is `semio-plugin-window-config` (559/743). The file named in the refusal is just the read that crossed the remaining budget.
- **What pushed it over.** A concurrent agent added test-only fixture-lifetime code to `🎚️config/🧭️owner-registry/🦀️.rs` (Nov 19 01:21). That code was ungated inside a production module, so it counted toward the production closure and pushed the crate about 0.3 KiB over.

## Fix

The fixture lifetime moved into its own cfg-gated module, `🧭️owner-registry/🧸️fixture/🦀️.rs`, which is declared as `#[cfg(any(test, feature = "artifact-app-testing"))] mod fixture;`. It contains `FixtureLifetime`, `FIXTURE_BANK`, `FIXTURE_TURN`, `fixture_partition_authority` and `fixture_close`. Production builds no longer read or compile the test fixture. The behaviour is unchanged.

Two earlier attempts raised `maxSourceBytes` in the window and window-config crate manifests. They had no effect because the policy minimum wins, and both were reverted.
