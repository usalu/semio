# Canonical Live Reconciler RED R43–R44

Canonical route: `@semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib surface_canonical_document_ -- --nocapture'`.

R43 exited 1 before test execution: six compiler errors, including two newly authored helper-scope errors and four missing canonical-root API/field errors. The helper module was moved under the existing fixture namespace without changing assertions.

R44 exited 1 before test execution: four errors, solely the missing `SurfaceReconciler::capture_document` and `document` authority. No native tests ran. Production payload maps remain unchanged at this RED boundary.

Exact captured R44 tail:

```text
error[E0599]: no method named `capture_document` found for struct `reconcile::SurfaceReconciler` in the current scope
error[E0609]: no field `document` on type `reconcile::SurfaceReconciler`
error[E0599]: no method named `capture_document` found for struct `reconcile::SurfaceReconciler` in the current scope
error[E0609]: no field `document` on type `reconcile::SurfaceReconciler`
error: could not compile `semio-framework-ui-runtime` (lib test) due to 4 previous errors
```

New strict domain fixture/schema pins nine actual live reconcilers, shared original reader identity, retained old reader across replacement, 1/64/4096-byte retirement grants, and unchanged 8MiB/32MiB ceilings. The existing runtime script now validates it with Ajv and independently checks Buffer alias backing identity. This is schema/source setup, not an executed new source-oracle PASS yet.

Original runtime R30/R31 and inline physical census REDs remain acceptance gates. No cleanup, repinning, other-ticket write, or compiler process remains active at this checkpoint.
