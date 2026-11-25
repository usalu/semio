# PDF Official Routes (2026-11-25)

Both official routes now work. Neither needed a wasm or cargo build.

## 1. `repo run @semio-tech/stdio-pdf:test`: green
- Command: `repo run @semio-tech/stdio-pdf:test --lang=en --exclude-dependencies --output=stream` (log `🗑️generated/official-test-3.log`).
- Result: "Successfully ran target test for project @semio-tech/stdio-pdf", task time 47 s, 21 `bun test` file runs (20 suites; `pdfjs-page` is registered twice in `📜️script.ts`), 0 failures.
- Why it never completed before: it was never a test failure and not the `bun`-on-PATH problem.
  - Planning dominated. Each new tree state means a fresh graph generation (about 10 to 25 min while 4 other agents edit the tree). The first attempts timed out inside "Planning 1 targets" (`door.script`), and the 2026-11-22 run died waiting in the resident service.
  - The run is then gated by memory capacity (waited about 60 s to grow from 512 MB to 3510 MB on this 16 GB Mac).
  - Concurrent edits to build sources make the resident service refuse with `BUILD_SOURCE_UNPARSEABLE` or exit 75 ("source file edited by another process"). The retry script handles exit 75.
- `bun` on PATH: the route does not need it.
  - The suite runner executes `runOwnedCommand(process.execPath, ["test", suite])` (`⚡️caching/📦️artifacts/🟦️typescript/🟦️.ts` line 97).
  - The closure worker observed in `ps` was the repo-managed `.cache/b/t/m/bun/<hash>/bun --eval`.
  - Only ad-hoc scripts that spell `bun` need an absolute path. The repo runtime lives at `~/Library/Caches/semio/runtimes/bun/<hash>/bun` and `.cache/b/t/m/bun/<hash>/bun`. `repo ./📜️script.ts` resolves it itself.
- Code change: none needed for this route.

## 2. `repo run @semio-tech/framework-os-dev:dev-stdio-pdf-1-7-react-prototype`: serves
- Cause of "never confirmed serving": the generated catalog `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json` (and its twins) dated 2026-11-22 had no `devContribution` for `stdio-pdf-1-7`, although `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧩️composition/🔣️.json` line 304 declares it.
  - `ServeScript` therefore fell back to the generic Vite config, which threw `Unknown playground module identity: stdio-pdf-1-7` (`🏗️builder/🌐️vite/🟦️.ts:90`). Log: `🗑️generated/dev-official-1.log`.
- Fix: regenerate the catalog through the official generator, `repo run @semio-tech/s-dev-composition:generate --exclude-dependencies` (42 s).
  - It rewrote `🎠️playgrounds.json`, `🚀️playgrounds.json`, `🎮️playgrounds/🟦️.ts`, `🔌️plugins.json`, `🖥️hosts/🦀️.rs`, `🗂️sources.json`, `🦀️catalog.json` and `🧩️plugins/🟦️.ts` in `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`.
  - `stdio-pdf-1-7` now carries `devContribution = ✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🧬️schema/🔣️.json`.
  - The regeneration also picked up other agents' pending catalog changes. The diff was not reviewed beyond the PDF row.
- Serving without the wasm chain: the target's dependency chain is `activate`, then `prepare`, then `:component` (a wasm build).
  - `--exclude-dependencies` runs only the serve task. It reads the already staged component (`.cache/b/p/m/browser/prototype/modules/📘️stdio-pdf`) and the existing activation receipt.
  - The serve log prints `[stale] stdio-pdf: source-changed ... run activate-...`. This is informational only; the staged component is used as is.
- Command (wrapper `serve-pdf-official.sh`, retries only on exit 75): `sh serve-pdf-official.sh 20`. Plain form: `repo run @semio-tech/framework-os-dev:dev-stdio-pdf-1-7-react-prototype --exclude-dependencies --lang=en`.
- Results (log `🗑️generated/dev-official.log`): `VITE v7.3.7 prototype ready in 957 ms`, `Local: http://127.0.0.1:6260/`. `curl` returned HTTP 200.
- Headless check `check-official-boot.ts`, run with the repo bun and the repo Playwright (log `🗑️generated/official-boot.log`, screenshot `🗑️generated/official-boot.png`):
  - beacon `ready:stdio-pdf-1-7`;
  - the action rail and the page and metadata inputs rendered;
  - no console errors, page errors, failed requests or HTTP 4xx/5xx.
- Server stopped: after SIGINT to the `repo run` process, port 6260 has no listener and no `dev-stdio-pdf` process remains.
- The ticket route (`serve-pdf-native.sh`, `serve-pdf-native.ts`) is no longer needed.

## Open
- A full `repo run ...dev-stdio-pdf-1-7-react-prototype` without `--exclude-dependencies` still builds the wasm component through the `:component` dependency. The wasm build was deliberately not run here.
- The `pdfjs-page` suite is listed twice in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🟦️typescript/📜️script.ts`. That file was not edited in this round.
- Another agent's unfinished edit to `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🚦️admission/🟦️.ts` (import path missing the `🧰️framework/` prefix) briefly broke planning for everyone. It was not touched and cleared by itself.

## Files
- Created: `serve-pdf-official.sh`, `check-official-boot.ts`, `🚀️pdf-official-routes.md`.
- Regenerated: the `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` files listed above.
- Logs in `🗑️generated/`: `official-test-1..3.log`, `dev-dry*.log`, `dev-official-1.log`, `dev-official.log`, `catalog-generate.log`, `official-boot.log`, `official-boot.png`. Delete them at ticket close.
