# CDN Final Steps Audit: mit-bestand Demonstrator

Read-only audit. No source, receipt or generated file was modified. No build, roster or Playwright run was started. The only executed check on the existing output was `validate-local-references.py` and read-only scans of `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out`.

Paths used below:

- Ticket: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS`
- Roster source: `/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json` (11 plugins + 21 extensions = 32)
- Browser release receipts: `/Users/ueli/Documents/semio/.cache/b/p/m/browser/release/modules`
- Last Vite release output: `/Users/ueli/Documents/semio/.cache/b/p/m/vite/release/5dc1b077e86de2aa/out` (2.6 GB)

## (a) Roster status

Confirmed receipts in `🗑️generated/confirmed/` (8 of 32): cad, energy, fem, flow-extension-bim, flow-extension-dictionary, gis, procedural, puzzle. No confirmed receipt exists outside the roster.

Missing (24 of 32), with the last state seen in each `🗑️generated/browser-<id>.log`:

| Status | Modules | Evidence |
|---|---|---|
| Running now | sourcing (pid 959), sourcing-module-windows (pid 55410) | Both `native-queue.sh` roots are live. Log at 15:38 shows 0/1 staged. |
| Failed: process / demonstrator | demonstrator, process | `semio-hub-demonstrator-component` and `semio-hub-process-component` (release) fail. Cause is `semio-s-artifact-process-process3d-editor` and `semio-s-artifact-stdio-xlsx-base-mutations` not compiling. Both are in `retry_due` in the state file. |
| Failed: shared Rust compile error | flow-extension-math, flow-extension-text, flow-extension-list, flow-extension-logic, flow-extension-primitive, flow-extension-draw, flow-extension-brep (warm, not confirmed), cad-extension-aec-building-energy, cad-extension-aec-building-structure, cad-extension-spatial-shape, sourcing-module-beams, sourcing-module-slabs, flow (different error, see below) | Most logs say `could not compile semio-plugin-window-config (lib)`. Some `cad-extension-aec-building` logs say `semio-framework-3d-brep-native-registry`. The rustc error line was not extracted; read the `report.md` under `.cache/b/r/m/components/runs/<run>/r/`. |
| Failed: other | flow (`semio-plugin-window-config`, 00:25), playbook (1 failure, 3526 s), cad-extension-aec-building (`semio-framework-3d-brep-native-registry`) | |
| Warm generation, no final result | playbook-module-procedural, process-extension-concrete, process-extension-metal, process-extension-robotic, process-extension-wood | Logs end at `warm generation ... 1 component build(s)`. Treat as not done. |

State file (`🗑️generated/roster-pairs-state.json`): `version 1`, `failed: []`, `retry_due: [demonstrator, process]`, `forced_demonstrator: true`, active roots `sourcing` and `sourcing-module-windows`.

Coordinator and lock: `roster-pairs-state.json.lock/owner.json` names pid 51044 (started 07:51). That process is still alive. Do not start a second `roster-pairs.py`. It would refuse with "roster coordinator advisory lock held" or "live owner".

Result: the final static acceptance cannot pass yet. 24 receipts are missing, and two roster modules (demonstrator, process) have hard compile failures.

## (b) Ordered commands: site build, static matrix, package, archive check

Preconditions (all must hold before step 1):

1. `ls "$T/🗑️generated/confirmed" | wc -l` prints 32, and no `*.json` is missing against the roster list in `cases.json`.
2. `ps -p 51044` shows no process (the roster coordinator has exited) and no queue roots are active in `roster-pairs-state.json`.
3. Decide the archive directory outside the ticket, for example `ARCHIVE_DIR=/Users/ueli/Documents/semio/.cache/site-archive`. The archive is a deliverable and must not sit in `🗑️generated`.

Set once:

```
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS"
ARCHIVE_DIR="/Users/ueli/Documents/semio/.cache/site-archive"
```

1. Site build and acceptance (build, capture, manifest, receipt verify, static matrix, accept):

   ```
   cd "$T" && VERIFY_QUEUE_RECEIPTS=1 MEM=4096 bash ./native-site-verify.sh 2>&1 | tee "$T/🗑️generated/cdn-site-verify.log"
   ```

   - Internals: `native-wasm.sh site` runs `repo ./🔨️modules/📦️site/📜️script.ts build` in `♻️mit-bestand/🧺️demonstrator` under `repo capacity run`.
   - Inputs: confirmed receipts, browser release modules, `🏗️builder/🌐️vite/🟦️.ts`, map tile cache, `🧪️e2e/🎚️config/🟦️.ts` and `🧫️cases.json`.
   - Outputs: a `SITE <OUT>` line, `🗑️generated/native-site.log`, `🗑️generated/static-before.sha256` and `static-after.sha256` (cmp-equal), `🗑️generated/static-accepted.sha256`, `static-playwright.json`, `static-verify.log`, `static-server.log`. A final line `16 pane/language boots passed, N origin requests, zero agent-bridge requests` means success.
   - Ends with `verify-static.sh <OUT> 8765`, which serves `<OUT>` on 127.0.0.1:8765 and runs Playwright with `PLAYWRIGHT_BASE_URL=http://cdn.localhost:8765/`.
   - Set `OUT` from the `SITE` line: `OUT="$(grep '^SITE ' "$T/🗑️generated/cdn-site-verify.log" | tail -1 | cut -c6-)"`.

