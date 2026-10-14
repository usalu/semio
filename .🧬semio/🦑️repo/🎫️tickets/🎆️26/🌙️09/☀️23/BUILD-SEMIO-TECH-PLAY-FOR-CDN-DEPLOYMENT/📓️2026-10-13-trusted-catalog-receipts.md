# Trusted Catalog Receipts (A2 Completion)

Date: 2026-10-13. Lane: trusted-catalog-receipts. Builds on `📓️2026-10-13-wave-a2-single-profile.md`.

## Outcome

Every consumer of a plugin's byte identity now carries **a semantic descriptor and a `ComponentReceiptV1`** instead of a byte-bound
descriptor. `PackageHashes` (`hashes.wasmSha256/coreWasmSha256/descriptorSha256`) no longer exists anywhere: not in Rust, not in TS,
not in a schema. A descriptor that still carries `hashes` is refused by every reader: serde `deny_unknown_fields`, the store worker,
kernel identity, module pages, the browser actor describe, catalog verification, native runtime publication and the hub oracles.

The law is the same everywhere ("binds"). A receipt is valid and of a shipped profile. Its `pluginId` and `packageId` equal the
package's. `wasmSha256` equals the component digest. `semanticDescriptorSha256` equals the digest of the descriptor bytes.
Wherever the byte length is known, `wasmBytes` must equal the component length too. Implementations:
- Rust: `ComponentReceiptV1::binds`.
- TS: `componentReceiptBindsV1`.
- JSON Schema: `DocumentShippedComponentReceiptV1` (structure plus `profile = wasm-release`). The hub enforces the digest equalities.

## Design

### One receipt contract, three languages

- **Schema:** `🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🔣️.json`. Its `if/then` branches are typed, so strict Ajv compiles it.
- **Rust:** `…/🧬️schema/🦀️.rs`, mounted in the kernel as `os_component_receipt`. It provides `validate`, `binds`,
  `encode_canonical` and `decode_canonical`.
- **Browser-safe TS:** `…/🧬️schema/🟦️.ts`. Node-only parts (deliverable, provenance, findings) stay in `🏷️component-receipt/🟦️.ts`,
  which re-exports the pure contract.
- **Canonical bytes:** 2-space JSON in schema key order, plus `\n`. These are the bytes the generation framing hashes, and the
  bytes of `🧾️receipt.json` in a deliverable and in a plugin module.

### Plan, lease and catalog binding

`DocumentOpenPackageV1.componentReceipt` is now required in Rust, TS and `📇️directory/🧬️schema/🔣️.json`.
- **Plan, lease and catalog binding:** each validates `receipt_binds()`. A lease additionally requires
  `component.byteLength == receipt.wasmBytes`.
- **TS parser:** a single shared `parseDocumentOpenPackageV1` serves both the plan and the lease parser.
- **Lease equality:** `sameLeaseFieldsV1` compares the receipt's canonical bytes.

### Component bound: 64 MiB raised to the shipped cap (256 MiB)

The new bound is `SHIPPED_COMPONENT_MAX_BYTES`, derived from the profile policy and never written as a literal twice. It applies to:
- the lease, the plan and the browser-installed target
- `TRUSTED_COMPONENT_MAX_BYTES`
- the catalog verification artifact bound
- the stdio catalog root and the stdio publication admission
- the browser host (TS and its staging schema)
- the execution-target relay limits
- the actor custody claim bound

The real deliverables need it: architect ships 97 253 648 B, bim 59 907 076 B and animate 60 811 101 B.

### Trusted bundle schemaVersion 4

- **Package record:** carries `receipt` inline (`TrustedBundleReceiptV1`).
- **Generation framing:** `semio/hub/trusted-profile-generation/v2\0` frames each package's receipt canonical bytes after its
  descriptor digest, so the generation id covers the byte identity.
