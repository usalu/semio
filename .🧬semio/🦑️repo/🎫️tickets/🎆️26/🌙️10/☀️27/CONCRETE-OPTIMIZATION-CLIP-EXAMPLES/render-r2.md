# Render R2 — BIM and Optimization Clips

Final 1080p30 render of all twelve rebuilt clips, in both languages and for both audiences (speaker and spectator).

## Command

```bash
export SEMIO_REPO_BINARY="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
bun "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --video <id> … --language <en|de> --quality h --fps 30 --no-play
python deliver.py <en|de> <speaker|spectator>
```

- **Why the shim:** the `repo` binary that the browser presentation layer expects is missing on this workstation. `native-repo-path` points to a deleted `repo.exe`, and the repo MCP server is down.
- **What the shim does:** [repo-shim.cmd](repo-shim.cmd) / [repo-shim.mjs](repo-shim.mjs) accept only `tools exec --light node -- …` and run node directly.
- **Proof:** [probe-repo-shim.py](probe-repo-shim.py) shows the shim works through the renderer's own subprocess path.

## Outcome

| Family | Videos | en speaker | en spectator | de speaker | de spectator |
|---|---|---|---|---|---|
| BIM (Villa Savoye) | information, objects, representations, models, exchange, consistency | 6/6 | 6/6 | 6/6 | 6/6 |
| Optimization (seminar-room façade) | extrema, sampling, experiments, index, pareto, algorithms | 6/6 | 6/6 | 6/6 | 6/6 |

- **Masters:** `.🧬semio/🎓️teaching/⚡️cache/videos/<language>/<audience>/rendered/*_1080p30.mp4`, each with a `.render.json` certificate.
- **Delivery:** H.264 High 4.1 exports with `delivery.json` provenance (sha256 of the export, the master and its certificate) are written to `…/<language>/<audience>/delivery/`.
- **Certificate check:** the speaker certificates for pareto and index hash the current `🏆️pareto`, `⚖️index` and `🎨️visuals/🏠️room` sources.
- **Spot frames:** frames from the BIM de spectator and optimization en spectator masters were checked visually: concrete values with their meaning, correct language, no overlaps.

## Incident: concurrent renders share segment slots

- **What failed:** the first full run rendered BIM and optimization at the same time for the same language. Several optimization segments then failed in Manim's `combine_files` with `FileNotFoundError: '<none>'`. That error is PyAV opening the in-memory concat manifest. The real cause was that partials `uncached_00000…00028` of `s/124/p/Beat5_ParetoFront` had disappeared, because both runs allocate from the same `videos/<language>/<audience>/s/<n>` slot directories.
- **What recovered:** the renderer's retry recovered the speaker segments. The spectator pass never started.
- **Fix:** the optimization videos were re-rendered on their own, after BIM had finished (en and de in parallel, which is safe because the language folders are separate). Every segment completed.
- **Rule:** never run two `render` invocations for the same language at the same time.

## Updates to deliver.py

[deliver.py](deliver.py) now covers both families (`BIM_<topic>` and `Optimization_<Topic>`) and writes one `delivery.json` per language and audience, with schema `semio.architecture.delivery/v1`.

Delivery results:
- **Exports:** 4 × 12 exports, each frame-checked against its master at 1920×1080.
- **Sizes:** en speaker 271 MB, en spectator 243 MB, de speaker 276 MB, de spectator 245 MB.
- **Durations:** 2:42–5:43 min per clip. For example, de spectator runs from BIM exchange (162 s) to Optimization algorithms (343 s).
- **Exit code:** the four runs exited 1 only because of the final `[DEBUG]` print of an emoji path to a cp1252 console, after `delivery.json` had been written. The script now reconfigures stdout to UTF-8.
