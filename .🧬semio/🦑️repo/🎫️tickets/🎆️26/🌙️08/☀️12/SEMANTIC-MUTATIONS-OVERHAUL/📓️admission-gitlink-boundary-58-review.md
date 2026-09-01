# Gitlink Boundary 58: Pure RED and Downstream Safety Review

## Released Result

The ticket-only desired contract is executable and RED against the unchanged actual exported projector. No production or canonical source/schema/vector was edited.

- Reference run [run-o1obcl](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/🧫️run-o1obcl/📓️receipt.md): **94/94**, exit 0.
- Actual run [run-N95VUc](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/🧫️run-N95VUc/📓️receipt.md): reference **94/94**, desired projector **0/30**, original invalid-file160000 retention **1/1**, exit 1. `failure:null`, `drift:[]`, all ten fixed input endpoints stable.
- All 30 actual results lack the required observation tag. Fourteen also differ in existing fields/status/diagnostics; sixteen differ only by the missing tag. The latter are not credited as new-contract passes.
- The actual gate invokes `projectTaxonomySourceAdmission` by importing its real N module. It does not execute a copied projector, map old outputs into new outputs, catch a mismatch as success, or substitute a synthetic ownership model.
- Independent reference checks comprise 85 Ajv2020 validations (closed packet, 30 inputs, 30 desired outputs, 24 positive/negative schema cases), five JSON/jsonc-parser agreement checks and four identity/schema-inheritance/preservation checks. Ajv validates the authored desired schema, not the implementation's current output as an oracle.

Actual receipt SHA-256: `aa363de06dc91aa98346d26fc879a6a7018b4b2b30fb486190a6ec41d2877c25`.

Reference receipt SHA-256: `1e3a937040c005397ba0fba33716a37ec78a538050edefb3e7cfca0c5f833ad4`.

Complete receipts, exact input copies, outputs and endpoint identities are retained inside each exclusive run. Complete JSON receipts are also embedded in unique sibling Markdown reports directly under the ticket:
[actual sibling](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58-actual-🧫️run-N95VUc.md) and [reference sibling](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58-reference-🧫️run-o1obcl.md). This is newly authored evidence, not restored lost evidence; duplicate placement is not a guarantee against loss.

## Authored Footprint

1. [Desired schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/🧬️schema/🔣️.json): canonical admission definitions reused, with one required output-only boundary tag, closed tagged tuple constraints, and the ticket case envelope.
2. [Neutral vectors](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/🔣️vectors.json): 30 complete desired input/result records and 24 independent schema cases.
3. [Controller](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/📜️script.ts): 213 lines, `reference|actual`, Bun/Nx; root discovered through the `.🧬semio` ancestor.
4. This review and generated retained run/sibling receipts.

No N, D, S, P, taxonomy, canonical tests, launch, Git index, native source or build target change was made. No nested filesystem/Git read or global roster retry was run.

