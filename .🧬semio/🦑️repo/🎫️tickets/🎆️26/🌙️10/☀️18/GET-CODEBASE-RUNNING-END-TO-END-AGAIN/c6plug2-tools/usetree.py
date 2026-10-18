"""📚 C6PLUG use-tree parser: expands one `use` item into bindings (bound name, full path, is_glob)."""
import os, sys, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex


def parse_use_item(toks):
    ts = [t for t in toks if t[0] != 'comment']
    i = 0
    while i < len(ts) and ts[i][1] != 'use':
        i += 1
    i += 1
    out = []

    def tree(i, prefix):
        path = list(prefix)
        if ts[i][1] == '{':
            i += 1
            while ts[i][1] != '}':
                i = tree(i, path)
                if ts[i][1] == ',':
                    i += 1
            return i + 1
        if ts[i][1] == '*':
            out.append(('*', '::'.join(path), True))
            return i + 1
        while True:
            if ts[i][1] == ':' and ts[i + 1][1] == ':':
                i += 2
                continue
            if ts[i][1] == '{':
                return tree(i, path)
            if ts[i][1] == '*':
                out.append(('*', '::'.join(path), True))
                return i + 1
            name = ts[i][1]
            i += 1
            if ts[i][1] == ':' and ts[i + 1][1] == ':':
                path.append(name)
                i += 2
                if ts[i][1] in ('{', '*'):
                    return tree(i, path)
                continue
            if name == 'self':
                bound = path[-1]
                full = '::'.join(path)
            else:
                bound = name
                full = '::'.join(path + [name])
            if ts[i][1] == 'as':
                bound = ts[i + 1][1]
                i += 2
            out.append((bound, full, False))
            return i
    tree(i, [])
    return out


def format_use(entries, vis=''):
    """entries: list of (bound, full). Groups by parent path into one `use` item each."""
    groups = {}
    for bound, full in entries:
        parts = full.split('::')
        leaf = parts[-1]
        parent = '::'.join(parts[:-1])
        item = leaf if bound == leaf else '%s as %s' % (leaf, bound)
        if leaf == 'self':
            item = 'self'
        groups.setdefault(parent, []).append(item)
    lines = []
    for parent in sorted(groups):
        items = sorted(set(groups[parent]))
        pre = (vis + ' ') if vis else ''
        if len(items) == 1 and not parent:
            lines.append('%suse %s;' % (pre, items[0]))
        elif len(items) == 1:
            it = items[0]
            lines.append('%suse %s::%s;' % (pre, parent, it))
        else:
            lines.append('%suse %s::{%s};' % (pre, parent, ', '.join(items)))
    return lines
