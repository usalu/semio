# Interpreter numeric split and Play selection

The three `optLevel: "3"` additions are in the native sections of the repository, runtime, and streaming profile groups. They changed a shared configuration input during the font-assets job. The job’s configuration check correctly requested a retry after Cargo completed.

The numeric package is a framework workspace member, appears in its member-manifest metadata and the build-owner registry, and is a direct interpreter dependency. The repository, framework, and Hub lockfiles all include the package and the interpreter’s dependency edge. The interpreter identity still includes all three numeric source files.

Play’s `flow-wasm` job runs the browser producer for `semio-os-flow-component` on `wasm32-unknown-unknown`. Traversing the selected root’s repository lockfile dependency edges reaches the Flow and WebGPU packages without reaching the core interpreter or numeric package. The shared profile edit can invalidate a configuration snapshot even when that job’s package dependency graph excludes the numeric crate.

The scoped native source scanner passed the numeric package at 3 files, 27,355 bytes, and 652 active lines. The interpreter core passed at 22 files, 260,691 bytes, and 3,720 lines. Its remaining 1,453 bytes are the only tight margin observed here. The interpreter runtime passed at 17 files, 226,870 bytes, and 1,989 lines. The normal limits are 32 files, 262,144 bytes, and 4,000 lines.

This audit used the current source and lockfiles. It did not start Cargo or change the release or source.

## Descriptor generation wuONgiG follow-up

The descriptor manifest directly selects the interpreter runtime. The runtime selects the interpreter core and imports, and the core selects numeric. The scoped native scanner evaluated their macOS arm64 source closures. The core roster included both pending instance files. The runtime roster included poll, snapshot operation and input records, and owned runtime.

| Owner | Files | Source bytes | Active lines | Remaining files | Remaining bytes | Remaining lines |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Interpreter core | 22 | 260,573 | 3,719 | 10 | 1,571 | 281 |
| Interpreter runtime | 17 | 227,885 | 2,012 | 15 | 34,259 | 1,988 |
| Interpreter numeric | 3 | 27,355 | 652 | 29 | 234,789 | 3,348 |
| Interpreter imports | 1 | 20,659 | 105 | 31 | 241,485 | 3,895 |

All four passed the 32-file, 262,144-byte and 4,000-active-line limits. Imports had the largest margin and the interpreter core had the smallest. Verification was source-only.
