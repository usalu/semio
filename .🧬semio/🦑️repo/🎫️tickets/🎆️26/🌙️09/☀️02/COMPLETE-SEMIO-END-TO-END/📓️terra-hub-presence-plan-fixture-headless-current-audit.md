# Terra — Headless Presence Plan Fixture Mismatch

**Scope.** Read-only inspection on 2026-09-05 after the reported headless SQLite receipt `presence-normalization-exact/exact-cargo-laws-UkC7ve/00` stopped at `🚀️bin.rs:7996` in `presence_normalization_socket_overwrites_identity_and_rejects_without_refresh`. No build, socket process, or source change was run for this audit.

## Diagnosis

This is a deterministic **test-state readiness/catalog mismatch**, before any presence ingress or socket-grant logic runs.

The failing law installs `TestDocumentOpenCatalog` at [`🚀️bin.rs:9983-9985`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9983>), then calls the real plan-and-exchange helper.  Its state comes from `lag_test_state`, whose readiness declares both `artifact_authority_ready = false` and `open_plan_ready = false` at [`🚀️bin.rs:7129-7130`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7129>).  `issue_document_open_plan_inner` rejects `!state.readiness.features.open_plan` as `CatalogUnavailable` *before authentication, descriptor lookup, or `openable_catalog` selection* at [`🚀️bin.rs:2251-2271`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2251>).  The helper discards that structured result and panics with the receipt's observed generic `issue document open plan` at [`🚀️bin.rs:7990-7997`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7990>).

The same latent setup defect is already present in the next selected law, `presence_lease_reconnect_rejects_old_live_refresh_and_close`: it starts from `test_state`, installs the test catalog, then immediately issues three real plans without switching readiness at [`🚀️bin.rs:10055-10067`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10055>).  Fixing only law two will therefore make law three fail at the same helper.

## Why the existing fixture is the correct headless authority

`TestDocumentOpenCatalog` is a test-only implementation of the same `DocumentOpenCatalogAuthorityV1` used by the production trusted catalog.  It resolves only an exact tuple of announced descriptor owner/package hash, artifact kind/schema/pack schema, requested surface, and write permission; its asset accessor also requires its own current generation and nonempty bytes ([`🚀️bin.rs:381-389`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:381>), [`🚀️bin.rs:600-638`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:600>)).  `document_open_catalog_for_descriptor` derives both editor/viewer selections from the actual just-announced descriptor, rather than accepting a plan, package, path, or grant from the test caller ([`🚀️bin.rs:7880-7925`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7880>).

The failing helper still uses the real route-inner functions: it submits a bounded canonical open intent, receives a real ledger-backed `DocumentOpenPlanV1`, then exchanges its actual receipt for the socket grant ([`🚀️bin.rs:7990-8003`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7990>).  The socket law consequently retains real session authentication, membership/revision revalidation, descriptor binding, catalog selection, plan receipt, and selected-surface checking.  It must stay that way.

Under the selected `--no-default-features --features sqlite` target, production `NativeCodecProviderSetV1` is deliberately absent behind `native-artifact-execution` ([`🚀️bin.rs:52-54`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:52>).  Replacing the test catalog with `NativeCodecProviderSetV1::linked()` or a full Stdio bundle would make this headless acceptance target depend on the very optional catalog graph it intentionally excludes.  That is neither needed nor a more authentic plan test.  The mismatch is that the existing descriptor-bound authority fixture was installed without its matching test readiness projection.

## Minimal correction

Add one **test-only setup helper** beside `document_open_catalog_for_descriptor`, conceptually:

```rust
fn install_test_document_open_catalog(state: &mut HubState, descriptor: &DocumentDescriptor) {
    state.openable_catalog = Some(document_open_catalog_for_descriptor(descriptor));
    state.readiness = Arc::new(hub_readiness(
        HubMode::Development,
        "loopback",
        "00112233445566778899aabbccddeeff".into(),
        true, true, true, true, true, false, false,
    ));
}
```

Use it immediately after descriptor lookup in both selected plan-backed presence laws:

1. `presence_normalization_socket_overwrites_identity_and_rejects_without_refresh` at [`🚀️bin.rs:9983-9985`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9983>).
2. `presence_lease_reconnect_rejects_old_live_refresh_and_close` at [`🚀️bin.rs:10061-10065`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10061>).

This is the established exact setup shape in the existing authenticated plan tests at [`🚀️bin.rs:8343-8347`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8343>).  Do **not** change `test_state` or `lag_test_state` globally: their default `openPlan: false` is the explicit unavailable-catalog state used to retain fail-closed route coverage.  Do **not** set readiness without installing the descriptor-bound catalog; that merely moves rejection to the later `openable_catalog` check and makes readiness lie.  Do **not** manufacture a `DocumentOpenPlanV1` or `SocketGrantReceiptV1` directly.

The helper may additionally assert that its catalog resolves the exact editor selection for the descriptor before storing it, but it must not preissue a plan or grant.  The current `DocumentOpenCatalogAuthorityV1` selection path in `issue_document_open_plan_inner` remains the authority boundary.

## Small diagnostic hardening

The generic `panic!("issue document open plan")` loses the status and canonical error code that distinguish readiness, auth, catalog, stale, and deadline failures.  In test-only helper code, destructure `Err((status, DirectoryJson(error)))` and include `status` plus `error.code` in the panic.  It preserves strict failure, changes no routing, and would have identified `503/CatalogUnavailable` without a source audit.

## Qualification implications

The reported native receipt is correctly **RED**.  This audit does not claim the two repaired laws pass.  After the paired setup correction, re-run only the registered `presence-normalization-check native` group; it remains the correct no-default SQLite target and will test the real plan→grant→authenticated socket boundary without loading production plugin codecs.
