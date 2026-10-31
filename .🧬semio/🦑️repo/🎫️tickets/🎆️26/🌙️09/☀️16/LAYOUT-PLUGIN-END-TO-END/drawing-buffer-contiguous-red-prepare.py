from pathlib import Path
import hashlib,json,os,re
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
core=Path("🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work")
pages=core/"📚️pages"
entry=ticket/"drawing-buffer-contiguous-red.test.ts"
roots=[core/"🟦️.ts",pages/"🧪️tests/contiguous.test.ts"]
def relative_import(p):
 value=os.path.relpath(root/p,entry.parent).replace(os.sep,"/")
 return value if value.startswith(".") else "./"+value
entry.write_text("/** 📚️ The actual completed-admission quote law registers from its original module. */\n"+"".join('import "'+relative_import(p)+'";\n' for p in [roots[1]]))
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
base=ticket/"🗑️generated/drawing-ts-buffer-contiguous-red"
for kind in ["test"]:
 area=base/kind
 env={"SEMIO_BUILD_MODE":"prototype","SEMIO_TEST_ARTIFACT_DIR":str(area),**{k:str(area/"tmp") for k in ["TMPDIR","TMP","TEMP","TEMPDIR"]}}
 if kind=="test":argv=["repo","--service=off","tools","exec","bun","--","test",str(entry)]
 else:argv=["repo","--service=off","tools","exec","bun","--",str(root/"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","preserve","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",*[str(root/p) for p in roots]]
 commands.append({"kind":kind,"cwd":str(root),"argv":argv,"environment":env,"log":str(area/("test.log" if kind=="test" else "strict.log"))})
packet={"state":"prepared-review-only","authority":False,"launchAuthorized":False,"attemptsExecuted":0,"expectedOwningLaws":1,"expectedIntendedFailure":"actual completed admission quote includes unneeded future directory","actualOutcomes":None,"inputs":rows,"literalEdges":edges,"externalTestToolSpecifiers":sorted(external),"commands":commands,"strictRoots":[str(p) for p in roots],"scope":"Focused existing-API completed-admission quote law only; no Source fix applied and no runtime yet","freeze":"Exact listed firstparty inputs, importer, package.json and bun.lock through actual terminal and postcheck; no blanket source freeze."}
p=ticket/"drawing-buffer-contiguous-red-command.json"
p.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n")
(ticket/"drawing-buffer-contiguous-red-preparation.md").write_text("# Exact Contiguous Quote Focused Preparation\n\nPREP ONLY. One actual existing-API law observes the real first page admission, its cached next demand and the zero-effect completion under zero extra physical grant. Expected intended failure is a quoted extra directory128 instead of zero physical currency, not missing API/import or cleanup. Exact literal closure and package/lock captured; normal managed Bun/all5ticket paths, no timeout or cap change. Root must review contract and release before one actual attempt. No Source fix or runtime has occurred.\n")

print(json.dumps({"packet":str(p),"sha256":hashlib.sha256(p.read_bytes()).hexdigest(),"inputCount":len(rows),"edges":len(edges),"external":sorted(external)},ensure_ascii=False))
