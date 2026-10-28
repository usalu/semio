import re, sys
f=sys.argv[1]; pat=re.compile(sys.argv[2]); lim=int(sys.argv[3]) if len(sys.argv)>3 else 6
W=int(sys.argv[4]) if len(sys.argv)>4 else 90
n=0
with open(f,encoding='utf-8',errors='replace') as h:
    for ln,line in enumerate(h,1):
        for m in pat.finditer(line):
            s=max(0,m.start()-W//3); e=min(len(line),m.end()+W)
            print(f"{ln}: ...{line[s:e].strip()}..."[:200]); n+=1
            if n>=lim: sys.exit(0)
