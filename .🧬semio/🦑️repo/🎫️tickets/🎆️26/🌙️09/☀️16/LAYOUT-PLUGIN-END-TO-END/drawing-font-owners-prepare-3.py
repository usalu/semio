from pathlib import Path
import hashlib,json,os,re
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
font=Path("🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font")
tests=font/"🎟️work/📈️producer/🧪️tests"
entry=ticket/"drawing-font-owners.test.ts"
roots=[font/"🟦️.ts",font/"🧵️shape/🌐️unicode/🟦️.ts",font/"🧵️shape/🌐️unicode/↩️line-break/🟦️.ts",font/"🧵️shape/🌐️unicode/🧵️cluster/🟦️.ts",font/"🧵️shape/🌐️unicode/🧵️normalize/🟦️.ts",font/"🎟️work/📈️producer/🟦️.ts",tests/"producer.test.ts",tests/"normalize.test.ts"]
expectedEntry=("/** 🔤️ Actual font owner laws register from their defining original modules. */\n"+"".join('import "'+os.path.relpath(root/p,entry.parent).replace(os.sep,"/")+'";\n' for p in roots[-2:]))
if entry.read_text()!=expectedEntry:raise RuntimeError("Registration importer drift")
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
base=ticket/"🗑️generated/drawing-ts-font-owners-3"
for kind in ["test","strict"]:
 area=base/kind
 env={"SEMIO_BUILD_MODE":"prototype","SEMIO_TEST_ARTIFACT_DIR":str(area),**{k:str(area/"tmp") for k in ["TMPDIR","TMP","TEMP","TEMPDIR"]}}
 if kind=="test":argv=["repo","--service=off","tools","exec","bun","--","test",str(entry)]
 else:argv=["repo","--service=off","tools","exec","bun","--",str(root/"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","preserve","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",*[str(root/p) for p in roots]]
 commands.append({"kind":kind,"cwd":str(root),"argv":argv,"environment":env,"log":str(area/("test.log" if kind=="test" else "strict.log"))})
packet={"state":"prepared-current-real-pages-and-direct-loan-cpu","authority":False,"launchAuthorized":False,"attemptsExecuted":0,"expectedOwningLaws":9,"expectedAdmissionLaws":8,"expectedNormalizationLaws":1,"actualOutcomes":None,"inputs":rows,"literalEdges":edges,"externalTestToolSpecifiers":sorted(external),"commands":commands,"strictRoots":[str(p) for p in roots],"scope":"Actual original admission8 plus normalization1/three ICU cases and strict9; no shipped-font IO or unfinished Run/Scene caller proof","freeze":"Exact listed firstparty inputs, importer, package.json and bun.lock through actual terminal and postcheck; no blanket source freeze."}
p=ticket/"drawing-font-owners-command-3.json"
p.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n")
(ticket/"drawing-font-owners-preparation-3.md").write_text("# Font Owners Scoped Preparation\n\nPREP ONLY. The two-import .test.ts importer registers the eight actual admission/real-child/exact-ceiling laws and the actual normalization law over three ICU cases. Expected nine registrations is distinct from an observed outcome. Current foundation producers use real pages, mandatory issued loans and direct leaf CPU; no epoch inference or duplicate row commit. No semantic command was launched.\n\nExact firstparty relative import/export closure plus JS declaration facet, importer, package.json and bun.lock are hashed. External Ajv/SQLite/OpenType and system ICU remain test-tool/reference boundaries; production has no new external runtime dependency. This scope reads no shipped-font files and imports no unfinished Run/Scene producer.\n\nBoth commands use normal managed repo tools Bun, prototype, all five dedicated ticket artifact/temp paths and absolute operands, with separate logs. Canonical strict9 includes six defining source roots, both test roots and actual registration importer. No changed limits, light semantic run or Source copies. Root must review and release before launch.\n")

print(json.dumps({"packet":str(p),"sha256":hashlib.sha256(p.read_bytes()).hexdigest(),"inputCount":len(rows),"edges":len(edges),"external":sorted(external)},ensure_ascii=False))
