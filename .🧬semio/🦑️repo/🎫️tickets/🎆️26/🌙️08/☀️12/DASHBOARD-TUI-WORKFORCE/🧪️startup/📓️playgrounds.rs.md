fn inject_playground_dev(root: &Path, trie: &mut TrieNode) {
    for row in crate::catalog::load_playground_catalog(root) {
        for renderer in ["react", "wgpu-wasm", "wgpu-native"] {
            for example in std::iter::once(None).chain(row.examples.iter().map(Some)) {
                let opts = crate::env_contract::DevOptions { renderer: renderer.into(), example: example.map_or(crate::options::Lock::All, |id| crate::options::Lock::Individual(id.clone())), language: crate::options::Lock::Individual("en".into()), terminology: crate::options::Lock::Individual("native".into()), ..Default::default() };
                let mut env = crate::env_contract::build_dev_env(&row.variant, Some(&row), &opts);
                env.extend(nx_env());
                let target = match renderer { "wgpu-native" => format!("run-{}-native-dev", row.variant), "wgpu-wasm" => format!("dev-{}-wgpu-dev", row.variant), _ => format!("dev-{}-react-dev", row.variant) };
                let mut args = vec!["nx".into(), "run".into(), format!("@semio-tech/framework-os-dev:{target}")];
                if renderer == "wgpu-native" { if let Some(id) = example { args.extend(["--".into(), "--example".into(), id.clone()]); } }
                let spec = CommandSpec { cmd: "bun".into(), args, cwd: root.to_path_buf(), env };
                let mut path = vec![segment("dev", "dev"), segment(segment_key(&row.plugin_id), row.plugin_id.clone()), segment(segment_key(&row.variant), row.variant.clone()), segment(renderer, renderer)];
                if let Some(id) = example { path.extend([segment("examples", "examples"), segment(id, id)]); }
                else { path.push(segment("all", "all examples")); }
                trie.insert_path(&path, CommandLeaf::Process(spec));
            }
        }
    }
}

