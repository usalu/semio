import json,sys
g=json.load(open('/tmp/r8a/filegraph.json'))
a,b=sys.argv[1],sys.argv[2]
for s in g:
    if a in s:
        for d,ns in g[s].items():
            if b in d: print(s[:50],'->',d[:50],ns[:12])
