import sys,re,collections
cnt=collections.Counter();own=None
for l in open(sys.argv[1],encoding='utf8'):
    if l.startswith('semio'):
        own=l.split()[0]
    else:
        parts=l.split()
        if len(parts)<2: continue
        dep=parts[1]
        if not own.startswith('semio-s-artifact-stdio-'): continue
        f=lambda n: re.match(r'semio-s-artifact-stdio-([a-z0-9]+)',n).group(1)
        if f(own)!=f(dep): cnt[(own,parts[0],dep)]+=1
for k in sorted(cnt): print(k)
