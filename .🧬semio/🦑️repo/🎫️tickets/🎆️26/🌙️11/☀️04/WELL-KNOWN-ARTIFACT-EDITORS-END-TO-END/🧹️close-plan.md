# 🧹️ Ticket Close Plan: Well-Known Artifact Editors End to End

Owner: T20 (haiku-documentation). Status: **plan only. Nothing has been deleted.** Snapshot taken 2026-11-04 after the last batch logs (about 07:15). Files written by other workers are still changing, so the close step must re-run the inventory first.

Rules applied (AGENTS.md, ticket section):
- Keep input files: scripts, configs, generators, oracles, JSON inputs. Do not delete them at close.
- Keep audits and reports (markdown).
- Delete tool-generated output: logs, txt, tsv, piped output and everything under `🗑️generated/`. Delete the `🗑️generated/` folder itself when done.
- Delete temporary files that must not survive the ticket.
- Files that are only temporary until a tool is fixed (bun on PATH) are deleted when that fix lands, not before.

## 1. Top-level inventory (51 entries)

### 1a. Input to keep (a): scripts, generators, oracles, JSON

| File | Purpose | Close action |
| --- | --- | --- |
| `apply-details-split.py` | applied Details split layout to 29 files (T19c) | keep |
| `audit-receipts.py` | audit receipt checker | keep |
| `build-acceptance-fixture.py` | acceptance fixture builder | keep |
| `build-fixture-manifest.py` | writes `🧫️fixture-manifest.json` | keep |
| `build-invalid-fixtures.py` | builds 54 invalid fixtures (T19) | keep |
| `crate-names-index.py` | writes `🗑️generated/crate-names-index.tsv` | keep |
| `crate-names-render.py` | renders `📦️crate-names.md` from the TSV | keep (needs the TSV; see section 2) |
| `🧪️formats-fixture-generator.py` | format fixtures | keep |
| `🧪️saved-at-fixture-generator.py` | saved-at fixtures | keep |
| `🧭️generate-format-resolution-fixture.py` | resolver fixture (shared Rust and TS) | keep |
| `🏗️batch-f-epw-csv-oracle.py` | independent EPW CSV oracle | keep |
| `🏗️batch-f-make-bcf-fixtures.py` | BCF fixtures (minimal, truncated, missing-version) | keep |
| `🅰️batch-a-chain.sh` | batch A test chain (waits on a pgrep loop) | keep until its runs finish, then keep as input |
| `🎵️batch-g-run.sh` | batch G test runner (writes logs into `🗑️generated`) | keep |
| `🧪️batch-h-chain.sh` | batch H test chain (waits on a pgrep loop) | keep |
| `🧫️fixture-manifest.json` | generated fixture manifest; **superseded** by the checked-in `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧫️fixtures/📇️samples.json` (coordinator decision) | **delete at close** (moved to section 1c). `build-fixture-manifest.py` stays as an input |
| `🐍️oracles/` (dir) | `drawing-node-edit-oracle.py`, `hex-splice-oracle.py` (third-party oracles) | keep |
| `🧊️batch-c-scripts/` (dir) | `generate_bmp_demo.py` | keep |
| `🧊️batch-e-scripts/` (dir) | `run-batch-e-tests.sh`, `transform-oracle.mjs`, `make-tetrahedron-demo-assets.py`, `check-tetrahedron-demo-assets.py` | keep |

### 1b. Audit or report to keep (b): markdown

| File | Kind | Close action |
| --- | --- | --- |
| `🌄️batch-c.md`, `📐️batch-d.md`, `🧊️batch-e.md`, `🏗️batch-f.md`, `🎵️batch-g.md`, `🎒️batch-h.md`, `🅱️batch-b.md` | batch reports | keep |
| `🗂️board.md` | coordinator board and concurrency log | keep |
| `🎟️capability-audit.md`, `🔍️data-audit.md`, `🚪️entry-audit.md`, `🧾️registration-audit.md`, `🎮️steamdeck-usability-audit.md` | audits | keep |
| `🔄️formats.md`, `🖥️host.md`, `💾️persistence.md`, `📇️registry.md`, `🛠️runtime.md`, `✅️validation.md` | contract and status reports | keep |
| `📙️recipe-edit-rules.md`, `📗️recipe-natural-file-and-native-receipt.md` | recipes | keep |
| `📘️commands.md`, `📊️coverage-matrix.md`, `📊️coverage-report.md`, `📦️crate-names.md` | deliverables | keep |
| `🧹️close-plan.md` | this file | keep |

