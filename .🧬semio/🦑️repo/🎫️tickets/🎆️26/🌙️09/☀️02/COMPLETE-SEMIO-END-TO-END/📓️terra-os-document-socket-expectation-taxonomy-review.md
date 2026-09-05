# Terra OS Document Socket Expectation Taxonomy Review

## Verdict

The current public contract is already the no-alias, full-lease design. `DocumentSocketSurfaceExpectationV1`, its `DocumentSocketExpectationV1.surface` field, and `DocumentSocketAuthorityV1::matches_surface` are intentionally removed. Production store/sync is already migrated. The WGPU-reported compiler break was two stale, test-only uses in the directory client; the concurrent repair has replaced them with `DocumentExecutionTargetLeaseFieldsV1` and source recheck now finds **0** occurrences of each removed symbol across the OS tree.

Do **not** restore the removed type, field, or subset predicate. The smallest coherent correction is the one now present: construct a verified complete lease when a local target exists; otherwise use `lease: None`; compare through `same_lease_fields_v1` before the receipt exchange.

This was a read-only source audit after the concurrent patch. No cold/native build was run, so this report does not claim compilation or runtime execution.

## Current public contract

| Concern | Current authority | Exact source |
|---|---|---|
| Socket expectation | `artifact_schema`, `pack_schema_hash`, optional non-authoritative `requested_surface_id`, optional complete `lease` | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:445-457` |
| Complete local target identity | Scope, descriptor/catalog, package and component/descriptor hashes + byte lengths, artifact/dialect, all surface fields, grant, checkpoint, revalidation | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1635-1655` |
| Lease validation | Enforces byte identity bindings, nonzero bounded byte lengths, surface/grant relation, checkpoint and revalidation invariants | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1657-1722` |
| One comparison relation | Projection from a plan plus exact structural equality; no partial transport comparison | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1725-1750` |
| Retained authority check | Reprojects retained plan authority using the verified byte lengths and invokes that same relation | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:408-436` |

`requested_surface_id` remains a selector only. The native client places it in `DocumentOpenIntentV1`, lets Hub select the plan, then merely requires the selected plan surface id to agree. It never transmits caller package/module authority. Before any receipt is exchanged it also checks plan validity, scope, artifact schema, pack schema hash, and—when supplied—the entire lease projection. The sole receipt exchange follows those checks at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1062-1103`.

## Current consumer matrix

| Consumer | Current shape | Assessment |
|---|---|---|
| Native directory admission | Optional full lease compared at plan admission; selector checked separately; receipt is exchanged only afterward | Correct at `directory/🔌️client/🦀️.rs:1073-1103` |
| Native store host | Holds `HashMap<ArtifactDocumentKey, DocumentExecutionTargetLeaseFieldsV1>` and accepts it only for Hub keys before open | Correct at `🏪️store/🔄️sync/🦀️.rs:1048-1065,1126-1137` |
| Native actor connect | Passes the full lease as `expectation.lease`; does not construct a removed surface expectation | Correct at `🏪️store/🔄️sync/🦀️.rs:1978-2003` |
| Native actor post-connect | Rechecks origin, scope, schema/hash, selector and full lease before retaining a socket | Correct at `🏪️store/🔄️sync/🦀️.rs:2045-2072` |
| WGPU persistence selector | `PersistenceBinding::Hub { surface }` is still a requested-surface selector, not the removed expectation taxonomy | Leave unchanged; it is not this compiler fault |

## Concurrent correction recheck

The two former test-only errors in `directory/🔌️client/🦀️.rs` have been repaired:

1. `document_expectation` now defaults to `lease: None`, and `document_lease_fields` decodes the neutral installed-target fixture into a complete lease (`:2216-2232`). The valid admission law supplies that full lease (`:2311-2324`).
2. `mismatched_local_plugin_selection_never_exchanges_a_plan_receipt` now starts from that complete lease, substitutes `package.package_id`, and asserts the plan receipt is never exchanged (`:2415-2429`).
3. `execution_target_lease_compares_every_plan_and_verified_byte_field` projects the neutral plan, verifies the fixture component/descriptor digests, then rejects at least 30 single-field hostile substitutions with the same relation used by retained authority (`:2235-2302`).

The native helper's browser-named JSON input is a language-neutral `installedTarget` wire fixture, not a browser runtime dependency; both its package/digests and the native plan fixture match. Its dynamically replaced scope, artifact schema, surface id and checkpoint document id match the corresponding dynamic plan projection.

## Minimal durable correction and executable laws

No production taxonomy correction remains. Preserve the current repair and execute these existing exact laws with `semio-framework-os-kernel --lib --features sync,ureq`:

1. `native_document_admission_issues_validates_and_exchanges_exactly_once` — a matching complete lease admits exactly two requests; only the receipt enters the second request.
2. `mismatched_local_plugin_selection_never_exchanges_a_plan_receipt` — a valid-shape lease with a foreign package id stops at the first plan request.
3. `hostile_or_cancelled_plan_never_reaches_receipt_exchange` — hostile schema/cancellation cannot exchange a plan receipt.
4. `execution_target_lease_compares_every_plan_and_verified_byte_field` — common plan/authority equality rejects every corpus field substitution and proves supplied bytes hash to declared identities.

The registered `os-hub:native-document-open-check` already selects the first three relevant admission laws (including the mismatch) in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:4755-4777`. It does **not** yet select the fourth corpus law. Adding that exact suffix is the only observed coverage improvement, but is not required for the compiler repair and should be a separate one-line gate-coverage change. That Hub target also launches WGPU checks/build first (`:4743-4750`); there is currently no narrower named OS-kernel document-open target. The generic registered kernel fallback is `@semio-tech/framework-os-kernel:test-native`, not a document-open-specific selector.

## Quantified status

- Removed public aliases remaining in OS source: **0 / 3** (`DocumentSocketSurfaceExpectationV1`, `expectation.surface`, `matches_surface`).
- Production admission/host/actor locations requiring taxonomy migration: **0 / 4**; all already use full lease or selector semantics.
- Repaired stale test constructions: **2 / 2**.
- Exact native admission laws already listed by the Hub gate: **3 relevant / 4 recommended**; the omitted fourth is corpus-completeness coverage, not a functional blocker.

