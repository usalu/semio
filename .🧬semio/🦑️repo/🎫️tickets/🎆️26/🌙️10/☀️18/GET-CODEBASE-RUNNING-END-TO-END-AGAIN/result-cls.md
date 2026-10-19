# Result cls: `ShardClient` and `DocumentBrowserActorReservation` redesigned to at most 600 lines per file

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo, no `bun repo`. `C` = this ticket folder. Tools: `C/cls-tools/`. Snapshots of the originals: `C/cls-tools/orig/` (`shard-client.ts`, `actor-reservation.ts`, `activation.ts`). Scratch outside the repo: `/Users/ueli/.cache/cls/` (generator output, `@xtuc/long` link for the test harness).

## 0. Verdict

| class | file | before | leaf after | largest new file | status |
| --- | --- | ---: | ---: | ---: | --- |
| `ShardClient` (ES `#private` fields) | `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` | 1,655 | 219 | 379 (`💾️resident-bootstrap`) | DONE |
| `DocumentBrowserActorReservation` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🧷️actor-reservation/🟦️.ts` | 1,162 | 95 | 173 (`🩹️ui-patch-reconciliation`) | DONE |
| `wgpu-frame-worker` taxonomy lists | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | 355 modules / 278 inputs | 382 / 305 | | DONE |

Public API of both classes is unchanged (same methods, same signatures, same identity semantics: `matchesResidentLedger`/`matchesActivation` brand checks on the single remaining ES private field `#residentLedger`, the facade is the shell passed to the resident ledger, the captured activation and the captured return).

## 1. Design (both classes)

Facade + hub + collaborators + one private state object, in the existing domain taxonomy tree (emoji folder, `🟦️.ts` leaf, relative `./` import from the leaf so `readModuleTreeSource(<leaf>)` reaches every child).

- The leaf keeps the facade class and the `Hub` composition root (`ShardClientHub`, `ReservationHub`); it builds the state object, builds the hub, and every public method is a one-line delegation. A collaborator receives the hub, copies `hub.state`/`hub.session`, and reaches siblings only through `this.hub.<collaborator>` at call time (never in a constructor).
- Many `#fields`/`private` fields became one state class (`ShardClientState`, `ReservationSession`, plus `ResidentAdmissionState` for the 18 admission phase fields, same names and order as before because the admission census prices them).
- Moved method bodies are textually identical modulo the indirection (`this.x` to `this.state.x`, `this.hub.<owner>.x`, bare `this` to `this.hub.shell`).

`ShardClient` (leaf 219): `🗃️state` 70, `💾️resident-bootstrap` 379, `📬️request-transport` 173 (spawn, inbound dispatch, pending ledger, `send`), `🧭️routing` 99, `🪪️exact-activation` 94, `🚪️instance-lifecycle` 172, `↩️captured-return` 231, `📥️ui-acknowledgement` 113, `🌉️host-effect-bridge` 130, `⏱️watchdog` 141. Facade keeps `turn`, `envelope`, `grant`, jobs, `checkpoint`, `restore`, `codec` and the thin `send`/`nextRequestId`/`requireShard` (the codec tests call `ShardClient.prototype.codec` on a fake `this` with exactly those four members).

`DocumentBrowserActorReservation` (leaf 95): `🗃️state` 65, `🛤️poll-lanes` 73, `📤️turn-effects` 138, `🩹️ui-patch-reconciliation` 173, `📬️document-backbone` 153, `🧊️cold-pair-transfer` 116, `🚀️guest-activation` 123, `♻️guest-retirement` 105 (retire + `close`), `🖼️surface-rendering` 159, `🎯️action-dispatch` 112, `🔌️link-suspension` 42.

Folder names: RGI emoji plus U+FE0F, unique first grapheme per sibling set, longest path 134 characters.

## 2. Pre-existing bug fixed on the way

`🔐️activation` exported `let submitCapturedReturn`/`reserveCapturedResponse`, and the old `ShardClient` static block assigned those imported bindings (the c6tsfw split moved the `let`s into another module). ES forbids it: vite-node and bun refused to load the module (`Cannot set property submitCapturedReturn of [object Module]`), so every ShardClient suite was dead. Now `🔐️activation` exports `bindCapturedReturnDispatch(submit, reserve)` and the facade static block calls it.

## 3. Verification (exact commands and outcomes)

Harness: `C/cls-tools/run-vitest.sh <test-dir> <json-out> [include]` runs `node_modules/.bin/vitest run --config C/cls-tools/vitest.config.ts --root <test-dir>` with a narrow root (a repo-root run scans the whole repo and dies of OOM). The config aliases the three packages that `node_modules` lacks (`immer`, `@webassemblyjs/leb128`, `@xtuc/long`, taken from `.cache/b/d/m/bun-cache`) and sets `SEMIO_TEST_ARTIFACT_DIR`. `C/cls-tools/summarize.py` flattens the JSON report.

| suite | before (with only the section 2 fix) | after |
| --- | --- | --- |
| `🔌️plugin/🧪️tests/🎭️actor-transport` (179 tests: settlement, response framing, retryable lifecycle, cancel-job reply, codec reply) | 174 pass, 5 fail | 179 pass, 0 fail |
| `💻️os/🧪️tests/📚️worker-law-suites` (100 tests, reservation, action dispatch, catch-up, recovery) | 99 pass, 1 fail (`path-to-regexp` missing) | 99 pass, 1 fail (same test, same cause) |
| `🎭️actor/🧬️codec/🧪️tests` (`bun test`, 9 tests, fake-`this` calls of `ShardClient.prototype.codec`) | not run before | 9 pass |
| `🎠️kernel/🧪️tests` package-assets and createturnoutcomebroadcast (real `ShardClient`) | not run before | 57 pass |
| renderer `turn-patch-batch`, `activation-owner`, `wgpu-extension-dispatch` | not run before | 33 pass (`inbound-request` fails: `@xyflow/react` missing, unrelated) |
| actor in-source suites (`📮️shard-client` command-ingress pages and the other `includeSource` files) | not run before | all pass except 7 `lifetime` tests that fail on missing `lodash-es` (unrelated) |

