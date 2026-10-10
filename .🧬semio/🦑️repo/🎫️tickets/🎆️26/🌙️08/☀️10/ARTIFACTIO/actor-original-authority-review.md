# Actor Original Authority Review

Fresh source review on 2026-10-10 supports carrying the caller's retained authority separately from scheduling Budget. No Actor source was edited for this review.

`framework/actor/retained-turn` already defines and encodes operation, generation, epoch, the five supplied grant axes, four spent axes, and conserved remaining axes. `RetainedTurnReceipt::validate_for(input)` checks exact input identity and conservation. This is the existing receipt contract to receive rather than synthesizing another one.

The actual activation, pinned activation, and reservation entry points receive `RetainedTurnInput`. Currently it is passed only into `lane_defaults::budget_for`; the four literals then refer to a removed Budget field. `ScheduledActor` and `TurnGrant` have no retained port. `Scheduler::drain_turn` scales the scheduling Budget and publishes a grant without retaining the original input. `Kernel::complete` processes metrics, failure state and status without validating `TurnResult.retained_receipt`.

The receiving repair therefore needs to retain the real input in scheduled/issued turn custody, publish it separately in `TurnGrant`, validate the returned receipt against that exact issued input before completion side effects, and conserve remaining currency. Another tick must not replay the original full allowance. Lane defaults can return the seven scheduling fields without accepting or discarding retained authority.

The existing native policy law and TypeScript source assertions currently target `budget.retained`. They need to target the actual original turn port and receipt, preserving identity, every currency and zero heap observations. They are not evidence that current runtime scheduling conserves authority: the compiler has not reached those assertions.

Evidence locations: Actor root Rust lines 644, 699, 3962, 3996, 4048, 4196, 4788, 4843, 5042; activation-reservation Rust lines 65–103; retained-turn Rust and its neutral schema, fixture and native/TypeScript tests.
