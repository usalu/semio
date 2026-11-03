# Integration of the shared norm surface crates

Project names: `@semio-s/plugin-norm-app-surface`, `@semio-s/plugin-norm-structural-scene` and `@semio-tech/norm-compliance-rs` do not exist; shared crates build through `@semio-tech/norm-artifact-contract-rs` and through each artifact `-rs` project.

## Builds
| # | command | result |
|---|---|---|
| 1 | `plugin-norm-structural-scene:check` | exit 1, "Unknown project in the closure" (wrong project name) |
| 2 | `plugin-norm-app-surface:check` | cancelled by SIGTERM (not mine) while waiting for capacity |
| 3 | `norm-artifact-contract-rs:check` | exit 1, compiler ingress refusal (framework defect, see `ingress-blocker.md`) |
| 4 | `norm-artifact-contract-rs:build` | exit 0 (15m44s; compliance, ndp, scene + foundation, app-surface foundation/live-core/viewport3d/visual-chrome/live-editor/panels listed) |
| 5 | `norm-artifact-contract-rs:test` | exit 1, cargo status 101 with no rustc diagnostic in the log (likely killed/aborted under memory pressure); not retried |
| 6 | `norm-en1995-rs:build` | exit 1, real rustc errors in `app-surface-panels`: 5x E0284 (`UiFixedList` const N inference, `checks` line 333) and E0615 (`FieldSpec::unit` is a method, `properties` line 57) |
| 7 | `norm-en1995-rs:build` (after fixes) | exit 1, "Insufficient physical compiler initial headroom" (host memory, infra) |
| 8 | `norm-en1995-rs:build` | exit 1, "Funded Nx graph cache capacity exhausted" (infra) |
| 9 | `norm-en1995-rs:build` with `SEMIO_NX_GRAPH_CAPACITY_WAIT_MS=5400000` | exit 1 after about 80 min: `Resource lease timed out: cache-prune` / `resource-directory-admission:v1` (infra contention; no rustc error reached). Logs kept in `🗑️generated/integrate/` |

## Fixes applied
- `📋️prelude/🦀️.rs`: removed the `store_ownership_macros` module mount (double ownership, source admission refusal); foundation owns it.
- `🪟️visual-windows/✅️checks/🦀️.rs:333`: `let mut children: UiFixedList<_> = UiFixedList::default();`.
- `🪟️visual-windows/🧾️properties/🦀️.rs:57`: `meta.unit` -> `meta.unit()`.
- compliance `🏛️norm-error-and-family`: `IncrementalNormFamily::evaluate_incrementally` gets `where Self: Sized`. `UnitFamily::SpecificHeat` and the `ColdMutation` bound on `NormHost::apply` were already present.

## Open
- Artifact `:build` of en1995 not yet green past app-surface; downstream crates (live-editor, facade, artifact editors/viewers) not yet compiled.
- Contract `:test` failure needs a retry with the capacity wait knob.
- Remaining nine artifacts, mechanics build/test and call-site breakage unverified.
