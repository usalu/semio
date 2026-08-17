# W5 — Shrink-Only Ratchet Report

Landed in `📜️script.ts`, region `//#region 🔧️PolicyRuleArtifactsOnlyPluginArchitecture` → new subregion `//#region 🔧️PolicyRuleApaRatchet` (right before the outer region's `//#endregion`). Also collapsed the two five-line wiring blocks (`VerifyScript.runGate`'s `dissolveBreaches` array and the `policy` export aggregator) into a single `...policyApaBreaches(...)` call each, so census and gate can never see divergent priorities for the same breach.

## Mid-task correction from the coordinator — applied before finalizing

The original brief asked for a ceiling on all eight APA breach kinds. Mid-task the coordinator reported a peer session's warning: 9 agents are actively dissolving artifact `⚙️engine/` directories and relocating `register*()` calls out of them right now, and a move transiently **increases** count (code briefly exists in both the old and new location, imports half-updated). A shrink-only ceiling recorded at a trough would gate the shared tree on the peer's own in-flight progress a minute later. Directive: withhold the ceiling on `plugin-registration-engine-backlog` outright, and apply the same exemption to any other row my own measurement showed moving.

**I did the cheap test the coordinator suggested — three `bun ./📜️script.ts policy` runs, ~90-100s apart — rather than trusting the brief's numbers or my own first run.**

## Three-run drift measurement (2026-08-13, ~00:12 / 00:14 / 00:16)

