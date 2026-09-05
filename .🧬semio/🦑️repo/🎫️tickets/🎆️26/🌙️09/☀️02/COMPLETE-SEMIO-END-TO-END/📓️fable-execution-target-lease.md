# Fable — Browser GIS WASM Execution-Target Lease (P0)

Lane: `fable-execution-target-lease`. Packet: `📓️terra-browser-gis-wasm-execution-target-lease-p0.md`, with
`📓️terra-gis-execution-target-lease-blueprint.md` as the design source.

## Boundary

This lane lands the schema-first `DocumentExecutionTargetLeaseFieldsV1` contract, one shared
`sameLeaseFieldsV1` relation in TypeScript and Rust, the hub's exact-selection asset port plus three
protected document-scoped routes, the browser broker/verify/lease path with an explicit localized
`renderer-unavailable` terminal, the neutral `document-execution-target-lease-v1` corpus with its
independent Node state-machine oracle, and three registered gates.

It does **not** claim WASI execution in a browser, WGPU map rendering, or a completed process test.
Those non-claims are restated at the end with the exact evidence that does exist.

## Contract

`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs}` now carry one
receipt-free public value:

```text
DocumentExecutionTargetLeaseFieldsV1 {
  schema "semio.os.document-execution-target-lease/v1", version 1
  scope { spaceId, documentId }          // both ids, never documentId alone
  descriptorDigestV1
  catalog { generationId }
  package { pluginId, packageId, version, componentSha256, componentBlake3, descriptorByteSha256 }
  component { sha256, blake3, byteLength }
  descriptor { sha256, byteLength }
  artifact { kind, schema, packSchemaHash }
  parentDialect { artifactKind, standard, subset }
  surface { surfaceId, appId, windowKindId, role, rendererTarget }
  grant { read: true, write, observe: true }
  checkpoint?  revalidation
}
```

Enforced invariants (both languages): `component.sha256 == package.componentSha256`,
`component.blake3 == package.componentBlake3`, `descriptor.sha256 == package.descriptorByteSha256`,
byte lengths in `1..=64 MiB` / `1..=4 MiB`, `parentDialect.artifactKind == artifact.kind`,
`grant.write == (surface.role == "editor")`, checkpoint descriptor-digest equality, and exactly one of
`sessionGeneration`/`shareGeneration`.

A plan receipt, socket grant, session token, hub origin, raw path and module URL are **not** fields and
**not** constructors. `hubOrigin` is retained only by the private owner, taken from the credential-owning
broker/client, never from hub JSON.

`InstalledDocumentExecutionTargetV1` was **replaced**, not aliased:
`PersistenceBinding.hub.installedTarget` is now `DocumentExecutionTargetLeaseFieldsV1`
(`🧰️framework/🛍️products/💻️os/🟦️.ts`). No compatibility form remains.

### One shared relation

- TypeScript: `leaseFieldsFromPlanV1(plan, byteLengths)` and `sameLeaseFieldsV1(left, right)`.
- Rust: `lease_fields_from_plan_v1(plan, component_len, descriptor_len)` and `same_lease_fields_v1`.

The plan constrains every identity but no byte length, so both lengths come from the installation under
comparison and are independently enforced against the exact streamed bytes before a lease exists. This is
documented on both functions.

Replaced partial predicates:

- `DocumentSocketAuthorityV1::matches_surface` (artifact/package/version/surface only) is **deleted**; it
  is now `matches_lease_fields`, which routes through `same_lease_fields_v1`.
- `DocumentSocketSurfaceExpectationV1` is **deleted**. `DocumentSocketExpectationV1.surface` became
  `lease: Option<DocumentExecutionTargetLeaseFieldsV1>`.
- The store-sync reconnect expectation (`🏪️store/🔄️sync/🦀️.rs`) retains
  `document_execution_target_lease` and compares it with `authority.matches_lease_fields(lease)` at the
  post-connect fence, replacing the schema/pack-hash/surface subset.

Two callers had **no verified bytes** and therefore can no longer claim any local execution target — an
honest downgrade, not a silent weakening:

- the MCP probe (`🌉️mcp/🏠️workspace/🦀️.rs`) dropped `probe_document_socket_surface()` and now binds only
  `PersistenceBinding::Hub { surface: Some(PROBE_SURFACE_ID) }`;
- the WGPU shell (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) `document_socket_surface_from_descriptor` keeps its
  descriptor-bound validation but returns the canonical **surface id** (a preference) instead of a
  forgeable partial authority.

