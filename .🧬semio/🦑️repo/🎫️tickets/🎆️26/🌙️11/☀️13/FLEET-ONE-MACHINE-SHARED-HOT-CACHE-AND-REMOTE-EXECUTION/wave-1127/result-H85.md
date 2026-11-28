# H85: remaining pre-existing reds of the fleet build system

| # | Red | Verdict | Cause (landing) | Fix | State |
|---|-----|---------|-----------------|-----|-------|
| 1a | fleet-status guard "never loads the graph" | pre-existing since H5 readers (00:24) | the offload reader imported the whole uploader (spool depth/bound), which imports `📥️inputs/🔏️revision` | spool helpers moved to new member `📏️spool-size` (policy + fs only, own tests incl. an import-allow-list test); reader and 9 importers use it | guard 4/0 |
| 1b | guard "no spawnSync" | pre-existing: readers (H5) + `⌨️command` hostname runner | `spawnSync("ps")` in the reader; stdio-inherit runner for `repo fleet hostname` inside the status command | reader: async `execFile`; runner moved to the topology command module as default of `run` | green; readers stay non-blocking |
| 2 | mcp build-surface: `bench latency` uncovered | regression of H74/H75 (`bench latency` added to the catalogue without a policy row) | no exposed/excluded row | excluded with bilingual reason (timing only valid on an unloaded machine, like `bench acceptance`) | 6/0 |
| 3 | graph `farm-isolation` 5/5 | transient (closure compiler edit churn, H82's observation); green on m06 and locally | none | none needed | 5/0 |
| 4 | retention-io, fleet count, counters (H81) | transient/fixed by peers; all three green locally and remote | none | none needed | 10/0, 20/0, 4/0 |
| 5 | units test (H48) | still 22/4, cause not fixed | (a) `Live Original resource control issuance required` at `reservation.recheck()` after `compileRustUnitV1` and `Original reader control changed` (test compares `originalControl.signal` with its own control; the product now hands a refined child control); (b) `Rust unit output inventory differs` in the child entry.mjs, plus `[lease-order] compiler-capacity (rank 3) requested while rust-unit namespace (rank 5) held` just before it; (c) `consumed-byte Source checksum or membership differs: .tmp/ca-*/lib.rs`; (d) `prepared.inputs.length` 0 in the ordinary Nx test | landed diagnostics: the inventory error now names the undeclared/duplicate/missing path; the rerun shows the exact file | OPEN: rerun units after the 15:00 slot, then fix |
| 6 | `📥️parent` 40/8 | pre-existing; H77 fixed the envelope, the rest was a drifted test (H43 single lease-image open, stale extracted-function ports hidden by the cascade) | test corpus and ports | corpus 7 rows -> 6 rows for the single open, with real image-change cases (recheck throws, close failure kept in the aggregate), exact traces; ports for freemem/chunk plan/read-plan/retained receipt/stage reader; deterministic cancel-between | 52/0 |
| 7 | H75 `🩺️diagnostics/🧪️tests` in no suite | pre-existing (H75) | no suite row | suite `build-diagnostics` + target `test-build-diagnostics` through `repo land` | suites law green, diagnostics 4/0 |

## Not verified / open
- `🧪️tests/📋️manifest` (unit-cache main dir) is red: "B hits the fleet with its first lookup" got `miss`; uploader, fleet 20/0 and spool tests pass, so not from the spool move; probably the H81 key change. Not investigated.
- Units (5) remains red: needs a rerun with the new diagnostics.
- `repo typecheck` of the landed files passed (53 files); not run on the parent test file (test-only).
- Scripts: `T/🧰️scripts/H85/`.
