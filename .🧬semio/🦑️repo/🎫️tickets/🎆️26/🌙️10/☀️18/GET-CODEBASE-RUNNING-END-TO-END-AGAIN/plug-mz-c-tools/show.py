import sys,json
pat=sys.argv[2]; n=int(sys.argv[3]) if len(sys.argv)>3 else 1
for l in open(sys.argv[1],errors="replace"):
    if l.startswith('{"E"') and pat in l:
        j=json.loads(l); print(j["rendered"][:2200]); n-=1
        if n==0: break
