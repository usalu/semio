import sys, difflib; sys.path.insert(0, '.')
import lawrewrite as W
for f in sys.argv[1:]:
    old = open(f, encoding='utf8').read()
    n, new = W.rewrite(f, False)
    for l in difflib.unified_diff(old.split('\n'), new.split('\n'), lineterm='', n=0):
        print(l[:300])