2. Re-run only the static matrix if needed (no accepted manifest is written by this step):

   ```
   cd "$T" && bash ./verify-static.sh "$OUT" 8765
   ```

3. Package (validates references, rebuilds the manifest, checks it against the accepted manifest, writes the archive):

   ```
   mkdir -p "$ARCHIVE_DIR" && bash "$T/package-site.sh" "$OUT" "$ARCHIVE_DIR" "$T/🗑️generated/static-accepted.sha256"
   ```

   - Requires `index.html`, `CNAME`, `.nojekyll` in `$OUT`. All present.
   - Runs `validate-local-references.py` (python via `repo tools exec python` unless `PACKAGE_PYTHON` is set), then `site-manifest.py`, then `cmp` against the accepted manifest.
   - Output: `$ARCHIVE_DIR/site-<UTC YYYYMMDDTHHMMSSZ>.tar.gz`, and the line `archive: <path>`. Inside the archive: all site files plus `./SHA256SUMS` at the top level.

4. Archive check (also run by package, rerunnable standalone):

   ```
   python3 -I "$T/verify-release-archive.py" "<archive path>" "$T/🗑️generated/static-accepted.sha256"
   ```

   Expected output: `Verified <N> files and <M> bytes in the release archive`. Optional: `tar -tzf "<archive>" | head`.

5. Close-out per CLAUDE.md: record results in the ticket markdown, then delete `🗑️generated` (all tool output, logs, receipts copies). Keep the scripts and this audit file. The archive stays outside the ticket.

Note on runtime: step 1 takes about 15 minutes for the Playwright run on this Mac (last full run 14.7 min). The site build needs network the first time for map tiles (see (c)).

## (c) Issues that would break or weaken the final run

Blockers:

1. Roster incomplete. `release-receipts.py verify` (run by step 1 with `VERIFY_QUEUE_RECEIPTS=1`) raises `awaiting successful queue receipts for ...` unless all 32 are confirmed. Demonstrator and process need real compile fixes, not retries.
2. Accepted manifest depends on `VERIFY_QUEUE_RECEIPTS=1`. In `native-site-verify.sh`, `static-accepted.sha256` is written only inside the `VERIFY_QUEUE_RECEIPTS == 1` branch, and `rm -f` clears it at start. Without the flag, `package-site.sh` fails with `accepted manifest missing`. The flag is therefore mandatory.
3. Dev agent-bridge requests fail the matrix. `verify-boot-matrix.py` fails on any access-log request to `/__semio/agent-bridge`, regardless of status. The release bundle `assets/🌐️-DLq6yBBu.js` contains the constant `HRe="/__semio/agent-bridge"`. The earlier `🗑️generated/static-server-debug.log` contains 173 `404` responses to that path from 2026-11-19. The current build's boot behaviour was not re-run, so this is unconfirmed for the current output, but it is the most likely matrix failure. The release build should not reference the dev bridge at all.
4. The last full matrix run in the ticket (`0515-static-matrix.log`) shows `6 passed`, not 16. No matrix in this ticket has yet passed for all 16 cases.