The hub source gate assertion that previously required the MCP preclaim was inverted to forbid it.

### Private owners

Browser (`🧵️backbone-worker.ts`, region `🪪️ExecutionTargetLease`): `DocumentExecutionTargetLease` is a
module-private class whose constructor demands a module-private mint symbol, is `Object.freeze`d, owns the
verified component/descriptor buffers and any private module URL in `#private` fields, exposes only a
frozen `structuredClone` fields view, and wipes buffers plus revokes the URL on `drop()`. It is never
posted, cloned, or encoded.

Native: `DocumentSocketAuthorityV1::lease_fields` projects the retained authority; the comparison input is
the full fields value. No `FromValue` constructor mints a lease from wire bytes.

## Server: exact-selection asset port

`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`:

```rust
pub fn assets_for_current_selection(
    &self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>,
    writable: bool, current_generation: &str,
) -> Option<VerifiedExecutionTargetAssets>
```

It is not a package lookup: it accepts no package id, digest, path or generation *selector*, answers only
while `current_generation` is still this immutable catalog's own, re-finds the retained package by full
identity **and** all three digests, and bounds both bodies by the catalog's own
`TRUSTED_COMPONENT_MAX_BYTES` (64 MiB) / `TRUSTED_DESCRIPTOR_MAX_BYTES` (4 MiB).

`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`, in a separated `//#region 🪪️ExecutionTargetLease` block, adds:

```text
POST /spaces/{space}/documents/{document}/execution-target/manifest
POST /spaces/{space}/documents/{document}/execution-target/component
POST /spaces/{space}/documents/{document}/execution-target/descriptor
```

Each accepts only the bounded `DocumentOpenIntentV1` (8 KiB, `application/json`, no query), rejects a
scope mismatch, requires the `openPlan` readiness fence, re-authenticates the exact session/share binding
and revalidates it, reloads the durable descriptor, resolves the current selection through the accessor
alone, recomputes `descriptorDigestV1`, the active checkpoint and the revalidation generations, builds and
`validate()`s the same `DocumentExecutionTargetLeaseFieldsV1`, and only then answers strict fields JSON or
raw `application/octet-stream` bytes with an exact `Content-Length` and `Cache-Control: no-store`. A
10-second request deadline bounds the whole handler.

Because every body request repeats the whole selection and authorization, a rotation or role change
between manifest and body is a mismatch or denial rather than a mixed answer. The plan exchange remains
the server's final stale fence; the asset port issues no receipt and is not a reusable download credential.

`DocumentOpenCatalogAuthorityV1` gained the accessor so the hub's erased catalog handle and the test
double both implement it; the test double now carries real component/descriptor bytes.

## Browser slice

All in `🧵️backbone-worker.ts`, inside the `🪪️ExecutionTargetLease` region plus the plan/authority seam:

1. `receiptFreeFields(plan, byteLengths)` delegates to the shared `leaseFieldsFromPlanV1`.
2. `browserExecutionTargetAssetRequest` is a broker operation that can make **only** the three protected
   asset calls: it re-derives the path, refuses anything that is not
   `/spaces/{s}/documents/{d}/execution-target/{manifest|component|descriptor}`, refuses a scope that is
   not the binding's own, always POSTs the bounded intent, and shares `state.docAbort.signal` and the
   `SOCKET_GRANT_REQUEST_TIMEOUT_MS` deadline with `requestDocumentSocketAuthority`.
3. `readExecutionTargetManifestJson` bounds the manifest at 8 KiB; `readExecutionTargetBody` requires an
   exact `Content-Length` equal to the manifest byte length, caps at the shared 64 MiB / 4 MiB maxima,
   checks cancellation before every chunk, wipes and cancels the reader on any failure, and emits bounded
   64 KiB progress carrying only `{stage, completedBytes, totalBytes}`.
4. SHA-256 comes from Web Crypto (`crypto.subtle.digest`). BLAKE3 comes from the **first-party** runtime
   module `🧰️framework/🔨️modules/🔏️hash/🟦️.ts`, which is the algorithm moved out of the dev-only
   `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`; that script is now an ordinary importer of it, as are the
   other existing consumers, via `@semio-tech/framework`. Web Crypto is never claimed to provide BLAKE3.
