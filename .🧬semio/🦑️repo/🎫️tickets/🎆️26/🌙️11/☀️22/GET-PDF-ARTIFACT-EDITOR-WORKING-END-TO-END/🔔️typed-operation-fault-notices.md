# Typed-operation fault notices

## Cause

A guest settles a refused typed operation (`stdio.pdf.page-number-out-of-range` with `page`/`last`, `stdio.pdf.unknown-field`, `stdio.pdf.save-declined`) with a fault page (lane 11) whose payload is the canonical Fault JSON. Two defects kept it from the person:

1. `consumeTypedOperationEffects` and `withTypedOperationCall` (`🔌️PluginRuntime/📬️typed-result`) decoded the payload as text and threw a plain `Error("typed-operation failed: ...")`. Command refusals are decoded by `invocationFromFrames` into a `SemioFaultError`, so the shell's `appFaultNoticeV1` (the app's `faultNotices` catalog, en/de, `{name}` from `params`) fires for them (`transfer-busy`); a plain `Error` never matches.
2. An operation settles on a drain turn after the dispatching call returned. The drain's `catch` (`🚚️handle-turn-drive`) only pushed `{instanceId, error}` onto `turnOutcomes`, which no shell code consumes, and a fault whose owning call already returned (or was evicted/never attributed) went to `console.error` only. The shell's notice paths (`program-dispatch`, command bar) are reached only for an error thrown to the dispatching call.

## Fix

- `typedOperationFaultError(text)` decodes every fault page into a `SemioFaultError` (code, params, severity, causes, scope kept). A payload that is not a Fault object becomes `interactive-job.undecodable-fault` carrying the text: no plain `Error`, no swallowed code.
- Both throw sites use it.
- New per-instance fan-out `operationFaultFanouts` / `publishOperationFault` and `PluginWasmHandle.subscribeOperationFaults(instanceId, listener)` (adapter implemented, test handle mocks extended). Published by the drain's `catch` and by the router for unattributed, evicted and outlived faults (the parked entries now remember their receiver). Without subscribers the fault is logged with its code.
- The shell (`⚡️host-effects/📡️operation-subscriptions`) subscribes for the session and spawned instances and shows `settledFaultNoticeV1` through `showTransientNotice`, i.e. the existing `TransientNotice` (`role="alert"`/`aria-live="assertive"` for error and warning, `data-notice-code` carries the code).
- `settledFaultNoticeV1` (`🛠️ShellHelpers/🛣️route-and-chrome`): the app's catalog notice in the shell locale, else a generic localized text naming the code (`The operation was refused ({code}).` / `Der Vorgang wurde abgelehnt ({code}).`); the locale comes from `currentShellLocaleV1()`, which throws without a language, so no default language is assumed.

## Tests (bun, 8 pass)

`🔌️PluginRuntime/📬️typed-result/fault-notices.test.ts` over the language-neutral fixture `🔌️PluginRuntime/🧪️tests/🧫️fixtures/🔔️typed-operation-fault-notices.json`:
- known code with two params, known code with one param and no severity: the consumed page throws a `SemioFaultError`; text in en and de with placeholders filled, severity, code;
- unknown code: generic localized notice plus the code, en and de;
- undecodable payload: `SemioFaultError` with `interactive-job.undecodable-fault`;
- parked fault rejects its own call with a `SemioFaultError`;
- an unowned settled fault reaches the instance subscribers exactly once and renders en/de;
- a successful operation (terminal page) throws and publishes nothing;
- no subscriber: logged with code.

The existing vitest registry suites (`🧪️tests/📃️typed-operation-pages`, transaction coordinator, via `🔌️plugin-runtime-suite`) assert fault text by substring; the undecodable path keeps the raw text as the message.

Third-party cross-check: the existing Ajv fault-schema law (`🛠️ShellHelpers/🧪️tests/🧪️fault-notices`) still passes (4/4); the new fixture reuses the same notice-table shape.

## Journey

`pdf-pane-journey.ts`: the out-of-range step now requires the notice with code `stdio.pdf.page-number-out-of-range` to be an assertive alert whose whole text matches the en/de sentence; a new Details step sets the unknown path `/info/nonexistent` and requires the `stdio.pdf.unknown-field` alert with the en/de sentence. To be run by another agent after the next wasm rebuild.

## Changed files

- `🔌️PluginRuntime/📬️typed-result/🟦️.ts`, `🧷️handle-contract/🟦️.ts`, `🔗️channel-adapter/🟦️.ts`, `🚚️handle-turn-drive/🟦️.ts`, `🟦️.tsx`
- `🏛️ShellHost/⚡️host-effects/🟦️.ts`, `⚡️host-effects/📡️operation-subscriptions/🟦️.ts`
- `🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx`
- `🧪️tests/🛠️plugin-runtime-helpers/🟦️.tsx`, `🧪️tests/🏗️transaction-coordinator/🟦️.tsx`
- new: `📬️typed-result/fault-notices.test.ts`, `🧪️tests/🧫️fixtures/🔔️typed-operation-fault-notices.json`
- ticket: `pdf-pane-journey.ts`, this file

## Verification gap closed

- The earlier 5-minute CPU spin was not a product hang: `bun test` on a vitest-only suite file (`import * as vitest from "vitest"`) spins; the files are run by the package's own runner. Intended runner: `runVitest` in `⚛️react/📦️packages/🟦️typescript/📜️script.ts` with config `../../🧪️tests/🎚️config/🟦️.ts` (needs `SEMIO_VITEST_POLICY`; the repo MCP `test_file` is not connected in this session). `run-vitest-one.sh` in the ticket builds that policy (node, `--maxWorkers=1`, `--pool=forks`, `--no-file-parallelism`, 3 GB heap) and runs one suite file.
- `🔌️plugin-runtime-suite` (includes the typed-operation pages tests, the transaction coordinator tests with the edited handle mocks, the foreign-fault routing and parked-capacity laws): **144 passed, 0 failed**, 4.5 s. Log `🗑️generated/vitest-plugin-runtime.log`. The transaction coordinator has no suite file of its own; the suite registers it (20 matching lines in the log).
- Subscription lifetime: `releaseInstanceMaps` (instance teardown, `🪢️handle-scope`) now also deletes `operationFaultFanouts` for the instance, as it already did for completion and diagnostic fan-outs. The shell's own subscriptions are released by the effect cleanup. New test: releasing an instance drops its fault fan-out.
- Ajv law: every fixture fault page decodes to a `Fault` admitted by the diagnostic Fault schema (`diagnosticControlledSchema`), as in the existing `fault-notices` law.
- `fault-notices.test.ts` (bun): **10 passed, 0 failed** (8 before plus the Ajv law and the teardown law). Existing `🛠️ShellHelpers/🧪️tests/🧪️fault-notices`: 4 / 0.
