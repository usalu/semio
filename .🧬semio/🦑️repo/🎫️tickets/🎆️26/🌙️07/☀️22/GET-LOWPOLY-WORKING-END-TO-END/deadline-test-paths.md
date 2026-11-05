# Deadline Test Paths

Read-only exploration, 2026-11-05. No tests/heavy command, edits to infrastructure or environment changes.

## Likely Failure Mechanism

🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:260 computes live control remaining time with creditedDeadlineRemainingV1 and admissionWaitedMillisecondsV1. However scriptResourceProcessEnvironmentV1:336 takes minimum raw absolute deadlines from original/carried envelopes, ancestor controls and projectedChildDeadlines, then :337 rejects an elapsed absolute deadline even if remaining(owner) is positive after admission credit. This is a concrete source inconsistency consistent with the reported post-capacity child-deadline failure, not an executed diagnosis.

## Existing Tests And Fixtures

- Same process owner /🧪️tests/⏳️admission/🟦️.ts has three tests: independent Python arithmetic model, overlapping/open admission wait accounting, and a control whose 600ms budget survives 450ms queued plus 300ms active. It checks remaining() and eventual expiry, never child environment projection. Cases are inline numeric rows, not a neutral JSON corpus.
- /🧪️tests/📤️fractional/🟦️.ts and 🔣️.json cover integer envelope deadlines, Python numeric oracle and Ajv schema. They use scriptInvocationEnvironment, not resource phase projection after admission.
- Repo build /📥️caller/🧪️tests/📥️birth/🟦️.ts:82 and 🔣️.json preparationCases cover scriptResourceProcessEnvironmentV1 phase narrowing, capability preservation and repeated projection nonextension, with SQLite oracle. No admission wait occurs.
- Exact resource policy schema: process /🧭️routing/🧬️schema/🔣️.json; process envelope schema /📥️invocation/🏃️process/🧬️schema/🔣️.json.

## Supported Entry Paths

Process /📋️project.json declares @semio-tech/framework-process:test-routing-owners, invoking repo ./📜️script.ts test routing-owners. Route table /🧪️testing/🗂️suites/🔣️.json selects entire routing tree with 300000ms budget; official route is darwin/linux only with python3-command platform cause. No dedicated admission-credit target exists. test-routing selects only two top-level files, not the admission test.

Smallest no-general-build-graph registered-tool route is repo tools exec bun -- test <exact admission test path>. tools exec is a public registered tool interface, and that file creates genuine fixture envelopes itself. Existing first test requires python3, so native Windows availability remains a condition; official suite deliberately skips Windows. Do not bypass this by spoofing a tool or envelope. A direct source-script suite invocation repo <process-script-path> test routing-owners avoids a requested build graph but retains official suite/platform admission and runs broad routing tests.

## Cargo Deadline Creation

Repo build /📜️script.ts:109 readDefaultBuildCliPolicyV1 uses buildBudgetMs(environment), default zero; :135 preserves an incoming policy before creating a default. Build runtime /⌨️command/⚙️runtime/🟦️.ts:49-52 creates issued cargo:build-entry control requested 0 and calls launchRepositoryCargoV1 with maximumMilliseconds:0. Cargo /🚪️launcher/🟦️.ts:185 derives admission with createRequestedIssuedResourceControlV1(phaseControl,options.maximumMilliseconds). Zero inherits original deadline rather than resetting it. Later owned process launch projects the current control into child wire data, exposing the inconsistency above.

A regression should cover genuine bounded original parent and narrowed control, overlapping/open credited wait, child projection after raw deadline elapsed but live credited remaining, repeated projection without new active-time budget, and true active-time exhaustion. Preserve exact original capabilities and cancellation ancestry.
