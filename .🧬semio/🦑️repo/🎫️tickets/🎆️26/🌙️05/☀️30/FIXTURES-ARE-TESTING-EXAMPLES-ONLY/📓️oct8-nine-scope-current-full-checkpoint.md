# Nine Scope Current Full Checkpoint

Schema checkpoint settled: False. Full broad check exits Nx1; no global pass is asserted. Normal permanent gate and current WGPU consumer regression repair remain pending.

| Receipt | Bytes | SHA-256 | Terminal |
| --- | ---: | --- | --- |
| gen | 751 | 81493e1aa475af676950683a70c9ac566315ef0a60f2ca77a12ad024c5d75998 | NX   Successfully ran target root-schema-generate for project ticket-fixture-verification; Run duration:      59.1s |
| docs | 492 | 4821ee85af26047221bb416787a457b72b8778fcf21d0bb74e1f996397155a5e | NX   Successfully ran target root-schema-docs for project ticket-fixture-verification; Run duration:      29.4s |
| full | 5451176 | 30b3429e30dd4817532c30685db96e6283eb6c4469d5397c019fb332253eb353 | NX   Running target root-schema-check for project ticket-fixture-verification failed; Run duration:      28.7s |

Every raw row was parsed: 12349 valid JSON/0 malformed; fixture/corpus=0, catalog=1, owned nine scopes=0. Catalog scopes=3643, SHA-256 fbfe2a3d076738df4165e93876c5a5b2330e4156250b6f9732efeae60823b087. All physical format file hashes and exact exports agree for nine scopes: True.

```text
[schema check] modules=4497 scopes=3643 findings=12349
[schema check] schema-catalog-stale=1
[schema check] schema-dialect-not-draft-07=83
[schema check] schema-document-id-duplicate=366
[schema check] schema-document-id-unaddressable=134
[schema check] schema-export-id-duplicate=284
[schema check] schema-export-id-invalid=255
[schema check] schema-export-incomplete=9386
[schema check] schema-module-id-inconsistent=10
[schema check] schema-module-id-missing=262
[schema check] schema-mutation-aggregate-id=6
[schema check] schema-mutation-leaf-id=75
[schema check] schema-owner-ineligible=530
[schema check] schema-placement-forbidden-filename=3
[schema check] schema-ref-unresolved=807
[schema check] schema-scope-ambiguous=147
[schema check] shared-code-table=46 unshared-codes=0
 NX   Running target root-schema-check for project ticket-fixture-verification failed
  Run duration:      28.7s
```

Broader counters: {"schema-catalog-stale": 1, "schema-dialect-not-draft-07": 83, "schema-document-id-duplicate": 366, "schema-document-id-unaddressable": 134, "schema-export-id-duplicate": 284, "schema-export-id-invalid": 255, "schema-export-incomplete": 9386, "schema-module-id-inconsistent": 10, "schema-module-id-missing": 262, "schema-mutation-aggregate-id": 6, "schema-mutation-leaf-id": 75, "schema-owner-ineligible": 530, "schema-placement-forbidden-filename": 3, "schema-ref-unresolved": 807, "schema-scope-ambiguous": 147}.

| Scope | Exports | Current physical formats | Matches |
| --- | ---: | --- | --- |
| hub.bootstrap | 7 | 🔣️.json 33c6f93b3db0d917c0c5c4453ce9cc86c43ce50ca87752f3b5c288906037b557 | True |
| repo.discovery.runtime | 23 | 🔣️.json 3690533367c57d80211db7b8541a7b48a15911313b7180d64a92fda34a166806 | True |
| os.plugin.browser-bundle | 23 | 🔣️.json c32c7b89824f61ad80940311cc1c37be25ee3c7e5127dc9a01af4e2aa26d5aa0 | True |
| os.dev.distribution | 10 | 🔣️.json 3ec59278818573a28e996993870ea63c65d78bf9be6e70c8cbd950a493646b40 | True |
| os.dev.verification.publication | 6 | 🔣️.json efd12568470ab21f04ff4550066578a651eb5f8b7284041290c61dd2ef901573 | True |
| repo.library.workspaces.cargo.preparation.custody | 5 | 🔣️.json 6ed52b410e06b30849578a7d6dd110df5285c4f7cbc7bf9d95aa3aa9458a6574 | True |
| s.dev.entry | 1 | 🔣️.json 5da6b929d68c6513de152605011222ff1c4c7b4f33961012b1aff1bf3cebd70d | True |
| os.plugin.registry.descriptor-verification | 3 | 🔣️.json e25829a763f79829b00b51ded56c0892494c02a8f39391d5c943346fbdb842fe | True |
| os.plugin.registry | 7 | 🔣️.json 2657c8dd983a7ee42b3ff49b6f1203d9bbefaf4b3cd8f758dd29efaa1c4e9e73; 🟦️.ts d009df574b9e399e5699981663a65e31c41b67eb3c03fc35155c2e14b179c4a7 | True |

Original three requested physical paths are absent with zero exact catalog references: True.

Concrete boundary findings:
```json
[
  {
    "code": "schema-catalog-stale",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json",
    "detail": "The catalog does not match the schema modules on disk."
  }
]
```

This schema result does not prove the test consumer runnable: Native separately observed a peer reintroduction of the deleted WGPU corpus schema read; its narrow repair is in progress with new peer laws preserved. Original normal compiler/publication/default acquisition/mounted HTTP and actual ticket closure remain outstanding.
