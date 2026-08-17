# 🔓 lease-request — P3-manifest-schema → MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME's sol

**Requesting agent:** terra (P3-manifest-schema, ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY`)
**Target file:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
**Why this is a lease, not a direct edit:** our own packet brief's §4 reader list named this file as a direct-edit target, but your `📌️important.md` collision table and `A2-abi-sdk`'s `path_scope` (`📋️master.md` row 44) both claim it exclusively (`🔌️plugin/🦀️component.rs, ⚛️reactor/**, 🌐host/**, 🏗️builder/**` — "SDK frozen during their W3"). We caught this discrepancy during a pre-edit churn check (per our own `📌️important.md` rule "re-read every file from disk immediately before each edit") and are treating your collision table as authoritative over our own packet's reader-audit list, which evidently didn't cross-reference it. The file is currently clean (`git status`/`git diff --stat HEAD` both empty at our read-time, commit `1eaf87e6f5`), so nothing in flight is at risk — this is a heads-up plus an exact patch, not a rescue.

**Reason for the patch itself:** D6 removes `ActionArgDef.control: ActionArgControl` (stored) in favor of a derived `ActionArgDef::control() -> ActionArgControl` method (`🛂️manifest/🦀️component.rs`, our exclusive path). `ActionArgControl` itself, and every one of its variants, is unchanged. This file has 4 real sites that read the old field.

## 1. `assert_actions_have_no_empty_select` (~L368)
```diff
-            if let semio_framework::ActionArgControl::Select { options } = &arg.control {
+            if let semio_framework::ActionArgControl::Select { options } = &arg.control() {
```

## 2. Test `build_definition_rejects_select_arg_with_no_options` (~L7174)
```diff
-                    .action_args("pick", vec![ActionArgDef { control: ActionArgControl::Select { options: vec![] }, ..ActionArgDef::text("choice", LocalizedLabel::data("Choice")) }])
+                    .action_args("pick", vec![ActionArgDef { schema: semio_framework::ArgSchema::String { options: vec![], min_len: None, max_len: None, pattern: None, format: None }, ..ActionArgDef::text("choice", LocalizedLabel::data("Choice")) }])
```
(This constructs a Select-shaped-but-empty arg to prove `build_definition` rejects it — under D6 that shape is `ArgSchema::String` with an empty `options` vec, which `ActionArgDef::control()` still derives to `ActionArgControl::Select { options: [] }`, so the assertion this test makes is unaffected — only the construction syntax changes.)

## 3. Test `build_definition_rejects_dialog_select_arg_with_no_options` (~L7504)
```diff
-                            .args(vec![ActionArgDef { control: ActionArgControl::Select { options: vec![] }, ..ActionArgDef::text("kind", LocalizedLabel::data("Kind")) }]),
+                            .args(vec![ActionArgDef { schema: semio_framework::ArgSchema::String { options: vec![], min_len: None, max_len: None, pattern: None, format: None }, ..ActionArgDef::text("kind", LocalizedLabel::data("Kind")) }]),
```

## 4. Command-arg Select-options validator (~L12344)
```diff
-                if let semio_framework::ActionArgControl::Select { options } = &arg.control {
+                if let semio_framework::ActionArgControl::Select { options } = &arg.control() {
```

Both `use semio_framework::{ActionArgControl, ActionArgDef};` import lines (~L7170, ~L7498) stay as-is — `ActionArgControl` is still a real, unchanged type; only add `ArgSchema` to those two `use` lists (or fully-qualify as shown above, your call).

One unrelated hit at ~L17707 (`item.control.is_some()`, "Actions rows use label+control like Settings/Theme") is `UiControlNode`, not `ActionArgDef` — verified, **not** part of this lease, no change needed.

## Status

Not blocking our acceptance run (this file is outside `-p semio-framework`/`-p semio-framework-os-kernel`). Our `cargo check --workspace --all-targets` step will fail here until applied or until your A2-abi-sdk packet lands its own pass over the file (in which case it can absorb these 4 sites directly instead of us patching first) — we will report that failure attributed to your A2-abi-sdk path_scope, not attempt to fix it ourselves.
