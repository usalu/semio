import pathlib, sys
path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
start = text.index("        /// ⏯️ One turn for `actor`, driven to settle. A turn the shard cut on its wall grant answers\n")
end = text.index("        /// 🎯️ One granted turn, answering what the guest still owes:")
old = text[start:end]
assert old.count("async fn run_turn(") == 1, "run_turn anchor drift"
new = '''        /// ⏯️ One request's turn for `actor`, settled the way the React host settles one
        /// (`🔌️PluginRuntime`'s `settlePluginTurn`, ticket 26/09/23 slice WG8, B1):
        ///
        /// - a grant the shard ended with the guest mid-call ([`ShardOutcome::Preempted`]) is resumed
        ///   inside [`Self::run_turn_once`] before anything else reaches the actor, so this method never
        ///   hands a mid-flight guest back to its caller;
        /// - a settled turn that published an `ActorInstanceLifecycleReceipt` is acknowledged on the next
        ///   turn (an open the host never acknowledges is re-offered on every later turn);
        /// - a completed turn answering `MoreWork` is continued with empty-event turns while its command
        ///   and cold-pair ingress are idle — a non-idle ingress belongs to the caller's own page driver
        ///   ([`Self::exchange_commands`] observes every ingress status, so a settle that swallowed one
        ///   would resend a page the guest already owns: measured as block2d's `addHandleKind` spinning
        ///   `MoreWork` until the settle budget, ticket 26/09/18 slice G7w run 13);
        /// - it stops once [`RUN_TURN_QUIESCENT_CONTINUATIONS`] continuations in a row carried nothing,
        ///   because a guest waiting on a host round trip (a typed-operation ACK, a backbone reply) is
        ///   answering `MoreWork` for something no empty turn can deliver.
        ///
        /// 🧵️ Events cross ONE per granted turn and each settles before the next is submitted: the shard
        /// executes one event per guest turn anyway, and a batch's later envelopes must not reach a guest
        /// the first one left owning ingress. [`RUN_TURN_SETTLE_BUDGET`] bounds the whole request.
        async fn run_turn(&mut self, actor: ActorId, instance: u32, events: Vec<Event>) -> Result<ExchangeOutcome, String> {
            let deadline = std::time::Instant::now() + RUN_TURN_SETTLE_BUDGET;
            let mut batches = events.into_iter().map(|event| vec![event]).collect::<std::collections::VecDeque<_>>();
            if batches.is_empty() {
                batches.push_back(Vec::new());
            }
            let mut settled: Option<ExchangeOutcome> = None;
            while let Some(batch) = batches.pop_front() {
                let (mut outcome, mut continuation) = self.run_turn_once(actor, instance, batch, deadline).await?;
                let mut quiet = 0usize;
                while let Some(events) = continuation.take() {
                    if std::time::Instant::now() >= deadline || quiet >= RUN_TURN_QUIESCENT_CONTINUATIONS {
                        break;
                    }
                    let (later, next) = self.run_turn_once(actor, instance, events, deadline).await?;
                    quiet = if later.carries_nothing() { quiet + 1 } else { 0 };
                    outcome.absorb(later)?;
                    continuation = next;
                }
                match settled.as_mut() {
                    Some(earlier) => earlier.absorb(outcome)?,
                    None => settled = Some(outcome),
                }
            }
            settled.ok_or_else(|| "kernel: a turn with no batch produced no outcome".to_string())
        }

'''
text = text[:start] + new + text[end:]
path.write_text(text)
print("run_turn rewritten")
