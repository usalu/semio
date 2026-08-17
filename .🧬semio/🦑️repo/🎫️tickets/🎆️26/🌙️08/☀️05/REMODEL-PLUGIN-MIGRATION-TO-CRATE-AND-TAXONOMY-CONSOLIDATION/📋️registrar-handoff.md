# 📋️ Registrar handoff — 📸️remodel

The migrating agent never edits root `Cargo.toml`/`Cargo.lock`, the registry script or `launch.json`
(TEMPLATE.md §10). Everything below is for the serialized registrar pass.

**Shape:** V2 (`26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST`) — the entry file lives at
`✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📦️lib.rs` with `[lib] path = "📦️lib.rs"`, and the member
line below is unchanged by that (it names the package dir, not the entry file).

## Remove these member lines from root `Cargo.toml`

```
    "✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/🖼️images/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/🎥️video/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/📷️camera/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/🌟️feature/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/📸️sfm/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/🌫️dense/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/🥽️mesh/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/🏃️motion/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/🗺️geo/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📸️remodel/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
```

18 lines out. **Add:**

```
    "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust",
```

## `[workspace.dependencies]`

Remove (all three are now dead — verified: no crate anywhere in the repo consumes them via
`workspace = true`, and every `path =` consumer was intra-remodel):

```
semio-s-app-remodel = { path = "✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/⚡️implementations/🦀️rust" }  # 8 refs
semio-s-plugin-remodel-camera = { path = "✏️s/🔌️plugins/📸️remodel/🔨️modules/📷️camera/⚡️implementations/🦀️rust" }  # 7 refs
semio-s-plugin-remodel-image = { path = "✏️s/🔌️plugins/📸️remodel/🔨️modules/🖼️images/⚡️implementations/🦀️rust" }  # 10 refs
```

Add, matching the other migrated plugins' rows:

```
semio-s-plugin-remodel = { path = "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust" }
```

The new crate already uses `{ workspace = true }` for `serde`, `serde_json`, `semio-framework-core`,
`semio-framework-os` and `semio-framework-plugin` (the last with `features = ["component-guest"]`), so
those resolve the moment the member line lands — no rewrite needed. Every renamed/aliased internal dep
(`dsl`/`store`/`protocol`/`pack`/`mathematical_*`) stays a plain `path =` + `package =` pair, per
TEMPLATE §13.4.

## Cross-cutting edits already applied by this agent

* `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/Cargo.toml`
  — `remodel` repointed from `semio-s-app-remodel` to the new `semio-s-plugin-remodel` crate.
* the same crate's `📦️lib.rs` — `use remodel::RemodelProjection` → `use remodel::artifacts::remodel::RemodelProjection`.
  (`RemodelProjection` is genuinely plugin-owned: `pub struct RemodelProjection` was defined in remodel's own app
  facade crate, not in any kernel crate — grep-verified, per the master doc's `DagDocument` lesson.)

That is the **only** cross-cutting dependent. Nothing else in the repo — no other plugin, no framework
crate, no `dsl/📇️registry` entry — referenced any of remodel's 18 old crate names, in either
`[dependencies]`/`[dev-dependencies]` or a `use <old-crate>::` line.

## Still un-run (needs a healthy workspace — not the migrating agent's step)

* `cargo metadata` / `cargo check --workspace`
* `bun 🧰️framework/…/📇️registry/📜️script.ts check` and the registry/`launch.json` regeneration
  (playground ports are UNCHANGED — `react = 6063`, `wgpu = 6163` — so the `launch.json` regen should
  be a no-op for remodel)
* `bun nx run @semio-tech/framework-os-dev:plugin -- remodel`, `bun ./📜️script.ts dev remodel`,
  `bun ./📜️script.ts verify gate`
