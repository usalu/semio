# Result fnsplit3 (restructuring of the 9 functions over 600 lines that fnsplit2 left)

Executor `fnsplit3` (Sonnet 5.5), foreground, no sub-agents, no modifying git command, no cargo, no `bun repo` command. `C` = this ticket folder. Freeze respected: nothing under `📚️library/`, no root `📜️script.ts`, no `package.json`/`bun.lock`, no `🔣️taxonomy.json` was edited. All nine files were older than 15 minutes when I started.

STATUS: all nine functions are restructured. No function of the nine (root or new helper) exceeds 292 lines (before: 941, 773, 759, 710, 688, 669, 663, 637, 635).

## Result per function (before -> root after, largest new helper)

| function | before | root after | largest helper | method |
| --- | ---: | ---: | --- | --- |
| `registerGisDocumentOpenTests` (`✏️s/…/🗺️gis/…/🧪️tests/🗺️document-open`) | 941 | 130 | `createColdPairWorkerClass` 292 | `probe` state object for 15 mutable `let`s, the module-level type aliases and `ColdPairProbe` moved to `📐️document-open-types`, the 839-line `it` cut into 8 phases (fixtures, session setup, fake worker class, 5 try-body phases); each phase takes the registrar bindings through `Pick<…>`/the three binding objects and the locals it needs and returns the locals later phases read |
| `useShellActions` (`🏛️ShellHost/🖱️shell-actions`) | 773 | 186 (hook) | `runShellInput` 181 | the `onAction` ladder: stage functions per branch (utility/tool activation, recovery, world navigation, sync card, panel tab, target resolution, remote undo, view state, direct actor, program dispatch) and `runShellInput` for the try body; typed context from the checker (`Pick<ShellCoreAfterShellUriAndDocuments, …> & {locals}`) |
| `useApplyHostEffects` (`🏛️ShellHost/⚡️host-effects`) | 663 | 144 | `applyReplayShellCommandEffect` 156 | one function per `"x" in effect` arm (14 arms), the one mutated local `nextViewState` flows in and out, `return;` of the pass becomes `null`/`"stop"`; the four operation `useEffect`s became two sub-hooks |
| `useBackboneWorker` (`🏛️ShellHost/🧵️backbone-worker`) | 635 | 220 | `handleDocumentEventMessage` 98 | the `worker.onmessage` ladder: one handler per message kind (16 handlers, narrowed with `Extract<BackboneWorkerResponse, {kind}>`), plus the plugin install state sub-hook |
| `createGisMapInferenceWorkerV1` (`💡️inference/👷️worker`) | 710 | 40 | `reconcileInferenceJob` 47 | closure factory -> explicit `GisInferenceContext {host, post, state}`; 29 inner functions moved to four modules (their calls pass `ctx`, the four `let`s live in `ctx.state`), the `test` seam keeps its shape through bound members |
| `loadPluginModule` (`🔌️plugin-module-loader`) | 688 | 12 | `createTurnDrivers` 199 | five `let`s in one `state` record, module `let nextGlobalInstanceId` in `globalInstanceIds`, statements reordered (no forward reference), then eight phases (psplit2) returning the closure bindings |
| `createBrowserBundleTests` (`🧪️tests/🌐️browser-bundle`) | 669 | 7 | `createBrowserActorBundleTests` 230 | factory of factories: codegen, actor bundle, host activation, component factory (which receives the others) |
| hub `two-client-document` e2e describe | 637 | 97 | `runCrashRecovery` 120 | seven shared `let`s in a `scenario` state object, workspace lookups and scenario types moved out, the 550-line try body cut into 8 phases |
| `plugin_exchange` (Rust) | 759 | 251 | `finish_exchange` 102 | `ExchangeFlags` replaces the two mutated locals; 33 `match command` arms became `arm_*` functions that re-match their own variant (`let AppCommand::X {..} = command else {…}`), the tail became `finish_exchange` |

## Changes (files)

