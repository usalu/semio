#!/usr/bin/env python3
from pathlib import Path

main = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/links/os-store")
sync = next(p for p in main.iterdir() if p.is_dir() and "sync" in p.name)
rs = next(p for p in sync.glob("*.rs"))
text = rs.read_text()
old = '            m10b_debug(&format!("[DEBUG] m10b start_connect BEGIN doc={document_id} schema={schema}"));\n            let expectation'
if old not in text:
    # show nearby
    idx = text.find("start_connect BEGIN")
    print(repr(text[idx-50:idx+200]))
    raise SystemExit("marker missing")
new = '''            m10b_debug(&format!("[DEBUG] m10b start_connect BEGIN doc={document_id} schema={schema}"));
            let expectation'''
# insert more logs after set_remote_state and after connect_future assign
old2 = """            self.set_remote_state(RemoteState::Connecting).await;
            self.connect_future = Some(Box::pin(async move {
"""
new2 = """            m10b_debug("[DEBUG] m10b start_connect before set_remote_state");
            self.set_remote_state(RemoteState::Connecting).await;
            m10b_debug("[DEBUG] m10b start_connect after set_remote_state; installing connect_future");
            self.connect_future = Some(Box::pin(async move {
                m10b_debug("[DEBUG] m10b connect_future entered");
"""
if old2 not in text:
    raise SystemExit("set_remote block missing")
text = text.replace(old2, new2, 1)

# log after admission
old3 = """                let admission = admission_receiver.await.map_err(|_| ())?.map_err(|_| ())?;
                if ctx.cancel.is_cancelled_now() || admission.authority.expires_at_unix_ms <= now_ms().await {
                    return Err(());
                }
"""
new3 = """                m10b_debug("[DEBUG] m10b connect_future waiting admission");
                let admission = admission_receiver.await.map_err(|_| ())?.map_err(|_| ())?;
                m10b_debug("[DEBUG] m10b connect_future admission ok");
                if ctx.cancel.is_cancelled_now() || admission.authority.expires_at_unix_ms <= now_ms().await {
                    m10b_debug("[DEBUG] m10b connect_future admission expired/cancelled");
                    return Err(());
                }
"""
if old3 not in text:
    raise SystemExit("admission block missing")
text = text.replace(old3, new3, 1)

# log before and after connect_async
old4 = """                let (mut stream, _response) = tokio::time::timeout(Duration::from_secs(5), tokio_tungstenite::connect_async(request)).await.map_err(|_| ())?.map_err(|_| ())?;
"""
new4 = """                m10b_debug("[DEBUG] m10b connect_future dialing websocket");
                let (mut stream, _response) = tokio::time::timeout(Duration::from_secs(5), tokio_tungstenite::connect_async(request)).await.map_err(|_| ())?.map_err(|_| ())?;
                m10b_debug("[DEBUG] m10b connect_future websocket up");
"""
if old4 not in text:
    raise SystemExit("connect_async block missing")
text = text.replace(old4, new4, 1)

rs.write_text(text)
print("ok")
