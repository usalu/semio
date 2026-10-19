import sys,re,glob,collections,urllib.parse
L="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/law1-out/"
for h in sys.argv[1:]:
    f=glob.glob(L+h+"-*.log")[0]
    lines=open(f,encoding="utf-8",errors="replace").read().split("\n")
    c=collections.Counter()
    for i,l in enumerate(lines):
        if l.startswith("error:") or re.match(r"^[A-Za-z]*Error",l):
            key=(l[:200]+" | "+(lines[i+1][:100] if i+1<len(lines) else "")+" | "+(lines[i+2][:100] if i+2<len(lines) else "")).strip()
            c[urllib.parse.unquote(key)]+=1
    print("####",h,f[len(L):][:80])
    for k,n in c.most_common(6): print("  ",n,k[:420])
