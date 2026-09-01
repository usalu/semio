# TypeScript Mutation Identity Binding 67

## Read-Only Finding

There is no repository-owned production TypeScript `Mutation` trait, provider interface, branded base type, or common discriminant that can identify all concrete authored mutations. The current forms are intentionally heterogeneous:

- [VCS aggregate](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts) imports six leaf aliases and unions them. Its [rename leaf](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-vcs/🟦️component.ts) uses an intersection plus literal `mutation: "renameVcs"`.
- [GIS terrain aggregate](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts) instead uses keyed variants such as `{ ChangeExaggeration: ChangeExaggeration }`; its [leaf](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚change-exaggeration/🟦️component.ts) declares only a payload interface and a `Kind` const.
- [Kernel](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts) has application-specific discriminated unions and opaque `KernelMutation` envelopes. [Replication](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️component.ts) has transport `MutationEnvelope`/`WireMutationEnvelope` shapes with opaque payloads. Neither is a provider contract for leaf declaration identity.

`inspectTypeScriptDeclarationFacts` at [discovery component](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:6479) deliberately returns only syntax facts: declaration names/forms/spans, unexpanded import or re-export aliases, and diagnostics. Its own public contract says that `complete` does not resolve types or mutation providers, and the function at line 7249 explicitly performs no IO, compiler dependency, evaluation, or provider inference. It cannot safely elevate an identifier named `Mutation` into a semantic identity.

## Smallest Safe Packet

Do not add a name-based scanner. Add a schema-first *optional* off-facet binding input whose rows are backed by already-captured source-index bytes:

```text
TypeScriptMutationBindingInput
  sourcePath              admitted TypeScript source locator
  declarationSpan         exact declaration-facts span
  localSymbol             declaration or imported alias spelling
  providerSpecifier       exact import specifier
  providerExport          exact exported symbol spelling
  descriptorOwner         existing leaf owner path
  descriptorSemanticKind  existing descriptor identity
  shape                   literal-discriminant | keyed-variant
```

The production parser may emit only `unresolved` candidates from declaration facts. A row becomes `bound` only when all of the following are supplied from the same captured source view: the local declaration/alias span, a named (not namespace/default) import alias, the resolved provider's exported declaration, and the existing descriptor owner/semantic kind. No fallback by name, suffix, or `Mutation` spelling is permitted. Existing keyed variants require an explicit `shape: keyed-variant` row; a literal `mutation` property alone cannot be generalized to them.

This needs one closed neutral schema/vector and a pure binding projector near the declaration-facts owner. It should consume the existing `MutationTaxonomySourceIndex` bytes/contents, not collect paths again. A test-only TypeScript `Program`/checker may be the independent reference for the same finite fixtures, but no compiler result becomes a production runtime dependency.

## Finite Oracle Cases

1. Direct leaf with an explicitly registered literal discriminant and exact descriptor binding: `bound`.
2. Named type import aliased locally, with the same resolved provider export: `bound`.
3. Same local `Mutation` name declared in a nested scope that shadows an import: `unresolved`.
4. Same-name unrelated interface or kernel transport envelope with no provider row: `unresolved`, never concrete.
5. Namespace/default import: `unresolved`; current declaration facts emits the corresponding unsupported diagnostic rather than fabricating a provider.
6. Re-export-only alias or module specifier absent from the captured source view: `unresolved`.
7. Explicit VCS literal-discriminant row and explicit GIS keyed-variant row: both bound only through their separate descriptor-backed shapes.
8. Incomplete syntax facts: every candidate is `unresolved`, regardless of spelling.

## Inputs

- declaration facts source: `5ef65775df39b8a8e435ffb48d6a7b41070364911b7e398de0f22cdc5b138956`
- kernel TS contracts: `3d432ad558885c62b0891abc2202008a105704bdbc1555880454b08435f20a90`
- replication TS contracts: `c435a50d67d9cbf2a600c8bc6cdce873f0a6b08ceef207cc78bbff76d307549f`
- VCS aggregate / leaf: `4ceac93ae733bb0662b97ea366e931db69412fabcd78612439b37f2479a86dbb` / `daa761eec82458eaf3fc1b31ea12d21827bd6a47ea8a50fe96b9a6219523a033`
- GIS terrain aggregate / leaf: `88c924ba81426bdcbe2698f7e05f2b8a2aff3fd7afaab202cc2c7050e549009f` / `5733ab1ed677cefa4e4716ea828b1506481b512caff8bdabd7e251128db4ea58`

No TypeScript program was executed. No source, collector, taxonomy, schema, test, or cache path was edited; Compose was not accessed.

## Correction: Bounded Contract Scope and Descriptor Authority

