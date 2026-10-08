# Current 3539 Full Stdout Independent Review

Independent complete raw stdout parse and physical catalog-source binding:

```json
{
  "bytes": 4992603,
  "sha256": "d5793846ef50e92d87be0900a5ab4f3095689460fca5319e4fcd7c3780b3f48d",
  "validRows": 11236,
  "malformed": [],
  "footer": [
    "[schema check] modules=4365 scopes=3539 findings=11236",
    "[schema check] schema-dialect-not-draft-07=81",
    "[schema check] schema-document-id-duplicate=366",
    "[schema check] schema-document-id-unaddressable=119",
    "[schema check] schema-export-id-duplicate=283",
    "[schema check] schema-export-id-invalid=255",
    "[schema check] schema-export-incomplete=8334",
    "[schema check] schema-module-id-inconsistent=10",
    "[schema check] schema-module-id-missing=223",
    "[schema check] schema-mutation-aggregate-id=6",
    "[schema check] schema-mutation-leaf-id=75",
    "[schema check] schema-owner-ineligible=518",
    "[schema check] schema-placement-forbidden-filename=3",
    "[schema check] schema-ref-unresolved=816",
    "[schema check] schema-scope-ambiguous=147",
    "[schema check] shared-code-table=46 unshared-codes=0"
  ],
  "fixtureCorpusCodes": {},
  "catalogCodes": {},
  "catalogSHA256": "73a0a4f2941506470b2a1a22dd1581ac178c29cc2e9f6755769898a8a000b0b3",
  "bindings": [
    {
      "scope": "hub.bootstrap",
      "exports": [
        "ActorProducerV1",
        "GenerationProducerV1",
        "ObservationClaimV1",
        "PackageProducerV1",
        "PublicationProducerV1"
      ],
      "source": "🌎️hub/🏗️bootstrap/🧾️provenance/🧬️schema/🔣️.json",
      "sha256": "33c6f93b3db0d917c0c5c4453ce9cc86c43ce50ca87752f3b5c288906037b557",
      "catalogHash": "33c6f93b3db0d917c0c5c4453ce9cc86c43ce50ca87752f3b5c288906037b557",
      "match": true
    },
    {
      "scope": "os.dev.distribution",
      "exports": [
        "DistributionSourceCoordinate"
      ],
      "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🧬️schema/🔣️.json",
      "sha256": "3ec59278818573a28e996993870ea63c65d78bf9be6e70c8cbd950a493646b40",
      "catalogHash": "3ec59278818573a28e996993870ea63c65d78bf9be6e70c8cbd950a493646b40",
      "match": true
    },
    {
      "scope": "os.plugin.browser-bundle",
      "exports": [
        "BrowserActorPhysicalClaimV1",
        "BrowserActorPhysicalInputV1",
        "BrowserActorProducerInputsV1"
      ],
      "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧬️schema/🔣️.json",
      "sha256": "b3294ad2900d51fb92dc06827532fe5b014b51702f22b66839d50ab2369bf7ad",
      "catalogHash": "b3294ad2900d51fb92dc06827532fe5b014b51702f22b66839d50ab2369bf7ad",
      "match": true
    },
    {
      "scope": "repo.discovery.runtime",
      "exports": [
        "RuntimeActorCargoSelectionV1"
      ],
      "source": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧬️schema/🔣️.json",
      "sha256": "3690533367c57d80211db7b8541a7b48a15911313b7180d64a92fda34a166806",
      "catalogHash": "3690533367c57d80211db7b8541a7b48a15911313b7180d64a92fda34a166806",
      "match": true
    }
  ]
}
```

Rows equal footer11236;0 malformed; all selected current genuine JSON leaf/coordinate/provenance scopes match physical source hashes. Nx1/53.5s represents broader schema debt, not a global schema pass. Fixture/corpus and catalog diagnostic codes are zero in this completed snapshot. The separate permanent gate failed at Cargo owner preparation before checking schemas and still requires actual repaired execution. No compiler, test, metadata, derive, HTTP or lifecycle was executed in this review.
