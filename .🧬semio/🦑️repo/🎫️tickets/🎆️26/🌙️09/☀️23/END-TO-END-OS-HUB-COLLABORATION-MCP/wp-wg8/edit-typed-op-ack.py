import pathlib, re
root = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine")
p = root / "🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"
t = p.read_text()
def swap(old, new, count=1):
    global t
    assert t.count(old) == count, (t.count(old), old[:100])
    t = t.replace(old, new)
# 1. request variant
swap('''        AcknowledgeJobProgress {
            token: JobProgressPresentationToken,
        },
        AcknowledgeTypedOperationResult {
            token: TypedOperationResultToken,
        },
    }''', '''        AcknowledgeJobProgress {
            token: JobProgressPresentationToken,
        },
    }''')
swap('''                | Self::AcknowledgeJobProgress { .. }
                | Self::AcknowledgeTypedOperationResult { .. } => (0, 0),''', '''                | Self::AcknowledgeJobProgress { .. } => (0, 0),''')
swap('''                // 🎫️ `AcknowledgeTypedOperationResult` joins this arm for the same reason the others
                // are here: `command_credits` reports `(0, 0)` for it, so it owns no retained page or
                // byte claim and a shutdown slice retires it whole in one step.
                KernelRequest::AdvanceRetained { .. }
                | KernelRequest::MountProductReplay { .. }
                | KernelRequest::RetireProductReplay { .. }
                | KernelRequest::RetireProductReplayRefusal { .. }
                | KernelRequest::AdvanceProductReplay { .. }
                | KernelRequest::AcknowledgeTypedOperationResult { .. } => (true, 1, 0, 0),''', '''                KernelRequest::AdvanceRetained { .. }
                | KernelRequest::MountProductReplay { .. }
                | KernelRequest::RetireProductReplay { .. }
                | KernelRequest::RetireProductReplayRefusal { .. }
                | KernelRequest::AdvanceProductReplay { .. } => (true, 1, 0, 0),''')
swap('''                KernelRequest::AcknowledgeTypedOperationResult { token } => {
                    let _ = state.deliver_typed_operation_result_ack(token).await;
                    continue;
                }
''', '')
swap('''        async fn deliver_typed_operation_result_ack(&mut self, token: TypedOperationResultToken) -> Result<(), String> {
            let Some(&actor) = self.instances.get(&token.receiver) else {
                return Err("kernel: typed-operation ACK receiver is not registered".to_string());
            };
            let event = Event::Message { source: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(token.receiver.to_string()) }, payload: TypedOperationResultPage::encode_ack(token) };
            let _ = self.run_turn(actor, token.receiver, vec![event]).await?;
            Ok(())
        }

''', '')
# 2. client / outcome accessors
swap('''        pub(crate) fn acknowledge_typed_operation_result(&self, token: TypedOperationResultToken) -> bool {
            typed_operation_result_exchange().get().is_some_and(|exchange| exchange.acknowledge(token))
        }

        pub(crate) fn take_surface''', '''        pub(crate) fn take_surface''')
swap('''            install_mounted_typed_operation_result_exchange();
            global_client()''', '''            global_client()''')
swap('''        pub(crate) fn acknowledge_typed_operation_result(&self, token: TypedOperationResultToken) -> bool {
            typed_operation_result_exchange().get().is_some_and(|exchange| exchange.acknowledge(token))
        }

        /// 🐣️ Compiles''', '''        /// 🐣️ Compiles''')
# 3. exchange region -> page type only
start = t.index('''    /// 🎯️ Object-safe renderer boundary for one retained page and its exact ACK token.''')
end = t.index('''    //#endregion 📬️TypedOperationResultExchange''')
removed = t[start:end]
assert "fn publish_typed_operation_result_page" in removed and "wgpu-renderer-kernel-runtime-typed-operation-result-exchange" in removed
t = t[:start] + t[end:]
swap('''    //#region 📬️TypedOperationResultExchange
    pub(crate) const TYPED_OPERATION_RESULT_PAGE_BYTES''', '''    //#region 📬️TypedOperationResultPage
    /// 📬️ One typed-operation result page a guest publishes to its shell (`semio.typed-operation-page.v1`),
    /// and the exact ACK (`semio.typed-operation-ack.v1`) the guest waits for before it retires the page
    /// and continues the operation. The kernel thread acknowledges every page inside the turn settle
    /// that received it ([`KernelPoolState::run_turn`]), exactly as the React host acknowledges inside
    /// `settlePluginTurn` (`typedOperationAcknowledgements`), and hands the pages to the caller on
    /// [`ExchangeOutcome::typed_results`]. Before this, the page waited in a renderer exchange no
    /// native consumer ever read, so every reserved tool job (undo, redo, checkpoint, copy, paste…)
    /// stalled its guest in `MoreWork` (ticket 26/09/23 slice WG8, measured on block2d's undo).
    pub(crate) const TYPED_OPERATION_RESULT_PAGE_BYTES''')
swap('''    //#endregion 📬️TypedOperationResultExchange''', '''    //#endregion 📬️TypedOperationResultPage''')
# 4. outcome field
swap('''        pub effects: Vec<Effect>,
        pub command_ingress: semio_framework::kernel::CommandIngressStatus,
    }''', '''        pub effects: Vec<Effect>,
        pub command_ingress: semio_framework::kernel::CommandIngressStatus,
        /// 📬️ Every typed-operation result page this exchange's turns published to the shell, in
        /// publication order — each already acknowledged to the guest inside the settle.
        pub typed_results: Vec<TypedOperationResultPage>,
    }''')
