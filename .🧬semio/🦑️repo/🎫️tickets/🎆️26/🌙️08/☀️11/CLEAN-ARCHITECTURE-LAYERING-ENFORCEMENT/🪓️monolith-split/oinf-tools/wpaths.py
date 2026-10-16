import os, re, sys
W = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world"
bad = []; n = 0; longest = 0
for dp, dn, fn in os.walk(W):
    if 'node_modules' in dp: continue
    for f in fn:
        p = os.path.join(dp, f)
        longest = max(longest, len(p.encode()))
        if not f.endswith('.rs'): continue
        t = open(p, encoding='utf-8').read()
        for m in re.finditer(r'(?:include_(?:str|bytes)!|include!)\(\s*"([^"]+)"', t):
            n += 1
            tgt = os.path.normpath(os.path.join(dp, m.group(1)))
            if not os.path.exists(tgt): bad.append((p[len(W):], m.group(1), tgt[-60:]))
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]', t):
            n += 1
            tgt = os.path.normpath(os.path.join(dp, m.group(1)))
            if not os.path.exists(tgt): bad.append((p[len(W):], m.group(1), tgt[-60:]))
print('literals checked', n, 'missing', len(bad), 'longest path bytes (abs)', longest)
for b in bad[:20]: print(b)
