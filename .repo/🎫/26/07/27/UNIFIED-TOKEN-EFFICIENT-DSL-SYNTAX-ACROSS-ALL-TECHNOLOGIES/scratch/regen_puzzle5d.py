#!/usr/bin/env python3
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from dsl_fixture_tool import tokenize, P, parse_record, print_flat_record

CATALOG_GRIP_TEMPLATE_2D = {k: 'scalar' for k in ['angle', 'grip_kind', 'radius']}
CATALOG_GRIP_TEMPLATE_3D = {'position': ('tuple', 3), 'direction': ('tuple', 3), 'radius': 'scalar'}
CATALOG_GRIP_TEMPLATE = {'grip_kind': 'scalar', 'grip_2d': 'record', 'grip_2d__fields': CATALOG_GRIP_TEMPLATE_2D, 'grip_3d': 'record', 'grip_3d__fields': CATALOG_GRIP_TEMPLATE_3D}
CATALOG_PART = {k: 'scalar' for k in ['id', 'name', 'label', 'mesh_url']}
CATALOG_PART['grips'] = ('list_record', CATALOG_GRIP_TEMPLATE)
CATALOG_GRIP = {k: 'scalar' for k in ['id', 'name', 'label', 'color', 'default_rope_kind']}
CATALOG_FASTENER = {k: 'scalar' for k in ['id', 'name', 'label']}
CATALOG_ROPE = {k: 'scalar' for k in ['id', 'name', 'label', 'default_fastener_kind']}
KIND_CATALOGS_FIELDS = {
    'parts': ('list_record', CATALOG_PART),
    'grips': ('list_record', CATALOG_GRIP),
    'fasteners': ('list_record', CATALOG_FASTENER),
    'ropes': ('list_record', CATALOG_ROPE),
}

GRIP_2D = {'angle': 'scalar', 'grip_kind': 'scalar', 'radius': 'scalar'}
GRIP_3D = {'position': ('tuple', 3), 'direction': ('tuple', 3), 'radius': 'scalar', 'label': 'scalar'}
GRIP = {'id': 'scalar', 'grip_kind': 'scalar', 'grip_2d': 'record', 'grip_2d__fields': GRIP_2D, 'grip_3d': 'record', 'grip_3d__fields': GRIP_3D}

PART_2D = {k: 'scalar' for k in ['x', 'y', 'shape', 'radius', 'width', 'height', 'text', 'icon_kind', 'hidden', 'locked']}
PART_3D = {k: 'scalar' for k in ['mesh_url', 'label']}
PART_3D['origin'] = ('tuple', 3)
PART_3D['orientation'] = ('tuple', 4)
PART_3D['scale'] = 'value_json'
PART = {'id': 'scalar', 'part_kind': 'scalar', 'part_2d': 'record', 'part_2d__fields': PART_2D, 'part_3d': 'record', 'part_3d__fields': PART_3D, 'grips': ('list_record', GRIP)}

FASTENER = {k: 'scalar' for k in ['id', 'source', 'target', 'fastener_kind']}
KC = {k: 'scalar' for k in ['source', 'target', 'bidirectional']}

CAMERA2D = {k: 'scalar' for k in ['x', 'y', 'zoom']}
CAMERA3D = {'position': ('tuple', 3), 'target': ('tuple', 3), 'zoom': 'scalar'}
META = {'description': 'scalar'}

ROOT_FIELDS = {
    'schema': 'scalar', 'domain': 'scalar', 'label': 'scalar',
    'camera2d': 'block', 'camera2d__fields': CAMERA2D,
    'camera3d': 'block', 'camera3d__fields': CAMERA3D,
    'meta': 'block', 'meta__fields': META,
    'kind_catalogs': 'record', 'kind_catalogs__fields': KIND_CATALOGS_FIELDS,
    'kind_compatibility': ('list_record', KC),
    'parts': ('list_record', PART),
    'fasteners': ('list_record', FASTENER),
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
    print(f"wrote {path_out} ({len(out)} bytes) — {len(root['parts'][1])} parts, {len(root['fasteners'][1])} fasteners")


if __name__ == '__main__':
    base = '/Users/ueli/Documents/semio/puzzle/5d/example'
    for name in ['concrete-forest', 'nakagin-capsule-tower']:
        convert(f'{base}/{name}.puzzle5d', f'{base}/{name}.puzzle5d.new')
