//! 🧭 `semio`: the monorepo orchestrator CLI + TUI dashboard, replacing `script.ts`'s dev/build front door.

use std::io::IsTerminal;
use std::path::PathBuf;

// #region 🔖Args
pub mod args {
    use std::collections::HashMap;

    /// 🎛️ One `semio <verb> [segments…] [--flag [value]]` invocation, split into its parts.
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct ParsedArgs {
        pub verb: String,
        pub segments: Vec<String>,
        pub flags: HashMap<String, Option<String>>,
    }

    impl ParsedArgs {
        pub fn flag(&self, name: &str) -> Option<&str> {
            self.flags.get(name).and_then(|v| v.as_deref())
        }

        pub fn has_flag(&self, name: &str) -> bool {
            self.flags.contains_key(name)
        }
    }

    /// ✂️ Splits raw argv into a verb, positional segments, and `--flag [value]` pairs.
    ///
    /// A flag consumes the next token as its value unless that token is itself a `--flag` or
    /// there is no next token, in which case the flag is boolean (`has_flag` only).
    pub fn parse(argv: &[String]) -> ParsedArgs {
        let mut iter = argv.iter().peekable();
        let verb = iter.next().cloned().unwrap_or_default();
        let mut segments = Vec::new();
        let mut flags = HashMap::new();
        while let Some(tok) = iter.next() {
            if let Some(name) = tok.strip_prefix("--") {
                let value = match iter.peek() {
                    Some(next) if !next.starts_with("--") => iter.next().cloned(),
                    _ => None,
                };
                flags.insert(name.to_string(), value);
            } else {
                segments.push(tok.clone());
            }
        }
        ParsedArgs { verb, segments, flags }
    }
}
// #endregion 🔖Args

// #region 🔖Workspace
pub mod workspace {
    use std::path::{Path, PathBuf};

    /// 🗺️ Walks up from `start` to find the Cargo workspace root (a `Cargo.toml` with `[workspace]`).
    pub fn find_root(start: &Path) -> PathBuf {
        let mut dir = start.to_path_buf();
        loop {
            let candidate = dir.join("Cargo.toml");
            if let Ok(text) = std::fs::read_to_string(&candidate) {
                if text.contains("[workspace]") {
                    return dir;
                }
            }
            if !dir.pop() {
                return start.to_path_buf();
            }
        }
    }
}
// #endregion 🔖Workspace

// #region 🔖Proc
pub mod proc {
    use std::path::Path;
    use std::process::Command;

    /// 🏃 Spawns `cmd` with inherited stdio, extending (not replacing) the current environment.
    pub fn spawn_inherit(cmd: &str, args: &[&str], cwd: &Path, env: &[(String, String)]) -> i32 {
        let status = Command::new(cmd).args(args).current_dir(cwd).envs(env.iter().cloned()).status();
        match status {
            Ok(s) => s.code().unwrap_or(1),
            Err(e) => {
                eprintln!("[semio] failed to run {cmd}: {e}");
                1
            }
        }
    }
}
// #endregion 🔖Proc

// #region 🔖Catalog
pub mod catalog {
    use std::fs;
    use std::path::{Path, PathBuf};

    //#region 🔖Entries
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct PluginRegistryEntry {
        pub plugin_id: String,
        pub crate_path: String,
        pub package_name: String,
        pub wasm_out: String,
        pub contributes: Vec<String>,
        pub consumes: Vec<String>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Ports {
        pub react: u32,
        pub wgpu: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct PlaygroundEntry {
        pub variant: String,
        pub plugin_id: String,
        pub crate_path: String,
        pub app: Option<String>,
        pub aliases: Vec<String>,
        pub ports: Ports,
        pub examples: Vec<String>,
        /// 🔌 Crate paths (e.g. `framework/surface/tiled-map/rs`) whose `wasm` build target must run
        /// for this playground variant, read from `engines = […]` on its `[[…playground]]` row.
        pub engines: Vec<String>,
        /// 🗂️ Dev-time asset serving needs for this variant, read from the crate's
        /// `[[package.metadata.semio.assets]]` rows (see [`AssetSpec`]).
        pub assets: Vec<AssetSpec>,
    }

    /// 🗂️ One `[[package.metadata.semio.assets]]` row: a dev-time asset-serving need declared by a
    /// plugin crate (tile proxy, static directory, or mesh collection). `app` optionally scopes the
    /// row to one playground variant of a multi-app crate (unset ⇒ every variant of the crate).
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct AssetSpec {
        pub kind: String,
        pub route: String,
        pub app: Option<String>,
        pub upstream: Option<String>,
        pub cache: Option<String>,
        pub root: Option<String>,
        pub roots: Vec<String>,
        pub placeholder: Option<String>,
        pub filter_from_examples: bool,
    }
    //#endregion 🔖Entries

    //#region 🔖TomlScan
    fn string_field(text: &str, key: &str) -> Option<String> {
        for line in text.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix(&format!("{key} = \"")) {
                if let Some(end) = rest.find('"') {
                    return Some(rest[..end].to_string());
                }
            }
        }
        None
    }

    fn string_array_field(block: &str, key: &str) -> Vec<String> {
        let Some(start) = block.find(&format!("{key} = [")) else { return Vec::new() };
        let Some(open) = block[start..].find('[') else { return Vec::new() };
        let abs_open = start + open;
        let Some(close_rel) = block[abs_open..].find(']') else { return Vec::new() };
        let inner = &block[abs_open + 1..abs_open + close_rel];
        let mut out = Vec::new();
        let mut rest = inner;
        while let Some(open_q) = rest.find('"') {
            rest = &rest[open_q + 1..];
            let Some(close_q) = rest.find('"') else { break };
            out.push(rest[..close_q].to_string());
            rest = &rest[close_q + 1..];
        }
        out
    }

    fn bool_field(block: &str, key: &str) -> bool {
        for line in block.lines() {
            if let Some(rest) = line.trim().strip_prefix(&format!("{key} = ")) {
                return rest.trim() == "true";
            }
        }
        false
    }

    fn ports_field(block: &str) -> Option<Ports> {
        let start = block.find("ports = {")?;
        let open = block[start..].find('{')? + start;
        let close = block[open..].find('}')? + open;
        let inner = &block[open + 1..close];
        let react = inner
            .split(',')
            .find_map(|part| part.trim().strip_prefix("react = ").and_then(|v| v.trim().parse().ok()))?;
        let wgpu = inner
            .split(',')
            .find_map(|part| part.trim().strip_prefix("wgpu = ").and_then(|v| v.trim().parse().ok()))?;
        Some(Ports { react, wgpu })
    }

    /// ✂️ Collects every block of lines directly following a header line matched by `is_header`, up
    /// to (but excluding) the next `[…]` header line — mirrors the TS `tomlBlocksAfterHeader` scan.
    fn blocks_after_header(text: &str, is_header: impl Fn(&str) -> bool) -> Vec<String> {
        let lines: Vec<&str> = text.lines().collect();
        let mut blocks = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if !is_header(line.trim()) {
                continue;
            }
            let mut body = Vec::new();
            for l in &lines[i + 1..] {
                if l.trim_start().starts_with('[') {
                    break;
                }
                body.push(*l);
            }
            blocks.push(body.join("\n"));
        }
        blocks
    }
    //#endregion 🔖TomlScan

    //#region 🔖Discovery
    /// 🚫 Directory names skipped while walking for plugin crates (build/vendor noise and any
    /// dot-directory, e.g. `.claude/worktrees/…`, which used to leak duplicate registry rows).
    fn skip_dir(name: &str) -> bool {
        name.starts_with('.') || matches!(name, "node_modules" | "generated" | "target")
    }

