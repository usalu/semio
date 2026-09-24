#!/usr/bin/env python3
from pathlib import Path

main = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/links/os-store")
sync = next(p for p in main.iterdir() if p.is_dir() and "sync" in p.name)
rs = next(p for p in sync.glob("*.rs"))
text = rs.read_text()

# Convert remaining eprintln! DEBUG blocks to m10b_debug(&format!(...))
replacements = [
(
'''                eprintln!(
                    "[DEBUG] m10b catchup wait: actual={}:{}:{} required={}:{}:{}",
                    actual.head_edit_ordinal, actual.last_commit_seq, actual.head_edit_id, required.head_edit_ordinal, required.last_commit_seq, required.head_edit_id
                );''',
'''                m10b_debug(&format!(
                    "[DEBUG] m10b catchup wait: actual={}:{}:{} required={}:{}:{}",
                    actual.head_edit_ordinal, actual.last_commit_seq, actual.head_edit_id, required.head_edit_ordinal, required.last_commit_seq, required.head_edit_id
                ));'''
),
(
'''                    eprintln!(
                        "[DEBUG] m10b Session confirmed actor={} required_tail={} outbox={} pending={}",
                        actor,
                        self.required_tail_frontier.is_some(),
                        self.outbox.len(),
                        self.pending_batches.len()
                    );''',
'''                    m10b_debug(&format!(
                        "[DEBUG] m10b Session confirmed actor={} required_tail={} outbox={} pending={}",
                        actor,
                        self.required_tail_frontier.is_some(),
                        self.outbox.len(),
                        self.pending_batches.len()
                    ));'''
),
(
'''                eprintln!(
                    "[DEBUG] m10b relay queue: confirmed={} hub={} bootstrap={} required_tail={} outbox→{} n={}",
                    self.socket_actor_confirmed,
                    self.semio_hub.is_some(),
                    self.artifact_bootstrap.is_some(),
                    self.required_tail_frontier.as_ref().map(|f| format!("{}:{}:{}", f.head_edit_ordinal, f.last_commit_seq, f.head_edit_id)).unwrap_or_else(|| "none".into()),
                    self.outbox.len() + envelopes.len(),
                    envelopes.len()
                );''',
'''                m10b_debug(&format!(
                    "[DEBUG] m10b relay queue: confirmed={} hub={} bootstrap={} required_tail={} outbox→{} n={}",
                    self.socket_actor_confirmed,
                    self.semio_hub.is_some(),
                    self.artifact_bootstrap.is_some(),
                    self.required_tail_frontier.as_ref().map(|f| format!("{}:{}:{}", f.head_edit_ordinal, f.last_commit_seq, f.head_edit_id)).unwrap_or_else(|| "none".into()),
                    self.outbox.len() + envelopes.len(),
                    envelopes.len()
                ));'''
),
]
for old, new in replacements:
    if old not in text:
        print("MISSING block")
        print(old[:120])
    else:
        text = text.replace(old, new)
        print("replaced ok")
rs.write_text(text)
print("done", rs)
