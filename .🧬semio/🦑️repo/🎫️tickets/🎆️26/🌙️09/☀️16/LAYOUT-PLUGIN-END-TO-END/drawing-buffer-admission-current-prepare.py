from pathlib import Path
import hashlib,json,os,re
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
core=Path("🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work")
pages=core/"📚️pages"
entry=ticket/"drawing-buffer-admission-current.test.ts"
roots=[core/"🟦️.ts",core/"🧪️tests/🟦️.ts",core/"⏱️turn/🧪️tests/work.test.ts",core/"⏱️turn/🧪️tests/effects.test.ts",core/"🧪️tests/absolute/absolute.test.ts",core/"♻️retirement/🧪️tests/retire.test.ts",pages/"🧪️tests/🟦️.ts",pages/"🧪️tests/contiguous.test.ts",pages/"🧪️tests/admission.test.ts",pages/"🧪️tests/admission-provenance.test.ts",pages/"🧪️tests/admission-behavior.test.ts",ticket/"drawing-buffer-ingress.test.ts",core/"📦️bytes/🧪️tests/🟦️.ts",Path("🧰️framework/🔨️modules/🔲️pixels/📷️png/✍️encode/🧪️tests/🟦️.ts"),Path("🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🎟️work/🧪️tests/🟦️.ts")]
def relative_import(p):
 value=os.path.relpath(root/p,entry.parent).replace(os.sep,"/")
 return value if value.startswith(".") else "./"+value
entry.write_text('/** 🎟️ Actual affected admission, row, ownership, close and caller laws execute from original modules. */\nimport {test} from "bun:test";\nimport {Database} from "bun:sqlite";\nimport {testRetainedWork,testRetainedWorkRetirement} from "'+relative_import(roots[1])+'";\n'+"".join('import "'+relative_import(p)+'";\n' for p in roots[2:])+'test("original retained-work ordering, shared holds and materialization",()=>testRetainedWork(values=>{const db=new Database(":memory:");try{return db.query<{value:number},[string]>("SELECT CAST(value AS INTEGER) AS value FROM json_each(?) ORDER BY CAST(value AS INTEGER)").all(JSON.stringify(values)).map(row=>row.value);}finally{db.close();}}));\ntest("original retained-work paid retirement and denial axes",()=>testRetainedWorkRetirement());\n')
roots.append(entry)
pattern=re.compile(r"""(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s*)["']([^"']+)["']""")
queue=[entry,*[root/p for p in roots]]
seen=set()
edges=[]
external=set()
while queue:
 p=queue.pop().resolve()
 if p in seen:continue
 if not p.is_relative_to(root):raise RuntimeError("Nonrepository literal "+str(p))
 if not p.is_file():raise RuntimeError("Missing literal "+str(p))
 seen.add(p)
 if p.suffix not in (".ts",".js",".mts",".cts",".mjs",".cjs"):continue
 for spec in pattern.findall(p.read_text()):
  if not spec.startswith("."):
   external.add(spec)
   continue
  target=(p.parent/spec).resolve()
  if not target.is_file():raise RuntimeError("Unresolved exact literal "+str(target))
  edges.append({"from":str(p.relative_to(root)),"specifier":spec,"to":str(target.relative_to(root))})
  queue.append(target)
  if target.suffix==".js":
   declaration=target.with_suffix(".d.ts")
   if declaration.is_file():queue.append(declaration)
seen.update([root/"package.json",root/"bun.lock"])
rows=[]
for p in sorted(seen):
 raw=p.read_bytes()
 units=len(str(p).encode("utf-16-le"))//2
 if units>256:raise RuntimeError("Path length "+str(p))
 rows.append({"path":str(p.relative_to(root)),"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"absoluteUtf16Units":units})
commands=[]
base=ticket/"🗑️generated/drawing-ts-buffer-admission-current"
for kind in ["test","strict"]:
 area=base/kind
 env={"SEMIO_BUILD_MODE":"prototype","SEMIO_TEST_ARTIFACT_DIR":str(area),**{k:str(area/"tmp") for k in ["TMPDIR","TMP","TEMP","TEMPDIR"]}}
 if kind=="test":argv=["repo","--service=off","tools","exec","bun","--","test",str(entry)]
 else:argv=["repo","--service=off","tools","exec","bun","--",str(root/"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","preserve","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",*[str(root/p) for p in roots]]
 commands.append({"kind":kind,"cwd":str(root),"argv":argv,"environment":env,"log":str(area/("test.log" if kind=="test" else "strict.log"))})
packet={"state":"prepared-review-only","authority":False,"launchAuthorized":False,"attemptsExecuted":0,"expectedOwningLaws":49,"actualOutcomes":None,"inputs":rows,"literalEdges":edges,"externalTestToolSpecifiers":sorted(external),"commands":commands,"strictRoots":[str(p) for p in roots],"scope":"All affected Core/turn/absolute/helpers/page/admission/provenance laws, original primitive function registrations with SQLite ordering, unchanged intrinsic baseline, Byte/PNG and numeric owning laws. Direct genuine admission/row CPU; wider Font/Drawing not proved.","freeze":"Exact listed firstparty inputs/importer/package/lock through actual terminal and postcheck; no blanket source freeze."}
p=ticket/"drawing-buffer-admission-current-command.json"
p.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n")
(ticket/"drawing-buffer-admission-current-preparation.md").write_text("# Current Admission and Affected Caller Preparation\n\nPrepared only for Root review. The actual .test.ts importer registers each owning module, plus the two existing original primitive functions using an independent SQLite ordering result. No fixture mirroring or assertion removal. Existing shared neutral values and finite stocks are retained; old scalar contiguous assumptions now observe real page/directory body demand and conservation. All current loans are genuine and admission/row CPU comes only from defining commits. Prior ownership/quote and CPU RED receipts remain immutable. Full Font/Drawing closure stays open. Normal managed test then strict over the same capture, dedicated all-five environment paths, no runtime yet.\n")

print(json.dumps({"packet":str(p),"sha256":hashlib.sha256(p.read_bytes()).hexdigest(),"inputCount":len(rows),"edges":len(edges),"external":sorted(external)},ensure_ascii=False))
