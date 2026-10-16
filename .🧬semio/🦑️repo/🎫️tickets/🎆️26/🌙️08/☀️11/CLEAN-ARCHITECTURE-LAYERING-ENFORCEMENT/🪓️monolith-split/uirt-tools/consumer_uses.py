# 🔎️ Collects the identifiers external crates import from semio_framework_ui_runtime (aliased or not).
import re,os,sys,collections
roots=["✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground","🌎️hub/🧩️compositions/🎪️demonstrator","🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"]
pat=re.compile(r'(?:semio_framework_ui_runtime|ui_runtime)::(\{(?:[^{}]|\{[^{}]*\})*\}|[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)',re.S)
names=collections.defaultdict(set)
files=set()
for r in roots:
    for dp,dn,fn in os.walk(r):
        dn[:]=[d for d in dn if d not in ('target','node_modules','dist','.git')]
        for f in fn:
            if f.endswith('.rs'):
                p=os.path.join(dp,f)
                t=open(p,errors='replace').read()
                for m in pat.finditer(t):
                    body=m.group(1)
                    for n in re.findall(r'[A-Za-z_][A-Za-z0-9_]*',body):
                        names[n].add(p)
                    files.add(p)
# also `use semio_framework_ui_runtime as X` aliases
for n in sorted(names): print(n,len(names[n]))
print('FILES',len(files),file=sys.stderr)
