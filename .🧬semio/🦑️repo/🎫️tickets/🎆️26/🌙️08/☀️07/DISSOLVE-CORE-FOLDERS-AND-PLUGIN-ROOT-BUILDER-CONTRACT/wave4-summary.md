# Wave 4 — Enforcement Flip

## Done
- Policies `policyBannedNameStemBreaches`, `policyEmojiPrefixBreaches`, `policyPluginRootShapeBreaches`, `policyPluginBuilderBreaches` → **high**
- Wired into `VerifyScript.runGate()` as dissolve-core / plugin-root check
- dependency-cruiser `no-core-path` → **error**
- Taxonomy `areas["✏️s/🔌️plugins"]` → **clean**
- Deleted `semio_plugin!` macro; SDK tests use `Plugin::builder` directly
- Renamed `📚️lib` → `📚️library` (banned stem `lib`)
- Removed empty `🪐️space/🔨️modules/🤝️shared` (banned stem `shared`)
- CAD `🟀️core` dissolved into concept modules + barrel

## Verify blockers (local)
- Xcode license may still block `cargo`/`cc`
- Full `bun ./📜️script.ts verify` / policy lint should be run when toolchain allows
