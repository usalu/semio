# `important` Semantic-Stem Residual Authority

## Scope and evidence boundary

This is a read-only residual census of the retained pre-transaction-v2 inventory. No production file, Git state, `compose/**`, or `temp/compose/**` path was changed or accessed. The inventory is evidence, not final v2 acceptance, because it predates the frozen mode/size/raw-symlink transaction record.

- Inventory: `📊️taxonomy-inventory/🔣️.json`
- Inventory bytes: `116,981,622`
- Inventory SHA-256: `f03a718e8069da55f53606add55a8417f1ccb91c1e0ead3f182daa08dfc19f10`
- Cohort predicate: `semanticStem == "important"` and violation code `semantic-stem-unresolved`
- Current content sizes were read only from the 460 named non-excluded files because the pre-v2 entries do not carry the final size field.

Representative evidence commands, run from the repository root:

```sh
bun -e '/* parse the retained inventory; select semanticStem=important plus semantic-stem-unresolved; group bytewise */'
bun -e '/* read only each selected file and its co-located 🎫️ticket.json; cross-tab status, shape, and empty/nonempty */'
```

## Exact census

The cohort is one physical form, not 460 distinct semantic meanings:

| Dimension | Exact result |
|---|---:|
| Entries / distinct source paths | 460 / 460 |
| Distinct inventory owners | 449 |
| Area | 460 `.🧬semio` |
| Node/file kind | 460 file / 460 Markdown |
| Basename | 460 `📌️important.md` |
| Normalized basename | 460 unchanged |
| Mode | 460 `100644` |
| Existing semantic kind | 0 |
| Content bytes | 293,412 |
| Empty / nonempty | 409 / 51 |
| Distinct content hashes | 52 |
| Incoming / outgoing references | 3 / 0 |
| Only `semantic-stem-unresolved` | 459 |
| Also `path-too-long` | 1 |

Owner multiplicity is stable: 446 owners contain one row, two owners contain two, and one owner contains ten. Relative to the inventory owner, 447 rows are owner-root files, 12 are one directory below it, and one is a depth-nine fixture file.

### Authority and lifecycle cross-tab

`🎫️ticket.json` was required in the same directory as the `📌️important.md` file. A ticket-looking path alone was not treated as authority.

| Physical shape | Adjacent manifest status | Empty | Nonempty | Total |
|---|---|---:|---:|---:|
| Owner-root | closed | 243 | 21 | 264 |
| Owner-root | open | 151 | 22 | 173 |
| Owner-root | status missing/invalid | 0 | 1 | 1 |
| Owner-root | manifest absent | 3 | 6 | 9 |
| Embedded one-level root | open | 10 | 0 | 10 |
| Embedded one-level root | manifest absent | 2 | 0 | 2 |
| Deep fixture | manifest absent | 0 | 1 | 1 |
| **Total** |  | **409** | **51** | **460** |

Accounting is exact and non-overlapping:

```text
460 = 438 canonical owner-root files with adjacent manifests
    + 10 legacy embedded ticket roots with adjacent manifests
    + 12 files without adjacent ticket-manifest authority
```

The one invalid-status manifest is:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️05/FIX-ENGAGEMENT-SUGGESTION-CLICK/🎫️ticket.json
```

Its sibling `📌️important.md` is 517 bytes and must remain blocked until the manifest is valid.

The 12 paths without adjacent manifest authority comprise nine owner-root paths, two empty phase subdirectories below `INTERACTIVE-JOB-RUNTIME-REFACTOR`, and one deep window-policy fixture. The deep fixture is a 139-byte placeholder describing an intentionally leafless presence facet; it is not ticket compulsory-action data. It should be governed by its fixture/`presence` owner and ultimately use a parent-owned kind-only Markdown leaf, not the ticket-important projection.

Ten one-level rows do have adjacent ticket manifests. They are real legacy embedded ticket roots already covered by the nested-ticket-manifest rejection/relocation concern; they must relocate before the canonical ticket-important projection is applied.

## Authority conclusion

Exactly one semantic directory meaning is justified: **ticket compulsory actions**.

Its canonical physical projection is:

```text
<canonical ticket root>/📌️important.md
→ <canonical ticket root>/📌️important/📝️.md
```

This follows the plan's semantic-directory and physical-leaf invariants: `important` is meaningful semantic identity, while Markdown is the physical format. It must not become a fixed-filename exception, and the directory kind must not globally authorize every `important` stem.

The valid authority is conjunctive:

1. The owner is an exact member of the existing `ticket-slug` fixed-directory contract.
2. The same owner directory contains the exact `ticket-manifest` fixed filename.
3. The manifest resolves and satisfies the ticket schema.
4. The source is exactly the Markdown file `📌️important.md` directly under that owner.
5. The destination is exactly `📌️important/📝️.md` under the same owner.

A broad path pattern, the basename alone, or a directory that merely resembles a ticket is insufficient.

## Minimal schema-first closure

Add one semantic kind and one exact, tagged projection contract rather than broadening generic stem inference.

### Semantic directory kind

Recommended ID: `ticket-important`.

```json
{
  "emoji": "📌️",
  "slugPattern": "^important$",
  "allowEmojiOnly": false,
  "inferWithoutEmoji": false
}
```

The existing `panels` kind also uses `📌️`; this is not ambiguous because the registered slug patterns are disjoint. The contract below, not the emoji, supplies ownership authority.

### Exact projection contract

Recommended closed record, named `ticket-important-markdown-v1`:

```json
{
  "contractId": "ticket-important-markdown-v1",
  "ownerFixedDirectoryContractId": "ticket-slug",
  "requiredSiblingFixedFilenameContractId": "ticket-manifest",
  "sourceFileKindId": "markdown",
  "sourceFilename": "📌️important.md",
  "semanticDirectoryKindId": "ticket-important",
  "destinationDirectoryName": "📌️important",
  "destinationFilename": "📝️.md"
}
```

Discovery must validate exact keys, NFC, resolvable contract IDs, disjoint source authority, and source/destination file-kind agreement. Resolution must check fixed-directory identity membership and sibling-manifest identity conjunctively; a matching pattern alone must never authorize a path. Normalization should consume the loaded contract before generic semantic-stem inference and emit a structured reference edit for each incoming path token.

Current production regions affected by a later implementation lane:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: `semanticDirectoryKinds` and a new exact semantic-file projection-contract registry.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`: `SemanticDirectoryKindSpec`, strict taxonomy validation, and exact owner/sibling contract resolution. Current `parentKindIds` matching cannot express a fixed ticket root plus co-located manifest.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`: strict parser and semantic-file canonicalization consumer. No change was made in this audit.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go`: `Ticket.ImportantPath`, `GetImportantFilePath`, create/update/reload, and finish-ticket lifecycle ownership. The owner is configurable repository code, so the legacy basename is not an unconfigurable fixed-name authority.

