# Inventory Artifact Sharding

## Outcome

The root taxonomy inventory command now publishes a deterministic, lossless sharded artifact instead of constructing and writing another inventory monolith. The retained pre-transaction-v2 `📊️taxonomy-inventory/🔣️.json` is not read, rewritten, or deleted by the publisher. No full inventory was run in this lane.

Production implementation is in `/Users/ueli/Documents/semio/📜️script.ts`:

- CLI path/option authority: lines 18037-18085.
- Pure shard builder, canonical chunk stream, validator, and publisher: lines 18087-18481.
- Seven-group plan reporting and apply terminal-state gate: lines 18502-18549.
- Root inventory/plan/apply/verify integration: lines 18606-18697.

Permanent ticket-local coverage is `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️inventory-artifact-shards.test.ts`.

## Frozen Artifact Contract

- Manifest: `📊️taxonomy-inventory/📊️shards/🔣️.json`.
- Payload: `📊️taxonomy-inventory/📊️shards/🔖️<sha256>/🔣️.json`.
- Payload envelope has exact keys `{entries,ownerId,schemaVersion}` and a terminal newline.
- Payload bytes are canonical JSON and strictly less than 5,242,880 bytes.
- Owners are UTF-8 byte-sorted. An owner's entries are UTF-8 byte-sorted by `sourcePath`; oversized owners split into contiguous zero-based parts.
- Payload identity is SHA-256 of exact payload bytes. Manifest descriptors record path, digest, owner, part, count, byte size, and first/last source path.
- The manifest records canonical inventory digest, metadata, entry/violation counts, canonical violation digest, descriptor-ledger digest, and descriptors.
- Canonical violation identity and ordering are the full UTF-8 byte tuple `(path, code, severity, message)`, independent of locale and insertion order.
- Builder/validator fail closed on one-entry overflow, duplicate source paths, duplicate descriptor identities, non-contiguous parts, noncanonical bytes, order drift, digest drift, count drift, boundary drift, missing payloads, and unreferenced payloads.

## No-Monolith Digest

`taxonomyInventoryCanonicalChunks()` branches on `entries` and `violations` before invoking `canonicalJson`; it yields punctuation, keys, and one row encoding at a time. `taxonomyInventoryIncrementalCanonicalDigest()` feeds those chunks directly into SHA-256. It never calls `canonicalJson` on either large array or the complete inventory.

The language-neutral witness proves:

- joined chunks equal the existing canonical JSON bytes for a small fixture;
- the incremental SHA-256 equals SHA-256 of the small canonical document;
- for a two-row 1.8 MB witness, no emitted chunk reaches the encoded array's byte size;
- reversed same-prefix violations with different severities produce the same canonical violations digest.

## Publication Protocol

1. Build and fully validate all manifest/payload bytes in memory. Entries are encoded one row at a time and no whole-inventory JSON string is constructed; the pure build result intentionally retains the independently bounded payload strings needed for validation/publication.
2. Validate any existing published manifest and its exact payload closure.
3. Reuse an existing content-addressed directory only when its sole real file matches exact bytes, byte count, and digest; otherwise fail as a digest collision/corruption.
4. Write each new payload into a deterministic `.inventory-shard-<digest>.staging` directory, verify it, then rename the directory to its immutable digest identity. Failed staging evidence remains fail-closed for diagnosis.
5. Re-read and verify every referenced payload.
6. Write and verify `.inventory-manifest-<digest>.staging`, then publish with one atomic rename to `🔣️.json`. The prior manifest remains readable until this rename.
7. Remove superseded immutable payload directories after manifest publication, validate exact closure again, and emit final `write-shards` progress.

The test proves replacement publication leaves the retained root monolith byte-identical and rejects a corrupted content-addressed payload on the next publication.

## CLI Safety and Correctness

- User `--plan`, `--resume`, and `--cancel-file` candidates are resolved lexically inside the repository before ticket resolution, inventory, or candidate reads.
- `compose`, `compose/**`, `temp/compose`, and `temp/compose/**` are rejected before any `lstat` of the candidate.
- Outside-repository candidates, symlink ancestors, non-directory ancestors, and directory final candidates are rejected; no candidate is realpathed or followed.
- Operation-specific options are closed: `--baseline` only on plan; `--digest` and `--resume` only on apply; `--plan` only on plan/apply; `--scope` and `--workers` only on inventory/plan/verify; `--fail-on-warning` only on verify.
- Root planning always passes `excludedTreeDigests: []`; apply rejects a plan with any opaque-tree digest opt-in.
- A plan's Markdown and console evidence share all seven operation groups, including `embeddedTicketRoots`.
- Apply writes its result/summary first and then throws unless the terminal state is exactly `committed`; `rolled-back` and `cancelled` therefore preserve evidence but return a failing process status.

The isolated CLI probes use lexical `compose/plan.json` without filesystem access and prove the opaque path fails before inventory. An inapplicable inventory `--resume` also fails before inventory.

## Test Evidence

Command:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️inventory-artifact-shards.test.ts'
```

Final result on Bun 1.3.14:

```text
7 pass
0 fail
49 expect() calls
Ran 7 tests across 1 file. [20.64s]
```

Coverage includes deterministic/lossless reconstruction, incremental canonical parity, no-whole-array chunk witness, full violation-tuple byte order, owner splitting, one-entry overflow, duplicate/order/digest/count/unreferenced rejection, manifest-last replacement, retained-monolith immutability, corruption collision rejection, progress, third-party `fast-glob` path parity, WebCrypto-versus-Node SHA-256 parity, lexical/no-follow CLI guards, operation option closure, all-seven plan visibility, and committed/rolled-back/cancelled process statuses.

`git diff --check` reported no whitespace errors for the owned root/test paths. Source checks found no remaining `taxonomyCliPath`, full `canonicalJson(inventory.entries|violations)`, backup-swap, or root-monolith inventory write in the implemented region.

Final SHA-256 before this report:

- `/Users/ueli/Documents/semio/📜️script.ts`: `d8e621417a3d5593cecfdfec26f75595dc1c85380b001191823c6eaa59b0c3cb`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️inventory-artifact-shards.test.ts`: `c1f95c7e9f2a93649fc8dc8d06dcf7e774807dec6d1bacff05735c21ed826f23`

## Acceptance Boundary

This lane did not run the 112 MB/16-minute inventory and does not claim a post-transaction-v2 canonical inventory. The next full inventory run must occur only after transaction-v2 freezes, using this sharded writer, and must verify each emitted payload remains below the ticket file limit and that the manifest/accounting reconstructs the canonical inventory digest exactly.
