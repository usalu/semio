# Terra Multi-Provider Verified Catalog Blueprint

Date: 2026-09-04  
Scope: current-source, read-only implementation blueprint for a compile-time linked native catalog that can grow beyond the accepted `stdio` slice. No Cargo/Nx build, generator, launch, or product/test source was changed or run. All positive statements below are source evidence, never artifact freshness or runtime acceptance.

## Decisive boundary

**RED — the hub has one compile-time provider, `stdio`, and zero admissible non-stdio providers.** The repository has several useful codec/application *sources*, but none of those packages currently supplies the complete authority tuple required for a trusted native binding:

1. a statically linked factory receipt with a private factory pointer;
2. exact `plugin_id`, Cargo component `package_id`, artifact kind, schema, extension, nonzero pack-schema hash, descriptor-codec identity, and runtime capability identity;
3. an immutable raw-component/descriptor bundle record with component SHA-256 **and** BLAKE3 plus descriptor SHA-256;
4. a freshly verified component/core/descriptor receipt and marker; and
5. a descriptor-derived isolated WASM app/window/role surface for the open target.

The current hub `Cargo.toml` links only `semio-s-plugin-stdio` at `🌎️hub/📦️packages/🦀️rust/Cargo.toml:31-45`; `NativeOpenableCatalogProviderV1` imports only its `NativeCodecFactoryReceipt` and accepts exactly its 26-receipt closure at `🌎️hub/🗿️artifact-authority/🗂️📇️native-openable-provider/🦀️.rs:5,18-59`. Repository-wide source census found no non-stdio occurrence of either `NativeCodecFactoryReceipt` or `native_codec_factory_receipts`.

This is intentionally fail-closed. The trusted loader requires a binding for every selected `(plugin, package, artifact kind, schema)` at `🌎️hub/🗿️artifact-authority/🗂️🛡️trusted-catalog/🦀️.rs:376-401`, rejects an undeclared selected binding at `:402-407`, and rejects every binding outside the selected closure at `:464-468` before its one assembly preflight/registration transaction at `:474-477`. Do not make a generated marketplace row, a checked-in descriptor, or a dynamic Wasm scan substitute for a linked factory receipt.

## What the current trusted path already enforces

The existing loader is the correct shared admission boundary and should be extended, not bypassed.