- **Plugin modules:** carry `🧾️receipt.json`. Their minimum file count went from 3 to 4.
  `TrustedPluginModuleSourceV1.receiptSha256` is set in Rust and TS, and so is the index entry's `receiptSha256`.
- **Loader:** checks `bundle.schema_version != TRUSTED_BUNDLE_SCHEMA_VERSION`, using the constant instead of a literal.
- **Version and legacy formats:** the format version lives in one place, `TRUSTED_BUNDLE_SCHEMA_VERSION`. No reader accepts v3.

### os-dev activation (`DevHubCatalogHeaderV1`)

The header is now format 4, and `receipt` is a required package key. A dev hub holding a v3 catalog is therefore classified
`stale` and republished with progress, as the existing freshness law already does for format 2.
- **Fixtures:** gained an inline receipt and a "lacks its component receipt" row.
- **Format law:** now pins the schema constant `TRUSTED_BUNDLE_SCHEMA_VERSION` instead of a literal in the loader.

### Producers and consumers

- **Hub fresh producer:** emits the semantic descriptor and a real `ComponentReceiptV1` derived from the component's cargo
  provenance. The `extract-core` stage is gone.
- **Registry catalog rows:** the fresh commit marker is v2 and holds raw component, semantic descriptor and receipt. There is no
  core. `verifyFreshCatalogPackageV1` and the bound validators are deleted.
- **Stdio catalog root:** stages the shipped deliverable, the committed pack and the receipt, then verifies them with independent
  WebCrypto oracles.
- **Store worker, kernel identity and module-page fetch:** identify the component by `🧾️receipt.json`. A `hashes` member is refused.
- **ProgramBridge (`load_resolved_program`) and the MCP remote:** take the lease receipt.
- **Native runtime publication (`publishNativeRuntime`):** now binds each component through its module page's receipt and
  descriptor pack. Before this change it trusted `descriptor.hashes.wasmSha256`.
- **Registry rebuild report:** reads the staged identity from the module's receipt.
- **Actor custody reader (`🎭️actors`):** reads bundle format 4 and requires every package receipt to bind.

## Verification (all run in this session)

| Check | Result |
|---|---|
| Rust receipt unit (shared fixture, canonical refusal, binds, policy mirror) | 4/4 pass |
| Manifest codec (`package_descriptor_is_semantic_and_refuses_a_byte_binding`) | 2/2 pass |
| Kernel lib `os_directory os_component_receipt catalog` (rerun after all edits) | 96 pass, 2 fail in `os_store::component::initialization_catalog_close` (E1b store layout: 384 vs 320 bytes; not this lane) |
| Receipt TS tests | 29/29 pass |
| Catalog binding TS (Ajv + draft-07 meta) | 7/7 pass |
| Lease TS law with Ajv shape oracle | 2/2 pass |
| Kernel identity (`plugin-load-progress`) | 17/17 pass |
| os plugin-module bundle, resolution and store | 84/84 pass |
| Registry: catalog-complete, catalog-publication, rebuild, hub-source (vitest) | 4 files, 75/75 pass |
| Registry: descriptor-verification and discovery (bun test) | 31/31 pass |
| Fresh-component suite | green |
| `os-hub:trusted-stdio-gis-bundle-check --source` (plan, lease, browser-open, generation-stage 33 cases, publication fence, codec source, rotation, bootstrap oracle with `receipt-laws=3`, `codecs=32`) | **pass (rc 0)** |
| `open-plan-source-check`, `execution-target-lease-check`, `execution-target-relay-check` | pass |
| os-dev `🚀️local-hub` catalog freshness and format laws | pass. Same run's unrelated failures: serve fixture Ajv `compositionConfigPath` and its six serve laws |
| os-dev browser-host staging | my two regressions fixed (missing materialization bound, moved bim descriptor path). Remaining 20 failures are peers': `reactor.channelVersion`, L1 component names, taxonomy install dir, timeouts |
| Store worker in-source tests | 20/20 pass |
| GIS service-composition (store worker + gis worker) | 148/149 pass. 1 unrelated: `identityActorConfig` now returns 5 keys |
| `publishNativeRuntime` direct law (foreign-wasm receipt, byte-bound descriptor, foreign semantic pack: all refused; bound inputs admitted) | pass |
| Stdio trusted publication test | 4/5. 1 unrelated: stdio codec factory ids differ |
| Dry receipt run over bim, animate, architect shipped deliverables (`trusted-catalog-receipts/🧪️dry-receipts.ts`) | 3/3 bind. Strict Ajv `TrustedBundleReceiptV1`, `TrustedBundleComponentV1`, `TrustedBundleFileV1` and `DocumentShippedComponentReceiptV1` all accept. Node and WebCrypto agree. Each receipt file is its own canonical framing bytes. A foreign or iterate receipt is refused |
| Plugin crate, describe crate | `cargo check` green |

