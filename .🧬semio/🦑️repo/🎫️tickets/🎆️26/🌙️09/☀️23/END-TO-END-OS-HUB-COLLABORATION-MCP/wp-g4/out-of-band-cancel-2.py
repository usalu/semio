import sys
base = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
def edit(rel, pairs):
    p = base + rel
    s = open(p, encoding="utf-8").read()
    for old, new in pairs:
        n = s.count(old)
        if n != 1:
            sys.exit(f"{rel}: anchor count {n}: {old[:100]!r}")
        s = s.replace(old, new)
    open(p, "w", encoding="utf-8").write(s)
edit("🧭️protocol/🦀️.rs", [
('''pub const METHOD_NOTIFICATIONS_CANCELLED: &str = "notifications/cancelled";
''', '''pub const METHOD_NOTIFICATIONS_CANCELLED: &str = "notifications/cancelled";

/// 🛑️ Out-of-band `notifications/cancelled`: a transport whose serve loop dispatches one request at a
/// time hands every inbound line here BEFORE queueing it, or the cancel would only be read after the
/// call it names had already finished. `true` means the line was that notification and is consumed —
/// a notification is never answered, so nothing is lost by not dispatching it again.
pub fn intercept_cancellation(line: &str) -> bool {
    if !line.contains(METHOD_NOTIFICATIONS_CANCELLED) {
        return false;
    }
    let Ok(value) = serde_json::from_str::<serde_json::Value>(line.trim()) else { return false };
    if value.get("method").and_then(serde_json::Value::as_str) != Some(METHOD_NOTIFICATIONS_CANCELLED) || value.get("id").is_some_and(|id| !id.is_null()) {
        return false;
    }
    if let Some(request_id) = value.get("params").and_then(|params| params.get("requestId")) {
        crate::notify::cancel_request(request_id);
    }
    true
}
'''),
])
edit("🚚️transport/🦀️.rs", [
('''                    Ok(_) => {
                        if sender.send(Ok(line)).is_err() {''', '''                    Ok(_) => {
                        if crate::protocol::intercept_cancellation(&line) {
                            continue;
                        }
                        if sender.send(Ok(line)).is_err() {'''),
])
print("ok")