    /// 🔍 Finds every plugin/module crate `Cargo.toml` under `root` (excluding the plugin SDK itself).
    pub fn find_plugin_cargo_files(root: &Path) -> Vec<PathBuf> {
        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            let Ok(entries) = fs::read_dir(dir) else { return };
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if skip_dir(&name) {
                    continue;
                }
                let Ok(meta) = entry.metadata() else { continue };
                if meta.is_dir() {
                    walk(&path, out);
                } else if name == "Cargo.toml" {
                    let path_str = path.to_string_lossy().replace('\\', "/");
                    if path_str.contains("/framework/plugin/rs/") {
                        continue;
                    }
                    let is_plugin = path_str.ends_with("/plugin/rs/Cargo.toml");
                    let is_module = {
                        let segs: Vec<&str> = path_str.split('/').collect();
                        segs.len() >= 4
                            && segs[segs.len() - 1] == "Cargo.toml"
                            && segs[segs.len() - 2] == "rs"
                            && segs[segs.len() - 4] == "module"
                    };
                    // 🏛️ Post-restructure plugin bundle crate: `s/plugin/<p>/rs/Cargo.toml` absorbs the
                    // former `<tech>/plugin/rs`. Kept alongside the legacy patterns during the migration.
                    let is_plugin_bundle = {
                        let segs: Vec<&str> = path_str.split('/').collect();
                        segs.len() >= 5
                            && segs[segs.len() - 1] == "Cargo.toml"
                            && segs[segs.len() - 2] == "rs"
                            && segs[segs.len() - 4] == "plugin"
                            && segs[segs.len() - 5] == "s"
                    };
                    if is_plugin || is_module || is_plugin_bundle {
                        out.push(path);
                    }
                }
            }
        }
        let mut out = Vec::new();
        walk(root, &mut out);
        out.sort();
        out
    }

    /// 🧾 Parses one plugin/module crate manifest's registry-relevant fields.
    pub fn parse_plugin_cargo_text(text: &str, crate_path: &str) -> Option<PluginRegistryEntry> {
        let package_name = string_field(text, "name")?;
        let component_block = blocks_after_header(text, |l| l == "[package.metadata.component]");
        let plugin_id = component_block.first().and_then(|b| string_field(b, "package")).and_then(|p| p.strip_prefix("semio:").map(str::to_string))?;
        let wasm_out = format!("{}.wasm", package_name.replace('-', "_"));
        let semio_block = blocks_after_header(text, |l| l == "[package.metadata.semio]");
        let (contributes, consumes) = match semio_block.first() {
            Some(b) => (string_array_field(b, "contributes"), string_array_field(b, "consumes")),
            None => (Vec::new(), Vec::new()),
        };
        Some(PluginRegistryEntry { plugin_id, crate_path: crate_path.to_string(), package_name, wasm_out, contributes, consumes })
    }

    /// 🎮 Parses every `[[package.metadata.semio.playground]]` row for one crate (examples unset;
    /// see `discover_examples_for_playground`, which needs the variant to disambiguate multi-app crates).
    pub fn parse_playgrounds_text(text: &str, plugin_id: &str, crate_path: &str) -> Vec<PlaygroundEntry> {
        blocks_after_header(text, |l| l == "[[package.metadata.semio.playground]]")
            .iter()
            .filter_map(|block| {
                let variant = string_field(block, "variant")?;
                let app = string_field(block, "app");
                let aliases = string_array_field(block, "aliases");
                let ports = ports_field(block)?;
                let engines = string_array_field(block, "engines");
                Some(PlaygroundEntry { variant, plugin_id: plugin_id.to_string(), crate_path: crate_path.to_string(), app, aliases, ports, examples: Vec::new(), engines, assets: Vec::new() })
            })
            .collect()
    }

    /// 🗂️ Parses every `[[package.metadata.semio.assets]]` row for one crate.
    pub fn parse_assets_text(text: &str) -> Vec<AssetSpec> {
        blocks_after_header(text, |l| l == "[[package.metadata.semio.assets]]")
            .iter()
            .filter_map(|block| {
                let kind = string_field(block, "kind")?;
                let route = string_field(block, "route")?;
                Some(AssetSpec {
                    kind,
                    route,
                    app: string_field(block, "app"),
                    upstream: string_field(block, "upstream"),
                    cache: string_field(block, "cache"),
                    root: string_field(block, "root"),
                    roots: string_array_field(block, "roots"),
                    placeholder: string_field(block, "placeholder"),
                    filter_from_examples: bool_field(block, "filter_from_examples"),
                })
            })
            .collect()
    }

    fn example_ids_in(dir: &Path) -> Vec<String> {
        let Ok(entries) = fs::read_dir(dir) else { return Vec::new() };
        let mut ids: Vec<String> = entries
            .flatten()
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if !name.ends_with(".json") {
                    return None;
                }
                name.split('.').next().map(str::to_string)
            })
            .collect();
        ids.sort();
        ids.dedup();
        ids
    }

    /// 🖼️ Example ids for one playground row: tries the crate's own `example/` dir (stripping a
    /// trailing `/rs` and `/plugin`), then — for multi-app crates where the playground `variant`
    /// diverges from the shared crate path (e.g. `puzzle/plugin/rs` hosting `puzzle2d`/`puzzle3d`) —
    /// the sibling module directory named after the variant's `pluginId`-stripped suffix
    /// (`puzzle2d` - `puzzle` = `2d` → `puzzle/2d/example`).
    pub fn discover_examples_for_playground(root: &Path, crate_path: &str, plugin_id: &str, variant: &str) -> Vec<String> {
        let trimmed = crate_path.strip_suffix("/rs").unwrap_or(crate_path);
        let own_candidates = [trimmed.to_string(), trimmed.strip_suffix("/plugin").unwrap_or(trimmed).to_string()];
        for base in &own_candidates {
            let dir = root.join(base).join("example");
            if dir.is_dir() {
                return example_ids_in(&dir);
            }
        }
        if let Some(suffix) = variant.strip_prefix(plugin_id).filter(|s| !s.is_empty()) {
            let segs: Vec<&str> = trimmed.split('/').collect();
            // 🏛️ Under `s/plugin/<p>/...` the tech root is the plugin folder (3 segments), not `s` itself.
            let tech_root = if segs.first() == Some(&"s") && segs.get(1) == Some(&"plugin") && segs.len() >= 3 {
                Some(segs[..3].join("/"))
            } else {
                segs.first().map(|s| s.to_string())
            };
            if let Some(tech_root) = tech_root {
                let dir = root.join(&tech_root).join(suffix).join("example");
                if dir.is_dir() {
                    return example_ids_in(&dir);
                }
            }
        }
        Vec::new()
    }

    /// 📚 Scans the whole workspace for plugin/module crates, sorted by `pluginId`.
    pub fn generate_plugin_registry(root: &Path) -> Vec<PluginRegistryEntry> {
        let mut entries: Vec<PluginRegistryEntry> = find_plugin_cargo_files(root)
            .iter()
            .filter_map(|path| {
                let text = fs::read_to_string(path).ok()?;
                let crate_path = path.parent()?.strip_prefix(root).ok()?.to_string_lossy().replace('\\', "/");
                match parse_plugin_cargo_text(&text, &crate_path) {
                    Some(e) => Some(e),
                    None => {
                        eprintln!("[DEBUG] plugin registry catalog: skipping {}", path.display());
                        None
                    }
                }
            })
            .collect();
        entries.sort_by(|a, b| a.plugin_id.cmp(&b.plugin_id));
        entries
    }

    /// 🕹️ Flattens every crate's playground rows into one repo-wide catalog, sorted by variant.
    pub fn generate_playground_registry(root: &Path) -> Vec<PlaygroundEntry> {
        let entries = generate_plugin_registry(root);
        let mut playgrounds = Vec::new();
        for entry in &entries {
            let manifest = root.join(&entry.crate_path).join("Cargo.toml");
            let Ok(text) = fs::read_to_string(&manifest) else { continue };
            let crate_assets = parse_assets_text(&text);
            for mut playground in parse_playgrounds_text(&text, &entry.plugin_id, &entry.crate_path) {
                playground.examples = discover_examples_for_playground(root, &entry.crate_path, &entry.plugin_id, &playground.variant);
                playground.assets = crate_assets
                    .iter()
                    .filter(|asset| asset.app.as_deref().map(|app| Some(app) == playground.app.as_deref()).unwrap_or(true))
                    .cloned()
                    .collect();
                playgrounds.push(playground);
            }
        }
        playgrounds.sort_by(|a, b| a.variant.cmp(&b.variant));
        playgrounds
    }
    //#endregion 🔖Discovery

    //#region 🔖Validate
    /// 🚦 Cross-checks the catalog for duplicate variants/aliases/ports and multi-app crate discipline.
    pub fn validate_playground_registry(playgrounds: &[PlaygroundEntry]) -> Vec<String> {
        use std::collections::HashMap;
        let mut errors = Vec::new();
        let mut variant_owners: HashMap<&str, &str> = HashMap::new();
        let mut alias_owners: HashMap<&str, &str> = HashMap::new();
        let mut port_owners: HashMap<(u32, u32), &str> = HashMap::new();
        let mut by_crate: HashMap<&str, Vec<&PlaygroundEntry>> = HashMap::new();
        for entry in playgrounds {
            if let Some(owner) = variant_owners.get(entry.variant.as_str()) {
                errors.push(format!("duplicate playground variant \"{}\" ({owner} and {})", entry.variant, entry.crate_path));
            } else {
                variant_owners.insert(&entry.variant, &entry.crate_path);
            }
            for alias in &entry.aliases {
                if let Some(owner) = alias_owners.get(alias.as_str()) {
                    errors.push(format!("duplicate playground alias \"{alias}\" (variants \"{owner}\" and \"{}\")", entry.variant));
                } else {
                    alias_owners.insert(alias, &entry.variant);
                }
            }
            let port_key = (entry.ports.react, entry.ports.wgpu);
            if let Some(owner) = port_owners.get(&port_key) {
                errors.push(format!(
                    "duplicate playground ports react={}/wgpu={} (variants \"{owner}\" and \"{}\")",
                    entry.ports.react, entry.ports.wgpu, entry.variant
                ));
            } else {
                port_owners.insert(port_key, &entry.variant);
            }
            by_crate.entry(&entry.crate_path).or_default().push(entry);
        }
        for group in by_crate.values() {
            if group.len() <= 1 {
                continue;
            }
            for entry in group {
                if entry.app.is_none() {
                    errors.push(format!("playground variant \"{}\" in {} must set \"app\" (crate declares {} playground entries)", entry.variant, entry.crate_path, group.len()));
                }
            }
        }
        errors
    }
    //#endregion 🔖Validate

    //#region 🔖Emit
    fn json_string(s: &str) -> String {
        serde_json::to_string(s).unwrap_or_default()
    }

    pub fn emit_plugins_json(entries: &[PluginRegistryEntry]) -> String {
        let value: Vec<serde_json::Value> = entries
            .iter()
            .map(|e| {
                serde_json::json!({
                    "pluginId": e.plugin_id,
                    "cratePath": e.crate_path,
                    "packageName": e.package_name,
                    "wasmOut": e.wasm_out,
                    "contributes": e.contributes,
                    "consumes": e.consumes,
                })
            })
            .collect();
        format!("{}\n", serde_json::to_string_pretty(&value).unwrap_or_default())
    }

    pub fn emit_plugins_ts(entries: &[PluginRegistryEntry]) -> String {
        let rows: Vec<String> = entries
            .iter()
            .map(|e| {
                format!(
                    "\t{{ pluginId: {}, cratePath: {}, wasmOut: {}, contributes: {}, consumes: {} }},",
                    json_string(&e.plugin_id),
                    json_string(&e.crate_path),
                    json_string(&e.wasm_out),
                    serde_json::to_string(&e.contributes).unwrap_or_default(),
                    serde_json::to_string(&e.consumes).unwrap_or_default(),
                )
            })
            .collect();
        format!(
            "/** @generated by repo/cli/rs (semio plugin registry generate) — do not edit. */\nexport type PluginBuildTarget = {{\n\treadonly pluginId: string;\n\treadonly cratePath: string;\n\treadonly wasmOut: string;\n\treadonly contributes: readonly string[];\n\treadonly consumes: readonly string[];\n}};\n\nexport const PLUGIN_BUILD_TARGETS: readonly PluginBuildTarget[] = [\n{}\n];\n\nexport const PROGRAM_TARGETS = PLUGIN_BUILD_TARGETS.map((target) => ({{\n\tpluginId: target.pluginId,\n\tmoduleUrl: `/plugin-modules/${{target.pluginId}}/${{target.wasmOut.replace(/\\.wasm$/, \".js\")}}`,\n}}));\n\nexport const pluginModuleUrl = (pluginId: string, fileName: string) =>\n\t`/plugin-modules/${{pluginId}}/${{fileName.replace(/\\.wasm$/, \".js\")}}`;\n",
            rows.join("\n")
        )
    }

    fn asset_spec_json(asset: &AssetSpec) -> serde_json::Value {
        let mut obj = serde_json::json!({ "kind": asset.kind, "route": asset.route });
        if let Some(v) = &asset.upstream {
            obj["upstream"] = serde_json::json!(v);
        }
        if let Some(v) = &asset.cache {
            obj["cache"] = serde_json::json!(v);
        }
        if let Some(v) = &asset.root {
            obj["root"] = serde_json::json!(v);
        }
        if !asset.roots.is_empty() {
            obj["roots"] = serde_json::json!(asset.roots);
        }
        if let Some(v) = &asset.placeholder {
            obj["placeholder"] = serde_json::json!(v);
        }
        if asset.filter_from_examples {
            obj["filterFromExamples"] = serde_json::json!(true);
        }
        obj
    }

    fn asset_spec_ts(asset: &AssetSpec) -> String {
        let mut fields = vec![format!("kind: {}", json_string(&asset.kind)), format!("route: {}", json_string(&asset.route))];
        if let Some(v) = &asset.upstream {
            fields.push(format!("upstream: {}", json_string(v)));
        }
        if let Some(v) = &asset.cache {
            fields.push(format!("cache: {}", json_string(v)));
        }
        if let Some(v) = &asset.root {
            fields.push(format!("root: {}", json_string(v)));
        }
        if !asset.roots.is_empty() {
            fields.push(format!("roots: {}", serde_json::to_string(&asset.roots).unwrap_or_default()));
        }
        if let Some(v) = &asset.placeholder {
            fields.push(format!("placeholder: {}", json_string(v)));
        }
        if asset.filter_from_examples {
            fields.push("filterFromExamples: true".to_string());
        }
        format!("{{ {} }}", fields.join(", "))
    }

    pub fn emit_playgrounds_json(playgrounds: &[PlaygroundEntry]) -> String {
        let value: Vec<serde_json::Value> = playgrounds
            .iter()
            .map(|p| {
                let mut obj = serde_json::json!({
                    "variant": p.variant,
                    "pluginId": p.plugin_id,
                    "cratePath": p.crate_path,
                    "aliases": p.aliases,
                    "ports": { "react": p.ports.react, "wgpu": p.ports.wgpu },
                    "examples": p.examples,
                    "engines": p.engines,
                    "assets": p.assets.iter().map(asset_spec_json).collect::<Vec<_>>(),
                });
                if let Some(app) = &p.app {
                    obj["app"] = serde_json::json!(app);
                }
                obj
            })
            .collect();
        format!("{}\n", serde_json::to_string_pretty(&value).unwrap_or_default())
    }

    pub fn emit_playgrounds_ts(playgrounds: &[PlaygroundEntry]) -> String {
        let rows: Vec<String> = playgrounds
            .iter()
            .map(|p| {
                let app = p.app.as_ref().map(|a| format!(", app: {}", json_string(a))).unwrap_or_default();
                let assets: Vec<String> = p.assets.iter().map(asset_spec_ts).collect();
                format!(
                    "\t{{ variant: {}, pluginId: {}, cratePath: {}{}, aliases: {}, ports: {{ react: {}, wgpu: {} }}, examples: {}, engines: {}, assets: [{}] }},",
                    json_string(&p.variant),
                    json_string(&p.plugin_id),
                    json_string(&p.crate_path),
                    app,
                    serde_json::to_string(&p.aliases).unwrap_or_default(),
                    p.ports.react,
                    p.ports.wgpu,
                    serde_json::to_string(&p.examples).unwrap_or_default(),
                    serde_json::to_string(&p.engines).unwrap_or_default(),
                    assets.join(", "),
                )
            })
            .collect();
        format!(
            "/** @generated by repo/cli/rs (semio plugin registry generate) — do not edit. */\nexport type PlaygroundAssetSpec =\n\t| {{ readonly kind: \"tile-proxy\"; readonly route: string; readonly upstream: string; readonly cache: string }}\n\t| {{ readonly kind: \"static-dir\"; readonly route: string; readonly root: string }}\n\t| {{ readonly kind: \"mesh-collection\"; readonly route: string; readonly roots: readonly string[]; readonly placeholder: string; readonly filterFromExamples?: boolean }};\n\nexport type PlaygroundBuildTarget = {{\n\treadonly variant: string;\n\treadonly pluginId: string;\n\treadonly cratePath: string;\n\treadonly app?: string;\n\treadonly aliases: readonly string[];\n\treadonly ports: {{ readonly react: number; readonly wgpu: number }};\n\treadonly examples: readonly string[];\n\treadonly engines: readonly string[];\n\treadonly assets: readonly PlaygroundAssetSpec[];\n}};\n\nexport const PLAYGROUND_BUILD_TARGETS: readonly PlaygroundBuildTarget[] = [\n{}\n];\n",
            rows.join("\n")
        )
    }
    //#endregion 🔖Emit

    //#region 🔖GenerateCheck
    fn generated_dir(root: &Path) -> PathBuf {
        root.join("framework/plugin/registry/generated")
    }

    /// ✍️ Regenerates `framework/plugin/registry/generated/*` from the current workspace state.
    pub fn write_registry(root: &Path) -> std::io::Result<(usize, usize)> {
        let entries = generate_plugin_registry(root);
        let playgrounds = generate_playground_registry(root);
        let out_dir = generated_dir(root);
        fs::create_dir_all(&out_dir)?;
        fs::write(out_dir.join("plugins.json"), emit_plugins_json(&entries))?;
        fs::write(out_dir.join("plugins.ts"), emit_plugins_ts(&entries))?;
        fs::write(out_dir.join("playgrounds.json"), emit_playgrounds_json(&playgrounds))?;
        fs::write(out_dir.join("playgrounds.ts"), emit_playgrounds_ts(&playgrounds))?;
        Ok((entries.len(), playgrounds.len()))
    }

    /// 🔎 Renders the catalog in memory and byte-compares it against `generated/*`; never writes.
    pub fn check_registry(root: &Path) -> Vec<String> {
        let entries = generate_plugin_registry(root);
        let playgrounds = generate_playground_registry(root);
        let out_dir = generated_dir(root);
        let expected = [
            ("plugins.json", emit_plugins_json(&entries)),
            ("plugins.ts", emit_plugins_ts(&entries)),
            ("playgrounds.json", emit_playgrounds_json(&playgrounds)),
            ("playgrounds.ts", emit_playgrounds_ts(&playgrounds)),
        ];
        let mut problems: Vec<String> = expected
            .iter()
            .filter(|(name, content)| fs::read_to_string(out_dir.join(name)).map(|actual| &actual != content).unwrap_or(true))
            .map(|(name, _)| format!("plugin registry catalog is stale: generated/{name} (run `semio plugin registry generate`)"))
            .collect();
        problems.extend(validate_playground_registry(&playgrounds));
        problems
    }

    /// 📖 Reads the committed catalog (empty if it has never been generated).
    pub fn load_playground_catalog(root: &Path) -> Vec<PlaygroundEntry> {
        let path = generated_dir(root).join("playgrounds.json");
        let Ok(text) = fs::read_to_string(path) else { return Vec::new() };
        let Ok(raw) = serde_json::from_str::<Vec<serde_json::Value>>(&text) else { return Vec::new() };
        raw.into_iter()
            .filter_map(|v| {
                Some(PlaygroundEntry {
                    variant: v.get("variant")?.as_str()?.to_string(),
                    plugin_id: v.get("pluginId")?.as_str()?.to_string(),
                    crate_path: v.get("cratePath")?.as_str()?.to_string(),
                    app: v.get("app").and_then(|a| a.as_str()).map(str::to_string),
                    aliases: v.get("aliases")?.as_array()?.iter().filter_map(|a| a.as_str().map(str::to_string)).collect(),
                    ports: Ports {
                        react: v.get("ports")?.get("react")?.as_u64()? as u32,
                        wgpu: v.get("ports")?.get("wgpu")?.as_u64()? as u32,
                    },
                    examples: v.get("examples").and_then(|e| e.as_array()).map(|a| a.iter().filter_map(|e| e.as_str().map(str::to_string)).collect()).unwrap_or_default(),
                    engines: v.get("engines").and_then(|e| e.as_array()).map(|a| a.iter().filter_map(|e| e.as_str().map(str::to_string)).collect()).unwrap_or_default(),
                    assets: v.get("assets").and_then(|e| e.as_array()).map(|a| a.iter().filter_map(asset_spec_from_json).collect()).unwrap_or_default(),
                })
            })
            .collect()
    }

    fn asset_spec_from_json(v: &serde_json::Value) -> Option<AssetSpec> {
        Some(AssetSpec {
            kind: v.get("kind")?.as_str()?.to_string(),
            route: v.get("route")?.as_str()?.to_string(),
            app: None,
            upstream: v.get("upstream").and_then(|x| x.as_str()).map(str::to_string),
            cache: v.get("cache").and_then(|x| x.as_str()).map(str::to_string),
            root: v.get("root").and_then(|x| x.as_str()).map(str::to_string),
            roots: v.get("roots").and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|e| e.as_str().map(str::to_string)).collect()).unwrap_or_default(),
            placeholder: v.get("placeholder").and_then(|x| x.as_str()).map(str::to_string),
            filter_from_examples: v.get("filterFromExamples").and_then(|x| x.as_bool()).unwrap_or(false),
        })
    }
    //#endregion 🔖GenerateCheck
}
// #endregion 🔖Catalog