5. `parseVerifiedPackageDescriptorV1` strict-decodes the raw descriptor pack, requires **canonical
   re-encoding byte equality**, and then requires `descriptorVersion == 1`, `packageId`, `manifest.pluginId`,
   `manifest.version`, `hashes.wasmSha256 == component.sha256`, `execution == "isolated"`, the exact app
   (`id == surface.appId == surface.surfaceId`), its role, all three parent-dialect coordinates, the window
   kind and the artifact kind/schema to equal the lease fields. A sibling JSON manifest or a caller URL is
   never descriptor authority.
6. The private lease is minted only after both byte verifications. `documentOpenPlanAuthority` then admits
   `rendererTarget !== "react"` **only** when a live lease is supplied and its own verified fields are the
   comparison input. The verified GIS Map target is routed to an explicit localized
   `renderer-unavailable` state; the lease region contains no `loadPluginModule`, `ActivationRegistry`,
   `load_wasm_plugins` or `attach_backbone` reference, and the registered gate asserts that.
7. The plan receipt is exchanged and the socket opened only while the lease is live. The lease is dropped
   (buffers wiped, private URL revoked) on cancellation, timeout, asset/manifest mismatch, plan
   expiry/stale response, revalidation change, descriptor/checkpoint change, catalog rotation, socket
   close/rebootstrap and `closeArtifact`.
8. `relayMutationsToHub` rejects a verified viewer-only target **locally**, before any outbox entry or
   worker frame, through the existing `CommandAckOutcome` vocabulary.

Accessibility: a new bounded `execution-target-status` worker response carries only a status code and byte
counters. `🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx` adds `reduceExecutionTargetUiState` and
`ExecutionTargetStatusNotice`, which render `role="status"`/`aria-live="polite"` for `verifying` and
`role="alert"`/`aria-live="assertive"` for `integrity-failed`, `stale`, `cancelled` and
`renderer-unavailable`, in explicit `"en" | "de"` with no default language. `🏛️ShellHost/🟦️.tsx` routes
the worker response into that live region and clears it on detach. The rendered text is the complete UI
payload: no origin, path, receipt, grant, digest or user identity, and nothing is persisted into the
shared document.

## Neutral corpus

`🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/{🔣️.json,🧬️.schema.json}`, mirroring
the sibling `📅️event-page-route-v1` folder convention.

One positive vector: a read-only GIS Map **viewer** (`s.gis.gismap`, `gis.map`,
`s.gis.gismap@1/*#viewer`, `gis2d-main`, `rendererTarget: "wasm"`, `grant.write: false`) with 1024 exact
component bytes and 607 exact canonical descriptor bytes pinned as hex, and their real SHA-256/BLAKE3
digests.

58 hostile rows, every one expecting `unpublished`: 35 single-field manifest substitutions (both scope
ids, descriptor digest, catalog generation, every package field, every component/descriptor byte field,
artifact, all three dialect coordinates, every surface coordinate, every grant bit, checkpoint fields and
every revalidation generation), 12 byte vectors (component substitution/truncation/extra byte/`max+1`/
missing body; descriptor substitution/trailing byte/noncanonical/self-hash/`max+1`/missing body), and 11
lifecycle vectors (generation-A manifest with generation-B component, stale plan after rotation,
cancellation at manifest/component/descriptor/hash, deadline, reconnect after lease invalidation, viewer
write attempt, caller URL/path/module substitution).

Oracles:

- **AJV 2020** validates the corpus and each mutated lease-fields candidate.
- An **independent Node state machine** (`executionTargetLeaseInstall` in the hub `📜️script.ts`) walks
  manifest → component → descriptor → verify → exchange with its own hand-written full-field relation and
  structural admission, Node `createHash` plus `webcrypto` SHA-256 agreement, and the first-party BLAKE3
  known-answer vector. It imports no production parser or comparison.
- A **Rust fixture reader** in `os_directory::client::tests` decodes the same corpus, hashes the same
  bytes with `semio_framework_hash`, and drives `lease_fields_from_plan_v1` / `same_lease_fields_v1` /
  `matches_lease_fields` over every single-field substitution.

## Gates

Registered in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `📋️project.json` and `.vscode/🧩️launch.seed.jsonc`
(`⚖️gate🔏️execution-target-lease🌎️hub` / `🖥️native` / `🌐️browser`), mirroring `browser-document-open-check`:

