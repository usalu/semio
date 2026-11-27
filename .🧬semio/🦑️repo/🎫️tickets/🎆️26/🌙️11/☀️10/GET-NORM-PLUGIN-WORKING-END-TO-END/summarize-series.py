import sys,json
for line in open(sys.argv[1],errors="ignore"):
    if line.startswith('{"exit"'):
        d=json.loads(line);print(d['peaks'],"samples",len(d['series']),"last",d['series'][-1] if d['series'] else None,"seconds",d['series'][-1][0]-d['series'][0][0] if d['series'] else 0)
