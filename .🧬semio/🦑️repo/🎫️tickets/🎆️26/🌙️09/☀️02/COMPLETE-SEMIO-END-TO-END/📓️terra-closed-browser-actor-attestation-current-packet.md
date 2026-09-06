# Closed Browser Actor Attestation: Current Integration Packet

## Decision

The smallest production path is a derived browser-actor record owned by the
trusted catalog, not another `PackageHashes` member and not a browser URL.
Its dependency order is acyclic:

```text
fresh verified raw component bytes + fixed codegen policy
  -> one closed ESM byte string + BrowserActor record
  -> trusted BundlePackage selection + catalog generation
  -> DocumentOpenPlan / TargetLease exact actor identity
  -> authenticated worker fetches, hashes, privately activates it
```

The descriptor does not describe the derived ESM, and an open plan does not
feed code generation. This is attestation only: current closed-bundle AST
fences and verified JCO provenance do not sandbox a guest. Production
activation still needs a worker-owned non-cooperative deadline/termination
and bounded aggregate argument/core-memory admission.

## Current seams

| Boundary | Current source | Required narrow change |
|---|---|---|
| Fresh package row | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2740` | The marker admits exactly `raw`, `core`, `descriptor`, and verifier returns their verified bytes. Generate `browser/closed-actor.mjs` only from that immutable raw output; add its exact marker receipt and row directory. |
| Package hashes | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs:9`, `:153`; `🖨️describe/📦️packages/🦀️rust/🦀️.rs:429` | Keep `PackageHashes` raw/core/descriptor-only. Its component build has no browser-codegen input. Adding a derived ESM hash would couple target-specific code generation to guest description and descriptor self-hashing. |
| JCO closure builder | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:72`, `:159`, `:308` | Export one build-only `buildClosedBrowserActorArtifactV1` here. It receives only verified raw bytes plus fixed policy, derives actual imports, calls the existing closure builder, and returns bytes plus identity. |
| Trusted catalog | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:102`, `:123`, `:210`, `:303`, `:347`, `:406`, `:670`, `:995` | `BundlePackage`, verified package, target assets, and generation currently omit an actor. Add/verify/retain the record/bytes, generation hash, and coherent Wasm target asset snapshot. |
| Plan / lease | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1362`, `:1441`, `:1635`, `:1728` | Add exact actor identity to plan and lease, `Some` iff renderer target is Wasm; copy it through `lease_fields_from_plan_v1` and strict equality. |
| Hub relay | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2310`, `:2475`, `:6386`, `:8502` | Add `browser-actor` as a fourth body kind through existing authenticated selection/final revision/catalog fence. No client receipt, URL, package id, or digest selector. |
| Private browser handoff | `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:521`, `:568`, `:725`, `:810`; `🏛️ShellHost/🟦️.tsx:1738` | Fetch/hash/activate only in the private branded lease and retain its close owner there. Never post bytes, Blob URL, receipt, lease, or module URL through broker, Shell, or plugin state. |

## Canonical record

Add this strict record to both trusted bundle and generated fresh marker:

```json
{
  "schema": "semio.os.closed-browser-actor.v1",
  "path": "browser/closed-actor.mjs",
  "byteLength": 0,
  "sha256": "lowercase-64-hex",
  "componentSha256": "selected BundlePackage.component.sha256",
  "descriptorByteSha256": "selected BundlePackage.descriptor.sha256",
  "importInterfaces": ["canonical sorted unique interface names"]
}
```

Rules:

1. `path` is a contained regular file inside the fresh row/bundle root; it is
   never sent to the browser.
2. Component and descriptor digests exactly equal the selected raw files. The
   latter is the descriptor file digest used by `DocumentOpenPackageV1`, not
   the descriptor's self-declared digest.
3. Imports are codegen-derived, sorted/unique, and restricted to framework
   `pure`, `host-async`, and exact first-party browser WASI interfaces.
4. Loader uses contained-path bounded read and hash/length verification before
   publication; absence is rejected for packages with a Wasm open target and
   presence is rejected otherwise.
