"""Adds nine schema-first mutation kinds to DIN EN 16798 so that every inspector field is editable (one-shot, not idempotent): change-zone-name, change-zone-id, change-vent-name, change-vent-id, change-vent-fan-flow, change-vent-fan-run-hours, change-vent-duct-test-pressure, change-vent-humidification-required, change-vent-humidification-provided."""
import copy, json, os, re

ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798'
S = f'{ART}/🏅️standards/🔖️1/🪆️subsets/✳️any'
M = f'{S}/🧬️schema/🧬️mutations'
FIX = f'{S}/🧪️tests/🧫️fixtures/🧬️mutations'
IO = f'{S}/🚪️io'
TAX = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json'
OWNER = '✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations'
FIRST_TAG = 41


def read(p):
    return open(p, encoding='utf8').read()


def write(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf8').write(t)


def patch(p, anchor, insertion, after=True):
    t = read(p)
    assert t.count(anchor) == 1, (p, anchor, t.count(anchor))
    i = t.index(anchor)
    j = i + len(anchor) if after else i
    write(p, t[:j] + insertion + t[j:])


def pascal(kind):
    return ''.join(w.capitalize() for w in kind.split('-'))


def snake(kind):
    return kind.replace('-', '_')


ZONE = dict(coll='zones', row='zone', id='zone_id', idj='zoneId', Delta='Din16798ZoneDelta', Patch='Din16798ZonePatch', diffname='zones', crate='zones', model='ZoneDocument', ref='zone', labels=('Zone', 'Zone'), refdesc=('The ventilation zone this mutation addresses.', 'Die Lüftungszone, die diese Mutation adressiert.'), found='zone not found', param='z')
VENT = dict(coll='vent_systems', row='vent', id='vent_id', idj='ventId', Delta='Din16798VentSystemDelta', Patch='Din16798VentSystemPatch', diffname='vent_systems', crate='ventilation', model='VentSystemDocument', ref='ventSystem', labels=('Ventilation system', 'Lüftungsanlage'), refdesc=('The ventilation system this mutation addresses.', 'Die Lüftungsanlage, die diese Mutation adressiert.'), found='vent not found', param='v')

SPECS = [
    dict(kind='change-zone-name', emoji='🏷️', t=ZONE, leaf='name', type='String', title=('Change name of the zone', 'Namen der Zone ändern'), entity='zone-name', ui=('Zone name', 'Zonenname'), group='identity', widget='text', desc=('The display name of the zone.', 'Der Anzeigename der Zone.'), before='Open office', after='Open-plan office'),
    dict(kind='change-zone-id', emoji='🆔️', t=ZONE, leaf='id', type='String', title=('Change identifier of the zone', 'Kennung der Zone ändern'), entity='zone-id', ui=('Zone identifier', 'Zonenkennung'), group='identity', widget='text', desc=('The new unique identifier of the zone.', 'Die neue eindeutige Kennung der Zone.'), before='zone-office', after='zone-office-1', ident=True, field='newId'),
    dict(kind='change-vent-name', emoji='📛', t=VENT, leaf='name', type='String', title=('Change name of the ventilation system', 'Namen der Lüftungsanlage ändern'), entity='vent-name', ui=('System name', 'Anlagenname'), group='identity', widget='text', desc=('The display name of the ventilation system.', 'Der Anzeigename der Lüftungsanlage.'), before='Central AHU', after='Central air handling unit'),
    dict(kind='change-vent-id', emoji='🪪', t=VENT, leaf='id', type='String', title=('Change identifier of the ventilation system', 'Kennung der Lüftungsanlage ändern'), entity='vent-id', ui=('System identifier', 'Anlagenkennung'), group='identity', widget='text', desc=('The new unique identifier; the zones served by the system follow.', 'Die neue eindeutige Kennung; die versorgten Zonen folgen.'), before='vent-central', after='vent-main', ident=True, field='newId'),
    dict(kind='change-vent-fan-flow', emoji='🌪️', t=VENT, leaf='fanQVM3S', type='f64', title=('Change fan volume flow of the ventilation system', 'Ventilator-Volumenstrom der Lüftungsanlage ändern'), entity='vent-fan-flow', ui=('Fan volume flow q_V', 'Ventilator-Volumenstrom q_V'), group='ventilation', widget='stepper', unit='m³/s', step=0.01, precision=3, minimum=0, before=0.28, after=0.35),
    dict(kind='change-vent-fan-run-hours', emoji='⏱️', t=VENT, leaf='fanTRunH', type='f64', title=('Change annual fan run hours of the ventilation system', 'Jährliche Ventilator-Laufzeit der Lüftungsanlage ändern'), entity='vent-fan-run-hours', ui=('Annual fan run hours t_run', 'Jährliche Ventilator-Laufzeit t_run'), group='ventilation', widget='stepper', unit='h', step=50, precision=0, minimum=0, maximum=8760, before=2500.0, after=3000.0),
    dict(kind='change-vent-duct-test-pressure', emoji='🧪', t=VENT, leaf='ductTestPressurePa', type='f64', title=('Change duct test pressure of the ventilation system', 'Kanal-Prüfdruck der Lüftungsanlage ändern'), entity='vent-duct-test-pressure', ui=('Duct test pressure p', 'Kanal-Prüfdruck p'), group='ducts', widget='stepper', unit='Pa', step=10, precision=0, minimum=0, before=400.0, after=500.0),
    dict(kind='change-vent-humidification-required', emoji='💦', t=VENT, leaf='humidificationRequiredKgH', type='f64', title=('Change required humidification of the ventilation system', 'Erforderliche Befeuchtung der Lüftungsanlage ändern'), entity='vent-humidification-required', ui=('Required humidification', 'Erforderliche Befeuchtungsleistung'), group='humidification', widget='stepper', unit='kg/h', step=0.1, precision=1, minimum=0, before=0.0, after=2.5),
    dict(kind='change-vent-humidification-provided', emoji='🚿', t=VENT, leaf='humidificationProvidedKgH', type='f64', title=('Change provided humidification of the ventilation system', 'Bereitgestellte Befeuchtung der Lüftungsanlage ändern'), entity='vent-humidification-provided', ui=('Provided humidification', 'Bereitgestellte Befeuchtungsleistung'), group='humidification', widget='stepper', unit='kg/h', step=0.1, precision=1, minimum=0, before=0.0, after=2.0),
]


def snake_of(camel):
    return re.sub(r'([A-Z])', lambda m: '_' + m.group(1).lower(), camel)


def kebab_of(camel):
    return snake_of(camel).replace('_', '-')


def derive(s):
    t = s['t']
    s['Pascal'] = pascal(s['kind'])
    s['snake'] = snake(s['kind'])
    s['dir'] = s['emoji'] + s['kind']
    s['value_field_json'] = s.get('field') or 'new' + s['leaf'][0].upper() + s['leaf'][1:]
    s['value_field'] = snake_of(s['value_field_json'])
    s['value_kebab'] = kebab_of(s['value_field_json'])
    s['patch_field'] = snake_of(s['leaf'])
    s['tag'] = FIRST_TAG + SPECS.index(s)
    s['TAG'] = 'TAG_' + s['snake'].upper()
    s['edit_path'] = f"{t['coll'].replace('vent_systems','ventSystems')}[].{s['leaf']}"
    return s


for s in SPECS:
    derive(s)


def leaf_rs(s):
    t = s['t']
    return f'''//! 🔧 `{s['kind']}`.
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct {s['Pascal']} {{
    pub {t['id']}: String,
    pub {s['value_field']}: {s['type']},
}}
impl<Op> protocol::MutationKind<Din16798Snapshot, Op> for {s['Pascal']} where Op: protocol::Mutation<Din16798Snapshot, Diff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::{s['snake']}::{s['Pascal']}> {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "change", entity: "{s['entity']}", kind: "{s['kind']}", record: "{s['Pascal']}" }};

    fn label(&self) -> semio_ui_locale::LocalizedLabel {{
        semio_ui_locale::LocalizedLabel::native("{s['title'][0]}", "{s['title'][1]}")
    }}
    fn target(&self) -> Vec<String> {{ vec![self.{t['id']}.clone()] }}
}}

impl<Op> protocol::ColdMutationKind<Din16798Snapshot, Op> for {s['Pascal']} where Op: protocol::ColdMutation<Din16798Snapshot, ReferenceDiff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::{s['snake']}::{s['Pascal']}> {{
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din16798Snapshot>>::ReferenceDiff> {{ crate::{s['snake']}::diff::diff(self, base) }}
    fn inverse(&self, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {{
    Ok({{ crate::{s['snake']}::inverse::inverse(self, base)? 
    }})
}}
}}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
'''


def diff_rs(s):
    t = s['t']
    head = f'''use crate::{s['snake']}::{s['Pascal']};
use semio_s_artifact_norm_din16798_diff::{{Din16798Diff, {t['Delta']}, {t['Patch']}}};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
'''
    if not s.get('ident'):
        value = f"payload.{s['value_field']}.clone()" if s['type'] == 'String' else f"payload.{s['value_field']}"
        return f'''//! {s['emoji']} `{s['kind']}` diff — patches the one field of the row with that id; an id the document does not hold is a `mutation.invariant`.

{head}
pub fn diff(payload: &{s['Pascal']}, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {{
    let Some(row) = base.{t['coll']}.iter().find(|row| row.id == payload.{t['id']}) else {{
        return protocol::MutationOutcome::fatal("mutation.invariant", "{t['found']}", Vec::<String>::new());
    }};
    let patch = {t['Patch']} {{ {s['patch_field']}: Some({value}), ..Default::default() }};
    protocol::MutationOutcome::new(Din16798Diff {{ {t['diffname']}: {t['Delta']}::modification(&row.id, patch), ..Default::default() }})
}}
'''
    if t['row'] == 'zone':
        return f'''//! 🆔️ `{s['kind']}` diff — replaces the row by a copy under the new id at the same position; an id the document does not hold is a `mutation.invariant`, an id it already holds a `mutation.duplicate-id`. No other row names a zone, so nothing else follows.

{head}
pub fn diff(payload: &{s['Pascal']}, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {{
    let Some(index) = base.zones.iter().position(|row| row.id == payload.zone_id) else {{
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    }};
    if payload.new_id == payload.zone_id {{
        return protocol::MutationOutcome::new(Din16798Diff::default());
    }}
    if base.zones.iter().any(|row| row.id == payload.new_id) {{
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A zone with id '{{}}' already exists.", payload.new_id), [payload.new_id.clone()]);
    }}
    let mut renamed = base.zones[index].clone();
    renamed.id = payload.new_id.clone();
    let mut zones = Din16798ZoneDelta::removal(&base.zones, index);
    zones.absorb(Din16798ZoneDelta::insertion(index, renamed));
    protocol::MutationOutcome::new(Din16798Diff {{ zones, ..Default::default() }})
}}
'''
    return f'''//! 🆔️ `{s['kind']}` diff — replaces the row by a copy under the new id at the same position and points every zone served by the system at the new id; an id the document does not hold is a `mutation.invariant`, an id it already holds or that a zone already names a `mutation.duplicate-id`.

use crate::{s['snake']}::{s['Pascal']};
use semio_s_artifact_norm_din16798_diff::{{Din16798Diff, Din16798VentSystemDelta, Din16798ZoneDelta, Din16798ZonePatch}};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

pub fn diff(payload: &{s['Pascal']}, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {{
    let Some(index) = base.vent_systems.iter().position(|row| row.id == payload.vent_id) else {{
        return protocol::MutationOutcome::fatal("mutation.invariant", "vent not found", Vec::<String>::new());
    }};
    if payload.new_id == payload.vent_id {{
        return protocol::MutationOutcome::new(Din16798Diff::default());
    }}
    if base.vent_systems.iter().any(|row| row.id == payload.new_id) || base.zones.iter().any(|zone| zone.vent_system_id == payload.new_id) {{
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A ventilation system with id '{{}}' already exists or is already named by a zone.", payload.new_id), [payload.new_id.clone()]);
    }}
    let mut renamed = base.vent_systems[index].clone();
    renamed.id = payload.new_id.clone();
    let mut vent_systems = Din16798VentSystemDelta::removal(&base.vent_systems, index);
    vent_systems.absorb(Din16798VentSystemDelta::insertion(index, renamed));
    let mut zones = Din16798ZoneDelta::default();
    for zone in base.zones.iter().filter(|zone| zone.vent_system_id == payload.vent_id) {{
        zones.absorb(Din16798ZoneDelta::modification(&zone.id, Din16798ZonePatch {{ vent_system_id: Some(payload.new_id.clone()), ..Default::default() }}));
    }}
    protocol::MutationOutcome::new(Din16798Diff {{ vent_systems, zones, ..Default::default() }})
}}
'''


def inverse_rs(s):
    t = s['t']
    head = f'''//! ↩️ `{s['kind']}` inverse.
use crate::{s['snake']}::{s['Pascal']};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &{s['Pascal']}, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{s['snake']}::{s['Pascal']}> {{
    Ok((|| {{
'''
    if s.get('ident'):
        return head + f'''    if !base.{t['coll']}.iter().any(|row| row.id == payload.{t['id']}) {{ return Vec::new(); }}
    vec![Op::from({s['Pascal']} {{ {t['id']}: payload.new_id.clone(), new_id: payload.{t['id']}.clone() }})]

    }})())
}}
'''
    old = f"row.{s['patch_field']}.clone()" if s['type'] == 'String' else f"row.{s['patch_field']}"
    return head + f'''    let Some(row) = base.{t['coll']}.iter().find(|row| row.id == payload.{t['id']}) else {{ return Vec::new(); }};
    vec![Op::from({s['Pascal']} {{ {t['id']}: payload.{t['id']}.clone(), {s['value_field']}: {old} }})]

    }})())
}}
'''


def mutation_json(s):
    t = s['t']
    return f'''{{
  "schemaVersion": 1,
  "owner": "{OWNER}/{s['dir']}",
  "semanticKind": "{s['kind']}",
  "displayName": "{' '.join(w.capitalize() for w in s['kind'].split('-'))}",
  "emoji": "{s['emoji']}",
  "aggregateVariant": "{s['Pascal']}",
  "payloadSchema": "🧬️schema/🔣️.json",
  "textOpcode": null,
  "binaryTag": {s['tag']},
  "invertibility": "explicit-mutation",
  "diffParticipation": "detect",
  "outcomeClasses": [
    "applied",
    "rejected"
  ],
  "composition": "atomic",
  "requiredLanguageSurfaces": [
    "rust",
    "json-schema",
    "text",
    "binary"
  ]
}}
'''


def payload_schema(s):
    t = s['t']
    value = {'type': 'string' if s['type'] == 'String' else 'number'}
    if s.get('minimum') is not None:
        value['minimum'] = s['minimum']
    if s.get('maximum') is not None:
        value['maximum'] = s['maximum']
    ui = {'widget': s['widget'], 'role': 'value', 'label': {'en': s['ui'][0], 'de': s['ui'][1]}}
    if s.get('desc'):
        ui['description'] = {'en': s['desc'][0], 'de': s['desc'][1]}
    if s.get('unit'):
        ui.update({'unit': s['unit'], 'step': s['step'], 'precision': s['precision']})
    ui.update({'group': s['group'], 'order': 20})
    value['x-semio-ui'] = ui
    return {
        '$schema': 'http://json-schema.org/draft-07/schema#',
        '$id': f"https://json.schemas.assets.semio-tech.com/s/norm/din16798/mutation/{s['kind']}/schema.json",
        'title': s['Pascal'], 'type': 'object', 'additionalProperties': False,
        'required': [t['idj'], s['value_field_json']],
        'properties': {
            t['idj']: {'type': 'string', 'x-semio-ui': {'widget': 'reference', 'role': 'target', 'label': {'en': t['labels'][0], 'de': t['labels'][1]}, 'description': {'en': t['refdesc'][0], 'de': t['refdesc'][1]}, 'ref': {'kind': t['ref']}, 'group': 'target', 'order': 10}},
            s['value_field_json']: value,
        },
    }


def twin_ts(s):
    t = s['t']
    imports = ['normWireObject', 'type NormWireReader', 'normWireRequired', 'normWireString']
    if s['type'] == 'f64':
        imports.append('normWireNumber')
        if s.get('minimum') is not None:
            imports.append('normWireRange')
    imports.sort(key=lambda x: x.replace('type ', '').lower())
    bounds = {k: s[k] for k in ('minimum', 'maximum') if s.get(k) is not None}
    if s['type'] == 'String':
        reader, ts = 'normWireString', 'string'
    elif bounds:
        reader, ts = f'normWireRange(normWireNumber, {json.dumps(bounds, separators=(",", ":"))})', 'number'
    else:
        reader, ts = 'normWireNumber', 'number'
    return f'''/** {s['emoji']} `{s['kind']}` wire twin: the leaf payload `{s['Pascal']}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ {', '.join(imports)} }} from "@semio-tech/norm-js/registry/contract";

export interface {s['Pascal']} {{
  {t['idj']}: string;
  {s['value_field_json']}: {ts};
}}

export const parse{s['Pascal']}: NormWireReader<{s['Pascal']}> = normWireObject<{s['Pascal']}>({{ {t['idj']}: normWireRequired(normWireString), {s['value_field_json']}: normWireRequired({reader}) }});
'''


# ---------------------------------------------------------------- fixtures
def load(p):
    return json.load(open(p, encoding='utf8'))


def dump(p, d):
    write(p, json.dumps(d, indent=2, ensure_ascii=False) + '\n')


BASE = load(f'{FIX}/🔊️change-zone-noise/✅apply/🔣️.before.json')
ZONE_KEYS = list(BASE['zones'][0].keys())
VENT_KEYS = list(BASE['ventSystems'][0].keys())


def empty_delta():
    return {'removed': [], 'inserted': [], 'moved': [], 'modified': []}


def fixtures(s):
    t = s['t']
    before = copy.deepcopy(BASE)
    if s['kind'] == 'change-vent-id':
        before['zones'] = []
    after = copy.deepcopy(before)
    row_key = 'zones' if t['row'] == 'zone' else 'ventSystems'
    keys = ZONE_KEYS if t['row'] == 'zone' else VENT_KEYS
    row = after[row_key][0]
    old_id = row['id']
    row[s['leaf']] = s['after']
    mutation = {s['Pascal']: {t['idj']: old_id, s['value_field_json']: s['after']}}
    diff = {k: None for k in ('annex', 'thetaRmC', 'outdoorCo2Ppm', 'envelopeN50HInv', 'envelopeVolumeM3', 'cellarAreaM2', 'cellarVentilationM3H', 'nightSetbackK')}
    diff['zones'], diff['ventSystems'] = empty_delta(), empty_delta()
    if s.get('ident'):
        original = copy.deepcopy(before[row_key][0])
        diff[row_key]['removed'] = [{'id': old_id, 'index': 0}]
        diff[row_key]['inserted'] = [{'index': 0, 'row': copy.deepcopy(row)}]
        assert original['id'] == old_id
    else:
        patch = {k: None for k in keys if k != 'id'}
        patch[s['leaf']] = s['after']
        diff[row_key]['modified'] = [{'id': old_id, 'patch': patch}]
    base = f"{FIX}/{s['dir']}/✅apply"
    dump(f'{base}/🔣️.before.json', before)
    dump(f'{base}/🔣️.after.json', after)
    dump(f'{base}/🔣️.mutation.json', mutation)
    dump(f'{base}/🔣️.outcome.json', {'status': 'applied'})
    dump(f'{base}/🔣️.diff.json', diff)


# ---------------------------------------------------------------- generation
for s in SPECS:
    t = s['t']
    d = f"{M}/{s['dir']}"
    write(f'{d}/🦀️.rs', leaf_rs(s))
    write(f'{d}/🔺️diff/🦀️.rs', diff_rs(s))
    write(f'{d}/↩️inverse/🦀️.rs', inverse_rs(s))
    write(f'{d}/🔣️.json', mutation_json(s))
    write(f'{d}/🧬️schema/🔣️.json', json.dumps(payload_schema(s), indent=2, ensure_ascii=False) + '\n')
    write(f'{d}/🧬️schema/🟦️.ts', twin_ts(s))
    fixtures(s)

zone_specs = [s for s in SPECS if s['t'] is ZONE]
vent_specs = [s for s in SPECS if s['t'] is VENT]

# group crates
zm = f'{M}/🗺️zones-more/🦀️.rs'
patch(zm, '//#endregion 🪢️Mounts', ''.join(f'#[path = "../{s["dir"]}/🦀️.rs"]\npub mod {s["snake"]};\n' for s in zone_specs), after=False)
zones = f'{M}/🗺️zones/🦀️.rs'
t = read(zones)
t = t.replace('pub use semio_s_artifact_norm_din16798_mut_zones_more::{change_zone_usage_type, change_zone_vent_method, change_zone_vent_system_id, insert_zone, remove_zone};', 'pub use semio_s_artifact_norm_din16798_mut_zones_more::{change_zone_id, change_zone_name, change_zone_usage_type, change_zone_vent_method, change_zone_vent_system_id, insert_zone, remove_zone};')
write(zones, t)

vm = f'{M}/🌀️ventilation-more'
more_cargo = read(f'{M}/🗺️zones-more/📦️packages/🦀️rust/Cargo.toml').replace('mut-zones-more', 'mut-ventilation-more').replace('zones mutation leaves (zones more)', 'ventilation mutation leaves (ventilation more)')
write(f'{vm}/📦️packages/🦀️rust/Cargo.toml', more_cargo)
write(f'{vm}/🦀️.rs', '''//! more ventilation mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
''' + ''.join(f'#[path = "../{s["dir"]}/🦀️.rs"]\npub mod {s["snake"]};\n' for s in vent_specs) + '//#endregion 🪢️Mounts\n')
vent_group = f'{M}/🌀️ventilation/🦀️.rs'
write(vent_group, read(vent_group).rstrip('\n') + f"\n\npub use semio_s_artifact_norm_din16798_mut_ventilation_more::{{{', '.join(sorted(s['snake'] for s in vent_specs))}}};\n")
vc = f'{M}/🌀️ventilation/📦️packages/🦀️rust/Cargo.toml'
patch(vc, 'semio-s-artifact-norm-din16798-model = { workspace = true }\n', 'semio-s-artifact-norm-din16798-mut-ventilation-more = { workspace = true }\n')
t = read(vc).replace('"semio-s-artifact-norm-din16798-model/test-support"]', '"semio-s-artifact-norm-din16798-model/test-support", "semio-s-artifact-norm-din16798-mut-ventilation-more/test-support"]', 1)
write(vc, t)
root_cargo = f'{ART}/Cargo.toml'
patch(root_cargo, '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermal-more/📦️packages/🦀️rust",\n', '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️ventilation-more/📦️packages/🦀️rust",\n', after=False)
patch(root_cargo, '[workspace.dependencies.semio-s-artifact-norm-din16798-mut-zones-more]', '[workspace.dependencies.semio-s-artifact-norm-din16798-mut-ventilation-more]\npath = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️ventilation-more/📦️packages/🦀️rust"\n\n', after=False)
script = f'{ART}/📦️packages/🦀️rust/📜️script.ts'
patch(script, '"semio-s-artifact-norm-din16798-mut-ventilation",', '"semio-s-artifact-norm-din16798-mut-ventilation-more",')

# aggregate
agg = f'{M}/🦀️.rs'
imports = ''.join(f"use semio_s_artifact_norm_din16798_mut_{s['t']['crate']}::{s['snake']};\n" for s in SPECS)
patch(agg, 'use semio_s_artifact_norm_din16798_mut_ventilation::change_vent_design_airflow;\n', imports)
patch(agg, '    ChangeVentDesignAirflow(change_vent_design_airflow::ChangeVentDesignAirflow),\n', ''.join(f"    {s['Pascal']}({s['snake']}::{s['Pascal']}),\n" for s in SPECS))
patch(agg, '    "change-vent-design-airflow",\n', ''.join(f'    "{s["kind"]}",\n' for s in SPECS))

aj = f'{M}/🔣️.json'
blocks = ''
for s in SPECS:
    blocks += f''',
    {{
      "type": "object",
      "additionalProperties": false,
      "required": [
        "{s['Pascal']}"
      ],
      "properties": {{
        "{s['Pascal']}": {{
          "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/din16798/mutation/{s['kind']}/schema.json"
        }}
      }}
    }}'''
patch(aj, '''          "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/din16798/mutation/change-vent-design-airflow/schema.json"
        }
      }
    }''', blocks)

gq = f'{M}/🔗️.graphql'
t = read(gq)
assert t.count(' change_vent_design_airflow }') == 1
write(gq, t.replace(' change_vent_design_airflow }', ' change_vent_design_airflow ' + ' '.join(s['snake'] for s in SPECS) + ' }'))

ts = f'{M}/🟦️.ts'
patch(ts, 'import { type ChangeZoneNoise, parseChangeZoneNoise } from "./🔊️change-zone-noise/🧬️schema/🟦️.ts";\n', ''.join(f'import {{ type {s["Pascal"]}, parse{s["Pascal"]} }} from "./{s["dir"]}/🧬️schema/🟦️.ts";\n' for s in SPECS))
patch(ts, '  | { ChangeVentDesignAirflow: ChangeVentDesignAirflow };\n', ''.join(f'  | {{ {s["Pascal"]}: {s["Pascal"]} }}\n' for s in SPECS)[:-1] + ';\n')
tt_ = read(ts).replace('  | { ChangeVentDesignAirflow: ChangeVentDesignAirflow };\n', '  | { ChangeVentDesignAirflow: ChangeVentDesignAirflow }\n', 1)
write(ts, tt_)
patch(ts, '  ChangeVentDesignAirflow: parseChangeVentDesignAirflow,\n', ''.join(f'  {s["Pascal"]}: parse{s["Pascal"]},\n' for s in SPECS))

# sample mutations
def sample(s):
    t = s['t']
    row = f"base.{t['coll']}[0]"
    if s.get('ident'):
        value = f'"{s["after"]}".into()'
    elif s['type'] == 'String':
        value = f'format!("{{}} B", {row}.{s["patch_field"]})'
    else:
        value = f'{row}.{s["patch_field"]} + 1.0'
    return f'        ("{s["kind"]}", Din16798Mutation::{s["Pascal"]}({s["snake"]}::{s["Pascal"]} {{ {t["id"]}: {row}.id.clone(), {s["value_field"]}: {value} }})),\n'

ut = f'{M}/🧪️tests/🔬️unit/🦀️.rs'
patch(ut, '        ("change-zone-vent-method", Din16798Mutation::ChangeZoneVentMethod(change_zone_vent_method::ChangeZoneVentMethod { zone_id: base.zones[0].id.clone(), new_vent_method: "method_3_predefined_rates".into() })),\n', ''.join(sample(s) for s in SPECS))
ut_text = read(ut)
m = re.search(r'assert_eq!\(cases\.len\(\), (\d+),', ut_text)
ut_text = ut_text.replace(m.group(0), f'assert_eq!(cases.len(), {int(m.group(1)) + len(SPECS)},')
ut_text += '''
#[semio_framework_async_macros::async_test]
async fn renaming_a_ventilation_system_points_its_zones_at_the_new_id_and_the_inverse_restores_them() {
    let base = Din16798Snapshot::default();
    let rename = Din16798Mutation::ChangeVentId(change_vent_id::ChangeVentId { vent_id: base.vent_systems[0].id.clone(), new_id: "vent-main".into() });
    let after = apply(&rename, &base);
    assert_eq!(after.vent_systems[0].id, "vent-main");
    assert!(after.zones.iter().all(|zone| zone.vent_system_id == "vent-main"), "every zone served by the system follows");
    assert_mutates_and_restores("change-vent-id", &base, rename);
}

#[semio_framework_async_macros::async_test]
async fn an_identifier_that_is_taken_is_refused() {
    let mut base = Din16798Snapshot::default();
    let mut second = base.zones[0].clone();
    second.id = "zone-second".into();
    base.zones.push(second);
    let refused = Din16798Mutation::ChangeZoneId(change_zone_id::ChangeZoneId { zone_id: "zone-office".into(), new_id: "zone-second".into() });
    assert!(<Din16798Mutation as protocol::ColdMutation<Din16798Snapshot>>::diff(&refused, &base).worst_level().is_some(), "a taken identifier is refused");
    let vent = Din16798Mutation::ChangeVentId(change_vent_id::ChangeVentId { vent_id: "vent-central".into(), new_id: "vent-central".into() });
    assert_eq!(apply(&vent, &base), base, "renaming to the same id changes nothing");
}
'''
write(ut, ut_text)

# edit rules
er = f'{M}/🧭️edit-rules/🦀️.rs'
rules = ''
for s in SPECS:
    t = s['t']
    rules += f'        SetFieldRule {{ path: "{s["edit_path"]}", kind: "{s["kind"]}", selectors: &[SelectorKey::Id("{"zoneId" if t["row"]=="zone" else "ventId"}")], value: "{s["value_field_json"]}" }},\n'
patch(er, '        SetFieldRule { path: "zones[].co2Ppm", kind: "change-zone-co2", selectors: &[SelectorKey::Id("zoneId")], value: "newCo2Ppm" },\n', rules)

# text io
tx = f'{IO}/📝️text/🧬️mutations/🦀️.rs'
patch(tx, 'use semio_s_artifact_norm_din16798_mut_ventilation::change_vent_design_airflow::ChangeVentDesignAirflow;\n', ''.join(f"use semio_s_artifact_norm_din16798_mut_{s['t']['crate']}::{s['snake']}::{s['Pascal']};\n" for s in SPECS))
fmt = ''
parse = ''
demo = ''
for s in SPECS:
    t = s['t']
    fmt += f'        Din16798Mutation::{s["Pascal"]}(p) => format!("{s["kind"]} {{}}", [format!("{kebab_of(t["idj"])}={{}}", enc_json(&p.{t["id"]})), format!("{s["value_kebab"]}={{}}", enc_json(&p.{s["value_field"]}))].join(" ")),\n'
    parse += f'        "{s["kind"]}" => Ok(Din16798Mutation::{s["Pascal"]}({s["Pascal"]} {{ {t["id"]}: dec_json(&arg("{kebab_of(t["idj"])}")?)?, {s["value_field"]}: dec_json(&arg("{s["value_kebab"]}")?)? }})),\n'
    demo += f'        Din16798Mutation::{s["Pascal"]}({s["Pascal"]} {{ {t["id"]}: "x".into(), {s["value_field"]}: {"\"x\".into()" if s["type"]=="String" else "1.0"} }}),\n'
patch(tx, '        Din16798Mutation::ChangeZoneVentMethod(p) => format!("change-zone-vent-method {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-vent-method={}", enc_json(&p.new_vent_method))].join(" ")),\n', fmt)
patch(tx, '        Din16798Mutation::ChangeZoneVentMethod(ChangeZoneVentMethod { zone_id: "x".into(), new_vent_method: "method_3_predefined_rates".into() }),\n', demo)
t_ = read(tx)
m = re.search(r'        "change-vent-duct-class" => [^\n]*\n', t_)
assert m
write(tx, t_.replace(m.group(0), m.group(0) + parse, 1))
patch(tx, 'use semio_s_artifact_norm_din16798_mut_ventilation::change_vent_design_airflow;\n', ''.join(f"use semio_s_artifact_norm_din16798_mut_{s['t']['crate']}::{s['snake']};\n" for s in SPECS))

# binary io
bn = f'{IO}/💾️binary/🧬️mutations/🦀️.rs'
patch(bn, 'use semio_s_artifact_norm_din16798_mut_ventilation::change_vent_design_airflow::ChangeVentDesignAirflow;\n', ''.join(f"use semio_s_artifact_norm_din16798_mut_{s['t']['crate']}::{s['snake']}::{s['Pascal']};\n" for s in SPECS))
tags = ''.join(f'\nconst {s["TAG"]}: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "{s["kind"]}");\n' for s in SPECS)
bt = read(bn)
m = re.search(r'const TAG_CHANGE_VENT_DESIGN_AIRFLOW: u8 = [^\n]*\n', bt)
assert m
bt = bt.replace(m.group(0), m.group(0) + tags, 1)
m = re.search(r'            Din16798Mutation::ChangeVentDesignAirflow\(_\) => TAG_CHANGE_VENT_DESIGN_AIRFLOW,\n', bt)
assert m
bt = bt.replace(m.group(0), m.group(0) + ''.join(f'            Din16798Mutation::{s["Pascal"]}(_) => {s["TAG"]},\n' for s in SPECS), 1)
m = re.search(r'            Din16798Mutation::ChangeVentDesignAirflow\(p\) => \{\n.*?\n            \}\n', bt, re.S)
assert m
enc = ''.join(f'''            Din16798Mutation::{s["Pascal"]}(p) => {{
                write_json_bin(&mut out, &p.{s["t"]["id"]});
                write_json_bin(&mut out, &p.{s["value_field"]});
            }}
''' for s in SPECS)
bt = bt.replace(m.group(0), m.group(0) + enc, 1)
m = re.search(r'            TAG_CHANGE_VENT_DESIGN_AIRFLOW => \{\n.*?\n            \}\n', bt, re.S)
assert m
dec = ''.join(f'''            {s["TAG"]} => {{
                let {s["t"]["id"]} = read_json_bin(&mut reader).map_err(|e| malformed("{s["t"]["id"]}", reader.position(), e))?;
                let {s["value_field"]} = read_json_bin(&mut reader).map_err(|e| malformed("{s["value_field"]}", reader.position(), e))?;
                Ok(Din16798Mutation::{s["Pascal"]}({s["Pascal"]} {{ {s["t"]["id"]}, {s["value_field"]} }}))
            }}
''' for s in SPECS)
bt = bt.replace(m.group(0), m.group(0) + dec, 1)
write(bn, bt)
proto = f'{IO}/💾️binary/🧬️mutations/📡️.protocol.semio'
pt = read(proto).rstrip('\n') + '\n'
pt += ''.join(f'\nrecord {s["kind"]} tag={s["tag"]}\nfield {kebab_of(s["t"]["idj"])}\nfield {s["value_kebab"]}\n' for s in SPECS)
write(proto, pt)

# oracle manifest
om = f'{S}/🔮️oracles/🔣️.json'
ot = read(om)
scen = ''.join(f''',
        {{
          "mutationId": "{s["kind"]}",
          "sourceMutationDirectoryName": "{s["dir"]}",
          "mutationDirectoryName": "{s["dir"]}",
          "scenarios": [
            {{
              "id": "apply",
              "directoryName": "✅apply"
            }}
          ]
        }}''' for s in SPECS)
a = '''          "mutationId": "change-vent-design-airflow",
          "sourceMutationDirectoryName": "🌬️change-vent-design-airflow",
          "mutationDirectoryName": "🌬️change-vent-design-airflow",
          "scenarios": [
            {
              "id": "apply",
              "directoryName": "✅apply"
            }
          ]
        }'''
assert ot.count(a) == 1
ot = ot.replace(a, a + scen)
a = '        "change-vent-design-airflow"\n'
assert ot.count(a) == 1
ot = ot.replace(a, '        "change-vent-design-airflow",\n' + ',\n'.join(f'        "{s["kind"]}"' for s in SPECS) + '\n')
cases = ''.join(f''',
        {{
          "id": "{s["kind"]}",
          "capability": "din16798-1-mutate",
          "payloadSchema": "🧬️.schema.json",
          "outcomes": [
            "applied",
            "rejected"
          ],
          "productionDispatch": {{
            "operation": "{s["kind"]}",
            "bridgeVersion": 1,
            "variant": "{s["Pascal"]}"
          }},
          "oracleRequirements": [
            {{
              "capability": "din16798-1-mutate",
              "qualifyingKind": "verified-native-second-implementation"
            }}
          ]
        }}''' for s in SPECS)
m = re.search(r'        \{\n          "id": "change-vent-design-airflow",\n.*?\n        \}', ot, re.S)
assert m
ot = ot.replace(m.group(0), m.group(0) + cases, 1)
write(om, ot)
json.loads(ot)

# python oracle + feature
py = f'{S}/🧪️tests/🌬️mutate-din16798-1/🐍️.py'
pt_ = read(py)
pt_ = pt_.replace('    "change-vent-design-airflow",\n]', '    "change-vent-design-airflow",\n' + ''.join(f'    "{s["kind"]}",\n' for s in SPECS) + ']', 1)
pt_ = pt_.replace('    "change-vent-design-airflow": ("🌬️change-vent-design-airflow", "✅apply"),\n', '    "change-vent-design-airflow": ("🌬️change-vent-design-airflow", "✅apply"),\n' + ''.join(f'    "{s["kind"]}": ("{s["dir"]}", "✅apply"),\n' for s in SPECS), 1)
write(py, pt_)
ft = f'{S}/🧪️tests/🌬️mutate-din16798-1/🥒️.feature'
tt = read(ft)
row_a = '      | change-vent-design-airflow   | 🌬️change-vent-design-airflow   | ✅apply  |\n'
assert tt.count(row_a) == 2
rows = ''.join(f'      | {s["kind"]:<28} | {s["dir"]:<30} | ✅apply  |\n' for s in SPECS)
tt = tt.replace(row_a, row_a + rows)
tt = tt.replace('the 41 kinds', f'the {41 + len(SPECS)} kinds').replace('every one of the 41 kinds', f'every one of the {41 + len(SPECS)} kinds').replace('The 41 kinds', f'The {41 + len(SPECS)} kinds')
write(ft, tt)

# taxonomy rows
tax = load(TAX)
rows = tax['rows']
last = max(i for i, r in enumerate(rows) if r['artifact'] == '🌬️din16798')
new = []
for s in SPECS:
    t = s['t']
    new.append({
        'aggregateVariant': s['Pascal'], 'artifact': '🌬️din16798',
        'edit': {'rule': 'set-field', 'path': s['edit_path'], 'selectors': [{'kind': 'id', 'key': t['idj']}], 'value': s['value_field_json']},
        'entity': s['entity'], 'kind': s['kind'], 'module': s['dir'], 'physicalLayout': 'direct', 'record': s['Pascal'],
        'source': f"🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{s['dir']}/🦀️.rs",
        'standard': '🔖️1', 'subset': '✳️any', 'type': s['Pascal'], 'verb': 'change',
    })
rows[last + 1:last + 1] = new
dump(TAX, tax)
print('generated', len(SPECS), 'kinds; tags', [s['tag'] for s in SPECS])
