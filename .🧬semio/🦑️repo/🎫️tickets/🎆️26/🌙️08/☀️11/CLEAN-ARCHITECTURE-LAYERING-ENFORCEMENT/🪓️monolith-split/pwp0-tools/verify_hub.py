# 🧭️ Verifies the WP0 mount hubs against the original plugin root: every mount resolves, to the same absolute file with the same module name and cfg attributes; regions balanced; rustfmt parses.
import collections, os, re, subprocess, sys
import hubsplit as h, rootunits as ru, crates as C
S = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
MOUNT = re.compile(r'((?:^[ \t]*#\[[^\n]*\]\n)*)^[ \t]*#\[\s*path\s*=\s*"([^"]+)"\s*\]\n((?:[ \t]*#\[[^\n]*\]\n)*)[ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', re.M)
INCLUDE = re.compile(r'((?:^[ \t]*#\[[^\n]*\]\n)*)^[ \t]*include!\("([^"]+)"\);', re.M)
def mounts(text, base):
    out = collections.Counter()
    for m in MOUNT.finditer(text):
        attrs = tuple(sorted(a.strip() for a in (m.group(1) + m.group(3)).split('\n') if a.strip() and not a.strip().startswith('#[path')))
        out[('mod', m.group(4), os.path.normpath(os.path.join(base, m.group(2))), attrs)] += 1
    for m in INCLUDE.finditer(text):
        out[('include', '', os.path.normpath(os.path.join(base, m.group(2))), tuple(sorted(a.strip() for a in m.group(1).split('\n') if a.strip())))] += 1
    return out
def regions(text):
    stack = []; ok = True
    for l in text.split('\n'):
        s = l.strip()
        if s.startswith('//#region'): stack.append(s[len('//#region'):].strip())
        elif s.startswith('//#endregion'):
            name = s[len('//#endregion'):].strip()
            if not stack or (name and stack[-1] != name): ok = False
            elif stack: stack.pop()
    return ok and not stack
def main():
    old = mounts(open(S + '/orig/plugin-root.rs', encoding='utf-8').read(), C.PLUG)
    files = [ru.ROOT_RS] + [h.HUBDIR + '/' + v[0] + '/🦀️.rs' for v in h.HUBS.values()]
    new = collections.Counter(); bad = 0
    for f in files:
        t = open(f, encoding='utf-8').read()
        ms = mounts(t, os.path.dirname(f)); new += ms
        if not regions(t): print('UNBALANCED regions', f); bad += 1
        for key in ms:
            if not os.path.isfile(key[2]) and not key[2].endswith('📥️mounts'): print('MISSING TARGET', key[2]); bad += 1
    inc = {k for k in new if k[0] == 'include' and C.MOUNTS_DIR in k[2]}
    for k in inc: del new[k]
    only_old = old - new; only_new = new - old
    expected_new = {k for k in only_new if k[0] == 'mod' and k[1].startswith('macros_wp') and '📣️macros' in k[2]}
    for k in expected_new: del only_new[k]
    print('expected new mounts (facade macro mounts, 12.5.6):', len(expected_new))
    print('mounts old', sum(old.values()), 'new', sum(new.values()), 'hub include lines (excluded)', len(inc))
    for k, v in only_old.items(): print('ONLY OLD', v, k[1], os.path.relpath(k[2], C.PLUG), k[3]); bad += 1
    for k, v in only_new.items(): print('ONLY NEW', v, k[1], os.path.relpath(k[2], C.PLUG), k[3]); bad += 1
    for f in files:
        r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--check', '--config', 'skip_children=true', f], capture_output=True, text=True)
        if r.returncode not in (0, 1): print('rustfmt PARSE ERROR', f, r.stderr[:200]); bad += 1
    print('hub verification', 'FAILED' if bad else 'PASSED', bad)
if __name__ == '__main__': main()