- It canonicalizes and bounds the bundle/component/descriptor paths and bytes; verifies component length, SHA-256 and BLAKE3; verifies descriptor length/SHA-256; and decodes the packed descriptor before accepting it (`trusted-catalog/🦀️.rs:321-374`).
- It checks exact package identity, role, plugin/version, component hash, dependency and artifact-count closure (`:641-769,787-826`). A bundle package is not an arbitrary app manifest row.
- It derives a document-open selection only after its artifact tuple matches a declared native codec. `validate_descriptor_open_target` then requires the descriptor app ID, canonical surface app ID, role, dialect artifact kind, window kind, isolated execution, and `wasm` renderer (`:408-450,512-534`).
- It produces a deterministic generation from the full package hashes, artifact tuple, and surface tuple (`:537-610`), then startup exposes the catalog/readiness from the one returned configured authority (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:380-396,5315-5321,5347-5359`).

The loader cannot authenticate a factory merely because a source tree has an `ArtifactCodec::of` call. Its public `NativeCodecBinding` is intentionally only `(plugin_id, package_id, artifact_kind, ArtifactCodec)` (`trusted-catalog/🦀️.rs:142-156`), so factory/descriptor/runtime-capability provenance must be checked before projection into it.

## Current compile-time/provider census

### Admissible provider today

| Package | Native factory receipt | Component/descriptor producer | Codec/schema and app/surface source | Status |
| --- | --- | --- | --- | --- |
| `stdio` / `semio:stdio` | **Yes.** `registry::native_codec_factory_receipts()` constructs 26 receipts after source/runtime/descriptor/factory bijection checks at `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:836-866,1043-1111`. | **Source-present, not run here.** Its owner `catalog-root` creates an isolated raw/core/descriptor row and commit marker then runs the generic fresh verifier at `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:448-534`. | The linked provider rechecks 26 unique identities/results at `🌎️hub/🗿️artifact-authority/🗂️📇️native-openable-provider/🦀️.rs:29-59`; the current source-focused gate covers the JSON viewer target. | Source-qualified only; no fresh build or production launch was run by this audit. |

`stdio` is the sole current answer for all five authority inputs. Its factory receipt is deliberately package-owned and validates the instantiated codec result. That is the pattern to generalize, not a registry-row convention to copy.

### Direct codec/application candidates beyond `stdio`

The following seven plugin roots each contain a direct `ArtifactCodec::of` in a public `IoDeclaration`, an owner `🛂️.descriptor.semio` file, an owner `🔣️.json`, and editor/viewer-looking app IDs. Their Cargo component identities are exact source inputs. They are **candidates**, not providers: none exports a receipt, none is linked by the hub, and every checked-in JSON descriptor lacks the required top-level `packageId`, so the pair cannot presently be accepted as a strict catalog descriptor.

| Candidate component | Current codec/schema source | Current canonical declaration/app source | Receipt / immutable-input status | First packet placement |
| --- | --- | --- | --- | --- |
| `vcs` / `semio:vcs` | `VcsSnapshot`/`VcsDemoMutation`, `vcs.vcs`: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:18-55`; Cargo identity at `📦️packages/🦀️rust/Cargo.toml:1-30`. | `s.vcs.vcs` declaration/dialect at `🗿️artifacts/🌿️vcs/🦀️.rs:5-17,77-81`; descriptor apps at `🔣️.json:8-16,52-67,3916`. | No receipt. The static descriptor JSON has no `packageId`; its static pack has not been freshly verified. | **First non-stdio candidate after an identity repair.** It has only the `stdio` plugin crate as a direct plugin Cargo dependency (`Cargo.toml:25-33`). |
| `animate` / `semio:animate` | `PresentationSnapshot`/`PresentationMutation`, `animate.presentation`: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs:10-20`; codec at `…/🚪️io/🦀️.rs:46-59`. | `s.animate.present` apps in `🔣️.json:8-16,2642`; source declaration names `s.animate.presentation`. | No receipt; checked-in JSON has no `packageId`; no fresh row/bundle. | Second wave after all descriptor/declaration identities agree. |
| `forms` / `semio:forms` | `FormsSnapshot`/`FormMutation`, `forms.form`: `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:16-22`; codec at `…/🚪️io/🦀️.rs:46-54`. | Declaration `s.forms.forms` at `🗿️artifacts/📋️forms/🦀️.rs:498`; apps at `🔣️.json:8-16,6021`. | No receipt; JSON lacks `packageId`; no fresh row/bundle. | Second wave. |
| `note` / `semio:note` | `NoteSnapshot`/`NoteMutation`, `note.document`: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs:107-119`; codec at `…/🚪️io/🦀️.rs:565-569`. | Declaration/dialect says `s.note.note` at `🗿️artifacts/🗒️note/🦀️.rs:79-83,107-115`; apps at `🔣️.json:8-16,7100`. | No receipt; JSON lacks `packageId`; no fresh row/bundle. Additionally `artifact_kind()` still emits `2d.note` at `:86-103`. | Block until this dual kind identity is eliminated. |
| `writer` / `semio:writer` | `WriterSnapshot`/`WriterMutation`, `writer.document`: `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🦀️.rs:9-18`; codec at `…/🚪️io/🦀️.rs:14-20`. | Declaration `s.writer.writer` at `🗿️artifacts/✒️writer/🦀️.rs:18,309`; apps at `🔣️.json:8-16,2574`. | No receipt; JSON lacks `packageId`; no fresh row/bundle. Cargo also imports `trinity` as well as `stdio` (`Cargo.toml:25-33`). | Later dependency-closure wave. |
| `draw` / `semio:draw` | `DrawingSnapshot`/`DrawingMutation`, `drawing.document`: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🦀️.rs:5,476,534`; codec at `…/🚪️io/🦀️.rs:228-232`. | Static apps say `s.draw.draw` (`🔣️.json:8-16,3118`), while source declaration says `s.draw.drawing`. | No receipt; JSON lacks `packageId`; no fresh row/bundle. It also carries the draw-FSM plugin crate dependency (`Cargo.toml:25-31`). | Keep behind the active native draw/renderer repair. |
| `sourcing` / `semio:sourcing` | `CurationSnapshot`/`SourcingMutation`, `sourcing.curation/v1`: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs:11-21,300`; codec at `…/🚪️io/🦀️.rs:61-65`. | Declaration `s.sourcing.curation`; apps at `🔣️.json:289,8752`. | No receipt; JSON lacks `packageId`; no fresh row/bundle. The root has multiple extension component crates; the base crate is `semio-s-plugin-sourcing` / `semio:sourcing` at `📦️packages/🦀️rust/Cargo.toml:1-48`. | Later multi-component/extension wave. |

The static JSON files include `wasmSha256` fields, but that does not remedy the missing `packageId` or certify the sibling packed descriptor. The generic strict parser explicitly requires `descriptorVersion`, role, matching Cargo `packageId`, manifest plugin ID, and all descriptor identities (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️📇️registry/📜️script.ts:2375-2455`). A checked-in hash is never a substitution for a fresh raw/core/descriptor receipt.

### Other direct codec roots are not first-wave provider candidates

Source census found direct `ArtifactCodec::of` under `writer`, `mathematical`, `vcs`, `animate`, `sequence`, `fem`, `reasoning`, `forms`, `playbook`, `trinity`, `dag`, `draw`, `stdio`, `note`, `puzzle`, `block`, and `sourcing`. The public `IoDeclaration` subset is only `writer`, `vcs`, `animate`, `forms`, `draw`, `stdio`, `note`, and `sourcing`. The remaining direct-codec roots must not be swept into a "complete executable profile" until they publish the same declared Io/descriptor/receipt evidence; `playbook`, `trinity`, and `block` also have no root descriptor JSON in this census. This is a deliberately conservative classification, not a claim that they can never become providers.

## The first useful set beyond stdio

### Selection: `native-stdio-vcs-v1`, not an all-plugin claim

The smallest topologically useful **multi-provider** candidate is a two-package set:

```text
compile-time provider set
  stdio / semio:stdio       -> complete existing 26-codec provider
  vcs   / semio:vcs         -> one declared VCS native codec provider after its identity repair

verified profile
  native-stdio-vcs-v1       -> exact emitted descriptor dependency closure
```

`vcs` is chosen because its component crate has only the direct `stdio` plugin-crate dependency, has one visible native codec and both viewer/editor app declarations, rather than bringing the writer/trinity, draw/FSM, or sourcing extension fan-in into the first proof. This is a *topological* choice, not acceptance of present VCS bytes.

There is a real VCS precondition: its new declaration tree and dialect use `s.vcs.vcs`, but `artifact_kind()` returns `vcs.document`, and plugin activation takes that latter ID (`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️.rs:16,22-37,77-81`; `✏️s/🔌️plugins/🌿️vcs/🦀️.rs:28-37`). The loader strips `s.` from the descriptor dialect and therefore expects the bundle artifact kind `vcs.vcs` (`trusted-catalog/🦀️.rs:512-534`). A VCS receipt must not arbitrarily paper over this disagreement. First make the declaration, activation, runtime capability, codec receipt, descriptor artifact kind, and `DocumentDescriptor` vocabulary one schema-first identity; then emit the provider.

`note` demonstrates the same reason for strictness: its declaration/dialect is `s.note.note`, but its old `ArtifactKindSpec` still says `2d.note` (`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs:79-115`). Neither is a safe "first" package merely because an editor is present in `🔣️.json`.

### Important profile-linearization defect to solve before aggregation

The current singleton provider returns all linked `stdio` bindings before the bundle/profile is read (`📦️bin.rs:394-396`). With multiple providers, an eager global `Vec<NativeCodecBinding>` would make every smaller profile fail: the loader rejects each linked binding that is not inside the selected closure (`trusted-catalog/🦀️.rs:464-468`). Thus simply replacing the singleton with a vector that eagerly returns all installed package codecs would make `native-stdio-v1` break as soon as VCS is linked.

The correct generalized boundary is a private compile-time provider registry, not ambient discovery:

```text
NativeCodecProviderSetV1 (fixed compile-time table)
  exact (plugin_id, package_id) -> package-owned provider function

TrustedCatalogLoader::load(..., provider_set, ...)
  1. bound/canonicalize/verify the selected profile and package descriptor bytes
  2. request bindings only for each selected, descriptor-verified package identity
  3. require the returned receipt closure to equal that package's declared codec closure
  4. retain all candidate bindings privately until every selected package succeeds
  5. preflight and register one complete selected closure
```

The table contains only compiled-in function references. It takes no plugin ID, component path, factory address, or selection from a browser/MCP/client input. The loader remains the authority for the profile, file bytes, descriptors, limits, and assembly. A package absent from the fixed table, an unexpected provider result, duplicate `(plugin, package, kind, schema)`, a zero/hash/schema mismatch, or a provider for an unselected package fails before publication.

This requires moving the receipt *contract* out of `semio-s-plugin-stdio::registry` into a dependency direction all plugin crates may use—for example a narrow framework plugin/catalog-contract module. The Rust function pointer remains private and non-serializable. `stdio` implements the new contract by adapting its already-verified source closure; each later plugin owns an independent function and factory list. The hub must not import a generic plugin loader or dynamically call a Wasm component to obtain a native codec.

## Precision amendment — VCS `packageId`, identity, and closure selection

### `packageId` has one legitimate guest-owned source

`PackageDescriptor.package_id` is a required schema field and serializes as JSON
`packageId` (`🧰️framework/🛍️products/💻️os/🔨️modules/🛂️manifest/🦀️.rs:4897-4929`).
The guest descriptor path is deliberately its owner: `describe_plugin()` copies
`plugin_runtime::plugin_descriptor_extras().package_id` into that descriptor
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs:137-156`).
The only valid origin of those extras is `PluginBuilder::package_id()`, which
records the value only after `try_build()` requires the exact canonical
`semio:<plugin-id>` spelling (`🏗️builder/🦀️.rs:212-217,622-627,720-725`).

The host-side describe executable may patch byte hashes, but must not invent
identity: it invokes the guest `describe()`, decodes its `PackageDescriptor`,
then patches the raw/core hashes and atomically writes the paired pack/JSON
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/🦀️.rs:421-462,325-384`).
The generic registry parser then rejects a descriptor whose emitted `packageId`
differs from Cargo component metadata (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️📇️registry/📜️script.ts:2375-2455`).

