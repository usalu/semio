install_first_party_light_flow_extensions_for_tests() {
        use std::sync::Once;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            let mut state = FLOW_EXTENSION_STATE.lock().expect("flow extension registry");
            let mut registry = neural::Registry::new();
            install_builtin_flow_extensions(&mut registry);
            semio_s_plugin_flow_extension_core::register(&mut registry);
            semio_s_plugin_flow_extension_math::register(&mut registry);
            semio_s_plugin_flow_extension_text::register(&mut registry);
            semio_s_plugin_flow_extension_logic::register(&mut registry);
            semio_s_plugin_flow_extension_dictionary::register(&mut registry);
            semio_s_plugin_flow_extension_list::register(&mut registry);
            registry.finalize();
            state.registry = std::sync::Arc::new(registry);
            state.generation += 1;
        });
    }

    fn fixture_kind_infos_json() -> String {
        install_first_party_light_flow_extensions_for_tests();
        serde_json::to_string(&flow_extension_registry().operator_catalogue()).unwrap_or_else(|_| "[]".into())
    }

    fn test_kind_infos_json() -> String {
        serde_json::to_string(&[
            NeuronKindInfo {
                id: "math.add".into(),
                extension: "math".into(),
                name: "Add".into(),
                abbreviation: "Add".into(),
                icon: "emoji:➕️".into(),
               