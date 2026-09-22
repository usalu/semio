# FP11 — `semio-framework-plugin --lib` 819/5 → ?, and the product lanes behind the reds

Slice FP11, 2026-09-22 (session 8). Continues FP10 (`📓️fp10-plugin-lib-and-lanes.md`), FP9, FP8–FP5.

Method (unchanged from FP5–FP10): private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp11`
+ the shared build-dir, `CARGO_INCREMENTAL=0`, `RUST_MIN_STACK=67108864`, whole-suite numbers are the
**serial** (`--test-threads=1`) reading. FP10's lesson applied: `cargo check … --profile test` first
(never uplifts), `--no-run` link second.

## 1. Round table

(filling)

## 2. The seven items

| # | item | FP10 state | FP11 |
|---|---|---|---|
| 1 | §2.2 async-task lane in production | diagnosed, test-only | (filling) |
| 2 | §2.3 composed settle discards the child lane | one line located | (filling) |
| 3 | §2.6 Worker-lane fault aborts the whole turn | diagnosed | (filling) |
| 4 | §2.4 `TestSnapshot` declares no child refs | not landed | (filling) |
| 5 | §2.5 reclamation law's sharper cause | re-expressed, red | (filling) |
| 6 | §3 flow `retained::*` — one additive `🏪️store` change | named, not landed | (filling) |
| 7 | order-dependent flake (`an_abandoned_ingress_owner…`) | observed | (filling) |

## 3. Item detail

(filling)

## 4. Dependent-crate re-checks

(filling)

## 5. Files changed

(filling)

## 6. Honest gaps

(filling)
