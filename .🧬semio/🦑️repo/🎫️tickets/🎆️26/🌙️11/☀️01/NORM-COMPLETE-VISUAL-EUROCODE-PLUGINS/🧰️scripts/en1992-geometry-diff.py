import json, pathlib, re
A = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any')
D = A / '🧬️schema/🔺️diff'
def rd(p): return p.read_text()
def rep(p, old, new, count=1):
    s = rd(p); n = s.count(old); assert n == count, (p.name, old[:90], n); p.write_text(s.replace(old, new))
assert 'SectionPoints' not in rd(D / '🦀️.rs'), 'applied'
MS = [('placement_x', 'placementX'), ('placement_y', 'placementY'), ('placement_z', 'placementZ'), ('placement_yaw', 'placementYaw')]
AS = [('head_diameter', 'headDiameter')] + MS
# lane: (field, camel, Pascal, Row, [(patch field, camel, rust type, kind)])
LANES = [
    ('section_points', 'sectionPoints', 'SectionPoints', 'SectionPoint', [('y', 'y', 'f64', 'n'), ('z', 'z', 'f64', 'n')]),
    ('bars', 'bars', 'Bars', 'BarPoint', [('layer_id', 'layerId', 'String', 's'), ('y', 'y', 'f64', 'n'), ('z', 'z', 'f64', 'n'), ('diameter', 'diameter', 'f64', 'n')]),
    ('tendon_points', 'tendonPoints', 'TendonPoints', 'TendonPoint', [('x', 'x', 'f64', 'n'), ('y', 'y', 'f64', 'n'), ('z', 'z', 'f64', 'n')]),
    ('stud_rings', 'studRings', 'StudRings', 'StudRing', [('radius', 'radius', 'f64', 'n'), ('count', 'count', 'u32', 'u'), ('diameter', 'diameter', 'f64', 'n')]),
]
M = 'semio_s_artifact_norm_en1992_model::'
r = D / '🦀️.rs'
lanes_rs = ''
for f, c, P, Row, flds in LANES:
    N = f'En1992Members{P}'
    lanes_rs += f'''/// 🩹️ Sparse patch of the `{f}` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct {N}Patch {{
''' + ''.join(f'    pub {a}: Option<{t}>,\n' for a, _, t, _ in flds) + f'''}}

impl protocol::list_delta::RowPatch<{M}{Row}> for {N}Patch {{
    fn commit_into(&self, row: &mut {M}{Row}, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {{
''' + ''.join(f'        if let Some(value) = &self.{a} {{\n            row.{a} = value.clone();\n        }}\n' for a, _, _, _ in flds) + '''        Ok(())
    }

    fn absorb(&mut self, later: Self) {
''' + ''.join(f'        if later.{a}.is_some() {{\n            self.{a} = later.{a};\n        }}\n' for a, _, _, _ in flds) + f'''    }}

    fn inverse(&self, row: &{M}{Row}) -> Self {{
        Self {{
''' + ''.join(f'            {a}: self.{a}.as_ref().map(|_| row.{a}.clone()),\n' for a, _, _, _ in flds) + '''        }
    }

    fn is_empty(&self) -> bool {
        ''' + ' && '.join(f'self.{a}.is_none()' for a, _, _, _ in flds) + f'''
    }}
}}

/// 🔑️ Key extractor of the foreign rows of `{N}Rows` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct {N}RowsKey;

impl protocol::list_delta::KeyOf<{M}{Row}> for {N}RowsKey {{
    type Key = String;
    fn key_of(row: &{M}{Row}) -> String {{
        row.id.clone()
    }}
}}

protocol::list_delta! {{
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `{f}` list (rows keyed by `id`).
    pub {N}Rows {{ removal: {N}Removed, insertion: {N}Inserted, relocation: {N}Moved, modification: {N}Modified, row: {M}{Row}, patch: {N}Patch, list: Vec<{M}{Row}>, key: String = by {N}RowsKey }}
}}

'''
rep(r, '/// 🩹️ Sparse patch of the `actions` row addressed by its key.', lanes_rs + '/// 🩹️ Sparse patch of the `actions` row addressed by its key.', 2) if rd(r).count('/// 🩹️ Sparse patch of the `actions` row addressed by its key.') == 2 else None
s = rd(r)
if 'En1992MembersSectionPointsPatch' not in s:
    k = s.index('/// 🩹️ Sparse patch of the `actions` row addressed by its key.')
    s = s[:k] + lanes_rs + s[k:]
