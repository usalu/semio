w12 slice — 🎞️animate / 🎬️sequence / 🎥️shooting / 🌊️flow unregistered mutation vocabularies
Date 2026-08-24. Every command below was actually run from
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test unless noted. Exit codes read from the tool's own
exit status, never through a pipe.

## 1. contract, per case — all four exit 0, zero breaches

$ bun ./📜️script.ts contract --owner 🎞️animate  --case mutate-present-1     -> exit 0
$ bun ./📜️script.ts contract --owner 🎬️sequence --case mutate-sequence-1    -> exit 0
$ bun ./📜️script.ts contract --owner 🎥️shooting --case mutate-shooting-1    -> exit 0
$ bun ./📜️script.ts contract --owner 🌊️flow     --case mutate-flow-1        -> exit 0

  0 high-priority breach(es) across 0 rule(s):

.🧬semio/🦑️repo/⚡️cache/breaches/testing.json is [] after the last run — the six
`unregistered-mutation-vocabulary` breaches this slice owned are gone:
  ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
  ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations
  ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
  ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations
  ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
  ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
(the gate keys on dirname(dirname(rel)), so ONE subset manifest closes both facets of a vocabulary
that is split across 🧬️schema and 🚪️io — animate and sequence each had two breach rows, one
vocabulary.)

## 2. Scenario expansion, read through the platform's own parseFeature

mutate-present-1   scenarios=19 errors=0 catalog=present-1-any    modes=conformance,property,round-trip
mutate-sequence-1  scenarios=17 errors=0 catalog=sequence-1-any   modes=conformance,property,round-trip
mutate-shooting-1  scenarios=63 errors=0 catalog=shooting-1-any   modes=conformance,property,round-trip
mutate-flow-1      scenarios=21 errors=0 catalog=flow-1-any       modes=conformance,property,round-trip
= 120 scenarios = 2x(9+8+31+10) kinds + 4 identity-round-trips.

## 3. oracle exhaustive, per case — all four report not-exercised

[test] not-exercised .../🎬️present/🧪️tests/mutate-present-1 (recorded no-oracle decision present-figure-deck-mutation-semantics — its evidence is discharged by the subject phase)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
[test] not-exercised .../🎬️sequence/🧪️tests/mutate-sequence-1 (recorded no-oracle decision sequence-step-graph-mutation-semantics — …)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
[test] not-exercised .../🎥️shooting/🧪️tests/mutate-shooting-1 (recorded no-oracle decision shooting-render-scene-mutation-semantics — …)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
[test] not-exercised .../🌊️flow/🧪️tests/mutate-flow-1 (recorded no-oracle decision flow-widget-graph-mutation-semantics — …)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1

This is the runner's designed behaviour for a recorded no-oracle case (📜️script.ts runPhases: the
oracle role is dispatched only when decision.implementation != null). All four artifacts are
semio-native, so the decision is honest, but it does mean these 120 scenarios execute in no phase
today — same standing as the 19 semio Pattern-B cases w11 counted.

## 4. subject exhaustive — blocked in framework dependencies, not in this slice

$ bun ./📜️script.ts subject exhaustive --owner 🎞️animate --case mutate-present-1
[test] ...: rust subject host exited 101 without emitting results
error[E0599]: `dsl::Fault` doesn't implement `std::fmt::Display`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/../../🧵️shard/🦀️component.rs:935:89
error: future cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/../../🧵️shard/🏃️executor.rs:601:36
error: could not compile `semio-framework-plugin-host` (lib) due to 8 previous errors

$ bun ./📜️script.ts subject exhaustive --owner 🎬️sequence --case mutate-sequence-1
[test] ...: rust subject host exited 101 without emitting results
error[E0425]: cannot find type `Arc` in this scope
  --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️gpu.rs:39:38
error: could not compile `semio-framework-ui` (lib) due to 1 previous error

Same picture from the repo root, distinct errors per crate:
$ cargo check -p semio-s-plugin-animate  --lib -> `semio-framework-plugin-host`, 8 errors
$ cargo check -p semio-s-plugin-shooting --lib -> `semio-framework-plugin-host`, 8 errors
$ cargo check -p semio-s-plugin-sequence --lib -> `semio-framework-ui` (wgpu), 1 error
$ cargo check -p semio-s-plugin-flow     --lib -> `semio-framework-ui` (wgpu), 1 error
Neither error is in this slice's files nor in the plugins' own artifact code. For contrast,
$ cargo check -p semio-s-plugin-stdio --lib -> Finished (exit 0), and
$ cargo check -p semio-framework-os-kernel --lib -> Finished (exit 0): w11's os-kernel blocker is
cleared, a different one took its place.

## 5. dependency purity — exit 0 after adding the artifact-level host manifests

$ bun ./📜️script.ts dependency -> exit 0
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
(unchanged counts; `semio-s-plugin-stdio-test-oracle` is a local path crate, so declaring it as a
rust oracleHostPackage for three more owners adds no third-party edge.)

## 6. The blocker this slice could not clear: MutationKind is sync, most leaves declare async

`protocol::MutationKind`'s trait methods are SYNC
(🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:206-217:
`fn diff(&self, base: &P) -> MutationOutcome<…>`, `fn inverse(&self, base: &P) -> Vec<Op>`), and
`#[derive(Mutations)]` generates a sync `impl Mutation`. Counted `    async fn diff(&self` over each
subset's `🧬️mutations/<kind>/🦠️mutation/🦀️component.rs`:
  🎬️present   0 of  9 leaves async   (already de-async'd — the corrected shape)
  🎬️sequence  8 of  8 leaves async
  🎥️shooting 31 of 31 leaves async
  🌊️flow      9 of 10 leaves async
Their dispatch-level entry points follow the same split: `apply_present_mutation` /
`inverse_present_mutation` are `pub fn`, while `apply_flow_mutation`, `inverse_flow_mutation`,
`apply_sequence_mutation`, `inverse_sequence_mutation`, `parse_dsl`/`print_dsl` (flow, sequence,
shooting) and `flow_working_scene` are `pub async fn` — and every existing in-crate caller, including
each vocabulary's own committed leaf tests, calls them WITHOUT `.await`. This slice wrote its
adapters and its new production bridges against the SYNC shape, which is what 🎬️present already
landed and what every call site assumes; it did not de-async the other three, because that is the
async-convention ticket's repo-wide job and a partial pass would be arbitrary.
