    /// 🗃️ Mounts paged file work using the same registered import admission and Store route.
    pub async fn advance_original_reserved_natural_source(
        &mut self, input: &mut Option<semio_plugin_decl_media::original_media::natural_source::NaturalFilePagedSource>,
        meta: &ActionMeta, finished: &mut Option<semio_framework_job::Operation>, cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if finished.is_some() { return Ok(true); }
        if self.original_reserved_prepares() {
            let mut render = self.as_render();
            if !render.prepare_reserved_media_cache(cx).await? || !render.advance_reserved_natural_source(input, meta, cx)? {
                return Ok(false);
            }
        }
        self.advance_original_reserved_mount(semio_plugin_decl_app::tool_contract::NATURAL_SOURCE_TOOL_ID, meta, finished, cx).await
    }


