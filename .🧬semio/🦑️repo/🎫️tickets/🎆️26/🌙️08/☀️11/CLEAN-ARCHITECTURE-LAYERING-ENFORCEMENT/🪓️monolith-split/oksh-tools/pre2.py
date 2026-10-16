# 📞️ Rewrites every caller of the former default method `ArtifactSqliteSnapshotHost::sqlite_codec` to the operations trait `ArtifactSqliteSnapshotOperations` (fully qualified, so no import changes), then prunes named Host imports that nothing uses any more.
# usage: pre2.py <files.json> dry|write
import json, os, re, sys
R = '/Users/ueli/Documents/semio'
files = json.load(open(sys.argv[1]))
WRITE = sys.argv[2] == 'write'
KERNEL_S = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs'
UFCS = re.compile(r'(<[^<>]*?\s+as\s+)[\w:]*ArtifactSqliteSnapshotHost(\s*>::sqlite_codec\(\))')
METHOD = re.compile(r'(?<![\w:>$])((?:[A-Za-z_]\w*::)*[A-Z]\w*)::sqlite_codec\(\)')
changed = []; pruned = []; skipped = []

def prefix_of(t, f):
    if 'store::os_store::' in t.replace('semio_framework_os_kernel::os_store::', ''): return 'store::os_store::'
    if 'semio_framework_os_kernel::os_store::' in t or 'semio_framework_os_kernel::{' in t: return 'semio_framework_os_kernel::os_store::'
    return 'crate::os_store::'

def prune(t):
    n = t
    n = re.sub(r'(?m)^use [\w:]*::serialization::ArtifactSqliteSnapshotHost;\n', '', n) if len(re.findall(r'ArtifactSqliteSnapshotHost', n)) == 1 else n
    if len(re.findall(r'ArtifactSqliteSnapshotHost', n)) == 1:
        for pat in (r'os_store::serialization::ArtifactSqliteSnapshotHost, ?', r', ?os_store::serialization::ArtifactSqliteSnapshotHost(?=[,}])', r'serialization::\{ ?ArtifactSqliteSnapshotHost ?\}'):
            m = re.sub(pat, '', n, count=1)
            if m != n: n = m; break
    return n

for f in files:
    p = f if os.path.isabs(f) else R + '/' + f
    if p == KERNEL_S: continue
    t = open(p, encoding='utf8').read(); o = t
    pre = prefix_of(t, f)
    tgt = pre + 'ArtifactSqliteSnapshotOperations'
    t = UFCS.sub(lambda m: m.group(1) + tgt + m.group(2), t)
    t = METHOD.sub(lambda m: '<' + m.group(1) + ' as ' + tgt + '>::sqlite_codec()', t)
    if t == o: skipped.append(f); continue
    changed.append(f)
    if WRITE: open(p, 'w', encoding='utf8').write(t)

pin = R + '/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧩️complete/🟦️.ts'
t = open(pin, encoding='utf8').read()
a = 'Some(<Self as store::os_store::serialization::ArtifactSqliteSnapshotHost>::sqlite_codec())'
assert t.count(a) == 1
if WRITE: open(pin, 'w', encoding='utf8').write(t.replace(a, 'Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())'))
print('changed', len(changed), 'skipped', len(skipped), 'pruned', len(pruned))
for f in pruned: print('pruned', f[-100:])
for f in skipped: print('skipped', f[-100:])
