# 🧪️ Pure Source Admission Adversarial Audit 51

## Result and scope

The actual exported `projectTaxonomySourceAdmission` is **RED: 9/16 adversarial cases, 113/126 checks** in [the approved-expectation capture](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/🧫️run-31060R/🔣️result.json>). The existing canonical baseline remains **18/18**. All eleven bound pure declarations and every captured input remained stable during this run. The whole N source was `342e780b71b6bd0fc9e6cc66b151e58fa9e78ecf0e149846ab297fe62659b0fe` before and after.

This packet changes only its ticket-local controller, reference schema, neutral vectors, and this report. It does not change N, canonical schema/tests/vectors, launch registration, or any production owner. No filesystem/Git source-admission API is invoked; candidate paths are strings only. Artifact reads/module loading and retained ticket evidence writes are not claimed to be IO-free, and no syscall instrumentation was performed. No real excluded subtree was traversed. No Cargo, native Rust, source cleanup, or git mutation occurred.

Root has reviewed the cases and approved the semantic decisions below. The terminal RED and source release were sent before root's production patch. This report is the bounded pre-fix audit, not a claim that later source is still RED or that root's broader IO gate was executed here.

## Schema-first executable owner

- [Actual controller](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/📜️script.ts>).
- [Neutral vectors: sixteen projections and two malformed candidates](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/🔣️vectors.json>).
- [Ticket schema, referencing the canonical schema](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/🧬️schema/🔣️.json>).

The schema is a `$ref` to `urn:semio:taxonomy:source-admission:v1#/$defs/sourceAdmissionCases`, not a local schema replica. Ajv2020 loads the actual canonical file and validates the complete neutral document, each input, each expected result, and each actual result. The two additional malformed candidates exercise the actual candidate schema and actual projector rejection. All authored expected results are accepted by the actual canonical schema.

The controller imports the actual N module and calls its pure export. It does not implement a second projector or use a source-substring simulation as execution proof. For every case it checks totality, exact result, input nonmutation, canonical schema validity, reverse input ordering, and repeated determinism. Reverse ordering covers candidate rows, generator root rows, origins, and index tuples. These are finite exercised permutations, not a proof for every permutation.

The independent third-party check here is real Ajv schema validation. The exact behavioral expectations are handcrafted domain fixtures; they are not mislabelled as an independently implemented semantic oracle. TypeScript's actual AST locates and fingerprints the executed projector's eleven local declarations. Module import and final reads must retain those exact slices; whole-N drift is reported separately.

