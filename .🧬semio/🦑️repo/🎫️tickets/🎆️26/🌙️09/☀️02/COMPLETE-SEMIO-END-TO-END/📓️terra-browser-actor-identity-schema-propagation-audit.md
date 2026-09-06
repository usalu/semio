# Browser Actor Identity Schema and Propagation Audit

## Scope and current status

Read-only source audit of the new standalone browser-actor schema/corpus and the current trusted-catalog, DocumentOpenPlan, and execution-target lease topology. No build or runtime claim is made.

The standalone schema is correctly shaped as a required tagged union:

```text
plan  = { kind: "none" } | ClosedIdentity
lease = { kind: "none" } | ClosedIdentity + { byteLength }
```

`ClosedIdentity` contains exactly `kind`, `schema`, `codegenPolicy`, `sha256`, `sourceComponentSha256`, `sourceDescriptorByteSha256`, `policySha256`, and `importInterfaces`. The public shapes deliberately exclude a path and policy body. The `lease`-only `byteLength` is also the right split: it belongs to the exact returned body, not to the plan receipt.

Sources:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🔣️.schema.json:3-50`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🧪️fixtures/🔣️.json:2-52`

During this audit the TypeScript twin landed at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts:1-109`. Its parser correctly admits only own-data records, exact keys, nonzero lowercase hashes, strictly increasing known interfaces, source-hash equality, renderer compatibility, and lease-only length. No Rust twin and no catalog/plan/lease consumer field had landed at the last source read.

### Current TypeScript twin correction

`documentBrowserActorLeaseFromPlanV1` at `🧬️schema/🌐️browser-actor/🟦️.ts:89-96` reconstitutes the package source from the supplied closed plan itself, then re-parses it as Wasm. Thus a cast/forged but self-consistent plan can be projected without comparison to the actual package or actual renderer. This is not a current network-authority bypass—the function has no actor byte owner and current plan/lease consumers have not adopted it—but it is the wrong public projection boundary.

Either make the helper private to the already-validated plan parser, or require `(plan, source, renderer, byteLength)` and pass the outer parsed package hashes and surface renderer. The latter is consistent with `parseDocumentOpenBrowserActorV1` / `parseDocumentExecutionTargetBrowserActorV1` at `:79-85` and prevents the future `leaseFieldsFromPlanV1` integration from accidentally turning a self-reference into proof.

Similarly, `sameDocumentBrowserActorV1` at `:100-108` treats any two runtime values with `kind: "none"` as equal without checking that both are exactly the one-field shape. That is fine only after strict parsing. Keep the comparator internal to parsed DTOs, or validate the `none` shape before returning equality; do not expose it as a generic hostile-input predicate.

## P0 semantic rule for the shared twins

JSON Schema validates one actor record in isolation; it cannot validate its selected package or renderer. The corpus explicitly demonstrates that distinction: `foreign-component` and `foreign-descriptor` are schema-valid but semantically rejected (`schemaAccepted: true`, `accepted: false`). Therefore the shared parser must not be “AJV/serde shape accepted”. Its final semantic admission must receive the parsed package hashes and `surface.rendererTarget`, and enforce:

```text
closed.sourceComponentSha256    == package.componentSha256
closed.sourceDescriptorByteSha256 == package.descriptorByteSha256
surface.rendererTarget == "wasm" iff actor.kind == "closed-browser-actor"
surface.rendererTarget in {"react", "wgpu"} iff actor.kind == "none"
```

The parser must also compare `importInterfaces` to its canonical increasing form after limiting it to the schema's 16-name vocabulary. Do not merely sort it on input: the current `reordered-interfaces` row is intentionally shape-valid but semantically rejected. `none` must reject every closed-only field; the public parser must never default an omitted actor to `none`.

## Corpus omissions worth adding before the parser law is called complete

The existing corpus is useful and already covers missing fields, private path/policy leakage, foreign source hashes, ordering, duplicate/unrecognised interfaces, and byte-length bounds. Five renderer × view cells are absent from its core semantic matrix:

| Missing vector | Expected result |
| --- | --- |
| `wasm-lease-none` | rejected |
| `react-lease-none` | accepted |
| `react-lease-closed` | rejected |
| `wgpu-plan-none` | accepted |
| `wgpu-plan-closed` | rejected |

Add three small hostile/pair laws as well:

1. A `none` plan carrying `sha256` (and a `none` lease carrying `schema`) is rejected by both twins, not merely by AJV.
2. A valid ordered 16-interface record is accepted and a 17th allowed-looking value is rejected. This proves the Rust bound rather than relying on the JSON schema's `maxItems`.
3. Project a closed plan into a lease, then independently alter each public actor identity field (`sha256`, both source hashes, `policySha256`, interfaces, and `byteLength`), and require full-field equality to reject it. The inverse `none` projection must remain exactly `{ kind: "none" }`.

The last law is material because the current TypeScript equality is handwritten rather than structural (`sameLeaseFieldsV1` at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1317-1367`). Adding a Rust field automatically affects derived `PartialEq`; TypeScript needs an explicit tagged actor comparator. A partial `kind` comparison is insufficient.

