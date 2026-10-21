import os, sys, re, json
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
def resolve(rel):
    cur = ROOT
    for part in rel.split("/"):
        hits = [n for n in os.listdir(cur) if n.replace("️", "") == part.replace("️", "")]
        assert len(hits) == 1, (part, hits)
        cur = os.path.join(cur, hits[0])
    return cur
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
REV = resolve(LIB + "🏗️build/📥️inputs/🔏️revision/🟨️.mjs")
REVD = REV.replace(".mjs", ".d.mts")
G = resolve(LIB + "🏗️build/🕸️graph/🟦️.ts")
OD = resolve(LIB + "🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs")
PROT_T = resolve(LIB + "🏗️build/🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts")
PROT_J = resolve(LIB + "🏗️build/🕸️graph/🧪️tests/🕰️generation-protocol/🔣️.json")
REV_T = resolve(LIB + "🏗️build/📥️inputs/🔏️revision/🧪️tests/🟦️.ts")
imp = lambda frm, to: ("" if os.path.relpath(to, os.path.dirname(frm)).startswith(".") else "./") + os.path.relpath(to, os.path.dirname(frm)).replace(os.sep, "/")
files = {}
def get(p):
    if p not in files: files[p] = [open(p, encoding="utf8", newline="").read(), False]
    return files[p]
def sub(p, done, old, new, count=1):
    row = get(p)
    if done in row[0]: print("applied", os.path.basename(p), done[:40]); return
    assert row[0].count(old) == count, (p[-40:], old[:70], row[0].count(old))
    row[0] = row[0].replace(old, new); row[1] = True; print("pending", os.path.basename(p), done[:40])

# 1 revision module: authority snapshot of one pinned invocation (lives in the pin context, shared by nested pins)
sub(REV, "pinnedAuthorityReadV1",
    "export async function withImplementationPinV1(body){const pin={active:true};try{return await pinContextV1.run(pin,body);}finally{pin.active=false;}}",
    "export async function withImplementationPinV1(body){const parent=pinContextV1.getStore(),pin={active:true,authority:parent?.active===true?parent.authority:new Map()};try{return await pinContextV1.run(pin,body);}finally{pin.active=false;}}\n"
    "/** 📸️ Reads authority data (the owner registry, declarations) once per pinned invocation: every later read of the same key returns the first answer, so a run completes consistently with the data it started from; outside a pin it reads every time. */\n"
    "export function pinnedAuthorityReadV1(key,read){const rows=pinContextV1.getStore()?.active===true?pinContextV1.getStore().authority:undefined;if(!rows)return read();if(!rows.has(key))rows.set(key,read());return rows.get(key);}\n"
    "/** 🧽️ Forgets the authority snapshot so the next read starts a new consistent view; a graph load calls it before it reads its authority. */\n"
    "export function clearPinnedAuthorityV1(){pinContextV1.getStore()?.authority?.clear();}")
row = get(REVD)
if "pinnedAuthorityReadV1" not in row[0]:
    row[0] += ("" if row[0].endswith("\n") else "\n") + "/** 📸️ Reads authority data once per pinned invocation and returns the first answer to every later read of the key. */\nexport function pinnedAuthorityReadV1<T>(key:string,read:()=>T):T;\n/** 🧽️ Forgets the authority snapshot. */\nexport function clearPinnedAuthorityV1():void;\n"; row[1] = True; print("pending d.mts")
# 2 graph glue
sub(G, "implementationPinnedV1,pinnedAuthorityReadV1", "implementationPinnedV1,physicalSourceOriginV1", "clearPinnedAuthorityV1,implementationPinnedV1,physicalSourceOriginV1,pinnedAuthorityReadV1")
sub(G, "return JSON.parse(pinnedAuthorityReadV1(url.href,", ',captured=capturePhysicalSourceTextV1(url,65536),observed=observationV1.observedFs.readFileSync(url,"utf8");if(observed!==captured)throw Error("Current graph declaration Source changed");return JSON.parse(observed);}',
    ';return JSON.parse(pinnedAuthorityReadV1(url.href,()=>{const captured=capturePhysicalSourceTextV1(url,65536),observed=observationV1.observedFs.readFileSync(url,"utf8");if(observed!==captured)throw Error("Current graph declaration Source changed");return observed;}));}')
