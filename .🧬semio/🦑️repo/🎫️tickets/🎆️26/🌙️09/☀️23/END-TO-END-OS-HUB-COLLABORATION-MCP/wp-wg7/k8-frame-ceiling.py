"""🐍️ WG7 K8 — the browser document actor never hands its socket a frame over the socket's own ceiling (apply|revert)."""
import sys
from pathlib import Path

SYNC = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs")
DOOR = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔌️socket-door/🦀️.rs")

HUNKS = {SYNC: [], DOOR: []}

def hunk(path, old, new):
    HUNKS[path].append((old, new))

hunk(SYNC, '''pub trait DocumentSocket {
    fn is_open(&self) -> bool;
    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), String>;''', '''pub trait DocumentSocket {
    fn is_open(&self) -> bool;
    /// 📏️ The largest frame this socket carries; the actor splits a `Commands` batch to fit it.
    fn max_frame_bytes(&self) -> usize;
    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), String>;''')

hunk(SYNC, '''        //#region 🔖️Relay
        /// @emoji 🧺️ Builds + sends one `Commands` batch under the hub-issued socket actor, tracking it in
        /// `pending_batches` for {@link WasmActor::handle_ack}. Mirrors the native `relay_operations_to_hub`:
        /// nothing leaves before the hub confirmed the socket actor and every catch-up completed.
        async fn relay_operations(&mut self, envelopes: &[MutationEnvelope]) {''', '''        //#region 🔖️Relay
        /// @emoji 🧺️ Builds + sends `Commands` batches under the hub-issued socket actor, tracking each in
        /// `pending_batches` for {@link WasmActor::handle_ack}. Mirrors the native `relay_operations_to_hub`:
        /// nothing leaves before the hub confirmed the socket actor and every catch-up completed. A batch
        /// whose frame exceeds the socket's own ceiling is halved until it fits; one envelope that can never
        /// fit is rolled back locally and reported, never retried into a reconnect loop.
        async fn relay_operations(&mut self, envelopes: &[MutationEnvelope]) {''')

hunk(SYNC, '''            let batch_id = self.next_batch_id;
            self.next_batch_id = self.next_batch_id.wrapping_add(1);
            let mut wire_envelopes: Vec<MutationEnvelope> = Vec::with_capacity(envelopes.len());
            for envelope in envelopes {
                let timestamp = next_timestamp(hlc_seed, &mut self.hlc_counter).await;
                wire_envelopes.push(MutationEnvelope { actor: ActorId(socket_actor.clone()), timestamp, ..envelope.clone() });
            }
            self.pending_batches.insert(batch_id, envelopes.to_vec());
            self.send_frame(&ClientFrame::Commands { batch_id, envelopes: wire_envelopes }, Lane::Command).await;
            self.emit_status_if_changed();
        }''', '''            let max_frame = self.socket.as_ref().map_or(usize::MAX, |socket| socket.max_frame_bytes());
            let mut wire_envelopes: Vec<MutationEnvelope> = Vec::with_capacity(envelopes.len());
            for envelope in envelopes {
                let timestamp = next_timestamp(hlc_seed, &mut self.hlc_counter).await;
                wire_envelopes.push(MutationEnvelope { actor: ActorId(socket_actor.clone()), timestamp, ..envelope.clone() });
            }
            let plan = commands_frames_within(max_frame, self.next_batch_id, envelopes.to_vec(), wire_envelopes).await;
            for frame in plan.frames {
                self.next_batch_id = frame.batch_id.wrapping_add(1);
                self.pending_batches.insert(frame.batch_id, frame.local);
                self.send_encoded(frame.bytes).await;
            }
            if !plan.oversized.is_empty() {
                self.refuse_oversized_batch(plan.oversized).await;
            }
            self.emit_status_if_changed();
        }

        /// 📏️ One envelope whose `Commands` frame exceeds the socket's ceiling on its own: the hub can never
        /// receive it, so its speculative local head is rolled back and the refusal is published.
        async fn refuse_oversized_batch(&mut self, local: Vec<MutationEnvelope>) {
            self.document_backbone_retention.release(&local);
            let mut rollbacks: Vec<MutationEnvelope> = Vec::with_capacity(local.len());
            for envelope in local.iter().rev() {
                rollbacks.push(rollback_envelope(envelope).await);
            }
            let _ = self.deliver_remote_operations(rollbacks).await;
            self.reject_document_backbone("document socket frame exceeds the socket's ceiling", vec![local.len().min(u8::MAX as usize) as u8]);
        }''')

