# Energy Retained-Post Owner and Dependency Trace — 2026-10-29

## Corrected test ownership

The two pending laws are **not owned by semio-s-artifact-energy-model-schema**:
- `energy_state_original_zone_receive_projection_and_inverse_preserve_paid_authority`
- `energy_native_sparse_zone_path_rebuilds_only_its_pages_and_branch_and_restores_original_inverse_authority`

Both are actually mounted by **semio-s-artifact-energy-model-snapshot**, manifest:

`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml`

Its actual lib entry resolves to snapshot/🦀️.rs, which mounts state; state mounts its tests under cfg(test). The canonical cfg-selected source reader selects both exact function declarations in `snapshot/🧳️state/🧪️tests/🦀️.rs`. Evidence `🗑️generated/energy-snapshot-mounted-laws.json` is diagnostic only (50files/361278bytes), not compiler admission. No source cap was changed.

The prior schema package's actual cfg(test) closure has only the unrelated async sanity tests `empty_snapshot_matches_schema` and `example_fixture_parses`. Therefore prior Root18221 cannot verify the pending two laws even if its dependency issue is repaired. Both completion-ledger rows now point to the true snapshot manifest/package/argv and explicitly retract any prior schema-root verification implication. Their actual runtime remains **NOT EXECUTED**.

## Authentic prior Cargo selection

Retained actual memo `3a3c4b5ac00aec9f9632bb7559f23c855506b008697382ca1e4094375e6a5eb7.plan.json.gz` has exactly one root422: schema library/mode=test.499units/412workspace. Shortest actual path:

`Energy model schema (test) → OS kernel (normal build) → OS store-engine (normal build)`

Evidence `🗑️generated/energy-captured-selection.json`. Root18221 stopped before compiler on the known genuine engine production byte boundary (18130byte next file/13708remaining). No Energy runtime law executed. Broad metadata did not cause a whole-workspace compiler selection.

## Genuine production integration; no neutral-harness shortcut

Schema's **normal** dependency already includes OS kernel, and its production artifact has typed ArtifactLink fields. Snapshot likewise has **normal** kernel dependency. Its production root implements `store::os_store::serialization::ArtifactStoreSnapshot` and creates actual `OriginalSnapshotPackView` objects. Snapshot also depends on real artifact window-kit/composition/registry owners.

Even a hypothetical removal of only schema's direct kernel edge leaves the authentic graph route:

`schema → energy model diff → OS kernel → store-engine`

Thus replacing a test helper cannot remove the actual production integration. No dependency edge was dropped, feature skipped, test moved, dummy authority supplied or Source quota relaxed.

The two state tests already use genuine domain-neutral controls:
- exact original RetainedCloneSource and paid NativeReceive/NativeReceiver construction recipients;
- original snapshot projection cursors and sealed owner identities;
- existing framework PostFixture with genuine projected post custody;
- exact grant/denial/heap/retirement observation;
- retained diff/preflight/commit traits from the canonical replication library (authentic Cargo extern name `protocol`).

They do not use the broad stdio decode helper, and the new IO-control test support is not a replacement for actual ArtifactStoreSnapshot integration. State support code's protocol alias resolves to semio-framework-replication in the actual captured Cargo graph; no fabricated alias was inferred.

## Coherent remaining options

1. Run the **correct snapshot owning full library suite** through ordinary Cargo when fresh normal capacity fits, preserving all snapshot tests and the two exact named laws. This can establish its actual owner/admission result, even if the genuine engine cap refuses first.
2. Repair the oversized engine through genuine production ownership decomposition by its owner, preserving canonical Store/artifact behavior and unchanged caps. Then rerun the snapshot owning suite and the original schema integration sanity scope; a successful state-only check would not replace those integrations.
3. A future domain-owned Energy state core extraction may separate genuinely domain-neutral state/projection/diff types and their complete existing tests from the artifact snapshot integration. That would require real production type ownership and caller migration, not duplicating source or relocating just the failing laws. Snapshot's actual OS integration and full tests would still remain and require verification.

No production edit or Cargo command was executed during this trace. An authorized correct-owner observation was kept **off the queue** after fresh capacity returned headroom-675MB/sixwaiters; normal governor/website priority is preserved. Correct future command, from energy model artifact root:

`SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=release --parallelism=1 -- test --manifest-path 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml --lib --no-fail-fast -- --test-threads=1`

No test-name filter. Require actual nonzero execution of both named laws and full owning suite success; source diagnosis is not runtime proof. Frontend captured-helper work remains the application priority.