### 1c. Tool-generated output or temporary, outside `🗑️generated` (c)

| File | Why it is output or temporary | Close action |
| --- | --- | --- |
| `vitest.host.config.mjs` | **temporary.** Ticket-only vitest config, written because `bun` is not on PATH. Reads `HOST_TEST_ROOT` and `HOST_TEST_ENV`. | **delete** once `repo run @semio-tech/framework-os:test` (or the project test target) runs the same suites. Until then keep, because host tests depend on it |
| `vitest.paint-labels.config.mjs` | **temporary.** Single-file jsdom config for the Paint2dHost focus test | delete after the bun-free runner exists, same condition as above |
| `vitest.batch-c.config.mjs`, `vitest.media.config.mjs`, `vitest.media-react.config.mjs`, `vitest.registry.config.mjs` | **temporary.** One-off vitest configs per batch. `vitest.registry.config.mjs` is the config behind the registry vitest result (4 tests, `📇️registry.md` Addendum 5) | delete with the same condition; confirm each is not referenced by a checked-in project target first; re-run and transcribe before deletion |
| `🧫️fixture-manifest.json` | superseded fixture manifest (coordinator decision) | delete at close, after the transcription step |

Note on `vitest.*.config.mjs`: they are configuration, but they were created for this ticket only. They are not checked in and not referenced by any project target. Treat them as temporary.

## 2. `🗑️generated/` inventory (30 entries)

All entries are tool-generated output or temporary, so they are classified (c). The folder itself is deleted at close.

| File | Kind | Close note |
| --- | --- | --- |
| `activate-s.log` (3.4 MB) | dev-server activation log | transcribe state, then delete |
| `a-txt-editor-test.log` | txt tests log | delete |
| `b08-csv.log`, `b08-idml.log` | 0-byte logs | delete |
| `crate-names-index.tsv` (210 KB) | index written by `crate-names-index.py` | delete **after** `crate-names-render.py` has run for the last time; regenerate with the index script if needed |
| `d-dxf-editor-baseline.log`, `d-dxf-editor-shared.log` (4.2 MB) | dxf cargo logs | delete |
| `[DEBUG]composition-prepare.log` | **temporary**, `[DEBUG]` prefix; capacity-wait only | delete |
| `[DEBUG]contract.txt` | **temporary**, `[DEBUG]` prefix; the contract result (192 of 194). **Transcribed** in `✅️validation.md` E1 | delete after the transcription step |
| `host-vitest.log` | host vitest re-run, rc=1 (96 tests pass; one suite fails to transform, owner T02). **Transcribed** in `✅️validation.md` E3 | delete after the transcription step |
| `registry-vitest.log`, `media-vitest.log`, `media-react-vitest.log` | light-suite re-runs, rc=0 (4, 20 and 10 tests). **Transcribed** in `✅️validation.md` E4 and E7 | delete after the transcription step |
| `contract-rerun.log` | acceptance contract re-run, rc=1 (192 pass, 2 fail, idml). **Transcribed** in `✅️validation.md` E1 | delete after the transcription step |
| `t01-os-io-format-resolution.log` | registry law result (5 of 5), `EXIT 0`. **Transcribed** in `✅️validation.md` E2 and `📇️registry.md` | delete after the transcription step |
| `energy-metadata.err`, `energy-metadata.json` (0 bytes) | energy metadata output | delete |
| `e-obj-editor-test.log`, `e-semio-s-artifact-stdio-obj-editor::obj.log` | obj logs | delete |
| `e-semio-s-artifact-stdio-contract-editing-transform.log`, `e-transform-test.log` | transform logs | delete |
| `f-batch.log` | batch F chain log (capacity-wait only) | delete (after its run finishes) |
| `batch-g-mp3.log`, `batch-g-run.out` (0 bytes), `batch-g-summary.log` (0 bytes) | batch G runner output (capacity-wait only) | delete |
| `b08-idml-run.txt` | idml run output (capacity-wait only) | delete |
| `t04-semio-test.txt` | semio test output (capacity-wait only) | delete |
| `t01-ui-validation.log` | UI validation (capacity-wait only; the Rust limits suite has not run) | delete |
| `e-semio-s-artifact-stdio-contract-editing-transform.log` | transform run (capacity-wait only) | delete |
| `energy-metadata.json`, `energy-metadata.err` | energy metadata output (0 bytes and stderr) | delete |
| `h-zip-editor-test.log`, `h-zip-editor-test2.log` | zip logs | delete |
| `repo-mcp-install.log`, `repo-mcp-install-2.log` | MCP install logs | delete |
| `show-projects.txt`, `show-stdio-plugin.txt` | `repo show` output | delete |
| `t01-os-io-format-resolution.log`, `t01-ui-validation.log` | test logs | delete |
| `t03-rust-sync-native.log`, `t03-rust-sync-native2.log`, `t03-rust-sync-native3.log` | persistence logs | delete |
| `t04_files.txt` | file listing | delete |
| `t13-wav-codec.log` | wav log | delete |
| `t20_stdio_cargo.txt` | my temporary listing of stdio Cargo.toml paths | delete |

