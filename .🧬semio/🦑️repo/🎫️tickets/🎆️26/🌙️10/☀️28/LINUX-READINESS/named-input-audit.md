# Canonical Test Environment Named Inputs

The actual workspace86952 failure was `Unknown selected named source input: testEnv`. The unknown-name guard is correct and remains unchanged.

## Cause and Ownership

A current physical manifest inventory found 246 manifests referencing testEnv. Of these, 245 declare a genuine local group containing SEMIO_TEST_LEVEL, SEMIO_TEST_BUDGET_MS and SEMIO_BUILD_BUDGET_MS. Only the workspace root references that name without a local definition. Both the canonical workspace build policy and Nx workspace named-input configuration lacked a global definition.

The actual `buildSourceContractGroupsV1` implementation merges workspace defaults and local project groups, so this is not a missing local-override implementation. The normalized project-input assembler retains authored local declarations. The graph's WORKSPACE_POLICY is read directly from `🏗️build/🔣️policy.json`; graph publication exposes that policy's namedInputs. No separate global Nx configuration emitter was found in the config-generation command: that command currently owns nextest scope configuration and dependency-manifest validation. Nx configuration therefore received the corresponding explicit declaration as well.

## Change

The canonical policy and nx.json now declare testEnv with the three existing authored test-control environment variables. This is a real current environment-input group, with no filesystem entries, aliases or missing-name fallback. Existing local project declarations remain valid overrides.

A closed language-neutral corpus and schema live under `🏗️build/📥️inputs/🌿️environment`. They cover workspace inheritance, environment plus a selected Source contract, a misspelled unknown environment group and an unselected Source contract. The test executes the actual root manifest's complete target input closure. SQLite independently computes recursive named-input reachability and contract/unknown classification; Ajv independently validates the corpus. The misspelled group still raises the original precise guard error.

The new test is registered in the ordinary workspace-build test list and its root runtimeSourceEntries. The existing exhaustive workspace-policy group fixture includes the new canonical group.

## Red and Green Evidence

Before policy correction, both new cases failed: actual root admission raised `Unknown selected named source input: testEnv`, and the independent SQLite closure classified testEnv as unknown. This reproduces the actual graph cause without a full graph run.

An existing normalized integration initially failed on separately stale opacity test ports. Narrow fixture repair restored malformed WORKSPACE_POLICY keys and supplied the actual imported opacity helpers to extracted input/assembly ports. No production opacity logic or Source guard changed.

Final handle84172 completed0: 32 tests, 1257 assertions, two suites, 4.55 seconds. This includes all 30 source-demand tests and the two new environment-input tests. The directly streamed log is temporarily retained at `🗑️generated/named-input-final.log`.

## Files

- Canonical `🏗️build/🔣️policy.json` and root `nx.json`.
- New `🏗️build/📥️inputs/🌿️environment/🧬️schema/🔣️.json` and adjacent test corpus/runner.
- `🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json` exhaustive policy roster.
- `🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts` fixture ports.
- `🏗️build/📜️script.ts` ordinary test registration and root `📋️project.json` runtime Source entry.

No full graph or setup retry was launched by this agent. Root owns the next actual workspace verification.
