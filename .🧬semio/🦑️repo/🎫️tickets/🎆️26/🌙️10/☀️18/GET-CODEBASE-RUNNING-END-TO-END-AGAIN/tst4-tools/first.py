import sys,re
t=open(sys.argv[1],encoding="utf-8",errors="replace").read().split("\n")
n=int(sys.argv[2]) if len(sys.argv)>2 else 1
c=0
for i,l in enumerate(t):
    if l.startswith("(fail)"):
        j=i
        while j>0 and not t[j-1].startswith(("(pass)","(fail)","(skip)")) and i-j<40: j-=1
        seg=[x for x in t[j:i] if x.strip()]
        print("#",l[:160])
        for x in seg:
            if re.match(r"\s*(error|Error|TypeError|ReferenceError|SyntaxError)|expect\(|Expected|Received|^\s+at <anonymous>|^\s*\^",x) or x.lstrip().startswith(("-","+")): print("   ",x[:300])
        c+=1
        if c>=n: break
