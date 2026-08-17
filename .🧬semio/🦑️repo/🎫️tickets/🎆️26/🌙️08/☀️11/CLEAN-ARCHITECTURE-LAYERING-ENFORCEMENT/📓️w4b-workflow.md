# W4b — workflow module: s. → os. schema-id rename

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`

## Scope

Rename the `s.`-namespaced os-shell schema ids in this file to `os.`:

- `s.workflow` → `os.workflow`
- `s.run` → `os.run`
- `s.automation` → `os.automation`

`s.stdio.*` is unrelated (separate in-progress work) — grepped for it in this file, none present, nothing touched.

## Occurrences updated (10 total)

| Line | Kind | Before | After |
|---|---|---|---|
| 17 | doc comment | `` `s.workflow` `` | `` `os.workflow` `` |
| 21 | const value | `pub const S_WORKFLOW_SCHEMA: &str = "s.workflow";` | `"os.workflow"` |
| 23 | doc comment | `` `s.run`/`s.automation` `` | `` `os.run`/`os.automation` `` |
| 28 | const value | `pub const S_RUN_SCHEMA: &str = "s.run";` | `"os.run"` |
| 29 | const value | `pub const S_AUTOMATION_SCHEMA: &str = "s.automation";` | `"os.automation"` |
| 1014 | doc comment | `` `s.workflow` `` | `` `os.workflow` `` |
| 1019 | `#[dsl(id = ...)]` attribute | `#[dsl(id = "s.workflow")]` | `#[dsl(id = "os.workflow")]` |
| 1714 | doc comment | `` `s.automation` `` | `` `os.automation` `` |
| 1869 | doc comment | `` `s.run` `` | `` `os.run` `` |
| 2541 | test fixture literal | `automation_ref: "s.automation/a1".into()` | `automation_ref: "os.automation/a1".into()` |

Const identifiers (`S_WORKFLOW_SCHEMA`, `S_RUN_SCHEMA`, `S_AUTOMATION_SCHEMA`) were left unchanged — task scope was the string literal values, not the Rust identifiers. All 5 call sites that reference these consts (`WorkflowSnapshot` default, `S_RUN_SCHEMA.into()` at 1901, both test-fixture `schema:` assignments at 2335, and both `assert_eq!` at 2353/2511) pick up the new value automatically since they go through the const, not a literal.

## Verification

Confirmed via grep no remaining `s.workflow` / `s.run` / `s.automation` literals in the file, and no `s.stdio` occurrences existed to accidentally touch.

Crate mounting this module: `semio-framework-os-flow` (confirmed via `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml`, `package.metadata.semio.id = "os-flow"`).

```
cargo check -p semio-framework-os-flow
```

Result: **0 errors**. Build finished (`Finished dev profile [unoptimized] target(s) in 35.16s`) with only pre-existing unrelated warnings (unused imports, private-interface lint in `vcs`/`brep-geometry`/`registry` components — not touched by this change).
