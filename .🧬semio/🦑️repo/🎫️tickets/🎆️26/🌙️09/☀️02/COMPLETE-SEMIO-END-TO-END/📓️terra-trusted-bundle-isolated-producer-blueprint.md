# Isolated Producer for the First Trusted Stdio + GIS Bundle

## Decision

Build the first trusted bundle with one **hub-owned tooling producer** that writes an immutable two-package generation under a private server root. It must never write a plugin owner descriptor, invoke registry generation, mutate a shared Cargo target, or register a codec. The candidate hub is the sole consumer which invokes the existing loader and process-global codec registration.

The packet produces exactly:

```text
profile:  local-stdio-gis-open-v1
packages: stdio / semio:stdio / exact Cargo version
          gis   / semio:gis   / exact Cargo version
codecs:   all 26 stdio rows + both GIS rows (Map and Terrain)
target:   only s.gis.gismap / gis.map / viewer / wasm / read+observe
```

This is a source blueprint. No producer, hub, browser, or native run was performed.

## Current Producer Boundaries

| Source | Reusable truth | Why it cannot be called as P0 |
| --- | --- | --- |
| [`plugin/describe` script](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:52) | `pluginWasmArtifactPath`, component build, JCO core extraction and descriptor emission are generic. | `describePluginComponent` writes directly to the caller owner root at [lines 89–108](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:89). |
| [`stdio catalog-root`](../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:533) | Has the best current isolated-root checks, bounded copying, child cancellation, JCO/WIT checks and staged row layout. | It snapshots/mutates the stdio owner descriptor and runs global registry generation at [lines 616–625](../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:616). It is stdio-specific. |
| [`gis describe`](../../../../../../✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:17) | Identifies the GIS package. | It calls the owner-root-writing generic describe helper; no isolated build/stage receipt exists. |
| [`registry` verifier](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2637) | `createFreshCatalogBuildVerifier` already verifies staged raw/core/pack bytes and a commit marker. | Its input is an owner-registry entry/source audit, so it cannot be the P0 bundle producer or require global generated rows. |
| [`TrustedCatalogLoader`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:348) | Bounded component+descriptor reads, SHA-256/BLAKE3 checks, descriptor/role checks, provider preview, full codec preflight, one later registration. | It registers codecs in its current process after verification ([line 497](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:497)); a tooling producer must not call it. |

## Canonical Limits

Adopt **4 MiB per descriptor representation**—both packed `.semio` and diagnostic JSON—as the one canonical descriptor boundary. The hub already enforces `TRUSTED_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024` ([`trusted-catalog`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:22)); its descriptor closure cap is 64 MiB. The registry instead applies `CATALOG_DESCRIPTOR_MAX_BYTES = 64 * 1024` to both owner pack and JSON ([`registry`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2037), [2391](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2391)).

The currently checked-in GIS pack is 51,116 bytes, but its rendered JSON is 214,795 bytes. It therefore fits the hub pack limit while failing the registry pair limit. That is an accidental producer/consumer split, not a security policy.

P0 change:

```text
TRUSTED_DESCRIPTOR_MAX_BYTES = 4 MiB       // Rust loader
CATALOG_DESCRIPTOR_MAX_BYTES = 4 MiB       // TS registry and fresh producer
fresh producer descriptorPackMax = 4 MiB
fresh producer descriptorJsonMax = 4 MiB
trusted descriptor closure max = 64 MiB    // unchanged, aggregate only
```

`64 KiB` remains an I/O chunk size, a Cargo component-manifest bound, or a protocol-specific cap where already named; it must not remain a second generic descriptor limit. The producer carries pack bytes into the trust bundle; JSON is diagnostic/equality evidence and is not a loader input. Keeping both under the same cap still makes the pair independently checkable without treating pretty-print expansion as a trust failure.

## Minimal Reusable Tooling API

