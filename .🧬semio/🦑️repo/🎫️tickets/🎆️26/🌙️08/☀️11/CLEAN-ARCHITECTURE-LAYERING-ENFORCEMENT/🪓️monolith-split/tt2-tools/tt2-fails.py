import re,sys
f=None;out=[]
for l in open(sys.argv[1],encoding='utf8'):
    l=l.rstrip('\n')
    if l.startswith('=== '): f=l[4:]
    elif l.startswith('(fail)'): out.append(f+' | '+re.sub(r' \[[0-9.]+m?s\]$','',l))
    elif re.match(r'^error: Cannot find module',l) and f: out.append(f+' | '+l[:120])
print('\n'.join(sorted(out)))