s = s.replace('    pub count: Option<u32>,\n}', '    pub count: Option<u32>,\n    pub lap_share: Option<f64>,\n}', 1)
s = s.replace('''        if let Some(value) = &self.count {
            row.count = value.clone();
        }
        Ok(())''', '''        if let Some(value) = &self.count {
            row.count = value.clone();
        }
        if let Some(value) = &self.lap_share {
            row.lap_share = value.clone();
        }
        Ok(())''', 1)
s = s.replace('''        if later.count.is_some() {
            self.count = later.count;
        }
    }''', '''        if later.count.is_some() {
            self.count = later.count;
        }
        if later.lap_share.is_some() {
            self.lap_share = later.lap_share;
        }
    }''', 1)
s = s.replace('            count: self.count.as_ref().map(|_| row.count.clone()),\n        }', '            count: self.count.as_ref().map(|_| row.count.clone()),\n            lap_share: self.lap_share.as_ref().map(|_| row.lap_share.clone()),\n        }', 1)
s = s.replace('self.diameter.is_none() && self.count.is_none()\n', 'self.diameter.is_none() && self.count.is_none() && self.lap_share.is_none()\n', 1)
# anchors patch
s = s.replace('    pub h_ef: Option<f64>,\n    pub a_s: Option<f64>,\n}', '    pub h_ef: Option<f64>,\n    pub a_s: Option<f64>,\n' + ''.join(f'    pub {a}: Option<f64>,\n' for a, _ in AS) + '}', 1)
s = s.replace('''        if let Some(value) = &self.a_s {
            row.a_s = value.clone();
        }
        Ok(())''', '''        if let Some(value) = &self.a_s {
            row.a_s = value.clone();
        }
''' + ''.join(f'        if let Some(value) = &self.{a} {{\n            row.{a} = value.clone();\n        }}\n' for a, _ in AS) + '        Ok(())', 1)
s = s.replace('''        if later.a_s.is_some() {
            self.a_s = later.a_s;
        }
    }''', '''        if later.a_s.is_some() {
            self.a_s = later.a_s;
        }
''' + ''.join(f'        if later.{a}.is_some() {{\n            self.{a} = later.{a};\n        }}\n' for a, _ in AS) + '    }', 1)
s = s.replace('            a_s: self.a_s.as_ref().map(|_| row.a_s.clone()),\n        }', '            a_s: self.a_s.as_ref().map(|_| row.a_s.clone()),\n' + ''.join(f'            {a}: self.{a}.as_ref().map(|_| row.{a}.clone()),\n' for a, _ in AS) + '        }', 1)
s = s.replace('self.h_ef.is_none() && self.a_s.is_none()\n', 'self.h_ef.is_none() && self.a_s.is_none()' + ''.join(f' && self.{a}.is_none()' for a, _ in AS) + '\n', 1)
# member patch
s = s.replace('    pub relative_humidity: Option<f64>,\n', '    pub relative_humidity: Option<f64>,\n' + ''.join(f'    pub {a}: Option<f64>,\n' for a, _ in MS) + ''.join(f'    pub {f}: Option<En1992Members{P}Rows>,\n' for f, _, P, _, _ in LANES), 1)
k = '        if let Some(rows) = &self.longitudinal {\n            row.longitudinal = rows.commit_onto'
assert s.count(k) == 1
s = s.replace(k, ''.join(f'        if let Some(value) = &self.{a} {{\n            row.{a} = value.clone();\n        }}\n' for a, _ in MS) + ''.join(f'        if let Some(rows) = &self.{f} {{\n            row.{f} = rows.commit_onto(&row.{f}, capability).map_err(|error| error.under(["{c}"]))?;\n        }}\n' for f, c, _, _, _ in LANES) + k)
k = '        if let Some(theirs) = later.longitudinal {'
assert s.count(k) == 1
s = s.replace(k, ''.join(f'        if later.{a}.is_some() {{\n            self.{a} = later.{a};\n        }}\n' for a, _ in MS) + ''.join(f'        if let Some(theirs) = later.{f} {{\n            match self.{f}.as_mut() {{\n                Some(mine) => mine.absorb(theirs),\n                None => self.{f} = Some(theirs),\n            }}\n        }}\n' for f, _, _, _, _ in LANES) + k)
k = '        self.longitudinal = self.longitudinal.take().filter(|rows| !rows.is_empty());\n'
assert s.count(k) == 1
s = s.replace(k, ''.join(f'        self.{f} = self.{f}.take().filter(|rows| !rows.is_empty());\n' for f, _, _, _, _ in LANES) + k)
k = '            longitudinal: self.longitudinal.as_ref().map(|rows| rows.inverse'
assert s.count(k) == 1
s = s.replace(k, ''.join(f'            {a}: self.{a}.as_ref().map(|_| row.{a}.clone()),\n' for a, _ in MS) + ''.join(f'            {f}: self.{f}.as_ref().map(|rows| rows.inverse(&row.{f})).filter(|rows| !rows.is_empty()),\n' for f, _, _, _, _ in LANES) + k)
k = ' && self.longitudinal.as_ref().map_or(true, |rows| rows.is_empty())'
assert s.count(k) == 1
s = s.replace(k, ''.join(f' && self.{a}.is_none()' for a, _ in MS) + ''.join(f' && self.{f}.as_ref().map_or(true, |rows| rows.is_empty())' for f, _, _, _, _ in LANES) + k)
r.write_text(s)
# twins
SNAP = 'https://json.schemas.assets.semio-tech.com/s/norm/en1992/snapshot.json#/$defs/'
def ptype(k): return {'n': 'double', 's': 'string', 'u': 'uint32'}[k]
def gtype(k): return {'n': 'Float', 's': 'String', 'u': 'Int'}[k]
def ttype(k): return 'number' if k != 's' else 'string'
def tparse(k): return {'n': 'normWireNumber', 's': 'normWireString', 'u': 'normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})'}[k]
pr = D / '🛰️.proto'
msgs = ''
for f, c, P, Row, flds in LANES:
    N = f'En1992Members{P}'
    pf = '\n'.join(f'  optional {ptype(k)} {a} = {i + 1};' for i, (a, _, _, k) in enumerate(flds))
    msgs += f'message {N}Patch {{\n{pf}\n}}\n\nmessage {N}Removed {{\n  string id = 1;\n  uint64 index = 2;\n}}\n\nmessage {N}Inserted {{\n  uint64 index = 1;\n}}\n\nmessage {N}Moved {{\n  string id = 1;\n  uint64 from = 2;\n  uint64 to = 3;\n}}\n\nmessage {N}Modified {{\n  string id = 1;\n  {N}Patch patch = 2;\n}}\n\nmessage {N}Rows {{\n  repeated {N}Removed removed = 1;\n  repeated {N}Inserted inserted = 2;\n  repeated {N}Moved moved = 3;\n  repeated {N}Modified modified = 4;\n}}\n\n'
