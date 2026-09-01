# Mutation Structural Source View Preparation

## Captured Baseline

The ticket controller executes extracted declarations from the current root [`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts): `policyListMutationDirs`, `policyMutationDescriptor` plus its actual JSON-subset validator, `policyMutationLeafHasRunnableTest`, `inspectMutationRootReachability`, and `policyMutationStructuralBreaches`. It uses a closed in-memory filesystem; it neither imports the root router nor reads the workspace through the mocked calls.

`red.json` records 37 assertions and the first/final-equal hashes. The actual current reader accepts a single unadmitted `➕add` directory, descriptor, mounted leaf source, and enabled test; the actual structural scanner also reads the unadmitted binary root. The captured reads prove these paths:

- `owner/🧬️mutations/➕add/🧬️schema/🔣️.json`
- `owner/🧬️mutations/➕add/🧪️tests/🦀️.rs`
- `owner/🧬️mutations/➕add/🦀️.rs`
- `owner/🧬️mutations/💾️binary/🦀️.rs`

The descriptor-schema reader also still contains the `WORKSPACE_ROOT` fallback. This is a genuine current-source RED packet, not a green implementation result. Current reachability needs `lstatSync`/`readFileSync`; therefore a captured-child, zero-filesystem positive case is intentionally not claimed before the view cutover.

Inputs and retained result:

- [controller](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/📜️script.ts)
- [neutral schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️fixtures/🔣️schema.json)
- [neutral vector](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️fixtures/🔣️vectors.json)
- [red result](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/red.json)

## Proposed Same-View Contract

`MutationStructuralSourceView` is a pure projection of the one complete `TaxonomySourceInventory` retained in `MutationTaxonomySourceIndex`. It must retain the complete admission, selected roots, regular source `bytes`/`contents`, and two captured schema byte records (taxonomy and mutation-descriptor schema), each bound to the admission’s taxonomy content hash.

Its directory facts are only explicit admitted directories or proper ancestors of admitted regular files. Each fact carries its supporting admitted observation paths. `regularFile`, `directory`, `children`, and `readText` resolve only those facts and captured bytes: absent entries and symlinks are distinct failures, never an empty directory or a filesystem fallback.

The post-cutover positive case will pass an already captured aggregate and its exact captured child into reachability with a filesystem sentinel that throws on every call. It must resolve the wrapper without any read. An out-of-view mounted child must remain unresolved even when the ambient filesystem contains it.

## Exact Future Function Footprint

1. `mutationTaxonomySourceIndex` retains the complete admission and constructs the structural projection; no second inventory is allowed.
2. `policyMutationStructuralBreaches` keeps its public boundary, acquires one source index there, and delegates to a view-taking structural implementation.
3. `policyMutationDirectOwnerBreaches`, `policyListMutationDirs`, direct-owner provenance, wrapped-type reachability, and inventory record classification consume the same view’s directory facts.
4. `policyMutationDescriptorSchema` and `policyMutationDescriptor` consume captured schema/descriptor bytes. Delete process-global cache and `WORKSPACE_ROOT` fallback from that path.
5. `inspectMutationRootReachability`, `policyMutationLeafOwnedRustSource`, and `policyMutationLeafHasRunnableTest` resolve only view paths/bytes. They must never call `lstatSync`, `readFileSync`, or a workspace authority helper after the boundary.
6. Structural surface and codec/schema parity readers obtain aggregate, leaf, text, binary, catalog, and payload bytes from the view. Missing items produce the existing structural breach; they cannot be satisfied by ambient files.

No production source was changed in this preparation packet. No Cargo or native command was run.

## Fallback Red Amendment

The first capture established that the fallback branch exists but did not force it. The second isolated closed-filesystem capture omits the local descriptor schema and supplies only `/workspace-must-not-be-read/schema/🔣️mutation.json`. The extracted current `policyMutationDescriptor` accepted the descriptor and read that workspace path. This is retained separately in [fallback RED](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/fallback-red/result.json): 38 assertions. The original `red.json` remains untouched.

## Pre-Splice Contract And Footprint

The taxonomy schema record has `{ path, bytes, sha256 }` and is valid only when `sha256(bytes) === admission.taxonomyContentHash`. The mutation descriptor schema is a separate `{ path, bytes, sha256 }` record, captured through `semanticOwnedInputFileSnapshot`; it has its own digest and is never compared with `taxonomyContentHash`.

`MutationStructuralSourceView` contains the full retained `TaxonomySourceInventory`, the selected source index maps, the two schema records, and a directory-fact map. Directory facts are materialized only from explicit admitted directory observations and proper ancestors of admitted regular files. The fact records supporting observation paths and never changes raw physical Unicode spelling; NFC is comparison-only.

The splice is limited to root `📜️script.ts` declarations:

1. Expand `MutationTaxonomySourceIndex` and `mutationTaxonomySourceIndex` to retain admission and construct the two captured-schema records and structural directory facts.
2. Add internal view readers for regular source, child folders, descriptor/schema, exact leaf-owned test paths, root-origin paths, and codec/catalog paths. Each returns explicit missing/symlink/absent states and reads no ambient filesystem.
3. Convert `policyListMutationDirs`, direct-owner checks, `inspectMutationRootReachability`, runnable-test reading, descriptor loading, and structural codec/catalog/parity reads to the view. Keep `policyMutationStructuralBreaches(repoRoot, roots?)` as a one-index boundary wrapper over a pure view-taking implementation.
4. Pass the same view from `inventoryMutationTaxonomy` into the structural scan and replace its aggregate reread, folder enumeration, direct/nested/current-central `existsSync` checks, and `resolveRustPathAttributes` filesystem use with captured-view results.

No policy family outside the structural/inventory path is in this footprint. There is no N/D/P or admission-helper change.

## Mounted Same-View Source Packet

The approved root-only splice is mounted in [`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts). `MutationTaxonomySourceIndex` now retains the admission, directory facts, taxonomy capture, and a separately hashed mutation-descriptor schema capture. The captured taxonomy hash is rejected unless it equals `admission.taxonomyContentHash`; the descriptor hash is bound separately in the source roster and endpoint digest.

