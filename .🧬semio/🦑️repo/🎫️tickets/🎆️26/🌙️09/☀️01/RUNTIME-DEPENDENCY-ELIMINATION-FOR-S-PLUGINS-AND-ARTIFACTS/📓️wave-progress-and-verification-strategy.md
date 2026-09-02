# 📊️ Conversion wave progress + why verification moved to the centre

## Measured progress (production serde refs, stripper method)
| plugin | before | now |
|---|---|---|
| 📕️norm | 1595 | 3 |
| 🧱️block | 695 | 29 |
| 📐️cad | 422 | 56 |
| 🌀️procedural | 458 | 226 |
| 🌍️gis | 283 | 152 |
| 📏️layout | 280 | 105 |
| 💠️lowpoly | 269 | 127 |
| 📸️remodel | 293 | 157 |
| 🧩️puzzle | 1716 | 1148 |
| 🗄️stdio | 582 | 550 |
| 🔋️energy | 364 | 339 |
| 🗒️note | 246 | 246 (not started) |

## ⚠️ Unresolved asymmetry — needs a compile, not a count
📕️norm: 1084 files, +2189/−2902, with **−1141 `Serialize` lines but only +651 `ToValue` lines**.
🧱️block: 271 files, +1328/−1293, −350 vs +223.
Part of the gap is legitimate (a removed `use serde::{Deserialize, Serialize};` needs no matching
add when the derives arrive via the os-kernel re-export). But the gap is large enough that it could
also be derives dropped with no replacement. Counting cannot settle it — only the compiler can.
Reassuring signal: oracle tests survived (norm still has 505 files referencing `serde_json`,
block 130), so the "delete the oracle to improve the scoreboard" failure mode did NOT occur.

## 🔁️ Verification strategy change
"FOREGROUND builds only" turned out to be UNACHIEVABLE for subagents, and this was not
disobedience: under this contention a workspace build exceeds the Bash tool's 10-minute cap, so the
tool auto-backgrounds it, and the agent's background child then dies when its turn ends. Four agents
lost their verification this way (space ×2, vcs, stdio, store).

13 agents each launching a full build is also self-defeating — they serialize on the same cargo lock
and sccache regardless, so parallel verification buys nothing over sequential.

**New rule for this ticket: agents do EDITS ONLY; the coordinator runs one central sequential
verification pass.** Central runs survive turn boundaries because the main session owns them.

## Reminder that still applies
A queued/lock-blocked `cargo check` compiles the tree as of its START time — its errors can describe
source that no longer exists. Always note when a check was launched relative to the edits it judges.

## 🔬️ The remaining work is TWO kinds, not one
Plugin production serde: **10,712 → 6,049**. Split by kind:
    serde_json:: value usage   1,906   ← the hard half: needs `DslValue`
    #[serde(…)] attributes     1,937   ← mechanical
    derive(… Serialize …)      1,394   ← mechanical
    use serde::                  736   ← mechanical

**Converting derives alone does NOT reach the goal.** 🌀️procedural is the proof: its derives are
100% converted, yet ~226 production refs remain and every one is `serde_json::Value` usage —
`use serde_json::{json, Value}`, `serde_json::Map::new()`, `serde_json::Value::Object(…)`,
`serde_json::to_string(&…)`. serde_json stays linked into the shipped component regardless of derives.

Worse, the natural repair pattern makes it permanent: an agent "bridged" call sites through
`DslValue <-> serde_json::Value`, which SATISFIES the compiler while PRESERVING the dependency. Any
agent told only to fix derives will reach for that bridge. Future waves must be told explicitly that
bridging through `serde_json::Value` is not a fix.

This is the same defect class as the queued `pack_rt` bridge wave — the first-party `DslValue` must
replace `serde_json::Value` in production signatures, not wrap it.

## 🧹️ Fleet hygiene
Agents that park waiting on a build are stopped once their edits are confirmed at source level;
their build result is redundant with the central pass. Stopped so far: space(×2), store, vcs.
