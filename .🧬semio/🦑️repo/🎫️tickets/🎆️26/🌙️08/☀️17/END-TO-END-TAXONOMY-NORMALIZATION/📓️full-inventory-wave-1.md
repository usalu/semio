# Full Inventory Wave 1

## Deterministic census

- Command: `bun ./📜️script.ts clean taxonomy inventory --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION`.
- Completed in approximately 60 seconds after the indexed reference-resolution change.
- Inventory entries: 103,892.
- Files: approximately 64,721; directories: approximately 37,987; remaining entries are symlinks and explicitly retained ticket evidence.
- Source-tree digest: `e8504fdfe1cb218b37d6abafadde51469c0d128db427db4ac05e22453ac89bc8`.
- Lexical exclusion: `compose`; active excluded tree: none, because the developer intentionally deleted Compose.
- Canonical machine artifact: `📊️taxonomy-inventory/🔣️.json`.

## Current blocking findings

The canonical aggregate contains 41,539 entry violations before the next repair waves. Aggregate counts in the JSON must not be added to per-entry counts because the aggregate is the stable projection of those same findings.

| Finding | Actual affected entries | Immediate diagnosis |
|---|---:|---|
| `file-kind-unresolved` | 15,767 | 15,755 are historical-ticket files falsely claimed by the scoped evidence rule even when their extension belongs to a global physical kind. The resolver must fall through when the scoped extension does not match. |
| `path-too-long` | 14,511 | Concentrated in deeply nested plugin artifact/schema/mutation/test trees; requires a separate minimal-frontier audit rather than arbitrary truncation or hashing. |
| `semantic-stem-unresolved` | 7,358 | Named assets, fixtures, schemas, and historical evidence need contextual semantic-folder resolution. |
| `directory-kind-unresolved` | 1,373 | Production artifact/test vocabulary plus 70 retained historical-ticket directories still need registered contextual semantics. |
| `directory-kind-ambiguous` | 915 | Mostly ticket evidence and glTF plus a small set of framework roots; broad slug grammars require owner/parent precedence. |
| `semantic-stem-ambiguous` | 853 | Named files match several broad directory kinds; file-stem context must select an exact semantic-folder kind. |
| `package-role-unresolved` | 341 | Tool configuration, fixtures, and unsupported package syntax need exact precedence/classification. |
| `package-implementation-destination-unresolved` | 390 | Includes actual implementation plus fixed `📜️script.ts`/`📋️project.json`/tool config that must classify as configuration before purity analysis. |
| `package-implementation-file` | 16 | Confirmed implementation remains beneath package boundaries and must relocate. |
| `symlink-absolute-target` | 13 | Must be resolved without following links across scope. |
| Windows reserved / trailing dot-space | 2 | Independently audited with the path-budget lane. |

## Invariants confirmed before mutation

- Physical file kinds are one-to-one by extension chain: Rust `🦀️`, TypeScript `🟦️`, JSON `🔣️`, and Markdown `📝️`; semantic role variants were removed.
- The focused normalization suite is 15/15 after replacing the obsolete `.test.ts` expectation.
- Inventory does not traverse the intentionally absent Compose tree.
- No full apply is permitted until the next canonical plan has zero unresolved findings.
