import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs")
text = path.read_text()
pairs = [
    ("""        fn reject_document_backbone(&mut self, reason: impl Into<String>, messages: Vec<u8>) {
            let batch_id = self.next_local_rejection_batch_id;
""", """        fn reject_document_backbone(&mut self, reason: impl Into<String>, messages: Vec<u8>) {
            let reason = reason.into();
            eprintln!("[DEBUG] wg8 actor backbone rejected actor={:x} document={} reason={reason}", self.hlc_seed, self.document_id);
            let batch_id = self.next_local_rejection_batch_id;
"""),
    ("""                    let envelopes = match decode_document_backbone_message_exact(&message) {
                        Ok(envelopes) if envelopes.iter().all(|envelope| envelope.document_id.0 == self.document_id) => envelopes,
""", """                    eprintln!("[DEBUG] wg8 actor backbone ids actor={:x} decoded={:?}", self.hlc_seed, decode_document_backbone_message_exact(&message).map(|envelopes| envelopes.iter().map(|envelope| envelope.document_id.0.clone()).collect::<Vec<_>>()));
                    let envelopes = match decode_document_backbone_message_exact(&message) {
                        Ok(envelopes) if envelopes.iter().all(|envelope| envelope.document_id.0 == self.document_id) => envelopes,
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
