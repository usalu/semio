# Bounded Warm Journal Projection Checkpoint

The actual shared physical journal reader now reuses an exact-byte immutable projection before replay. Real Source counters across all nine complete neutral histories show **25 cold event parses and 0 warm event parses**, with matching independent SQLite state. **All eight final commands pass.** The full build/cache goal remains open; Source DATA tests and source wiring do not prove actual Native throughput or physical quota.

## Production Change

The former path-keyed map was consulted after readCleanupJournal had already replayed every event. It did not avoid replay CPU. The physical reader now delegates to one private bounded projection owner after stat, bounded descriptor read, re-stat identity/time checks, fatal UTF8 decoding and complete-line observation. Actual backing, operation, journal exclusion, control checks, persistence and cleanup paths remain. There is one actual owner and one physical projection call, with no old post-replay get/set or second cache.

Projection entries use exact journal bytes, deeply immutable State and the maximum charged bytes reached by every historical prefix. A warm entry is reusable only when that peak fits the current capacity. Otherwise canonical replay runs and preserves its refusal. This prevents accepting a small final state whose earlier reservation or pre-rebudget charge exceeded capacity. Changed bytes replay once before later reuse; malformed/cancelled/partial projections never install entries.

The owner retains at most the existing eight entries, each input bounded by the existing 1 MiB journal limit. Request DATA is owned from exact primitive fields before callbacks; proxies, getters, extras, wrong version, invalid capacities and nontext are refused without their effects. Byte count is enforced independently from character count. A private loaded schema copy prevents imported JSON alias changes from weakening the loaded event/request semantics or entry bound. No disk cache, allocation provider, physical grant, new runtime library, migration, byte/entry frontier or output cap is introduced.

A genuine reentrant check exposed a ninth-entry bug in the first memo implementation. A warm hit could be evicted while its check callback filled eight other entries, then be added back without eviction. The neutral regression observed warm0 where eviction/cold1 was required. One retention operation now evicts before insertion on both cold and warm paths. It handles callback reentry and does not temporarily install a ninth cache entry. Callback DATA tests do not resume any failed Native Original.

## Test-First Evidence

The request schema and closed language-neutral corpus precede implementation. Initial red 2.419s: all six prior cases pass, all seven new memo/wiring cases fail for the missing implementation. First implementation passes both mode test/type pairs. Source coverage then broadens to the existing framework event/checkpoint conformance laws while preserving all prior cases. Actual runtime summaries establish eight other framework cases filtered, including system image capture; the initial estimate of four based on literal registration sites was corrected. No Native/OS image test is selected.

Reentrant red 1.708s then demonstrates one real defect: expected evicted/cold1, received cached/warm0; the other cases pass. The corrected retention implementation precedes the final full matrix. Raw red/green outputs remain; no failed Original is repaired or retried unchanged.

Final runtime scope includes all 14 caller/memo cases plus the two framework laws: 16 tests / 472 assertions / two files per mode, eight framework cases filtered. SQLite independently models complete event histories and exact snapshot JSON. Actual JSON.parse calls are counted transparently around the real projector, not a substitute producer. Tests cover exact warm reuse, distinct bytes and LRU eviction, retirement/rebudget historical peaks, cold-equivalent capacity refusal, cancellation/partial and malformed failure, immutable loaded schema, inert nested proxies, and retained physical-before-projection wiring.

The complete byte boundary is exercised: 1,048,576 bytes parse successfully once and then reuse with zero parses; a multibyte-owner change creates 1,048,578 bytes with the same character length and is rejected before parsing. No smaller frontier is used. Returned State remains frozen/detached and never acquires Native port/backing/control authority.

## Complete Final Matrix

| Gate | Prototype Seconds | Release Seconds | Selected Result |
| --- | --- | --- | --- |
| test | 1.806 | 2.408 | 16 / 472 / 2 files; 8 framework cases filtered |
| types | 28.669 | 26.678 | Complete prior compiler-caller roots plus full framework storage test file |
| source | 84.068 | 72.657 | 157 / 1,643 / 11 files |
| artifact | 52.434 | 44.598 | 73 / 713 / 6 files; 27 Native cases filtered |

