import pathlib
path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
def swap(old, new):
    global text
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
swap("        async fn run_turn_once(&mut self, actor: ActorId, instance: u32, events: Vec<Event>) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {\n",
     "        async fn run_turn_once(&mut self, actor: ActorId, instance: u32, events: Vec<Event>, deadline: std::time::Instant) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {\n")
swap('''                        ShardOutcome::Resumed { actor: reported, operation } if reported == actor.0 => {''',
     '''                        ShardOutcome::Preempted { actor: reported } => {
                            if std::time::Instant::now() >= deadline {
                                return Err(format!("kernel: actor {reported} stayed preempted past its {RUN_TURN_SETTLE_BUDGET:?} turn budget"));
                            }
                            let resume = Envelope {
                                to: ActorId(reported),
                                from: Origin::Kernel,
                                lane: Lane::Interactive,
                                seq: next_seq()?,
                                deadline_ms: None,
                                coalesce: None,
                                cancel_of: None,
                                payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).map_err(|error| error.to_string())? },
                            };
                            if !matches!(self.runtime.submit(&resume).await, Backpressure::Accept) {
                                return Err(format!("kernel: preempted actor {reported} refused its resume envelope"));
                            }
                        }
                        ShardOutcome::Resumed { actor: reported, operation } if reported == actor.0 => {''')
swap('''                        (None, semio_framework::kernel::TurnStatus::MoreWork) => Some(Vec::new()),''',
     '''                        (None, semio_framework::kernel::TurnStatus::MoreWork) if matches!(result.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle) && matches!(result.cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Idle) => Some(Vec::new()),''')
path.write_text(text)
print("run_turn_once edited")
