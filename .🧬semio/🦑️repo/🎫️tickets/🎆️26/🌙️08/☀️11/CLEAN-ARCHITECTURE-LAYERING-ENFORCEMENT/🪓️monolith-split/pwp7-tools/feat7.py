import sys; sys.path.insert(0,'.')
from ext_text7 import *
TT_IMPL='''impl InstanceExtension for TimeTravelExtension {
    fn claims_action(action: &str) -> bool {
        is_time_travel_action_id(action)
    }

    fn reads_history(action: &str) -> bool {
        is_time_travel_action_id(action)
    }

    fn admitted_while_loading(action: &str) -> bool {
        action == HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID
    }

    async fn dispatch_action<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let mut ctx = TimeTravelCtx::of_extension(ctx);
        ctx.as_render().freeze_tool_machines();
        ctx.dispatch_time_travel_action(action, args, meta, identity, cx).await
    }

    fn reserved_action_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, action: &str) -> Option<Fault> {
        (time_travel.freezes_local_emits() && time_travel_freezes_reserved_action(action)).then(|| time_travel_frozen_fault(action))
    }

    fn freezes_document<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        time_travel.freezes_local_emits()
    }

    fn document_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, verb: &str) -> Option<Fault> {
        time_travel.freezes_local_emits().then(|| time_travel_frozen_fault(verb))
    }

    fn transaction_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, txn_id: &str) -> Option<Fault> {
        time_travel.freezes_local_emits().then(|| MediaTransfers::<A>::transaction_fault(FaultOrigin::Framework, semio_framework_time_travel::TIME_TRAVEL_FROZEN_CODE, format!("transaction {txn_id:?} would change the document while a history edit is open on this instance")))
    }

    fn has_pending_work<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &ExtensionRef<'_, A, M, X>) -> bool {
        TimeTravelRef::of_extension_ref(ctx).time_travel_has_pending_work()
    }

    fn holds_receipt<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        time_travel.has_command_receipt()
    }

    fn terminal_is_empty<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        time_travel.terminal_is_empty()
    }

    async fn drive_turn<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, retained: &mut RetainedCloneGrant, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, crate::PluginLifecycleFailure> {
        let mut ctx = TimeTravelCtx::of_extension(ctx);
        if !ctx.shared().time_travel_has_pending_work() {
            return Ok(false);
        }
        let grant = crate::plugin_runtime::original_plugin_turn_grant(ctx.mounted_policy.maintenance, *retained);
        ctx.drive_time_travel_turn(grant, identity, cx).await?;
        Ok(ctx.time_travel.has_command_receipt())
    }
}'''
TR_IMPL='''impl InstanceExtension for ToolRunExtension {
    fn claims_action(action: &str) -> bool {
        is_tool_run_action_id(action)
    }

    async fn dispatch_action<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, action: &str, args: Option<&DslValue>, meta: &ActionMeta, _identity: &mut EntityIdentityAuthority<'_>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        ToolRunCtx::of_extension(ctx).dispatch_tool_run_action(action, args, meta).await
    }

    fn mutation_refusal<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>, verb: &str) -> Option<Fault> {
        tool_runs.freezes_local_emits().then(|| tool_run_busy_fault(verb))
    }

    fn window_config_published<A: ArtifactApp>(_time_travel: &mut TimeTravelLedger<A>, tool_runs: &mut ToolRunLedger<A>) {
        tool_runs.note_window_config_published();
    }

    fn has_pending_work<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &ExtensionRef<'_, A, M, X>) -> bool {
        ToolRunRef::of_extension_ref(ctx).tool_run_has_pending_work()
    }

    fn holds_receipt<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>) -> bool {
        tool_runs.has_original_receipt()
    }

    fn terminal_is_empty<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>) -> bool {
        tool_runs.terminal_is_empty()
    }

    async fn drive_turn<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, retained: &mut RetainedCloneGrant, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, crate::PluginLifecycleFailure> {
        let mut ctx = ToolRunCtx::of_extension(ctx);
        if !ctx.shared().tool_run_has_pending_work() {
            return Ok(false);
        }
        let before = cx.retained_progress();
        ctx.drive_tool_run_turn(cx, identity).await?;
        if cx.retained_progress() != before {
            if !ctx.tool_runs.has_original_receipt() {
                let progress = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before, cx.retained_progress())?;
                ctx.tool_runs.retain_original_receipt(*retained, progress)?;
            }
            return Ok(true);
        }
        Ok(ctx.tool_runs.has_original_receipt())
    }

    fn render_body<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, body_key: &str, view_state: &ViewModel) -> Option<UiAssemblyResult<BuiltNode>> {
        if body_key != FRAMEWORK_TOOL_RUN_BODY_KEY {
            return None;
        }
        let (registry, locale) = (&ctx.tools.registry, view_state.locale);
        let ready_tool = [view_state.active_utility_id.as_deref(), view_state.active_tool_id.as_deref()].into_iter().flatten().find(|tool_id| registry.tool_run(tool_id).is_some());
        Some(ctx.tool_runs.panel(&registry.controller_id, locale, ready_tool, |tool_id| registry.tool_run(tool_id).map(|(label, _)| label.resolve(Terminology::Native, locale).to_string())))
    }

    fn after_app_render<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, body_key: &str, view_state: &ViewModel) -> Result<(), Fault> {
        ToolRunCtx::of_extension(ctx).inject_tool_run_trace_lane(body_key, view_state)
    }
}'''
def replace_block(text, start_marker, new):
    M=mask(text)
    i=text.index(start_marker); assert text.count(start_marker)==1
    ob=M.index('{',i); d=0
    for j in range(ob,len(M)):
        if M[j]=='{': d+=1
        elif M[j]=='}':
            d-=1
            if d==0: break
    return text[:i]+new+text[j+1:]
def sub1(text, old, new):
    assert text.count(old)==1,(text.count(old),old[:80])
    return text.replace(old,new)
def patch_tt(text):
    text=replace_block(text,'impl InstanceExtension for TimeTravelExtension {',TT_IMPL)
    pat='let VcsArtifactApp { store, time_travel, children, .. } = self;'
    assert text.count(pat)==3
    text=text.replace(pat,'let TimeTravelCtx { stores, time_travel, child_composition, .. } = self;\n        let (store, children) = (&mut stores.store, &mut child_composition.children);')
    text=sub1(text,'let VcsArtifactApp{store,time_travel,children,child_content_retirements,child_content_generation,..}=self;',
       'let TimeTravelCtx{stores,time_travel,child_composition,..}=self;\n        let (store,children,child_content_retirements,child_content_generation)=(&mut stores.store,&mut child_composition.children,&mut child_composition.child_content_retirements,&mut child_composition.child_content_generation);')
    text=sub1(text,'let VcsArtifactApp { store, time_travel, .. } = self;','let TimeTravelCtx { stores, time_travel, .. } = self;\n            let store = &mut stores.store;')
    return text
def patch_tr(text):
    text=replace_block(text,'impl InstanceExtension for ToolRunExtension {',TR_IMPL)
    text=sub1(text,'let VcsArtifactApp{tool_runs,children,..}=self;','let ToolRunCtx{tool_runs,child_composition,..}=self;\n        let children=&mut child_composition.children;')
    return text
