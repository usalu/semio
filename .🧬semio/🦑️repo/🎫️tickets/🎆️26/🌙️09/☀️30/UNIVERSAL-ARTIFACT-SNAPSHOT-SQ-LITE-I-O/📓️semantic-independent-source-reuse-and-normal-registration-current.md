# Independent Current Source and Registration Audit

Fresh read-only source inspection on 2026-10-05. No runtime tests run by this auditor. Root owns mounting and verification.

## Source reuse

[Projection source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts:73) still calls artifactSqliteDatabase and therefore reparses at finish. [Validator source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts:338) still tokenizes every received table SQL. Neither held optimization was mounted at inspection.

[Projection held delta](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/projection-owned-source-authored-schema-reuse-held.json) changes only the private finish assembly route to use the existing private schema. The public assembly route still parses caller SQL; private assembly still checks table count and invokes complete final validator. It introduces no trusted external schema API. Existing insert budget, owned BLOB copy, row sorting and duplicate checks remain intact.

[Literal held delta](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/controlled-source-literal-schema-reuse-held.json) adds authored source alongside parsed tokens and columns. Equality delegates to [bounded equalText](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts:215), in 16384 code-unit chunks with validateSchema progress. Both actual table name and SQL UTF-8 are measured before equality. All aggregate table, row width, signed INTEGER identity, duplicate identity and INTEGER alias checks still execute. Different source falls back to the original token comparison. This is semantic-preserving inspection, not runtime credit.

## Coverage limits

[Operation neutral fixture](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧫️fixtures/💰️operation/🔣️.json) covers Unicode scalar bytes, debt, cancellation frontiers and projection/export/import ownership. Its authored SQL is short and does not independently exercise long exact-literal equality, a late differing token fallback, or cancellation specifically during equality. [Operation third-party authority test](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/💰️operation/🟦️.ts:12) validates neutral corpus with Ajv and owned schema validator; it cannot alone establish schema comparison branch coverage. Require real semantic input cases for exact authored SQL, token-equivalent changed spelling, changed literal refusal, invalid row width, duplicate/out-of-range rowid and alias mismatch under same literal SQL. Avoid implementing an equality mirror as the oracle.

## Normal registration

[Held owner hooks](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/flow-run-dag-normal-registration-root-current-provider-held.json) target actual FlowHostSnapshot, RunArtifact and DagSnapshot ArtifactPack implementations. Fresh source inspection found sqlite_snapshot_codec but no native_snapshot_registration overrides in those three owners. FlowHostSnapshotDsl is a distinct intermediary and is not the target.

Dialect/schema values agree with current owner constants: flow.host_snapshot / FLOW_DOCUMENT_SCHEMA, os.run / S_RUN_SCHEMA, dag.host_snapshot / DAG_DOCUMENT_SCHEMA. They must remain distinct from plugin s.flow.flow. [ArtifactCodec::bare](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11564) uses the actual owner and mutation generic parameters. [Store construction](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17783) and [history hydration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:296) consume the existing native_snapshot_registration hook. No new registry abstraction is needed. Existing manually registered public tests are insufficient evidence of ordinary Store publication. Genuine Before failures and After runtime verification remain Root-owned.

This audit does not promote Layout, native fleet, PDF VT or provider-count claims.
