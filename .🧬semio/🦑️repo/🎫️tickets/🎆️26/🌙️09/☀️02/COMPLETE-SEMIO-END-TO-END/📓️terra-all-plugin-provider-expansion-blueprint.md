# All-Plugin Linked Provider Expansion Blueprint

Current-source, read-only blueprint — 2026-09-04. No build or runtime command
was run for this audit. “Available” below means a package exposes a real,
linked native-factory receipt today; it does **not** mean that the package can
currently be installed, opened by a client, rendered, or edited in a real hub.

## Verdict

**RED: the repository has 59 discovered component identities, but only two
package-owned native factory-receipt producers.** The hub hard-codes those two
producers in `NativeCodecProviderSetV1::linked`:

* `semio:stdio` exposes a checked factory/receipt bijection for 26 codecs;
* `semio:gis` exposes two receipts, Map and Terrain.

That is not a general provider registry. It is a hand-maintained hub table
which cannot scale to the 33 plugin / 26 extension inventory without drift.
The existing all-plugin matrix remains the baseline: 92 artifact directories,
zero selected trusted bundles, and zero ordinary-startup runtime proofs.

The first expansion must therefore generate **a closed availability index and
a compiled linked-provider table from package-owned declarations**. It must
not infer a factory from an `ArtifactCodec::of` call, a descriptor, an app
factory, or an artifact directory.

## Current authority and exact gaps

