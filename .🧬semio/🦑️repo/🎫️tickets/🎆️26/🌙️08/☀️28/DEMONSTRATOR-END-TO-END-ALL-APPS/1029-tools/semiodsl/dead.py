import re, sys, pathlib, collections

log = sys.argv[1]
lines = open(log, encoding='utf-8', errors='replace').read().split('\n')
targets = collections.defaultdict(set)
for i, l in enumerate(lines):
    m = re.match(r'warning: function `(\w+)` is never used', l)
    if not m:
        continue
    for j in range(i + 1, min(i + 4, len(lines))):
        mm = re.match(r'\s*--> (.*?):(\d+):(\d+)', lines[j])
        if mm:
            if '💾️binary/📸️snapshot' in mm.group(1):
                targets[pathlib.Path(mm.group(1))].add(m.group(1))
            break

root = pathlib.Path('/Users/ueli/Documents/semio')
for rel, names in targets.items():
    cands = [p for p in [pathlib.Path(rel)] if p.is_absolute()]
    # relative path in log is relative to the cargo workspace root of the semio artifact
    base = root / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio'
    path = (base / rel).resolve() if not rel.is_absolute() else rel
    if not path.exists():
        print('MISSING', rel); continue
    src = path.read_text(encoding='utf-8').split('\n')
    removed = 0
    for name in sorted(names):
        for i, l in enumerate(src):
            if re.match(r'^\s*(pub(\(crate\))? )?fn %s\b' % name, l):
                depth = 0; started = False; j = i
                while True:
                    depth += src[j].count('{') - src[j].count('}')
                    if '{' in src[j]: started = True
                    if started and depth == 0: break
                    j += 1
                k = i
                while k > 0 and re.match(r'^\s*(///|//|#\[)', src[k - 1]):
                    k -= 1
                del src[k:j + 1]
                removed += 1
                break
    text = '\n'.join(src)
    text = re.sub(r'\n{3,}', '\n\n', text)
    path.write_text(text, encoding='utf-8')
    print(str(path).split('subsets/')[-1], removed, '/', len(names))
