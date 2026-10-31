# Current Energy Retained-Post Test Ownership

This is read-only preparation, not Cargo execution. Both deferred laws remain NOT EXECUTED. Current manifests were parsed with the independent @iarna/toml reader; exact Source hashes and path lengths are retained in generated/energy-post-owner-current.json.

The current Source supersedes the older JSON claim that the production snapshot lib mounts its state tests. Production semio-s-artifact-energy-model-snapshot resolves its library to snapshot/🦀️.rs and imports production state, but neither that library nor state/🦀️.rs mounts these two laws. The explicit model-workspace member semio-s-artifact-energy-model-snapshot-tests resolves its library to snapshot/🧪️tests/🦀️.rs. Its state module reexports actual snapshot::state production types, then mounts ../🧳️state/🧪️tests/🦀️.rs at line27. That file has unconditional #[test] functions:

- energy_state_original_zone_receive_projection_and_inverse_preserve_paid_authority at line29.
- energy_native_sparse_zone_path_rebuilds_only_its_pages_and_branch_and_restores_original_inverse_authority at line50.

The dedicated test owner depends on actual production snapshot/diff/model packages and keeps semio-framework-os-kernel features protocol-laws and mutation-testing plus the Native-owned testkit. It mounts the full snapshot law families (state, fields/key/read, original, Pack, genesis, reference roundtrip/composition), not only a helper. The production library retains its own carrier unit tests. One invocation selecting both packages therefore preserves the complete intended snapshot library/test-owner scope and includes both deferred laws without replacing production integration or raising any Source cap.

Run from /home/deck/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model only after coordinated capacity is available:

```sh
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=release --parallelism=1 -- test --manifest-path '🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml' --package semio-s-artifact-energy-model-snapshot --package semio-s-artifact-energy-model-snapshot-tests --lib --no-fail-fast -- --test-threads=1
```

There is no schema package, name filter, helper-only target, direct Cargo, feature disabling or boundary override in that command. Actual execution must later show both exact named laws with nonzero counts, all selected library tests unfiltered, and authoritative success; a prerequisite/admission refusal is not a law result. Source-boundary refusal must be diagnosed and decomposed by its real owner while retaining production integration/caps.

No manifest, library, law body or completion JSON was modified during preparation. The two stale ledger entries need current ownership reconciliation by their ledger owner before execution. Managed4262 and its readiness/browser/TSC obligations remain unchanged.
