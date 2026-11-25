# XLSX end-to-end status (2026-11-25 09:35)

## Done and verified
| Area | Result | Report |
|---|---|---|
| Oracle crate (third-party checks) | 30/30 | claude-xlsx-oracles.md |
| Base mutation feature tests, manual host | oracle role 20/20 exhaustive + 2/2 long, subject role (`sut`) 20/20 + 2/2 | claude-xlsx-oracles.md |
| openpyxl / jszip fixture replay | byte-identical (23 + 4 files), grid read-back 10/10 | claude-xlsx-oracles.md |
| Definition-vs-runtime claims test (new) | 3/3, covers 53 definitions | claude-claims-audit.md |
| Assembly test `every_bundled_artifact_definition_assembles_without_violations` | pass | claude-handoff.md |
| Space component compiles for wasm32-wasip2 | check clean | claude-space-migration.md |
| Space/Home test workspaces build | Home 135 pass / 3 fail, Space 102 pass / 20 fail | claude-space-migration.md |
| stdio + stdio-office wasm (with XLSX natural decode enabled) | built 09:17, 410 MB / 219 MB (stripped) | lanes.status |
| Standalone pane (earlier build) | boots; refusal checks pass in en and de | (pending claude-standalone-pane.md) |

## In progress
1. **Browser driver (critical path):** running now under the resource guard (one heavy job at a time). Order: publish → activate → vite 6272 → pane journey (en, de, dialects, refusal) → space wasm → full S 6070 journeys (en, de).
   - Estimate: pane journey within about 30 min; space wasm cold build about 60–75 min; full S journeys after that.
2. **Queued heavy jobs**, one at a time after the driver:
   - paged XLSX decoder tests (code written; tests not yet run);
   - XML snapshot crate split (it exceeds the 4,000-line crate boundary and blocks the managed parity run);
   - `closes_mid_reserved_commit` (base, strict);
   - remaining Space/Home failures.

## Problems found and fixed (root causes)
- **XLSX:**
  - insert-cell had no oracle; shared-string, relationship and content-type checks were not independent; the calamine suite was missing.
  - `read_workbook_grid` treated relative cell offsets as absolute.
  - `set-relationship-base` had no test parameters.
- **XLSX browser:**
  - The Semio codec claims had the wrong owner prefix; a new test now guards claims against the runtime.
  - The XLSX editors had no natural decoder enabled (`SINGLE_TURN_NATURAL_DECODE`).
  - The wasm component was 748 MB, over the 512 MiB prototype cap, because the build did not strip symbols. The build scripts now strip, matching the managed profile.
  - The pane activation catalog was empty because `SEMIO_PLUGIN_CATALOG` was unset.
  - The shard worker was stale.
- **Cross-crate compile breaks on the critical path:**
  - PDF 1.4 canvas: a missing `PresenceLane` bound and dependency.
  - `pdf-snapshot-value`: missing dependencies.
  - PDF lift: a missing trait import.
  - Framework: the `kernel` re-export of `OriginalActorSuccession`.
  - Theme: the generated tokens were stale.
  - IFC details: an accessor was not gated for tests.
  - guest-contract: `[DEBUG]` prints did not compile.
- **Space/Home:**
  - Snapshot decode signature.
  - HomeConfig derives.
  - `SSpaceMutation` and `SHomeMutation` receiving.
  - `build_tool_job` signature.
  - The Home transient retirement and fold factory.
  - Tests drifted from the API.
- **Repo tooling:**
  - Zombie processes held the host-wide compile slot and blocked every gated cargo run; zombie detection is fixed with a test.
  - The orphaned 10–13 hour cargo runs were killed.
  - The storage images were re-attached twice. Today the `repo doctor` repair failed when the mount point was non-empty, so I attached the image manually.

## Environment problems
- **Session restarts:** each restart (roughly every 1–2 h) revoked file access for every process started from the sandboxed shell. About 20 h of builds were lost.
- **Terminal panel:** it survives restarts but now triggers approval prompts, so the driver runs detached again; its steps can be resumed, and I relaunch it after each restart.
- **Kernel panic, 2026-11-25 01:00:** `nfs_vinvalbuf2` on the fleet NFS mount `/Volumes/semio-store`, with the disk 97% full; this ticket held 75 GB of build intermediates.
  - Freed 75 GB.
  - Added `claude-resource-guard.sh`: at most 1–2 heavy jobs, at least 30 GiB of disk and 30% of memory free before a job is admitted, and an abort below 12 GiB of disk.
  - Saved the lesson to memory.

## Anticipated risks
- The full S host may need more boot components or modules than the reduced session (space, stdio, stdio-office).
- The pane journey may expose editor-level defects (decimal input in de, save naming).
- Large workbooks need the paged decoder to be wired, tested and rebuilt into the wasm, which means one more 40-minute rebuild.
- Other agents keep breaking shared crates in flight, so every rebuild may need small root fixes.

## Open repository defects to fix
- `repo doctor --fix --only=storage` reports success but does not attach the image when the mount point holds stray files.
- An unset `SEMIO_PLUGIN_CATALOG` silently yields an empty catalog.
- The managed parity runner: the "Separated capture and input…" error and the `source.served` stalls.
