"""🚪️ G11 kernel patch (guest-linked `semio-framework-os-kernel`, frozen until ALL PUBLISHED; land first in the next window):
the space-wide directory socket grant is issued with the EMPTY body the hub's route contract declares.

Measured 2026-09-27 14:0x against 7800 (B3, current-tree os-hub): `POST /directory/socket-grants` with the body `{}` → `413
Failed to buffer the request body: length limit exceeded` (route `DefaultBodyLimit::max(0)` + `body.is_empty()` check since
09-26 11:22, hostile-input fixture row `"body": {"kind": "empty"}`), 0.7 ms. `DirectoryClient::open_stream_ws` still sends
`b"{}"`, so every native directory stream dial fails: the semio MCP gateway cannot bind a hub space at all ("hub directory stream
is unavailable; authenticated snapshot was not activated"), and every other native client of `DirectoryClient::stream`
(the wgpu shell's directory door) is refused the same way. The scoped grant already sends `b""`.
Law: `the_space_wide_directory_grant_carries_the_empty_body_the_hub_route_declares` (unit, recorded request body).
usage: python3 g11-directory-grant-empty-body.py [--apply]"""
import sys

CLIENT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"
TESTS = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs"
HUNKS = [
    (CLIENT, """        let mut receipt = self.issue_socket_grant(ctx, "/directory/socket-grants", b"{}", timeout_ms)?;""", """        let mut receipt = self.issue_socket_grant(ctx, "/directory/socket-grants", b"", timeout_ms)?;"""),
    (TESTS, """#[semio_framework_async_macros::async_test]
async fn backoff_doubles_and_caps() {""", """/// 🚪️ The hub admits `POST /directory/socket-grants` only with an empty body (`DefaultBodyLimit::max(0)`, hostile-input row
/// `"body": {"kind": "empty"}`); a `{}` body is refused `413` before the grant is minted, which took down every native directory
/// stream dial (measured on the 2026-09-27 B3 hub).
#[semio_framework_async_macros::async_test]
async fn the_space_wide_directory_grant_carries_the_empty_body_the_hub_route_declares() {
    let transport = FakeTransport::default();
    push_grant(&transport).await;
    transport.push_ws(Ok(std::collections::VecDeque::new())).await;
    let client = authenticated_client(transport.clone(), "tok");
    let _connection = client.open_stream_ws(&root_ctx(), 0, 100).expect("space-wide socket opens");
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url, "http://hub.local/directory/socket-grants");
    assert!(requests[0].body.is_empty(), "the grant body must be empty, got {:?}", String::from_utf8_lossy(&requests[0].body));
}

#[semio_framework_async_macros::async_test]
async fn backoff_doubles_and_caps() {"""),
]


def main():
    texts = {}
    for path, old, new in HUNKS:
        texts.setdefault(path, open(path, encoding="utf-8").read())
        found = texts[path].count(old)
        print(f"{path.split('/')[-4]}: anchor {found} (expected 1)")
        if found != 1:
            sys.exit(1)
        texts[path] = texts[path].replace(old, new)
    if "--apply" not in sys.argv:
        print("dry run clean")
        return
    for path, text in texts.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied; next: cargo check -p semio-framework-os-kernel --lib --tests; cargo test -p semio-framework-os-kernel --lib -- the_space_wide_directory_grant")


main()
