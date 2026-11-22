# Rust image space (2026-11-21 23:2x)

## Before and after

| | before (df) | after (df) |
|---|---|---|
| host `/` | 13Gi used, 9.4Gi avail | 25Gi avail |
| `.cache/b/r/m` (96Gi image) | 30Gi used, 5.0Gi avail | 18Gi used, 24Gi avail |
| `components/runs` | 9.7G, 47 runs | 5.2G, 18 runs |

The collector itself freed 4.5 GiB (4,868,796,416 allocated bytes, 30 runs). Host free also rose from about 21Gi to 25Gi between my samples through concurrent activity not attributable to this action; the image tracks host free.

## What was removed (30 runs, via `collectComponentRunsV1`, the collector `repo cache gc` already calls)

- Superseded older builds of a component key: wmJDEkF (demonstrator 9e519b5a..., superseded by wc3Mufm), w79IQpU (energy bfc4d2be..., superseded by confirmed w2q3K5F), wqks1dx (procedural dc8f6f5f..., superseded by wmr64VB).
- 27 failed runs (logs and reports only, no materialized component), oldest first.

## Kept

- Every run backing a confirmed receipt, matched by provenance, source tree digest and wasm sha in each run's `r/provenance` and `l`: cad wWqEAMA, energy w2q3K5F, fem wFz8qpa, flow-extension-bim wVg9WoJ, flow-extension-dictionary wxhhk1M, gis wsl3Qi1, process wAPpMMO, puzzle wcbyL2f, sourcing-module-windows wqPqTuV. All verified present afterwards.
- Newest materialization of each key (wc3Mufm, wmr64VB, wtuEMdW), the 3 newest failed runs, and the running builds (held, `lsof` showed only wcNSkAp open).

## Root cause and fix

`repo cache gc` already wires the collector (`cargoIncrementalReclaim`, `🏗️build/🧹️cache-gc/⌨️command/🟦️.ts`), but the planner never selected anything (dry run: 0 of 47) because
1. optimizer scratch folders named `semio-hub-*-component` under `w/` counted as components, so the three superseded builds above looked like the only owners of a unique component;
2. failed runs newer than the newest materialized run were never removable, so each failed build leaked 50 to 130 MiB of logs forever.

Changes in `⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧩️runs/`:
- Schema policy: `artifactMarker` (`.semio-artifact.json`) and `retainedFailedRuns` (3). `componentRunComponentsV1` counts only marker-carrying output folders (and `.linked` twins).
- Planner: failed runs beyond the newest `retainedFailedRuns` are superseded whatever their age.
- Oracle `🐍️.py` reads the same policy; new fixture vector; test adds a scratch-folder case. 4 tests pass (`repo tools exec bun test`).

## Not done

- No image compaction (`repo cache gc` lease-timeout while a build holds the lease); the delete frees blocks inside the mounted image and trim on a diskutil-attached image returned them to the host as measured.
- Unit caches under `u` (11G) untouched; still governed by the normal gc pressure rule.
- Helper: `rust-image-run-collector.ts` in this folder (args: store, needBytes, dry|apply).
