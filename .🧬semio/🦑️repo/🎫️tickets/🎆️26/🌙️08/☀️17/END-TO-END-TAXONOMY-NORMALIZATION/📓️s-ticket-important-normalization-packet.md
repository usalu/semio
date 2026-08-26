# Ticket Important Normalization Packet

## Decision

`📌️important` is a projection-only semantic directory. It is never inferred from its emoji/name alone. Exact ticket-owner and sibling-manifest authority governs both migration from `📌️important.md` and acceptance of the canonical `📌️important/📝️.md` shape.

The integration has three ordered authorities:

1. Resolve an embedded ticket root to its canonical ticket owner before interpreting any important leaf.
2. Project an exact raw sibling `📌️important.md` only through `semanticOwnedFileProjectionAuthority`.
3. Validate an already canonical `📌️important/📝️.md` through the same owner, manifest status, exact directory, exact leaf, and no-extra-member constraints.

This order prevents a nested fixture or malformed historical directory from counterfeiting a ticket owner.

## Inventory Authority

After physical leaf admission and before generic mutation/artifact projection:

- Locate raw candidates only by exact basename `📌️important.md`.
- Require an immediate parent with the unique `ticket-slug` fixed-directory contract.
- Require the exact sibling `🎫️ticket.json` entry with the unique `ticket-manifest` fixed-filename contract.
- Require both nodes to be regular no-follow files and the source to resolve as Markdown.
- Decode only the manifest as UTF-8/JSON; bind its inventory content hash and full leaf preimage.
- Invoke the pure discovery authority with exact owner, manifest, source, kind, and byte-length facts.
- For `project`, replace only the source leaf's normalized path with `<owner>/📌️important/📝️.md` and attach rationale `ticket-important-markdown-projection-v1`.
- For `remove`, record a typed planning candidate; do not hide or mutate the inventory leaf.
- For `problem`, attach an error violation to the source and owner manifest identity.
- For `unclaimed`, leave generic normalization unchanged so counterfeit/malformed evidence remains visible.

Generic directory canonicalization may recognize the `ticket-important` name only provisionally. Before inventory freeze, every physical projection-only directory must prove:

- an exact ticket-slug parent;
- an exact sibling manifest with explicit `status: "open"`;
- exactly one regular child `📝️.md`;
- no raw sibling `📌️important.md` and no additional directory members.

On success the provisional projection-only violation is removed. Otherwise the directory and its members retain an error. This makes the second inventory/plan empty without blessing arbitrary `📌️important` directories.

## Planning Authority

Add a strict removal authority variant rather than weakening byte-identical evidence:

```text
kind: owner-manifest-status
contractId: ticket-important-markdown-v1
ownerPath: <exact ticket owner>
manifestPath: <owner>/🎫️ticket.json
manifestPreimage: <full regular-file contentHash/mode/size>
status: closed
contentState: zero-byte
authorityDigest: sha256(canonical exact fields)
```

The removal itself retains the important source's full regular-file preimage and uses rationale `ticket-important-closed-empty-v1`. Parsing requires exact keys and rejects unknown status, nonzero size, symlinks, malformed manifest identities, digest mismatch, or a source outside the owner.

Planning order is:

1. embedded ticket-root identity and canonical owner mapping;
2. important projection/removal decisions against the final owner;
3. collision grouping excluding typed removals;
4. ordinary moves and reference edits;
5. generator and symlink planning;
6. affected pre/post digests including manifest authority and destination ancestors.

For an embedded raw important leaf, evaluate the canonical owner manifest. An open source relocates directly to the canonical projected destination. A closed zero-byte source is a typed removal. A closed nonzero or invalid/colliding manifest blocks the entire embedded disposition. Embedded tree closure still requires one exact disposition for every frozen source leaf.

## Transaction Revalidation

Before staging a typed closed-zero removal, rederive all of the following from the workspace without following symlinks:

- source is a zero-byte regular file matching its complete preimage;
- owner and manifest paths still match their exact fixed contracts;
- manifest is a regular file matching its complete preimage;
- parsed status remains exactly `closed`;
- authority digest and operation ID recompute exactly;
- no unowned incoming reference remains.

The source and manifest enter the affected pre-state; the source becomes absent in post-state while the manifest remains byte/mode identical. Rollback restores the source from its typed backup and never rewrites the manifest. Backup history remains retained in terminal Journal v2.

## Reference Closure

Moves from raw to canonical open paths use the existing structured/raw reference planner and require exact preimages for every edit. A closed removal has no replacement path; it is legal only when no live incoming reference remains after excluding its own owner authority. Historical textual mentions that are not path references remain content, while exact path tokens are updated or block planning.

The Go owner already writes only the canonical path. Its regression test intentionally asserts that the old raw path is absent and is not a stale consumer.

## Permanent TDD Matrix

The normalization suite needs disposable, language-neutral cases for:

1. open zero-byte raw source projects;
2. open nonzero raw source projects byte-for-byte;
3. closed zero-byte raw source becomes a typed removal;
4. closed nonzero blocks without mutation;
5. missing, malformed, null, non-string, and unknown status block/unclaim exactly;
6. counterfeit owner, manifest, source kind, basename, symlink, and nonregular nodes do not claim authority;
7. canonical open directory with exactly `📝️.md` is accepted and replans empty;
8. canonical closed directory, extra member, raw+canonical duplicate, wrong leaf, or counterfeit parent blocks;
9. destination collision under open projection blocks unless an existing canonical leaf is covered by an explicit identical-evidence authority;
10. embedded open, embedded closed-zero, embedded closed-nonzero, divergent canonical manifest, and many-to-one collision cases;
11. exact raw/structured reference edits plus a closed leaf with an incoming reference;
12. failure injection before/after backup, staging, removal, installation, edit publication, verification, and rollback with exact filesystem snapshots;
13. canonical Plan/Journal parser rejection for every removal-authority key/type/digest mutation;
14. third-party JSON parser parity for manifest status and physical census parity without symlink following;
15. a second inventory/plan with zero operations and zero unresolved violations.

## Live Corpus Gate

The current live census is frozen in `📓️s-ticket-important-live-census.md`:

- 183 open projections;
- 243 closed-zero removals;
- 21 closed-nonzero blockers;
- one invalid-status blocker;
- 12 unclaimed missing-manifest evidence leaves.

The first implementation/apply wave may include only the 426 exact owner-governed operations. The remaining 34 cannot be coerced or deleted; full repository convergence additionally requires explicit owner repair or retention-manifest projection for each blocked record.