// #region 🔖Options
pub mod options {
    /// 🔀 One CLI option pick: switchable at runtime, or locked to one value at boot.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Lock {
        All,
        Individual(String),
    }

    /// 📥 Parses a `--flag` value: `"all"` (case-insensitive) means runtime-switchable.
    pub fn parse_lock(raw: &str) -> Lock {
        if raw.eq_ignore_ascii_case("all") {
            Lock::All
        } else {
            Lock::Individual(raw.to_string())
        }
    }
}
// #endregion 🔖Options

// #region 🔖EnvContract
pub mod env_contract {
    use crate::catalog::PlaygroundEntry;
    use crate::options::Lock;

    #[derive(Debug, Clone, Default)]
    pub struct DevOptions {
        pub renderer: String,
        pub port: Option<u16>,
        pub example: Lock,
        pub language: Lock,
        pub terminology: Lock,
        pub theme: Lock,
        pub appearance: Lock,
        pub skip_plugin_build: bool,
        pub skip_engine_build: bool,
        pub skip_wgpu_build: bool,
    }

    impl Default for Lock {
        fn default() -> Self {
            Lock::All
        }
    }

    /// 🔌 Resolves the dev-server port: `--port`, else the catalog's port for this renderer, else 6066.
    pub fn resolve_port(playground: Option<&PlaygroundEntry>, renderer: &str, explicit: Option<u16>) -> u16 {
        if let Some(port) = explicit {
            return port;
        }
        match playground {
            Some(row) if renderer == "wgpu" => row.ports.wgpu as u16,
            Some(row) => row.ports.react as u16,
            None => 6066,
        }
    }