Reproduction from the workspace root:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/📜️script.ts' reference
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/📜️script.ts' actual
```

The controller itself has no hardcoded workspace root. Commands above identify the actual retained controller; launch registration remains root-owned.

## One Closed Representation

Adopt the root-owned [Gitlink contract](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-57-contract.md): mandatory `repositoryBoundary: "gitlink" | null` on each existing observation. Do not add a second public boundary roots array, duplicate path/OID, reinterpret physical mode 160000 as a directory, or accept a caller-supplied tag.

The desired observation schema requires a tagged record to have tracked origin, exactly one deduplicated index entry with stage 0/mode 160000, and either:

| Actual physical tuple | Boundary meaning |
| --- | --- |
| directory / 040000 / explicitDirectory true | Observed initialized terminal directory |
| absent / null / explicitDirectory false | Index-owned absent/uninitialized terminal boundary |

Absence retains the existing nonblocking `tracked-path-absent` diagnostic. It is never fabricated into a directory. File, symlink, other, unsafe/unobserved, contradictory physical facts, conflicting index identities or nonzero stages retain a null tag and rejection. An ordinary directory and a tracked row without an index cannot invent a Gitlink boundary.

Canonical `candidate` remains closed and forbids the new output tag. Existing fields occur only once in the observation; all roots are derived from the same captured index facts, not a second naming/taxonomy policy. The tag's cross-field JSON constraints do not alone prove input-row consistency or deduplication; the actual projector cases exercise those obligations.

The original canonical case `inconsistent-physical-facts-and-gitlink-are-both-rejected` has its input and all pre-existing result fields unchanged. Only explicit null tags are required on its desired output. Both the reference preservation check and actual existing-behavior control passed.

### Explicit desired rejection vocabulary

- `repository-boundary-descendant`: “Candidate is below an index-owned repository boundary”. The supplied descendant remains in the rejected result with its supplied physical/index/origin facts; it is not silently filtered or newly probed.
- `scope-inside-repository-boundary`: “Scope is below an index-owned repository boundary”. Scope strictly inside a fence rejects with no observations.
- `generator-root-inside-repository-boundary`: “Generator output root is below an index-owned repository boundary”. A declared ignored output root below a fence rejects even without a supplied descendant, retaining the observed parent record.

The final two parent-requested cases were authored before both executions: ignored generator root strictly below the fence, and an NFD raw fence with NFC descendant scope. Existing NFC scope comparison may be used for ancestry guarding; raw source observations remain distinct and are not normalized or merged.

## Exact IO and Source-Index Joins

These are implementation requirements, not IO runtime proof from this packet.

1. [N sourceAdmissionPrepareOptions](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:2712) currently observes the taxonomy input before index capture; current collector cancellation probes also precede index enumeration. Move capture of the existing full stage-aware index rows after lexical/root-chain checks but before taxonomy/cancellation candidate observation. The index reader's taxonomy argument is currently unused. Carry the captured rows and their derived fences in the existing private prepared value; do not enumerate a second independent authority in the collector.
2. [N collector](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:2815) already requests full index rows before its `add` scope filter. Derive conservative fences from every 160000 index fact before scope filtering, including conflicting facts. Conservative fence eligibility is intentionally wider than valid output-tag eligibility.
3. Reject taxonomy/cancellation inputs at or below a fence, and scope/ticket/declared-output roots strictly below one, before nested probing. Exact fence roots may be observed; [N walker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:2772) must stop before `readdir`. Its existing `.git` stop is not a stop at the Gitlink root. Nofollow does not mean no descent into a real directory.
4. [S mutationTaxonomyStructuralDirectories](/Users/ueli/Documents/semio/📜️script.ts:20702) currently adds every observed directory; skip a tagged boundary. [S mutationTaxonomySourceFiles](/Users/ueli/Documents/semio/📜️script.ts:20762) and [S policyFindAllMutationsDirs](/Users/ueli/Documents/semio/📜️script.ts:28169) must likewise exclude tagged rows from authored files/roots and direct-child accounting. Supplied descendants must already reject admission rather than becoming structural evidence.
5. [S mutationTaxonomySourceIndex](/Users/ueli/Documents/semio/📜️script.ts:20789) retains the admission object once, and its digest already includes `admission.membershipDigest`. Preserve that identity/digest connection. A separate boundary path/OID list is unnecessary and creates reconciliation risk.
6. Current S final mutation inventory serializes roots/sourceRoster/records/digest, not the complete admission; do not claim that serialized view independently exposes boundary identities. Its two source snapshots bind the membership digest internally.

The timing contract is relative to captured index authority, not an atomic guarantee against arbitrary concurrent index replacement. Observed source/index drift must fail capture. Existing opaque/Compose, physical/no-state and marker-observation rules remain unchanged.

## Full Normalization Inventory: Fail Before Classification

[N TaxonomyInventory](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:366) currently carries authored entries, violations and digests, not index/boundary identity. [N inventoryTaxonomyWithSourceParentPruning](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:6048) collects admission, rejects blocking diagnostics, then converts each physical file/directory/symlink observation into an authored candidate. A successful tagged directory would otherwise enter `canonicalDirectory` classification. An absent tagged row would otherwise be dropped from authored candidates.

The minimal accepted packet must explicitly reject **any retained boundary in that admission** immediately after collection and before candidate/directory classification. It must not drop the boundary and then normalize a containing authored directory. Keep the existing full-inventory result shape unchanged because this packet does not claim complete normalization in the presence of a boundary.

In the inspected normalization schema owner, the JSON schema is for source admission; the full normalization inventory contract is the TypeScript interface and assembly/digest functions. Do not confuse it with the separate root mutation-inventory JSON schema.

For later complete full-inventory support, retain the same source-admission carrier once alongside derived authored entries, rather than add a duplicate boundary identity array. Bind that carrier's membership digest into the full source/inventory and plan authority. Current [sourceTreeDigest](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:5062) hashes entries only; omitting boundaries would make a Gitlink OID change invisible. [inventoryWithoutTransactionEvidence](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:5066) also recomputes that entry-only digest and must preserve the future boundary authority.

## Downstream Closure Is Separately Unsupported

**Yes: safe complete normalization/planning requires explicit boundary guards beyond classification.** The new full-inventory refusal is not stale-plan/apply safety.

Important correction: [ordinary plan moves](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:7150) already exclude directory entries (line 7161). There is not a generic current loop renaming every authored ancestor. The actual subtree hazards are:

| Actual consumer | Exposure requiring a future early guard |
| --- | --- |
| `planEmbeddedTicketRoots` N6919–6994 | Recursively digests the source metadata root before final closure validation; an omitted Gitlink child could be visited before rejection. |
| `generatorTreeInventory` N6300–6313 / `generatorInputPaths` N6316–6368 | Recursively reads generator output/input trees. Guard the relevant roots before their views enumerate; an unrelated scoped admission is not sufficient global authority. |
| `planTaxonomy` N7396–7460 | Calls embedded/generator planning and destination preimage work before any boundary-aware overlap guard. A caller can supply an inventory object; an earlier inventory refusal alone is not proof for this entry point. |
| `applyTaxonomyPlan` N10019 onward | `assertPlanOutsideTransaction` at N10068 already probes planned ancestry. `actualAffectedPreDigest` at N10336 precedes the fresh inventory at N10338. Its directory branch recursively calls `noFollowTreeDigest`. |
| Apply recovery N10231/N10247 | Terminal attempts may call recursive `actualAffectedDigest` before fresh inventory. |
| Embedded staging N10677 onward | Stages the whole source metadata directory after directory-tree checks; this is the concrete ancestor relocation path. |
| Source-parent/destination pruning N9752–9783 | Current nonempty checks are useful but do not constitute Gitlink authority or a general ancestor-protection rule. |

For complete support, derive guards from the same authoritative index/admission carrier, reject source/destination/recursive-root overlap before reads or moves, protect boundary roots/descendants and mutable ancestors, bind fresh index identities at apply/resume, and retain no-descent in all recursive views. Distinguish a harmless observation of a containing directory from moving/pruning/recursively reading its entire subtree. Do not introduce a skip list or coerce rejected status into complete.

These planner/apply guards are **not implemented or tested here** and require taxonomy coordination. Root's admission/full-inventory refusal packet must not advertise normalization or apply readiness.

An incidental source/schema mismatch was observed but not changed or executed: the root mutation-inventory schema permits sourceRoster roles `source|assignment-ledger`, while current S also emits taxonomy-schema and mutation-descriptor-schema records. This schema is not the full N inventory authority and does not justify weakening boundary admission.

## Future Native/IO Law Joins

The executed laws here are pure TypeScript and schema checks only. Root/IO owner still needs actual collector/prepared/walker laws for before-probe input fences, exact-directory/absence observations, conflict fences, all origin unions, no descent and source drift. Full-inventory tests must prove rejection before classification and content reads. Future complete planner/apply support additionally needs actual preimage/embedded-root/generator/ancestor-overlap and stale/resume tests against existing concrete functions, not a toy mutation model.

No native/Cargo, new real Git fixture, candidate traversal, normalization/apply execution or global census claim follows from 94 reference checks or these 30 pure projector executions.

## Frozen Source and Evidence Identity

Every fixed read used any-case lexical Compose exclusion, complete nofollow ancestry checks, O_NOFOLLOW open, descriptor identity checks before/after the read and endpoint recapture. Inputs were never overwritten by endpoint values. Full receipts retain metadata and captured bytes; these hashes describe the executed RED, not future source state.

| Input | Bytes | SHA-256 | Actual endpoint |
| --- | ---: | --- | --- |
| N | 897206 | `34ca6ab7cdf9bee2738766d88d463be76541c405666f52fe6a59c272e3a9588f` | stable |
| D | 655775 | `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423` | stable |
| S | 2831244 | `e41abcb93ee624c43b443d42b0848a100bddaf39f052519064c673327c1134d7` | stable |
| taxonomy | 386042 | `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce` | stable |
| canonicalSchema | 7783 | `ed117e588c2aa2e1a0622455ab4710cec623725dce829af6ddb4a5f6328bb1a5` | stable |
| canonicalVectors | 46906 | `14a4b2cf5adcb8d09bf0cb481c7c693be1d3a817a81b28431edfae91ab2cdf91` | stable |
| schema | 11281 | `abf2569aa5517e76905f62c6f7a9c3cb5214e63c5020fc47e6a5398323d9ce83` | stable |
| vectors | 70444 | `6cd0dfa02223d11c2a1a86302daf14613eccbcb9be28d25134f8c3216407108e` | stable |
| contract | 4288 | `304d9e8a980bdb93e208392e91ecd08fb4d454f4236bafeb0c956b76d7c8a548` | stable |
| controller | 15168 | `61e315837cd488820bd298de7a88bac2376a93cb4b1ba8cb7d756c39c6b0b5fd` | stable |

### Additional reviewed N declaration slices

Line ranges below refer to N34ca6ab7. These are read-only source evidence, not runtime execution of planner/apply.

| Declaration | Lines | SHA-256 |
| --- | --- | --- |
| TaxonomyInventory | 366–378 | `4b2aae58ac822d2c237a56d7eab78884aa3e41fd2d0f16b27021883f6fa00350` |
| sourceTreeDigest | 5062–5064 | `be20e1a0957b70d4cb5aa560e23be69af75d18f7a503bb2f4caca8333f03aeee` |
| inventoryWithoutTransactionEvidence | 5066–5072 | `2119ff2003cd4de33016765ec3604238d7e154eaf1b9e97811536524b9cbae79` |
| generatorTreeInventory | 6300–6313 | `c5a1cde41526617e24fc583de638ed89acb7eccb2ba61551afd4207bd233744e` |
| generatorInputPaths | 6316–6368 | `20e3dbb9c857e70e780aafe445a132d8131fdfe607421766c0d086a648968322` |
| planEmbeddedTicketRoots | 6919–6994 | `8aed333934869e3dc84ae36a48055de21a89085b4a6dec1d30eee0baf806e10d` |
| planMoveReferenceAuthority | 7150–7195 | `6fb5e6d76c1b1d60532714474347e8b564c56418a453d1022728f47d1c801698` |
| planTaxonomy | 7396–7460 | `90367dbfb09b94342c001642b951acd815071de917b8418d47b1bfc33a60f95f` |
| actualAffectedDigest | 9417–9439 | `6c971bb44302df718dfb128e01ccc69cba37fd9f734b962342b6adc539f9abb7` |
| actualAffectedPreDigest | 9441–9463 | `76beec34bef9327367547356e7e8107a5f2eca32be53a930de2308fbc432b4a9` |
| committedSourceParentPrunePaths | 9761–9774 | `60c46aa330b2a0736ab4eb46a032992188a30d570ab7ca650076b3a228898b24` |
| applyTaxonomyPlan | 10019–10881 | `f0ab6668d5e2a1dc492006104bf9e689e29a647eeeabcdad9938c8bc1518b3d0` |

The separately inspected root mutation-inventory schema was `f29ff0d9fcd179110d41249f634a9c4aee9240d0fd9d153b67a4702a9e9accee` (1701 bytes). It is outside the ten-input pure controller because it is audit evidence, not an input to the projector.

## Thirty Desired Projector Cases

1. `declared-ignored-output-root-below-index-fence-is-rejected`
2. `nfd-index-fence-rejects-nfc-descendant-scope`
3. `observed-gitlink-directory-retains-exact-parent-index-identity`
4. `uninitialized-gitlink-is-absent-not-fabricated-directory`
5. `ordinary-file-requires-explicit-null-boundary`
6. `ordinary-directory-does-not-become-a-repository-boundary`
7. `duplicate-index-identity-and-overlapping-origin-union`
8. `duplicate-row-order-does-not-change-boundary-or-origins`
9. `duplicate-identical-stage-zero-tuple-collapses-once`
10. `contradictory-stage-zero-object-ids-reject-not-boundary`
11. `unmerged-gitlink-stages-remain-rejected`
12. `stage-zero-plus-conflict-is-not-an-unambiguous-boundary`
13. `regular-stage-zero-plus-gitlink-conflict-remains-rejected`
14. `index-owned-boundary-requires-tracked-origin`
15. `gitlink-index-with-physical-file-is-rejected`
16. `gitlink-index-with-physical-symlink-is-rejected`
17. `gitlink-index-with-physical-other-is-rejected`
18. `unsafe-ancestor-suppresses-physical-claims-without-losing-index`
19. `unobserved-gitlink-is-not-an-absent-boundary`
20. `contradictory-directory-and-absence-is-not-a-boundary`
21. `opaque-gitlink-retains-no-physical-or-index-authority`
22. `supplied-descendant-is-diagnosed-and-not-silently-filtered`
23. `scope-strictly-below-gitlink-rejects-before-observations`
24. `scope-at-exact-boundary-retains-terminal-observation`
25. `all-four-origins-and-generator-match-retained-once`
26. `raw-nfd-and-nfc-boundary-identities-remain-distinct`
27. `segment-prefix-lookalike-is-not-a-boundary-descendant`
28. `tracked-directory-without-index-cannot-invent-boundary`
29. `inconsistent-physical-facts-and-gitlink-are-both-rejected`
30. `conflicted-gitlink-still-fences-scoped-descendant`
