import re,sys,os,collections
errs=collections.defaultdict(list)
for l in open(sys.argv[1],encoding="utf8"):
    m=re.match(r'^(.*?)\((\d+),(\d+)\): error TS2769',l)
    if m: errs[m.group(1)].append((int(m.group(2)),int(m.group(3))))
def matches(line):
    stack=[];pairs={};i=0;n=len(line);q=None
    while i<n:
        c=line[i]
        if q:
            if c=="\\":i+=2;continue
            if c==q:q=None
        elif c in "\"'`":q=c
        elif c=="(":stack.append(i)
        elif c==")" and stack:pairs[i]=stack.pop()
        i+=1
    return pairs
for f,locs in errs.items():
    lines=open(f,encoding="utf8").read().split("\n");changed=0
    for ln,col in sorted(set(locs),reverse=True):
        line=lines[ln-1];head=line[:col-1]
        m=re.search(r'\)\.(toBe|toEqual|toStrictEqual)\($',head)
        if not m: print("SKIP",f[-50:],ln,col);continue
        close=m.start();pairs=matches(line)
        if close not in pairs: print("NOPAIR",f[-50:],ln,col);continue
        o=pairs[close]
        if line[o-6:o]!="expect": print("NOTEXPECT",f[-50:],ln,col);continue
        lines[ln-1]=line[:o]+"<unknown>"+line[o:];changed+=1
    if changed:
        t=f+".h54tmp";open(t,"w",encoding="utf8").write("\n".join(lines));os.replace(t,f);print("fixed",changed,f[-60:])
