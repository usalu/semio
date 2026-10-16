# 🔧 Rewrites the kernel-alias paths `store::…` left in the os-io tree to the crates that own the items; dry|write.
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools')
import okcrates as C
from wp1tree import tree
model = C.build()[1]
WRITE = sys.argv[1] == 'write'
RULES = [
    (r'(?<![\w:])store::NativeSnapshot(BodyWallet|DecodeOwner|EncodeOwner)\b', r'semio_framework_io_control::NativeSnapshot\1'),
    (r'(?<![\w:])store::channel::', 'semio_framework_os_channel::'),
    (r'(?<![\w:])store::snapshot_read::ErasedSnapshotRead\b', 'semio_framework_os_store_read::ErasedSnapshotRead'),
    (r'(?<![\w:])store::serialization::ArtifactSqliteSnapshot(Export|Import)File\b', r'semio_framework_os_store_text_io::ArtifactSqliteSnapshot\1File'),
]
tot = 0
for f in tree([C.node_abs('os-io', model) + '/🦀️.rs']):
    t = open(f, encoding='utf8').read(); n = t
    for rx, rep in RULES: n = re.sub(rx, rep, n)
    left = re.findall(r'(?<![\w:$])store::[\w:]+', re.sub(r'(?m)^\s*//.*$', '', n))
    if n != t:
        tot += 1; print('edit', os.path.relpath(f, C.M)[-70:], 'left', left[:3])
        if WRITE: open(f, 'w', encoding='utf8').write(n)
    elif left: print('LEFT', os.path.relpath(f, C.M)[-70:], left[:3])
print('files', tot)