Extract only the generic build/stage primitives from the two existing scripts into [`plugin/describe/📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts), where component build and descriptor emission already belong. This remains Bun tooling only—no Rust runtime dependency and no new package.

```ts
type FreshComponentRequestV1 = Readonly<{
  pluginId: string;
  cargoPackage: string;
  componentPackageId: string;
  outputName: string;
  componentProfile: "wasm-release";
  rootCdylib: boolean;
}>;

type FreshComponentReceiptV1 = Readonly<{
  pluginId: string;
  packageId: string;
  version: string;
  component: { readonly relativePath: string; readonly byteLength: number; readonly sha256: string; readonly blake3: string };
  descriptor: { readonly relativePath: string; readonly byteLength: number; readonly sha256: string };
  coreSha256: string;
  witExports: readonly string[];
}>;

type FreshBuildControlV1 = Readonly<{
  cancelled(): boolean;
  remainingMs(): number;
  checkpoint(stage: string, completed: number, total: number): void;
}>;

async function produceFreshComponentV1(
  request: FreshComponentRequestV1,
  freshTargetRoot: string,
  packageStageRoot: string,
  control: FreshBuildControlV1,
): Promise<FreshComponentReceiptV1>;
```

The implementation takes the useful code from stdio’s `requireEmptyFreshRoot`, `runControlled`, `copyCatalogArtifact`, JCO extraction and WIT checks, but it has no owner-root argument and no call to `atomicDescriptorPair`, `auditPluginCatalogSources`, `plugin-registry:generate`, or `createFreshCatalogCommitMarker`. It receives an already-created private target root, sets `CARGO_TARGET_DIR` only to that root, disables incremental compilation, derives the component filename through `pluginWasmArtifactPath`, and emits descriptor pack/JSON only below `packageStageRoot`.

The registry module should expose only a side-effect-free `verifyFreshCatalogPackageV1(receipt, expectedIdentity)` over the staged output. It may reuse the strict pack/JSON canonicality and hash checks from [`validateCatalogDescriptorPair`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2385), but accepts a supplied receipt instead of discovering an owner descriptor. Hub tooling calls this once per package.

## Output and Publication Protocol

The new `trusted-stdio-gis-bootstrap` command belongs in [`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:586) and is registered through its existing `project.json` router. The server creates the root below its own data/run directory; neither an HTTP request nor a client launch value may choose it.

```text
<server-private-root>/trusted-catalog/
  staging-<random-128-bit>/                 # same filesystem as generations
    trusted-catalog.json                     # closed v2 profile + full generation
    packages/
      stdio/
        component.wasm
        descriptor.semio
      gis/
        component.wasm
        descriptor.semio
  generations/<profile-generation-sha256>/  # atomic rename of complete staging tree
  current.json                               # one small server-owned generation pointer
```

Core WASM and rendered descriptor JSON are held only in private build work directories until their receipt checks finish; they are not executable/loadable files in the final trust root. All bundle paths are canonical relative paths under `packages/<plugin>/`, unique across component and descriptor entries. The existing loader’s containment and duplicate-path checks remain the final consumer guard ([`trusted-catalog`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:861)).

Publication sequence:

1. Make a non-symlink staging directory with owner-only permissions and unique component/descriptor files created with exclusive create. Check cancellation/deadline before and after every spawned build, byte chunk, JCO inspection, descriptor decode, hash and fsync.
2. Build **stdio** and **GIS** in distinct Cargo targets and work roots. Never read `target/`, development module cache, or existing owner descriptor files as a source of trust.
3. Validate each receipt’s Cargo package id/version, actual component SHA-256+BLAKE3, descriptor SHA-256, isolated role, descriptor app/dialect, and canonical pack/JSON equality. Write the bundle only after both receipts are complete.
4. Fsync each output, the bundle and staging directory; rename the complete staging directory once to the immutable generation path. A cancelled/failed attempt removes only staging.
5. Start a candidate hub with the immutable bundle path/profile. It alone calls `TrustedCatalogLoader::load`, so a failed candidate cannot pollute the old hub’s process-global registration state.
6. Only after candidate `/readyz` reports one open target and plan issuance succeeds, atomically replace `current.json` on the same volume with `{ profileId, generationId, bundleSha256 }`. A failed candidate leaves the old pointer and old process intact.

Use a regular data file for `current.json`, not a symlink, to keep Windows/macOS/Linux semantics uniform. The pointer is selection metadata, never a substitute for loading/verifying the immutable bundle bytes.

## Package Version and Full Closure Authority

GIS already compares its linked receipt’s `package_version` to the bundle record ([`preview_gis_bindings`](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:52)). Stdio currently discards the record version ([`preview_stdio_bindings`](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:48)); [`NativeCodecFactoryReceipt`](../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:852) must acquire a `package_version` from the compiled Cargo package and validate it before factory instantiation.

