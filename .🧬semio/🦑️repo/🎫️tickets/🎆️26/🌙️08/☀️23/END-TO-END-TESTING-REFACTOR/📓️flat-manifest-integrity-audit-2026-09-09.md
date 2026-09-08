# Flat Manifest Integrity Audit

## Scope and Method

Read-only validation covered the flat JSON string arrays in the coordinator, native, TypeScript final, initial inline, and legacy attribution reports, plus the ten Rust authored-array parts named by `📓️test-layout-rust-changed-files-2026-09-08.md`. Each JSON block was parsed; all values were checked for duplicates, empty or traversal-like forms, absolute or URI forms, `AGENTS.md`, and the `🗑️generated` directory. Current filesystem existence was checked only to validate paths explicitly identified as current destinations in their reports.

## Array Integrity

All 15 arrays parse as flat JSON string arrays and are internally unique. No array contains a malformed, absolute, generated-directory, or `AGENTS.md` path.

| Manifest | Entries | Current-existence note |
| --- | ---: | --- |
| Coordinator changed files | 207 | 15 entries no longer exist; two are invalid references to Rust authored parts 011 and 012, while the other 13 are historical/deleted sources or fixtures and are not labelled current destinations. |
| Native closure | 106 | Explicit destination check has two failures below. |
| TypeScript final | 196 | Explicit destination check has one failure below. |
| Initial inline | 526 | All explicit destinations exist. |
| Legacy attribution | 130 | All 59 proven destinations exist. |
| Rust authored parts 001–009 | 900 each | 8,100 entries total. |
| Rust authored part 010 | 280 | Rust total is 8,380. |

The Rust index declares ten authored parts and 8,380 unique authored paths. The ten arrays total exactly 8,380, so their reported cardinality matches. Their entries are not individually labelled old versus current destination; 458 values do not exist in the current tree, including 77 direct `🧪️tests/<case>/<implementation>` shapes. That is not reported as a failed proven destination: the index separately records deleted/matched source history, and only its 17 mapping-part reports identify old-to-new relationships.

## Findings

1. The coordinator array references nonexistent `📓️test-layout-rust-authored-paths-2026-09-08-part-011.md` and `part-012.md`. The Rust index names exactly parts 001–010. These two report references are stale.

2. The TypeScript final report's 90 unique explicit table destinations contain one nonexistent claimed canonical leaf:

   `temp/merge/mit-bestand/präsentation/33.projektetage/js/🧪️tests/📽️deck/🟦️.ts`

   The corresponding source `temp/merge/mit-bestand/präsentation/33.projektetage/js/index.ts` still exists; no replacement is inferred here.

3. The native report's 32 explicit Go/Python destination strings contain two nonexistent claimed fixture targets:

   `♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/check_stage1.py`

   `♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/check_stage2.py`

   The neighboring `dump_snapshot.py` and `golden/` fixture files exist, but these two do not. No replacement is inferred.

4. The initial inline report says it proves 171 old-to-current mappings, while its `Proven Baseline Test-File Moves` table has 179 rows. Its separate guarded-extraction table has 71 rows, so the body records 250 distinct current destinations rather than the 242 implied by 171 plus 71. All 250 table destinations exist; this is a report-count discrepancy, not a missing-destination finding.

5. The legacy report's correction states the flat closure array has 130 paths, which matches the parsed array: 59 attributable old paths, 59 proven destinations, 11 changed callers/configurations, and the report. Its earlier phrase that the array contains “every baseline legacy path” conflicts with the stated exclusion of the six ticket-private scratch paths. The evidence rows retain all 65 baseline paths, so the correction gives the operative scope.

No runtime, Nx, source, manifest, or Git-state operation was performed.

## Coordinator Resolution

Removed the two stale Rust authored-part references from the root array; the Rust index remains the authority for its ten actual arrays. The temporary merge presentation case already exists one directory above the intermediate `js/🧪️tests` path, and its guarded caller already imports that final location; corrected the report and added the final path to its array. The two Python stage modules already exist under the data/model semantic owners and were previously compiled/imported by the coordinator; corrected the native destination rows and included final paths while retaining intermediate removed paths. Corrected the legacy scope sentence to exclude six un-attributed historical scratch paths. The initial-inline report was concurrently being extended during this audit and now reports 179 canonical mappings and a validated 536-path array; the extra follow-up tables include separately classified noncanonical/support evidence. These were metadata corrections; no production or test source was changed.