## 3. Misplaced generated output (outside `🗑️generated/`)

1. **`.🧬️semio/` at the repo root** (note the variation selector; it is not the ticket path `.🧬semio/`). Created by a batch C command with a typo. It contains only `🗑️generated/c-bmp-editor.txt` (45 KB), a test output. **Delete the whole `.🧬️semio` directory at the repo root** after confirming the path with `ls -la` (it must not be the real `.🧬semio`). It is outside the ticket and not checked in.
2. **`🧫️fixture-manifest.json` at the top of the ticket.** Generator output. Decision needed (section 1a). Not a misplacement by default.
3. **`📦️crate-names.md` (209 KB).** A deliverable report, not a misplacement. It is large because it lists all 938 packages. Keep as markdown.

## 4. Temporary files that must not survive

| Item | Location | Condition to delete |
| --- | --- | --- |
| `vitest.host.config.mjs` | ticket top level | a bun-free project runner covers the host suites |
| `vitest.paint-labels.config.mjs`, `vitest.batch-c.config.mjs`, `vitest.media.config.mjs`, `vitest.media-react.config.mjs`, `vitest.registry.config.mjs` | ticket top level | same condition; confirm each is not used |
| `[DEBUG]contract.txt`, `[DEBUG]composition-prepare.log` | `🗑️generated/` | at close |
| `.🧬️semio/` stray tree | repo root | at close, after `ls` check |
| `/tmp/t20_*` files (`t20_changed.txt`, `t20_changed_s.txt`, `t20_untracked.txt`, `t20_untracked_s.txt`, `t20_untracked_all.txt`, `t20_stdio_cargo.txt`, `t20_debug_files.txt`, `t20_debug_hits.txt`, `t20_debug_changed.txt`, `t20_all_candidates.txt`) | outside the repo, created by T20 for this plan | delete now or at close. Not repo files, but they are temporary |
| `/tmp/claude-1000/...` task outputs | outside the repo, harness-owned | not ours; leave to the harness |

## 5. Repo files outside the ticket that agents created

**5a. `[DEBUG]` tags in source.** Scan method: all changed or untracked files in the working tree (`git -c core.quotePath=false --no-optional-locks diff --name-only HEAD` and `ls-files --others --exclude-standard`), about 1,967 candidate files with source, JSON, script or markdown extensions, searched for the literal `[DEBUG]`.
- Result: **two hits, both ticket documents** (`📘️commands.md` and `✅️validation.md`, where the tag is quoted as documentation). **No source file** in the changed or untracked set carries a `[DEBUG]` tag.
- The broader grep found 26 tracked source files with `[DEBUG]` in their assertion or log text (for example `🏪️store/📒️ledger/…/🧪️tests/🦀️.rs`, `✏️s/🔌️plugins/🗄️stdio/…/🧪️tests/🟦️.ts`). None of them is changed in the working tree, so they are pre-existing repository content, not agent output. **Leave them.**
- Conclusion for the close: no `[DEBUG]` source edits to remove. Re-run the scan at close; if a changed file appears, remove the tagged line.

**5b. Untracked repo files created by the fleet (not temporary).** 220 untracked paths outside `🎫️tickets` in total. They are test fixtures (truncated, wrong-magic, natural-open, invalid), new source files (for example `🧭️format-resolution`, `💾️saved-state`, `♿️host-accessibility`, `🖌️Paint2dHost/✍️editing/⌨️keyboard`, `📐️transform`, `🖼️preview`, `🎚️mesh-scene`), and new surface crates (`idml` story and surface). **Keep all of them.** They are product content, not temporary. Their authorship is the fleet's, so they are listed here for the record.