All eight are terminal status 0. Each of the four broad Source/artifact test invocations also runs its mandatory adapter cohort (10 tests / 215 assertions). The existing caller flag retains its earlier adapter policy. Final dedicated jobs use at most two outer lanes; broad complete invocations run sequentially outside while keeping each internal list, order and original controls. Same deadlines/reserves/counts/frontiers/output caps. Earlier Source/artifact deadline failures remain historical. Different scheduling/load prevents a causal elapsed speed ratio or universal budget-reliability claim. The measured improvement is elimination of event replay on an exact warm byte match; every physical read/check still executes.

## Current Source Qualification

The prospective post-reentrant-fix window observes 1,821 inherited source/config/schema/test paths. All 114 owned identities and all six files authored in this slice remain stable. Maximum observed absolute source path is 191 UTF-16 units. Three other changes occur:

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🗂️system/🟦️.ts`: `0fae5051050cf6ef0373532482139a829f5eb12bd070a92b07ea03aa25b17dc6` → `5fbd117bdaea536d200fd4337b42843c2951324294e87b75589b0db7e20f8dff` (29043 bytes).
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🗂️system/🧬️schema.json`: `4cde8825a562d303bae2e7299e13a1623b60f5c52d84d900de91f0252b62e379` → `50e4da62706d9ccbdf9fa169c2aff8601424a2af32ba10f6d43fb236935e1090` (4764 bytes).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`: `a5fa649657046fefbfb86d4283506717729517bc81a9827f10c523072e8d23bc` → `b6119df62e15b5ea3662e2ef0ed6833cb3d5bf74a168d5bed862d58359d6cf12` (140525 bytes).

The Windows system runtime/schema and dependency-storage runtime are substantive possible execution/input changes. They are not assumed harmless, unreachable or nonexecuting on this host. The three historical missing publication/SDK paths remain recorded in source-after-verification.json; they are not attributed or restored. This inherited census does not discover every new path, determine every mid-execution loaded revision, measure OS/image/repository footprint, or establish a complete current execution closure. All gate claims retain their recorded source window. No peer edits are reverted.

## Files Authored or Updated

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🗃️compiler-state/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🗃️compiler-state/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🗃️compiler-state/🧬️schema/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts`

The existing permanent ticket Script is extended to include both neutral framework laws and their complete strict file. All earlier caller cases remain neutral/selected. The same four registered prototype/release test/types launch commands are verified; launch.json is unchanged in this slice. No new permanent Script, AGENTS edit, modifying Git command or worktree. Existing three workers are terminal after application network permission revocation and are not restarted or bypassed. Repository ticket/goal MCP tools remain absent; the same active ticket and all necessary generated evidence are retained.

## Full Goal Remains Open

The pure State/Event/snapshot/projection layer still resides in the physical Native storage module. The requested architecture split must move the complete schema-owned data engine into a pure layer without a Root→memo→Root cycle or divergent event engines, leaving Native I/O/custody in orchestration. This memo does not complete that split or optimize canonical cold-writer replay.

Actual current Native toolchain/compiler/Nx behavior, precise per-crate/dependent invalidation, no unaffected recompilation, simultaneous shared warm builds, full CPU scaling, prototype/release runtime/size, future crate growth and native/devcontainer platform workflows remain unqualified. Sealed code/receipt and task option identity coupling still require integration; a command recipe alone is not a sufficient cache key.

The requested physical cache/maximum repository ceiling, including in-flight/journal/recovery/staging/retained/output metadata, fixed backing pressure/ENOSPC/close/reopen/history, full 65,536/65,537 physical inventory and all-platform backing behavior remain open. Native/default SDK/publication/compiler/cache-image/OS-inventory holds persist. No pure DATA memo, schema/logical bound or ordinary Source fixture proves physical enforcement. Goal and ticket remain active.

Authoritative generated evidence: rust-journal-memo-red-prototype.json/log, first-verification.json/logs, expanded-verification.json/logs, reentrant-red-prototype.json/log, final-verification.json/logs, latest-verification.json, runtime-counters.json, current-source-audit.json, launch-verification.json, source-before-reentrant-final-verification.json and source-after-verification.json (prefix rust-journal-memo where omitted).
