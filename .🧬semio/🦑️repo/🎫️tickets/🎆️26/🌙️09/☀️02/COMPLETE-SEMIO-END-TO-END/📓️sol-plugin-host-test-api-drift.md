# Plugin Host Test API Drift

Date: 2026-09-03
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`

## Outcome

The plugin-host `cfg(test)` target compiles again against the current actor lifetime and patch-receipt contracts. Production APIs and codecs were not changed.

The current red census was exactly 23 compiler errors:

- 7 `TurnResult` initializers missing `ui_patch_receipt`;
- 15 obsolete unit `Event::InstanceClose` constructor/serialization uses after the variant became `InstanceClose(ActorInstanceCloseRequest)`;
- 1 obsolete `InstanceOpen { instance }` field after the variant moved to `InstanceOpen { request: ActorInstanceOpenRequest, ... }`.

The prior codec packet's expected 8/14 split had changed in the live tree; this packet records the compiler-observed current 7/15 split rather than repeating the older estimate.

## Changes

- Added `ui_patch_receipt: None` to six empty-patch test results.
- Added one valid `ActorUiPatchReceipt` to the `ShardOutcome` round-trip vector whose opaque `ui_patches` body is non-empty, preserving the current one-body/one-receipt pairing law.
- Replaced every obsolete close unit constructor with a valid close event using one shared `#[cfg(test)]` helper. The helper owns the shared lifecycle fixture identity: activation generation 1, instance 7, guest lifetime 13, request sequence 9.
- Updated the ignored process-shard open helper to carry `ActorInstanceOpenRequest { activation_generation: 1, instance_id: checked actor id, request_sequence: envelope sequence }`; no compatibility field or shim was added.
- Reused `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json`. A permanent Rust test compares first-party JSON for the close request against the independent `serde_json::Value` fixture structure, then separately proves the enclosing existing Event serde boundary round-trips. It does not make serde authoritative over the first-party value.

Exact source census after the changes: 0 stale unit-close references and 0 obsolete `instance:` fields in the process open helper.

## Compile verification

All Cargo output used the isolated target:

`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/plugin-host-test-api-drift-target`

```text
RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR='<ticket target>' bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache -- --no-run --message-format=short
```

- Red: exit 1 with exactly the 23 errors above.
- First green: exit 0; one lib-test executable built in 46.63s.
- Final green after consolidating the close fixture helper: exit 0; one lib-test executable built in 42.33s; 0 compiler errors.

```text
RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR='<ticket target>' bun nx run @semio-tech/framework-plugin-host:check --skip-nx-cache -- --message-format=short
```

Result: PASS, exit 0; production check finished in 1m44s.

## Runtime verification and next blocker

Focused commands used the same Nx target and isolated Cargo target.

| Selector | Result |
| --- | --- |
| `instance_close_event_matches_the_shared_first_party_fixture_and_serde_structure` | PASS, 1/1; 230 filtered; final run built in 40.64s and the test completed in 0.00s. |
| `shard_outcome_owned_pack_round_trips_every_variant` | PASS, 1/1; 230 filtered; build 34.30s, test 0.00s. This exercises the added paired patch receipt through the owned pack codec. |
| `shard_executor_drives_a_turn_for_a_registered_actor_via_the_worker_pool` | BLOCKED at runtime by an independent stack overflow; process aborted with SIGABRT after a 43.99s build. |
| `to_actor_turn_result` | 1 passed and 1 failed. `to_actor_turn_result_maps_status_and_carries_host_measured_usage` passed; the status-loop test failed on its later case with `plugin.turn-patches-admission` after a 49.41s build. |

The complete default suite was run once:

```text
RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR='<ticket target>' bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache
```

It compiled successfully in 53.38s and started 231 tests. Exactly 9 tests reported `ok`; the 10th, `component::artifact_mutation_router_tests::plan_drives_the_registered_owners_mutation_plan_job_to_completion`, overflowed its stack and aborted the process with SIGABRT. Because the harness aborted, the remaining 221 tests did not report results and there is no truthful suite pass count beyond those 9. The separate executor stack overflow and patch-transport admission failure show that runtime repair is a broader independent packet; neither was modified here.

An attempted `--exact` selector matched 0 tests because Cargo requires the fully-qualified test path; it was discarded as evidence and the non-exact one-test selector above was run instead.

## Hygiene

Owned test-only edits are in:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs`
- this report.

`git diff --check` over the four source paths passed with no output. The explicit trailing-whitespace scan passed. No production API, dependency, manifest, script, hub, DB, directory authority, launch configuration, or `AGENTS.md` file was changed. No new executable command was added.
