//! 🖥️ Plugin-based OS kernel: hot-swappable WASM components, media graph, document VCS.

pub mod host {
// #region host
//! 🔌 Plugin host, studio document VCS store, backbone, and catalog.

use crate::instance::{
    create_default_os_parameter, create_os_document_id, create_os_id, patch_os_parameter, OsAppInstance,
    OsDocumentRef, OsInstanceState, OsParameter, OsParameterFieldBinding, OsParameterType,
};
use crate::media_graph::{
    empty_media_graph, media_graph_node_for_instance, sync_media_graph_parameter_ports,
    MediaGraphPosition, OsMediaGraph, OsMediaGraphEdge, OS_MEDIA_GRAPH_SCHEMA,
    OS_STUDIO_SCHEMA,
};
use crate::registry::{
    os_app_primary_output_kind, os_app_registration, PluginRegistry,
};
use semio_framework_core::{
    AppDefinition, ActionContext, ActionInvocation, InvocationResult,
    Contribution, HybridLogicalTimestamp, InverseOperation, KernelOperation, DocumentDiff, DocumentHandle, DocumentVersion,
    OperationId, PluginManifest, SchemaId, UndoGroup, UndoPolicy, ViewState,
};
use ui_wgpu::{ActionDescriptor, UiButtonNode, UiNode, ui_stack_vertical, ui_text};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};
use vcs::{
    create_document_vcs_envelope, document_backbone_ref, materialize_document_projection, DocumentBackboneRef,
    DocumentVcs, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff, VcsError,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginHotSwapEvent {
    pub plugin_id: String,
    pub version: String,
    pub added_apps: Vec<String>,
    pub removed_apps: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoadedPlugin {
    pub plugin_id: String,
    pub manifest: PluginManifest,
    pub artifact_uri: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginContributionEntry {
    pub plugin_id: String,
    pub contribution: Contribution,
}

//#region 🔖PluginSupervisorState
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluginSupervisorState {
    Loaded,
    Running,
    Crashed,
    TimedOut,
    Restarting,
    Quarantined,
    Unloaded,
}
//#endregion 🔖PluginSupervisorState

pub struct PluginHost {
    registry: PluginRegistry,
    instances: HashMap<u32, OsInstanceState>,
    next_instance_id: u32,
    plugins: HashMap<String, LoadedPlugin>,
    supervisor: HashMap<String, PluginSupervisorState>,
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            instances: HashMap::new(),
            next_instance_id: 1,
            plugins: HashMap::new(),
            supervisor: HashMap::new(),
        }
    }

    pub fn supervisor_state(&self, plugin_id: &str) -> Option<PluginSupervisorState> {
        self.supervisor.get(plugin_id).copied()
    }

    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }

    pub fn load_plugin(&mut self, plugin: LoadedPlugin) -> PluginHotSwapEvent {
        let plugin_id = plugin.plugin_id.clone();
        let version = plugin.manifest.version.clone();
        let previous_apps: Vec<String> = self
            .plugins
            .get(&plugin_id)
            .map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect())
            .unwrap_or_default();
        let next_apps: Vec<String> = plugin.manifest.apps.iter().map(|app| app.id.clone()).collect();
        for app in &plugin.manifest.apps {
            self.registry.register_app(app.clone());
        }
        for program in &plugin.manifest.programs {
            self.registry.register_program(program.clone());
        }
        self.plugins.insert(plugin_id.clone(), plugin);
        self.supervisor
            .insert(plugin_id.clone(), PluginSupervisorState::Running);
        PluginHotSwapEvent {
            plugin_id,
            version,
            added_apps: next_apps
                .iter()
                .filter(|app| !previous_apps.contains(app))
                .cloned()
                .collect(),
            removed_apps: previous_apps
                .iter()
                .filter(|app| !next_apps.contains(app))
                .cloned()
                .collect(),
        }
    }

    pub fn hot_swap_plugin(&mut self, plugin: LoadedPlugin) -> PluginHotSwapEvent {
        let plugin_id = plugin.plugin_id.clone();
        let rollback = HotSwapRollback {
            previous_plugin: self.plugins.get(&plugin_id).cloned(),
            instance_generations: self
                .instances
                .iter()
                .map(|(id, state)| (*id, state.generation))
                .collect(),
        };

        if let Err(error) = validate_plugin_manifest(&plugin) {
            self.supervisor
                .insert(plugin_id.clone(), PluginSupervisorState::Loaded);
            return rollback.emit_failure(plugin_id, error);
        }

        let previous_apps: Vec<String> = rollback
            .previous_plugin
            .as_ref()
            .map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect())
            .unwrap_or_default();
        let next_apps: Vec<String> = plugin.manifest.apps.iter().map(|app| app.id.clone()).collect();

        if let Err(error) = self.validate_swap_apps(&plugin) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }
        if let Err(error) = self.validate_swap_instances(&plugin_id, &plugin) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }
        if let Err(error) = self.validate_swap_app_retention(&plugin, rollback.previous_plugin.as_ref()) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }
        if let Err(error) = self.validate_swap_window_kinds(&plugin) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }

        let controller_rebindings = self.plan_controller_rebindings(&plugin_id, &plugin);
        let version = plugin.manifest.version.clone();
        for app in &plugin.manifest.apps {
            self.registry.register_app(app.clone());
        }
        for program in &plugin.manifest.programs {
            self.registry.register_program(program.clone());
        }
        self.plugins.insert(plugin_id.clone(), plugin);
        for (instance_id, controller_id) in controller_rebindings {
            if let Some(instance) = self.instances.get_mut(&instance_id) {
                instance.controller_id = controller_id;
            }
        }
        for instance in self.instances.values_mut() {
            instance.generation += 1;
        }
        self.supervisor
            .insert(plugin_id.clone(), PluginSupervisorState::Running);
        PluginHotSwapEvent {
            plugin_id,
            version,
            added_apps: next_apps
                .iter()
                .filter(|app| !previous_apps.contains(app))
                .cloned()
                .collect(),
            removed_apps: previous_apps
                .iter()
                .filter(|app| !next_apps.contains(app))
                .cloned()
                .collect(),
        }
    }

    pub fn apps(&self) -> Vec<AppDefinition> {
        self.registry.apps()
    }

    pub fn contributions(&self) -> Vec<PluginContributionEntry> {
        let mut entries = Vec::new();
        for plugin in self.plugins.values() {
            for contribution in &plugin.manifest.contributions {
                entries.push(PluginContributionEntry {
                    plugin_id: plugin.plugin_id.clone(),
                    contribution: contribution.clone(),
                });
            }
        }
        entries
    }

    pub fn contributions_json(&self) -> String {
        serde_json::to_string(&self.contributions()).unwrap_or_else(|_| "[]".into())
    }

    pub fn create_instance(&mut self, app_id: &str, document_json: String) -> Option<u32> {
        let app = self.registry.find_app(app_id)?;
        let id = self.next_instance_id;
        self.next_instance_id += 1;
        self.instances.insert(
            id,
            OsInstanceState {
                id,
                app_id: app.id.clone(),
                controller_id: app.controller_id.clone(),
                document_json,
                view_state: ViewState::default(),
                generation: 0,
            },
        );
        Some(id)
    }

    pub fn instance(&self, instance_id: u32) -> Option<&OsInstanceState> {
        self.instances.get(&instance_id)
    }

    pub fn instance_mut(&mut self, instance_id: u32) -> Option<&mut OsInstanceState> {
        self.instances.get_mut(&instance_id)
    }

    pub fn commit_action_result(
        &mut self,
        instance_id: u32,
        result: &InvocationResult,
    ) -> Result<(), String> {
        let Some(instance) = self.instances.get_mut(&instance_id) else {
            return Err("instance not found".into());
        };
        for operation in &result.operations {
            if operation.diff.schema_id.0 != JSON_PATCH_SCHEMA_ID {
                continue;
            }
            let op_json =
                serde_json::to_string(&operation.diff.payload).map_err(|error| error.to_string())?;
            instance.document_json = apply_kernel_patch_op(&instance.document_json, &op_json)?;
            instance.generation += 1;
        }
        Ok(())
    }
    pub fn invoke_action(
        &mut self,
        invocation: ActionInvocation,
    ) -> Result<InvocationResult, String> {
        if invocation.action.0.trim().is_empty() {
            return Err("action id must not be empty".into());
        }
        let instance_id: u32 = invocation
            .app
            .0
            .parse()
            .map_err(|_| "invalid app instance id".to_string())?;
        let (document_json, view_state, generation) = {
            let instance = self
                .instances
                .get(&instance_id)
                .ok_or_else(|| "instance not found".to_string())?;
            (
                instance.document_json.clone(),
                instance.view_state.clone(),
                instance.generation,
            )
        };
        let document_projection =
            serde_json::from_str(&document_json).unwrap_or(Value::Null);
        let _context = ActionContext {
            invocation: invocation.clone(),
            document_projection,
            view_state,
            granted_capabilities: vec![],
        };
        let document = DocumentHandle(instance_id as u128);
        let base_version = DocumentVersion(generation);
        let patch_ops = extract_patch_ops(&invocation.input);
        let operations: Vec<KernelOperation> = patch_ops
            .iter()
            .enumerate()
            .map(|(index, op)| {
                kernel_operation_from_patch_op(&invocation, op, index, base_version, document)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let operation_ids: Vec<OperationId> = operations.iter().map(|op| op.id.clone()).collect();
        let inverse_operations: Vec<InverseOperation> =
            operations.iter().map(|op| op.inverse.clone()).collect();
        let result = InvocationResult {
            output: invocation.input.clone(),
            operations,
            inverse_group: UndoGroup {
                invocation_id: invocation.id.clone(),
                operations: operation_ids,
                inverse_operations,
            },
            diagnostics: vec![],
            requested_effects: vec![],
            events: vec![],
            ui_scope: Default::default(),
        };
        self.commit_action_result(instance_id, &result)?;
        Ok(result)
    }

    //#region 🔖ActionKernel

    pub fn recovery_ui(&self, plugin_id: &str) -> UiNode {
        let state = self
            .supervisor
            .get(plugin_id)
            .copied()
            .unwrap_or(PluginSupervisorState::Unloaded);
        if state != PluginSupervisorState::Quarantined {
            return ui_stack_vertical(vec![ui_text("Plugin is not quarantined.")]);
        }
        ui_stack_vertical(vec![
            ui_text("This app stopped responding."),
            UiNode::Button(UiButtonNode {
                id: Some("recovery-restart-app".into()),
                icon_id: "restart".into(),
                label: "Restart app".into(),
                action: ActionDescriptor {
                    controller_id: plugin_id.into(),
                    action: "recovery.restartApp".into(),
                    args: None,
                },
                style: None,
                disabled: None,
                loading: None,
            }),
            UiNode::Button(UiButtonNode {
                id: Some("recovery-disable-plugin".into()),
                icon_id: "disable".into(),
                label: "Disable plugin".into(),
                action: ActionDescriptor {
                    controller_id: plugin_id.into(),
                    action: "recovery.disablePlugin".into(),
                    args: None,
                },
                style: None,
                disabled: None,
                loading: None,
            }),
            UiNode::Button(UiButtonNode {
                id: Some("recovery-show-diagnostics".into()),
                icon_id: "diagnostics".into(),
                label: "Show diagnostics".into(),
                action: ActionDescriptor {
                    controller_id: plugin_id.into(),
                    action: "recovery.showDiagnostics".into(),
                    args: None,
                },
                style: None,
                disabled: None,
                loading: None,
            }),
        ])
    }
    //#endregion 🔖ActionKernel

    pub fn set_view_state(&mut self, instance_id: u32, view_state: ViewState) {
        if let Some(instance) = self.instances.get_mut(&instance_id) {
            instance.view_state = view_state;
            instance.generation += 1;
        }
    }

    pub fn render_body(&self, instance_id: u32, body_key: &str, ui: UiNode) -> UiNode {
        let _ = (instance_id, body_key);
        ui
    }

    fn hot_swap_failed(
        &mut self,
        plugin_id: String,
        error: String,
        rollback: HotSwapRollback,
    ) -> PluginHotSwapEvent {
        rollback.restore(self);
        self.supervisor
            .insert(plugin_id.clone(), PluginSupervisorState::Loaded);
        rollback.emit_failure(plugin_id, error)
    }

    fn validate_swap_apps(&self, plugin: &LoadedPlugin) -> Result<(), String> {
        for app in &plugin.manifest.apps {
            if app.id.trim().is_empty() {
                return Err("app id must not be empty".into());
            }
            if app.controller_id.trim().is_empty() {
                return Err(format!("app {} controller_id must not be empty", app.id));
            }
        }
        Ok(())
    }

    fn validate_swap_instances(&self, plugin_id: &str, plugin: &LoadedPlugin) -> Result<(), String> {
        let next_app_ids: HashSet<String> = plugin
            .manifest
            .apps
            .iter()
            .map(|app| app.id.clone())
            .collect();
        let previous_app_ids: HashSet<String> = self
            .plugins
            .get(plugin_id)
            .map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect())
            .unwrap_or_default();
        for instance in self.instances.values() {
            if !previous_app_ids.contains(&instance.app_id) {
                continue;
            }
            if !next_app_ids.contains(&instance.app_id) {
                return Err(format!(
                    "instance {} references removed app {}",
                    instance.id, instance.app_id
                ));
            }
        }
        Ok(())
    }

    /// @emoji 🔢 Same-version hot-swaps must not silently drop apps — a version bump is required to
    /// shrink the app set, so a client relying on document/instance continuity can detect the change.
    fn validate_swap_app_retention(
        &self,
        plugin: &LoadedPlugin,
        previous: Option<&LoadedPlugin>,
    ) -> Result<(), String> {
        if let Some(previous) = previous {
            if previous.manifest.version == plugin.manifest.version
                && previous.manifest.apps.len() > plugin.manifest.apps.len()
            {
                return Err("cannot hot-swap to fewer apps within the same version".into());
            }
        }
        Ok(())
    }

    fn validate_swap_window_kinds(&self, plugin: &LoadedPlugin) -> Result<(), String> {
        for app in &plugin.manifest.apps {
            if app.window_kinds.is_empty() {
                return Err(format!("app {} must declare at least one window kind", app.id));
            }
            for window_kind in &app.window_kinds {
                if window_kind.body_key.trim().is_empty() {
                    return Err(format!(
                        "app {} window kind {} body_key must not be empty",
                        app.id, window_kind.id
                    ));
                }
            }
        }
        Ok(())
    }

    fn plan_controller_rebindings(
        &self,
        plugin_id: &str,
        plugin: &LoadedPlugin,
    ) -> Vec<(u32, String)> {
        let apps_by_id: HashMap<&str, &AppDefinition> = plugin
            .manifest
            .apps
            .iter()
            .map(|app| (app.id.as_str(), app))
            .collect();
        let previous_app_ids: HashSet<String> = self
            .plugins
            .get(plugin_id)
            .map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect())
            .unwrap_or_default();
        self.instances
            .values()
            .filter(|instance| previous_app_ids.contains(&instance.app_id))
            .filter_map(|instance| {
                apps_by_id
                    .get(instance.app_id.as_str())
                    .map(|app| (instance.id, app.controller_id.clone()))
            })
            .collect()
    }
}

const JSON_PATCH_SCHEMA_ID: &str = "semio.kernel.json-patch";

struct HotSwapRollback {
    previous_plugin: Option<LoadedPlugin>,
    instance_generations: HashMap<u32, u64>,
}

impl HotSwapRollback {
    fn emit_failure(self, plugin_id: String, _error: String) -> PluginHotSwapEvent {
        let version = self
            .previous_plugin
            .as_ref()
            .map(|plugin| plugin.manifest.version.clone())
            .unwrap_or_default();
        PluginHotSwapEvent {
            plugin_id,
            version,
            added_apps: vec![],
            removed_apps: vec![],
        }
    }

    fn restore(&self, host: &mut PluginHost) {
        if let Some(previous) = &self.previous_plugin {
            for app in &previous.manifest.apps {
                host.registry.register_app(app.clone());
            }
            for program in &previous.manifest.programs {
                host.registry.register_program(program.clone());
            }
            host.plugins
                .insert(previous.plugin_id.clone(), previous.clone());
        }
        for (instance_id, generation) in &self.instance_generations {
            if let Some(instance) = host.instances.get_mut(instance_id) {
                instance.generation = *generation;
            }
        }
    }
}

fn validate_plugin_manifest(plugin: &LoadedPlugin) -> Result<(), String> {
    if plugin.plugin_id.trim().is_empty() {
        return Err("plugin_id must not be empty".into());
    }
    if plugin.manifest.plugin_id.trim().is_empty() {
        return Err("manifest.plugin_id must not be empty".into());
    }
    if plugin.manifest.version.trim().is_empty() {
        return Err("manifest.version must not be empty".into());
    }
    if plugin.plugin_id != plugin.manifest.plugin_id {
        return Err("plugin_id must match manifest.plugin_id".into());
    }
    Ok(())
}

fn extract_patch_ops(input: &Value) -> Vec<String> {
    input
        .get("ops")
        .and_then(|value| value.as_array())
        .map(|ops| {
            ops.iter()
                .filter_map(|op| {
                    op.as_str()
                        .map(str::to_string)
                        .or_else(|| serde_json::to_string(op).ok())
                })
                .collect()
        })
        .unwrap_or_default()
}

fn kernel_operation_from_patch_op(
    invocation: &ActionInvocation,
    op_json: &str,
    index: usize,
    base_version: DocumentVersion,
    document: DocumentHandle,
) -> Result<KernelOperation, String> {
    let payload: Value = serde_json::from_str(op_json).map_err(|error| error.to_string())?;
    let operation_id = OperationId(format!("{}:{index}", invocation.id.0));
    let inverse_diff = DocumentDiff {
        schema_id: SchemaId("semio.kernel.json-patch.inverse".into()),
        payload: Value::Null,
    };
    Ok(KernelOperation {
        id: operation_id.clone(),
        document,
        base_version,
        invocation_id: invocation.id.clone(),
        diff: DocumentDiff {
            schema_id: SchemaId(JSON_PATCH_SCHEMA_ID.into()),
            payload,
        },
        inverse: InverseOperation {
            target_operation: operation_id,
            inverse_diff,
            base_version,
            dependencies: vec![],
            undo_policy: UndoPolicy::ExactBaseOnly,
        },
        dependencies: invocation.causal_context.clone(),
        author: invocation.actor.clone(),
        timestamp: HybridLogicalTimestamp::new(0, 0),
    })
}

fn apply_kernel_patch_op(document_json: &str, op_json: &str) -> Result<String, String> {
    let mut document: serde_json::Value =
        serde_json::from_str(document_json).map_err(|error| error.to_string())?;
    let op: serde_json::Value = serde_json::from_str(op_json).map_err(|error| error.to_string())?;
    match op.get("op").and_then(|value| value.as_str()) {
        Some("setDocument") => {
            if let Some(next) = op.get("document") {
                document = next.clone();
            }
        }
        Some("patch") => {
            if let Some(patch) = op.get("patch") {
                merge_json(&mut document, patch);
            }
        }
        _ => {}
    }
    serde_json::to_string(&document).map_err(|error| error.to_string())
}

fn merge_json(target: &mut serde_json::Value, patch: &serde_json::Value) {
    match (target, patch) {
        (serde_json::Value::Object(target_map), serde_json::Value::Object(patch_map)) => {
            for (key, value) in patch_map {
                if value.is_null() {
                    target_map.remove(key);
                } else {
                    let entry = target_map
                        .entry(key.clone())
                        .or_insert(serde_json::Value::Null);
                    merge_json(entry, value);
                }
            }
        }
        (target_slot, patch_value) => {
            *target_slot = patch_value.clone();
        }
    }
}

