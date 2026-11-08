"""Generates the per-row mutation leaves of DIN V 18599 (insert/remove/change for zones and envelope elements), their crates, schema twins, wire registrations, vectors and oracle registrations, and removes `replace-zones` / `replace-elements`.

Run once from anywhere: `python3 gen-row-leaves.py`. Idempotent for the generated files; the registration edits assert their anchors.
"""
import json
import os
import re
import shutil

ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599'
SUB = f'{ART}/🏅️standards/🔖️1/🪆️subsets/✳️any'
MUT = f'{SUB}/🧬️schema/🧬️mutations'
FIX = f'{SUB}/🧪️tests/🧫️fixtures/🧬️mutations'
TAXONOMY = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json'
OWNER = '✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations'
SCHEMA_BASE = 'https://json.schemas.assets.semio-tech.com/s/norm/din18599'
FIRST_TAG = 20

SNAPSHOT = json.load(open(f'{SUB}/🧬️schema/📸️snapshot/🔣️.json', encoding='utf-8'))
BASE = json.load(open(f'{FIX}/🌡️change-element-u/✅apply/🔣️.before.json', encoding='utf-8'))


def rd(p):
    return open(p, encoding='utf-8').read()


def wr(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf-8').write(t)


def dump(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + '\n'


def pascal(slug):
    return ''.join(part.capitalize() for part in slug.split('-'))


def snake(slug):
    return slug.replace('-', '_')


def camel(slug):
    p = pascal(slug)
    return p[0].lower() + p[1:]


ENTITIES = {
    'zone': dict(coll='zones', row='ThermalZone', rows='Din18599ZonesRows', patch='Din18599ZonesPatch', defs='ThermalZone', label=('Thermal zone', 'Thermische Zone'), noun_en='zone', noun_de='Zone', crate_rows='zone-rows', crate_fields='zone-fields'),
    'element': dict(coll='elements', row='EnvelopeElement', rows='Din18599ElementsRows', patch='Din18599ElementsPatch', defs='EnvelopeElement', label=('Envelope element', 'Bauteil der Gebäudehülle'), noun_en='element', noun_de='Bauteil', crate_rows='element-rows', crate_fields='element-fields'),
}

# (slug, rust field, json key, rust type, schema kind, action en, action de, crate bucket)
FIELDS = {
    'zone': [
        ('label-en', 'label_en', 'labelEn', 'String', 'string', 'Change zone label (English)', 'Zonenbezeichnung (Englisch) ändern', 'rows'),
        ('label-de', 'label_de', 'labelDe', 'String', 'string', 'Change zone label (German)', 'Zonenbezeichnung (Deutsch) ändern', 'rows'),
        ('usage-profile', 'usage_profile', 'usageProfile', 'UsageProfile', 'enum:UsageProfile', 'Change zone usage profile', 'Nutzungsprofil der Zone ändern', 'rows'),
        ('occupants', 'occupants', 'occupants', 'u32', 'integer', 'Change zone occupants', 'Personenzahl der Zone ändern', 'rows'),
        ('area-m2', 'area_m2', 'areaM2', 'f64', 'number', 'Change zone floor area', 'Nettogrundfläche der Zone ändern', 'fields'),
        ('volume-m3', 'volume_m3', 'volumeM3', 'f64', 'number', 'Change zone air volume', 'Nettoluftvolumen der Zone ändern', 'fields'),
        ('theta-i-heat-c', 'theta_i_heat_c', 'thetaIHeatC', 'f64', 'number', 'Change zone heating set-point', 'Heiz-Solltemperatur der Zone ändern', 'fields'),
        ('theta-i-cool-c', 'theta_i_cool_c', 'thetaICoolC', 'f64', 'number', 'Change zone cooling set-point', 'Kühl-Solltemperatur der Zone ändern', 'fields'),
        ('internal-gains-w-m2', 'internal_gains_w_m2', 'internalGainsWM2', 'f64', 'number', 'Change zone internal gains', 'Interne Gewinne der Zone ändern', 'fields'),
        ('lighting-power-w-m2', 'lighting_power_w_m2', 'lightingPowerWM2', 'f64', 'number', 'Change zone lighting power', 'Beleuchtungsleistung der Zone ändern', 'fields'),
    ],
    'element': [
        ('label-en', 'label_en', 'labelEn', 'String', 'string', 'Change element label (English)', 'Bauteilbezeichnung (Englisch) ändern', 'rows'),
        ('label-de', 'label_de', 'labelDe', 'String', 'string', 'Change element label (German)', 'Bauteilbezeichnung (Deutsch) ändern', 'rows'),
        ('kind', 'kind', 'kind', 'ElementKind', 'enum:ElementKind', 'Change element kind', 'Bauteilart ändern', 'rows'),
        ('zone-id', 'zone_id', 'zoneId', 'String', 'reference:zone', 'Move element to another zone', 'Bauteil in eine andere Zone verschieben', 'rows'),
        ('area-m2', 'area_m2', 'areaM2', 'f64', 'number', 'Change element area', 'Bauteilfläche ändern', 'fields'),
        ('orientation-deg', 'orientation_deg', 'orientationDeg', 'f64', 'number', 'Change element orientation', 'Bauteilorientierung ändern', 'fields'),
        ('tilt-deg', 'tilt_deg', 'tiltDeg', 'f64', 'number', 'Change element tilt', 'Bauteilneigung ändern', 'fields'),
        ('g-value', 'g_value', 'gValue', 'f64', 'number', 'Change element g-value', 'g-Wert des Bauteils ändern', 'fields'),
        ('fc', 'fc', 'fc', 'f64', 'number', 'Change element shading factor', 'Abminderungsfaktor des Bauteils ändern', 'fields'),
        ('adjacency', 'adjacency', 'adjacency', 'Adjacency', 'enum:Adjacency', 'Change element adjacency', 'Angrenzung des Bauteils ändern', 'fields'),
    ],
}

NEW_VALUES = {
    ('zone', 'labelEn'): 'Living room', ('zone', 'labelDe'): 'Wohnzimmer', ('zone', 'usageProfile'): 'Office', ('zone', 'occupants'): 5,
    ('zone', 'areaM2'): 150.0, ('zone', 'volumeM3'): 390.0, ('zone', 'thetaIHeatC'): 21.0, ('zone', 'thetaICoolC'): 25.0,
    ('zone', 'internalGainsWM2'): 4.0, ('zone', 'lightingPowerWM2'): 7.5,
    ('element', 'labelEn'): 'Northern wall', ('element', 'labelDe'): 'Nordwand außen', ('element', 'kind'): 'Door', ('element', 'zoneId'): 'zone-annex',
    ('element', 'areaM2'): 40.0, ('element', 'orientationDeg'): 45.0, ('element', 'tiltDeg'): 80.0, ('element', 'gValue'): 0.5, ('element', 'fc'): 0.8, ('element', 'adjacency'): 'Unheated',
}

TARGET = {'zone': 'zone-living', 'element': 'wall-n'}

# (entity, json key) -> (minimum, maximum, minimum is exclusive): the physical domain of the field; absent means unbounded.
BOUNDS = {
    ('zone', 'areaM2'): (0.0, 1.0e6, True), ('zone', 'volumeM3'): (0.0, 1.0e7, True),
    ('zone', 'thetaIHeatC'): (10.0, 30.0, False), ('zone', 'thetaICoolC'): (18.0, 35.0, False),
    ('zone', 'occupants'): (0, 10000, False),
    ('zone', 'internalGainsWM2'): (0.0, 100.0, False), ('zone', 'lightingPowerWM2'): (0.0, 100.0, False),
    ('element', 'areaM2'): (0.0, 1.0e5, True), ('element', 'orientationDeg'): (0.0, 360.0, False), ('element', 'tiltDeg'): (0.0, 180.0, False),
    ('element', 'gValue'): (0.0, 1.0, False), ('element', 'fc'): (0.0, 1.0, False),
}

# out-of-domain value of the rejected vector
REJECTED = {
    ('zone', 'areaM2'): -1.0, ('zone', 'volumeM3'): -1.0, ('zone', 'thetaIHeatC'): 45.0, ('zone', 'thetaICoolC'): 5.0, ('zone', 'occupants'): 20000,
    ('zone', 'internalGainsWM2'): -1.0, ('zone', 'lightingPowerWM2'): -1.0,
    ('element', 'areaM2'): -1.0, ('element', 'orientationDeg'): 400.0, ('element', 'tiltDeg'): 200.0, ('element', 'gValue'): 1.5, ('element', 'fc'): -0.1,
}

EMOJI = {}
_pool = ['🟠', '🟡', '🟢', '🔵', '🟣', '🟤', '🔴', '🟥', '🟧', '🟨', '🟩', '🟦', '🟪', '🟫', '⬜', '🔶', '🔷', '🔸', '🔹', '🔻']
EMOJI.update({'insert-zone': '➕', 'remove-zone': '➖', 'insert-element': '🆕', 'remove-element': '🚮'})


class Leaf:
    def __init__(self, entity, verb, slug, bucket, field=None):
        self.entity, self.verb, self.field, self.bucket = entity, verb, field, bucket
        self.e = ENTITIES[entity]
        self.kind = f'{verb}-{entity}' if verb != 'change' else f'change-{entity}-{slug}'
        self.struct = pascal(self.kind)
        self.mod = snake(self.kind)
        if self.kind not in EMOJI:
            EMOJI[self.kind] = _pool.pop(0)
        self.dir = EMOJI[self.kind] + self.kind
        self.crate = self.e['crate_rows'] if bucket == 'rows' else self.e['crate_fields']
        self.camel = camel(self.struct)


LEAVES = []
for entity in ('zone', 'element'):
    LEAVES.append(Leaf(entity, 'insert', None, 'rows'))
    LEAVES.append(Leaf(entity, 'remove', None, 'rows'))
    for f in FIELDS[entity]:
        LEAVES.append(Leaf(entity, 'change', f[0], f[7], f))
CRATES = ['zone-rows', 'zone-fields', 'element-rows', 'element-fields']
CRATE_DIR = {'zone-rows': '🔰zone-rows', 'zone-fields': '🔰zone-fields', 'element-rows': '🔱element-rows', 'element-fields': '🔱element-fields'}
CRATE_DESC = {'zone-rows': 'zone row insert, remove and label leaves', 'zone-fields': 'zone numeric field leaves', 'element-rows': 'envelope element row insert, remove and label leaves', 'element-fields': 'envelope element numeric field leaves'}

DERIVE = 'dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone'
HEAD = f'''#[derive(Clone, Debug, PartialEq, {DERIVE})]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]'''


def rust_type(t):
    return {'UsageProfile': 'semio_s_artifact_norm_din18599_model::UsageProfile', 'ElementKind': 'semio_s_artifact_norm_din18599_model::ElementKind', 'Adjacency': 'semio_s_artifact_norm_din18599_model::Adjacency'}.get(t, t)


def partner(leaf):
    other = 'remove' if leaf.verb == 'insert' else 'insert'
    return f'{other}-{leaf.entity}'


def leaf_rs(l):
    e = l.e
    partner_bound = ''
    froms = f'From<crate::{l.mod}::{l.struct}>'
    if l.verb in ('insert', 'remove'):
        p = partner(l)
        froms = f'From<crate::{snake(p)}::{pascal(p)}>'
    if l.verb == 'change':
        f = l.field
        id_field = f'{l.entity}_id'
        payload = f'    pub {id_field}: String,\n    pub new_{f[1]}: {rust_type(f[3])},\n'
        label_en, label_de, entity_slug, record = f[5], f[6], f'{l.entity}-{f[0]}', 'Changed' + pascal(f'{l.entity}-{f[0]}')
        verb = 'change'
    elif l.verb == 'insert':
        payload = f'    pub index: usize,\n    pub {l.entity}: semio_s_artifact_norm_din18599_model::{e["row"]},\n'
        label_en, label_de = f'Insert {e["noun_en"]}', f'{e["noun_de"]} einfügen'
        entity_slug, record, verb = l.entity, 'Inserted' + pascal(l.entity), 'insert'
    else:
        payload = '    pub id: String,\n'
        label_en, label_de = f'Remove {e["noun_en"]}', f'{e["noun_de"]} entfernen'
        entity_slug, record, verb = l.entity, 'Removed' + pascal(l.entity), 'remove'
    return f'''//! 🧬️ `{l.kind}` mutation leaf.

use semio_s_artifact_norm_din18599_diff::Din18599Diff;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

{HEAD}
pub struct {l.struct} {{
{payload}}}

impl<Op> protocol::MutationKind<Din18599Snapshot, Op> for {l.struct} where Op: protocol::Mutation<Din18599Snapshot, Diff = semio_s_artifact_norm_din18599_diff::Din18599Diff> + {froms} {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{
        verb: "{verb}",
        entity: "{entity_slug}",
        kind: "{l.kind}",
        record: "{record}",
    }};

    fn label(&self) -> semio_ui_locale::LocalizedLabel {{
        semio_ui_locale::LocalizedLabel::native("{label_en}", "{label_de}")
    }}
}}

impl<Op> protocol::ColdMutationKind<Din18599Snapshot, Op> for {l.struct} where Op: protocol::ColdMutation<Din18599Snapshot, ReferenceDiff = semio_s_artifact_norm_din18599_diff::Din18599Diff> + {froms} {{
    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {{
        crate::{l.mod}::diff::diff(self, base)
    }}
    fn inverse(&self, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {{
        crate::{l.mod}::inverse::inverse(self, base)
    }}
}}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
'''


def diff_rs(l):
    e = l.e
    coll, rows, patch = e['coll'], e['rows'], e['patch']
    if l.verb == 'change':
        f = l.field
        idf = f'{l.entity}_id'
        clone = '.clone()' if f[3] == 'String' else ''
        what = f'{e["noun_en"].capitalize()} {f[0].replace("-", " ")}'
        bound = BOUNDS.get((l.entity, f[2]))
        guard = ''
        if bound:
            lo, hi, exclusive = bound
            lo_s, hi_s = (repr(float(lo)), repr(float(hi))) if f[3] == 'f64' else (str(int(lo)), str(int(hi)))
            low = f'payload.new_{f[1]} {"<=" if exclusive else "<"} {lo_s}'
            finite = f'!payload.new_{f[1]}.is_finite() || ' if f[3] == 'f64' else ''
            rng = f'{"above" if exclusive else "from"} {lo:g} to {hi:g}'
            below = '' if (f[3] != 'f64' and lo == 0) else f'{low} || '
            guard = f'''    if {finite}{below}payload.new_{f[1]} > {hi_s} {{
        return protocol::MutationOutcome::fatal("mutation.invariant", "{f[0]} must lie {rng}.", Vec::<String>::new());
    }}
'''
        return f'''//! 🔺️ `{l.kind}` sparse diff.

use crate::{l.mod}::{l.struct};
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{{Din18599Diff, {rows}, {patch}}};

pub fn diff(payload: &{l.struct}, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {{
{guard}    let Some(row) = base.{coll}.iter().find(|row| row.id == payload.{idf}) else {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{e["noun_en"].capitalize()} \\"{{}}\\" does not exist.", payload.{idf}), [payload.{idf}.clone()]);
    }};
    if row.{f[1]} == payload.new_{f[1]} {{
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "{what} already has this value.");
    }}
    protocol::MutationOutcome::new(Din18599Diff {{
        {coll}: Some({rows}::modification(&payload.{idf}, {patch} {{ {f[1]}: Some(payload.new_{f[1]}{clone}), ..Default::default() }})),
        ..Default::default()
    }})
}}
'''
    if l.verb == 'insert':
        return f'''//! 🔺️ `{l.kind}` sparse diff: one inserted row.

use crate::{l.mod}::{l.struct};
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{{Din18599Diff, {rows}}};

pub fn diff(payload: &{l.struct}, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {{
    if payload.index > base.{coll}.len() {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Position {{}} is beyond the {coll}.", payload.index), [payload.index.to_string()]);
    }}
    if base.{coll}.iter().any(|row| row.id == payload.{l.entity}.id) {{
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{e["noun_en"].capitalize()} id {{}} already exists.", payload.{l.entity}.id), [payload.{l.entity}.id.clone()]);
    }}
    protocol::MutationOutcome::new(Din18599Diff {{ {coll}: Some({rows}::insertion(payload.index, payload.{l.entity}.clone())), ..Default::default() }})
}}
'''
    return f'''//! 🔺️ `{l.kind}` sparse diff: one removed row.

use crate::{l.mod}::{l.struct};
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{{Din18599Diff, {rows}}};

pub fn diff(payload: &{l.struct}, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {{
    let Some(index) = base.{coll}.iter().position(|row| row.id == payload.id) else {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{e["noun_en"].capitalize()} \\"{{}}\\" does not exist.", payload.id), [payload.id.clone()]);
    }};
    protocol::MutationOutcome::new(Din18599Diff {{ {coll}: Some({rows}::removal(&base.{coll}, index)), ..Default::default() }})
}}
'''


def inverse_rs(l):
    e = l.e
    coll = e['coll']
    if l.verb == 'change':
        f = l.field
        idf = f'{l.entity}_id'
        clone = '.clone()' if f[3] == 'String' else ''
        return f'''//! ↩️ `{l.kind}` inverse.

use crate::{l.mod}::{l.struct};

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &{l.struct}, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{l.mod}::{l.struct}> {{
    Ok(base.{coll}.iter().find(|row| row.id == payload.{idf}).map(|row| vec![Op::from({l.struct} {{ {idf}: payload.{idf}.clone(), new_{f[1]}: row.{f[1]}{clone} }})]).unwrap_or_default())
}}
'''
    p = partner(l)
    pm, ps = snake(p), pascal(p)
    if l.verb == 'insert':
        return f'''//! ↩️ `{l.kind}` inverse: the removal of the inserted row.

use crate::{l.mod}::{l.struct};
use crate::{pm}::{ps};

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &{l.struct}, _base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<{ps}> {{
    Ok(vec![Op::from({ps} {{ id: payload.{l.entity}.id.clone() }})])
}}
'''
    return f'''//! ↩️ `{l.kind}` inverse: the row back at its position.

use crate::{l.mod}::{l.struct};
use crate::{pm}::{ps};

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &{l.struct}, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<{ps}> {{
    Ok(base.{coll}.iter().position(|row| row.id == payload.id).map(|index| vec![Op::from({ps} {{ index, {l.entity}: base.{coll}[index].clone() }})]).unwrap_or_default())
}}
'''


def ui_of(entity, json_key):
    return SNAPSHOT['$defs'][ENTITIES[entity]['defs']]['properties'][json_key].get('x-semio-ui', {})


def schema_json(l):
    e = l.e
    ref_base = f'{SCHEMA_BASE}/snapshot/schema.json#/$defs'
    props = {'mutation': {'const': l.camel}}
    required = ['mutation']
    group = l.entity
    if l.verb == 'change':
        f = l.field
        idk = f'{l.entity}Id'
        props[idk] = {'type': 'string', 'x-semio-ui': {'widget': 'reference', 'role': 'target', 'label': {'en': e['label'][0], 'de': e['label'][1]}, 'description': {'en': f'The {e["noun_en"]} this mutation addresses.', 'de': f'Das Objekt ({e["noun_de"]}), das diese Mutation adressiert.'}, 'ref': {'kind': l.entity}, 'group': 'target', 'order': 10}}
        base_ui = dict(ui_of(l.entity, f[2]))
        label = base_ui.pop('label', {'en': f[5], 'de': f[6]})
        ui = {'role': 'value', 'label': label}
        for k in ('description', 'widget', 'unit', 'step', 'precision'):
            if k in base_ui:
                ui[k] = base_ui[k]
        ui.update({'group': group, 'order': 20})
        kind = f[4]
        if kind.startswith('enum:'):
            slot = {'$ref': f'{ref_base}/{kind[5:]}'}
        elif kind.startswith('reference:'):
            slot = {'type': 'string'}
            ui.update({'widget': 'reference', 'ref': {'kind': kind[10:]}})
        elif kind == 'integer':
            slot = {'type': 'integer'}
        else:
            slot = {'type': kind}
        bound = BOUNDS.get((l.entity, f[2]))
        if bound:
            lo, hi, exclusive = bound
            slot['exclusiveMinimum' if exclusive else 'minimum'] = lo
            slot['maximum'] = hi
        slot['x-semio-ui'] = ui
        props['new' + f[2][0].upper() + f[2][1:]] = slot
        required += [idk, 'new' + f[2][0].upper() + f[2][1:]]
        required = required
    elif l.verb == 'insert':
        props['index'] = {'type': 'integer', 'minimum': 0, 'x-semio-ui': {'widget': 'stepper', 'role': 'value', 'label': {'en': 'Position', 'de': 'Position'}, 'description': {'en': 'Zero-based list position at which the record is inserted.', 'de': 'Nullbasierte Listenposition, an der der Datensatz eingefügt wird.'}, 'step': 1, 'precision': 0, 'group': 'target', 'order': 10}}
        props[l.entity] = {'$ref': f'{ref_base}/{e["defs"]}', 'x-semio-ui': {'role': 'value', 'label': {'en': e['label'][0], 'de': e['label'][1]}, 'description': {'en': f'The complete {e["noun_en"]} record to insert.', 'de': f'Der vollständige einzufügende Datensatz ({e["noun_de"]}).'}, 'group': 'record', 'order': 20}}
        required += ['index', l.entity]
    else:
        props['id'] = {'type': 'string', 'x-semio-ui': {'widget': 'reference', 'role': 'target', 'label': {'en': e['label'][0], 'de': e['label'][1]}, 'description': {'en': f'The {e["noun_en"]} this mutation removes, by id.', 'de': f'Das zu entfernende Objekt ({e["noun_de"]}), per Kennung.'}, 'ref': {'kind': l.entity}, 'group': 'target', 'order': 10}}
        required += ['id']
    doc = {'$schema': 'http://json-schema.org/draft-07/schema#', '$id': f'{SCHEMA_BASE}/mutation/{l.kind}/schema.json', 'title': l.struct, 'type': 'object', 'additionalProperties': False, 'required': required, 'properties': props}
    return dump(doc)


def ts_twin(l):
    e = l.e
    snap = '"../../../📸️snapshot/🟦️.ts"'
    tag = f'mutation: normWireRequired(normWireLiteral("{l.camel}"))'
    if l.verb == 'change':
        f = l.field
        idk = f'{l.entity}Id'
        nk = 'new' + f[2][0].upper() + f[2][1:]
        kind = f[4]
        names = {'normWireLiteral', 'normWireObject', 'normWireRequired', 'normWireString'}
        extra_import = ''
        if kind.startswith('enum:'):
            tn = 'Din18599' + kind[5:]
            ts_t, reader = tn, f'parse{tn}'
            extra_import = f'import {{ type {tn}, parse{tn} }} from {snap};\n'
        elif kind == 'number':
            ts_t, reader = 'number', 'normWireNumber'
            names.add('normWireNumber')
        elif kind == 'integer':
            ts_t, reader = 'number', 'normWireRange(normWireInteger, {"minimum":0})'
            names |= {'normWireInteger', 'normWireRange'}
        else:
            ts_t, reader = 'string', 'normWireString'
        imports = ', '.join(sorted(names | {'type NormWireReader'}, key=lambda s: s.replace('type ', '')))
        return f'''/** {EMOJI[l.kind]} `{l.kind}` wire twin: the leaf payload `{l.struct}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ {imports} }} from "@semio-tech/norm-js/registry/contract";
{extra_import}
export interface {l.struct} {{
  mutation: "{l.camel}";
  {idk}: string;
  {nk}: {ts_t};
}}

export const parse{l.struct}: NormWireReader<{l.struct}> = normWireObject<{l.struct}>({{ {tag}, {idk}: normWireRequired(normWireString), {nk}: normWireRequired({reader}) }});
'''
    if l.verb == 'insert':
        return f'''/** {EMOJI[l.kind]} `{l.kind}` wire twin: the leaf payload `{l.struct}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired }} from "@semio-tech/norm-js/registry/contract";
import {{ type {e["row"]}, parse{e["row"]} }} from {snap};

export interface {l.struct} {{
  mutation: "{l.camel}";
  index: number;
  {l.entity}: {e["row"]};
}}

export const parse{l.struct}: NormWireReader<{l.struct}> = normWireObject<{l.struct}>({{ {tag}, index: normWireRequired(normWireRange(normWireInteger, {{"minimum":0}})), {l.entity}: normWireRequired(parse{e["row"]}) }});
'''
    return f'''/** {EMOJI[l.kind]} `{l.kind}` wire twin: the leaf payload `{l.struct}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString }} from "@semio-tech/norm-js/registry/contract";

export interface {l.struct} {{
  mutation: "{l.camel}";
  id: string;
}}

export const parse{l.struct}: NormWireReader<{l.struct}> = normWireObject<{l.struct}>({{ {tag}, id: normWireRequired(normWireString) }});
'''


def meta_json(l, tag):
    return dump({
        'schemaVersion': 1, 'owner': f'{OWNER}/{l.dir}', 'semanticKind': l.kind, 'displayName': ' '.join(p.capitalize() for p in l.kind.split('-')),
        'emoji': EMOJI[l.kind], 'aggregateVariant': l.struct, 'payloadSchema': '🧬️schema/🔣️.json', 'textOpcode': None, 'binaryTag': tag,
        'invertibility': 'explicit-mutation', 'diffParticipation': 'detect', 'outcomeClasses': ['applied', 'no-op'], 'composition': 'atomic',
        'requiredLanguageSurfaces': ['rust', 'json-schema', 'text', 'binary'],
    })


def crate_files():
    for c in CRATES:
        leaves = [l for l in LEAVES if l.crate == c]
        d = f'{MUT}/{CRATE_DIR[c]}'
        mounts = ''.join(f'#[path = "../{l.dir}/🦀️.rs"]\npub mod {l.mod};\n' for l in leaves)
        wr(f'{d}/🦀️.rs', f'''//! 🔰 DIN V 18599 {CRATE_DESC[c]}.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
{mounts}//#endregion 🪢️Mounts
''')
        zoning = rd(f'{MUT}/🗺️zoning/📦️packages/🦀️rust/Cargo.toml')
        cargo = zoning.replace('semio-s-artifact-norm-din18599-mut-zoning', f'semio-s-artifact-norm-din18599-mut-{c}').replace('DIN V 18599 zoning mutation leaves', f'DIN V 18599 {CRATE_DESC[c]}')
        wr(f'{d}/📦️packages/🦀️rust/Cargo.toml', cargo)


def leaf_files():
    tag = FIRST_TAG
    tags = {}
    for l in LEAVES:
        d = f'{MUT}/{l.dir}'
        wr(f'{d}/🦀️.rs', leaf_rs(l))
        wr(f'{d}/🔺️diff/🦀️.rs', diff_rs(l))
        wr(f'{d}/↩️inverse/🦀️.rs', inverse_rs(l))
        wr(f'{d}/🧬️schema/🔣️.json', schema_json(l))
        wr(f'{d}/🧬️schema/🟦️.ts', ts_twin(l))
        wr(f'{d}/🔣️.json', meta_json(l, tag))
        tags[l.kind] = tag
        tag += 1
    return tags


def vector(l):
    before = json.loads(json.dumps(BASE))
    after = json.loads(json.dumps(BASE))
    mutation = {'mutation': l.camel}
    diff = {k: None for k in ('buildingCategory', 'attachment', 'useClass', 'method', 'netFloorAreaM2', 'heatedVolumeM3', 'gegQpFactor', 'deltaUWbWM2k', 'automationClass', 'zones', 'elements', 'heating', 'dhw', 'ventilation', 'cooling', 'lighting', 'renewables', 'climate')}
    coll = l.e['coll']
    rows_of = lambda doc: doc[coll]
    empty = lambda: {'removed': [], 'inserted': [], 'moved': [], 'modified': []}
    delta = empty()
    if l.verb == 'change':
        f = l.field
        idk = f'{l.entity}Id'
        nk = 'new' + f[2][0].upper() + f[2][1:]
        target = TARGET[l.entity]
        if l.entity == 'element' and f[0] == 'zone-id':
            before['zones'].append({**before['zones'][0], 'id': 'zone-annex', 'labelEn': 'Annex', 'labelDe': 'Anbau'})
            after['zones'].append({**before['zones'][0], 'id': 'zone-annex', 'labelEn': 'Annex', 'labelDe': 'Anbau'})
        value = NEW_VALUES[(l.entity, f[2])]
        row = next(r for r in rows_of(after) if r['id'] == target)
        row[f[2]] = value
        mutation.update({idk: target, nk: value})
        patch = {k: None for k in ('labelEn', 'labelDe', 'usageProfile', 'areaM2', 'volumeM3', 'thetaIHeatC', 'thetaICoolC', 'occupants', 'internalGainsWM2', 'lightingPowerWM2')} if l.entity == 'zone' else {k: None for k in ('labelEn', 'labelDe', 'kind', 'zoneId', 'areaM2', 'uValueWM2k', 'orientationDeg', 'tiltDeg', 'gValue', 'fc', 'adjacency')}
        patch[f[2]] = value
        delta['modified'].append({'id': target, 'patch': patch})
    elif l.verb == 'insert':
        new = dict(rows_of(before)[0])
        new.update({'id': f'{l.entity}-extra', 'labelEn': 'Extra', 'labelDe': 'Zusatz'})
        index = 1
        rows_of(after).insert(index, new)
        mutation.update({'index': index, l.entity: new})
        delta['inserted'].append({'index': index, 'row': new})
    else:
        index = 0
        gone = rows_of(after).pop(index)
        mutation.update({'id': gone['id']})
        delta['removed'].append({'id': gone['id'], 'index': index})
    diff[coll] = delta
    return before, mutation, after, {'status': 'applied'}, diff


def fixture_files():
    for l in LEAVES:
        before, mutation, after, outcome, diff = vector(l)
        base = f'{FIX}/{l.dir}/✅apply'
        for role, value in (('before', before), ('mutation', mutation), ('after', after), ('outcome', outcome), ('diff', diff)):
            wr(f'{base}/🔣️.{role}.json', dump(value))


def must(text, old, new, where):
    assert old in text, f'{where}: {old[:70]}'
    return text.replace(old, new, 1)


def register(tags):
    kinds_new = [l.kind for l in LEAVES]
    gone = ('replace-zones', 'replace-elements')
    # mutations root
    p = f'{MUT}/🦀️.rs'
    t = rd(p)
    t = re.sub(r'use semio_s_artifact_norm_din18599_mut_zoning::replace_zones;\nuse semio_s_artifact_norm_din18599_mut_zoning::replace_elements;\n', '', t)
    imports = ''.join(f'use semio_s_artifact_norm_din18599_mut_{snake(l.crate)}::{l.mod};\n' for l in LEAVES)
    t = must(t, 'use semio_s_artifact_norm_din18599_mut_zoning::change_element_u;\n', 'use semio_s_artifact_norm_din18599_mut_zoning::change_element_u;\n' + imports, p)
    t = must(t, '    ReplaceZones(replace_zones::ReplaceZones),\n    ReplaceElements(replace_elements::ReplaceElements),\n', ''.join(f'    {l.struct}({l.mod}::{l.struct}),\n' for l in LEAVES), p)
    t = must(t, '    "replace-zones",\n    "replace-elements",\n', ''.join(f'    "{k}",\n' for k in kinds_new), p)
    wr(p, t)
    # mutations Cargo
    p = f'{MUT}/📦️packages/🦀️rust/Cargo.toml'
    t = rd(p)
    deps = ''.join(f'semio-s-artifact-norm-din18599-mut-{c} = {{ workspace = true }}\n' for c in CRATES)
    t = must(t, 'semio-s-artifact-norm-din18599-mut-zoning = { workspace = true }\n', 'semio-s-artifact-norm-din18599-mut-zoning = { workspace = true }\n' + deps, p)
    feats = ''.join(f', "semio-s-artifact-norm-din18599-mut-{c}/test-support"' for c in CRATES)
    t = must(t, '"semio-s-artifact-norm-din18599-mut-zoning/test-support"]', '"semio-s-artifact-norm-din18599-mut-zoning/test-support"' + feats + ']', p)
    wr(p, t)
    # artifact Cargo.toml
    p = f'{ART}/Cargo.toml'
    t = rd(p)
    members = ''.join(f'  "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{CRATE_DIR[c]}/📦️packages/🦀️rust",\n' for c in CRATES)
    t = must(t, '  "🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust",\n', '  "🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust",\n' + members, p)
    t = t.rstrip('\n') + '\n' + ''.join(f'\n[workspace.dependencies.semio-s-artifact-norm-din18599-mut-{c}]\npath = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{CRATE_DIR[c]}/📦️packages/🦀️rust"\n' for c in CRATES)
    wr(p, t)
    p = f'{ART}/📦️packages/🦀️rust/📜️script.ts'
    t = rd(p)
    t = must(t, '"semio-s-artifact-norm-din18599-mut-zoning",', '"semio-s-artifact-norm-din18599-mut-zoning",' + ''.join(f'"semio-s-artifact-norm-din18599-mut-{c}",' for c in CRATES), p)
    wr(p, t)
    # zoning mounts
    p = f'{MUT}/🗺️zoning/🦀️.rs'
    t = rd(p)
    t = must(t, '#[path = "../🧩replace-elements/🦀️.rs"]\npub mod replace_elements;\n#[path = "../🗺️replace-zones/🦀️.rs"]\npub mod replace_zones;\n', '', p)
    wr(p, t)
    for d in ('🧩replace-elements', '🗺️replace-zones'):
        shutil.rmtree(f'{MUT}/{d}', ignore_errors=True)
        shutil.rmtree(f'{FIX}/{d}', ignore_errors=True)
    # family schema json
    p = f'{MUT}/🔣️.json'
    t = rd(p)
    for g in gone:
        t = must(t, f'    {{\n      "$ref": "{SCHEMA_BASE}/mutation/{g}/schema.json"\n    }},\n', '', p)
    new_refs = ''.join(f'    {{\n      "$ref": "{SCHEMA_BASE}/mutation/{l.kind}/schema.json"\n    }},\n' for l in LEAVES)
    t = must(t, f'    {{\n      "$ref": "{SCHEMA_BASE}/mutation/change-element-u/schema.json"\n    }},\n', new_refs + f'    {{\n      "$ref": "{SCHEMA_BASE}/mutation/change-element-u/schema.json"\n    }},\n', p)
    wr(p, t)
    # family ts
    p = f'{MUT}/🟦️.ts'
    t = rd(p)
    t = must(t, 'import { parseReplaceZones, type ReplaceZones } from "./🗺️replace-zones/🧬️schema/🟦️.ts";\nimport { parseReplaceElements, type ReplaceElements } from "./🧩replace-elements/🧬️schema/🟦️.ts";\n', ''.join(f'import {{ type {l.struct}, parse{l.struct} }} from "./{l.dir}/🧬️schema/🟦️.ts";\n' for l in LEAVES), p)
    t = must(t, '  | ReplaceZones\n  | ReplaceElements\n', ''.join(f'  | {l.struct}\n' for l in LEAVES), p)
    t = must(t, '  replaceZones: parseReplaceZones,\n  replaceElements: parseReplaceElements,\n', ''.join(f'  {l.camel}: parse{l.struct},\n' for l in LEAVES), p)
    wr(p, t)
    # binary protocol
    p = f'{SUB}/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio'
    t = rd(p)
    t = must(t, 'record replace-zones tag=17\nfield payload bytes\nrecord replace-elements tag=18\nfield payload bytes\n', '', p)
    t = t.rstrip('\n') + '\n' + ''.join(f'record {l.kind} tag={tags[l.kind]}\nfield payload bytes\n' for l in LEAVES)
    wr(p, t)
    # binary rs imports
    p = f'{SUB}/🚪️io/💾️binary/🧬️mutations/🦀️.rs'
    t = rd(p)
    t = must(t, '{change_element_u, replace_elements, replace_zones, update_climate}', '{change_element_u, update_climate}', p)
    wr(p, t)
    # text dsl
    p = f'{SUB}/🚪️io/📝️text/🧬️mutations/🦀️.rs'
    t = rd(p)
    t = must(t, '{change_element_u, replace_elements, replace_zones, update_climate}', '{change_element_u, update_climate}', p)
    t = must(t, '''    ReplaceZones {
        #[dsl(block)]
        new_zones: Vec<semio_s_artifact_norm_din18599_model::ThermalZone>,
    },
    ReplaceElements {
        #[dsl(block)]
        new_elements: Vec<semio_s_artifact_norm_din18599_model::EnvelopeElement>,
    },
''', ''.join(dsl_variant(l) for l in LEAVES), p)
    t = must(t, '        Din18599Mutation::ReplaceZones(payload) => Din18599MutationDsl::ReplaceZones { new_zones: payload.new_zones.clone() },\n        Din18599Mutation::ReplaceElements(payload) => Din18599MutationDsl::ReplaceElements { new_elements: payload.new_elements.clone() },\n', ''.join(to_dsl(l) for l in LEAVES), p)
    t = must(t, '        Din18599MutationDsl::ReplaceZones { new_zones } => Din18599Mutation::ReplaceZones(replace_zones::ReplaceZones { new_zones }),\n        Din18599MutationDsl::ReplaceElements { new_elements } => Din18599Mutation::ReplaceElements(replace_elements::ReplaceElements { new_elements }),\n', ''.join(from_dsl(l) for l in LEAVES), p)
    imports = ''.join(f'use semio_s_artifact_norm_din18599_mut_{snake(c)}::{{{", ".join(l.mod for l in LEAVES if l.crate == c)}}};\n' for c in CRATES)
    t = must(t, 'use semio_s_artifact_norm_din18599_mut_systems::', imports + 'use semio_s_artifact_norm_din18599_mut_systems::', p)
    wr(p, t)
    # oracle manifest
    p = f'{SUB}/🔮️oracles/🔣️.json'
    m = json.loads(rd(p))
    ctx = m['oracles'] if False else None
    walk_manifest(m, tags)
    wr(p, dump(m))
    # taxonomy
    t = json.loads(rd(TAXONOMY))
    rows = t['rows']
    last = max(i for i, r in enumerate(rows) if r['artifact'].endswith('din18599'))
    rows[:] = [r for r in rows if not (r['artifact'].endswith('din18599') and r['kind'] in gone)]
    last = max(i for i, r in enumerate(rows) if r['artifact'].endswith('din18599'))
    rows[last + 1:last + 1] = [taxonomy_row(l) for l in LEAVES]
    wr(TAXONOMY, dump(t))


def dsl_variant(l):
    if l.verb == 'change':
        f = l.field
        t = {'String': 'String', 'f64': 'f64', 'u32': 'u32'}.get(f[3], rust_type(f[3]))
        return f'    {l.struct} {{\n        {l.entity}_id: String,\n        new_{f[1]}: {t},\n    }},\n'
    if l.verb == 'insert':
        return f'    {l.struct} {{\n        index: usize,\n        #[dsl(block)]\n        {l.entity}: semio_s_artifact_norm_din18599_model::{l.e["row"]},\n    }},\n'
    return f'    {l.struct} {{\n        id: String,\n    }},\n'


def fields_of(l):
    if l.verb == 'change':
        return [f'{l.entity}_id', f'new_{l.field[1]}']
    if l.verb == 'insert':
        return ['index', l.entity]
    return ['id']


def is_copy(l):
    return l.verb == 'change' and l.field[3] != 'String'


def to_dsl(l):
    parts = []
    for name in fields_of(l):
        copy = l.verb == 'change' and name.startswith('new_') and is_copy(l)
        parts.append(f'{name}: payload.{name}' + ('' if copy or name == 'index' else '.clone()'))
    return f'        Din18599Mutation::{l.struct}(payload) => Din18599MutationDsl::{l.struct} {{ {", ".join(parts)} }},\n'


def from_dsl(l):
    names = ', '.join(fields_of(l))
    return f'        Din18599MutationDsl::{l.struct} {{ {names} }} => Din18599Mutation::{l.struct}({l.mod}::{l.struct} {{ {names} }}),\n'


def taxonomy_row(l):
    e = l.e
    if l.verb == 'change':
        edit = {'rule': 'set-field', 'path': f'{e["coll"]}[].{l.field[2]}', 'selectors': [{'kind': 'id', 'key': f'{l.entity}Id'}], 'value': 'new' + l.field[2][0].upper() + l.field[2][1:]}
        entity, verb, record = f'{l.entity}-{l.field[0]}', 'change', 'Changed' + pascal(f'{l.entity}-{l.field[0]}')
    elif l.verb == 'insert':
        edit = {'rule': 'insert-item', 'path': e['coll'], 'selectors': [], 'index': 'index', 'item': l.entity}
        entity, verb, record = l.entity, 'insert', 'Inserted' + pascal(l.entity)
    else:
        edit = {'rule': 'id-removal', 'path': e['coll'], 'selectors': [], 'id': 'id'}
        entity, verb, record = l.entity, 'remove', 'Removed' + pascal(l.entity)
    return {'aggregateVariant': l.struct, 'artifact': '⚡️din18599', 'edit': edit, 'entity': entity, 'kind': l.kind, 'module': l.dir, 'physicalLayout': 'direct', 'record': record,
            'source': f'⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{l.dir}/🦀️.rs', 'standard': '🔖️1', 'subset': '✳️any', 'type': l.struct, 'verb': verb}


def walk_manifest(m, tags):
    gone = ('replace-zones', 'replace-elements')
    new_kinds = [l.kind for l in LEAVES]

    def fix(node):
        if isinstance(node, dict):
            for key, value in list(node.items()):
                if isinstance(value, list):
                    node[key] = fix_list(key, value)
                else:
                    fix(value)
        elif isinstance(node, list):
            for item in node:
                fix(item)

    def fix_list(key, items):
        out = []
        for item in items:
            ident = item.get('mutationId') if isinstance(item, dict) else None
            ident = ident or (item.get('id') if isinstance(item, dict) else None)
            if isinstance(item, str) and item in gone:
                if item == 'replace-zones':
                    out.extend(new_kinds)
                continue
            if isinstance(item, dict) and ident in gone:
                if ident == 'replace-zones':
                    out.extend(entry_for(item, l) for l in LEAVES)
                continue
            fix(item)
            out.append(item)
        return out

    def entry_for(template, l):
        e = json.loads(json.dumps(template))
        if 'mutationId' in e:
            e['mutationId'] = l.kind
            e['sourceMutationDirectoryName'] = l.dir
            e['mutationDirectoryName'] = l.dir
        else:
            e['id'] = l.kind
            e['productionDispatch'] = {'operation': l.kind, 'bridgeVersion': 1, 'variant': l.struct}
        return e

    fix(m)


def main():
    crate_files()
    tags = leaf_files()
    fixture_files()
    register(tags)
    print('generated', len(LEAVES), 'leaves')


main()
