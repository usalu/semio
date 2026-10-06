# External Lock and Physics Helper Audit — 2026-10-06

Read-only inspection; only this note written. Exact guest lock currently absent, not found at the canonical source path; Cargo.toml remains present with standalone workspace and wit-bindgen0.57.1(macros,async-spawn).

Taxonomy external-cargo-locks explicitly has external ownership, null ownerPath/target, and reason no exact repository Nx regeneration target. Root workspace deps-cargo-lock DOES exist and calls the existing native dependency script sync cargo-lock; that script38 loops discoverCargoWorkspaces and runs cargo update --workspace --manifest-path per discovered owner. This is broader than a precise guest-only generation and must not be mistaken for exact target. A standalone root lock cannot substitute for guest resolution. Correct external authority is Cargo for exact guest manifest; zero-touch permanent repair belongs in existing dependency/workspace script and registered target if missing discovery or precise route requires change.

## Current Physics Contract

Permanent 🚀️physics neutral schema/vectors has3 real public sci-kinematics kinds×4 domains×2 theme/languages=24 cases, with omitted,−4..8,reversed8..−4 and exact current default-equal0..3. The default-equal case distinguishes authored value from omitted branch default. Canonical domain values are emitted strings through existing mutation/replay/inverse and inference. Domain base differs deliberately to produce exactly one guarded edit. Full scientific sequence and dedicated phase both invoke helper; meaningful route also invokes it.

Independent projectile ballistic Math, orbit conic Math and phase damped oscillator exact Math align with source formulas. Phase RK4 approximation error at dt.1/10steps is small enough for existing.03mm physical and likely.001mm mapped numeric tolerance, but actual measurement—not source assumption—must certify the numeric threshold. Fit is sampled extrema expanded.08/.12/.1. Strict actual stroke/cardinality excludes focus dot and2-point zero-lines; independent probe and PDF assertions preserve all2104 point observations. Probe/path inventory currently coupled, so missing actual path would stop its probe comparison, but normal cardinality baseline reaches unconditional value assertion.

Required final source pins include exact Physics and Field native staged bytes, neutral fixture and current helper, canonical inference/mutation/diff emission inputs, per-case authored TikZ, compiler template/themes/fonts and actual terminal receipt. Current run progress is not success; default-equal must remain explicit during branch provenance repair.

Exact discovery exclusion confirmed: library/🗂️workspaces/🦀️cargo/🟦️.ts discovers only root Cargo.toml or workspace.metadata.semio.repository admitted workspaces. Guest [workspace] lacks that metadata, so workspace:deps-cargo-lock does not own this precise guest lock. No registered exact target exists, consistent with taxonomy external contract.


Root restored the exact externally Cargo-owned missing guest lock through88146 scoped bun/Nx exec with projects=workspace and excludeTaskDependencies. Installed Nx source was inspected to bind project selection rather than ignore cycles. Actual Cargo producer locked46 compatible packages, retained wit-bindgen0.57.1 and preserved manifest bytes. Generated lock source contains semio-jcoprobe-guest. Previous16153 failed before executing any Cargo because unscoped Nx exec selected all projects with an unrelated project cycle; the scoped rerun performs only the intended producer.
