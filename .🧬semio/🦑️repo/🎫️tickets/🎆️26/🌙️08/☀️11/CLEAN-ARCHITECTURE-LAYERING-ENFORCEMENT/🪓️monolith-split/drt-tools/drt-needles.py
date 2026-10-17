import re,sys,os,glob
root="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing"
test=root+"/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts"
rs=[]
for d,_,fs in os.walk(root):
    for f in fs:
        if f.endswith(".rs"): rs.append(os.path.join(d,f))
src={p:open(p,encoding="utf8").read() for p in rs}
t=open(test,encoding="utf8").read()
lines=t.split("\n")
for i,l in enumerate(lines,1):
    for m in re.finditer(r'(?:includes|toContain)\(("(?:[^"\\]|\\.)*")\)',l):
        needle=eval(m.group(1).replace("\\\\","\\")) if False else __import__("json").loads(m.group(1))
        hits=[p.split("🖍️drawing/")[1] for p,s in src.items() if needle in s]
        print(i,repr(needle)[:90],"->",hits[:4])