| Current authority | Evidence | Limitation to remove |
| --- | --- | --- |
| Component census and dependency topology | [`registry/📜️script.ts`](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts#L2443) discovers Cargo component metadata and computes dependency-first order at lines 2501–2519. | It is a source census, not a linked factory declaration or trusted profile. |
| Current provider linkage | [`native-openable-provider/🦀️.rs`](../../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs#L24) manually names `stdio` and `gis`; the stdio preview does not receive/version-check the requested package version at lines 48–50. | A package added to the repository has no automatic declared status, and a version substitution could be missed by the stdio branch. |
| Package-owned receipt evidence | [`stdio registry`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs#L849) validates its factory/manifest/descriptor bijection; [`GIS receipts`](../../../../../../../✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs#L27) return exactly Map and Terrain. | There is no equivalent Flow receipt module or receipt producer for any other plugin package. |
| Trusted loader | [`trusted-catalog/🦀️.rs`](../../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs#L342) checks selected component SHA-256/BLAKE3, descriptor SHA-256, descriptor/package equality, and factory output before one assembly registration. | Bundle codecs carry only kind/schema/pack hash (lines 63–67), bindings carry only plugin/package/kind (148–161), and generation hashes only open targets (497). There is no factory ID, visibility, host-platform availability, or zero-target-package generation input. |
| Current descriptor closure rule | `validate_descriptor` requires descriptor artifact-kind count to equal `native_codecs` count ([lines 819–858](../../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs#L819)). | It cannot distinguish a public root from a private child codec. |
| Fresh build isolation | [`createFreshCatalogBuildVerifier`](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts#L2619) rejects the shared `target` and dev cache and verifies exact marker/triplet bytes. | Existing `executeCatalogVerificationPlan` is deliberately sequential ([2237–2274](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts#L2237)); it has no provider declaration/index, bounded dependency scheduler, or atomic profile publisher. |

The registry currently limits descriptors to 64 KiB ([2037–2050](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts#L2037)), while the hub trusted-loader policy is 4 MiB. This expansion must use one **4 MiB descriptor limit** everywhere (registry source audit, producer, neutral fixture, and hub) rather than preserve a per-package exception.

### Explicit availability census

| Index state | Current components | Required generated behavior |
| --- | --- | --- |
| `available-source-only` | `stdio` (26 concrete receipt rows) and `gis` (Map + Terrain receipt rows) | Index them, but do not call them installed/openable until a fresh server-owned trusted profile verifies their exact closure. GIS Terrain must be marked `child-only`; it cannot become a public document target merely because a codec exists. |
| `partial-source-only` | `stdio` source roots `ifc`, `epw`, `wav`, `bmp`, `semio`, `html`, `txt`, `binary`, `gif`, `tsv` are not in the static native factory closure. | Emit one explicit unavailable codec/root row for each; never substitute a neighboring stdio codec. |
| `unavailable-no-receipt` | `animate`, `architect`, `block`, `cad`, `dag`, `demonstrator`, `draw`, `energy`, `fem`, `flow`, `forms`, `imperative`, `layout`, `lowpoly`, `mathematical`, `norm`, `note`, `playbook`, `procedural`, `process`, `puzzle`, `raster`, `reasoning-mindmap`, `remodel`, `s`, `sequence`, `shooting`, `sourcing`, `trinity`, `vcs`, `writer`. | Emit a package-level unavailable row with a bounded reason code. Several contain local `ArtifactCodec::of` calls, but none currently emits a package-owned native factory receipt; that is not a linked provider. |
| `extension-host-only` | All 26 extension components. | Emit no standalone codec or open target. They may later appear in a host package’s verified dependency/contribution closure only after their descriptor/role integrity is repaired. |

`flow` is the next useful unavailable package, not a special exception. Its
plugin factory has real editor/viewer surfaces and exact `semio:flow` identity
([`🌊️flow/🦀️.rs:24–37`](../../../../../../../✏️s/🔌️plugins/🌊️flow/🦀️.rs#L24)), but its document is composed over the stdio `s.stdio.semio@v1/flow` child
([artifact source lines 40–47](../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs#L40)) and it has no native receipt module. Its `document_codec` declaration
([lines 390–397](../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs#L390)) is insufficient as a hub factory binding.

## Schema-first provider contract

Add one canonical `ProviderIndexV1` schema and neutral fixture beneath the
registry source tree. It is generated from **package-owned** declaration input,
then read by the producer and the generated Rust linker. The index is a
diagnostic/completeness authority; it is not executable client input.

```text
ProviderIndexV1 {
  schemaVersion: 1,
  platform: { os, arch, componentTarget },
  entries: [
    {
      pluginId, packageId, version, role, dependencies[],
      availability: Available {
        factoryId, factoryAbiSha256,
        codecs: [{ kind, schema, packSchemaSha256,
                   visibility: publicOpenable | childOnly | headless }],
        supportedHosts: [{ os, arch, componentSha256, componentBlake3 }]
      }
      | Unavailable { reason: noFactory | unsupportedPlatform |
                       hostOnlyExtension | incompleteClosure }
    }
  ]
}
```

Rules:

1. Cargo package/component identity, version, role and dependency edges come
   from the existing source census, not from a descriptor or a directory name.
   Exactly one index entry must exist for every discovered component identity;
   no entry may be duplicated, omitted, or point at an absent dependency.
2. An `Available` entry is authored in its package next to the factory source.
   It supplies exact `factoryId`, public/child visibility, schema and pack
   hashes, and supported native host triples. The package test constructs its
   receipt and proves a bijection with that declaration. `ArtifactCodec::of`
   without this declaration stays `Unavailable`.
3. An extension must be `hostOnlyExtension`, have zero codecs and zero public
   targets. A plugin may have zero public targets only in the availability
   index; a selected executable trusted profile requires at least one public
   target overall.
4. A `childOnly` codec is permitted in a selected closure and contributes to
   generation/digests, but can never be exposed through `openTargets`. A public
   target must name exactly one `publicOpenable` codec plus an exact surface,
   role, and renderer target.
5. The generator must reject a declaration whose package/version/dependencies
   diverge from Cargo or whose factory output has a missing, extra, duplicate,
   zero-hash, wrong-kind, wrong-schema, or wrong-visibility codec. This closes
   both ordinary provider drift and Terrain-as-public-target drift.

### Generated, compiled linkage—not handwritten hub branches

Extend the existing registry `📜️script.ts` with a `provider-index` render/check
subcommand. It renders two checked-in generated outputs from the same canonical
index:

1. an index JSON/pack used by the isolated trusted-profile producer; and
2. a private hub Rust `🤖️linked-provider-v1.rs` which imports **only** packages
   declared `Available` for the current host and produces
   `LinkedProviderDeclarationV1 { identity, factory_id, preview }` rows.

The generated Rust file is an implementation artifact, not a second authority:
the registry check byte-compares it to current declarations; the hub compiles
it; and each package's receipt law proves its row. Replace the manual `stdio` /
`gis` match in `NativeCodecProviderSetV1::linked` with this generated function.
An unavailable package has an index row but no Rust import and no factory
callback. That makes a missing Flow receipt a deterministic `Unavailable`, not
a link error or accidental generic codec.

Extend `BundleCodec` and `NativeCodecBinding` with exact `factoryId` and
`visibility`. Add selected `platform` / component target to the bundle package
and bind both in `TrustedCatalogLoader::preflight_selection` before a component
or descriptor read. Replace the descriptor/native-codec cardinality equality
with three exact set checks:

* descriptor artifact declarations equal the declared artifact set;
* all receipt codecs equal the factory declaration closure, including children;
* open targets are a duplicate-free subset of `publicOpenable` codecs.

The component SHA-256/BLAKE3 and descriptor SHA-256 checks already present in
the loader remain mandatory; `factoryId`, platform and visibility add authority
that those byte hashes cannot express.

## Deterministic profile and generation

Keep two separate artifacts:

* **Availability index:** all 59 components, including explicit unavailable
  rows. It is never an executable profile and cannot give a client a target.
* **Trusted profile:** only the dependency closure of requested available
  plugin roots for one server host platform. No unavailable row, missing
  dependency, extension-only root, or unsupported host triple can enter it.

Canonicalize selected package rows by dependency-first topological order, with
the existing plugin-before-extension/id lexical tie break. Canonically order
every nested dependency, codec and target list by their full identity. Its
`catalogGenerationV2` must hash the domain/version, profile id, host platform,
and **every selected package row**: plugin/package/version/role, dependencies,
component SHA-256+BLAKE3, descriptor SHA-256, factory ID/ABI hash, every codec
(including `childOnly`), and every target (including zero-target package rows).
The current open-target-only generation at trusted-catalog line 497 is not
sufficient: changing a selected private child codec or a zero-target dependency
must invalidate client plans.

Per-platform availability is a fact produced by a fresh build for that host,
not a client preference. A macOS profile cannot select a factory that was only
compiled/receipt-checked on Linux. Browser WASM bytes may be shared only when
their exact component SHA-256/BLAKE3 is equal; native factory linkage remains a
host-specific receipt.

## Isolated production and atomic publication

Build this in the registry `📜️script.ts`, invoked through its existing Nx
project entry rather than ad-hoc scripts.

1. Create one producer-owned temporary root outside the repo target/dev cache;
   give every `(package, host triple)` its own target directory. Reuse the
   current fresh-root containment and commit-marker rules, never a checked-in
   descriptor or marketplace/cache row.
2. Start ready dependency nodes with a bounded worker pool of four. Completion
   order does not affect canonical profile order. Emit
   `ProviderBuildProgressV1 { profileId, platform, phase, completed, total,
   packageId, status }` for discovery/build/hash/receipt/closure/publish.
3. Before scheduling and between bounded hash/read chunks, check cancellation.
   On cancellation/failure stop admitting jobs, wait for running children to
   exit, mark dependents `blocked`, remove only this staging root, and leave the
   published profile/pointer untouched. Do not publish a partially successful
   profile.
4. Verify all selected rows, re-read each producer marker and factory receipt,
   construct the canonical profile/index bytes, write them plus a final manifest
   last to an isolated staging directory, fsync, then atomically rename it to a
   new immutable generation. Switch the server-owned `current` pointer only
   after hub preflight succeeds. Rotation is restart-only; a failed replacement
   preserves the prior generation and a restart reads either complete old or
   complete new state.

This changes the present sequential verification executor only for profile
production. The existing [`executeCatalogVerificationPlan`](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts#L2237) remains useful as the deterministic, serial source verifier; do not add
parallel writes to its publication callback.

## Dependency-ordered implementation packets

### P0 — Generated index with current two producers

Implement the schema, generator/checker, and generated hub linkage. The
generated output must reproduce exactly the current stdio and GIS receipt
closures, label GIS Terrain `childOnly`, and label all other 57 component rows
unavailable. Repair stdio provider preview to compare the requested package
version before returning receipts. Do not claim an installed profile yet.

### P1 — Server-owned `stdio + gis` trusted profile

Use the first-producer blueprint's isolated root and exact two-package closure.
The selected root can be GIS Map; stdio is a dependency and Terrain remains
private. Publish only after loader preflight/codec registration succeeds. This
is the first possible actual hub plan, but it still does not prove a browser
lease, a WGPU renderer, Map editing, or any other plugin.

### P2 — Flow as the first expansion package

Only after the public member-open work and P1 are accepted, add a package-owned
Flow receipt declaration. It must prove the public Flow parent codec, the
required `s.stdio.semio@v1/flow` child codec dependency, exact package/version
and full factory closure. It must resolve the current dual vocabulary from
source (`s.flow.flow` / `computation.flow`) against one descriptor identity;
the generator must reject either mismatch. Flow's visible app factories are
not an open-target proof until the trusted profile names their exact surface
and the client execution-target lease exists.

### P3 — Codec-producing plugin packages, one closure at a time

Promote only packages with a real typed factory receipt and native law, in
their generated Cargo dependency order. Local codec construction candidates in
draw, forms, mathematical, note, reasoning, sequence, sourcing, VCS, writer,
etc. enter here only after receipt/closure work; they must remain unavailable
before then. Each packet adds package-owned declaration, factory law, index
row, and one selected-profile fixture—never a hub `match` arm.

### P4 — Extensions and remaining platforms

Repair extension descriptor/host identity first. Add host contributions to a
selected host package's closure without assigning an extension a document
codec/open target. Repeat native receipts and profile production for each host
triple; only then flip that platform's availability state. An unsupported
platform remains explicit unavailable.

## Current-byte revalidation: first post-Flow wave

The manual linked provider is still precisely two entries: `stdio` and `gis`
([`native-openable-provider/🦀️.rs:24–33`](../../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs#L24)). GIS now verifies its requested package version and its two receipt identities before returning bindings
([lines 52–73](../../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs#L52)); the stdio callback still ignores the supplied `version` entirely
([lines 48–50](../../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs#L48)). Therefore version equality remains a P0 generated-link requirement, not a completed shared invariant.

**After the Flow parent/child closure is genuinely available, `semio:draw` is
the smallest next provider wave.** This is an ordering decision, not an
availability claim:

* Draw has exactly one component package, `semio:draw`, and its only
  cross-plugin Cargo dependency is `semio-s-plugin-stdio`
  ([`draw Cargo.toml:11–27`](../../../../../../../✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml#L11)). Its local FSM crate is an internal implementation dependency, not a
  separately catalogued plugin component.
* It declares one canonical parent artifact (`s.draw.drawing`)
  ([`drawing/🦀️.rs:476`](../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🦀️.rs#L476)) with a concrete typed `ArtifactCodec::of<
  DrawingSnapshot, DrawingMutation>` at
  [`io/🦀️.rs:218–233`](../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs#L218).
* Its production artifact source has no `ArtifactChild` field/owner; the only
  matches are a generator comment and an oracle rationale. Thus the first
  Draw receipt can prove one direct parent codec and its stdio import/export
  dependencies without expanding the already-blocked child-member publication
  protocol.
* Draw has **no** package receipt API today. The `ArtifactCodec::of` call,
  visible editor/viewer factory, import/export entries, and playground ports
  are source capabilities only; none may enter a linked provider until a
  package-owned receipt closes them.

The smallest Draw packet is one `draw/native-codecs` declaration/receipt with
one `publicOpenable` `s.draw.drawing` row, exact package/version/factory id,
pack-schema SHA-256, actual factory instantiation, and an explicit dependency
closure on only the selected stdio codec rows required by the declared open
path. It must **not** include every SVG/PDF/PNG/DWG/DXF conversion endpoint as
an open target merely because `IoDeclaration.entries()` lists them. Those are
format transformers; provider visibility concerns document codecs.

The first neutral fixture must reject package/version/factory/kind/schema/pack
hash substitution; a missing selected stdio dependency; accidental `childOnly`
or transformer target exposure; an extra Draw codec; and a generated-link row
without a matching receipt. A focused native package receipt law then proves
the exact single-row closure, while a trusted-loader law proves that a profile
with Draw plus the required stdio closure yields one Draw target and that every
failure leaves no partial binding. Browser/WGPU rendering remains outside this
provider wave.

`semio:note` is deliberately **not** the next packet despite its single parent
artifact. Its snapshot carries a `SemioTextSnapshot` child
([`note/🦀️.rs:296`](../../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs#L296)) and its current helper mints a content-derived child id using
`DefaultHasher` ([lines 357–371](../../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs#L357)). It therefore depends on the unaccepted stable-child
identity/publication program rather than providing a simpler provider proof.

## Required evidence gates

These are proposed registrations; none is credited as currently run.

1. **Neutral/source:** `provider-index-v1` JSON schema plus independent Bun
   oracle. Cover all 59 identities, a 26-codec stdio partial closure, GIS Map /
   Terrain visibility, an unavailable Flow row, extensions with no codec,
   duplicate/missing package, package-version substitution, absent/extra
   factory, dependency cycle, wrong platform, wrong visibility, zero/changed
   hash, and canonical generation changes from a private codec and a zero-target
   selected row. Register a registry `provider-index-check` target through its
   `📜️script.ts` and generate launch entries from the launch seed.
2. **Native:** a hub generated-link check compiles the current stdio/GIS
   registry, asks every available factory for its closure, and proves no
   unavailable package is linkable. Trusted-loader laws reject digest/factory/
   version/platform/visibility substitutions, missing child codec, public
   Terrain, mixed-package closure, failed/cancelled build, stale generation and
   profile rotation; success verifies exactly one Map target and leaves no
   partial codec registration on denial.
3. **Process:** a server-only fresh-root producer creates an immutable
   `stdio+gis` generation, restarts the hub to it, and obtains one verified Map
   document-open plan. Separate browser/native negative laws reject a plan for
   unavailable Flow, stale generation, wrong component byte hash, or Terrain as
   an open target before fetch/render. A later Flow profile repeats this for the
   true Flow parent/stdio-child closure.

## Honest nonclaims

This is not a 59-plugin activation claim. There is presently no complete
trusted bundle, no Flow provider receipt, no ordinary hub boot that loads every
component, no browser execution-target lease, and no native WGPU attachment or
renderer proof. Generated provider linkage establishes only a fail-closed path
from package-owned factory declarations to a verified selected catalog; client
installation, document open, mutation, and rendering remain separate packets.
