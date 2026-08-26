# Pre-Transaction-v2 Current-Tree Inventory

## Status

This is the completed **pre-transaction-v2 current-tree residual snapshot**, not final v2 acceptance. The single requested inventory process exited `0`; it was not restarted. The retained 116,981,622-byte inventory is immutable evidence and was neither deleted nor rewritten.

Final acceptance requires a fresh run after transaction v2 freezes because every one of the 103,796 entries in this snapshot lacks the new `mode` and `size` evidence, and all 15 symlinks lack the raw `symlinkTarget` evidence. The permanent `temp/compose/` lexical exclusion also landed after this snapshot. This snapshot records only `pathExclusions=["compose"]`; its admitted source and normalized path ledgers nevertheless contain zero `compose` and zero `temp/compose` entries.

## Command and duration

```sh
bun ./📜️script.ts clean taxonomy inventory --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION
```

- Exit: `0`
- Observed wall duration: `00:16:38` (`998 s`)
- Output: `[clean taxonomy inventory] entries=103796 source=a22e0fbe7ef10295f6c9c57b41ce149187b2593f2082085777986d19926ec956 -> .../END-TO-END-TAXONOMY-NORMALIZATION`
- Effective throughput: `104.00 entries/s`; retained-output throughput: `117,216 bytes/s`
- Process samples: at `04:31`, `58.3%` CPU and `3.4%` memory; at `10:20`, `81.2%` CPU and `0.1%` memory; at `16:24`, `89.4%` CPU and `10.6%` memory.
- The command emitted no phase/profile telemetry before its terminal line. The samples prove sustained forward CPU use, but cannot attribute the 998 seconds to Git enumeration, content hashing, reference parsing, classification, aggregation, or serialization. Missing phase evidence is itself an acceptance gap.

The 998-second runtime and 111.562 MiB retained monolith fail the repository's maximum-performance and ticket-evidence-size expectations. They are acceptance blockers even though the process completed successfully.

## Immutable artifacts and digests

- Original canonical inventory: `📊️taxonomy-inventory/🔣️.json`
- Generated inventory note: `📓️taxonomy-inventory/📝️.md`
- Audit ledger: `📊️pre-transaction-v2-current-inventory/🔣️.json`
- Reproducible ticket audit: `🧪️pre-v2-inventory-audit.ts`
- Inventory bytes: `116,981,622` (`111.562 MiB`, `116.982 MB`)
- Raw inventory artifact SHA-256: `f03a718e8069da55f53606add55a8417f1ccb91c1e0ead3f182daa08dfc19f10`
- Source-tree digest: `a22e0fbe7ef10295f6c9c57b41ce149187b2593f2082085777986d19926ec956`
- Inventory digest: `c8c7fcb76232ec2348067167a0df727b090f521c00f2f93def2ff038a60bcefd`
- Taxonomy canonical digest derived at snapshot: `3152db3c4ef25e5813b3931eddf542570970c7f833baf8ed1bb7c5c5fa19da2e`
- Post-guard taxonomy canonical digest when the audit ledger was generated: `7090d73fdcd229e230b5bf94cc04174a352b3d0195aa77959fce06fc081074e7`
- Taxonomy file SHA-256 when the audit ledger was generated: `15ecd7d4f1f516ed902c0b0c25c3cf4b095b692ac277c6de2cc7e60a36757928`

The source-tree and inventory digests were independently recomputed from the stored schema-v1 fields and match byte-for-byte. The snapshot taxonomy digest is the audit-time canonical taxonomy with the exact post-census `temp-compose` exclusion delta reversed: delete `pathExclusions["temp-compose"]`, restore `opaquePathExclusionIds=["compose"]`, and restore the prior area-state sentence. No other taxonomy mutation between census exit and audit-ledger generation is assumed. A later Cargo cache-tag authority changed taxonomy again; its separately recorded strict-green digest is not used to reinterpret this snapshot.

The prior canonical inventory had source-tree digest `e8504fdfe1cb218b37d6abafadde51469c0d128db427db4ac05e22453ac89bc8`. It differs from this run. Therefore the inventories are not merged and no continuity or trend claim is made from their counts.

## Census

| Measure | Count |
| --- | ---: |
| Entries | 103,796 |
| Files | 65,429 |
| Directories | 38,352 |
| Symlinks | 15 |
| Violations | 37,362 |
| Errors | 30,474 |
| Warnings | 6,888 |

