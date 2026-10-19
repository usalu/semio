"""📖 C6UI emulation of readModuleTreeSource (module-tree/🟦️.ts): parent text first, then each `#[path]` / `include!` child in declaration order, depth-first, tests excluded. usage: treetext.py <repo-rel-root-file> -> prints the concatenated text to stdout; import `tree(path)` from python."""
import os, re, sys
R = '/Users/ueli/Documents/semio/'
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;')
INC = re.compile(r'(?<![\w!])include!\(\s*"([^"]+)"\s*\)\s*;')


def tree(path, seen=None):
    seen = seen if seen is not None else set()
    path = os.path.normpath(path)
    if path in seen or not os.path.isfile(path):
        return []
    seen.add(path)
    text = open(path, encoding='utf-8').read()
    out = [(path, text)]
    kids = []
    for m in re.finditer(r'(#\[\s*path\s*=\s*"([^"]+)"\s*\](?:\s*#\[[^\]]*\])*\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;)|((?<![\w!])include!\(\s*"([^"]+)"\s*\)\s*;)', text):
        rel = m.group(2) or m.group(4)
        kids.append((m.start(), rel))
    for _, rel in sorted(kids):
        if '🧪️tests' in rel.split('/'):
            continue
        out.extend(tree(os.path.join(os.path.dirname(path), rel), seen))
    return out


def text(path):
    return ''.join(t if t.endswith('\n') else t + '\n' for _, t in tree(path))


if __name__ == '__main__':
    sys.stdout.write(text(os.path.join(R, sys.argv[1])))
