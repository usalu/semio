# Native build storage watchdog and stale statfs

Defect: the watchdog (`nativeBuildStorageFloor` in `component-pass/🟦️.ts`) confirmed pressure with `min(volume statfs, host free)`. On macOS the volume statfs of the Rust image goes stale-low and never recovers, so every release build was stopped with "No space left on device for the admitted native build reserve" near its end although the image held 75 GiB unallocated and the host 14 GiB.

Fix:
- `nativeImageRealAvailableBytesV1(volume, container, host)` in `💾️storage/🍎️image/⚖️availability/🟦️.ts` = `min(host, nativeImageEffectiveAvailableBytesV1(...))`, the host-capped container estimate. Re-exported from the package entry.
- `component-pass/🟦️.ts`: one local `realBytes` used by `reconcileHeadroom` (planning, same figure as before) and by the watchdog's confirm step. The cheap `min(statfs, host)` check still runs every second; the `diskutil` container probe runs only when that figure is below the reserve.
- Tests: corpus `floor` (5 cases) in `♻️reclaim/🧪️tests/🔣️.json`, Python oracle command `floor` in `🐍️.py`, test "storage floor corpus" in `🟦️.ts`. Cases: stale-low volume (1.9 GiB) with 14 GiB host does not stop; same volume with 1 GiB host or host one byte under the reserve stops; consistent low volume stops; unknown container keeps a roomy volume.
- Registered the file in the suite list (`🧪️testing/🗂️suites/🔣️.json`, id `native-image-availability`) and `📋️project.json` (`test-native-image-availability`) because the reclaim test file had no repo target.

Result: `repo run @semio-tech/framework-process:test-native-image-availability` passes, 2 tests (availability corpus, storage floor corpus), oracle included. The driver-level suite and a real build were not run.