rep(pr, 'message En1992MembersPatch {', msgs + 'message En1992MembersPatch {')
rep(pr, '  optional double bridge_delta_sigma_s = 19;\n}', '  optional double bridge_delta_sigma_s = 19;\n' + ''.join(f'  optional double {a} = {20 + i};\n' for i, (a, _) in enumerate(MS)) + ''.join(f'  optional En1992Members{P}Rows {f} = {24 + i};\n' for i, (f, _, P, _, _) in enumerate(LANES)) + '}')
rep(pr, '  optional double a_s = 2;\n}', '  optional double a_s = 2;\n' + ''.join(f'  optional double {a} = {3 + i};\n' for i, (a, _) in enumerate(AS)) + '}')
rx = lambda p, pat, new: p.write_text(re.sub(pat, new, rd(p), count=1, flags=re.S))
rx(pr, r'(message En1992MembersLongitudinalPatch \{\n  optional double diameter = 1;\n  optional uint32 count = 2;\n)', r'\1  optional double lap_share = 3;\n')
gq = D / '🔗️.graphql'
types = ''
for f, c, P, Row, flds in LANES:
    N = f'En1992Members{P}'
    types += f'type {N}Patch {{\n' + ''.join(f'  {cc}: {gtype(k)}\n' for _, cc, _, k in flds) + f'}}\n\ntype {N}Removed {{\n  id: String!\n  index: Int!\n}}\n\ntype {N}Inserted {{\n  index: Int!\n  row: {Row}!\n}}\n\ntype {N}Moved {{\n  id: String!\n  from: Int!\n  to: Int!\n}}\n\ntype {N}Modified {{\n  id: String!\n  patch: {N}Patch!\n}}\n\ntype {N}Rows {{\n  removed: [{N}Removed!]!\n  inserted: [{N}Inserted!]!\n  moved: [{N}Moved!]!\n  modified: [{N}Modified!]!\n}}\n\n'
