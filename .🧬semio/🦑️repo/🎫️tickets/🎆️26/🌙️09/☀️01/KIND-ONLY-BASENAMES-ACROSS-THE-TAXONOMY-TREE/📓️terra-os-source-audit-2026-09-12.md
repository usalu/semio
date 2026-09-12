# OS and Hub Source Ownership Independent Audit

Date: 2026-09-12

## Scope and method

This was a bounded read-only review of the 30 source-owner rows in the portable OS-source topology fixture. It covered the new contextual directory owners and ancestors, direct TypeScript referent resolution, the nine native Cargo manifests, the WGPU sibling Node/Rust packages and their declared generated-output authority, hub bootstrap separation, the descriptor binary adapter, and the Windows registration leaf. No source, schema, configuration, generated output, session output, Git state, AGENTS file, ticket lifecycle, or process was changed.

The registered check used a private `NX_WORKSPACE_DATA_DIRECTORY` below `🗑️generated/terra-os-source`, with `NX_DAEMON=false` and `NX_ISOLATE_PLUGINS=false`. The remaining probes parse live inputs and metadata only.

## Independent acceptance evidence

- `bunx nx run @semio-tech/repo-lib:test-os-source-topology --skip-nx-cache` passed: **5 tests, 246 expectations, 0 failures** (444 ms test time; 802 ms Nx elapsed). This is the current registered, non-staging topology route.
- An independent fixture walk checked all **30** rows against the live tree: each expected owner exists with its kind-specific anonymous basename, its recorded current SHA-256 matches, each declared consumer exists, and each removed source has the stated disposition. This corroborates the route but is deliberately not used as the sole identity oracle.
- Direct `loadTaxonomy()`/`semanticDirectoryKindId()` ancestry tracing under current taxonomy schema version 7 resolved every ancestor of all **30** owners without a missing kind. It also inspected the exact fourteen scoped registrations:
  `os-hub-integration-harness`, `os-module-effect-backbone`, `os-canvas-icon-name-value`, `os-mcp-inference-bridge`, `os-mcp-schema-validation`, `os-renderer-artifact-creation`, `os-renderer-packed-text`, `os-plugin-installation`, `os-registry-launch`, `os-descriptor-emission`, `os-dev-bench-web-harness`, `os-dev-vite-plugins`, `os-dev-executable-source`, and `os-hostile-batch-edge-fixture`.
  Their configured parent kind and literal semantic member name match the corresponding live owner path. This is a scoped closure result only; it does not claim that ambient repository taxonomy findings are closed.
- An independent TypeScript AST traversal of the **11** moved TS/TSX owners found **49** current relative static or dynamic module specifiers. Filesystem resolution of every candidate succeeded. The executor's 47 figure denotes rebased referents; the two counts are not expected to match because this audit resolves all current relative specifiers, including unchanged ones.
- `cargo metadata --no-deps --format-version 1` returned zero for all **nine** relevant manifests. Selecting the package whose `manifest_path` exactly equals the requested manifest found exactly one package each. Every configured target path exists: hub bootstrap; OS `pack`, `semio`, and `spr`; infinite font dumper; MCP and run bootstraps; descriptor-emission library and binary; host; actor-import guest; and WGPU library, binary, and custom build selector.

## Boundary decisions confirmed

`🌎️hub/🏗️bootstrap/🦀️.rs` is the Cargo `os-hub` binary target. The distinct `🌎️hub/🚀️local-bootstrap/🦀️.rs` remains exposed by the hub library via an explicit path module and retains its local credential/pipe schema and fixtures. The two owners were not conflated.

The descriptor crate's `📦️packages/🦀️rust/💾️binary/🦀️.rs` contains only `main`, which invokes the neutral descriptor `run` through `semio_framework_async::block_on` and exits with its status. Its substantive Rust body is at `🛂️descriptor-emission/🦀️.rs`; the Cargo `[lib].path` points there. This is actual entrypoint glue rather than a package-body exemption.

The Windows association source is the anonymous PowerShell kind leaf `🖥️associations/🪟️windows/🔵️.ps1`. Its `Semio.Document` registration remains present. It was inspected as text only; no Windows runtime behavior is claimed on macOS.

The WGPU split is live and coherent: the Node root has `package.json` and `📋️project.json` with the common name `@semio-tech/framework-renderer-wgpu`, exports `./📚️library/🟦️.ts`, and that entry exists. The sibling Rust root has `Cargo.toml` and no Node manifest. Static generator metadata has **93** unique, byte-ordered existing browser source paths and **128** unique, byte-ordered input patterns. The browser profile declares the canonical `🎞️frame-worker` output as tracked and the `🚀️browser-boot` output as ignored; both sources and outputs exist. The sole native artifact receipt observed belongs to the Rust Cargo owner, as expected. No generator or generated-output check was run in this audit.

## Result

No concrete causal defect was found in the owned OS/hub source move, native target, WGPU package-boundary, or introduced-context scope. The prior framework omission incident was addressed by resolving live owner ancestry and registrations, rather than trusting only the fixture or a frozen hash.

## Limits retained

- This audit did not rerun generation, WGPU output checks, registry generation, the session fixture, or any route that could write canonical outputs. The executor report records those completed checks.
- The full `@semio-tech/framework-os:test-quick` route was not repeated: its registered budget is 30 seconds. The executor records that its move-sensitive three-test parity filter passed after the exact Rust referent repair; that is not a claim that the whole quick route passes.
- The executor's combined native check reached two current MCP feature errors after native target resolution: `DirectorySessionAuthorityV1.expires_at_ms` versus current `expires_at`, and missing `ArtifactCompositionFields` on `ProbeSnapshot`. Their provenance was not established here, so they are retained as current limits rather than assigned a baseline or a taxonomy cause.
- Manifestless OS/plugin source bodies, large Hub/development `📜️script.ts` bodies, persistent Flow/JCO/distribution producer work, and ambient contexts outside these fourteen registrations remain assigned to their separate lanes.

## Probe retention

The private logs, parsed metadata, AST-resolution records, and isolated Nx state were written below `🗑️generated/terra-os-source` during the review, then removed. The exact observations they established are recorded above; no generated audit output is retained.
