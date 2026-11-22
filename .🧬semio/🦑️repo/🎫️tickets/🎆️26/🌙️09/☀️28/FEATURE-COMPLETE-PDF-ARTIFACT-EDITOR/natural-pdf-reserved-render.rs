    /// 📦 Moves the original paged source directly into the registered natural-file importer.
    pub fn advance_reserved_natural_source(
        &mut self, input: &mut Option<semio_plugin_decl_media::original_media::natural_source::NaturalFilePagedSource>,
        meta: &ActionMeta, cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_value::{RetainedCloneProgress, ValueError};
        use semio_plugin_decl_app::{NATURAL_FILE_PORT, tool_contract::NATURAL_SOURCE_TOOL_ID};
        if let Some(owner) = self.pending.reserved_preparation.as_mut() {
            owner.validate(self.stores.store.content_revision(), cx)?;
            if owner.job.is_some() { return Ok(true); }
            if owner.request.is_some() || owner.body.is_some() { return owner.advance_builder(cx); }
        }
        if !self.prepare_reserved_header(NATURAL_SOURCE_TOOL_ID, meta, cx)? { return Ok(false); }
        let owner = self.pending.reserved_preparation.as_mut().ok_or_else(|| ValueError::invariant("natural source requires reserved preparation"))?;
        match owner.input.as_ref() {
            None => {
                if !owner.stage_header(1, cx)? || !owner.copy_metadata(4, [NATURAL_FILE_PORT, ""], cx)?
                    || !owner.stage_header(4, cx)? { return Ok(false); }
                let Some(source) = input.as_ref().and_then(|source| source.original()) else { return Ok(false); };
                let maximum = A::natural_file_paged_maximum_bytes() as usize;
                if source.is_empty() || source.len() > maximum || owner.raw_wire.is_some() || owner.original_picker {
                    return Err(ValueError::invariant("paged natural source changed its reserved input or extent").into());
                }
                let copied_bytes = size_of::<Option<ArtifactReservedToolInput>>() + size_of_val(input)
                    + size_of::<Option<Vec<u8>>>() + size_of::<String>();
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() };
                if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() == 0
                    || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
                    return Ok(false);
                }
                let headers = owner.headers.as_mut().ok_or_else(|| ValueError::invariant("natural source port was already donated"))?;
                cx.consume_retained(progress)?;
                cx.consume_fuel(1);
                let port = std::mem::take(&mut headers[4]);
                *owner.input = Some(ArtifactReservedToolInput::NaturalPaged { port, source: input.take().expect("verified natural source") });
                *owner.raw_wire = Some(Vec::new());
                return Ok(false);
            }
            Some(ArtifactReservedToolInput::NaturalPaged { .. }) if input.is_none() => {}
            _ => return Err(ValueError::invariant("reserved natural source already owns another input").into()),
        }
        if !self.prepare_reserved_snapshot_read(cx)? || !self.prepare_reserved_wire_admission(cx)? { return Ok(false); }
        self.assemble_reserved_request(NATURAL_SOURCE_TOOL_ID, meta, &mut None, 1, cx)?;
        Ok(false)
    }