**5c. Other untracked items.** `artifact-editors-steamdeck-launch-kit/` (README, agents.json, fleet.md, prompt.txt, run.sh). **Not created by the fleet**; it is the coordinator's launch kit. Leave it out of the close.

**5d. Changed-file scope.** About 1,919 tracked files differ from HEAD across the whole workspace, from many workers and from before this ticket. Authorship per file cannot be proven from git alone. The close must not delete or revert any tracked change; it only removes the files listed in sections 1c, 2 and 3.

## 6. Ordered close steps (to run only when the coordinator confirms)

Order matters: transcribe before delete, so the evidence the coverage report cites survives the `🗑️generated/` deletion.

1. Wait for the pgrep-gated chains (`🅰️batch-a-chain.sh`, `🧪️batch-h-chain.sh`, `🎵️batch-g-run.sh`, and the `f-batch.log` run) to finish. Check with `ps -eo pid,args | grep repo`.
2. Re-run the inventory: `find . -maxdepth 1 -mindepth 1 -printf '%y\t%s\t%f\n' | sort -k3` and `find 🗑️generated -mindepth 1 -printf '%y\t%s\t%f\n'`. Compare with sections 1 and 2.
3. Run `crate-names-render.py` once more if the index changed, then delete `🗑️generated/crate-names-index.tsv`.
4. Confirm the bun-free runner exists and passes. Only then delete the `vitest.*.config.mjs` files.
5. Delete the stray `.🧬️semio/` directory at the repo root after `ls -la` (it holds `c-bmp-editor.txt`; transcribe its content first if it is cited, which the current reports do not).
6. **Transcribe evidence (before any deletion in `🗑️generated/`).** For every result that `📊️coverage-report.md` or `📊️coverage-matrix.md` cites, make sure the markdown holds: the command, the exit code (or "not recorded"), the pass and fail counts, and the key failure lines. Current transcriptions: `✅️validation.md` E1 (contract, 192 of 194, rc=1 re-run), E2 (os-io registry law, 5 of 5), E3 (host vitest, rc=1, 96 pass, one suite fails to transform), E4 (registry vitest, rc=0, 4 pass), E5 (failed starts), E6 (runs with no result), E7 (media vitest, rc=0, 20 and 10 pass); and the per-batch files (`📐️batch-d.md`, `🧊️batch-e.md`, `🎒️batch-h.md`, `🏗️batch-f.md`, `🎵️batch-g.md`, `🌄️batch-c.md`, `🅱️batch-b.md`, `🖥️host.md`, `📇️registry.md`). Check each citation resolves to a transcription before continuing. If a log has changed since the transcription, transcribe the new result first.
7. Delete the `🗑️generated/` folder (all of section 2, including the `[DEBUG]` files).
8. Delete the `/tmp/t20_*` files.
9. Re-run the `[DEBUG]` scan (section 5a) on the changed set.
10. Delete `🧫️fixture-manifest.json` (superseded; coordinator decision 1). Keep `build-fixture-manifest.py`.
11. Keep every file in sections 1a and 1b.

Dry run before step 6: `find "🗑️generated" -mindepth 1 -print` (no deletion).

## 7. Unknowns and risks

- **Running jobs.** Deleting logs while a pgrep-gated chain or a capacity-queued cargo job still writes to them will produce broken logs or a failed close. Wait for the jobs first.
- **Authorship.** Git cannot prove which fleet worker touched a tracked file (section 5d). The close plan only deletes temporary files and generated output, so this does not block it.
- **Index dependency.** `crate-names-render.py` reads the TSV in `🗑️generated`. Delete the TSV after the last render, not before.
- **Shared `🗑️generated` owners.** Other workers may still write there until the coordinator closes the ticket. Confirm that no worker is active before step 6.
- **Stale content.** This plan was written from a snapshot. Section 2 lists 30 entries; new logs may appear.
- **Not verified.** The `show` and `cargo` logs were not re-read for results at close time. That is not needed for deletion, but the coverage report should cite only logs that contain results (its own rule); delete the logs after the report cites them, or keep the cited ones as evidence only if the coordinator asks.
