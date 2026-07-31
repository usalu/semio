#!/usr/bin/env python3
"""Fix broken TS/TSX relative imports across the emoji tree using structural translation
(strip leading ../, apply the word->emoji rule translator, recompute relative path)."""
import re, os, glob, sys
sys.path.insert(0, os.path.dirname(__file__))
from importlib import import_module
translate_mod = import_module('translate-path'.replace('-', '_')) if False else None
import importlib.util
spec = importlib.util.spec_from_file_location("translate_path", os.path.join(os.path.dirname(__file__), "translate-path.py"))
translate_path = importlib.util.module_from_spec(spec)
spec.loader.exec_module(translate_path)

pattern = re.compile(r'((?:from\s+|import\()\s*")(\.\.?/[^"]+)(")')

INDEX_ = {}
def build_index():
    from collections import defaultdict
    idx = defaultdict(list)
    for root, dirs, filenames in os.walk('.'):
        if any(skip in root for skip in ('/target', '/node_modules', '/.repo', '/.git', 'compose')):
            dirs[:] = []
            continue
        dirs[:] = [d for d in dirs if d not in ('target', 'node_modules', '.repo', '.git', 'compose')]
        for fn in filenames:
            idx[fn].append(os.path.normpath(os.path.join(root, fn)))
    return idx

def _rename_basename(base):
    if base in ('index.ts',):
        return '📦.ts'
    if base in ('index.tsx',):
        return '📦.tsx'
    if base.endswith('.js'):
        stem = base[: -len('.js')]
        return [stem + '.ts', stem + '.tsx']
    return None

def multi_segment_match(rel_base, n_segments=2):
    """Match the last N path segments against the index (disambiguates common basenames).
    Also tries the entry-renamed basename (index.ts -> 📦.ts) since the original no longer exists."""
    parts = [p for p in rel_base.split('/') if p not in ('..', '.')]
    if len(parts) < 1:
        return None
    base = parts[-1]
    basenames_to_try = [base]
    renamed = _rename_basename(base)
    if isinstance(renamed, list):
        basenames_to_try += renamed
    elif renamed:
        basenames_to_try.append(renamed)
    for try_base in basenames_to_try:
        candidates = INDEX_.get(try_base, [])
        if not candidates:
            continue
        if len(candidates) == 1:
            return candidates[0]
        suffix_parts = parts[-n_segments:-1] + [try_base] if len(parts) >= n_segments else parts[:-1] + [try_base]
        suffix = '/'.join(suffix_parts)
        matches = [c for c in candidates if c.endswith('/' + suffix) or c == suffix]
        if len(matches) == 1:
            return matches[0]
    return None

def resolves(path):
    if '/pkg/' in path or path.endswith('/pkg'):
        crate_dir = path.split('/pkg/')[0] if '/pkg/' in path else path
        return os.path.isdir(crate_dir)
    candidates = [path, path + '.ts', path + '.tsx', path + '.js',
                  path + '/index.ts', path + '/index.tsx', path + '.json',
                  path + '.wasm', path + '.css', path + '/📦.ts', path + '/📦.tsx']
    # TS "./foo.js" resolves "foo.ts"/"foo.tsx" convention
    if path.endswith('.js'):
        stem = path[: -len('.js')]
        candidates += [stem + '.ts', stem + '.tsx']
    return any(os.path.exists(c) for c in candidates)

def strip_ups(rel):
    n = 0
    parts = rel.split('/')
    while parts and parts[0] == '..':
        n += 1
        parts = parts[1:]
    return n, '/'.join(parts)

def process(f):
    try:
        content = open(f, encoding='utf-8').read()
    except Exception:
        return False, []
    state = {'changed': False}
    unresolved = []
    def repl(m):
        prefix, rel, suffix = m.group(1), m.group(2), m.group(3)
        if '${' in rel:
            return m.group(0)  # template literal, not a real static import
        query = ''
        rel_base = rel
        if '?' in rel:
            rel_base, query = rel.split('?', 1)
            query = '?' + query
        resolved = os.path.normpath(os.path.join(os.path.dirname(f), rel_base))
        if resolves(resolved):
            return m.group(0)
        # Try same-directory entry-rename first (index.ts/tsx -> 📦.ts/tsx), no translation needed.
        base = os.path.basename(rel_base)
        if base in ('index.ts', 'index.tsx'):
            renamed_rel = rel_base[: -len(base)] + ('📦.tsx' if base.endswith('.tsx') else '📦.ts')
            renamed_resolved = os.path.normpath(os.path.join(os.path.dirname(f), renamed_rel))
            if resolves(renamed_resolved):
                state['changed'] = True
                return prefix + renamed_rel + query + suffix
        _, suffix_path = strip_ups(rel_base)
        new_target = translate_path.translate(suffix_path)
        if new_target is not None and (os.path.exists(new_target) or resolves(new_target)):
            newrel = os.path.relpath(new_target, os.path.dirname(f))
            if not newrel.startswith('.'):
                newrel = './' + newrel
            state['changed'] = True
            return prefix + newrel + query + suffix
        # fallback: multi-segment basename match against a full-tree index
        match = multi_segment_match(rel_base, n_segments=2)
        if match is None:
            match = multi_segment_match(rel_base, n_segments=3)
        if match is not None:
            newrel = os.path.relpath(match, os.path.dirname(f))
            if not newrel.startswith('.'):
                newrel = './' + newrel
            state['changed'] = True
            return prefix + newrel + query + suffix
        reason = f'TRANSLATED_STILL_MISSING:{new_target}' if new_target else f'NO_RULE_MATCH:{suffix_path}'
        unresolved.append((f, rel, reason))
        return m.group(0)
    new_content = pattern.sub(repl, content)
    if state['changed']:
        open(f, 'w', encoding='utf-8').write(new_content)
    return state['changed'], unresolved

INDEX_ = build_index()

fixed = 0
all_unresolved = []
files = []
for root in ["🧰", "✏️", "🌎", "♻️"]:
    files += glob.glob(f'{root}/**/*.ts', recursive=True) + glob.glob(f'{root}/**/*.tsx', recursive=True)
files = [f for f in files if '/node_modules/' not in f and '/target/' not in f]

for f in files:
    changed, unresolved = process(f)
    if changed:
        fixed += 1
    all_unresolved.extend(unresolved)

print(f"files fixed: {fixed}")
print(f"unresolved: {len(all_unresolved)}")
for x in all_unresolved:
    print(x)
