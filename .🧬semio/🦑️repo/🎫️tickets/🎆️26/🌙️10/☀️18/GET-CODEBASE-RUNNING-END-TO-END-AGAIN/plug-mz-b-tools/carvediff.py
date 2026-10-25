import re, subprocess, sys, os
files = [l for l in subprocess.run(['rg','-l',r'🚪️io/[^"]*🔺️diff/🦀️.rs"','✏️s/🔌️plugins','-g','🦀️.rs'],capture_output=True,text=True).stdout.split('\n') if '🧬️mutations/📦️packages' in l]
for lib in files:
    t = open(lib, encoding='utf-8').read()
    crate_dir = os.path.dirname(lib)
    snap_dir = crate_dir.replace('🧬️mutations/📦️packages/🦀️rust', '📸️snapshot/📦️packages/🦀️rust')
    if not os.path.isdir(snap_dir): print('NOSNAP', lib[-90:]); continue
    cargo = open(os.path.join(snap_dir, 'Cargo.toml'), encoding='utf-8').read()
    name = re.search(r'name = "([^"]+)"', cargo).group(1).replace('-', '_')
    if f'pub use {name}::standards' not in t.replace('semio_s_artifact', 'semio_s_artifact') and name not in t: print('NOREF', name, lib[-90:]); continue
    mirror = os.path.join(snap_dir, '🪞️mirror/🦀️.rs')
    mtxt = open(mirror, encoding='utf-8').read() if os.path.exists(mirror) else open(os.path.join(snap_dir, '🦀️.rs'), encoding='utf-8').read()
    mpath = mirror if os.path.exists(mirror) else os.path.join(snap_dir, '🦀️.rs')
    changed = False
    for m in list(re.finditer(r'(?P<ind>[ \t]*)#\[path = "(?P<p>[^"]*🚪️io/(?P<kind>[^/]+)/🔺️diff/🦀️.rs)"\]\n[ \t]*pub mod diff;\n', t)):
        kind = m.group('kind'); ind = m.group('ind')
        # nest of enclosing modules
        before = t[:m.start()]
        stack = []
        for line in before.split('\n'):
            mm = re.match(r'([ \t]*)pub mod (\w+) \{', line)
            if mm:
                while stack and stack[-1][0] >= len(mm.group(1)): stack.pop()
                stack.append((len(mm.group(1)), mm.group(2)))
        nest = '::'.join(n for _, n in stack)
        # sibling snapshot mount in the snapshot mirror
        sm = re.search(r'(?P<ind>[ \t]*)#\[path = "(?P<p>[^"]*🚪️io/%s/📸️snapshot/🦀️.rs)"\]\n[ \t]*pub mod snapshot;\n' % re.escape(kind), mtxt)
        if not sm: print('NOSIBLING', kind, lib[-80:]); continue
        newmount = f'{sm.group("ind")}#[path = "{sm.group("p").replace("📸️snapshot", "🔺️diff")}"]\n{sm.group("ind")}pub mod diff;\n'
        if '🔺️diff/🦀️.rs"]' not in mtxt.split(sm.group(0))[1][:200] and newmount not in mtxt:
            mtxt = mtxt.replace(sm.group(0), sm.group(0) + newmount, 1)
        t = t.replace(m.group(0), f'{ind}pub use {name}::{nest}::diff;\n', 1)
        changed = True
        print('fixed', kind, name, nest)
    if changed:
        open(lib, 'w', encoding='utf-8').write(t)
        open(mpath, 'w', encoding='utf-8').write(mtxt)
