import sys, difflib, os
sys.path.insert(0,'.')
import carve, model
b = carve.Batch(sys.argv[1:])
out = b.build(); b.finish()
for p, t in sorted(out.items()):
    old = open(p, encoding='utf8').read() if os.path.exists(p) else ''
    d = list(difflib.unified_diff(old.split('\n'), t.split('\n'), 'a/' + model.short(p)[-60:], 'b/' + model.short(p)[-60:], lineterm='', n=0))
    print('#####', model.short(p)[-90:], '(new)' if not old else '', len(d), 'diff lines')
    for l in d[:60]: print(l[:220])
for v in b.rw.rep.violations: print('VIOLATION', v)
