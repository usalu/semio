import glob, json, os, re
REPO = '/home/deck/semio'
ART = REPO + '/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
TICKET = os.path.dirname(os.path.abspath(__file__))
MANIFEST = os.path.join(TICKET, '🧫️fixture-manifest.json')
TRUNCATED = '✂️truncated'
WRONG = '🪄️wrong-magic'
BAD_MAGIC = b'\xff\xfe\xfd\xfc'
SUFFIX = {TRUNCATED: '-truncated', WRONG: '-invalid-magic'}
WALKER_SAFE = re.compile(r'invalid|truncated|corrupt|malformed|broken', re.I)
MAX_BYTES = 256
IFC_WALKED_ROOT = ART + '/🏗️ifc/🏅️standards/🔖️2x3'
IFC_TRUNCATED_HOME = ART + '/🏗️ifc/🧪️tests/🧫️fixtures/' + TRUNCATED
binary = {}
for p in glob.glob(ART + '/*/📜️artifact-definition.json'):
    d = json.load(open(p, encoding='utf8'))
    binary[d['artifact']] = any(r.get('is_binary') for r in d.get('representations', []))
m = json.load(open(MANIFEST, encoding='utf8'))

def fixtures_dir(src, art_dir):
    cur = os.path.dirname(src)
    while cur.startswith(art_dir):
        if os.path.basename(cur) == '🧫️fixtures':
            return cur
        cur = os.path.dirname(cur)
    subset = src
    while os.path.basename(os.path.dirname(subset)) != '🪆️subsets' and subset != art_dir:
        subset = os.path.dirname(subset)
    cand = os.path.join(subset, '🧪️tests', '🧫️fixtures')
    if os.path.isdir(cand):
        return cand
    found = sorted(f for f in glob.glob(os.path.join(art_dir, '**', '🧫️fixtures'), recursive=True) if os.path.isdir(f))
    if found:
        return found[0]
    os.makedirs(cand, exist_ok=True)
    return cand

# 1. plan from current inputs (all reads happen before any deletion)
plan = []
for e in m['artifacts']:
    if e['validSampleSource'] != 'artifact' or not e['validSamples']:
        continue
    if 'invalidFixtureOrigin' not in e and e['invalidSamples']:
        continue  # authored invalid fixtures already exist
    src_rel = e['validSamples'][0]
    src = os.path.join(REPO, src_rel)
    data = open(src, 'rb').read()
    art_dir = os.path.join(REPO, e['directory'])
    home = fixtures_dir(src, art_dir)
    name = os.path.basename(src)
    stem, ext = os.path.splitext(name)
    if e['artifact'] == 'ifc' and src.startswith(IFC_WALKED_ROOT + os.sep):
        trunc_home = IFC_TRUNCATED_HOME
    else:
        trunc_home = os.path.join(home, TRUNCATED)
    plan.append((e['artifact'], os.path.join(trunc_home, stem + SUFFIX[TRUNCATED] + ext), data[: len(data) // 2]))
    if binary[e['artifact']] and len(data) >= 4:
        plan.append((e['artifact'], os.path.join(home, WRONG, stem + SUFFIX[WRONG] + ext), BAD_MAGIC + data[4:]))

# 2. remove the generated files of the previous run and their now-empty folders
for e in m['artifacts']:
    for rel in e['invalidSamples']:
        if 'invalidFixtureOrigin' in e and os.path.isfile(os.path.join(REPO, rel)):
            os.remove(os.path.join(REPO, rel))
for dirpath, _, _ in os.walk(ART, topdown=False):
    if os.path.basename(dirpath) in (TRUNCATED, WRONG) and not os.listdir(dirpath):
        os.rmdir(dirpath)

# 3. write at the final names and rewrite the manifest entries
new_entries = {}
for artifact, target, payload in plan:
    assert not os.path.exists(target), target
    assert WALKER_SAFE.search(os.path.basename(target)), target
    assert len(os.path.relpath(target, REPO).encode('utf8')) < MAX_BYTES, target
    if artifact == 'ifc':
        assert '🔖️2x3' not in target, target
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, 'wb') as fh:
        fh.write(payload)
    new_entries.setdefault(artifact, []).append(os.path.relpath(target, REPO))
count = 0
for e in m['artifacts']:
    authored = [p for p in e['invalidSamples'] if 'invalidFixtureOrigin' not in e]
    generated = new_entries.get(e['artifact'], [])
    if generated:
        e['invalidSamples'] = authored + generated
        e['invalidFixtureStatus'] = 'present'
        e['invalidFixtureOrigin'] = 'generated-by-build-invalid-fixtures.py from ' + e['validSamples'][0]
        count += len(generated)
m['invalidFixtureSummary'] = {
    'generatedArtifacts': len(new_entries),
    'generatedFiles': count,
    'artifactsStillWithoutInvalid': [e['artifact'] for e in m['artifacts'] if not e['invalidSamples']],
    'layout': 'truncated: <fixtures>/✂️truncated/<stem>-truncated<ext>; wrong magic: <fixtures>/🪄️wrong-magic/<stem>-invalid-magic<ext>; ifc truncated under 🏗️ifc/🧪️tests/🧫️fixtures/✂️truncated (outside the 2x3 walker root)',
}
with open(MANIFEST, 'w', encoding='utf8') as fh:
    json.dump(m, fh, ensure_ascii=False, indent=2)
    fh.write('\n')
print('generated', count, 'artifacts', len(new_entries), 'still without invalid', m['invalidFixtureSummary']['artifactsStillWithoutInvalid'])
