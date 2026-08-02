# Total JSON Purge — session file list (2026-08-01)

## Framework kernel
- `🧰️framework/⚡️implementation/🦀️rust/Cargo.toml`
- `🧰️framework/⚡️implementation/🦀️rust/📦️lib.rs`
- `🧰️framework/⚡️implementation/🦀️rust/bindings/TutorialBase.ts`
- `🧰️framework/⚡️implementation/🟦️typescript/📦️index.ts`

## Store sync fixtures
- `🧰️framework/🛍️product/💻️os/🔨️module/🏪️store/🔄️sync/⚡️implementation/🦀️rust/📦️lib.rs`
- `🧰️framework/🛍️product/💻️os/🔨️module/🏪️store/🔄️sync/⚡️implementation/🦀️rust/🧫️fixtures/🔄️basic-remote-operations/🔣️fixture.dsl`
- `🧰️framework/🛍️product/💻️os/🔨️module/🏪️store/🔄️sync/⚡️implementation/🦀️rust/🧫️fixtures/📥️remote-operations-backlog/🔣️fixture.dsl`
- `🧰️framework/🛍️product/💻️os/🔨️module/🏪️store/🔄️sync/⚡️implementation/🦀️rust/🧫️fixtures/📸️snapshot-replaced/🔣️fixture.dsl`

## Plugin / OS / renderers
- `🧰️framework/🛍️product/💻️os/🔨️module/🔌️plugin/⚡️implementation/🦀️rust/📦️lib.rs`
- `🧰️framework/🛍️product/💻️os/⚡️implementation/🦀️rust/📦️lib.rs`
- `🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementation/🦀️rust/📦️lib.rs`
- `🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementation/🟦️typescript/📦️index.tsx`
- `✏️s/🔌️plugin/🎞️animate/🎛️app/🎬️present/🔨️module/⚙️engine/⚡️implementation/🦀️rust/📦️lib.rs`

## Verify
- `cargo check --workspace` — green (2026-08-01)

## Deferred
- Jack graph DSL test migration (`🧰️framework/🔨️module/🧮️math/🕸️graph/🗣️dsl` still uses JSON fixtures in tests)
- Legacy `🔣️fixture.json` manifests (superseded by `.dsl`; JSON files may be removed in a follow-up)
- Animate present `player.js` still parses deck body as JSON text inside `text/dsl` script tag