### Not achieved (blocked by concurrent lanes, honestly unverified)

- **Hub Rust:** `semio-hub` lib (sqlite and default features), the trusted_catalog unit and profile tests, bin-unit, plugin-module
  tests, hub wasm and native checks, and the `os-hub` binary. The last attempt failed first in the `semio-framework-os-kernel` lib
  (E1b store seams: `🏪️store/🧩️composition/🗄️durable-group` and 102 more errors). Earlier attempts failed in `db` (pack refactor) and
  `plugin-host` (`receiving_turn_fault`, `PluginHostError`). The hub-side Rust edits of this lane were therefore **never compiled**.
  They cover the trusted catalog loader, schema, plugin-module, tests, bin-unit and count/profile tests.
- **MCP and wgpu renderer test builds:** they fail on peer API churn (`Budget.retained`, `TurnResult.retained_receipt`, the
  `ArtifactDialect::to_coordinate` removal, and `os_directory::descriptor_digest_v1`/`hex_lower`, which moved into `🚪️io/🧱️binary`
  five days ago by commit `da607013f32`). None of their errors is in a receipt path.
- **Full dry trusted-catalog publish (`os-hub trusted-catalog publish` into a scratch data root):** needs the hub binary, which
  cannot be built. It is also blocked by the stale generated registry: `cratePath` still points at the library crate after L1, so
  `trusted-catalog-preflight --packages bim,animate,architect` fails reading `<composition>/🔣️.json`. The deliverables that do exist
  sit under the library crate's `dist/components`. After L1 regenerates the registry, the deliverables must be rebuilt under the
  component crates. The receipt-level dry run above is the part runnable today.

## Re-verification on 2026-10-14 (tree moved about 24 h; all rerun in this session)

### Receipt ownership

The Codex fleet extracted the receipt into the crate `semio-framework-plugin-component-receipt`. Its ticket is
`26/10/13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES`; see `crate-production-boundaries.md`. That crate's `[lib] path` is this
lane's single source file, `🏷️component-receipt/🧬️schema/🦀️.rs`, so there is exactly one Rust definition of `ComponentReceiptV1`.
- **Kernel:** no longer `#[path]`-mounts the file. It re-exports the crate explicitly
  (`pub use semio_framework_plugin_component_receipt as os_component_receipt;`) for the hub and kernel test callers.
- **Directory schema:** names the crate directly.

No duplicate remained, so no edit to their packages was needed.

### Results

| Check | Result |
|---|---|
| `cargo test -p semio-framework-plugin-component-receipt` | 4/4 pass |
| Receipt TS + lease TS law (bun) | 31/31 pass |
| Catalog binding TS (wgpu sqlite) | 7/7 pass |
| Kernel identity `plugin-load-progress` | 17/17 pass. Vitest now needs an explicit `SEMIO_VITEST_POLICY` |
| os plugin-module bundle/resolution/store (node vitest) | 84/84 pass. Under the bun runtime, the two retry laws time out |
| os-dev `🚀️local-hub` catalog laws | pass. The 7 serve-fixture failures are unrelated |
| Store worker in-source | 20/20 pass |
| Registry catalog-complete, catalog-publication, deployment, hub-source (vitest) | 70/70 pass. Fixture paths repointed, see below |
| `publishNativeRuntime` receipt law | pass: 3 refusals, 1 admission. The peer renamed the profile `dev` to `prototype` |
| Dry receipt run over bim, animate, architect | 3/3 bind. The oracle is now direct third-party Ajv over the three schema documents |