### Lifecycle dispositions

Path authority and lifecycle validity are separate:

- 173 open canonical ticket roots: eligible for projection.
- 10 open embedded ticket roots: relocate the ticket root first, then project.
- 243 closed and empty: lifecycle residue; deterministically remove rather than create a new empty semantic document.
- 21 closed and nonempty: fail closed as a ticket lifecycle contradiction; never discard content automatically.
- 1 invalid-status manifest: fail closed until schema-valid.
- 12 without an adjacent manifest: remain unclaimed by this contract. Do not invent authority.

The Go ticket owner must then write/read the canonical `📌️important/📝️.md` path and retain the finish rule: nonempty compulsory actions block closing; successful non-bulk close removes the compulsory-action artifact. Bulk/legacy residue needs an explicit lifecycle transaction, not compatibility behavior.

## References, collisions, and path budget

Three incoming references require deterministic structured edits:

- `NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT/🔧️wave0-scaffold.mjs` references its ticket's important file.
- `FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🔧️update-handoff-tests.mjs` references its ticket's important file.
- `FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🔧️write-handoff.mjs` references its ticket's important file.

The projected suffix is eight UTF-8 bytes longer than the source suffix. Across all 460 hypothetical mappings there are zero existing destination files, zero existing destination directories, and zero duplicate destinations. One deep fixture grows from 284 to 292 bytes and remains over the 240-byte policy; this is further evidence that it must not be claimed by the ticket contract. Across the 448 adjacent-manifest rows, the projected maximum is 228 bytes and none exceeds 240 bytes.

## Permanent TDD acceptance checks

1. Strict taxonomy load resolves `ticket-important-markdown-v1` and rejects missing, extra, empty, non-NFC, unresolved, or overlapping fields.
2. A canonical ticket root plus exact adjacent manifest projects byte-for-byte to `📌️important/📝️.md`.
3. The same source basename without the exact sibling manifest remains `semantic-stem-unresolved`.
4. A counterfeit path that matches a ticket-looking pattern but is not an exact fixed-directory-contract member is rejected.
5. An embedded ticket root remains rejected until the existing relocation projection places it at a canonical ticket root.
6. The deep `presence` fixture is not resolved as `ticket-important`.
7. Open, closed-empty, closed-nonempty, and invalid-status fixtures produce the four explicit dispositions above.
8. The three JavaScript reference locations produce structured edits with preimage hashes, and a stale preimage blocks apply.
9. A collision fixture with an occupied `📌️important/📝️.md` fails closed; no suffixing or hashing is allowed.
10. A language-neutral JSON golden contains the exact source, authority IDs, destination, disposition, and reference edits. A test-only `fast-glob`/`picomatch` parity check may validate candidate selection, but exact identity plus adjacent-manifest checks remain the repository authority.
11. Census regression accounting is exact: `460 = 438 + 10 + 12`, with 409 empty, 51 nonempty, and no double count.

## Residual blockers

- The transaction-v2 inventory must be rerun before final acceptance; this report intentionally preserves the pre-v2 evidence digest.
- Ten embedded tickets depend on the already identified nested-ticket relocation.
- Twenty-one closed tickets retain nonempty compulsory actions and require an explicit lifecycle decision.
- One adjacent manifest lacks a valid status.
- Twelve files have no adjacent ticket-manifest authority; nine are ticket-root-shaped historical orphans, two are empty phase artifacts, and one is a fixture placeholder. They must stay unresolved until their actual owners supply exact evidence.

No global fixed filename, basename wildcard, path-only ticket inference, or semantic emoji invention is warranted.
