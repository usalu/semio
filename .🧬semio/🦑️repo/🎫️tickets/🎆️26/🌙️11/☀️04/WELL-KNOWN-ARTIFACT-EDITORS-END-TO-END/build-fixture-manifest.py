import glob, json, os, re, collections
REPO = '/home/deck/semio'
ART = REPO + '/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
EXT_ROOT = REPO + '/🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions'
MIT = REPO + '/♻️mit-bestand'
TICKET = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(TICKET, '🧫️fixture-manifest.json')
SKIP_DIRS = {'target', 'node_modules'}
INVALID = re.compile(r'invalid|malformed|truncat|corrupt|broken|damag|reject-dib', re.I)
META = {'🔣️.json', '📜️artifact-definition.json', 'package.json', '📋️project.json', 'Cargo.toml'}
PRIORITY = ['🧫️fixtures', '🧪️tests', '📚️examples', '🔮️oracles']
KEYCAP = re.compile(r'^(?:[0-9]️?⃣|[^\x00-\x7f])+')

def clean(name):
    return KEYCAP.sub('', name) or name

def rel(p):
    return os.path.relpath(p, REPO)

def walk(base):
    for root, dirs, files in os.walk(base):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for f in files:
            yield os.path.join(root, f)

# surfaces: component manifests list "<artifactKind>@<standard>/<subset>#<role>"
surf = collections.defaultdict(lambda: {'editors': set(), 'viewers': set()})
for p in glob.glob(EXT_ROOT + '/*/🧩️component/🔣️.json') + [REPO + '/🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🔣️.json']:
    text = open(p, encoding='utf8').read()
    for m in re.finditer(r's\.stdio\.([a-z0-9]+)@[^"\s,\]]*?#(editor|viewer)s?(?![a-z])', text):
        full = m.group(0)
        artifact = m.group(1)
        role = 'editors' if m.group(2) == 'editor' else 'viewers'
        surf[artifact][role].add(full)
# composition catalog surface for json
trusted = json.load(open(REPO + '/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/trusted-stdio-catalog.json', encoding='utf8'))
for t in trusted.get('openTargets', []):
    art = t['artifactKind'].split('.')[-1]
    surf[art]['editors' if t['role'] == 'editor' else 'viewers'].add(t['surfaceId'])

defs = {}
for p in sorted(glob.glob(ART + '/*/📜️artifact-definition.json')):
    d = json.load(open(p, encoding='utf8'))
    defs[d['artifact']] = (os.path.dirname(p), d)

