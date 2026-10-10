# Current Carrier and CSV Cause Audit

Read-only source audit; no builds or tests executed. Concurrent Root edits were already visible at inspection.

## Actor fixture consistency

The retained-turn policy fixture, schema constants, Source assertions, docstring and DEBUG wording now agree on `Budget.retained` and `retainedAndScheduling`. A framework-wide exact search found no remaining `TurnGrant.retained`, `schedulingOnly` or the old independent-carrier DEBUG phrase in Rust, TypeScript or JSON. The wire fixture remains 64 + 34 + 8 + 2 + 1 = 109 bytes.

Owning paths beneath `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/`:

- `📃️policy/🧫️fixtures/🔣️.json`, `📃️policy/🧬️schema/🔣️.json`, `📃️policy/🧪️tests/🟦️.ts`: the only inspected consumers of the two descriptive carrier strings.
- `🧪️tests/🦀️.rs::receiving_input` and `original_actor_lane_policy_preserves_supplied_retained_input_without_heap_or_renewal`: consume `input`; the latter uses `turn.budget.retained` and round-trips the current TurnGrant.
- `🫴️receiving/🧪️tests/🟦️.ts`: consumes `policy.input.grant` and guards current sole Budget carrier.
- `🫴️receiving/🧪️tests/📬️public/🦀️.rs::original` and `original_public_actor_turn_pack_preserves_independent_input_and_full_grant_without_heap`: consume input and the 109-byte wire total; current assertions use `budget.retained`.

Host fixture-dependent owning modules under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/` are `🧪️tests/🔬️unit/🦀️.rs`, `🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs`, and `🧵️executor/🧪️tests/🔬️unit/🦀️.rs`. Their original-input constructors deserialize only `input`; shard wire law constructs `Budget { retained: original, ..budget }`. Other search matches in OS access-policy and Host wake refer to their own policy fixtures, not this Actor fixture.

## CSV exact native cause

Owning file: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`, current continuation law at lines 240–243 and helper at 214–224.

The binder captures its actual refusal message pointer and Borrowed Cow status. The final returned cause compares that same pointer after generic Record owner promotion and helper cleanup, and separately checks Canceled kind, exact neutral message and borrowedLiteral storage. This is the right same-error-path assertion. `ValueError::with_retained_progress` only replaces progress and preserves its Cow; `under` would allocate a new formatted message but is not this path.

The canceled funded-close refusal separately checks kind, message contents and Borrowed Cow; it does not compare its pointer to the binder cause. This correctly avoids claiming identity between distinct close and body errors.

The continuation receives the same original forwarded NativeControl after owner drop. It verifies canceled state, retained recipient presence and unchanged owned bytes on funded-close refusal, then rearms the captured original observer Cells. The helper subsequently funds close steps and requires terminal recipient removal before returning the original cause. Zero-grant denial precedes continuation and preserves recipient presence. The observer closure remains borrowed by NativeControl throughout; Cells provide shared state without replacement callback/controller.

These are source-level assertions, not Native qualification. The retained schema pointer currently proves a nonzero typed partial; it does not independently prove typed backing identity after owner erasure. No aggregate heap measurement is introduced by this law.
