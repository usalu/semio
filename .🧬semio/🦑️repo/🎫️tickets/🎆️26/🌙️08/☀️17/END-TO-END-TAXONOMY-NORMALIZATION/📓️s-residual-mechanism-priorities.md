# Residual Mechanism Priorities

## Evidence status

This is a read-only prioritization of the retained pre-transaction-v2 inventory. It is not a current-tree acceptance census and does not authorize a live plan or apply. The source inventory remains byte-unchanged.

## Highest-leverage ambiguity correction

The retained inventory has 10,477 `semantic-stem-ambiguous` records. Exact grouping by message is:

| Competing directory kinds | Count |
| --- | ---: |
| `standard`, `subset` | 8,029 |
| `test-case`, `test-fixture-member` | 2,251 |
| `mutation-test-profile`, `test-case` | 184 |
| `standard`, `subset`, `taxonomy-registry` | 5 |
| `configuration`, `standard`, `subset` | 2 |
| `capabilities`, `standard`, `subset` | 2 |
| `schema`, `standard`, `subset` | 1 |
| `plan`, `standard`, `subset` | 1 |
| `editor`, `standard`, `subset` | 1 |
| `asset-subject`, `test-fixture-asset` | 1 |

The first group is a schema-context defect, not 8,029 independent semantic decisions: `standard` and `subset` have broad slug patterns but no parent-kind constraint, so ordinary semantic stems outside the standards tree collide. Their normative locations are direct children of `standards` and `subsets`; parent constraints must be frozen and tested before a fresh census.

The test groups require a separate resolver audit. `test-case`, `test-fixture-member`, and `mutation-test-profile` already declare parent contexts, so the fresh v2 result must distinguish a stale pre-v2 census from a current context-propagation defect before changing schema.

The corresponding `directory-kind-ambiguous` histogram reinforces that split:

| Competing directory kinds | Count |
| --- | ---: |
| `mutation-test-profile`, `test-case` | 1,100 |
| `standard`, `subset` | 400 |
| `test-case`, `test-fixture-member` | 23 |
| `test-fixture-member`, `ticket-test-evidence` | 4 |
| all remaining groups | 12 |

`mutation-test-profile` and `test-case` intentionally overlap lexically beneath tests. The exact mutation catalogs, rather than a broader regex or guessed emoji, must decide those 1,100 paths. A fresh scoped census must prove the catalog projection removes the generic ambiguity before any schema widening.

## Ordered next mechanism work

1. Finish and independently sign off transaction-v2 resume, conflict, opaque-boundary, and disposition fixtures.
2. Materialize deterministic sub-5 MiB inventory shards and phase telemetry; do not overwrite the retained pre-v2 monolith.
3. Freeze `standard`/`subset` parent constraints with negative counterexamples and rerun a small scoped census.
4. Audit test-context propagation before altering any test directory kind.
5. Profile a scoped v2 inventory by closed phase, then optimize the measured dominant phase before the final full rerun.
6. Use the fresh v2 shards, not this snapshot, for physical residual packets and the global zero-unresolved plan.

## Commands used

The two reads were streaming `jq` projections over the retained ticket JSON: one sample of `semantic-stem-ambiguous` path/message rows and one exact message histogram. Neither excluded workspace prefix was accessed.