`policyMutationStructuralBreaches` is now a one-index boundary wrapper over `policyMutationStructuralBreachesView`. The view implementation uses captured aggregate/leaf/surface/codec/catalog bytes, captured schema bytes, captured directory facts, view-backed descriptor parsing, view-backed test discovery, and view-backed origin reachability. The old process-global descriptor cache and workspace fallback were removed. `inventoryMutationTaxonomy` passes its existing `before` index view into the structural scan and uses it for root bytes, folder facts, path-mount evidence, direct/nested state, and `currentCentralComponent`; its consumer-target resolver receives the captured taxonomy instead of calling `loadTaxonomy`. No second collector is introduced.

The new [view-green result](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/view-green/result.json) ran through scoped Bun/Nx with 26 assertions. It executes extracted actual view readers: a captured aggregate and child resolve without filesystem injections; an absent child and a symlink observation remain distinct rejections; an absent descriptor and absent leaf test fail; and the actual view scanner body has no ambient filesystem reader. This is source/reference evidence only, not a full repository semantic or native proof.

Source checkpoint after the final green capture: `📜️script.ts` SHA-256 `09be8a16a3e401afb2a7126832547ba4cf5fbd21c9e40e205e8ffc4dd286dd17`.

## Review Repair

The exported `inspectMutationRootReachability` is now a one-index wrapper: it rejects unsafe locators and a supplied aggregate string that does not equal the captured aggregate bytes, then delegates to the sole view algorithm. The obsolete live root validator and live leaf-owned source/test readers were removed. Explicit structural roots now reject malformed, opaque, absent, symlinked, and out-of-view paths rather than filtering them to an empty success. Test mounts reject dot segments, absolutes, backslashes, opaque segments, and escapes before any lookup.

Schema bytes are reused from the initial taxonomy/descriptor captures when either path also belongs to the source-file map. The controller now creates a UUID-named receipt directory on every execution, preserving every prior result. Its latest [parse-ready source receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/view-green-10d405ee-d83d-4720-8bff-3d5ba40f333a/result.json) has 37 assertions, including whole-root TypeScript parse/transpile diagnostics, exact one-index wrapper invocation, strict requested-root rejection, absent/symlink distinction, no-filesystem captured-child reachability, and escaped test-mount rejection. Root source SHA-256 is `ed231001553d3eeb95c1acb6f50861e7601ee8b0ad53503987af7599fa33a8ec`.

## Complete Captured Scanner Law Amendment

The earlier view receipts were reader and wrapper evidence only. The retained [complete view receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/view-green-cad89321-5e14-46b2-b1e0-8b6b40514d9e/result.json) executes the extracted actual `policyMutationStructuralBreachesView` with the real discovery Rust inspectors, real captured taxonomy bytes, and the authoritative captured mutation-descriptor schema. Its complete closed view has one direct `➕add-item` leaf, valid root/leaf binary counterparts, full descriptor metadata, and an enabled direct Rust test. The actual scanner produces zero codec or test-presence breaches for that admitted counterpart.

Two negative views use the same actual scanner with no filesystem callback: a binary file only observed outside `contents` does not satisfy the missing exact leaf binary contribution and produces `mutation/language-parity`; removing the direct enabled test produces `mutation/test-presence`. This establishes direct captured ownership rather than a body-regex claim. The receipt has 65 assertions and first/final-equal hashes for the root router, discovery parser, taxonomy schema, descriptor schema, neutral fixture/schema, and controller.

