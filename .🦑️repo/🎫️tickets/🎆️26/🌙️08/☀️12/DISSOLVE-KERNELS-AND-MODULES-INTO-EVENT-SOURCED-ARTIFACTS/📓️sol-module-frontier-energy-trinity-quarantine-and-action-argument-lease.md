# Module Frontier: Energy/Trinity Quarantine and Action-Argument Lease

## Energy

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation` is not a one-file module. It contains approximately fifty domain-specific Rust engine leaves covering envelope, HVAC, air, water, electrical, schedules, simulation, results, geometry, and other coupled responsibilities. Its artifact owner is concurrently dirty. The stale census's zero-consumer/delete disposition is not authoritative because it misses cumulative Rust path/package edges.

Disposition: quarantine the whole simulation SCC until its package/referrer frontier and concurrent artifact changes are stable. Do not infer deletion or one shared responsibility from the parent directory name.

## Trinity Jack LSP

The Trinity Jack LSP subtree is clean but not a safe routine relocation:

- TypeScript worker hash: `6039e3d142e1406d3219c5486fc29d21d4a489a8708ee05256a94a14cf5d8d63`.
- Rust package glue hash: `6808f823aec9fce2f7ae8652ce26e4fe62710cd5e1ba66e1825edcdeb0a6910a`.
- The Jack app uses its direct Rust language implementation and does not mount the worker or `JackLspSession`.
- The TS worker expects a generated `JackLspSession` absent from the current generated declaration.
- The Rust package describes itself as a compatibility shim around OS `dsl_lsp`, which violates the no-compatibility rule.
- No production consumer exists outside the LSP packages and root workspace registrations, but those packages are themselves registered Nx/Cargo/WASM tool entrypoints.
- The subtree contains an `AGENTS.md` that may not be edited, moved, or deleted. Root Cargo/package/Bun registrars and externally dirty lock state also prevent an atomic relocation/deletion lease.

Disposition: quarantine. Do not duplicate the tool inside the Rust-only Jack app. A later global registrar lease must decide the terminal tool identity and remove the compatibility shim atomically.

## Action-Argument Resolution Lease

The clean TypeScript `🧰️framework/🔨️modules/🎯️action-bus/🟦️component.ts` at hash `9d17344e90aa8d9afe20fbce64d61c12a59a4e3df472c334f8b90598db629978` mixes five responsibilities. Two functions form one independently shared capability:

- `effectiveActionArgs` resolves staged values and declarative defaults.
- `missingRequiredArgs` identifies required arguments still unresolved.

They have at least two independent production semantic consumers: framework UI `UIDialog` and OS renderer `ShellHelpers`; the ui-react target is an additional production referrer. All consume them through the framework package assembly. The functions expose only repository-owned manifest contract types.

The Terra source lease owns only removal of those functions/imports from the old component and creation of `🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts`. It must not add a forwarding export or touch consumers. The coordinator alone owns the clean root TypeScript assembly glue at hash `16aa07b5fd2492c2a7b309ccfbd404438366c50fa27974a7b915a07f6adf82c1` and will export the new component directly after source integration.

The remaining action-bus facets stay quarantined: TypeScript utility/tool/window behavior terminates in the moving renderer cluster, while Rust combines a protected one-consumer ephemeral bus with a widely shared JSON-to-DSL action conversion whose public external JSON type requires an atomic consumer-boundary design.

## Coordinator Registrar

Terra produced:

- remaining action-bus TypeScript hash `e3ab0e4ef72494f28c794f0d34a6ff70bae451bd6a2201ee002abe851dba9207`;
- new action-argument-resolution hash `04a708965baa5a25e7a3e6cf85c0c6011c06f4bdc5cde94ebf94aba5c9b5bc2e`.

The coordinator updated only `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`, replacing no old API and adding a direct assembly export for the new component. Its post-integration hash is `45f9e589322aaf7001ef89750d3fc9a89c04a7bdeb4b5b647a83eabc0ac2b743`.

## Structural Validation

- Narrow semantic-ID report for `framework.module.action-argument-resolution`: exit `0`, but `0` components/`0` findings because the missing framework modules manifest means the new member has no resolvable semantic ID yet. This is not a clean result.
- Framework-scoped report: exit `0`, `18` components, `248` errors, `0` warnings.
- Framework-scoped enforce: exit `1` with `248` errors. The renderer currently labels its output `Mode: report` even for enforce; the thrown enforce error is the authoritative mode outcome.
- Exact findings for the new member:
  - `collection-manifest-missing` at `🧰️framework/🔨️modules`;
  - `manifest-child-missing` at `🧰️framework/🔨️modules/🧮️action-argument-resolution`;
  - `module-production-consumer-minimum` reporting zero resolved consumers because the current graph adapter does not resolve the root package export to the UIDialog/ShellHelpers terminal components.

The 150/150 framework quick tests establish behavioral/package integration only. Structural release is pending the global exact-bijection framework-module registrar and corrected root-package consumer graph; no partial manifest is permitted.
