import os, sys, json, hashlib, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
TAX = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json'
ROW = '    "crate-module-children": {'
TAGS = ['bim', 'process', 'raster', 'vdi', 'en1999', 'en1993']
SKIP = {'♻️retirement'}


def wanted():
    out = set()
    for t in TAGS:
        sp = importlib.import_module('specs.' + t).SPEC
        for c in sp['children']:
            if c['folder'] not in SKIP:
                out.add(c['folder'])
    return out


def patch(text, names):
    lines = text.split('\n')
    starts = [i for i, l in enumerate(lines) if l == ROW]
    assert len(starts) == 1, starts
    s = starts[0]
    a = next(i for i in range(s, s + 20) if lines[i] == '      "memberNames": [')
    b = next(i for i in range(a, a + 400) if lines[i] == '      ],')
    body = lines[a + 1:b]
    existing = [json.loads(l.strip().rstrip(',')) for l in body]
    assert existing == sorted(existing)
    add = sorted(n for n in names if n not in existing)
    merged = sorted(existing + add)
    new_body = ['        ' + json.dumps(n, ensure_ascii=False) + (',' if i < len(merged) - 1 else '') for i, n in enumerate(merged)]
    return '\n'.join(lines[:a + 1] + new_body + lines[b:]), add


if __name__ == '__main__':
    mode = sys.argv[1]
    raw = open(TAX, 'rb').read()
    text = raw.decode('utf-8')
    new, add = patch(text, wanted())
    old_j = json.loads(text)
    new_j = json.loads(new)
    row = new_j['semanticDirectoryMemberKinds']['crate-module-children']['memberNames']
    for n in add:
        row.remove(n)
    assert new_j == old_j, 'patch changed more than the new names'
    print('adds', len(add))
    if mode == 'out':
        open(sys.argv[2], 'w', encoding='utf-8').write(new)
    elif mode == 'apply':
        again = open(TAX, 'rb').read()
        assert hashlib.sha256(again).hexdigest() == hashlib.sha256(raw).hexdigest(), 'taxonomy changed meanwhile; rerun'
        tmp = TAX + '.pfa-tmp'
        open(tmp, 'w', encoding='utf-8').write(new)
        os.replace(tmp, TAX)
        print('applied')