    /// 📡 Builds the env vars a `framework-os-dev` dev session (or its browser) needs to boot,
    /// including the `SEMIO_LOCKED_*` shell locks for any `Individual` option pick.
    pub fn build_dev_env(variant: &str, playground: Option<&PlaygroundEntry>, opts: &DevOptions) -> Vec<(String, String)> {
        let mut env = Vec::new();
        let renderer = if opts.renderer.starts_with("wgpu") { "wgpu" } else { "react" };
        let port = resolve_port(playground, renderer, opts.port);
        env.push(("SEMIO_PLUGIN".to_string(), variant.to_string()));
        env.push(("SEMIO_RENDERER".to_string(), renderer.to_string()));
        env.push(("S_OS_PORT".to_string(), port.to_string()));
        env.push(("VITE_SEMIO_PLUGIN".to_string(), playground.map(|p| p.plugin_id.clone()).unwrap_or_else(|| variant.to_string())));
        env.push(("VITE_SEMIO_RENDERER".to_string(), renderer.to_string()));
        if let Some(app) = playground.and_then(|p| p.app.as_ref()) {
            env.push(("VITE_SEMIO_APP_ID".to_string(), app.clone()));
        }
        if let Lock::Individual(id) = &opts.example {
            env.push(("PLAYGROUND_LOCKED_EXAMPLE_ID".to_string(), id.clone()));
            env.push(("VITE_SEMIO_LOCKED_EXAMPLE".to_string(), id.clone()));
        }
        if let Lock::Individual(v) = &opts.language {
            env.push(("SEMIO_LOCKED_LOCALE".to_string(), v.clone()));
        }
        if let Lock::Individual(v) = &opts.terminology {
            env.push(("SEMIO_LOCKED_TERMINOLOGY".to_string(), v.clone()));
        }
        if let Lock::Individual(v) = &opts.theme {
            env.push(("SEMIO_LOCKED_THEME".to_string(), v.clone()));
        }
        if let Lock::Individual(v) = &opts.appearance {
            env.push(("SEMIO_LOCKED_APPEARANCE".to_string(), v.clone()));
        }
        if opts.skip_plugin_build {
            env.push(("SKIP_PLUGIN_BUILD".to_string(), "1".to_string()));
        }
        if opts.skip_engine_build {
            env.push(("SKIP_ENGINE_BUILD".to_string(), "1".to_string()));
        }
        if opts.skip_wgpu_build {
            env.push(("SKIP_WGPU_BUILD".to_string(), "1".to_string()));
        }
        env.push(("NX_NATIVE_COMMAND_RUNNER".to_string(), "false".to_string()));
        env.push(("NX_TASKS_RUNNER_DYNAMIC_OUTPUT".to_string(), "false".to_string()));
        env.push(("NX_TUI".to_string(), "false".to_string()));
        env
    }
}
// #endregion 🔖EnvContract

