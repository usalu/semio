import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs")
text = path.read_text()
pairs = [
    ("""                ArtifactActorMsg::DocumentBackbone { message } => {
                    let envelopes = match decode_document_backbone_message_exact(&message) {
""", """                ArtifactActorMsg::DocumentBackbone { message } => {
                    eprintln!("[DEBUG] wg8 actor backbone actor={:x} bytes={} space={:?}", self.hlc_seed, message.len(), self.hub_space_id);
                    let envelopes = match decode_document_backbone_message_exact(&message) {
"""),
    ("""            if !self.socket_actor_confirmed || self.semio_hub.is_none() || self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                self.queue_outbox(envelopes.iter().cloned());
                return;
            }
            let batch_id = self.next_batch_id;""", """            if !self.socket_actor_confirmed || self.semio_hub.is_none() || self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                eprintln!("[DEBUG] wg8 actor outbox actor={:x} confirmed={} hub={} bootstrap={} tail={}", self.hlc_seed, self.socket_actor_confirmed, self.semio_hub.is_some(), self.artifact_bootstrap.is_some(), self.required_tail_frontier.is_some());
                self.queue_outbox(envelopes.iter().cloned());
                return;
            }
            let batch_id = self.next_batch_id;"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
native_end = text.index("mod wasm_actor {")
for old, new in pairs:
    at = text.find(old)
    assert 0 <= at < native_end, old[:80]
    text = text[:at] + new + text[at + len(old):]
path.write_text(text)
print("ok")
