import json, os
ROOT = "/Users/ueli/Documents/semio"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G = B + "/🕸️graph/🕰️generations"
HERE = os.path.dirname(os.path.abspath(__file__))

def write(directory, spec, files):
    out = os.path.join(HERE, "staged", directory)
    os.makedirs(out, exist_ok=True)
    for name, text in files.items():
        os.makedirs(os.path.dirname(os.path.join(out, name)) or out, exist_ok=True)
        open(os.path.join(out, name), "w", encoding="utf-8").write(text)
    json.dump(spec, open(os.path.join(out, "edits.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)

# ---- batch 1: win-withheld
mjs = open(os.path.join(ROOT, G, "🟨️.mjs"), encoding="utf-8").read()
dts = open(os.path.join(ROOT, G, "🟨️.d.mts"), encoding="utf-8").read()
old_import = 'import{dirname,join}from"node:path";'
old_head = 'export function unfoundedWithheldOwnersV1(index,owners){'
old_loop = '   const row=index.rows[id];if(!kinds.has(row.kind))continue;\n   for(const config of configs)if(row.path===config||row.path.endsWith("/"+config)){'
new_loop = '   const row=index.rows[id];if(!kinds.has(row.kind))continue;\n   const path=separator==="/"?row.path:row.path.split(separator).join("/");\n   for(const config of configs)if(path===config||path.endsWith("/"+config)){'
old_sig = 'owners:Readonly<Record<string,{kind:string;tag?:string;config?:string}>>):string[];'
for text, needle in ((mjs, old_import), (mjs, old_head), (mjs, old_loop), (dts, old_sig)):
    assert text.count(needle) == 1, needle
test = '''/** \U0001FA9F\ufe0f Contract: the row paths of a fact index are native absolute paths (`resolve` of the observation module), the config of a withheld owner is a workspace relative `/` path; a native Windows path must be judged by components like a POSIX path, else every withheld owner counts as unfounded on that host and is inferred again by every build. */
import{expect,test}from"bun:test";
import{createFactIndexV1,unfoundedWithheldOwnersV1}from"../../../\U0001F7E8\ufe0f.mjs";

const fact={sha256:"0".repeat(64),size:1,mtimeMs:1,ctimeMs:1,ino:1};
const row=(path:string)=>({kind:"file",path,fact});
const owners={founded:{kind:"unavailable",config:"p/x.ts"},unfounded:{kind:"unavailable",config:"p/y.ts"},root:{kind:"unavailable",config:"z.ts"}};
const index=()=>createFactIndexV1(new Map([["founded",[row("C:\\\\ws\\\\p\\\\x.ts")]],["unfounded",[row("C:\\\\ws\\\\p\\\\other.ts")]],["root",[row("C:\\\\ws\\\\z.ts")]]]) as never);

test("a native Windows path is judged by its components",()=>{
 expect(unfoundedWithheldOwnersV1(index() as never,owners,"\\\\")).toEqual(["unfounded"]);
});

test("a backslash is an ordinary character where the separator is a slash",()=>{
 expect(unfoundedWithheldOwnersV1(index() as never,owners,"/").sort()).toEqual(["founded","root","unfounded"]);
});

test("a POSIX path is unchanged by either separator rule",()=>{
 const posix=createFactIndexV1(new Map([["founded",[row("/ws/p/x.ts")]],["unfounded",[row("/ws/p/other.ts")]],["root",[row("/ws/z.ts")]]]) as never);
 expect(unfoundedWithheldOwnersV1(posix as never,owners,"/")).toEqual(["unfounded"]);
 expect(unfoundedWithheldOwnersV1(posix as never,owners,"\\\\")).toEqual(["unfounded"]);
});
'''
spec = {"edits": [
  {"file": G + "/🟨️.mjs", "old": old_import, "new": 'import{dirname,join,sep}from"node:path";'},
  {"file": G + "/🟨️.mjs", "old": old_head, "new": 'export function unfoundedWithheldOwnersV1(index,owners,separator=sep){'},
  {"file": G + "/🟨️.mjs", "old": old_loop, "new": new_loop},
  {"file": G + "/🟨️.d.mts", "old": old_sig, "new": 'owners:Readonly<Record<string,{kind:string;tag?:string;config?:string}>>,separator?:string):string[];'},
], "newFiles": [{"path": G + "/🧪️tests/🚧️unfounded-withheld/🪟️windows/🟦️.ts", "from": "files/windows-law.ts"}]}
write("win-withheld", spec, {"files/windows-law.ts": test})
print("win-withheld written")

# ---- batch 2: upgrade-before-custody
marks_path = B + "/🌡️host-capacity/🚏️marks/🟦️.ts"
marks = open(os.path.join(ROOT, marks_path), encoding="utf-8").read()
graph_path = B + "/🕸️graph/🟦️.ts"
graph = open(os.path.join(ROOT, graph_path), encoding="utf-8").read()
tail = marks[marks.rindex(" return(await import("):]
assert marks.endswith(tail) and marks.count(tail) == 1
new_tail = tail + '''
/** ⏫️ Grows a quiet forecast grant into its full request BEFORE the graph builder custody is taken: the wait for capacity is a fair queue with no timer, so it must not happen while a custody is held that every other builder queues for, and not while other runs hold full grants and wait for that custody (each would wait for the other); a process under no grant, or one that only inherits it, asks for nothing. */
export async function upgradeBuildCapacityBeforeRebuildV1(environment:Readonly<Record<string,string|undefined>>=process.env):Promise<boolean>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined||environment["SEMIO_HOST_CAPACITY_PHASE_MARKS"]==="off")return false;
 await(await import("../\U0001F7E6️.ts")).upgradeBuildCapacityV1(undefined,environment);return true;
}
'''
old_import = 'import{markBuildCapacityPhaseV1}from"../🌡️host-capacity/🚏️marks/🟦️.ts";'
old_image = '      let image:string|undefined;\n      const result=await withBuildGraphStateV1(root,invocation,phaseControl,(store,control)=>{image=store.root;return body(store,control);},generationScopeV1);'
assert graph.count(old_import) == 1 and graph.count(old_image) == 1
law = '''/** \U0001F53C️ Contract: a run that was forecast quiet asks for its full capacity before the graph builder custody is taken, never while holding it; the phase mark inside the custody is bookkeeping and waits for nothing. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{upgradeBuildCapacityBeforeRebuildV1}from"../../../\U0001F321️host-capacity/\U0001F68F️marks/\U0001F7E6️.ts";

const source=readFileSync(new URL("../../\U0001F7E6️.ts",import.meta.url),"utf8");
const builder=source.indexOf("withBuilder:async<T>");

test("the capacity upgrade of a rebuild is asked before the builder custody is taken",()=>{
 const upgrade=source.indexOf("await upgradeBuildCapacityBeforeRebuildV1();",builder),custody=source.indexOf("withBuildGraphStateV1(root,invocation,phaseControl,(store,control)=>{image=store.root;return body(store,control);},generationScopeV1)",builder);
 expect(builder).toBeGreaterThan(0);expect(upgrade).toBeGreaterThan(builder);expect(custody).toBeGreaterThan(upgrade);
});

test("the phase mark stays inside the custody body and the only wait before the custody is the upgrade",()=>{
 const body=source.slice(builder,source.indexOf("generationScopeV1)",builder));
 expect(body).toContain('const body=async(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>{await markBuildCapacityPhaseV1("rebuild");return build(store,control);};');
 expect(body.match(/upgradeBuildCapacity/gu)?.length).toBe(1);
});

test("a process under no grant, or one that only inherits it, asks for nothing",async()=>{
 expect(await upgradeBuildCapacityBeforeRebuildV1({})).toBe(false);
 expect(await upgradeBuildCapacityBeforeRebuildV1({SEMIO_HOST_CAPACITY_GRANT:"0".repeat(24),SEMIO_HOST_CAPACITY_PHASE_MARKS:"off"})).toBe(false);
});
'''
spec = {"edits": [
  {"file": marks_path, "old": tail, "new": new_tail},
  {"file": graph_path, "old": old_import, "new": 'import{markBuildCapacityPhaseV1,upgradeBuildCapacityBeforeRebuildV1}from"../🌡️host-capacity/🚏️marks/🟦️.ts";'},
  {"file": graph_path, "old": old_image, "new": '      await upgradeBuildCapacityBeforeRebuildV1();\n' + old_image},
], "newFiles": [{"path": B + "/🕸️graph/🧪️tests/🔼️upgrade-before-custody/🟦️.ts", "from": "files/upgrade-law.ts"}]}
write("upgrade-before-custody", spec, {"files/upgrade-law.ts": law})
print("upgrade-before-custody written")
