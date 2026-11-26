# 🪚️ Sourcing hub component carving under the beyond-host rule (2026-11-26)

## Problem

`unitBeyondHostV1` (admission, `🏔️ceiling`) refuses on a 16 GB host every unit whose largest known peak (carving entry `measuredMegabytes` or learned `<crate>.peak`) is above the final budget (4096 MB). The carving list still held the thin-LTO baseline of the sourcing closure from 2026-11-24, so the hub component final (5519) and the slabs/windows finals (4196/4116) were refused, and the hub lib (4273) too. The sourcing app could not be rebuilt.

## What dominates the hub final (measured on the 2026-11-25 artifact and the 2026-11-24/25 runs)

- Not debug info. The built `semio_hub_sourcing_component.wasm` is 280 286 068 bytes; its one core module holds a 261 473 880 byte code section (93 percent of the file, 737 468 functions), 16.99 MB data, 0.8 MB element, no names/DWARF section. The component row already has `debug: 0` and `strip: symbols`; nothing is left to strip.
- The final crate itself is a 1-line `cdylib` entry (`semio-hub-sourcing-component`, only `semio_plugin` glue); the cost is the closure it links (918 crates, 744 of the workspace).
- The memory is the link-time stage of the compiler: with `lto: thin` the compiler holds the bitcode of the whole closure (5519 MB, rerun 4450 MB, 1694 s). With the `component-lean` row (`lto: off`, `opt-level s`, 16 codegen units, serial back end) the same final measured 2523 MB, 465 s (ticket WARDEN-COVERAGE... `🪚️sourcing-unit-carving.md` section 4); wasm-ld then only gc-sections the closure (255 MB wasm, 722 320 functions, 28 percent head room to the 1 000 000 function limit).
- The profile schema already selects that row per composition: `Protocol.leanCompositions.prototype` lists `semio-hub-sourcing-component` and the three plugin finals (beams, slabs, windows), and the ingress derives the composition from the single `-p` of the cargo call, which `native-wasm-build.sh` always passes. So the profile needed no change; the stale part was the carving list, which still carried the thin-LTO numbers.

## Changes

1. `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json` (`Policy.unitCeiling.carving`):
   - removed `semio_hub_sourcing_component` (5519/5600), `semio_s_plugin_sourcing_slabs_component` (4196/4400) and `semio_s_plugin_sourcing_windows_component` (4116/4300): the lean row measured 2523 MB for the hub final and every one of them is a final under the 4096 MB final budget; the warden still cancels any of them above 4096 while it runs.
   - `semio_hub_sourcing` lib: 4273/4500 (unsplit, parallel back end) became 2824/3100, the measured serial-back-end floor of the unsplit crate (`-Z threads=1 -Z no-parallel-backend`, section 3 of the earlier carving report), a measured upper bound for the crate after the editor/viewer app split; below the 4096 MB refusal line. It is removed once a post-split run shows at most 1536 MB.
2. Tests/fixtures that named the removed rows as their "listed final" were moved to another listed final (`semio_hub_cad_component` 8036/8500) and a law was added that no sourcing closure final is carved: `🏔️…/🧪️tests/🧢️unit-ceiling/🟦️.ts` and its corpus `🔣️.json`; the warden ceiling test and fixture (`🛡️warden/🧪️tests/🟦️.ts`, `🧫️fixtures/🔣️.json`) follow the new `semio_hub_sourcing` numbers (a peer had already moved the hub final there to the default final ceiling).
3. Learned peaks: no `.peak` file exists for any sourcing closure crate (`.cache/b/r/m/u/c`), so no stale learned value keeps refusing. The rule is deliberately left as it is: a learned peak above 4096 only decays by measurement (`offloadLearnPeakMegabytesV1` halves toward a lower measurement), and a refused unit is never measured. The existing re-measure path is deleting the crate's `<crate>.peak` file of the cost folder (`readUnitPeaksV1`) after carving, or an admitted replay (`🪚️unit-variant.sh` under `repo capacity run`); neither was needed.

## Before / after (peak resident MB)

| unit | before (thin LTO, 2026-11-24) | after |
|---|---|---|
| `semio_hub_sourcing_component` final | 5519 / 4450 | 2523 (lean, 2026-11-25) |
| `semio_s_plugin_sourcing_slabs_component` final | 3565 / 4196 | lean build: see run below |
| `semio_s_plugin_sourcing_windows_component` final | 2148 / 4116 | lean build: see run below |
| `semio_s_plugin_sourcing_beams_component` final | 1591 | lean build: see run below |
| `semio_hub_sourcing` lib | 4273 | provisional 2824 (serial floor) |
| hub wasm | 269 MB (thin) | 255 MB (lean) / 280 MB in the exported 2026-11-25 artifact |

## Measurement status

A fresh rebuild of the closure was started through the admitted entry (`hub-carve-build.sh`, then `hub-carve-finish.sh` for the three extensions, under the shared lock) but the machine has one compile slot (`.cache/b/r/m/u/s/0.lock`), held for hours by a peer unit that waits on a remote worker, so each unit waits the full 10 minute slot timeout and the cold 918 crate lane (the old lane was garbage collected, 17 GB free) advances about six units per half hour. The build keeps running in the background; its results land in `🗑️generated/hub-carve/` (`build-hub.txt`, `build-ext.txt`, `peaks.txt`). The numbers of the table marked "see run" and the final value of the `semio_hub_sourcing` entry must be taken from `peaks.txt` (`python3 hub-carve-peaks.py <since-epoch> sourcing` prints `peakRssBytes` of the `miss` events of the lane logs).