```sh
bun nx run os-hub:execution-target-lease-check --skip-nx-cache
bun nx run os-hub:execution-target-lease-check --skip-nx-cache -- --native
bun nx run os-hub:execution-target-lease-browser-check --skip-nx-cache
```

The existing `browser-document-open-check` oracle no longer hard-asserts `rendererTarget === "react"`: its
hand-written relation now compares the complete lease-fields projection and takes the renderer target from
the installation, so the same regression covers a lease-path target.

### Evidence

All commands were run from the repository root on 2026-09-05 unless a `cwd` is given. Durations are wall
clock. The shared `target/debug` lock was held by concurrent peer builds (`cargo test -p
semio-s-plugin-process --lib`, `cargo check -p semio-s-plugin-lowpoly --lib`, and others) throughout, so
every cargo entry below records its queue time as part of its duration.

#### Corpus identity

The positive vector's exact bytes are pinned in the fixture and re-derived by both oracles:

```text
component  byteLength 1024  sha256 0fb19bbf5000750836d0343893c24471a0d65cbc0ef8e7a9ef9ce4208d58cb5c
                             blake3 96eda1f99692027627c313f59e1acc20ccf394e6f5716dbaad855aaee9a84996
descriptor byteLength  607  sha256 0aa4c78847b0ae7835bb85b8e5871d2088167bbfac622af41240ef6635aa946b
catalog generation A 7cf0515d5cb12f9404020fef548b46f6b0b3dca140d4e1f2b58329174a40ebef
catalog generation B 73021a591414fb30135171f32473f6264a1b85677ba78feb00f7db2c94dd462b
hostile rows 58
```

`cargo check` produces no executable, so no build artifact hash is recorded for the check entries; the
component/descriptor digests above are the packet's byte identity and are asserted from three independent
implementations (Node `createHash`, `webcrypto.subtle`, and Rust `semio_framework_hash`).

#### `os-hub:execution-target-lease-check` (source + neutral oracle)

```sh
bun nx run os-hub:execution-target-lease-check --skip-nx-cache
```

exit 0 in 66 s. Verbatim:

```text
execution-target-lease-oracle: ajv=1 positive=1 manifest-fields=35 byte-vectors=12 lifecycle=11 hostile=58 component-bytes=1024 descriptor-bytes=607 node+webcrypto-sha256=agree first-party-blake3=known-answer status=5 passed
execution-target-lease-source: browser lease region=renderer-free hub routes=3 accessor=generation-bound native=full-field passed

 NX   Successfully ran target execution-target-lease-check for project os-hub
```

35 single-field manifest substitutions, 12 byte vectors and 11 lifecycle vectors were each required to
end `unpublished`; the positive vector was required to reach `published`.

#### `os-hub:execution-target-lease-browser-check`

```sh
bun nx run os-hub:execution-target-lease-browser-check --skip-nx-cache
```

exit 0 in 131 s. Verbatim tail:

```text
execution-target-lease-oracle: ajv=1 positive=1 manifest-fields=35 byte-vectors=12 lifecycle=11 hostile=58 component-bytes=1024 descriptor-bytes=607 node+webcrypto-sha256=agree first-party-blake3=known-answer status=5 passed
execution-target-lease-source: browser lease region=renderer-free hub routes=3 accessor=generation-bound native=full-field passed
 Test Files  1 passed | 2 skipped (3)
      Tests  3 passed | 255 skipped (258)
   Duration  41.70s (transform 95.29s, setup 0ms, import 110.62s, tests 2.86s, environment 0ms)
execution-target-lease-browser-check: neutral corpus, source boundary and browser Worker verify/reject/renderer-unavailable runtime passed
```

The three passing browser tests are the packet's exact names:
`browser execution target lease verifies GIS wasm bytes before plan exchange`,
`browser execution target lease rejects every single-field substitution without publication`,
`browser GIS viewer exposes localized renderer-unavailable after verified lease`.

The first drives a real worker open in which the caller supplies **only** `requestedSurfaceId` — nothing
forgeable — and asserts the exact request order
`open-plan → execution-target/manifest → execution-target/component → execution-target/descriptor →
socket-grants`, that each asset body is a POST of the bounded intent containing no receipt, that the
receipt appears only in the final exchange, that the private lease is live and field-equal to the
manifest, that all four progress stages were announced, and that no status payload contains the receipt,
socket grant, hub origin, any digest or a `blob:` URL. The second replays all 58 hostile rows through
`requestDocumentSocketAuthority` and requires an error, a null lease, no `socket-grants` request and zero
WebSocket instances for every one. The third asserts the EN/DE text and `status`/`alert` roles against the
corpus, that the lease region's source contains no `loadPluginModule`, `ActivationRegistry`,
`load_wasm_plugins` or `attach_backbone`, and that a viewer-only mutation is rejected locally with an
empty outbox and no socket frame.

