# Result APP-CHANNEL-DECOMPOSE (prefix `acd`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `M` = `🧰️framework/🔨️modules/🧾️app-channel`. Tools `F/acd-tools/`, logs `F/🗑️generated/acd-*`, pre-edit snapshot `F/🗑️generated/acd-snap/app-channel.orig.ts` (sha1 71c71e4caadc4f0559f88b291f1bbc1f11fd36cb).

## 0. Verdict

**DONE.** `AppChannelClient` (688-line class, 712-line root) is now a **300-line composer plus 13 collaborators** (largest 124 lines), each in `M/<emoji>️<kebab>/🟦️.ts`. The public API is unchanged (same 39 members and constructor, every delegation returns the owner's promise unchanged), so no importer outside the module tree changed except the SDK wrapper's type re-export. The orphan JSDoc stubs are deleted (10 blocks in `🎫️command` and `🛠️admin`; `📄️event-page` had none left). Evidence: tsc identical (1 pre-existing diagnostic), os vitest per-test status identical plus 5 new passing tests, os bun oracles, renderer subset and descriptor-digest identical; the 4 behaviour tests also pass against the pre-decomposition class and 3 mutation controls on the new code are caught.

## 1. Found state

Fresh slice. `M/🟦️.ts` is the 712-line root left by CENSUS4-PKG-50 (`AppChannelClient`, 688 lines), modified against HEAD only by that split. Six children exist: `🏷️codec-primitives`, `🎛️command-codec`, `🎞️frame-codec`, `📋️report-decoders`, `🔢️request-sequence`, `🗃️archive-load-host`.

## 2. Baselines (before the first edit)

| Check | Command | Before |
| --- | --- | --- |
| tsc | `F/acd-tools/tsc.sh before` (26 roots: the module tree, its SDK wrapper and 9 importers/tests; `noEmit` over `💻️os/tsconfig.json`) | exit 2, **1 diagnostic**: `TS7016` for `jsdom` in `🎬️wgpu-browser-media` test (pre-existing, unrelated) |
| os vitest, whole `@semio-tech/framework-os` config | `F/acd-tools/vitest.sh os-before 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript ../../🧪️tests/🎚️config/🟦️.ts` | 35 files (29 passed, 6 failed), 722 tests: 608 passed, 114 failed (all pre-existing: actor-transport 107, directory-client-http 1, schema-vocabulary 2, execution-target-retry 1, surface-opens-kind 1, app-channel-codec 2). `🧪️tests/📞️app-channel-client`: 23 of 23 passed. Per-test status `acd-vt-os-before.set` |
| os bun channel oracles | `F/acd-tools/buntest.sh os-oracles-before` on `os/🧪️tests/🧪️{document-archive-load-host,attached-document-replacement,document-port-control-turn,folder-archive-persistence,folder-read-back}/🟦️.ts` | 10 tests across 5 files: 7 passed, 3 unhandled errors between tests (missing fixtures), exit 1 |
| renderer react subset | `F/acd-tools/vitest.sh rend-before 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript ../../🧪️tests/🎚️config/🟦️.ts window-scope activation-owner space-administration actor-backbone wgpu-extension-dispatch framework-plugin-runtime tool-run-trace` | 9 files, 88 tests, all passed |
| descriptor-digest bun test | `F/acd-tools/buntest.sh digest-before os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🟦️.ts` | 8 of 8 passed |
| source-text readers of app-channel | `git grep` over ts/tsx/json/mjs/cjs for the module path | none read the module text; path lists only (`⚙️config-graph.json` deny list, `🔣️taxonomy.json` wgpu-frame-worker lists, Nx `📋️project.json` globs `*/🟦️.ts`) |

## 3. The decomposition

`M` = `🧰️framework/🔨️modules/🧾️app-channel`. A collaborator only knows those above it in this order (no base class, no interface, no extension trait; the only back-reference is the closure that hands `LocalInteractionQuery` its `settled` callback, bound to `OutcomePump.finishDisposal`, like `Outbox` in the QuizSession split).

```
ChannelLifecycle   (flags only)
ReplyLedger(handle, request sequence, instance, ChannelLifecycle)     DocumentPackCache(ChannelLifecycle)     UnsolicitedFeed(instance, ChannelLifecycle)
LocalInteractionQuery(instance, ReplyLedger, ChannelLifecycle, settled)
OutcomePump(handle, instance, ChannelLifecycle, ReplyLedger, LocalInteractionQuery, DocumentPackCache, UnsolicitedFeed)
reply-faults (functions)
DocumentReads, ArchiveTransfer, InteractionCommands, MediaCommands, ReplicationCommands, TransactionCommands   (ReplyLedger, and cache / reply-faults where they need them)
AppChannelClient   (builds all of them, retire and dispose ordering, one-line delegations)
```

| Folder (lines) | Class | Owns | Old members (all from the 688-line class) |
| --- | --- | --- | --- |
| `🟦️.ts` (300) | `AppChannelClient` | the collaborators | constructor wiring, `retire`, `dispose` ordering, 37 one-line delegations |
| `🔌️channel-lifecycle` (36) | `ChannelLifecycle` | `retired`, `disposed` | the two flags (`retired`/`disposed` getters, `closed`, `retire()` returns whether it newly retired, `dispose()`) |
| `📮️reply-ledger` (124) | `ReplyLedger` | pending waiter FIFO, the request sequence | `pending`, `sequenceOwner`, `nextSeq`, `sendCommand` (now `send`), the receipt push of `sendLocalInteractionQuery` (`sendQuery`), the waiter loops of `pumpOutcomes` (`rejectOldest`, `rejectCommands`, `closeAll`, `settle`), the waiter loops of `retire` (`retireCommands`) and `finishLocalInteractionQuery` (`dropQueryReceipts`), `nextQuery` |
| `📦️document-pack-cache` (55) | `DocumentPackCache` | `cachedPack`, `cachedSpr` | `documentPack`, `captureDocumentFrames` (`capture`), the cache writes of `loadDocumentArchive`/`mergeDocumentArchive`/`readDocumentArchive`/`retire`/`pumpOutcomes` (`remember`, `forget`) |
| `📣️unsolicited-feed` (100) | `UnsolicitedFeed` | completion and progress subscribers, `lastEphemeral` | `ephemeral`, `onOperationCompleted`, `onOperationProgress`, `publishOperationCompletion`, `publishOperationProgress`, type `AppChannelOperationProgressV1` |
| `🏠️local-interaction-query` (121) | `LocalInteractionQuery` | the one running `localQuery` | `readLocalInteractionPages`, `sendLocalInteractionQuery`, `cancelLocalInteractionQuery`, `finishLocalInteractionQuery`, `receiveLocalInteractionQuery`, the local-query part of `retire` |
| `🔁️outcome-pump` (85) | `OutcomePump` | the outcome iterator | `pumpOutcomes` (`run`), `finishDisposal`, the iterator creation of the constructor |
| `🧯️reply-faults` (18) | `channelLabel`, `throwOnFault` | nothing | the repeated find-`Error`-frame-and-throw lines of seven readers and the `AppChannelClient.<method>(<appId>)` label |
| `🪪️document-reads` (90) | `DocumentReads` | nothing | `readDocument`, `readDocumentIdentity`, `readChildHeads`, `readHistory`, `loadWindowConfig`, `readWindowConfigs` |
| `🧳️archive-transfer` (119) | `ArchiveTransfer` | nothing | `loadDocumentArchive` (`load`), `mergeDocumentArchive` (`merge`), `driveDocumentArchive` (`drive`), `readDocumentArchive` (`read`) |
| `🎚️interaction-commands` (82) | `InteractionCommands` | nothing | `command`, `configure`, `contextMenu`, `applyEnvelopes`, `openArtifact`, `setDefaultApp`, `clearDefaultApp` |
| `🎬️media-commands` (49) | `MediaCommands` | nothing | `mediaOut`, `mediaIn`, `submitMediaExport`, `pollMediaExport`, `cancelMediaExport`, `takeMediaExportChunk` |
| `⚖️replication-commands` (52) | `ReplicationCommands` | nothing | `setMergePolicy`, `resolveConflict`, `readConflicts`, `pushPresence` (regions `Merge`, `Presence` move with them and stay on the delegations) |
| `🎫️transaction-commands` (65) | `TransactionCommands` | nothing | `transactionPrepareOwner` (`prepareOwner`), `transactionPreparePlanned` (`preparePlanned`), `commit`, `rollback`, `undo`, `redo` (region `Transaction`) |

Existing children are untouched: `🏷️codec-primitives` 56, `🎛️command-codec` 373, `🎞️frame-codec` 369, `📋️report-decoders` 76, `🔢️request-sequence` 107, `🗃️archive-load-host` 172.

Decisions:
- **Public API unchanged.** The composer's delegations are non-`async` and return the owner's promise as is; the owners keep the original `async` wrappers, so there is exactly one async hop per command as before and the microtask order of replies is the same.
- **`retire()` and `dispose()` stay in the composer** because they are the only orchestration across collaborators; their statement order is the original (`retire`: flag, clear completion subscribers, drop the pack, reject queued commands, cancel the running query; `dispose`: `retire`, flag, `finishDisposal`).
- **Dead `actor` removed** (coordinator follow-up, section 9): the constructor is now `(handle, sequenceOwner, instanceId, appId)`.
- **`AppChannelOperationProgressV1`** moved to its owner `📣️unsolicited-feed`; no re-export from the root. The SDK wrapper `📦️packages/🟦️typescript/🧾️app-channel/🟦️.ts` re-exports it from there; `@semio-tech/framework-os` already re-exports it through the wrapper. New exported helper types (`AppChannelEphemeralV1`, `EphemeralFrame`) are internal to the module.
- **`reply-faults`** dedups the seven copies of "find the `Error` frame, throw `<label>: <fault text>`"; every error message is byte-identical to the original (`document identity: ...` and `AppChannelClient.<method>(<appId>): ...`).

## 4. Orphan JSDoc stubs

The ten blocks were docs of functions that the directory io/text extraction had already moved. Verified by reading the live owner `🧰️framework/🔨️modules/📇️directory/🚪️io/📝️text/🟦️.ts`: it holds the same ten doc lines directly above `parseDirectoryEventV1` (line 32), `parseDirectoryEventPageV1` (103), the canonical command decoders (132, 198), `directoryCommandSha256` (204), `sealDirectoryCommandRequestV1` (208), `directoryCommandRequestJson` (212), `parseDirectoryCommandRequestV1` (219), `sealDirectoryCommandReceiptV1` (228) and `parseDirectoryCommandReceiptV1` (234); no definition of those functions exists in any schema child, and the live importers (`@semio-tech/framework/directory/io/text`) use the io/text ones. Git history cannot say more (the repo carries one squashed commit `b84bf526c7b` for this file; `git log -S` finds the strings only there; the HEAD `🧬️schema/🟦️.ts` already had the same stubs). Deleted: `🎫️command` 2 + 2 blocks, `🛠️admin` 6 blocks. `📄️event-page` contained none any more (it keeps its two real docstrings), so nothing to delete there. Check: a TypeScript-AST scan of every schema child for nodes carrying more than one JSDoc block finds none now (`orphans2.cjs` was run before: it listed exactly those 10).

## 5. Tests

New (in the existing `os/🧪️tests/📞️app-channel-client/🟦️.ts`, +105 lines; no new entry point, so `launch.json`, the launch seed, the os vitest include and the taxonomy `members-of-tests` need no change): 5 tests with a hand-answered handle whose outcome iterator counts `return()` calls.
1. surface pin: all 39 public members are functions and the prototype carries nothing else (the composer owns no private logic).
2. `retire()` fails the queued command with `app-channel.disposed`, refuses new commands and queries, silences completion subscribers but not progress subscribers, drops the cached pack, is idempotent, does not end the subscription; `dispose()` ends it exactly once; no progress after dispose.
3. an ended outcome stream rejects waiters (`app-channel.closed`) and the running local interaction query (`local-interaction.channel-closed`) and forgets the pack.
4. an error outcome fails only the oldest waiter; a sibling instance's frames and error settle nobody.
5. an undecodable frame fails every waiting command and the channel stays usable.

Tests 2 to 5 were also run against the pre-decomposition class (snapshot `F/acd-tools/orig/🟦️.ts`, throwaway config `F/acd-tools/orig/vitest.config.ts`, test copy `F/acd-tools/orig/wiring-orig.test.ts`): 4 of 4 passed, so they pin behaviour, not the new structure (test 1 is structural by design). Negative controls on the new code, each restored byte for byte (sha1 checked): `UnsolicitedFeed.clearCompletions` no-op, test 2 fails; `ReplyLedger.rejectOldest` rejecting all waiters, test 4 fails; `OutcomePump.finishDisposal` never ending the iterator, test 2 fails (logs `acd-vt-mut1/2/3.*`).

## 6. Verification (every command was run; results are the real ones)

| Check | Command | After |
| --- | --- | --- |
| tsc, 53 roots (module tree, SDK wrapper, directory schema tree, importers) | `F/acd-tools/tsc.sh after1`, `after2` | identical to the baseline: the one pre-existing `TS7016` (`jsdom`), nothing else (`acd-tsc-after2.txt`) |
| tsc liveness | `tsc --noEmit -p acd-unused.json` (`noUnusedLocals` and `noUnusedParameters` on the same roots) | 485 diagnostics repo-wide, so the check reports; in my files only the pre-existing unused `actor` field and `editedArtifactFrontier` (`🎫️command`, a private helper unused since before PKG-50); no generated import is unused |
| os vitest, whole config | `vitest.sh os-after1` (before the new tests), `os-after2` (with them) | after1: 722 tests, per-test `.set` byte-identical to the baseline; after2: 727 tests, 613 passed, the same 114 failures, diff = exactly the 5 new passing tests |
| os bun channel oracles | `buntest.sh os-oracles-after1`, `after2` | per-test set identical (7 passed, the same 3 unhandled missing-fixture errors); the `expect()` count jumps between runs (815 to 831) with identical code, so it is not a signal |
| renderer react subset | `vitest.sh rend-after1`, `rend-after2` | 9 files, 88 tests, set identical |
| descriptor-digest bun test | `buntest.sh digest-after1`, `after2` | 8 of 8, set identical |
| app-channel-client suite alone | `vitest.sh wiring-after1 ... app-channel-client` | 28 of 28 (23 existing + 5 new) |
| wgpu frame-worker browser bundle | `bun F/acd-tools/wgpu-observed.ts` (Bun.build of the three profile entries) | `frame-worker true`, `browser-boot true`, `renderer-boot true`: the bundle resolves and includes the 13 new files (observed set: `acd-wgpu-observed.json`) |
| Import cycles | `node F/acd-tools/cycles.cjs` over the 20 module files | 0 strongly connected components of value imports |
| Taxonomy | `bun F/acd-tools/taxcheck.ts` (repo `matchDirectoryKind` over the ancestor chain) | 19 of 19 resolved as `crate-module-children`, 0 unresolved, 0 ambiguous |
| Static | script over all 20 files | first docstring starts with the folder emoji in all; docstring emojis unique per new file; `//#region`/`//#endregion` balanced in each; no comment lines inside definitions; no `[DEBUG]`; longest path in the module 120 bytes, in my tools 202 |
| Emoji check | `acd-tools/emojicheck.py` | every folder emoji already occurs as a taxonomy member prefix (the new ones: 🔌 41, 📮 5, 📦 38, 📣 12, 🏠 26, 🔁 31, 🧯 3, 🪪 41, 🧳 5, 🎚 26, 🎬 26, ⚖ 29, 🎫 7) |

Path lists updated: `🔣️taxonomy.json` `wgpu-frame-worker` `packageGeneration.browserProfile.sourceModulePaths` 240 to 253 and `inputPatterns` 257 to 270 (13 new paths each, byte-sorted, only those two lists differ structurally); `⚙️config-graph.json` deny list 29 to 42 (all paths exist). The Nx `📋️project.json` input lists use the globs `*/🟦️.ts` and already cover the children. No source-text reader reads the module text (`git grep`), so no `readModuleTreeSource` change.

## 7. Files

Created (13): `M/{🔌️channel-lifecycle,📮️reply-ledger,📦️document-pack-cache,📣️unsolicited-feed,🏠️local-interaction-query,🔁️outcome-pump,🧯️reply-faults,🪪️document-reads,🧳️archive-transfer,🎚️interaction-commands,🎬️media-commands,⚖️replication-commands,🎫️transaction-commands}/🟦️.ts`.
Rewritten: `M/🟦️.ts` (712 to 300).
Edited: `🧰️framework/📦️packages/🟦️typescript/🧾️app-channel/🟦️.ts` (one `export type` line re-pointed), `🧰️framework/🔨️modules/📇️directory/🧬️schema/{🎫️command,🛠️admin}/🟦️.ts` (stubs removed), the os `🧪️tests/📞️app-channel-client/🟦️.ts` (+5 tests), `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/⚙️config-graph.json`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.
Tools and logs: `F/acd-tools/`, `F/🗑️generated/acd-*` (snapshots in `acd-snap/`).

## 8. Residual risks and hand-offs

- `os-dev` test `🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts` cannot run: it reads `../../🧫️fixtures/⚙️config-graph.json` relative to its own folder, a path that does not exist (pre-existing; the fixture lives under `🧪️tests/🧫️fixtures`). The deny entries were therefore checked for existence only.
- `bun test` over the throwaway original-snapshot test segfaulted (Bun 1.3.14 panic, "Segmentation fault at address 0x8020"); the same file ran fine under vitest, which is how the original-class run above was done.
- Construction order in the composer matters in one place: `OutcomePump.start()` is the last statement of the constructor (the original started the loop at the end of its constructor too), and `LocalInteractionQuery`'s `settled` closure reads `this.pump` lazily.
- Prettier is not available in this checkout; formatting follows the neighbouring files by hand (printWidth 250 style).

## 9. Coordinator follow-up: no dead API

- **`actor` removed.** `AppChannelClient`'s constructor is `(handle, sequenceOwner, instanceId, appId)`; the never-read parameter and field are gone. Callers updated: `PluginRuntime/🔗️channel-adapter` (passed `currentPluginRuntimeActor`) and the wgpu `🐚️plugin-bridge` (passed `"local"`). Every other `new AppChannelClient(` (tests, the browser-media and extension-dispatch suites) already used four arguments; no story constructs one; tsc over the roots that contain both callers and all tests confirms no leftover 5-argument call. The module-level `currentPluginRuntimeActor`/`setPluginRuntimeActor` stay because `module-loader` (`{ kind: "open", input: { appId, actor: currentPluginRuntimeActor ... } }`) and the hub home-browser check still read and set them; I only corrected their two doc sentences that claimed every `AppChannelClient` is stamped with it.
- **`editedArtifactFrontier` removed** from `🧬️schema/🎫️command` together with its `//#region 🌊️EditedArtifactFrontier` pair and the then-unused `editedArtifactFrontierIsValid`/`EditedArtifactFrontierV1` import. Verified first: the function was module-private (`function`, not exported), `git grep` of the tracked tree finds the identifier only in two other files that import `isEditedArtifactFrontierV1` under the same local alias for their own use (`📝️text`, the gis `🧬️schema`; neither references this function), and the `🎫️command` file itself (untracked) had only the definition. No `?raw` or text reader names it.
- **Re-run** (`acd-*-after3`): tsc over the 53 roots: the same single pre-existing `jsdom` TS7016, nothing else; `noUnusedLocals`/`noUnusedParameters` run: nothing reported in the app-channel tree or `🎫️command` any more (the remaining plugin-bridge findings, e.g. `actorId`, `extra`, `leftoverFriendly`, are unrelated and pre-existing); os vitest: 727 tests, 613 passed, 114 failed, per-test set identical to `os-after2`; wiring suite (`app-channel-client`): 28 of 28; renderer react subset: 9 files, 88 tests, set identical to the baseline.