rep(gq, 'type En1992MembersPatch {', types + 'type En1992MembersPatch {')
rep(gq, '  bridgeDeltaSigmaS: Float\n', '  bridgeDeltaSigmaS: Float\n' + ''.join(f'  {c}: Float\n' for _, c in MS) + ''.join(f'  {c}: En1992Members{P}Rows\n' for _, c, P, _, _ in LANES))
rep(gq, 'type En1992AnchorsPatch {\n  hEf: Float\n  aS: Float\n', 'type En1992AnchorsPatch {\n  hEf: Float\n  aS: Float\n' + ''.join(f'  {c}: Float\n' for _, c in AS))
rx(gq, r'(type En1992MembersLongitudinalPatch \{\n  diameter: Float\n  count: Int\n)', r'\1  lapShare: Float\n')
ts = D / '🟦️.ts'
ifaces = ''; parsers = ''
for f, c, P, Row, flds in LANES:
    N = f'En1992Members{P}'
    ifaces += f'export interface {N}Inserted {{\n  index: number;\n  row: {Row};\n}}\n\nexport interface {N}Modified {{\n  id: string;\n  patch: {N}Patch;\n}}\n\nexport interface {N}Moved {{\n  id: string;\n  from: number;\n  to: number;\n}}\n\nexport interface {N}Patch {{\n' + ''.join(f'  {cc}: {ttype(k)} | null;\n' for _, cc, _, k in flds) + f'}}\n\nexport interface {N}Removed {{\n  id: string;\n  index: number;\n}}\n\nexport interface {N}Rows {{\n  removed: {N}Removed[];\n  inserted: {N}Inserted[];\n  moved: {N}Moved[];\n  modified: {N}Modified[];\n}}\n\n'
    I = '{"minimum":0}'
    parsers += (f'export const parse{N}Inserted: NormWireReader<{N}Inserted> = normWireObject<{N}Inserted>({{ index: normWireRequired(normWireRange(normWireInteger, {I})), row: normWireRequired(parse{Row}) }});\n'
        f'export const parse{N}Modified: NormWireReader<{N}Modified> = normWireObject<{N}Modified>({{ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parse{N}Patch)) }});\n'
        f'export const parse{N}Moved: NormWireReader<{N}Moved> = normWireObject<{N}Moved>({{ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {I})), to: normWireRequired(normWireRange(normWireInteger, {I})) }});\n'
        f'export const parse{N}Patch: NormWireReader<{N}Patch> = normWireObject<{N}Patch>({{ ' + ', '.join(f'{cc}: normWireRequired(normWireNullable({tparse(k)}))' for _, cc, _, k in flds) + ' });\n'
        f'export const parse{N}Removed: NormWireReader<{N}Removed> = normWireObject<{N}Removed>({{ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {I})) }});\n'
        f'export const parse{N}Rows: NormWireReader<{N}Rows> = normWireObject<{N}Rows>({{ removed: normWireRequired(normWireArray(normWireRef(() => parse{N}Removed))), inserted: normWireRequired(normWireArray(normWireRef(() => parse{N}Inserted))), moved: normWireRequired(normWireArray(normWireRef(() => parse{N}Moved))), modified: normWireRequired(normWireArray(normWireRef(() => parse{N}Modified))) }});\n')
