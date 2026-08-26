# Ticket Important Schema Authority

## Outcome

The taxonomy now owns `📌️important` as the projection-only semantic directory kind and owns the exact `ticket-important-markdown-v1` sibling-manifest projection contract. The pure discovery authority requires the `ticket-slug` owner, the adjacent `ticket-manifest`, the Markdown source identity, an own explicit `closed` or `open` status, and exact source byte length. It returns only `project`, `remove`, `problem`, or `unclaimed`; no missing-status default exists.

The canonical projected leaf is `<ticket>/📌️important/📝️.md`. Closed sources are removable only at exactly zero bytes; nonzero closed documents are blocking problems.

## Language-Neutral and Independent Evidence

The permanent repo-library fixture `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️ticket-important-owner-authority/🔣️.json` freezes ten owner, manifest, status, and content vectors. The TypeScript test compares every vector with `semanticOwnedFileProjectionAuthority` and independently validates status ownership with test-only Ajv JSON Schema. The ticket-local eight-vector copy retains the original TDD evidence.

Red was observed before implementation: Bun reported the missing `semanticOwnedFileProjectionAuthority` export. The converged focused run on 2026-08-26 produced 8 passing tests, 0 failures, and 98 assertions across the important authority and transaction-artifact directory suites.

## Transaction Schema Addition

The same schema checkpoint adds `transaction-backup-preparation` and `transaction-restore-preparation` under `transaction-backup`, with exact names `🚧️backup-<24-lowercase-hex>-<positive-pid>-<uuid-v4>` and `🚧️restore-<24-lowercase-hex>-<positive-pid>-<uuid-v4>`. Their candidate directories own exactly one deterministic `<24hex>.backup` leaf for atomic backup publication or rollback restoration. Positive and negative parent/hash/PID/UUID vectors are included in the language-neutral transaction-attempt fixture.

## Remaining Integration

Normalization must consume this already-frozen registry without generic semantic-stem inference, construct lifecycle removals with manifest/preimage authority, account for all governed references, and preserve the Compose exclusion. The Go ticket owner lifecycle is an independent parallel implementation slice.