// #region 🔖Dev
pub mod dev {
    use crate::args::ParsedArgs;
    use crate::catalog::{load_playground_catalog, PlaygroundEntry};
    use crate::env_contract::{build_dev_env, DevOptions};
    use crate::options::parse_lock;
    use crate::proc::spawn_inherit;
    use std::path::Path;

    /// 🎯 Longest-prefix multi-word alias resolution against the catalog (`"puzzle 3d"`, `"gis 2d"`, …).
    pub fn resolve_playground<'a>(catalog: &'a [PlaygroundEntry], segments: &[String]) -> Option<(&'a PlaygroundEntry, Vec<String>)> {
        for len in (1..=segments.len()).rev() {
            let alias = segments[..len].join(" ");
            if let Some(row) = catalog.iter().find(|r| r.variant == alias || r.aliases.iter().any(|a| a == &alias)) {
                return Some((row, segments[len..].to_vec()));
            }
        }
        None
    }

    /// 🩹 Maps the legacy `fixture <slug>` / `example <slug>` positional prefix onto `--example`.
    pub fn consume_legacy_example_prefix(segments: &[String]) -> (Vec<String>, Option<String>) {
        if segments.len() >= 2 && (segments[0] == "fixture" || segments[0] == "example") {
            (segments[2..].to_vec(), Some(segments[1].clone()))
        } else {
            (segments.to_vec(), None)
        }
    }

    fn dev_options_from_args(args: &ParsedArgs) -> DevOptions {
        let (rest, legacy_example) = consume_legacy_example_prefix(&args.segments[1.min(args.segments.len())..]);
        let _ = rest;
        DevOptions {
            renderer: args.flag("renderer").unwrap_or("react").to_string(),
            port: args.flag("port").and_then(|p| p.parse().ok()),
            example: args.flag("example").map(parse_lock).or(legacy_example.as_deref().map(parse_lock)).unwrap_or(crate::options::Lock::All),
            language: args.flag("language").map(parse_lock).unwrap_or(crate::options::Lock::All),
            terminology: args.flag("terminology").map(parse_lock).unwrap_or(crate::options::Lock::All),
            theme: args.flag("theme").map(parse_lock).unwrap_or(crate::options::Lock::All),
            appearance: args.flag("appearance").map(parse_lock).unwrap_or(crate::options::Lock::All),
            skip_plugin_build: args.has_flag("skip-plugin-build"),
            skip_engine_build: args.has_flag("skip-engine-build"),
            skip_wgpu_build: args.has_flag("skip-wgpu-build"),
        }
    }

    /// ▶️ Resolves a `semio dev <variant…>` invocation and spawns the dev session with its env.
    pub fn run_dev(root: &Path, args: &ParsedArgs) -> i32 {
        let catalog = load_playground_catalog(root);
        let (segments, _) = consume_legacy_example_prefix(&args.segments);
        let Some((playground, _rest)) = resolve_playground(&catalog, &segments) else {
            eprintln!("[semio dev] unknown plugin/variant {:?} — run `semio catalog` to list playgrounds", segments.join(" "));
            return 1;
        };
        let opts = dev_options_from_args(args);
        let env = build_dev_env(&playground.variant, Some(playground), &opts);
        println!("[semio dev] {} via {} on port {}", playground.variant, opts.renderer, env.iter().find(|(k, _)| k == "S_OS_PORT").map(|(_, v)| v.as_str()).unwrap_or("?"));
        spawn_inherit("bun", &["nx", "run", "@semio-tech/framework-os-dev:dev"], root, &env)
    }
}
// #endregion 🔖Dev

// #region 🔖PluginRegistryCommand
pub mod plugin_registry_command {
    use crate::catalog::{check_registry, write_registry};
    use std::path::Path;

    pub fn run(root: &Path, subcommand: &str) -> i32 {
        match subcommand {
            "check" => {
                let problems = check_registry(root);
                if problems.is_empty() {
                    println!("plugin registry catalog is fresh.");
                    0
                } else {
                    for p in &problems {
                        eprintln!("{p}");
                    }
                    1
                }
            }
            _ => match write_registry(root) {
                Ok((plugins, playgrounds)) => {
                    println!("plugin registry catalog refreshed ({plugins} plugin crates, {playgrounds} playgrounds).");
                    0
                }
                Err(e) => {
                    eprintln!("[semio plugin registry generate] {e}");
                    1
                }
            },
        }
    }
}
// #endregion 🔖PluginRegistryCommand