**Current VCS RED:** `Plugin::<VcsApps>::builder("vcs")` sets label and
version but omits `.package_id("semio:vcs")`
(`✏️s/🔌️plugins/🌿️vcs/🦀️.rs:28-38`). Its fresh guest assembly therefore
fails before it can emit a strict descriptor; the checked-in VCS JSON's absent
`packageId` is a stale derivative symptom, not an authority source. The VCS
Cargo component metadata is already `semio:vcs`
(`✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/Cargo.toml:1-30`).

### `vcs.document` is a single-source activation mismatch

The VCS declaration, dialect and declared artifact use `s.vcs.vcs`, whose
trusted-loader open-target projection is `vcs.vcs`: the loader strips only the
leading `s.` from a descriptor dialect before comparing the bundle kind
(`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️.rs:5-17,40-63,77-81`; `🌎️hub/🗿️artifact-authority/🗂️🛡️trusted-catalog/🦀️.rs:512-534`).
But the same artifact factory returns `ArtifactKindSpec { id: "vcs.document" }`
(`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️.rs:20-37`). Both the root
activation and editor consume that function, so they inherit the old identity
rather than independently disagreeing (`✏️s/🔌️plugins/🌿️vcs/🦀️.rs:28-38`; `✏️s/🔌️plugins/🌿️vcs/✏️editor/🦀️.rs:915-930`).

