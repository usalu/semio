# Framework Space Artifact Extraction

## Scope

This packet extracts the persisted `os.space` and `os.collection` artifact implementations from the framework OS host into independently compilable packages. Draft catalogs, archive import/export, `BlobStore`, and other host orchestration remain in the OS host.

## Package boundaries

The source and declaration for `semio-framework-artifact-space-space` are:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🦀️.rs`;
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/Cargo.toml`, whose `lib.path` mounts the taxonomy source;
- artifact definition and validation schema under the artifact's `🧬️schema` directory;
- the language-neutral `📚️examples/🪐️demo.space` fixture and package-local Rust tests.

The source and declaration for `semio-framework-artifact-space-collection` use the corresponding `🗿️artifacts/🗂️collection` tree, including `📚️examples/🎬️demo.collection`.

Each leaf depends only on the OS kernel, the value derive package, and serde at runtime. The leaves do not depend on `semio-framework-os` or on each other. The OS host activates both through its existing `space-guest` feature and reexports their public model APIs for its internal host module surface.

## Ownership split

The space leaf owns space membership, roles, the persisted space snapshot, its mutation and diff protocols, codecs, reconciliation laws, and backbone URI.

The collection leaf owns collection folders and entries, artifact body references, the persisted collection snapshot, its mutation and diff protocols, codecs, integrity/path laws, and collection/artifact backbone URIs.

The former combined host module now owns only draft and catalog behavior, space backbone ports, archive import/export, blob storage, and host integration tests. Artifact packages therefore have no back-edge into archive, filesystem, or OS host orchestration.

Manual mutation descriptor owners were updated to their complete new taxonomy paths. This preserves the schema system's invariant that every mutation descriptor identifies its actual source owner.

## Language-neutral package contracts

Each artifact definition declares its canonical artifact id, Rust package, Nx project, taxonomy source, fixture, and empty dependency inventory. The Rust package parser reads the embedded JSON through the first-party OS pack JSON codec, applies semantic identity checks, and compares canonical fields with an independent `serde_json` oracle in tests. A syntactically valid identity mutation is accepted by the oracle and rejected by the package validator.

The `.space` fixture was completed with the required empty durable-extension list, allowing the moved `SpaceSnapshot` codec to parse it as a complete language-neutral document. Package tests also exercise DSL and pack equivalence, operation text and operation application round trips, mutation descriptor validity, and deterministic domain reconciliation.

## Direct consumers

The space plugin, open-space command, set-active-example test, home create-studio command, home editor test, and framework DSL fixture sweep now import model types from the two artifact crates directly. The space plugin imports only archive and host operations from `semio-framework-os`. A source audit finds no remaining `semio_framework_os::space` or qualified OS-host model imports.

## Validation

Passing:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo test -p semio-framework-artifact-space-space -p semio-framework-artifact-space-collection --lib --message-format short
```

Result before the final Nx identity-only rename: 4/4 tests pass in each package, 8/8 total. The current package definitions and validators consistently use the final collision-free Nx names; rerunning this command is included in the handoff queue below.

```text
bun -e <identity audit> <both artifact-definition.json files>
```

This verifies `os.space` / `os.collection`, their canonical Rust package names, and `@semio-tech/framework-space-space-rs` / `@semio-tech/framework-space-collection-rs`.

```text
cargo metadata --offline --no-deps --format-version 1
```

This resolves both new members and workspace dependency declarations in the current multi-agent workspace graph.

```text
cargo tree -p semio-framework-artifact-space-space --prefix none --depth 2
cargo tree -p semio-framework-artifact-space-collection --prefix none --depth 2
```

Both trees terminate at the OS kernel and support crates and contain no `semio-framework-os` host package.

## Validation handoff

The current OS host integration gate remains attached as unified command session `25581`:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check -p semio-framework-os --features os-host-full --lib --message-format short
```

It was still compiling the cold host runtime dependency stack at handoff and had reported no package-boundary diagnostic. It must be polled to completion. Because the final Nx project identity strings were updated while this build was active, run the following gates in order after it exits:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo test -p semio-framework-artifact-space-space -p semio-framework-artifact-space-collection --lib --message-format short
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check -p semio-framework-os --features os-host-full --lib --message-format short
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check -p semio-s-plugin-space --lib --message-format short
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check -p 'semio-s-artifact-norm-*' --lib --message-format short
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo test -p semio-s-plugin-norm --lib --message-format short
```

## Changed files

Created artifact-owned files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/{🦀️.rs,📦️packages/🦀️rust/Cargo.toml,🧪️tests/🦀️.rs,🧬️schema/📜️artifact-definition.json,🧬️schema/🧬️.schema.json,📚️examples/🪐️demo.space}`;
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/{🦀️.rs,📦️packages/🦀️rust/Cargo.toml,🧪️tests/🦀️.rs,🧬️schema/📜️artifact-definition.json,🧬️schema/🧬️.schema.json,📚️examples/🎬️demo.collection}`.

Updated composition and consumer files:

- root `Cargo.toml` workspace members and dependencies;
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`;
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs`;
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` and `✏️s/🔌️plugins/🪐️space/🦀️.rs`;
- open-space, set-active-example, create-studio, and home-editor Rust sources under the space plugin;
- framework DSL fixture-sweep manifest and Rust test.

Removed the two corresponding fixtures from the former host-owned `📚️examples` directory after moving them to their artifact roots.

## Nx handoff

Nx project declarations are owned by the Nx execution packet. Their native inputs must include the full artifact taxonomy source, local schema, example, and tests because the Cargo manifests mount source outside the package directories. The final project identities are `@semio-tech/framework-space-space-rs` and `@semio-tech/framework-space-collection-rs`; the `framework-` prefix prevents collision with the existing space plugin artifact project.

## Artifact body casing audit

The post-extraction value-derive casing audit considered adding `rename_all_fields = "camelCase"` to `ArtifactBody`. No pre-existing language-neutral JSON schema, fixture, or direct JSON consumer declares a `documentId` field for this type. The artifact schema describes package identity only, and `📚️examples/🎬️demo.collection` declares the separate DSL spelling `document-id`. The candidate casing change and its circular inline assertion were therefore reverted. The retained serde and first-party value contracts both use the existing `document_id` field name; the `Document` and `Blob` variant tags remain camel case through their existing `rename_all = "camelCase"` declarations.
