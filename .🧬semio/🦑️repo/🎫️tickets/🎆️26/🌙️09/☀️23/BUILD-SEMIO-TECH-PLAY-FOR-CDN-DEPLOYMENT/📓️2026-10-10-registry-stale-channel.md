# Registry stale-channel audit (read-only)

Question: why does `🤖️generated/🩺️diagnostics.json` refuse 43 plugins with `stale-channel` (descriptor channel 20, host channel 23), and will a fresh build fix it?

Base for repo-relative paths: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` is written as `PLUGIN/`. `📇️registry/` is written as `REG/`.

## Conclusion

- A fresh build does NOT fix it by itself.
- Build targets come from the deployed registry, which withholds the stale plugins. Withheld crates are never cargo-built and never re-described, so their checked-in descriptors stay at channel 20 and stay withheld.
- Fix: build targets must include withheld crates (see "Required change").

## 1. Host channel 23 (single source, consistent)

- Rust: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:26` `pub const CHANNEL_VERSION: u32 = 23;`
- TS twin: `🧰️framework/🛍️products/💻️os/🟦️.ts:3748` `export const APP_CHANNEL_VERSION = 23;`
- Pin: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json` `channelVersion: 23`, asserted by both sides.
- Descriptor contract: `PLUGIN/REG/🛂️descriptor-verification/🧬️schema/🔣️.json:256` `appChannelVersion.const: 23`.
- Host constant: `PLUGIN/REG/🧬️schema/🟦️.ts:9` `REGISTRY_HOST_APP_CHANNEL_VERSION` reads that const.
- Gate: `PLUGIN/REG/🧬️schema/🟦️.ts:26` throws `StaleChannelDescriptorError` when descriptor channel differs.
- Discovery: `PLUGIN/REG/🔎️discovery/🟦️.ts:343-358` (`generatePluginRegistryReport`) collects `stale-channel` rows and throws unless `staleChannel: "exclude"`.

## 2. Where a descriptor's channel comes from

- The value is compiled into each plugin component, not written by hand: `PLUGIN/🛂️describe/🦀️.rs:203` and `:311` `ExecutionProtocol { app_channel_version: protocol::CHANNEL_VERSION }`. The native describe binary (`PLUGIN/🖨️describe/🛂️descriptor-emission/🟦️.ts`) only extracts it.
- So a re-described crate emits 23 automatically, provided its wasm is recompiled against the current workspace (path deps on `os_spr`).
- Storage: tracked (not gitignored) owner-root files `<owner>/🔣️.json` plus `<owner>/🛂️.descriptor.semio`. Path rule: `PLUGIN/REG/🔎️discovery/🟦️.ts:140-156`, `DESCRIPTOR_JSON_REL_PATH = ../../🔣️.json` (taxonomy `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:27188`).
- Rewritten only by `PLUGIN/🏗️build/🛂️descriptor/🟦️.ts:91-103` `describeBuiltPlugin` (writes at 101-102). Its only caller is `PLUGIN/🏗️build/📦️materialization/🟦️.ts:154` in `materializePlugin`, which runs after `buildPluginCargo` (about 120-130).
- `dist/` is irrelevant: descriptors live at owner roots, not in `dist/`.

Current descriptor state (verified by reading files):

- The 43 refused plugins all have owner-root `🔣️.json` at channel 20, mtimes Oct 1 to Oct 8. 40 are under `🌎️hub/🧩️compositions/<x>/` and 3 under `✏️s/🔌️plugins/` (`📜️imperative/🧩️extensions/📣️effect`, `📜️imperative/🧩️extensions/🧠️logic`, `📖️playbook/🧩️extensions/🌀️procedural`).
- Admitted: `bim`, `draw`, `puzzle` (`🌎️hub/🧩️compositions/{🏙️bim,🖍️draw,🧩️puzzle}/🔣️.json`). Each is at channel 23 with mtime Oct 9 05:53, Oct 6 00:32 and Oct 7 10:37. They were last described after the host moved to 23, and none depends on a stale plugin.
- The 24 `stale-channel-dependency` rows are extensions at channel 23 whose host (`flow`, `cad`, `imperative`, `process`, `sourcing`) or dependency (`procedural` needs `flow-extension-bim`) is stale. They are withheld by propagation only.

## 3. nx targets

- Plugin crates have no `describe` target. Descriptors are produced inside the plugin build: `build` / `build-fresh` to `PLUGIN/🏗️build/🏃️execution/🟦️.ts` `buildPlugins` to `buildPluginCatalog` to `materializePlugin`.
- The only `describe` nx target is `PLUGIN/🖨️describe/📦️packages/🦀️rust/📋️project.json:45`. It runs the native describe binary (dependsOn its `build`) and does not iterate over plugins.
- Target selection (the blocking step):
  - `PLUGIN/🏗️build/🏃️execution/🟦️.ts:113-116` `preparePluginBuildTargets` calls `ensurePluginRegistry(filterPlugin)`, then reads `filterProjectedPluginRegistry(readGeneratedCatalogProjection(), ...)` (line 115).
  - `REG/🔄️refresh/🟦️.ts:75-80` `ensurePluginRegistry` runs registry `generate` (exclude, so withholding applies), then syncs descriptors only for admitted rows (line 79).
  - `PLUGIN/🏗️build/📋️plan/🟦️.ts:66-76` `resolvePluginBuildTargets` takes its input from the same deployed projection. `SEMIO_PLUGIN_ONLY` (lines 67-70) matches only deployed entries, and so does the filter at line 73.
  - `REG/📽️projection/🟦️.ts:353` `GenerateScript` renders with `"exclude"`, which is what writes the 67 diagnostics and the 3-row `🔌️plugins.json`.
- Result: the registry contains 3 rows, so only 3 crates are cargo-built and described. The 43 stale crates are never targeted. Their descriptors are never rewritten, so they are never admitted. This is a loop with no exit.
- Release path: `🏢️semio-tech/🎡️play/📋️project.json:260-270` `build-fresh` to `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts:15-25` to nx `@semio-tech/semio-tech-play:build --skip-nx-cache` (see `🆕️fresh-build/🟦️.ts`), which depends on `@semio-tech/framework-os-dev:prepare-*-react-release`. I did not trace each `prepare-*` target to the end. Their variant builds appear to take the same projection-based path.
- Also: `📜️script.ts:33` (`CatalogScript`) renders with `"refuse"`, so the play catalog step will fail while any plugin is stale.

## Required change

1. `PLUGIN/🏗️build/🏃️execution/🟦️.ts:115`, `PLUGIN/🏗️build/📋️plan/🟦️.ts:73` and `REG/🔄️refresh/🟦️.ts:79`: build targets must come from the discovered crate set including withheld rows (the pre-channel-filter entries of `generatePluginRegistryReport`, which already carry `pluginId` and `cratePath` at `REG/🔎️discovery/🟦️.ts:354`), not from the deployed projection. Same for `PLUGIN/🏗️build/📋️plan/🟦️.ts:67-70` (`SEMIO_PLUGIN_ONLY`).
2. With that change, `materializePlugin` (`PLUGIN/🏗️build/📦️materialization/🟦️.ts:154`) re-describes the 43 crates, `describeBuiltPlugin` rewrites the 43 owner-root `🔣️.json` files at channel 23, and `generate` admits them. The 43 rewritten descriptors are tracked files and will appear as git changes.
3. Unverified: whether the 43 crates compile against the current SDK. They were not built, per the rules. Expect possible compile failures, since their code last built at channel 20.

## Caveats

- The working tree is being edited concurrently. During this audit the identifiers `staleChannel` (discovery, projection, `buildPlugins`) flipped to a mangled `n` and back, which looks like an in-progress rename by a peer. Line numbers may shift. Re-check before editing.
- The `semio` MCP server failed to connect; it was not needed for this audit.
- No files were written except this report.