The smallest breaking, schema-first repair is exactly:

1. Add `.package_id("semio:vcs")` to that VCS builder.
2. Change the one `ArtifactKindSpec.id` factory value from `vcs.document` to
   `vcs.vcs`; retain no alias, dual activation, legacy route, or fallback.
3. Update only the two source-owned stale expectations: the VCS envelope
   fixture (`…/🧬️schema/⚙️operations/🦀️.rs:66`) and descriptor-manifest
   assertion (`…/✏️editor/🦀️.rs:1405-1417`). Regenerate, rather than hand
   edit, the derived `🔣️.json` entries that still contain `vcs.document`
   (notably `:3875,4922`).
4. Have the existing guest describe + host atomic emitter produce a new pair;
   only then add a VCS receipt/provider and an immutable bundle row.

The language-neutral `native-stdio-vcs-v1` corpus must include positive
emitted `packageId: "semio:vcs"` and `artifactKind: "vcs.vcs"` vectors, plus
missing package ID, `semio:other`, `vcs.document`, mismatched descriptor
dialect, descriptor/Cargo package mismatch, and stale JSON-without-package-ID
negatives. A Rust VCS owner law must prove the emitted guest descriptor carries
the package ID and the factory/declaration/activation all expose exactly the
one kind. The independent Bun/AJV/WebCrypto oracle checks canonical data and
hashes only; it must not import the Rust builder or receipt function.

