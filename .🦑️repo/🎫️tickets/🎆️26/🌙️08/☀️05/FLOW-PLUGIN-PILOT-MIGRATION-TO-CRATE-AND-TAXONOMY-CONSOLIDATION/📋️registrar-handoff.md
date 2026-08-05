# 📋️ Registrar handoff — 🌊️flow (W1 pilot)

The merged crate is built, verified and the 8 old crates are deleted. The root workspace is RED until a
registrar pass lands the edits below — the 8 member lines now point at directories that no longer exist.
Nothing else in the repo is broken (verified: `cargo metadata` reports only these 8, and
`dependency-cruiser` is clean).

## 1. root `Cargo.toml` — remove these 8 member lines (currently lines 263–271, minus the bim line)

```
    "✏️s/🔌️plugins/🌊️flow/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
```

**KEEP** `"✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/⚡️implementations/🦀️rust"` (line 264) — the bim
extension is out of scope and untouched.

## 2. root `Cargo.toml` — add one member line

```
    "✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust",
```

## 3. root `Cargo.toml` — remove the now-dangling `[workspace.dependencies]` row (line 705)

```
semio-s-app-flow = { path = "✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/⚡️implementations/🦀️rust" }  # 6 refs
```

Nothing referenced it via `workspace = true`; its last real consumer (the dsl fixture sweep) was
repointed at `semio-framework-os-kernel-flow-core` as part of this ticket.

## 4. Then, in the same serialized pass

1. `cargo metadata` to settle `Cargo.lock`.
2. Regenerate the plugin registry: `bun nx run @semio-tech/plugin-registry:generate` — flow's
   `cratePath` moves to `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust`; `packageName`, `wasmOut`,
   `pluginId`, `contributes`/`consumes` and the playground row (variant `flow`, ports react 6016 /
   wgpu 6116) are all unchanged.
3. Regenerate `.vscode/launch.json` — expected to be a **no-op** (ports and variant unchanged).
4. Re-run the checks this ticket could not: `bun …/📇️registry/📜️script.ts check` (now reaches the
   taxonomy audit — expected clean, mirrored locally by `🔍️taxonomy-audit.ts` in this folder),
   `bun nx run @semio-tech/framework-os-dev:plugin -- flow`, `bun ./📜️script.ts dev flow` boot smoke,
   `bun ./📜️script.ts verify gate`, `cargo check --workspace`.

## Files edited outside the flow plugin dir

| File | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🦀️rust/📦️lib.rs` | `app_commands!`: added keyed (`"id" as "key"`) and `ctx = <T>` arms + attribute passthrough; `testkit::assert_constitutional_crates` made taxonomy-aware |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/Cargo.toml` | `flow_app` dev-dep repointed from the deleted `semio-s-app-flow` facade to `semio-framework-os-kernel-flow-core` (the real owner of `FlowFixture`); no `lib.rs` change needed |
