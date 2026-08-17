#!/usr/bin/env python3
"""Generate FEM artifact/snapshot/diff schema leaves for fem2d and fem3d."""
from __future__ import annotations
from pathlib import Path
import json

FEM = Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem')
ARTS = {
    'fem2d': FEM / '🗿️artifacts' / '◻2d',
    'fem3d': FEM / '🗿️artifacts' / '🧊️3d',
}

# Field inventories: (camel, rust_snake, rust_type, state, optional, cardinality)
# cardinality: scalar|list
# For diff, types change (Option wrappers / deltas)

FEM2D_PERSISTENT = [
    ('nodes', 'nodes', 'Vec<FemNode>', 'persistent', False, 'list', 'FemNode'),
    ('elements', 'elements', 'Vec<FemElement>', 'persistent', False, 'list', 'FemElement'),
    ('regions', 'regions', 'Vec<FemRegion>', 'persistent', False, 'list', 'FemRegion'),
    ('materials', 'materials', 'Vec<FemMaterial>', 'persistent', False, 'list', 'FemMaterial'),
    ('sections', 'sections', 'Vec<FemSection>', 'persistent', False, 'list', 'FemSection'),
    ('supports', 'supports', 'Vec<FemSupport>', 'persistent', False, 'list', 'FemSupport'),
    ('loadCases', 'load_cases', 'Vec<FemLoadCase>', 'persistent', False, 'list', 'FemLoadCase'),
    ('combinations', 'combinations', 'Vec<FemCombination>', 'persistent', False, 'list', 'FemCombination'),
    ('analysis', 'analysis', 'FemAnalysisSettings', 'persistent', False, 'scalar', 'FemAnalysisSettings'),
]

FEM2D_UI = [
    ('resultSourceId', 'result_source_id', 'Option<String>', 'shared_ui', True, 'scalar', 'string'),
    ('resultMode', 'result_mode', 'String', 'shared_ui', False, 'scalar', 'string'),
    ('resultModeIndex', 'result_mode_index', 'u32', 'shared_ui', False, 'scalar', 'uint32'),
    ('camera', 'camera', 'FemCamera', 'local_ui', False, 'scalar', 'FemCamera'),
    ('locale', 'locale', 'String', 'local_ui', False, 'scalar', 'string'),
]

FEM2D_PREVIEW = [
    ('solverResultsJson', 'solver_results_json', 'String', 'preview', False, 'scalar', 'string'),
    ('meshPreviewJson', 'mesh_preview_json', 'String', 'preview', False, 'scalar', 'string'),
]

FEM3D_PERSISTENT = [
    ('nodes', 'nodes', 'Vec<FemNode>', 'persistent', False, 'list', 'FemNode'),
    ('elements', 'elements', 'Vec<FemElement>', 'persistent', False, 'list', 'FemElement'),
    ('materials', 'materials', 'Vec<FemMaterial>', 'persistent', False, 'list', 'FemMaterial'),
    ('sections', 'sections', 'Vec<FemSection>', 'persistent', False, 'list', 'FemSection'),
    ('solids', 'solids', 'Vec<FemSolid>', 'persistent', False, 'list', 'FemSolid'),
    ('supports', 'supports', 'Vec<FemSupport>', 'persistent', False, 'list', 'FemSupport'),
    ('loadCases', 'load_cases', 'Vec<FemLoadCase>', 'persistent', False, 'list', 'FemLoadCase'),
    ('combinations', 'combinations', 'Vec<FemCombination>', 'persistent', False, 'list', 'FemCombination'),
    ('analysis', 'analysis', 'FemAnalysisSettings', 'persistent', False, 'scalar', 'FemAnalysisSettings'),
]

