import json, pathlib, re
A = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any')
S = A / '🧬️schema'
Q = A / '🚪️io/🪶️sqlite/📸️snapshot'
def rd(p): return p.read_text()
def wr(p, s): p.write_text(s)
def rep(p, old, new, count=1):
    s = rd(p); n = s.count(old); assert n == count, (p.name, old[:80], n); wr(p, s.replace(old, new))
def rx(p, pat, new, count=1, flags=re.S):
    s = rd(p); n = len(re.findall(pat, s, flags)); assert n == count, (p.name, pat[:80], n); wr(p, re.sub(pat, new, s, flags=flags))
import shutil
BK = pathlib.Path('/tmp/claude-1000/-home-deck-semio/84faf2e3-4721-4a15-8b57-5d44cdd1d7f5/scratchpad/std-backup2/🔖️1/🪆️subsets/✳️any')
TOUCHED = [S / n for n in ('🟦️.ts', '🛰️.proto', '🔗️.graphql', '🔣️.json')] + [S / '📸️snapshot' / n for n in ('🟦️.ts', '🛰️.proto', '🔗️.graphql', '🔣️.json', '🦀️.rs')] + [Q / '🗄️.sql']
for t in TOUCHED: shutil.copy(BK / t.relative_to(A), t)

MS = [('placement_x', 'placementX'), ('placement_y', 'placementY'), ('placement_z', 'placementZ'), ('placement_yaw', 'placementYaw')]
AS = [('head_diameter', 'headDiameter')] + [('placement_x', 'placementX'), ('placement_y', 'placementY'), ('placement_z', 'placementZ'), ('placement_yaw', 'placementYaw')]
COLL = [
    ('section_points', 'sectionPoints', 'SectionPoint', 'section_point', [('id', 'id', 's'), ('y', 'y', 'f'), ('z', 'z', 'f')]),
    ('bars', 'bars', 'BarPoint', 'bar_point', [('id', 'id', 's'), ('layer_id', 'layerId', 's'), ('y', 'y', 'f'), ('z', 'z', 'f'), ('diameter', 'diameter', 'f')]),
    ('tendon_points', 'tendonPoints', 'TendonPoint', 'tendon_point', [('id', 'id', 's'), ('x', 'x', 'f'), ('y', 'y', 'f'), ('z', 'z', 'f')]),
    ('stud_rings', 'studRings', 'StudRing', 'stud_ring', [('id', 'id', 's'), ('radius', 'radius', 'f'), ('count', 'count', 'u'), ('diameter', 'diameter', 'f')]),
]
LABEL = {
    'placementX': ('Position x', 'Position x', 'm'), 'placementY': ('Position y', 'Position y', 'm'), 'placementZ': ('Position z', 'Position z', 'm'), 'placementYaw': ('Yaw angle about z', 'Drehwinkel um z', 'rad'),
    'headDiameter': ('Anchor head diameter d_h (0 = derived 2 d)', 'Ankerkopfdurchmesser d_h (0 = abgeleitet 2 d)', 'm'), 'lapShare': ('Lapped share of bars', 'Gestoßener Anteil der Stäbe', '1'),
    'y': ('Coordinate y', 'Koordinate y', 'm'), 'z': ('Coordinate z', 'Koordinate z', 'm'), 'x': ('Coordinate x along the member', 'Koordinate x entlang des Bauteils', 'm'),
    'diameter': ('Bar diameter Ø', 'Stabdurchmesser Ø', 'm'), 'radius': ('Ring radius from the column face', 'Ringradius ab Stützenrand', 'm'), 'count': ('Studs per ring', 'Dübel je Ring', None),
    'id': ('Id', 'Id', None), 'layerId': ('Layer id', 'Lage-Id', None),
}
def ui(c):
    en, de, unit = LABEL[c]
    u = {'label': {'en': en, 'de': de}, 'widget': 'stepper'}
    if unit == 'm': u.update({'unit': 'm', 'displayUnit': 'mm', 'displayFactor': 1000, 'precision': 1, 'step': 0.005})
    elif unit == 'rad': u.update({'unit': 'rad', 'displayUnit': '°', 'displayFactor': 57.29577951308232, 'precision': 1, 'step': 0.017453292519943295})
    elif unit == '1': u.update({'unit': '1', 'displayUnit': '%', 'displayFactor': 100, 'precision': 0, 'step': 0.05})
    elif c == 'count': u.update({'precision': 0, 'step': 1})
    elif c in ('id', 'layerId'): u = {'label': {'en': en, 'de': de}}
    return u
