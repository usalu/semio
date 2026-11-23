# Stale volume figure of the macOS cache images: refresh, escalation and reserve

Defect: the raw UDRW `fixed.dmg` volumes sample the host free space when they attach and never refresh it. After enough writes the volume reports `0 B` available (container and host still have 15+ GiB), every write and even `rm` fail with ENOSPC, `hdiutil detach` answers "Resource busy". The only cure was a manual `diskutil unmount force`, detach and `diskutil image attach` (`remount-image.sh`).

What already existed: the pure correction `nativeImageEffectiveAvailableBytesV1` (admission and the watchdog no longer believe the stale figure), `nativeImageVolumeLatchedV1` and a `hdiutil detach -force` after a busy refusal only for a latched volume. Nothing ever made the volume itself true again, so the kernel kept refusing writes.

## What the `u/ballast` file is for

32 MiB of real bytes (`ballastFile`/`ballastBytes` in the unit-cache schema), created by the unit-cache collector and headroom pass while the image has four times its size free and released on ENOSPC so the collector can still create its claim and record files and then free the rest (`tryClaimWithBallastV1`, `roomy`, collector launch). It covers only those cache-internal writers and only the Rust image. It cannot help once the volume is at exactly zero and a delete needs metadata, and nobody released it before a detach.

## Design (schema first)

Schema `💾️storage/🧬️schema/🔣️.json`, `reclaim.availability.refresh`: per-OS `mechanisms` (only `darwin`: `diskutil unmount force`), `probeBelowBytes` (16 GiB), `retryMilliseconds` (5 min) and the image `reserve` (`.semio-reserve`, 32 MiB, created at 4x free; added to `reservedApplicationEntries`, so it is a platform service entry that application paths cannot enter and inventories ignore).

1. Pure decisions, language-agnostic corpus plus Python oracle (`♻️reclaim/🧪️tests/🔣️.json` `refresh`, `🐍️.py refresh`): `nativeImageRefreshProbeDueV1` (volume below the probe bound and below the host by more than the tolerance; avoids a `diskutil` spawn for healthy images) and `nativeImageRefreshDueV1` (effective availability exceeds the volume figure by more than the schema tolerance). `nativeImageRefreshMechanismV1(platform)` is null on linux and windows.
2. Port (`♻️reclaim/🟦️.ts`): `refreshNativeImageV1(port,hooks)` detaches the attached image and attaches it at its canonical mount point, restores the attachment on every exit path (failed attach, cancellation, refused detach), skips an unattached image, refuses a non-canonical port. `NativeImageReclaimPortV1` got an optional `volumeAvailableBytes`. The darwin port got `quiescent`: only then a busy refusal escalates, regardless of the latch (the caller holds the exclusive lease), via `hdiutil detach -force`, then `diskutil unmount force <mount>` and `hdiutil detach -force`, the last one tolerated when the device is already gone. Non-quiescent behaviour is unchanged (force only for a latched volume). Linux and Windows ports ignore it; the mechanism is null there, so the action is refused before any port exists.
3. Orchestrator (`opening/🔄️refresh/🟦️.ts`, schema `🧬️schema`): `refreshStaleImageIfQuiescentV1`. Order: platform, pending retirement, backing file, mounted, cheap `statfs` probe, container figure, retry stamp (tmp dir, keyed by the image directory, written at cycle start so a failed cycle is not repeated), exclusive nonwaiting `native-fixed-image:<dir>` lease (held means skip, never wait), re-measure under the lease, release reserve, cycle, create reserve again, receipt with before/after figures and `cured`. All effects are injectable ports. Only cancellation is raised.
4. Triggers: the partition opener (`openRepositoryCachePartitionAttemptV1`) runs it before opening any partition except `leases`, so a stale image is cured before a command can refuse for space, and a held image is left alone; `repo cache gc` / `--idle` run it for every partition before the host-block reclaim and print a bilingual line.
5. Reserve (`💾️storage/🍎️image/🛟️reserve`): create, release and `withNativeImageReserveV1` (release and retry once on ENOSPC). The opener keeps it whole (best effort). The unit-cache ballast now delegates its file logic to the same module (one implementation, its own file and size stay).

## Limits and open points

- The leases image itself is never refreshed from inside a command (its lease database is what proves quiescence); it needs the manual remount or the next process-free start.
- Mid-build admission (the component-pass watchdog) cannot cycle an image it uses; it keeps the existing effective-availability correction. The cure happens at the next opener.
- At exactly zero the reserve helps only the unmount flush and later deletes if it is released before the volume latches; the refresh action releases it first.
- Not run here (no real image cycle): the force-unmount ladder is verified with a scripted runner only; a real latched volume was not reproducible without host pressure.

## Tests (`repo tools exec bun -- test ./<file>`)

- `♻️reclaim/🧪️tests/🟦️.ts -t "refresh|corpus|latched|availability|floor"`: 20 pass (refresh and ladder corpus, oracle, mechanism table, cycle, cancellation, failed attach). The corpus schema gained the previously undeclared `floor` array, which made the schema-validity test fail before.
- `🛟️reserve/🧪️tests/🟦️.ts`: 10 pass (creation corpus with Python oracle, ENOSPC retry).
- `opening/🔄️refresh/🧪️tests/🟦️.ts`: 18 pass (14 scenario corpus rows, receipt schema, failing cycle, cancellation).
- Regression: unit-cache ballast and headroom tests, opening tests, funding replacement tests: 37 pass.
