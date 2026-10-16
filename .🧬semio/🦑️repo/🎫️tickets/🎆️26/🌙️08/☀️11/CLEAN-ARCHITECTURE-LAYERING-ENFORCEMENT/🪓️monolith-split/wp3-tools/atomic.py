"""🧷️ Runs an edit script all-or-nothing: copies every file under the given roots first and restores them when the script fails."""
import os, shutil, subprocess, sys
script = sys.argv[1]; roots = sys.argv[2:]
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/wp3-atomic'
shutil.rmtree(SCR, ignore_errors=True); os.makedirs(SCR)
files = []
for r in roots:
    for dp, dn, fn in os.walk(r):
        for f in fn:
            if f.endswith('.rs'): files.append(os.path.join(dp, f))
for i, f in enumerate(files): shutil.copy2(f, f'{SCR}/{i}.rs')
open(SCR + '/index.txt', 'w', encoding='utf8').write('\n'.join(files))
before = set(files)
rc = subprocess.call(['python3', script])
if rc != 0:
    for i, f in enumerate(files): shutil.copy2(f'{SCR}/{i}.rs', f)
    for dp, dn, fn in os.walk(roots[0]): pass
    for r in roots:
        for dp, dn, fn in os.walk(r):
            for f in fn:
                p = os.path.join(dp, f)
                if f.endswith('.rs') and p not in before: os.remove(p)
    print('RESTORED after failure', rc); sys.exit(rc)
