# Mindmap Registrar Request

Apply this only after the concurrent glTF mount relocation is complete and the consumer edits in the accompanying lease report are present.

```diff
diff --git a/Cargo.toml b/Cargo.toml
--- a/Cargo.toml
+++ b/Cargo.toml
@@
-    "✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust",
@@
-semio-s-mindmap = { path = "✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust" }
```

Then regenerate `Cargo.lock` through the repository's normal Bun/Nx workflow if Cargo removes the now-unreachable package record. Once the resulting root registry has no mindmap member or workspace dependency, remove only these retired code files:

- `✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔨️modules/💭️mindmap/🧩️extension/🦀️component.rs`

`✏️s/🔨️modules/💭️mindmap/AGENTS.md` is explicitly excluded from this request.
