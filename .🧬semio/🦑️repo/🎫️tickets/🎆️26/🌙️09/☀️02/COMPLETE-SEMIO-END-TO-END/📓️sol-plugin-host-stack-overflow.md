# Plugin Host Stack Overflow Repair

## Scope

This packet repairs the reproducible default-stack abort in
`artifact_mutation_router_tests::plan_drives_the_registered_owners_mutation_plan_job_to_completion`
and the same fixed-footprint defect in the pooled shard executor. It is associated with open goal
`🎯r2603` and umbrella ticket `26/09/02/COMPLETE-SEMIO-END-TO-END`. Ticket and goal state were not
changed.

## Red evidence and diagnosis

The isolated default-stack command was:

```sh
RUSTFLAGS='-Awarnings' RUST_BACKTRACE=full CARGO_TERM_COLOR=never CARGO_TARGET_DIR='<ticket>/🗑️generated/plugin-host-stack-overflow/target' bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache -- plan_drives_the_registered_owners_mutation_plan_job_to_completion
```

It aborted with `thread ... has overflowed its stack` and SIGABRT. Raising only
`RUST_MIN_STACK=16777216` avoided the abort but left the mutation test parked, which exposed a
separate relay-progress defect after the footprint failure stopped masking it. A process sample
showed idle pool workers and no recursive call chain.

Temporary native size probes located fixed stack growth, not recursion:

| Value | Before | After |
| --- | ---: | ---: |
| `GuestRelayMountedSlot` | 38,072 B | 38,072 B |
| `GuestRelayMountedRegistry` | 609,416 B | 24 B |
| `ShardLoop` | 6,642,576 B | 968 B |
| `ShardExecutorState` | 6,747,088 B | 1,048 B |
| `ShardExecutor` | 6,802,912 B | 1,584 B |

The concrete cause was retained fixed-capacity owner arrays stored inline: 16 mounted relay slots,
256 replay seeds, 512 replay refusals, and several 256-entry deferred-owner rings. Async construction
and movement placed those multi-megabyte values on Rust's default test stack.

The first post-heap executor run no longer overflowed. It completed its first drive immediately as
`ShardDrive::Fault`, with `consumed_epoch=Some(1)`, `work_remains=false`, no terminal frame or
overflow, and `pack: invalid issued UI patch receipt: actor-patch.unpaired-authority`. This proved
there was no executor lost wake: the empty kernel patch owner had incorrectly been published as a
non-empty 32-byte actor transport token with no receipt. The same behavior explained the independent
`to_actor_turn_result` arena-admission failure.

## Implementation

- Preserved every fixed item/byte capacity while moving mounted relay slots, replay seeds, replay
  refusals, and `FixedOwnerRing` slots to boxed slices.
- Kept one-opportunity relay semantics. A newly retained outcome wakes its owner for retirement;
  resume wakes only for immediate work; pending async work relies on the registered slot waker.
- Distinguished close `Complete`, `Pending`, and `Blocked`. Pending bounded close work
  self-reschedules, blocked work waits for its registered owner wake, and completion transfers once.
- Validated kernel patch/receipt pairing before publication. A validated empty owner is incrementally
  closed and becomes canonical actor `ui_patches=[]` with no receipt. Populated owners retain the
  existing fixed transport, single-claim token, and exact close lifecycle.

## Neutral fixture and independent oracle

`🧫️fixtures/🔣️stack-authority.json` is the language-neutral authority for four heap-backed
registries, their exact capacities, and the 65,536-byte inline-value ceiling. Two Rust tests validate
the fixture against actual mounted/shard storage and `size_of` values for `ShardLoop`,
`ShardExecutorState`, and `ShardExecutor`.

An independent Node implementation parsed the JSON without Rust code and verified schema version,
all ids/capacities/storage modes, the ceiling, and the expected summary:

```text
4/4 heap-mounted registries; inline owners <= 65536 bytes
```

No permanent script was introduced.

## Verification

All focused commands used the default stack and the isolated ticket target:

| Command/filter | Result |
| --- | --- |
| `stack_authority_matches_the_neutral_fixture` | PASS, 2/2; 232 filtered |
| `empty_turns_bypass_patch_transport_while_one_populated_owner_claims_once` | PASS, 1/1; 128 empty turns, then one populated owner claimed/closed once |
| `to_actor_turn_result` | PASS, 2/2; independent patch-admission case is green |
| `shard_executor_drives_a_turn_for_a_registered_actor_via_the_worker_pool` | PASS, 1/1 in 0.01 s |
| `plan_drives_the_registered_owners_mutation_plan_job_to_completion` | PASS, 1/1 in 0.01 s |
| `bun nx run @semio-tech/framework-plugin-host:check --skip-nx-cache` | PASS; production dev check finished in 1m 01s |
| `git diff --check -- <four packet paths>` | PASS |

The broader `bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache` compiled and
started 234 tests. It passed every repaired focus above, reported 19 failures in other concurrent
effect/relay/shard lifecycle tests, then aborted when
`cancel_job_effect_failure_retires_the_actor_and_surfaces_the_typed_fault` panicked and
`MountedReplaySeed::drop` panicked during unwind. A serial relay-group follow-up parked in its
first background-cleanup case for over 60 seconds and was interrupted. These residual lifecycle
failures are broader than the repaired stack, wake, and empty-patch publication paths; the full suite
is therefore not claimed green.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🔣️stack-authority.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-plugin-host-stack-overflow.md`

Temporary `[DEBUG] shard ...` instrumentation and size-probe tests were removed. The unrelated
pre-existing owned-WASI debug line was left untouched.
