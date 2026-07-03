//! 🧩 Declarative app builder and plugin trait.

use semio_framework_core::{
    AppDefinition, CommandDescriptor, ExampleDefinition, Keybinding, ModeDefinition,
    PanelTabDefinition, PluginManifest, ProgramDefinition, UiNode, ViewState, WindowKindDefinition,
};
use serde_json::Value;
use std::collections::HashMap;

pub struct ModeSpec {
    pub id: String,
    pub label: String,
}

pub struct WindowKindSpec {
    pub id: String,
    pub label: String,
    pub body_key: String,
}

pub struct PanelTabSpec {
    pub id: String,
    pub label: String,
    pub group: String,
    pub body_key: String,
}

pub struct KeybindingSpec {
    pub keys: String,
    pub controller_id: String,
    pub command: String,
}

pub struct AppBuilder {
    id: String,
    label: String,
    icon_id: Option<String>,
    controller_id: String,
    modes: Vec<ModeSpec>,
    default_mode_id: Option<String>,
    window_kinds: Vec<WindowKindSpec>,
    panel_tabs: Vec<PanelTabSpec>,
    keybindings: Vec<KeybindingSpec>,
}

impl AppBuilder {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            controller_id: id.clone(),
            id,
            label: label.into(),
            icon_id: None,
            modes: Vec::new(),
            default_mode_id: None,
            window_kinds: Vec::new(),
            panel_tabs: Vec::new(),
            keybindings: Vec::new(),
        }
    }

    pub fn icon_id(mut self, icon_id: impl Into<String>) -> Self {
        self.icon_id = Some(icon_id.into());
        self
    }

    pub fn mode(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.modes.push(ModeSpec {
            id: id.into(),
            label: label.into(),
        });
        self
    }

    pub fn default_mode_id(mut self, id: impl Into<String>) -> Self {
        self.default_mode_id = Some(id.into());
        self
    }

    pub fn window_kind(mut self, id: impl Into<String>, label: impl Into<String>, body_key: impl Into<String>) -> Self {
        self.window_kinds.push(WindowKindSpec {
            id: id.into(),
            label: label.into(),
            body_key: body_key.into(),
        });
        self
    }

    pub fn panel_tab(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        group: impl Into<String>,
        body_key: impl Into<String>,
    ) -> Self {
        self.panel_tabs.push(PanelTabSpec {
            id: id.into(),
            label: label.into(),
            group: group.into(),
            body_key: body_key.into(),
        });
        self
    }

    pub fn keybinding(mut self, keys: impl Into<String>, command: impl Into<String>) -> Self {
        self.keybindings.push(KeybindingSpec {
            keys: keys.into(),
            controller_id: self.controller_id.clone(),
            command: command.into(),
        });
        self
    }

    pub fn build_definition(self) -> AppDefinition {
        let default_mode_id = self
            .default_mode_id
            .or_else(|| self.modes.first().map(|mode| mode.id.clone()));
        AppDefinition {
            id: self.id,
            label: self.label,
            icon_id: self.icon_id,
            controller_id: self.controller_id,
            modes: self
                .modes
                .into_iter()
                .map(|mode| ModeDefinition {
                    id: mode.id,
                    label: mode.label,
                })
                .collect(),
            default_mode_id,
            window_kinds: self
                .window_kinds
                .into_iter()
                .map(|window| WindowKindDefinition {
                    id: window.id,
                    label: window.label,
                    body_key: window.body_key,
                })
                .collect(),
            panel_tabs: self
                .panel_tabs
                .into_iter()
                .map(|tab| PanelTabDefinition {
                    id: tab.id,
                    label: tab.label,
                    group: tab.group,
                    body_key: tab.body_key,
                })
                .collect(),
            keybindings: self
                .keybindings
                .into_iter()
                .map(|binding| Keybinding {
                    keys: binding.keys,
                    command: CommandDescriptor {
                        controller_id: binding.controller_id,
                        command: binding.command,
                        args: None,
                    },
                })
                .collect(),
        }
    }
}

pub struct App {
    pub definition: AppDefinition,
    pub examples: Vec<ExampleDefinition>,
    pub program: Option<ProgramDefinition>,
}

impl App {
    pub fn builder(id: impl Into<String>, label: impl Into<String>) -> AppBuilder {
        AppBuilder::new(id, label)
    }

    pub fn from_builder(builder: AppBuilder) -> Self {
        Self {
            definition: builder.build_definition(),
            examples: Vec::new(),
            program: None,
        }
    }

    pub fn example(mut self, id: impl Into<String>, label: impl Into<String>, document_json: impl Into<String>) -> Self {
        self.examples.push(ExampleDefinition {
            id: id.into(),
            label: label.into(),
            document_json: document_json.into(),
        });
        self
    }

    pub fn program(mut self, program_id: impl Into<String>, label: impl Into<String>, yields: impl Into<String>) -> Self {
        self.program = Some(ProgramDefinition {
            program_id: program_id.into(),
            app_id: self.definition.id.clone(),
            label: label.into(),
            yields: yields.into(),
        });
        self
    }
}

pub trait PluginApp: Send {
    fn app_id(&self) -> &str;
    fn initial_document_json(&self) -> String;
    fn handle_command(&mut self, command: &str, args: Option<&Value>, document_json: &str, view_state: &ViewState) -> Vec<String>;
    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode;
}

pub struct AppInstance {
    pub id: u32,
    pub app: Box<dyn PluginApp>,
    pub document_json: String,
}

pub trait Plugin: Send {
    fn manifest(&self) -> PluginManifest;
    fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>>;
}

pub struct PluginBundle {
    pub manifest: PluginManifest,
    apps: HashMap<String, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>>,
}

impl PluginBundle {
    pub fn new(plugin_id: impl Into<String>, label: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            manifest: PluginManifest {
                plugin_id: plugin_id.into(),
                label: label.into(),
                version: version.into(),
                apps: Vec::new(),
                programs: Vec::new(),
                examples: Vec::new(),
            },
            apps: HashMap::new(),
        }
    }

    pub fn register_app(
        mut self,
        app: App,
        factory: impl Fn() -> Box<dyn PluginApp> + Send + 'static,
    ) -> Self {
        self.manifest.apps.push(app.definition);
        self.manifest.examples.extend(app.examples);
        if let Some(program) = app.program {
            self.manifest.programs.push(program);
        }
        self.apps
            .insert(self.manifest.apps.last().unwrap().id.clone(), Box::new(factory));
        self
    }

    pub fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {
        self.apps.get(app_id).map(|factory| factory())
    }
}

impl Plugin for PluginBundle {
    fn manifest(&self) -> PluginManifest {
        self.manifest.clone()
    }

    fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {
        PluginBundle::create_app(self, app_id)
    }
}
