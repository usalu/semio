# Handoff Prompt: Continue Mit-Bestand Demonstrator CDN Redeploy

Continue the open ticket `2026/11/17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS` (folder `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS`). Follow `CLAUDE.md`. Read `🚀️redeploy-log.md` and every `🔬️*.md` note in the ticket first.

## Live continuation on November 21, 08:32 PST

The original Rust image is mounted at its canonical path and had 16 GiB available at this check. The holders are active release compilers. The roster supervisor (PID 51044 at this checkpoint) is managing both required JOBS=2 queue roots. Demonstrator wakSUm7 and Procedural w5PGoij are linking. Both linked-browser probes are armed. GIS wsl3Qi1 passed 11 stages with a certified optimized core. The latest full site result is 6/16, and the current published Generator predates the correction in w5PGoij. The release verdict and subsequent checkpoints are in 📋️readiness-report.md and 🚀️redeploy-log.md.

## Goal (unchanged)

Get mit-bestand demonstrator ready to be redeployed on a CDN as fast as you can. It doesn't matter for this redeploy if it contains bugs, it just needs to run every app. At the same time parallelly the shared work fleet mechanism is being developed. If it doesn't work, just fallback to native tools. Under no means get stuck or hold your work. Use the main chat with GPT 6.2 Astra Medium for main plan coordination, use multiple GPT 6.2 Sol Medium agents for medium to high complexity task execution, use multiple GPT 6.1 Terra Medium agents for read-only exploration, audits and low complexity tasks. Finish the goal as fast as you can with all the resources you can get and make sure to never idle and always be saturated. When you find fundamental issues which are unrelated to the task fix them as well. Treat the goal completely end to end. There are other agents working on other logical parts, even if they work on the same files, don't stop and keep on to work in conjunction with them, just focus on your task.

## State at handoff (2026-11-19 ~17:00)

- The site builds: `MEM=4096 bash native-wasm.sh site`, output `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out`, last built 15:01 with current TS.
- `verify-static.sh <out> 8765` runs the 16 boot tests (8 panes × en/de). Currently 0/16 pass.
- Overlay probe server: `node overlay-serve.mjs <site-out> .cache/b/p/m/browser/release/modules 8767` serves the site with live staged modules, so you can test without a site rebuild.
- Boot test against the overlay: from `♻️mit-bestand/🧺️demonstrator`, run `PLAYWRIGHT_BASE_URL=http://127.0.0.1:8767/ PLAYWRIGHT_BROWSERS_PATH=$ROOT/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright node <@playwright/test cli.js> test --config ./🔨️modules/🧪️e2e/🎚️config/🟦️.ts --grep "<pane> boot$" --project en`.
- After any TS change in the shard worker, restage it with `native-lanes.sh step browser-support "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript" repo ./📜️script.ts support`. The site build does not refresh it.
- Module rebuilds:
  - Use `JOBS=2 ./native-queue.sh <ids…>`. It runs bounded concurrency and retries once.
  - Never launch 20+ describes at once: the headroom queue plus the watchdog kills them.
  - `native-sep.sh <id>` builds a single module.
- **Blocker 0 (disk):** the rust image `.cache/b/r/m` reports 0 B available (43 of 96 GiB used; host has 21 GiB free), so writes stall and even shells hang. This is stale statfs after mount, see `💽️image-reclaim.md`. `remount-image.sh r` just failed with "Resource busy": find the holder with `lsof +D` (likely a leftover cargo, rustc or bun from another session or an agent), stop it, remount, then confirm the host and image free space. Optionally run `repo cache gc`, and delete stage cache or old runs under `.cache/b/r/m/components`.
- Fresh modules, built with all value fixes up to 14:40: `procedural` and `energy`.
- Every other module must be rebuilt after the fixes below compile. That covers the hubs cad, demonstrator, fem, playbook, sourcing, puzzle, gis, flow and process, and all extensions.

## Open blockers (in order)

1. **generator (procedural):** `setContributions` refused with "command wire exceeds its declared maximum".
   - The fix is written but uncompiled: four apps override `original_action_command_producer` with `OriginalActionCommandBridge`, priced at their declared contributions constant (`🔬️command-wire-limit.md`).
   - Run `repo cargo --store=shared --configuration=prototype -- check -j 2 -p semio-s-artifact-procedural-3d-editor -p semio-s-artifact-procedural-2d-editor -p semio-s-artifact-flow-flow-editor -p semio-s-artifact-procedural-3d-viewer`.
   - Then run the test `a_pack_wider_than_the_ordinary_runtime_wire_reaches_its_command`, rebuild `procedural`, and re-probe `#generator`.
   - Risk: a single ~557 KB buffer may exceed the guest soft ceiling. If so, add a paged encoder through `ArtifactOriginalCommandWire`.
2. **energie (energy):** `plugin-ui.native-lifetime-required`.
   - The open crossing stalls after about 21,600 epochs on identical no-progress turns.
   - Candidate steps and the next debugging step (add `[DEBUG]` step labels, rebuild energy, run `POLL=2 node energie-console.mjs`) are in `🔬️energie-native-lifetime.md`.
3. **Other panes** (koordinator, aggregator, aussuchen, bearbeiten, verfolgen, statik): they fail on stale, unrebuilt hubs with "original total exceeds its persistent source recipient" and `settle table_false`. Rebuild all modules with the current source first, then re-diagnose per pane.
4. **Release site polls `/__semio/agent-bridge` (404 spam):** fixed in TS, so it needs a site rebuild. The 7 vitest-only tests are unverified.

## Fixed this session (verify in the log; tests mostly via `repo tools exec bun -- test`)

- Owner keys and 8 GiB artifact bounds.
- Post-build snapshot race.
- Vite capacity-before-artifacts lock order and admission wait.
- Lease `SQLITE_IOERR_VNODE`, open and exec reopen paths, and the claim `nlink` race.
- Headroom gate: queue wait now counts as progress, plus effective free space.
- Value crate source-owner and holder re-binding between actions.
- Flow catalogue owner and tool-run use `with_original_source_effect`.
- Energy genesis `StagedStep`.
- Host command-page re-offer on backpressure.
- Shard `replyError` bigint serialization.
- Catalog fixture counts.
- `dsl-notation` u32 byte ranges.

## Not caused here

`semio-os-flow-host-laws` has 234 pre-existing compile errors, so the catalogue law there is unrun.

## Finish

1. Once the final queue-verified 16/16 boot tests pass, run `package-site.sh <site> <durable-output> <ticket>/🗑️generated/static-accepted.sha256`.
2. Update `📋️readiness-report.md` with the verdict, and list every changed file in `🗂️changed-files.md`.
3. Remove the temporary `[DEBUG]` lines added by this ticket.
4. Delete `🗑️generated`.
5. Run `ticket_close` with a summary and the file list.