The 5 before-failures were stale source-law tests that read the monolith leaf after c6tsfw had moved declarations into `🔐️activation`; they pass now (see section 4).

Static oracles:
- `node C/cls-tools/verify-lossless.mjs <orig> <class> <dir>`: every member of the original class reappears, textually equal modulo the indirection, in exactly one class of the new files. `ShardClient` 81 of 81 identical, `DocumentBrowserActorReservation` 43 of 43 identical, 0 mismatches.
- `tsc --noEmit --noUnusedLocals` (`C/cls-tools/tsconfig.{shard,res,users}.json`): 0 errors in any shard-client or reservation file, apart from the pre-existing `CapturedShardActivation is declared but never used` (side-effect class). The files that consume `ShardClient` (kernel, shard-runtime, PluginRuntime, wgpu bridge, ui resident) report no member error.
- `bun C/cls-tools/wgpu-closure.ts` (the same `Bun.build` `onLoad` oracle as `testBrowserInputClosure`): observed 382 modules, taxonomy 382, missing 0, unused 0, clean build of all three wgpu entries.
- A mutation of one comparison in `🎯️action-dispatch` (`<=` to `<`) was NOT caught by the worker-law suites: the textual lossless check, not the behavioural suites, is the evidence for that predicate.

## 4. Tests and fixtures changed (no behaviour change, only where they looked)

- Private-field readers now go through `shardClientInternals(client)` (new export of `🧪️shardclient-reserved-response-settlement/🧰️settlement-test-kit`, returns `{ hub, state }`): `shards`, `pending`, `instanceLifecycles` in `🧾️reserved-response-settlement`, `🔀️captured-return-routing`, `💓️heartbeat-and-liveness-watchdogs`, `🔓️captured-return-admission`; the `recomputeOldestPending` hook now patches `hub.transport`.
- Source-law readers use `readModuleTreeSource(<leaf>)`: `💾️resident-composition` (activation class, and the `ResidentAdmissionState` field census), `🚀️worker-bootstrap` (`ResidentAdmissionState`), `🔓️captured-return-admission` (2 places), `🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies` (inbox census, `OutboundMessage`/`InboundMessage`, handler order now checked on `ShardRequestTransport.handleMessage`), `⏱️retryable-lifecycle-deadline` (anchors `async sendInstanceLifecycle(` to `acceptInstanceLifecycleResult(`).
- Inbox census fixture/schema `🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox`: group `client` is now 3 declarations (`ShardClient` 2 fields, `ShardClientState` 29, `ResidentAdmissionState` 18), `minimumShells.client` 976 bytes, 3 slots, 3 owners (was 816, 1, 1); schema `layouts` min/max 8 to 10.
- `🌎️hub/🧪️tests/🔗️execution-target-lease-checks`: the browser lease region is read with `readModuleTreeSource` (reservation children).
- `wgpu/📦️packages/🟦️typescript/📋️project.json` `frameWorkerSources`: added `shard-client/*/🟦️.ts` and `Diagram/📐️layout/*/🟦️.ts` (the `ui/scene` wildcard already existed).

## 5. Taxonomy (`wgpu-frame-worker`)

`generatorContracts.wgpu-frame-worker.packageGeneration.browserProfile.sourceModulePaths` and `inputPatterns`: inserted 27 modules each in byte order (JSON diff: 54 added lines, 0 removed): 16 `shard-client` children (the 6 value-imported ones from c6tsfw and my 10), 6 `ui/scene` children, 5 `Diagram/layout` children. Source of truth was the independent bundler closure, so the list is exact (382). Backup of the previous file: `/Users/ueli/.cache/cls/taxonomy.before.json`.

## 6. Open risks

1. Not run: the full `testWgpuGeneratorOwnership`/`testBrowserInputClosure` bun entry (needs the library runner); only its closure oracle ran. Its last loop (every module exact in `frameWorkerSources` or in the browser-build closure) was already failing for about 200 modules before this change (kernel, value, async children); not touched.
2. `taxonomy.json` is shared and 1.2 MB: if a peer pins a hash or a frozen-evidence index of the two arrays, re-pin. The frozen-evidence code derives pointers from the path value, not a stored index.
3. `dev` `browser-graph.json` `maxModules` 1600 and `watch-policy.json` for the worker children were not evaluated (that suite fails on missing `enhanced-resolve` and on the vite-config graph, unrelated to this change). The shard-client change adds 10 browser-served modules and the reservation change adds 11 worker-served modules.
4. `💻️os/📦️packages/🟦️typescript/📋️project.json` still lists only the worker leaf as Nx input; the worker children (c6tsfw) and the 11 reservation children are not hashed there. Not changed (outside this task); same class of issue as the c6tsfw list.
5. Tests that reach into the client now depend on the names `hub`, `state`, `transport`; intended and centralised in `shardClientInternals`.
6. Tool files: `C/cls-tools/` (`splitclient.mjs`, `splitreservation.mjs`, `verify-lossless.mjs`, `wgpu-closure.ts`, `wgpu-ownership-inputs.ts`, `run-vitest.sh`, `vitest.config.ts`, `summarize.py`, `tsconfig.*.json`, `orig/`). No logs left in the repo or in `C/🗑️generated`.
