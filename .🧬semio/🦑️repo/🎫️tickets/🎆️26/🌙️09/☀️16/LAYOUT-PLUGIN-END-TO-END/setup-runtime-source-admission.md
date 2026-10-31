# Setup Runtime Source Admission Preflight

Read-only static API inspection on 2026-10-29. No runtime capture, Original issuance, snapshot construction, effects, builds, native jobs, publication, cache operations, or source edits were performed.

## Actual Entry and Collector

The runtime declaration is `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/📜️script.ts:9`: nativeDirectoryRuntimeEntryV1 declares its own script module. Its runCapturedBuildSourceV1 requires the authenticated runtime-entry proof before executing the directory action (line 11).

The actual setup bridge is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts:10–14`. withRepositoryRuntimeEntriesV1 requires a resource control bound to the original invocation, resolves that declaration, and calls captureBuildSourceClosureV1 with the default closure scope. It then serializes the resulting snapshot and prepares a checksum-bound stdin loader/provider. This is the actual runtime admission path, not a pure inventory command.

Canonical collector root: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts`.

- captureBuildSourceClosureV1 at line 55 requires an issued ScriptInvocation immediately. It obtains the exact parent-admitted resolution and physical-source maps from import.meta.semioCapturedSourceResolutions and import.meta.semioCapturedPhysicalSources. sourceResolutionMapV1 validates matching complete roots, paths, hashes, imports, and canonical order.
- The collector follows that admitted declaring-file import map, module-owned schemas, and admitted quoted source/JSON literals. This cannot be reproduced faithfully with a generic disk import crawl or directory/file count.
- Its source reads recheck the issued invocation, exact hashes, no-follow ancestry and descriptors. During collection it publishes progress and yields through the Original control; afterward it constructs a frozen snapshot, validates it and requires admitted parent source selection again.
- The only exposed parsing helper staticImportSpecifiersV1 does not inventory the default closure or supply the parent-admitted resolution map. Snapshot serialization/identity helpers require a supplied captured closure; they are not inventory APIs.

## Measurement Result and Exact Limitation

The inspected canonical module exposes no supported pure/read-only inventory API that returns the actual selected closure without an issued Original and admitted evaluated-parent source maps. Calling captureBuildSourceClosureV1 would itself construct the runtime source snapshot and exercise Original progress/control. Therefore actual selected file count, source bytes, and active lines were not obtained under this read-only authorization. No weaker collector, fake ScriptInvocation, synthetic admitted map, or snapshot proof was substituted.

Correction: the 32-file / 262144-byte / 4000-active-line Rust Source boundary is not passed or enforced in this directory TypeScript runtime-entry capture path. It is not an applicable gate here, and no comparison against it is required. The canonical runtime closure schema itself currently declares a separate collector safety ceiling of 2048 files, 16777216 source bytes, 4194304 bytes per file and 25165824 snapshot bytes in `🧬️schema/🔣️.json` under that closure root. It has no active-line counting policy. These are the applicable canonical collector limits; they are not substituted values for a narrower Rust gate.

The separate Cargo source admission API (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts`) handles Rust unit entries/cfg under resource control and storage reservation. It is not the TypeScript runtime-entry closure collector and cannot provide the missing runtime inventory.

A genuine authorized setup Script with its actual admitted parent can establish this selection. This preflight intentionally stops before that runtime admission and makes no runtime-capture or publication claim. Active native run 84839 was left untouched.


## Applicable Runtime Policy — Clarified

No runtime-entry options override the closure policy: nativeDirectoryRuntimeEntryV1 declares only version, moduleURL/optional physicalSourceURL, and sibling specifier; withRepositoryRuntimeEntriesV1 calls captureBuildSourceClosureV1(root,entry,original) with no fourth argument. Thus default `{kind:closure}` applies. This path never calls Cargo Source admission or supplies maxSourceFiles/maxSourceBytes/maxActiveLines.

Applicable limits in canonical closure schema (`🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json`, definitions.Policy.const):

| Boundary | Limit |
| --- | ---: |
| Captured file count | 2048 |
| Individual source file bytes | 4194304 |
| Aggregate source bytes | 16777216 |
| Serialized closure snapshot bytes | 25165824 |
| Source read chunk bytes | 65536 |
| Physical source absolute path units | 256, enforced in closure pathIn |

The runtime transport imposes a further, tighter snapshot cap. `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🚪️entry/🧬️schema/🔣️.json`, definitions.Policy.const, sets maximumSnapshotBytes=4194304, maximumLoaderBytes=32768, maximumURLUnits=4096, maximumPathUnits=256. Its Command schema requires exactly four arguments, each at most 32768 string units, program at most 256 units, and only BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 in the environment.

`readCapturedRuntimeEntryCommandV1` in the adjacent `🟦️.ts:31` validates the publication input both by string length and UTF8 byte length against 4194304, requires lossless UTF8 and matching implementation digest, and caps the loader UTF8 bytes at 32768 with its matching digest. Input is deliberately replaced by an empty string for the ordinary process JSON-envelope reader, then restored and validated under this runtime schema; the unrelated general JSON input ceiling is not applied to snapshot text. The actual command uses process.execPath, `--eval`, loader, `--`, and publication hash. Its bridge calls this constructor before installing the provider, and captureRuntimeEntryV1 calls it again when retaining the command.

The stdin loader generated by closure `🍞️bun/🟦️.ts:101,110` separately checks encoded snapshot bytes against the closure 25165824 cap, file count against 2048 and admitted image file/total bytes against 4194304/16777216. Because the runtime constructor accepts at most 4194304 snapshot bytes first, the effective directory runtime publication transport ceiling is 4194304 UTF8 bytes. Base64, paths, hashes and import maps are part of those serialized bytes, so this does not translate to an equivalent raw-source-byte allowance.

The actual selected closure remains unmeasured due to the issued-Original/admitted-parent limitation stated above. Static inspection establishes which policies apply and that production enforces them; it does not establish that the current selected closure fits. There is no active-line cap in this runtime path. The earlier report wording importing Rust caps is superseded by this clarification.
