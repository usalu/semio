# Manifest Dependency Validation Privacy Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Writable source: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` only, plus a unique acceptance record.
- Required source SHA-256: `92fbd693b8fb1f983c9c934381e531b516ac61e7e050662bb6a20a2dae893673`.
- Preserve the external ordinary insertion of `ArtifactDialect::export()` near line 5763 exactly. Cached state must be reread and preserved.

## Disposition

`validate_dependency_graph` has no production consumer outside the manifest owner. Its only production caller is same-component `resolve_load_order`; host registration and hot reload consume `resolve_load_order`, not the helper. Make only `validate_dependency_graph` private (`pub fn` to `fn`). Keep `DependencyGraphError` public because the live public resolver exposes it. Keep tests, generated/schema leaves, TypeScript, glue, and every other API unchanged.

## Evidence

Use `apply_patch` only and no modifying Git command. Abort on hash mismatch or a newly overlapping diff. Confirm the helper has no external reference and remains called by `resolve_load_order`; run scoped ordinary/cached diff checks. Run `bun nx run @semio-tech/framework:test --skip-nx-cache` only if it does not contend on the shared Cargo build directory; otherwise record source-complete/not-green and queue validation. Report the final SHA and separately characterize the external dialect insertion and this one-line visibility change.
