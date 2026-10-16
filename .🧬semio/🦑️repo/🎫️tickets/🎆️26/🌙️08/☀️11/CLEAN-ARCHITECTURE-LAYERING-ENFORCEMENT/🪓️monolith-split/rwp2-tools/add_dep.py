"""➕️ Copies dependency lines (with their target tables) from the old top-crate manifest into a crate manifest, one directory deeper."""
import re, sys
from model import *
PKG = T + '/📦️packages/🦀️rust'
def sections(text):
    out = []; cur = None
    for i, l in enumerate(text.split('\n')):
        m = re.match(r'^\[(.+)\]\s*$', l)
        if m: cur = m.group(1); continue
        out.append((cur, l))
    return out
def lines_for(ident):
    names = {ident, ident.replace('_', '-')}
    found = []
    for sec, l in sections(open(PKG + '/Cargo.toml', encoding='utf8').read()):
        if sec and sec.endswith('dependencies') and not sec.startswith('dev') and 'build' not in sec:
            m = re.match(r'^([A-Za-z0-9_\-]+)\s*=', l)
            if m and m.group(1) in names:
                found.append((sec, re.sub(r'path = "((?:\.\./)+)', lambda mm: 'path = "../' + mm.group(1), l)))
    return found
def add(crate, idents):
    p = T + '/' + FOLDER[crate] + '/📦️packages/🦀️rust/Cargo.toml'
    t = open(p, encoding='utf8').read()
    for ident in idents:
        got = lines_for(ident); assert got, ident
        for sec, line in got:
            key = line.split('=')[0].strip()
            body_sec = re.search(r'(?ms)^\[' + re.escape(sec) + r'\]\n(.*?)(?=^\[|\Z)', t)
            if body_sec and re.search(r'(?m)^' + re.escape(key) + r'\s*=', body_sec.group(1)): continue
            if body_sec:
                end = body_sec.end(1)
                body = body_sec.group(1).rstrip('\n')
                t = t[:body_sec.start(1)] + body + '\n' + line + '\n\n' + t[end:].lstrip('\n') if t[end:].strip() else t[:body_sec.start(1)] + body + '\n' + line + '\n'
            else:
                marker = '[dev-dependencies]'
                new = '[%s]\n%s\n\n' % (sec, line)
                t = t.replace(marker, new + marker, 1) if marker in t else t.rstrip('\n') + '\n\n' + new
    open(p, 'w', encoding='utf8').write(t)
if __name__ == '__main__':
    add(sys.argv[1], sys.argv[2:])
