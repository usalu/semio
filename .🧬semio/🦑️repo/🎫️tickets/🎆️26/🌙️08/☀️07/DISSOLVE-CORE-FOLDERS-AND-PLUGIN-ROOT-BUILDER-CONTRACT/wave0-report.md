# Wave 0 report

## Created
- 🔌️plugin/🏗️builder/🦀️component.rs (typestate PluginBuilder)
- ticket 🎫️ticket.json + mcp-unavailable.txt

## Updated
- 🔣️taxonomy.json: pluginDirName, pluginChildDirs, bannedNameStems, requireEmojiPrefixWithVs16
- 🔍️discovery/🟦️component.ts: Taxonomy fields + validateTaxonomy checks
- 🔌️plugin/🦀️component.rs: PluginBundle→Plugin, Plugin→PluginProgram, register_app_factory, builder wire, soft plugin-root assert
- space + demonstrator: PluginBundle→Plugin
- 📜️script.ts: policyBannedNameStemBreaches, policyEmojiPrefixBreaches, policyPluginRootShapeBreaches, policyPluginBuilderBreaches (medium); rewritten policyTaxonomyLibShapeBreaches
- .dependency-cruiser.cjs: no-core-path (warn)
- registry 📜️script.ts: validateTaxonomyTree soft-checks 🔌️plugin/ when present

## Notes
- semio_plugin! kept as thin wrapper over Plugin::builder until Wave 3 migrates all plugins; deleted in Wave 4
- Rust assert_taxonomy_components validates 🔌️plugin/ shape only when the folder already exists
