# 📋️ Multiview Fixer Brief (2026-11-24)

The last open staffage ticket is `26/11/02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE` (`K`): its acceptance is a coverage matrix without a failing cell over the whole catalog. `U/multiview-diagnosis.md` is the binding analysis (read sections 2, 3, 5 and 8): 227 assets need a change. You are a fixer for ONE batch of assets: `U/🧰️matrix/batches/<BATCH>.tsv` (columns: category, asset, sync findings, anchor findings, band copy, in-band, cue; from the scan of 2026-11-24).

## Paths

- `R` = `/Users/ueli/Documents/semio` (read `R/CLAUDE.md` first; it is binding).
- `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage` (`S/README.md` is the contract; the suite is `T` = `S/🧪️tests/🔭️multiview-coverage`).
- `K` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE` (decisions D18 to D46 in `🗂️work-board.md`).
- `U` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS`.
- Your scripts: `U/🧰️matrix/<BATCH>/`; tool output: `U/🗑️generated/matrix/<BATCH>/`; report: `U/multiview-<BATCH>.md`.
- Measuring: the authors' gate of the suite if it exists (`T/🔎️scan.ts`, see its header), else `U/🧰️matrix/catalog-scan.ts <S/🧪️tests> <out.jsonl> <ids…>`; the confirmation per asset is `T/📊️matrix.ts --out <dir> --jobs 1 --assets <id>` (slow for people and plants: run it once per asset at the end).
- Tools of the silhouette rounds you may read and copy from (do not edit them): `U/🧰️silhouette/*` (`apply.ts`, ops files, `outline.ts` of R2-O, make scripts) and the reports `U/silhouette-*.md`.

## Job

For every asset of your batch, every finding the scan and the matrix report for it (not only the kind your batch is named after):

1. Measure now. The asset files changed since the diagnosis.
2. Repair the asset `S/🗂️catalog/<folder>/<id>/🔣️.json` the way section 5 of the diagnosis says for the cause: a copied band view is redrawn so the finer band shows what its scale can print; front and plan shapes of bones that a pose turns become solids whose profile is the elevation shape (`S/README.md` requires a `solid` for every part that turns); `followTip` limbs are corrected or become solids; rest-pose extents of the three views are made equal; anchors are bound to the bone that draws their place, get their real `z` and lumped contacts are split (D45; structural fixtures that pin an anchor follow). The repair is the last step of the asset's reproducible chain: where generators, ops (`U/🧰️silhouette/<batch>/ops/<id>.json` with `apply.ts`) or make scripts exist for the asset, yours runs after them and must not undo them; otherwise a named idempotent script in your folder.
3. Prove per asset, each run to the end in this session: scan without a hard finding; matrix of the asset without a `fail` cell; `validateAsset` nothing new; check CLI `✓` (`S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>`); the real `🧊️catalog-silhouette` test of the asset still passes (385/385 on 2026-11-24, it must stay so); the asset's tests in `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `asset-validation`, `asset-power` (name filter) as before; every authored scale prints. Open the sheets of every changed view and scale; a repair that makes a drawing uglier or wrong is not kept.
4. Declared dimensions: do not edit the manifest; the coordinator syncs it from the asset files.
5. An asset you cannot bring to clean: leave it in its best proven state (never worse in any other test) and state the remaining findings with numbers. Go on; do not stop.

## Rules

- Never weaken a rule, threshold, test or tool. Shared engine, schema, validator, tests, fixtures, README, site code: do not edit (another worker owns `T` and its fixture now); a defect there with a measurement goes to `U/requests.md` as `## multiview <BATCH> — <subject>` (re-read before appending).
- Touch only the assets of your batch; other fixers work at the same time in the same live worktree (replicated to 50+ machines): re-read a file right before you write it. NO git command that changes anything, no worktrees, no stash, no IDE, no `repo dashboard`, no `repo run`: pinned bun `ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1` with `timeout`.
- Shared host (16 GB, 10 cores): exactly one tool process at a time, in the foreground; never a suite without a name filter.
- No sub-agents. Never stop to ask, never stop halfway.
- `[DEBUG] ` prefix on temporary logs. No comments inside definitions; docstrings start with a unique fitting emoji. Paths under 256 characters. Texts `{ en, de }`.
- When done delete `U/🗑️generated/matrix/<BATCH>`; keep scripts and the report.

## Report `U/multiview-<BATCH>.md`

Per asset: findings before and after by code, the proof lines, what was changed; every file created, updated or removed (repo-relative); what stays open, plainly. Final message: the report path and at most six lines.
