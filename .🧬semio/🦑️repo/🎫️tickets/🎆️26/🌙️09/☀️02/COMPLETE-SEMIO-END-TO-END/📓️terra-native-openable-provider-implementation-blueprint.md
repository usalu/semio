# Terra Native Openable Catalog Provider V1 Blueprint

Date: 2026-09-04  
Scope: current-source, read-only implementation blueprint for the first real hub native codec provider. No product/test source, build, Cargo target, launcher, or runtime was changed or run. “Source-backed” is deliberately not runtime evidence.

## Decision

**Implement one static `stdio` provider set, with the single first document-open target `stdio.json` in a viewer surface.** The provider must enumerate the complete statically executable `stdio` codec closure, not just JSON: the trusted loader requires the package manifest artifact-kind count and the selected trust record's `nativeCodecs` to agree, and rejects an unconsumed/extra native binding. A profile may expose one JSON viewer `openTarget`, but its `stdio` package row must carry—and the hub must bind—the exact full declared codec set.

This is the smallest honest way to make hub D0 `open_plan_ready` nonempty. It does **not** claim a browser/WGPU/MCP document mount, renderer availability, a generated marketplace row, or installed-component provenance beyond the verified bundle inputs below. The current hub supplier is still a deliberate empty fail-closed stub at [`📦️bin.rs:393-395`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:393); startup consequently reports an open plan only when a configured catalog actually contains a target at [`📦️bin.rs:5248-5283`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5248).

`stdio.json` is the first target because it already has a public static factory receipt path. Its artifact kind and codec schema are both `stdio.json` ([`🗿️artifacts/🔣️json/🦀️.rs:11-29`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🦀️.rs:11)); its factory identifier is `stdio.native.json.v1` ([`📇️registry/🦀️.rs:875,910`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:875)). This must not be confused with a separately declared app dialect such as `s.stdio.json`: the trust bundle must take its `appId`, `windowKindId`, and surface identifier from the emitted descriptor rather than hand-written source constants.

## Current Trusted Inputs And What They Prove

| Input | Required producer/admission fact | What it does **not** prove |
| --- | --- | --- |
| Stdio raw component and core module | `@semio-tech/stdio-plugin:catalog-root` emits bounded raw/core Wasm only in a caller-owned empty root, runs independent WebCrypto/WIT/descriptor checks, and atomically stages the row ([`📜️script.ts:434-545`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:434)). | A current cache row or a marketplace registry row is not a deployment receipt. |
| Descriptor pack/JSON and catalog commit marker | The same command publishes the owner descriptor pair only after staged row publication, then checks generated registry and creates the strict receipt/commit marker ([`📜️script.ts:512-538`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:512)). | The generic registry's `catalog-complete` audit has historically ignored owner descriptors; it cannot substitute for this stdio-specific receipt. |
| Trusted bundle + profile | Hub accepts `OS_HUB_TRUSTED_CATALOG_BUNDLE` and `OS_HUB_TRUSTED_CATALOG_PROFILE` only as a pair ([`📦️bin.rs:379-390`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:379)). Loader bounds/canonicalizes paths, validates component byte length plus SHA-256+BLAKE3, descriptor SHA-256, descriptor/package identity, dependencies, codecs and targets ([`trusted-catalog/🦀️.rs:337-476,765-815`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:337)). | There is no source evidence that the bundle is signed. Call it a verified trusted deployment input, not a signature. |
| Native static receipt | `native_codec_factory_receipts()` validates source schema, runtime artifact/descriptor bijections, `component_package_id`, non-zero lower-case pack hash, and instantiates each exact function before returning receipts ([`📇️registry/🦀️.rs:813-840,981-1040`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:813)). | Compile-time linking a Rust factory does not show it was built from the same Wasm bytes furnished in a deployment bundle. |

The static registry presently defines **26** native factories ([`📇️registry/🦀️.rs:861-924`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:861)). This is a source-backed executable factory set; it is not yet linked into hub: the hub Cargo manifest has no `semio-s-plugin-stdio` dependency ([`Cargo.toml:30-48`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/Cargo.toml:30)), and the active binding function returns no entries.

## V1 Boundary And Exact Data Flow

Add a hub-private bootstrap module adjacent to the artifact-authority code, with a narrow result API such as:

```text
NativeOpenableCatalogProviderV1
  provider_id: "stdio/native-codecs/v1"
  bindings: Vec<NativeCodecBinding>     // private `ArtifactCodec` function handles

linked_native_codec_bindings() -> Result<Vec<NativeCodecBinding>, AuthorityError>
```

It is an in-process compiled provider, not a route, plugin discovery mechanism, environment selector, or client contract. Its only first implementation should:

