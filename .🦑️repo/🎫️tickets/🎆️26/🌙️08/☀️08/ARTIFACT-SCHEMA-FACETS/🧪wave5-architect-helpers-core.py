STATE_GQL = {
    'persistent': 'PERSISTENT',
    'shared_ui': 'SHARED_UI',
    'local_ui': 'LOCAL_UI',
    'preview': 'PREVIEW',
    'effect': 'EFFECT',
}
STATE_KEBAB = {
    'persistent': 'persistent',
    'shared_ui': 'shared-ui',
    'local_ui': 'local-ui',
    'preview': 'preview',
    'effect': 'effect',
}

COLLECTIONS_2D = ['nodes','elements','regions','materials','sections','supports','loadCases','combinations']
COLLECTIONS_3D = ['nodes','elements','materials','sections','solids','supports','loadCases','combinations']

DELTA_ITEM = {
    'nodes': 'FemNode',
    'elements': 'FemElement',
    'regions': 'FemRegion',
    'materials': 'FemMaterial',
    'sections': 'FemSection',
    'solids': 'FemSolid',
    'supports': 'FemSupport',
    'loadCases': 'FemLoadCase',
    'combinations': 'FemCombination',
}

def rust_attr_state(s):
    return s  # already snake

def write(path: Path, content: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)
    print('wrote', path.relative_to(PLUGIN))

def json_prop_for(camel, optional, cardinality, scalar_token, state, is_json_blob=False):
    state_k = STATE_KEBAB[state]
    if cardinality == 'list':
        # items as $ref or object
        if scalar_token in ('string','bool','int32','uint32','int64','float32','float64'):
            items = {'type': {'string':'string','bool':'boolean','int32':'integer','uint32':'integer','int64':'integer','float32':'number','float64':'number'}[scalar_token]}
            if scalar_token.startswith('int') or scalar_token.startswith('uint'):
                items['format'] = scalar_token
            if scalar_token == 'float32': items['format']='float'
            if scalar_token == 'float64': items['format']='double'
        else:
            items = {'$ref': f'#/$defs/{scalar_token}'}
        return {'type':'array','items':items,'x-semio-state':state_k}
    # scalar
    if is_json_blob or (scalar_token == 'string' and camel.endswith('Json')):
        prop = {'type':'string','contentMediaType':'application/json','x-semio-state':state_k}
        return prop
    if scalar_token == 'string':
        prop = {'type':'string','x-semio-state':state_k}
    elif scalar_token == 'bool':
        prop = {'type':'boolean','x-semio-state':state_k}
    elif scalar_token == 'uint32':
        prop = {'type':'integer','format':'uint32','minimum':0,'x-semio-state':state_k}
    elif scalar_token == 'int32':
        prop = {'type':'integer','format':'int32','x-semio-state':state_k}
    elif scalar_token == 'int64':
        prop = {'type':'integer','format':'int64','x-semio-state':state_k}
    elif scalar_token == 'float64':
        prop = {'type':'number','format':'double','x-semio-state':state_k}
    elif scalar_token == 'float32':
        prop = {'type':'number','format':'float','x-semio-state':state_k}
    else:
        prop = {'$ref': f'#/$defs/{scalar_token}', 'x-semio-state': state_k}
    return prop

def gen_artifact_json(prefix, plugin_key, art_key, fields, defs):
    props = {}
    required = []
    for camel, snake, rtype, state, optional, card, scalar in fields:
        is_blob = camel.endswith('Json') and scalar == 'string'
        props[camel] = json_prop_for(camel, optional, card, scalar, state, is_blob)
        if not optional:
            required.append(camel)
    doc = {
        '$schema': 'https://json-schema.org/draft/2020-12/schema',
        '$id': f'https://semio.tech/schema/s/{plugin_key}/{art_key}/artifact.json',
        'title': f'{prefix}Artifact',
        'type': 'object',
        'additionalProperties': False,
        'required': required,
        'properties': props,
        '$defs': defs,
    }
    return json.dumps(doc, indent=2) + '\n'

def gen_snapshot_json(prefix, plugin_key, art_key, fields, defs):
    props = {}
    required = []
    for camel, snake, rtype, state, optional, card, scalar in fields:
        props[camel] = json_prop_for(camel, optional, card, scalar, 'persistent')
        if not optional:
            required.append(camel)
    doc = {
        '$schema': 'https://json-schema.org/draft/2020-12/schema',
        '$id': f'https://semio.tech/schema/s/{plugin_key}/{art_key}/snapshot.json',
        'title': f'{prefix}Snapshot',
        'type': 'object',
        'additionalProperties': False,
        'required': required,
        'properties': props,
        '$defs': defs,
    }
    return json.dumps(doc, indent=2) + '\n'

