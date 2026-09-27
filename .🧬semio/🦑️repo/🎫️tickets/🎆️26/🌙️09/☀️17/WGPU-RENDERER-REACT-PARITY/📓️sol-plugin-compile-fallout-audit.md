# Plugin Compile Fallout Audit

The native Shell run reported these three `semio-framework-plugin` diagnostics without source locations:

- `Label: AsRef<str>` is not satisfied.
- A two-argument function received one argument.
- `try_id` was called on `Result<T, E>`.

Together they identify one call shape: the two-argument plugin panel helper `tree_item(id, label) -> UiAssemblyResult<BuiltNode>` had been called with a single `Label`, and the returned `Result` was then treated as a contract builder. The in-flight Document window implementation was the only changed plugin block matching that shape.

Current shared source at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` uses `ui::tree_item(ui_label(...)?)`, explicitly selecting the one-argument semantic UI contract builder whose return type is `TreeItemBuilder`. The adjacent `TextEditorScene::base` call also supplies its current three arguments. No invalid call remains in the current source, so the failing run compiled an earlier in-flight snapshot. No plugin source was changed during this audit.
