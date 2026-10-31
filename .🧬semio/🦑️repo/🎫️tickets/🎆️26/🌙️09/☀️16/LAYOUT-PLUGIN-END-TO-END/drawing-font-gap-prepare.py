from pathlib import Path
import hashlib,json,os,re
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
core=Path("🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work")
pages=core/"📚️pages"
entry=ticket/"drawing-font-gap.test.ts"
font=Path("🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font")
workspace=font/"🧵️shape/🧩️context/🪄️apply/🏪️workspace"
roots=[workspace/"🟦️.ts",workspace/"🎟️work/🧪️tests/🟦️.ts",font/"🧵️shape/📇️face/🧵️scalar/🟦️.ts",font/"🧵️shape/📇️face/🟦️.ts"]
def relative_import(p):
 value=os.path.relpath(root/p,entry.parent).replace(os.sep,"/")
 return value if value.startswith(".") else "./"+value
entry.write_text('/** 🏬️ Actual gap ownership laws register from the defining original test module. */\nimport "'+relative_import(roots[1])+'";\n')
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
base=ticket/"🗑️generated/drawing-ts-font-gap"
for kind in ["test","strict"]:
 area=base/kind
 env={"SEMIO_BUILD_MODE":"prototype","SEMIO_TEST_ARTIFACT_DIR":str(area),**{k:str(area/"tmp") for k in ["TMPDIR","TMP","TEMP","TEMPDIR"]}}
 if kind=="test":argv=["repo","--service=off","tools","exec","bun","--","test",str(entry)]
 else:argv=["repo","--service=off","tools","exec","bun","--",str(root/"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","preserve","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",*[str(root/p) for p in roots]]
 commands.append({"kind":kind,"cwd":str(root),"argv":argv,"environment":env,"log":str(area/("test.log" if kind=="test" else "strict.log"))})
packet={"state":"prepared-review-only","authority":False,"launchAuthorized":False,"attemptsExecuted":0,"expectedOwningLaws":5,"actualOutcomes":None,"inputs":rows,"literalEdges":edges,"externalTestToolSpecifiers":sorted(external),"commands":commands,"strictRoots":[str(p) for p in roots],"scope":"Actual5 glyph gap/output custody laws +canonical strict5 workspace/actualtests/scalar/cluster. Original finite wallet/SQL output and every denial/cancel/sole-owner transfer retained. Wider Run/Outline/Context/Drawing unproved.","freeze":"Exact listed firstparty inputs/importer/package/lock through terminal/postcheck; no blanket freeze."}
p=ticket/"drawing-font-gap-command.json"
p.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n")
(ticket/"drawing-font-gap-preparation.md").write_text("# Glyph Gap and Actual Output Owner Preparation\n\nPrepared only, no runtime. Existing five canonical schema-first owning laws execute unchanged through the actual registration module. Same finite wallet/fuel and independent SQLite list output, positive-axis denials, partial source-clear cancellation, exact deletion-operation refusal and original object identity/sole output page transfer remain. Source-body producer signatures now use genuine direct leaf CPU loans. Strict5 includes actual workspace and its law plus newly migrated Scalar/Cluster caller roots. No glyph-corpus/full Run/Drawing claim. Normal managed test then captured strict, all five dedicated ticket envs/separate outputs; Root review required.\n")

print(json.dumps({"packet":str(p),"sha256":hashlib.sha256(p.read_bytes()).hexdigest(),"inputCount":len(rows),"edges":len(edges),"external":sorted(external)},ensure_ascii=False))
