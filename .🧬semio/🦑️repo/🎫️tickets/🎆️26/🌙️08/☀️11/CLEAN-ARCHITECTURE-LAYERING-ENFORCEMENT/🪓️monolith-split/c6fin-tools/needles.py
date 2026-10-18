"""🔎 Module-tree needle check (the bun module-tree helper cannot load here: workspace links missing). Reads the root and every file reachable through #[path] and include!("literal") mounts, parent first, and checks needles (leading ! = must be absent). usage: needles.py <root.rs> <needle>..."""
import os, re, sys
PATH = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
INC = re.compile(r'include!\(\s*"([^"]+)"\s*\)')
def tree(root):
    seen, order = set(), []
    def visit(f):
        if f in seen or not os.path.isfile(f):
            return
        seen.add(f); order.append(f)
        t = open(f, encoding='utf-8').read()
        for m in PATH.finditer(t):
            if m.group(1).endswith('.rs'):
                visit(os.path.normpath(os.path.join(os.path.dirname(f), m.group(1))))
        for m in INC.finditer(t):
            visit(os.path.normpath(os.path.join(os.path.dirname(f), m.group(1))))
    visit(root)
    return order
if __name__ == '__main__':
    root, needles = sys.argv[1], sys.argv[2:]
    files = tree(root)
    text = ''.join(open(f, encoding='utf-8').read() + '\n' for f in files)
    bad = 0
    for n in needles:
        neg = n.startswith('!')
        needle = n[1:] if neg else n
        if (needle in text) == neg:
            bad += 1
            print('FAIL', 'present' if neg else 'missing', repr(needle))
    print('files=%d needles=%d bad=%d' % (len(files), len(needles), bad))
