# Flow Mutation Retirement 44

## Current Root Review

The current root source/neutral checkpoint is79/79 in `🧪️flow-mutation-retirement-44/🧫️run-1weRqh`. The earlier source checks are retained below as historical evidence, not native acceptance. No Rust test in this packet has been compiled or executed.

Root corrected the handoff model: transferring the outer mutation does not complete its nested retirement. Four byte-owner vectors incorrectly expected completion on the first inner close. The corrected neutral model covers only refusal, handoff and delegation; actual terminal completion remains a native law, not a claim from an abstract owner boolean. Root's test-first run `🧫️run-0N2TD4` retained six failures: four incorrect neutral completion rows, the missing fixture disposal and the absent false-completion regression.

The native retained-fixture test now explicitly retires its nonempty dictionary/set fixture. Additional native assertions cover zero grants after handoff and rejection of an injected false Complete while the inner owner remains live. The injected error test exercises the actual wrapper after a real handoff; it does not claim any production FlowRetirement error was observed. Four native tests are authored and unexecuted.

The controller captures the actual retained fixture data and shared retirement source, preserves first-read hashes, and verifies the exact protected neighboring prefix and member-store suffix against the immutable released handoff. Source hashes for this checkpoint are in the retained result JSON.

## Scope

Implemented only the released Flow VCS mutation-retirement handoff. The protected adjacent Flow fixture and snapshot retirement implementations were not edited. Root captured the released boundary as prefix `2ebed791bf928d91aec265ab45151eab2609933b3313fc696a7217037d301e36`, suffix `2103d43202de038a34b61aea3a3e1accdacc2f6ffa5556dccc64176848a69483`, and authorized preimage `2a4a720fb602d627d0d7890feca54ff85e1b02cb7a9bcf162cf4e5bb52d7a116` in [mutation-retirement-handoff.json](../🧪️flow-vcs-direct-41/🧫️mutation-retirement-handoff.json).

`FlowMutationRetirement` remains the VCS-owned marker and `MemberStoreOwner<FlowMutation>` continues to construct it. Its frontier is now mounted at [retirement leaf](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs).

## Ownership Handoff

| Direct leaf | Retained owner |
| --- | --- |
| AddWidget | Widget |
| RemoveWidget, MoveWidget | Bytes(id) |
| ChangeWidget | Bytes(id), Widget |
| AddSynapse | Specs |
| RemoveSynapse, MoveSynapse | Bytes(id) |
| ChangeSynapse | Bytes(id), Specs |
| ChangeLayout | Layout |
| ReplaceFlowFixture | Fixture |

The frontier rejects a zero-item or zero-byte grant without taking the mutation. A positive grant transfers exactly one direct mutation into `FlowRetirement`, then delegates bounded byte-accounted close steps. It reports `Complete` only when both the original mutation slot and nested retirement are empty. There are no observed production `FlowRetirement::close_step` error branches; the fault vector and native test label their fault as injected ownership-propagation modelling only.

## Evidence

The schema-first neutral fixtures are [schema](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧬️schema/🧹️retirement/🧬️schema/🔣️.json) and [vectors](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧬️schema/🧹️retirement/🧪️fixtures/🔣️vectors.json). The durable Bun controller [script](../🧪️flow-mutation-retirement-44/📜️script.ts) validates them with Ajv 2020 and jsonc-parser, checks all ten owner paths and the mounted source contract, and rehashes every input plus itself after assertions.

The retained red run is [result](../🧪️flow-mutation-retirement-44/🧫️run-8Vqrkv/🔣️result.json): 50 assertions, 2 expected missing-helper/source-contract failures. The retained final green run is [result](../🧪️flow-mutation-retirement-44/🧫️run-azKVd7/🔣️result.json): 67 assertions, 0 failures, `nativeRustExecuted: false`.

Native Rust tests were authored at [retirement tests](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧬️schema/🧹️retirement/🧪️tests/🦀️.rs), but were not compiled or run: Cargo/rustc are root-owned and the Flow build remains held. They cover the ten actual direct variants, zero-grant refusal, bounded progression, terminal-empty completion, and an explicitly injected fault model that drains before drop.

## Green Checkpoint

Final controller input hashes from the retained green result:

- VCS component: `505e09501ad70dc42ea4fecf6b0f9f250d5ad733969edd40f8f912867778c979`
- Retirement helper: `e305f7e369d1cd365036a4b5ddf92e6c931f608a5dce8a94ab552a03fa40b4c2`
- Retirement schema: `2ad9e61f8f57b5f77c1b85843f9781ab701a098310d8bbeae60002ab26010737`
- Neutral vectors: `6d9e1c46123c6987a7168da22531a7aa73ade632bb92d7287bc559c0106ddad7`
- Native source test: `732f2b3eb7dee86e0d6e4b95310f6162bae694fbe75a3f6cfa65442c845182dd`
- Controller: `171c7b9f7f7f5e596f2a124a6074010fa702934dbfa1d74c404a866512a99a37`

## Mount-Boundary Correction

The first mount placed the helper declaration before `FlowMutationRetirement`, which expanded the protected prefix and correctly failed root's neighboring-retirement assertion. The declaration now follows the struct and precedes its implementation, entirely inside the released mutation-retirement interval. The current prefix hash from `struct FlowFixtureRetirement {` to `struct FlowMutationRetirement {` is exactly `2ebed791bf928d91aec265ab45151eab2609933b3313fc696a7217037d301e36`, matching the immutable handoff capture. The rerun retirement gate is [result](../🧪️flow-mutation-retirement-44/🧫️run-RWHBkV/🔣️result.json): 67/67 source-neutral assertions; root's retained actual-file gate is [result](../🧪️flow-vcs-direct-41/🧫️run-6fTr5L/🔣️result.json): 331/331. Both explicitly report native Rust tests not executed.

## Ownership Hardening

The untransferred mutation slot is now `ManuallyDrop<Option<FlowMutation>>`; its frontier has a terminal-empty `Drop` assertion, so a refused mutation cannot silently release through ordinary dropping. Native coverage now uses the retained ownership base containing a nonempty neuron `Dictionary` and output-preview `OrderedSet`. The injected-error test first performs a real mutation-to-frontier handoff, invokes the test-only inner-close seam with an injected error, asserts the outer owner remains nonterminal, then drains it normally. This is an injected test seam, not an observed `FlowRetirement` production error.

The controller now retains the first hash and rejects a changed reread, rejects paths outside the workspace or under `compose`, checks the workspace and every input ancestor for symlinks before reading, and captures the shared FlowRetirement source and immutable handoff JSON. The retained final controller run is [result](../🧪️flow-mutation-retirement-44/🧫️run-dUt5UE/🔣️result.json): 74/74, native Rust not executed. The companion root source gate rerun is [result](../🧪️flow-vcs-direct-41/🧫️run-a7kxZO/🔣️result.json): 331/331, native Rust not executed.
