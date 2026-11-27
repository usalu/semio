# 🧯️ Closing Round 2 Brief (2026-11-27, fifth coordination chat)

Paths: `R` = `/Users/ueli/Documents/semio`, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, `K` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE`, `G` = `U/🗑️generated/closing`.

## State

- Full multiview matrix over 375 assets (`G/matrix/summary.json`): 64,868 ok, 1 fail (`used-bentwood-dining-chairs`, 1:500 band-max at 1:1000, elevation, forces, `language-geometry`: "the de drawing has other shape geometry than the English one").
- Every other staffage suite green (`G/summary.txt`), except `🧊️model-catalog` (`S/🧪️tests/🧊️model-catalog`), run in 15 shards of 24 assets (`G/modelcat-shards/NN`, JSON `G/model-catalog-NN.json`, log `G/model-catalog-NN.log`). Issues of the finished shards, one per line: `U/modelcat-issues.txt`. Shards 04, 06, 08, 10, 12, 13, 14 are still running; read their logs as they land and take the issues of your kind.
- Issue kinds: `budget pose:<p>: GLB of N MB (budget 8 MB)` (107), `fbx clip:<c>: assimp: undefined | import_error | Aborted()` (89), `json …: ajv /nodes/N/id must NOT have more than 64 characters` (78), `budget <asset>: <mode> triangles grow from 1:A (n) to 1:B (m)` and `N <mode> triangles at 1:X (budget N)` (about 30), `closed …: box is not watertight` (5), `ModelInvalidError: self-intersection at /solids/N/profile` (`shower-head`, `gas-boiler`).

## Rules (binding, on top of `📋️takeover-brief.md` and `📋️multiview-brief.md`)

- Never weaken a test, budget, schema limit or rule to make it pass. Cure the cause in the asset or in the shared code. If a limit itself is wrong, prove it with numbers and an independent reference in your report and change it only with that proof.
- Every measuring run goes through `repo capacity run --memory=<MB> -- timeout <s> …` (single-asset model-catalog run about 2500 MB; matrix of one asset 800 to 1200 MB). At most two requests at a time per lane. Use the pinned bun: `ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1`. `repo run` hangs for staffage.
- Single-asset model-catalog run, from `R`: `SEMIO_STAFFAGE_MODEL_ASSETS=<id>[,<id>] repo capacity run --memory=2500 -- timeout 3600 $BUN node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000`.
- An asset you write must stay clean in every other proof: light proof and post-write proof as in `closing-CL-H.md` (gate, silhouette guard, three-views sweep, check CLI `--file`, matrix of the asset with 0 fail cells). Template tools: `U/🧰️matrix/CL-H/` (`light.sh`, `prove.sh`, `fix.ts`, `q.sh`).
- Kill only processes you started, by pid. Never `pkill` by pattern.
- No git command that modifies the tree. Re-read a file right before editing it; others write concurrently.
- Your tools and inputs go in `U/🧰️matrix/<LANE>/`, logs and outputs in `U/🗑️generated/matrix/<LANE>/` (delete that folder when you finish). Report `U/closing-<LANE>.md`, first line `FINAL (<date time>: <one-line result>)` when done, otherwise `IN WORK`.
- Code style per `R/CLAUDE.md`: concise, no comments inside definitions, docstrings start with a unique emoji, `[DEBUG] ` prefix on temporary logs, bilingual (en, de) user texts.
- Append one line to `U/requests.md` per catalog file you rewrite: `<LANE> rewrote <asset> at <date time>`.
