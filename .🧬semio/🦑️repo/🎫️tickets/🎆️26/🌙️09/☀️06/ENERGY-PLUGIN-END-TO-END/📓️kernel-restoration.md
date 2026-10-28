# Native Kernel Restoration

The simulation kernel now compiles independently and its 13 unit tests pass with the repository-pinned nightly toolchain.

- `SimulationEnvironment` and `ConvergenceTolerances` implement the first-party canonical JSON tree protocol required by `SimulationConfig`.
- The funded timestep operation exposes its receipt and checkpoint signature across the kernel/simulation crate boundary.
- Nonthermal timestep stages return explicit progress receipts. A zero item or zero depth grant cannot mutate their continuation. Completed work reports no progress.
- The new language-neutral `timestep-grants.json` fixture tests these grant boundaries. Existing retained thermal tests continue to validate free, controlled and constrained balances.

Validation: ticket `📜️energy-cargo.py test --manifest-path ✏️s/🔌️plugins/🔋️energy/Cargo.toml -p semio-s-plugin-energy-engine-kernel --quiet`; 13 passed, 0 failed.

A combined kernel/simulation attempt advanced through kernel compilation and revealed separate simulation authority/wire API errors, recorded in the generated simulation log and delegated to the coordinator. This kernel result does not claim full engine or browser completion.
