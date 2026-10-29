# Schema Readiness Triage

Read-only Source/current log review of Root gate19363 terminal1. No compiler, schema generation, migration or diagnostic rerun by reviewer.

The23031 findings are separate measurements, not23031 independently proved broken contracts. Current catalog has3656 scope rows. schemaContractDiagnostics performs one Source walk for placement/eligibility/fixture isolation, compares schema module membership with the catalog, then performs catalog-driven export/reference checks. The orchestration separately appends mutation UI/payload/label/editability and fault notice diagnostics. This structure explains potential cascades but does not justify suppressing them.

| Finding family | Count | Interpretation |
|---|---:|---|
| export incomplete / parser missing |5042 /1272| Catalog-declared exports and format files disagree; stale rows or absent format/parser implementations can multiply findings. |
| fixture defines schema |3775| Authoritative contract facets live below test/example collections, or schema-shaped data lacks explicit inert declaration. |
| mutation input UI / fault notice |3462 /3448| Independent domain reports; not proved removable by catalog refresh. |
| catalog stale / malformed / files missing |1943 /110 /9| Foundational identity/membership/format evidence needs reconciliation first. |
| ref unresolved / export unknown / format undeclared |841 /197 /146| Catalog resolution findings; stale ownership/format metadata can cause downstream failures. |
| owner ineligible / placement outside module / forbidden filename |820 /523 /256| Actual taxonomy placement policy findings, independent of output index generation. |
| dialect not draft07 |518| Actual dialect declarations contradict repository normative contract. |
| mutation payload parity / editability |664 /2| Domain wire/history assertions, requiring their own Source evidence. |
| cross-scope dependency / internal ref broken |2 /1| Concrete declared dependency/reference boundaries once catalog identity is verified. |

Checked actual first specimens: the WFC fill contract is a real object JSON Schema declaring2020-12, so the draft07 diagnostic is not a random-fixture shape guess. The procedural text inference specimen is a real draft07 string schema outside the required schema module. Do not fix these by merely regenerating an index or changing the dialect label without semantic review.

Fixture isolation has an explicit inert-data mechanism: inertSchemaDataPaths reads inertSchemaData names from taxonomy contribution documents, while schemaCollectionContractPath deliberately refuses treating a contract facet below a test collection as inert. Some parser corpus data may need correct explicit inert classification; no3775-wide false-positive conclusion is supported by the truncated log. Move/promote actual owned contracts through authored domain design rather than add a blanket test-tree exemption.

The0/0 stage result means discoverSchemaFixtures found no nonempty schemaFixtures arrays in the taxonomy-selected contribution files. It is absence of measured fixture coverage, not success. Existing language-neutral test corpora elsewhere are not automatically schema-stage contributions. Preserve actual fixture stage binding rather than lower expected counts.

Get complete evidence through the existing compiler-free owned leaf: `SEMIO_LANGUAGE=en repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --json`, redirecting to this ticket’s generated/schema-diagnostics.json. It emits full {diagnostics,fixtures} and retains failure exit status. Root schema test --json also runs oracle/log output, so it is not a clean standalone JSON artifact. This route was inspected, not executed by reviewer.

Suggested order: capture full evidence; group foundational membership/eligibility/dialect/placement findings by actual owner and compare catalog provenance; repair those authored boundaries; regenerate catalog/docs only from corrected legitimate owners; rerun bounded owner diagnostics and distinguish residual export/parser/ref failures from domain UI/fault/payload obligations. Do not assume catalog generation repairs absent implementations or domain metadata. Root separately owns the Vitest explicit policy startup failure, which is distinct from these harness findings.
