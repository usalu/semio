#!/usr/bin/env python3
"""Makes the private `use <child>::…;` imports of a host file absolute (`use crate::…::host::<child>::…;`), so that lifting a child out of the host carries a valid import. usage: abschild.py <host.rs>..."""
import re, sys, os
sys.path.insert(0, os.path.dirname(__file__))
src = open(os.path.join(os.path.dirname(__file__), 'fixsuper.py'), encoding='utf-8').read().rsplit('\nmain()', 1)[0]
exec(src)
for host in sys.argv[1:]:
    t = read(host)
    mods = set(re.findall(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*(?:;|\{)', t, re.M))
    mp = '::'.join(modpath(host))
    n = 0
    def fix(m):
        global n
        if m.group(1) in mods:
            n += 1
            return m.group(0).replace('use ' + m.group(1) + '::', 'use ' + mp + '::' + m.group(1) + '::', 1)
        return m.group(0)
    t2 = re.sub(r'^(?!pub)(?:#\[[^\n]*\]\s*)*use\s+(\w+)::', fix, t, flags=re.M)
    if t2 != t:
        open(host, 'w', encoding='utf-8').write(t2)
    print(os.path.basename(os.path.dirname(host)), n)
