# Sealed door closure under the admission bound

## Cause

The sealed door entry (`🏛️sealed/🚪️entry`) was captured at 2049 to 2050 files and 18.2 MB against the declared bound of 2048 files and 20 MiB, so no snapshot could be promoted. Measured with `seal-census.ts` (the same census the promoter uses, installed packages on): 1300 code files by import, 713 JSON files by data discovery, 36 declared companions. The verb kernels were already outside the door. The door itself holds one dense graph: `door/main` reaches `script.ts`, which reaches about 1800 files through many independent paths. A single cut rarely drops more than a few dozen files.

Key finding: the scanner follows literal dynamic `import()`. Making a command family lazy therefore never shrinks the closure; only removing the last edge any door module has to it does. The seal already supports per-command closures as separate entries (the door plus eight kernels in one partition group), so the right mechanism is: what only a kernel runs must be unreachable from the door.

## Changes (one mechanism: unreachable from the door)

1. Warm manifest and entry (about 100 files). `🏡️resident/🏗️composition/🔣️.json` no longer warms `🔧️tools/💾️storage` and the Bazel script; the entry no longer exports `sealedWarmStorageV1`. Only the task kernel calls `prepareBuildToolStorageV1`, so the door never needed the storage image, and with it the three backend adapters (declared roots of the loader) and the Bazel script left the door.
2. Backend facets. `🔌️backends/🟦️.ts` is now the light registry (rows, ids, defaults, environment). `🔌️backends/🚚️loading` holds `loadBuildBackendV1` (kernels and workers only; owner of the declared computed roots in the authorizer schema). `🔌️backends/🚀️launch` holds `buildBackendBootstrapV1`, which the door asks for the launch of a backend's own tool; each registry row names an optional `launch` module (`🔧️tools/🧶️nx/🚀️launch`), so the door loads a ten line module instead of an adapter. `bootstrapArguments` left the `BuildBackendV1` interface. All call sites moved to the module that owns what they need.
3. Door argument pre-validation. `📥️caller/📤️child` no longer imports the incremental, conformance, concurrency and bench argument readers; each command re-parses its own arguments in its kernel and refuses the same way (31 files).
4. README of the sealed layer corrected (the entry carries the door only).

## Result

| | files | bytes |
|---|---|---|
| before | 2050 | 18.2 MB |
| after | 1922 to 1926 | 16.2 MB |
| bound | 2048 | 20 MiB |

Headroom 122 files and 3.7 MiB. Law: `🏛️sealed/🧪️tests/📏️closure` keeps the entry at least 64 files and 2 MiB under the bound, asserts that no backend adapter, loader, storage image or verb kernel is in the door closure, and that every catalog operation and every selection case still resolves to an existing kernel (kernel closures stay bounded by `📥️source/🔏️closure/🧪️tests/📏️kernels`). Passed: 3 tests.

## Promotion

`SEMIO_REPO_SEAL=promote repo` exit 0. Fleet status: `🏛️sealed/🚪️entry 5d20d8b498d1 promoted, 1926 files, fresh` and `⌨️command/🩺️preflight fda52f8c76e7, 1451 files`. Other kernels promote on first use.

`repo run workspace:components -- describe norm` now starts from the sealed snapshot (no live capture, no slice worker restart). In this session it stopped later at the graph service: it was denied access to the checkout and restarted once more; that is the service host access issue, not the seal, and the service host was left untouched.

## Not fixed and follow-ups

- The door closure is still about 1900 files. Real structural headroom needs the resident host, graph and the `build`/`test` script classes in their own entry; every single remaining edge was measured and none removes more than about 60 files alone.
- `🔌️backends/🧪️tests` fails on `🏃️execution/🧬️schema/🌐️remote-cache.json` (3 backend names, limit 2): a leak by another change, unrelated to this one. The native caller tests need the repo test harness and could not run under bare bun.
- Inputs: `seal-census.ts` (census by entry), `seal-edges.ts` (edge dump).
