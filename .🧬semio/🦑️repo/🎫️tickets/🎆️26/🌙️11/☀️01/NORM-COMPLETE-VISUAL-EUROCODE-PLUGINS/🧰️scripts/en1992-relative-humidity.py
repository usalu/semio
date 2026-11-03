import re, pathlib, sys
A = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any')
def rd(p): return (A / p).read_text()
def wr(p, s): (A / p).write_text(s)
def sub(p, old, new, count=1, regex=False):
    s = rd(p)
    if 'relativeHumidity' in new and new in s or ('relative_humidity' in new and new in s): print('skip', p); return
    n = len(re.findall(old, s)) if regex else s.count(old)
    assert n == count, (p, old[:60], n)
    s = re.sub(old, new, s) if regex else s.replace(old, new)
    wr(p, s)
S = '🧬️schema/'
for ts in (S + '🟦️.ts', S + '📸️snapshot/🟦️.ts'):
    sub(ts, '  bridgeDeltaSigmaS: number;\n', '  bridgeDeltaSigmaS: number;\n  relativeHumidity: number;\n')
    sub(ts, 'bridgeDeltaSigmaS: normWireRequired(normWireNumber) }', 'bridgeDeltaSigmaS: normWireRequired(normWireNumber), relativeHumidity: normWireRequired(normWireNumber) }')
for pr in (S + '🛰️.proto', S + '📸️snapshot/🛰️.proto'):
    sub(pr, ' double bridge_delta_sigma_s = 32;\n', ' double bridge_delta_sigma_s = 32;\n double relative_humidity = 33;\n')
for g in (S + '🔗️.graphql', S + '📸️snapshot/🔗️.graphql'):
    sub(g, '  bridgeDeltaSigmaS: Float!\n', '  bridgeDeltaSigmaS: Float!\n  relativeHumidity: Float!\n')
block = '''        "relativeHumidity": {
          "type": "number",
          "x-semio-ui": {
            "label": {
              "en": "Relative humidity RH",
              "de": "Relative Luftfeuchte RH"
            },
            "widget": "stepper",
            "precision": 2,
            "step": 0.05,
            "snaps": [
              0.5,
              0.8
            ]
          }
        }
'''
for j in (S + '🔣️.json', S + '📸️snapshot/🔣️.json'):
    sub(j, '        "bridgeDeltaSigmaS",\n', '        "bridgeDeltaSigmaS",\n        "relativeHumidity",\n')
    s = rd(j)
    m = re.search(r'(        "bridgeDeltaSigmaS": \{.*?\n        \})\n', s, re.S)
    assert m
    s = s[:m.end(1)] + ',\n' + block.rstrip('\n') + '\n' + s[m.end():]
    wr(j, s)
rs = S + '📸️snapshot/🦀️.rs'
sub(rs, '    pub bridge_delta_sigma_s: f64,\n', '    pub bridge_delta_sigma_s: f64,\n    pub relative_humidity: f64,\n')
sub(rs, r'( *)bridge_delta_sigma_s: 0\.0,\n', r'\1bridge_delta_sigma_s: 0.0,\n\1relative_humidity: 0.5,\n', 5, True)
sub('✏️editor/🏷️field-meta/🦀️.rs', '    ("members[].bridgeDeltaSigmaS", m("Bridge steel stress range", "Brücken-Stahlschwingbreite", Some(UnitId::Pascal), None)),\n',
 '    ("members[].bridgeDeltaSigmaS", m("Bridge steel stress range", "Brücken-Stahlschwingbreite", Some(UnitId::Pascal), None)),\n    ("members[].relativeHumidity", m("Relative humidity RH", "Relative Luftfeuchte RH", None, None)),\n')
Q = '🚪️io/🪶️sqlite/📸️snapshot/'
sub(Q + '🗄️.sql', 'bridge_delta_sigma_s REAL,\n', 'bridge_delta_sigma_s REAL, relative_humidity REAL,\n')
sub(Q + '🗄️.sql', 'bridge_delta_sigma_s_ieee754_bits INTEGER NOT NULL, bridge_delta_sigma_s_ieee754_class TEXT NOT NULL\n', 'bridge_delta_sigma_s_ieee754_bits INTEGER NOT NULL, bridge_delta_sigma_s_ieee754_class TEXT NOT NULL,\n relative_humidity_ieee754_bits INTEGER NOT NULL, relative_humidity_ieee754_class TEXT NOT NULL\n')
r = Q + '🦀️.rs'
sub(r, 'FloatColumn::Binary64(28)];', 'FloatColumn::Binary64(28),FloatColumn::Binary64(29)];')
sub(r, 'Cell::Real(v.bridge_delta_sigma_s)]', 'Cell::Real(v.bridge_delta_sigma_s),Cell::Real(v.relative_humidity)]')
sub(r, 'bridge_delta_sigma_s:v.real(28)?', 'bridge_delta_sigma_s:v.real(28)?,relative_humidity:v.real(29)?')
sub(r, '"en1992_member",29,MEMBER', '"en1992_member",30,MEMBER')
t = Q + '🟦️.ts'
sub(t, 'bridgeDeltaSigmaS:Binary64};', 'bridgeDeltaSigmaS:Binary64;relativeHumidity:Binary64};')
sub(t, '"bridgeSigmaC","bridgeDeltaSigmaS"]', '"bridgeSigmaC","bridgeDeltaSigmaS","relativeHumidity"]')
sub(t, 'v.bridgeSigmaC,v.bridgeDeltaSigmaS],MEMBER', 'v.bridgeSigmaC,v.bridgeDeltaSigmaS,v.relativeHumidity],MEMBER')
sub(t, 'bridgeDeltaSigmaS:readBinary64(v,28,MEMBER),', 'bridgeDeltaSigmaS:readBinary64(v,28,MEMBER),relativeHumidity:readBinary64(v,29,MEMBER),')
sub(t, 'MEMBER:readonly Ieee754Column[]=[11,12,13,14,15,17,19,22,23,24,25,26,27,28]', 'MEMBER:readonly Ieee754Column[]=[11,12,13,14,15,17,19,22,23,24,25,26,27,28,29]')
sub(t, 'entities(t[4]!,29,MEMBER,o)', 'entities(t[4]!,30,MEMBER,o)')
sub(Q + '🧪️tests/🦀️.rs', 'v.bridge_delta_sigma_s=w;', 'v.bridge_delta_sigma_s=w;v.relative_humidity=w;')
sub(Q + '🧪️tests/🧫️fixtures/🔣️.json', '        "bridgeDeltaSigmaS": 0.5\n', '        "bridgeDeltaSigmaS": 0.5,\n        "relativeHumidity": 0.5\n')
c = Q + '📏️cells/🦀️.rs'
sub(c, '("en1992_member",57)', '("en1992_member",60)')
sub(c, 'shape(value,32)?;\n   c.row("en1992_member"', 'shape(value,33)?;\n   c.row("en1992_member"')
sub(c, '(30,Float),(31,Float)],n)?)?,n)?;\n   collection(field(value,15)', '(30,Float),(31,Float),(32,Float)],n)?)?,n)?;\n   collection(field(value,15)')
n = 0
for p in (A / '🧪️tests/🧫️fixtures').rglob('*.json'):
    s = p.read_text()
    if 'bridgeDeltaSigmaS' not in s or 'relativeHumidity' in s: continue
    s2 = re.sub(r'^( *)("bridgeDeltaSigmaS": [^\n,]+)\n', r'\1\2,\n\1"relativeHumidity": 0.5\n', s, flags=re.M)
    p.write_text(s2); n += 1
print('fixtures', n)
