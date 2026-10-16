# 🧷 Wrapper of okwp23-tools/carve.py with the OS-KERNEL-SQLITE-HOST assignment overrides: the core host trait and its receiving defaults live in space-history (lowest implementor), the derived operations trait in os-io.
# usage: oksh.py <carve.py arguments>
import os, runpy, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE + '/../okwp0-tools')
import assign, model
S = model.S
SH, IO = 'os-store-space-history', 'os-io'
SNAP = S + '/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot'
WR = S + '/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs'
model.OVERRIDE[(S + '/🚪️io/🦀️.rs', 'ArtifactSqliteSnapshotHost')] = SH
for n in ('overflow', 'unbacked', 'receiving_allocation', 'project', 'reconstruct', 'validate_decoding', 'validate_encoding'): model.OVERRIDE[(WR, n)] = SH
model.OVERRIDE[(SNAP + '/🚦️native/🦀️.rs', 'retire_projection')] = SH
model.FILE_OVERRIDE[S + '/📦️codec/🪶️snapshot-capability/🧰️codec-operations/🦀️.rs'] = IO
model.FILE_OVERRIDE[SNAP + '/⚖️settlement/🦀️.rs'] = SH
IOF = model.M + '/🚪️io/🦀️.rs'
for n in ('IoFallbackDispatcher', 'IO_FALLBACK', 'IoFallbackRegistrationError', 'set_io_fallback_dispatcher'): model.OVERRIDE[(IOF, n)] = IO
import re
model.OVERRIDE_RX.append((IOF, re.compile(r'(?:Display|Error) for IoWireError\b'), IO))
OPN = S + '/📦️codec/🎮️operation/🦀️.rs'
model.FILE_OVERRIDE[OPN] = 'os-store-engine-owners'
model.FILE_OVERRIDE[OPN.replace('/🦀️.rs', '/🧪️tests/🦀️.rs')] = 'os-store-engine-owners'
for _it in model.file_model(OPN).items:
    if _it['crate']: _it['crate'] = 'os-store-engine-owners'
import plan as P
_slug0 = P.Plan._slug
def _slug(self, f):
    return '🚪️io-composer' if f == IOF else _slug0(self, f)
P.Plan._slug = _slug
model.FILE_OVERRIDE[S + '/📦️codec/📥️lifted/🚪️io/🦀️.rs'] = 'os-store-codec-registry'
S0 = S + '/📦️codec/📥️lifted/🚪️io/🦀️.rs'
def _inject_s0():
    from okrs import items
    src = open(S0, encoding='utf8').read()
    rows = []
    for it in items(src):
        sl = src.count('\n', 0, it['start']) + 1; el = src.count('\n', 0, it['end']) + 1
        if it['kind'] is None: continue
        io_item = it['name'] and it['name'].split(' ')[0] in ('ArtifactAssemblyRegistryPlan', 'register_native_snapshot_codec', 'publish_native_snapshot', 'register_native_document_codec', 'commit_artifact_assembly_registry_plan', 'ArtifactAssemblyRegistryError') or (it['kind'] == 'impl' and 'ArtifactAssemblyRegistry' in (it['name'] or ''))
        rows.append({'start': sl, 'end': el, 'kind': it['kind'], 'name': it['name'], 'crate': IO if io_item else 'os-store-codec-registry'})
    model.cof()['items'][S0] = rows
_inject_s0()
import carve
from okrs import tokenize
_ids0 = carve.idents_in
def _free_idents(text):
    toks = [x for x in tokenize(text) if x[0] not in ('ws', 'comment')]
    out = set()
    for i, x in enumerate(toks):
        if x[0] != 'id': continue
        if i >= 2 and toks[i - 1][1] == ':' and toks[i - 2][1] == ':': continue
        out.add(x[1])
    return out
_sib0 = carve.Batch.sibling_imports
def _sib(self, st, its, body):
    carve.idents_in = _free_idents
    try: return _sib0(self, st, its, body)
    finally: carve.idents_in = _ids0
carve.Batch.sibling_imports = _sib
if __name__ == '__main__':
    import textwrap
    src = open(HERE + '/../okwp23-tools/carve.py', encoding='utf8').read()
    main = textwrap.dedent(src.split("if __name__ == '__main__':\n", 1)[1])
    sys.argv = ['carve.py'] + sys.argv[1:]
    exec(compile(main, 'carve-main', 'exec'), carve.__dict__)
