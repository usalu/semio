## Endgame — ✏️editor/🦀️component.rs (editor root)

Scope note: NONE of my four fixes are compile-verified. Every `cargo check -p semio-s-plugin-lowpoly`
run I attempted was blocked before reaching the lowpoly crate — first by `semio-framework-os-kernel`
serde errors (`SpaceAlternative`/`SpaceCheckpoint`/`OwnerRef`, unrelated concurrent wave), then by a
`framework-graph` taxonomy-schema build-script failure (`jco-package-adapter` missing generated file,
also unrelated). No run in this session showed `semio-s-plugin-lowpoly` being compiled at all.

### 1. `:281` `ToValue`/`FromValue` unreachable from `$crate` — WRITTEN, unverified
Root cause confirmed by reading, not guessed: `app_commands!`'s doc comment
(`🧰️framework/…/🔌️plugin/🦀️component.rs:10523`) states both derives "are spelled via `$crate` … both
are proc-macros re-exported at this crate's own root (`📦️glue.rs`)" — but
`🧰️framework/…/🔌️plugin/📦️packages/🦀️rust/📦️glue.rs` had no such re-export (grepped the whole
component.rs and glue.rs, confirmed absent). This is a framework gap, not fixable from my file alone —
`$crate::ToValue` resolves against the *defining* crate's root regardless of what the invoker imports.
Added the missing re-export to that glue.rs (framework file, not lowpoly, but additive/1-line and
matches the documented intent):
```rust
pub use semio_framework_value_derive::{FromValue, ToValue};
```

### 2. `:2099` `ComponentTree: serde::Serialize` — WRITTEN, unverified
`ComponentTree` (`🧰️framework/…/🖱️ui/🧠️runtime/…/🦀️present.rs:84`) derives only `Debug`; its single
field `root: TreeNode` (= `ui_contract::BuiltNode`) does derive `Serialize`. Changed testkit `render()`
to serialize `.root` instead of the whole tree.

### 3. `:2110` `&DslValue` vs `&Value` — WRITTEN, unverified
`handle_action`'s `args: Option<&DslValue>` (was `&serde_json::Value`). Confirmed
`impl From<&serde_json::Value> for DslValue` exists (`🧰️framework/🔨️modules/🌱️value/🦀️component.rs:134`).
Wrapped the existing `serde_json::json!({...})` call site in `protocol::DslValue::from(&...)`.

### 4. `:2086`/`:2091` missing `.await` — WRITTEN, unverified
`testkit::new_app`/`new_app_with_registry` are now `pub async fn`. Made `app()`/`app_with_registry()`
async and added `.await` to both, plus the 3 in-file call sites (`app().await` x2, `app_with_registry()
.await` x1). Left `lowpoly_manifest_for_testkit()` sync — `new_app_with_registry` takes a plain `fn()
-> App` pointer, not a future.

### Known follow-up (not mine to edit, reporting as handoff)
5 call sites outside my file still call `app()`/`app_with_registry()` without `.await` — now a type
error since I made them async. Owned by the modes/panels/commands agent (already reported done, so
likely stale relative to my change — flagging, not editing):
- `✏️editor/📌️panels/📄️artifact/🦀️component.rs:121`
- `✏️editor/🎮️commands/🧲️transform/🦀️component.rs:120`
- `✏️editor/🎮️commands/➕️add-primitive/🦀️component.rs:54,63,81`

### Bottom line
All 4 assigned fixes are written and individually reasoned against the actual current framework
signatures (not guessed), but zero of them have a green `cargo check` behind them this session — every
attempt died upstream before lowpoly compiled. Needs a clean-tree run to confirm.
