# 📋️ Registrar handoff — 🏛️architect

Remove these member lines from root `Cargo.toml` (currently lines 389–397):

```
    "✏️s/🔌️plugins/🏛️architect/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏛️architect/🔨️modules/🦴️spine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
```

Add:

```
    "✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust",
```

`[workspace.dependencies]`: **nothing to remove** — architect never had an alias there (verified by
grepping the whole table). **Nothing to add either**; the new crate already uses
`serde.workspace = true`, `serde_json.workspace = true`, `thiserror.workspace = true`, all three of
which are already present in the root table (`thiserror = "2.0.18"`, i.e. a 2.0.12 → 2.0.18 bump for
this crate, verified green: check + clippy + 250 tests). The four renamed internal path deps
(`store`/`protocol`/`dsl`/`mathematical_graph`) stay plain `path =` + `package =` pairs per
TEMPLATE §13.4 — `workspace = true` + a renamed `package =` does not resolve.

## Cross-cutting edits already applied by this agent

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️implementations/🟦️typescript/🧪️index.test.ts:1100` —
  a stale `resolveCargoPackageName` fixture asserted the now-deleted spine-module path/package name.
  Repointed to `("semio-s-plugin-architect", "✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust")`, matching
  the `semio-s-plugin-energy` row right below it. Same class of stale fixture 🔋️energy's migration hit.

## Cross-plugin dependents

**None.** Verified two ways per the note-plugin lesson:
1. `grep -rn "semio-s-plugin-architect-spine\|semio-s-plugin-architect-spine-module\|semio-s-app-architect" --include=Cargo.toml .`
   → every hit was inside `✏️s/🔌️plugins/🏛️architect/` itself (its own old crates).
2. `grep -rn --include="*.rs" -e architect_spine -e "architect_op\b" -e architect_engine
   -e architect_protocol -e semio_s_app_architect -e semio_s_plugin_architect .` → zero hits outside
   the plugin dir.
Specifically checked and CLEAN: `🧰️framework/…/🗣️dsl/🧪️fixture-sweep` (no architect dep, no `use` line)
and `🧰️framework/…/🗣️dsl/📇️registry` (no architect reference at all). 🎪️demonstrator does not depend on
architect. This matches the approved plan's own note ("🏛️architect absorbs 🦴️spine … no external
dependents") — confirmed empirically, not assumed.

## Commands still un-run (need a healthy root workspace — registrar's, not mine)

- `cargo check -p semio-s-plugin-architect` / `cargo metadata` / `cargo check --workspace`
- `bun 🧰️framework/…/📇️registry/📜️script.ts check` (+ `generate`)
- `bun nx run @semio-tech/framework-os-dev:plugin -- architect`
- `bun ./📜️script.ts dev architect`, `bun ./📜️script.ts verify gate`

Root workspace was red for the whole session from OTHER sessions' in-flight migrations —
`📜️imperative` and `📕️norm` both have member lines pointing at already-deleted crate dirs
(`failed to load manifest for workspace member …/📜️imperative/🛂️manifest/…` and
`…/📕️norm/🔨️modules/🫀️core/…`). Not architect-caused; noted so the next registrar pass isn't surprised.