s_ = rd(ts)
i = s_.index('export interface En1992MembersLongitudinalInserted'); s_ = s_[:i] + ifaces + s_[i:]
i = s_.index('export const parseEn1992MembersLongitudinalInserted'); s_ = s_[:i] + parsers + s_[i:]
ts.write_text(s_)
rep(ts, '  bridgeDeltaSigmaS: number | null;\n', '  bridgeDeltaSigmaS: number | null;\n' + ''.join(f'  {c}: number | null;\n' for _, c in MS) + ''.join(f'  {c}: En1992Members{P}Rows | null;\n' for _, c, P, _, _ in LANES))
rep(ts, 'bridgeDeltaSigmaS: normWireRequired(normWireNullable(normWireNumber)), ', 'bridgeDeltaSigmaS: normWireRequired(normWireNullable(normWireNumber)), ' + ''.join(f'{c}: normWireRequired(normWireNullable(normWireNumber)), ' for _, c in MS) + ''.join(f'{c}: normWireRequired(normWireNullable(normWireRef(() => parseEn1992Members{P}Rows))), ' for _, c, P, _, _ in LANES))
rep(ts, '  hEf: number | null;\n  aS: number | null;\n}', '  hEf: number | null;\n  aS: number | null;\n' + ''.join(f'  {c}: number | null;\n' for _, c in AS) + '}')
s_ = rd(ts)
m = re.search(r'(export const parseEn1992AnchorsPatch: [^\n]*?aS: normWireRequired\(normWireNullable\(normWireNumber\)\))( \}\);)', s_)
assert m
s_ = s_[:m.end(1)] + ''.join(f', {c}: normWireRequired(normWireNullable(normWireNumber))' for _, c in AS) + s_[m.end(1):]
s_ = s_.replace('  diameter: number | null;\n  count: number | null;\n}', '  diameter: number | null;\n  count: number | null;\n  lapShare: number | null;\n}', 1)
m = re.search(r'(export const parseEn1992MembersLongitudinalPatch: [^\n]*?count: normWireRequired\(normWireNullable\(normWireRange\(normWireInteger, \{"minimum":0,"maximum":4294967295\}\)\)\))( \}\);)', s_)
assert m
s_ = s_[:m.end(1)] + ', lapShare: normWireRequired(normWireNullable(normWireNumber))' + s_[m.end(1):]
ts.write_text(s_)
jp = D / '🔣️.json'
d = json.loads(rd(jp)); defs = d['$defs']
def nullable(ref): return {'anyOf': [{'$ref': ref}, {'type': 'null'}]}
for f, c, P, Row, flds in LANES:
    N = f'En1992Members{P}'
    defs[N + 'Inserted'] = {'type': 'object', 'additionalProperties': False, 'required': ['index', 'row'], 'properties': {'index': {'type': 'integer', 'minimum': 0}, 'row': {'$ref': SNAP + Row}}}
    defs[N + 'Modified'] = {'type': 'object', 'additionalProperties': False, 'required': ['id', 'patch'], 'properties': {'id': {'type': 'string'}, 'patch': {'$ref': f'#/$defs/{N}Patch'}}}
    defs[N + 'Moved'] = {'type': 'object', 'additionalProperties': False, 'required': ['id', 'from', 'to'], 'properties': {'id': {'type': 'string'}, 'from': {'type': 'integer', 'minimum': 0}, 'to': {'type': 'integer', 'minimum': 0}}}
    defs[N + 'Patch'] = {'type': 'object', 'additionalProperties': False, 'required': [cc for _, cc, _, _ in flds], 'properties': {cc: nullable(SNAP + f'{Row}/properties/{cc}') for _, cc, _, _ in flds}}
    defs[N + 'Removed'] = {'type': 'object', 'additionalProperties': False, 'required': ['id', 'index'], 'properties': {'id': {'type': 'string'}, 'index': {'type': 'integer', 'minimum': 0}}}
    defs[N + 'Rows'] = {'type': 'object', 'additionalProperties': False, 'required': ['removed', 'inserted', 'moved', 'modified'], 'properties': {k: {'type': 'array', 'items': {'$ref': f'#/$defs/{N}{k.capitalize()}'}} for k in ['removed', 'inserted', 'moved', 'modified']}}
