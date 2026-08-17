# W4 Lane 4-A — repo gates + launch.json + nx

Region `🔧️PolicyRuleMutationOutcomeMergePolicy` added to `📜️script.ts` (between
`🔧️PolicyRuleMutationArtifactEngines` and `🔧️PolicyRuleInferenceFamily`), 7 exported gate functions +
1 internal aggregate `policyMutationOutcomeMergePolicyBreaches`, wired into:
- the big `policy` `defineLint` export (`bun policy` sweep),
- `VerifyScript.runGate()` (new "mutation-outcome / merge-policy law" block, hard-throws on any
  `priority: "high"` breach — `verify gate` fails honestly right now, see below),
- a new standalone `bun ./📜️script.ts verify mutation-outcome-law` subcommand,
- `📋️project.json` `mutation-outcome-law` target (next to `stdio-mutation-law`) + `verify-gate`'s
  `dependsOn: ["mutation-outcome-law"]`,
- `.vscode/launch.json`: 3 new `4_gate` entries (`⚖️gate🎯️mutation-outcome`,
  `⚖️gate🧹️no-crdt-vocabulary`, `⚖️gate🛡️merge-policy-parity`) + 1 new `3_dev` entry
  (`🛠️dev🖥️s⚛️react🛡️policy-vigilant`, port 6071, `SEMIO_MERGE_POLICY=vigilant`).

## Real breach counts (bun -e per gate, logs in this folder)

| Gate | Count | Log |
|---|---|---|
| `policyMutationOutcomeBreaches` | **170** | `🧪️w4-gates-mutation-outcome.txt` |
| `policyMutationMessageCodeBreaches` | **2** | `🧪️w4-gates-message-code.txt` |
| `policyNoCrdtVocabularyBreaches` | **9** | `🧪️w4-gates-no-crdt-vocabulary.txt` |
| `policyNoValidateOverrideBreaches` | **0** | `🧪️w4-gates-no-validate-override.txt` |
| `policySeverityInfoBreaches` | **0** | `🧪️w4-gates-severity-info.txt` |
| `policyMergePolicyParityBreaches` | **1** | `🧪️w4-gates-merge-policy-parity.txt` |
| `policyDeriveMirrorBreaches` | **0** | `🧪️w4-gates-derive-mirror.txt` |
| **Aggregate (`verify mutation-outcome-law`, CLI + nx target)** | **182** | `🧪️w4-gates-mutation-outcome-law-cli.txt`, `🧪️w4-gates-nx-target.txt` |

170 of the 182 are un-converted `🔺️diff` leaves (mostly missing a message code — real, un-fanned-out
mutation kinds owned by other lanes). The 2 message-code breaches are a plain-English fault string in
`🎞️animate` and a `"mutation.rejected"` literal in gltf's dispatch — neither is one of the 7 frozen codes.
The 9 CRDT-vocabulary hits are real (`🎠️kernel`, `🌿️vcs`, `📡️spr/🎮️command`, `📡️spr/🧾️wire`,
`🛢️db/⚔️conflict`, `🛢️db/📄️artifact` — C10 deletion not landed there yet). The 1 merge-policy-parity
breach is the i18n bundle (`🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`) — zero of the 3
`MergePolicy` variant names exist there yet (C9 i18n keys not landed). No-validate-override,
severity-info, and derive-mirror are genuinely clean today.

`bun ./📜️script.ts verify gate` fails at its very FIRST step (`bunx dependency-cruiser`, 828 pre-existing
circular-dependency violations) — unrelated to this lane, before it ever reaches the new block. Verified
the new block's logic directly via `bun ./📜️script.ts verify mutation-outcome-law` (exit 1, 182 breaches,
matches the sum above) and via `bun nx run workspace:mutation-outcome-law` (same 182, log
`🧪️w4-gates-nx-target.txt`) — both prove the wiring is correct; full `verify gate` log at
`🧪️w4-gates-verify-gate-full.txt`.

Not weakened: all 7 gates are `priority: "high"` and hard-throw. Breach counts above are real and
un-massaged — the coordinator should dispatch the remaining fan-out lanes.

## Bug found and fixed while building this

The repo root has a `script.ts -> 📜️script.ts` symlink (compat alias). A repo-wide `.ts` scan (needed for
rules 5/6, which check TS surfaces too) reads through the symlink to identical bytes, so the policy
region's own source — which necessarily contains the string `Severity::Hint` / `"hint"` as regex/doc
literals — self-matched via that second name. Fixed by excluding both `📜️script.ts` and `script.ts` in
`policyMutationLawSourceFiles`.

## Files touched

- `/Users/ueli/Documents/semio/📜️script.ts` — new region `🔧️PolicyRuleMutationOutcomeMergePolicy`
  (~340 lines, 7 exported gate fns + 1 internal aggregate), `policy` export extended, `VerifyScript`
  gets a `mutation-outcome-law` CLI branch + `runMutationOutcomeLaw()` + a `runGate()` block.
- `/Users/ueli/Documents/semio/📋️project.json` — new `mutation-outcome-law` target,
  `verify-gate.dependsOn` extended.
- `/Users/ueli/Documents/semio/.vscode/launch.json` — 3 new `4_gate` entries, 1 new `3_dev` entry.
- Logs/scratch: `🧪️w4-gates-*.txt` in this folder.
