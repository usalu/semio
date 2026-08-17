# Dissolving Core Folders And Plugin Root Builder Contract — Completion

## Verdict
Product-tree `core` folders are dissolved; all 32 plugins expose `🔌️plugin/` via `Plugin::builder`; Wave 4 enforcement is flipped to high/error.

## Waves
1. **Wave 0** — taxonomy, PluginBuilder SDK, policies (medium), cruiser warn
2. **Wave 1** — dissolve framework/flow/pack/db/spr/dsl/ui/plugin/animate/cad cores
3. **Wave 2** — deferred shared renames (`flow_core`→`flow`, framework-core→framework, …)
4. **Wave 3** — 32× `🔌️plugin/` roots; glue → `plugin_exports!(plugin::plugin)`
5. **Wave 4** — policies high, cruiser error, plugins area `clean`, delete `semio_plugin!`, rename `📚️lib`→`📚️library`, VS16 emoji-prefix fixups

## CAD
`🟀️core` split into `📔️registry`, `📐️geometry`, `🧬️typology`, `🗺️spatial`, `🎬️actions`, `📄️document`, `🧪tests`(→`🧪️tests`) + `🟦️index.ts` barrel.

## Local verify
Xcode license / toolchain may block full `cargo` + `bun ./📜️script.ts verify`. Policy wiring is in `VerifyScript.runGate()`.