//#region 🔖OsDocument
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsProjection {
    pub programs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    pub app_instances: Vec<OsAppInstance>,
    pub media_graph: OsMediaGraph,
    #[serde(default)]
    pub parameters: Vec<OsParameter>,
    #[serde(default)]
    pub parameter_bindings: Vec<OsParameterFieldBinding>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum OsOp {
    SetActiveProgram {
        #[serde(skip_serializing_if = "Option::is_none")]
        program_id: Option<String>,
    },
    SetActiveAlternative {
        #[serde(skip_serializing_if = "Option::is_none")]
        alternative_id: Option<String>,
    },
    SpawnAppInstance {
        instance: OsAppInstance,
        position: MediaGraphPosition,
    },
    RemoveAppInstance {
        instance_id: String,
    },
    ConnectMediaPorts {
        edge: OsMediaGraphEdge,
    },
    DisconnectMediaEdge {
        edge_id: String,
    },
    MoveMediaNode {
        node_id: String,
        x: f64,
        y: f64,
    },
    PatchAppInstance {
        instance_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    },
    AddParameter {
        parameter: OsParameter,
    },
    RemoveParameter {
        parameter_id: String,
    },
    PatchParameter {
        parameter_id: String,
        parameter: OsParameter,
    },
    BindParameterField {
        binding: OsParameterFieldBinding,
    },
    UnbindParameterField {
        instance_id: String,
        field_path: String,
    },
    SyncParameterPorts,
}

pub type OsVcs = DocumentVcs<OsProjection, OsOp>;

/// @emoji 🩹 Explicit reexport of `serde_json::Value` — the shape of a parameter patch, so callers
/// don't reach across the crate boundary into `serde_json` directly.
pub type OsParameterPatch = Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsDocument {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub vcs: OsVcs,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applied_edit_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<DocumentBackboneRef>,
}

pub type OsEnvelope = DocumentVcsEnvelope<OsProjection, OsOp>;

pub fn default_os_projection() -> OsProjection {
    OsProjection {
        programs: Vec::new(),
        active_program_id: None,
        active_alternative_id: None,
        app_instances: Vec::new(),
        media_graph: empty_media_graph(),
        parameters: Vec::new(),
        parameter_bindings: Vec::new(),
    }
}

pub fn create_empty_os_document(id: &str, name: &str) -> OsDocument {
    OsDocument {
        schema: OS_STUDIO_SCHEMA.into(),
        id: id.into(),
        name: name.into(),
        vcs: create_document_vcs_envelope(
            OS_STUDIO_SCHEMA,
            id,
            default_os_projection(),
            None,
        )
        .vcs,
        applied_edit_ids: Vec::new(),
        backbone: None,
    }
}

pub fn apply_os_operation(projection: &OsProjection, operation: &OsOp) -> OsProjection {
    let mut next = projection.clone();
    match operation {
        OsOp::SetActiveProgram { program_id } => {
            next.active_program_id = program_id.clone();
        }
        OsOp::SetActiveAlternative { alternative_id } => {
            next.active_alternative_id = alternative_id.clone();
        }
        OsOp::SpawnAppInstance { instance, position } => {
            if !next.programs.contains(&instance.program_id) {
                next.programs.push(instance.program_id.clone());
            }
            if let Some(registration) = os_app_registration(&instance.program_id, &instance.app_id) {
                let node = sync_media_node_parameter_ports(
                    &media_graph_node_for_instance(
                        instance,
                        &registration,
                        position,
                        &create_os_id("node"),
                    ),
                    &next.parameter_bindings,
                );
                next.media_graph.nodes.push(node);
            }
            next.app_instances.push(instance.clone());
        }
        OsOp::RemoveAppInstance { instance_id } => {
            let node_id = next
                .media_graph
                .nodes
                .iter()
                .find(|node| node.instance_id == *instance_id)
                .map(|node| node.id.clone());
            next.app_instances.retain(|instance| instance.id != *instance_id);
            next.parameter_bindings
                .retain(|binding| binding.instance_id != *instance_id);
            next.media_graph
                .nodes
                .retain(|node| node.instance_id != *instance_id);
            if let Some(node_id) = node_id {
                next.media_graph.edges.retain(|edge| {
                    edge.source_node_id != node_id && edge.target_node_id != node_id
                });
            }
        }
        OsOp::ConnectMediaPorts { edge } => next.media_graph.edges.push(edge.clone()),
        OsOp::DisconnectMediaEdge { edge_id } => next
            .media_graph
            .edges
            .retain(|edge| edge.id != *edge_id),
        OsOp::MoveMediaNode { node_id, x, y } => {
            for node in &mut next.media_graph.nodes {
                if node.id == *node_id {
                    node.x = *x;
                    node.y = *y;
                }
            }
        }
        OsOp::PatchAppInstance { instance_id, label } => {
            if let Some(label) = label {
                for instance in &mut next.app_instances {
                    if instance.id == *instance_id {
                        instance.label = label.clone();
                    }
                }
            }
        }
        OsOp::AddParameter { parameter } => next.parameters.push(parameter.clone()),
        OsOp::RemoveParameter { parameter_id } => {
            next.parameters.retain(|parameter| parameter_entity_id(parameter) != *parameter_id);
            next.parameter_bindings
                .retain(|binding| binding.parameter_id != *parameter_id);
            next.media_graph =
                sync_media_graph_parameter_ports(&next.media_graph, &next.parameter_bindings);
        }
        OsOp::PatchParameter {
            parameter_id,
            parameter,
        } => {
            for entry in &mut next.parameters {
                if parameter_entity_id(entry) == *parameter_id {
                    *entry = parameter.clone();
                }
            }
        }
        OsOp::BindParameterField { binding } => {
            next.parameter_bindings.retain(|entry| {
                !(entry.instance_id == binding.instance_id && entry.field_path == binding.field_path)
            });
            next.parameter_bindings.push(binding.clone());
            next.media_graph =
                sync_media_graph_parameter_ports(&next.media_graph, &next.parameter_bindings);
        }
        OsOp::UnbindParameterField {
            instance_id,
            field_path,
        } => {
            next.parameter_bindings.retain(|binding| {
                !(binding.instance_id == *instance_id && binding.field_path == *field_path)
            });
            next.media_graph =
                sync_media_graph_parameter_ports(&next.media_graph, &next.parameter_bindings);
        }
        OsOp::SyncParameterPorts => {
            next.media_graph =
                sync_media_graph_parameter_ports(&next.media_graph, &next.parameter_bindings);
        }
    }
    next
}

fn sync_media_node_parameter_ports(
    node: &crate::media_graph::OsMediaGraphNode,
    bindings: &[OsParameterFieldBinding],
) -> crate::media_graph::OsMediaGraphNode {
    sync_media_graph_parameter_ports(
        &OsMediaGraph {
            schema: OS_MEDIA_GRAPH_SCHEMA.into(),
            nodes: vec![node.clone()],
            edges: Vec::new(),
        },
        bindings,
    )
    .nodes
    .into_iter()
    .next()
    .unwrap_or_else(|| node.clone())
}

fn parameter_entity_id(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { id, .. }
        | OsParameter::Categorical { id, .. }
        | OsParameter::Toggle { id, .. }
        | OsParameter::Text { id, .. } => id,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum OsDiff {
    #[default]
    Empty,
    SetActiveProgram {
        #[serde(skip_serializing_if = "Option::is_none")]
        program_id: Option<String>,
    },
    SetActiveAlternative {
        #[serde(skip_serializing_if = "Option::is_none")]
        alternative_id: Option<String>,
    },
    SpawnAppInstance {
        instance: OsAppInstance,
        position: MediaGraphPosition,
    },
    RemoveAppInstance {
        instance_id: String,
    },
    ConnectMediaPorts {
        edge: OsMediaGraphEdge,
    },
    DisconnectMediaEdge {
        edge_id: String,
    },
    MoveMediaNode {
        node_id: String,
        x: f64,
        y: f64,
    },
    PatchAppInstance {
        instance_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    },
    AddParameter {
        parameter: OsParameter,
    },
    RemoveParameter {
        parameter_id: String,
    },
    PatchParameter {
        parameter_id: String,
        parameter: OsParameter,
    },
    BindParameterField {
        binding: OsParameterFieldBinding,
    },
    UnbindParameterField {
        instance_id: String,
        field_path: String,
    },
    SyncParameterPorts,
}

impl OperationDiff<OsProjection> for OsDiff {
    fn apply(&self, projection: &OsProjection) -> OsProjection {
        let op = match self {
            OsDiff::Empty => return projection.clone(),
            OsDiff::SetActiveProgram { program_id } => OsOp::SetActiveProgram {
                program_id: program_id.clone(),
            },
            OsDiff::SetActiveAlternative { alternative_id } => OsOp::SetActiveAlternative {
                alternative_id: alternative_id.clone(),
            },
            OsDiff::SpawnAppInstance { instance, position } => OsOp::SpawnAppInstance {
                instance: instance.clone(),
                position: position.clone(),
            },
            OsDiff::RemoveAppInstance { instance_id } => OsOp::RemoveAppInstance {
                instance_id: instance_id.clone(),
            },
            OsDiff::ConnectMediaPorts { edge } => OsOp::ConnectMediaPorts { edge: edge.clone() },
            OsDiff::DisconnectMediaEdge { edge_id } => OsOp::DisconnectMediaEdge {
                edge_id: edge_id.clone(),
            },
            OsDiff::MoveMediaNode { node_id, x, y } => OsOp::MoveMediaNode {
                node_id: node_id.clone(),
                x: *x,
                y: *y,
            },
            OsDiff::PatchAppInstance { instance_id, label } => OsOp::PatchAppInstance {
                instance_id: instance_id.clone(),
                label: label.clone(),
            },
            OsDiff::AddParameter { parameter } => OsOp::AddParameter {
                parameter: parameter.clone(),
            },
            OsDiff::RemoveParameter { parameter_id } => OsOp::RemoveParameter {
                parameter_id: parameter_id.clone(),
            },
            OsDiff::PatchParameter {
                parameter_id,
                parameter,
            } => OsOp::PatchParameter {
                parameter_id: parameter_id.clone(),
                parameter: parameter.clone(),
            },
            OsDiff::BindParameterField { binding } => OsOp::BindParameterField {
                binding: binding.clone(),
            },
            OsDiff::UnbindParameterField {
                instance_id,
                field_path,
            } => OsOp::UnbindParameterField {
                instance_id: instance_id.clone(),
                field_path: field_path.clone(),
            },
            OsDiff::SyncParameterPorts => OsOp::SyncParameterPorts,
        };
        apply_os_operation(projection, &op)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, OsDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<OsProjection> for OsOp {
    type Diff = OsDiff;

    fn diff(&self, _projection: &OsProjection) -> OsDiff {
        match self {
            OsOp::SetActiveProgram { program_id } => OsDiff::SetActiveProgram {
                program_id: program_id.clone(),
            },
            OsOp::SetActiveAlternative { alternative_id } => OsDiff::SetActiveAlternative {
                alternative_id: alternative_id.clone(),
            },
            OsOp::SpawnAppInstance { instance, position } => OsDiff::SpawnAppInstance {
                instance: instance.clone(),
                position: position.clone(),
            },
            OsOp::RemoveAppInstance { instance_id } => OsDiff::RemoveAppInstance {
                instance_id: instance_id.clone(),
            },
            OsOp::ConnectMediaPorts { edge } => OsDiff::ConnectMediaPorts { edge: edge.clone() },
            OsOp::DisconnectMediaEdge { edge_id } => OsDiff::DisconnectMediaEdge {
                edge_id: edge_id.clone(),
            },
            OsOp::MoveMediaNode { node_id, x, y } => OsDiff::MoveMediaNode {
                node_id: node_id.clone(),
                x: *x,
                y: *y,
            },
            OsOp::PatchAppInstance { instance_id, label } => OsDiff::PatchAppInstance {
                instance_id: instance_id.clone(),
                label: label.clone(),
            },
            OsOp::AddParameter { parameter } => OsDiff::AddParameter {
                parameter: parameter.clone(),
            },
            OsOp::RemoveParameter { parameter_id } => OsDiff::RemoveParameter {
                parameter_id: parameter_id.clone(),
            },
            OsOp::PatchParameter {
                parameter_id,
                parameter,
            } => OsDiff::PatchParameter {
                parameter_id: parameter_id.clone(),
                parameter: parameter.clone(),
            },
            OsOp::BindParameterField { binding } => OsDiff::BindParameterField {
                binding: binding.clone(),
            },
            OsOp::UnbindParameterField {
                instance_id,
                field_path,
            } => OsDiff::UnbindParameterField {
                instance_id: instance_id.clone(),
                field_path: field_path.clone(),
            },
            OsOp::SyncParameterPorts => OsDiff::SyncParameterPorts,
        }
    }

    fn backwards(&self, projection: &OsProjection) -> Vec<Self> {
        match self {
            OsOp::SetActiveProgram { .. } => vec![OsOp::SetActiveProgram {
                program_id: projection.active_program_id.clone(),
            }],
            OsOp::SetActiveAlternative { .. } => vec![OsOp::SetActiveAlternative {
                alternative_id: projection.active_alternative_id.clone(),
            }],
            OsOp::SpawnAppInstance { instance, .. } => vec![OsOp::RemoveAppInstance {
                instance_id: instance.id.clone(),
            }],
            OsOp::RemoveAppInstance { instance_id } => projection
                .app_instances
                .iter()
                .find(|instance| instance.id == *instance_id)
                .map(|instance| {
                    let node = projection
                        .media_graph
                        .nodes
                        .iter()
                        .find(|entry| entry.instance_id == *instance_id);
                    vec![OsOp::SpawnAppInstance {
                        instance: instance.clone(),
                        position: MediaGraphPosition {
                            x: node.map(|entry| entry.x).unwrap_or(0.0),
                            y: node.map(|entry| entry.y).unwrap_or(0.0),
                        },
                    }]
                })
                .unwrap_or_default(),
            OsOp::ConnectMediaPorts { edge } => vec![OsOp::DisconnectMediaEdge {
                edge_id: edge.id.clone(),
            }],
            OsOp::DisconnectMediaEdge { edge_id } => projection
                .media_graph
                .edges
                .iter()
                .find(|edge| edge.id == *edge_id)
                .map(|edge| vec![OsOp::ConnectMediaPorts { edge: edge.clone() }])
                .unwrap_or_default(),
            OsOp::MoveMediaNode { node_id, .. } => projection
                .media_graph
                .nodes
                .iter()
                .find(|node| node.id == *node_id)
                .map(|node| {
                    vec![OsOp::MoveMediaNode {
                        node_id: node_id.clone(),
                        x: node.x,
                        y: node.y,
                    }]
                })
                .unwrap_or_default(),
            OsOp::PatchAppInstance { instance_id, .. } => projection
                .app_instances
                .iter()
                .find(|instance| instance.id == *instance_id)
                .map(|instance| {
                    vec![OsOp::PatchAppInstance {
                        instance_id: instance_id.clone(),
                        label: Some(instance.label.clone()),
                    }]
                })
                .unwrap_or_default(),
            OsOp::AddParameter { parameter } => vec![OsOp::RemoveParameter {
                parameter_id: parameter_entity_id(parameter).into(),
            }],
            OsOp::RemoveParameter { parameter_id } => projection
                .parameters
                .iter()
                .find(|parameter| parameter_entity_id(parameter) == *parameter_id)
                .map(|parameter| vec![OsOp::AddParameter {
                    parameter: parameter.clone(),
                }])
                .unwrap_or_default(),
            OsOp::PatchParameter {
                parameter_id,
                parameter,
            } => projection
                .parameters
                .iter()
                .find(|entry| parameter_entity_id(entry) == *parameter_id)
                .map(|current| {
                    vec![OsOp::PatchParameter {
                        parameter_id: parameter_id.clone(),
                        parameter: current.clone(),
                    }]
                })
                .unwrap_or_else(|| {
                    vec![OsOp::PatchParameter {
                        parameter_id: parameter_id.clone(),
                        parameter: parameter.clone(),
                    }]
                }),
            OsOp::BindParameterField { binding } => vec![OsOp::UnbindParameterField {
                instance_id: binding.instance_id.clone(),
                field_path: binding.field_path.clone(),
            }],
            OsOp::UnbindParameterField {
                instance_id,
                field_path,
            } => projection
                .parameter_bindings
                .iter()
                .find(|binding| binding.instance_id == *instance_id && binding.field_path == *field_path)
                .map(|binding| {
                    vec![OsOp::BindParameterField {
                        binding: binding.clone(),
                    }]
                })
                .unwrap_or_default(),
            OsOp::SyncParameterPorts => Vec::new(),
        }
    }
}

pub fn materialize_os_projection(
    document: &OsDocument,
    applied_edit_ids: &[String],
) -> Result<OsProjection, VcsError> {
    let envelope = OsEnvelope {
        schema: document.schema.clone(),
        id: document.id.clone(),
        vcs: document.vcs.clone(),
        backbone: document.backbone.clone(),
        active_alternative_id: document.vcs.initial_projection.active_alternative_id.clone(),
    };
    materialize_document_projection(&envelope, applied_edit_ids)
}

pub fn os_document_to_json(document: &OsDocument) -> Result<String, VcsError> {
    serde_json::to_string_pretty(document).map_err(|error| VcsError::Serialize(error.to_string()))
}

pub fn os_document_from_json(json: &str) -> Result<OsDocument, VcsError> {
    let document: OsDocument =
        serde_json::from_str(json).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    if document.schema != OS_STUDIO_SCHEMA {
        return Err(VcsError::Deserialize(format!(
            "expected schema {OS_STUDIO_SCHEMA}"
        )));
    }
    Ok(document)
}
//#endregion 🔖OsDocument

//#region 🔖OsStore
pub struct OsStore {
    inner: DocumentVcsStore<OsProjection, OsOp>,
    name: String,
}

impl OsStore {
    pub fn new(document: OsDocument) -> Self {
        let applied_edit_ids = document.applied_edit_ids.clone();
        let envelope = OsEnvelope {
            schema: document.schema,
            id: document.id,
            vcs: document.vcs,
            backbone: document.backbone,
            active_alternative_id: None,
        };
        let mut inner = DocumentVcsStore::new(envelope);
        if !applied_edit_ids.is_empty() {
            let snapshot = inner.envelope().clone();
            inner.set_envelope(snapshot, applied_edit_ids);
        }
        Self {
            inner,
            name: document.name,
        }
    }

    pub fn generation(&self) -> u64 {
        self.inner.generation()
    }

    pub fn projection(&self) -> Result<OsProjection, VcsError> {
        self.inner.projection()
    }

    pub fn document(&self) -> OsDocument {
        let envelope = self.inner.envelope();
        OsDocument {
            schema: envelope.schema.clone(),
            id: envelope.id.clone(),
            name: self.name.clone(),
            vcs: envelope.vcs.clone(),
            applied_edit_ids: self.inner.applied_edit_ids().to_vec(),
            backbone: envelope.backbone.clone(),
        }
    }

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        self.inner.dispatch_json(command_json)
    }

    pub fn dispatch_apply(&mut self, operations: Vec<OsOp>) -> Result<(), VcsError> {
        self.inner.dispatch(DocumentVcsCommand::Apply {
            operations,
            description: None,
        })
    }

    pub fn set_studio_name(&mut self, name: &str) {
        self.name = name.into();
        let _ = self.inner.generation();
    }

    pub fn spawn_app_instance(
        &mut self,
        program_id: &str,
        app_id: &str,
        label: Option<&str>,
        position: MediaGraphPosition,
    ) -> Result<String, VcsError> {
        let registration = os_app_registration(program_id, app_id)
            .ok_or_else(|| VcsError::Deserialize(format!("unknown app {program_id}/{app_id}")))?;
        let instance_id = create_os_id("app");
        // 🆔 Minted once, here, at dispatch time; the id is embedded in the stored `OsOp` itself so
        // replay is deterministic (it never re-mints) — same idempotency property `create_os_id`
        // already relies on for `instance_id`.
        let document_id = create_os_document_id();
        let instance = OsAppInstance {
            id: instance_id.clone(),
            program_id: program_id.into(),
            app_id: app_id.into(),
            label: label
                .map(str::to_string)
                .unwrap_or_else(|| registration.label.clone()),
            yields: os_app_primary_output_kind(&registration),
            document: OsDocumentRef {
                document_id,
                schema: registration.source_format.clone(),
            },
        };
        self.dispatch_apply(vec![OsOp::SpawnAppInstance {
            instance,
            position,
        }])?;
        Ok(instance_id)
    }

    pub fn add_parameter(
        &mut self,
        parameter_type: &OsParameterType,
        name: &str,
    ) -> Result<String, VcsError> {
        let parameter = create_default_os_parameter(parameter_type, name, None);
        let parameter_id_value = parameter_entity_id(&parameter).to_string();
        self.dispatch_apply(vec![OsOp::AddParameter { parameter }])?;
        Ok(parameter_id_value)
    }

    pub fn patch_parameter(&mut self, target_parameter_id: &str, patch: &OsParameterPatch) -> Result<(), VcsError> {
        let projection = self.projection()?;
        let current = projection
            .parameters
            .iter()
            .find(|parameter| parameter_entity_id(parameter) == target_parameter_id)
            .cloned()
            .ok_or_else(|| VcsError::Deserialize(format!("unknown parameter {target_parameter_id}")))?;
        let next = patch_os_parameter(&current, patch);
        self.dispatch_apply(vec![OsOp::PatchParameter {
            parameter_id: target_parameter_id.into(),
            parameter: next,
        }])
    }

    /// @emoji 📡 Pumps any queued inbound backbone messages into the edit timeline.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.inner.tick()
    }

    /// @emoji 🔗 Resolves and attaches a backbone by uri. Only available inside the wasm sandbox
    /// (every scheme forwards to the host over the injected `BackboneChannelPort`, a pure queue) —
    /// see {@link attach_backbone} for the native counterpart, which takes an explicit
    /// `Box<dyn vcs::Backbone>` since native has no URI→IO auto-resolution anymore (`framework/sync`'s
    /// `host_runtime` module owns constructing the real endpoint via `DocumentHost`).
    #[cfg(target_arch = "wasm32")]
    pub fn attach_backbone(&mut self, uri: &str) -> Result<(), VcsError> {
        self.inner.attach_backbone_uri(uri)
    }

    /// @emoji 🔗 Attaches an explicit native backbone channel (typically a `channel_backbone` handed
    /// out by `framework/sync`'s `DocumentHost::open`, per `host_runtime`'s canonical sequence).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn attach_backbone(&mut self, backbone: Box<dyn vcs::Backbone>) -> Result<(), VcsError> {
        self.inner.attach_backbone(backbone)
    }

    pub fn detach_backbone(&mut self) {
        self.inner.detach_backbone();
    }

    pub fn backbone_ref(&self) -> Option<&DocumentBackboneRef> {
        self.inner.backbone_ref()
    }
}
//#endregion 🔖OsStore

//#region 🔖Backbone
pub trait OsBackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<String, VcsError>;
    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
}

impl<T: vcs::BackbonePort> OsBackbonePort for T {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        vcs::BackbonePort::read(self, uri)
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        vcs::BackbonePort::write(self, uri, payload)
    }
}

fn sync_os_studio_document(
    document: &OsDocument,
    backbone_uri: &str,
    port: &Arc<dyn OsBackbonePort>,
) -> Result<(), VcsError> {
    let mut synced = document.clone();
    synced.backbone = Some(document_backbone_ref(backbone_uri));
    port.write(backbone_uri, &os_document_to_json(&synced)?)
}
//#endregion 🔖Backbone

// 🫀 Presence used to be a `presence:` backbone-URI polling hack (`OS_PRESENCE_URI_PREFIX` /
// `write_os_presence` / `read_os_presence_peers`) — deleted. Presence now flows through the hub's
// duplex `PresencePeer`/`HubServerFrame::Presence` frames (`framework/core/rs`'s 🔖HubProtocol
// region) via `framework/sync`'s `DocumentHost::subscribe` yielding `DocumentEvent::Presence`; the
// `host_runtime` module below is where a native host translates that event into
// `ViewState.presence_peers_json` — the plugin read-side contract is unchanged.

//#region 🔖StudioCatalog
pub const OS_HOME_VFS_ROOT_ID: &str = "os-home-root";
pub const OS_STUDIO_BACKBONE_URI_PREFIX: &str = "studio://";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsStudioCatalogEntry {
    pub id: String,
    pub name: String,
    pub backbone_uri: String,
    pub app_count: usize,
    pub node_count: usize,
    pub updated_at: String,
}

static STUDIO_CATALOG_URIS: LazyLock<Mutex<HashMap<usize, HashSet<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn port_key(port: &Arc<dyn OsBackbonePort>) -> usize {
    Arc::as_ptr(port) as *const () as usize
}

fn track_os_studio_backbone_uri(port: &Arc<dyn OsBackbonePort>, uri: &str) {
    STUDIO_CATALOG_URIS
        .lock()
        .expect("lock")
        .entry(port_key(port))
        .or_default()
        .insert(uri.into());
}