The node-kind partition is exactly 103,796. All 103,796 `sourcePath` values are unique. Top-level violations and violations flattened from entries are the same 37,362-record multiset. Code, area, and owner partitions each sum to 37,362. Package roles, including absent directory roles, sum to 103,796. There is no double-count delta.

## Deterministic residual buckets

| Violation code | Count |
| --- | ---: |
| `semantic-stem-ambiguous` | 10,477 |
| `semantic-stem-unresolved` | 7,272 |
| `path-too-long` | 7,038 |
| `opaque-reference-target` | 6,814 |
| `directory-kind-ambiguous` | 1,539 |
| `directory-kind-unresolved` | 1,501 |
| `projection-catalog-unrealized` | 931 |
| `projection-member-unresolved` | 931 |
| `tracked-path-missing` | 250 |
| `file-kind-unresolved` | 225 |
| `package-implementation-destination-unresolved` | 212 |
| `package-implementation-file` | 74 |
| `package-role-unresolved` | 60 |
| `projection-authority-invalid` | 25 |
| `symlink-absolute-target` | 13 |

Top areas by violation membership are `.\ud83e\uddecsemio` 22,225, `✏️s/🔌️plugins` 11,570, `🧰️framework` 2,470, `.cursor` 665, and `♻️mit-bestand` 184. Top owners are `✏️s/🔌️plugins/📕️norm` 3,922, `✏️s/🔌️plugins/🗄️stdio` 2,123, `✏️s/🔌️plugins/🏛️architect` 1,673, the `NX-PLUGIN-V2-AND-ATOMIC-CUSTOM-PLUGIN-RELOCATION` ticket 1,395, and the `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket 1,361. These memberships are exact and are not inferred priorities.

## Fixed contracts and package roles

The three rejection contracts contribute 23 exact admitted identities, each found once:

- 19 `relocate` operations for nested ticket manifests.
- 4 `normalize` operations: two retained ticket Go module files and two historical progress documents.
- Missing rejection identities: `0`.

Package-role partition:

| Role | Count |
| --- | ---: |
| `not-package` | 64,395 |
| absent, all directories | 38,351 |
| `configuration` | 563 |
| `implementation` | 286 |
| `declaration` | 119 |
| `unresolved` | 60 |
| `thin-delegation` | 11 |
| `bootstrap` | 9 |
| `registration` | 2 |

The 60 unresolved package roles split into 40 beneath Rust package-language directories and 20 beneath directories literally named `fixtures`. The latter are recorded as current classifier evidence; no package-language meaning is invented for them. The audit ledger contains the complete fixed-operation and package language/role records.

## Path budget and collision ledger

- Configured maximum: `240` UTF-8 bytes.
- `path-too-long`: 7,038 entries.
- Maximum source and normalized path: `379` bytes.
- By area: plugins 6,502; ticket evidence 526; framework 10.
- Largest owner groups: norm 2,558; architect 1,132; stdio 970; this ticket 511; fem 310.
- Windows-reserved-name violations: `0`.
- Trailing-dot-or-space violations: `0`.

Planner-equivalent grouping over `normalizedPath` yields 176 collision groups: 35 each for byte, NFC, case-fold, and same-kind, plus 36 VS16-fold groups. The groups contain 362 source memberships over 74 unique source paths. Collision-ledger digest: `99ec7d02d98422c4c5443cf3099b565e739cc847864db24e2fd25a631645abb0`. These are a deterministic projection of the stored inventory, not top-level inventory violations; the complete groups and IDs are in the audit ledger.

## Opaque lexical guard

Neither forbidden tree was traversed for this audit. The snapshot strings prove zero admitted source or normalized paths under both `compose/` and `temp/compose/`. After the census exited, the taxonomy/discovery contract was narrowed to exactly the ordered opaque IDs `compose` and `temp-compose`, with paths `compose/` and `temp/compose/`; strict taxonomy validation and lexical string probes were green. The normalization parser was handed the same exact two-ID contract with no fallback. Final v2 inventory must record both IDs in `pathExclusions`, irrespective of whether either prefix is physically absent.

## Cargo cache-tag authority correction

The immutable snapshot contains 62 exact `CACHEDIR.TAG` leaves, all previously `file-kind-unresolved`: 40 are direct children of canonical `🧪️target-*` ticket evidence directories, 19 are nested below the Cargo platform directories `wasm32-unknown-unknown` or `wasm32-wasip2`, and three are below unprefixed `cargo-target`/`scratch-fem-3d-target` directories. Six of the 40 direct cases live in embedded ticket roots beneath framework owners and blocked transaction planning. Those six have exactly four parent identities: `🧪️target-os-errors`, `🧪️target-os-process-pool`, `🧪️target-owned-wasm-core`, and `🧪️target-shell-owned-schema`.

The narrow schema authority now consists of:

- Directory kind `ticket-cargo-target-evidence`: emoji `🧪️`, slug `^target-[a-z0-9]+(?:-[a-z0-9]+)*$`, emoji-only forbidden.
- Fixed filename contract `cargo-cache-tag`: path pattern `**/CACHEDIR.TAG`, authority `Cargo`, scope `{kind:"directory-kind",directoryKindId:"ticket-cargo-target-evidence"}`.

The basename pattern cannot authorize a file by itself: runtime resolution is conjunctive with the immediate registered parent kind. It therefore covers the 40 evidenced direct Cargo target roots, including all six transaction blockers, but intentionally does not cover the 19 platform descendants or three unprefixed historical target roots. Those 22 remain explicit physical/schema decisions rather than receiving a suffix or basename wildcard allowance.

Strict taxonomy validation is green after the correction. Ticket-local TDD command:

```sh
bun test .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cargo-cache-tag.test.ts
```

Result: `3 pass`, `0 fail`, `16 expect()` calls. The test proves zero validation problems, resolution of all six exact embedded blocker paths, and rejection without the directory-kind context or with a counterfeit parent. Post-correction taxonomy: 221,710 bytes, file SHA-256 `c16838c76f5cd4d38fabc32eb375a1f835d969894292dfebcba57911d6d17930`, canonical digest `c6760dcac8535b794ca713652745e363eb8f46de7e80993bd2f1b25b3f2bcacc`.

## Deterministic sharding acceptance design

The audit simulated, but did not materialize, a lossless owner-shard layout from the immutable snapshot:

- Manifest schema `1`; owner IDs sorted by UTF-8 bytes; entries inside each owner sorted by `sourcePath` UTF-8 bytes.
- Shard envelope: canonical JSON `{entries, ownerId, schemaVersion}`.
- Shard paths use the manifest-declared byte-order owner ordinal and part ordinal, avoiding unsafe owner text in artifact paths.
- Exclusive byte ceiling: `5,242,880` bytes; every shard must be strictly smaller than 5 MiB.
- Simulated result: 3,613 owners, 3,620 shards, 103,796 entries, zero duplicate `sourcePath` values.
- Largest individual entry: 1,337,534 bytes; largest shard: 5,242,658 bytes.
- Ordered shard-ledger digest: `6131ce56c05671b2277009bfe465509496d0b6ac155794029fe221aef4acc7b2`.

The permanent writer should stream one owner at a time into byte-capped canonical shards and write one small manifest containing: schema/taxonomy versions, source-tree and inventory digests, ordered shard records (`path`, owner, part, byte count, entry count, first/last source path, SHA-256), global counts, exclusion IDs, and a digest of the ordered shard records. It must reject an entry that cannot fit alone, duplicate source paths across shards, non-byte-sorted boundaries, a shard at or above 5 MiB, any shard digest mismatch, and any manifest count/digest that cannot reconstruct the canonical inventory. Top-level violations should be reconstructed deterministically by flattening entries, not stored as a second 37,362-record payload.

This design keeps every retained file beneath the 5 MiB ceiling, makes partial owner review possible, and preserves full digest accounting without duplicate entries. The current 111.562 MiB monolith remains untouched as required.

### Permanent CLI evidence taxonomy closure

The CLI permanently names eight ticket artifact directories in `/Users/ueli/Documents/semio/📜️script.ts:17932`: data directories `📊️taxonomy-{inventory,plan,apply,verification}` and summary directories `📓️taxonomy-{inventory,plan,apply,verification}`. It writes kind-only leaves `🔣️.json` or `📝️.md` at line 18000. None of those eight exact directory identities has a `semanticDirectoryKinds` authority today. The only transaction-specific registered kinds are `taxonomy-transaction` (`🧾️taxonomy-transaction`) and its `transaction-digest` (`🔖️<sha256>`) child; the generic `plan` kind is `🧩️plan` and does not authorize `📊️taxonomy-plan`.

The snapshot happened to admit the existing `📊️taxonomy-inventory` and `📓️taxonomy-inventory` directories with no violation, plus the physical Markdown summary leaf. That is not registry convergence: schema v1 does not record a directory kind on the entries, and no exact registry key exists. The JSON data leaf is intentionally absent from the snapshot because it is published after enumeration. Plan/apply/verification directories were not present, so they have no current-tree evidence at all.

The exact schema-first closure should add these non-overlapping kinds, with no wildcard:

| Kind ID | Emoji | Exact slug pattern |
| --- | --- | --- |
| `taxonomy-inventory-data` | `📊️` | `^taxonomy-inventory$` |
| `taxonomy-plan-data` | `📊️` | `^taxonomy-plan$` |
| `taxonomy-apply-data` | `📊️` | `^taxonomy-apply$` |
| `taxonomy-verification-data` | `📊️` | `^taxonomy-verification$` |
| `taxonomy-inventory-summary` | `📓️` | `^taxonomy-inventory$` |
| `taxonomy-plan-summary` | `📓️` | `^taxonomy-plan$` |
| `taxonomy-apply-summary` | `📓️` | `^taxonomy-apply$` |
| `taxonomy-verification-summary` | `📓️` | `^taxonomy-verification$` |
| `taxonomy-inventory-shards` | `📊️` | `^shards$`, parent `taxonomy-inventory-data` |
| `taxonomy-inventory-shard-digest` | `🔖️` | `^[a-f0-9]{64}$`, parent `taxonomy-inventory-shards` |

The data and summary emoji come from the existing permanent CLI paths; the digest child reuses the registered transaction-digest precedent. The convergent inventory layout is:

```text
📊️taxonomy-inventory/
  🔣️.json                         # small canonical manifest
  📊️shards/
    🔖️<sha256-of-canonical-shard>/
      🔣️.json                     # canonical shard, strictly < 5 MiB
