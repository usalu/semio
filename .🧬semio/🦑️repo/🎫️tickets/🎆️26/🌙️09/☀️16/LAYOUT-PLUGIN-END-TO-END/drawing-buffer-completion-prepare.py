from pathlib import Path
import hashlib,json,os,re
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
core=Path("🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work")
pages=core/"📚️pages"
entry=ticket/"drawing-buffer-completion.test.ts"
roots=[core/"🟦️.ts",pages/"🧪️tests/🟦️.ts",pages/"🧪️tests/contiguous.test.ts",ticket/"drawing-buffer-ingress.test.ts"]
def relative_import(p):
 value=os.path.relpath(root/p,entry.parent).replace(os.sep,"/")
 return value if value.startswith(".") else "./"+value
entry.write_text("/** 📚️ Actual page ownership laws and the unchanged intrinsic-mutation regression register from original modules. */\n"+"".join('import "'+relative_import(p)+'";\n' for p in [roots[1],roots[2],roots[3]]))
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
base=ticket/"🗑️generated/drawing-ts-buffer-completion"
for kind in ["test","strict"]:
 area=base/kind
 env={"SEMIO_BUILD_MODE":"prototype","SEMIO_TEST_ARTIFACT_DIR":str(area),**{k:str(area/"tmp") for k in ["TMPDIR","TMP","TEMP","TEMPDIR"]}}
 if kind=="test":argv=["repo","--service=off","tools","exec","bun","--","test",str(entry)]
 else:argv=["repo","--service=off","tools","exec","bun","--",str(root/"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","preserve","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",*[str(root/p) for p in roots]]
 commands.append({"kind":kind,"cwd":str(root),"argv":argv,"environment":env,"log":str(area/("test.log" if kind=="test" else "strict.log"))})
packet={"state":"prepared-review-only","authority":False,"launchAuthorized":False,"attemptsExecuted":0,"expectedOwningLaws":11,"expectedPageLaws":8,"expectedCompletionLaws":2,"expectedExistingBehavioralRegressionLaws":1,"independentAdmissionCpuProved":False,"actualOutcomes":None,"inputs":rows,"literalEdges":edges,"externalTestToolSpecifiers":sorted(external),"commands":commands,"strictRoots":[str(p) for p in roots],"scope":"Actual eight page laws + two completion/nested quote laws + unchanged intrinsic baseline, strict4; independent admission CPU and wider Font/Drawing unproved","freeze":"Exact listed firstparty inputs, importer, package.json and bun.lock through actual terminal and postcheck; no blanket source freeze."}
p=ticket/"drawing-buffer-completion-command.json"
p.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n")
(ticket/"drawing-buffer-completion-preparation.md").write_text("# Actual Page and Completion Refresh Preparation\n\nPREP ONLY. Three actual source imports register eight original page laws, both standalone and nested-held exact completion quote laws, and the unchanged intrinsic-mutation baseline. Expected eleven is separate from actual. Strict4 follows actual terminal/postcheck on the same exact closure. Separate test/strict all5ticket paths, prototype, normal managed tools, unchanged caps/profiles/stock and compiler options. No new runtime.\n\nThe genuine focused quote RED is retained separately. Only the completed quote conditional changed in defining production; reversing that exact conditional reconstructs the entire prior core hash. New nested schema/law preceded that fix. Independent admission CPU remains unproved: the existing fixture epoch fallback is explicit and is not interpreted as production CPU. Root review/release required before one new attempt.\n")

print(json.dumps({"packet":str(p),"sha256":hashlib.sha256(p.read_bytes()).hexdigest(),"inputCount":len(rows),"edges":len(edges),"external":sorted(external)},ensure_ascii=False))