1. depend on the `semio-s-plugin-stdio` Rust crate with the minimal feature shape required to call `registry::native_codec_factory_receipts()`; resolve any root/export-symbol collision at Cargo feature design time, never by falling back to component loading or a raw Wasm `dlopen`;
2. obtain the receipts once during startup; reject any error before `configured_artifact_authority` is called;
3. require the constants `plugin_id == "stdio"` and returned package id to be exactly `semio:stdio`, then require unique `factory_id`, `descriptor_codec_id`, and `(artifact_kind, schema)`; reject a zero hash;
4. call `receipt.instantiate()` once per receipt, re-check the returned codec schema/hash against the receipt, and construct one `NativeCodecBinding::new(plugin_id, package_id, artifact_kind, codec)`; and
5. pass that complete fixed vector unchanged to `TrustedCatalogLoader::load`. The loader—not the provider—selects a profile, reads the bundle, matches descriptor rows, and decides which surface is openable.

`NativeCodecBinding` intentionally lacks a factory identifier ([`trusted-catalog/🦀️.rs:144-154`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:144)). Keep factory IDs inside the bootstrap provider for its self-validation and test diagnostics; do not add them to browser/MCP/public plan data merely for observability.

The first immutable trusted bundle profile must contain exactly one selected package, with all source-emitted stdio `nativeCodecs`, plus exactly one `openTarget` for the verified descriptor's JSON viewer surface. The row must carry:

- plugin `stdio`, package `semio:stdio`, exact descriptor version/version/dependencies;
- raw component relative path, byte length, SHA-256 and BLAKE3; descriptor relative path, byte length and SHA-256;
- all 26 exact `{ artifactKind, artifactSchema, packSchemaHash }` records, matching the emitted manifest; and
- one JSON viewer target whose artifact tuple equals the declared JSON codec and whose app/window/surface/renderer values exactly equal the emitted descriptor.

Do not bind only `stdio.json`: `validate_descriptor` requires manifest/artifact count equality, while the loader rejects a selected package with undeclared bindings and rejects bindings outside its declared closure ([`trusted-catalog/🦀️.rs:401-466,775-800`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:401)). Do not place generated `plugins.ts`, a developer cache path, an owner descriptor path, a module URL, an environment-provided plugin ID, or a catalog directory scan on this authority path.

## Atomic Publication And Readiness Invariants

1. **Build publication.** The build packet may produce the stdio raw/core/descriptor receipt in an isolated root; cancellation, timeout, oracle disagreement, or registry check failure must restore/remove its staged row and owner descriptor pair, as its current script already intends. Package the trusted bundle only from that completed receipt, not from a cache discovery.
2. **Hub startup.** First build the complete static provider vector in memory. Then have `TrustedCatalogLoader::load` verify the entire selected closure before taking `begin_artifact_assembly`; it preflights every codec before one shared registration action ([`trusted-catalog/🦀️.rs:467-476`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:467)). Any provider/bundle failure leaves no `ConfiguredArtifactAuthority`, no openable catalog, and no D0 readiness.
3. **Codec registry.** The store locks the registry and validates all prospective codecs before insertion; same schemas require exact function-pointer equality, so an owner is never replaced ([`🏪️store/🦀️.rs:9410-9524`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9410)). The registry is process-global/additive: do not add live profile reload, revocation, or partial re-publication. Bundle/profile changes require a process restart.
4. **Readiness.** Install the returned catalog and validating authority together in `HubState`; derive `artifact_authority_ready` only from a returned configuration and `open_plan_ready` only from `catalog.open_target_count() > 0`. Never set a readiness bit from the configured environment, a generated entry, a static provider count, or an unverified receipt.

The stronger relation “these native Rust function pointers were generated from these exact component bytes” is outside current evidence. The bundle authenticates its component/descriptor bytes and the provider authenticates its statically linked factory semantics; joining those two build domains needs a later signed/immutable build-attestation protocol. Do not overclaim that generated/cache evidence supplies it in V1.

## Required Hostile Laws

Run these against the production provider/loader boundary, not a copied map helper:

- **Closure:** current stdio provider has 26 receipts/bindings with exact unique factory, descriptor-codec and `(kind,schema)` keys; the selected JSON viewer target produces one usable catalog target and `open_plan_ready` only then.
- **Missing/extra/duplicate:** reject one missing binding, an extra unused binding, duplicate factory/codec key, a bundle native codec absent from the descriptor, duplicate target, or a selected descriptor kind count different from the trust row.
- **Factory integrity:** reject wrong package/plugin, unknown factory id, zero/non-canonical/wrong pack hash, wrong schema or extension from an instantiated factory, changed `component_package_id`, and a factory/result substitution that otherwise shares a schema. The final preflight must reject differing function pointers for an established schema.
- **Bundle integrity/bounds:** reject raw/core swap, byte-length/SHA-256/BLAKE3 disagreement, malformed/zero descriptor hash, escaped/symlink/duplicate path, dependency mismatch, unknown profile, missing one half of bundle/profile configuration, oversized package/codec/target closure, wrong JSON surface/role/renderer, and an open target without an exact declared codec.
- **No partial publication:** a synthetic conflict or any previous hostile condition must leave no new codec visible and must keep catalog/readiness unavailable. Isolate process-global registry tests or use the assembly seam so previous test registration cannot mask this assertion.
- **No generated bypass:** a generated registry/cache/owner-descriptor-only fixture with no trusted component + descriptor + bundle record must fail; a valid source receipt without the static linked factory must fail. Assert no plan route/readiness/catalog activation after either failure.