fn untrack_os_studio_backbone_uri(port: &Arc<dyn OsBackbonePort>, uri: &str) {
    if let Some(uris) = STUDIO_CATALOG_URIS.lock().expect("lock").get_mut(&port_key(port)) {
        uris.remove(uri);
    }
}

fn os_studio_backbone_uri(studio_id: &str) -> String {
    format!("{OS_STUDIO_BACKBONE_URI_PREFIX}{studio_id}")
}

fn os_studio_id_from_backbone_uri(uri: &str) -> Option<String> {
    uri.strip_prefix(OS_STUDIO_BACKBONE_URI_PREFIX)
        .filter(|id| !id.is_empty())
        .map(str::to_string)
}

fn os_studio_catalog_entry_from_document(
    backbone_uri: &str,
    document: &OsDocument,
) -> Result<OsStudioCatalogEntry, VcsError> {
    let studio_id = os_studio_id_from_backbone_uri(backbone_uri)
        .unwrap_or_else(|| document.id.clone());
    let projection = materialize_os_projection(document, &[])?;
    let updated_at = document
        .vcs
        .changes
        .last()
        .map(|change| change.saved_at.clone())
        .unwrap_or_else(|| "0".into());
    Ok(OsStudioCatalogEntry {
        id: studio_id,
        name: document.name.clone(),
        backbone_uri: backbone_uri.into(),
        app_count: projection.app_instances.len(),
        node_count: projection.media_graph.nodes.len(),
        updated_at,
    })
}

/// @emoji 📚 Lists persisted studio documents from the dev backbone namespace.
pub fn list_os_studio_catalog_entries(
    port: Arc<dyn OsBackbonePort>,
) -> Result<Vec<OsStudioCatalogEntry>, VcsError> {
    let mut entries = Vec::new();
    let uris: Vec<String> = STUDIO_CATALOG_URIS
        .lock()
        .expect("lock")
        .get(&port_key(&port))
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect();
    for uri in uris {
        let json = port.read(&uri)?;
        if json.is_empty() {
            continue;
        }
        let document = os_document_from_json(&json)?;
        entries.push(os_studio_catalog_entry_from_document(&uri, &document)?);
    }
    entries.sort_by(|left, right| {
        right
            .updated_at
            .cmp(&left.updated_at)
            .then_with(|| left.name.cmp(&right.name))
    });
    Ok(entries)
}

/// @emoji 🆕 Creates an empty studio document on the dev backbone.
pub fn create_os_studio(
    name: &str,
    port: Arc<dyn OsBackbonePort>,
) -> Result<OsStudioCatalogEntry, VcsError> {
    let id = create_os_id("studio");
    let document = create_empty_os_document(&id, name.trim());
    let backbone_uri = os_studio_backbone_uri(&id);
    sync_os_studio_document(&document, &backbone_uri, &port)?;
    track_os_studio_backbone_uri(&port, &backbone_uri);
    os_studio_catalog_entry_from_document(&backbone_uri, &document)
}

/// @emoji 🗑️ Deletes a studio document from the dev backbone.
pub fn delete_os_studio(studio_id: &str, port: Arc<dyn OsBackbonePort>) -> Result<(), VcsError> {
    let uri = os_studio_backbone_uri(studio_id);
    untrack_os_studio_backbone_uri(&port, &uri);
    port.write(&uri, "")
}

/// @emoji 📥 Imports a studio document JSON onto the dev backbone.
pub fn import_os_studio_from_json(
    json: &str,
    port: Arc<dyn OsBackbonePort>,
) -> Result<OsStudioCatalogEntry, VcsError> {
    let mut document = os_document_from_json(json)?;
    let studio_id = if document.id.is_empty() {
        create_os_id("studio")
    } else {
        document.id.clone()
    };
    let backbone_uri = os_studio_backbone_uri(&studio_id);
    document.id = studio_id;
    sync_os_studio_document(&document, &backbone_uri, &port)?;
    track_os_studio_backbone_uri(&port, &backbone_uri);
    os_studio_catalog_entry_from_document(&backbone_uri, &document)
}

/// @emoji 📂 Loads a studio document from the dev backbone.
pub fn load_os_studio_document(
    studio_id: &str,
    port: Arc<dyn OsBackbonePort>,
) -> Result<OsDocument, VcsError> {
    let backbone_uri = os_studio_backbone_uri(studio_id);
    let json = port.read(&backbone_uri)?;
    if json.is_empty() {
        return Err(VcsError::Backbone(format!("unknown os studio: {studio_id}")));
    }
    os_document_from_json(&json)
}