swap('''            self.frames.is_empty() && self.effects.is_empty() && self.surfaces.is_empty() && matches!(self.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle)''', '''            self.frames.is_empty() && self.effects.is_empty() && self.surfaces.is_empty() && self.typed_results.is_empty() && matches!(self.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle)''')
swap('''            self.frames.extend(later.frames);
            self.effects.extend(later.effects);''', '''            self.frames.extend(later.frames);
            self.effects.extend(later.effects);
            self.typed_results.extend(later.typed_results);''')
# every ExchangeOutcome literal gains the field
n = t.count("command_ingress: semio_framework::kernel::CommandIngressStatus::Idle }")
t = t.replace("command_ingress: semio_framework::kernel::CommandIngressStatus::Idle }", "command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() }")
print("idle literals", n)
# 5. apply_turn_result
swap('''            let mut frames = Vec::new();
            let mut effects = Vec::new();
            for effect in result.effects {
                if let Effect::SendMessage { target: MessageEndpoint::Shell { instance: target_instance }, payload } = &effect {
                    if target_instance.0 == instance.to_string() {
                        if let Some(page) = TypedOperationResultPage::decode_guest_message(payload) {
                            if page.token.receiver == instance {
                                let queue = self.request_queue.clone();
                                let acknowledge = Arc::new(move |token| queue.try_push(KernelRequest::AcknowledgeTypedOperationResult { token }, Arc::new(ResponseSlot::default()), None).is_ok());
                                let _ = publish_typed_operation_result_page(page, acknowledge);
                                continue;
                            }
                        }''', '''            let mut frames = Vec::new();
            let mut effects = Vec::new();
            let mut typed_results = Vec::new();
            for effect in result.effects {
                if let Effect::SendMessage { target: MessageEndpoint::Shell { instance: target_instance }, payload } = &effect {
                    if target_instance.0 == instance.to_string() {
                        if let Some(page) = TypedOperationResultPage::decode_guest_message(payload).filter(|page| page.token.receiver == instance) {
                            typed_results.push(page);
                            continue;
                        }''')
swap('''            Ok(ExchangeOutcome { frames, surfaces, effects, command_ingress: result.command_ingress })''', '''            Ok(ExchangeOutcome { frames, surfaces, effects, command_ingress: result.command_ingress, typed_results })''')
# 6. continuation computed after apply
swap('''                Some(result) => {
                    let continuation = match (result.lifecycle_receipt, &result.status) {
                        (Some(receipt), _) => Some(vec![Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt })]),
                        (None, semio_framework::kernel::TurnStatus::MoreWork) if matches!(result.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle) && matches!(result.cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Idle) => Some(Vec::new()),
                        (None, _) => None,
                    };
                    self.apply_turn_result(actor, instance, result).await.map(|outcome| (outcome, continuation))
                }''', '''                Some(result) => {
                    let lifecycle_ack = result.lifecycle_receipt.map(|receipt| Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
                    let more_work = matches!(result.status, semio_framework::kernel::TurnStatus::MoreWork)
                        && matches!(result.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle)
                        && matches!(result.cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Idle);
                    let outcome = self.apply_turn_result(actor, instance, result).await?;
                    let owed: Vec<Event> = lifecycle_ack
                        .into_iter()
                        .chain(outcome.typed_results.iter().map(|page| Event::Message { source: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: TypedOperationResultPage::encode_ack(page.token) }))
                        .collect();
                    let continuation = if !owed.is_empty() {
                        Some(owed)
                    } else {
                        more_work.then(Vec::new)
                    };
                    Ok((outcome, continuation))
                }''')
# 7. run_turn: owed events cross one per turn
swap('''            while let Some(batch) = batches.pop_front() {
                let (mut outcome, mut continuation) = self.run_turn_once(actor, instance, batch, deadline).await?;
                let mut quiet = 0usize;
                while let Some(events) = continuation.take() {
                    if std::time::Instant::now() >= deadline || quiet >= run_turn_quiescent_continuations() {
                        break;
                    }
                    let (later, next) = self.run_turn_once(actor, instance, events, deadline).await?;
                    quiet = if later.carries_nothing() { quiet + 1 } else { 0 };
                    outcome.absorb(later)?;
                    continuation = next;
                }''', '''            while let Some(batch) = batches.pop_front() {
                let (mut outcome, continuation) = self.run_turn_once(actor, instance, batch, deadline).await?;
                let mut owed = run_turn_continuation_turns(continuation);
                let mut quiet = 0usize;
                while let Some(events) = owed.pop_front() {
                    if std::time::Instant::now() >= deadline || quiet >= run_turn_quiescent_continuations() {
                        break;
                    }
                    let (later, next) = self.run_turn_once(actor, instance, events, deadline).await?;
                    quiet = if later.carries_nothing() { quiet + 1 } else { 0 };
                    outcome.absorb(later)?;
                    owed.extend(run_turn_continuation_turns(next));
                }''')
swap('''    fn run_turn_quiescent_continuations() -> usize {''', '''    /// 🧵️ The turns one continuation takes: every owed event (a lifecycle ACK, a typed-operation ACK)
    /// on a turn of its own — a guest left owning ingress by the first must not receive the second in
    /// the same grant — or one empty-event turn for a guest that only answered `MoreWork`.
    fn run_turn_continuation_turns(continuation: Option<Vec<Event>>) -> std::collections::VecDeque<Vec<Event>> {
        match continuation {
            None => std::collections::VecDeque::new(),
            Some(events) if events.is_empty() => std::collections::VecDeque::from([Vec::new()]),
            Some(events) => events.into_iter().map(|event| vec![event]).collect(),
        }
    }

    fn run_turn_quiescent_continuations() -> usize {''')
p.write_text(t)
print("renderer refactored")
