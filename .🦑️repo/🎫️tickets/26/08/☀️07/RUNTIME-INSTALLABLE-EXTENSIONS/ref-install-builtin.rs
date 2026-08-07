fn install_builtin_flow_extensions(registry: &mut neural::Registry) {
    flow_extension_core::register(registry);
    flow_extension_math::register(registry);
    flow_extension_text::register(registry);
    flow_extension_logic::register(registry);
    flow_extension_dictionary::register(registry);
    flow_extension_list::register(registry);
    flow_extension_brep::register(registry);
    flow_extension_draw::register(registry);
}

struct ContributedExtensionStub {
    extension_id: String,
    operator_id: String,
}

impl neural::Operation for ContributedExtensionStub {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let node_hash = neural::node_hash(&self.operator_id, input);
        Err(EvalError::PendingExtension { extension_id: self.extension_id.clone(), operator_id: self.operator_id.clone(), node_hash })
    }
}

fn register_contributed_manifest(registry: &mut neural::Registry, plugin_id: &str, manifest_json: &str) {
    let Ok(manifest) = serde_json::from_str::<FlowExtensionManifest>(manifest_json) else { return };
    for schema in manifest.contributes.schemas {
        if !registry.schema_catalogue().iter().any(|existing| existing.id == schema.id) {
            registry.register_schema(schema);
        }
    }
    for info in manifest.contributes.operators {
        if registry.operator_info(&info.id).is_some() {
            continue;
        }
        let extension_id = manifest.id.clone();
        let operator_id = info.id.clone();
        registry.register_operator(
            info,
            vec![OperatorImpl { schemas: vec![], operation: Box::new(ContributedExtensionStub { extension_id, operator_id }) }],
            &[],
        );
    }
    let _ = plugin_id;
    registry.finalize();
}

fn build_flow_extension_registry(contributed: &BTreeMap<String, ContributedFlowExtension>) -> neural::Registry {
    let mut registry = neural::Registry::new();
    install_builtin_flow_extensions(&mut registry);
    for entry in contributed.values() {
        register_contributed_manifest(&mut registry, &entry.plugin_id, &entry.manifest_json);
    }
    registry
}

fn rebuild_flow_extension_registry(state: &mut FlowExtensionRegistryState) {
    state.generation += 1;
    state.registry = Arc::new(build_flow_extension_registry(&state.contributed));
}

/// 🔌️ Installs a built-in extension spec (idempotent on `id`).
pub fn install_flow_extension(spec: FlowExtensionSpec) {
    let mut state = FLOW_EXTENSION_STATE.lo