def gen_diff_json(prefix, plugin_key, art_key, art_fields, collections, defs):
    props = {
        'artifact': {
            'title': f'{prefix}Artifact',
            'type': 'object',
            'x-semio-state': 'persistent',
        }
    }
    # per field
    for camel, snake, rtype, state, optional, card, scalar in art_fields:
        state_k = STATE_KEBAB[state]
        if state == 'effect':
            continue
        if camel in collections:
            delta = f'{prefix}{camel[0].upper()+camel[1:]}Delta' if False else None
            # Fem2dNodesDelta from nodes
            delta_name = f'{prefix}{camel[0].upper()+camel[1:]}Delta'
            # fix: nodes -> Nodes, loadCases -> LoadCases
            pascal = camel[0].upper() + camel[1:]
            delta_name = f'{prefix}{pascal}Delta'
            props[camel] = {'$ref': f'#/$defs/{delta_name}', 'x-semio-state': state_k}
        elif optional and scalar == 'string':
            # Option<String> field -> Option<Option<String>> in diff = oneOf null|string
            props[camel] = {'oneOf': [{'type':'null'}, {'type':'string'}], 'x-semio-state': state_k}
        elif card == 'scalar':
            if scalar == 'string':
                if camel.endswith('Json'):
                    props[camel] = {'type':'string','contentMediaType':'application/json','x-semio-state':state_k}
                else:
                    props[camel] = {'type':'string','x-semio-state':state_k}
            elif scalar == 'bool':
                props[camel] = {'type':'boolean','x-semio-state':state_k}
            elif scalar == 'uint32':
                props[camel] = {'type':'integer','format':'uint32','minimum':0,'x-semio-state':state_k}
            elif scalar == 'float64':
                props[camel] = {'type':'number','format':'double','x-semio-state':state_k}
            else:
                props[camel] = {'$ref': f'#/$defs/{scalar}', 'x-semio-state': state_k}
        else:
            props[camel] = {'$ref': f'#/$defs/{scalar}', 'x-semio-state': state_k}

    # delta defs
    delta_defs = dict(defs)
    for camel in collections:
        pascal = camel[0].upper() + camel[1:]
        delta_name = f'{prefix}{pascal}Delta'
        delta_defs[delta_name] = {
            'title': delta_name,
            'type': 'object',
            'additionalProperties': False,
            'required': ['added','removed','patched'],
            'properties': {
                'added': {'type':'array','items':{'type':'object'}},
                'removed': {'type':'array','items':{'type':'string'}},
                'patched': {'type':'array','items':{'type':'object'}},
                'reordered': {'type':'array','items':{'type':'string'}},
            }
        }
    doc = {
        '$schema': 'https://json-schema.org/draft/2020-12/schema',
        '$id': f'https://semio.tech/schema/s/{plugin_key}/{art_key}/diff.json',
        'title': f'{prefix}Diff',
        'type': 'object',
        'additionalProperties': False,
        'required': [],
        'properties': props,
        '$defs': delta_defs,
    }
    return json.dumps(doc, indent=2) + '\n'

# Minimal $defs for nested records referenced by $ref
DEFS_SHARED_CAMERA_2D = {
    'FemCamera': {
        'title': 'FemCamera',
        'type': 'object',
        'additionalProperties': False,
        'required': ['x','y','zoom'],
        'properties': {
            'x': {'type':'number','format':'double'},
            'y': {'type':'number','format':'double'},
            'zoom': {'type':'number','format':'double'},
        }
    },
    'FemAnalysisSettings': {
        'title': 'FemAnalysisSettings',
        'type': 'object',
        'additionalProperties': False,
        'required': ['modalCount','bucklingCount','deformationScale'],
        'properties': {
            'modalCount': {'type':'integer','format':'uint32','minimum':0},
            'bucklingCount': {'type':'integer','format':'uint32','minimum':0},
            'deformationScale': {'type':'number','format':'double'},
        }
    },
    'FemNode': {'title':'FemNode','type':'object'},
    'FemElement': {'title':'FemElement','type':'object'},
    'FemRegion': {'title':'FemRegion','type':'object'},
    'FemMaterial': {'title':'FemMaterial','type':'object'},
    'FemSection': {'title':'FemSection','type':'object'},
    'FemSupport': {'title':'FemSupport','type':'object'},
    'FemLoadCase': {'title':'FemLoadCase','type':'object'},
    'FemCombination': {'title':'FemCombination','type':'object'},
}

