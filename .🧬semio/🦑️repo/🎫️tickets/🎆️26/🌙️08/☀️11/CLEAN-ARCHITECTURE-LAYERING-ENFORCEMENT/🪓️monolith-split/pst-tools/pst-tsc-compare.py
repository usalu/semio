import re,sys,collections
def load(p):
    c=collections.Counter()
    for line in open(p,encoding="utf8"):
        m=re.match(r"^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$",line.rstrip("\n"))
        if m: c[(m.group(1),m.group(4),m.group(5)[:150])]+=1
    return c
a=load(sys.argv[1]); b=load(sys.argv[2])
new=b-a; gone=a-b
print("before",sum(a.values()),"after",sum(b.values()),"new",sum(new.values()),"gone",sum(gone.values()))
byfile=collections.Counter()
for (f,code,msg),n in new.items(): byfile[f]+=n
for f,n in byfile.most_common(60): print(n,f.replace("✏️s/🔌️plugins/","").replace("🗿️artifacts/","")[:170])
if len(sys.argv)>3:
    for (f,code,msg),n in list(new.items())[:int(sys.argv[3])]: print(code,msg[:140],"|",f[-70:])