hunk(SYNC, '''        async fn send_frame(&mut self, frame: &ClientFrame, lane: Lane) {
            if self.hello.is_some() || !self.socket.as_ref().is_some_and(|socket| socket.is_open()) {
                return;
            }
            let bytes = encode_client_frame(frame, lane).await;
            let Some(socket) = self.socket.as_mut() else { return };
            if socket.send_binary(bytes).is_err() {
                self.disconnect();
            }
        }''', '''        async fn send_frame(&mut self, frame: &ClientFrame, lane: Lane) {
            let bytes = encode_client_frame(frame, lane).await;
            self.send_encoded(bytes).await;
        }

        async fn send_encoded(&mut self, bytes: Vec<u8>) {
            if self.hello.is_some() || !self.socket.as_ref().is_some_and(|socket| socket.is_open()) {
                return;
            }
            let Some(socket) = self.socket.as_mut() else { return };
            if socket.send_binary(bytes).is_err() {
                self.disconnect();
            }
        }''')

hunk(SYNC, '''/// @emoji 🎲️ One document actor's hybrid-logical-clock replica seed: platform entropy, so two replicas''', '''/// @emoji 📦️ One `Commands` frame ready for a socket: its batch id, the local envelopes it relays (the
/// rollback owner) and its encoded bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandsFrame {
    pub batch_id: u64,
    pub local: Vec<MutationEnvelope>,
    pub bytes: Vec<u8>,
}

/// @emoji 📏️ The frames one relay sends, every one within the socket's ceiling, plus the envelopes that
/// cannot fit a frame even alone.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandsFramePlan {
    pub frames: Vec<CommandsFrame>,
    pub oversized: Vec<MutationEnvelope>,
}

/// @emoji ✂️ Splits one relay into `Commands` frames no larger than `max_frame_bytes`, halving a batch
/// until it fits and keeping envelope order; batch ids run consecutively from `first_batch_id`. `local`
/// and `wire` are the same envelopes before and after socket stamping.
pub async fn commands_frames_within(max_frame_bytes: usize, first_batch_id: u64, local: Vec<MutationEnvelope>, wire: Vec<MutationEnvelope>) -> CommandsFramePlan {
    let mut plan = CommandsFramePlan::default();
    let mut batch_id = first_batch_id;
    let mut pending = std::collections::VecDeque::from([(local, wire)]);
    while let Some((local, wire)) = pending.pop_front() {
        let bytes = encode_client_frame(&ClientFrame::Commands { batch_id, envelopes: wire.clone() }, Lane::Command).await;
        if bytes.len() <= max_frame_bytes {
            plan.frames.push(CommandsFrame { batch_id, local, bytes });
            batch_id = batch_id.wrapping_add(1);
        } else if local.len() > 1 {
            let middle = local.len() / 2;
            let (local_head, local_tail) = local.split_at(middle);
            let (wire_head, wire_tail) = wire.split_at(middle);
            pending.push_front((local_tail.to_vec(), wire_tail.to_vec()));
            pending.push_front((local_head.to_vec(), wire_head.to_vec()));
        } else {
            plan.oversized.extend(local);
        }
    }
    plan
}

/// @emoji 🎲️ One document actor's hybrid-logical-clock replica seed: platform entropy, so two replicas''')

hunk(DOOR, '''        fn is_open(&self) -> bool {
            self.socket.lane().is_open()
        }
''', '''        fn is_open(&self) -> bool {
            self.socket.lane().is_open()
        }

        fn max_frame_bytes(&self) -> usize {
            super::SOCKET_DOOR_SEND_MAX_BYTES
        }
''')

def run(mode):
    for path, pairs in HUNKS.items():
        text = path.read_text(encoding="utf-8")
        for old, new in pairs:
            source, target = (old, new) if mode == "apply" else (new, old)
            if text.count(target) == 1 and text.count(source) == 0:
                continue
            assert text.count(source) == 1, (mode, path.name, text.count(source), source[:100])
            text = text.replace(source, target, 1)
        path.write_text(text, encoding="utf-8")
    print(f"k8 {mode}: settled")

run(sys.argv[1] if len(sys.argv) > 1 else "apply")