### No reusable selected-provider resolver exists today

There is no current provider-selection trait or reusable selected-closure
resolver. `TrustedCatalogLoader::load()` accepts one eager slice of
`NativeCodecBinding`, calls private `validate_bundle()` for a bare dependency
order, and validates the whole binding map before iterating a selected package
(`🌎️hub/🗿️artifact-authority/🗂️🛡️trusted-catalog/🦀️.rs:321-342,641-769`).
`VerifiedTrustedCatalog::resolve_document_open()` and
`TrustedArtifactCatalog::resolve()` are post-admission document lookups, not
factory-provider selection (`:261-316`). `NativeOpenableCatalogProviderV1`
is a singleton source of all stdio bindings, not an abstraction that receives
the selected closure (`🌎️hub/🗿️artifact-authority/🗂️📇️native-openable-provider/🦀️.rs:18-59`).

Keep the new resolver private to artifact authority. Refactor
`validate_bundle()` to return a private `SelectedTrustedBundleV1` containing
the validated dependency-first package indexes/identities rather than exposing
another public profile API. A fixed compile-time
`NativeCodecProviderSetV1` maps exact `(plugin_id, package_id)` to private
package-owned provider functions. After each selected component/descriptor has
passed loader verification, the loader requests that exact provider and checks
its result is a complete, duplicate-free equality with that verified record's
declared codec closure. It retains all resulting registrations until every
selected provider and package succeeds, then preserves the existing one
preflight/register transaction (`trusted-catalog/🦀️.rs:464-477`).

Required resolver laws are: only selected provider functions run; a VCS-linked
binary still accepts `native-stdio-v1`; `native-stdio-vcs-v1` receives exactly
the two closures; missing/foreign/extra/duplicate provider output and a
second-package failure leave no binding, catalog generation, readiness, or
registration published. That is the reusable *design seam*; no such current
abstraction can be reused unchanged.

