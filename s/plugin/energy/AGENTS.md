---
technology: energy
emoji: ⚡
---

# Energy

Headless Rust building energy model (BEM) engine. EnergyPlus-class transient whole-building simulation via typed Rust API — no IDF/epJSON, templates, scripting, or language bindings.

## Engine (`energy/engine/rs`)

- **Model** — single native typed input representation (SI units)
- **Engine::run** — deterministic predictor-corrector simulation kernel
- **Results** — canonical time series, meters, summaries, sizing, diagnostics
- **site** — native EPW weather ingest, design days, solar position, ground models
- **economics** — optional tariffs / LCCA post-pass (non-physics)

## Conventions

- Docstrings start with a unique emoji; no comments inside definitions
- Regions in `lib.rs` or `src/` modules; unit tests in each module
- `bun ./script.ts test` via nx `@semio-tech/energy-engine`
- Do not depend on `norm_*` or CAD energy modules

## Stack

- Rust crate `energy_engine`
- `mathematical_algebra` / `mathematical_geometry` for spatial math
- `serde` for result serialization