#### `os-hub:browser-document-open-check` (existing regression)

```sh
bun nx run os-hub:browser-document-open-check --skip-nx-cache
```

Its neutral-oracle phase — the phase this lane changed — passed verbatim:

```text
browser-document-open-oracle: ajv=1 paths=3 installed-target=1 scope-keys=2 authority=1 exchange=1 websocket=1 rust-worker-bypass=denied hostile=25 bound=65536 redaction=6 passed
```

All 25 hostile vectors are still denied by the rewritten full-field relation, which no longer hard-asserts
`rendererTarget === "react"` and instead takes the renderer target from the installation.

The gate as a whole exits 1 in 160 s, in a phase this lane did not change. After the oracle passes, its
Playwright phase starts a fresh Vite dev server for the OS shell and gives the first page load a
hard-coded 10-second budget, which this box does not meet under the concurrent load described above:

```text
await page.goto(uiOrigin, { waitUntil: "domcontentloaded", timeout: 10_000 });
TimeoutError: goto: Timeout 10000ms exceeded.
Call log:
  - navigating to "http://127.0.0.1:52940/", waiting until "domcontentloaded"
      at proveBrowserDocumentOpenRuntime (🌎️hub/📦️packages/🦀️rust/📜️script.ts:1943:16)
```

That is an environment/load timeout on a cold shell bundle (the same run logs Babel deoptimising two
500 KB+ modules), not an assertion failure and not a regression from this lane. Its later
`open-plan-server-check` tail would in any case hit the `semio-framework-plugin-host` blocker recorded
below. An earlier attempt of the same gate was killed by my own 10-minute command cap mid-Vite-build; the
160 s run above is the complete one.

#### `🧵️backbone-worker.ts` vitest file (whole file, not filtered)

```sh
bun node_modules/vitest/vitest.mjs run --config 🧪️tests/🟦️.ts --testTimeout 120000 --run ../../🧵️backbone-worker.ts
# cwd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript
```

exit 1 in 25 s: `Tests 1 failed | 55 passed (56)`. The single failure is **outside this lane's regions** —
it is the sibling `fable-space-administration` lane's invite-capability test:

```text
FAIL ../../🧵️backbone-worker.ts > backbone-worker space administration > retains the invite capability until exact clipboard success and rejects duplicate results without redisclosure
AssertionError: expected false to be true // Object.is equality
- Expected  + Received
- true
+ false
 ❯ ../../🧵️backbone-worker.ts:3374
```

Line 3374 asserts
`message.kind === "directory-administration-state" && message.inviteCapabilityPending === true`, a
`DirectoryAdministrationOperation` assertion this lane never touches. Every document-open, execution-target
lease, broker, directory-command and scoped-stream test in the same file passed.

The single 10-minute retry the brief allows was run and the external blocker had cleared — the peer landed
their fix in the interval. The same command then exited 0 in 13 s:

```text
 Test Files  1 passed (1)
      Tests  57 passed (57)
```

That is the current terminal for this file: the whole `🧵️backbone-worker.ts` suite, including this lane's
three execution-target-lease tests and the pre-existing browser-document-open tests, is green.

#### First-party BLAKE3 relocation

```sh
bun -e '<import the dev script, the composition-identity script and @semio-tech/framework>'
```

```text
dev re-export blake3Hex: function function
abc: 6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85
composition identity module loaded: true
framework blake3Hex: function
```

The official BLAKE3 vectors for `abc` and the empty input
(`af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262`) both reproduce from the new runtime
module, and every pre-existing importer of the dev script's `blake3Hex` still resolves.

#### Launch generation

```sh
bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache
bun nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache
```

`plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 45 framework packages) … .vscode/launch.json regenerated`,
then `plugin registry generated catalog and launch bytes are fresh.` The first `check-generated` attempt
reported `Generated registry output is stale: .vscode/launch.json` because a peer edited the seed between
my generate and check; a regenerate-then-check pair immediately after was clean.

#### Native — external blocker

```sh
CARGO_BUILD_JOBS=4 cargo check -p semio-hub --bin os-hub --message-format=short
```