// #region 🔖Tui
pub mod tui_dashboard {
    use crate::catalog::{load_playground_catalog, PlaygroundEntry};
    use crate::env_contract::{build_dev_env, DevOptions};
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use std::process::{Child, Command, Stdio};
    use std::sync::mpsc::{channel, Receiver};
    use std::time::Duration;
    use ui_tui::backend::{NativeTerminal, TerminalBackend};
    use ui_tui::chrome::{shell, ChromeState, FooterState, KeyHint, NavItem, NavbarState, WindowState};
    use ui_tui::engine::Tui;
    use ui_tui::event::{Event, Key};
    use ui_tui::geometry::Size;
    use ui_tui::layout::{even_window_layout, Constraint, Dimension, Direction};
    use ui_tui::scene::{Node, NodeContent, NodeId};
    use ui_tui::theme::Theme;
    use ui_tui::widget::{LogState, TableAlign, TableColumn, TableRow, TableState, WidgetSignal, WidgetState};
    use ui_styling::appearance::AppearanceName;

    //#region 🔖CatalogTable
    /// 🌳 Groups the catalog by `pluginId` into a two-level table: one parent row per plugin, one
    /// child row per playground variant. Returns the rows alongside a flat-row-index → catalog-index
    /// map (`None` for parent rows) so an `Activated` signal can resolve back to a `PlaygroundEntry`.
    pub fn group_catalog_into_table(catalog: &[PlaygroundEntry]) -> (Vec<TableRow>, Vec<Option<usize>>) {
        let mut plugin_ids: Vec<&str> = catalog.iter().map(|e| e.plugin_id.as_str()).collect();
        plugin_ids.sort();
        plugin_ids.dedup();
        let mut rows = Vec::new();
        let mut mapping = Vec::new();
        for plugin_id in plugin_ids {
            rows.push(TableRow::parent(plugin_id.to_string(), vec![plugin_id.to_string(), String::new(), String::new(), String::new()]));
            mapping.push(None);
            for (i, entry) in catalog.iter().enumerate() {
                if entry.plugin_id != plugin_id {
                    continue;
                }
                rows.push(TableRow::child(
                    entry.variant.clone(),
                    vec![entry.variant.clone(), entry.ports.react.to_string(), entry.ports.wgpu.to_string(), entry.examples.len().to_string()],
                    1,
                ));
                mapping.push(Some(i));
            }
        }
        (rows, mapping)
    }

    fn plugin_table_columns() -> Vec<TableColumn> {
        vec![
            TableColumn::new("Plugin / App", 0, TableAlign::Left),
            TableColumn::new("React", 6, TableAlign::Right),
            TableColumn::new("Wgpu", 6, TableAlign::Right),
            TableColumn::new("Examples", 9, TableAlign::Right),
        ]
    }
    //#endregion 🔖CatalogTable

    //#region 🔖Sessions
    /// ▶️ One supervised child process (a dev or build pipeline run), streaming its combined
    /// stdout/stderr into `rx` line by line from two reader threads.
    struct Session {
        child: Child,
        rx: Receiver<String>,
        variant: String,
    }

    fn spawn_session(root: &Path, verb: &str, variant: &str, row: &PlaygroundEntry) -> std::io::Result<Session> {
        let env = build_dev_env(variant, Some(row), &DevOptions { renderer: "react".into(), ..Default::default() });
        let mut child = Command::new("bun")
            .args(["nx", "run", &format!("@semio-tech/framework-os-dev:{verb}")])
            .current_dir(root)
            .envs(env)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let (tx, rx) = channel();
        if let Some(stdout) = child.stdout.take() {
            let tx = tx.clone();
            std::thread::spawn(move || {
                for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                    if tx.send(line).is_err() {
                        break;
                    }
                }
            });
        }
        if let Some(stderr) = child.stderr.take() {
            std::thread::spawn(move || {
                for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                    if tx.send(line).is_err() {
                        break;
                    }
                }
            });
        }
        Ok(Session { child, rx, variant: variant.to_string() })
    }

    impl Session {
        fn stop(mut self) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }

    /// 📡 Drains whatever log lines have arrived since the last tick into the window's `Log` widget.
    fn pump_session(tui: &mut Tui, session: &Option<Session>, log_id: NodeId) {
        let Some(session) = session else { return };
        while let Ok(line) = session.rx.try_recv() {
            if let Some(WidgetState::Log(log)) = tui.scene.node_mut(log_id).widget() {
                log.push(&line);
            }
        }
    }
    //#endregion 🔖Sessions

    /// 🎛️ The bare-`semio` interactive dashboard: `dev` and `build` windows, each a table of
    /// plugins (parent rows) with their apps (child rows), and a log pane streaming the session
    /// launched from whichever row is activated.
    pub fn run(root: &Path) -> i32 {
        let catalog = load_playground_catalog(root);
        if catalog.is_empty() {
            eprintln!("[semio] plugin registry catalog is empty — run `semio plugin registry generate` first.");
            return 1;
        }
        let Ok(mut term) = NativeTerminal::new() else {
            eprintln!("[semio] failed to attach to the terminal");
            return 1;
        };
        if term.enter().is_err() {
            return 1;
        }
        let size = term.size().unwrap_or(Size { width: 100, height: 32 });
        let mut tui = Tui::new(size, Theme::new(AppearanceName::Dark));

        let navbar = NavbarState { left: vec![NavItem { id: "logo".into(), label: "semio".into(), active: true }], center: vec![], right: vec![] };
        let footer = FooterState {
            hints: vec![
                KeyHint { key: "\u{2191}\u{2193}".into(), label: "select".into() },
                KeyHint { key: "\u{21b5}".into(), label: "launch/toggle".into() },
                KeyHint { key: "Tab".into(), label: "window".into() },
                KeyHint { key: "x".into(), label: "stop".into() },
                KeyHint { key: "q".into(), label: "quit".into() },
            ],
            status: format!("{} playgrounds", catalog.len()),
        };
        let built = shell(&mut tui.scene, navbar, footer, &even_window_layout(&[]));
        tui.scene.node_mut(built.canvas).set_constraint(Constraint { direction: Direction::Row, height: Dimension::Weight(1), ..Default::default() });

        let dev_window = tui.scene.add(built.canvas, Node::new(NodeContent::Chrome(ChromeState::Window(WindowState::new("dev")))));
        tui.scene.node_mut(dev_window).set_constraint(Constraint { width: Dimension::Weight(1), direction: Direction::Column, padding: [2, 1, 1, 1], gap: 1, ..Default::default() });
        let build_window = tui.scene.add(built.canvas, Node::new(NodeContent::Chrome(ChromeState::Window(WindowState::new("build")))));
        tui.scene.node_mut(build_window).set_constraint(Constraint { width: Dimension::Weight(1), direction: Direction::Column, padding: [2, 1, 1, 1], gap: 1, ..Default::default() });

        let (dev_rows, dev_map) = group_catalog_into_table(&catalog);
        let dev_table = tui.scene.add(dev_window, Node::new(NodeContent::Widget(WidgetState::Table(TableState::new(plugin_table_columns(), dev_rows)))));
        tui.scene.node_mut(dev_table).set_constraint(Constraint { height: Dimension::Weight(3), ..Default::default() });
        let dev_log = tui.scene.add(dev_window, Node::new(NodeContent::Widget(WidgetState::Log(LogState::new(500)))));
        tui.scene.node_mut(dev_log).set_constraint(Constraint { height: Dimension::Weight(1), ..Default::default() });

        let (build_rows, build_map) = group_catalog_into_table(&catalog);
        let build_table = tui.scene.add(build_window, Node::new(NodeContent::Widget(WidgetState::Table(TableState::new(plugin_table_columns(), build_rows)))));
        tui.scene.node_mut(build_table).set_constraint(Constraint { height: Dimension::Weight(3), ..Default::default() });
        let build_log = tui.scene.add(build_window, Node::new(NodeContent::Widget(WidgetState::Log(LogState::new(500)))));
        tui.scene.node_mut(build_log).set_constraint(Constraint { height: Dimension::Weight(1), ..Default::default() });

        tui.set_focus(Some(dev_table));
        term.present(&tui.render_full()).ok();

        let mut dev_session: Option<Session> = None;
        let mut build_session: Option<Session> = None;

        loop {
            let events = term.poll(Duration::from_millis(80)).unwrap_or_default();
            let mut quit = false;
            for event in &events {
                if let Event::Key(k) = event {
                    if k.key == Key::Char('q') {
                        quit = true;
                        break;
                    }
                    if k.key == Key::Char('x') {
                        let focus = tui.focus();
                        if focus == Some(dev_table) || focus == Some(dev_log) {
                            if let Some(session) = dev_session.take() {
                                session.stop();
                            }
                        } else if focus == Some(build_table) || focus == Some(build_log) {
                            if let Some(session) = build_session.take() {
                                session.stop();
                            }
                        }
                        continue;
                    }
                }
                for (node_id, signal) in tui.dispatch(event) {
                    let WidgetSignal::Activated(row_idx) = signal else { continue };
                    let (verb, map, window_id) = if node_id == dev_table {
                        ("dev", &dev_map, dev_window)
                    } else if node_id == build_table {
                        ("build", &build_map, build_window)
                    } else {
                        continue;
                    };
                    let Some(Some(catalog_idx)) = map.get(row_idx) else { continue };
                    let Some(row) = catalog.get(*catalog_idx) else { continue };
                    if let Ok(session) = spawn_session(root, verb, &row.variant, row) {
                        let log_id = if window_id == dev_window { dev_log } else { build_log };
                        if let Some(WidgetState::Log(log)) = tui.scene.node_mut(log_id).widget() {
                            log.clear();
                            log.push(&format!("[{verb}] launching {}\u{2026}", row.variant));
                        }
                        let old = if window_id == dev_window { dev_session.replace(session) } else { build_session.replace(session) };
                        if let Some(old) = old {
                            old.stop();
                        }
                    }
                }
            }
            if quit {
                break;
            }

            pump_session(&mut tui, &dev_session, dev_log);
            pump_session(&mut tui, &build_session, build_log);

            let focus = tui.focus();
            let dev_focused = focus == Some(dev_table) || focus == Some(dev_log);
            let build_focused = focus == Some(build_table) || focus == Some(build_log);
            if let Some(ChromeState::Window(w)) = tui.scene.node_mut(dev_window).chrome() {
                w.focused = dev_focused;
            }
            if let Some(ChromeState::Window(w)) = tui.scene.node_mut(build_window).chrome() {
                w.focused = build_focused;
            }
            let status = match (&dev_session, &build_session) {
                (None, None) => format!("{} playgrounds", catalog.len()),
                (Some(d), None) => format!("dev: {}", d.variant),
                (None, Some(b)) => format!("build: {}", b.variant),
                (Some(d), Some(b)) => format!("dev: {} \u{2502} build: {}", d.variant, b.variant),
            };
            if let Some(ChromeState::Footer(f)) = tui.scene.node_mut(built.footer).chrome() {
                f.status = status;
            }

            let patch = tui.render();
            if !patch.0.is_empty() {
                term.present(&patch).ok();
            }
        }
        term.leave().ok();

        if let Some(session) = dev_session {
            session.stop();
        }
        if let Some(session) = build_session {
            session.stop();
        }
        0
    }
}
// #endregion 🔖Tui

