import hashlib, pathlib, re, sys, json

ROOT = pathlib.Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio')
SKIP = ('🌊️flow/', '🌊️pipeline', '🌐️envelope', '🎬️demo')


def envelope_id(data):
    m = re.search(rb'(s\.stdio\.semio(?:\.[a-z]+)?)\.pack v1', data)
    return m.group(1).decode() if m else None


def twin(path):
    d = path.parent
    if path.name == '🎒️.pack.semio':
        t = d / '🗣️.dsl.semio'
    else:
        t = d / path.name.replace('📦️', '📝️').replace('.pack.semio', '.dsl.semio')
    if t.exists():
        return t
    nested = sorted(d.glob('*/🗣️.dsl.semio'))
    return nested[0] if len(nested) == 1 else None


def plan():
    packs = [p for p in ROOT.rglob('*pack.semio') if 'node_modules' not in p.parts and not any(s in str(p) for s in SKIP)]
    groups = {}
    for p in packs:
        groups.setdefault(hashlib.sha256(p.read_bytes()).hexdigest(), []).append(p)
    rows = []
    for key, members in groups.items():
        data = members[0].read_bytes()
        source = next((twin(m) for m in members if twin(m)), None)
        header = re.match(r'semio (\S+)\.dsl v1', source.read_text(encoding='utf-8')) if source else None
        env = header.group(1) if header else envelope_id(data)
        rows.append({'env': env, 'dsl': str(source) if source else None, 'targets': [str(m) for m in members]})
    return rows


if __name__ == '__main__':
    rows = plan()
    json.dump(rows, open(sys.argv[1], 'w'), ensure_ascii=False, indent=1)
    for r in rows:
        print(r['env'], 'DSL' if r['dsl'] else 'NO-DSL', len(r['targets']), r['targets'][0].split('subsets/')[-1])
