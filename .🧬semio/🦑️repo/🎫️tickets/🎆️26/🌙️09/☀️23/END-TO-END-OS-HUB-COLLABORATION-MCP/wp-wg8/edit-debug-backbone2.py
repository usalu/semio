import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs")
text = path.read_text()
pairs = [
    ("""        async fn relay_operations_to_hub(&mut self, envelopes: &[MutationEnvelope]) {
            if envelopes.is_empty() {
""", """        async fn relay_operations_to_hub(&mut self, envelopes: &[MutationEnvelope]) {
            eprintln!("[DEBUG] wg8 actor relay actor={:x} envelopes={} expired={} socket_actor={:?}", self.hlc_seed, envelopes.len(), self.socket_authority_deadline.is_some_and(|deadline| deadline <= Instant::now()), self.socket_actor);
            if envelopes.is_empty() {
"""),
    ("""                    if let Err(reason) = self.document_backbone_retention.retain(message.len(), &envelopes) {
                        self.reject_document_backbone(reason, vec![envelopes.len().min(u8::MAX as usize) as u8]);
                        return false;
                    }
""", """                    eprintln!("[DEBUG] wg8 actor backbone decoded actor={:x} envelopes={}", self.hlc_seed, envelopes.len());
                    if let Err(reason) = self.document_backbone_retention.retain(message.len(), &envelopes) {
                        eprintln!("[DEBUG] wg8 actor backbone retention refused actor={:x} {reason}", self.hlc_seed);
                        self.reject_document_backbone(reason, vec![envelopes.len().min(u8::MAX as usize) as u8]);
                        return false;
                    }
"""),
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