FEM3D_UI = [
    ('resultSourceId', 'result_source_id', 'Option<String>', 'shared_ui', True, 'scalar', 'string'),
    ('resultMode', 'result_mode', 'String', 'shared_ui', False, 'scalar', 'string'),
    ('resultModeIndex', 'result_mode_index', 'u32', 'shared_ui', False, 'scalar', 'uint32'),
    ('camera', 'camera', 'FemCamera', 'local_ui', False, 'scalar', 'FemCamera'),
]

FEM3D_PREVIEW = [
    ('solverResultsJson', 'solver_results_json', 'String', 'preview', False, 'scalar', 'string'),
    ('meshPreviewJson', 'mesh_preview_json', 'String', 'preview', False, 'scalar', 'string'),
]

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
    print('wrote', path.relative_to(FEM))

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

def gen_snapshot_rust(prefix, schema_id, art_mod, fields, envelope_id, extension):
    # Snapshot with DSL + codecs, first type must be XSnapshot
    field_lines = []
    for camel, snake, rtype, state, optional, card, scalar in fields:
        # add dsl attrs matching original document
        dsl = ''
        if snake in ('nodes','regions','materials','sections','supports','load_cases','combinations','solids'):
            dsl = '    #[dsl(table)]\n'
        elif snake == 'elements':
            dsl = '    #[dsl(statements, block)]\n'
        elif snake == 'analysis':
            dsl = '    #[dsl(block)]\n'
        field_lines.append(f'{dsl}    #[state(persistent)]\n    pub {snake}: {rtype},')

    body = '\n'.join(field_lines)
    return f'''//! 🧬️ {prefix} snapshot schema — persistent fields only.

use crate::artifacts::{art_mod}::{{FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport, {("FemRegion, " if any(f[1]=="regions" for f in fields) else "")}{("FemSolid, " if any(f[1]=="solids" for f in fields) else "")}}};
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Snapshot
/// 📸️ Persisted {prefix.lower()} document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "{envelope_id}", layout = "lines")]
#[artifact_schema(id = "{schema_id}")]
pub struct {prefix}Snapshot {{
{body}
}}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for {prefix}Snapshot {{
    const EXTENSION: &'static str = "{extension}";
    fn envelope_id() -> &'static str {{ "{envelope_id}" }}
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {{
        let body = match store::semio_format::split_text_preamble(text) {{
            Ok((_, rest)) => rest,
            Err(_) => text,
        }};
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions {{ limits: dsl::Limits::default(), mode: dsl::SourceMode::Document }},
        )?;
        Self::__dsl_from_record(&record)
    }}
    fn print_dsl(&self) -> String {{
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }}
}}

impl store::DocumentPack for {prefix}Snapshot {{
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {{
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }}
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {{
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {{
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {{}}, got {{}}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }}
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }}
    fn record_spec() -> Option<dsl::RecordSpec> {{ Some(Self::__dsl_spec()) }}
}}
//#endregion 🔖️HandcraftedDocumentCodecs
//#endregion 🔖️Snapshot
'''