## Neutral Fixture And Oracle

Add a language-neutral `native-openable-catalog-provider-v1` fixture family under the existing trusted-catalog/oracle test ownership. It has a canonical minimal stdio package projection, a complete ordered factory-receipt projection, component/descriptor byte vectors and a single JSON viewer target. Its public values are strings/digests/roles only—no Rust function pointers or secrets.

An independent Bun Node/AJV/WebCrypto oracle must parse the schema strictly, enforce ordering/uniqueness, hash the supplied raw/core/descriptor bytes, and cross-check factory receipt fields against the bundle/target. It must emit the same allow/deny result for the positive case and every hostile vector above. Rust must separately apply the production static provider then trusted loader; the oracle may not import the Rust registry or reuse its parsing function. Include stale receipt, unknown factory, swapped raw/core, extra factory, duplicate target, malformed hash, and changed JSON viewer surface vectors.

## Implementation Partitions And Gates

| Order | Sol packet | Exact ownership and outcome |
| --- | --- | --- |
| 1 | Stdio receipt/build attestation input | Preserve `catalog-root`'s bounded fresh-root/atomic receipt behavior; define the immutable deployment handoff with raw/core/descriptor hashes. It must not claim a generated registry is enough. |
| 2 | Hub static provider | Add the Cargo link and private `NativeOpenableCatalogProviderV1`; project all verified stdio receipts into bindings; test all factory/closure rejects. This is the first D0-readiness packet. |
| 3 | Trusted profile fixture + oracle | Build a full 26-codec stdio row with one JSON viewer target; add cross-language exact positive/negative vectors and no-partial-publication law. |
| 4 | Hub readiness/launch | Wire the successful result to current startup/readiness, retain fail-closed absence, and add an actual isolated hub process journey that observes false→true only with the valid bundle. |
| 5 | Client execution authority | Separately address the existing native/WGPU/bare-document-id/browser/MCP execution gaps. This provider packet must not turn their source presence into an acceptance claim. |

Existing registrations to retain and strengthen are:

- `bun nx run @semio-tech/stdio-plugin:catalog-root -- --build-root <absolute empty directory>` (`📋️project.json:55-64`; launch `📦️catalog-root🗄️stdio` at [`.vscode/launch.json:6049-6070`](/Users/ueli/Documents/semio/.vscode/launch.json:6049));
- `bun nx run @semio-tech/plugin-registry:check-generated`, after descriptor publication, as consistency evidence only;
- `bun nx run os-hub:open-plan-check --skip-nx-cache` ([`🌎️hub/.../📋️project.json:111-117`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json:111); launch `.vscode/launch.json:4433-4441`), extended to list/require exactly one FQN for every provider law then exact-run it; and
- `bun nx run os-hub:open-plan-server-check --skip-nx-cache` only as its declared subset, never as a substitute for the all-feature provider/full journey.

Add a permanent `native-openable-catalog-provider-check` subcommand in the existing hub `📜️script.ts` and an Nx/launch registration next to `open-plan-check` if an independent provider fixture/oracle needs its own target. It must run (1) the neutral oracle, (2) one exact listed provider law for each named case, (3) the full hub feature check, and (4) an isolated real hub readiness journey. A Cargo no-run, a generated check, or an open-plan unit test that never supplies a bundle cannot accept D0 readiness.

## Remaining Full-Catalog Blockers

Even after the stdio provider packet passes, do not accept “all plugins/artifacts”:

- the runtime census found 59 generated rows (33 plugins/26 extensions) but only inventory/build metadata for many rows; 19 lack current descriptor/pack authority;
- other plugins do not yet expose a similarly verified static native factory-receipt provider, and package manifest artifact-kind projections remain incomplete/drifted;
- WGPU still routes document opening to a retired `attach_backbone` rejection; browser and native actor maps still have the broader D0 execution/scope issues; and MCP has no generic headless D0 open contract;
- browser dev assets and registry rows are not an executable/provider proof, and generated output/owner descriptor cache material remains untrusted until included in an exact fresh receipt/bundle path; and
- no command has been run for this report. Source review, linkage, and registration do not establish runtime behavior.

The bounded acceptance claim after this packet is therefore only: **a hub, given a freshly verified stdio deployment bundle/profile and the complete static stdio codec provider, admits one exact JSON viewer open target atomically and reports D0 catalog readiness.** It does not claim client rendering, mutation, collaborator visibility, all-catalog activation, or component-to-native binary provenance beyond the current separate checks.
