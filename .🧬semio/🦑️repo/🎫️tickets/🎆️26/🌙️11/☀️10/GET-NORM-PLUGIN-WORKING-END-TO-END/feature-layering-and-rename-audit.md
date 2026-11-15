# 🔎 Feature Layering and Rename Audit (2026-11-13, read-only, manifest reading only — no `cargo tree`)

## 1. Test-only code leaked into the production component: YES (fixed in manifests, unverified by build)
- `LocalizedText` (`⚖️compliance/📐️quantities-clauses-and-text/🦀️.rs:195-199`) derives serde only under `any(test, feature = "compliance-testing")`; every struct holding it is gated the same way, so production needs no serde.
- Leak: the vdi3805 and iso16757 artifact root crates listed their model under `[dependencies]` with `features = ["test-support"]`; the hub composition depends on both, so feature unification switched `compliance-testing` (serde, serde_json, artifact-reference, design-case testing) on in the production cdylib.
- Fix applied: feature removed from the two `[dependencies]` lines (the `[dev-dependencies]` entries already carry it). The wave's `test-support → compliance-testing` forwarding stays (test-only edge).
- To verify: full component check/build; `cargo tree -e features -i semio-s-plugin-norm-compliance-core` through `repo cargo`.

## 2. din18599 `-w-m2` → `-wm2` rename
- No stale leaf-kind strings outside the tickets folder.
- Leftovers: Rust module identifiers `change_zone_internal_gains_w_m2` / `change_zone_lighting_power_w_m2` (`🧬️mutations/🔰zone-fields/🦀️.rs:21,23`, `🧬️mutations/🦀️.rs:45-46`, leaf `🦀️.rs:33,36`, their `🔺️diff` / `↩️inverse` imports) still use the old spelling for `-wm2` folders.
- Proto field names differ: `🧬️mutations/🛰️.proto:18` `internal_gains_w_m2` vs `🧬️schema/🛰️.proto:27,29` and `📸️snapshot/🛰️.proto:27,29` `internal_gains_wm2` / `lighting_power_wm2` — check whether these are the same message before aligning.
- Not leftovers: unit-suffix names in DSL assets, energy grammar, sqlite columns `internal_gains_w_m2` (`📸️snapshot/🦀️.rs:194-195`, `🗄️.sql:28`).
