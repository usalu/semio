# Layout Baselines and WASI Recheck

The goal remains incomplete. The latest full native pass and full WASI pass have not yet been shown clean.

## Runtime Baselines

- layout989 CAD build stopped at the ambiguous borrowed BTreeMap range in window transient retirement. The lookup now explicitly uses str.
- layout989 raster native execution reached its size assertion after the independent serde asset decoder and first-party mutation JSON comparison passed. Actual size was 448 bytes against 128 bytes. The exact assertion failed as expected; subsequent mesh/chunk selectors did not run in that invocation.
- cad999 native build completed in 15m 31s. The native test passed selection and binary mutation round-trip assertions, then failed at 1,544 bytes against 64 bytes. Its later restored-state and text assertions were not reached, and the contribution selector was not run.
- Both large payload representations are now boxed. Their green runtime runs remain pending and are included in the combined regression catalog.

## Additional Repairs

Reconstruction helpers return their infallible emission directly; the app_commands! handlers preserve the required Result interface with exact-site expectations. FEM mounted-domain slots and the already-boxed note block cursor keep their deliberate inline ownership, with narrowly documented large-enum expectations. Existing bounded closure and nested materialization regressions were added to the combined catalog. Plugin registry qualifications and JPEG feature-dependent imports were repaired after the focused build exposed them.

## Current Verification

The combined catalog was validated by runtime993: 2 batches, 74 unique package targets, and 554 unique exact selectors, with its existing VS Code launch configuration present. The combined runtime has not yet run.

wasm1005 is the current full strict WASI recheck: 160 packages, wasm32-wasip2 libraries, Clippy -D warnings, existing RUSTFLAGS preserved. Receipts and complete diagnostics are retained in generated output. Fresh native and browser passes, green runtime regressions, and actual native/59 WASI component links remain required.

The read-only ticket cache review found approximately 14.3 GiB in the reusable compiler target, with all other ticket directories much smaller. No cache was removed.
