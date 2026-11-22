# process-js, mutation bridge, hub composition: status

## Result: not green; blocked by shared tooling and ungenerated files

### Fixed
- `admissionReserveBytesV1` was imported by the cargo launcher (`repo/.../cargo/launcher/🟦️.ts`) but never defined. This failed the project graph slice worker 3 times (exit 75), so every `repo run` died. Added to `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts` and re-exported in `.../📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts`.
- Generated `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs` (gitignored) by calling `generateStylingArtifacts` (script `🗑️generated/gen-styling.ts`).

### Open blockers
- `repo run` (process-js:test and s-dev-composition:generate): the graph read ran for 35 min and then 1h48m, ending with a resident service crash (ref 639a8c20). The shared fleet is overloaded or stuck.
- dev-route test fails on load: `BUILD_HOST_CAPACITY_POLICY_V1` rejects `🛍️products/🦑️repo/.../🏗️build/🔣️policy.json` hostCapacity.swap. It now has `activeBasisPointsPerSecond/outBasisPointsPerSecond/shortBasisPoints`; the schema in `🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json` and `📏️decision/🟦️.ts` still require `activeMegabytesPerSecond/outMegabytesPerSecond`. Another agent's half-done migration (policy.json edited minutes earlier). Not touched.
- Bridge `cargo build` (native rustc, repo ingress bypassed with `CARGO_BUILD_RUSTC` and an empty `CARGO_BUILD_RUSTC_WRAPPER`): fails on missing ungenerated files, e.g. `🧬️schema/⚛️component/../🤖️generated/🏷️entity-kinds/🦀️.rs`. Needs the generate target to run (the repo is unusable for that right now), or the generators to be called directly.
- Hub composition `cargo check`: not reached.

### Next steps
1. Finish the host-capacity swap policy migration (schema and decision code) so `repo` graph loads work.
2. Run `@semio-tech/s-dev-composition:generate` once the fleet is healthy, then rerun the three items.
3. A `bun test` of the three example tests worked directly with absolute paths (2 of 3 pass; dev-route fails on the policy error above).
