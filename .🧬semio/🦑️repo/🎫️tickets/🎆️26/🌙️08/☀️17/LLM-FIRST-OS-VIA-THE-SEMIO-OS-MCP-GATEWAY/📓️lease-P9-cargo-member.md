# 🔓 lease-request — P9-shellstate-module → sol

**Requesting agent:** terra (P9-shellstate-module)
**Target files (registrar-only, sol edits):** root `Cargo.toml`, root `package.json`
**Reason:** new crate `semio-framework-os-shell` / package `@semio-tech/framework-os-shell` cannot build via `cargo test -p semio-framework-os-shell` until it is a workspace member; the TS package cannot be resolved by bun/nx until it is a `package.json` workspace entry. Per packet §5 this lease is emitted in the first few minutes, before the acceptance run.

## 1. `Cargo.toml` `[workspace] members` — insert one line

Insertion point: the `💻️os/🔨️modules/*` cluster starting at (current) line 7. The cluster is not in strict codepoint order today (`🧠️neural` at line 9 sorts after `🏃️run` at line 10 despite `🧠` > `🏃` numerically), so exact placement is a judgment call — placing `🖥️shell` immediately after the `🔌️plugin/🖥️host` line keeps same-emoji-prefix (`🖥️`) entries adjacent, which is the nearest visible convention:

```diff
     "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust",
     "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust",
+    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust",
     "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust",
```

Exact line to add (current file uses 4-space indent, trailing comma):
```toml
    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust",
```

## 2. `Cargo.toml` `[workspace.dependencies]` — alias

Sibling greenfield crates (e.g. `semio-framework-actor`) get an alias immediately on landing even before downstream consumers exist, so later adoption packets (H1–H4, the MCP gateway) don't need a second lease just to depend on this crate. Requesting the same:

```toml
semio-framework-os-shell = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust" }
```

Suggested position: alongside the other `💻️os/🔨️modules/*` aliases (near `semio-framework-plugin`, line ~170 today).

## 3. Root `package.json` `workspaces` — insert one line

Within the `🧰️` (framework) alphabetical block, alongside the other `💻️os/🔨️modules/*/📦️packages/🟦️typescript` entries (e.g. immediately before or after `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript`):

```json
"🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript",
```

## 4. Status

Not blocking — continuing to build the module against a local `CARGO_TARGET_DIR` per-crate `cargo test`/`cargo build` invocation that does not require workspace membership for authoring/compiling (it will need `-p` to resolve once wired, or can be checked via `cargo check --manifest-path` directly against the crate's own `Cargo.toml` in the interim). Will run full `-p semio-framework-os-shell` acceptance once sol confirms this lease applied, and will say plainly in the report if that confirmation has not landed yet.
