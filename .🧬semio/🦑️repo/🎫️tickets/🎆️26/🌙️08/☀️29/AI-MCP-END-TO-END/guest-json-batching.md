# Guest JSON Input: control turns versus per-unit cost

## Result in one paragraph

On the current source the first Poll Input is already drained by a handful of substantive control turns, not one per unit. Natively, the 681-byte Note wire needs 81 outer `advance_step` turns at a wide grant (85 at 4,096 items per turn, 172-179 at 256), and only 5 of them carry real work: parse plus source admission (2,143 units in one turn), typed receive (6,132-6,340 units in one turn at a wide grant) and take (422 units). The other 74-76 turns are bookkeeping of 1-3 units (13 admit, 61 decoder cleanup). A native sampling profile puts 92% of the time in typed receiving (`NativeReceivingOriginal::advance` 81%, its `take` 12%), about 5% in decoder control, 0.6% in the parser and 0.7% in cleanup. Batching more control turns or removing parser waste therefore cannot move the 1.35 billion guest instructions; the cost sits in `🌱️value/🛬️decode/🧳️owned/**`, which is out of bounds here. I changed no production behaviour; phase 2 is not sound on this evidence (reasons below).

## Measurements (native, current source, prototype profile)

Harness: `🔌️plugin/⚛️reactor/🎟️context/🌉️call/📥️input/🧪️tests/📏️decode-turns/🦀️.rs` (crate `semio-plugin-actor-input`). It rebuilds a 681-byte Note InstanceOpen Poll from the checked-in `actor-input.json` fixture with the live effective limits (47,836,160 items, 2,949,668,352 copy, 1,946,157,056 capacity, 1,476,395,008 release, depth 64, work 979,000,000, fuel 1,000,000, deadline 86,415,461,134, 256/1,048,576/256) and pads the actor string to the exact length. The length is exact (asserted), the FNV-1a fingerprint `0e41700825f65343` is not reproduced because the live principal and session strings are unknown. It drives `advance_step` exactly like `advance_in_turn`, counting outer steps, work, fuel, receipts, clock reads and the phase before and after each step.

Before (state of the tree at 11:10, before another machine changed typed receiving at about 11:30):

| Grant (items per caller turn) | Caller turns | Outer `advance_step` turns | Work = fuel | Clock reads | Items | Copy bytes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 4,096 | 2 | 85 | 9,039 | 451 | 6,975 | 1,874,415 |
| 256 | 28 | 179 | 9,159 | 494 | 7,095 | 1,893,803 |
| 65,536 | 1 | 81 | 9,033 | 449 | 6,969 | 1,873,477 |