📓️taxonomy-inventory/
  📝️.md                           # human summary
```

Each digest directory identifies exactly one canonical shard payload. The manifest retains byte-sorted owner/part order and maps it to the digest directory, so content addressing does not replace semantic owner identity. The writer must compute the digest before publication, use the physical JSON leaf `🔣️.json`, reject a payload at or above 5,242,880 bytes, reject duplicate digest paths with unequal bytes, and fail if any referenced shard is absent or any unreferenced shard remains. Plan/apply/verification retain their current single-manifest physical JSON leaves and Markdown summaries; if any later exceeds the same ticket limit, it needs its own separately registered, operation-specific shard root rather than inheriting the inventory contract.

## Performance acceptance design

The v2 rerun should be accepted only when all of the following hold on the same workspace/machine class:

1. Progress begins within 2 seconds and emits at least every 5 seconds with closed phase IDs: `git-index`, `content-evidence`, `reference-parse`, `classify`, `aggregate`, and `write-shards`.
2. Each event records `completed`, `total`, `elapsedMs`, `ratePerSecond`, worker count, and RSS bytes. Cancellation is checked between bounded batches in every phase.
3. One cold run completes within 300 seconds for approximately 104k entries; a warm run may not be slower than the cold run by more than 10%. This is at least a 3.32x wall-time improvement over the 998-second baseline.
4. Post-index sustained classification throughput is at least 1,000 entries/s. No phase may consume more than 60% of wall time without a phase-specific profile attached to the ticket.
5. Peak RSS remains below 2 GiB, no canonical monolith is constructed in memory or written, every shard is below 5 MiB, and manifest publication is atomic after all shard digests verify.
6. Two consecutive runs over the same source/taxonomy digests produce byte-identical manifests, shard paths, shard bytes, source-tree digest, inventory digest, group counts, and zero-double-count ledger.

The 300-second and 1,000-entry/s thresholds are explicit acceptance targets, not claims about the present implementation. The final v2 run must retain its phase log and machine-readable manifest so a later regression can be compared by matching source and taxonomy digests only.

## Acceptance checks for the required v2 rerun

- Exact lexical exclusions are `compose/` and `temp/compose/` before Git or filesystem access; zero forbidden-prefix entries.
- Every entry contains `mode` and `size`; every symlink contains raw link-target evidence; source-tree digest covers all three fields.
- Entry/file/directory/symlink, violation, fixed rejection, package role/disposition, path-budget, and collision counts have zero partition deltas.
- The owner-shard manifest satisfies the sub-5 MiB, digest, ordering, uniqueness, and atomic-publication contract.
- Runtime/profile evidence satisfies the performance contract above.
- No comparison to this pre-v2 snapshot is made unless both source-tree and taxonomy digests match; this snapshot's digest is preserved as historical evidence.
