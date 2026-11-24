# 🧪️ Plugin TS triage after the sync stage exports

Ticket `26/09/01/SOURCING-END-TO-END`, 2026-11-23. Follows `🧯️async-param-leak.md`.

Set: 50 bun test files under the plugin `🌐️browser-bundle`, `🧪️tests/🧬️component-codec-reply`, the new `🧪️tests/🧭️actor-failure-ordinals`, `🧬️schema/🎟️actor-input/🧪️tests` and `🧰️framework/🔨️modules/🎭️actor` (list in `🗑️generated/ts-plugin-files.txt`).

Before: 131 pass, 20 fail (`🗑️generated/ts-plugin.txt`). After: **163 pass, 0 fail** (`🗑️generated/ts-plugin-4.txt`), run with `SEMIO_TEST_ARTIFACT_DIR` set to a directory outside the repo. Without that variable the typegen, lifetime and two-checkout tests fail by design (they require the runner's artifact directory); 3 of the original 20 were that.

## Caused by the stage-export change

| Failure | Cause | Fix |
| --- | --- | --- |
| `General Actor source and test mounts reject product references` (retained-turn) | `originalStagedGuest`, with the plugin export names and the plugin WIT path in its docstring, lived in the general actor module | Moved to the plugin product layer, see below |
| `Actor public facets publish only their defining ... rosters` | The roster lacked seven runtime exports and ten types that earlier peers had added to `retained-turn` (stale at HEAD); `originalStagedGuest` then added one more | `originalStagedGuest` no longer exported by the general facet. Roster fixture and its closed schema enum updated to the real facet |
| `WIT selector ordinals match the neutral semantic frontier through SQLite` | Stale at HEAD (`stage-command-page` already existed) and wrong now: it asserted budget parameters on `publish/read/acknowledge-actor-failure`. It also read product WIT from a general test, the second owner-boundary violation | Neutral test keeps the SQLite ordinal density check. The WIT assertions moved to the plugin test `🔌️plugin/🧪️tests/🧭️actor-failure-ordinals`, asserting the new staged signatures |
| `the closed document actor module lowers every reactor.poll and jobs invocation ...` | Calls now include the stage calls | Test uses the `sequence` helper like its sibling tests |
| `the WIT oracle refuses the pre-admission shapes ...` | `poll`, `describe`, `restore`, `startJob` etc. now take no budget | Test restated against `stageActorContextBudget`, `stageJobBudget` and the budgetless consumers (arity messages `n arguments for 0/1 parameters`) |
| `actual OS materializer preserves the original bootstrap receipt and clock` | Pre-existing at HEAD (bridge already used `reactor.poll(slice)` and `receiveOriginal(receipt, ...)`), not caused by this change | Strings updated to the bridge as it is |

## Layering fix

`originalStagedGuest` (neutral) became `pluginStagedGuest` in the new plugin leaf `🔌️plugin/🌐️browser-bundle/🎚️staged-guest/🟦️.ts`:
- `PLUGIN_STAGED_BUDGET_GROUPS` is derived at load from the authored WIT (schema first): every async function of `reactor`, `checkpoint`, `describe`, `codec` behind `reactor.stageActorContextBudget` and of `jobs` behind `jobs.stageJobBudget`, minus the declared unbudgeted exports (`stageCommandPage`, `channelVersion`, `takeSegmentedDownloadChunk`). Loading fails when the sync stage export is absent.
- `stagedGuest(raw, groups)` is the neutral mechanism, `pluginStagedGuestSource()` emits it into the worker and bridge sources.
- To derive asynchrony, `🛂️descriptor` gained `parseActorComponentFunctions` and `ACTOR_COMPONENT_FUNCTIONS`; `parseActorComponentExports` now projects from it (same results).
- New test `🎚️staged-guest/🧪️tests`: the derived roster equals an explicit roster and the independent JCO declarations (Promise returning exports minus the unbudgeted ones); the behavioural test checks stage then call order for the function and for the emitted source.
- The general module no longer mentions a plugin export, product path or `stage-*` name. Consumers: `🧩️actor-module` and `🏗️materialization/🌉️component-bridge-source` embed `pluginStagedGuestSource()` after `originalOperationWorkerSource()`; `🧬️wit-lowering` does the same.

## Pre-existing failures (independent of the change), fixed

All were already failing at HEAD (checked: files unchanged by the stage work, causes are peer drift).

- `General Actor ... reject product references`, second hit: `🎟️retained-turn/📨️actor-input-wire/🧪️tests` read the plugin actor-input corpus. Moved to `🔌️plugin/🧬️schema/🎟️actor-input/🧪️tests/🧰️wire-encoder/🟦️.ts` (product tests may import the neutral encoder).
- Five `🎟️context` tests and `original Actor context is a public General owner`: schema of the phases corpus (9 phases now, `source-prepare` ...), decoder schema lacked `interruptedReceiving`, the boundary roster lacked the new private `ActorBorrow` enum, hard coded item sums, and Rust substrings renamed (`borrow_context` to `borrow_context_for_turn`, `fn close_decoder` to `fn close_owned`, `OriginalCallerSourceParent::admit` to `OriginalCallerSourceAdmission::prepare_demand`, `consume_receiving_retained(grant,progress)` to `cx.consume_retained(progress`). Schemas, fixtures and tests updated.
- `original Actor receiving keeps original input preserved ...`: ordering anchor `meta.metrics.record_turn` is now `self.record_turn_outcome(`.
- `original actor resource lanes preserve supplied authority ...`: `TurnGrant` gained `loan: OriginalActorTurnLoan`, so the empty turn grant is 193 bytes (the Rust test pins 193); fixture gained `loanBytes: 60`, the SQL formula adds it.
- `original host wire receiving matches Buffer ABI ...`: fixture field `minimumPresentationTurns`, schema extended.
- `actor typegen ownership ...`: Nx `namedInputs.default` now holds an object entry (type guard in the test) and lacked the typegen test input (added to `🎭️actor/📦️packages/🦀️rust/📋️project.json`).
- `Actor and Job examples belong strictly to neutral tests`: `🧵️job/🧪️testing/♻️close` held fixtures outside `🧪️tests`. Moved to `🧵️job/🧪️tests/♻️close`; five Rust `#[path]` mounts updated (ui host window-unit, wgpu prepared, ui physical-job-close, plugin raw-allocation-close-oracle, board-host icon-clip). Paths verified to exist; the crates were NOT compiled.
- `neutral component model readers retain exact two-intent receipt predicates`: the receipt check moved into `readRetainedComponentModelV1`; corpus row renamed, the store-entry test got its own `storeEntry` corpus key (schema extended).
- `native transpile and wasm-opt give the same files ...` and its dependent: `optimizeStageRequestV1` takes the optimizer digest (`sha(binaryen)`), and fleet hits are counted as `fleetHits`.

## Not fixed

None open. Environment note: the typegen, lifetime and two-checkout tests need `SEMIO_TEST_ARTIFACT_DIR` (outside any Cargo config tree, otherwise the scratch base falls back to the system temp dir).

## Changed files (all TS or JSON or Rust test mounts; none touches the wasm guest)

General actor (`🧰️framework/🔨️modules/🎭️actor/`): `🎟️retained-turn/🟦️.ts`, `📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts`, `🧰️framework/📦️packages/🟦️typescript/🎭️actor/🎟️retained-turn/🟦️.ts` (`originalStagedGuest` removed); tests and fixtures: `🧪️tests/📣️public-facets/{🔣️,🧬️}.json`, `⚠️failure/🧪️tests/🟦️.ts`, `🎟️context/🧬️schema/{🔣️,🛬️decoder}.json`, `🎟️context/🧪️tests/🟦️.ts`, `🎟️context/🧪️tests/🏛️boundary/{🔣️,🧬️}.json`, `📃️policy/🧪️tests/{🟦️.ts,🧫️fixtures/🔣️.json}`, `🫴️receiving/🧪️tests/🟦️.ts`, `🖼️wire-turn/📬️typed-operation/📥️receiving/🧬️schema/🔣️.json`, `🧪️tests/🧬️typegen/🟦️.ts`, `📦️packages/🦀️rust/📋️project.json`; removed `📨️actor-input-wire/🧪️tests/🟦️.ts`.

Plugin (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`): new `🌐️browser-bundle/🎚️staged-guest/🟦️.ts` and `🧪️tests/🟦️.ts`; `🌐️browser-bundle/🛂️descriptor/🟦️.ts`, `🧩️actor-module/🟦️.ts`, `🏗️materialization/🌉️component-bridge-source/🟦️.ts`; tests `🏗️materialization/🧪️tests/{🧬️wit-lowering,🏛️bootstrap,🪞️two-checkout}/🟦️.ts`, `🏗️materialization/🚀️commands/🧪️tests/🎚️mode/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json}`; new `🧪️tests/🧭️actor-failure-ordinals/🟦️.ts`, new `🧬️schema/🎟️actor-input/🧪️tests/🧰️wire-encoder/🟦️.ts`.

Job and others: `🧵️job/🧪️testing/♻️close` moved to `🧵️job/🧪️tests/♻️close`, plus the five Rust `#[path]` lines listed above.

## Wasm impact

None of these changes affects the Rust guest or the WIT. The only runtime-relevant change is TS host side: the stage lowering moved from the actor module into the plugin layer and is emitted into the generated worker, bridge and actor module sources, which behave as before (same stage then call sequence, now verified by `🎚️staged-guest` and `wit-lowering`). A server restart suffices. The Rust `#[path]` edits only touch test mounts of non wasm crates and still need a `cargo check` of those crates when the wasm rebuild is done.

## Leftover observation

`📜️reactor.wit` header comments (lines 10 to 12) still say a plain sync `func` export is uncallable on any host; that now contradicts the sync stage exports. The comment belongs to the peer who owns the schema text and was left as is.
