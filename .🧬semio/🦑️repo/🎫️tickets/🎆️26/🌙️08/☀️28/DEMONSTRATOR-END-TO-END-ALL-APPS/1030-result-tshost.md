STATUS: GREEN. The TS browser host (shard bridge and document actor child) now lowers reactor/jobs calls exactly as the WIT declares; a WIT-derived oracle test pins it. Not verified in a real browser with compiled wasm (no component build run).

Paths: M = `🧰️framework/🛍️products/💻️os/🔨️modules`, P = `M/🔌️plugin`, A = `🧰️framework/🔨️modules/🎭️actor`.

## Schema-first
No TS mirror of the WIT existed and none is hand-maintained now. The oracle is the WIT itself: the test derives declarations with JCO `types()` (same independent source as the descriptor test), parses them with the TS compiler API, and (a) type-checks the host lowering helpers against them with a real `ts.createProgram`, (b) validates every recorded runtime call exactly (exact field set, required/optional, bigint/number/bytes, arity).

## Changes
- `A/🎟️retained-turn/🟦️.ts` (single source embedded into every worker via `originalOperationWorkerSource`):
  - `originalTurnWit(turn)` now takes the issued turn and builds `retained-turn-input` including the `work` record (`clockBinding` = operation id, `absoluteDeadlineUs`, `fuel`).
  - New `originalBudgetWit(turn, shape)` builds the full `budget` (retained, retainedPolicy, workFuel, fuel, deadlineUs, max-*), `originalGrantWit`, `optionValue` (the bridge-private `unwrapOption`, now shared).
  - New `OriginalJobFunds`: port of the native `JobFunds` relay (self-issued retained turn, loan settle per crossing) driving start-job (`option<job-source>` + job-budget), step-job, read-job-output pages, acknowledge-job-output, cancel-job (job-budget). Returns the `{status,value|progress}` step shape callers already declare (replaces `normalizeJobStep`).
  - New `OriginalSelfIssuedTurns`: for a guest with no Kernel grant (document browser actor child): one self-issued `OriginalOperationSource`, admits the actor context once, then issues each `poll` (one event max, refused otherwise) and each job crossing.
  - Facades: `A/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts`, `🧰️framework/📦️packages/🟦️typescript/🎭️actor/🎟️retained-turn/🟦️.ts` re-export the new names/types.
- `P/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`: admit/close pass `(input, retainedPolicy)`; receipt work units come from the call (`call.workUnitsSpent`, `retainedReceipt.workUnitsSpent`, no longer `0n`/`fuelUsed`); poll budget = `originalBudgetWit`; start/step/cancelJob go through `OriginalJobFunds`.
- Document browser actor path (raw jco `reactor.poll` with a stale budget): `P/🌐️browser-bundle/📜️script.ts` generated actor module now routes `reactor.poll`, `jobs.startJob|stepJob|cancelJob` through `OriginalSelfIssuedTurns`; its template moved to new `P/🌐️browser-bundle/🧩️actor-module/🟦️.ts` (`browserActorModuleSource`, testable without the build closure). `M/🏪️store/👷️worker/🧩️actor-wire-helpers/🟦️.ts`: `browserActorTurnBudget` -> `browserActorTurnShape` (max-effects/patch-bytes/frames only); `M/🏪️store/👷️worker/🧷️actor-reservation/🛤️poll-lanes/🟦️.ts` sends the shape and `{deadlineMs}` for steps. `P/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts`: child host port now has a real monotonic `operationClockUs` (was `undefined`).
- GIS fakes: `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🗺️document-open/` `🪤️cold-pair-worker-fake` (one event per poll, owed-surface queue, lazy-render `more-work` modelled as guest state), `↩️undo-owner-phases` (hostile render: one event per turn; reservation accessors moved to `hub.session`/`hub.surfaces`), `🚚️transfer-and-render-phases` (`hub.session.viewRefresh`). The last two accessor fixes were pre-existing breakage from the reservation split, not from this change.
- Staging fakes: `M/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🧰️test-kit` guest now answers WIT-shaped calls (`workUnitsSpent`, `remaining`, `remainingFuel`); `🧬️jco-rewrites`, `🛂️routes-and-bridge` use the shape budget and assert the new bridge call text.
- Tests updated for the `originalTurnWit(turn)` signature: `A/🎟️retained-turn/🏛️bootstrap/🧪️tests`, `A/🎟️retained-turn/🤝️admission/🧪️tests/🕸️kernel`, `P/🌐️browser-bundle/🏗️materialization/🧪️tests/🏛️bootstrap`.
- NEW test `P/🌐️browser-bundle/🏗️materialization/🧪️tests/🧬️wit-lowering/🟦️.ts` (5 cases): real generated bridge vs recording guest; `OriginalSelfIssuedTurns`; the real closed actor module (`browserActorModuleSource`) invoked like the child; negative case proving the oracle rejects the pre-fix shapes (missing `work`, `retained-policy`, `work-fuel`, extra `deadlineMs`, 1-arg admit/close/cancel, 3-arg start-job); tsc assignability of the helpers against the JCO declarations (verified to fail on a deliberate mismatch).

## Verification
- `bun test` (explicit paths): wit-lowering 5/5; materialization `🏛️bootstrap` 3/3, `🚪️lifetime` 2/2; retained-turn `🧪️tests` 2, `admission/kernel` 1, `bootstrap` 3, `job/arguments` 1.
- vitest staging (`1030-tools/tshost/vitest-dev.ts`, long): 72 pass, 3 fail = the same 3 environmental failures as 1029 (2x `SEMIO_SCRIPT_PROVIDERS`, vendor `cli.js` byte identity). Was 8 failing before the test-kit fix (log `🗑️generated/1030/tshost-vt1.txt` vs `-vt2.txt`).
- vitest GIS service-composition (`vitest-gis.ts`): 143/143 (`tshost-gis2.txt`). vitest worker-law-suites (`vitest-os.ts`): 100/100.
- tsc wide (`1030-tools/tshost/tsconfig.json`, adds retained-turn, store worker, staging tests, GIS tests, new actor-module): no error in any touched file. Remaining: `@semio-tech/puzzle-wasm` / `@semio-tech/flow-core` not found (env); the last run (tsc3) also shows a foreign peer mid-refactor of `🏃️process/📦️artifacts/🏗️native-build/💾️storage` (missing sibling modules; the earlier tsc2 was clean of it).

## Notes / not done
- Stale gitignored jco copies under `plugin-modules`/`extension-modules` untouched (activation regenerates them).
- `script.ts` cannot be imported in isolation right now (foreign: `🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower` imports a missing `♻️retirement/🧭️continuation/🟦️.ts`), which is why the actor module template was split out.
- New dir `🧩️actor-module` under browser-bundle: if a taxonomy/artifact-name registry gate lists browser-bundle members, it may need the entry (not checked by a gate run).
- Jobs `read-job-output` collection follows the native relay; a job whose guest never returns `output-closed` with a page faults `original-job.output-stalled` like native.
- Open (from audit §6): no fixture test that a 2-event `poll` is refused by the Rust guest too; TS side now covered (bridge + driver + lane).
- Tools: `C/1030-tools/tshost/{tsconfig.json,vitest.ts,vitest-dev.ts,vitest-gis.ts,vitest-os.ts}`.
