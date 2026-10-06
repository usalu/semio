# Repo Library Asset And README Authority Audit — 2026-10-06

Read-only semantic audit of the five current library assets JSON files and three README schemas. No runtime tests executed. Root owns testing-readme-coordinates closure.

The four other asset JSON files are runtime authority catalogs, not exclusively plain test examples. Preserve their runtime meaning. Runtime discovery and normalization actively read these exact files to authorize normalization transactions.

- `🖼️assets/⚖️readme-license-owner-authority/🔣️.json`: actual frozen owner/license source catalog. Discovery `🟦️.ts` line6055 validates cohort and walks catalog cases; normalization resolves exact-owner source and destination preimages and source authority. Test coordinate frozenAuthority references its digest; a path move requires reference/hash coordination, and calling it test-only would destroy actual transaction authority.
- `🖼️assets/📽️nested-cargo-package-projection/🔣️.json`: actual exact package projection catalog (source/destination/manifests/content hashes/retirement authority), consumed by discovery semanticPackageProjectionContracts line4446 and normalization lines3462,5436,5724,7112. Preserve.
- `🖼️assets/🚨️transaction-sentinel-cases/🔣️.json`: actual serialized sentinel authority. Normalization constant380, loader5782, inventory freezing5802, removal authorization5816, repeat authority check9286. Although row names use cases/expectedViolationCode, those rows are retained transaction authority; preserve.
- `🖼️assets/💉️ticket-important-exact-mutations/🔣️.json`: actual exact move/removal source preimage catalog, constant381 and loader4480 in normalization. Preserve.
- `🖼️assets/🗺️testing-readme-coordinates/🔣️.json`: pure fixed README/source/parser expectation corpus, only confirmed test reader, root is closing path to fixtures and removing corpus admission.

## README Schemas

`🧬️schema/🚚️readme-move-source-authority/🔣️.json` root describes a real TaxonomyMove with operationId, sourcePath, destinationPath, sourcePreimage, rationaleRule, ownerId, referenceEdits and sourceAuthority. Actual production normalization types at75–76 and parser547 preserve `exact-owner-current-source-revision-v1` authority. Keep root and real sourceAuthority/filePreimage definitions. The test reads this schema at `🧪️tests/🚚️readme-move-source-authority/🟦️.ts` line14, but validating individual generated moves is legitimate; it is not whole-corpus admission. Incidental `definitions.execution` and `strictCompilation` are fixed testing route/output declaration shapes and can be removed if no genuine per-value report consumers need them.

`🧬️schema/🔖️readme-current-source-revision/🔣️.json` root describes real frozen currentSourceRevisions authority keyed by testing-readme-protocol-v2-reviewed. Production normalization4338–4364 resolves it against original catalog bytes and transaction source authority; discovery4463–4464 parses it and validates expectation path policy. Keep authority root despite fixed revision key/case index. Incidental `definitions.execution` and `fixtureInputs` name exact test route/manifest/hash and are test setup shapes; remove only these if no actual authority consumer requires them. Its test reads schema at `🧪️tests/🔖️readme-current-source-revision/🟦️.ts` line21.

`🧬️schema/🗺️testing-readme-coordinates/🔣️.json` root is entirely a fixed test corpus. Root agent owns removal.

Both README move/current revision tests retain TypeScript/Bun compilation comparisons, JSONC parser checks, SHA/source assertions and actual semantic domain checks. They must not be dropped with incidental test setup schema definitions.

Exact semantic conclusions sent to runtime owner and root. No source edits made.