def prop(c, kind):
    if kind == 's': return {'type': 'string', 'x-semio-ui': ui(c)} if c in LABEL else {'type': 'string'}
    if kind == 'u': return {'type': 'integer', 'minimum': 0, 'x-semio-ui': ui(c), 'maximum': 4294967295}
    return {'type': 'number', 'x-semio-ui': ui(c)}

# schema twins
for base in (S, S / '📸️snapshot'):
    ts = base / '🟦️.ts'
    ifaces = ''; parsers = ''
    for f, c, T, tbl, flds in COLL:
        body = ''.join(f'  {cc}: {"string" if k == "s" else "number"};\n' for _, cc, k in flds)
        ifaces += f'export interface {T} {{\n{body}}}\n\n'
        rd_ = ', '.join(f'{cc}: normWireRequired({"normWireString" if k == "s" else ("normWireRange(normWireInteger, {\"minimum\":0,\"maximum\":4294967295})" if k == "u" else "normWireNumber")})' for _, cc, k in flds)
        parsers += f'export const parse{T}: NormWireReader<{T}> = normWireObject<{T}>({{ {rd_} }});\n'
    rep(ts, 'export interface BarLayer {', ifaces + 'export interface BarLayer {')
    s = rd(ts); i = s.index('export const parseBarLayer'); wr(ts, s[:i] + parsers + s[i:])
    rep(ts, '  relativeHumidity: number;\n', '  relativeHumidity: number;\n' + ''.join(f'  {c}: number;\n' for _, c in MS) + ''.join(f'  {c}: {T}[];\n' for _, c, T, _, _ in COLL))
    rep(ts, 'relativeHumidity: normWireRequired(normWireNumber) }', 'relativeHumidity: normWireRequired(normWireNumber), ' + ''.join(f'{c}: normWireRequired(normWireNumber), ' for _, c in MS) + ', '.join(f'{c}: normWireRequired(normWireArray(normWireRef(() => parse{T})))' for _, c, T, _, _ in COLL) + ' }')
    rep(ts, '  aggregateSize: number;\n}', '  aggregateSize: number;\n  lapShare: number;\n}')
    rep(ts, 'aggregateSize: normWireRequired(normWireNumber) }', 'aggregateSize: normWireRequired(normWireNumber), lapShare: normWireRequired(normWireNumber) }')
    rep(ts, '  fCk: number;\n  actions: LoadCaseActions[];\n}', '  fCk: number;\n  actions: LoadCaseActions[];\n' + ''.join(f'  {c}: number;\n' for _, c in AS) + '}')
    rep(ts, 'fCk: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseLoadCaseActions))) }', 'fCk: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseLoadCaseActions))), ' + ', '.join(f'{c}: normWireRequired(normWireNumber)' for _, c in AS) + ' }')
    pr = base / '🛰️.proto'
    msgs = ''.join('message %s { %s }\n' % (T, ' '.join(f'{"string" if k == "s" else ("uint32" if k == "u" else "double")} {fn} = {i + 1};' for i, (fn, _, k) in enumerate(flds))) for _, _, T, _, flds in COLL)
    rep(pr, 'message Anchor {', msgs + 'message Anchor {')
    rep(pr, ' double relative_humidity = 33;\n', ' double relative_humidity = 33;\n' + ''.join(f' double {f} = {34 + i};\n' for i, (f, _) in enumerate(MS)) + ''.join(f' repeated {T} {f} = {38 + i};\n' for i, (f, _, T, _, _) in enumerate(COLL)))
    rep(pr, ' double aggregate_size = 8; }', ' double aggregate_size = 8; double lap_share = 9; }')
    rep(pr, ' repeated LoadCaseActions actions = 10; }', ' repeated LoadCaseActions actions = 10; ' + ' '.join(f'double {f} = {11 + i};' for i, (f, _) in enumerate(AS)) + ' }')
    gq = base / '🔗️.graphql'
    gt = ''.join('type %s { %s }\n' % (T, ' '.join(f'{cc}: {"String" if k == "s" else ("UInt32" if k == "u" else "Float")}!' for _, cc, k in flds)) for _, _, T, _, flds in COLL)
    rep(gq, 'type Anchor {', gt + '\ntype Anchor {')
    rep(gq, '  relativeHumidity: Float!\n', '  relativeHumidity: Float!\n' + ''.join(f'  {c}: Float!\n' for _, c in MS) + ''.join(f'  {c}: [{T}!]!\n' for _, c, T, _, _ in COLL))
    rep(gq, '  aggregateSize: Float!\n}', '  aggregateSize: Float!\n  lapShare: Float!\n}')
    rx(gq, r'(  actions: \[LoadCaseActions!\]!\n)(\}\n\ntype En1992)', lambda m: m.group(1) + ''.join(f'  {c}: Float!\n' for _, c in AS) + m.group(2))
    jp = base / '🔣️.json'
    d = json.loads(rd(jp)); defs = d['$defs']; new = {}
    for k, v in defs.items():
        new[k] = v
        if k == 'BarLayer':
            for f, c, T, tbl, flds in COLL:
                new[T] = {'title': T, 'type': 'object', 'additionalProperties': False, 'required': [cc for _, cc, _ in flds], 'properties': {cc: prop(cc, kk) for _, cc, kk in flds}, 'x-semio-formats': v['x-semio-formats']}
    d['$defs'] = new; defs = new
    rc = defs['RcMember']; i = rc['required'].index('relativeHumidity') + 1
    rc['required'][i:i] = [c for _, c in MS] + [c for _, c, _, _, _ in COLL]
    for _, c in MS: rc['properties'][c] = prop(c, 'f')
    for _, c, T, _, _ in COLL:
        rc['properties'][c] = {'type': 'array', 'items': {'$ref': f'#/$defs/{T}'}, 'x-semio-ui': {'label': {'en': {'sectionPoints': 'Section outline points', 'bars': 'Explicit bars', 'tendonPoints': 'Tendon control points', 'studRings': 'Stud rails (rings)'}[c], 'de': {'sectionPoints': 'Querschnittsumriss', 'bars': 'Einzelstäbe', 'tendonPoints': 'Spanngliedstützpunkte', 'studRings': 'Dübelleisten (Ringe)'}[c]}}}
    bl = defs['BarLayer']; bl['required'].append('lapShare'); bl['properties']['lapShare'] = {**prop('lapShare', 'f'), 'minimum': 0, 'maximum': 1}
    an = defs['Anchor']
    for _, c in AS: an['required'].append(c); an['properties'][c] = prop(c, 'f')
    wr(jp, json.dumps(d, indent=2, ensure_ascii=False) + '\n')

