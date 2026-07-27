#!/usr/bin/env python3
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from dsl_fixture_tool import tokenize, P, parse_record, kebab, fmt_scalar_value

HANDLE_FIELDS = {k: 'scalar' for k in ['id', 'handle_kind', 'angle', 'radius', 'color', 'icon_kind', 'scale', 'visible', 'locked']}
NODE_FIELDS = {k: 'scalar' for k in ['id', 'node_kind', 'shape', 'x', 'y', 'radius', 'width', 'height', 'text', 'icon_kind', 'root', 'scale', 'visible', 'locked']}
NODE_FIELDS['handles'] = ('list_record', HANDLE_FIELDS)
EDGE_FIELDS = {k: 'scalar' for k in ['id', 'source', 'target', 'edge_kind', 'source_tip', 'target_tip', 'visible', 'locked']}
KC_FIELDS = {k: 'scalar' for k in ['bidirectional', 'specificity', 'source', 'target']}
META_FIELDS = {'manifest_id': 'scalar', 'kind_catalogs': 'value_json'}
META_FIELDS['kind_compatibility'] = ('list_record', KC_FIELDS)
CAMERA_FIELDS = {k: 'scalar' for k in ['x', 'y', 'zoom']}
ROOT_FIELDS = {'schema': 'scalar', 'camera': 'block', 'camera__fields': CAMERA_FIELDS, 'nodes': ('list_record', NODE_FIELDS), 'edges': ('list_record', EDGE_FIELDS), 'meta': 'block', 'meta__fields': META_FIELDS}


def fmt_row_cell(rec, key, shape):
    if key not in rec:
        return '_'
    v = rec[key]
    if shape == 'scalar' if isinstance(shape, str) else False:
        return fmt_scalar_value(v)
    if isinstance(shape, tuple) and shape[0] == 'list_record':
        subfields = shape[1]
        items = v[1]
        if not items:
            return '[ ]'
        inner = ' '.join(print_inline_record(item, subfields) for item in items)
        return f"[ {inner} ]"
    raise ValueError(f"unhandled cell shape {shape} for {key}")


def print_inline_record(rec, field_shapes):
    parts = []
    for key in rec['__order__']:
        v = rec[key]
        newkey = kebab(key)
        shape = field_shapes.get(key, 'scalar')
        if isinstance(shape, tuple) and shape[0] == 'list_record':
            items = v[1]
            inner = ' '.join(print_inline_record(item, shape[1]) for item in items)
            parts.append(f"{newkey}=[ {inner} ]")
        else:
            parts.append(f"{newkey}={fmt_scalar_value(v)}")
    return ' '.join(parts)


def used_columns(records, field_shapes):
    """Columns actually present (non-absent) in at least one record, in field declaration order."""
    order = list(field_shapes.keys())
    used = []
    for key in order:
        if any(key in r for r in records):
            used.append(key)
    return used


TYPE_TAG = {
    'id': 'TEXT', 'node_kind': 'TEXT', 'shape': 'TEXT', 'x': 'NUM', 'y': 'NUM', 'radius': 'NUM',
    'width': 'NUM', 'height': 'NUM', 'text': 'TEXT', 'icon_kind': 'TEXT', 'root': 'BOOL', 'scale': 'NUM',
    'visible': 'BOOL', 'locked': 'BOOL', 'handles': 'LIST', 'source': 'TEXT', 'target': 'TEXT',
    'edge_kind': 'TEXT', 'source_tip': 'TEXT', 'target_tip': 'TEXT', 'bidirectional': 'BOOL', 'specificity': 'ENUM',
}


def emit_table(field_name, records, field_shapes):
    if not records:
        return f"{kebab(field_name)} [] {{ }}"
    cols = used_columns(records, field_shapes)
    header = ' '.join(f"{kebab(c)}:{TYPE_TAG[c]}" for c in cols)
    rows = []
    for r in records:
        cells = [fmt_row_cell(r, c, field_shapes[c]) for c in cols]
        rows.append(' '.join(cells))
    body = '  '.join(rows)
    return f"{kebab(field_name)} [{header}] {{ {body} }}"


def convert(path_in, path_out):
    text = open(path_in).read()
    toks = tokenize(text)
    p = P(toks)
    root = parse_record(p, ROOT_FIELDS)
    assert p.peek()[0] == 'EOF', f"leftover tokens at {p.pos}: {p.toks[p.pos:p.pos+10]}"

    lines = []
    lines.append(f"schema={fmt_scalar_value(root['schema'])}")
    cam = root['camera'][1]
    cam_parts = ' '.join(f"{kebab(k)}={fmt_scalar_value(cam[k])}" for k in cam['__order__'])
    lines.append(f"camera {{ {cam_parts} }}")
    lines.append(emit_table('nodes', root['nodes'][1], NODE_FIELDS))
    lines.append(emit_table('edges', root['edges'][1], EDGE_FIELDS))
    meta = root['meta'][1]
    meta_parts = []
    if 'manifest_id' in meta:
        meta_parts.append(f"manifest-id={fmt_scalar_value(meta['manifest_id'])}")
    kc_records = meta['kind_compatibility'][1] if 'kind_compatibility' in meta else []
    meta_lines = []
    if meta_parts:
        meta_lines.append('  ' + ' '.join(meta_parts))
    meta_lines.append('  ' + emit_table('kind_compatibility', kc_records, KC_FIELDS))
    lines.append("meta {\n" + '\n'.join(meta_lines) + "\n}")
    out = '\n'.join(lines) + '\n'
    with open(path_out, 'w') as f:
        f.write(out)
    print(f"wrote {path_out} ({len(out)} bytes) — {len(root['nodes'][1])} nodes, {len(root['edges'][1])} edges, {len(kc_records)} kind-compat rows")


if __name__ == '__main__':
    base = '/Users/ueli/Documents/semio/puzzle/2d/example'
    for name in ['concrete-forest', 'nakagin-capsule-tower']:
        convert(f'{base}/{name}.puzzle2d', f'{base}/{name}.puzzle2d.new')
