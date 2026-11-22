# 🚀️ Play CDN Redeploy Readiness (2026-11-22)

## How to redeploy

1. Build and verify the release: `bash native-play.sh site && bash native-play.sh verify play-release`, run from this ticket folder. This is the native fallback path; the graph target is `@semio-tech/semio-tech-play:build`.
2. Export the verified cohort: `repo run @semio-tech/semio-tech-play:release-export`. Add `-- <folder>` to pick a target folder; the default is `🏢️semio-tech/🎡️play/dist/cdn`.
3. Upload `dist/cdn/{play,map,media,modules}`, one CDN host root per family, all from one export. The `CNAME` file in each family names its host.

## CDN rules

These are also written to `dist/cdn/🔣️cdn.json`; details are in [cdn-export.md](cdn-export.md).

- Serve each family at the root of its own host.
- Return 404 for missing files. There is no SPA fallback.
- Send `Cache-Control: no-cache` on every family.
- Send `Access-Control-Allow-Origin: *` on `map`, `media` and `modules`.
- Serve `.wasm` as `application/wasm`, `.js` and `.mjs` as `text/javascript`, and `.json` as `application/json`.
- Do not set COOP or COEP.

## What this session fixed

The fixes are numbered 155 to 169 in [📓️2026-11-17-redeploy.md](📓️2026-11-17-redeploy.md).

| Area | Fix | Report |
|---|---|---|
| Export | New `release-export` target. It copies the cohort under the retained reader, rechecks digests and writes CDN rules. | [cdn-export.md](cdn-export.md) |
| Surface WASM | The pick tolerance now matches f32 meshes. The browser law rounds query points to f32. | [surface-nodata-pick.md](surface-nodata-pick.md) |
| Storage | A reclaim mount stranded the dependency image. Reclaim cleanup and recovery, doctor detection and the loader fail-fast are fixed. | [stranded-storage-image.md](stranded-storage-image.md) |
| Churn | A wrapped concurrent edit made jobs exit 1 instead of restarting. The catalogue law now covers every `concurrentEditV1` site. | [workspace-length-churn.md](workspace-length-churn.md) |
| WASM churn | WASM producers now compile with retained admitted sources and dep-info proof. | [wasm-retained-source.md](wasm-retained-source.md) |
| Source limits | The plugin instance, PDF page editor and puzzle-2d snapshot crates are split. The ticket probe now runs the real admission. | [plugin-instance-split.md](plugin-instance-split.md), [pdf-page-editor-split.md](pdf-page-editor-split.md), [puzzle-2d-source-limits.md](puzzle-2d-source-limits.md), [boundary-probe-parity.md](boundary-probe-parity.md) |
| API drift | Fixed: job context retained-charge methods, time-travel and flow callers, and the hub closure crates. | [flow-wasm-api-drift.md](flow-wasm-api-drift.md), [closure-precheck.md](closure-precheck.md) |
| Supervisor | Boundary failures now wait and retry. `ATTEMPTS` defaults to 16. | `native-play.sh` |

## Status

As of 2026-11-22 17:00, no release cohort has been published yet.

- Supervisor 4139 is running. It is at about 650 units in the job stage, cold because the target dirs were re-keyed.
- The furthest an attempt has got: every WASM job passed and the Vite bundle ran (it now builds standalone), with 1 of 73 components staged.
- Everything after the component phase was preflighted, and two publication defects were fixed.
- Handoff: [📓️2026-11-22-handoff.md](📓️2026-11-22-handoff.md).
