# Final Current Source Full Stdout Independent Audit

Independently parsed the completed log: 4993062 bytes, SHA256 `79dfc5fc52ba8f39aa139dab18b12f559934eabde5902e642ac5a1971e432407`, 11238 valid JSON diagnostic rows and zero malformed JSON rows. Footer:

- [schema check] modules=4365 scopes=3538 findings=11238
- [schema check] schema-dialect-not-draft-07=81
- [schema check] schema-document-id-duplicate=366
- [schema check] schema-document-id-unaddressable=120
- [schema check] schema-export-id-duplicate=283
- [schema check] schema-export-id-invalid=255
- [schema check] schema-export-incomplete=8334
- [schema check] schema-module-id-inconsistent=10
- [schema check] schema-module-id-missing=224
- [schema check] schema-mutation-aggregate-id=6
- [schema check] schema-mutation-leaf-id=75
- [schema check] schema-owner-ineligible=518
- [schema check] schema-placement-forbidden-filename=3
- [schema check] schema-ref-unresolved=816
- [schema check] schema-scope-ambiguous=147
- [schema check] shared-code-table=46 unshared-codes=0

Zero fixture-specific and zero catalog-specific diagnostic codes occur. Nx exited 1 in22.3s: the11238 remaining diagnostics are broader schema/taxonomy debt; this is not a global schema pass. Generation17770 and docs45843 success/counts are parent-owned separate receipts.

Current catalog SHA256: `b9c4e2a15346acc12f2f5cacc67ee3e43f9e3e0ab131c2ddd22bd844bc296c84`. Exact source-backed leaf/coordinate/provenance scopes and canonical file hashes:

```json
[
  {
    "scope": "hub.bootstrap",
    "path": "🌎️hub/🏗️bootstrap/🧾️provenance/🧬️schema/🔣️.json",
    "exports": [
      "ActorProducerV1",
      "GenerationProducerV1",
      "ObservationClaimV1",
      "PackageProducerV1",
      "PublicationProducerV1"
    ],
    "expected": "33c6f93b3db0d917c0c5c4453ce9cc86c43ce50ca87752f3b5c288906037b557",
    "actual": "33c6f93b3db0d917c0c5c4453ce9cc86c43ce50ca87752f3b5c288906037b557",
    "match": true
  },
  {
    "scope": "os.plugin.browser-bundle",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧬️schema/🔣️.json",
    "exports": [
      "BrowserActorPhysicalClaimV1",
      "BrowserActorPhysicalInputV1",
      "BrowserActorProducerInputsV1"
    ],
    "expected": "b3294ad2900d51fb92dc06827532fe5b014b51702f22b66839d50ab2369bf7ad",
    "actual": "b3294ad2900d51fb92dc06827532fe5b014b51702f22b66839d50ab2369bf7ad",
    "match": true
  },
  {
    "scope": "repo.discovery.runtime",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧬️schema/🔣️.json",
    "exports": [
      "RuntimeActorCargoSelectionV1"
    ],
    "expected": "3690533367c57d80211db7b8541a7b48a15911313b7180d64a92fda34a166806",
    "actual": "3690533367c57d80211db7b8541a7b48a15911313b7180d64a92fda34a166806",
    "match": true
  }
]
```

All identified scope JSON hashes match current physical canonical sources. Original requested paths independently checked:

```json
[
  {
    "path": "🌎️hub/🧩️compositions/🗄️stdio/🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧬️schema/🔣️.json",
    "exists": false,
    "exactCatalogReferences": 0
  },
  {
    "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🚦️public/🧬️schema/🔣️.json",
    "exists": false,
    "exactCatalogReferences": 0
  },
  {
    "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🎛️control/🧬️schema/🔣️.json",
    "exists": false,
    "exactCatalogReferences": 0
  }
]
```

No compiler, HTTP, lifecycle, or schema regeneration was executed in this audit. This final derived source snapshot is separate from pending actual production compiler/publication/default runtime proof.

Dev DistributionSourceCoordinate is a definition of the distribution schema, not a separately named catalog export in this snapshot. Exact schema facet custody:

```json
[]
```

A bounded literal scan of all catalog scope records contains no `🚚️distribution` coordinate. Full diagnostic rows mentioning that owner:

```json
[
  {
    "code": "schema-module-id-missing",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/⚙️inputs/🧬️schema",
    "detail": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/⚙️inputs/🧬️schema/🔣️.json must declare the $id that names this module's scope."
  },
  {
    "code": "schema-document-id-unaddressable",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/⚙️inputs/🧬️schema/🔣️.json",
    "detail": "$id \"https://semio.tech/os/dev/distribution/build-inputs/v1\" does not resolve to a dotted scope id under https://json.schemas.assets.semio-tech.com/."
  },
  {
    "code": "schema-module-id-missing",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧬️schema",
    "detail": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧬️schema/🔣️.json must declare the $id that names this module's scope."
  },
  {
    "code": "schema-document-id-unaddressable",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧬️schema/🔣️.json",
    "detail": "$id \"https://semio.tech/os/dev/distribution/browser-components/v1\" does not resolve to a dotted scope id under https://json.schemas.assets.semio-tech.com/."
  },
  {
    "code": "schema-module-id-missing",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🧬️schema",
    "detail": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🧬️schema/🔣️.json must declare the $id that names this module's scope."
  },
  {
    "code": "schema-document-id-unaddressable",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🧬️schema/🔣️.json",
    "detail": "$id \"https://semio.tech/schema/os/dev/distribution/component.json\" does not resolve to a dotted scope id under https://json.schemas.assets.semio-tech.com/."
  }
]
```
The coordinate schema cannot currently be claimed catalog-hash-bound by this audit. This is separate from the observed zero fixture/catalog diagnostic codes.


## Subsequent Owned Dev ID Correction

After this full stdout checkpoint, the existing genuine distribution schema ID changed to `https://json.schemas.assets.semio-tech.com/os/dev/distribution/component.json`. Current plain source examples add illustrative contract metadata (id/scopeId/leaf). A test-owned law checks the actual taxonomy parser against `os.dev.distribution`, independently checks URL host/path grammar, and resolves the genuine DistributionSourceCoordinate Ajv leaf before admitting an ordinary permitted coordinate. No separate fixture schema, aggregate Ajv compilation, domain alias or copied corpus authority was introduced. Existing per-coordinate production contract remains genuine.

This corrects the identified addressability seam in source. The prior complete stdout/catalog remains historical before this one ID change; subsequent actual derive/fullcheck is required to prove catalog eligibility/hash binding. Parent actual RED42581 reports owned-parser null (3pass/1fail/266expects); GREEN32934 was pending when reviewed. No tests or generation were executed by this auditor.
