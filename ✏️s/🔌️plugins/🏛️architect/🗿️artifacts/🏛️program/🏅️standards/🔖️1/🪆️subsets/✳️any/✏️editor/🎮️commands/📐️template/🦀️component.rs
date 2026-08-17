//! 📐️ Architect play app command — applying a sector/project template to the program.

pub mod apply {
    use crate::editor::architect::behavior::apply_template;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::{EntityId, ProgramSnapshot};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "apply-template")]
    pub struct ApplyTemplate {
        pub template_id: String,
    }

    pub fn handle(payload: &ApplyTemplate, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let template_id = EntityId(payload.template_id.clone());
        let Some(template) = program.templates.iter().find(|row| row.header.id == template_id).cloned() else {
            return Ok(Emit::default());
        };
        let mut scratch = program.clone();
        Ok(Emit::mutations(apply_template(&mut scratch, &template)))
    }
}
