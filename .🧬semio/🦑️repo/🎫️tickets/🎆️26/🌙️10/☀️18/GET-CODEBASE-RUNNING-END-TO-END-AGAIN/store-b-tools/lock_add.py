import sys,re
# usage: lock_add.py <lockfile> <package> <dep> [<dep>...]
lock,pkg,*deps=sys.argv[1:]
t=open(lock).read()
m=re.search(r'\[\[package\]\]\nname = "%s"\nversion = "[^"]*"\n(?:source = "[^"]*"\n)?(?:checksum = "[^"]*"\n)?dependencies = \[\n(.*?)\n\]'%re.escape(pkg),t,re.S)
assert m,pkg
body=m.group(1).split('\n');items=[b.strip().strip(',').strip('"') for b in body]
for d in deps:
    if d not in items:items.append(d)
items=sorted(set(items))
new=',\n'.join(' "%s"'%i for i in items)+','
t=t[:m.start(1)]+new+t[m.end(1):]
open(lock,'w').write(t);print('ok',pkg,deps)
