# Framework Boundary Audit, 2026-10-14

Read-only source audit. Root `AGENTS.md` and `🧰️framework/🛍️products/AGENTS.md` were read. No tests were run, no runtime behavior is claimed, and no code or Git state was changed.

## Current General Boundary

The General Cargo workspace in `🧰️framework/Cargo.toml` owns General module packages, with product packages explicitly excluded. The General facade manifest has no confirmed `semio-s-*` or hub dependency. Targeted source import/use scanning found no confirmed actual import from General source into `✏️s` or `🌎️hub`; most string hits are documentation and architecture/test fixtures. This is a bounded textual audit, not a full resolver graph proof.

Existing checks:

- `🧰️framework/🧪️tests/🧱️ownership/🟦️.ts` compares Bun TOML and `@iarna/toml`, checks dependency sections in the General facade manifest, exports in its Rust facade, workspace member prefixes, owning workspace references, and workspace path dependencies.
- Its fixture is `🧰️framework/🧪️tests/🧱️ownership/🧫️fixtures/🔣️.json`.
- `🧰️framework/📏️script-boundary/🧪️tests/🟦️.ts` compiles strict owned types, parses router registration with TypeScript, and bundles the actual General router with esbuild while refusing product, s, and hub sources. It prints a `[DEBUG]` source-owner count.
- Existing Nx target `@semio-tech/framework:test-script-boundary` is defined in `🧰️framework/📦️packages/🟦️typescript/📋️project.json` and dispatched by its `📜️script.ts`.

The Cargo ownership test inspects the facade and workspace declaration rather than every transitive Rust source edge. The esbuild refusal test proves the General command router graph, not all General browser entrypoints. Extend the existing enforcement at the actual relevant entrypoints rather than introducing a duplicate checker.

## Concrete OS Composition Coupling

### 1. Plugin Rebuild Owns Downstream Deployment

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🔣️.json` declares:

- `activate-s` through `@semio-tech/framework-os-dev:activate-s-react-prototype`.
- `verify-s` through `@semio-tech/plugin-registry:verify-staged --variant s`.
- `preflight-catalog` and `publish-catalog` through `os-hub:trusted-catalog-preflight` and `os-hub:trusted-catalog-bootstrap`.
- A guest framework check of `✏️s/Cargo.toml` and `semio-s-plugin-stdio`.
- Checks mixing General and OS package names under root `Cargo.toml` instead of their declared owning workspace manifests.

The executor at adjacent `🟦️.ts` already provides schema-validated `GuestFrameworkCheckV1` and derives Cargo argument vectors from declared workspace/package/features. Preserve this neutral executor and move the specific step roster to the consumer composition owner. Allow the OS task to receive an explicit composition definition, and make the consumer-owned script select that definition. Do not merely delete activation and publication behavior.

Available regression owners include:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🔁️rebuild/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧩️composition/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-publication/🟦️.ts`

### 2. OS Service Verification Requires GIS Receiver Sources

`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts:21` builds a receiver roster under `🌎️hub/🧩️compositions/🌍️gis/🧪️tests` with `🌉️component-cold-map-patch`, `💡️inference-control`, and `📇️native-codecs`, then typechecks those with the OS service source law.

Keep the OS `installed-service-source` command focused on its own `🔨️modules/💡️inference/🔌️service/🧪️tests/🧱️io-boundary/🟦️.ts`. Move the receiver strict verification into the GIS composition script, consuming the same public service contracts. Register or update Nx/launch commands where needed. Preserve all three receiver checks.

### 3. OS Database Oracles Read Hub Bootstrap

`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:1344` reads `🌎️hub/🏗️bootstrap/🦀️.rs` and checks the downstream ensure-document caller. The surrounding command is the document-mount single-flight oracle and already has owned database fixtures/native laws.

The same file at line 2264 reads hub bootstrap inside `directorySessionAuthorityOracle` and asserts the hub session response binds the existing digest and returns `Json<DirectorySessionAuthorityV1>`.

Keep producer/codec checks with the OS; put bootstrap integration assertions at the hub consumer. Preserve the assertions and connect them through explicit consumer verification commands. The `directory-session-authority-check` registration is in the same script. Document mount native law names are already present immediately after the hub textual assertion.

## Additional Candidates, Lower Priority for This Slice

- OS mutation derive macro at `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:33` uses a fixed global `MUTATION_AUTHORITY_LOCATOR`. At lines 75–85 it discovers a workspace root, reads the projection and loads domain owners and aggregate sources. The generated projection `🔣️mutation-authority.json` contains specific `✏️s` owners. This is a build/compiler-resource composition coupling, not a Rust crate import. Long-term isolate consumer-owned projection authority behind an explicit resource definition and preserve fail-closed no-follow/source tracking laws. Existing laws are `🧪️tests/🔬️mutation-source-authority/🦀️.rs` and `🧪️tests/🔬️mandatory-mutations/🦀️.rs`.
- Mounted-owner crate is physically `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/📦️packages/🦀️rust/Cargo.toml` but declares `package.metadata.semio.role = "framework"`, has its own workspace, and depends only on General job/value crates. This may indicate taxonomy ownership needs review; do not move it without checking concrete callers and package discovery.
- Repository product discovery/policy code legitimately knows repository roots. These are not automatically reusable General dependencies. Its hardcoded s scanning is an owner-policy question, separately from neutral framework runtime imports.

## Handoff

Highest leverage immediate implementation: separate OS producer verification from GIS/hub receiver verification, then move plugin rebuild deployment roster into the specific composition while preserving its executor/schema and all existing activation/publication checks. Validate through the relevant registered Nx routes and actual source/bundle oracles. No verification result is asserted by this audit.
