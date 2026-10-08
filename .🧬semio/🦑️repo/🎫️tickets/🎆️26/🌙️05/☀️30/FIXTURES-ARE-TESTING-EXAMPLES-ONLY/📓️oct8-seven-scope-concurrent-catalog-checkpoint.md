# Seven Scope Checkpoint With Concurrent Catalog Drift

Every row of the latest full raw source check was parsed. Fixture/corpus diagnostics are zero and all seven owned genuine schema scopes have zero diagnostics, their current physical bytes matching the actual catalog. All three paths from the request are absent and have no exact catalog scope references. This is useful current boundary evidence, but this attempted coordinated checkpoint is not settled: one schema-catalog-stale row appeared while other developers updated the repository between generation and full check. The permanent gate and actual native producer remain held while the current catalog is refreshed.

| Receipt | Bytes | SHA-256 | Terminal |
| --- | ---: | --- | --- |
| gen | 754 | c416e0fcd66de2e1c7af0658bd882eb226ce547239fca31fc38abcad5e097a49 | NX   Successfully ran target root-schema-generate for project ticket-fixture-verification; Run duration:      1m 36s |
| docs | 710 | dd07fc9def1ec47e56fe597e2b8fbaa8dee5ad7b48a86779d9bd5e8f684b0ed9 | NX   Successfully ran target root-schema-docs for project ticket-fixture-verification; Run duration:      36.6s |
| full | 5472124 | cecf2f1f1189913fcfdfb00ecb880ef781b246cc402833c9b204e4aa323ed75b | NX   Running target root-schema-check for project ticket-fixture-verification failed; Run duration:      1m 10s |

Actual catalog: 3630 scopes; SHA-256 04b3a2832b33c56a96dc126ccfb6b74087c6b403124c02417cd8b6172678cc2c. Full check: 4,477 modules, 3,630 scopes, 12,395 valid JSON findings, zero malformed rows, 46 shared codes and zero unshared codes. The full command exited Nx1 because broader repository schema findings remain and one concurrent catalog mismatch is present; no global pass is asserted.

All broader counts: `{"schema-module-id-missing": 253, "schema-owner-ineligible": 530, "schema-dialect-not-draft-07": 82, "schema-mutation-leaf-id": 75, "schema-export-incomplete": 9405, "schema-export-id-invalid": 255, "schema-ref-unresolved": 848, "schema-mutation-aggregate-id": 6, "schema-document-id-unaddressable": 129, "schema-document-id-duplicate": 366, "schema-module-id-inconsistent": 10, "schema-export-id-duplicate": 285, "schema-scope-ambiguous": 147, "schema-placement-forbidden-filename": 3, "schema-catalog-stale": 1}`.

| Owned Scope | Source SHA-256 | Exports |
| --- | --- | --- |
| hub.bootstrap | 33c6f93b3db0d917c0c5c4453ce9cc86c43ce50ca87752f3b5c288906037b557 | 7 |
| repo.discovery.runtime | 3690533367c57d80211db7b8541a7b48a15911313b7180d64a92fda34a166806 | 23 |
| os.plugin.browser-bundle | c32c7b89824f61ad80940311cc1c37be25ee3c7e5127dc9a01af4e2aa26d5aa0 | 23 |
| os.dev.distribution | 3ec59278818573a28e996993870ea63c65d78bf9be6e70c8cbd950a493646b40 | 10 |
| os.dev.verification.publication | efd12568470ab21f04ff4550066578a651eb5f8b7284041290c61dd2ef901573 | 6 |
| repo.library.workspaces.cargo.preparation.custody | aff63098ae5cd30a8130465af64f70849192e6fc2b643c25fa13dc6d5c952a34 | 3 |
| s.dev.entry | 39decdc91b797c45320ff6cb77b5b90a069f0e0dc952e9aad5a28ae020a967d1 | 1 |

The 982 fixture findings in the earlier resumed RED are retired in this actual full observation: true corpus schemas were removed or their genuine payload contract extracted, and domain snapshots with an empty properties map remain ordinary data. No domain snapshot bytes or broad schema guard exemption were changed. The currentness issue is derived catalog drift, not a fixture authority regression.

Raw logs and machine receipt remain in ticket generated output only until their consumers finish; this Markdown retains the durable research result. Ticket and goal remain active.
