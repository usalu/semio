//! 🗂️ Plugin manifest registry.

use semio_framework_core::{AppDefinition, ProgramDefinition};
use std::collections::HashMap;

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
