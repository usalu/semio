from pathlib import Path
import hashlib,json,os,re
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
byte=Path("🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📦️bytes")
png=Path("🧰️framework/🔨️modules/🔲️pixels/📷️png/✍️encode")
entry=ticket/"drawing-bytes-png.test.ts"
roots=[byte/"🟦️.ts",byte/"🧪️tests/🟦️.ts",png/"🟦️.ts",png/"🧪️tests/🟦️.ts"]
entry.write_text("/** 📦️ Actual byte and PNG owning laws register from their original modules. */\n"+"".join('import "'+os.path.relpath(root/p,entry.parent).replace(os.sep,"/")+'";\n' for p in [roots[1],roots[3]]))
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
base=ticket/"🗑️generated/drawing-ts-bytes-png"
for kind in ["test","strict"]:
 area=base/kind
 env={"SEMIO_BUILD_MODE":"prototype","SEMIO_TEST_ARTIFACT_DIR":str(area),**{k:str(area/"tmp") for k in ["TMPDIR","TMP","TEMP","TEMPDIR"]}}
 if kind=="test":argv=["repo","--service=off","tools","exec","bun","--","test",str(entry)]
 else:argv=["repo","--service=off","tools","exec","bun","--",str(root/"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","preserve","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",*[str(root/p) for p in roots]]
 commands.append({"kind":kind,"cwd":str(root),"argv":argv,"environment":env,"log":str(area/("test.log" if kind=="test" else "strict.log"))})
packet={"state":"prepared-review-only","authority":False,"launchAuthorized":False,"attemptsExecuted":0,"expectedOwningLaws":10,"expectedByteLaws":5,"expectedPngLaws":5,"actualOutcomes":None,"inputs":rows,"literalEdges":edges,"externalTestToolSpecifiers":sorted(external),"commands":commands,"strictRoots":[str(p) for p in roots],"scope":"Actual original Byte5+PNG5 laws and canonical strict4 only; wider Drawing/font callers unproved","freeze":"Exact listed firstparty inputs, importer, package.json and bun.lock through actual terminal and postcheck; no blanket source freeze."}
p=ticket/"drawing-bytes-png-command.json"
p.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n")
(ticket/"drawing-bytes-png-preparation.md").write_text("# Byte and PNG Scoped Verification Preparation\n\nPREP ONLY. The actual .test.ts importer statically registers the five original Byte laws and five PNG laws from their defining source modules; it copies no Source. Expected ten registrations is distinct from an observed outcome. No semantic command was launched.\n\nThe retained preparation script follows exact relative static import/export edges, records builtin/external test-tool imports without treating installed libraries as firstparty Source, includes the .js declaration facet, and hashes package.json/bun.lock. Every literal path resolves in the actual checkout and each absolute operand is within the 256 UTF16-unit limit. Canonical strict uses both defining modules and both test roots. Both commands have separate ticket artifact/temp directories and all five required environment variables, prototype mode and ordinary managed repo tools admission. No light semantic run or changed caps.\n\nRoot must review/release the exact packet before launch. While frozen, disjoint font/Drawing Source work may continue. Current source-only stock/output-close and actual witness laws remain unproved until genuine outcomes.\n")
print(json.dumps({"packet":str(p),"sha256":hashlib.sha256(p.read_bytes()).hexdigest(),"inputCount":len(rows),"edges":len(edges),"external":sorted(external)},ensure_ascii=False))
