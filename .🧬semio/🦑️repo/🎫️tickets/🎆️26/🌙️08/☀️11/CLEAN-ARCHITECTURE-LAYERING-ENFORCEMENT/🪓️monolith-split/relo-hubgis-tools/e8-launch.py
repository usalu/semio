"""🚀️ Registers the six producer launch routes of semio-hub-inference-binding in launch.json and the seed."""
import re
ROOT="/Users/ueli/Documents/semio/"
CRATE="semio-hub-inference-binding"; PROJECT="@semio-tech/hub-inference-binding-rs"
ROUTES=[("🏗️build","build","4_build",["900.01611","900.01612"]),("🔎️check","check","4_gate",["922.04611","922.04612"]),("🧪️test","test","4_test",["903.58611","903.58612"])]
def block(indent):
    out=[]
    for label,target,group,orders in ROUTES:
        for (cfg,cfglabel),order in zip([("prototype","🛠️prototype"),("release","🚀️release")],orders):
            out.append(f'''{indent}{{
{indent}  "name": "🦀️ {CRATE} {label} {cfglabel}",
{indent}  "type": "node-terminal",
{indent}  "request": "launch",
{indent}  "command": "bun repo run {PROJECT}:{target} --configuration={cfg} --parallel=1 -- --lib --offline",
{indent}  "cwd": "${{workspaceFolder}}",
{indent}  "presentation": {{
{indent}    "group": "{group}",
{indent}    "order": {order}
{indent}  }}
{indent}}},''')
    return "\n".join(out)+"\n"
for f in [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]:
    p=ROOT+f
    t=open(p,encoding='utf-8').read()
    assert t.count(f'"name": "🦀️ {CRATE} ')==0
    anchor='"name": "🦀️ semio-framework-repo-cli-verbs 🧪️test 🚀️release",'
    assert t.count(anchor)==1,(f,t.count(anchor))
    i=t.index(anchor)
    j=t.index("\n    },\n",i)+len("\n    },\n")
    t=t[:j]+block("    ")+t[j:]
    open(p,'w',encoding='utf-8').write(t)
    print(f,"inserted")
