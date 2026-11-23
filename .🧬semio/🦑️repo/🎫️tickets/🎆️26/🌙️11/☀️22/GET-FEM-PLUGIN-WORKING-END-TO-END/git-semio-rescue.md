# Rescue of `C:\git\semio` into `C:\semio`

Read-only git was used on both repositories. Raw lists: `🗑️generated/rescue-*`. Scripts: `rescue-*.py` in this folder.
Verdict at the end.

## 1. Commits

Old checkout: 4372 commits over all refs; `C:\semio`: 4538 (shallow, root 3d62b2e). 30 commits are not in `C:\semio`'s object database:

| Commits | What | Content status |
|---|---|---|
| 01324745f80 (`...🚩️752`, 2026-11-22 13:42, only on `refs/remotes/origin/🐙ueli/⛳wip`, i.e. fetched from GitHub) | Archivist snapshot, parent = `C:\semio` HEAD b4927d2 (751), 8559 files | 7937 identical in `C:\semio`, 47 deletions already applied, 304 ticket/script files evolved since (`C:\semio` newer, kept), 102 files where `C:\semio` still equals 751 (99 real changes, 3 `[DEBUG]` prints), 180 files absent in `C:\semio`, 3 deletions not applied. Not applied (see below). Preserved in `git-semio-rescue-commit-752-only-files.zip` (282 files, 2.4 MB; `changed-vs-751/` and `absent-in-C-semio/`) and on GitHub. |
| 27 `Agent host session ...` checkpoint commits (refs `refs/agents/{a0c238e3..., e1af9218...}/checkpoints/turn/0-2` plus the intermediate "turn N start" commits, 2026-10-02/04 and 2026-11-22) | Agent-host snapshots | The 2026-10 sessions' tips exist in `C:\semio` objects. The two 2026-11-22 sessions only contain the `PREVENT-BACKGROUND-SYNC-TERMINAL-POPUPS` ticket (6 files) and nothing for a0c238e3: copied (section 3). |
| 2 further commits in 01324745's ancestry | none (parent is 751) | n/a |

All 11 local branch tips of the old checkout that are not branches in `C:\semio` (`kinan/dev3`, `premigration/mit-bestand`, `backup`, `🦢niloufar/⛳wip`, `🐏adrian/🛟backup`, `🐙ueli/🌿1`, `📢main`, `b`, `origin-🦢niloufar/⛳wip`, `agents/resolve-merge-conflicts-teaching-quiz`, `🐳kinan/⛳wip`) are reachable from at least one ref in `C:\semio` (`rescue-refs-reachability.txt`). All 25 old stash entries are reachable via `refs/old-repo/*/stash-*` in `C:\semio`'s git (`rescue-stash-reach.txt`). Old HEAD 5ececaa (746) exists in `C:\semio`; its committed content is covered by 747-751.

Why 752's content was not applied: it is a snapshot of another machine's worktree at 13:42; `C:\semio` does not contain its 99 non-debug changes (staffage catalog JSONs, repo memory governor, Rust tests, ...), its deletions of cleanup scripts did not happen, and 180 scratch files (3D rework scripts, flow-view work) are gone. Applying other agents' unreviewed work-in-progress into the live canonical tree would risk breaking it. It is not lost: it is on GitHub `origin/🐙ueli/⛳wip` and in the zip.

## 2. Uncommitted work in `C:\git\semio` (706 entries: 111 modified, 595 untracked)

Classification by comparing old worktree, old HEAD blob and `C:\semio` worktree (`rescue-status-classes.tsv`):

| Class | Meaning | Count | Action |
|---|---|---|---|
| a | identical in `C:\semio` | 352 | none |
| b | `C:\semio` unchanged since base, change missing | 9 | old file copied into `C:\semio` (teaching energy/optimization/bim/excel/grasshopper/morph visuals and stage tests) |
| c | both diverged (modified) | 47 | 3-way merge, see below |
| c | both diverged (untracked, no base) | 19 | old (11-14, final render state) copied over; the 19 pre-merge `C:\semio` files are in `git-semio-rescue-backup/` (`index.tsv`) |
| d | only in old | 279 | copied (277); 2 tool logs skipped (`tests-final/rerun.txt`, `rerun3.txt`) |

