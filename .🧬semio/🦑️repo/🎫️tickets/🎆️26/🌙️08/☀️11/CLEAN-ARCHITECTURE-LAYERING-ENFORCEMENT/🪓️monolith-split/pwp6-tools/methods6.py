import sys; sys.path.insert(0, '.')
from fields6 import *
# extract all methods of impl VcsArtifactApp (inherent + PluginApp) across the plugin tree, current state
INSTANCE_FILES = ['🧵️instance/🦀️.rs','🧵️instance/🚪️ingress/🦀️.rs','🧵️instance/📨️dispatch/🦀️.rs','🧵️instance/🖼️render/🦀️.rs','🧵️instance/🧷️command-routes/🦀️.rs','🧵️instance/🧾️typed-operations/🦀️.rs','🧵️instance/🔌️plugin-app-impl/🦀️.rs','🧵️instance/🛠️tool-machine/🦀️.rs','🧵️instance/🤝️group/🦀️.rs','🧵️instance/🔄️full-operation/🦀️.rs','🧵️instance/⏳️operation-progress/🦀️.rs','🪜️close-ladder/🦀️.rs','🪜️envelope-ladder/🦀️.rs','🪜️maintenance-ladder/🦀️.rs','⏪️time-travel/🧵️instance/🦀️.rs','⏯️tool-run/🧵️instance/🦀️.rs','🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs','🎮️command/🎟️lease/🦀️.rs','♻️lifecycle/🏪️stores/🦀️.rs','🎮️command/📥️ingress/🦀️.rs','🪆️child/👁️read/🧪️test-view/🦀️.rs']
FN = re.compile(r'^(\s*)((?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+))')
def extract(files=INSTANCE_FILES, plugp=PLUGP):
    meths = []
    for rel in files:
        its, L, R = pt.outline(ROOT + plugp + rel)
        for ii, it in enumerate(its):
            if it['kind'] != 'impl': continue
            tgt, tr = pt.impl_target(it['head'])
            if tgt != 'VcsArtifactApp': continue
            s, e = it['s'] - 1, it['e']
            # find first '{' line of impl
            depth = 0; cur = None
            for k in range(s, e):
                l = L[k]
                before = depth
                m = FN.match(l) if before == 1 else None
                if m:
                    # attributes/docs above
                    a = k
                    while a - 1 > s and R[a - 1].lstrip().startswith(('///', '#[', '//')): a -= 1
                    cur = dict(name=m.group(3), file=rel, start=k, attr_start=a, impl=ii, trait=tr, vis=(re.match(r'\s*(pub(?:\([^)]*\))?)\s', l).group(1) if re.match(r'\s*pub', l) else ''), implhead=it['head'])
                    meths.append(cur)
                depth = before + l.count('{') - l.count('}')
                if cur is not None and cur.get('end') is None and depth == 1 and k > cur['start'] and before >= 1:
                    pass
            # compute ends: next method attr_start - 1 or impl end - 1
            ms = [m for m in meths if m['file'] == rel and m['impl'] == ii]
            for a, b in zip(ms, ms[1:] + [None]):
                a['end'] = (b['attr_start'] if b else e - 1)
            for m in ms:
                m['text'] = '\n'.join(R[m['attr_start']:m['end']]); m['body'] = '\n'.join(L[m['start']:m['end']])
    return meths
if __name__ == '__main__':
    ms = extract()
    print(len(ms), 'methods;', sum(m['end'] - m['attr_start'] for m in ms), 'lines')
    c = collections.Counter(m['name'] for m in ms)
    print('dup names', [(n, k) for n, k in c.items() if k > 1])
