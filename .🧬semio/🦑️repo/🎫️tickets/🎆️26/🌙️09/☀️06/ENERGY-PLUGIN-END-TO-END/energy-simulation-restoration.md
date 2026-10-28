# Native Energy Simulation Restoration

## Production Corrections

- Restored sim crate sibling ownership visibility for close-managed fields and the abandonment slot release function. Fields remain crate-internal.
- Moved the already projected `input_identity_bytes` and `input_identity_ready` fields from the mutable job authority to the actual public `EnergyJobCursor`. They describe computed progress, not independently mutable authority.
- Imported actual weather helpers used by packet preparation.
- Moved the exact immutable borrowed `ArtifactCanonicalTreeSource` implementation from operation-wire publication into its true canonical-JSON owner. The operation-wire composition facade explicitly reexports that same type. Energy now depends directly on canonical traversal without importing the whole OS kernel or broader publication machinery.
- Added `EnergyBatchRun::cursor()` to expose the real retained numerical/input progress to hosts and diagnostics.
- Corrected a real close stall: `precompute_close_demands` quoted controlled retirement of the precompute builder or completed model, but `close_step` never performed either retirement. Added original-owner `begin_granted` and `step_granted` frontiers. Before this correction, preview and numerical cleanup hit finite million-turn bounds; afterward the complete suite finishes in under a second.

## Test Infrastructure Corrections

Enabled the actual job testkit and sim testing feature for companion tests. Replaced companion access to private implementation fields/stage types with explicit feature-gated testkit inspection and stage-selection functions. This preserves production field privacy and retains the same stage coverage. Added the actual global `HeapWitness` allocator to the companion test executable; without it, its original-source receipt checks observed zero allocations while correctly quoted roots cost64 bytes.

No receipt assertion was weakened, no retirement grants or fixture stocks were increased, no numerical expectations were relaxed, and no source copies or mocked runtime types were introduced.

## Native Verification

- `semio-s-plugin-energy-engine-sim --lib`: 2 passed, 0 failed.
- `semio-s-plugin-energy-engine-sim-unit-tests --lib`: 28 passed, 0 failed, exit0, reported runtime0.40s.
- The focused preview test previously failed with `finite Energy numerical close remained pending` after7.61s. After adding the omitted real precompute retirement it passes in0.02s.

The28-test suite covers actual single-zone/full-topology simulation, ASHRAE140 Case600, heating-day behavior, deterministic repeatability, calendar run periods, energy conservation, per-surface air balance, warmup accounting, schema-first numerical admission limits, previews/checkpoints/commit leases, cancellation/deadlines/stale generations, weather chronology, original Model+Config canonical bytes against third-party Serde, paid cancellation prefixes, and delegated physical precompute retirement.

The independent annual BESTEST harness remains owned by the IDF lane; these passing bounded native tests do not establish annual compliance by themselves.

## Source Files Changed by This Lane

- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧱️state/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📮️packets/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🔚️close/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📡️wire/🛡️job-vocabulary-and-census/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️close/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧺️batch/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️operation-wire/🦀️.rs`
