/// 🧭 Routes an authenticated natural source turn within one unpublished native lifetime.
impl<PA: semio_plugin_reactor_app::PluginApp> NativeLifetimeSlot<PA> {
    fn natural_source_operation(key: NativeCloseKey) -> NaturalSourceOperation {
        NaturalSourceOperation { operation: 1, generation: key.lifetime.guest_lifetime, instance: key.instance() }
    }

    /// 🏷️ Captures the exact natural launch marker after the app supplies its paged decoder declaration.
    pub fn arm_natural_source(
        &mut self, key: NativeCloseKey, declaration: NaturalSourceDeclaration<'static>,
        maximum: u32, selected: NaturalSourceOffer<'_>, cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.natural_source_launch.is_some() || self.natural_source.is_some() || declaration != selected.declaration
            || selected.extent == 0 || selected.extent > maximum || maximum > NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES
        {
            return Ok(false);
        }
        if !self.admit_command_origin(key, cx)? { return Ok(false); }
        let progress = semio_framework_value::RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<NaturalSourceLaunchGrant>>(), ..Default::default()
        };
        if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() == 0
            || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant())
        {
            return Ok(false);
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        self.natural_source_launch = Some(NaturalSourceLaunchGrant { declaration, maximum, extent: selected.extent });
        Ok(true)
    }

    pub fn prepare_natural_source(
        &mut self, key: NativeCloseKey, seq: u64, selected: NaturalSourceOffer<'_>,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<(NaturalSourceStatus, Option<NaturalSourceLoan<'static>>), semio_framework_job::OriginalTurnFailure> {
        let Some(launch) = self.natural_source_launch else { return Ok((NaturalSourceStatus::Stale, None)); };
        if selected.extent != launch.extent || selected.declaration != launch.declaration {
            return Ok((NaturalSourceStatus::Refused, None));
        }
        if !self.admit_command_origin(key, cx)? { return Ok((NaturalSourceStatus::Retry, None)); }
        let current = Self::natural_source_operation(key);
        let result = NaturalSourceGuest::prepare(current, true, launch.declaration, launch.maximum, seq, selected, &mut self.natural_source, cx)?;
        let loan = self.natural_source.as_ref().filter(|_| result == NaturalSourceStatus::Accepted).map(NaturalSourceGuest::loan);
        Ok((result, loan))
    }

    pub fn control_natural_source(
        &mut self, key: NativeCloseKey, fresh: bool, format_closed: bool, control: NaturalSourceControl,
        offer: NaturalSourceBegin<'_>, cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<NaturalSourceStatus, semio_framework_job::OriginalTurnFailure> {
        if self.natural_source.is_none() { return Ok(NaturalSourceStatus::Stale); }
        if !self.admit_command_origin(key, cx)? { return Ok(NaturalSourceStatus::Retry); }
        let source = self.natural_source.as_mut().expect("admitted natural source");
        let current = Self::natural_source_operation(key);
        match control {
            NaturalSourceControl::Begin => source.begin(offer, current, fresh, cx),
            NaturalSourceControl::Complete => source.complete(offer, current, fresh, cx),
            NaturalSourceControl::Cancel => source.cancel(offer, current, format_closed, cx),
        }
    }

    pub fn receive_natural_source_page(
        &mut self, key: NativeCloseKey, fresh: bool, incoming: &mut Option<(CommandPageCursor, FixedCommandPage)>,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<NaturalSourceStatus, semio_framework_job::OriginalTurnFailure> {
        if self.natural_source.is_none() { return Ok(NaturalSourceStatus::Stale); }
        if !self.admit_command_origin(key, cx)? { return Ok(NaturalSourceStatus::Retry); }
        let current = Self::natural_source_operation(key);
        self.natural_source.as_mut().expect("admitted natural source").stage_page(incoming, current, fresh, cx)
    }

    pub fn poll_natural_source_page(
        &mut self, key: NativeCloseKey, fresh: bool, cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<Option<u32>, semio_framework_job::OriginalTurnFailure> {
        if self.natural_source.is_none() || !self.admit_command_origin(key, cx)? { return Ok(None); }
        let current = Self::natural_source_operation(key);
        self.natural_source.as_mut().expect("admitted natural source").poll_page(current, fresh, cx)
    }
}

#[derive(Clone, Copy)]
struct NaturalSourceLaunchGrant {
    declaration: NaturalSourceDeclaration<'static>,
    maximum: u32,
    extent: u32,
}

#[derive(Clone, Copy)]
pub enum NaturalSourceControl { Begin, Complete, Cancel }
