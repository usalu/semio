//! 📐️ Architect play app command — applying a sector/project template to the program.

pub mod apply {
    use crate::editor::architect::behavior::apply_template;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::op::ProgramMutation;
    use crate::{EntityId, ProgramSnapshot};
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "apply-template")]
    pub struct ApplyTemplate {
        pub template_id: String,
    }

    pub fn handle(payload: &ApplyTemplate, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let template_id = EntityId(payload.template_id.clone());
        let template = program.templates.iter().find(|row| row.header.id == template_id).cloned().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("architect.template-missing"), format!("applyTemplate found no template \"{}\"", payload.template_id)))?;
        let mut scratch = program.clone();
        Ok(Emit::mutations(apply_template(&mut scratch, &template)))
    }
}