The opening global statement is superseded. This audit inspected only the VCS mutation TypeScript aggregate/leaf, the GIS terrain configuration mutation owner, Kernel and Replication transport declarations, the direct-mutation descriptor schema, and discovery metadata authorities. It did **not** census monorepo TypeScript. Within that inspected set, no shared production TypeScript mutation trait, provider interface, branded base type, or common discriminant was found.

The previous GIS example was stale. Current GIS terrain configuration has the snapshot TypeScript interface at [config schema](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🟦️component.ts), a JSON aggregate for `setCamera`/`setLocale`, and direct descriptor leaves [set camera](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🔣️.json) and [set locale](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🔣️.json). It has no mutation TypeScript aggregate or leaf under this owner. The former `ChangeExaggeration`/keyed-variant claim must not be used.

### Existing Authority and Its Gap

[Mutation descriptor v1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json) is already the authoritative one-record-per-direct-leaf contract. It requires `owner`, `semanticKind`, `aggregateVariant`, `payloadSchema`, codec metadata, and `requiredLanguageSurfaces`. [Root structural policy](/Users/ueli/Documents/semio/📜️script.ts:28660) derives canonical direct-owner identity from that descriptor and checks only each *declared* physical surface.

It cannot establish TypeScript declaration identity: v1 contains no leaf export, aggregate export, encoding, discriminant/key, source module identity, or resolved provider. This is observable now: the VCS rename descriptor and the GIS descriptors omit `typescript` from `requiredLanguageSurfaces`; VCS nevertheless has a TypeScript aggregate and leaf. Such a surface is not complete evidence merely because a matching name exists.

[MutationMetadataSource](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:8491) is not a reusable TypeScript route. Its providers are specifically Rust/Cargo `protocol`, OS facade, and `dsl_derive`; its input resolves Rust module graphs and `inspectRustMutationMetadataFacts`.

### Mandatory Descriptor-Owned Follow-Up

Do not add optional binding rows, a name-based allowlist, or a second identity ledger. The smallest safe follow-up is a versioned extension of the existing direct-leaf descriptor. Whenever `requiredLanguageSurfaces` includes `typescript`, one closed `typescriptDeclaration` object is required:

```text
aggregateExport       exactly descriptor.aggregateVariant
leafExport            canonical direct-leaf export
aggregateEncoding     literal-discriminant | keyed-variant
discriminator         required only for literal-discriminant
key                   required only for keyed-variant
```

Canonical aggregate/leaf file locations remain taxonomy-derived, not fields supplied by a second registry. A pure resolver consumes only the captured source index and descriptor bytes, proves aggregate named-import resolution to the declared leaf export, then checks the declared encoding. A shadowed local, namespace/default import, ambiguous or uncaptured provider, incomplete declaration facts, or wrong encoding is a completeness breach. Alias spelling is resolution evidence only, never identity.

The current v1 descriptor cannot claim that proof. An observed TypeScript mutation surface with no such required descriptor contract must therefore be an explicit unsupported-completeness breach; a descriptor declaring TypeScript but failing resolution must likewise fail. This makes the current VCS surface intentionally incomplete until descriptor surface truth and a mandatory versioned contract arrive.

### Finite Future Oracle

1. Literal aggregate with a declared named alias, exact export, and exact discriminator resolves.
2. Keyed aggregate with declared exports and key resolves.
3. A nested declaration shadowing a named alias fails.
4. Namespace/default import, re-export ambiguity, or out-of-view provider fails.
5. Same-name Kernel/Replication transport declarations fail without the direct descriptor contract.
6. Incomplete declaration facts fail despite matching names and paths.
7. A physical TypeScript mutation leaf omitted from descriptor surfaces is reported as unsupported, not silently omitted.

A test-only TypeScript checker can independently validate these finite resolutions. Production stays a pure projector over the one captured source index; it does not execute TypeScript or collect paths anew.

### Corrected Captures

- discovery declaration/metadata authorities: `5ef65775df39b8a8e435ffb48d6a7b41070364911b7e398de0f22cdc5b138956`
- descriptor schema: `db1c30ab7f19ab9a0f46539c71a427ba6ce51789c5c7904ea4d93dd9ea488aee`
- root structural consumer: `c539f565831cce420d5c755f2efe54362956d0f0d769a29ea3805a2e9f62d005`
- VCS aggregate / rename leaf / descriptor: `4ceac93ae733bb0662b97ea366e931db69412fabcd78612439b37f2479a86dbb` / `daa761eec82458eaf3fc1b31ea12d21827bd6a47ea8a50fe96b9a6219523a033` / `9c80c6c030ba4ccffffefde1e97979d981800bfd34d185dd9ba9a815aeda4464`
- GIS terrain config TS / mutation aggregate / set-camera descriptor / set-locale descriptor: `b089ea76aca9792e782f590bfe05886d5323c02c6fcb75e8cbf8d5b81b821e5b` / `3d9e79f4c4f8517828ea90e8ebaef70d72803384fcb3310d96906449508355b5` / `ee54ed41237b534449ca66e672106b0cb257e7fb7c961ce353f37503956e7821` / `4497cacaba427f29d63aed910b602b82830710b3634221744848339e089b2848`

