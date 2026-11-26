# F70 result: whole-library type errors by class, declaration fixes, `repo doctor` area

Invocation: `NODE_OPTIONS=--max-old-space-size=12000 tsc --noEmit -p <library tsconfig>` (about 3 min at load 40, 191 s inside `repo doctor` at load 100). Classifier `T/🧰️scripts/F70/classify.mjs` (TypeScript API: an error is "declaration" when the statement it sits in touches a symbol declared in, or initialised from, a `.d.mts`; "fixture" otherwise; "product" outside test paths). It over-approximates "declaration" (a test that uses both a declaration and a loose local fixture counts as declaration).

## Counts (errors)
| Class | Start (F67 end: 721; re-run 722) | Now (classifier 604; `repo doctor` 603) |
|---|---|---|
| product (non-test) | 2 (`owned-execution` ChildProcess union) | 1 (new, peer: see below) |
| declaration-side (tests using a `.d.mts` export) | 333 | 244 |
| fixture/stub (F68's) | 374 | 359 |
| total | 709 (722 by `grep error TS`, 13 are continuation/global lines) | 604 |

By area now (doctor report): build/graph 254, build/resident 61, build/observation 33, build/memory 28, caching/bootstrap 28, ...

## Fixed
- Product: `settleOwnedChildV1(child: OwnedSettleableChildV1, ...)` (structural: pid, streams, three events). Both errors gone; settlement test 4/0.
- Declarations (wrong or too narrow; the runtime was right): graph-hooks seam `previous` (+`built`), cutoff policy / `CLOSURE_SETTLE_V1` / settler options and result / derive answer, closure-graph save result, take policy now derived from its schema JSON (the hand list lacked 12 fields), adopt counters, cpu progress, generations `published`, plan type, new `🧭️plan` d.mts (plan has `cutoff.skipped`), doubt/stamps `published`/`save` boolean, row log, memo store, resident service pool. About 100 test errors disappeared; no `any`, no `@ts-ignore`, no casts in tests.
- Tests with a stale shape: graph-hooks (`kind: "added" as const`), cutoff (`readonly` hold parameter).

## Wired: `repo doctor` area `typecheck`
`repo doctor` reads the stored whole-library measurement (machine cache `library-report.json`) and reports errors by area; `repo doctor --only=typecheck` measures again (minutes) unless the stored one is younger than 600 s. Rules: `typecheck.product-errors` (warn, only product code), `typecheck.errors` (info, per area), `typecheck.stale` (info, older than 24 h). It never blocks (exit 0 even with `--strict`, except the warn degrades the verdict). Live proof (this machine): `repo doctor --only=typecheck` found 603 errors, 1 in product (`build/🌐️remote-execution`), 602 in tests, in 191 s.
Proof by tests: typecheck 20/0 (report equals an independent count of the compiler output, Ajv and the in-repo validator agree, cache round trip), doctor 17/0 (3 new fixture cases checked against the Python oracle), probes 24/0 (probe measures only when named and not fresh), command 9/0.

## Not fixed / for others
- The 1 product error is a peer's half landing: `🌐️remote-execution/🛰️agent/🟦️.ts:74` (05:30) passes `reservationOwner` to `buildRemoteExecutionLoadReportV1`, whose input type in `🌡️load/🟦️.ts` (unchanged since yesterday) lacks it; the owner of that edit must add the field (or drop it).
- The remaining declaration-side errors are single-purpose (examples: `HostEndpointsV1` and `HostPolicyV1` spreads in resident/invocation tests, `IndexHandleV1` fixtures, `settleSuspectsV1` without a d.mts, host `workers.active`, `executorSize`, `launchctl`/`probe` ports of the consent service, `RowRefV1` verifier shape); each needs the declaration read against its `.mjs` and is a candidate for the next lane. Many "declaration" rows are really `unknown` results (`result.graph`, `frame`) of stubbed ports.
- `operations` test file: 14 pass, 2 fail (`cache gc --reclaim` scoping, known; real-CLI help test timed out at 120 s under load 100).