The post-cleanup root checkpoint is `📜️script.ts` SHA-256 `03710a7e67ccfdcb57e61e324ec9884a6d7b1ba7940acc43d4510668417013e0`. A direct `tsc --noEmit` experiment against the root script was attempted but followed the monorepo graph and failed on broad existing configuration/import/Bun-ambient diagnostics; it is not type-green evidence. The retained controller's TypeScript AST parse/transpile diagnostic remains syntax evidence only, not whole-program type evidence or native proof.

## Deferred Census Limits

`policyStructuralMutationDirs` still classifies a direct child by physical one-segment directory shape, then excludes the three hardcoded facets `📚️examples`, `💾️binary`, and `📝️text` and silently excludes names beginning with `.`. The taxonomy's mutation ownership rule describes optional organizational facets but does not provide a captured schema-owned direct-child classifier for this scanner. Consequently an admitted hidden malformed child can currently be suppressed rather than reported as an invalid concrete owner. This packet does not widen the released source scope; the follow-up needs a schema-first direct-child/facet classification test, including an admitted dot-prefixed candidate and an admitted unknown one-segment child.

The retained same-view release covers `policyMutationStructuralBreachesView`, its public one-index wrapper, and the inventory consumer. Legacy `policyMutationDirectOwnerBreaches` and `policyMutationImplPresenceBreaches` remain ambient readers and are explicitly outside this packet. They must not be cited as same-source-view coverage.

## Neutral Fixture And Type Diagnostics Amendment

The full scanner fixture and its expected removals now live in the schema-validated [neutral vector](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️fixtures/🔣️vectors.json): aggregate/leaf/binary source, all descriptor fields, zero expected good breaches, the exact out-of-view binary observation, and the exact test-removal leaf source. The updated [65-assertion receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/view-green-fd0b3124-2890-4fd1-905e-9f8912c23d1f/result.json) requires `goodBreaches.length === 0`.

This is intentionally a structural membership fixture: its binary sources contain only identity/tag constants and its leaf has an empty trait implementation. It proves captured direct ownership, schema metadata, test reachability, and breach selection; it does not prove executable binary codec behavior or mutation semantics.

The targeted compiler receipt is [type diagnostics](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/type-diagnostics-eaa7219e-d2e5-406c-b020-32a1be8094fe/result.json). It uses real root-tsconfig compiler options, one root entrypoint, and a compiler host that rejects any compose-path access. The dependency graph has 506 diagnostics overall; line 20713 has the two directly relevant errors: `TS2769` and `TS2345` because the `new Map([...supporting].map(...))` callback widens its entry from a two-element tuple to `(string | object)[]`. No other diagnostics occur in lines 20694–20825. This is a real type failure, not a green check.

The source repair types the map callback as `[string, MutationTaxonomyStructuralDirectory]`, sorts that typed entry array by the established byte comparator, and constructs `Map` from it. It preserves directory facts and ordering. Pre-repair root SHA-256 was `03710a7e67ccfdcb57e61e324ec9884a6d7b1ba7940acc43d4510668417013e0`; post-repair SHA-256 is `2652ad0e29fa675355d8844b8645ebf3388f02b1f7fdaae2b5c3d6948bd54dd2`.

The post-repair [type receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/type-diagnostics-eaa15186-f91d-459a-8613-ef8b6eeaef47/result.json) has 504 graph diagnostics, but none in lines 20694–20825; the two tuple diagnostics disappeared. The post-repair [scanner receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/view-green-c763ea18-b9d2-469b-9f8f-ba56f1aa7d35/result.json) remains 65/65 under one stable input capture. The broad residual diagnostics remain outside this packet and are not represented as a type-green result.

## Direct-Child Classifier Compatibility Amendment

The new exhaustive classifier legitimately identifies the fixture's root `💾️binary` folder as `root-infrastructure`: the taxonomy permits binary only below a concrete direct owner, not at the mutation collection root. This is a pre-existing placement conflict, so the schema-validated structural fixture keeps its aggregate/leaf/binary language-parity and test-presence laws and now expects one explicit `mutation/direct-owner` breach at `owner/🧬️mutations/💾️binary`; it does not call the complete scanner zero-breach green.

The retained [67-assertion receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/view-green-004afd41-28df-4aef-93f8-334cf307c0f4/result.json) runs the real extracted view scanner with closed captured bytes. It still proves a direct binary leaf is required and a missing direct leaf test is reported; both negative cases preserve the one root-infrastructure breach as separate evidence. The [targeted compiler receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-structural-source-view-52/🧫️runs/type-diagnostics-44fb8e1e-fe95-44fd-931b-359d141953ad/result.json) reports zero real diagnostics in the dynamically located classifier range 27817–27854, while retaining 499 unrelated graph diagnostics outside this narrow packet.
