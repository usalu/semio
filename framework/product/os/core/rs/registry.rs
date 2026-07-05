//! 🗂️ Plugin manifest registry and OS program/resource catalog.

use crate::instance::{media_port_id_for_spec, OsParameterFieldSpec};
use semio_framework_core::{AppDefinition, ModeDefinition, ProgramDefinition};
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
        _ => OsResourceDescriptor {
            kind: kind.into(),
            name: kind.into(),
            source_format: kind.into(),
            component_kind: "panel".into(),
            dimension: "unknown".into(),
        },
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

/// @emoji 🧩 Resolves the AppDefinition backing an embedded os app instance.
pub fn resolve_os_app_definition(
    program_id: &str,
    app_id: &str,
) -> Option<AppDefinition> {
    let registration = os_app_registration(program_id, app_id)?;
    let program = os_program_by_id(program_id)?;
    let app = program.apps.iter().find(|entry| entry.id == app_id)?;
    Some(AppDefinition {
        id: registration.id,
        label: registration.label,
        icon_id: None,
        controller_id: registration.controller_id,
        modes: if app.modes.is_empty() {
            vec![ModeDefinition {
                id: "edit".into(),
                label: "Edit".into(),
                tools: Vec::new(),
            }]
        } else {
            app.modes.clone()
        },
        default_mode_id: app
            .default_mode_id
            .clone()
            .or(registration.default_mode_id),
        window_kinds: Vec::new(),
        panel_tabs: Vec::new(),
        keybindings: Vec::new(),
        named_layouts: Vec::new(),
        default_layout: None,
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
