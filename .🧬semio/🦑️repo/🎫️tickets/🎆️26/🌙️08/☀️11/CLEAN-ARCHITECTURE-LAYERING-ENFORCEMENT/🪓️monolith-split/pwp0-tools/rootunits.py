# 🧱️ Parses the mount units of the plugin crate root (PLUG/🦀️.rs): the `mod` items with their attributes, docs, trailing `pub use` lines and enclosing region.
import re, sys, os
R = '/Users/ueli/Documents/semio'
PLUG = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
ROOT_RS = PLUG + '/🦀️.rs'
ATTR = re.compile(r'^\s*(#\[|///|//!|//)')
PATHLIT = re.compile(r'(#\[\s*path\s*=\s*")([^"]+)"')
def units(text):
    lines = text.split('\n'); n = len(lines); out = []; i = 0
    depth = 0; region = []
    # top-level scan (brace depth 0)
    starts = []
    d = 0
    for ln in lines:
        starts.append(d)
        d += ln.count('{') - ln.count('}')
    i = 0
    while i < n:
        l = lines[i]
        if starts[i] == 0:
            s = l.strip()
            if s.startswith('//#region'): region.append((s[len('//#region'):].strip(), i)); i += 1; continue
            if s.startswith('//#endregion'): region.pop(); i += 1; continue
            m = re.match(r'^(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', l)
            if m:
                a = i
                while a > 0 and lines[a - 1].strip() and ATTR.match(lines[a - 1]) and not lines[a - 1].strip().startswith('//#'): a -= 1
                e = i
                name = m.group(1)
                while e + 1 < n:
                    nx = lines[e + 1]
                    if re.match(r'^pub use ' + re.escape(name) + r'::', nx):
                        e += 1
                        while not lines[e].rstrip().endswith(';'): e += 1
                    else: break
                body = '\n'.join(lines[a:e + 1])
                pm = PATHLIT.search(body)
                out.append(dict(name=name, start=a, end=e, region=region[-1][0] if region else None, path=pm.group(2) if pm else None, text=body, cfg=re.findall(r'#\[cfg\((.*?)\)\]', body)))
                i = e + 1; continue
        i += 1
    return out
if __name__ == '__main__':
    t = open(ROOT_RS, encoding='utf-8').read()
    for u in units(t): print(u['start'] + 1, u['end'] + 1, u['region'], u['name'], u['path'], u['cfg'])
