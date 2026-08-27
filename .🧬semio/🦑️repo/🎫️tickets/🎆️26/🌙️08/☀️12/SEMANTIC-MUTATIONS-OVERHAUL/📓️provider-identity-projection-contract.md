# Provider Identity Projection Contract

## Audit Decision

The read-only audit distinguished Cargo package identity from dependency keys, library targets, and Rust aliases. Replication is package `semio-framework-replication` with library `protocol`; the kernel is package `semio-framework-os-kernel` with library `semio_framework_os_kernel`; derive is package `semio-framework-os-kernel-dsl-derive` with proc-macro library `dsl_derive`. STDIO's `extern crate semio_framework_os_kernel as protocol` names the genuine kernel facade, not the lower package.

Existing regex manifest/module helpers lose `package =`, target identity, and extern aliases. They cannot be the approval authority. The complete metadata syntax inspector supplies declaration/alias facts but does not resolve Cargo identities.

Full `cargo metadata --frozen` is not a valid exclusion guard: local help states that frozen is locked plus offline, not a prohibition on filesystem reads. `--no-deps` omits dependency resolution and does not document a no-traversal guarantee. The current root lists explicit members without a literal compose path, but this is not a permanent guarantee for future path or platform dependencies. No full Cargo metadata subprocess is authorized for this packet.

Bun's system TOML parser exists and passes the bounded alias and duplicate-key vectors, but the retained multiline-string initial-newline vector fails. This must not be relabeled conformant. A repository-owned projection interface may use the built-in parser only behind explicit fail-closed syntax/shape checks, including rejection of actual multiline strings until their semantics are correctly supported and independently tested. Do not add an external runtime parser, silently repair the expected oracle, or fall back to regex approval.

## Bounded Foundation Packet

FND-PROVIDER-PROJECTION-16 adds pure manifest projections, not filesystem traversal, Cargo execution, graph approval, or active policy. The caller supplies raw source and the exact repository-relative manifest locator already established by existing no-follow source authority. The helper must still reject unsafe/absolute/escaping/excluded locators before parsing. It never follows dependency paths.

Return repository-owned types for package name, package version information when present, explicit library name/path/crate types/proc-macro flag, workspace dependency declarations, and normal/development/build dependency declarations with their exact key, package override, local path, workspace inheritance flag, and target condition. Preserve unsupported/external-edge facts as unapproved information; do not approve registry/git/version-only edges. Do not reject an otherwise usable manifest merely because an unrelated dependency is conditional; the eventual authority resolver must reject conditional edges used to establish the provider.

Quoted keys, ordinary literal/basic strings, dependency tables, inline tables, workspace aliases, package/lib identity divergence, and irrelevant string/comment header decoys need schema-first tests. Reject duplicate keys, malformed TOML, wrong field types, ambiguous forms, unsupported multiline strings, and unsafe manifest locators. A filename or alias spelling is never evidence of provider identity. Do not synthesize Cargo package IDs or guess Rust extern names in this packet.

The parser interface exposes no Bun-specific types. All permanent code belongs in the existing discovery component with regions and explicit repository exports. The next filesystem/alias policy layer will validate the selected consumer and canonical provider manifests through pre-I/O exclusion and no-follow resolution; it will compare exact provider manifest/package/lib identities, not a mutation allowlist.

## Required Verification

Commit neutral JSON vectors and schema; compare projected accepted facts with an independent TOML oracle in test tooling, and assert the explicit rejected subset separately. Test fixtures must include multiline-string header decoys without weakening the known standard expectation. Run the registered focused repository-library tests and an independent coordinator replay. Every artifact remains under this ticket, and no actual compose path is accessed.

The first proposed standard-library oracle was unavailable: the host Python is 3.9.6 without tomllib/tomli. The coordinator approved the already installed `@iarna/toml` library as the independent test-only oracle instead. This adds no production dependency and must preserve the exact standard multiline-string expected value. The unavailable-Python probe is not a parser or projection test result.