5. Hash the complete actor record into `trusted_profile_generation`. Otherwise
   a changed ESM can retain a previous plan's generation fence.

Do not bind `coreWasmSha256` in this first record. JCO extracts one or more
embedded cores from the raw component while the descriptor's single core file
is separately produced and not loaded by trusted catalog. Equating them is
unproven; raw component digest plus derived ESM digest is sufficient.

Choose one explicit product output cap enforced by codegen, catalog loader,
Hub relay and worker. The generic builder permits 128 MiB cores, whose Base64
ESM may exceed 170 MiB. A lower 64 MiB GIS product cap is the safer first
admission if its real component fits; do not silently reuse incompatible input
and output bounds.

## Codegen and wiring

1. Extract the static-import inspection local to `closedBrowserComponentFactory`
   into an internal reusable result; do not use string matching.
2. `buildClosedBrowserActorArtifactV1(rawComponentBytes, fixedPolicy)` uses
   the installed JCO parse/transpile path already exercised at script line 308.
   Its map contains only admitted first-party interfaces, preserving full
   versioned JCO import keys. It derives actual imports, closes the ESM,
   UTF-8 hashes/bounds it, and returns `{bytes, sha256, byteLength, imports}`.
3. `createFreshCatalogBuildVerifier.verify` is sole input authority. Write a
   fixed browser filename and version-bump the marker with its receipt; extend
   `requireExactCatalogRow` so missing or unmarked browser files fail. Do not
   involve dev `🔌️plugin-modules`, a client path, or runtime directory scan.
4. Extend `BundlePackage`/schema, `VerifiedTrustedPackage`, and
   `VerifiedExecutionTargetAssets`; retain actor bytes and return all three
   assets from one catalog generation.
5. Add a required nullable `DocumentOpenBrowserActorV1` to Plan and lease. It
   is `{schema, sha256, byte_length, import_interfaces}` for Wasm and `null`
   for React/WGPU; validators reject the two inverse combinations. Move all
   producers and equality fences in the same change, without a legacy default.
6. Make `document_execution_target_selection` construct the plan/lease actor
   identity from trusted assets. Its new fourth relay body retains the existing
   8s bound, octet/no-store/content-length response, and final authority/head/
   catalog revalidation.
7. Extend the worker's private lease asset sequence. It validates all four
   assets before actor construction and uses an explicit retained async
   `Closing` state: all close/revoke/error sites await actor close once, then
   wipe bytes/revoke module URL. Do not fire-and-forget from synchronous drop.

## Acceptance corpus

1. Fresh-codegen neutral/source gate: real JCO fixture creates stable receipt;
   mutation of raw component, descriptor, import name/order, ESM bytes, core
   path, static/dynamic import rejects before marker publication.
2. Trusted native: missing/mismatched/escaped actor and unknown/noncanonical
   imports deny; valid Wasm selection returns exact component/descriptor/actor
   bytes; actor change produces a new generation.
3. Directory schema oracle: matching Wasm plan/lease passes; SHA, length,
   import, scope, or generation mutation fails; `Wasm + null` and
   `React/WGPU + actor` fail.
4. Extend Hub native law
   `execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body`
   with `browser-actor`: typed octet/no-store success and role/descriptor/
   revision/catalog/final-fence denial before body delivery.
5. Worker process: real Hub-served actor is digest-checked before import;
   invocation requires original-plan equality; close rejects later calls and
   retires resources. Public events remain status/progress only and
   `BrowserBrokerPortRequestV1` remains unchanged. No physical-module fallback.
6. Separate P0 resource laws: a non-cooperative guest is worker-terminated at
   deadline; oversized aggregate arguments/core memory reject pre-invocation.
   Existing source/core byte and invocation-count limits are not substitutes.

## Non-claim

`closedBrowserActorBundle` is real guest-qualified in fixtures but is absent
from trusted catalog rows, plans, leases, target routes, and worker activation.
The present `renderer-unavailable` status is therefore accurate. Implement the
codegen/marker/catalog/plan/relay path first; activate only after retained
async close and the execution-resource acceptance above.
