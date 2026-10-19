# audit-close-1019: close-out preparation for ticket C

Read-only audit by `aud-close`, snapshot 2026-10-19 21:20 CEST. Nothing was deleted or edited. Written: this file and `close-files-candidate.json` (both in the ticket root). The ticket is still being written to during the audit: the ticket walk went from 7,539 to 7,793 files, `fnsplit3-tools/` appeared today, and `🗑️generated/` grew from 4,643 to 4,868 files. Re-run every count below right before any deletion.

## Verdict

Not ready to execute the close-out. HANDOVER section 5 says to delete only after the critical path is verified. The last coordination entry (20:45) still reports the dependency sync blocked (`Same canonical Native and prospective Source provider instance required`, attempt 31) and `env-pass` ran 0 of 118 suites. The inventory and candidate list below are ready for when that gate opens.

## 1. Inventory

### 1.1 `🗑️generated/` (delete at close)
- 739 top-level entries, 4868 files, 1350 dirs, 1.75 GB (`du`). Gitignored (`.gitignore:20`), so `ticket_close` never lists it.
- It holds 1,568 `*.log`/`*.out`/`*.errs`/`*.done`/`*.skipped` files (earlier count, same class).
- 1,321 of its files are byte-identical to tracked repo files (copies). Deleting it removes them.
- 16 of its paths exceed 256 UTF-16 units from the repo root (max 277). Nothing outside it does.
- HANDOVER section 5 figures (1,306 entries, 3.2 GB, 147 long paths) are stale.

### 1.2 `orig/` snapshots (delete at close)
Five directories, 72 files. HANDOVER said 13 directories and 978 files; that is stale. `fnsplit3-tools/orig` appeared today.

| Directory | Files | Tracked in git |
|---|---|---|
| `split-fw-tools/orig` | 7 | 7 |
| `split-fw-tools/rs/orig` | 2 | 2 |
| `fnsplit2-tools/orig` | 48 | 37 |
| `fnsplit3-tools/orig` | 9 | 0 |
| `split-s-tools/orig` | 6 | 6 |

- 52 files are tracked, so deleting them shows as removals in git (include them in the close list as removals, see section 3).
- 17 files are byte-identical to tracked product sources (AGENTS violation: copied checked-in sources), 12 of them in `orig/`.

