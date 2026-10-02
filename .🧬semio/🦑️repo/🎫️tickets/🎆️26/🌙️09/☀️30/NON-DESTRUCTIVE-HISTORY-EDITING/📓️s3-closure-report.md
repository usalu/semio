# 📓️ S3-CLOSURE Report — deleting every amend/coalesce/bracket path, derived fold footprints

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Executor S3-CLOSURE (Opus), session 3. Contract: `📋️design.md` §13, §15, §17.5,
§17.7, §20 (overrides census D1: no amend on ANY lane). Input: `📓️s3-closure-census.md` §(b)–(d). Scratch: `🗑️generated/s3-closure/`.

## Session 3 — 2026-10-02

Status: IN PROGRESS (started 12:05). Sections below are updated at every milestone.

### Step 1 — RED gate (DONE 12:55)

- Root `📜️script.ts` region `//#region 🚫️HistoryClosure`: `HISTORY_CLOSURE_RULES` (8 rules, literal `anchors` prefilter + regex),
  pure `historyClosureFindings(sources, rules)` (rule `functions` = enclosing-fn allowlist for declared whole-document intents),
  `historyClosureSources` (one `git ls-files -co --exclude-standard` over `✏️s 🧰️framework 🌎️hub`, `.rs .ts .tsx .js .py .feature .json`),
  `policyHistoryClosureBreaches`. Command `bun ./📜️script.ts verify history-closure [--self-test|--json]` (VerifyScript route
  `runHistoryClosure`); launch entry `⚖️gate🚫️history-closure` (order 411.425) in `.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json`.
  NOT in `runGate` yet (red until step 8).
- Rules: `amend-emit` (`Emit::amend`, `amend_config`), `amend-last`, `coalesce-key` (`coalesce_key|coalesceKey|set_coalesce_key`, all
  files incl. fixtures/wire), `preview-contract`, `bracket-verb` (allow: `🖐️gumball-verb-audience.json`, puzzle 3d `🔬️unit` bracket-absence
  law), `host-snapshot-bracket` (`setHostSnapshot`, coordinator 12:40), `edit-literal` (plugin production `protocol::Edit {`),
  `footprint-hand` (`for_one_invertible_item|for_one_item|ArtifactStoreOneItemFootprint { work_items` outside `🏪️store`, non-test).
  `snapshot_edit_set_snapshot` is NOT banned (§20.3: survives for whole-document intents; D3 owners).
- Planted-violation test `🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️history-closure-policy/🟦️.ts` (27 cases + clean;
  banned tokens assembled from fragments so the file itself is clean). Taxonomy scope report clean.
- Superseded and deleted: tool-run predicate `interactivityToolRunAmendFailures` (+ dead `interactivityToolRunBlock`, the dead
  `actions` requirement column ×26 rows, trigger tokens `Emit::amend|coalesce_key`, 8 amend cases in
  `⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy/🟦️.ts`), draw gate `coalesce_key: Some(` check + its hostile case
  (`🔌️plugin/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner/🟦️.ts`), puzzle fill fixture `coalesce` fn (2 cases re-anchored).
- Verification: self-tests run via `🗑️generated/s3-closure/selftests.ts` → tool-run 29 PASS, drawing 20 PASS, closure 28 PASS,
  puzzle-fill 32 PASS; `verify history-closure --self-test` PASS (28). Full scan (68 s) census 12:50:
  `amend-emit=9 amend-last=67 coalesce-key=351 preview-contract=3 bracket-verb=12 host-snapshot-bracket=40 edit-literal=45 footprint-hand=90`
  (`🗑️generated/s3-closure/gate-1.json`).

### Step 2 — document lane deletion
_pending_

### Step 3 — `next_edit` helper
_pending_

### Step 4 — derived fold footprint
_pending_

### Step 5 — config lane (no amend)
_pending_

### Step 6 — store/wire deletion
_pending_

### Step 7 — plugin sweeps
_pending_

### Step 8 — laws, acceptance greps, coordinator actions
_pending_
