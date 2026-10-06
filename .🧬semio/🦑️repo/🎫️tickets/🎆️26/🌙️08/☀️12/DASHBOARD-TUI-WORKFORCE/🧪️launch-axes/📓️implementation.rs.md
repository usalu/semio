fn inject_playground_dev(root: &Path, trie: &mut TrieNode) {
    for row in crate::catalog::load_playground_catalog(root) {
        for renderer in ["react", "wgpu-wasm", "wgpu-native"] {
            for example in std::iter::once(None).chain(row.examples.iter().map(Some)) {
                for locale in ui_locale::Locale::ALL {
                    for terminology in ui_locale::Terminology::ALL {
                        let opts = crate::env_contract::DevOptions { renderer: renderer.into(), example: example.map_or(crate::options::Lock::All, |id| crate::options::Lock::Individual(id.clone())), language: crate::options::Lock::Individual(locale.as_str().into()), terminology: crate::options::Lock::Individual(terminology.as_str().into()), ..Default::default() };
                        let mut env = crate::env_contract::build_dev_env(&row.variant, Some(&row), &opts);
                        env.extend(nx_env());
                        let target = match renderer { "wgpu-native" => format!("run-{}-native-dev", row.variant), "wgpu-wasm" => format!("dev-{}-wgpu-dev", row.variant), _ => format!("dev-{}-react-dev", row.variant) };
                        let mut args = vec!["nx".into(), "run".into(), format!("@semio-tech/framework-os-dev:{target}")];
                        if renderer == "wgpu-native" { if let Some(id) = example { args.extend(["--".into(), "--example".into(), id.clone()]); } }
                        let spec = CommandSpec { cmd: "bun".into(), args, cwd: root.to_path_buf(), env };
                        let mut path = vec![segment("dev", "dev"), segment(segment_key(&row.plugin_id), row.plugin_id.clone()), segment(segment_key(&row.variant), row.variant.clone()), segment(renderer, renderer)];
                        if let Some(id) = example { path.extend([segment("examples", "examples"), segment(id, id)]); }
                        else { path.push(segment("all", "all examples")); }
                        let language = match locale { ui_locale::Locale::En => "English", ui_locale::Locale::De => "Deutsch" };
                        let terms = match terminology { ui_locale::Terminology::Native => "Native / Eigene Begriffe", ui_locale::Terminology::Reuse => "Reuse / Wiederverwendung" };
                        path.extend([segment("language", "Language / Sprache"), segment(locale.as_str(), language), segment("terminology", "Terminology / Begriffe"), segment(terminology.as_str(), terms)]);
                        trie.insert_path(&path, CommandLeaf::Process(spec));
                    }
                }
            }
        }
    }
}