## Required implementation packets

### P0 — neutral receipt contract and selected-provider resolver

**Owners:** framework plugin contract + hub artifact authority + stdio.

1. Define a versioned private `NativeCodecFactoryReceiptV1` contract in a framework-level crate, not in a plugin crate and not on a wire. It contains the exact plugin/package/factory/descriptor-codec/runtime-capability/artifact-kind/schema/extension/nonzero pack hash and a private native factory function.
2. Preserve `stdio`'s existing 26-way runtime/descriptor/factory bijection. Its adapter must produce the new receipt type without weakening the source-owned checks at `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:1043-1111`.
3. Replace the start-of-main eager `linked_native_codec_bindings()` vector with the fixed provider set described above. It must resolve only selected package identities after the trusted loader has bound the selected profile, while withholding every binding until the full closure has passed.
4. Keep restart-only behavior. The process-global codec registry is additive/immutable and checks existing schema ownership by exact function-pointer equality (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9403-9438`); live profile reload, removal, or partial generation replacement is outside this packet.

**Required production laws:** exact selection keeps `native-stdio-v1` valid after VCS is linked; exact `stdio+vcs` selection returns precisely both package closures; a profile requesting VCS without its provider, a provider result for an unselected package, duplicate/foreign package result, zero pack hash, factory output substitution, or one failed selected provider publishes no codec/catalog/readiness.

### P1 — generic immutable component-row producer and activation bundle

**Owners:** plugin registry build script + descriptor emitter; no hub route/client work.

The existing generic registry verifier already defines the needed receipt layout and rejects shared target/development-cache roots: raw/core/packed descriptor in an exact row with a last commit marker (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️📇️registry/📜️script.ts:2601-2704`). But `catalog-complete` only verifies an externally produced root and explicitly audits sources with `ownerDescriptors: "ignored"` (`:2708-2745`); generic candidate `describe` commands instead build a component, write owner descriptors, and delete their temporary core (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts:89-109`). They do not produce a catalog row or hub bundle.

Factor the trusted parts of `stdio`'s `catalog-root` producer (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:433-534`) into one registry-owned explicit-package producer:

- input is an exact static package list/profile name, an absolute empty caller-owned root, a cancellation file/deadline, and no cache/ambient target authority;
- for each requested component: isolated Cargo target, raw component, independently extracted core, packed+JSON descriptor pair, bounded byte/hash receipt, WIT/descriptor oracle where applicable, and a commit marker published last;
- reject the whole generation if any row is missing, added, changed, symlinked, oversized, noncanonical, stale, inconsistent with Cargo component identity, or lacks the top-level `packageId`;
- run `catalog-complete` against the finished root and then construct `CatalogActivationBundleV1` from those **verified retained bytes**, calculating the component BLAKE3 using one first-party/controlled build implementation in addition to the generic SHA-256 receipts; and
- atomically publish the final bundle/profile only after the entire requested root passes. Do not put mutable owner descriptors, `target/`, generated marketplace rows, or an individual row path directly in `OS_HUB_TRUSTED_CATALOG_BUNDLE`.

The `CatalogActivationBundleV1` source model must contain the current loader's exact `Bundle` fields (`trusted-catalog/🦀️.rs:52-140`): package/plugin/version/role, descriptor-derived package dependencies, component `{relative path,length,sha256,blake3}`, descriptor `{relative path,length,sha256}`, complete native codec list, and descriptor-derived open targets. Its profile roots select an exact dependency closure. Keep raw/core receipts in the build generation even though current hub bundle reads the component plus descriptor; core remains material evidence for the descriptor's own core identity and must not be silently discarded.

### P2 — VCS identity repair, first non-stdio receipt, and two-provider proof

**Owners:** VCS plugin + hub provider-set integration.

