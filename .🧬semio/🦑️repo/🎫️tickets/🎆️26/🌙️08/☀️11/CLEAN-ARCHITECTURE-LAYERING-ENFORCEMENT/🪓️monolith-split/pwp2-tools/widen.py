#!/usr/bin/env python3
# 🔓️ Widening pass of PLUGIN-WP2: every `pub(crate)` / `pub(super)` item, method and field of a carved file whose name is mentioned by a file outside its crate becomes `pub` (narrowest change a cross-crate use demands, name based).
# usage: widen.py   (reads and rewrites OUT in place; needs WS/in and WS/out from build.py; writes WS/widen.log)
import re, os, sys, collections, json
from common import *
import model as M
import modtree
from rs import blank

PLUG_IN = WS + '/in/' + '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
VIEW = WS + '/view/' + '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
ITEM = re.compile(r'(?m)^(\s*)pub\((?:crate|super)\)(\s+(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(?:fn|struct|enum|const|static|type|trait|mod|union)\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*))')
FIELD = re.compile(r'(?m)^(\s+)pub\((?:crate|super)\)(\s+([a-z_][A-Za-z0-9_]*)\s*:)')
USEX = re.compile(r'(?m)^(\s*)pub\((?:crate|super)\)(\s+use\s+[^;]*;)')
TOK = re.compile(r'(?<![A-Za-z0-9_$])[A-Za-z_][A-Za-z0-9_]*')

def make_view():
    import shutil, subprocess
    shutil.rmtree(WS + '/view', ignore_errors=True)
    subprocess.check_call(['cp', '-R', WS + '/in', WS + '/view'])
    subprocess.check_call(['rsync', '-a', WS + '/out/', VIEW + '/'])

def crate_files(key):
    return {f for f in modtree.tree(VIEW + '/' + M.ROOT[key])}

def main():
    make_view()
    own = {k: crate_files(k) for k in M.WP2}
    # lift files written for foreign destination crates belong to that crate, not to a WP2 crate
    allf = []
    for dp, dn, fn in os.walk(VIEW):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target')]
        for f in fn:
            if f.endswith('.rs'):
                allf.append(os.path.join(dp, f))
    toks = {}
    for f in allf:
        toks[f] = set(TOK.findall(blank(read(f))))
    log = []
    total = 0
    for key in M.WP2:
        ext = set()
        owned = own[key]
        for f, ts in toks.items():
            if f not in owned:
                ext |= ts
        for f in sorted(owned):
            rel = os.path.relpath(f, VIEW)
            if not os.path.exists(WS + '/out/' + rel):
                continue
            text = read(f)
            n = 0
            def item(m):
                nonlocal n
                if m.group(3) in ext:
                    n += 1
                    return m.group(1) + 'pub' + m.group(2)
                return m.group(0)
            def field(m):
                nonlocal n
                if m.group(3) in ext:
                    n += 1
                    return m.group(1) + 'pub' + m.group(2)
                return m.group(0)
            def use(m):
                nonlocal n
                names = set(TOK.findall(blank(m.group(2)))) - {'use', 'as', 'self', 'super', 'crate'}
                if names & ext:
                    n += 1
                    return m.group(1) + 'pub' + m.group(2)
                return m.group(0)
            for name in M.WIDEN_EXTRA.get(rel, []):
                text, k = re.subn(r'(?m)^(\s*)((?:async |const |unsafe )*(?:fn|struct|enum|const|static|type|trait) )(' + re.escape(name) + r')\b', lambda m: m.group(1) + 'pub ' + m.group(2) + m.group(3), text)
                k2 = 0
                def inblock(m):
                    nonlocal k2
                    body, c = re.subn(r'(?m)^(\s+)(' + re.escape(name) + r')(\s*:)', lambda mm: mm.group(1) + 'pub ' + mm.group(2) + mm.group(3), m.group(0))
                    k2 += c
                    return body
                text = re.sub(r'(?m)^(?:pub(?:\([^)]*\))?\s+)?struct\s+\w+[^{;]*\{(?:[^{}]|\{[^{}]*\})*\}', inblock, text)
                n += k + k2
            for f0, name in M.UNGATE:
                if f0 == rel:
                    pat = re.compile(r'(?m)^([ \t]*)#\[cfg\(test\)\]\n((?:[ \t]*(?:#\[[^\n]*\]|///[^\n]*)\n)*)([ \t]*)(?:pub\((?:crate|super)\)[ \t]+|pub[ \t]+)?((?:async |const |unsafe )*(?:fn|struct|enum|const|static) +)(' + re.escape(name) + r')\b')
                    text, k = pat.subn(lambda m: m.group(1) + '#[doc(hidden)]\n' + m.group(2) + m.group(3) + 'pub ' + m.group(4) + m.group(5), text)
                    assert k >= 1, ('ungate', rel, name)
                    n += k
            new = ITEM.sub(item, text)
            new = FIELD.sub(field, new)
            new = USEX.sub(use, new)
            if new != text:
                write(WS + '/out/' + rel, new)
                log.append(f'{key} {rel} widened {n}')
                total += n
    write(WS + '/widen.log', '\n'.join(log))
    print('widened', total, 'definitions in', len(log), 'files')

if __name__ == '__main__':
    main()