### Peer churn that blocks the rest (not this lane)

- **Repo-wide fixture relocation `<owner>/🧫️fixtures` → `<owner>/🧪️tests/🧫️fixtures` (about 15 h ago).** References were not
  rewritten:
  - The GIS service-composition suite fails 76 tests on ENOENT.
  - The registry `descriptor-verification` test now reads a doubled `🧪️tests/🧪️tests/🧫️fixtures`.
  - The registry `rebuild` test fails on a changed rebuild-step schema.
  - In my own two registry tests (catalog-complete, catalog-publication) I only repointed `../../🧫️fixtures` to `../🧫️fixtures`.
- **`bun nx` refuses with "Original immutable graph publication authorizer required".** This is the fleet's nx patch.
- **The hub script cannot load:** `repositoryCargoPreparationStorageV1` was removed from the cargo preparation storage module and
  its consumers were not updated. So `trusted-stdio-gis-bundle-check --source`, `open-plan-check`, `execution-target-lease-check`
  and `execution-target-relay-check` could not be rerun today. They passed on 2026-10-13.
- **`hubSchemaExport` refuses** because the new peer schema module `🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧬️schema` is not
  catalogued. Regenerating the catalog did not admit it.
- **Shipped deliverables need rebuilding:** they now belong under `dist/components/release` (profile policy change). The existing
  bim, animate and architect deliverables still sit in `dist/components`, and their bytes still bind.
- **Hub Rust:** polled for 9 minutes. The build is blocked below the hub by 6–7 crates from the extraction, among them
  `os-pack`, `ui-theme`, `ui-layout`, `job`, `os-command` and `store-message-ledger`. The hub-side Rust of this lane is still
  **uncompiled**.

## Deviations and decisions

