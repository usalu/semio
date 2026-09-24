P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
s = open(P).read()
def rep(old, new, count=1):
    global s
    n = s.count(old)
    assert n == count, (n, old[:140])
    s = s.replace(old, new)

# Validate falls through into the first real unit
rep("""            match std::mem::replace(&mut commit.stage, FrameworkReservedCommitStage::Validate) {
                FrameworkReservedCommitStage::Validate => {""", """            if matches!(commit.stage, FrameworkReservedCommitStage::Validate) {""")
rep("""                            commit.total = 2;
                            FrameworkReservedCommitStage::Route { pins: Vec::new() }
                        }
                    };
                    Ok(None)
                }
                FrameworkReservedCommitStage::CheckpointChildren""", """                            commit.total = 1;
                            FrameworkReservedCommitStage::Route { pins: Vec::new() }
                        }
                    };
            }
            match std::mem::replace(&mut commit.stage, FrameworkReservedCommitStage::Validate) {
                FrameworkReservedCommitStage::Validate => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stage"), format!("framework route '{}' re-entered its validation unit", commit.action))),
                FrameworkReservedCommitStage::CheckpointChildren""")
rep("""                            commit.total = keys.len() as u64 + 2;""", """                            commit.total = keys.len() as u64 + 1;""")

# G: test helpers
rep("""        app.complete_reserved_spawned_job_inner(job, output).await?.ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-missing"), "framework reserved spawn-job had no pending admit"))
    }""", """        settle_framework_reserved_commit(app, job, output).await
    }

    /// 🪜️ Most units one reserved commit may take in a settle law — one per composed child or undone
    /// edit, well above anything a fixture composes.
    pub const FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS: usize = 4_096;

    /// 🪜️ Hands a finished reserved spawn-job to `app`'s commit queue and drives the commit one unit
    /// per call — the same units the reactor's typed-operation continuation runs across turns — to its
    /// outcome.
    pub async fn settle_framework_reserved_commit<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, job: u64, output: Result<Vec<u8>, Fault>) -> Result<InvocationResult, Fault> {
        if !app.admit_framework_reserved_commit(job, output)? {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-missing"), "framework reserved spawn-job had no pending admit"));
        }
        for _ in 0..FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS {
            app.step_framework_reserved_commit().await?;
            if let Some(outcome) = app.take_framework_reserved_commit_outcome() {
                return outcome;
            }
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stall"), format!("framework reserved commit did not finish within {FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS} units")))
    }""")
rep("""        let settled = app.complete_reserved_spawned_job_inner(job, output).await?.ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-missing"), "framework reserved spawn-job had no pending admit"))?;""",
    """        let settled = settle_framework_reserved_commit(app, job, output).await?;""")

# H: Drop / close / conjunction
rep("""            while let Some((_, id)) = self.pending_reserved.next_id_from(0) {
                if let Some(pending) = self.pending_reserved.remove(id) {
                    pending.permit.finish();
                } else {
                    break;
                }
            }
        }""", """            while let Some((_, id)) = self.pending_reserved.next_id_from(0) {
                if let Some(pending) = self.pending_reserved.remove(id) {
                    pending.permit.finish();
                } else {
                    break;
                }
            }
            while let Some(commit) = self.reserved_commits.pop_front() {
                commit.permit.finish();
            }
        }""")
rep("""            if let Some((_, id)) = self.pending_reserved.next_id_from(0) {
                if let Some(pending) = self.pending_reserved.remove(id) {
                    pending.permit.finish();
                }
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }""", """            if let Some((_, id)) = self.pending_reserved.next_id_from(0) {
                if let Some(pending) = self.pending_reserved.remove(id) {
                    pending.permit.finish();
                }
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(commit) = self.reserved_commits.pop_front() {
                commit.permit.finish();
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.reserved_commit_outcome.take().is_some() {
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }""")
rep("""                && self.pending_reserved.is_empty()
                && self.media_closures.is_empty()""", """                && self.pending_reserved.is_empty()
                && self.reserved_commits.is_empty()
                && self.reserved_commit_outcome.is_none()
                && self.media_closures.is_empty()""")

# I: typed-op lane
rep("""        async fn advance_typed_operation_publication(&mut self) -> Result<(), Fault> {
            if self.tool_run_has_pending_work() {""", """        async fn advance_typed_operation_publication(&mut self) -> Result<(), Fault> {
            if !self.reserved_commits.is_empty() && self.reserved_commit_outcome.is_none() {
                return self.step_framework_reserved_commit().await;
            }
            if self.tool_run_has_pending_work() {""")
rep("""        fn has_pending_typed_operations(&self) -> bool {
            self.tool_run_has_pending_work()
                || !self.tool_operations.is_empty()""", """        fn has_pending_typed_operations(&self) -> bool {
            self.tool_run_has_pending_work()
                || !self.reserved_commits.is_empty()
                || self.reserved_commit_outcome.is_some()
                || !self.tool_operations.is_empty()""")
rep("""        fn has_runnable_typed_operations(&self) -> bool {
            self.tool_run_has_pending_work()
                || !self.tool_operations.is_empty()""", """        fn has_runnable_typed_operations(&self) -> bool {
            self.tool_run_has_pending_work()
                || !self.reserved_commits.is_empty()
                || self.reserved_commit_outcome.is_some()
                || !self.tool_operations.is_empty()""")

