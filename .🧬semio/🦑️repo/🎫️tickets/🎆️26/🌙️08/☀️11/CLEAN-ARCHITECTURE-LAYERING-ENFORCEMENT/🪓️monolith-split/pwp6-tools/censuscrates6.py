import sys; sys.path.insert(0, '.')
from lib6 import *
import s0plan as SP
CRATES = ['instance-stores', 'instance-history', 'instance-children', 'instance-jobs', 'instance-docs', 'instance-commands', 'instance-typed-ops', 'instance-render', 'instance-ladders', 'time-travel-turns', 'tool-run-turns', 'instance-app']
CLASS_A = {'instance-stores', 'instance-history', 'instance-children', 'instance-jobs', 'instance-docs'}
ENTRY_B = {'instance-commands': '🧵️instance/🧷️command-routes/🦀️.rs', 'instance-typed-ops': '🧵️instance/🧾️typed-operations/🦀️.rs', 'instance-render': '🧵️instance/🖼️render/🦀️.rs', 'instance-ladders': '🪜️maintenance-ladder/🦀️.rs', 'time-travel-turns': '⏪️time-travel/🧵️instance/🦀️.rs', 'tool-run-turns': '⏯️tool-run/🧵️instance/🦀️.rs', 'instance-app': '🧵️instance/🦀️.rs'}
def staging_files(crate):
    d = PLUG + SP.CRATE_ROOT[crate] + '/📥️staging'
    out = []
    if os.path.isdir(d):
        for dp, dn, fn in os.walk(d):
            for f in fn:
                if f.endswith('.rs'): out.append(os.path.relpath(os.path.join(dp, f), PLUG))
    return sorted(out)
def build():
    own = {}; entry = {}; mounts = {}; extra = {}
    for c in CRATES:
        root = SP.CRATE_ROOT[c]
        sf = staging_files(c)
        if c in CLASS_A:
            entry[c] = [root + '/🦀️.rs', 'A']; own[c] = [root + '/📥️staging/']; mounts[c] = sf
        else:
            e = ENTRY_B[c]; entry[c] = [e, 'B']; own[c] = [e, root + '/📥️staging/']
            if c == 'instance-app': own[c].append('🧵️instance/🔌️plugin-app-impl/')
            mounts[c] = [] if c == 'instance-app' else sf
        extra[c] = []
    json.dump(dict(own=own, entry=entry, mounts=mounts, extra=extra), open(GEN + '/pwp6-crates6.json', 'w'), ensure_ascii=False, indent=1)
    print({c: len(mounts[c]) for c in CRATES})
if __name__ == '__main__': build()
