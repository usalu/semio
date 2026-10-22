import os,re,subprocess,json
R="/Users/ueli/Documents/semio"
base=R+"/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d"
old=base+"/🤖️generated/📇️registry/📦️packages/🦀️rust"
new=base+"/🛂️manifest/📦️packages/🦀️rust"
src=open("/private/tmp/pgc-ov/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🤖️generated/📇️registry/📦️packages/🦀️rust/Cargo.toml",encoding='utf8').read()
# dependencies: rewrite relative paths (old dir -> new dir)
def rel_fix(m):
    path=m.group(1)
    target=os.path.normpath(os.path.join(old,path))
    return 'path = "'+os.path.relpath(target,new)+'"'
text=re.sub(r'path = "([^"]+)"',lambda m: rel_fix(m) if m.group(1)!="🦀️.rs" else m.group(0),src)
text=text.replace('workspace = "../../../.."','workspace = "../../.."').replace('[lib]\npath = "🦀️.rs"','[lib]\npath = "../../🤖️generated/📇️registry/🦀️.rs"')
text=text.replace('description = "Component crate of semio-s-artifact-puzzle-2d: graph-manifest"','description = "Graph manifest registry of the 2D puzzle: the manifests generated from the authored graph manifests"')
os.makedirs(new,exist_ok=True)
open(new+"/Cargo.toml","w",encoding='utf8').write(text)
print("crate written",len(text))
# dependents
files=subprocess.run(["rg","-l","--no-messages","-g","Cargo.toml","semio-s-artifact-puzzle-2d-graph-manifest = \\{ path",base],capture_output=True,text=True).stdout.split("\n")
n=0
for f in files:
    if not f: continue
    s=open(f,encoding='utf8').read()
    d=os.path.dirname(f)
    def fix(m):
        target=os.path.normpath(os.path.join(d,m.group(1)))
        if target!=old: return m.group(0)
        return 'semio-s-artifact-puzzle-2d-graph-manifest = { path = "'+os.path.relpath(new,d)+'" }'
    t=re.sub(r'semio-s-artifact-puzzle-2d-graph-manifest = \{ path = "([^"]+)" \}',fix,s)
    if t!=s: open(f,"w",encoding='utf8').write(t); n+=1
print("dependents",n)
# workspace members
w=base+"/Cargo.toml"; s=open(w,encoding='utf8').read()
o='"🤖️generated/📇️registry/📦️packages/🦀️rust",'
assert s.count(o)==1
open(w,"w",encoding='utf8').write(s.replace(o,'"🛂️manifest/📦️packages/🦀️rust",')); print("members ok")
# owners
p=R+"/📋️build-owners.json"; s=open(p,encoding='utf8').read()
a='"root": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🤖️generated/📇️registry/📦️packages/🦀️rust",\n      "manifest": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🤖️generated/📇️registry/📦️packages/🦀️rust/Cargo.toml",'
assert s.count(a)==1
open(p,"w",encoding='utf8').write(s.replace(a,'"root": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🛂️manifest/📦️packages/🦀️rust",\n      "manifest": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🛂️manifest/📦️packages/🦀️rust/Cargo.toml",')); print("owners ok")