/// @emoji 🌱 Seeds the demo studio when the catalog is empty.
pub fn seed_os_studio_catalog_if_empty(
    seed_document: OsDocument,
    port: Arc<dyn OsBackbonePort>,
) -> Result<Option<OsStudioCatalogEntry>, VcsError> {
    if !list_os_studio_catalog_entries(port.clone())?.is_empty() {
        return Ok(None);
    }
    let studio_id = if seed_document.id.is_empty() {
        "default".into()
    } else {
        seed_document.id.clone()
    };
    let backbone_uri = os_studio_backbone_uri(&studio_id);
    let mut seeded = seed_document;
    seeded.id = studio_id;
    sync_os_studio_document(&seeded, &backbone_uri, &port)?;
    track_os_studio_backbone_uri(&port, &backbone_uri);
    Ok(Some(os_studio_catalog_entry_from_document(
        &backbone_uri,
        &seeded,
    )?))
}
//#endregion 🔖StudioCatalog

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{merge_os_program_definition, os_baseline_resource, OsPlatformAppInput, OsPlatformInput};
    use crate::media_graph::{empty_media_graph, validate_media_graph};
    use semio_framework_core::{
        ActorId, AppInstanceId, ActionId, InvocationId, ModeDefinition, PluginManifest,
        WindowKindDefinition,
    };
    use ui_wgpu::SurfaceKind;
    use std::sync::Arc;
    use vcs::MemoryBackbonePort;

    #[test]
    fn loads_plugin_apps_into_registry() {
        let mut host = PluginHost::new();
        let manifest = PluginManifest {
            plugin_id: "draw".into(),
            label: "Draw".into(),
            version: "0.1.0".into(),
            apps: vec![AppDefinition {
                id: "draw-play".into(),
                label: "Draw".into(),
                document: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: semio_framework_core::Modes::one(ModeDefinition {
                    id: "edit".into(),
                    label: "Edit".into(),
                    tools: Vec::new(),
                    layout_id: None,
                    commands: Vec::new(),
                }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: "Canvas".into(),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: None,
                    options: ui_wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    tools: Vec::new(),
                    params_schema: None,
                    document_projection_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                actions: vec![],
                tools: Vec::new(),
                commands: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                introduction: None,
                dialogs: Vec::new(),
            }],
            programs: vec![],
            capabilities: vec![],
            contributions: vec![],
            examples: vec![],
            commands: vec![],
        };
        host.load_plugin(LoadedPlugin {
            plugin_id: "draw".into(),
            manifest,
            artifact_uri: "plugin://draw".into(),
        });
        assert_eq!(host.apps().len(), 1);
    }

    #[test]
    fn hot_swap_bumps_instance_generation_and_tracks_app_changes() {
        let mut host = PluginHost::new();
        let draw_app = AppDefinition {
            id: "draw-play".into(),
            label: "Draw".into(),
            document: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: semio_framework_core::Modes::one(ModeDefinition {
                id: "edit".into(),
                label: "Edit".into(),
                tools: Vec::new(),
                layout_id: None,
                commands: Vec::new(),
            }),
            default_mode_id: "edit".into(),
            window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                id: "composite".into(),
                label: "Canvas".into(),
                body_key: "composite".into(),
                surface_kind: SurfaceKind::Canvas2d,
                icon_id: None,
                options: ui_wgpu::WindowOptions::default(),
                actions: Vec::new(),
                tools: Vec::new(),
                params_schema: None,
                document_projection_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: vec![],
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            actions: vec![],
            tools: Vec::new(),
            commands: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            introduction: None,
            dialogs: Vec::new(),
        };
        let note_app = AppDefinition {
            id: "note-play".into(),
            label: "Note".into(),
            document: vec!["semio".into(), "note".into()],
            icon_id: None,
            controller_id: "note-play".into(),
            modes: semio_framework_core::Modes::one(ModeDefinition {
                id: "edit".into(),
                label: "Edit".into(),
                tools: Vec::new(),
                layout_id: None,
                commands: Vec::new(),
            }),
            default_mode_id: "edit".into(),
            window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                id: "composite".into(),
                label: "Canvas".into(),
                body_key: "composite".into(),
                surface_kind: SurfaceKind::Canvas2d,
                icon_id: None,
                options: ui_wgpu::WindowOptions::default(),
                actions: Vec::new(),
                tools: Vec::new(),
                params_schema: None,
                document_projection_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: vec![],
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            actions: vec![],
            tools: Vec::new(),
            commands: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            introduction: None,
            dialogs: Vec::new(),
        };
        host.load_plugin(LoadedPlugin {
            plugin_id: "draw".into(),
            manifest: PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "0.1.0".into(),
                apps: vec![draw_app.clone()],
                programs: vec![],
                capabilities: vec![],
                contributions: vec![],
                examples: vec![],
                commands: vec![],
            },
            artifact_uri: "plugin://draw".into(),
        });
        let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
        let generation_before = host.instance(instance_id).expect("instance").generation;
        let event = host.hot_swap_plugin(LoadedPlugin {
            plugin_id: "draw".into(),
            manifest: PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "0.2.0".into(),
                apps: vec![draw_app, note_app],
                programs: vec![],
                capabilities: vec![],
                contributions: vec![],
                examples: vec![],
                commands: vec![],
            },
            artifact_uri: "plugin://draw".into(),
        });
        assert_eq!(event.added_apps, vec!["note-play".to_string()]);
        assert!(event.removed_apps.is_empty());
        assert_eq!(event.plugin_id, "draw");
        assert_eq!(event.version, "0.2.0");
        assert!(
            host.instance(instance_id).expect("instance").generation > generation_before,
            "hot swap must bump instance generation"
        );
        assert_eq!(host.apps().len(), 2);
    }

    #[test]
    fn hot_swap_rollback_on_invalid_manifest_keeps_old_plugin() {
        let mut host = PluginHost::new();
        let draw_app = AppDefinition {
            id: "draw-play".into(),
            label: "Draw".into(),
            document: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: semio_framework_core::Modes::one(ModeDefinition {
                id: "edit".into(),
                label: "Edit".into(),
                tools: Vec::new(),
                layout_id: None,
                commands: Vec::new(),
            }),
            default_mode_id: "edit".into(),
            window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                id: "composite".into(),
                label: "Canvas".into(),
                body_key: "composite".into(),
                surface_kind: SurfaceKind::Canvas2d,
                icon_id: None,
                options: ui_wgpu::WindowOptions::default(),
                actions: Vec::new(),
                tools: Vec::new(),
                params_schema: None,
                document_projection_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: vec![],
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            actions: vec![],
            tools: Vec::new(),
            commands: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            introduction: None,
            dialogs: Vec::new(),
        };
        host.load_plugin(LoadedPlugin {
            plugin_id: "draw".into(),
            manifest: PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "0.1.0".into(),
                apps: vec![draw_app],
                programs: vec![],
                capabilities: vec![],
                contributions: vec![],
                examples: vec![],
                commands: vec![],
            },
            artifact_uri: "plugin://draw".into(),
        });
        let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
        let generation_before = host.instance(instance_id).expect("instance").generation;
        let event = host.hot_swap_plugin(LoadedPlugin {
            plugin_id: "draw".into(),
            manifest: PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "".into(),
                apps: vec![],
                programs: vec![],
                capabilities: vec![],
                contributions: vec![],
                examples: vec![],
                commands: vec![],
            },
            artifact_uri: "plugin://draw".into(),
        });
        assert_eq!(event.plugin_id, "draw");
        assert_eq!(event.version, "0.1.0");
        assert!(event.added_apps.is_empty());
        assert_eq!(host.apps().len(), 1);
        assert_eq!(
            host.instance(instance_id).expect("instance").generation,
            generation_before
        );
        assert_eq!(
            host.plugins.get("draw").expect("plugin").manifest.version,
            "0.1.0"
        );
    }

    #[test]
    fn contributions_track_plugin_load_and_hot_swap() {
        let mut host = PluginHost::new();
        let contribution = Contribution::ProtocolBlockKind {
            app_id: "protocol-module-procedural".into(),
            block_kind: "buildingComponent".into(),
            label: "Building Component".into(),
            icon_id: "building".into(),
            default_value_json: "{}".into(),
            params_body_key: "params".into(),
            preview_body_key: "preview".into(),
        };
        host.load_plugin(LoadedPlugin {
            plugin_id: "protocol-module-procedural".into(),
            manifest: PluginManifest {
                plugin_id: "protocol-module-procedural".into(),
                label: "Protocol Module Procedural".into(),
                version: "0.1.0".into(),
                apps: vec![],
                programs: vec![],
                capabilities: vec![],
                contributions: vec![contribution.clone()],
                examples: vec![],
                commands: vec![],
            },
            artifact_uri: "plugin://protocol-module-procedural".into(),
        });
        assert_eq!(host.contributions().len(), 1);
        assert_eq!(host.contributions()[0].plugin_id, "protocol-module-procedural");
        host.hot_swap_plugin(LoadedPlugin {
            plugin_id: "protocol-module-procedural".into(),
            manifest: PluginManifest {
                plugin_id: "protocol-module-procedural".into(),
                label: "Protocol Module Procedural".into(),
                version: "0.2.0".into(),
                apps: vec![],
                programs: vec![],
                capabilities: vec![],
                contributions: vec![],
                examples: vec![],
                commands: vec![],
            },
            artifact_uri: "plugin://protocol-module-procedural".into(),
        });
        assert!(host.contributions().is_empty());
    }

    #[test]
    fn invoke_action_applies_patch_ops_and_returns_kernel_operations() {
        let mut host = PluginHost::new();
        let manifest = PluginManifest {
            plugin_id: "draw".into(),
            label: "Draw".into(),
            version: "0.1.0".into(),
            apps: vec![AppDefinition {
                id: "draw-play".into(),
                label: "Draw".into(),
                document: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: semio_framework_core::Modes::one(ModeDefinition {
                    id: "edit".into(),
                    label: "Edit".into(),
                    tools: Vec::new(),
                    layout_id: None,
                    commands: Vec::new(),
                }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: "Canvas".into(),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: None,
                    options: ui_wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    tools: Vec::new(),
                    params_schema: None,
                    document_projection_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                actions: vec![],
                tools: Vec::new(),
                commands: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                introduction: None,
                dialogs: Vec::new(),
            }],
            programs: vec![],
            capabilities: vec![],
            contributions: vec![],
            examples: vec![],
            commands: vec![],
        };
        host.load_plugin(LoadedPlugin {
            plugin_id: "draw".into(),
            manifest,
            artifact_uri: "plugin://draw".into(),
        });
        let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
        let patch_op = serde_json::json!({
            "op": "patch",
            "patch": { "title": "Hello" }
        })
        .to_string();
        let result = host
            .invoke_action(ActionInvocation {
                id: InvocationId("invoke-1".into()),
                app: AppInstanceId(instance_id.to_string()),
                action: ActionId("setTitle".into()),
                input: serde_json::json!({ "ops": [patch_op] }),
                actor: ActorId("tester".into()),
                causal_context: vec![],
            })
            .expect("invoke");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(result.inverse_group.operations.len(), 1);
        let document: serde_json::Value =
            serde_json::from_str(&host.instance(instance_id).expect("instance").document_json)
                .expect("document json");
        assert_eq!(document.get("title").and_then(|value| value.as_str()), Some("Hello"));
    }

    #[test]
    fn recovery_ui_renders_actions_for_quarantined_plugin() {
        let mut host = PluginHost::new();
        host.supervisor
            .insert("draw".into(), PluginSupervisorState::Quarantined);
        let ui = host.recovery_ui("draw");
        match ui {
            UiNode::Stack(stack) => assert_eq!(stack.children.len(), 4),
            other => panic!("expected recovery stack, got {other:?}"),
        }
    }

    fn seed_draw_program() {
        let mut resources = HashMap::new();
        resources.insert(
            "draw".into(),
            os_baseline_resource("2d.drawing", "draw.document", "draw"),
        );
        merge_os_program_definition(
            "draw",
            &OsPlatformInput {
                id: "draw".into(),
                name: "Draw".into(),
                api_version: "1".into(),
                apps: vec![OsPlatformAppInput {
                    id: "draw".into(),
                    label: "Draw".into(),
                    document: vec!["semio".into(), "draw".into()],
                    controller_id: "draw-play".into(),
                    modes: vec![ModeDefinition {
                        id: "edit".into(),
                        label: "Edit".into(),
                        tools: Vec::new(),
                        layout_id: None,
                        commands: Vec::new(),
                    }],
                    default_mode_id: None,
                }],
            },
            &resources,
        )
        .expect("merge");
    }

    #[test]
    fn spawns_and_removes_app_instances() {
        seed_draw_program();
        let mut store = OsStore::new(create_empty_os_document("studio", "Studio"));
        store
            .spawn_app_instance(
                "draw",
                "draw",
                None,
                MediaGraphPosition { x: 40.0, y: 40.0 },
            )
            .expect("spawn");
        assert_eq!(store.projection().expect("projection").app_instances.len(), 1);
        store
            .dispatch_json(r#"{"kind":"undo"}"#)
            .expect("undo");
        assert_eq!(store.projection().expect("projection").app_instances.len(), 0);
    }

    #[test]
    fn adds_and_patches_studio_parameters() {
        let mut store = OsStore::new(create_empty_os_document("studio", "Studio"));
        let parameter_id = store
            .add_parameter(&OsParameterType::Numeric, "Zoom")
            .expect("add");
        store
            .patch_parameter(&parameter_id, &serde_json::json!({ "value": 12.0, "max": 10.0 }))
            .expect("patch");
        match &store.projection().expect("projection").parameters[0] {
            OsParameter::Numeric { value, .. } => assert_eq!(*value, 10.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn creates_and_lists_studio_catalog_entries() {
        let port = Arc::new(MemoryBackbonePort::new());
        let entry = create_os_studio("Catalog Studio", port.clone()).expect("create");
        let listed = list_os_studio_catalog_entries(port.clone()).expect("list");
        assert!(listed.iter().any(|row| row.id == entry.id));
        delete_os_studio(&entry.id, port.clone()).expect("delete");
        assert!(!list_os_studio_catalog_entries(port)
            .expect("list")
            .iter()
            .any(|row| row.id == entry.id));
    }

    #[test]
    fn validates_media_graph_cycles() {
        assert!(validate_media_graph(&empty_media_graph()).ok);
    }

    // 🫀 The old `presence_upserts_prunes_and_excludes_self` test exercised the deleted `presence:`
    // backbone-URI hack (`write_os_presence`/`read_os_presence_peers`). Presence now flows through
    // the hub's `PresencePeer`/`HubServerFrame::Presence` frames and `framework/sync`'s
    // `DocumentEvent::Presence` — see `framework/product/os/hub/rs/bin.rs` and
    // `framework/sync/rs/lib.rs` for that layer's own coverage.
}
// #endregion host
}

pub mod backbone {
// #region backbone
//! 🗄️ Trusted host-side backbone ports for local studio storage — reads/writes the raw persisted
//! json directly, bypassing the duplex `Backbone` channel since there is no other process here.

use crate::host::OsBackbonePort;
use crate::media_graph::OS_STUDIO_SCHEMA;
use std::sync::Arc;
use vcs::{MemoryBackbonePort, VcsError};

/// @emoji 🗂️ Conventional single-document id used inside a folder-backed studio backbone — a studio
/// folder holds exactly one os document at its root (app documents get their own document ids once
/// {@link OsDocumentRef} routes them through `framework/sync`'s multi-document `DocumentHost`).
const STUDIO_FOLDER_DOCUMENT_ID: &str = "studio";

enum StudioPortKind {
    #[cfg(not(target_arch = "wasm32"))]
    File(String, vcs::FileJsonStorage),
    #[cfg(not(target_arch = "wasm32"))]
    Folder(String, vcs::FolderSqliteStorage),
}

pub struct StudioBackbonePort {
    kind: Option<StudioPortKind>,
    memory: MemoryBackbonePort,
}

impl StudioBackbonePort {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn file(file_path: &str) -> Result<Self, VcsError> {
        let uri = format!("file://{file_path}");
        Ok(Self {
            kind: Some(StudioPortKind::File(
                uri,
                vcs::FileJsonStorage::new(std::path::PathBuf::from(file_path)),
            )),
            memory: MemoryBackbonePort::new(),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn folder(folder_path: &str) -> Result<Self, VcsError> {
        let uri = format!("folder://{folder_path}");
        Ok(Self {
            kind: Some(StudioPortKind::Folder(
                uri,
                vcs::FolderSqliteStorage::new(std::path::PathBuf::from(folder_path)),
            )),
            memory: MemoryBackbonePort::new(),
        })
    }
}

impl OsBackbonePort for StudioBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        if let Some(kind) = &self.kind {
            match kind {
                #[cfg(not(target_arch = "wasm32"))]
                StudioPortKind::File(file_uri, storage) if uri == file_uri => {
                    return storage
                        .read()?
                        .ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")));
                }
                #[cfg(not(target_arch = "wasm32"))]
                StudioPortKind::Folder(folder_uri, storage) if uri == folder_uri => {
                    return storage
                        .read(STUDIO_FOLDER_DOCUMENT_ID)?
                        .ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")));
                }
                _ => {}
            }
        }
        self.memory.read(uri)
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        if let Some(kind) = &self.kind {
            match kind {
                #[cfg(not(target_arch = "wasm32"))]
                StudioPortKind::File(file_uri, storage) if uri == file_uri => {
                    return storage.write(payload);
                }
                #[cfg(not(target_arch = "wasm32"))]
                StudioPortKind::Folder(folder_uri, storage) if uri == folder_uri => {
                    return storage.write(STUDIO_FOLDER_DOCUMENT_ID, OS_STUDIO_SCHEMA, payload);
                }
                _ => {}
            }
        }
        self.memory.write(uri, payload)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_folder_studio_backbone(folder_path: &str) -> Result<Arc<dyn OsBackbonePort>, VcsError> {
    Ok(Arc::new(StudioBackbonePort::folder(folder_path)?))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_file_studio_backbone(file_path: &str) -> Result<Arc<dyn OsBackbonePort>, VcsError> {
    Ok(Arc::new(StudioBackbonePort::file(file_path)?))
}
// #endregion backbone
}

/// @emoji 🧵 Canonical native document-open sequencing shared by every native host that links this
/// crate (currently the wgpu shell). Native-only: it depends on `framework/sync`'s `DocumentHost`,
/// whose actor is a native-thread (or wasm `spawn_local`) concern — WASI-P2 plugins never see it, and
/// the browser React shell talks to its own TS twin (`framework/product/os/core/js/backbone-worker.ts`)
/// through a different FFI boundary (the WIT plugin sandbox), not through this Rust module. Keeping
/// this doc-comment as the single canonical description of the sequence — referenced from both
/// `os-shell.tsx`'s `openDocument` and `framework/renderer/wgpu/rs/lib.rs` — is how the two stay in
/// lockstep without a literal shared code path across the Rust/TS boundary.
#[cfg(not(target_arch = "wasm32"))]
pub mod host_runtime {
// #region host_runtime
//! 🧵 Native `DocumentHost` sequencing shared by every native caller (wgpu shell today).
//!
//! ## Canonical open/spawn/effect sequence (mirrored in TS by `os-shell.tsx`'s `openDocument`):
//! 1. Build a `DocumentActorConfig{document_id, schema, bindings, watch_external, actor}` for the
//!    document being opened — either the os/studio document itself, or one app instance's
//!    {@link crate::instance::OsDocumentRef}.
//! 2. `DocumentHost::open(config)` → `DocumentChannels{cmd_tx, channel_backbone}`.
//! 3. Attach `channel_backbone` to the document's own store: `store.attach_backbone(Box::new(...))`.
//!    For a native WASM plugin instance this ALSO means calling `framework/plugin/host`'s
//!    `WasmPluginRuntime::register_host_backbone(uri, Box::new(channel_backbone))` so the sandboxed
//!    plugin's `backbone-send`/`backbone-poll` host imports reach the same channel — this crate does
//!    not link `framework/plugin/host` directly (no existing dependency edge), so the wgpu shell,
//!    which links both, is the one that actually performs that registration call using the
//!    {@link OpenedDocument} this module hands back.
//! 4. `DocumentHost::subscribe(&document_id)` → `broadcast::Receiver<DocumentEvent>`; on each event:
//!    - `RemoteOps`/`SnapshotReplaced` are already pushed into the store's inbound queue by the actor
//!      — the caller just needs to call `store.tick()` (step 5) to materialize them.
//!    - `Presence{peers}` translates into `ViewState.presence_peers_json` via
//!      {@link presence_peers_json} — the ONLY place presence now flows through; the old `presence:`
//!      backbone-URI hack is gone entirely.
//!    - `Status`/`Conflict` surface on the shell's sync-status badge / conflict card.
//! 5. Every tick/frame: `store.tick()` drains the attached backbone's inbound queue into the store.
//! 6. On `HostEffect::SpawnPluginInstance`/`OpenPluginInstance` from an action result: mint (if
//!    needed) a fresh `OsDocumentRef` (see {@link crate::instance::create_os_document_id}), then repeat
//!    steps 1-5 for that app's own document.
//! 7. On close: send `DocumentActorMsg::Detach` (flushes pending ops) via `host.send(id, Detach)`, then
//!    `DocumentHost::close(&id)`, then `store.detach_backbone()` /
//!    `WasmPluginRuntime::deregister_host_backbone(uri)`.

use crate::instance::OsDocumentRef;
use semio_framework_sync::{
    DocumentActorConfig, DocumentActorMsg, DocumentChannels, DocumentEvent, DocumentHost, PersistenceBinding,
};

/// @emoji 📌 The local persistence binding for a folder-backed document (one row per `document_id`
/// in the folder's `.semio` sqlite store — see `vcs::FolderSqliteStorage`).
pub fn folder_binding(folder_path: std::path::PathBuf) -> PersistenceBinding {
    PersistenceBinding::Folder { path: folder_path }
}

/// @emoji ☁️ The hub persistence binding for a document.
pub fn hub_binding(base_url: impl Into<String>, token: Option<String>) -> PersistenceBinding {
    PersistenceBinding::Hub { base_url: base_url.into(), token }
}

/// @emoji 🔗 Builds the `DocumentActorConfig` to open an app instance's own document, from its
/// `OsDocumentRef` — step 1 of the canonical sequence.
pub fn app_document_config(
    document: &OsDocumentRef,
    bindings: Vec<PersistenceBinding>,
    actor: &str,
) -> DocumentActorConfig {
    DocumentActorConfig {
        document_id: document.document_id.clone(),
        schema: document.schema.clone(),
        bindings,
        watch_external: true,
        actor: actor.to_string(),
    }
}

/// @emoji 🧵 Channels + a fresh event receiver for one opened document — steps 2 and 4 of the
/// canonical sequence.
pub struct OpenedDocument {
    pub channels: DocumentChannels,
    pub events: tokio::sync::broadcast::Receiver<DocumentEvent>,
}

/// @emoji 🚀 Opens a document on `host` and subscribes to its events in one call (steps 1-2 & 4).
pub fn open_document(
    host: &DocumentHost,
    document_id: &str,
    schema: &str,
    bindings: Vec<PersistenceBinding>,
    actor: &str,
) -> OpenedDocument {
    let channels = host.open(DocumentActorConfig {
        document_id: document_id.to_string(),
        schema: schema.to_string(),
        bindings,
        watch_external: true,
        actor: actor.to_string(),
    });
    let events = host.subscribe(document_id);
    OpenedDocument { channels, events }
}

/// @emoji ✂️ Detaches and closes a document's actor (step 7's `DocumentHost` half).
pub fn close_document(host: &DocumentHost, document_id: &str) {
    host.send(document_id, DocumentActorMsg::Detach);
    host.close(document_id);
}

/// @emoji 👥 Translates a `DocumentEvent::Presence` into the `ViewState.presence_peers_json` contract
/// plugins already read (`semio_framework_core::PresencePeer` → JSON array) — the new (only) source
/// of presence data; the deleted `presence:` backbone hack used to be it.
pub fn presence_peers_json(event: &DocumentEvent) -> Option<String> {
    match event {
        DocumentEvent::Presence { peers } => serde_json::to_string(peers).ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opens_a_document_and_subscribes_to_its_events() {
        let host = DocumentHost::new();
        let opened = open_document(&host, "doc-1", "test.schema", vec![], "actor-1");
        drop(opened.events);
        close_document(&host, "doc-1");
    }

    #[test]
    fn app_document_config_carries_the_document_ref_through() {
        let document = OsDocumentRef { document_id: "doc-2".into(), schema: "draw.document".into() };
        let config = app_document_config(&document, vec![], "actor-1");
        assert_eq!(config.document_id, "doc-2");
        assert_eq!(config.schema, "draw.document");
    }

    #[test]
    fn presence_peers_json_only_matches_presence_events() {
        use semio_framework_core::PresencePeer;
        let peers = vec![PresencePeer {
            actor: "a".into(),
            label: Some("Ada".into()),
            selection_json: None,
            connected_at_ms: 0,
            user_id: None,
            role: None,
        }];
        let json = presence_peers_json(&DocumentEvent::Presence { peers: peers.clone() }).expect("json");
        assert!(json.contains("\"actor\":\"a\""));
        assert!(presence_peers_json(&DocumentEvent::Status(Default::default())).is_none());
    }
}
// #endregion host_runtime
}

pub mod instance {
// #region instance
//! 📦 App instance schemas, parameters, and studio bindings.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

pub const OS_PARAMETER_PORT_PREFIX: &str = "param.";

//#region 🔖Schemas
/// @emoji 🔗 Handle to an app's own `framework/sync`-hosted vcs document — the os document never
/// embeds app content, only this reference (mirrors `framework/sync`'s `DocumentActorConfig`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsDocumentRef {
    pub document_id: String,
    pub schema: String,
}

/// @emoji 🆔 Mints a fresh app document id — uuid-v7 (time-ordered), matching the id shape hub already
/// uses for its own entities (`framework/product/os/hub/rs/bin.rs`'s `Uuid::now_v7()`).
pub fn create_os_document_id() -> String {
    uuid::Uuid::now_v7().to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsAppInstance {
    pub id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub yields: String,
    pub document: OsDocumentRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsInstanceState {
    pub id: u32,
    pub app_id: String,
    pub controller_id: String,
    pub document_json: String,
    pub view_state: semio_framework_core::ViewState,
    pub generation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsParameterType {
    Numeric,
    Categorical,
    Toggle,
    Text,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsParameterFieldSpec {
    pub field_path: String,
    pub label: String,
    #[serde(rename = "type")]
    pub parameter_type: OsParameterType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsParameterFieldBinding {
    pub parameter_id: String,
    pub instance_id: String,
    pub field_path: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum OsParameter {
    Numeric {
        id: String,
        name: String,
        value: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
    },
    Categorical {
        id: String,
        name: String,
        value: String,
        options: Vec<String>,
    },
    Toggle {
        id: String,
        name: String,
        value: bool,
    },
    Text {
        id: String,
        name: String,
        value: String,
    },
}
//#endregion 🔖Schemas

//#region 🔖Parameters
static OS_ID: AtomicU64 = AtomicU64::new(0);

/// @emoji 🆔 Allocates stable ids for OS studio entities.
pub fn create_os_id(prefix: &str) -> String {
    let n = OS_ID.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

/// @emoji 🎛️ Reads the runtime value from a studio parameter definition.
pub fn os_parameter_value(parameter: &OsParameter) -> Value {
    match parameter {
        OsParameter::Numeric { value, .. } => Value::from(*value),
        OsParameter::Categorical { value, .. } => Value::from(value.clone()),
        OsParameter::Toggle { value, .. } => Value::from(*value),
        OsParameter::Text { value, .. } => Value::from(value.clone()),
    }
}

/// @emoji 🎛️ Returns whether a parameter type can drive a bindable field type.
pub fn os_parameter_types_compatible(left: &OsParameterType, right: &OsParameterType) -> bool {
    left == right
}

/// @emoji 🎛️ Creates a default studio parameter of the given type.
pub fn create_default_os_parameter(
    parameter_type: &OsParameterType,
    name: &str,
    id: Option<&str>,
) -> OsParameter {
    let parameter_id = id
        .map(str::to_string)
        .unwrap_or_else(|| create_os_id("param"));
    match parameter_type {
        OsParameterType::Numeric => OsParameter::Numeric {
            id: parameter_id,
            name: name.into(),
            value: 0.0,
            min: Some(0.0),
            max: Some(100.0),
            step: Some(1.0),
        },
        OsParameterType::Categorical => OsParameter::Categorical {
            id: parameter_id,
            name: name.into(),
            value: "Option A".into(),
            options: vec!["Option A".into(), "Option B".into()],
        },
        OsParameterType::Toggle => OsParameter::Toggle {
            id: parameter_id,
            name: name.into(),
            value: false,
        },
        OsParameterType::Text => OsParameter::Text {
            id: parameter_id,
            name: name.into(),
            value: String::new(),
        },
    }
}

fn clamp_numeric_value(value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> f64 {
    let mut next = value;
    if let Some(min) = min.filter(|v| v.is_finite()) {
        next = next.max(min);
    }
    if let Some(max) = max.filter(|v| v.is_finite()) {
        next = next.min(max);
    }
    if let Some(step) = step.filter(|v| v.is_finite() && *v > 0.0) {
        let anchor = min.filter(|v| v.is_finite()).unwrap_or(0.0);
        next = anchor + ((next - anchor) / step).round() * step;
        if let Some(min) = min.filter(|v| v.is_finite()) {
            next = next.max(min);
        }
        if let Some(max) = max.filter(|v| v.is_finite()) {
            next = next.min(max);
        }
    }
    next
}

/// @emoji 🎛️ Applies a partial patch to a studio parameter, enforcing type constraints.
pub fn patch_os_parameter(parameter: &OsParameter, patch: &Value) -> OsParameter {
    let name = patch
        .get("name")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| parameter_name(parameter));
    let patch_type = patch.get("type").and_then(|v| v.as_str());
    let use_numeric = patch_type == Some("numeric")
        || (patch_type.is_none() && matches!(parameter, OsParameter::Numeric { .. }));
    if use_numeric {
        let current = match parameter {
            OsParameter::Numeric { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Numeric, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Numeric {
            id,
            min: current_min,
            max: current_max,
            step: current_step,
            value: current_value,
            ..
        } = current
        {
            let min = patch
                .get("min")
                .and_then(|v| v.as_f64())
                .or(current_min);
            let max = patch
                .get("max")
                .and_then(|v| v.as_f64())
                .or(current_max);
            let step = patch
                .get("step")
                .and_then(|v| v.as_f64())
                .or(current_step);
            let raw_value = patch
                .get("value")
                .and_then(|v| v.as_f64())
                .unwrap_or(current_value);
            return OsParameter::Numeric {
                id,
                name,
                min,
                max,
                step,
                value: clamp_numeric_value(raw_value, min, max, step),
            };
        }
    }
    let use_categorical = patch_type == Some("categorical")
        || (patch_type.is_none() && matches!(parameter, OsParameter::Categorical { .. }));
    if use_categorical {
        let current = match parameter {
            OsParameter::Categorical { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Categorical, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Categorical {
            id,
            value: current_value,
            options: current_options,
            ..
        } = current
        {
            let options = patch
                .get("options")
                .and_then(|v| v.as_array())
                .map(|entries| {
                    entries
                        .iter()
                        .filter_map(|entry| entry.as_str().map(str::to_string))
                        .collect::<Vec<_>>()
                })
                .unwrap_or(current_options);
            let unique_options = if options.is_empty() {
                vec!["Option A".into()]
            } else {
                options
            };
            let value = patch
                .get("value")
                .and_then(|v| v.as_str())
                .filter(|v| unique_options.iter().any(|option| option == *v))
                .map(str::to_string)
                .or_else(|| {
                    unique_options
                        .iter()
                        .find(|option| **option == current_value)
                        .cloned()
                })
                .unwrap_or_else(|| unique_options[0].clone());
            return OsParameter::Categorical {
                id,
                name,
                options: unique_options,
                value,
            };
        }
    }
    if patch_type == Some("toggle")
        || (patch_type.is_none() && matches!(parameter, OsParameter::Toggle { .. }))
    {
        let current = match parameter {
            OsParameter::Toggle { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Toggle, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Toggle {
            id,
            value: current_value,
            ..
        } = current
        {
            let value = patch
                .get("value")
                .and_then(|v| v.as_bool())
                .unwrap_or(current_value);
            return OsParameter::Toggle { id, name, value };
        }
    }
    let current = match parameter {
        OsParameter::Text { .. } => parameter.clone(),
        _ => create_default_os_parameter(&OsParameterType::Text, &name, Some(parameter_id(parameter))),
    };
    if let OsParameter::Text {
        id,
        value: current_value,
        ..
    } = current
    {
        let value = patch
            .get("value")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .unwrap_or(current_value);
        return OsParameter::Text { id, name, value };
    }
    parameter.clone()
}

fn parameter_id(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { id, .. }
        | OsParameter::Categorical { id, .. }
        | OsParameter::Toggle { id, .. }
        | OsParameter::Text { id, .. } => id,
    }
}

fn parameter_name(parameter: &OsParameter) -> String {
    match parameter {
        OsParameter::Numeric { name, .. }
        | OsParameter::Categorical { name, .. }
        | OsParameter::Toggle { name, .. }
        | OsParameter::Text { name, .. } => name.clone(),
    }
}

fn json_pointer_segments(pointer: &str) -> Vec<String> {
    if let Some(rest) = pointer.strip_prefix('/') {
        rest.split('/').filter(|segment| !segment.is_empty()).map(str::to_string).collect()
    } else {
        pointer
            .split('.')
            .filter(|segment| !segment.is_empty())
            .map(str::to_string)
            .collect()
    }
}

/// @emoji 🎛️ Deep-sets a JSON-pointer path on a plain object projection.
pub fn set_json_pointer_value(root: &mut Value, pointer: &str, value: Value) {
    let segments = json_pointer_segments(pointer);
    if segments.is_empty() {
        return;
    }
    let mut current = root;
    for segment in &segments[..segments.len() - 1] {
        if !current.is_object() {
            *current = Value::Object(Default::default());
        }
        let object = current.as_object_mut().expect("object");
        let entry = object
            .entry(segment.clone())
            .or_insert_with(|| Value::Object(Default::default()));
        if !entry.is_object() {
            *entry = Value::Object(Default::default());
        }
        current = entry;
    }
    if let Some(object) = current.as_object_mut() {
        object.insert(segments.last().cloned().unwrap_or_default(), value);
    }
}

/// @emoji 🎛️ Applies bound studio parameter values onto an app projection via JSON pointers.
pub fn apply_parameter_values_to_projection(
    projection: Value,
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
    instance_id: &str,
) -> Value {
    let instance_bindings: Vec<_> = bindings
        .iter()
        .filter(|binding| binding.instance_id == instance_id)
        .collect();
    if instance_bindings.is_empty() {
        return projection;
    }
    let mut clone = projection;
    for binding in instance_bindings {
        let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
            continue;
        };
        set_json_pointer_value(&mut clone, &binding.field_path, os_parameter_value(parameter));
    }
    clone
}

trait OsParameterId {
    fn id(&self) -> &str;
}

impl OsParameterId for OsParameter {
    fn id(&self) -> &str {
        parameter_id(self)
    }
}

/// @emoji 🎛️ Resolves bound parameter values for an app instance as a field-path map.
pub fn resolve_parameter_values_for_instance(
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
    instance_id: &str,
) -> HashMap<String, Value> {
    let mut values = HashMap::new();
    for binding in bindings
        .iter()
        .filter(|entry| entry.instance_id == instance_id)
    {
        let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
            continue;
        };
        values.insert(binding.field_path.clone(), os_parameter_value(parameter));
    }
    values
}

/// @emoji 🎛️ Builds the media graph input port id for a bound studio parameter.
pub fn parameter_port_id(instance_id: &str, parameter_id: &str) -> String {
    media_port_id_for_spec(instance_id, &format!("{OS_PARAMETER_PORT_PREFIX}{parameter_id}"), "in")
}

/// @emoji 🎛️ Returns whether a media port id denotes a studio parameter input channel.
pub fn is_parameter_port_id(port_id: &str) -> bool {
    media_port_spec_id(port_id)
        .map(|spec_id| spec_id.starts_with(OS_PARAMETER_PORT_PREFIX))
        .unwrap_or(false)
}

/// @emoji 🎛️ Extracts the studio parameter id from a parameter input port id.
pub fn parameter_id_from_port_id(port_id: &str) -> Option<String> {
    let spec_id = media_port_spec_id(port_id)?;
    spec_id
        .strip_prefix(OS_PARAMETER_PORT_PREFIX)
        .map(str::to_string)
}

pub fn media_port_id_for_spec(instance_id: &str, spec_id: &str, direction: &str) -> String {
    format!("{instance_id}:{spec_id}:{direction}")
}

pub fn media_port_spec_id(port_id: &str) -> Option<String> {
    let parts: Vec<_> = port_id.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    Some(parts[1..parts.len() - 1].join(":"))
}
//#endregion 🔖Parameters

//#region 🔖Materialize
use std::sync::{Mutex, OnceLock};

static OS_FIXTURE_JSON: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn os_fixture_json_registry() -> &'static Mutex<HashMap<String, String>> {
    OS_FIXTURE_JSON.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @emoji 📎 Registers bundled fixture JSON for `payloadRef` materialization.
pub fn register_os_fixture_json(slug: &str, json: &str) {
    os_fixture_json_registry()
        .lock()
        .expect("fixture registry")
        .insert(slug.into(), json.into());
}

/// @emoji 📎 Looks up bundled fixture JSON by slug — the seed content for a freshly spawned app
/// document. Replaces the old `OsSourceDocument.payloadRef = "fixture:…"` resolution: since app
/// content no longer embeds in the os document, seeding now happens once, host-side, at
/// {@link OsDocumentRef} creation time (see `host_runtime`), not on every materialize/read.
pub fn os_fixture_json(slug: &str) -> Option<String> {
    os_fixture_json_registry().lock().ok().and_then(|registry| registry.get(slug).cloned())
}

/// @emoji 🧩 Overlays bound parameter values onto an app instance's current document projection.
/// Content itself lives in the app's own `framework/sync`-hosted document (referenced by
/// {@link OsDocumentRef}, read host-side and passed in as `current_document_json`) — this function
/// no longer resolves embedded/upstream source documents; that concept was deleted with
/// `OsSourceDocument`. Cross-instance ("upstream") dataflow through media-graph edges is deferred
/// (see `host_runtime` doc-comment) to a follow-up that reads the upstream app's live document.
pub fn materialize_os_app_instance_document_json(
    current_document_json: &str,
    instance_id: &str,
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
) -> String {
    let projection: Value = serde_json::from_str(current_document_json).unwrap_or_else(|_| json!({}));
    let with_params = apply_parameter_values_to_projection(projection, bindings, parameters, instance_id);
    serde_json::to_string(&with_params).unwrap_or_else(|_| "{}".into())
}

/// @emoji 🔀 Host-side hook for the common case: when a bound parameter's value changes, computes the
/// patched document JSON for every app instance with a field bound to it, keyed by document id — the
/// host dispatches each as a snapshot replace into that app's own document store (e.g. via the plugin
/// WIT boundary's `load-app-document`, or `framework/sync`'s document actor once the app is wired onto
/// `DocumentHost`). This covers the "common/simple case" per the JSON-pointer overlay convention
/// {@link apply_parameter_values_to_projection} already established — a true typed op into the bound
/// app's own `Op` vocabulary requires that app's real (non-opaque) Op type and is left to each app's
/// own `DocumentApp` migration (WS-F); until then this snapshot-replace path is the host's only lever.
pub fn app_instance_document_patches_for_binding(
    parameter_id: &str,
    instances: &[OsAppInstance],
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
    current_document_json: impl Fn(&str) -> Option<String>,
) -> Vec<(String, String)> {
    let bound_instance_ids: HashSet<String> = bindings
        .iter()
        .filter(|binding| binding.parameter_id == parameter_id)
        .map(|binding| binding.instance_id.clone())
        .collect();
    instances
        .iter()
        .filter(|instance| bound_instance_ids.contains(&instance.id))
        .filter_map(|instance| {
            let current_json = current_document_json(&instance.document.document_id)?;
            let patched =
                materialize_os_app_instance_document_json(&current_json, &instance.id, bindings, parameters);
            Some((instance.document.document_id.clone(), patched))
        })
        .collect()
}
//#endregion 🔖Materialize

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patches_numeric_parameter_with_constraints() {
        let parameter = create_default_os_parameter(&OsParameterType::Numeric, "Zoom", None);
        let patched = patch_os_parameter(
            &parameter,
            &serde_json::json!({ "value": 12.0, "max": 10.0 }),
        );
        match patched {
            OsParameter::Numeric { value, .. } => assert_eq!(value, 10.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn applies_json_pointer_parameter_overrides() {
        let projection = serde_json::json!({ "brushSize": 8 });
        let overridden = apply_parameter_values_to_projection(
            projection,
            &[OsParameterFieldBinding {
                parameter_id: "p1".into(),
                instance_id: "i1".into(),
                field_path: "/brushSize".into(),
            }],
            &[OsParameter::Numeric {
                id: "p1".into(),
                name: "Brush".into(),
                value: 42.0,
                min: None,
                max: None,
                step: None,
            }],
            "i1",
        );
        assert_eq!(overridden["brushSize"], 42.0);
    }

    #[test]
    fn resolves_fixture_json_by_slug() {
        register_os_fixture_json("semio.draw.json", r#"{"schema":"draw.document","id":"semio"}"#);
        let json = os_fixture_json("semio.draw.json").expect("registered fixture");
        let parsed: Value = serde_json::from_str(&json).expect("json");
        assert_eq!(parsed["schema"], "draw.document");
        assert_eq!(parsed["id"], "semio");
    }

    #[test]
    fn materializes_instance_documents_with_parameter_overrides() {
        let json = materialize_os_app_instance_document_json(
            r#"{"schema":"draw.document","id":"semio"}"#,
            "app-draw-1",
            &[],
            &[],
        );
        let parsed: Value = serde_json::from_str(&json).expect("json");
        assert_eq!(parsed["schema"], "draw.document");
        assert_eq!(parsed["id"], "semio");
    }
}
//#endregion 🧪Tests
// #endregion instance
}

pub mod media_export_raster {
// #region media_export_raster
//! 🖼️ SVG rasterization, DWG flattening, and media-export registration helpers.

use crate::media_graph::{
    register_os_media_export_handler, register_os_media_import_handler, OsMediaFormat, OsMediaExportResult,
};
use base64::Engine;
use png::{BitDepth, ColorType, Encoder};
use semio_framework_core::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// @emoji 🖼️ Rasterizes SVG markup to a base64-encoded PNG payload.
pub fn rasterize_svg_to_png_base64(svg: &str, width: u32, height: u32) -> Result<String, String> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).map_err(|error| error.to_string())?;
    let size = tree.size();
    let render_w = if width > 0 {
        width
    } else {
        size.width().ceil().max(1.0) as u32
    };
    let render_h = if height > 0 {
        height
    } else {
        size.height().ceil().max(1.0) as u32
    };
    let mut pixmap = tiny_skia::Pixmap::new(render_w, render_h).ok_or_else(|| "invalid raster dimensions".to_string())?;
    let scale_x = render_w as f32 / size.width().max(1.0);
    let scale_y = render_h as f32 / size.height().max(1.0);
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale_x, scale_y),
        &mut pixmap.as_mut(),
    );
    let png_bytes = encode_rgba_png(pixmap.data(), pixmap.width(), pixmap.height())?;
    Ok(base64::engine::general_purpose::STANDARD.encode(png_bytes))
}

fn encode_rgba_png(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    {
        let mut encoder = Encoder::new(&mut bytes, width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|error| error.to_string())?;
        writer.write_image_data(pixels).map_err(|error| error.to_string())?;
    }
    Ok(bytes)
}

/// @emoji 📐 Flattens SVG markup into a DWG drawing by walking usvg path geometry into layered polylines.
pub fn svg_to_dwg_bytes(svg: &str) -> Result<Vec<u8>, String> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).map_err(|error| error.to_string())?;
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    let height = tree.size().height() as f64;
    collect_svg_children(tree.root().children(), &mut drawing, layer, height);
    semio_framework_core::dwg_to_bytes(&drawing)
}

fn collect_svg_children(nodes: &[usvg::Node], drawing: &mut DwgDrawing, layer: usize, height: f64) {
    for node in nodes {
        match node {
            usvg::Node::Group(group) => {
                let id = node.id();
                let group_layer = if id.is_empty() { layer } else { drawing.ensure_layer(id) };
                collect_svg_children(group.children(), drawing, group_layer, height);
            }
            usvg::Node::Path(path) => collect_svg_path(path, drawing, layer, height),
            _ => {}
        }
    }
}

fn collect_svg_path(path: &usvg::Path, drawing: &mut DwgDrawing, layer: usize, height: f64) {
    let transform = path.abs_transform();
    let mut vertices: Vec<[f64; 2]> = Vec::new();
    let mut closed = false;
    for segment in path.data().segments() {
        match segment {
            usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                flush_svg_polyline(drawing, layer, &mut vertices, &mut closed);
                vertices.push(transformed_svg_point(transform, p, height));
            }
            usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                vertices.push(transformed_svg_point(transform, p, height));
            }
            usvg::tiny_skia_path::PathSegment::QuadTo(c, p) => {
                flatten_quad_into(&mut vertices, transform, c, p, height);
            }
            usvg::tiny_skia_path::PathSegment::CubicTo(c1, c2, p) => {
                flatten_cubic_into(&mut vertices, transform, c1, c2, p, height);
            }
            usvg::tiny_skia_path::PathSegment::Close => {
                closed = true;
            }
        }
    }
    flush_svg_polyline(drawing, layer, &mut vertices, &mut closed);
}

fn transformed_svg_point(transform: usvg::Transform, point: usvg::tiny_skia_path::Point, height: f64) -> [f64; 2] {
    let mut p = point;
    transform.map_point(&mut p);
    [p.x as f64, height - p.y as f64]
}

fn flatten_quad_into(vertices: &mut Vec<[f64; 2]>, transform: usvg::Transform, ctrl: usvg::tiny_skia_path::Point, to: usvg::tiny_skia_path::Point, height: f64) {
    let from = vertices.last().copied().unwrap_or([0.0, 0.0]);
    let ctrl_p = transformed_svg_point(transform, ctrl, height);
    let to_p = transformed_svg_point(transform, to, height);
    const STEPS: usize = 12;
    for step in 1..=STEPS {
        let t = step as f64 / STEPS as f64;
        let mt = 1.0 - t;
        vertices.push([
            mt * mt * from[0] + 2.0 * mt * t * ctrl_p[0] + t * t * to_p[0],
            mt * mt * from[1] + 2.0 * mt * t * ctrl_p[1] + t * t * to_p[1],
        ]);
    }
}

fn flatten_cubic_into(vertices: &mut Vec<[f64; 2]>, transform: usvg::Transform, c1: usvg::tiny_skia_path::Point, c2: usvg::tiny_skia_path::Point, to: usvg::tiny_skia_path::Point, height: f64) {
    let from = vertices.last().copied().unwrap_or([0.0, 0.0]);
    let c1p = transformed_svg_point(transform, c1, height);
    let c2p = transformed_svg_point(transform, c2, height);
    let to_p = transformed_svg_point(transform, to, height);
    const STEPS: usize = 16;
    for step in 1..=STEPS {
        let t = step as f64 / STEPS as f64;
        let mt = 1.0 - t;
        vertices.push([
            mt * mt * mt * from[0] + 3.0 * mt * mt * t * c1p[0] + 3.0 * mt * t * t * c2p[0] + t * t * t * to_p[0],
            mt * mt * mt * from[1] + 3.0 * mt * mt * t * c1p[1] + 3.0 * mt * t * t * c2p[1] + t * t * t * to_p[1],
        ]);
    }
}

fn flush_svg_polyline(drawing: &mut DwgDrawing, layer: usize, vertices: &mut Vec<[f64; 2]>, closed: &mut bool) {
    if vertices.len() > 1 {
        let count = vertices.len();
        drawing.entities.push(DwgEntity {
            layer,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::LwPolyline { closed: *closed, elevation: 0.0, vertices: std::mem::take(vertices), bulges: vec![0.0; count] },
        });
    } else {
        vertices.clear();
    }
    *closed = false;
}

/// @emoji 📐 Renders a DWG drawing back to flat SVG markup (lines and closed polygons), for the raster import path.
pub fn dwg_drawing_to_svg(drawing: &semio_framework_core::DwgDrawing) -> Result<(String, u32, u32), String> {
    let width = (drawing.extmax[0] - drawing.extmin[0]).max(1.0).ceil() as u32;
    let height = (drawing.extmax[1] - drawing.extmin[1]).max(1.0).ceil() as u32;
    let mut paths = String::new();
    for entity in &drawing.entities {
        if let DwgGeometry::LwPolyline { vertices, closed, .. } = &entity.geometry {
            if vertices.is_empty() {
                continue;
            }
            let mut d = format!("M {} {}", vertices[0][0] - drawing.extmin[0], drawing.extmax[1] - vertices[0][1]);
            for v in &vertices[1..] {
                d.push_str(&format!(" L {} {}", v[0] - drawing.extmin[0], drawing.extmax[1] - v[1]));
            }
            if *closed {
                d.push_str(" Z");
            }
            paths.push_str(&format!("<path d=\"{d}\" fill=\"none\" stroke=\"black\" stroke-width=\"1\"/>"));
        }
    }
    let svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">{paths}</svg>");
    Ok((svg, width, height))
}

/// @emoji 💾 Registers SVG, PNG, and DWG export handlers for one 2D resource kind.
pub fn register_2d_export_handlers(
    resource_kind: &'static str,
    file_stem: &'static str,
    document_to_svg: fn(&Value) -> Result<(String, u32, u32), String>,
) {
    register_os_media_export_handler(resource_kind, OsMediaFormat::Svg, move |doc| {
        let (svg, _width, _height) = document_to_svg(doc)?;
        Ok(OsMediaExportResult {
            data: svg,
            mime_type: OsMediaFormat::Svg.mime_type().into(),
            file_name: format!("{file_stem}.svg"),
            encoding: None,
        })
    });
    register_os_media_export_handler(resource_kind, OsMediaFormat::Png, move |doc| {
        let (svg, width, height) = document_to_svg(doc)?;
        let data = rasterize_svg_to_png_base64(&svg, width, height)?;
        Ok(OsMediaExportResult {
            data,
            mime_type: OsMediaFormat::Png.mime_type().into(),
            file_name: format!("{file_stem}.png"),
            encoding: Some("base64".into()),
        })
    });
    register_os_media_export_handler(resource_kind, OsMediaFormat::Dwg, move |doc| {
        let (svg, _width, _height) = document_to_svg(doc)?;
        let bytes = svg_to_dwg_bytes(&svg)?;
        Ok(OsMediaExportResult {
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            mime_type: OsMediaFormat::Dwg.mime_type().into(),
            file_name: format!("{file_stem}.dwg"),
            encoding: Some("base64".into()),
        })
    });
}

/// @emoji 📥 Registers a DWG import handler for one 2D resource kind, rasterizing DWG geometry into flat SVG first.
pub fn register_dwg_import_handler(resource_kind: &'static str, from_dwg: fn(&DwgDrawing) -> Result<Value, String>) {
    register_os_media_import_handler(resource_kind, OsMediaFormat::Dwg, move |bytes| {
        let drawing = semio_framework_core::dwg_from_bytes(bytes)?;
        from_dwg(&drawing)
    });
}

/// @emoji 🧵 Registers one `MeshExporter` format (Obj/Glb/Stl/…) for a mesh resource kind; call once per format — `mesh_from_document` bridges the OS media-graph's per-document export pipeline down to the format-agnostic `MeshData` the exporter instance actually encodes. DWG stays on `register_mesh_dwg_import_handler`'s sibling below; it is not part of the `MeshExporter` mechanism.
pub fn register_mesh_exporter(
    resource_kind: &'static str,
    file_stem: &'static str,
    mesh_from_document: fn(&Value) -> Result<semio_framework_plugin::MeshData, String>,
    exporter: Box<dyn semio_framework_plugin::MeshExporter>,
) {
    let format = exporter.format();
    let ext = format.as_str();
    let mime_type = format.mime_type().to_string();
    let binary = format.is_binary();
    register_os_media_export_handler(resource_kind, format, move |doc| {
        let mesh = mesh_from_document(doc)?;
        let bytes = exporter.export(&mesh)?;
        let data = if binary {
            base64::engine::general_purpose::STANDARD.encode(&bytes)
        } else {
            String::from_utf8(bytes).map_err(|error| error.to_string())?
        };
        Ok(OsMediaExportResult {
            data,
            mime_type: mime_type.clone(),
            file_name: format!("{file_stem}.{ext}"),
            encoding: if binary { Some("base64".into()) } else { None },
        })
    });
}

/// @emoji 🧵 Registers one `MeshImporter` format (Obj/Glb/Stl/…) for a mesh resource kind; `document_from_mesh` bridges the decoded `MeshData` back into the app's own document shape.
pub fn register_mesh_importer(
    resource_kind: &'static str,
    document_from_mesh: fn(&semio_framework_plugin::MeshData) -> Result<Value, String>,
    importer: Box<dyn semio_framework_plugin::MeshImporter>,
) {
    let format = importer.format();
    register_os_media_import_handler(resource_kind, format, move |bytes| {
        let mesh = importer.import(bytes)?;
        document_from_mesh(&mesh)
    });
}

/// @emoji 📥 Registers a DWG import handler for one mesh resource kind.
pub fn register_mesh_dwg_import_handler(resource_kind: &'static str, document_from_mesh: fn(&semio_framework_plugin::MeshData) -> Result<Value, String>) {
    register_os_media_import_handler(resource_kind, OsMediaFormat::Dwg, move |bytes| {
        let drawing = semio_framework_core::dwg_from_bytes(bytes)?;
        let mesh = semio_framework_core::dwg_drawing_to_mesh(&drawing);
        document_from_mesh(&mesh)
    });
}

/// @emoji 💾 Registers a DWG export handler for one mesh resource kind; DWG is not part of the `MeshExporter` mechanism (it flattens a mesh into a DWG drawing, not a mesh codec), so it stays a dedicated registrar alongside `register_mesh_exporter`.
pub fn register_mesh_dwg_export_handler(
    resource_kind: &'static str,
    file_stem: &'static str,
    mesh_from_document: fn(&Value) -> Result<semio_framework_plugin::MeshData, String>,
) {
    register_os_media_export_handler(resource_kind, OsMediaFormat::Dwg, move |doc| {
        let mesh = mesh_from_document(doc)?;
        let drawing = semio_framework_core::mesh_to_dwg_drawing(&mesh);
        let bytes = semio_framework_core::dwg_to_bytes(&drawing)?;
        Ok(OsMediaExportResult {
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            mime_type: OsMediaFormat::Dwg.mime_type().into(),
            file_name: format!("{file_stem}.dwg"),
            encoding: Some("base64".into()),
        })
    });
}

//#region SolidMediaExport
type SolidExporterRegistry = HashMap<String, Box<dyn kernel_3d_brepkit::SolidExporter>>;

fn solid_exporters() -> &'static Mutex<SolidExporterRegistry> {
    static HANDLERS: OnceLock<Mutex<SolidExporterRegistry>> = OnceLock::new();
    HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
}

type SolidImporterRegistry = HashMap<String, Box<dyn kernel_3d_brepkit::SolidImporter>>;

fn solid_importers() -> &'static Mutex<SolidImporterRegistry> {
    static HANDLERS: OnceLock<Mutex<SolidImporterRegistry>> = OnceLock::new();
    HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn solid_registry_key(resource_kind: &str, format: &OsMediaFormat) -> String {
    format!("{}:{}", resource_kind, format.as_str())
}

/// @emoji 🧊 Registers a B-Rep solid exporter (STEP/STL/OBJ/GLB, operating on `GeometryHandle` via `kernel_3d_brepkit::BrepkitKernel` rather than a tessellated `MeshData`) for a resource kind; call once per format.
pub fn register_solid_exporter(resource_kind: &str, exporter: Box<dyn kernel_3d_brepkit::SolidExporter>) {
    let key = solid_registry_key(resource_kind, &exporter.format());
    solid_exporters().lock().expect("lock").insert(key, exporter);
}

/// @emoji 🧊 Registers a B-Rep solid importer for a resource kind; see `register_solid_exporter`.
pub fn register_solid_importer(resource_kind: &str, importer: Box<dyn kernel_3d_brepkit::SolidImporter>) {
    let key = solid_registry_key(resource_kind, &importer.format());
    solid_importers().lock().expect("lock").insert(key, importer);
}

/// @emoji 🧊 Looks up a previously registered solid exporter for a resource kind + format.
pub fn solid_exporter_for(resource_kind: &str, format: &OsMediaFormat) -> bool {
    solid_exporters().lock().expect("lock").contains_key(&solid_registry_key(resource_kind, format))
}

/// @emoji 🧊 Exports `shapes` from `kernel` through the solid exporter registered for `resource_kind` + `format`.
pub fn export_registered_solid(
    resource_kind: &str,
    format: &OsMediaFormat,
    kernel: &kernel_3d_brepkit::BrepkitKernel,
    shapes: &[kernel_3d_engine::GeometryHandle],
    deflection: f64,
) -> Result<Vec<u8>, String> {
    let key = solid_registry_key(resource_kind, format);
    let handlers = solid_exporters().lock().expect("lock");
    let exporter = handlers.get(&key).ok_or_else(|| format!("no solid export handler for {key}"))?;
    exporter.export(kernel, shapes, deflection).map_err(|error| error.to_string())
}

/// @emoji 🧊 Imports bytes into `kernel` through the solid importer registered for `resource_kind` + `format`.
pub fn import_registered_solid(
    resource_kind: &str,
    format: &OsMediaFormat,
    kernel: &mut kernel_3d_brepkit::BrepkitKernel,
    data: &[u8],
    tolerance: f64,
) -> Result<Vec<kernel_3d_engine::GeometryHandle>, String> {
    let key = solid_registry_key(resource_kind, format);
    let handlers = solid_importers().lock().expect("lock");
    let importer = handlers.get(&key).ok_or_else(|| format!("no solid import handler for {key}"))?;
    importer.import(kernel, data, tolerance).map_err(|error| error.to_string())
}
//#endregion SolidMediaExport
// #endregion media_export_raster
}

pub mod media_export_simple {
// #region media_export_simple
//! 🖼️ Lightweight SVG builders for simple document exports.

use serde_json::Value;

fn escape_svg_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// @emoji 🖼️ Wraps SVG body markup with explicit dimensions.
pub fn wrap_svg(width: u32, height: u32, body: &str) -> (String, u32, u32) {
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">{body}</svg>"#
    );
    (svg, width, height)
}

/// @emoji 🏷️ Builds a title-card SVG from a document JSON value.
pub fn title_card_svg(value: &Value, label: &str, width: u32, height: u32) -> Result<(String, u32, u32), String> {
    let title = value
        .get("title")
        .and_then(|entry| entry.as_str())
        .or_else(|| value.get("id").and_then(|entry| entry.as_str()))
        .unwrap_or(label);
    let body = format!(
        "<rect width=\"100%\" height=\"100%\" fill=\"white\"/><text x=\"32\" y=\"64\" font-size=\"32\" fill=\"#111827\">{}</text>",
        escape_svg_text(title)
    );
    Ok(wrap_svg(width, height, &body))
}

/// @emoji 📄 Serializes page-like rectangles from a `pages` array.
pub fn pages_rects_svg(value: &Value, fallback_label: &str) -> Result<(String, u32, u32), String> {
    let pages = value
        .get("pages")
        .and_then(|entry| entry.as_array())
        .cloned()
        .unwrap_or_default();
    if pages.is_empty() {
        return title_card_svg(value, fallback_label, 1024, 768);
    }
    let mut max_x = 0.0f64;
    let mut max_y = 0.0f64;
    let mut body = String::new();
    for (index, page) in pages.iter().enumerate() {
        let width = page.get("width").and_then(|entry| entry.as_f64()).unwrap_or(800.0);
        let height = page.get("height").and_then(|entry| entry.as_f64()).unwrap_or(600.0);
        let x = page.get("x").and_then(|entry| entry.as_f64()).unwrap_or((index as f64) * (width + 24.0));
        let y = page.get("y").and_then(|entry| entry.as_f64()).unwrap_or(0.0);
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
        body.push_str(&format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" fill=\"white\" stroke=\"#94a3b8\" stroke-width=\"2\"/>"
        ));
    }
    Ok(wrap_svg(max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32, &body))
}

/// @emoji 🗺️ Serializes point features from common GIS fixture fields.
pub fn map_points_svg(value: &Value, fallback_label: &str) -> Result<(String, u32, u32), String> {
    let positions = value
        .get("positions")
        .or_else(|| value.get("points"))
        .and_then(|entry| entry.as_array())
        .cloned()
        .unwrap_or_default();
    if positions.is_empty() {
        return title_card_svg(value, fallback_label, 1024, 768);
    }
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;
    for position in &positions {
        let Some(coords) = position.as_array() else { continue };
        let x = coords.first().and_then(|entry| entry.as_f64()).unwrap_or(0.0);
        let y = coords.get(1).and_then(|entry| entry.as_f64()).unwrap_or(0.0);
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    let pad = 32.0;
    let width = ((max_x - min_x) + pad * 2.0).max(256.0).round() as u32;
    let height = ((max_y - min_y) + pad * 2.0).max(256.0).round() as u32;
    let shifted = positions
        .iter()
        .filter_map(|position| position.as_array())
        .map(|coords| {
            let x = coords.first().and_then(|entry| entry.as_f64()).unwrap_or(0.0) - min_x + pad;
            let y = coords.get(1).and_then(|entry| entry.as_f64()).unwrap_or(0.0) - min_y + pad;
            format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"6\" fill=\"#2563eb\"/>")
        })
        .collect::<Vec<_>>()
        .join("");
    if shifted.is_empty() {
        return title_card_svg(value, fallback_label, 1024, 768);
    }
    Ok(wrap_svg(width, height, &shifted))
}
// #endregion media_export_simple
}

pub mod media_graph {
// #region media_graph
//! 🎬 Media graph, VFS projection types, and media export registry.

use crate::host::OsOp;
use crate::instance::{
    create_os_id, is_parameter_port_id, media_port_spec_id, parameter_id_from_port_id,
    parameter_port_id, OsAppInstance, OsParameter, OsParameterFieldBinding,
};
use crate::registry::{os_app_primary_output_kind, os_app_registration, OsAppRegistration};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

pub const OS_STUDIO_SCHEMA: &str = "s.studio";
pub const OS_MEDIA_GRAPH_SCHEMA: &str = "s.media-graph";
pub const OS_MEDIA_GRAPH_VFS_ROOT_ID: &str = "os-media-graph-root";
pub const OS_MEDIA_FLOW_MODULE_ID: &str = "os-media";

//#region 🔖MediaGraph
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaPort {
    pub id: String,
    pub resource_kind: String,
    pub direction: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphNode {
    pub id: String,
    pub instance_id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub inputs: Vec<OsMediaPort>,
    pub outputs: Vec<OsMediaPort>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraph {
    pub schema: String,
    pub nodes: Vec<OsMediaGraphNode>,
    pub edges: Vec<OsMediaGraphEdge>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaGraphPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MediaGraphValidation {
    pub ok: bool,
    pub errors: Vec<String>,
}

pub fn empty_media_graph() -> OsMediaGraph {
    OsMediaGraph {
        schema: OS_MEDIA_GRAPH_SCHEMA.into(),
        nodes: Vec::new(),
        edges: Vec::new(),
    }
}

pub fn media_graph_node_for_instance(
    instance: &OsAppInstance,
    registration: &OsAppRegistration,
    position: &MediaGraphPosition,
    node_id: &str,
) -> OsMediaGraphNode {
    let (inputs, outputs) =
        crate::registry::media_graph_node_ports_for_registration(&instance.id, registration);
    let port_count = inputs.len().max(outputs.len()).max(1);
    OsMediaGraphNode {
        id: node_id.into(),
        instance_id: instance.id.clone(),
        x: position.x,
        y: position.y,
        width: 220.0,
        height: 56.0 + port_count as f64 * 18.0,
        inputs,
        outputs,
    }
}

fn sync_media_node_parameter_ports(
    node: &OsMediaGraphNode,
    bindings: &[OsParameterFieldBinding],
) -> OsMediaGraphNode {
    let instance_bindings: Vec<_> = bindings
        .iter()
        .filter(|binding| binding.instance_id == node.instance_id)
        .collect();
    let base_inputs: Vec<_> = node
        .inputs
        .iter()
        .filter(|port| !is_parameter_port_id(&port.id))
        .cloned()
        .collect();
    let parameter_inputs: Vec<_> = instance_bindings
        .iter()
        .map(|binding| OsMediaPort {
            id: parameter_port_id(&node.instance_id, &binding.parameter_id),
            resource_kind: "parameter.value".into(),
            direction: "in".into(),
        })
        .collect();
    let inputs: Vec<_> = base_inputs.into_iter().chain(parameter_inputs).collect();
    let port_count = inputs.len().max(node.outputs.len()).max(1);
    OsMediaGraphNode {
        inputs,
        height: 56.0 + port_count as f64 * 18.0,
        ..node.clone()
    }
}

pub fn sync_media_graph_parameter_ports(
    graph: &OsMediaGraph,
    bindings: &[OsParameterFieldBinding],
) -> OsMediaGraph {
    OsMediaGraph {
        schema: OS_MEDIA_GRAPH_SCHEMA.into(),
        nodes: graph
            .nodes
            .iter()
            .map(|node| sync_media_node_parameter_ports(node, bindings))
            .collect(),
        edges: graph.edges.clone(),
    }
}

/// @emoji ✅ Validates media graph connectivity and cycle freedom.
pub fn validate_media_graph(graph: &OsMediaGraph) -> MediaGraphValidation {
    let mut errors = Vec::new();
    let node_ids: HashSet<_> = graph.nodes.iter().map(|node| node.id.clone()).collect();
    for edge in &graph.edges {
        if !node_ids.contains(&edge.source_node_id) {
            errors.push(format!("missing source node {}", edge.source_node_id));
        }
        if !node_ids.contains(&edge.target_node_id) {
            errors.push(format!("missing target node {}", edge.target_node_id));
        }
    }
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        adjacency
            .entry(edge.source_node_id.clone())
            .or_default()
            .push(edge.target_node_id.clone());
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    fn dfs(
        node_id: &str,
        adjacency: &HashMap<String, Vec<String>>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        errors: &mut Vec<String>,
    ) {
        if visited.contains(node_id) {
            return;
        }
        if visiting.contains(node_id) {
            errors.push(format!("cycle detected at {node_id}"));
            return;
        }
        visiting.insert(node_id.to_string());
        for next in adjacency.get(node_id).into_iter().flatten() {
            dfs(next, adjacency, visiting, visited, errors);
        }
        visiting.remove(node_id);
        visited.insert(node_id.to_string());
    }
    for node in &graph.nodes {
        dfs(&node.id, &adjacency, &mut visiting, &mut visited, &mut errors);
    }
    MediaGraphValidation {
        ok: errors.is_empty(),
        errors,
    }
}

pub fn os_media_neuron_kind_for_node(node_id: &str) -> String {
    format!("os.media.node.{node_id}")
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for OsMediaGraphCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

pub fn os_media_graph_to_flow_fixture(
    graph: &OsMediaGraph,
    instances: &[OsAppInstance],
    camera: &OsMediaGraphCamera,
) -> Value {
    let instance_by_id: HashMap<_, _> = instances.iter().map(|instance| (instance.id.clone(), instance)).collect();
    let widgets: Vec<_> = graph
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            json!({
                "kind": "neuron",
                "id": node.id,
                "neuronKind": os_media_neuron_kind_for_node(&node.id),
                "inputPorts": node.inputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                "outputPorts": node.outputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                "params": {
                    "instanceId": node.instance_id,
                    "programId": instance.map(|entry| &entry.program_id).unwrap_or(&String::new()),
                    "appId": instance.map(|entry| &entry.app_id).unwrap_or(&String::new()),
                },
                "preview": true,
            })
        })
        .collect();
    let layout: HashMap<_, _> = graph
        .nodes
        .iter()
        .map(|node| {
            (
                node.id.clone(),
                json!({ "x": node.x + node.width / 2.0, "y": node.y + node.height / 2.0 }),
            )
        })
        .collect();
    let synapses: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            json!({
                "id": edge.id,
                "from": edge.source_node_id,
                "to": edge.target_node_id,
                "fromPort": edge.source_port_id,
                "toPort": edge.target_port_id,
            })
        })
        .collect();
    json!({
        "schema": "flow.fixture",
        "camera": { "x": camera.x, "y": camera.y, "zoom": camera.zoom },
        "widgets": widgets,
        "synapses": synapses,
        "layout": layout,
    })
}

/** @emoji 🔁 Diffs a flow fixture back into media-graph operations — inverse of [`os_media_graph_to_flow_fixture`]. */
pub fn apply_flow_fixture_to_os_media_graph(graph: &OsMediaGraph, fixture_json: &str) -> Vec<OsOp> {
    let Ok(fixture) = serde_json::from_str::<Value>(fixture_json) else {
        return Vec::new();
    };
    let mut ops = Vec::new();
    if let Some(layout) = fixture.get("layout").and_then(Value::as_object) {
        for node in &graph.nodes {
            let Some(position) = layout.get(&node.id) else { continue };
            let (Some(center_x), Some(center_y)) = (
                position.get("x").and_then(Value::as_f64),
                position.get("y").and_then(Value::as_f64),
            ) else {
                continue;
            };
            let x = center_x - node.width / 2.0;
            let y = center_y - node.height / 2.0;
            if (x - node.x).abs() > 1e-6 || (y - node.y).abs() > 1e-6 {
                ops.push(OsOp::MoveMediaNode { node_id: node.id.clone(), x, y });
            }
        }
    }
    let mut removed_node_ids = HashSet::new();
    if let Some(widgets) = fixture.get("widgets").and_then(Value::as_array) {
        let widget_ids: HashSet<&str> = widgets
            .iter()
            .filter_map(|widget| widget.get("id").and_then(Value::as_str))
            .collect();
        for node in &graph.nodes {
            if !widget_ids.contains(node.id.as_str()) {
                removed_node_ids.insert(node.id.clone());
                ops.push(OsOp::RemoveAppInstance { instance_id: node.instance_id.clone() });
            }
        }
    }
    let synapse_endpoints = |synapse: &Value| -> Option<(String, String, String, String)> {
        Some((
            synapse.get("from").and_then(Value::as_str)?.into(),
            synapse.get("fromPort").and_then(Value::as_str)?.into(),
            synapse.get("to").and_then(Value::as_str)?.into(),
            synapse.get("toPort").and_then(Value::as_str)?.into(),
        ))
    };
    let edge_endpoints = |edge: &OsMediaGraphEdge| {
        (
            edge.source_node_id.clone(),
            edge.source_port_id.clone(),
            edge.target_node_id.clone(),
            edge.target_port_id.clone(),
        )
    };
    let synapses = fixture.get("synapses").and_then(Value::as_array).cloned().unwrap_or_default();
    let fixture_endpoints: HashSet<_> = synapses.iter().filter_map(synapse_endpoints).collect();
    let graph_endpoints: HashSet<_> = graph.edges.iter().map(edge_endpoints).collect();
    for synapse in &synapses {
        let Some(endpoints) = synapse_endpoints(synapse) else { continue };
        if graph_endpoints.contains(&endpoints) {
            continue;
        }
        let (source_node_id, source_port_id, target_node_id, target_port_id) = endpoints;
        let id = synapse
            .get("id")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| create_os_id("edge"));
        ops.push(OsOp::ConnectMediaPorts {
            edge: OsMediaGraphEdge {
                id,
                source_node_id,
                source_port_id,
                target_node_id,
                target_port_id,
            },
        });
    }
    if fixture.get("synapses").and_then(Value::as_array).is_some() {
        for edge in &graph.edges {
            if fixture_endpoints.contains(&edge_endpoints(edge)) {
                continue;
            }
            if removed_node_ids.contains(&edge.source_node_id) || removed_node_ids.contains(&edge.target_node_id) {
                continue;
            }
            ops.push(OsOp::DisconnectMediaEdge { edge_id: edge.id.clone() });
        }
    }
    ops
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaNodeGraphPayload {
    pub nodes_json: String,
    pub edges_json: String,
    pub viewport_json: String,
    pub find_items_json: String,
}

/** @emoji 🕸️ Serializes an OS media graph into generic node-graph scene payloads. */
pub fn os_media_graph_to_node_graph_payload(
    graph: &OsMediaGraph,
    instances: &[OsAppInstance],
) -> OsMediaNodeGraphPayload {
    let instance_by_id: HashMap<_, _> = instances.iter().map(|instance| (instance.id.clone(), instance)).collect();
    let nodes: Vec<_> = graph
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            let label = instance
                .map(|entry| format!("{} / {}", entry.program_id, entry.app_id))
                .unwrap_or_else(|| node.instance_id.clone());
            json!({
                "id": node.id,
                "instanceId": node.instance_id,
                "label": label,
                "x": node.x,
                "y": node.y,
                "width": node.width,
                "height": node.height,
                "inputs": node.inputs.iter().map(|port| json!({
                    "id": port.id,
                    "resourceKind": port.resource_kind,
                    "direction": port.direction,
                    "label": port.id,
                })).collect::<Vec<_>>(),
                "outputs": node.outputs.iter().map(|port| json!({
                    "id": port.id,
                    "resourceKind": port.resource_kind,
                    "direction": port.direction,
                    "label": port.id,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let edges: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            json!({
                "id": edge.id,
                "sourceNodeId": edge.source_node_id,
                "sourcePortId": edge.source_port_id,
                "targetNodeId": edge.target_node_id,
                "targetPortId": edge.target_port_id,
            })
        })
        .collect();
    let find_items: Vec<_> = graph
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            json!({
                "id": node.instance_id,
                "label": instance
                    .map(|entry| format!("{} / {}", entry.program_id, entry.app_id))
                    .unwrap_or_else(|| node.instance_id.clone()),
                "category": "Media graph",
            })
        })
        .collect();
    OsMediaNodeGraphPayload {
        nodes_json: serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        edges_json: serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
        viewport_json: r#"{"x":0,"y":0,"zoom":1}"#.into(),
        find_items_json: serde_json::to_string(&find_items).unwrap_or_else(|_| "[]".into()),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaFlowChannelSpec {
    pub name: String,
    pub code: String,
    pub abbreviation: String,
    pub full_name: String,
    pub operators: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaFlowOperatorInfo {
    pub id: String,
    pub module: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    pub inputs: Vec<OsMediaFlowChannelSpec>,
    pub outputs: Vec<OsMediaFlowChannelSpec>,
}

fn os_media_flow_channel_spec(port_id: &str, resource_kind: &str, label: &str) -> OsMediaFlowChannelSpec {
    let code = port_id
        .chars()
        .next()
        .map(|ch| ch.to_uppercase().collect::<String>())
        .unwrap_or_else(|| "P".into());
    let abbreviation = if label.chars().count() <= 3 {
        label.into()
    } else {
        label.chars().take(3).collect()
    };
    OsMediaFlowChannelSpec {
        name: port_id.into(),
        code,
        abbreviation,
        full_name: label.into(),
        operators: vec![resource_kind.into()],
    }
}

fn parameter_label(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { name, .. }
        | OsParameter::Categorical { name, .. }
        | OsParameter::Toggle { name, .. }
        | OsParameter::Text { name, .. } => name,
    }
}

fn parameter_entity_id(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { id, .. }
        | OsParameter::Categorical { id, .. }
        | OsParameter::Toggle { id, .. }
        | OsParameter::Text { id, .. } => id,
    }
}

/// @emoji 🧩 Registers per-node neuron metadata for the OS media graph flow extension.
pub fn build_os_media_flow_operator_infos(
    graph: &OsMediaGraph,
    instances: &[OsAppInstance],
    parameters: &[OsParameter],
) -> Vec<OsMediaFlowOperatorInfo> {
    let instance_by_id: HashMap<_, _> = instances.iter().map(|row| (row.id.clone(), row)).collect();
    let parameter_by_id: HashMap<_, _> = parameters
        .iter()
        .map(|row| (parameter_entity_id(row).to_string(), row))
        .collect();
    graph
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            let registration = instance
                .and_then(|row| os_app_registration(&row.program_id, &row.app_id));
            let neuron_kind = os_media_neuron_kind_for_node(&node.id);
            OsMediaFlowOperatorInfo {
                id: neuron_kind,
                module: OS_MEDIA_FLOW_MODULE_ID.into(),
                name: instance
                    .map(|row| row.label.clone())
                    .unwrap_or_else(|| node.instance_id.clone()),
                abbreviation: instance
                    .map(|row| {
                        if row.app_id.chars().count() <= 3 {
                            row.app_id.clone()
                        } else {
                            row.app_id.chars().take(3).collect()
                        }
                    })
                    .unwrap_or_else(|| "app".into()),
                icon: format!(
                    "emoji:{}",
                    registration
                        .map(|row| row.component_kind.clone())
                        .unwrap_or_else(|| "s".into())
                ),
                summary: instance
                    .map(|row| format!("{}/{}", row.program_id, row.app_id))
                    .unwrap_or_else(|| "App instance".into()),
                inputs: node
                    .inputs
                    .iter()
                    .map(|port| {
                        let parameter_id = parameter_id_from_port_id(&port.id);
                        let label = parameter_id
                            .as_ref()
                            .and_then(|id| parameter_by_id.get(id))
                            .map(|parameter| parameter_label(*parameter).to_string())
                            .or_else(|| media_port_spec_id(&port.id))
                            .unwrap_or_else(|| port.id.clone());
                        os_media_flow_channel_spec(&port.id, &port.resource_kind, &label)
                    })
                    .collect(),
                outputs: node
                    .outputs
                    .iter()
                    .map(|port| {
                        let label = media_port_spec_id(&port.id).unwrap_or_else(|| port.id.clone());
                        os_media_flow_channel_spec(&port.id, &port.resource_kind, &label)
                    })
                    .collect(),
            }
        })
        .collect()
}
//#endregion 🔖MediaGraph

//#region 🔖ProgramRegistry
#[derive(Clone, Debug, Default)]
pub struct ProgramRegistry {
    instances: HashMap<String, OsAppInstance>,
}

impl ProgramRegistry {
    pub fn materialize_instance(&mut self, instance: OsAppInstance) {
        self.instances.insert(instance.id.clone(), instance);
    }

    pub fn get_instance(&self, instance_id: &str) -> Option<&OsAppInstance> {
        self.instances.get(instance_id)
    }
}
//#endregion 🔖ProgramRegistry

//#region 🔖MediaExport
/// 🗂️ Defined in `semio_framework_core` (below this crate in the dependency graph) so `MeshExporter`/`MeshImporter` there can name it too; re-exported here verbatim.
pub use semio_framework_core::OsMediaFormat;

//#region 🔖MediaCapability
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsMediaCapability {
    MeshOnly,
    Brep,
}

/// 🧬 Resolves which geometry backend a resource kind's exporters target: `cad`/`process`/`forms` sit on the real B-Rep kernel (`kernel_3d_brepkit`) and additionally require STEP; every other 3D/5D resource kind is backed by the lighter dependency-free `MeshData` representation and stops at OBJ/GLB/STL/DWG.
pub fn os_resource_media_capability(kind: &str) -> OsMediaCapability {
    match kind {
        "3d.cad" | "3d.process" | "form.dictionary" => OsMediaCapability::Brep,
        _ => OsMediaCapability::MeshOnly,
    }
}
//#endregion 🔖MediaCapability

#[derive(Clone, Debug, PartialEq)]
pub struct OsMediaExportResult {
    pub data: String,
    pub mime_type: String,
    pub file_name: String,
    pub encoding: Option<String>,
}

type OsMediaExportHandler = Box<dyn Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync>;

fn export_handlers() -> &'static Mutex<HashMap<String, OsMediaExportHandler>> {
    static HANDLERS: OnceLock<Mutex<HashMap<String, OsMediaExportHandler>>> = OnceLock::new();
    HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn os_media_export_key(resource_kind: &str, format: &OsMediaFormat) -> String {
    format!("{}:{}", resource_kind, format.as_str())
}

/// @emoji 💾 Registers an export handler for a media resource kind and format.
pub fn register_os_media_export_handler(
    resource_kind: &str,
    format: OsMediaFormat,
    handler: impl Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync + 'static,
) {
    export_handlers()
        .lock()
        .expect("lock")
        .insert(os_media_export_key(resource_kind, &format), Box::new(handler));
}

/// 📐 Required export formats per dimension; 3D/5D mesh-only apps stop at OBJ/GLB/STL/DWG, B-Rep apps (`os_resource_media_capability`) additionally require STEP.
pub fn required_os_media_export_formats(dimension: &str, capability: OsMediaCapability) -> Vec<OsMediaFormat> {
    match dimension {
        "2d" => vec![OsMediaFormat::Svg, OsMediaFormat::Png, OsMediaFormat::Dwg],
        "3d" | "5d" => match capability {
            OsMediaCapability::Brep => vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl, OsMediaFormat::Step, OsMediaFormat::Dwg],
            OsMediaCapability::MeshOnly => vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl, OsMediaFormat::Dwg],
        },
        _ => Vec::new(),
    }
}

/// @emoji ✅ Ensures every known resource kind has required export handlers.
pub fn assert_os_media_export_coverage() -> Result<(), String> {
    let handlers = export_handlers().lock().expect("lock");
    let mut missing = Vec::new();
    for descriptor in crate::registry::list_os_resource_descriptors() {
        let capability = os_resource_media_capability(&descriptor.kind);
        for format in required_os_media_export_formats(&descriptor.dimension, capability) {
            if !handlers.contains_key(&os_media_export_key(&descriptor.kind, &format)) {
                missing.push(format!("{}:{}", descriptor.kind, format.as_str()));
            }
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("missing os media export handlers: {}", missing.join(", ")))
    }
}

pub fn export_os_app_instance_media(
    instance: &OsAppInstance,
    source_document: &Value,
    format: OsMediaFormat,
) -> Result<OsMediaExportResult, String> {
    let handlers = export_handlers().lock().expect("lock");
    let handler = handlers
        .get(&os_media_export_key(&instance.yields, &format))
        .ok_or_else(|| format!("no export handler for {}:{}", instance.yields, format.as_str()))?;
    handler(source_document)
}

pub fn os_media_export_extension_for_format(format: &OsMediaFormat) -> &'static str {
    format.as_str()
}

type OsMediaImportHandler = Box<dyn Fn(&[u8]) -> Result<Value, String> + Send + Sync>;

fn import_handlers() -> &'static Mutex<HashMap<String, OsMediaImportHandler>> {
    static HANDLERS: OnceLock<Mutex<HashMap<String, OsMediaImportHandler>>> = OnceLock::new();
    HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @emoji 📥 Registers an import handler for a media resource kind and format; the handler turns raw bytes into a complete source document.
pub fn register_os_media_import_handler(
    resource_kind: &str,
    format: OsMediaFormat,
    handler: impl Fn(&[u8]) -> Result<Value, String> + Send + Sync + 'static,
) {
    import_handlers()
        .lock()
        .expect("lock")
        .insert(os_media_export_key(resource_kind, &format), Box::new(handler));
}

/// @emoji 📥 Formats every resource kind of the given dimension must accept for import; 2D stays DWG-only, 3D/5D mirrors `required_os_media_export_formats`.
pub fn required_os_media_import_formats(dimension: &str, capability: OsMediaCapability) -> Vec<OsMediaFormat> {
    match dimension {
        "2d" => vec![OsMediaFormat::Dwg],
        "3d" | "5d" => match capability {
            OsMediaCapability::Brep => vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl, OsMediaFormat::Step, OsMediaFormat::Dwg],
            OsMediaCapability::MeshOnly => vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl, OsMediaFormat::Dwg],
        },
        _ => Vec::new(),
    }
}

/// @emoji ✅ Ensures every known resource kind has required import handlers.
pub fn assert_os_media_import_coverage() -> Result<(), String> {
    let handlers = import_handlers().lock().expect("lock");
    let mut missing = Vec::new();
    for descriptor in crate::registry::list_os_resource_descriptors() {
        let capability = os_resource_media_capability(&descriptor.kind);
        for format in required_os_media_import_formats(&descriptor.dimension, capability) {
            if !handlers.contains_key(&os_media_export_key(&descriptor.kind, &format)) {
                missing.push(format!("{}:{}", descriptor.kind, format.as_str()));
            }
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("missing os media import handlers: {}", missing.join(", ")))
    }
}

/// @emoji 📥 Imports raw bytes for an app instance's resource kind, returning the new inline source document.
pub fn import_os_app_instance_media(instance: &OsAppInstance, data: &[u8], format: OsMediaFormat) -> Result<Value, String> {
    let handlers = import_handlers().lock().expect("lock");
    let handler = handlers
        .get(&os_media_export_key(&instance.yields, &format))
        .ok_or_else(|| format!("no import handler for {}:{}", instance.yields, format.as_str()))?;
    handler(data)
}
//#endregion 🔖MediaExport

//#region 🔖MediaGraphVfs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsDescriptorKind {
    pub id: String,
    pub name: String,
    pub presentation: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsFileNodeDescriptor {
    pub id: String,
    pub descriptor_kind_id: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsFileNodeKind {
    pub id: String,
    pub name: String,
    pub descriptors: Vec<OsMediaGraphVfsFileNodeDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsSchema {
    pub descriptor_kinds: HashMap<String, OsMediaGraphVfsDescriptorKind>,
    pub file_node_kinds: HashMap<String, OsMediaGraphVfsFileNodeKind>,
    pub descriptor_column_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsNodeRecord {
    pub id: String,
    pub file_node_kind_id: String,
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub has_children: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigate_uri: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub descriptor_values: HashMap<String, String>,
}

pub fn os_media_graph_vfs_schema() -> OsMediaGraphVfsSchema {
    let mut descriptor_kinds = HashMap::new();
    descriptor_kinds.insert(
        "text".into(),
        OsMediaGraphVfsDescriptorKind {
            id: "text".into(),
            name: "Text".into(),
            presentation: "text".into(),
        },
    );
    let binding = OsMediaGraphVfsFileNodeDescriptor {
        id: "binding".into(),
        descriptor_kind_id: "text".into(),
        label: "Binding".into(),
    };
    let mut file_node_kinds = HashMap::new();
    for kind in ["root", "instance", "folder", "source", "input"] {
        file_node_kinds.insert(
            kind.into(),
            OsMediaGraphVfsFileNodeKind {
                id: kind.into(),
                name: kind.into(),
                descriptors: vec![binding.clone()],
            },
        );
    }
    file_node_kinds.insert(
        "export".into(),
        OsMediaGraphVfsFileNodeKind {
            id: "export".into(),
            name: "Export".into(),
            descriptors: vec![
                binding.clone(),
                OsMediaGraphVfsFileNodeDescriptor {
                    id: "format".into(),
                    descriptor_kind_id: "text".into(),
                    label: "Format".into(),
                },
            ],
        },
    );
    OsMediaGraphVfsSchema {
        descriptor_kinds,
        file_node_kinds,
        descriptor_column_ids: vec!["binding".into(), "format".into()],
    }
}

pub fn os_media_graph_vfs_instance_id(node_id: &str) -> Option<String> {
    let captures = regex_lite(node_id, r"^inst:([^:]+)(?::|$)");
    captures
}

pub fn os_media_graph_vfs_instance_folder_id(instance_id: &str) -> String {
    format!("inst:{instance_id}")
}

pub fn os_media_graph_vfs_source_id(instance_id: &str) -> String {
    format!("inst:{instance_id}:source")
}

pub fn os_media_graph_vfs_inputs_folder_id(instance_id: &str) -> String {
    format!("inst:{instance_id}:inputs")
}

pub fn os_media_graph_vfs_outputs_folder_id(instance_id: &str) -> String {
    format!("inst:{instance_id}:outputs")
}

pub fn os_media_graph_vfs_input_port_id(instance_id: &str, port_spec_id: &str) -> String {
    format!("inst:{instance_id}:input:{port_spec_id}")
}

pub fn os_media_graph_vfs_export_id(
    instance_id: &str,
    port_spec_id: &str,
    format: &OsMediaFormat,
) -> String {
    format!("inst:{instance_id}:export:{port_spec_id}:{}", format.as_str())
}

pub fn os_media_graph_vfs_import_id(instance_id: &str, format: &OsMediaFormat) -> String {
    format!("inst:{instance_id}:import:{}", format.as_str())
}

fn regex_lite(input: &str, pattern: &str) -> Option<String> {
    if pattern == r"^inst:([^:]+)(?::|$)" {
        let rest = input.strip_prefix("inst:")?;
        let instance_id = rest.split(':').next()?;
        if instance_id.is_empty() {
            None
        } else {
            Some(instance_id.to_string())
        }
    } else {
        None
    }
}

/// @emoji 📁 Lists VFS children for one media graph folder node.
pub fn list_os_media_graph_vfs_children(
    parent_id: &str,
    instances: &[OsAppInstance],
    graph: &OsMediaGraph,
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
) -> Vec<OsMediaGraphVfsNodeRecord> {
    if parent_id == OS_MEDIA_GRAPH_VFS_ROOT_ID {
        return instances
            .iter()
            .map(|instance| {
                let registration = os_app_registration(&instance.program_id, &instance.app_id);
                OsMediaGraphVfsNodeRecord {
                    id: os_media_graph_vfs_instance_folder_id(&instance.id),
                    file_node_kind_id: "instance".into(),
                    name: format!(
                        "{} ({}.{}))",
                        instance.label, instance.program_id, instance.app_id
                    ),
                    path: format!("/{}", instance.label),
                    parent_id: Some(OS_MEDIA_GRAPH_VFS_ROOT_ID.into()),
                    has_children: true,
                    icon: registration
                        .as_ref()
                        .map(|entry| entry.component_kind.clone()),
                    navigate_uri: None,
                    descriptor_values: HashMap::from([("binding".into(), instance.yields.clone())]),
                }
            })
            .collect();
    }
    let Some(instance_id) = os_media_graph_vfs_instance_id(parent_id) else {
        return Vec::new();
    };
    let Some(instance) = instances.iter().find(|entry| entry.id == instance_id) else {
        return Vec::new();
    };
    let registration = os_app_registration(&instance.program_id, &instance.app_id);
    if parent_id == os_media_graph_vfs_instance_folder_id(&instance_id) {
        return vec![
            OsMediaGraphVfsNodeRecord {
                id: os_media_graph_vfs_source_id(&instance_id),
                file_node_kind_id: "source".into(),
                name: "source.json".into(),
                path: format!("/{}/source.json", instance.label),
                parent_id: Some(parent_id.into()),
                has_children: false,
                icon: Some("json".into()),
                navigate_uri: Some(format!("os://instance/{}", instance.id)),
                descriptor_values: HashMap::from([(
                    "binding".into(),
                    registration
                        .as_ref()
                        .map(|entry| entry.source_format.clone())
                        .unwrap_or_else(|| instance.yields.clone()),
                )]),
            },
            OsMediaGraphVfsNodeRecord {
                id: os_media_graph_vfs_inputs_folder_id(&instance_id),
                file_node_kind_id: "folder".into(),
                name: "inputs".into(),
                path: format!("/{}/inputs", instance.label),
                parent_id: Some(parent_id.into()),
                has_children: registration.as_ref().map(|entry| !entry.inputs.is_empty()).unwrap_or(false)
                    || bindings.iter().any(|binding| binding.instance_id == instance_id),
                icon: Some("folder-input".into()),
                navigate_uri: None,
                descriptor_values: HashMap::new(),
            },
            OsMediaGraphVfsNodeRecord {
                id: os_media_graph_vfs_outputs_folder_id(&instance_id),
                file_node_kind_id: "folder".into(),
                name: "outputs".into(),
                path: format!("/{}/outputs", instance.label),
                parent_id: Some(parent_id.into()),
                has_children: registration.as_ref().map(|entry| !entry.outputs.is_empty()).unwrap_or(false),
                icon: Some("folder-output".into()),
                navigate_uri: None,
                descriptor_values: HashMap::new(),
            },
        ];
    }
    if parent_id == os_media_graph_vfs_inputs_folder_id(&instance_id) {
        let mut rows = Vec::new();
        if let Some(registration) = registration.as_ref() {
            for spec in &registration.inputs {
                rows.push(OsMediaGraphVfsNodeRecord {
                    id: os_media_graph_vfs_input_port_id(&instance_id, &spec.id),
                    file_node_kind_id: "input".into(),
                    name: spec.id.clone(),
                    path: format!("/{}/inputs/{}", instance.label, spec.id),
                    parent_id: Some(parent_id.into()),
                    has_children: false,
                    icon: Some("plug".into()),
                    navigate_uri: None,
                    descriptor_values: HashMap::from([("binding".into(), spec.resource_kind.clone())]),
                });
            }
        }
        for binding in bindings
            .iter()
            .filter(|entry| entry.instance_id == instance_id)
        {
            let parameter = parameters
                .iter()
                .find(|entry| match entry {
                    crate::instance::OsParameter::Numeric { id, .. }
                    | crate::instance::OsParameter::Categorical { id, .. }
                    | crate::instance::OsParameter::Toggle { id, .. }
                    | crate::instance::OsParameter::Text { id, .. } => id == &binding.parameter_id,
                });
            rows.push(OsMediaGraphVfsNodeRecord {
                id: os_media_graph_vfs_input_port_id(&instance_id, &format!("param.{}", binding.parameter_id)),
                file_node_kind_id: "input".into(),
                name: parameter
                    .map(|entry| match entry {
                        crate::instance::OsParameter::Numeric { name, .. }
                        | crate::instance::OsParameter::Categorical { name, .. }
                        | crate::instance::OsParameter::Toggle { name, .. }
                        | crate::instance::OsParameter::Text { name, .. } => name.clone(),
                    })
                    .unwrap_or_else(|| binding.field_path.clone()),
                path: format!("/{}/inputs/param.{}", instance.label, binding.parameter_id),
                parent_id: Some(parent_id.into()),
                has_children: false,
                icon: Some("sliders-horizontal".into()),
                navigate_uri: None,
                descriptor_values: HashMap::from([(
                    "binding".into(),
                    parameter
                        .map(|entry| match entry {
                            crate::instance::OsParameter::Numeric { name, .. }
                            | crate::instance::OsParameter::Categorical { name, .. }
                            | crate::instance::OsParameter::Toggle { name, .. }
                            | crate::instance::OsParameter::Text { name, .. } => name.clone(),
                        })
                        .unwrap_or_else(|| binding.parameter_id.clone()),
                )]),
            });
        }
        let descriptor = crate::registry::os_resource_descriptor(&instance.yields);
        for format in required_os_media_import_formats(&descriptor.dimension, os_resource_media_capability(&descriptor.kind)) {
            let ext = format.as_str();
            rows.push(OsMediaGraphVfsNodeRecord {
                id: os_media_graph_vfs_import_id(&instance_id, &format),
                file_node_kind_id: "import".into(),
                name: format!("import.{ext}"),
                path: format!("/{}/inputs/import.{ext}", instance.label),
                parent_id: Some(parent_id.into()),
                has_children: false,
                icon: Some(ext.into()),
                navigate_uri: Some(format!("os://import/{}/{}/{}", instance.id, descriptor.kind, format.as_str())),
                descriptor_values: HashMap::from([("format".into(), format.as_str().into())]),
            });
        }
        return rows;
    }
    if parent_id == os_media_graph_vfs_outputs_folder_id(&instance_id) {
        let descriptor = crate::registry::os_resource_descriptor(&instance.yields);
        let formats = required_os_media_export_formats(&descriptor.dimension, os_resource_media_capability(&descriptor.kind));
        let mut rows = Vec::new();
        if let Some(registration) = registration.as_ref() {
            for spec in &registration.outputs {
                for format in &formats {
                    let ext = os_media_export_extension_for_format(format);
                    rows.push(OsMediaGraphVfsNodeRecord {
                        id: os_media_graph_vfs_export_id(&instance_id, &spec.id, format),
                        file_node_kind_id: "export".into(),
                        name: format!("{}.{}", spec.id, ext),
                        path: format!("/{}/outputs/{}.{}", instance.label, spec.id, ext),
                        parent_id: Some(parent_id.into()),
                        has_children: false,
                        icon: Some(ext.into()),
                        navigate_uri: Some(format!(
                            "os://export/{}/{}/{}",
                            instance.id,
                            spec.id,
                            format.as_str()
                        )),
                        descriptor_values: HashMap::from([
                            ("binding".into(), spec.resource_kind.clone()),
                            ("format".into(), format.as_str().into()),
                        ]),
                    });
                }
            }
        }
        return rows;
    }
    let _ = graph;
    let _ = parameter_id_from_port_id;
    let _ = media_port_spec_id;
    let _ = os_app_primary_output_kind;
    Vec::new()
}
//#endregion 🔖MediaGraphVfs

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instance::OsDocumentRef;
    use crate::registry::{merge_os_program_definition, os_baseline_resource, OsPlatformAppInput, OsPlatformInput};

    #[test]
    fn validates_empty_media_graph() {
        assert!(validate_media_graph(&empty_media_graph()).ok);
    }

    #[test]
    fn export_coverage_accepts_registered_handlers() {
        for descriptor in crate::registry::list_os_resource_descriptors() {
            for format in required_os_media_export_formats(&descriptor.dimension, os_resource_media_capability(&descriptor.kind)) {
                register_os_media_export_handler(&descriptor.kind, format, |_| {
                    Ok(OsMediaExportResult {
                        data: "export".into(),
                        mime_type: "application/octet-stream".into(),
                        file_name: "export.bin".into(),
                        encoding: None,
                    })
                });
            }
        }
        assert!(assert_os_media_export_coverage().is_ok());
    }

    #[test]
    fn import_coverage_accepts_registered_handlers() {
        for descriptor in crate::registry::list_os_resource_descriptors() {
            for format in required_os_media_import_formats(&descriptor.dimension, os_resource_media_capability(&descriptor.kind)) {
                register_os_media_import_handler(&descriptor.kind, format, |_| Ok(serde_json::json!({})));
            }
        }
        assert!(assert_os_media_import_coverage().is_ok());
    }

    #[test]
    fn svg_to_dwg_round_trip_produces_a_polyline() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect x="1" y="1" width="4" height="4"/></svg>"#;
        let bytes = crate::media_export_raster::svg_to_dwg_bytes(svg).expect("svg to dwg");
        let drawing = semio_framework_core::dwg_from_bytes(&bytes).expect("dwg from bytes");
        assert!(!drawing.entities.is_empty());
    }

    #[test]
    fn mesh_dwg_registrar_round_trips_a_box() {
        use base64::Engine;
        crate::media_export_raster::register_mesh_dwg_export_handler("3d.__dwg_test", "box", |_| Ok(semio_framework_plugin::mesh_from_kind("box")));
        let result = export_handlers()
            .lock()
            .expect("lock")
            .get(&os_media_export_key("3d.__dwg_test", &OsMediaFormat::Dwg))
            .expect("dwg handler registered")(&serde_json::json!({}))
        .expect("export dwg");
        let bytes = base64::engine::general_purpose::STANDARD.decode(result.data).expect("decode base64");
        let drawing = semio_framework_core::dwg_from_bytes(&bytes).expect("dwg from bytes");
        assert!(!drawing.entities.is_empty());
    }

    #[test]
    fn mesh_exporter_registrar_round_trips_a_box_through_glb() {
        use base64::Engine;
        crate::media_export_raster::register_mesh_exporter(
            "3d.__mesh_exporter_test",
            "box",
            |_| Ok(semio_framework_plugin::mesh_from_kind("box")),
            Box::new(semio_framework_plugin::GlbExporter),
        );
        let result = export_handlers()
            .lock()
            .expect("lock")
            .get(&os_media_export_key("3d.__mesh_exporter_test", &OsMediaFormat::Glb))
            .expect("glb handler registered")(&serde_json::json!({}))
        .expect("export glb");
        let bytes = base64::engine::general_purpose::STANDARD.decode(result.data).expect("decode base64");
        let mesh = semio_framework_core::mesh_from_glb(&bytes).expect("glb decodes back to a mesh");
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn mesh_importer_registrar_round_trips_a_box_through_obj() {
        crate::media_export_raster::register_mesh_importer(
            "3d.__mesh_importer_test",
            |mesh| Ok(serde_json::json!({ "vertexCount": mesh.vertex_count() })),
            Box::new(semio_framework_plugin::ObjImporter),
        );
        let obj_bytes = semio_framework_core::mesh_to_obj(&semio_framework_plugin::mesh_from_kind("box"), "box").into_bytes();
        let handlers = import_handlers().lock().expect("lock");
        let handler = handlers
            .get(&os_media_export_key("3d.__mesh_importer_test", &OsMediaFormat::Obj))
            .expect("obj handler registered");
        let document = handler(&obj_bytes).expect("import obj");
        assert!(document["vertexCount"].as_u64().expect("vertex count") > 0);
    }

    #[test]
    fn solid_exporter_and_importer_registrars_round_trip_a_box_through_step() {
        let mut kernel = kernel_3d_brepkit::BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).expect("box");
        crate::media_export_raster::register_solid_exporter("3d.__solid_test", Box::new(kernel_3d_brepkit::StepSolidExporter));
        crate::media_export_raster::register_solid_importer("3d.__solid_test", Box::new(kernel_3d_brepkit::StepSolidImporter));
        assert!(crate::media_export_raster::solid_exporter_for("3d.__solid_test", &OsMediaFormat::Step));
        let bytes = crate::media_export_raster::export_registered_solid("3d.__solid_test", &OsMediaFormat::Step, &kernel, &[solid], 0.1).expect("export step");
        assert!(!bytes.is_empty());
        let imported = crate::media_export_raster::import_registered_solid("3d.__solid_test", &OsMediaFormat::Step, &mut kernel, &bytes, 0.1).expect("import step");
        assert!(!imported.is_empty());
    }

    #[test]
    fn flow_fixture_projects_neuron_preview() {
        let mut resources = HashMap::new();
        resources.insert(
            "draw".into(),
            os_baseline_resource("2d.drawing", "draw.document", "draw"),
        );
        let platform = OsPlatformInput {
            id: "draw".into(),
            name: "Draw".into(),
            api_version: "1".into(),
            apps: vec![OsPlatformAppInput {
                id: "draw".into(),
                label: "Draw".into(),
                document: vec!["semio".into(), "draw".into()],
                controller_id: "draw-play".into(),
                modes: vec![],
                default_mode_id: None,
            }],
        };
        merge_os_program_definition("draw", &platform, &resources).expect("merge");
        let registration = os_app_registration("draw", "draw").expect("registration");
        let instance = OsAppInstance {
            id: "app-1".into(),
            program_id: "draw".into(),
            app_id: "draw".into(),
            label: "Draw".into(),
            yields: os_app_primary_output_kind(&registration),
            document: OsDocumentRef { document_id: "doc-app-1".into(), schema: "draw.document".into() },
        };
        let mut graph = empty_media_graph();
        graph.nodes.push(media_graph_node_for_instance(
            &instance,
            &registration,
            &MediaGraphPosition { x: 0.0, y: 0.0 },
            "node-1",
        ));
        let fixture = os_media_graph_to_flow_fixture(&graph, &[instance.clone()], &OsMediaGraphCamera::default());
        assert_eq!(fixture["schema"], "flow.fixture");
        assert_eq!(fixture["widgets"][0]["preview"], true);
        assert_eq!(fixture["widgets"][0]["params"]["instanceId"], "app-1");
        assert_eq!(fixture["widgets"][0]["params"]["programId"], "draw");
        assert_eq!(fixture["widgets"][0]["params"]["appId"], "draw");
        let operators = build_os_media_flow_operator_infos(&graph, &[instance], &[]);
        assert_eq!(operators.len(), 1);
        assert_eq!(operators[0].id, "os.media.node.node-1");
        assert_eq!(operators[0].module, OS_MEDIA_FLOW_MODULE_ID);
        assert_eq!(operators[0].name, "Draw");
    }

    #[test]
    fn vfs_inputs_folder_lists_a_dwg_import_row_for_2d_kinds() {
        let mut resources = HashMap::new();
        resources.insert("draw".into(), os_baseline_resource("2d.drawing", "draw.document", "draw"));
        let platform = OsPlatformInput {
            id: "draw-vfs".into(),
            name: "Draw".into(),
            api_version: "1".into(),
            apps: vec![OsPlatformAppInput { id: "draw".into(), label: "Draw".into(), document: vec!["semio".into(), "draw".into()], controller_id: "draw-play".into(), modes: vec![], default_mode_id: None }],
        };
        merge_os_program_definition("draw-vfs", &platform, &resources).expect("merge");
        let registration = os_app_registration("draw-vfs", "draw").expect("registration");
        let instance = OsAppInstance {
            id: "app-vfs-1".into(),
            program_id: "draw-vfs".into(),
            app_id: "draw".into(),
            label: "Draw".into(),
            yields: os_app_primary_output_kind(&registration),
            document: OsDocumentRef { document_id: "doc-app-vfs-1".into(), schema: "draw.document".into() },
        };
        let graph = empty_media_graph();
        let inputs_folder = os_media_graph_vfs_inputs_folder_id(&instance.id);
        let rows = list_os_media_graph_vfs_children(&inputs_folder, std::slice::from_ref(&instance), &graph, &[], &[]);
        let import_row = rows.iter().find(|row| row.file_node_kind_id == "import").expect("import row present");
        assert_eq!(import_row.name, "import.dwg");
        assert_eq!(import_row.navigate_uri, Some(format!("os://import/{}/2d.drawing/dwg", instance.id)));
    }

    fn media_node(id: &str, instance_id: &str, x: f64, y: f64) -> OsMediaGraphNode {
        OsMediaGraphNode {
            id: id.into(),
            instance_id: instance_id.into(),
            x,
            y,
            width: 160.0,
            height: 72.0,
            inputs: vec![OsMediaPort { id: format!("{instance_id}:in"), resource_kind: "2d.drawing".into(), direction: "in".into() }],
            outputs: vec![OsMediaPort { id: format!("{instance_id}:out"), resource_kind: "2d.drawing".into(), direction: "out".into() }],
        }
    }

    #[test]
    fn flow_fixture_round_trips_camera_and_diffs_back_to_ops() {
        let mut graph = empty_media_graph();
        graph.nodes.push(media_node("node-1", "app-1", 40.0, 80.0));
        graph.nodes.push(media_node("node-2", "app-2", 300.0, 80.0));
        graph.edges.push(OsMediaGraphEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "app-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "app-2:in".into(),
        });
        let camera = OsMediaGraphCamera { x: 12.0, y: -8.0, zoom: 1.5 };
        let fixture = os_media_graph_to_flow_fixture(&graph, &[], &camera);
        assert_eq!(fixture["camera"]["x"], 12.0);
        assert_eq!(fixture["camera"]["zoom"], 1.5);
        let unchanged = apply_flow_fixture_to_os_media_graph(&graph, &fixture.to_string());
        assert!(unchanged.is_empty());
        let mut moved = fixture.clone();
        moved["layout"]["node-1"] = json!({ "x": 220.0, "y": 156.0 });
        let ops = apply_flow_fixture_to_os_media_graph(&graph, &moved.to_string());
        assert_eq!(ops, vec![OsOp::MoveMediaNode { node_id: "node-1".into(), x: 140.0, y: 120.0 }]);
    }

    #[test]
    fn flow_fixture_diff_connects_disconnects_and_removes() {
        let mut graph = empty_media_graph();
        graph.nodes.push(media_node("node-1", "app-1", 0.0, 0.0));
        graph.nodes.push(media_node("node-2", "app-2", 200.0, 0.0));
        graph.edges.push(OsMediaGraphEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "app-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "app-2:in".into(),
        });
        let mut fixture = os_media_graph_to_flow_fixture(&graph, &[], &OsMediaGraphCamera::default());
        fixture["synapses"] = json!([
            { "id": "", "from": "node-2", "fromPort": "app-2:out", "to": "node-1", "toPort": "app-1:in" }
        ]);
        let ops = apply_flow_fixture_to_os_media_graph(&graph, &fixture.to_string());
        assert!(matches!(
            &ops[0],
            OsOp::ConnectMediaPorts { edge } if edge.source_node_id == "node-2" && edge.target_port_id == "app-1:in" && !edge.id.is_empty()
        ));
        assert!(ops.contains(&OsOp::DisconnectMediaEdge { edge_id: "edge-1".into() }));
        let mut removal = os_media_graph_to_flow_fixture(&graph, &[], &OsMediaGraphCamera::default());
        removal["widgets"] = json!([{ "id": "node-1" }]);
        removal["synapses"] = json!([]);
        let removal_ops = apply_flow_fixture_to_os_media_graph(&graph, &removal.to_string());
        assert!(removal_ops.contains(&OsOp::RemoveAppInstance { instance_id: "app-2".into() }));
        assert!(!removal_ops.iter().any(|op| matches!(op, OsOp::DisconnectMediaEdge { .. })));
    }
}
//#endregion 🧪Tests
// #endregion media_graph
}

pub mod registry {
// #region registry
//! 🗂️ Plugin manifest registry and OS program/resource catalog.

use crate::instance::{media_port_id_for_spec, OsParameterFieldSpec};
use semio_framework_core::{AppDefinition, ModeDefinition, ProgramDefinition, WindowKindDefinition};
use ui_wgpu::SurfaceKind;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub type OsResourceKindId = String;

pub const OS_RESOURCE_KIND_IDS: &[&str] = &[
    "2d.note",
    "2d.drawing",
    "2d.raster",
    "2d.map",
    "2d.procedural",
    "2d.shooting",
    "2d.puzzle",
    "3d.puzzle",
    "5d.puzzle",
    "3d.procedural",
    "3d.cad",
    "computation.flow",
    "graph.trinity",
    "graph.dag",
    "text.document",
    "form.dictionary",
    "kit.compose",
    "presentation.deck",
    "3d.mesh",
    "catalogue.kinds",
    "3d.lowpoly",
    "computation.sequence",
    "2d.layout",
    "computation.imperative",
    "vcs.document",
    "parameter.value",
];

//#region 🔖ResourceDescriptors
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsResourceDescriptor {
    pub kind: OsResourceKindId,
    pub name: String,
    pub source_format: String,
    pub component_kind: String,
    pub dimension: String,
}

fn descriptor_presentation(kind: &str) -> OsResourceDescriptor {
    match kind {
        "2d.note" => OsResourceDescriptor {
            kind: kind.into(),
            name: "2D Note".into(),
            source_format: "note.document".into(),
            component_kind: "note".into(),
            dimension: "2d".into(),
        },
        "2d.drawing" => OsResourceDescriptor {
            kind: kind.into(),
            name: "2D Drawing".into(),
            source_format: "draw.document".into(),
            component_kind: "draw".into(),
            dimension: "2d".into(),
        },
        "2d.raster" => OsResourceDescriptor {
            kind: kind.into(),
            name: "2D Raster".into(),
            source_format: "raster.document".into(),
            component_kind: "raster".into(),
            dimension: "2d".into(),
        },
        "graph.dag" => OsResourceDescriptor {
            kind: kind.into(),
            name: "DAG".into(),
            source_format: "flow.dag".into(),
            component_kind: "dag".into(),
            dimension: "graph".into(),
        },
        "parameter.value" => OsResourceDescriptor {
            kind: kind.into(),
            name: "Parameter".into(),
            source_format: "parameter.value".into(),
            component_kind: "parameter".into(),
            dimension: "data".into(),
        },
        "text.document" => OsResourceDescriptor {
            kind: kind.into(),
            name: "Text Document".into(),
            source_format: "writer.document".into(),
            component_kind: "writer".into(),
            dimension: "text".into(),
        },
        "presentation.deck" => OsResourceDescriptor {
            kind: kind.into(),
            name: kind.into(),
            source_format: kind.into(),
            component_kind: "panel".into(),
            dimension: "2d".into(),
        },
        _ => OsResourceDescriptor {
            kind: kind.into(),
            name: kind.into(),
            source_format: kind.into(),
            component_kind: "panel".into(),
            dimension: descriptor_dimension_from_kind_prefix(kind).into(),
        },
    }
}

fn descriptor_dimension_from_kind_prefix(kind: &str) -> &'static str {
    if kind.starts_with("2d.") {
        "2d"
    } else if kind.starts_with("3d.") {
        "3d"
    } else if kind.starts_with("5d.") {
        "5d"
    } else {
        "unknown"
    }
}

/// @emoji 📚 Lists all known OS resource descriptors.
pub fn list_os_resource_descriptors() -> Vec<OsResourceDescriptor> {
    OS_RESOURCE_KIND_IDS
        .iter()
        .map(|kind| descriptor_presentation(kind))
        .collect()
}

/// @emoji 📚 Resolves presentation metadata for one resource kind.
pub fn os_resource_descriptor(kind: &str) -> OsResourceDescriptor {
    descriptor_presentation(kind)
}

/// @emoji 🔗 Returns whether two resource kinds are interchangeable.
pub fn resources_compatible(left: &str, right: &str) -> bool {
    left == right
}
//#endregion 🔖ResourceDescriptors

//#region 🔖ProgramRegistry
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsPortSpec {
    pub id: String,
    pub label: String,
    pub resource_kind: OsResourceKindId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsAppRegistration {
    pub id: String,
    pub label: String,
    pub document: Vec<String>,
    pub controller_id: String,
    pub inputs: Vec<OsPortSpec>,
    pub outputs: Vec<OsPortSpec>,
    pub source_format: String,
    pub component_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_fields: Vec<OsParameterFieldSpec>,
    pub modes: Vec<ModeDefinition>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsProgramDefinition {
    pub id: String,
    pub name: String,
    pub api_version: String,
    pub apps: Vec<OsAppRegistration>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsPlatformAppInput {
    pub id: String,
    pub label: String,
    pub document: Vec<String>,
    pub controller_id: String,
    pub modes: Vec<ModeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsPlatformInput {
    pub id: String,
    pub name: String,
    #[serde(default = "default_api_version")]
    pub api_version: String,
    pub apps: Vec<OsPlatformAppInput>,
}

fn default_api_version() -> String {
    "1".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsAppResourceSpec {
    pub inputs: Vec<OsPortSpec>,
    pub outputs: Vec<OsPortSpec>,
    pub source_format: String,
    pub component_kind: String,
    pub modes: Vec<ModeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_fields: Vec<OsParameterFieldSpec>,
}

pub fn os_out_port(resource_kind: &str, id: &str, label: &str) -> OsPortSpec {
    OsPortSpec {
        id: id.into(),
        label: label.into(),
        resource_kind: resource_kind.into(),
        required: None,
    }
}

pub fn os_in_port(resource_kind: &str, id: &str, label: &str, required: bool) -> OsPortSpec {
    OsPortSpec {
        id: id.into(),
        label: label.into(),
        resource_kind: resource_kind.into(),
        required: Some(required),
    }
}

pub fn os_app_primary_output_kind(registration: &OsAppRegistration) -> OsResourceKindId {
    registration
        .outputs
        .first()
        .map(|port| port.resource_kind.clone())
        .unwrap_or_else(|| "graph.dag".into())
}

pub fn os_baseline_resource(
    resource_kind: &str,
    source_format: &str,
    component_kind: &str,
) -> OsAppResourceSpec {
    OsAppResourceSpec {
        inputs: Vec::new(),
        outputs: vec![os_out_port(resource_kind, "out", "Out")],
        source_format: source_format.into(),
        component_kind: component_kind.into(),
        modes: vec![ModeDefinition {
            id: "edit".into(),
            label: "Edit".into(),
            tools: Vec::new(),
            layout_id: None,
            commands: Vec::new(),
        }],
        default_mode_id: None,
        parameter_fields: Vec::new(),
    }
}

static BUILTIN_PROGRAMS: LazyLock<Mutex<Vec<OsProgramDefinition>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));
static EXTENSION_PROGRAMS: LazyLock<Mutex<HashMap<String, OsProgramDefinition>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// @emoji 📚 Registers a built-in os program prepended to list_os_programs.
pub fn register_os_builtin_program(program: OsProgramDefinition) {
    let mut registry = BUILTIN_PROGRAMS.lock().expect("lock");
    if registry.iter().any(|entry| entry.id == program.id) {
        return;
    }
    registry.push(program);
}

/// @emoji 📚 Registers a fully materialized os program definition.
pub fn register_os_program_definition(program: OsProgramDefinition) {
    EXTENSION_PROGRAMS
        .lock()
        .expect("lock")
        .insert(program.id.clone(), program);
}

/// @emoji 🧩 Merges a platform definition into the os program registry with port metadata.
pub fn merge_os_program_definition(
    program_id: &str,
    definition: &OsPlatformInput,
    resource_by_app_id: &HashMap<String, OsAppResourceSpec>,
) -> Result<(), String> {
    let fallback_resource = resource_by_app_id
        .values()
        .next()
        .ok_or_else(|| format!("merge_os_program_definition requires resourceByAppId for {program_id}"))?
        .clone();
    let apps = definition
        .apps
        .iter()
        .map(|app| {
            let resource = resource_by_app_id
                .get(&app.id)
                .cloned()
                .unwrap_or_else(|| fallback_resource.clone());
            OsAppRegistration {
                id: app.id.clone(),
                label: app.label.clone(),
                document: app.document.clone(),
                controller_id: app.controller_id.clone(),
                inputs: resource.inputs,
                outputs: resource.outputs,
                source_format: resource.source_format,
                component_kind: resource.component_kind,
                parameter_fields: resource.parameter_fields,
                modes: if app.modes.is_empty() {
                    resource.modes
                } else {
                    app.modes.clone()
                },
                default_mode_id: app.default_mode_id.clone().or(resource.default_mode_id),
            }
        })
        .collect();
    register_os_program_definition(OsProgramDefinition {
        id: program_id.into(),
        name: definition.name.clone(),
        api_version: definition.api_version.clone(),
        apps,
    });
    Ok(())
}

/// @emoji 🌱 Seeds the extension registry from a resource map for tests and offline tooling.
pub fn seed_os_program_registry_from_resource_map(
    resource_by_program: &HashMap<String, HashMap<String, OsAppResourceSpec>>,
) {
    let mut registry = EXTENSION_PROGRAMS.lock().expect("lock");
    for (program_id, resources) in resource_by_program {
        if registry.contains_key(program_id) {
            continue;
        }
        let name = program_id
            .split('.')
            .map(|segment| {
                let mut chars = segment.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        let apps = resources
            .iter()
            .map(|(app_id, resource)| OsPlatformAppInput {
                id: app_id.clone(),
                label: {
                    let mut chars = app_id.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                },
                document: vec!["semio".into(), app_id.to_lowercase()],
                controller_id: format!("{}-play", program_id.replace('.', "-")),
                modes: resource.modes.clone(),
                default_mode_id: resource.default_mode_id.clone(),
            })
            .collect();
        let platform = OsPlatformInput {
            id: program_id.clone(),
            name,
            api_version: "1".into(),
            apps,
        };
        drop(registry);
        let _ = merge_os_program_definition(program_id, &platform, resources);
        registry = EXTENSION_PROGRAMS.lock().expect("lock");
    }
}

pub fn list_os_programs() -> Vec<OsProgramDefinition> {
    let builtins = BUILTIN_PROGRAMS.lock().expect("lock").clone();
    let extensions = EXTENSION_PROGRAMS
        .lock()
        .expect("lock")
        .values()
        .cloned()
        .collect::<Vec<_>>();
    builtins.into_iter().chain(extensions).collect()
}

pub fn os_program_by_id(program_id: &str) -> Option<OsProgramDefinition> {
    list_os_programs()
        .into_iter()
        .find(|program| program.id == program_id)
}

pub fn os_app_registration(program_id: &str, app_id: &str) -> Option<OsAppRegistration> {
    os_program_by_id(program_id)?.apps.into_iter().find(|app| app.id == app_id)
}

/// @emoji 🧩 Resolves the AppDefinition backing an embedded os app instance. Returns `None` if the
/// registration declares zero modes — every app must declare at least one, so an ad hoc "inject a
/// fake edit mode" fallback would just hide a mis-registered app instead of surfacing it. An embedded
/// os app instance renders through exactly one component surface, so this synthesizes the single
/// window kind that represents it rather than leaving `window_kinds` empty (now impossible).
pub fn resolve_os_app_definition(
    program_id: &str,
    app_id: &str,
) -> Option<AppDefinition> {
    let registration = os_app_registration(program_id, app_id)?;
    let program = os_program_by_id(program_id)?;
    let app = program.apps.iter().find(|entry| entry.id == app_id)?;
    let modes = semio_framework_core::Modes::try_from(app.modes.clone()).ok()?;
    let default_mode_id = app
        .default_mode_id
        .clone()
        .or_else(|| registration.default_mode_id.clone())
        .unwrap_or_else(|| modes.first().id.clone());
    let window_kinds = semio_framework_core::WindowKinds::one(WindowKindDefinition {
        id: registration.component_kind.clone(),
        label: registration.label.clone(),
        body_key: registration.component_kind.clone(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: None,
        options: ui_wgpu::WindowOptions::default(),
        actions: Vec::new(),
        tools: Vec::new(),
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    });
    Some(AppDefinition {
        id: registration.id,
        label: registration.label,
        document: registration.document,
        icon_id: None,
        controller_id: registration.controller_id,
        modes,
        default_mode_id,
        window_kinds,
        panel_tabs: Vec::new(),
        keybindings: Vec::new(),
        actions: Vec::new(),
        tools: Vec::new(),
        commands: Vec::new(),
        named_layouts: Vec::new(),
        default_layout: None,
        terminologies: Vec::new(),
        introduction: None,
        dialogs: Vec::new(),
    })
}

pub fn media_graph_node_ports_for_registration(
    instance_id: &str,
    registration: &OsAppRegistration,
) -> (Vec<crate::media_graph::OsMediaPort>, Vec<crate::media_graph::OsMediaPort>) {
    let inputs = registration
        .inputs
        .iter()
        .map(|spec| crate::media_graph::OsMediaPort {
            id: media_port_id_for_spec(instance_id, &spec.id, "in"),
            resource_kind: spec.resource_kind.clone(),
            direction: "in".into(),
        })
        .collect();
    let outputs = registration
        .outputs
        .iter()
        .map(|spec| crate::media_graph::OsMediaPort {
            id: media_port_id_for_spec(instance_id, &spec.id, "out"),
            resource_kind: spec.resource_kind.clone(),
            direction: "out".into(),
        })
        .collect();
    (inputs, outputs)
}
//#endregion 🔖ProgramRegistry

//#region 🔖PluginRegistry
pub struct PluginRegistry {
    apps: HashMap<String, AppDefinition>,
    programs: HashMap<String, ProgramDefinition>,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            apps: HashMap::new(),
            programs: HashMap::new(),
        }
    }

    pub fn register_app(&mut self, app: AppDefinition) {
        self.apps.insert(app.id.clone(), app);
    }

    pub fn register_program(&mut self, program: ProgramDefinition) {
        self.programs.insert(program.program_id.clone(), program);
    }

    pub fn find_app(&self, app_id: &str) -> Option<&AppDefinition> {
        self.apps.get(app_id)
    }

    pub fn find_program(&self, program_id: &str) -> Option<&ProgramDefinition> {
        self.programs.get(program_id)
    }

    pub fn apps(&self) -> Vec<AppDefinition> {
        self.apps.values().cloned().collect()
    }

    pub fn programs(&self) -> Vec<ProgramDefinition> {
        self.programs.values().cloned().collect()
    }
}
//#endregion 🔖PluginRegistry

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_program_definition_with_resource_map() {
        let mut resources = HashMap::new();
        resources.insert(
            "draw".into(),
            os_baseline_resource("2d.drawing", "draw.document", "draw"),
        );
        let mut by_program = HashMap::new();
        by_program.insert("draw".into(), resources);
        seed_os_program_registry_from_resource_map(&by_program);
        let registration = os_app_registration("draw", "draw").expect("registration");
        assert_eq!(registration.source_format, "draw.document");
    }
}
//#endregion 🧪Tests
// #endregion registry
}


#[cfg(not(target_arch = "wasm32"))]
pub use backbone::{open_file_studio_backbone, open_folder_studio_backbone};
pub use host::{
    apply_os_operation, create_empty_os_document, create_os_studio, default_os_projection,
    delete_os_studio, import_os_studio_from_json, list_os_studio_catalog_entries,
    load_os_studio_document, materialize_os_projection, os_document_from_json, os_document_to_json,
    seed_os_studio_catalog_if_empty,
    LoadedPlugin, OsBackbonePort, OsDiff, OsDocument, OsEnvelope,
    OsOp, OsProjection, OsStore, OsStudioCatalogEntry, OsVcs, PluginHost, PluginHotSwapEvent,
    PluginSupervisorState, OS_HOME_VFS_ROOT_ID, OS_STUDIO_BACKBONE_URI_PREFIX,
};
pub use instance::{
    apply_parameter_values_to_projection, create_default_os_parameter, create_os_document_id, create_os_id,
    is_parameter_port_id, media_port_id_for_spec, media_port_spec_id, os_parameter_types_compatible,
    os_parameter_value, parameter_id_from_port_id, parameter_port_id, patch_os_parameter,
    resolve_parameter_values_for_instance, set_json_pointer_value, materialize_os_app_instance_document_json,
    os_fixture_json, register_os_fixture_json, OsAppInstance, OsDocumentRef, OsInstanceState,
    OsParameter, OsParameterFieldBinding, OsParameterFieldSpec, OsParameterType,
    OS_PARAMETER_PORT_PREFIX,
};
pub use media_export_raster::{
    dwg_drawing_to_svg, rasterize_svg_to_png_base64, register_2d_export_handlers, register_dwg_import_handler,
    register_mesh_dwg_export_handler, register_mesh_dwg_import_handler, register_mesh_exporter, register_mesh_importer,
    register_solid_exporter, register_solid_importer, solid_exporter_for, export_registered_solid, import_registered_solid,
    svg_to_dwg_bytes,
};
pub use media_export_simple::{map_points_svg, pages_rects_svg, title_card_svg, wrap_svg};
pub use media_graph::{
    apply_flow_fixture_to_os_media_graph, assert_os_media_export_coverage, assert_os_media_import_coverage,
    empty_media_graph, export_os_app_instance_media, import_os_app_instance_media,
    list_os_media_graph_vfs_children, media_graph_node_for_instance, os_media_export_extension_for_format,
    os_media_graph_to_flow_fixture, os_media_graph_to_node_graph_payload, os_media_graph_vfs_export_id, os_media_graph_vfs_import_id, os_media_graph_vfs_instance_folder_id,
    build_os_media_flow_operator_infos, OsMediaFlowOperatorInfo, OsMediaGraphCamera, OsMediaNodeGraphPayload,
    os_media_graph_vfs_instance_id, os_media_graph_vfs_schema, os_media_graph_vfs_source_id,
    os_media_neuron_kind_for_node, register_os_media_export_handler, register_os_media_import_handler,
    required_os_media_export_formats, required_os_media_import_formats, os_resource_media_capability,
    sync_media_graph_parameter_ports, validate_media_graph, MediaGraphPosition, MediaGraphValidation, OsMediaCapability,
    OsMediaFormat, OsMediaExportResult, OsMediaGraph, OsMediaGraphEdge, OsMediaGraphNode,
    OsMediaGraphVfsNodeRecord, OsMediaGraphVfsSchema, OsMediaPort, ProgramRegistry,
    OS_MEDIA_FLOW_MODULE_ID, OS_MEDIA_GRAPH_SCHEMA, OS_MEDIA_GRAPH_VFS_ROOT_ID, OS_STUDIO_SCHEMA,
};
pub use registry::{
    list_os_programs, list_os_resource_descriptors, merge_os_program_definition, os_app_primary_output_kind,
    os_app_registration, os_baseline_resource, os_in_port, os_out_port, os_program_by_id,
    os_resource_descriptor, register_os_builtin_program, register_os_program_definition,
    resolve_os_app_definition, resources_compatible, seed_os_program_registry_from_resource_map,
    OsAppRegistration, OsAppResourceSpec, OsPlatformAppInput, OsPlatformInput, OsPortSpec,
    OsProgramDefinition, OsResourceDescriptor, OsResourceKindId, PluginRegistry,
    OS_RESOURCE_KIND_IDS,
};
pub use semio_framework_core::*;
pub use ui_wgpu::*;
pub use vcs::{
    document_backbone_ref, set_host_backbone_port, Author, Checkpoint, DocumentBackboneRef, DocumentVcsCommand,
    LocalStorageBackbonePort, MemoryBackbonePort, VcsError,
};