DEFS_SHARED_CAMERA_3D = {
    'FemCamera': {
        'title': 'FemCamera',
        'type': 'object',
        'additionalProperties': False,
        'required': ['json'],
        'properties': {
            'json': {'type':'string','contentMediaType':'application/json'},
        }
    },
    'FemAnalysisSettings': DEFS_SHARED_CAMERA_2D['FemAnalysisSettings'],
    'FemNode': {'title':'FemNode','type':'object'},
    'FemElement': {'title':'FemElement','type':'object'},
    'FemSolid': {'title':'FemSolid','type':'object'},
    'FemMaterial': {'title':'FemMaterial','type':'object'},
    'FemSection': {'title':'FemSection','type':'object'},
    'FemSupport': {'title':'FemSupport','type':'object'},
    'FemLoadCase': {'title':'FemLoadCase','type':'object'},
    'FemCombination': {'title':'FemCombination','type':'object'},
}

def gql_type(scalar, card, optional, list_inner_bang=True):
    if card == 'list':
        inner = f'[{scalar}!]!' if not optional else f'[{scalar}!]'
        # wait: required list is [T!]!, optional list is [T!]
        return f'[{scalar}!]!' if not optional else f'[{scalar}!]'
    # scalar
    base = {'string':'String','bool':'Boolean','uint32':'Int','int32':'Int','int64':'Int','float64':'Float','float32':'Float'}.get(scalar, scalar)
    if optional:
        return base  # no !
    return f'{base}!'

def gen_artifact_graphql(prefix, fields):
    lines = [f'# 🧬️ {prefix} artifact schema — every field with its state class.', '', f'type {prefix}Artifact {{']
    for camel, snake, rtype, state, optional, card, scalar in fields:
        sc = {'string':'String','bool':'Boolean','uint32':'Int','int32':'Int','int64':'Int','float64':'Float','float32':'Float'}.get(scalar, scalar)
        typ = gql_type(sc if card=='scalar' else scalar, card, optional)
        # fix list: use entity name
        if card == 'list':
            typ = f'[{scalar}!]!' if not optional else f'[{scalar}!]'
        lines.append(f'  {camel}: {typ} @state(class: {STATE_GQL[state]})')
    lines.append('}')
    lines.append('')
    # stub nested types so GraphQL is parseable — scanners only take first type
    # still need them? Extractor takes first type only. Stubs optional.
    return '\n'.join(lines) + '\n'

def gen_snapshot_graphql(prefix, fields):
    lines = [f'# 🧬️ {prefix} snapshot schema — persistent fields only.', '', f'type {prefix}Snapshot {{']
    for camel, snake, rtype, state, optional, card, scalar in fields:
        if card == 'list':
            typ = f'[{scalar}!]!'
        else:
            sc = {'string':'String','bool':'Boolean','uint32':'Int','float64':'Float'}.get(scalar, scalar)
            typ = f'{sc}!'
        lines.append(f'  {camel}: {typ} @state(class: PERSISTENT)')
    lines.append('}')
    lines.append('')
    return '\n'.join(lines) + '\n'

