# Inference Fixture Ownership Repair

Root canonical build-inference gate 126714 settled status 1, signal null, at 2026-10-30 01:46:08 UTC. Its log, 🗑️generated/graph-module-memo-canonical.log, identifies concrete live-store fence refusals in three assigned files:

- 🕸️graph/📥️inference/🧪️tests/🔗️late-closures/🟦️.ts
- 🕸️graph/📥️inference/🧪️tests/🚧️withheld-rows/🟦️.ts
- 🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🟦️.ts

Paths are relative to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/. These tests used mkdtemp under tmpdir, but the canonical runner places TMPDIR inside the checkout store. Their ordinary record namespaces therefore reached fenced live storage before assertions could run.

The three test files now use the existing privateRepositoryWorkspaceV1("graph"), owned scratch directories, and genuine repository.invoke contexts. No production observation or storage interface was replaced. The existing live-store fence, grant issuance, optional publication limits, deadlines and native ownership checks remain intact. Source parses cleanly through the independent TypeScript parser; no ambient mkdtemp/tmpdir scratch remains in these three files.

The closure-facts child now uses the real captureOwnedCommandResultV1 and scriptInvocationEnvironment interfaces, receives its actual invocation through receiveScriptProcessInvocation, and checks actual status/signal before decoding its result. Its warm reuse, damaged-block recompute, healing and repeat-reuse expectations are unchanged. Late-closure and withheld-row Python oracles now resolve the interpreter through the existing first-party requireTestToolV1 interface; they are required and not skipped.

All existing neutral schemas, Python/esbuild oracle comparisons, closure roster and grouping assertions, observation count/reuse assertions, source-repair/stale assertions, contradictory-project pass behavior and block sharing/recovery checks are preserved. No test is skipped or weakened. Only fixture ownership/context and the actual child protocol were corrected.

Focused normal validation was submitted once: handle 10613 / PID 219060, label graph-inference-private-fixtures-focused. The outer durable Node logger is light; the actual repo tools exec bun test command is normal. It selects only these three explicit files. It is pending admission, so no runtime acceptance is claimed yet. Root owns the separate workspace-closure, mounted-source and provider-artifact repairs; the external Course owner handles course-input gaps. The whole canonical gate has not been rerun.

## Focused Actual Result

`graph-inference-private-fixtures-focused` closed 1/null at 2026-10-30T02:19:40.558Z: 22 pass, 10 fail, 166 assertions. No live-store fence remains. Eight text-oracle failures were stale function-end docstring markers, corrected to actual next declaration boundaries while preserving all content/order/absence predicates and independent Python oracle. One withheld case failed ENOENT because shared helper afterAll retired its memoized private workspace after the first file. One block-sharing case retained its strict small-header assertion and exposed 79,043 public-header bytes against the 2,960-byte bound; authority observations need causal inspection. No whole-gate rerun or shared helper/header edit made.

The actual registered `repo tools exec --light python` metadata oracle over the updated neutral closure fixture returned exit 0 and `{failures: []}`. This validates all existing order/presence/absence predicates against actual current Source independently of Bun. It does not certify fixture lifetime or header runtime laws.

## Canonical Topology Correction

The bundled three-file invocation above violated the authored build suite contract: each test file receives its own genuine Original process via runProcessTestSuiteV1 / genuineBirthLaunchersV1. Its ENOENT and oversized authority header were cross-file aggregation artifacts, not canonical defects. No private helper or production header/schema was changed. Unrun authority optimization test additions were removed at Root direction. The complete existing strict small-header, block corruption/healing, facts, stale and independent oracle assertions remain.

Actual one-file genuine runners: withheld status 0/null at 2026-10-30T02:39:10.074Z, 6 passed / 39 assertions (`inference-withheld-private-genuine`); closure status 0/null at 2026-10-30T02:39:20.574Z, 22 passed / 106 assertions (`inference-authority-header-genuine-red`, whose label describes its intended probe, not its observed result). Late-closures has its own normal genuine owner PID 255177 / handle 68565 pending. All runs use existing genuine-birth/script.ts with --file and --root, no name filter.

Final actual late-closures genuine outcome: status 0/null at 2026-10-30T02:41:48.412Z, 4 passed / 52 assertions (`inference-late-private-genuine`). All three owned private-fixture files are accepted independently with their complete inventories; total 32 tests / 197 assertions, no assertion filters. Parent owns coherent whole-suite execution after peers finish their separate sources.
