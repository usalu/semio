//! 📐️ Architect play app command — applying a sector/project template to the program.

pub mod apply {
    use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::engine::template::apply_template;
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::{EntityId, Program};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "apply-template")]
    pub struct ApplyTemplate {
        pub template_id: String,
    }

    pub fn handle(payload: &ApplyTemplate, doc: &DocumentView<'_, Program>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let program = doc.projection;
        let template_id = EntityId(payload.template_id.clone());
        let Some(template) = program.templates.iter().find(|row| row.header.id == template_id).cloned() else {
            return Ok(Emit::default());
        };
        let mut scratch = program.clone();
        Ok(Emit::operations(apply_template(&mut scratch, &template)))
    }
}