def gen_diff_graphql(prefix, art_fields, collections):
    lines = [f'# 🧬️ {prefix} diff schema — sparse field delta.', '', f'type {prefix}Diff {{']
    lines.append(f'  artifact: {prefix}Artifact @state(class: PERSISTENT)')
    for camel, snake, rtype, state, optional, card, scalar in art_fields:
        if state == 'effect':
            continue
        st = STATE_GQL[state]
        if camel in collections:
            pascal = camel[0].upper() + camel[1:]
            lines.append(f'  {camel}: {prefix}{pascal}Delta @state(class: {st})')
        elif optional and scalar == 'string':
            lines.append(f'  {camel}: String @state(class: {st})')  # Option<Option> still optional String in GQL
        else:
            sc = {'string':'String','bool':'Boolean','uint32':'Int','float64':'Float'}.get(scalar, scalar)
            lines.append(f'  {camel}: {sc} @state(class: {st})')
    lines.append('}')
    lines.append('')
    for camel in collections:
        pascal = camel[0].upper() + camel[1:]
        lines.append(f'type {prefix}{pascal}Delta {{')
        lines.append('  added: [JSON!]!')
        lines.append('  removed: [String!]!')
        lines.append('  patched: [JSON!]!')
        lines.append('  reordered: [String!]')
        lines.append('}')
        lines.append('')
    # JSON scalar stub - actually GraphQL may not have JSON. Use generic object names.
    # Replace JSON with String for simplicity? Better use generic Object types.
    # Recreate without JSON:
    out = [f'# 🧬️ {prefix} diff schema — sparse field delta.', '', f'type {prefix}Diff {{']
    out.append(f'  artifact: {prefix}Artifact @state(class: PERSISTENT)')
    for camel, snake, rtype, state, optional, card, scalar in art_fields:
        if state == 'effect': continue
        st = STATE_GQL[state]
        if camel in collections:
            pascal = camel[0].upper() + camel[1:]
            out.append(f'  {camel}: {prefix}{pascal}Delta @state(class: {st})')
        else:
            sc = {'string':'String','bool':'Boolean','uint32':'Int','float64':'Float'}.get(scalar, scalar)
            out.append(f'  {camel}: {sc} @state(class: {st})')
    out.append('}')
    out.append('')
    for camel in collections:
        pascal = camel[0].upper() + camel[1:]
        item = DELTA_ITEM[camel]
        out.append(f'type {prefix}{pascal}Delta {{')
        out.append(f'  added: [{item}!]!')
        out.append('  removed: [String!]!')
        out.append(f'  patched: [{prefix}{pascal}PatchEntry!]!')
        out.append('  reordered: [String!]')
        out.append('}')
        out.append('')
        out.append(f'type {prefix}{pascal}PatchEntry {{')
        out.append('  id: String!')
        out.append(f'  item: {item}!')
        out.append('}')
        out.append('')
    return '\n'.join(out) + '\n'

def ts_type(scalar, card, optional):
    prim = {'string':'string','bool':'boolean','uint32':'number','int32':'number','int64':'number','float64':'number','float32':'number'}.get(scalar, scalar)
    if card == 'list':
        t = f'{scalar}[]'
    else:
        t = prim
    return t

def gen_artifact_ts(prefix, fields):
    lines = [f'/** 🧬️ {prefix} artifact schema — every field with its state class. */', '', f'export interface {prefix}Artifact {{']
    for camel, snake, rtype, state, optional, card, scalar in fields:
        t = ts_type(scalar, card, optional)
        opt = '?' if optional else ''
        lines.append(f'  /** @state {STATE_KEBAB[state]} */')
        lines.append(f'  {camel}{opt}: {t};')
    lines.append('}')
    lines.append('')
    return '\n'.join(lines) + '\n'

def gen_snapshot_ts(prefix, fields):
    lines = [f'/** 🧬️ {prefix} snapshot schema — persistent fields only. */', '', f'export interface {prefix}Snapshot {{']
    for camel, snake, rtype, state, optional, card, scalar in fields:
        t = ts_type(scalar, card, optional)
        lines.append(f'  /** @state persistent */')
        lines.append(f'  {camel}: {t};')
    lines.append('}')
    lines.append('')
    return '\n'.join(lines) + '\n'

def gen_diff_ts(prefix, art_fields, collections):
    lines = [f'/** 🧬️ {prefix} diff schema — sparse field delta. */', '', f'export interface {prefix}Diff {{']
    lines.append(f'  /** @state persistent */')
    lines.append(f'  artifact?: {prefix}Artifact;')
    for camel, snake, rtype, state, optional, card, scalar in art_fields:
        if state == 'effect': continue
        lines.append(f'  /** @state {STATE_KEBAB[state]} */')
        if camel in collections:
            pascal = camel[0].upper() + camel[1:]
            lines.append(f'  {camel}?: {prefix}{pascal}Delta;')
        elif optional and scalar == 'string':
            lines.append(f'  {camel}?: string | null;')
        else:
            t = ts_type(scalar, 'scalar', False)
            lines.append(f'  {camel}?: {t};')
    lines.append('}')
    lines.append('')
    for camel in collections:
        pascal = camel[0].upper() + camel[1:]
        item = DELTA_ITEM[camel]
        lines.append(f'export interface {prefix}{pascal}Delta {{')
        lines.append(f'  added: {item}[];')
        lines.append('  removed: string[];')
        lines.append(f'  patched: {prefix}{pascal}PatchEntry[];')
        lines.append('  reordered?: string[];')
        lines.append('}')
        lines.append('')
        lines.append(f'export interface {prefix}{pascal}PatchEntry {{')
        lines.append('  id: string;')
        lines.append(f'  item: {item};')
        lines.append('}')
        lines.append('')
    return '\n'.join(lines) + '\n'

def proto_type(scalar):
    return {'string':'string','bool':'bool','uint32':'uint32','int32':'int32','int64':'int64','float64':'double','float32':'float'}.get(scalar, scalar)

