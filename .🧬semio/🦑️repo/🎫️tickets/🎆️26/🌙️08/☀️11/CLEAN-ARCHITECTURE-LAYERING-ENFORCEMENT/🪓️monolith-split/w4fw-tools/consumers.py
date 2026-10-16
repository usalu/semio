import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lex import mask
from imports import flatten
R = "/Users/ueli/Documents/semio/"
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/"
files = [p for p in open(S + 'rs-files.z', 'rb').read().split(b'\0') if p]
USE = re.compile(r'(?<![A-Za-z0-9_:])(?:pub\s+)?use\s+(?:::)?semio_framework(::[^;]+);', re.S)
INLINE = re.compile(r'(?<![A-Za-z0-9_:])(?:::)?semio_framework((?:::[A-Za-z_][A-Za-z0-9_]*)+)')
paths = collections.Counter()
who = collections.defaultdict(set)
for fb in files:
    f = fb.decode()
    if f.startswith('.🧬semio'): continue
    try: t = open(R + f, encoding='utf-8').read()
    except Exception: continue
    if 'semio_framework' not in t: continue
    if 'semio_framework::' not in t and 'semio_framework ' not in t: continue
    m = mask(t)
    for um in USE.finditer(m):
        tree = 'semio_framework' + um.group(1)
        for path, alias in flatten(tree.replace('\n', ' ')):
            if alias == '*': path = path + ['*']
            key = '::'.join(path)
            paths[key] += 1; who[key].add(f)
    for im in INLINE.finditer(m):
        key = 'semio_framework' + im.group(1)
        paths[key] += 1; who[key].add(f)
json.dump({k: [v, sorted(who[k])[:3]] for k, v in paths.items()}, open(S + 'consumer-paths.json', 'w'), ensure_ascii=False)
print(len(paths))
