# 📌 Splits the echo-suppression source pins by platform: each actor file pins its own admission, authoring and settlement sites.
import os
R = '/Users/ueli/Documents/semio'
SYNC = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync'
def once(t, a, b):
    assert t.count(a) == 1, (t.count(a), a[:90]); return t.replace(a, b)
p = SYNC + '/🧪️tests/🔬️document-echo-suppression/🦀️.rs'
t = open(p, encoding='utf8').read()
t = once(t, '''    assert_eq!(source.matches("admit_remote_envelopes(&mut self.applied_op_ids").count(), 2, "both actors admit by operation identity");
    assert_eq!(source.matches("note_authored_envelopes(&mut self.applied_op_ids, envelopes);").count(), 2, "both actors record what they author");
''', '')
t = once(t, '''    assert_eq!(source.matches("let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);").count(), 2, "native and wasm32 Commands arms settle");
''', '')
open(p, 'w', encoding='utf8').write(t)
PIN = '''//! 📌️ Source pin of the {name} actor: it admits what it receives and records what it authors by operation identity, settles committed operations before admission, discards no frame by its origin and pushes no unchecked rollback.

#[test]
fn the_{slug}_actor_admits_authors_and_settles_by_operation_identity() {{
    let source = include_str!("../../🎭️actor/🦀️.rs");
    assert!(!source.contains("Some(origin.0.as_str())"), "no Commands frame is discarded by its origin");
    assert_eq!(source.matches("admit_remote_envelopes(&mut self.applied_op_ids").count(), 1, "the {slug} actor admits by operation identity");
    assert_eq!(source.matches("note_authored_envelopes(&mut self.applied_op_ids, envelopes);").count(), 1, "the {slug} actor records what it authors");
    assert_eq!(source.matches("let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);").count(), 1, "the {slug} Commands arm settles");
    assert_eq!(source.matches("rollbacks.push(rollback_envelope(").count(), 0, "no rollback path pushes an unchecked rollback");
}}
'''
MOUNT = '\n#[cfg(test)]\n#[path = "../🧪️tests/🔬️echo-suppression-pin/🦀️.rs"]\nmod echo_suppression_pin;\n'
for sub, name, slug in (('🖥️native', 'native', 'native'), ('🌐️wasm', 'browser', 'browser')):
    d = SYNC + '/' + sub + '/🧪️tests/🔬️echo-suppression-pin'
    os.makedirs(d, exist_ok=True)
    open(d + '/🦀️.rs', 'w', encoding='utf8').write(PIN.format(name=name, slug=slug))
    a = SYNC + '/' + sub + '/🎭️actor/🦀️.rs'
    at = open(a, encoding='utf8').read()
    assert 'echo_suppression_pin' not in at
    open(a, 'w', encoding='utf8').write(at.rstrip('\n') + '\n' + MOUNT)
print('ok')
