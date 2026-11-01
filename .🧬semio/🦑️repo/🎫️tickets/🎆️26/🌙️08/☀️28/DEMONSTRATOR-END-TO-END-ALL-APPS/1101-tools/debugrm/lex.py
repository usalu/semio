import sys, re, os
ROOT="/Users/ueli/Documents/semio"
def string_mask(src, lang):
    mask = [False]*len(src); i=0; n=len(src)
    while i < n:
        c = src[i]
        if lang=="rs" and src.startswith("//", i):
            j = src.find("\n", i); j = n if j<0 else j; i=j; continue
        if lang=="rs" and src.startswith("/*", i):
            j = src.find("*/", i+2); j = n if j<0 else j+2; i=j; continue
        if lang=="ts" and src.startswith("//", i):
            j = src.find("\n", i); j = n if j<0 else j; i=j; continue
        if lang=="ts" and src.startswith("/*", i):
            j = src.find("*/", i+2); j = n if j<0 else j+2; i=j; continue
        m = None
        if lang=="rs":
            m = re.match(r'r(#*)"', src[i:i+20]) or re.match(r'b?r(#*)"', src[i:i+20])
            if m:
                hashes = m.group(1); end = '"'+hashes
                j = src.find(end, i+m.end()); j = n if j<0 else j+len(end)
                for k in range(i,j): mask[k]=True
                i=j; continue
        if c in '"\'`' and not (lang=="rs" and c=="'" and re.match(r"'(\\.|[^\\'])'", src[i:i+6]) is None and not re.match(r"'\\", src[i:i+3])):
            if lang=="rs" and c=="'":
                # char literal or lifetime
                mm = re.match(r"'(\\[^']*|[^\\'])'", src[i:i+12])
                if mm:
                    i += mm.end(); continue
                i+=1; continue
            q=c; j=i+1
            while j<n:
                if src[j]=='\\': j+=2; continue
                if src[j]==q: break
                j+=1
            for k in range(i, min(j+1,n)): mask[k]=True
            i=j+1; continue
        i+=1
    return mask
if __name__=="__main__":
    S=sys.argv[1]
    cands=[]
    for line in open(S+"/args.tsv",encoding="utf-8"):
        flag,rel,n,test,args=line.rstrip("\n").split("\t",4)
        cands.append((rel,int(n)))
    cache={}
    for rel,n in cands:
        p=os.path.join(ROOT,rel)
        if rel not in cache:
            src=open(p,encoding="utf-8",errors="replace").read()
            lang="rs" if p.endswith(".rs") else ("ts" if p.endswith((".ts",".tsx",".js",".mjs",".cjs")) else "py")
            if lang=="py": cache[rel]=(src,None)
            else: cache[rel]=(src,string_mask(src,lang))
        src,mask=cache[rel]
        if mask is None: print("PY",rel,n); continue
        lines=src.split("\n")
        off=sum(len(x)+1 for x in lines[:n-1])
        L=lines[n-1]; k=L.find("[DEBUG]")
        # macro start: last macro token before [DEBUG] on line
        mpos=max([m.start() for m in re.finditer(r'(eprintln!|println!|console\.(log|error))\s*\(',L) if m.start()<=k] or [k])
        inside = mask[off+mpos]
        print(("EMBEDDED" if inside else "CODE"), rel, n)