Latest run (after the other machine's typed-receive edit, my code unchanged): 4,096 items 85 turns, 8,831 work, 450 clock reads; 256 items 172 turns, 8,945 work, 492 reads; 65,536 items 81 turns, 8,825 work, 449 reads. The parse and source side is identical in both (2,143 units).

Phase split at the wide grant (work units, outer turns):

| Phase | Turns | Work | Note |
| --- | ---: | ---: | --- |
| admit (return table, binding, decoder source) | 14 | 24 | alternates 1-unit settle turn and 2-unit action turn |
| parse, bind, source, receiver construction (UTF-8 validation, strings, keys, grammar) | 1 | 2,143 | one turn |
| typed receive | 2 | 6,132 (6,340 before) | one turn does all work |
| take | 2 | 422 | |
| retirement of parser and decoder owners (`ready`) | 61 | 101 | alternating 1 and 2-3 unit turns |

Control turns that are not bookkeeping: 5. The measured-vs-live gap is large: live reported 3,158 clock imports and 21,590 units; natively 449-494 reads and 8.8-9.0k units. Either the captured component predates the loops now in `OriginalJsonOutputReceiving::advance` and `advance_in_turn`, or the live grants differ. This needs the component rebuild to decide.

Native profile (`sample`, 300 decodes of the wide-grant case, 1,603 samples on the decoding thread):

| Where | Samples | Share |
| --- | ---: | ---: |
| `OriginalJsonOutputSource::receive_step` (all of decoding) | 1,580 | 98.6% |
| `OriginalJsonOutputReceiving::advance_turn` | 1,497 | 93% |
| of which `NativeReceivingOriginal::advance` (typed receive) | 1,295 | 81% |
| of which `NativeReceivingOriginal` take (`enum_machine`) | 186 | 12% |
| decoder control outside `advance_turn` (`with_control`, frames, loans, capture) | about 83 | 5% |
| JSON parser (`step_owned_source`) | 10 | 0.6% |
| cleanup (`close_native_receiver`) | 11 | 0.7% |
| `_platform_memmove` (top of stack), almost all under `NativeReceiveSlot`, `machine_slot`, `NativeReceivingOriginal` | 435 plus 55 stub | 30% |

For whoever owns `🌱️value/🛬️decode/🧳️owned/**`: the typed receive of this wire is about 6,100 units at 81% of native time with struct copies (memmove) being the top-of-stack cost, so the per-unit instruction cost in the guest is probably dominated by moves of large receiver frames inside `NativeReceiveSlot::advance`, `machine_slot` and `enum_machine`. That is a per-unit constant, not a control-turn count.

## Why no production change

* Control turns: outer turns carrying real work are 5; the 76 bookkeeping turns are about 5% of native time. Fusing the settle turn with the next action (the only safe batching I found, halving them with the same yield check in between, so identical clock reads, work and receipts) would save at most 2-3% and touches the return-place protocol that the host laws pin.
* Parser waste: string double walk, validation per scalar and quadratic key comparison are real but total 0.6% of native time; they cannot be removed without changing recorded work units (measure and materialize are separate unit counts), which the task forbids.
* Typed receive: out of bounds.

## Tests added

* `🎒️pack/🔤️json/📥️decode/🧳️receiving/🫴️unique/🧪️tests/📏️work-equivalence/{🦀️.rs,🔣️.json}`: language-neutral fixture of 18 inputs (empty, whitespace, small valid, long and escaped long string, unicode and surrogates, 400 values, 48 unknown keys, shared-prefix keys, duplicate key, depth 15 and depth 40, trailing data, truncated, float and overflow in bytes, two invalid UTF-8 wires). Each row pins acceptance (also checked against `serde_json`), decoded value (against the `serde_json` oracle), refusal, parse and source work, total work, cleanup work and a control-turn ceiling (8 to 11 turns). 5 of 5 tests in the crate pass.
* `🔌️plugin/⚛️reactor/🎟️context/🌉️call/📥️input/🧪️tests/📏️decode-turns/{🦀️.rs,🔣️.json}`: the harness above as a law: parse and source work 2,168 at the wide grant, fuel equals work, ceilings on outer turns (100, 260, 100), caller turns and clock reads. `SEMIO_DECODE_REPORT=1` prints the per-phase table with `[DEBUG]` prefix; `SEMIO_DECODE_REPEAT=<n>` repeats the decode for sampling. Passes.
* Existing laws: unique crate 5/5. Input crate 5 of 7; two failures not caused by my edits (I added only accessors and test modules) and not verifiable on a pristine tree: `compact_job_source_matches_serde_and_retires_its_partial_and_complete_owners` (fixture row has `Job`, `ActorInput` now expects `Poll`, `Codec`, `Restore`) and `bounded_actor_input_admission_uses_multiple_paid_steps_before_returning` (assertion `checked_capacity`, line 83), both most likely from the concurrent typed-receive and `ActorInput` edits that also moved the typed work from 6,340 to 6,132.

## Observable numbers that moved

None by my change. The production source only gained two read-only accessors (`JsonGrammarCursor::validated_position`, `OriginalJsonOutputReceiving::grammar_phase`) that add no field, so `size_of` based receipts are unchanged. The typed-receive and total-work drift in the numbers above (9,039 to 8,831 at 4,096 items) comes from the other machine's edits.

## Changed files

* `🧰️framework/🔨️modules/🎒️pack/🔤️json/🧮️parser-and-grammar-cursor/🦀️.rs`: `validated_position`.
* `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🧳️receiving/🫴️unique/🦀️.rs`: `grammar_phase`.
* `.../🫴️unique/🧪️tests/🦀️.rs`: declares the new module.
* `.../🫴️unique/🧪️tests/📏️work-equivalence/🦀️.rs`, `🔣️.json` (new).
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📥️input/🦀️.rs`: declares `decode_turns`.
* `.../📥️input/🧪️tests/📏️decode-turns/🦀️.rs`, `🔣️.json` (new).
* `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml`: removed a duplicated member line (`💡️inferences/🧪️tests/⚖️laws`) that made every `repo cargo` run fail with "Invalid Cargo workspace member patterns"; not my file, restored nothing else.

## Build notes

`repo cargo ... -- test -p <crate>` fails with "requires dev-dependencies and is not a member of the workspace" for these framework crates; `-- test --manifest-path <crate>/📦️packages/🦀️rust/Cargo.toml --lib` works.

## Unverified

* The real guest instruction count: needs the component rebuild (25-40 minutes). Native sampling uses the prototype profile (opt-level 0), the guest uses a different optimisation level.
* Whether the captured component matches this source (3,158 live clock imports against 449-494 native).
* The exact live wire fingerprint (principal and session unknown).
