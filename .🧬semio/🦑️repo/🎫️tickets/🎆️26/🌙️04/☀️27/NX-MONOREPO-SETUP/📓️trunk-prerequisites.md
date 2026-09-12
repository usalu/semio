# Trunk Preparation Through Nx

WGPU `wasm`, `serve` and `dev` now depend on the uncached `workspace:deps-trunk` target. The leaf uses the existing root setup authority to acquire Trunk 0.21.14 with Cargo's locked install and prepare `wasm32-unknown-unknown` only when absent. Build and serve no longer perform those installations internally. `workspace:deps-wasm` depends on this leaf and retains wasm-pack, lock-selected wasm-bindgen and the WASI target setup. The existing pin has one source; it was moved, not duplicated.

The language-neutral Trunk fixture now covers an already ready environment, a stale Trunk version and missing tooling. The regression failed before implementation because no prerequisite existed (94 ms). After implementation and correcting missing executor metadata in its isolated graph, it passed in 2.3 seconds. TypeScript executes the production setup methods under controlled process probes, native Nx builds the consumer task graph from the authored dependencies, and Bun/smol-toml retain the existing locked metadata contract. No shared tool was installed in this test. Fresh native acquisition and the complete WGPU compiler are not qualified by this controlled test.

The new editor launcher is in the seed. Actual registry generation completed successfully through Nx in 1m41s, including input discovery, and refreshed `.vscode/launch.json`. This run overlapped the expensive default playground discovery in the broad suite; it is not an isolated performance baseline.

## Compiler Output Follow-Up

The compiler still requires an exclusive finite output contract separate from live plugin/extension stores and serve/watch output. Upstream Trunk 0.21.14 [build configuration](https://raw.githubusercontent.com/trunk-rs/trunk/v0.21.14/src/config/rt/build.rs) places staging under the final distribution directory. Its [build command](https://raw.githubusercontent.com/trunk-rs/trunk/v0.21.14/src/cmd/build.rs) exposes a distribution override and release/configuration environment variables. Those are relevant to output isolation and hashing; merely declaring the current shared directory would include mutable runtime stores. The old documentation domain returned unrelated content and was discarded as a source.

The complete compiler's optimizer/bindgen selection, live routes, watcher lifecycle and source/prerequisite closure still need validation. This change specifically removes the observed unversioned Trunk install from build/serve and exposes its preparation to Nx.
