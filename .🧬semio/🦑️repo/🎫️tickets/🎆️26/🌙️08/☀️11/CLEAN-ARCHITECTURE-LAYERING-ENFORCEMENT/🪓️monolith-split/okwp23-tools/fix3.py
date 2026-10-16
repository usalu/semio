# retirement prep: the engine-core HistoryColumn retire impl and the command custody test mount move to the kernel root (their type / test belong there)
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ns, apply as A
S = ns.S
rp = S + '/♻️retirement/🦀️.rs'
t = open(rp, encoding='utf8').read()
macro = "artifact_retire_struct!(super::HistoryColumn { checkpoint_id, timestamp, labels, authors, parent_checkpoint_id, description, lane, alternative_ids });\n"
mount = '#[cfg(test)]\n#[path="../📬️command/♻️retirement/🧪️tests/🦀️.rs"]\nmod command_custody_tests;\n'
assert t.count(macro) == 1 and t.count(mount) == 1
A.write(rp, t.replace(macro, '').replace(mount, ''))
root = S + '/🦀️.rs'
r = open(root, encoding='utf8').read()
add = '\n// ♻️ `HistoryColumn` belongs to the engine core, so its retirement impl and the command custody test live beside it until the engine crates land.\nsemio_framework_value::artifact_retire_struct!(HistoryColumn { checkpoint_id, timestamp, labels, authors, parent_checkpoint_id, description, lane, alternative_ids });\n#[cfg(test)]\n#[path = "📬️command/♻️retirement/🧪️tests/🦀️.rs"]\nmod command_custody_tests;\n'
A.write(root, r.rstrip('\n') + '\n' + add)
print('ok')
