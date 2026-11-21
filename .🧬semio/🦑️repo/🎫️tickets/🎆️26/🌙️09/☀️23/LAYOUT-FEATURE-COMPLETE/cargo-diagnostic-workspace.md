# Cargo Diagnostic Paths in Nested Workspaces

The component pass in run `w4Rn48Y` received an error from `semio_hub_layout` whose compiler span was `🧩️compositions/📏️layout/📦️packages/🦀️rust/../../🦀️.rs`. Cargo emitted the span relative to `🌎️hub`. Recovery resolved it against the repository root and classified the resulting missing path as a concurrent deletion. That false classification replaced the original compiler failure with restart status 75.

Source admission now carries `workspace_root` from the Cargo metadata of the admitted invocation into the component pass, including when the pass is re-admitted for retry. Recovery resolves ordinary relative spans against that workspace. Package-version remaps continue to use the package manifest, absolute paths retain their anchors, and the repository root still determines which paths the pass probes.

## Verification

- Both language-neutral nested-workspace cases failed before the fix because the span lost its workspace prefix. Both now pass. The SQLite file-state oracle covers an unchanged compiler failure and a genuinely vanished source.
- Scoped Bun verification passed 28 failure-churn tests, 10 component-pass tests, and 6 source-admission tests. The remaining source-admission case requires an original script-process envelope and cannot run under bare Bun. The clean component selection excludes native builds.
- Replayed an authentic `semio_hub_layout` error from `w4Rn48Y/l/cargo-prototype-00-retry.jsonl`. The helper returned the existing Cargo manifest, package entry, and `🌎️hub/🧩️compositions/📏️layout/🦀️.rs`. With unchanged timestamps, the concurrent-edit probe returned no edit.
- The changed source and fixtures passed `repo tools exec --light git -- diff --check`.
