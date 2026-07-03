//! 🎬 Media graph and program registry for OS studio composition.

use crate::instance::OsAppInstance;
use semio_framework_core::ProgramDefinition;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct MediaNode {
    pub id: String,
    pub program_id: String,
    pub label: String,
}

#[derive(Clone, Debug, Default)]
pub struct MediaGraph {
    pub nodes: Vec<MediaNode>,
    pub edges: Vec<(String, String)>,
}

impl MediaGraph {
    pub fn add_node(&mut self, node: MediaNode) {
        self.nodes.push(node);
    }

    pub fn connect(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.edges.push((from.into(), to.into()));
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProgramRegistry {
    programs: HashMap<String, ProgramDefinition>,
    instances: HashMap<String, OsAppInstance>,
}

impl ProgramRegistry {
    pub fn register_program(&mut self, program: ProgramDefinition) {
        self.programs.insert(program.program_id.clone(), program);
    }

    pub fn materialize_instance(&mut self, instance: OsAppInstance) {
        self.instances.insert(instance.id.clone(), instance);
    }

    pub fn get_program(&self, program_id: &str) -> Option<&ProgramDefinition> {
        self.programs.get(program_id)
    }

    pub fn get_instance(&self, instance_id: &str) -> Option<&OsAppInstance> {
        self.instances.get(instance_id)
    }
}
