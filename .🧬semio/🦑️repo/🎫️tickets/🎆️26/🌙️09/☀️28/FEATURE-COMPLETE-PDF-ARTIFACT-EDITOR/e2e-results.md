# 🧪 E2E Results: PDF Artifact Editor (mounted browser suite)

## Run 1 (2026-11-22)

| Step | Result | Detail |
|---|---|---|
| Preflight (catalog json, port 6260 free, disk 56 GiB) | pass | catalog `plugins.json` exists, 6260 free |
| 1a `pdf-browser-direct.ts catalog` | queued | `repo tools exec bun` waits on capacity gate: need 2048 MB, short by ~3 GB (load 15-18, behind semio-plugin-interaction). Log: `🗑️generated/e2e/catalog.log` |
| 1b session / 1c components / 2 serve | not started | depend on 1a |
| 3 suite cases (starter, rotation, content, style, page-history, png, large-*, cancel, reject-invalid, image-cancel, first-archive, hub-check-in) | not run | |

## Chain runner (started 15:14)

`e2e-chain.sh` runs catalog, session, components, then each suite case separately with retries. Progress is in `🗑️generated/e2e/chain.log`, per-step logs beside it. Rows below are updated as steps finish.

| Step | Result |
|---|---|
| 1a catalog | pass (15:19, 74 plugin crates) |
| 1b session | pass (15:23) |
| 1c components | running (15:23) |
| 1c components attempts 1-5 (15:23-15:53) | exit 75 each: launcher restarts when other agents edit its source mid-run (`Another process kept editing ... (1 of 6)`), gave up |
| Resume 20:51 | `RESUME=1 ./e2e-chain.sh` skips catalog/session, components with 30 tries |