# snapshot rust
rs = S / '📸️snapshot/🦀️.rs'
structs = ''
for f, c, T, tbl, flds in COLL:
    structs += f'''/// 📍 {T} of an explicit geometry collection of a member (lengths in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct {T} {{
''' + ''.join(f'    pub {fn}: {"String" if k == "s" else ("u32" if k == "u" else "f64")},\n' for fn, _, k in flds) + '}\n\n'
rep(rs, '/// 🏛️ Reinforced/prestressed concrete member.', structs + '/// 🏛️ Reinforced/prestressed concrete member.')
rep(rs, '    pub relative_humidity: f64,\n}', '    pub relative_humidity: f64,\n' + ''.join(f'    pub {f}: f64,\n' for f, _ in MS) + ''.join(f'    #[dsl(table)]\n    pub {f}: Vec<{T}>,\n' for f, _, T, _, _ in COLL) + '}')
rep(rs, '    pub aggregate_size: f64,\n}', '    pub aggregate_size: f64,\n    pub lap_share: f64,\n}')
rep(rs, '    pub actions: Vec<LoadCaseActions>,\n}\n\n/// 🗿️ The computed-compliance', '    pub actions: Vec<LoadCaseActions>,\n' + ''.join(f'    pub {f}: f64,\n' for f, _ in AS) + '}\n\n/// 🗿️ The computed-compliance')
rx(rs, r'( *)relative_humidity: 0\.5,\n', lambda m: None or '', 0) if False else None
s = rd(rs)
s, n = re.subn(r'( *)relative_humidity: 0\.5,\n', lambda m: m.group(0) + ''.join(f'{m.group(1)}{f}: 0.0,\n' for f, _ in MS) + ''.join(f'{m.group(1)}{f}: vec![],\n' for f, _, _, _, _ in COLL), s); assert n == 5, n
s, n = re.subn(r'(aggregate_size: [^,}\n]+?)(,?) \}', r'\1, lap_share: 1.0 }', s); assert n == 6, n
s, n = re.subn(r'(f_ck: [^\n]+\n( +)actions: vec!\[\n.*?\n\2\],\n)', lambda m: m.group(1) + ''.join(f'{m.group(2)}{f}: 0.0,\n' for f, _ in AS), s, flags=re.S); assert n == 2, n
wr(rs, s)

