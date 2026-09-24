import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs")
text = path.read_text()
pairs = [
    ("""        async fn on_hub_frame(&mut self, frame: ServerFrame) {
            match frame {
""", """        async fn on_hub_frame(&mut self, frame: ServerFrame) {
            eprintln!("[DEBUG] wg8 hub recv actor={:x} {}", self.hlc_seed, format!("{frame:?}").chars().take(160).collect::<String>());
            match frame {
"""),
    ("""        async fn send_client_frame(&mut self, frame: ClientFrame, lane: Lane) {
            let bytes = encode_client_frame(&frame, lane).await;
""", """        async fn send_client_frame(&mut self, frame: ClientFrame, lane: Lane) {
            eprintln!("[DEBUG] wg8 hub send actor={:x} connected={} {}", self.hlc_seed, self.semio_hub.is_some(), format!("{frame:?}").chars().take(160).collect::<String>());
            let bytes = encode_client_frame(&frame, lane).await;
"""),
    ("""        async fn set_remote_state(&mut self, state: RemoteState) {
""", """        async fn set_remote_state(&mut self, state: RemoteState) {
            eprintln!("[DEBUG] wg8 hub state actor={:x} {state:?}", self.hlc_seed);
"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
