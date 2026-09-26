# Impl Gate — Cross-Family Compliance Gate

## Deliverable

Permanent integration test binary `compliance_gate` in `semio-s-plugin-norm`:

- Source: `✏️s/🔌️plugins/📕️norm/🧪️tests/🚦️compliance-gate/🦀️.rs`
- Registered via `[[test]] name = "compliance_gate"` in `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml` → path `../../🧪️tests/🚦️compliance-gate/🦀️.rs`
- Dev-dep: `semio-framework-os-kernel` (for `ArtifactDsl` / `ToValue` / `FromValue` / `Locale`)
- Table-driven over all 15 families; aggregates per-family failures (does not stop at first family)

## Assertions per family

1. Default + every registered example DSL decodes (`ArtifactDsl::parse_dsl`) and evaluates; ≥1 compliant example (`complies()`) and ≥1 non-compliant with ≥2 `Fail` checks
2. Every check has non-empty, non-identical `title`/`explanation` in en and de (`LocalizedCopy::resolve`); `subject.path` and remedy `target.path` parse + resolve via `parse_path` / `get_value_at_path`
3. Every applicable Fail remedy applied via `apply_remedy_edit` clears that check id (partial utilization-only effect fails the gate while `applicable`)
4. `NotApplicable` checks carry localized explanation (reason)
5. DE vs EN annex divergence on limit/computed, or family listed in `ANNEX_IDENTICAL_ALLOWLIST`

## Annex allowlist

`din4108`, `din18599`, `iso16757`, `vdi3805` — no annex axis (or no EN/DE recommended-value split under evaluate).

## launch.json

Not registered: launch.json lists norm playgrounds only, not per-crate test configs.

## Helper requests

None — gate uses existing public `parse_path` / `get_value_at_path` / `set_value_at_path` / `apply_remedy_edit`.

## Runner

Command: `bun nx run @semio-tech/norm-plugin:test -- --test compliance_gate --no-fail-fast`

Polled ~40 min (12 attempts). iso16757 recovered at attempt 7; fleet stayed compile-blocked as other family agents kept editing.

### Latest Summary (final attempt 2026-09-26 14:15)

```
error: command cargo test --no-run … --package semio-s-plugin-norm --test compliance_gate exited with code 101
```

Blocking `could not compile` lines from `🗑️generated/gate/gate-run-final.txt`:

- `error: could not compile `semio-s-artifact-norm-din16798` (lib) due to 3 previous errors; 2 warnings emitted`
- `error: could not compile `semio-s-artifact-norm-din18599` (lib) due to 9 previous errors; 23 warnings emitted`
- `error: could not compile `semio-s-artifact-norm-din4108` (lib) due to 1 previous error; 4 warnings emitted`
- `error: could not compile `semio-s-artifact-norm-en1990` (lib) due to 26 previous errors; 2 warnings emitted`
- `error: could not compile `semio-s-artifact-norm-en1995` (lib) due to 60 previous errors`
- `error: could not compile `semio-s-artifact-norm-en1998` (lib) due to 6 previous errors; 4 warnings emitted`

## Per-family gate results

Gate binary never linked, so no PASS/FAIL assertion outcomes. Status reflects whether that family's crate was among the final compile failures (direct) or blocked only because the plugin cannot link (transitive).

| Family | Gate |
|--------|------|
| `din4108` | BLOCKED (compile) |
| `din16798` | BLOCKED (compile) |
| `din18599` | BLOCKED (compile) |
| `en1990` | BLOCKED (compile) |
| `en1991` | BLOCKED (transitive) |
| `en1992` | BLOCKED (transitive) |
| `en1993` | BLOCKED (transitive) |
| `en1994` | BLOCKED (transitive) |
| `en1995` | BLOCKED (compile) |
| `en1996` | BLOCKED (transitive) |
| `en1997` | BLOCKED (transitive) |
| `en1998` | BLOCKED (compile) |
| `en1999` | BLOCKED (transitive) |
| `iso16757` | BLOCKED (transitive) |
| `vdi3805` | BLOCKED (transitive) |

## Logs

- `🗑️generated/gate/poll-loop.txt` — poll timeline
- `🗑️generated/gate/gate-run-attempt-*.txt` — nx outputs during poll
- `🗑️generated/gate/gate-run-final.txt` — final attempt after deadline
