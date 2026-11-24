# 📋️ Silhouette Fixer Brief (2026-11-23)

The full `🧊️catalog-silhouette` suite of 2026-11-23 16:27 fails for 202 of 385 tests (one per asset). It is the acceptance of ticket `26/11/14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE`. You are a fixer for ONE batch of assets: `U/🧰️silhouette/<BATCH>.tsv` (category, asset id, first failing expectation; the test stops at the first one, more may follow behind it).

## Paths

- `R` = `/Users/ueli/Documents/semio` (read `R/CLAUDE.md` first; it is binding).
- `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage` (`S/README.md` is the contract; the test is `S/🧪️tests/🧊️catalog-silhouette/🟦️.ts`).
- `U` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS`.
- Your scripts: `U/🧰️silhouette/<BATCH>/`; your tool output: `U/🗑️generated/silhouette/<BATCH>/`; your report: `U/silhouette-<BATCH>.md`.
- Existing tools you may read and reuse (copy what you need into your folder, do not edit them): `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/REP-B` (`auto.ts` greedy silhouette repair, `vt.sh` one real test with a name filter, `gate.sh`, `apply.ts`, `dims.ts`, `bridge.ts`, `nudge.ts`), `U/🧰️three-views`, `U/🧰️fix-two`, and the method in `U/humans-b-repair.md`, `U/fix-jogger-wheelchair.md`.

## Job

For every asset of your batch, in the order of the file:

1. Measure every failing check of the asset now (bounding box per axis, IoU and largest boundary distance per scale and view), not only the first.
2. Repair the asset `S/🗂️catalog/<folder>/<id>/🔣️.json` so the real test passes. The volume is the reference where drawing and volume disagree, unless the volume itself is wrong (then repair the volume first). Where the asset has a generator or repair ops in a ticket (search the tickets folder for the id), the repair is added there as its last step so the file stays reproducible; otherwise write a named, idempotent repair script in your folder.
3. Prove per asset, each run to the end in this session: real `🧊️catalog-silhouette` test for the asset passes; `validateAsset` reports nothing new; product check CLI `✓` (`S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>`); the asset's tests in `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `asset-validation` and `asset-power` (name filter) pass as before or better. Open at least the 1:50 and 1:500 sheets of every repaired asset and look at them: a repair that makes the drawing uglier or wrong is not kept.
4. Declared dimensions or power that change: do NOT edit the manifest `S/🗂️catalog/🔣️.json`. Write `U/🧰️silhouette/<BATCH>/manifest-changes.json` as `{ "<id>": { "widthM": …, "heightM": …, "depthM": …, "powerW": … } }` (only changed fields); the coordinator applies it.
5. An asset you cannot bring to green in reasonable effort: leave its file in its best proven state (never worse than before in any other test), and state the remaining failing checks with numbers in the report. Go on with the next asset; do not stop.

## Rules

- Never weaken a rule, threshold, test or tool. Shared engine, schema, validator, tests, fixtures, README, site code: do not edit; a defect there with a measurement goes to `U/requests.md` as a section `## silhouette <BATCH> — <subject>` (re-read before appending), then work around it and continue.
- Touch only the assets of your batch. Eight other fixers work on other assets at the same time in the same live worktree (replicated to 50+ machines): re-read a file right before you write it. NO git command that changes anything, no worktrees, no stash, no IDE, no `repo dashboard`, no `repo run` (it hangs for staffage): use the pinned bun `ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1` with `timeout`.
- The host is shared (16 GB, 10 cores): exactly one tool process at a time, in the foreground; never run a suite without a name filter.
- No sub-agents. Never stop to ask, never stop halfway.
- Temporary logs carry the `[DEBUG] ` prefix. No comments inside definitions; docstrings start with a unique fitting emoji. Paths stay under 256 characters. Texts are `{ en, de }`.
- When done delete `U/🗑️generated/silhouette/<BATCH>`; keep scripts and the report.

## Report `U/silhouette-<BATCH>.md`

Per asset: failing checks before, after, the proof lines, what was changed; then every file created, updated or removed (repo-relative); then what stays red, plainly. Your final message is only the report path and at most six lines.