OLD_REG = 'requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8")));'
NEW_REG = 'requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(pinnedAuthorityReadV1(join(root,"📋️build-owners.json"),()=>observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8"))));'
sub(G, NEW_REG, OLD_REG, NEW_REG, count=2)
sub(G, "clearPinnedAuthorityV1();return withImplementationPinV1", "return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));", "{clearPinnedAuthorityV1();return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));}")
# 3 owner discovery
rev_imp = imp(OD, REV)
sub(OD, "pinnedAuthorityReadV1", 'readFileSync(join(workspaceRoot,"📋️build-owners.json"),"utf8")', 'pinnedAuthorityReadV1(join(workspaceRoot,"📋️build-owners.json"),()=>readFileSync(join(workspaceRoot,"📋️build-owners.json"),"utf8"))')
sub(OD, 'pinnedAuthorityReadV1(join(workspaceRoot,"📋️build-owners.json"),()=>readPhysicalSource', 'return readPhysicalSource(join(workspaceRoot,"📋️build-owners.json")).toString("utf8");}', 'return pinnedAuthorityReadV1(join(workspaceRoot,"📋️build-owners.json"),()=>readPhysicalSource(join(workspaceRoot,"📋️build-owners.json")).toString("utf8"));}')
row = get(OD)
if "import { pinnedAuthorityReadV1 }" not in row[0]:
    m = re.search(r'^import [^\n]*\n', row[0], re.M)
    row[0] = row[0][:m.end()] + 'import { pinnedAuthorityReadV1 } from "' + rev_imp + '";\n' + row[0][m.end():]; row[1] = True; print("pending OD import")
# 4 laws
def pj(data):
    names = {r["name"] for r in data["present"]}
    for name, fn, text in [("the part discovery reads the owner registry through the authority snapshot", "discoverBuildGraphPartV1", 'pinnedAuthorityReadV1(join(root,"'), ("the generation load starts a new authority snapshot", "loadBuildGraphV1", "clearPinnedAuthorityV1();return withImplementationPinV1(")]:
        if name not in names: data["present"].append({"name": name, "function": fn, "text": text})
row = get(PROT_J); data = json.loads(row[0]); before = json.dumps(data, ensure_ascii=False, indent=1) + "\n"
assert before == row[0] or before.rstrip("\n") == row[0].rstrip("\n")
n0 = len(data["present"]); pj(data)
if len(data["present"]) != n0: row[0] = json.dumps(data, ensure_ascii=False, indent=1) + "\n"; row[1] = True; print("pending protocol json")
row = get(REV_T)
if "authority snapshot" not in row[0]:
    row[0] += '''
test("authority data is read once per pinned invocation and again after the snapshot is cleared; outside a pin every read is fresh",async()=>{
 const fs=await import("node:fs"),os=await import("node:os"),home=fileURLToPath(new URL("..",import.meta.url)),listing=fs.readdirSync(home),moduleName=listing.find(name=>name.endsWith(".mjs"))!,schemaName=listing.find(name=>name.endsWith("schema"))!;
 const scratch=fs.mkdtempSync(join(fs.realpathSync(os.tmpdir()),"revision-snapshot-"));
 try{
  fs.mkdirSync(join(scratch,schemaName));const schemaFile=fs.readdirSync(join(home,schemaName))[0]!;fs.copyFileSync(join(home,schemaName,schemaFile),join(scratch,schemaName,schemaFile));fs.copyFileSync(join(home,moduleName),join(scratch,moduleName));
  const reader=await import(join(scratch,moduleName));let version=0;const read=()=>++version;
  expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(1);expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(2);
  await reader.withImplementationPinV1(async()=>{
   expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(3);expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(3);expect(reader.pinnedAuthorityReadV1("policy",read)).toBe(4);
   await reader.withImplementationPinV1(async()=>{expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(3);});
   expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(3);reader.clearPinnedAuthorityV1();expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(5);
  });
  expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(6);
  await reader.withImplementationPinV1(async()=>{expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(7);});
  await reader.withImplementationPinV1(async()=>{expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(8);});
 }finally{fs.rmSync(scratch,{recursive:true,force:true});}
});
'''
    row[1] = True; print("pending revision test")
if "--write" in sys.argv:
    for p, (t, ch) in files.items():
        if ch: open(p, "w", encoding="utf8", newline="").write(t)
    print("WRITTEN")
