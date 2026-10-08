import json,re
def write(p,s):
    f=open(p,'r+',encoding='utf8',newline=''); f.write(s); f.truncate(); f.close()
W='🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript'
p=W+'/📋️project.json'
s=open(p,encoding='utf8',newline='').read()
d=json.loads(s)
# cut each top-level 4-space-indented target block by key
def cut(s,key):
    start=s.index('\n    "%s": {\n'%key)+1
    m=re.compile(r'\n    \},?\n').search(s,start)
    end=m.end()
    return s[:start]+s[end:]
for k in ['serve','dev','native','native-release']:
    assert s.count('\n    "%s": {\n'%k)==1,k
    s=cut(s,k)
d2=json.loads(s)
assert set(d['targets'])-set(d2['targets'])=={'serve','dev','native','native-release'}
write(p,s)
p=W+'/package.json'
s=open(p,encoding='utf8',newline='').read()
for l in ['    "serve": "bun nx run @semio-tech/framework-renderer-wgpu:serve",\n','    "dev": "bun nx run @semio-tech/framework-renderer-wgpu:dev",\n','    "native": "bun nx run @semio-tech/framework-renderer-wgpu:native",\n']:
    assert s.count(l)==1
    s=s.replace(l,'')
json.loads(s)
write(p,s)
