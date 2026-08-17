# Wave 2b Report — Root Policy Regions (Operations → Mutations)

## Scope

Root `📜️script.ts` policy regions only. No taxonomy.json / registry / discovery (Wave 2a). No plugins or kernel Rust.

## Changes

### 1. Renames (document Operation → Mutation)

| Site | Change |
|------|--------|
| `POLICY_PROTOCOL_MIGRATION_NAMES` | `Operation*` → `Mutation*`; `OpDag` → `MutationDag`; kept `OpText` / `Edit` |
| `PolicyRuleDslCompleteness` | `type Operation =` → `type Mutation =`; field `mutationType`; messages updated; kept `OpText` / `DslOps` |
| `POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST` | `SetDocumentOperation` → `SetDocumentMutation` |
| `PolicyRuleCommandEnvelopeCompleteness` | `Edit<Operation>` → `Edit<Mutation>`; `OperationEnvelope` → `MutationEnvelope` |
| `PolicyRuleDiffCompleteness` | scans `MutationDiff`; comments updated; `/op` allowlist paths kept (facet folder) |
| `POLICY_HANDCRAFTED_FACETS` | kept `🔧️op`; added `🧬️mutations` |
| `.ops` op-log scanner | kept structurally; messages say mutation-text / op grammar (not document `Operation` type) |

### 2. TS facade allowlist → structural rule

- Removed ~260-entry `POLICY_TS_FACADE_ALLOWLIST`.
- Added `POLICY_TS_FACADE_CONSTITUTIONAL_FACETS` (`🗣️dsl`, `🔧️op`, `🔺️diff`, `🎒️pack`, `📡️spr`, `🧬️mutations`, `⚙️engine`).
- `policyTsFacadeBreaches` accepts WASM scaffold stubs under those facets and under `🧬️mutations/<mut>/{🦠️mutation,🔺️diff,↩️inverse}`; breaches only misplaced stubs.
- Observed: **0** TS-facade breaches (all current stubs are constitutional).

### 3. New scanners (wired into `export const policy`)

| Scanner | Kind | Result on this tree |
|---------|------|---------------------|
| Mutation triad completeness | `mutation-migration/triad-completeness` | **54** — all artifacts missing `🧬️mutations/` |
| `impl Mutation` presence | `mutation-migration/impl-presence` | **0** (no mutation leaves yet) |
| ArtifactEngine / `⚙️engine` | `mutation-migration/artifact-engine` | **54** — 3 folder missing, 51 impl missing |
| Grammar `start mutation` | `mutation-migration/op-grammar-start` | **52** — all still `start operation` |
| Specific-emoji uniqueness | `mutation-migration/emoji-uniqueness` | **0** (no mutation dirs yet) |
| Dispatch coverage | placeholder | **0** (intentionally empty until Wave 3) |

Aggregator: `breaches.push(...policyMutationArtifactEngineBreaches(repoRoot))`.

### 4. Collateral fix

Inserting the new region used a global replace of the PolicyExport region marker, which collided with a `policyDeclaredUseBreaches` RegExp string that already embedded that marker. Restored the RegExp replacement to the standard `\\$&` form.

## Policy run

- `bun ./📜️script.ts policy` → exit **1** (expected: `runPolicyExit` exits on any `priority: "high"` breach; no summary printed).
- Full inventory via `runPolicyScript`: **752** breaches total (669 high); Wave 2b kinds above.
- Log: `🧪wave2b-policy.txt`.

## Gate readiness

Wave 2b policy machinery is in place and reports unmigrated artifacts. Wave 3 (lowpoly pilot) should clear lowpoly from triad / engine-impl / `start mutation` scanners; fan-out shrinks the rest.

## Files touched

- `📜️script.ts` (policy regions)
- Ticket: `🧪wave2b-policy.txt`, `🧪wave2b-report.md`
