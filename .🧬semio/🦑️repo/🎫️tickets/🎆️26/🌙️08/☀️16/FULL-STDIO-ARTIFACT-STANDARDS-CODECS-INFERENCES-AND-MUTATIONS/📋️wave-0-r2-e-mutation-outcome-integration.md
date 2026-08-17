# Wave 0 R2-E Mutation Outcome Integration

## Scope

Restored the framework-plugin compile boundary after `protocol::Mutation::diff` changed from a bare diff to `MutationOutcome<Diff>`.

## Changes

- Updated the framework-owned `NoConfigMutation`, `NoPresenceMutation`, `NoTransientMutation`, and `InteractionConfigMutation` leaves to construct successful typed outcomes.
- Updated composite proposal simulation and transaction preparation to inspect `MutationOutcome` under the default merge policy before applying its diff.
- Removed the obsolete `Mutation::validate` call from transaction preparation; typed rejection now travels through outcome messages.
- Updated all seven framework-plugin test mutation fixtures to return the same typed outcome contract, and replaced the removed `Severity::Hint` fixture value with `Severity::Info`.

## Verification

`cargo check -p semio-framework-plugin` exited `0` on the current combined tree. The crate still emits existing warnings; no warning was represented as a test or runtime pass.

`cargo test -p semio-framework-plugin --lib dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying` exited `0`: one filtered transaction test passed and 216 were filtered out. This test proves a mutation carrying foreign steps becomes a proposal instead of being applied locally.

## Handoff

The restored boundary unblocks filtered stdio/glTF tests. It does not close the separate glTF migration from the legacy closed mutation enum to the 222 descriptor-owned command leaves.
