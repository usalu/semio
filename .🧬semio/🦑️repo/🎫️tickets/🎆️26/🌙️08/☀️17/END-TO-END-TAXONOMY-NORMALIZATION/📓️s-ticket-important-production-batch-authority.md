# Ticket-Important Production Batch Authority

## Boundary

This packet freezes the remaining physical `📌️important.md` cohort after the first committed history projection. It is a read-only pre-apply authority: no production leaf, Git state, schema, script, actual `compose/**`, or actual `temp/compose/**` path was changed or inspected.

The pinned normalization baseline remains:

```text
9f449b10659b95148c8bcb3f91ce583bf7446973
```

Two consecutive physical censuses were byte-identical. Each ledger row binds the NFC source path, regular-file mode, byte size, SHA-256 content preimage, and the applicable manifest state or exact catalog disposition. The combined ledger has SHA-256:

```text
3b5a3cc30081441a661ca692a4ecc22b068f58aa8651ee65cf1a60b3f2a48a02
```

## Exact cohorts

| Cohort | Remaining leaves | Ledger SHA-256 | Transaction disposition |
|---|---:|---|---|
| Open ticket owner | 183 | `f5bd818f4520c27e4a63d67a1a54cc2ec7976812bdc10f05e7cc0038e92b128a` | `📌️important.md -> 📌️important/📝️.md` |
| Closed zero-byte ticket owner | 243 | `496cf49c0e2f0e8b1de5f8059edea9ed6c28c17022b844ade883f29f4831f4ff` | manifest-bound removal |
| Historical ticket owner | 30 | `26639815c3b6da1a65dc47d6fb0c9d81840298b5a3f1c453e3e70867eaab9654` | `📌️important.md -> 📓️important/📝️.md` |
| Exact catalog | 3 | `d0c5d3aaa4f60f2b2947920f3d7b4d7e29f6ecc195f2be2671c429407f37861e` | two removals and one presence-owned move |

The 459 remaining leaves plus the already committed CLI history leaf reconcile the original 460-leaf authority census. The historical owner count is now 30 because the first of the frozen 31 history cases already converged.

The three exact-catalog rows remain bound to the checked-in `ticket-important-exact-mutations` preimages:

- nested phase 1.5: zero-byte removal;
- nested phase 9: zero-byte removal;
- window-policy presence fixture: exact `👥️presence/📝️.md` move.

They are excluded from manifest-owner inference even though their basename is the same.

## Destination proof

The 213 manifest-owned move destinations are all absent. Their exact, NFC, locale-independent case-fold, and VS16-fold comparison sets contain zero collisions. Every destination is NFC. The longest manifest-owned destination is 228 UTF-8 bytes, below the schema-owned 240-byte maximum:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FULL-PHOTOGRAMMETRY-AND-VIDEOGRAMMETRY-STACK-FOR-REMODEL/REWRITE-REMODEL-PLUGIN-TO-WIRE-THE-FULL-PHOTOGRAMMETRY-STACK-INTO-THE-APP-UI/📌️important/📝️.md
```

The exact presence destination is separately catalog-bound and is not misclassified as a 292-byte historical owner projection.

## Apply sequencing

The generic plan currently has one scope prefix, so a broad ticket-tree scope would admit unrelated retained evidence violations. Production convergence must therefore use exact source-leaf scopes and the already integrated transaction-v2 authority. For every leaf or deliberately serialized small set:

1. revalidate the ledger preimage and destination absence;
2. create a fresh plan against the pinned baseline;
3. require the exact expected rationale and zero unresolved decisions;
4. apply only after fixture-test processes are idle and schema/discovery/normalization owners are released;
5. require a committed terminal journal;
6. replan the identical original source scope and require zero operations and zero unresolved decisions.

The production apply must remain serialized while transaction fixtures are active. Attempt `000001` of the first leaf proved that concurrently removed ticket-local fixture journals can race a transaction scan. No batch application is authorized from this evidence alone while parallel normalization edits remain in progress.

An owner-directory probe confirmed the boundary rather than merely assuming it. The closed `FIX-CODEBASE-SECTIONS-AND-TICKETS` owner plan derived the correct single manifest-bound removal, but failed closed on its separate raw `ticket.md` leaf with `semantic-stem-unresolved`. The same owner is therefore safe only through the exact `📌️important.md` source scope until the independent ticket-document lane is integrated.

## Verification command shape

Each fresh exact plan and apply uses the established root router:

```text
bun ./📜️script.ts clean taxonomy plan --baseline 9f449b10659b95148c8bcb3f91ce583bf7446973 --scope '<exact-source-path>' --ticket '26/08/17/END-TO-END-TAXONOMY-NORMALIZATION' --plan '<ticket-local-plan-path>' --workers 1
bun ./📜️script.ts clean taxonomy apply --baseline 9f449b10659b95148c8bcb3f91ce583bf7446973 --ticket '26/08/17/END-TO-END-TAXONOMY-NORMALIZATION' --plan '<ticket-local-plan-path>' --digest '<fresh-plan-digest>'
```

This packet does not claim that any of the 459 leaves has yet been applied.
