# 🗂️ LIVE PACKET ROSTER — sol's addressing table (keep current; misaddressing has cost twice)

| agent id | packet | path_scope | status |
|---|---|---|---|
| `a7a976cb202b6f5ad` | `stdio-green` | `✏️s/🔌️plugins/🗄️stdio/**` | running — 18,758 -> 12,077, round 3 |
| `afd72549c2470c9ea` | `oskernel-sync-features` | `🏪️store/🔄️sync/**`, `📇️directory/🔌️client` | running |
| `ac7e3aaeafb3ca029` | `fleet-extensions-green` | `📚️compiler`, `✏️s/🔨️modules`, fleet minus stdio | running — quiet 83 min, pinged |
| `a59ddbf8a072b1148` | `shellhost-tsgen` | ShellHost tsx, framework TS generator, 2 typecheck registrations | running |
| `a08c1e84c991cb708` | `sdk-test-compile` | `🔌️plugin/**` `#[cfg(test)]` only | running — 383 -> ~320 |
| `adab544d017299e66` | `ui-engine-green` | `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/**` | running — 682 errors |

## Completed
`ui-w4-core` · `scene-surface` · `react-w4` · `sdk-wire` (M1+M2) · `shard-lane` (M3 pieces 1-2) ·
`extension-activation` (M6 kernel+web) · `bench-budget5` (correctly reported UNMEASURABLE)

## ⚠️ ADDRESSING DISCIPLINE — sol has misaddressed messages TWICE
Both times sol tracked agent ids by dispatch ORDER and got them wrong when two packets were dispatched
in one message. Both times the wrong recipient noticed it did not match its scope, declined to act, and
said so — `stdio-green` first, then `sdk-test-compile`. **That pushback is the only thing that caught
it**, which means the failure mode is invisible unless executors are willing to contradict the
coordinator. Worth stating in briefs: *if an instruction does not match your scope, say so instead of
complying.*

**Rule for sol: re-read this table (or run `ListAgents`) before every `SendMessage`. Never infer an id
from the order two agents were spawned in.**