No TypeScript program was executed. No production, collector, taxonomy, schema, or test source was edited; this ticket Markdown is the only changed file. Compose was not accessed.

## Correction 2: Original GIS Owner and Exact TypeScript Join

The preceding correction incorrectly substituted a different GIS configuration owner. The original inspected files both exist as regular files and remain the valid evidence:

- [GIS terrain mutation aggregate](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts) exports `GisTerrainMutation` as `{ ChangeExaggeration: ChangeExaggeration } | { ChangeImportedFeatures: ChangeImportedFeatures }`.
- [Change exaggeration leaf](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚change-exaggeration/🟦️component.ts) exports `ChangeExaggeration` and `ChangeExaggerationKind`.

The other terrain configuration owner, with `set-camera`/`set-locale`, is separate evidence and does not supersede the original observation. The previous text calling `ChangeExaggeration` stale is superseded by this correction.

The prior proposed equality `aggregateExport = descriptor.aggregateVariant` is also wrong. Existing descriptors identify a direct aggregate **variant** (`RenameVcs`, `ChangeExaggeration`), while current TypeScript aggregate exports are `VcsMutation` and `GisTerrainMutation`. The VCS union imports `RenameVcsMutation`, which differs from descriptor `aggregateVariant`; the GIS keyed aggregate imports `ChangeExaggeration`, which happens to equal its variant. Equality by spelling is neither a cross-form contract nor safe identity.

### Revised Mandatory Surface Binding

The descriptor-owned future contract must be closed and mandatory only for a descriptor that declares the `typescript` surface. It must state the exact aggregate and direct-leaf relationship rather than infer it from `aggregateVariant`:

```text
typescriptDeclaration
  aggregateExport          TypeScript aggregate export, e.g. VcsMutation
  leafMutationExport       direct leaf export imported into that aggregate
  aggregateEncoding        discriminated-intersection | keyed-object
  discriminant             exact property/value, required for discriminated-intersection
  key                      exact object key, required for keyed-object
```

`aggregateVariant` remains the existing descriptor/Rust aggregate identity and is not duplicated as a TypeScript export claim. Canonical aggregate and leaf file locations remain taxonomy-derived. The owned resolver may only read the one captured source view: it verifies the aggregate named import resolves to `leafMutationExport`, then verifies the declared union member form and its declared discriminant or key. There is no name fallback, descriptor-local path override, optional binding row, namespace/default import acceptance, or external identity ledger.

This contract can establish the **declared-surface join** for direct leaves: descriptor → canonical TypeScript files → aggregate named import → leaf export → declared union shape. It cannot prove exhaustive off-facet TypeScript coverage, discover omitted declarations, or turn an unadvertised TypeScript file into a concrete mutation. Those remain explicitly outside this packet. A physically observed TypeScript mutation surface absent from descriptor `requiredLanguageSurfaces` is unsupported evidence, not a successful join.

### Neutral Vector Proposal

1. VCS form: `aggregateExport: VcsMutation`, `leafMutationExport: RenameVcsMutation`, encoding `discriminated-intersection`, discriminator `{ property: mutation, value: renameVcs }`; named import and union member resolve.
2. GIS form: `aggregateExport: GisTerrainMutation`, `leafMutationExport: ChangeExaggeration`, encoding `keyed-object`, key `ChangeExaggeration`; named import and keyed member resolve.
3. Aggregate variant spelling differs from the leaf export (VCS) yet resolves through the declared binding.
4. A shadowed local, namespace/default import, unresolved provider, or incomplete declaration facts rejects the join.
5. A descriptor that declares `typescript` but has a missing direct leaf, missing aggregate import, wrong encoding, wrong discriminator/key, or an alias pointing at another export rejects.
6. A physical TypeScript mutation source without `typescript` in descriptor surfaces remains `unsupported`, with no completeness claim.

These are future schema/vector requirements only. No schema, controller, production source, or TypeScript execution was added in this read-only refinement.

### Correction 2 Captures

- GIS aggregate: `88c924ba81426bdcbe2698f7e05f2b8a2aff3fd7afaab202cc2c7050e549009f`
- GIS change-exaggeration leaf: `5733ab1ed677cefa4e4716ea828b1506481b512caff8bdabd7e251128db4ea58`
- VCS aggregate: `4ceac93ae733bb0662b97ea366e931db69412fabcd78612439b37f2479a86dbb`
- VCS rename leaf: `daa761eec82458eaf3fc1b31ea12d21827bd6a47ea8a50fe96b9a6219523a033`