def gen_diff_rust(prefix, schema_id, art_mod, art_fields, collections):
    lines = [
        f'//! 🧬️ {prefix} diff schema — sparse field delta over the artifact.',
        '',
        f'use crate::artifacts::{art_mod}::{{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport, {("FemRegion, " if "regions" in collections else "")}{("FemSolid, " if "solids" in collections else "")}}};',
        'use schema::ArtifactSchema;',
        'use serde::{Deserialize, Serialize};',
        '',
        '//#region 🔖️Diff',
        f'/// 🔺️ Sparse field delta for the {prefix.lower()} artifact.',
        '#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]',
        '#[serde(rename_all = "camelCase", default)]',
        f'#[artifact_schema(id = "{schema_id}")]',
        f'pub struct {prefix}Diff {{',
        f'    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::{art_mod}::schema::{prefix}Artifact>>,',
    ]
    for camel, snake, rtype, state, optional, card, scalar in art_fields:
        if state == 'effect': continue
        if camel in collections:
            pascal = camel[0].upper() + camel[1:]
            lines.append(f'    #[state({state})] pub {snake}: Option<{prefix}{pascal}Delta>,')
        elif optional and 'Option<String>' in rtype:
            lines.append(f'    #[state({state})] pub {snake}: Option<Option<String>>,')
        else:
            # wrap in Option for sparse
            inner = rtype
            lines.append(f'    #[state({state})] pub {snake}: Option<{inner}>,')
    lines.append('}')
    lines.append('//#endregion 🔖️Diff')
    lines.append('')
    lines.append('//#region 🔖️DeltaHelpers')
    for camel in collections:
        pascal = camel[0].upper() + camel[1:]
        item = DELTA_ITEM[camel]
        lines.append(f'/// 🧩 Identified-collection delta for `{camel}`.')
        lines.append('#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]')
        lines.append('#[serde(rename_all = "camelCase", default)]')
        lines.append(f'pub struct {prefix}{pascal}Delta {{')
        lines.append(f'    pub added: Vec<{item}>,')
        lines.append('    pub removed: Vec<String>,')
        lines.append(f'    pub patched: Vec<{prefix}{pascal}PatchEntry>,')
        lines.append('    pub reordered: Option<Vec<String>>,')
        lines.append('}')
        lines.append('')
        lines.append(f'/// 🩹 One patched `{camel}` entry (whole-entity replacement).')
        lines.append('#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]')
        lines.append('#[serde(rename_all = "camelCase")]')
        lines.append(f'pub struct {prefix}{pascal}PatchEntry {{')
        lines.append('    pub id: String,')
        lines.append(f'    pub item: {item},')
        lines.append('}')
        lines.append('')
    lines.append('//#endregion 🔖️DeltaHelpers')
    lines.append('')
    return '\n'.join(lines)

def gen_artifact_conversions_and_descriptor(prefix, art_mod, schema_id, persistent_snakes, ui_defaults):
    # Appended to artifact rust after struct
    snap_fields = ', '.join(f'{s}: self.{s}.clone()' for s in persistent_snakes)
    from_fields = ', '.join(f'{s}: snapshot.{s}' for s in persistent_snakes)
    set_fields = '\n'.join(f'        self.{s} = snapshot.{s};' for s in persistent_snakes)
    default_ui = ',\n            '.join(f'{k}: {v}' for k,v in ui_defaults.items())
    pers_default = ',\n            '.join(f'{s}: Default::default()' for s in persistent_snakes)
    return f'''
//#region 🔖️Conversions
impl Default for {prefix}Artifact {{
    fn default() -> Self {{
        Self {{
            {pers_default},
            {default_ui},
        }}
    }}
}}

impl {prefix}Artifact {{
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::{art_mod}::{prefix}Snapshot {{
        crate::artifacts::{art_mod}::{prefix}Snapshot {{
            {snap_fields},
        }}
    }}

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI/preview fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::{art_mod}::{prefix}Snapshot) -> Self {{
        Self {{
            {from_fields},
            ..Self::default()
        }}
    }}

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::{art_mod}::{prefix}Snapshot) {{
{set_fields}
    }}
}}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `{schema_id}` — fifteen handcrafted schema leaves.
pub fn {art_mod}_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {{
    schema::ArtifactSchemaDescriptor {{
        id: "{schema_id}",
        artifact: schema::FacetLeaves {{
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        }},
        snapshot: schema::FacetLeaves {{
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        }},
        diff: schema::FacetLeaves {{
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        }},
    }}
}}
//#endregion 🔖️Descriptor
'''