1. Delete the VCS dual artifact identity before exposing any receipt. Derive activation, declaration, codec/native receipt, emitted descriptor dialect/app and bundle artifact kind from one schema-owned `vcs.vcs` identity. Do not retain `vcs.document` as a legacy second route.
2. Add `vcs`'s package-owned factory receipt(s), including a real `ArtifactCodec::of::<VcsSnapshot, VcsDemoMutation>("vcs.vcs")` result, its extension and nonzero pack schema hash, a stable private factory ID, descriptor codec/capability identity, and a complete local bijection law. The plugin must reject an emitted descriptor/app/surface/codec mismatch itself before the hub sees it.
3. Link `semio-s-plugin-vcs` explicitly in hub Cargo alongside stdio only after P0. Add it to the fixed table; no conditionally loaded/native ABI plugin discovery.
4. Produce `native-stdio-vcs-v1` only from P1's two verified rows. The exact dependency list is taken from the newly emitted VCS descriptor, never copied from `Cargo.toml`. Select an editor or viewer target only if its decoded descriptor passes existing `validate_descriptor_open_target`; no generated app row may fill the tuple.
5. Run a real hub process with the produced pair and prove only the two admitted providers. It must be not-ready/no-plan before full verification and ready only after the whole selected closure publishes. It does not claim browser/WGPU/MCP rendering or every installed plugin.

### P3 — progressively complete the remaining eligible roots

After the two-provider proof, add packages in independent identity-first lanes: `animate`, `forms`, `note` after its `2d.note` conflict is removed, `writer` after its descriptor dependency closure includes the required `trinity` package(s), `draw` after the active draw/FSM and renderer work, and `sourcing` only with its extension-component closure. Each lane supplies its own receipts, fresh row, descriptor identity/surface proof and independent oracle. A profile never admits an incomplete package merely to claim coverage.

The no-Io/direct-codec roots remain outside P3 until they gain the same explicit native-openable contract. The final “all executable catalog” profile can be introduced only when the fixed provider table and P1 receipts cover every descriptor-declared executable artifact. It must reject any catalog row which cannot do that rather than calling it executable.

## Neutral oracle and hostile corpus

Add one language-neutral fixture family, e.g. `native-openable-catalog-provider-set/v1`, beside the existing provider fixture at `🌎️hub/🗿️artifact-authority/🗂️📇️native-openable-provider/🧪️fixtures/🧬️v1/🔣️.json`. It contains only canonical strings, bounded byte vectors, hashes, package/dependency/profile rows, factory identities and surface tuples—never Rust pointers, raw secrets, environment values or host paths.

An independent Bun/Node/AJV/WebCrypto oracle must not import a Rust provider or loader. It validates strict JSON schema, canonical ordering, byte bounds, SHA-256, closure topology, selection filtering, codec/surface joins and zero publication for at least:

- `stdio`-only and `stdio+vcs` positive projections;
- a selected package with no compiled provider; a provider for an unselected package; duplicate factory/descriptor-codec/(kind,schema) identity; wrong plugin/package; factory result mismatch; zero or substituted pack-schema hash;
- missing/extra package, cycle, duplicate root, missing dependency, descriptor package ID/role/plugin/version mismatch, stale static JSON with no `packageId`, escaped/symlink/reused path, raw/core/component/descriptor length or digest mismatch, BLAKE3 mismatch, malformed/oversize row and cancellation at each package boundary;
- app ID/surface ID/role/window/dialect/renderer mismatch, target without exact codec, duplicate target, non-isolated descriptor; and
- one selected-provider failure or registry conflict proving no binding, catalog generation, readiness or route plan becomes visible.

Rust production laws must exact-list and exact-run the package-owned provider laws, resolver/loader laws and a process-backed hub launch. The oracle proves data semantics only; it cannot certify a native function pointer or a real process.

## Registered build, bundle, profile, and launch wiring

