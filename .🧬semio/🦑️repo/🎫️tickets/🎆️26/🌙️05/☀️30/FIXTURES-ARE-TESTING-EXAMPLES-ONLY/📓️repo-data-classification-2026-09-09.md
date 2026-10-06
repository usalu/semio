# Repo Test Data Classification

Per-file structure inspection before relocation. Schema candidates define validation contracts; the remaining files require owner/consumer validation as examples or build-support inputs.

```json
[
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/📦️package/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "epoch",
      "timezone",
      "time",
      "date",
      "files",
      "expectedFiles",
      "expectedZip"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/🧩️host-build/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "define",
      "externals",
      "source",
      "library",
      "expected"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🧪️abstraction-ownership/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "artifactSchemas",
      "cases",
      "schemaCases",
      "sourceCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🪪️field-parity/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "cases",
      "parity",
      "discovery"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cohortId",
      "opaquePrefixesNotTraversed",
      "sourceCensus",
      "semanticDirectoryPrecedents",
      "fixedContracts",
      "projectionContracts",
      "ownerEvidence",
      "referenceOwners",
      "generatorOwners",
      "knownPublisherBlockers",
      "publisherVerifications",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/✂️source-parent-prune-journal/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "scope",
      "files",
      "expectedPrunePaths",
      "retainedParents",
      "invalidPrunePaths"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/❄️frozen-coordinate-evidence/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "contractId",
      "registration",
      "coordinateCounts",
      "absoluteCases",
      "versionCases",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "owner",
      "subset",
      "ownerReadiness",
      "execution",
      "sourceInputs",
      "cases",
      "payloadAuthority",
      "payloadDescriptor",
      "documentationCases",
      "negativeCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🎫️ticket-document-owner-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "ownerPath",
      "destinationFilename",
      "preflightProgress",
      "scanProgress",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🎯️scoped-inventory-pathspec/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "opaqueExclusions",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🎲️transaction-disposition-outcomes/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "expectedDispositionOperations",
      "affectedStateCases",
      "negativeDispositionCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🏭️generator-preview/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "contractId",
      "nodes",
      "schemaVersion",
      "staleRemovals"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/💉️ticket-important-exact-mutations/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/💎️nested-cargo-package-purity/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "authority",
      "decisionState",
      "pathBudgetBytes",
      "protectedOpaqueRoots",
      "baseStructuralGolden",
      "counts",
      "roots",
      "roleAuthority",
      "mappingFields",
      "mappings",
      "retainedStructuralLeaves",
      "adapters",
      "manifestEdits",
      "schemaPrerequisites"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📈️taxonomy-cli-progress/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "functionName",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "mappingDigestAlgorithm",
      "referencePreimageHashAlgorithm",
      "referenceConsumers",
      "projections"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "boundaries",
      "transactionLedgers",
      "workspaceLedgers"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📦️extension-installation-owner/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schema",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📦️extension-installation-owner/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📽️nested-cargo-package-projection/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractKind",
      "packages",
      "referenceConsumers",
      "referenceTokenTransforms"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔀️generator-input-transitions/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "scope",
      "files",
      "source",
      "destination",
      "consumer",
      "prospectiveInputPaths",
      "movedInputReference",
      "failureStages",
      "absentOutputParent",
      "dynamicInputs",
      "tamperCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔍️discovery-schema-handoff/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "module",
      "fullSchemaExpression",
      "consumers"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🕰️ticket-important-history-owner-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🕸️scoped-incoming-reference-closure/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "scope",
      "source",
      "destination",
      "payload",
      "consumers",
      "unsupported",
      "symlink",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🗂️mutation-directory-classification/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "sourceMutationNames",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚏️router-import-boundary/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "entrypoint",
      "deferredModule",
      "catalogRoots",
      "consumers",
      "runtimeMarker"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "virtualPathPolicyCases",
      "symlinkFlavorCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚶️scoped-admission-walk/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "ticketRoot",
      "generatorRoots",
      "files",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🛑️taxonomy-cli-cancellation/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "sourcePath",
      "source",
      "cancelPath",
      "phase",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🛤️mutation-path-projection/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "standardDirectoryName",
      "subsetDirectoryName",
      "profileDirectoryName",
      "registryCounts",
      "sourceGlob",
      "bundle",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🤝️transaction-protocol/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "failureStages",
      "journalStates",
      "virtualPreimageNodes"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🦀️nested-cargo-package-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "mappingSemantics",
      "pathBudgetBytes",
      "protectedOpaqueRoots",
      "censusExcludedEvidenceRoots",
      "censusExcludedEvidencePaths",
      "packages",
      "referenceTokenTransforms",
      "liveReferenceSummary",
      "additionalReferenceEdits",
      "preservedNonReferences",
      "referenceConsumers"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧩️artifact-component-leaf-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "cases",
      "mutationPilot"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "joinArguments",
      "tokenNarrowing",
      "execution",
      "oracle",
      "transaction",
      "referenceUniverse",
      "manifestCandidates",
      "manifestPaths",
      "assertionMessages"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle/⚙️.toml",
    "kind": "data-candidate",
    "keys": [
      "nonjson"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🧬️join-provenance/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "authority",
      "decisionState",
      "observedAt",
      "pathBudgetBytes",
      "protectedOpaqueRoots",
      "census",
      "sourceByteDrift",
      "exclusions",
      "taxonomy",
      "mappingFields",
      "mappings",
      "collisions",
      "schemaPrerequisites",
      "referenceAuthority"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "launcherPath",
      "testPath",
      "absoluteBundleExpression",
      "runIdExpression",
      "runIdEntropyCall",
      "bundleRootFunction",
      "runOwnerPath",
      "runRootPrefix",
      "bundleDirectory",
      "runRootEnvironment",
      "runRootValidator",
      "runIdPattern",
      "invalidRunIds",
      "rejections"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🪪️ticket-important-owner-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🫙️artifact-empty-facet-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "sourceRoot",
      "sourceFilename",
      "destinationFilename",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "productionPath",
      "testPath",
      "production",
      "test",
      "law"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "contract",
      "files",
      "expected",
      "result",
      "rejected",
      "generatedImport"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "semantics",
      "cases",
      "oracle",
      "retention",
      "registration",
      "attributeCompilerCases",
      "scope"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "defaults",
      "processes"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "factory",
      "rounds",
      "uniqueNormalizedPatterns",
      "normalization",
      "integration",
      "oracle",
      "cases",
      "invalidPatterns"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "command",
      "target",
      "source",
      "runner",
      "budget",
      "launchName",
      "launchGroup",
      "launchOrder"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚙️root-script-compiler/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "declarations",
      "fieldNames",
      "schemaFields",
      "eagerVocabulary",
      "fixtureRetention",
      "glue",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "properties",
      "additionalProperties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "mutationRoot",
      "leaf",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "semantics",
      "cases",
      "batch",
      "spanIntegrity",
      "registration",
      "retention"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "semantics",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🧬️energy-source-coordinates/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "documents"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "selection",
      "fallback",
      "cases",
      "invalid",
      "liveRegression",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🧪️imported-data/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "selection",
      "dataGrammar",
      "fallback",
      "cases",
      "graph"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🧪️imported-data/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌱️mutation-root-discovery/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "ownedRoots",
      "physicalOwners",
      "includedRoots",
      "excludedRoots",
      "filePaths",
      "rejections"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌱️mutation-root-discovery/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "additionalProperties",
      "definitions",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "properties",
      "additionalProperties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "canonicalLocator",
      "childLocator",
      "cases",
      "startCases",
      "anchorCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "execution",
      "nonRustAdapters",
      "tokens",
      "rustCases",
      "nonRustSupported",
      "nonRustUnsupported",
      "scale"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎯️cargo-target-discovery-skip/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "taxonomyDirectoryKind",
      "skipped",
      "admitted",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🎮️command-vectors.json",
    "kind": "data-candidate",
    "keys": [
      "schema",
      "cases",
      "rejected"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🗺️artifact-location.json",
    "kind": "data-candidate",
    "keys": [
      "schema",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "properties",
      "additionalProperties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "mutationRoot",
      "name",
      "attributedAggregate",
      "malformedAggregate",
      "ambiguousAggregate",
      "wrongMountAggregate",
      "privateMountAggregate",
      "wrongVariantAggregate",
      "scopedAggregate",
      "unrelatedDocAggregate",
      "nestedAggregateDecoy",
      "nestedAggregateScopes",
      "unmatchedAggregate"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️owned-generator-preview-inventory/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "previewLimits",
      "compilerInputAuthorities",
      "compilerManifest",
      "previewRoutes",
      "ownedGeneratorIds"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️owned-generator-preview-inventory/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🛂️schema/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "type",
      "required",
      "additionalProperties",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "authority",
      "execution",
      "historicalDocument",
      "sourcePreimage",
      "identityCases",
      "historicalCoordinates",
      "fixture"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🧬️energy-source-coordinates/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "scope",
      "counts",
      "documents",
      "undeclaredNeighbors",
      "duplicateValueCase",
      "negativeCases",
      "coordinateValueEncoding"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "contract",
      "layout",
      "module",
      "cases",
      "expectedPackages",
      "expectedTests"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "execution",
      "fixtureAuthority",
      "fixtureAuthoritySha256",
      "runParent",
      "isolation",
      "copies",
      "inputCases",
      "repeatedCapture",
      "childSafety",
      "directoryCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🎯️reviewed-expectations/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "execution",
      "documents",
      "frozenAuthority",
      "canonicalTree",
      "kindLeaves",
      "inlineCoordinates",
      "registrySchema",
      "paragraphs",
      "preservedMarkdown",
      "rejectedMarkdown",
      "parserCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/📥️reviewed-source/📝️.md",
    "kind": "data-candidate",
    "keys": [
      "nonjson"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "catalog",
      "revision",
      "provenance",
      "inputs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💠️inventory-artifact-shards/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "violationSchema",
      "cases",
      "orderedOwners",
      "orderedCoordinateHistory",
      "rejections",
      "artifactRejections",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💥️nested-cargo-collision-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "fixture",
      "caseSchema",
      "cases",
      "membershipCases",
      "performance",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "semantics",
      "registration",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎫️ticket-role-routing/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎫️ticket-role-routing/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "$id",
      "type",
      "required",
      "properties",
      "additionalProperties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎭️source-roster-roles/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "roles",
      "validInventory",
      "unknownRole",
      "extraRosterField"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎭️source-roster-roles/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "additionalProperties",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/📸️source-index-capture/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "expectedRoots",
      "existingEvidence",
      "cases",
      "unknownPath",
      "cancelFile",
      "cancelProbePath"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/📸️source-index-capture/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "additionalProperties",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "properties",
      "additionalProperties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "baseline",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧫️fixtures/🧪️consumers/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "assignmentLedger",
      "files"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "additionalProperties",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "authority",
      "observation",
      "foldVectors",
      "ancestorSwap",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🧪️registration/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "command",
      "target",
      "source",
      "runner",
      "budget",
      "launchName",
      "launchGroup",
      "launchOrder"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🧪️registration/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🛂️schema/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "type",
      "required",
      "properties",
      "additionalProperties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/💥️malformed/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/💥️malformed/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "additionalProperties",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🚫️unsupported/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🚫️unsupported/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "required",
      "additionalProperties",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "required",
      "additionalProperties",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📥️mutation-input-carriers/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "mutationRoot",
      "aggregate",
      "compilerPrelude",
      "compilerAssertion",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️semantic-package-source-manifest-identity/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "scope",
      "api",
      "row",
      "facts",
      "cases",
      "phases",
      "syntaxGaps"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️semantic-package-source-manifest-identity/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "accepted",
      "rejected"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🛂️schema/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "type",
      "required",
      "additionalProperties",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔍️filesystem/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "$schema",
      "schemaVersion",
      "directories",
      "files",
      "symlinks",
      "expected"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/📍️projection-scope/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "activeKeys",
      "movingTargets",
      "semantics",
      "oracle",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "semantics",
      "cases",
      "projections",
      "corpus",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "mutationCatalogSourceOwnership",
      "gltfFixtureCoordinates",
      "assetLogoKeyframes",
      "assetDocumentation",
      "assetIconPaths",
      "taxonomyEmojiIdentities",
      "selectorFreeZwjOwners",
      "mutationPayloadOwnership",
      "mutationDomainContract",
      "projectionOracleDirectories",
      "subsetDirectoryOverrides",
      "semanticManifestFilenameOverrides",
      "projectionScenarios",
      "genericEmojiIdentities"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔐️mutation-codec-ownership/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "mutationRoot",
      "aggregate",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "fixtureInputs",
      "execution",
      "revisionId",
      "catalogPath",
      "catalogSha256",
      "expectedRevisionDigest",
      "revisions",
      "strictCompilation",
      "selectionBoundaries",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "definitions",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "grammar",
      "extraction",
      "oracle",
      "limits",
      "cases",
      "cacheSequence",
      "stress"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "helper",
      "segmenter",
      "semantics",
      "rounds",
      "oracle",
      "cases",
      "scalingCases",
      "oracleDivergences"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "command",
      "target",
      "source",
      "runner",
      "budget",
      "launchName",
      "launchGroup",
      "launchOrder"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🧪️registration/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔭️mutation-scope/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "mutationRoot",
      "aggregate",
      "acceptedSelections",
      "missingSelections",
      "binaryCases",
      "codecCases",
      "aggregateCases",
      "rejectedSelections"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "semantics",
      "execution",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🧬️energy-source-coordinates/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "id",
      "contract",
      "size",
      "mode",
      "rootEntries",
      "coordinate",
      "originalContracts"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "directoryName",
      "filename",
      "fileMode",
      "writePolicy",
      "nodeCases",
      "invalidContracts",
      "progress",
      "singleCase",
      "unsignedPlan"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/📨️submitted-proof/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "producerContext",
      "launchSeed",
      "catalogContext",
      "cargoModuleRoot",
      "cadConsumerMount",
      "owner",
      "members",
      "consumers",
      "retention",
      "oracle"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🙈️residue-ignore/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "rootIgnore",
      "residueSegments",
      "leaf",
      "content",
      "lowerPrioritySource",
      "authoritativeSource",
      "writePolicy",
      "beforeIgnored",
      "absentIgnored",
      "presentIgnored",
      "siblingIgnored"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🙈️residue-ignore/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛟️residue-recovery/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "childExitCode",
      "foreignInput",
      "generatorStarts",
      "inverseState",
      "preservation",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛟️residue-recovery/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛣️commit-route/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "command",
      "target",
      "budgetMs",
      "bunTimeoutMs",
      "testName",
      "runner",
      "extraArguments",
      "launchName",
      "launchOrder"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛣️commit-route/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛫️preflight-context/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "ticketDir",
      "transactionDirectory",
      "planPath",
      "sourcePath",
      "foreignPath",
      "unrelatedPath",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛫️preflight-context/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "execution",
      "documents",
      "frozenAuthority",
      "canonicalTree",
      "kindLeaves",
      "inlineCoordinates",
      "registrySchema",
      "paragraphs",
      "preservedMarkdown",
      "rejectedMarkdown",
      "parserCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "scope",
      "strictCompilation",
      "execution",
      "semantics",
      "revision",
      "source",
      "inputs",
      "canonicalArrayKeys",
      "cases",
      "phaseCases",
      "forwardBoundary",
      "forwardOrderCases",
      "resumeErrorCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "definitions",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚧️cargo-discovery-exclusions/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "opaquePaths",
      "virtualRoots",
      "traversal",
      "symlinks",
      "manifests",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛟️transaction-recovery-authority/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "registration",
      "semantics",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛤️typescript-path-collection/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "semantics",
      "scanBoundarySchema",
      "scanBoundaryCases",
      "caseSchema",
      "cases",
      "liveInput",
      "mapBoundaryCases",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛫️preflight-reference-basis/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "semantics",
      "fixture",
      "queries",
      "mutations",
      "markerBytesCases",
      "physicalCases",
      "execution",
      "caseSchema"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "scope",
      "execution",
      "contractId",
      "revisionId",
      "revisionInput",
      "revisionInputSha256",
      "taxonomyPath",
      "catalogPath",
      "catalogSha256",
      "catalogCaseIndex",
      "baseline",
      "expectedRevisionDigest",
      "shippedPublication"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "ownerPath",
      "packagePath",
      "sourcePath",
      "testPath",
      "fixturePath",
      "member",
      "packageFiles",
      "cargo",
      "project",
      "wasmTargets",
      "imports",
      "rejectedEntryPaths",
      "launch"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "semantics",
      "oracleCompilation",
      "languages",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "ownerPath",
      "packagePath",
      "packageFiles",
      "cargo",
      "members",
      "admission",
      "abi",
      "project",
      "imports",
      "nativeCases",
      "checks",
      "rejectedSegments",
      "internalDependencies"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🖥️ui-host-package/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "cases",
      "corpus"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "semantics",
      "cases",
      "correlated",
      "writable",
      "registration",
      "retention"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🧪️fixture/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schema",
      "compilerStorage",
      "stageEnvironment",
      "package",
      "target",
      "laws",
      "executableBytesHex",
      "executableSha256",
      "nativeArguments",
      "capturedOutput",
      "activeLease",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "mutationRoot",
      "leaf",
      "rustFilename",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🛂️schema/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "type",
      "required",
      "additionalProperties",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-rust-entries/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "$id",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-rust-entries/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "contract",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-scope-catalog/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "$id",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "contract",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🧫️fixture/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "version",
      "ticketsRoot",
      "tickets",
      "special",
      "cases",
      "reopened"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️registry-catalog-gitlink-boundary/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contract",
      "virtualRoot",
      "boundary",
      "ancestorFile",
      "siblingFile",
      "nestedFiles",
      "execution"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧫️fixture/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "version",
      "checkout",
      "components"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧫️fixture/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "accepted",
      "rejected",
      "traces"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🛂️schema/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "type",
      "required",
      "additionalProperties",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🛂️schema/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "type",
      "required",
      "additionalProperties",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🧫️fixtures/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "source",
      "expected"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "command",
      "target",
      "source",
      "runner",
      "budget",
      "launchName",
      "launchGroup",
      "launchOrder"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📋️registration/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "readBytes",
      "readChunkBytes",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📨️request/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "sourceContractId",
      "subsetSegments",
      "leaf",
      "customContent",
      "customMode",
      "directoryMode",
      "surfaceLayout",
      "subsetLayout",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/☑️options.json",
    "kind": "data-candidate",
    "keys": [
      "contractId",
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🛂️schema/☑️options.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "title",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "definitions",
      "oneOf"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧪️registration/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "command",
      "target",
      "source",
      "runner",
      "budget",
      "launchName",
      "launchGroup",
      "launchOrder"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧪️registration/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/💥️generic-stem-collision-resolution/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "siblingCases",
      "gluePurityCases",
      "packageBoundaryHoistCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "glueRoleCases",
      "scopeSpecificityOrder"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "cases",
      "schemaRejections",
      "schemaCases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "cases"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🛂️schema/🔣️.json",
    "kind": "schema",
    "keys": [
      "$schema",
      "type",
      "additionalProperties",
      "required",
      "properties",
      "$defs"
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🔣️.json",
    "kind": "data-candidate",
    "keys": [
      "schemaVersion",
      "contractId",
      "scope",
      "darwinLayout",
      "windowsLayout",
      "observations",
      "decisionCases",
      "darwinFailures",
      "linuxStat",
      "linuxFailures",
      "windowsFiletime",
      "roles",
      "orderingCases",
      "nativeProbe"
    ]
  }
]
```