## Mandatory propagation census

The following is a source-backed census from `rg` of the explicit public construction/projection points. All must receive a **required** `browserActor` field in one greenfield format change; no nullable or omitted fallback is coherent.

| Boundary | Exact current source and required propagation |
| --- | --- |
| Rust public plan | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1444-1462` (`DocumentOpenPlanV1`), `:1534-1600` (`validate`). Add required actor plus package/renderer semantic validation. |
| TS public plan | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:999-1014`, parser allow-list and output at `:1096-1160`. Add `browserActor` to the required key set and parsed return object. |
| Rust public lease | `directory/🧬️schema/🦀️.rs:1639-1721`; add the length-bearing actor, validator, and `lease_fields_from_plan_v1` projection at `:1728-1747`. |
| TS public lease | `directory/🧬️schema/🟦️.ts:1188-1203`, strict parser at `:1214-1290`, projector at `:1296-1312`, and manual equality at `:1317-1367`. These four locations must move together. |
| Directory JSON schema and neutral fixtures | `directory/🧬️schema/🔣️.json` plan/lease definitions (current lease at `:1098+`); `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json`; `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json` and its `.schema.json`. Each currently has actor-free valid plan/installed-target values. |
| Hub retained plan authority | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1147-1215`. `DocumentOpenPlanAuthorityV1` must retain the actor, validate its package/renderer binding, and `public_plan` must serialize it. Otherwise the issued receipt loses a catalog-selected identity. |
| Hub plan issuer | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2287-2326` creates the retained authority from `VerifiedDocumentOpenSelectionV1`; copy the selected actor here before the issue fence. |
| Hub plan exchange and socket validity | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2406-2410` and `:3017-3022` compare selection fields manually. Both must compare actor exactly, so an actor-only catalog rotation invalidates a prior plan/socket authority. |
| Hub execution-target manifest | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2437-2517`, especially the literal at `:2487-2502`. Project the selected actor into the lease; length comes from the retained actor bytes only. |
| Catalog interface / selection | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:296-325` defines `VerifiedExecutionTargetAssets` and `VerifiedDocumentOpenSelectionV1`; `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:381-399` exposes them through `DocumentOpenCatalogAuthorityV1`. Both the selection and assets need actor identity/owned bytes. |
| Catalog test adapter | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:600-635` (`TestDocumentOpenCatalog`) and its selection literals at `:8193-8220` must add the actor owner/value. A fixture-only `Arc<[u8]>` is adequate for route identity testing, but is not browser-codegen evidence. |
| Bundle record / loader | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:106-135` currently has only `BundleComponent` and `BundleFile`; `BundlePackage` is the required catalog-private path holder. The bundle schema and every test-support producer must add a required `browserActor` tagged record before `VerifiedDocumentOpenSelectionV1` can truthfully expose one. |
| Native directory client retained authority | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:390-438` constructs its own receipt-free lease from `DocumentSocketAuthorityV1`. Add actor to that retained authority, `lease_fields`, `matches_lease_fields`, and the plan-to-authority transfer at `:1137-1187`; otherwise native equality silently drops it. |
| Native fixture constructors | Direct lease literals are only `directory/🔌️client/🦀️.rs:412`, `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3401`, and Hub `🚀️bin.rs:2487`. The client also materializes neutral plan JSON at `directory/🔌️client/🦀️.rs:2289-2355`. Update them in the same schema change. |
| Browser worker | `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:582-583` calls the shared projection; `:729-770` verifies and retains the private target lease; `:4160-4186` has a casted test lease; `:4683-4715` loads the real execution-target fixture. All must include actor identity, but actual actor byte ownership/fetch stays a later contained-worker slice. |
| Public TS façade | `🧰️framework/🛍️products/💻️os/🟦️.ts:558-565` re-exports the lease type/parser/equality. Export the actor public type and parser deliberately with that façade if it is intended for host callers; do not expose catalog-private path or bytes. |

The direct `rg` result is intentionally small: one production plan literal (`Hub bin.rs:1199`), three production/fixture lease literals (`Hub :2487`, native client `:412`, Store sync `:3401`), plus projections. The larger responsibility is retained selection/authority state, where an actor-only change can otherwise evade the current manual revalidation comparisons.

## Minimal integration order

1. Land the Rust/TS tagged twins plus strict schema and the expanded neutral corpus. Parse semantic relations only after package and surface parsing; use the exact bounded allowed-interface list shared with the actor builder.
2. Change trusted bundle package/loader/selection/assets atomically, including `none` for non-Wasm and retained actor bytes only for Wasm. Fold all actor metadata into the catalog generation identity.
3. Thread the required projection through plan authority, plan/lease, native client, all equality/revalidation sites, and every fixture in one compile slice.
4. Only then add the protected fourth body and private worker byte admission. Do not activate an actor until the dedicated Worker containment boundary has its own proof.

## Non-claims

The standalone corpus establishes the desired record vocabulary, not trusted-catalog publication, authenticated actor delivery, browser activation, or sandboxing. Current target assets still contain only component and descriptor bytes, and the worker's `renderer-unavailable` state remains the correct current behavior.
