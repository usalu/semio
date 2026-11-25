# 🔬 Site Build Preflight (read-only audit, 2026-11-24)

Scope: read-only inspection. Nothing was built, deleted, or changed in the repo or the ticket. Paths are relative to `/Users/ueli/Documents/semio` unless absolute.

## Blockers first

1. **31 of 32 roster modules have no queue confirmation.** `🗑️generated/confirmed/` contains only `procedural.json` (19:53). `native-site-verify.sh` with `VERIFY_QUEUE_RECEIPTS=1` and `package-site.sh` both refuse to continue until `release-receipts.py status` reports 32 of 32. Re-recording is possible (see 2), but see the caveat in 1a.
2. **A live process serves the old site output.** `node overlay-serve.mjs .../.cache/b/p/m/vite/release/5dc1b077e86de2aa/out .../browser/release/modules 8775` has been running for about 1d21h (pid 37114). Rebuilding the Vite release may collide with it. Its owner should stop it first. I did not touch it.
3. **Wasm prerequisites are not invoked by `native-wasm.sh site`.** That script runs `🔨️modules/📦️site/📜️script.ts build` directly. The README says `build` depends on the surface, editor, node-graph, puzzle, flow and canvas tasks. Their outputs must already exist (see 3), otherwise the Vite build may fail or ship incomplete output.

No script, helper or lock is missing. All helpers exist in T (`native-wire.ts`, `native-receipt.mjs`, `captured-site.py`, `site-manifest.py`, `verify-static.sh`, `verify-release-archive.py`, `validate-local-references.py`, `verify-browser-core-lineage.ts`, `authored-release-descriptors.ts`, `verify-extension-imports.mjs`, `static-origin.py`, `verify-boot-matrix.py`). No `static-verification.lock` exists. Port 8765 is free. Playwright Chromium `chromium-1248` is present.

## 1. Roster, staged modules, receipts

- Roster (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json`): 11 `pluginIds` plus 21 `extensionIds` = 32 modules.
- Staged in `.cache/b/p/m/browser/release/modules`: all 32 roster modules have exactly one `🧾️receipt.json`. Two extra dirs (`🪞️vendor`, `🧵️shard`) are support directories.
- Receipts in `🗑️generated/confirmed`: 1 of 32 (`procedural`). Missing: the other 31.
- Re-recording is supported: `python3 T/release-receipts.py record <modules> <confirmed> <id>` reads the staged receipt, re-certifies optimized browser cores through `verify-browser-core-lineage.ts` (stage cache under `.cache/b/r/m/components/stages`: transpile 25, optimize 25, probe 64 entries present) and writes `<confirmed>/<id>.json`. It works for already-staged modules.
- 1a. Caveat: `record` does not check the authored semantic descriptor. Only `verify` does (`authored-release-descriptors.ts`). A bad staged build would be recorded and only fail later. Run `status` and then the full verify before shipping.

## 2. Command sequence and env

```
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS
M=/Users/ueli/Documents/semio/.cache/b/p/m/browser/release/modules
C=$T/🗑️generated/confirmed
# for each of the 32 ids in cases.json (31 still missing; procedural already present):
python3 $T/release-receipts.py record $M $C <id>
python3 $T/release-receipts.py status $M $C            # expect 32 of 32, exit 0
VERIFY_QUEUE_RECEIPTS=1 bash $T/native-site-verify.sh   # builds, prints "SITE <path>", static acceptance on port 8765, writes 🗑️generated/static-accepted.sha256
bash $T/package-site.sh <SITE path from output> <OUT_DIR> $T/🗑️generated/static-accepted.sha256
```

- `native-site-verify.sh` sets `MEM=4096` and `NATIVE_SITE_RUN_LOG` itself. `native-wasm.sh` sets `SEMIO_BUILD_MODE=release`. Required on PATH: `repo` (`/Users/ueli/.local/bin/repo`), `node`, `python3`, `curl`. `BUN` and `PACKAGE_PYTHON` are optional overrides.
- Input files that were lost and are regenerated, not needed as input: `native-site.log` (written by the build run), `static-accepted.sha256`, `static-before/after.sha256`, the lock dir, `static-server.log`.
- `captured-site.py` reads the run log produced by the same invocation, so no old log is required.
- Not needed for the site path (lost but unrelated): `roster-pairs-state.json`, `0751-expanded-roster.log`, `browser-procedural.log`, `describe-build.log`.
- `package-site.sh` has no default output dir. Pick one outside the ticket, e.g. under `.cache/b/p/m/`.
- Verify requires a real GPU adapter (ANGLE Metal on macOS; the map pane cannot be graded under swiftshader, per the README). The build prefetches map tiles and needs network. Neither was tested.

## 3. Wasm prerequisites

| Engine | Receipt present | Output bytes located |
|---|---|---|
| surface-rs | `🗺️surface/📦️packages/🦀️rust/.🧾️store/wasm.json` (Nov 18) | no |
| editor-rs | `✍️editor/.../.🧾️store/wasm.json` (Nov 18) | no |
| node-graph | `NodeGraph/.../.🧾️store/wasm.json` (Nov 18) | no |
| puzzle | `🌎️hub/.../🧩️puzzle/.../.🧾️store/wasm.json` (Nov 18) | no |
| flow browser | `🎁️browser-package/.../.🧾️store/wasm.json` (Nov 18) | no |
| canvas runtime assets | `🦀️rust/.🧾️store/stage-runtime-assets.json` (Nov 18); targets under `.cache/b/p/m/canvas/release/guest-assets/` | directory present, contents not checked |

I did not find the four engine `.wasm` outputs under the package dirs or under `.cache` (`find` for `*.wasm` found only tree-sitter, manifold, esbuild and draco). They are presumably in the shared store (`.🧬semio/🦑️repo/⚡️cache`, 590 MB) or the products store, which I did not trace. Confirm them before the build, or run `repo run @semio-tech/mit-bestand-demonstrator:build` (graph-driven) instead of the direct script.

## 4. Disk

- Free: 31 GiB on `/System/Volumes/Data` (85% used).
- Previous site output `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out` (Nov 22): 2.5 GiB, 2103 files, includes `CNAME` (38 B), `.nojekyll`, `index.html`, `404.html`, `.semio-artifact.json`.
- Largest files: energy `core.wasm` 267.6 MB; procedural 69.4 MB; puzzle `_bg.wasm` 72.5 MB.
- Tar.gz size: not measured. Estimate 1 to 1.5 GiB.
- Build plus packaging peak: roughly 4 to 5 GiB extra. Fits in 31 GiB.

## 5. Deploy target (`♻️mit-bestand/🧺️demonstrator/README.md`, Deploy section)

- Served at the root of `demonstrator.entwerfen.mit-bestand.de` (CNAME record). No sub path, because routes and assets are root-absolute.
- CDN must serve `.wasm` as `application/wasm`, `.pbf` as `application/x-protobuf`, serve dotfiles, and return 404 (not `index.html`) for missing files.
- `package-site.sh` enforces `index.html`, `CNAME` and `.nojekyll`. Output also includes `404.html`.
- GitHub Pages limits (100 MB per file, 1 GB site) would fail: the energy core wasm is 267.6 MB and the out is 2.5 GiB. Irrelevant for a CDN, but a blocker if the target is GitHub Pages. The README says CDN.
