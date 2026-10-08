# Eight Scope Current Full Checkpoint

Source checkpoint settled: False. Full schema check exits Nx1 for broader findings; no global pass is asserted. The normal permanent fixture boundary and independent complete raw audit remain pending.

| Receipt | Bytes | SHA-256 | Terminal |
| --- | ---: | --- | --- |
| gen | 751 | a122434fcf27165c21e20acefbff1c96ef9d192955f4e0904a0d5a5f63e1bafd | NX   Successfully ran target root-schema-generate for project ticket-fixture-verification; Run duration:      1m 3s |
| docs | 710 | c6555dc30430701229054566eff1e5e13e8e92cd970bd8881a9784fdc3343c3d | NX   Successfully ran target root-schema-docs for project ticket-fixture-verification; Run duration:      49.0s |
| full | 5452778 | f19460e73db02c606549525c7ba3f39207414475d700cf11cc3aa8aaf5b24c93 | NX   Running target root-schema-check for project ticket-fixture-verification failed; Run duration:      42.0s |

Every raw full row was parsed: 12355 valid JSON, 0 malformed, fixture/corpus=2, catalog=1, owned eight scopes=1. Actual catalog scopes=3633, SHA-256 f2d0a69bc27a436fc237e5f22ab8bcfa03079e0a0fa9684a435263e6db1ca362. All eight current physical policy hashes/export sets match: True.

```text
[schema check] modules=4485 scopes=3633 findings=12355
[schema check] schema-catalog-stale=1
[schema check] schema-dialect-not-draft-07=83
[schema check] schema-document-id-duplicate=366
[schema check] schema-document-id-unaddressable=131
[schema check] schema-export-id-duplicate=284
[schema check] schema-export-id-invalid=262
[schema check] schema-export-incomplete=9386
[schema check] schema-fixture-defines-schema=2
[schema check] schema-module-id-inconsistent=10
[schema check] schema-module-id-missing=261
[schema check] schema-mutation-aggregate-id=6
[schema check] schema-mutation-leaf-id=75
[schema check] schema-owner-ineligible=530
[schema check] schema-placement-forbidden-filename=3
[schema check] schema-ref-unresolved=808
[schema check] schema-scope-ambiguous=147
[schema check] shared-code-table=46 unshared-codes=0
 NX   Running target root-schema-check for project ticket-fixture-verification failed
  Run duration:      42.0s
```

Broader counters: {"schema-catalog-stale": 1, "schema-dialect-not-draft-07": 83, "schema-document-id-duplicate": 366, "schema-document-id-unaddressable": 131, "schema-export-id-duplicate": 284, "schema-export-id-invalid": 262, "schema-export-incomplete": 9386, "schema-fixture-defines-schema": 2, "schema-module-id-inconsistent": 10, "schema-module-id-missing": 261, "schema-mutation-aggregate-id": 6, "schema-mutation-leaf-id": 75, "schema-owner-ineligible": 530, "schema-placement-forbidden-filename": 3, "schema-ref-unresolved": 808, "schema-scope-ambiguous": 147}.

| Scope | Physical SHA-256 | Exports | Current catalog matches |
| --- | --- | ---: | --- |
| hub.bootstrap | 33c6f93b3db0d917c0c5c4453ce9cc86c43ce50ca87752f3b5c288906037b557 | 7 | True |
| repo.discovery.runtime | 3690533367c57d80211db7b8541a7b48a15911313b7180d64a92fda34a166806 | 23 | True |
| os.plugin.browser-bundle | c32c7b89824f61ad80940311cc1c37be25ee3c7e5127dc9a01af4e2aa26d5aa0 | 23 | True |
| os.dev.distribution | 3ec59278818573a28e996993870ea63c65d78bf9be6e70c8cbd950a493646b40 | 10 | True |
| os.dev.verification.publication | efd12568470ab21f04ff4550066578a651eb5f8b7284041290c61dd2ef901573 | 6 | True |
| repo.library.workspaces.cargo.preparation.custody | 6ed52b410e06b30849578a7d6dd110df5285c4f7cbc7bf9d95aa3aa9458a6574 | 5 | True |
| s.dev.entry | 21d8e85278338ce67994bfc7b7c4f9e6d136f3af02e67705d855134c848d975b | 1 | True |
| os.plugin.registry.descriptor-verification | e25829a763f79829b00b51ded56c0892494c02a8f39391d5c943346fbdb842fe | 3 | True |

Concrete boundary findings:
```json
[
  {
    "code": "schema-fixture-defines-schema",
    "path": "🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🧬️schema/🔣️.json",
    "detail": "🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🧬️schema/🔣️.json defines schema authority inside examples; contract identity cannot be declared inert."
  },
  {
    "code": "schema-fixture-defines-schema",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🧾️publication/🧪️tests/🔐lease/🧬️schema/🔣️.json",
    "detail": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🧾️publication/🧪️tests/🔐lease/🧬️schema/🔣️.json defines schema authority inside examples; contract identity cannot be declared inert."
  },
  {
    "code": "schema-catalog-stale",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json",
    "detail": "The catalog does not match the schema modules on disk."
  },
  {
    "code": "schema-ref-unresolved",
    "path": "✏️s/🧑‍💻dev/🚀️entry/🧬️schema/🔣️.json",
    "detail": "/properties/catalog/$ref references \"https://json.schemas.assets.semio-tech.com/os/plugin-catalog.json\"; no catalogued document declares that $id."
  }
]
```

The original normal native compiler, four guest checks, selected-six Hub/Registry/Dev/protected publication, default no-ticket runtime acquisition, mounted HTTP and final actual ticket closure remain outstanding. Machine output remains owned generated evidence until final consumers finish.
