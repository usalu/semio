"""🚚️ Wave Z depth: one-shot hand-migration of every committed JSON document of the BIM plugin to the new records.

Opening.sill (added to the window type sill) -> Opening.sill_override (replaces it); Stair and Railing gain their construction
fields with defaults that keep the old geometry; the patches and payloads that named `sill` now name `sill_override`.
Run from the repo root: python r7-z-depth-migrate.py [--dry]. Idempotent: records that already carry the new fields are left alone.
"""
import json, os, re, sys

ROOT = os.path.join('✏️s', '🔌️plugins', '🏙️bim')
SKIP = {'target', 'node_modules', '🗑️generated', '.git', 'dist'}
sys.stdout.reconfigure(encoding="utf-8")
DRY = '--dry' in sys.argv


def load(path):
    text = open(path, encoding='utf-8', newline='').read()
    return json.loads(text, parse_float=lambda x: '@@' + x + '@@'), '\r\n' in text


def dump(doc, crlf):
    text = json.dumps(doc, indent=2, ensure_ascii=False)
    text = re.sub(r'"@@(.*?)@@"', r'\1', text) + '\n'
    return text.replace('\n', '\r\n') if crlf else text


def num(x):
    return float(x[2:-2]) if isinstance(x, str) and x.startswith('@@') else x


def keep(x, value):
    return value if value is not None else x


def fmt(value):
    text = repr(round(value, 9))
    return '@@' + (text[:-2] if text.endswith('.0') else text) + '@@'


def is_opening(node):
    return isinstance(node, dict) and 'flip_hand' in node and 'flip_facing' in node and 'host' in node and 'kind' in node


def is_stair(node):
    return isinstance(node, dict) and {'storey', 'start', 'direction', 'width', 'flight', 'top', 'max_riser', 'min_tread'} <= node.keys()


def is_railing(node):
    return isinstance(node, dict) and {'storey', 'path', 'height', 'post_spacing', 'material', 'base_offset'} <= node.keys()


def type_sill(window_types, kind):
    if isinstance(kind, dict) and 'Window' in kind:
        row = window_types.get(kind['Window']['window_type'])
        if row is not None:
            return num(row['sill'])
    return 0.0


def override_of(old_sill, window_types, kind):
    old = num(old_sill)
    if old == 0:
        return None
    base = type_sill(window_types, kind) if isinstance(kind, dict) and 'Window' in kind else 0.0
    return fmt(base + old)


def insert_after(node, key, items):
    out = {}
    for k, v in node.items():
        out[k] = v
        if k == key:
            out.update(items)
    return out


class Ctx:
    def __init__(self, window_types, openings):
        self.window_types = window_types
        self.openings = openings


def find_key(node, key):
    if isinstance(node, dict):
        if key in node and isinstance(node[key], dict):
            return node[key]
        for v in node.values():
            found = find_key(v, key)
            if found is not None:
                return found
    elif isinstance(node, list):
        for v in node:
            found = find_key(v, key)
            if found is not None:
                return found
    return None


def migrate_opening(node, ctx):
    if 'sill' not in node:
        return node
    override = override_of(node['sill'], ctx.window_types, node['kind'])
    out = {}
    for k, v in node.items():
        if k == 'sill':
            if override is not None:
                out['sill_override'] = override
        else:
            out[k] = v
    return out


def migrate_stair(node):
    if 'stringer' in node:
        return node
    return insert_after(node, 'min_tread', {
        'stringer': {'kind': 'None', 'width': '@@0.05@@', 'depth': '@@0.25@@'},
        'nosing': 0,
        'tread_thickness': '@@0.04@@',
        'riser': 'Closed',
        'landing_depth': node['width'],
    })


def migrate_railing(node):
    if 'profile' in node:
        return node
    return insert_after(node, 'post_spacing', {
        'profile': {'Rectangle': {'width': '@@0.06@@', 'depth': '@@0.04@@'}},
        'post_profile': {'Rectangle': {'width': '@@0.05@@', 'depth': '@@0.05@@'}},
        'infill': 'None',
    })


def walk(node, ctx, key=None, parent_key=None):
    if isinstance(node, list):
        return [walk(v, ctx, key) for v in node]
    if not isinstance(node, dict):
        return node
    if is_opening(node):
        node = migrate_opening(node, ctx)
    elif is_stair(node):
        node = migrate_stair(node)
    elif is_railing(node):
        node = migrate_railing(node)
    elif node.get('mutation') == 'setOpening' and 'sill' in node:
        opening = ctx.openings.get(node.get('id'))
        kind = opening['kind'] if opening else {}
        raw = num(node['sill'])
        override = fmt(type_sill(ctx.window_types, kind) + raw) if raw >= 0 and opening else node['sill']
        node = {('sill_override' if k == 'sill' else k): ({'value': override} if k == 'sill' else v) for k, v in node.items()}
    elif parent_key == 'openings' and node.get('entry') == 'Patched' and 'sill' in node:
        opening = ctx.openings.get(key)
        kind = opening['kind'] if opening else {}
        override = fmt(type_sill(ctx.window_types, kind) + num(node['sill']))
        node = {('sill_override' if k == 'sill' else k): ({'value': override} if k == 'sill' else v) for k, v in node.items()}
    return {k: walk(v, ctx, k, key) for k, v in node.items()}


def tree(root):
    for base, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in SKIP]
        for name in files:
            if name.endswith('.json'):
                yield os.path.join(base, name)


def case_context(path):
    parts = path.replace('\\', '/').split('/')
    if '🧬️mutations' in parts and '🧫️fixtures' in parts:
        index = parts.index('🧬️mutations')
        case = '/'.join(parts[:index + 3])
        windows, openings = {}, {}
        for name in ('⬅️before', '➡️after'):
            snapshot = os.path.join(case, '📸️snapshot', name, '🔣️.json')
            if os.path.exists(snapshot):
                doc, _ = load(snapshot)
                windows.update(doc.get('window_types', {}))
                if name == '⬅️before':
                    openings.update(doc.get('openings', {}))
        return Ctx(windows, openings)
    return None


def context_of(doc, path):
    case = case_context(path)
    if case is not None:
        return case
    windows = find_key(doc, 'window_types') or {}
    openings = find_key(doc, 'openings') or {}
    return Ctx(windows, openings)


def main():
    changed = 0
    for path in tree(ROOT):
        doc, crlf = load(path)
        if isinstance(doc, dict) and '$schema' in doc:
            continue
        before = json.dumps(doc, sort_keys=False)
        ctx = context_of(doc, path)
        out = walk(doc, ctx)
        if json.dumps(out, sort_keys=False) != before:
            changed += 1
            print(('would write ' if DRY else 'wrote ') + path)
            if not DRY:
                open(path, 'w', encoding='utf-8', newline='').write(dump(out, crlf))
    print('files changed:', changed)


main()