rep("""        /// 🕰️ Completes one host-driven framework-reserved spawn-job. Default is a no-op so only
        /// `VcsArtifactApp` owns the pending-admit registry.
        async fn complete_reserved_spawned_job(&mut self, _job: u64, _output: Result<Vec<u8>, Fault>) -> Result<Option<InvocationResult>, Fault> {
            Ok(None)
        }""", """        /// 🕰️ Hands one host-driven framework-reserved spawn-job to the app's reserved-commit queue,
        /// whose units the typed-operation continuation then drives across turns. Answers `false` for a
        /// job the app never admitted. Default is a no-op so only `VcsArtifactApp` owns the queue.
        fn admit_reserved_spawned_job(&mut self, _job: u64, _output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
            Ok(false)
        }
        /// 🧾️ Takes the result of the last finished reserved commit — the payload of one unsolicited
        /// `AppFrame::Invocation` (or `Error`), framed before that operation's `OperationCompleted`.
        fn take_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
            None
        }
        /// 📊️ Progress of the reserved commit this app is driving, if any.
        fn reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
            None
        }""")
rep("""        async fn complete_reserved_spawned_job(&mut self, job: u64, output: Result<Vec<u8>, Fault>) -> Result<Option<InvocationResult>, Fault> {
            self.complete_reserved_spawned_job_inner(job, output).await
        }""", """        fn admit_reserved_spawned_job(&mut self, job: u64, output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
            self.admit_framework_reserved_commit(job, output)
        }

        fn take_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
            self.take_framework_reserved_commit_outcome()
        }

        fn reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
            self.framework_reserved_commit_progress()
        }""")

# J: plugin_runtime
start = s.index("    /// 🕰️ Host-driven completion for one framework-reserved spawn-job.")
end = s.index("    /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): dispatches every `intents` entry")
s = s[:start] + """    /// 🕰️ Host-driven completion for one framework-reserved spawn-job: the job's output joins the app's
    /// reserved-commit queue and one typed-operation continuation unit runs at once. A commit that
    /// needs more units (composed children, revert walks) stays runnable and the reactor's
    /// continuation finishes it on this and later turns. A job the app never admitted is a no-op so
    /// fill and other isolated jobs keep their existing JobCompleted path.
    pub async fn plugin_complete_reserved_spawned_job<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, job: u64, output: Result<Vec<u8>, Fault>) -> PluginExchangeOutput {
        let admitted = with_instances_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            instance.app.admit_reserved_spawned_job(job, output)
        })
        .await;
        let advanced = match admitted {
            Ok(false) => return PluginExchangeOutput::default(),
            Ok(true) => {
                with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    advance_typed_operation_output(&mut instance.app, instance_id)
                })
                .await
            }
            Err(fault) => Err(fault),
        };
        match advanced {
            Ok(output) => output,
            Err(fault) => {
                let mut frames = Vec::new();
                push_app_fault(&mut frames, None, fault).await;
                let mut output = PluginExchangeOutput::default();
                for frame in frames.iter() {
                    output.frames.push(protocol::encode_app_frame(frame).await);
                }
                output
            }
        }
    }

""" + s[end:]

rep("""    fn advance_typed_operation_output_with_leftover<PA: PluginApp>(app: &mut PA, instance: u32) -> Result<(PluginExchangeOutput, Option<TypedOperationLeftover>), Fault> {
        resolve_ready(app.advance_typed_operation_publication())?;
        trace_typed_operation_slot_occupancy(app, instance);
        let mut output = PluginExchangeOutput::default();""", """    fn advance_typed_operation_output_with_leftover<PA: PluginApp>(app: &mut PA, instance: u32) -> Result<(PluginExchangeOutput, Option<TypedOperationLeftover>), Fault> {
        resolve_ready(app.advance_typed_operation_publication())?;
        trace_typed_operation_slot_occupancy(app, instance);
        let mut output = PluginExchangeOutput::default();
        match app.take_reserved_commit_outcome() {
            Some(Ok(result)) => {
                output.frames.push(resolve_ready(protocol::encode_app_frame(&protocol::AppFrame::Invocation {
                    in_reply_to: 0,
                    output: encode_wire_serialized(&result.output),
                    diagnostics: encode_wire_serialized(&result.diagnostics),
                    ui_scope: encode_wire_serialized(&result.ui_scope),
                    history_patch: encode_wire_serialized(&result.history_patch),
                    messages: Vec::new(),
                    mutations: encode_wire_serialized(&result.mutations),
                    inverse_group: encode_wire_serialized(&result.inverse_group),
                })));
                resolve_ready(push_invocation_side_frames(&mut output.effects, &mut output.events, &result));
            }
            Some(Err(fault)) => {
                let mut frames = Vec::new();
                resolve_ready(push_app_fault(&mut frames, None, fault));
                for frame in frames.iter() {
                    output.frames.push(resolve_ready(protocol::encode_app_frame(frame)));
                }
            }
            None => {}
        }""")
open(P, "w").write(s)

T = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs"
t = open(T).read()
old = "let output = semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_complete_reserved_spawned_job(runtime, instance, job, reserved_output));"
assert t.count(old) == 1
t = t.replace(old, "let output = crate::plugin_runtime::plugin_complete_reserved_spawned_job(runtime, instance, job, reserved_output).await;")
open(T, "w").write(t)
print("ok")
