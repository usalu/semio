# Duplicate Member Scanner Caller And Control Closure

Readonly source snapshot. No production edits, Cargo or runtime assertions. New canonical floor direction supplied by Root: physical `pack/🔤️json` independent package, member contract `json/🧬️schema/🧩️members`, TS `json/🧩️members/🟦️.ts`, mandatory Reject/Replace with production Reject; Replace only original normalization tests. Names proposed below for control concepts are design recommendations, not claims that new APIs already exist.

## Exact Direct Roster

The full matching roster follows. Root script import is currently an import only; discovery has import/reexport plus actual use. Test injection sites are tests, not production runtime consumers.

```text

```

## Current APIs And Concrete Callers

Repo scanner: `jsonDocumentDuplicateKeys(source: string): string[]` synchronously regex-tokenizes the full string, maintains Set per object, decodes names with system JSON.parse, and returns messages. No byte ceiling/deadline/control/progress is accepted. It assumes syntactically valid input; the direct consumers normally platform-parse first.

- Discovery `mutationPayloadSchemaProblems` line 1698 platform-parses captured content then merges duplicate problems and document-schema problems. Caller is synchronous and receives a node lookup callback. Async propagation must start at mutation schema authority and its normalization callers; do not merely await a scanner inside a still synchronous source loop.
- Publication `renderOwnerPublications` line 84 has synchronous nested decode at line 104; line 107 checks duplicates. It uses owned captured input view or no-follow file admission and byte limits, fatal UTF-8, and sorted package traversal. Add owned operation control at render boundary and pass through decode and package iteration; callers must await the result before publishing projection. Preserve exact captured view; no fresh reads during parsing.
- Normalization line 4572 checks descriptor bytes after platform parse and UTF-8 round-trip equality. Its surrounding competing-descriptor loop platform-parses other JSON and silently continues on errors. Propagate source-operation control across the descriptor and child loops; do not turn malformed competing-input evidence into an accepted empty descriptor.
- Structural reachability `policyMutationDescriptorView` line 187 platform-parses captured schema and descriptor, checks scanner line 193, then subset-validates. This helper is synchronous and returns `{descriptor?,problem?}`. Async source/closure preparation should admit strict captured documents first; synchronous policy over those admitted owned records then remains cheap. Avoid introducing arbitrary accessors into canonical JSON input authority.
- OS Registry `decodeRegistryDescriptorV1` line 8 imports the Repo scanner directly, max 4 MiB, fatal UTF-8; `readDescriptorJson` discovery line 251 calls it after exact no-follow captured-view admission. Replace its product-to-Repo dependency with direct canonical JSON member admission. Pass operation control from registry capture/admission; return owned parsed document or validated descriptor after await, not an alias forwarding old scanner.

OS scanner: `rejectDuplicateJsonObjectNames(source: string): void`, exported line 363, three actual direct uses, all same catalog-verification file. Recursive parser with depth limit 128, primitive token delegated to JSON.parse, object key sets, non-JSON `\s` whitespace accepted locally but final JSON.parse gives syntax authority. It lacks cancellation/progress/deadline input. Delete definition/export after cuts; no old-name forwarder.

- `verifyDescriptorPairBytesV1` line 516 calls scanner at 522, fatal UTF-8 and fixed descriptor JSON/pack limits. `verifyFreshCatalogPackageV1` line 546 calls this pair verifier. Descriptor-emission line 121 also calls it: emission guards check timeout/cancellation before verify and before publish, but cannot interrupt scanner. Make pair verification await canonical owned member admission with the SAME cancellation and remaining emission deadline; then pack comparison and hash stages need checkpoints too. Publishing must await completed verification.
- `validateCatalogDescriptorPair` line 559 calls scanner at 570; decode uses Buffer.toString UTF-8 replacement rather than fatal admission. Called from catalog source audit line 663 and descriptor-verification line 127. Thread source audit cancellation/progress through exact byte admission and JSON parse; replace lossy UTF-8 decoder with canonical fatal byte validation.
- `createFreshCatalogBuildVerifier(...).verify(entry, control={})` line 809 calls scanner on commit-marker at 816; marker decode is lossy Buffer.toString. Subsequent artifact chunk readers already carry control cancellation/progress. Move same control through marker capture and canonical JSON admission BEFORE marker identity/canonical-byte checks. Caller `executeCatalogVerificationPlan` is async, but verifier currently synchronous; await verifier.verify in executor and hub stdio script callsite line 1207. Marker parsing must not become an uncontrolled gap between cancellable artifact reads.

