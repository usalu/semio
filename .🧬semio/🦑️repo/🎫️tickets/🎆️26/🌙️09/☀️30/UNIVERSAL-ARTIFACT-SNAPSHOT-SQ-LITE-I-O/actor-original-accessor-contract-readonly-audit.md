# Actor Original Accessor Contract Audit

Read-only source audit; no builds or Source runs. Parent reports TXT Source24363 terminal12passed/0failed/182expectations; owning Native7620 in progress. This audit does not alter that corpus.

## Actual semantic seam and current mismatches

`🧰️framework/🔨️modules/🎭️actor/🦀️.rs:4034` defines `TurnGrant::original_input(&self)->&RetainedTurnInput`, currently borrowing `self.budget.retained`. This is the genuine public semantic seam. Budget::scaled preserves its retained input. Kernel::retained_turn returns original scheduler authority; Kernel::complete validates exact issued receipt before replacing remaining grant and recording metrics; checked epoch advancement belongs to drain_turn/can_issue.

Current policy fixture/schema nevertheless require TurnGrant.retained/pureScheduling, while production presently has sole Budget.retained. Policy Source explicitly requires absent Budget retained and present TurnGrant retained. Receiving Source similarly pins entry.retained; checkpoint Source requires self.retained.validate and absent Budget retained. These are stale physical-layout requirements. Replace these single-contract declarations coherently; do not accept either old string or either field path as alternatives.

Some Native source consumers are also inconsistent now: retained-turn/🧪️tests/🦀️.rs invalid-original-wire test constructs TurnGrant{retained:input} and calls one-argument budget_for, while current production has no TurnGrant retained field and requires budget_for(lane,input). Other public Native laws use current budget.retained. This is a real source consistency issue, not grounds for a production carrier flip.

## Stable schema-first contract

Author one closed semantic contract: authorityAccessor=`TurnGrant.original_input`, authorityStorage=`borrowedOriginal`, schedulingOverride=`preservesOriginalAuthority`, returnAuthority=`issuedReceipt.remaining`, epochAuthorizer=`scheduler.nonempty-mailbox-admission.checked-next-epoch`. Keep exact original3identity/five currencies, four lanes and wire109 as independently authored observations. Reject forged accessor/override/return/epoch contracts through strict Ajv. Remove carrier-field and pureScheduling strings instead of introducing dual layouts or compatibility fallbacks.

Independent SQLite law applies authored scheduling changes to only fuel/wall/effect resource columns and verifies original identity/grant columns remain identical. A second original-return table proves four spent currencies subtract from exact issued grant, depth unchanged, and next issued input preserves operation/generation with checked epoch+1. Include zero spend, overflow and empty admission from existing neutral fixtures. These prove language-neutral semantics, not execution of Rust.

Source proof should locate the exact public original_input signature and verify it returns a borrowed reference to original RetainedTurnInput without allocation, cloning, derived limits or invented identity. Do not assert the field spelling of its return expression. Parse/extract the accessor's current Rust body and resolve its concrete field to its declared RetainedTurnInput type; this is one current source graph, not two accepted layout strings. Check actual native caller law mounts use this accessor, wire producers validate accessor authority before first output mutation, and scheduling/return call sites remain registered. Source body-order guards may support admission ordering but should be explicitly described as source obligations, not behavioral proof.

Native behavioral proof is essential: create current genuine issued turn, borrow original_input, apply scaling/resource override and verify full input and pointer still original; round-trip authentic wire; return spent receipt via Kernel::complete and compare Kernel::retained_turn then next issued original_input. Source changes alone cannot claim this Native behavior passed. Existing field-based Native tests can be updated to use the one accessor without defining a second authority.

## Resource override and wire caution

Host activation `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs::tick_and_dispatch` and renderer engine runtime equivalent accept `Fn(ActorId,Budget)->Budget` and call budget_for(grant.actor,grant.budget). An arbitrary override can replace Budget.retained; actual shard `pack_encode_grant` validates/encodes grant.original_input separately then encodes supplied budget. Current shard source has an additional retained field and encodes retained separately although current Budget::pack_encode itself also writes retained. Thus Source's CPU-only override witness does not prove arbitrary resource overrides preserve authority, or that shard grant wire contains one authority. Audit that actual native path independently before claiming semantic override conservation; do not weaken schema to authorize duplicated/divergent authority.