Current wiring is insufficient: `@semio-tech/plugin-registry:catalog-complete` is registered at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️📇️registry/📋️project.json:27-33`, but it accepts only a prebuilt fresh root; `os-hub:native-openable-catalog-provider-check` is registered at `🌎️hub/📦️packages/🦀️rust/📋️project.json:111-117` and `.vscode/launch.json:4452-4459`, but its own script explicitly makes no all-plugin/client-mount claim (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2882-2911`). The current hub dev launcher builds, then inherits the parent environment through `startLocalHub` (`📜️script.ts:605-625,3121-3145`); `.vscode/launch.json:4529-4537` sets only port/data. There is no registered immutable bundle/profile pair. Secure smoke expressly clears the pair (`📜️script.ts:615-620`).

Add only script-owned registered targets, in this order:

1. **`@semio-tech/plugin-registry:catalog-activation-bundle`** — calls that directory's existing `📜️script.ts`; accepts an explicit profile and fresh absolute root, creates/validates all selected rows, and outputs one bounded immutable bundle path plus profile ID. It must fail on PostgreSQL/renderer/client unrelated issues only if they are a required selected component build; otherwise isolate the package profile truthfully.
2. **`os-hub:native-openable-catalog-set-check`** — calls the existing hub `📜️script.ts`; runs the neutral oracle, exact-one per-package receipt/identity laws, selected-profile resolver/loader laws, provider-set all-feature check, then one process-backed `stdio+vcs` launch. It must print each selected fully-qualified law and fail on zero/multiple selection. Retain `native-openable-catalog-provider-check` as the bounded historical stdio gate rather than rewriting its claim.
3. **`os-hub:dev catalog <profile>`** (or an equally explicit subcommand of its existing `DevScript`) — obtains P1's output itself, passes precisely its final bundle/profile into its direct hub child, and owns cancellation/cleanup. It must not inherit an arbitrary `OS_HUB_TRUSTED_CATALOG_*` pair. Secure launch variants choose an explicit catalog profile or intentionally clear it and show `openPlan=false`; neither becomes a full-catalog claim accidentally.
4. Extend the **registry launch generator**, not `.vscode/launch.json` by hand. That output is written by `generateLaunchJson` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️📇️registry/📜️script.ts:1887-1894` and freshness-checks it at `:2749-2760`. Register a catalog-set gate after the existing provider gate (currently order `411.1085`) and explicit developer profiles after the hub dev entry; then require `@semio-tech/plugin-registry:check-generated`.

Expected gate sequence once implemented (not run in this audit):

```text
bun nx run @semio-tech/plugin-registry:catalog-activation-bundle --skip-nx-cache -- --profile native-stdio-vcs-v1 --build-root <absolute-empty-root>
bun nx run @semio-tech/plugin-registry:catalog-complete --skip-nx-cache -- --build-root <same-verified-generation-root>
bun nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache
bun nx run os-hub:native-openable-catalog-set-check --skip-nx-cache
```

The actual interactive command must be the generated launch entry, which calls the existing hub `📜️script.ts` subcommand and supplies the generated receipt—not a user shell export. Native/browser/MCP rendering, generic command ABI, app attachment, public catalog discovery and dynamic profile reload remain intentionally outside this provider-set packet.

## Acceptance and nonclaims

Accept the first multi-provider packet only when the registered uncached gate and process proof show `native-stdio-v1` remains valid, `native-stdio-vcs-v1` admits exactly both verified closures, every negative leaves no partial registry/catalog/readiness, and the launch owner supplies a real immutable bundle/profile pair. The generic all-executable profile remains RED until every admitted package has its own receipt, current descriptor pair, verified build row and descriptor-owned open surface.

This report does **not** claim that checked-in hashes are fresh, that static Wasm descriptor packs decode successfully, that VCS compiles, that any non-stdio provider renders in WGPU/browser/MCP, or that the catalog represents every installed plugin/artifact.