### 1.3 Other copies of checked-in files
- 2,813 ticket files are tracked and identical to themselves. Not copies.
- 1,872 files are identical to a different tracked file: 1,321 inside `🗑️generated/`, and 551 elsewhere in C.
  - 17 copies of product sources (AGENTS violation). Delete after confirming no tool reads them: 12 in `orig/` (`fnsplit2-tools`, `fnsplit3-tools`) and 5 in `jsi-tools/manual/` (`root.mjs`, `components.schema.json`, `revision-graph.d.mts`, `revision-graph.mjs`, `bootstrap.mjs`).
  - 534 copies of tool scripts that are tracked in another folder (for example `c7w-tools/frag.py` = `split-s-tools/frag.py`, `lw10b-tools/scc.ts` = `scc-tools/scc.ts`, `c6ui-tools/show.py` = an earlier ticket's `c5b-tools/show.py`). Inputs to keep unless the coordinator decides on dedupe. Decision needed, not a blocker.

### 1.4 Tool-generated outputs outside `🗑️generated/` (delete at close)
- `*.log`, `*.out`, `*.errs`, `*.done`, `*.skipped`: none outside `🗑️generated/`.
- `*.jsonl`: `tst3c-tools/baseline-results.jsonl` (1 file).
- `*.txt`, `*.tsv`, `*.list`, `*.mod`, `*.orig`: 115 files, almost all lists written by a tool (`changed`, `files`, `list`, `areas`, `touched`, `offenders`, `refs`, `scopes`). Listed in Appendix A. Not verified file by file. Keep `go.mod` (module files, 4) and `README.txt` (`bzl-act-tools`).
- JSON with output-like names: 36 files (Appendix B). Verify each: names such as `*-needed.json`, `*-final.json`, `*-report`, `*-plan`, `census*` look generated. `close-files-candidate.json` (this audit) is also a tool output: delete after `ticket_close`.
- Dated scratch files: none found by name. `status-1019.md` is a report: keep.
- `stale-publication-intents/`: see section 2.

### 1.5 Inputs to keep
- Scripts: 2,258 (`py` 1,100+, `ts` 836+, `tsx`, `sh` 141, `cjs` 63, `mjs` 48, `js`, `go`, `mts`, `rs`). Root-level scripts stay: `demo-rewrite-launch.ts`, `deps4-loop.sh`, `deps4-run.sh`, `gocomptest-*`, `osdev-targets.ts`, `relativize-bare-imports.ts`, `scan-bare-imports.ts`.
- Markdown reports: 249 (keep, including `HANDOVER.md`, `coordination.md`, `status-1019.md`, `compile-plan.md`, `brief-*.md`, `audit-*.md`, `result-*.md`).
- JSON not listed in Appendix B, plus `🎫️ticket.json`.

### 1.6 Path length
- Repo-relative paths over 256 UTF-16 units outside `🗑️generated/`: 0. The longest candidate in `close-files-candidate.json` is 207 units.

## 2. Backups and stale publication items
- `.cache/b-d-hfs-retired`: not present (already removed, or never created at that path). `.cache/b/d/` exists (modified 2026-10-18 02:15); it was not checked against HANDOVER.
- `*-image.json.orig`: 2 files, both in `stale-publication-intents/` (`d-image.json.orig`, `t-image.json.orig`, 126 bytes each, 2026-10-18 01:13). None under `.cache/`.
- `stale-publication-intents/`: 4 files, all from 2026-10-18 01:12 to 01:13 (`d-intent.json`, `t-intent.json`, and the two `.orig`). Intents say `purpose: current-prepared-publication`, `initialState: mounted`, backing images 8,588,886,016 bytes. The backing inodes (336283444, 336215214) were not found under `.cache/` at depth 5, so their state is unverified. Status: keep until the publication owner confirms (HANDOVER section 5).

## 3. Candidate files for `ticket_close`
`close-files-candidate.json` holds 718 entries: 213 inside C, 496 elsewhere in the repo, and 9 removals. An ASCII anchor (`.github/workflows/build-system.yml`) is first, because an all-emoji `files` array fails (memory `feedback-ticket-close-emoji-only-files-array-fails`).

Method (heuristic, not an oracle):
1. Read the 97 `result-*.md` files with mtime on or after 2026-10-19 00:00.
2. Take backticked path-like tokens from bullet/table lines and from headings about changes, files, tests, tools, or outputs. Expand aliases defined in the same file (`B`, `FW/M`, ...) and braces.
3. Match each token to a tracked or untracked repo file: exact path, a unique suffix match, or a unique suffix match among files with mtime today. 1,886 tokens gave 879 matching files. 148 tokens were ambiguous and 752 did not match, so they were dropped.
4. Keep files with mtime on or after 2026-10-19 00:00. Drop 1 gitignored path (`.agents/skills/build/SKILL.md`), 1 absolute path outside the repo (`/Users/ueli/.cache/split-s/lockbak/5d.lock`), and 8 ticket tool outputs (`.txt`, `.tsv`, `.jsonl`, `.orig`).
5. Add the 9 tracked removals found by `git ls-files -d` (deleted from the worktree, not staged):
- `✏️s/🔌️plugins/🔱️trinity/Cargo.toml`
- `🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/⌨️command-and-action-staging-actions/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🎛️general-settings-actions/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🧩️plugin-extension-and-chat-actions/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layout-and-window-actions/🦀️.rs`

Known gaps:
- mtime today also catches edits by other fleets to the same files. No per-executor attribution.
- `.vscode/` removal (commit `bd9e4e02bd0`, deliberate retirement) is not in the index, so it is not trackable and not listed. Its status is a user decision.
- The 97 staged deletions in the index belong to other owners and are excluded.
- The list covers only files the result files name. Files changed without being named are missing.

Before calling `ticket_close`: re-run the resolution, drop paths that no longer exist (unless they are removals), drop gitignored paths, pass the explicit ticket path, and never probe the tool with throwaway content (memories `feedback-ticket-close-explicit-path`, `feedback-ticket-close-needs-trackable-files`). `📌️important.md` is absent at the ticket root, so the close precondition holds.

## 4. Draft session summary (coordination.md, 12:40 to 20:45)
- 12:40: new session. node_modules generation `4151d2262cb4a4e7a25a`. Auto-commit at 12:21 landed about 10k files. The chain's lock failed at once (dependency reader scope) and went to deps3. A codex peer owns the course repair.
- 13:20: audits done (Bazel state, neutral build, products). `.vscode/` deleted in `bd9e4e02bd0` as the dashboard control-plane retirement (user decision pending).
- 13:40 to 15:05: prod-fix (SEMIO_RENDERER removed, scripts moved to neutral), bzl-zt, rootimp, glue, cc2, tst3a/b/c, bzl-act (native actions with declared inputs, 70 tests pass), nx-bound, aud-xplat (Linux directory partitions, Windows VHD refused, devcontainer cache volume, PR CI matrix).
- 15:40 to 17:05: rules-fix (90 `[DEBUG]` removed), xplat-ci (`build-system.yml`), cache-s/cache-fw (deterministic cache-off cut sharply), bzl-label (emoji labels OK in Bazel), deps3 lock green at 17:05, chain2 sync blocked by EPERM.
- 17:30 to 18:40: deps4 (cache quarantine and generation GC), bzl-sandbox (Bazel 8.4.2 double-encodes non-ASCII sandbox paths; ASCII output base is the workaround), store-out receipts, docs-build, cmdreg, xplat-win2, bzl-breadth (8,010 task rules generated; `bazel build --nobuild //...` 0 errors over 8,869 targets), rg-* crate splits, cargo roster fixed, cli-fix, tax, aud-gates (prod 0 over gate).
- 19:00 to 20:00: go-all (32/32 Go modules), fsevents, rg-test, docs-fix, tst4/tst5, lay15 (cycles 27 to 1), fem-fix, bzl-suite (source parity 72/3), tax2, aud-nct.
- 19:45: HARD FREEZE on `package.json`, `bun.lock`, build modules, root `📜️script.ts`, taxonomy JSON (until deps4 sync is green).
- 20:45: sync blocker (attempt 31) is a real defect (same as attempt 14). Performance blocker: each `bun repo run` re-walks the store for 25 to 50 min, so `env-pass` ran nothing.
- Open user decisions: `.vscode/` retirement, neutral `bun repo run` in `package.json` (AGENTS conflict), `jcoprobe` tracked generated output.

## 5. Blockers and risks before close
1. The critical path is not verified (sync blocked at 20:45). Per HANDOVER section 5, no deletion yet.
2. The ticket is still written to (see the snapshot note above). Re-run inventory and candidate list at execution time.
3. HANDOVER says the strict freeze began at 18:30; the brief and coordination say 19:45. Clarify the start and lift it explicitly, and tell all executors and the deps owner explicitly (HANDOVER section 5, freeze bullet).
4. Deleting 52 tracked `orig/` files creates removals in git. Confirm that no running tool still reads them.
5. The 534 duplicate tool scripts need a coordinator decision (section 1.3).
6. HANDOVER section 5 asks to remove scratch `node_modules`. None found in C. Two empty `node_modules` dirs exist in the session scratchpad (outside the repo, 0 KB).

## Appendix A: tool-output-like files outside `🗑️generated/` (115)
- `aud-rules-tools/evidence/`: `item1-debug-hits-src.txt`, `item1-debug-logcalls-src.txt`, `item2-exec-bit-files-nonticket.txt`, `item2-hooks-and-bootstrap.txt`, `item2-shell-ps-files-nonticket.txt`, `item3-project-json-deviations.txt`, `item4-compat-keyword-hits.txt`, `item4-reexport-only-all.txt`, `item4-reexport-with-compat-wording.txt`, `item5-body-comments-touched.txt`, `item6-docstrings-without-emoji-touched.txt`, `item7-package-dependencies.tsv`, `item8-hash-named-tracked.txt`, `item8-ticket-copies-of-tracked-code.txt`
- `bzl-act-tools/`: `README.txt`
- `c6db1-tools/`: `files.tsv`
- `c6db2-tools/`: `files.tsv`
- `c6flow-tools/`: `files.tsv`
- `c6inf-tools/`: `files.tsv`
- `c6plug2-tools/`: `files.tsv`
- `c6store-tools/`: `files.tsv`
- `c6ui-tools/`: `files.tsv`
- `cc-tools/`: `none-330.txt`, `refused.txt`, `violators.txt`
- `env-pass-tools/`: `refs.txt`
- `fem-fix-tools/`: `suites.txt`
- `gocanon/`: `modules.txt`
- `gocli-tools/`: `go.mod`, `plan-cli-main.tsv`, `plan-exh.tsv`, `plan-mcp.tsv`, `plan-move.tsv`, `plan-test.tsv`
- `gocomptest/`: `go.mod`
- `gofix2-tools/reach/`: `go.mod`
- `gofix2-tools/rmdecl/`: `go.mod`
- `law1-tools/`: `list-fw-rows.txt`, `notes.tsv`
- `law2-tools/`: `decisions.tsv`, `list-mine.txt`
- `lay1-tools/`: `renames1.txt`, `renames2.txt`
- `lay13-tools/`: `ncpr-changed.txt`
- `lay14-tools/`: `rfix-changed.txt`
- `lay15-tools/`: `areas-fwm.txt`, `areas-late.txt`, `areas-nofwm.txt`, `areas.txt`, `files-a.txt`, `files-all.txt`, `files-b.txt`, `files-nb.txt`, `glue-edited.txt`, `nb-tests.txt`, `renames.txt`
- `lw1-tools/`: `touched-framework.txt`, `touched-plugins.txt`
- `lw10a-tools/`: `changed-files.txt`, `changed-tests.txt`, `cli-closure.txt`
- `lw10b-tools/`: `cli-closure-union.txt`, `ncpr-changed.txt`
- `lw1b-tools/`: `files.txt`
- `lw4-tools/`: `changed-files.txt`
- `lw5-tools/`: `touched.txt`
- `lw8-tools/`: `ncpr-changed.txt`
- `qe2e-tools/`: `scratch-symlinks.txt`
- `repot-tools/`: `list.txt`
- `repot2-tools/`: `list-red.txt`, `list.txt`
- `rules-fix-tools/`: `debug-stripped-files.txt`
- `tax-tools/`: `router-flips-applied.txt`, `strays-deleted.txt`
- `tax2-tools/`: `changed-files.txt`, `introduced-directory-kind-findings.txt`, `remaining-leaf-findings.tsv`, `remaining-router-law.tsv`
- `tax2b-tools/`: `changed-by-alias.txt`, `removed-renames.tsv`, `value-pairs.tsv`
- `tax2b-tools/cruise/`: `areas.txt`
- `tax3-tools/`: `scopes.txt`
- `tst3a-tools/`: `list.txt`
- `tst3c-tools/`: `baseline-results.jsonl`, `l-a.txt`, `l-art.txt`, `l-art2.txt`, `l-c1.txt`, `l-c2.txt`, `l-d1.txt`, `l-e1.txt`, `l-e2.txt`, `l-job.txt`, `list-all.txt`, `list-all2.txt`, `list-b.txt`, `list-new.txt`, `list-newred.txt`, `list-red.txt`, `list-red2.txt`, `list-red3.txt`, `list-red4.txt`, `offenders-fw.txt`, `real-c.txt`
- `tst4-tools/`: `envelope-gated.txt`, `final-list.txt`, `list.txt`
- `uicarve-tools/lists/`: `list-ALL-test.tsv`, `list-ALL.tsv`, `list-default-test.tsv`, `wgpu-old-root.rs.txt`
- `vcfg-tools/`: `os-dev.list`
- `vit2-tools/`: `list.txt`
- `vtax-tools/`: `context-imports.txt`

## Appendix B: JSON with output-like names (36)
`aud-census8-tools/census8.json`, `c6store-tools/census-crates.json`, `close-files-candidate.json`, `lay12b-tools/exports-needed.json`, `lay13-tools/exports-needed.json`, `lay13-tools/ncpr-plan-applied.json`, `lay14-tools/exports-needed.json`, `lay14-tools/plan-yield.json`, `lay15-tools/glue-report-1.json`, `lay15-tools/glue-report-2.json`, `lay6-tools/exports-needed.json`, `lay7-tools/exports-needed.json`, `lay7-tools/tsconfig.touched.json`, `lay9-tools/exports-needed.json`, `lw1-tools/exports-needed.json`, `lw10a-tools/exports-needed.json`, `lw10b-tools/exports-needed.json`, `lw1b-tools/exports-needed.json`, `lw2-tools/exports-needed.json`, `lw2b-tools/exports-needed.json`, `lw3-tools/exports-needed.json`, `lw4-tools/exports-needed.json`, `lw5-tools/dependencies-needed.json`, `lw5-tools/exports-needed.json`, `lw5-tools/remaining-edges.json`, `lw5b-tools/dependencies-needed.json`, `lw5b-tools/exports-needed.json`, `lw8-tools/exports-needed.json`, `repoapi-tools/exports-final.json`, `repoapi-tools/manifests-needed.json`, `rg-rest-tools/specs/tiff-snapshot.json`, `rgplug-tools/specs/pdf-snapshot.json`, `tst3c-tools/final-results.json`, `tst4-tools/final-classes.json`, `uicarve-tools/families/probe.json`, `uicarve-tools/lists/census-final.json`

## Appendix C: copies of product sources (17)
- `fnsplit2-tools/orig/apply-taxonomy-plan.ts` <= `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🚀️apply/🟦️.ts`
- `fnsplit2-tools/orig/browser-bundle-tests.ts` <= `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts`
- `fnsplit2-tools/orig/gis-document-open.ts` <= `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🗺️document-open/🟦️.ts`
- `fnsplit3-tools/orig/backbone-worker.ts` <= `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧵️backbone-worker/🟦️.ts`
- `fnsplit3-tools/orig/browser-bundle-tests.ts` <= `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/fnsplit2-tools/orig/browser-bundle-tests.ts`
- `fnsplit3-tools/orig/gis-document-open.ts` <= `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/fnsplit2-tools/orig/gis-document-open.ts`
- `fnsplit3-tools/orig/gis-worker.ts` <= `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/👷️worker/🟦️.ts`
- `fnsplit3-tools/orig/host-effects.ts` <= `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/⚡️host-effects/🟦️.ts`
- `fnsplit3-tools/orig/plugin-exchange.rs` <= `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs`
- `fnsplit3-tools/orig/plugin-module-loader.ts` <= `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🔌️plugin-module-loader/🟦️.ts`
- `fnsplit3-tools/orig/shell-actions.ts` <= `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🖱️shell-actions/🟦️.ts`
- `fnsplit3-tools/orig/two-client.ts` <= `🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts`
- `jsi-tools/manual/bootstrap.mjs` <= `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔁️bootstrap/🟨️.mjs`
- `jsi-tools/manual/components.schema.json` <= `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🗂️components/🧬️schema/🔣️.json`
- `jsi-tools/manual/revision-graph.d.mts` <= `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🪢️revision-graph/🟨️.d.mts`
- `jsi-tools/manual/revision-graph.mjs` <= `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🪢️revision-graph/🟨️.mjs`
- `jsi-tools/manual/root.mjs` <= `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs`

## Appendix D: removals in candidate list (9)
- `✏️s/🔌️plugins/🔱️trinity/Cargo.toml`
- `🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/⌨️command-and-action-staging-actions/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🎛️general-settings-actions/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🧩️plugin-extension-and-chat-actions/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layout-and-window-actions/🦀️.rs`
