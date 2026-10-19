import os,re,sys
root=os.getcwd()
F="🧰️framework"
targets={
 "semio-framework-os-store-link-schema":f"{F}/🔨️modules/🧬️schema/🗿️artifact-reference/🔗️link/🧬️schema/📦️packages/🦀️rust",
 "semio-framework-os-store-blob-identity":f"{F}/🔨️modules/🧬️schema/🗿️artifact-reference/📦️blob/🧬️schema/📦️packages/🦀️rust",
}
files=[l.strip() for l in sys.stdin if l.strip()]
for f in files:
    t=open(f,encoding="utf8").read()
    d=os.path.dirname(os.path.join(root,f))
    def fix(m):
        name=m.group(1)
        rel=os.path.relpath(os.path.join(root,targets[name]),d).replace(os.sep,"/")
        return f'{name} = {{ path = "{rel}" }}'
    n=re.sub(r'^(semio-framework-os-store-(?:link-schema|blob-identity)) = \{ path = "[^"]*" \}',fix,t,flags=re.M)
    if n!=t:
        open(f,"w",encoding="utf8").write(n); print("fixed",f[-90:])
