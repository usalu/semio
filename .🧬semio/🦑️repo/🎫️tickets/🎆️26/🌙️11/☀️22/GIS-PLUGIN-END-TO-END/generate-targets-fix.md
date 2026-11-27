# Generate/typegen target failures

| Target | Real error | Cause | Fix | Result |
|---|---|---|---|---|
| framework-async-rs:typegen | `the package 'semio-framework-async' does not contain this feature: typegen` | Script ran `cargo test --features typegen` on the async crate manifest; the feature no longer exists, the `exports_typescript_bindings` test lives in companion crate `semio-framework-async-tests` | `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts`: both typegen calls (typegen, preview-generated) use the companion manifest via `typegenManifestPath`, no feature flag | `repo run ... --lang=en` exit 0, bindings regenerated |
| framework-actor-rs:typegen | host-warden cancel (critically low memory), not a code defect | parallel cargo on a 16 GB Mac | none (target and `[[test]] typegen` agree) | pending, see below |
| framework-rs:generate | host-warden cancel (same) | same | none | pending |
| ui-contract-rs:generate | memory-related cargo lines in the log; warden cancel not confirmed | same | none (`typegen` feature exists in Cargo.toml) | pending |

Feature-vs-target audit: only `ui-contract` declares a `typegen` feature (used by its script); `os` declares an empty one; actor uses `--test typegen`. No other target passes a missing feature.

Remaining three are serialized by `gen-run-rest.sh` behind the shared compile lock (held by another lane); results in `🗑️generated/gen-results.txt`.