# sqlite sql
sq = Q / '🗄️.sql'
rep(sq, 'bridge_delta_sigma_s REAL, relative_humidity REAL,\n', 'bridge_delta_sigma_s REAL, relative_humidity REAL, ' + ', '.join(f'{f} REAL' for f, _ in MS) + ',\n')
rep(sq, ' relative_humidity_ieee754_bits INTEGER NOT NULL, relative_humidity_ieee754_class TEXT NOT NULL\n', ' relative_humidity_ieee754_bits INTEGER NOT NULL, relative_humidity_ieee754_class TEXT NOT NULL' + ''.join(f',\n {f}_ieee754_bits INTEGER NOT NULL, {f}_ieee754_class TEXT NOT NULL' for f, _ in MS) + '\n')
rep(sq, ' diameter REAL, count INTEGER NOT NULL CHECK(count BETWEEN 0 AND 4294967295), position TEXT NOT NULL, anchorage_length REAL, lap_length REAL, bond_condition TEXT NOT NULL, aggregate_size REAL,\n', ' diameter REAL, count INTEGER NOT NULL CHECK(count BETWEEN 0 AND 4294967295), position TEXT NOT NULL, anchorage_length REAL, lap_length REAL, bond_condition TEXT NOT NULL, aggregate_size REAL, lap_share REAL,\n')
rep(sq, ' aggregate_size_ieee754_bits INTEGER NOT NULL, aggregate_size_ieee754_class TEXT NOT NULL\n', ' aggregate_size_ieee754_bits INTEGER NOT NULL, aggregate_size_ieee754_class TEXT NOT NULL,\n lap_share_ieee754_bits INTEGER NOT NULL, lap_share_ieee754_class TEXT NOT NULL\n')
rep(sq, 'c1 REAL, f_ck REAL,\n', 'c1 REAL, f_ck REAL, ' + ', '.join(f'{f} REAL' for f, _ in AS) + ',\n')
rep(sq, ' c1_ieee754_bits INTEGER NOT NULL, c1_ieee754_class TEXT NOT NULL,\n f_ck_ieee754_bits INTEGER NOT NULL, f_ck_ieee754_class TEXT NOT NULL\n', ' c1_ieee754_bits INTEGER NOT NULL, c1_ieee754_class TEXT NOT NULL,\n f_ck_ieee754_bits INTEGER NOT NULL, f_ck_ieee754_class TEXT NOT NULL' + ''.join(f',\n {f}_ieee754_bits INTEGER NOT NULL, {f}_ieee754_class TEXT NOT NULL' for f, _ in AS) + '\n')
tables = ''
for f, c, T, tbl, flds in COLL:
    cols = ''.join(', %s %s' % (fn, 'TEXT NOT NULL' if k == 's' else ('INTEGER NOT NULL CHECK(%s BETWEEN 0 AND 4294967295)' % fn if k == 'u' else 'REAL')) for fn, _, k in flds if fn != 'id')
    bits = ',\n'.join(f' {fn}_ieee754_bits INTEGER NOT NULL, {fn}_ieee754_class TEXT NOT NULL' for fn, _, k in flds if k == 'f')
    tables += f'CREATE TABLE en1992_{tbl} (\n id INTEGER PRIMARY KEY, member_id INTEGER NOT NULL REFERENCES en1992_member(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL{cols},\n{bits}\n);\n'
s = rd(sq); wr(sq, s.rstrip('\n') + '\n' + tables)
print('schema layers done')
