# Direct Rustc State Root Review

2026-10-16 00:49 UTC. Read-only review of the active Storage Source epoch; no compiler, image, allocation, cleanup or capacity operation ran. These findings were sent to the implementing agent before its freeze.

## Required Corrections

1. The volume opener currently finds a parent allocation by owner label. When no durable Binding or Intent exists and the backing directory is absent, it recovers that candidate and authors a fresh intent. That adopts interrupted state by owner name. An existing unbound candidate must retain/refuse; a crash between acquiring the charge and persisting its intent does not confer new custody to a subsequent invocation.
2. The new state owner/model/layout/inventory/volume import and use mutable shared policy DATA. Copy/freeze the primitive private policy and bind an actual public DATA mutation RED before qualifying slot counts, transport limits, allocation geometry or operation ceilings.
3. Check the malformed-catalog reset branch against the reviewed contract. Exact private allocation/backing and actual image EX are necessary for any reset; malformed or ambiguous Binding/Intent cannot authorize one. The implementation must distinguish a genuinely authenticated owned transaction from unknown contents and record its refusal/reset decision in the neutral corpus.

## Runtime Boundary

Initial creation still requires a genuinely additional4GiB charge plus the unchanged4GiB floor, so the earlier approximately4.3GiB free observation cannot fund it. No new physical measurement or reservation is asserted. The current root-global writer remains exclusive; eight warm slots do not qualify parallel producers. Actual interrupted transaction, complete compiler write containment and cold/warm/edit/restore behavior remain runtime gates.

## Reviewed Moving Source

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/💾️volume/🟦️.ts` — `73f6dcc077893b6a658e4840bdfb7b9e0cb721e74f601a9ba0d76fd7f8451266`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🟦️.ts` — `5e885f2319f91431adf504b34a68915ccf4bb515e2fd951c70fc4b57eaf7741f`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🔣️policy.json` — `fc181c69584948a8517375a05ed5a70a7b65136a68f25b01505e4d1f37468efc`
