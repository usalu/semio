# 🧭️ Verifies the WP0 mount hubs against the original renderer root and library root: every mount resolves, to the same absolute file with the same cfg attributes, every region is balanced, rustfmt parses.
import collections, os, re, subprocess, sys
import hubsplit as h
S = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
R_DIR = os.path.dirname(h.ROOT_RS); LIB_DIR = os.path.dirname(h.LIB_RS)
MOUNT = re.compile(r'((?:^[ \t]*#\[[^\n]*\]\n)*)^[ \t]*#\[\s*path\s*=\s*"([^"]+)"\s*\]\n((?:[ \t]*#\[[^\n]*\]\n)*)[ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', re.M)
INCLUDE = re.compile(r'((?:^[ \t]*#\[[^\n]*\]\n)*)^[ \t]*include!\("([^"]+)"\);', re.M)
def mounts(text, base):
    out = collections.Counter(); names = {}
    for m in MOUNT.finditer(text):
        attrs = tuple(a.strip() for a in (m.group(1) + m.group(3)).split('\n') if a.strip() and not a.strip().startswith('#[path'))
        target = os.path.normpath(os.path.join(base, m.group(2)))
        out[('mod', m.group(4), target, tuple(sorted(attrs)))] += 1
    for m in INCLUDE.finditer(text):
        attrs = tuple(sorted(a.strip() for a in m.group(1).split('\n') if a.strip()))
        target = os.path.normpath(os.path.join(base, m.group(2)))
        out[('include', '', target, attrs)] += 1
    return out
def regions(text):
    stack = []; ok = True
    for i, l in enumerate(text.split('\n')):
        s = l.strip()
        if s.startswith('//#region'): stack.append(s[len('//#region'):].strip())
        elif s.startswith('//#endregion'):
            name = s[len('//#endregion'):].strip()
            if not stack or (name and stack[-1] != name): ok = False
            elif stack: stack.pop()
    return ok and not stack
def main():
    old = mounts(open(S + '/orig/renderer-root.rs', encoding='utf-8').read(), R_DIR) + mounts(open(S + '/orig/library-root.rs', encoding='utf-8').read(), LIB_DIR)
    new = collections.Counter()
    files = [h.ROOT_RS, h.LIB_RS] + [h.HUBDIR + '/' + v[0] + '/🦀️.rs' for v in h.HUBS.values()]
    bad = 0
    for f in files:
        t = open(f, encoding='utf-8').read()
        new += mounts(t, os.path.dirname(f))
        if not regions(t): print('UNBALANCED regions', f); bad += 1
        for key in mounts(t, os.path.dirname(f)):
            if not os.path.isfile(key[2]): print('MISSING TARGET', key[2]); bad += 1
    for f in files:
        for lit in re.findall(r'include_str!\("([^"]+)"\)', open(f, encoding='utf-8').read()):
            target = os.path.normpath(os.path.join(os.path.dirname(f), lit))
            if not os.path.isfile(target): print('MISSING include_str target', os.path.relpath(f, h.T), lit); bad += 1
    hub_includes = {k for k in new if k[0] == 'include' and '📥️mounts' in k[2]}
    for k in hub_includes: del new[k]
    only_old = old - new; only_new = new - old
    print('mounts old', sum(old.values()), 'new', sum(new.values()), 'hub include lines (excluded)', len(hub_includes))
    for k, v in only_old.items(): print('ONLY OLD', v, k[1], os.path.relpath(k[2], h.T), k[3]); bad += 1
    for k, v in only_new.items(): print('ONLY NEW', v, k[1], os.path.relpath(k[2], h.T), k[3]); bad += 1
    for f in files:
        r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--check', '--config', 'skip_children=true', f], capture_output=True, text=True)
        parse_error = 'error:' in r.stderr and 'Diff in' not in r.stdout and 'error: ' in r.stderr and 'could not parse' in r.stderr.lower()
        print('rustfmt', os.path.relpath(f, h.T), 'exit', r.returncode, ('PARSE ERROR ' + r.stderr[:200]) if r.returncode not in (0, 1) else '')
        if r.returncode not in (0, 1): bad += 1
    print('hub verification', 'FAILED' if bad else 'PASSED', bad)
if __name__ == '__main__': main()
