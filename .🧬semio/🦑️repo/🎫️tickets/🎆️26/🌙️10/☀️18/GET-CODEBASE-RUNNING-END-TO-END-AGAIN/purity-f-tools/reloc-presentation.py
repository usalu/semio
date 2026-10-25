import os,sys,shutil,re
R="/Users/ueli/Documents/semio/"
P=R+"🧰️framework/🛍️products/🎤️presentation/"
OLD=P+"📦️packages/🟦️typescript/🎯️targets/⚛️react/"
NEW=P+"🎯️targets/⚛️react/📦️packages/🟦️typescript/"
OWN=P+"🎯️targets/⚛️react/"
apply = len(sys.argv)>1 and sys.argv[1]=="apply"
assert os.path.isdir(OLD) and not os.path.exists(NEW), "state"
styles=sorted(os.listdir(OLD+"🎨️styles"))
plan=[(OLD+"package.json",NEW+"package.json"),(OLD+"📋️project.json",NEW+"📋️project.json"),(OLD+"📜️script.ts",NEW+"📜️script.ts"),(OLD+"🟦️.tsx",NEW+"🟦️.tsx"),(OLD+"🎨️.css",NEW+"🎨️.css"),(OLD+"🧰️vitest.setup.ts",OWN+"🧪️tests/🧰️vitest-setup/🟦️.ts"),(OLD+"node_modules",NEW+"node_modules")]
for s in styles: plan.append((OLD+"🎨️styles/"+s+"/🎨️.css",OWN+"🎨️styles/"+s+"/🎨️.css"))
for a,b in plan: print(a[len(R):],"->",b[len(R):])
if not apply: sys.exit()
for a,b in plan:
    os.makedirs(os.path.dirname(b),exist_ok=True); os.rename(a,b)
for s in styles: os.rmdir(OLD+"🎨️styles/"+s)
os.rmdir(OLD+"🎨️styles"); os.rmdir(OLD); os.rmdir(P+"📦️packages/🟦️typescript/🎯️targets")
def edit(path,pairs):
    t=open(path,encoding='utf8').read()
    for a,b in pairs:
        assert t.count(a)>=1,(path,a)
        t=t.replace(a,b)
    open(path,'w',encoding='utf8').write(t)
edit(NEW+"🟦️.tsx",[('"../../../../🎯️targets/⚛️react/🟦️.tsx"','"../../🟦️.tsx"')])
css=open(NEW+"🎨️.css",encoding='utf8').read()
css=css.replace('"./🎨️styles/','"../../🎨️styles/').replace('@source ".";','@source "../..";')
open(NEW+"🎨️.css",'w',encoding='utf8').write(css)
edit(NEW+"📜️script.ts",[('"../../../../🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts"','"../../🧪️tests/🎚️config/🟦️.ts"')])
edit(NEW+"package.json",[('"directory": "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react"','"directory": "🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/📦️packages/🟦️typescript"')])
edit(NEW+"📋️project.json",[('"cwd": "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react"','"cwd": "🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/📦️packages/🟦️typescript"')])
edit(OWN+"🧪️tests/🎚️config/🟦️.ts",[('"../../../../📦️packages/🟦️typescript/🎯️targets/⚛️react"','"../../📦️packages/🟦️typescript"'),('resolve(root, "../../🟦️.ts")','resolve(root, "../../../../📦️packages/🟦️typescript/🟦️.ts")'),('resolve(root, "🧰️vitest.setup.ts")','resolve(root, "../../🧪️tests/🧰️vitest-setup/🟦️.ts")')])
print("moved+edited")
