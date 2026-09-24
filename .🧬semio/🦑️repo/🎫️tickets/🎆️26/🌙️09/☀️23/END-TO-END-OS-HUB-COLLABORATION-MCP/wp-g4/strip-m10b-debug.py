import re, sys
p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"
s = open(p, encoding="utf-8").read()
orig = s
def rep(old, new):
    global s
    n = s.count(old)
    if n != 1:
        sys.exit(f"anchor count {n}: {old[:80]!r}")
    s = s.replace(old, new)
rep('''fn m10b_debug(msg: &str) {
    use std::io::Write;
    let path = "/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/generated/actor-debug.log";
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{}", msg);
        let _ = f.flush();
    }
}

''', '')
s = re.sub(r'\n\s*m10b_debug\(&format!\(\n(?:.*\n)*?\s*\)\);', '', s)
rep('''                let mut request = url.into_client_request().map_err(|error| {
                    m10b_debug(&format!("[DEBUG] m10b connect_future bad url: {error}"));
                })?;''', '''                let mut request = url.into_client_request().map_err(|_| ())?;''')
rep('''                let (mut stream, _response) = match tokio::time::timeout(Duration::from_secs(5), tokio_tungstenite::connect_async(request)).await {
                    Ok(Ok(pair)) => {
                        m10b_debug("[DEBUG] m10b connect_future websocket up");
                        pair
                    }
                    Ok(Err(error)) => {
                        m10b_debug(&format!("[DEBUG] m10b connect_future websocket refuse: {error}"));
                        return Err(());
                    }
                    Err(_) => {
                        m10b_debug("[DEBUG] m10b connect_future websocket timeout 5s");
                        return Err(());
                    }
                };''', '''                let (mut stream, _response) = tokio::time::timeout(Duration::from_secs(5), tokio_tungstenite::connect_async(request)).await.map_err(|_| ())?.map_err(|_| ())?;''')
rep('''                if conn.write.send(message).await.is_err() {
                    m10b_debug(&format!("[DEBUG] m10b send_raw FAIL — tearing down socket"));
                    failed = true;
                } else {
                    m10b_debug(&format!("[DEBUG] m10b send_raw OK bytes_pending_batches={}", self.pending_batches.len()));
                }''', '''                if conn.write.send(message).await.is_err() {
                    failed = true;
                }''')
rep('''                // ⏱️ A 4 ms timeout here treated scheduler/backpressure delay as a dead socket,
                // tore down the connection, and re-queued every Commands batch — so a live hub
                // never saw the agent's envelopes (M10b, head_seq stayed 0 despite relayedBatches≥1).
                // Only a real sink error is fatal; the actor drive loop already yields between phases.
''', '')
rep('''                // 🔐 Framework `/scopes/.../document/ws` authenticates the session on
                // `Sec-WebSocket-Protocol: semio.session.v1, <session>` (C4b). The open-plan
                // socket grant still mints `actor_id` + authority; it does not open the WS.
''', '')
s = re.sub(r'\n[ \t]*m10b_debug\([^\n]*\);[ \t]*(?=\n)', '', s)
s = re.sub(r'let Some\((\w+)\) = ([^\n]*?) else \{\n\s*return;\n\s*\};', r'let Some(\1) = \2 else { return };', s)
if "m10b" in s:
    for i, l in enumerate(s.splitlines(), 1):
        if "m10b" in l: print("LEFT", i, l)
    sys.exit(1)
open(p, "w", encoding="utf-8").write(s)
print("removed lines:", orig.count("\n") - s.count("\n"))
