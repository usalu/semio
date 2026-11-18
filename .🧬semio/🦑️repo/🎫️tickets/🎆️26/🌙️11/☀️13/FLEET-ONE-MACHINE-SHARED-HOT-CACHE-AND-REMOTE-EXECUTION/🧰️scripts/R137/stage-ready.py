import os,re,sys
ROOT='/Users/ueli/Documents/semio'
B='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R137/stage/ready'
R=ROOT+'/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready'
OLD='{"name": "workspace", "kind": "repo", "arguments": ["run", "workspace:setup"], "timeoutMilliseconds": 14400000, "inputs": [], "bootstrapInput": false}'
NEW='{"name": "workspace", "kind": "repo", "arguments": ["run-many", "--projects=@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp", "--targets=build"], "timeoutMilliseconds": 3600000, "inputs": [], "bootstrapInput": false}'
PROBES='''    "Probes": {
      "const": {
        "version": 1,
        "target": "build",
        "projects": [
          {"name": "@semio-tech/norm-din4108", "kind": "typescript"},
          {"name": "@semio-tech/norm-din4108-rs", "kind": "rust"},
          {"name": "repo-mcp", "kind": "go"}
        ]
      }
    },
'''
s=open(R+'/🧬️schema/🔣️.json').read()
assert s.count(OLD)==1 and s.count('    "Texts": {')==1
s=s.replace(OLD,NEW).replace('    "Texts": {',PROBES+'    "Texts": {')
os.makedirs(B+'/🧬️schema',exist_ok=True);open(B+'/🧬️schema/🔣️.json','w').write(s)
t=open(R+'/🟦️.ts').read()
anchor='/** 🧬️ The schema of `ready.json`. */'
assert t.count(anchor)==1
t=t.replace(anchor,'/** 🎯️ The declared probes of the `workspace` stage: one target of a handful of projects of different kinds, so readiness reads their closure and never the whole workspace. */\nexport const READY_PROBES_V1=schema.definitions.Probes.const as Readonly<{version:1;target:string;projects:readonly Readonly<{name:string;kind:string}>[]}>;\n'+anchor)
def absolute(m):
    q=m.group(1);p=m.group(2)
    if p.startswith('./🧬️schema'): return m.group(0)
    return q+os.path.normpath(os.path.join(R,p))+q
t=re.sub(r'(["\'])(\.\.?/[^"\']+)\1',absolute,t)
open(B+'/🟦️.ts','w').write(t)
x=open(os.path.dirname(B)+'/probes.test.ts').read()
os.makedirs(B+'/🧪️tests/🎯️probes',exist_ok=True)
x=x.replace('let at=import.meta.dir;','let at="'+ROOT+'";')
open(B+'/🧪️tests/🎯️probes/🟦️.ts','w').write(x)
print('staged')
