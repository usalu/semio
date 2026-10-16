"""🧰 Shared helpers for the os-kernel-db carve: paths, use-tree parsing, item tables.

Analysis only; nothing here writes into the repository.
"""
import os
import re

from outline import outline
from rs import strip, idents

REPO = '/Users/ueli/Documents/semio'
DB_REL = '🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db'
DB_REPO = f'{REPO}/{DB_REL}'
SP = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osdb'
PRISTINE = f'{SP}/pristine/db'
STAGE_ROOT = f'{SP}/stage'
STAGE = f'{STAGE_ROOT}/{DB_REL}'
TOOLS = f'{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/osdb-tools'

MODULE_DIRS = {
    'ids': '🆔️ids', 'durability': '💾️durability', 'policy': '🎚️policy', 'observe': '👁️observe', 'actor': '🎭️actor',
    'security': '🔒️security', 'state': '🔘️state', 'conflict': '⚔️conflict', 'preview': '🔮️preview',
    'projection': '📽️projection', 'cluster': '🌐️cluster', 'snapshot': '📸️snapshot', 'index': '🔢️index',
    'query': '🔍️query', 'compact': '🗜️compact', 'wal': '📝️wal', 'sync': '🔄️sync', 'cli': '⌨️cli',
    'storage': '🗄️storage', 'artifact': '🗿️artifact', 'engine': '⚙️engine',
}
MODULE_IDENT = {
    'ids': 'db_ids', 'durability': 'db_durability', 'policy': 'db_policy', 'observe': 'db_observe', 'actor': 'db_actor',
    'security': 'db_security', 'state': 'db_state', 'conflict': 'db_conflict', 'preview': 'db_preview',
    'projection': 'db_projection', 'cluster': 'db_cluster', 'snapshot': 'db_snapshot', 'index': 'db_index',
    'query': 'db_query', 'compact': 'db_compact', 'wal': 'db_wal', 'sync': 'db_sync', 'cli': 'db_cli',
    'storage': 'db_storage', 'artifact': 'db_artifact', 'engine': 'db_engine',
}
IDENT_MODULE = {v: k for k, v in MODULE_IDENT.items()}


def read(path):
    with open(path, encoding='utf-8') as f:
        return f.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w', encoding='utf-8') as f:
        f.write(text)


TOKEN = re.compile(r'::|[{},*;]|[A-Za-z_][A-Za-z0-9_]*')


def parse_use(tree):
    """Parse a use tree (text between `use` and `;`) into leaves: (path tuple, alias|None, is_glob)."""
    toks = TOKEN.findall(tree)
    pos = [0]

    def parse_tree(prefix):
        out = []
        path = list(prefix)
        while pos[0] < len(toks):
            t = toks[pos[0]]
            if t == '::':
                pos[0] += 1
                continue
            if t == '{':
                pos[0] += 1
                while True:
                    out.extend(parse_tree(tuple(path)))
                    if pos[0] < len(toks) and toks[pos[0]] == ',':
                        pos[0] += 1
                        if pos[0] < len(toks) and toks[pos[0]] == '}':
                            pos[0] += 1
                            return out
                        continue
                    if pos[0] < len(toks) and toks[pos[0]] == '}':
                        pos[0] += 1
                        return out
                    return out
            if t == '*':
                pos[0] += 1
                out.append((tuple(path), None, True))
                return out
            if t in (',', '}', ';'):
                out.append((tuple(path), None, False)) if len(path) > len(prefix) or not prefix else None
                return out
            if t == 'as':
                pos[0] += 1
                alias = toks[pos[0]]
                pos[0] += 1
                out.append((tuple(path), alias, False))
                return out
            path.append(t)
            pos[0] += 1
            if pos[0] >= len(toks) or toks[pos[0]] in (',', '}', ';'):
                out.append((tuple(path), None, False))
                return out
        if len(path) > len(prefix):
            out.append((tuple(path), None, False))
        return out

    return parse_tree(())


def top_uses(path, text=None):
    """Top-level `use` items of a file: [(first_line, last_line, attrs[], vis, tree_text)]."""
    text = text if text is not None else read(path)
    lines = text.split('\n')
    out = []
    for kind, name, s, e, h, vis in outline_text(text):
        if kind != 'use':
            continue
        decl = '\n'.join(lines[h - 1:e])
        m = re.match(r'(pub(?:\([^)]*\))?\s+)?use\s+(.*?);\s*$', decl, re.S)
        if not m:
            continue
        attrs = [l for l in lines[s - 1:h - 1] if l.strip().startswith('#[')]
        out.append((s, e, h, attrs, (m.group(1) or '').strip(), m.group(2)))
    return out


def outline_text(text):
    import tempfile
    with tempfile.NamedTemporaryFile('w', suffix='.rs', delete=False, encoding='utf-8') as f:
        f.write(text)
        p = f.name
    try:
        return outline(p)
    finally:
        os.unlink(p)


def collect_files(root):
    """Files reachable from a lib root through #[path = "..."] mounts (and the root itself)."""
    seen = [root]
    todo = [root]
    while todo:
        f = todo.pop()
        t = read(f)
        for m in list(re.finditer(r'#\[path = "([^"]+)"\]', t)) + list(re.finditer(r'(?<![A-Za-z0-9_])include!\(\s*"([^"]+)"\s*\)', t)):
            p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            if os.path.exists(p) and p not in seen and p.endswith('.rs'):
                seen.append(p)
                todo.append(p)
    return seen
