# Cargo Cfg Query Reuse Review

2026-10-16. Read-only Source audit; no compiler, capture, cache, lease, image or timing probe ran.

The initial audited epoch keyed a per-pass observed set by package/entry/profile/operation/platform/features. That unsafe skip is now removed by [unit coverage](cargo-source-admission-unit-coverage-current.md); every workspace unit receives its required facts and invokes `port.cfg`. The actual port launches `rustc --print cfg` every time. Distinct crates with identical cfg-query arguments do not reuse that observation. This is a concrete Source redundancy; its elapsed cost has not been measured and no warm latency estimate is asserted.

A safe future optimization must bind reuse to the actual retained compiler/toolchain identity, exact argv, actual immutable execution environment and cwd. The current Core actual port accepts only an environment object/cwd/reservation, so a global or arguments-only cache would invent compiler identity. Do not introduce one. A bounded per-operation owner can share exact successful cfg DATA only after its real compiler reader/control custody is explicit, rechecking that custody before and after every consumer. Failed/cancelled queries remain failures of the same Original; no replacement control or success adoption. Preserve finite entry/aggregate bytes and unchanged query transport ceilings.

Actual `rustc --print cfg` observation is separate from per-crate Source hashing/artifact invalidation. Source changes must still enter the affected unit key; sharing cfg DATA cannot cause dependent artifacts or source closures to be skipped. The active CPU profile/configuration consumption transaction changes this seam and should land before the optimization.

Reviewed Source SHA256: `8a41c08dbc0da8fb3080f6ea7141004f978905de2183b38ed6b61ff9e36a5ad9` for `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts`.

The Source hash above identifies the earlier audit epoch. The successor report binds the corrected current implementation; the query-reuse design remains pending and no timings were measured.