The bundle has exact closure rules:

```text
stdio nativeCodecs == all 26 linked stdio receipt identities
gis   nativeCodecs == [gismap, gisterrain] in canonical factory-id order
provider preview(package, version) == that package's declared closure exactly
no extra binding, no missing binding, no duplicate schema/kind/factory
only gismap appears in descriptor discoverability and the sole openTarget
terrain is a retained child codec; it has no target and no synthetic app/kind
```

This requires changing the loader’s current `descriptor.manifest.artifact_kinds.len() == record.native_codecs.len()` condition ([`trusted-catalog`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:833)) to a descriptor-discoverability subset check. Every descriptor-listed artifact kind must still match a declared codec; each target must still match both descriptor app and codec; the provider remains the authoritative proof of the two GIS codec rows. This keeps Terrain correctly available for typed child restoration without falsely making it independently openable.

## Generation and Restart Rotation

Replace target-only generation hashing at [`document_open_catalog_generation`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:606). It currently omits zero-target package bytes, so a changed stdio row can retain the same generation.

Define `profileGenerationV1` as SHA-256 over length-prefixed canonical fields:

```text
domain = "semio/hub/trusted-profile-generation/v1\0"
profile id
canonical selected package count
for each selected package, plugin/package/version/role,
  component SHA-256, component BLAKE3, descriptor SHA-256,
  sorted dependencies, sorted codec rows
sorted open-target rows including artifact, full parent dialect,
  surface/app/window/role/renderer/grant
```

The v2 bundle carries this generation id; the loader recomputes it after all byte and provider checks and rejects a mismatch. The document-open plan and socket/receipt revalidation use the recomputed full-profile generation. It covers a stdio-only rotation even though the only public target is GIS.

Rotation is restart-only. There is no unregistration/live replacement API after the loader commits codecs. A new immutable generation starts a candidate process; only a ready candidate moves `current.json` and causes old plan/receipt generations to be refused/reissued. The old process remains serving if candidate build, verification, load, bind or readiness fails.

## Required Gates

Register the following through the hub `📜️script.ts`/`project.json`; every focused native law must list first and require exactly one FQN before exact execution.

| Gate | Evidence required |
| --- | --- |
| `bun nx run os-hub:trusted-stdio-gis-bundle-check -- --source` | AJV schema plus independent Buffer/DataView framing, Node SHA-256 and first-party BLAKE3 oracle. Covers paths, 4 MiB boundary, package/version/role, 26+2 closure, terrain-not-target, complete generation and all cancellation points. |
| `bun nx run os-hub:trusted-stdio-gis-bundle-check -- --native` | Actual fresh stdio+GIS staged bytes, exact hub loader/provider, all-or-none 28 codec registration, one authenticated GIS map viewer plan and exact full-profile generation. No `abc`/synthetic descriptor substitute. |
| `bun nx run os-hub:trusted-stdio-gis-bundle-process-check` | Server-owned materializer → candidate hub → `/readyz` → authenticated plan → successful restart to new generation; failure injection leaves old process/current pointer usable. Register a seed-derived launch target, then generate `launch.json`; do not edit its generated entry directly. |
| `bun nx run os-hub:browser-document-open-check` | Negative boundary only: browser plan parsing/issue exchange must not convert this P0 plan into an arbitrary `pluginId,moduleUrl` load or claim execution. It stays red until the plan-derived immutable execution-target lease owns byte fetching and hash checks. |

Hostile vectors must include: stale target-only generation after a stdio byte change; SHA-256/BLAKE3 and descriptor-digest substitution; wrong stdio/GIS version; missing/extra Terrain; missing Map surface/window/role or non-WASM renderer; third mixed package/root/dependency; path escape/symlink/reused path; pack or JSON at 4 MiB+1; cancellation at every bounded stage; candidate crash before pointer update; restart with stale plan; and a failed rotation preserving the old pointer/process.

## Nonclaims

- There is still no signature/key-id in the bundle schema. This P0 is local server-root trust, not a remote signed distribution system.
- Browser and native clients do not consume the verified component/descriptor yet. Browser loading by module URL and native directory scanning remain invalid for this target; WGPU backbone attachment is separately RED.
- This producer does not make Flow openable, execute GIS inference, alter document membership, or activate a complete registry catalog.
