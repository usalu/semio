# Wave 0.8 — Policy Scaffolding for Semantic Mutations (📜️script.ts)

## Scope

Added the policy-engine half of Wave 0 (the framework mechanism half — `SemanticDescriptor`/`MutationKind`/`SemanticMutation`/`#[derive(dsl_derive::Mutations)]` — was already landed by a prior pass and was left untouched here). Only `📜️script.ts` was edited; no `.rs` files, no plugin/facet files, no new files created.

All new code lives in the existing `//#region 🔧️PolicyRuleMutationArtifactEngines` region (script.ts, ~lines 5280–6047), right next to `policyMutationTriadCompletenessBreaches` / `policyMutationImplPresenceBreaches` / `policyArtifactEnginePresenceBreaches`, which it reuses (`policyReaddirSafe`, `policyWalkRelFiles`, `policyListMutationDirs`, `policyStripEmoji`, `POLICY_MUTATIONS_FACET`, `POLICY_RS_COMPONENT_LEAF_NAME`, `POLICY_TS_COMPONENT_LEAF`).

## What was added

1. **`policyFindAllMutationsDirs`** — new shared helper. Walks `✏️s` for every dir literally named `🧬️mutations`, at whatever depth it actually sits (the real taxonomy nests it under `🏅️standards/<slug>/🪆️subsets/<slug>/🧬️schema/🧬️mutations`, deeper than `policyListPluginArtifactDirs`'s `<artifact>/🧬️mutations` assumption used by the pre-existing `policyMutationTriadCompletenessBreaches`, which was left untouched). Found 106 real mutation-facet dirs today.

2. **`policySemanticVocabularyBreaches`** (new rule 1) + **`POLICY_SEMANTIC_VOCABULARY_ALLOWLIST`** (new shrink-only allowlist, 342 entries) — mirrors `POLICY_DIFF_ALGEBRA_ALLOWLIST`'s exact shape (stale-entry detection included: an allowlisted file that's already clean fires its own `"low"`-priority breach telling you to prune it). Scans every `.rs` file under a `🧬️mutations/` facet or a `🎮️commands/` app-command dir (346 dirs of the latter exist) for:
   - `SetSnapshot` / `NoMutation` / `CollectionMutation<`/`CollectionMutation::` — `"high"` priority, gated behind the allowlist. Seeded via `grep -rlE "SetSnapshot|NoMutation|CollectionMutation(<|::)" ✏️s --include="*.rs" | grep -E "🧬️mutations|🎮️commands"` at ticket time — exactly 342 files, all listed in the allowlist.
   - Bare `Set[A-Z]\w*` dispatch-variant identifiers (excluding `SetSnapshot`, already counted above) — `"medium"`, deliberately **not** allowlisted (medium never gates `bun policy`, and 562-ish instances across ~480+266 files made per-file seeding pure overhead for a non-blocking signal).

3. **`policyMutationDispatchCoverageBreaches`** (real implementation, replacing the old `return []` Wave-3 placeholder) — for every `🧬️mutations/🦀️component.rs` dispatch file, extracts `pub enum \w*Mutation\w* { … }` variant names (brace-depth scan, not just regex, so struct-literal payload types inside `<>` don't confuse it) and diffs them against the triad-dir stems beside it (kebab → PascalCase via new `policyKebabToPascal`, minus emoji via `policyStripEmoji`). Kept at `"medium"` per the task's explicit instruction — not `"high"` — because **zero** facets have adopted the 1:1 variant-per-triad-dir shape yet; the docstring states the graduation condition (fan-out wave lands real triad wiring → flip to `"high"`), mirroring `policyMutationImplPresenceBreaches`'s own "advisory while Wave 3 pilot lands" wording. No allowlist needed since it doesn't gate anything today.

4. **`policyMutationTsMirrorBreaches`** (new rule 3) — flags any `.ts` leaf under a `🧬️mutations/` facet that is a trivial `export {};`/empty stub. Kept `"low"`, unallowlisted (1222 hits today — allowlisting all of them would have been pure overhead for a non-gating signal).

5. **`policyMutationArtifactEngineBreaches`** (existing aggregator) — extended with the three calls above.

## Left untouched (per explicit scope)

- `POLICY_DIFF_ALGEBRA` / `POLICY_DIFF_ALGEBRA_ALLOWLIST` — stdio-only scope unchanged.
- `policyArtifactEnginePresenceBreaches` — confirmed via `grep -rn "trait ArtifactEngine" --include="*.rs"` that no such trait exists anywhere in the repo today, but since fixing/removing that rule wasn't in this wave's explicit item list (1–3), it was left exactly as-is.
- `policyMutationTriadCompletenessBreaches` / `policyMutationImplPresenceBreaches` — pre-existing, unrelated bug noted but not fixed: they check `<artifact>/🧬️mutations` directly, which doesn't match the real (deeper) taxonomy, so they currently fire 91 `"high"` breaches each (`mutation-facet-missing`, `artifact-engine-folder-missing`) against every artifact. Confirmed pre-existing (unchanged breach count before/after this wave's edits) and out of scope — flagging here for whoever owns those two rules next.

## Verification

- `bunx tsc --noEmit -p tsconfig.json` — zero errors attributable to `📜️script.ts` (pre-existing unrelated errors remain in other files mid-refactor by other sessions: `✏️s/🔌️plugins/🔱️trinity/…/🟦️component.ts`, two stdio schema `🟦️component.ts` files, `…vscode/…/🟦️extension.ts`).
- `bun ./📜️script.ts policy` (repo root) — runs to completion, no crash. New rule breach tally: `mutation-migration/semantic-vocabulary` → 0 high / 585 medium / 0 low(stale); `mutation-migration/dispatch-coverage` → 105 medium; `mutation-migration/ts-mirror` → 1222 low. **Zero new `"high"`-priority breaches introduced.** The command's exit code is 1 both before and after this wave's edit — entirely due to pre-existing, unrelated `"high"` breaches (`os-state-authority/*`, `budget/no-budget-null`, `handcrafted-grammar/spec-distinctness` ~19340, `taxonomy/*`, etc., 21609 high-priority total, same order of magnitude before/after). This command was already red at baseline before any of this wave's edits.
- `bun ./📜️script.ts verify gate` (the nx `verify-gate` target) — read the full `runGate()` method (script.ts lines 669–773) end to end: it does **not** call `policyMutationArtifactEngineBreaches` or the root `policy` export at all (confirmed via `grep -n "policyMutationArtifactEngineBreaches" 📜️script.ts` → single call site, the `policy` aggregator at the old line ~10746, never inside `VerifyScript`). Ran it once before any edits and once after: **both runs fail identically**, at the very first step (`[verify] dependency-cruiser boundaries…`), with 764 pre-existing `no-circular` violations in `🧰️framework` TypeScript glue files — unrelated to mutations/policy work, almost certainly a concurrent session's in-progress refactor (this repo is live and multi-session per CLAUDE.md). This wave's edits do not touch any file `verify gate` inspects and cannot have caused or worsened that failure.

## Allowlist maintenance note for later waves

`POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` is shrink-only: as the fan-out wave hand-crafts a facet's mutations (removing `SetSnapshot`/`NoMutation`/`CollectionMutation`), remove that file's entry. Leaving a now-clean entry in the allowlist produces a `"low"` stale-entry breach (same UX as `POLICY_DIFF_ALGEBRA_ALLOWLIST`) as a nudge, but never blocks the gate.
