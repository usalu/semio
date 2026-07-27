#!/usr/bin/env python3
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from dsl_fixture_tool import tokenize, P, parse_record, kebab, print_flat_record

VORTEX_TEMPLATE_FIELDS = {k: 'scalar' for k in ['vortex_kind', 'position', 'direction', 'radius']}
VORTEX_TEMPLATE_FIELDS['position'] = ('tuple', 3)
VORTEX_TEMPLATE_FIELDS['direction'] = ('tuple', 3)

OBJECT_KIND_FIELDS = {k: 'scalar' for k in ['id', 'label', 'name', 'mesh_url']}
OBJECT_KIND_FIELDS['vortices'] = ('list_record', VORTEX_TEMPLATE_FIELDS)

VORTEX_KIND_FIELDS = {k: 'scalar' for k in ['id', 'label', 'name', 'color', 'default_cable_kind']}
CABLE_KIND_FIELDS = {k: 'scalar' for k in ['id', 'label', 'name', 'default_attraction_kind']}
ATTRACTION_KIND_FIELDS = {k: 'scalar' for k in ['id', 'label', 'name']}

KIND_CATALOGS_FIELDS = {
    'objects': ('list_record', OBJECT_KIND_FIELDS),
    'vortices': ('list_record', VORTEX_KIND_FIELDS),
    'cables': ('list_record', CABLE_KIND_FIELDS),
    'attractions': ('list_record', ATTRACTION_KIND_FIELDS),
}

KC_FIELDS = {k: 'scalar' for k in ['source', 'target', 'bidirectional', 'important', 'specificity']}

META_FIELDS = {'kind_catalogs': 'record', 'kind_catalogs__fields': KIND_CATALOGS_FIELDS, 'kind_compatibility': ('list_record', KC_FIELDS)}

CAMERA_FIELDS = {'position': ('tuple', 3), 'target': ('tuple', 3), 'zoom': 'scalar', 'up': ('tuple', 3), 'projection': 'value_json'}

VORTEX_FIELDS = {k: 'scalar' for k in ['id', 'vortex_kind', 'label', 'radius', 'hidden', 'locked']}
VORTEX_FIELDS['position'] = ('tuple', 3)
VORTEX_FIELDS['direction'] = ('tuple', 3)

OBJECT_FIELDS = {k: 'scalar' for k in ['id', 'label', 'object_kind', 'mesh_url', 'hidden', 'locked']}
OBJECT_FIELDS['origin'] = ('tuple', 3)
OBJECT_FIELDS['orientation'] = ('tuple', 4)
OBJECT_FIELDS['scale'] = 'value_json'
OBJECT_FIELDS['vortices'] = ('list_record', VORTEX_FIELDS)

ATTRACTION_FIELDS = {k: 'scalar' for k in ['id', 'attracting', 'attracted', 'gap', 'shift', 'rise', 'rotation', 'turn', 'tilt']}

TARGET_VOLUME_FIELDS = {k: 'scalar' for k in ['id', 'hidden', 'locked']}
TARGET_VOLUME_FIELDS['origin'] = ('tuple', 3)
TARGET_VOLUME_FIELDS['orientation'] = ('tuple', 4)
TARGET_VOLUME_FIELDS['scale'] = 'value_json'

REFERENCE_SOURCE_FIELDS = {k: 'scalar' for k in ['url', 'media_kind']}
REFERENCE_FIELDS = {k: 'scalar' for k in ['id', 'width_world', 'locked', 'hidden']}
REFERENCE_FIELDS['source'] = 'record'
REFERENCE_FIELDS['source__fields'] = REFERENCE_SOURCE_FIELDS
REFERENCE_FIELDS['origin'] = ('tuple', 3)

ROOT_FIELDS = {
    'schema': 'scalar', 'domain': 'scalar',
    'camera': 'block', 'camera__fields': CAMERA_FIELDS,
    'meta': 'block', 'meta__fields': META_FIELDS,
    'objects': ('list_record', OBJECT_FIELDS),
    'attractions': ('list_record', ATTRACTION_FIELDS),
    'target_volumes': ('list_record', TARGET_VOLUME_FIELDS),
    'references': ('list_record', REFERENCE_FIELDS),
}


def convert(path_in, path_out):
    text = open(path_in).read()
    toks = tokenize(text)
    p = P(toks)
    root = parse_record(p, ROOT_FIELDS)
    assert p.peek()[0] == 'EOF', f"leftover tokens at {p.pos}: {p.toks[p.pos:p.pos+10]}"
    out = print_flat_record(root, ROOT_FIELDS) + '\n'
    with open(path_out, 'w') as f:
        f.write(out)
    print(f"wrote {path_out} ({len(out)} bytes) — {len(root['objects'][1])} objects, {len(root['attractions'][1])} attractions")


if __name__ == '__main__':
    base = '/Users/ueli/Documents/semio/puzzle/3d/example'
    for name in ['concrete-forest', 'nakagin-capsule-tower']:
        convert(f'{base}/{name}.puzzle3d', f'{base}/{name}.puzzle3d.new')
