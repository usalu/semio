# 🔋️ Energy plugin end to end — status

App under test: 🔋️energy plugin `✏️s/🔌️plugins/🔋️energy` (artifact `🗿️artifacts/🔋️model`, standard `🔖️1`, subset `✳️any`, engine `🔨️modules/⚡️simulation/⚙️engine`, ~26k lines Rust across 50 modules).
Ticket opened 2026-09-06 by session ⚪411af150 (Fable 5.1 coordinator). Repo MCP timed out at start; bookkeeping is manual on disk. Goal: `ENERGY`.
Ticket start commit: `🗑️generated/start-commit.txt`.

## Definition of done
1. Energy crate `cargo check` green natively and on `wasm32-wasip2`.
2. Every mutation of the 🔋️model artifact has a real-world fixture test, passing in Rust and validated against a third-party oracle (Honeybee-energy → OpenStudio → EnergyPlus, plus lightweight Python/TS oracles where E+ is not the right instrument); Rust lib tests + oracle tests green.
3. Owner-root `🔣️.json` / `🛂️.descriptor.semio` regenerated via `describe`; registry check accepts energy.
4. Energy playground boots (react renderer, then wgpu wasm): every window (structure, zones, simulation) renders non-empty, examples load and switch, editor actions dispatch at runtime, confirmed with console logs.

## Log
- open. Host: 10 cores, heavy peer load (many rustc/cargo from other sessions, Codex + 5 claude sessions). No EnergyPlus/OpenStudio/honeybee on host; Docker 29 available; Python 3.14 venv via uv.
- Baseline facts: one mutation kind `♻️replace-model` (payload `newModelJson: string`, whole-model swap); engine 19k lines / 50 modules under `🔨️modules/⚡️simulation/⚙️engine`; previous oracle survey (recorded in `✳️any/🔮️oracle/🔣️.json`) DECLINED EnergyPlus/OpenStudio, this ticket reverses that per the goal (Honeybee → OpenStudio → EnergyPlus as physics + semantics oracle). TS `package.json` is a cad copy-paste. Docker daemon down; no E+/OpenStudio/honeybee installed.
- Started background native `cargo check -p semio-s-plugin-energy --lib --tests` with private `CARGO_TARGET_DIR=target-energy-e2e`, `RUSTC_WRAPPER=` (log in scratchpad `check-native-1.txt`).
- Exploration fleet (8 × Sonnet, read-only, no builds) launched: mutation vocabulary, mutation scaffolding recipe, test+oracle infra, engine API+validation, editor UI+boot, build gates+history, oracle toolchain research (web), sibling ticket recipes. Reports land as `📓️explore-*.md` here.
- Reports landed: `📓️explore-sibling-recipes.md`, `📓️explore-oracle-toolchain.md` (E+ 26.1.0, OpenStudio 3.11.0 bundling E+ 25.2.0, honeybee-energy 1.123.32 / honeybee-openstudio 0.7.2, openstudio PyPI wheels → oracle venv on Python 3.12; BESTEST IDFs no longer in E+ testfiles → NREL/BESTEST-GSR), `📓️explore-engine-api-and-validation.md` (`Engine::run(Model, SimulationConfig)`, `Model::validate` is `#[cfg(test)]`-only, bundled EPW is a 1-day Hannover file, first-order CTF → 600/600FF/900/900FF/610 feasible).
- Known compile blocker (from S-END-TO-END catalog audit): `semio_framework::` unresolved (3 sites), 279 errors. Load average 200+.
- Wrote `📓️bestest-contract.md` (shared interface between the oracle worker and the engine worker).
- Wave 1 Opus workers launched: W-A compile-green (native + wasm), W-B oracle toolchain provisioning + honeybee translator + BESTEST reference results, W-C BESTEST models as committed fixtures + engine comparison harness.