Replay from repository root:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/📜️script.ts'
```

The controller derives the repository root from its own ticket taxonomy, rather than embedding this workstation path. Exact inputs undergo lexical exclusion before traversal, ancestor symlink checks, `O_NOFOLLOW` where available, descriptor/path identity checks, and size/mtime stability checks. First fingerprints are preserved instead of being replaced by final reads. These checks bind the evidence files; they do not claim atomic snapshot isolation against all possible concurrent writes.

## Retained RED sequence

| Capture | Neutral cases | Actual checks | Canonical baseline | N whole SHA prefix |
|---|---:|---:|---:|---|
| [Initial RED](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/🧫️run-BYPPWh/🔣️result.json>) | 9/15 | 107/119 | 18/18 | `9afb1790` |
| [Same-vector fingerprint-aligned RED](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/🧫️run-31e0Oa/🔣️result.json>) | 9/15 | 107/119 | 18/18 | `342e780b` |
| [Root-approved expectation RED](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/🧫️run-31060R/🔣️result.json>) | 9/16 | 113/126 | 18/18 | `342e780b` |

All three runs exited 1 as genuine negative outcomes. The second controller revision makes the final N fingerprint and final AST slices derive from the same read. It changes no case semantics. Original controller SHA `bc5dc40492acef4ab46e59628f1773ef22a7657872529596f7a9b7e4f00773bb` is retained in the initial result; final controller SHA is `88288b68e7bb7eada4e977dbcd7167feec8f9d013482ab96cd28cd36cfe3a949`.

Between the first two runs, root changed the disjoint N IO region. The N whole-file hash changed from `9afb17907b1e14d9fdc5bfe14faa77b95f9121de1463306695fb706a9a248bd4` to `342e780b71b6bd0fc9e6cc66b151e58fa9e78ecf0e149846ab297fe62659b0fe`. All eleven pure hashes remained identical across the runs. None of the three runs observed whole-N drift inside its own capture interval.

After root's review, only one existing expected result changed: the empty-scope case with a candidate now requires `observations: []`. Root also requested the new `cancelledDuring: ""` case. Original vector SHA `81c13c445994cbb7289ec75e6875f83b07377160b3f0aa11df8d5f53f98f2e63` and original case expectations remain in the two earlier results. Updated vector SHA is `d5dd5193e2ee49947814aacf9a4a5b29ef1ed950390349a32278e6cb3f4ea0f8`.

The controller was intentionally unchanged for this last capture. Its `expectationAuthority` labels therefore retain the initial “proposed semantic refinement” wording for unsafe-ancestor and generator-conflict cases, and the initial location wording for empty scopes. **This report records root's subsequent approval; those labels do not indicate an unresolved policy decision.**

## Concrete reproduced failures and approved decisions

### Invalid scopes: result-schema failure and non-total projection

Both empty-scope cases are valid inputs under the existing schema. The current projector produces `diagnostics[0].path: ""`. Actual Ajv rejects this at `/diagnostics/0/path` with `minLength: 1`.

With `scope: "../outside"` and a valid supplied candidate, the projector records an invalid-scope diagnostic but continues to `inScope`. `sourceRelative` then throws `Error: Path escapes repository scope: ../outside`. The call fails to return its structured rejected result. This was a pure string call; no candidate path was read from disk.

Approved rule: invalid scope immediately returns `status: "rejected"` and **no observations**. Diagnostic path is the raw nonempty scope, or `"$"` for an empty scope. The initial empty-scope-with-candidate expectation preserved the observation to isolate schema validity; root explicitly chose the stronger no-observations rule because an invalid scope cannot authorize projection.

### Empty cancellation phase

`cancelledDuring: ""` is a schema-valid supplied cancellation value. The current truthiness guard ignores it and returns `status: "complete"`. The new actual case is RED.

Approved rule: a supplied string cancellation phase, including empty, rejects with no observations; empty uses diagnostic path `"$"`. This does not change the existing absent/null non-cancellation contract.

### Unsafe ancestors and claimed physical files

The current projector emits “A symlink or non-directory ancestor prevented observation” yet retains `observedKind: "file"` and `worktreeMode: "100644"` when a row claims those values with `unsafeAncestor: true`. A duplicate safe/unsafe pair also keeps the physical file because physical tuple comparison omits the unsafe flag.

These outputs satisfy the existing result schema; the failure is the approved semantic expectation, not a schema rejection.

Approved rule: any unsafe ancestor produces `unobserved/null/false` physical fields while retaining all supplied origins, exact index identities, and generator identities. Keep the existing unsafe diagnostic and rejected status. The two actual cases cover single-row and duplicate-row forms.

### Conflicting generator policy for one raw identity

For the exact same raw `contractId: "generator"` and `rootPath: "generated"`, the supplied inclusion values `tracked` and `ignored` currently yield `status: "complete"`. The projector retains both tuples and derives the ignored-generator origin without diagnosing the contradictory declaration.

Approved rule: reject **once per exact raw contract/root identity** while retaining both distinct tuples and provenance. The neutral expectation uses `contradictory-generator-output` at the root path with message `One generator contract/root identity declares conflicting inclusion policies`. Distinct contract IDs and physically distinct Unicode root strings are not this conflict.

The current single-identity case exercises one diagnostic; it does not independently establish a many-identity diagnostic-count law.

## Preserved behavior actually exercised

The nine passing cases confirm these finite examples:

- Null scope with an empty input produces a schema-valid complete empty result.
- Identical generator tuples deduplicate; different contract IDs remain distinct authorities.
- Raw NFC/NFD source spellings remain separate physical observations even when NFC scope matching selects both.
- Both Unicode physical observations retain all four origins in canonical order: tracked, nonignored-untracked, ignored-generator, explicit-ticket.
- Exact independent index identities stay attached to the corresponding raw source paths.
- Raw NFC/NFD generator roots remain distinct tuples even when normalized matching relates both to a candidate.
- Contradictory stage-zero identities and stages 0–3 survive rejected admission rather than being silently chosen or discarded.
- Physical file/link disagreement produces unobserved physical fields while preserving all provenance.
- A safe symlink leaf remains an observed link with mode 120000.

The two malformed candidates are rejected by actual Ajv and the actual projector: a string instead of a boolean `unsafeAncestor`, and an unsupported `observedKind: "device"`.

Full current roster:

| Neutral projection | Actual result |
|---|---|
| `empty-null-scope-is-a-valid-empty-result` | PASS |
| `empty-string-scope-keeps-result-schema-valid` | RED |
| `empty-string-scope-with-candidate-keeps-result-schema-valid` | RED |
| `empty-cancellation-phase-still-rejects` | RED |
| `unsafe-ancestor-cannot-claim-observed-file` | RED |
| `duplicate-safe-and-unsafe-physical-claims-do-not-select-file` | RED |
| `same-generator-contract-root-conflicting-inclusion-rejects` | RED |
| `identical-generator-tuples-merge-once` | PASS |
| `different-generator-contracts-preserve-distinct-authorities` | PASS |
| `physically-distinct-unicode-generator-roots-remain-distinct` | PASS |
| `unicode-physical-source-distinction-keeps-all-four-origins` | PASS |
| `contradictory-stage-zero-keeps-both-exact-index-identities` | PASS |
| `all-index-stages-and-all-origins-survive-rejection` | PASS |
| `physical-conflict-keeps-all-provenance-without-kind-selection` | PASS |
| `safe-symlink-leaf-remains-an-observed-link` | PASS |
| `invalid-traversal-scope-returns-rejection-not-exception` | RED |

## Exact pre-fix fingerprints

All first/final pairs below are from the approved-expectation RED capture and are equal.

| Input | First SHA256 | Final |
|---|---|---|
| [🧪️source-admission-51/🧪️adversarial/📜️script.ts](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/📜️script.ts>) | `88288b68e7bb7eada4e977dbcd7167feec8f9d013482ab96cd28cd36cfe3a949` | identical |
| [🧪️tests/🧪️source-admission/🟦️.ts](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🧪️source-admission/🟦️.ts>) | `66c88ad01dfa3941a6f41b99bb5197be28e08266b21cf2e95dd8574545747d79` | identical |
| [☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️source-admission-projection-51.md](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️source-admission-projection-51.md>) | `c6476c9c3cc5c2073713833e4c796a9ae89d3d40b8deef847026fbbbfd677b10` | identical |
| [🧪️source-admission-51/🧪️adversarial/🔣️vectors.json](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/🔣️vectors.json>) | `d5dd5193e2ee49947814aacf9a4a5b29ef1ed950390349a32278e6cb3f4ea0f8` | identical |
| [🧪️adversarial/🧬️schema/🔣️.json](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/🧪️adversarial/🧬️schema/🔣️.json>) | `604abc25fe0597c095850463e84c71c7ac5cf0a80e0ac6ebeb88748635b92434` | identical |
| [🧹️normalization/🧬️schema/🔣️.json](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🔣️.json>) | `ed117e588c2aa2e1a0622455ab4710cec623725dce829af6ddb4a5f6328bb1a5` | identical |
| [🧪️tests/🧪️source-admission/🔣️.json](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🧪️source-admission/🔣️.json>) | `ca1f61dd23fbfcf9077635b79308834b9be296ba9d6b8c90f0eba78675958656` | identical |
| [📚️library/🧹️normalization/🟦️.ts](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts>) | `342e780b71b6bd0fc9e6cc66b151e58fa9e78ecf0e149846ab297fe62659b0fe` | identical |

The runtime function-string fingerprint is `bcb408f7c1bee2f75412a3010ed6165e9411733e3754c5b337839907a48cee36`. This supplements, rather than replaces, the actual source slice hashes.

| Bound declaration | Pre-fix N lines | SHA256 |
|---|---:|---|
| `normalizeRelative` | 2122–2124 | `2901f7dad8d99b8801afc4294996c6dc232186332bb02e0922adf0ca5b70cf1b` |
| `sourceRelative` | 2126–2132 | `3eeddd8eb5631e8651725e6f27ba377d12fcf4cbb82c222876e5df20405a1c04` |
| `inScope` | 2172–2177 | `e213fb6813a71cfe1025bb7048fc7c7629651f1901bfe21d513327ab82a9fb2d` |
| `SOURCE_ADMISSION_ORIGINS` | 2401–2401 | `f8ad80a07e40ce1495e517588721a3b847138399782bcf8fbf0aa5d9bf63b0f1` |
| `sourceAdmissionByteCompare` | 2402–2402 | `a8ea553a484f18c49e1a07ebbc9d7825f6a323f661e43445d0192a77b7b3f6b1` |
| `sourceAdmissionSafePath` | 2404–2406 | `62dd9e376e3656cee2fe2bfae51f39174d15e9bebab7fbc5f1185338d089a9fd` |
| `sourceAdmissionOpaque` | 2408–2414 | `afdcf174a6931f5c4d6e51dc0a67fe8d5fe7f19d515b57f1fd9352e2de88b34b` |
| `sourceAdmissionRecord` | 2416–2418 | `be299c61fd5208e526774227071a278d4af1797d01a25d0977ed06dfe7e95cdc` |
| `sourceAdmissionInputShape` | 2420–2433 | `4f5cb5d528e2fc771be1fa5676e4d5b7dc2154e64affa14ab10b65729e576ee2` |
| `sourceAdmissionPhysicalConsistent` | 2435–2440 | `89828695ff549ec10220674921e3cae7a6bf5911056eccff5b27fc8f2b9ed844` |
| `projectTaxonomySourceAdmission` | 2443–2495 | `935e2f805a69739a28fd20d4decf496cdca8a3cf1e39f03b3f666596ebdf599a` |

## Released boundary

Root may repair the demonstrated pure cases in N and replay this unchanged approved vector set. Production edits, canonical test adoption, and any further IO readiness claim remain root-owned. This packet itself is complete as a reproducible pre-fix audit; it does not count the canonical18 baseline or root's separate actualIO28 as exhaustive projector coverage.

