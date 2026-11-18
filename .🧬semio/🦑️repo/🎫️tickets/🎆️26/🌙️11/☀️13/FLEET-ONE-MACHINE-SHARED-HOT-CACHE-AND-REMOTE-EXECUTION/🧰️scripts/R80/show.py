import sys,json
for l in sys.stdin:
    l=l.strip()
    if l.startswith('{'):
        try:d=json.loads(l)
        except: print(l[:300]);continue
        print({k:d[k] for k in ['owners','sec','heapUsedMB','externalMB','rssMB','objs','verifiedBlockRows','settledReads','interned','layerCounts','closure','scripts.files','scripts.closures','scripts.modules'] if k in d})
    else: print(l[:900])
