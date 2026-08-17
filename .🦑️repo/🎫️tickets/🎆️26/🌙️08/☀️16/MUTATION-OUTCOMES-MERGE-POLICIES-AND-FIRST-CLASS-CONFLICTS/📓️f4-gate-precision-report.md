# Lane F4 — gate precision + tombstone-comment cleanup

## Before (`🧪️f4-before.txt`)
message-code **2**, no-crdt-vocabulary **9**, merge-policy-parity **1**, outcome **170**. Total 182.

## After (`🧪️f4-final.txt`)
message-code **0**, no-crdt-vocabulary **0**, merge-policy-parity **0**. All three targets met.
outcome **92** (sibling lanes' territory — dropped from 170 while I worked; not touched).

## A — message-code gate false positive (📜️script.ts, `policyMutationMessageCodeBreaches`)
Split the old single regex (any `.info(`/`.warn(` anywhere) into: (1) definite builders
(`MutationOutcome::error/fatal`, `MutationMessage::info/warn/error/fatal`) checked file-wide, and
(2) the chainable `.info(..)`/`.warn(..)` shorthand, now checked only inside a `fn` body
(`policyExtractFnBody`, scanning from each `POLICY_FN_DECL_RE` match) that itself contains the text
`MutationOutcome`. The animate plugin's `console.warn(...)` (JS string literal embedded in a Rust
file, zero `MutationOutcome` occurrences in that whole file) no longer matches. Verified with a probe
repo (`bun -e` against `policyMutationMessageCodeBreaches`): a real `outcome.warn("mutation.bogus-code",
..)` inside a `MutationOutcome`-returning fn still flags; a bare `console.warn(...)` in an unrelated fn
does not.

## B — merge-policy-parity gate false positive (`policyMergePolicyParityBreaches`)
Added `POLICY_MERGE_POLICY_VARIANT_SPELLINGS`: Rust surfaces still require the exact PascalCase
(`LaissezFaire`/`Normal`/`Vigilant`); TS surfaces now accept either that PascalCase mirror or the
camelCase key TS idiomatically uses (`laissezFaire`/`normal`/`vigilant`). The i18n bundle
(`🖱️ui/…/📦️index.tsx`) already had all 3 as `laissezFaire`/`normal`/`vigilant` keys at lines
2439/2443/2447 (de) and 3219/3223/3227 (en) — confirmed by direct read before touching the gate.
Probed both directions with a throwaway temp repo (`bun -e`, deleted after): a surface genuinely
missing `Vigilant` still reports a breach; a surface with only camelCase spellings reports zero.

## C — gltf allowlist extended to the message-code gate
`POLICY_MUTATION_GLTF_ROOT`'s doc comment + allowlist entry (b) doc comment now note it exempts both
`policyMutationOutcomeBreaches` (rule 1, pre-existing) and `policyMutationMessageCodeBreaches` (rule 2,
new). `policyMutationMessageCodeBreaches` now skips any relPath under the gltf root the same way rule 1
does, covering `🧭️mutation-dispatch/🦀️component.rs:262`'s `"mutation.rejected"` — that file's own
separate `GltfTopLevelMutationRejection` vocabulary, not a §C2 breach.

## D — 9 CRDT-vocabulary tombstone comments rewritten (real cleanup, no gate change)
Every hit named a deleted type (`MergeStrategyKind`, `merge_concurrent_diffs`, `merge_strategy`,
`ConflictRule`, `ResolutionPlan`) purely to say it was gone. Rewrote each to describe the *current*
mechanism (`MergePolicy` / `📡️spr/⚔️conflict`) without naming the dead type, keeping the ticket
reference where useful, keeping every doc comment's leading emoji:
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:653-656`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs:410-412`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:62-64` and `:96-97`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs:145-149`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚔️conflict/🦀️component.rs:18-20` and `:284-286`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs:255-256`

## Build verification
`cargo check -p semio-framework-os-kernel -p semio-framework-os-kernel-db` → `Finished \`dev\` profile`
(0 errors; 62 pre-existing unrelated warnings, e.g. "unnecessary qualification" in db_artifact/db_engine,
none touching the edited comment regions). Log: `🧪️f4-cargo-check.txt`.

## Files touched
- `/Users/ueli/Documents/semio/📜️script.ts` — `policyMutationMessageCodeBreaches` rewritten (builder
  vs. fn-scoped chain split), `POLICY_MERGE_POLICY_VARIANT_SPELLINGS` added +
  `policyMergePolicyParityBreaches` updated, `POLICY_MUTATION_GLTF_ROOT`/allowlist doc comments extended.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚔️conflict/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs`
- Logs: `🧪️f4-before.txt`, `🧪️f4-after.txt`, `🧪️f4-final.txt`, `🧪️f4-cargo-check.txt` (this folder).