Relocatability and CDN issues:

5. Absolute root paths inside JS. `base: "./"` in `🏗️builder/🌐️vite/🟦️.ts` only rewrites static asset references in HTML and CSS. Runtime strings in the bundle still use absolute routes such as `/mesh`, `/renderer-modules`, `/🔌️plugin-modules`, `/🧩️extension-modules`, `/infinite-assets`, `/cad-assets`, `/osm`, `/vt`. The site works only when served from the domain root, which matches the CNAME `demonstrator.entwerfen.mit-bestand.de`. A subpath CDN (for example `host/demonstrator/`) would break. This is a decision to make before CDN deployment, not a script bug.
6. Map tile prefetch is network-dependent. `🔨️modules/📦️site/🟦️.ts` calls `prefetchDemonstratorMapTiles`, which downloads tiles unless they are cached (`skipExisting: true`). A clean machine or offline build fails, and the shipped tiles depend on the provider state at build time. Not hermetic.
7. Dev-only config in the builder directory. `🏗️builder/🐚️shell/🟦️.ts` sets `browserEntry: "/@fs/" + ...`. It is not used by the site build (no `/@fs/` string in the output), but it is an absolute dev path in the builder tree.
8. Shipped test fixtures. The output contains 22 `🧪️tests/` paths under `assets/` (for example `🖼️assets/🗺️tile-proxy/🧪️tests/🧫️fixtures/🔣️.json`). They add bytes to the archive and should be excluded from the Vite asset plugin input.
9. Size. The output is 2.6 GB (`🔌️plugin-modules` 1.7 GB, `🧩️extension-modules` 573 MB, `assets` 96 MB, `vt` 69 MB). The archive will be large and slow to build. Expect several minutes in `tar -czf` and a streamed re-read in `verify-release-archive.py`.

Scripts and paths:

10. Dev local-reference scan passes. `validate-local-references.py` on the current `out` returned 0 failures. The `localhost` allowance covers only the main `🌐️-*.js` bundle and `pdf-*.js`, so any new local literal elsewhere will fail package.
11. Hard-coded absolute and repo-relative paths in ticket scripts:
    - `roster-pairs.py` computes `ROOT = TICKET.parents[6]`. `verify-boot-matrix.py` uses `parents[7]`. Both are correct only while the ticket stays at its current depth. Moving the ticket breaks them silently.
    - `verify-static.sh` hard-codes `PLAYWRIGHT_BROWSERS_PATH="$ROOT/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"`. It exists here (chromium-1248) but is machine-specific.
    - `verify-boot-matrix.py` reads a different cases file (`🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json`, 26 titles, 8 with ` boot`). The roster uses `🧩️runtime/🧫️cases.json`. Two sources of truth. The matrix count (16) is correct today.
12. Portability for devcontainer, Windows, Linux (per CLAUDE.md). `native-site-verify.sh`, `verify-static.sh`, `native-wasm.sh`, `package-site.sh` are bash-only (`mktemp`, `mkdir` locks, `kill -0`, `tar -czf` with `COPYFILE_DISABLE`, `date -u`). `native-site-verify.sh` and `verify-static.sh` call bare `python3` and `curl`. `package-site.sh` calls `repo tools exec python`. Mixed. `roster-pairs.py` handles Windows (`powershell.exe`) but the release scripts do not.
13. `package-site.sh` output directory has no default and no guard against being inside the ticket. The archive mode from `mktemp` is 0600 and is not reset before `mv`.
14. `native-site-verify.sh` uses `verify-static.sh ... 8765` with a fixed port. A stale server on 8765 makes the readiness loop succeed against the wrong origin. Check `lsof -i :8765` before step 1.
15. `release-receipts.py verify` shells out to `repo tools exec --light bun` (authored descriptors, core lineage) and `node verify-extension-imports.mjs`. Both helpers exist in the ticket. Step 1 therefore depends on `repo` and Bun being available; this was not executed here.

Missing files: none of the scripts named in the task is missing. `static-origin.py`, `verify-browser-core-lineage.ts`, `verify-extension-imports.mjs` (referenced by `release-receipts.py`), `authored-release-descriptors.ts` and `captured-site.py` all exist in the ticket folder.
