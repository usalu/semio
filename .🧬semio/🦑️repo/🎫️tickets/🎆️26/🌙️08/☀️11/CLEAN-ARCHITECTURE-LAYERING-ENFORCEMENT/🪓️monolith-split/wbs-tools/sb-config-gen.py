import os,re,shutil
R="/Users/ueli/Documents/semio"
D="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/wbs-tools/sb-config"
os.makedirs(D,exist_ok=True)
def absolutize(s):
    s=re.sub(r'(from\s*|import\()"\.\./',lambda m:m.group(1)+'"'+R+'/',s)
    s=re.sub(r'(from\s*|import\()"\./',lambda m:m.group(1)+'"'+R+'/.storybook/',s)
    return s
m=open(R+"/.storybook/main.ts",encoding="utf8").read()
m=absolutize(m)
imp=re.search(r'import \{ canonicalWasmBuildVitePlugin \} from "[^"]+";\n',m); assert imp
stub='''const STUB = /^(@semio-tech\\/(framework-surface-rs|framework-editor-rs|framework-os-node-graph-rs|puzzle-wasm|flow-core)(\\/.*)?|[^?]*\\.wasm(\\?.*)?)$/u;
function canonicalWasmBuildVitePlugin(_root: string, _mode: string) {
  return { name: "wbs-wasm-stub", enforce: "pre" as const, resolveId(id: string) { if (STUB.test(id)) return { id: "\\0wbs-stub:" + id, syntheticNamedExports: true }; }, load(id: string) { if (id.startsWith("\\0wbs-stub:")) return "export default {};"; } } as never;
}
'''
m=m.replace(imp.group(0),stub)
old='const repoRootPath = resolve(__dirname, "..");'; assert old in m
m=m.replace(old,'const repoRootPath = "'+R+'";')
old='stories: buildScopeStoryGlobs(activeScopes),'; assert old in m
m=m.replace(old,'stories: buildScopeStoryGlobs(activeScopes).map((g) => resolve(repoRootPath, ".storybook", g)),')
open(D+"/main.ts","w",encoding="utf8").write(m)
p=open(R+"/.storybook/preview.tsx",encoding="utf8").read()
open(D+"/preview.tsx","w",encoding="utf8").write(absolutize(p))
for name in ["🎨️styling"]:
    t=D+"/"+name
    if os.path.lexists(t): os.remove(t)
    os.symlink(R+"/.storybook/"+name,t)
print("generated")