There is no scanner-specific timeout today. Fixed input limits and depth ceiling bound work but do not constitute progress or cooperative cancellation. CatalogArtifactControl contains cancellation callback and artifact progress; CatalogVerificationControl similarly carries cancellation/progress; emission uses elapsed deadline guards. Adapt caller controls explicitly into owned JSON control at the product boundary, without exporting product controls from the neutral JSON provider.

## Async Owned Control Propagation

Canonical JSON member admission should take immutable owned/captured bytes or text, explicit Reject policy, and owned limits/control. Count consumed byte offsets and completed member items monotonically; yield between bounded chunks so cancellation callbacks can observe new signals. A cancellation checkpoint inside one synchronous megabyte scan still prevents event-loop-driven cancellation from arriving. Carry a deadline/remaining budget at product admission; do not reset time budgets per document. Capture and decode failure must preserve exact source coordinate and owned diagnostic kind (malformed, duplicate, UTF8, budget, cancelled).

Each caller awaits admitted document before semantic validation/pair checks; reject partial output. Capture schema dependencies in deterministic order and aggregate parse progress without double-counting reused resources. Close/retire partial captures on cancellation using the same owned workflow. Existing semantic policy layers may synchronously inspect fully admitted own data; do not accept getters, prototypes, sparse arrays, or arbitrary objects as text authority. System JSON.parse only runs after canonical member/syntax byte authority and cannot erase duplicate evidence first.

## Language-Agnostic Tricky Case Draft

| Case | Input / Setup | Required Observation |
| --- | --- | --- |
| Decoded duplicate | `{"a":1,"\u0061":2}` | Reject same decoded name, deterministic second-member source offset; Replace explicit oracle keeps 2. |
| Escape slash | `{"a/b":1,"a\/b":2}` | Reject decoded duplicate. |
| Distinct nested names | `{"a":1,"nested":{"a":2}}` | Admit independent object scopes. |
| Empty key pointer | `{"": {"": true}}`, reference `#//` | Resolve both empty segments; never filter empty components. |
| Percent fragment | `{"$defs":{"a b":true}}`, `#/$defs/a%20b` | Resolve per chosen URI-fragment policy or explicit unsupported error; never silent lookup. Decode once before pointer unescape. |
| Pointer escapes | keys `a/b`, `~x`, refs `#/a~1b`, `#/~0x` | Exact decoded key; reject invalid `~2` escape under strict policy. |
| Root/sibling collision | root and sibling same `$id` with different bodies | Reject resource identity conflict before any projection. |
| Duplicate siblings | two physical files same `$id` | Reject deterministically; no last wins. |
| Cyclic refs | A → B → A | Finite captured closure with each resource once; validation cycles get explicit bounded semantics. |
| Malformed JSON | trailing bytes/comma, control character in string, incomplete exponent | Refuse syntax; never partial resource registration. |
| Unknown reference | exact unknown document ID or missing pointer | Explicit source-coordinate refusal; no default/null/empty projection. |
| Nested scope | document outer `$id`, nested relative `$id`, nested `$ref` | Resolve by explicit supported scoped policy or refuse unsupported; never use outer cache accidentally. |
| Invalid UTF-8 | isolated continuation/truncated multibyte sequences | Refuse before TextDecoder replacement can change identity. |
| UTF-8 order | resource names `a`, `z`, `ä`, `😀` | Exact deterministic byte/coordinate ordering defined by contract, independent of localeCompare/platform. |
| Prototype names | keys `__proto__`, `constructor` | Ordinary JSON owned members; no inherited accessor resolution. |
| Budget/cancel | same large input at several cancel byte offsets | No accepted partial output; bounded progress and close work; cancellation observable after event-loop yield. |

## Exact Input Receipt

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts` | 27552 | `e87d6fc8a3f22edd1571e2c4390af6e3d8bb77e1264c3184f0e11223cf055edc` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts` | 10680 | `20bd5f930f276bfa071ba349f20ce6040bd9ccc934cdeb67d9f2de45b0dbcd5b` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🟦️.ts` | 7953 | `d2ccc69a2a4a3420e5df5dd4e39b55c5d16317637c09430a3c540c177b4da09b` |