New folders (one `🟦️.ts`/`🦀️.rs` each), grouped by root (roots keep their path):
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🗺️document-open/🟦️.ts`: `🧱️cold-pair-fixtures`, `🪤️cold-pair-worker-fake`, `🔌️cold-pair-session-setup`, `🚚️transfer-and-render-phases`, `⚡️action-and-command-phases`, `↩️undo-owner-phases`, `📐️document-open-types`.
- `…/🏛️ShellHost/🖱️shell-actions/🟦️.ts`: `🧭️input-funnel` with `🧰️utility-and-tool-activation`, `🔄️recovery-navigation-sync-and-panel-actions`, `🎯️dispatch-target-and-view-state`, `🚀️program-dispatch` below it.
- `…/🏛️ShellHost/⚡️host-effects/🟦️.ts`: `🎛️view-state-effects`, `📥️document-load-effects`, `📤️export-effects`, `🔁️dispatch-media-and-extension-effects`, `🎯️replay-shell-command-effect`, `🌱️spawn-and-open-instance-effects`, `📡️operation-subscriptions`.
- `…/🏛️ShellHost/🧵️backbone-worker/🟦️.ts`: `🪟️browser-actor-messages`, `🔌️socket-actor-messages`, `🧠️inference-history-messages`, `🌱️artifact-creation-messages`, `📇️directory-messages`, `🥾️bootstrap-messages`, `📬️document-event-messages`, `🔧️plugin-install-state`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/👷️worker/🟦️.ts`: `🧰️worker-context-and-types`, `↩️approval-undo-owner`, `🔁️port-job-turns` (with `📡️port-status-and-broker` below it), `🎛️port-lifecycle-and-decisions`.
- `…/🐚️plugin-bridge/🔌️plugin-module-loader/🟦️.ts`: `🪪️descriptor-and-instance-state`, `🖼️render-and-turn-drivers`, `📞️extension-and-request-actors`, `🧲️contributions-and-backbone`, `🔌️plugin-handle` (with `🧰️module-instance-ids` below it).
- `…/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts`: `🧬️browser-codegen-tests`, `🛡️actor-bundle-tests`, `🪢️host-activation-tests`, `🏭️component-factory-test`.
- `🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts`: `🧭️workspace-paths`, `🧰️scenario-harness` (with `📐️scenario-types` below it), `🌱️space-and-artifact-setup`, `🔌️sockets-presence-and-commands`, `🤖️agent-delegation-and-resume`, `🛑️shutdown-and-crash-recovery`.
- `…/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs`: `🎯️document-and-child-commands`, `📦️document-archive-commands`, `⚙️config-commands`, `🎞️media-commands`, `🔁️transaction-commands`, `🌐️relay-and-conflict-commands`, `🏁️exchange-finish`.

Other edited files (three source-text laws that globbed `🏛️ShellHost/*/🟦️.ts`, one level only, and so could not see any split below `ShellHost/<x>/<y>/`):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎭️actor-window-actions/🟦️.ts` (now `readModuleTreeSource` of `🖱️shell-actions`, the slice starts at `const onAction = useCallback(`), `…/🩺️window-fault/🟦️.ts` and `…/🧭️route-ledger/🟦️.ts` (glob `**/🟦️.ts` with `!…/🧪️tests/**`). `route-ledger` was already red before this run because of fnsplit2's deeper splits and is green now.

Tools and inputs only under `C/fnsplit3-tools/` (new): `bx.cjs` (block/range/if-arm extractor with checker-derived parameter types), `ctxify.cjs` (closure factory -> explicit context), `fsplit.cjs` (factory of factories), `letbox.cjs` (boxes `let`s into one state object, `lateInit` keeps declaration order), `stmtmove.cjs`, `nest.py` (moves folders and re-points specifiers), `rsarms.py` + `rscap.py` (Rust arm extractor in `rematch` mode, async, generics), `plugin-exchange-prep.py`/`-tail.py`/`-spec.py`, `doc-open-prep.py`/`-redo.sh`, `two-client-prep.py`/`-redo.sh`, `refgraph.cjs`, `diag.cjs`, `reach.ts`, `fnlen.cjs`, `ccheck.py`, `tidy.py`, vitest drivers `vt.sh`/`vt.py`/`vtsum.py`/`vtdiff.py`/`swap-run.sh`/`swap-gis.sh`, harnesses `bb-run.ts`/`bb-owners.ts`/`bb-exec-copy.ts`, `specs/*.json`, pre-split snapshots in `orig/`.

## Verification (exact commands and outcome)

RESULTS_PLACEHOLDER

## Open items

OPEN_PLACEHOLDER