entries = []
for art in sorted(defs):
    dpath, d = defs[art]
    kind = d['id']
    # dialect coordinates from standards/subsets folders
    dialects = []
    std_root = os.path.join(dpath, '🏅️standards')
    if os.path.isdir(std_root):
        for sd in sorted(os.listdir(std_root)):
            sp = os.path.join(std_root, sd)
            if not os.path.isdir(sp):
                continue
            subs = sorted(os.listdir(os.path.join(sp, '🪆️subsets'))) if os.path.isdir(os.path.join(sp, '🪆️subsets')) else []
            subs = [s for s in subs if os.path.isdir(os.path.join(sp, '🪆️subsets', s))]
            std = clean(sd)
            if not subs:
                dialects.append({'coordinate': f'{kind}/{std}/null', 'standard': std, 'subset': None, 'folder': rel(sp)})
            for s in subs:
                sub = clean(s)
                dialects.append({'coordinate': f'{kind}/{std}/{sub}', 'standard': std, 'subset': sub, 'folder': rel(os.path.join(sp, '🪆️subsets', s))})
    else:
        for rep in d.get('representations', []):
            dialects.append({'coordinate': f"{kind}/{rep['standard'].split('.standard.')[-1].split('.')[0] if False else rep['standard'].rsplit('.standard.',1)[-1].split('.representation')[0]}/null", 'standard': rep['standard'], 'subset': None, 'folder': None})
        seen = set(); dd = []
        for x in dialects:
            if x['coordinate'] not in seen:
                seen.add(x['coordinate']); dd.append(x)
        dialects = dd
    mimes = sorted({m for r in d.get('representations', []) for m in r.get('mimes', [])})
    exts = sorted({e for r in d.get('representations', []) for e in r.get('extensions', [])})
    native = [c for c in d.get('codecs', []) if c.get('native_factory')]
    codec_id = native[0]['id'] if native else None
    native_ext = native[0]['native_factory']['extension'] if native else None
    # samples
    dot_exts = set(exts)
    art_files = [f for f in walk(dpath) if os.path.splitext(f)[1].lower() in dot_exts and os.path.getsize(f) > 0]
    art_files = [f for f in art_files if os.path.getsize(f) <= 5 * 1024 * 1024]
    valid = [f for f in art_files if not INVALID.search(os.path.basename(f))]
    invalid = [f for f in art_files if INVALID.search(os.path.basename(f))]
    def score(f):
        r = os.path.relpath(f, dpath)
        pri = next((i for i, token in enumerate(PRIORITY) if token in r), len(PRIORITY))
        return (os.path.basename(f) in META, pri, len(r), r)
    valid.sort(key=score)
    picked = []; parents = set()
    for f in valid:
        par = os.path.dirname(f)
        if par in parents: continue
        parents.add(par); picked.append(f)
        if len(picked) == 3: break
    source = 'artifact'
    if not picked:
        mit = [f for f in walk(MIT) if os.path.splitext(f)[1].lower() in dot_exts and os.path.getsize(f) > 0 and not INVALID.search(os.path.basename(f))]
        mit.sort(key=lambda f: (len(f), f))
        picked = mit[:3]
        source = 'mit-bestand' if picked else 'none'
    if not picked:
        ext_hits = []
        for f in walk(REPO):
            if '/.' in f or 'node_modules' in f or '/target/' in f: continue
            if os.path.splitext(f)[1].lower() in dot_exts and os.path.getsize(f) > 0 and not INVALID.search(os.path.basename(f)):
                ext_hits.append(f)
        ext_hits.sort(key=lambda f: (len(f), f))
        picked = ext_hits[:3]
        source = 'repo-external' if picked else 'none'
    invalid_sorted = sorted(invalid, key=lambda f: os.path.relpath(f, REPO))[:3]
    s = surf.get(art, {'editors': set(), 'viewers': set()})
    entries.append({
        'artifact': art,
        'artifactKind': kind,
        'directory': rel(dpath),
        'mimes': mimes,
        'extensions': exts,
        'nativeCodecId': codec_id,
        'nativeExtension': native_ext,
        'dialects': dialects,
        'editorAppIds': sorted(s['editors']),
        'viewerAppIds': sorted(s['viewers']),
        'validSamples': [rel(f) for f in picked],
        'validSampleSource': source,
        'invalidFixtureStatus': 'present' if invalid_sorted else 'absent',
        'invalidSamples': [rel(f) for f in invalid_sorted],
        'hasValidFixture': bool(picked),
        'hasSurface': bool(s['editors'] or s['viewers']),
    })

out = {
    'schema': 'semio.stdio.fixture-manifest/v1',
    'owner': 'T19 haiku-registration (manifest), consumer T06 sonnet-validation',
    'repoRoot': REPO,
    'artifactCount': len(entries),
    'lacking': {
        'validFixture': [e['artifact'] for e in entries if not e['hasValidFixture']],
        'surface': [e['artifact'] for e in entries if not e['hasSurface']],
    },
    'artifacts': entries,
}
with open(OUT, 'w', encoding='utf8') as fh:
    json.dump(out, fh, ensure_ascii=False, indent=2)
    fh.write('\n')
print('wrote', OUT, len(entries))
print('lacking validFixture', out['lacking']['validFixture'])
print('lacking surface', out['lacking']['surface'])
