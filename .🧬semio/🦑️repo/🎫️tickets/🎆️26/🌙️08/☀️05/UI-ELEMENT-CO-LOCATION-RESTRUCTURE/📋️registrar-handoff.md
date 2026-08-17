# 📋️ Registrar handoff — UI-ELEMENT-CO-LOCATION-RESTRUCTURE (finish pass)

Outside exclusive ownership `🖱️ui/**`. Root `Cargo.toml` only.

## Problem
Workspace member path is stale after repo CLI Shape V2 move. Blocks all `cargo check -p …` including `semio-framework-ui`.

## Root `Cargo.toml` — `[workspace].members`

**Remove:**
```
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/⚡️implementations/🦀️rust",
```

**Add (or replace with):**
```
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust",
```

Confirm package name at the new path before applying (read its `Cargo.toml` `[package].name`).
Update `[workspace.dependencies]` path for that package if an alias still points at the Shape-V1 path.

## Not requested
- Do **not** recreate `🖱️ui/📦️packages/🦀️rust/🎯️targets/*/Cargo.toml`
- Do **not** change `semio-framework-ui` workspace member/dependency (already correct)
