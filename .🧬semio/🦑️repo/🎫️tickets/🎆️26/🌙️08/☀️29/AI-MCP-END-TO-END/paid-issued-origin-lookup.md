# Paid Issued Origin Lookup

Each guest loan reserves its epoch from the same monotonic treasury counter as host turns. The driver holds one loan until its escrow settlement. Its history holds the actual descriptor and native producer together in an append-only paged list. The issuer verifies the next epoch against the funded lookup of the last entry, then performs the existing paid append. A repeated or decreasing epoch is refused.

The frozen receipt and claim resolvers now keep a binary search range over the occupied entries. Each probe checks the actual retained grant and depth, posts the paged-list path work and metadata receipt, and checks the provenance of an equal epoch before retaining its index. Receipt matching requires the exact producer, serial, purpose and budget. A claim against an earlier entry requires its paid receipt to have settled. The existing frozen borrow authenticates the snapshot and keeps the history stable. Retirement still traverses the actual retained owners.

The language-neutral law exercises 2,913 sparse epochs and covers monotonic insertion, duplicate and decreasing epoch refusal, missing epochs, forged receipt descriptors, wrong producers and the requirement for earlier claims to be settled. The independent SQLite oracle passed all nine lookup and five issuance cases. An epoch value in the live driver also counts host turns, so it does not measure the number of retained descriptors. The opt-in trace reports the actual committed issuance count.

The exact native Wake relink passed all 33 tests. Across the nine origin lookups, the measured ledger recorded 72 funded probes and 762 work units. A full linear scan for each lookup would visit 26,217 entries.