Class c (47 modified energy teaching files): the old side is a descendant of what `C:\semio` holds (energy video finals, 11-13/14), and `C:\semio` received later independent edits in some files.
- Clean 3-way merges (`git merge-file`) applied to 7 files: `🏗️authoring/🎨️visuals/🐍️.py` (adds `reference_chip`), visuals tests (`staffage_only`), running example (`utilisationFactor`, `ETA_UTILISATION`, annual demand), `physics/❓️quiz` citations, `ventilation/📦️volume` test, `clip tests` (os.walk).
- Skipped on purpose: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts` (clean merge would delete the `isElement` helper that `C:\semio`'s heavily evolved file still uses 10 times; old side is stale). Applied once by mistake and reverted immediately to the exact HEAD blob bdbaf31.
- 36 files with real conflicts (162 hunks), revised after the coordinator's review (see table below). Evidence on age: `C:\semio` git history is shallow (root 3d62b2e, 2026-11-18; 35 files last touched there, `WindowChrome`-adjacent `browser/🟦️.tsx` also by 5db034f 2026-11-20), so it cannot show whether the `C:\semio` side changed after the old 11-14 finals; file mtimes were overwritten by the first pass. Rule applied per file: a conflict hunk may take the old side only if it is the `_din_ref` to `reference_chip` refactor, whitespace-only, or the old side contains every `C:\semio` line that is not in the 746 base; if any hunk has `C:\semio`-only lines, the whole `C:\semio` file is kept and the old change is an open item (patch in `git-semio-rescue-backup/open-old-change-NN.patch`, base 746 to old, LF-normalised). Result: 14 files old side, 22 files `C:\semio` kept. `git-semio-rescue-overridden-hunks.txt` is from the first pass and only valid for the 14 old-side files.
- Checks after the merge: all merged `.py` compile, all merged `.json` parse, no conflict markers, `render.mjs` passes `node --check`, running-example unit tests: 11 passed. The Manim clip tests/renders were not run.
- Remaining differences after the merge (11 files, `rescue-status-classes2.tsv`) are `C:\semio`-only newer changes that were intentionally kept: quiz pool `draw`/`zipf` settings, `_regroup(self, eq)` in system-losses, a kwarg in primary-energy, staffage visuals/tests superset, CRLF/whitespace, and the stale `shared-border.ts`.

## 3. Tickets and other content

| Item | Action |
|---|---|
| Ticket `11/13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS` (101 files; `🎫️ticket.json` was a 1 KB stub in `C:\semio`) | copied, `ticket.json` replaced by the closed version (backup in `git-semio-rescue-backup`) |
| Tickets `11/22/PREVENT-BACKGROUND-SYNC-TERMINAL-POPUPS` (6), `11/22/GET-TRINITY-AND-JACK-WORKING-END-TO-END/📌️important/📝️.md` | copied |
| Poster PNG/JSON for energy, optimization, bim, excel, grasshopper (170, language folders `de/`, `en/`) | copied |
| `🏗️authoring/🎨️visuals/🧪️tests/{🐍️,🔣️}reference_chip.*`, `🔥️heating/🎬️clip/🧮️calculation/🧪️tests/🐍️.py` | copied |
| Git-ignored ticket documents (names matched `target*`): AI-MCP-END-TO-END (6), GET-LOWPOLY (2), CONCRETE-BIM-CLIP-EXAMPLES (3), CONCRETE-OPTIMIZATION-CLIP-EXAMPLES (1) | copied (12, `rescue-copy-ignored-ticket-docs.log`) |
| `temp/` (scratch, 66 MB, 1632 files incl. typology assignment docx), launch kits (`staffage-launch-kit`, `teaching-morph-preview-launch-kit`, `puzzle-artifact-editors-windows-kit`), `holder/` claims, `.claude/settings.local.json` | archived outside both repos (not replicated): `C:\git-semio-rescue-archive\{temp.zip, launch-kits.zip, claude-settings.local.json}` |
| Skipped as generated/caches: all `🗑️generated`, `__pycache__`, `.cache`, `.nx`, `.venv`, `.repo/cache`, `dist/`, `.🧬semio/🎓️teaching` (27k e2e files), `.🧬semio/🦑️repo/⚡️cache`, `storybook-static`, `media/`, `x/`, `test-results`, TOP-FRONT-AND-SIDE-VIEWS `*.out`, bun lock backups (`.17c605d` etc.), `activation.log`, verify logs | not copied |
| 437 tracked files of the old HEAD that do not exist in `C:\semio` (`rescue-tracked-missing-*.txt`) | deliberate deletions in 747-751 (392) or in `C:\semio`'s worktree (45); not restored |

Path limit: longest copied path is 177 chars (limit 256). No node recursive copy was used.

### Per-file decision for the 36 conflict files

| # | File (under `🎓️teaching/🏛️architecture/⚡️energy/`) | Decision |
|---|---|---|
| 00 | `❄️cooling/❓️quiz/🔣️.json` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 01 | `🌬️ventilation/🎬️clip/README.md` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 02 | `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-02.patch` |
| 03 | `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-03.patch` |
| 04 | `🏗️authoring/🎬️clip/README.md` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-04.patch` |
| 05 | `🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-05.patch` |
| 06 | `🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟨️render.mjs` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-06.patch` |
| 07 | `🏗️authoring/📏️running-example/🧪️tests/🐍️.py` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 08 | `📊️demand/❓️quiz/🔣️.json` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 09 | `📊️demand/🎬️clip/README.md` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 10 | `🔥️heating/❓️quiz/🔣️.json` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 11 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 12 | `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-12.patch` |
| 13 | `📊️demand/🎬️clip/🧪️practice/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-13.patch` |
| 14 | `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-14.patch` |
| 15 | `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-15.patch` |
| 16 | `📊️demand/🎬️clip/🏭️primary-energy/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-16.patch` |
| 17 | `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 18 | `❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 19 | `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 20 | `🔥️heating/🎬️clip/🌬️convection/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-20.patch` |
| 21 | `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 22 | `❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 23 | `🌬️ventilation/🎬️clip/❓️need/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-23.patch` |
| 24 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-24.patch` |
| 25 | `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-25.patch` |
| 26 | `📊️demand/🎬️clip/♨️system-losses/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-26.patch` |
| 27 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 28 | `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-28.patch` |
| 29 | `📊️demand/🎬️clip/🏷️certificate/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-29.patch` |
| 30 | `🔥️heating/🎬️clip/👥️internal-gains/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-30.patch` |
| 31 | `📊️demand/🎬️clip/🔁️recap/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-31.patch` |
| 32 | `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\semio` line |
| 33 | `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-33.patch` |
| 34 | `📊️demand/🎬️clip/⚖️regulations/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-34.patch` |
| 35 | `📊️demand/🎬️clip/⚡️demand-power/🐍️.py` | `C:\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-35.patch` |

## 4. Open items for the owner

- `refs/remotes/origin/🐙ueli/⛳wip` (commit 752) exists only on GitHub and in the zip; `C:\semio`'s worktree does not contain its 99 changed files. Decide whether 752's other-machine changes should be reviewed.
- 22 conflict files keep the `C:\semio` version and their old-side change is an open item (patches in `git-semio-rescue-backup/`); the 14 old-side files are listed in the table. Re-run of the running-example tests after the revision: 11 passed; all merged `.py`/`.json` still compile/parse.
- `shared-border.ts` old change deliberately not merged.

## Verdict

`C:\git\semio` is safe to delete (the 22 open old-side energy changes are preserved as patches in `git-semio-rescue-backup/`): every commit and branch is reachable in `C:\semio` or on GitHub (752, zip), all uncommitted work is either identical, merged, copied or archived in `C:\git-semio-rescue-archive`, and the only intentionally dropped content is generated output and stale files listed above.
Do not delete `C:\git-semio-rescue-archive` or the zip until the open items are decided.
