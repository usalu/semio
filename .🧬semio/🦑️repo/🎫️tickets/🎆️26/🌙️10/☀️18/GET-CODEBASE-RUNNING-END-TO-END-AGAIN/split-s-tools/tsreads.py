"""📖 c7w TS reader repoint: tsreads.py <file> <old> <new>... replaces each exact `old` (must occur) by `new`; when `new` uses readModuleTreeSource and the file lacks the import it adds one with the relative path to the module-tree reader."""
import sys, os
REPO = '/Users/ueli/Documents/semio'
MT = REPO + '/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts'
def run(path, pairs):
    p = path if path.startswith('/') else os.path.join(REPO, path)
    t = open(p, encoding='utf-8').read()
    for old, new in pairs:
        n = t.count(old)
        assert n >= 1, ('missing', old[:100])
        t = t.replace(old, new)
    uses = [fn for fn in ('readModuleTreeSource', 'sliceModuleTreeSource') if fn in t]
    if uses and 'module-tree/🟦️.ts"' not in t:
        rel = os.path.relpath(MT, os.path.dirname(p))
        lines = t.split('\n')
        last = max((i for i, l in enumerate(lines) if l.startswith('import ')), default=-1)
        lines.insert(last + 1, 'import { readModuleTreeSource } from "%s";' % rel)
        t = '\n'.join(lines)
    open(p, 'w', encoding='utf-8').write(t)
if __name__ == '__main__':
    a = sys.argv[1:]
    run(a[0], list(zip(a[1::2], a[2::2])))
    print('ok', a[0][-80:])