mp = defs['En1992MembersPatch']
keys = list(mp['properties']); i = keys.index('longitudinal')
newprops = {}
for k in keys:
    if k == 'longitudinal':
        for _, c in MS: newprops[c] = nullable(SNAP + f'RcMember/properties/{c}')
        for _, c, P, _, _ in LANES: newprops[c] = nullable(f'#/$defs/En1992Members{P}Rows')
    newprops[k] = mp['properties'][k]
mp['properties'] = newprops
req = mp['required']; j = req.index('longitudinal'); mp['required'] = req[:j] + [c for _, c in MS] + [c for _, c, _, _, _ in LANES] + req[j:]
ap = defs['En1992AnchorsPatch']
for _, c in AS: ap['required'].append(c); ap['properties'][c] = nullable(SNAP + f'Anchor/properties/{c}')
bp = defs['En1992MembersLongitudinalPatch']; bp['required'].append('lapShare'); bp['properties']['lapShare'] = nullable(SNAP + 'BarLayer/properties/lapShare')
d['$defs'] = {k: defs[k] for k in sorted(defs)}
jp.write_text(json.dumps(d, indent=2, ensure_ascii=False) + '\n')
# fixtures: add null keys to every patch
def fix(o):
    if isinstance(o, dict):
        if 'patch' in o and isinstance(o['patch'], dict):
            p = o['patch']
            if 'stirrupsSpacing' in p or 'longitudinal' in p:
                new = {}
                for k, v in p.items():
                    if k == 'longitudinal':
                        for _, c in MS: new.setdefault(c, None)
                        for _, c, _, _, _ in LANES: new.setdefault(c, None)
                    new[k] = v
                o['patch'] = new
            elif 'hEf' in p:
                for _, c in AS: p.setdefault(c, None)
            elif 'diameter' in p and 'count' in p:
                p.setdefault('lapShare', None)
        for v in o.values(): fix(v)
    elif isinstance(o, list):
        for v in o: fix(v)
n = 0
for p in (A / '🧪️tests/🧫️fixtures').rglob('🔣️.diff.json'):
    s0 = rd(p); d0 = json.loads(s0); fix(d0)
    s1 = json.dumps(d0, indent=2, ensure_ascii=False) + '\n'
    if s1 != s0: p.write_text(s1); n += 1
print('diff done, fixtures', n)