1. **Shipped acquisition stays a follow-up.** Materialization still cold-builds, because actor custody (`🏗️bootstrap/🎭️actors`)
   proves the retained `cargoInvocations` of `PackageProducerV1`. A publish that acquires shipped deliverables by receipt is the
   next wave. It needs a custody claim for receipt-acquired packages (the receipt's `provenanceSha256` replaces cargo invocations)
   and runtime selection schema changes. A2 deviation 1 (the bound descriptor kept for the hub) is resolved.
2. **The 67 committed descriptor pairs still byte-bound are now refused at every reader.** They must be re-described (semantic) by
   the describe lane. Nothing reads them leniently.
3. **The live codec closure grew from 29 to 30 stdio codecs** (`stdio.pdf.1.7`). This had silently broken the bootstrap oracle. The
   projector also still demanded a GIS `hostile` member that commit `d4597d0410a` removed when it moved hostile rows into fixtures.
   The projector now bounds the closure (1..1024) instead of pinning 29, and drops the dead `hostile` check. The bootstrap fixture
   derives `limits.codecCount` from its profile. The fixture's `generationId` was recomputed by the oracle to
   `1c3dee2c…3010b`.
4. **The document-open `package-mismatch` vector now mutates `package.version`.** Changing `componentSha256` alone now unbinds the
   receipt, which is structurally `denied`, not `stale`. Three receipt vectors were added: unbound, foreign descriptor, iterate profile.
5. **Schema catalog regenerated** (`workspace:schema-generate`). It adds `os.plugin.describe.component-receipt`, which hub schema
   exports resolve through.

## Files (this lane)

Receipt contract: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/{🟦️.ts,🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}}`, kernel mount `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`.

Directory and binding:
- `📇️directory/🧬️schema/{🦀️.rs,🔣️.json,🟦️.ts}`
- `📇️directory/🧪️tests/🔏️document-execution-target-lease-v1/🟦️.ts`
- `📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs`
- `📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/{🧭️document-open-plan-v1.json,🌐️browser-document-open-v1.json,⏯️execution-target-resume-v1.json}`
- `🚪️io/🪪️bindings/🏛️catalog/{🦀️.rs,🟦️.ts,🧬️schema/🔣️.json,🧫️fixtures/🔣️.json}`
- the wgpu sqlite test

Store, MCP and renderer:
- `🏪️store/👷️worker/🟦️.ts` and the store sync tests
- `🐚️Shell/🧪️tests/🔀️wgpu-document-relay/🦀️.rs`
- `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` and the MCP quick/remote tests and source builders
- the `🛂️manifest/🧫️fixtures/📦️catalog-package/🔣️.json` fixture
- `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`
- `⌨️native-entrypoint/📦️modules/{🟦️.ts,📜️script.ts}`

Manifest: `🧰️framework/🔨️modules/🛂️manifest/{🦀️.rs,🧬️schema/📽️projection/🦀️.rs,🧪️tests/🔬️package-descriptor-value-codec/🦀️.rs}`, plugin `🛂️describe/🦀️.rs`, reactor runtime and descriptor emission.

Plugin, registry and kernel:
- `🔌️plugin/📇️registry/{✅️catalog-verification/🟦️.ts,🔁️rebuild/🟦️.ts,📦️deployment/{🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs},🧫️fixtures/🧩️plugin-module/🔣️.json},🌎️hub-source/🧫️fixtures/{🔍️resolution,🗄️store}/🔣️.json,🧪️tests/{✅️catalog-complete,✅️catalog-publication}/🟦️.ts}`
- `🌐️browser-bundle/{🛂️descriptor,🧾️describe}/🟦️.ts` and the describe fixture
- `🖨️describe/{🏭️fresh-component,🧾️source-epoch}/🟦️.ts` and the fresh tests and staging fixture
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`
- the kernel `🫀️plugin-load-progress` test

os-dev:
- `🧑‍💻dev/🧬️schema/🔣️.json`
- `🧑‍💻dev/🧫️fixtures/🚀️local-hub.json`
- `🧑‍💻dev/🧪️tests/🚀️local-hub/🟦️.ts`
- `🧑‍💻dev/♻️activation/🌐️browser-host/{🟦️.ts,🧬️schema/🔣️.json}`
- `🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`
- `🧑‍💻dev/🔎️verification/🧾️publication/🧪️tests/🟦️.ts`

Hub:
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/{🦀️.rs,🧬️schema/{🦀️.rs,🔣️.json},🧩️plugin-module/🦀️.rs,🧪️tests/🔬️unit/🦀️.rs,🧫️fixtures/{👥️two-package,🧱️generation-stage,🧬️stdio-gis-bootstrap}/🔣️.json}`
- `🌎️hub/🧪️tests/{🔏️trusted-catalog-profile,🪶️count,🔬️bin-unit}/🦀️.rs`
- `🌎️hub/🧫️fixtures/🪪️execution-target-relay-v1/🔣️.json`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/🏗️bootstrap/🎭️actors/🟦️.ts`
- `🌎️hub/🧩️compositions/🗄️stdio/{📦️packages/🦀️rust/📜️script.ts,📇️publication/{✅️trusted-stdio-catalog/🟦️.ts,📜️native-catalog.json},🧫️fixtures/🌳️catalog-root/🔣️.json}`

Other:
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-runtime/🟦️.ts`
- the generated `🔣️schema-catalog.json`

Ticket inputs: `trusted-catalog-receipts/{🏗️cargo.sh,🧪️dry-receipts.ts}`. Re-verification edits: `📇️registry/🧪️tests/{✅️catalog-complete,✅️catalog-publication}/🟦️.ts` (fixture paths), the generated `🔣️schema-catalog.json`.