def emit_artifact(key, prefix, art_path, persistent, ui, preview, collections, defs, imports, envelope_id, extension, ui_defaults):
    fields = persistent + ui + preview
    schema_id = f's.fem.{key}'
    # dirs
    for sub in ['🧬️schema', '📸️snapshot/🧬️schema', '🔺️diff/🧬️schema']:
        (art_path / sub).mkdir(parents=True, exist_ok=True)

    # JSON
    write(art_path/'🧬️schema'/'🔣️component.json', gen_artifact_json(prefix, 'fem', key, fields, defs))
    write(art_path/'📸️snapshot'/'🧬️schema'/'🔣️component.json', gen_snapshot_json(prefix, 'fem', key, persistent, defs))
    write(art_path/'🔺️diff'/'🧬️schema'/'🔣️component.json', gen_diff_json(prefix, 'fem', key, fields, collections, defs))

    # GraphQL
    write(art_path/'🧬️schema'/'🔗️component.graphql', gen_artifact_graphql(prefix, fields))
    write(art_path/'📸️snapshot'/'🧬️schema'/'🔗️component.graphql', gen_snapshot_graphql(prefix, persistent))
    write(art_path/'🔺️diff'/'🧬️schema'/'🔗️component.graphql', gen_diff_graphql(prefix, fields, collections))

    # TS
    write(art_path/'🧬️schema'/'🟦️component.ts', gen_artifact_ts(prefix, fields))
    write(art_path/'📸️snapshot'/'🧬️schema'/'🟦️component.ts', gen_snapshot_ts(prefix, persistent))
    write(art_path/'🔺️diff'/'🧬️schema'/'🟦️component.ts', gen_diff_ts(prefix, fields, collections))

    # Proto
    write(art_path/'🧬️schema'/'🛰️component.proto', gen_artifact_proto(prefix, 'fem', key, fields))
    write(art_path/'📸️snapshot'/'🧬️schema'/'🛰️component.proto', gen_snapshot_proto(prefix, 'fem', key, persistent))
    write(art_path/'🔺️diff'/'🧬️schema'/'🛰️component.proto', gen_diff_proto(prefix, 'fem', key, fields, collections))

    # Rust artifact
    art_rs = gen_artifact_rust(prefix, schema_id, fields, imports)
    pers_snakes = [f[1] for f in persistent]
    art_rs += gen_artifact_conversions_and_descriptor(prefix, key, schema_id, pers_snakes, ui_defaults)
    write(art_path/'🧬️schema'/'🦀️component.rs', art_rs)

    # Rust snapshot
    write(art_path/'📸️snapshot'/'🧬️schema'/'🦀️component.rs', gen_snapshot_rust(prefix, schema_id, key, persistent, envelope_id, extension))

    # Rust diff schema
    write(art_path/'🔺️diff'/'🧬️schema'/'🦀️component.rs', gen_diff_rust(prefix, schema_id, key, fields, collections))

# Fem2d
emit_artifact(
    'fem2d', 'Fem2d', ARTS['fem2d'],
    FEM2D_PERSISTENT, FEM2D_UI, FEM2D_PREVIEW, COLLECTIONS_2D, DEFS_SHARED_CAMERA_2D,
    'fem2d::{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport}',
    'fem.fem2d', 'fem2d',
    {
        'result_source_id': 'None',
        'result_mode': '"static".into()',
        'result_mode_index': '0',
        'camera': 'FemCamera::default()',
        'locale': '"en-US".into()',
        'solver_results_json': 'String::new()',
        'mesh_preview_json': 'String::new()',
    },
)

# Fem3d
emit_artifact(
    'fem3d', 'Fem3d', ARTS['fem3d'],
    FEM3D_PERSISTENT, FEM3D_UI, FEM3D_PREVIEW, COLLECTIONS_3D, DEFS_SHARED_CAMERA_3D,
    'fem3d::{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport}',
    'fem.fem3d', 'fem3d',
    {
        'result_source_id': 'None',
        'result_mode': '"static".into()',
        'result_mode_index': '0',
        'camera': 'FemCamera::default()',
        'solver_results_json': 'String::new()',
        'mesh_preview_json': 'String::new()',
    },
)

print('ALL LEAVES GENERATED')