// #region 🔖Dispatch
/// 🚦 Runs one `semio` invocation and returns its process exit code.
pub fn run(argv: Vec<String>) -> i32 {
    let root = workspace::find_root(&std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    if argv.is_empty() {
        if !std::io::stdout().is_terminal() {
            print_usage();
            return 1;
        }
        return tui_dashboard::run(&root);
    }
    let parsed = args::parse(&argv);
    match parsed.verb.as_str() {
        "dev" => dev::run_dev(&root, &parsed),
        "catalog" => {
            let catalog = catalog::load_playground_catalog(&root);
            if parsed.has_flag("json") {
                println!("{}", catalog::emit_playgrounds_json(&catalog));
            } else {
                for row in &catalog {
                    println!("{}\t{}\treact:{}\twgpu:{}", row.variant, row.plugin_id, row.ports.react, row.ports.wgpu);
                }
            }
            0
        }
        "plugin" if parsed.segments.first().map(String::as_str) == Some("registry") => {
            plugin_registry_command::run(&root, parsed.segments.get(1).map(String::as_str).unwrap_or("generate"))
        }
        _ => {
            let mut forward = vec!["./script.ts".to_string(), parsed.verb.clone()];
            forward.extend(parsed.segments.clone());
            let forward_refs: Vec<&str> = forward.iter().map(String::as_str).collect();
            proc::spawn_inherit("bun", &forward_refs, &root, &[])
        }
    }
}

fn print_usage() {
    eprintln!("semio — semio monorepo orchestrator\n\nUsage:\n  semio                 interactive TUI dashboard (requires a TTY)\n  semio dev <variant…>  start a plugin dev session\n  semio catalog         list playgrounds\n  semio plugin registry generate|check\n  semio <verb> …        forwarded to `bun ./script.ts <verb> …`");
}
// #endregion 🔖Dispatch

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use crate::args::parse;
    use crate::catalog::{parse_assets_text, parse_playgrounds_text, parse_plugin_cargo_text, validate_playground_registry, PlaygroundEntry, Ports};
    use crate::dev::{consume_legacy_example_prefix, resolve_playground};
    use crate::env_contract::{build_dev_env, resolve_port, DevOptions};
    use crate::options::{parse_lock, Lock};
    use crate::tui_dashboard::group_catalog_into_table;

    #[test]
    fn args_split_verb_segments_and_flags() {
        let argv: Vec<String> = ["dev", "puzzle", "2d", "--renderer", "wgpu-wasm", "--skip-plugin-build"].iter().map(|s| s.to_string()).collect();
        let parsed = parse(&argv);
        assert_eq!(parsed.verb, "dev");
        assert_eq!(parsed.segments, vec!["puzzle", "2d"]);
        assert_eq!(parsed.flag("renderer"), Some("wgpu-wasm"));
        assert!(parsed.has_flag("skip-plugin-build"));
        assert_eq!(parsed.flag("skip-plugin-build"), None);
    }

    #[test]
    fn plugin_cargo_text_parses_component_and_semio_blocks() {
        let text = r#"
[package]
name = "puzzle-2d"

[package.metadata.component]
package = "semio:puzzle2d"

[package.metadata.semio]
contributes = ["puzzle.geometry"]
consumes = ["puzzle.solver"]
"#;
        let entry = parse_plugin_cargo_text(text, "puzzle/2d/rs").expect("parses");
        assert_eq!(entry.plugin_id, "puzzle2d");
        assert_eq!(entry.package_name, "puzzle-2d");
        assert_eq!(entry.wasm_out, "puzzle_2d.wasm");
        assert_eq!(entry.contributes, vec!["puzzle.geometry"]);
        assert_eq!(entry.consumes, vec!["puzzle.solver"]);
    }

    #[test]
    fn plugin_cargo_text_without_component_package_is_none() {
        let text = "[package]\nname = \"not-a-plugin\"\n";
        assert!(parse_plugin_cargo_text(text, "x/rs").is_none());
    }

    #[test]
    fn playground_blocks_parse_variant_app_aliases_ports() {
        let text = r#"
[[package.metadata.semio.playground]]
variant = "puzzle2d"
app = "puzzle2d"
aliases = ["2d"]
ports = { react = 6012, wgpu = 6112 }

[[package.metadata.semio.playground]]
variant = "puzzle3d"
app = "puzzle3d"
aliases = []
ports = { react = 6013, wgpu = 6113 }
"#;
        let entries = parse_playgrounds_text(text, "puzzle", "puzzle/plugin/rs");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].variant, "puzzle2d");
        assert_eq!(entries[0].aliases, vec!["2d"]);
        assert_eq!(entries[0].ports, Ports { react: 6012, wgpu: 6112 });
        assert_eq!(entries[0].examples, Vec::<String>::new());
    }

    #[test]
    fn playground_blocks_parse_engines() {
        let text = r#"
[[package.metadata.semio.playground]]
variant = "gis2d"
app = "gis2d"
aliases = ["gis 2d"]
ports = { react = 6040, wgpu = 6140 }
engines = ["framework/surface/tiled-map/rs"]
"#;
        let entries = parse_playgrounds_text(text, "gis", "gis/plugin/rs");
        assert_eq!(entries[0].engines, vec!["framework/surface/tiled-map/rs"]);
    }

    #[test]
    fn assets_blocks_parse_all_kinds() {
        let text = r#"
[[package.metadata.semio.assets]]
kind = "tile-proxy"
route = "/osm"
upstream = "https://tile.openstreetmap.org/{z}/{x}/{y}.png"
cache = "osm-tiles"

[[package.metadata.semio.assets]]
kind = "static-dir"
route = "/cad-fixture"
root = "cad/fixture"

[[package.metadata.semio.assets]]
kind = "mesh-collection"
route = "/mesh"
roots = ["framework/asset/metabolism/representation", "framework/asset/abbau-aufbau"]
placeholder = "framework/asset/mesh/placeholder.glb"
filter_from_examples = true
app = "puzzle3d"
"#;
        let assets = parse_assets_text(text);
        assert_eq!(assets.len(), 3);
        assert_eq!(assets[0].kind, "tile-proxy");
        assert_eq!(assets[0].upstream.as_deref(), Some("https://tile.openstreetmap.org/{z}/{x}/{y}.png"));
        assert_eq!(assets[0].cache.as_deref(), Some("osm-tiles"));
        assert_eq!(assets[1].kind, "static-dir");
        assert_eq!(assets[1].root.as_deref(), Some("cad/fixture"));
        assert_eq!(assets[2].kind, "mesh-collection");
        assert_eq!(assets[2].roots, vec!["framework/asset/metabolism/representation", "framework/asset/abbau-aufbau"]);
        assert_eq!(assets[2].placeholder.as_deref(), Some("framework/asset/mesh/placeholder.glb"));
        assert!(assets[2].filter_from_examples);
        assert_eq!(assets[2].app.as_deref(), Some("puzzle3d"));
    }

    #[test]
    fn validate_flags_duplicate_variant_and_missing_app() {
        let a = PlaygroundEntry { variant: "x".into(), plugin_id: "p".into(), crate_path: "c1".into(), ports: Ports { react: 1, wgpu: 2 }, ..Default::default() };
        let mut b = a.clone();
        b.crate_path = "c2".into();
        let errors = validate_playground_registry(&[a, b]);
        assert!(errors.iter().any(|e| e.contains("duplicate playground variant")));
    }

    #[test]
    fn resolve_playground_prefers_longest_multi_word_alias() {
        let catalog = vec![
            PlaygroundEntry { variant: "puzzle2d".into(), aliases: vec!["puzzle 2d".into(), "2d".into()], ..Default::default() },
            PlaygroundEntry { variant: "puzzle3d".into(), aliases: vec!["puzzle 3d".into()], ..Default::default() },
        ];
        let segments = ["puzzle".to_string(), "3d".to_string(), "fixture".to_string(), "concrete".to_string()];
        let (row, rest) = resolve_playground(&catalog, &segments).expect("resolves");
        assert_eq!(row.variant, "puzzle3d");
        assert_eq!(rest, vec!["fixture", "concrete"]);
    }

    #[test]
    fn legacy_fixture_prefix_maps_to_example() {
        let segments = ["fixture".to_string(), "concrete".to_string()];
        let (rest, example) = consume_legacy_example_prefix(&segments);
        assert!(rest.is_empty());
        assert_eq!(example, Some("concrete".to_string()));
    }

    #[test]
    fn env_contract_sets_locks_only_for_individual() {
        let row = PlaygroundEntry { variant: "puzzle2d".into(), plugin_id: "puzzle2d".into(), ports: Ports { react: 6012, wgpu: 6112 }, ..Default::default() };
        let opts = DevOptions {
            renderer: "react".into(),
            example: Lock::Individual("concrete-forest".into()),
            language: Lock::All,
            terminology: Lock::Individual("reuse".into()),
            theme: Lock::All,
            appearance: Lock::Individual("dark".into()),
            ..Default::default()
        };
        let env = build_dev_env("puzzle2d", Some(&row), &opts);
        let get = |k: &str| env.iter().find(|(key, _)| key == k).map(|(_, v)| v.clone());
        assert_eq!(get("S_OS_PORT"), Some("6012".to_string()));
        assert_eq!(get("PLAYGROUND_LOCKED_EXAMPLE_ID"), Some("concrete-forest".to_string()));
        assert_eq!(get("SEMIO_LOCKED_TERMINOLOGY"), Some("reuse".to_string()));
        assert_eq!(get("SEMIO_LOCKED_APPEARANCE"), Some("dark".to_string()));
        assert_eq!(get("SEMIO_LOCKED_LOCALE"), None);
        assert_eq!(get("SEMIO_LOCKED_THEME"), None);
    }

    #[test]
    fn resolve_port_prefers_explicit_then_catalog_then_fallback() {
        let row = PlaygroundEntry { ports: Ports { react: 6012, wgpu: 6112 }, ..Default::default() };
        assert_eq!(resolve_port(Some(&row), "react", Some(9999)), 9999);
        assert_eq!(resolve_port(Some(&row), "wgpu", None), 6112);
        assert_eq!(resolve_port(None, "react", None), 6066);
    }

    #[test]
    fn parse_lock_all_is_case_insensitive() {
        assert_eq!(parse_lock("All"), Lock::All);
        assert_eq!(parse_lock("dark"), Lock::Individual("dark".to_string()));
    }

    #[test]
    fn group_catalog_into_table_nests_apps_under_their_plugin_with_a_catalog_index_map() {
        let catalog = vec![
            PlaygroundEntry { variant: "puzzle3d".into(), plugin_id: "puzzle".into(), ports: Ports { react: 6013, wgpu: 6113 }, examples: vec!["a".into()], ..Default::default() },
            PlaygroundEntry { variant: "puzzle2d".into(), plugin_id: "puzzle".into(), ports: Ports { react: 6012, wgpu: 6112 }, ..Default::default() },
            PlaygroundEntry { variant: "draw".into(), plugin_id: "draw".into(), ports: Ports { react: 6064, wgpu: 6164 }, ..Default::default() },
        ];
        let (rows, map) = group_catalog_into_table(&catalog);

        // plugins sorted alphabetically, each followed immediately by its child rows in catalog order
        assert_eq!(rows.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), vec!["draw", "draw", "puzzle", "puzzle3d", "puzzle2d"]);
        assert_eq!(rows.iter().map(|r| r.level).collect::<Vec<_>>(), vec![0, 1, 0, 1, 1]);
        assert!(rows[0].has_children && rows[2].has_children);
        assert!(!rows[1].has_children && !rows[3].has_children && !rows[4].has_children);

        // the map resolves each child row back to its catalog entry; parent rows map to None
        assert_eq!(map, vec![None, Some(2), None, Some(0), Some(1)]);
        assert_eq!(catalog[map[1].unwrap()].variant, "draw");
        assert_eq!(catalog[map[3].unwrap()].variant, "puzzle3d");

        // cell contents: ports and example count on children, blank on the parent row
        assert_eq!(rows[3].cells.iter().map(String::as_str).collect::<Vec<_>>(), vec!["puzzle3d", "6013", "6113", "1"]);
        assert_eq!(rows[2].cells.iter().map(String::as_str).collect::<Vec<_>>(), vec!["puzzle", "", "", ""]);
    }
}
// #endregion 🔖Tests

