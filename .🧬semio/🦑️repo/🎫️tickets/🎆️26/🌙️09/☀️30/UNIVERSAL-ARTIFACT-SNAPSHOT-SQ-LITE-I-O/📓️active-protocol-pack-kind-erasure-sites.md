# Active Protocol Pack Kind Erasure Sites

Read-only current source census,2026-10-03. No Cargo or source changes. Findings sent directly to shared_provider_allocation for active producer repair; no speculative enum rewrite in this lane.

Actual protocol PackError lives in replication `⚙️codec/🦀️.rs`; OS Pack and Store reexport its identity. The standalone framework PackError typed variants do not prove the active protocol producer has been repaired.

| Actual Source | Current Producer Conversion | Location |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs` | Native controlled schema, charge, step, copy_text, copy_bytes, octet_buffer, copy_utf8 => `Schema(error.into_message())` |2009,2010,2011,2013,2014,2015,2018 twice|
| Same | Controlled schema graph/hash => same erasure |2912,3003|
| Same | Actual controlled document/body begin_stage => same erasure |3130,3181|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs` | `OutputError::into_pack`: Refusal => Schema(into_message); Text => Schema(to_string) |11|
| `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs` | Controlled Deflate ValueError => Schema(into_message) |413|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | `text_error_to_pack_error`: TextError => Schema(to_string) |10853|

Every listed conversion discards the typed cause before the outer enum mapper can preserve it. These wrappers need the active producer's explicit ValueError/TextError variants or equivalent typed mapping. Message classification cannot recover lost Canceled, OwnershipLimit, AllocationFailed, or InvariantViolated identity. Ordinary `manifest not loaded` construction is excluded: it is a source-assigned schema error and not erasure of an existing typed cause.
