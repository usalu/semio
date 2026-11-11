# 🧪 Norm Test Map

- `repo run @semio-tech/norm-js:test` — language-agnostic suite (`🧪️tests/`, bun:test + JSON fixtures), project `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript`; also `:test-wire-twins`.
- Per-artifact `check`/`test` targets for each `🗿️artifacts/*` (Rust + TS), plus `test-snapshot-sqlite[-source|-native]` on most Rust artifacts.
- Hub `🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust`: `test`, `check`, `surface-render-test`, `results-window-config-test`, `component-budget-check`, `mutation-leaf-taxonomy-check`.
- Dev `✏️s/🧑‍💻dev/📕️norm/📦️packages/🟦️typescript`: `verify-norm-document-contract`, `verify-norm-results-window-ownership`.
- Layout: `🧪️tests/` at plugin root and subprojects, `🧫️fixtures/` JSON, `🧬️schema-sync` (Python), `🔮️catalogue`, `🧬️mutation-aggregate`, `🔮️oracles/`.
- Cached results: only en1995 subset mutate oracle (Python) 415/415 pass; norm-artifact-contract-rs compile step pass. No other norm runs cached → most norm targets never executed.