| ratchet key | brief (coordinator, "minutes ago") | run1 00:12 | run2 00:14 | run3 00:16 | verdict |
|---|---:|---:|---:|---:|---|
| `plugin-closed-shape` | 41 | 41 | 41 | 41 | **flat — safe to ceiling** |
| `plugin-dependency-allowlist` | 105 | 105 | 105 | 105 | **flat — safe to ceiling** |
| `plugin-dependency-os-host` | 10 | 10 | 10 | 10 | **flat — safe to ceiling** |
| `effect-capability-parity` | 47 | 47 | 47 | 47 | **flat — safe to ceiling** |
| `plugin-purity` (all sub-kinds summed) | 116 | 116 | 118 | 125 | **moving — exempted** (filesystem-io alone: 35→35→42; interior-mutability-mutex: 20→22→22) |
| `plugin-registration-engine-backlog` | 372 | 359 | 359 | 359 | **moving — exempted per coordinator's explicit directive** (already −13 vs. the brief before I took my first measurement) |
| `plugin-registration-violation` | 562 | 570 | 580 | 600 | **moving — exempted** (+30 in ~4 minutes, the same dual-write window reaching the sibling kind) |
| `plugin-registration-setup-callback` | 14 | 14 | 14 | 15 | **moving — exempted** (small but real; the same window `📓️status.md`'s "FINAL STATE" section describes the `.setup()` 33→11 conversion landing in) |

**Discrepancy vs. the brief, stated explicitly per the ticket's instruction:** four rows (closed-shape, dependency-allowlist, dependency-os-host, effect-capability-parity) match the brief exactly across all three of my runs — genuinely settled. The other four differ from the brief and from each other run-to-run — `engine-backlog` had already dropped 372→359 before I ever measured it, `violation` climbed monotonically the entire time I watched it (562→570→580→600), `purity` climbed too despite being designed as inventory-only (116→118→125), and `setup-callback` ticked up by one. None of this is measurement error; `git log --oneline -5` during this window showed a live commit (flag 498) landing in `✏️s/🔌️plugins`, and `git diff --stat` at report time shows 257 files with in-flight, uncommitted changes across exactly the artifact-engine-dissolution shape the coordinator described (dozens of `⚙️engine/🦀️component.rs` deletions paired with new `🚪️io/` and `🧬️schema/💡️inferences/` additions, e.g. under `🌀️procedural`, `🌿️vcs`, `🏗️fem`, `🏭️process`, `📏️layout`, `📕️norm`, `🖍️draw`, `🗒️note`, `🧩️puzzle`, `🪵️sourcing`) — none of it mine.

## Ratchet semantics (implemented)

- **Grouping key** (`policyApaRatchetKey`): every `taxonomy/plugin-purity-*` sub-kind collapses onto one shared `"plugin-purity"` key (matches the brief's "purity (all sub-kinds summed)" framing); every other APA kind keeps its own key 1:1.
- **Ceiling table** (`POLICY_APA_RATCHET_CEILINGS`, one constant, comment states "ceilings may be lowered freely as work lands; raising one requires a ticket citation"):
  ```
  plugin-closed-shape:          41
  plugin-dependency-allowlist:  105
  plugin-dependency-os-host:    10
  effect-capability-parity:     47
  ```
  The other four keys are deliberately **absent** from the table — absence means no ceiling, permanently `medium`, documented inline with the measured evidence above.
- **Application** (`policyApaRatchetApply`): groups the combined breach list by key; for a key present in the table, the first `ceiling` breaches (stable order) stay at their original `medium` priority and only the breaches *beyond* the ceiling — the measured excess — are promoted to `priority: "high"` with a message naming the key, the ceiling, the actual count, and "shrink-only — raising the ceiling requires a ticket citation." Keys absent from the table pass through untouched.
- **Single call site** (`policyApaBreaches`): both `VerifyScript.runGate`'s `dissolveBreaches` block and the `policy` export aggregator now call this one function instead of independently spreading the five raw rule outputs, so the ratchet is applied exactly once and identically in both paths.

## Verification 1 — `bun ./📜️script.ts policy` runs clean, zero APA kinds at `high`

Final run, 2026-08-13 00:24:05:
```
$ bun ./📜️script.ts policy
EXIT:1
```
Exit 1 is the pre-existing, unrelated `runPolicyExit` behavior (22077 `handcrafted-grammar/spec-distinctness` + 1028 `taxonomy/emoji-prefix` + others — same 25 high-priority rules as before this change, none of them APA's). Per-rule APA counts and priorities from the cache (`.🦑️repo/⚡️cache/breaches/compose.json`, top-level key is `breachs`, not `breaches`):
```
total apa breaches 1274, priorities Counter({'medium': 1274})
effect-capability-parity 47
plugin-closed-shape 41
plugin-dependency-allowlist 105
plugin-dependency-os-host 10
plugin-purity 123
plugin-registration-engine-backlog 294
plugin-registration-setup-callback 15
plugin-registration-violation 639
```
**Zero of 1274 APA breaches at `high`.** The four ceilinged kinds (closed-shape, dependency-allowlist, dependency-os-host, effect-capability-parity) are all still exactly at their ceiling, so nothing overflows; the four exempted kinds have no ceiling regardless of how far they've moved since the last measurement (engine-backlog kept falling to 294, violation kept climbing to 639 — both still `medium`, correctly unaffected).

## Verification 2 — the ratchet fires (fire-test, then restored)

Temporarily set `"plugin-closed-shape": 5` in place of `41` (a scratch in-memory edit to `📜️script.ts`, restored immediately after — never committed as a real value):
```
$ bun ./📜️script.ts policy
EXIT:1
...
taxonomy/plugin-closed-shape  ✏️s/🔌️plugins/🌊️flow  RATCHET REGRESSION on "plugin-closed-shape": ceiling is 5, measured 41 — shrink-only, raising the ceiling requires a ticket citation — "✏️s/🔌️plugins/🌊️flow/🧩️extensions" is a plugin-root entry outside the closed apps+artifacts shape
taxonomy/plugin-closed-shape  ✏️s/🔌️plugins/🏭️process  RATCHET REGRESSION on "plugin-closed-shape": ceiling is 5, measured 41 — shrink-only, raising the ceiling requires a ticket citation — "✏️s/🔌️plugins/🏭️process/🛂️manifest" is a plugin-root entry outside the closed apps+artifacts shape
```
Cache confirms the exact split: `taxonomy/plugin-closed-shape` → **5 medium + 36 high = 41 total** (the first 5 breaches stayed at the ceiling's priority, the remaining 36 — the excess over the 5-ceiling — were promoted). `dependency-cruiser`-independent gate tally also moved from 25 to 26 high-priority rule kinds during the fire test (24621 → 24656 high-priority breaches), confirming the promoted breaches reach the gate's filter.

Restored `"plugin-closed-shape": 41` immediately after. `diff` against the pre-fire-test copy of the file confirmed byte-identical restoration:
```
$ diff <pre-firetest-backup> 📜️script.ts && echo "RESTORED CLEAN — no diff vs pre-firetest"
RESTORED CLEAN — no diff vs pre-firetest
```
Re-ran policy after restoring:
```
$ bun ./📜️script.ts policy
24616 high-priority breach(es) across 25 rule(s):
```
Back to 25 rule kinds, 0 `RATCHET REGRESSION` lines, and the cache shows `total apa 1272, priorities Counter({'medium': 1272})` — all-medium again, confirming the restore.

## Verification 3 — `bun ./📜️script.ts verify gate` pass/fail unchanged

```
$ bun ./📜️script.ts verify gate
EXIT:1
```
**Fails at the exact same step, for a reason unrelated to APA, both before and after this change.** `runGate` calls `dependency-cruiser` as its literal first check (`📜️script.ts:712`, before any policy rule runs at all):
```
error no-circular: 🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts → 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts → ... → 🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts
x 769 dependency violations (620 errors, 149 warnings). 8702 modules, 5394 dependencies cruised.
error: bunx dependency-cruiser ... exited with status 108
    at runCmd (.../📦️index.ts:1390:31)
    at runGate (/Users/ueli/Documents/semio/📜️script.ts:713:5)
```
This is a pre-existing circular-dependency failure among `🧰️framework` modules (kernel/platform/manifest/assets/ui-styling), unrelated to `✏️s/🔌️plugins` and to anything this ticket touches. Proof the failure is identical, not merely similar:
1. `git show HEAD:📜️script.ts` lines 705-714 (the `dependency-cruiser` invocation, before my edit) are byte-identical to the current file at the same lines — I never touched anything above line ~789, where the `dissolveBreaches` block (my edited call site) lives.
2. `dependency-cruiser` scans `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand` — repo-root `📜️script.ts` itself is not in that scan, so nothing I changed in it can move this count.
3. `git diff --stat -- 📜️script.ts` shows my entire change is additive/localized (see below); `runGate` throws and returns before ever calling `policyApaBreaches` or any of the five underlying rule functions.

The gate was already red for this unrelated reason before my change landed, and remains red for the identical reason after — **no worse than before**, confirmed structurally and empirically.

## Verification 4 — scope discipline

```
$ git diff --stat -- 📜️script.ts
 📜️script.ts | 104 ++++++++++++++++++++++++++++++++++++++++++++++++++++++------
 1 file changed, 94 insertions(+), 10 deletions(-)
```
The 10 deletions are exactly the two five-line wiring blocks collapsed into one `...policyApaBreaches(...)` call each — confirmed by inspecting every `^-` line in the diff (11 lines incl. the diff header, all ten being the two spread-blocks). `git diff -- 📜️script.ts | grep -n "PolicyRuleMutationArtifactEngines"` returns nothing — that region, and every pre-existing allowlist constant, is untouched. `git diff --stat` (whole tree) shows 257 other files with in-flight changes from concurrent sessions (the artifact-engine-dissolution wave described above); none of them are mine or touched by me.

## Which rules are flat and why (per the brief's explicit ask)

- **`effect-capability-parity`** — flat 47/47/47/47 (brief + 3 runs). Needs the `.capability(ArtifactKind, …)` declaration mechanism (M5); today only 🪐️space declares any capability at all, and it doesn't even cover its own constructed effects. No W3/W4 work targets this.
- **`plugin-dependency-allowlist`** — flat 105/105/105/105. Needs the curated SDK re-export list for the non-os-host framework crates (`semio-framework-editor`, `semio-framework-math`, `semio-framework-ui`, `semio-framework-os-flow`, …) — unstarted; no wave has attempted it.
- **`plugin-dependency-os-host`** — flat 10/10/10/10. Same unstarted-SDK-surface blocker, os-host-crate-specific subset.
- **`plugin-closed-shape`** — flat 41/41/41/41. W3's plugin migration already burned this down from the original census's higher count; what remains needs the open cross-app-shared-code and extension-crate-axis rulings (`📓️w0-census.md` §6) before further reduction, and nobody is actively working it tonight.
- **`plugin-purity`** — *intended* to be flat by design (inventory-only; nothing in APA ever attempted to reduce it — Draft-lane facets that would replace `thread_local!`/interior-mutability idioms can't be authored until per-app verb sets clear SMO review). **Measured NOT flat tonight** (116→118→125→123): exempted on the measured evidence, not the design intent.
- **`plugin-registration-engine-backlog`**, **`plugin-registration-violation`**, **`plugin-registration-setup-callback`** — all three sit inside the same live wave: 9 peer agents dissolving `⚙️engine/` directories and relocating `register*()` calls, with `📓️status.md`'s own "FINAL STATE" section describing `ArtifactDeclaration` landing and `.setup()` hooks converting from 33→11 in the same window. Exempted per the coordinator's explicit directive plus my own three-run confirmation.

## Sanity requirement — gate no worse than before

Confirmed twice, independently: (a) the `policy` command's cache shows 0/1274 APA breaches at `high` in the final run, same as before this change (0/1727 in W2's original report, 0 in every intermediate measurement tonight); (b) `verify gate` fails at the identical pre-existing `dependency-cruiser` step, proven structurally identical via `git show HEAD` diff and empirically via matching error signature. The ratchet is live and armed (fire-test above) but nothing currently exceeds a set ceiling, so the gate's pass/fail state is unchanged by this landing.

## Files touched

- `/Users/ueli/Documents/semio/📜️script.ts` — only file touched. New subregion `//#region 🔧️PolicyRuleApaRatchet` inside the existing `//#region 🔧️PolicyRuleArtifactsOnlyPluginArchitecture`; two pre-existing five-line wiring blocks (inside my own region's prior wiring, not `PolicyRuleMutationArtifactEngines` or any pre-existing allowlist) collapsed to one-line calls.
- This report: `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w5-ratchet-report.md`.
- Scratch (not ticket-persisted, per protocol): `policy_before.txt`, `policy_run2.txt`, `policy_run3.txt`, `policy_after1.txt`, `policy_firetest.txt`, `policy_after_restore.txt`, `policy_final.txt`, `verify_gate_after.txt`, `measure1.txt`, `measure2.txt`, `measure3.txt`, `script.ts.pre-firetest.bak` under `/private/tmp/claude-501/-Users-ueli-Documents-semio/5128c8d3-abfa-49da-81ac-33286ba73278/scratchpad/`.
