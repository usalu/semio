import os, re, sys
PREFIX = ["semio_", "serde", "protocol::", "store::", "dsl::", "pack::", "value_derive::", "framework_schema::", "tokio"]
USE = re.compile(r'\buse\s+((?:' + '|'.join(re.escape(p) for p in PREFIX) + r')[^;]*\*[^;]*);')
LEAF = re.compile(r'(?:[A-Za-z0-9_]+::)*\*')
def fix(m):
    body = m.group(1)
    if '{' not in body:
        return m.group(0)
    new = LEAF.sub('', body)
    new = re.sub(r',\s*,', ',', new)
    new = re.sub(r'\{\s*,', '{', new)
    new = re.sub(r',\s*\}', '}', new)
    return 'use ' + new + ';'
n = 0
for root in sys.argv[1:]:
    for d, _, fs in os.walk(root):
        for f in fs:
            if not f.endswith('.rs'): continue
            p = os.path.join(d, f)
            t = open(p, encoding='utf8').read()
            u = USE.sub(fix, t)
            if u != t:
                open(p, 'w', encoding='utf8').write(u); n += 1
print('patched', n)