def gen_artifact_proto(prefix, plugin_key, art_key, fields):
    lines = [
        'syntax = "proto3";',
        f'package semio.s.{plugin_key}.{art_key}.artifact;',
        '',
        f'// 🧬️ {prefix} artifact schema',
        f'message {prefix}Artifact {{',
    ]
    n = 1
    for camel, snake, rtype, state, optional, card, scalar in fields:
        lines.append(f'  // @state {STATE_KEBAB[state]}')
        if card == 'list':
            lines.append(f'  repeated {scalar} {snake} = {n};')
        elif optional:
            pt = proto_type(scalar) if scalar in ('string','bool','uint32','int32','int64','float64','float32') else scalar
            lines.append(f'  optional {pt} {snake} = {n};')
        else:
            pt = proto_type(scalar) if scalar in ('string','bool','uint32','int32','int64','float64','float32') else scalar
            lines.append(f'  {pt} {snake} = {n};')
        n += 1
    lines.append('}')
    lines.append('')
    return '\n'.join(lines) + '\n'

def gen_snapshot_proto(prefix, plugin_key, art_key, fields):
    lines = [
        'syntax = "proto3";',
        f'package semio.s.{plugin_key}.{art_key}.snapshot;',
        '',
        f'// 🧬️ {prefix} snapshot schema',
        f'message {prefix}Snapshot {{',
    ]
    n = 1
    for camel, snake, rtype, state, optional, card, scalar in fields:
        lines.append('  // @state persistent')
        if card == 'list':
            lines.append(f'  repeated {scalar} {snake} = {n};')
        else:
            pt = proto_type(scalar) if scalar in ('string','bool','uint32','int32','int64','float64','float32') else scalar
            lines.append(f'  {pt} {snake} = {n};')
        n += 1
    lines.append('}')
    lines.append('')
    return '\n'.join(lines) + '\n'

def gen_diff_proto(prefix, plugin_key, art_key, art_fields, collections):
    lines = [
        'syntax = "proto3";',
        f'package semio.s.{plugin_key}.{art_key}.diff;',
        '',
        f'// 🧬️ {prefix} diff schema',
        f'message {prefix}Diff {{',
    ]
    n = 1
    lines.append('  // @state persistent')
    lines.append(f'  optional {prefix}Artifact artifact = {n};')
    n += 1
    for camel, snake, rtype, state, optional, card, scalar in art_fields:
        if state == 'effect': continue
        lines.append(f'  // @state {STATE_KEBAB[state]}')
        if camel in collections:
            pascal = camel[0].upper() + camel[1:]
            lines.append(f'  optional {prefix}{pascal}Delta {snake} = {n};')
        else:
            pt = proto_type(scalar) if scalar in ('string','bool','uint32','int32','int64','float64','float32') else scalar
            lines.append(f'  optional {pt} {snake} = {n};')
        n += 1
    lines.append('}')
    lines.append('')
    for camel in collections:
        pascal = camel[0].upper() + camel[1:]
        item = DELTA_ITEM[camel]
        lines.append(f'message {prefix}{pascal}Delta {{')
        lines.append(f'  repeated {item} added = 1;')
        lines.append('  repeated string removed = 2;')
        lines.append(f'  repeated {prefix}{pascal}PatchEntry patched = 3;')
        lines.append('  repeated string reordered = 4;')
        lines.append('}')
        lines.append('')
        lines.append(f'message {prefix}{pascal}PatchEntry {{')
        lines.append('  string id = 1;')
        lines.append(f'  {item} item = 2;')
        lines.append('}')
        lines.append('')
    return '\n'.join(lines) + '\n'

def gen_artifact_rust(prefix, schema_id, fields, imports):
    lines = [
        f'//! 🧬️ {prefix} artifact schema — every field of the artifact with its state class.',
        '',
        f'use crate::artifacts::{imports};',
        'use schema::ArtifactSchema;',
        'use serde::{Deserialize, Serialize};',
        '',
        '//#region 🔖️Artifact',
        f'/// 🧬️ Full {prefix.lower()} artifact state across persistent, shared-ui, local-ui and preview classes.',
        '#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]',
        '#[serde(rename_all = "camelCase")]',
        f'#[artifact_schema(id = "{schema_id}")]',
        f'pub struct {prefix}Artifact {{',
    ]
    for camel, snake, rtype, state, optional, card, scalar in fields:
        lines.append(f'    #[state({state})] pub {snake}: {rtype},')
    lines.append('}')
    lines.append('//#endregion 🔖️Artifact')
    lines.append('')
    return '\n'.join(lines) + '\n'

