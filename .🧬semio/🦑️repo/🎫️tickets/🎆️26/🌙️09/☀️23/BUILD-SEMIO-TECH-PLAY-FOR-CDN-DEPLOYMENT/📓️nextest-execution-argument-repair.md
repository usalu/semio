# Nextest Execution Argument Repair

The registered Process3D native gate exited before compilation because top-level --nocapture was forwarded to cargo nextest list. Existing status and final-status controls were already correctly execution-owned; no repair was needed for them. Earlier Cad success-output likewise reached a compiler invocation incorrectly.

The registered framework-process:test-cargo-driver reproduced the new neutral capture partition failure before implementation: nocapture appeared in buildArgs rather than executionArgs. Installed Nextest run help independently confirms capture and success/failure output options. A Node parseArgs oracle checks value and alias meaning without using the implementation partition.

The narrow partition repair places both capture spellings and success/failure output values in the assertion step. Selection, compiler flags, byte capture bounds and test budgets remain unchanged. The neutral fixture also supplies a dependency-free Cargo package whose one actual assertion is run through the production driver in three capture/output configurations. Final expanded source/native driver validation remains pending.

Authored files:

- 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts
- 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧫️fixtures/🎬️execution/🔣️.json
- 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🟦️.ts

The expanded registered driver suite passed: 4 tests, 0 failures, actual Nx exit 0. The production driver compiled and ran the independent fixture assertion in all three configurations; each reports one native assertion passed and emits the expected DEBUG owner marker. Existing empty-selection rejection still passed. Native command flags are now validated through the actual Nextest compiler/metadata/executor, not only argument projections.