This cannot pass on current source, for a reason outside this lane. `semio-hub` depends on
`semio-framework-plugin-host` (`🌎️hub/📦️packages/🦀️rust/Cargo.toml:41`), which fails to compile:

```text
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/../../📥️imports/🦀️.rs:428:14: error[E0004]: non-exhaustive patterns: `component::actor_bindings::semio::framework::effects::Effect::RequestInferenceProposal(_)` not covered: pattern `component::actor_bindings::semio::framework::effects::Effect::RequestInferenceProposal(_)` not covered
error: could not compile `semio-framework-plugin-host` (lib) due to 1 previous error; 9 warnings emitted
```

`RequestInferenceProposal` is the concurrent GIS-inference lane's new WIT effect variant. That lane
updated `🔌️plugin/🖥️host/🦀️.rs`, `🔌️plugin/⚛️reactor/🦀️.rs` and `🎠️kernel/🦀️.rs`, but not the second,
deliberately duplicated `wit_effect_to_kernel` match in `🔌️plugin/🖥️host/📥️imports/🦀️.rs` (that function's
own docstring calls itself a "Local copy … see this region's own doc above for why a second copy"). This
lane touches no effect, plugin-host, reactor or inference source and did not repair another lane's file.

Consequence: the two hub-side laws —
`artifact_authority::trusted_catalog::tests::selected_execution_target_assets_are_generation_and_digest_bound`
(`semio-hub --lib`) and
`bin::tests::execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body`
(`--bin os-hub`) — are **written and registered but unrun**. They are not claimed as passing.

Contention context: the shared `target/debug` lock was held continuously by peer builds
(`cargo test -p semio-s-plugin-process --lib`, `cargo check -p semio-s-plugin-lowpoly --lib`,
`cargo check -p semio-hub --lib --features sqlite`, `cargo check -p semio-hub --bin os-hub --all-features
--tests` from the sibling `fable-directory-command-receipt` lane, among ~12 concurrent cargo processes).
My shared-target check sat at `Blocking waiting for file lock on build directory` for 35 minutes at 0 % CPU
and was retired; the diagnostic above came from an isolated `CARGO_TARGET_DIR` build that was never
lock-blocked, so it is a real current-source diagnostic, not a stale queued one.

#### Native — kernel law

```sh
CARGO_TARGET_DIR=<private> CARGO_BUILD_JOBS=4 RUST_MIN_STACK=268435456 \
  cargo test -p semio-framework-os-kernel --lib execution_target --message-format=short -- --test-threads=1
```

`semio-framework-os-kernel` has no dependency on `semio-framework-plugin-host`, so this law is not blocked
by the inference-lane breakage above. It was run three times against an isolated `CARGO_TARGET_DIR` (never
lock-blocked): 1123 s cold, then 184 s and 240 s warm.

The first run produced **two diagnostics owned by this lane**, and they were real:

```text
🔨️modules/📇️directory/🔌️client/🦀️.rs:2395:34: error[E0425]: cannot find function `document_lease_fields` in this scope: not found in this scope
🔨️modules/📇️directory/🔌️client/🦀️.rs:2502:25: error[E0425]: cannot find function `document_lease_fields` in this scope: not found in this scope
```

Cause: this lane deleted `document_lease_fields` as unused after replacing
`DocumentSocketExpectationV1.surface` with `lease`, while a concurrent editor had meanwhile added two
native admission tests that call it (`expectation.lease = Some(document_lease_fields(...))` for the exact
case, and a `lease.package.package_id = "foreign.package"` substitution for the hostile case). The helper
was restored, projecting the same neutral `🧭️document-open-plan-v1.json` `validPlan` that
`push_document_plan` serves through the one shared `lease_fields_from_plan_v1` at pinned byte lengths, so
those tests can only fail by changing a real field. Both diagnostics are gone from the two later runs.

The law still did **not execute**: the kernel test binary does not link because of seven unresolved
symbols in the sibling `fable-directory-command-receipt` lane's in-progress tests in the same file:

```text
🔨️modules/📇️directory/🔌️client/🦀️.rs:2724:13: error[E0425]: cannot find function `directory_command_sha256` in this scope: not found in this scope
🔨️modules/📇️directory/🔌️client/🦀️.rs:2725:13: error[E0433]: cannot find type `DirectoryCommandOutcomeV1` in this scope: use of undeclared type `DirectoryCommandOutcomeV1`
🔨️modules/📇️directory/🔌️client/🦀️.rs:2727:13: error[E0433]: cannot find type `DirectoryCommandResultV1` in this scope: use of undeclared type `DirectoryCommandResultV1`
🔨️modules/📇️directory/🔌️client/🦀️.rs:2754:38: error[E0425]: cannot find function `directory_command_sha256` in this scope: not found in this scope
🔨️modules/📇️directory/🔌️client/🦀️.rs:2759:83: error[E0425]: cannot find function `directory_command_sha256` in this scope: not found in this scope
🔨️modules/📇️directory/🔌️client/🦀️.rs:2759:119: error[E0433]: cannot find type `DirectoryCommandOutcomeV1` in this scope: use of undeclared type `DirectoryCommandOutcomeV1`
🔨️modules/📇️directory/🔌️client/🦀️.rs:2759:179: error[E0433]: cannot find type `DirectoryCommandResultV1` in this scope: use of undeclared type `DirectoryCommandResultV1`
error: could not compile `semio-framework-os-kernel` (lib test) due to 7 previous errors; 84 warnings emitted
```

Those three symbols are theirs, are referenced only from their tests, and are absent from the file's
`use super::schema::{…}` list; their line numbers moved between runs (2709→2724), which is direct evidence
they are mid-edit. The mandated single 10-minute retry was taken and the import was still not present.
This lane did not edit their region.

So the count is: of the three registered native laws, **zero executed**; this lane's own compile
diagnostics went 2 → 0, and the remaining blockers are two other lanes'.

#### `os-hub:execution-target-lease-check -- --native`

Not run to completion. Its first selector is
`cargo test --manifest-path Cargo.toml --lib selected_execution_target_assets_are_generation_and_digest_bound -- --list`
against `semio-hub`, which cannot compile for the `semio-framework-plugin-host` reason above, so the gate
fails its non-vacuity preflight before any law. Nothing is claimed from it. The gate itself is registered
and its exact-one-law selection contract mirrors `open-plan-server-check`; it has simply never had a
compilable tree to run against during this lane's window.

## Nonclaims

- No browser or native process has executed a WASI component through this lease. The browser terminal is
  an explicit localized `renderer-unavailable` state, and that is exactly what the passing browser tests
  assert.
- No WGPU rendering, GIS map UI, document member open, collaboration mutation, plugin hot rotation or
  inference authority is claimed.
- **No native law ran.** All three registered native laws
  (`artifact_authority::trusted_catalog::tests::selected_execution_target_assets_are_generation_and_digest_bound`,
  `bin::tests::execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body`,
  `os_directory::client::tests::execution_target_lease_compares_every_plan_and_verified_byte_field`) are
  written, registered and selected by the gate, but none has executed. The hub pair is blocked by the
  inference lane's `Effect::RequestInferenceProposal` non-exhaustive match in
  `semio-framework-plugin-host`; the kernel one is blocked by the directory-command lane's seven
  unresolved test symbols. No Rust assertion in this packet is claimed as passing.
- `cargo check -p semio-hub --bin os-hub` has **never** completed on current source during this lane's
  window, so the hub route region is unproven by a compiler. The only compile evidence this lane owns is
  negative-turned-clean: its own two `document_lease_fields` diagnostics appeared and were fixed, and no
  other diagnostic in this lane's files appeared in any run.
- `os-hub:browser-document-open-check` did not reach green end to end; only its neutral-oracle phase —
  the phase this lane rewrote — is claimed.
- The real process test (authenticated viewer opens GIS Map against a server-owned current generation,
  verifies assets through the protected route, exchanges once, shows verified-but-renderer-unavailable,
  then rotation invalidates generation A and requires a fresh B lease) is **not** run: it depends on the
  trusted-bundle native gate, which `📓️sol-trusted-stdio-gis-bundle.md` still records as unrun.
- What *is* proven by execution: the schema-first contract, the one shared `sameLeaseFieldsV1` relation in
  TypeScript, the neutral corpus with its AJV + independent Node state-machine oracle (58 hostile rows,
  all `unpublished`), the source boundary of every region above, and the browser Worker path end to end —
  manifest/component/descriptor fetch, streamed bounded reads, Web Crypto SHA-256 plus first-party BLAKE3,
  strict canonical descriptor admission, private lease minting, plan exchange only while live, and local
  viewer-write rejection.
