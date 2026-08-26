# Taxonomy CLI Artifact Directory Kinds

## Result

Implemented the permanent schema authority proposed by the pre-v2 inventory report. The CLI writer was not changed.

The schema now owns exactly eight operation roots:

- Data: `taxonomy-inventory-data`, `taxonomy-plan-data`, `taxonomy-apply-data`, `taxonomy-verification-data` for the exact `📊️taxonomy-*` names.
- Summaries: `taxonomy-inventory-summary`, `taxonomy-plan-summary`, `taxonomy-apply-summary`, `taxonomy-verification-summary` for the exact `📓️taxonomy-*` names.
- Sharding: `taxonomy-inventory-shards` for exact child `📊️shards` beneath `taxonomy-inventory-data`; `taxonomy-inventory-shard-digest` for exact `🔖️<64 lowercase hex>` beneath the shard root.

The previous broad `taxonomy-data` and `taxonomy-note` patterns overlapped all eight exact roots and made the first strict test red with eight ambiguous-resolution problems. They had only one current-tree match each, the inventory paths now owned exactly, and no production consumer. They were removed rather than retained as a compatibility surface.

Discovery strict validation now fixes every ID, emoji, exact anchored slug pattern, `allowEmojiOnly:false`, key set, and parent list, and proves each canonical name resolves uniquely. The digest child is parent-scoped separately from the existing transaction digest kind.

## TDD evidence

```sh
bun test .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cli-artifact-directory-kinds.test.ts .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cargo-cache-tag.test.ts
```

Final Bun 1.3.14 result: `6 pass`, `0 fail`, `33 expect()` calls. The three new tests prove strict taxonomy validity, exact resolution of all eight operation roots, shard-parent scoping, inventory/transaction digest separation, and rejection of the new shard identity for foreign parents or non-hex slugs.

## Digests

- Taxonomy: 222,726 bytes; file SHA-256 `5174ae86579fa2abbe030b10c8585d46d4dd6e2c34c2124a5d4816c68a5ca903`; canonical digest `dea32a2ae9b8364ba809d4f82689ac6a6bfdf8a830fd3c9dae09fc85771b47d8`.
- Discovery: 269,678 bytes; SHA-256 `843fe3304e80c5feb96b07e43c12fa9d6c04e1fb6728c2d1d6506b3813707f9d`.
- Ticket test: 2,322 bytes; SHA-256 `2a3e1f1d487b666f725909ed90fd72d5073e9ac0caa08b0fa9b4173e86679a9f`.

## Append-only transaction attempts

A rolled-back journal is immutable audit evidence and therefore cannot occupy the retry path. The schema now freezes this canonical hierarchy:

```text
<ticket>/🧾️taxonomy-transaction/🔖️<planDigest>/🔂️attempts/🔢️<six-digit ordinal>/🔣️.json
```

`transaction-attempts` is exact child `🔂️attempts` beneath `transaction-digest`; `transaction-attempt` is exact `🔢️[0-9]{6}` beneath that collection. Parent counterexamples and five-/seven-digit ordinals reject. The permanent test first failed with `transaction-attempts` unresolved, then completed with `4 pass`, `0 fail`, and `38 expect()` calls. Its language-neutral JSON golden covers both valid ordinal boundaries and all parent/length negatives; test-only `picomatch` independently reproduces the lexical candidate set while the repository resolver supplies parent authority.

The allocation contract is append-only: attempt `000001` is first; no-follow inspection plus atomic directory creation chooses the next ordinal; a nonterminal attempt blocks fresh allocation and must be resumed; `rolled-back` permits the next ordinal; `committed` makes a fresh apply stale. Resume binds its strict journal ordinal and plan digest to an exact existing canonical attempt path. Staging and backups belong to that attempt directory.

The attempt owns three exact active-only semantic children: `🚧️stage`, `💾️backup`, and `🔒️lease`. The lease contains a kind-only JSON leaf and supplies exclusive resume authority; terminal attempts retain only their journal leaf. Atomic crash recovery also owns exact transient identities for attempt publication (`🚧️prepare-<ordinal>-<pid>-<uuid>`), journal WAL (`🚧️journal` beneath stage), and lease preparation/quarantine (`🚧️lease-<pid>-<uuid>-(preparing|stale)` beneath backup). Wrong parents, zero PIDs, malformed UUIDs, pluralized backup, and suffixed canonical lease names reject. These additions were developed red-to-green through the same language-neutral and third-party parity boundary.

At this checkpoint strict taxonomy validation remains empty. Current fingerprints are taxonomy 225,426 bytes / file SHA-256 `db5bc86a4c2c4102e8af93ffa9be4fba3177da15548b97fcf533e32337992a6a` / canonical digest `236da3f185cf02c05df49778e749294ccdca83253e978e16e18d87b88f9d8438`; discovery 274,075 bytes / SHA-256 `7eed1dee1ccc492e6883da63db72defc62487c641e481fb7bb918f229332ace7`; test 5,064 bytes / SHA-256 `16a88f3c6ca9f8a22b56e236f9965ec43c9c76e696d647704c0d81e3e82fb340`; golden 3,143 bytes / SHA-256 `2c473e48aa626ae62d4146525a6b5e074984aa9cb14f45273d866e4f960288c4`.

The combined permanent schema gate after the complete attempt hierarchy additions completed with `14 pass`, `0 fail`, and `122 expect()` calls across five files in 6.88 seconds.

The broader library `loadTaxonomy|validateTaxonomy` selector also remained green with `43 pass`, `196 filtered out`, `0 fail`, and `222 expect()` calls in 4.67 seconds.
