from pathlib import Path
import hashlib,json,os,re
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
base=root/"🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten"
law=base/"🧪️tests/🎟️work/🟦️ts/funded.test.ts"
coverageLaw=base/"🧪️tests/🎟️work/🟦️ts/🎨️coverage/coverage.test.ts"
settlementLaw=base/"🧪️tests/🎟️work/🟦️ts/🏁️settlement/settlement.test.ts"
coverageTypes=base/"🧪️tests/🎟️work/🟦️ts/🎨️coverage/oracle.d.mts"
schemaLaw=base/"🧪️tests/🎟️work/schema.test.ts"
entry=ticket/"drawing-ts-flatten-owning-4.test.ts"
entry.write_text("/** 📏️ Actual funded flatten five laws plus exact shared schema validation. */"+chr(10)+"".join("import "+json.dumps(os.path.relpath(p,entry.parent),ensure_ascii=False)+";"+chr(10) for p in [law,schemaLaw,coverageLaw,settlementLaw]))
strictRoots=[base/"🟦️.ts",law,schemaLaw,coverageLaw,settlementLaw,coverageTypes,entry]
pattern=re.compile(r"""(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s*)["']([^"']+)["']""")
seen=set();edges=[];external=set();queue=[entry,*strictRoots]
def target_value(value):
 if isinstance(value,str):return value
 if isinstance(value,dict):
  for key in ["bun","source","import","default","types"]:
   if key in value:return target_value(value[key])
 raise RuntimeError("Unsupported export "+str(value))
while queue:
 p=queue.pop().resolve()
 if p in seen:continue
 if not p.is_file() or not p.is_relative_to(root):raise RuntimeError("Nonrepository/missing input "+str(p))
 seen.add(p)
 if p.suffix not in [".ts",".js",".mjs",".cjs",".mts",".cts"]:continue
 for spec in pattern.findall(p.read_text()):
  if spec.startswith("."):q=(p.parent/spec).resolve()
  elif spec.startswith("@semio-tech/"):
   parts=spec.split("/");name="/".join(parts[:2]);manifest=(root/"node_modules"/name/"package.json").resolve()
   if not manifest.is_relative_to(root) or not manifest.is_file():raise RuntimeError("Missing first-party package "+name)
   seen.add(manifest);pkg=json.loads(manifest.read_text());key="." if len(parts)==2 else "./"+"/".join(parts[2:]);exports=pkg.get("exports",{})
   if key not in exports:raise RuntimeError("Missing literal package export "+spec)
   q=(manifest.parent/target_value(exports[key])).resolve()
  else:external.add(spec);continue
  if not q.is_file():raise RuntimeError("Missing literal "+str(q))
  edges.append({"from":str(p.relative_to(root)),"specifier":spec,"to":str(q.relative_to(root))});queue.append(q)
  if q.suffix==".js" and q.with_suffix(".d.ts").is_file():queue.append(q.with_suffix(".d.ts"))
  if q.suffix==".mjs" and q.with_suffix(".d.mts").is_file():queue.append(q.with_suffix(".d.mts"))
seen.update([root/"package.json",root/"bun.lock"])
rows=[]
for p in sorted(seen):
 b=p.read_bytes();units=len(str(p).encode("utf-16-le"))//2
 if units>256:raise RuntimeError("Overlong path "+str(p))
 rows.append({"path":str(p.relative_to(root)),"bytes":len(b),"sha256":hashlib.sha256(b).hexdigest(),"absoluteUtf16Units":units})
commands=[]
for kind in ["test","strict"]:
 area=ticket/"🗑️generated/drawing-ts-flatten-owning-4"/kind
 (area/"tmp").mkdir(parents=True,exist_ok=True)
 env={"SEMIO_BUILD_MODE":"prototype","SEMIO_TEST_ARTIFACT_DIR":str(area),**{k:str(area/"tmp") for k in ["TMPDIR","TMP","TEMP","TEMPDIR"]}}
 argv=["repo","--service=off","tools","exec","bun","--","test",str(entry)] if kind=="test" else ["repo","--service=off","tools","exec","bun","--",str(root/"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","preserve","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",*map(str,strictRoots)]
 commands.append({"kind":kind,"cwd":str(root),"argv":argv,"environment":env,"log":str(area/("test.log" if kind=="test" else "strict.log"))})
packet={"state":"prepared-review-only-unexecuted","authority":False,"attemptsExecuted":0,"expectedOwningLaws":8,"inputs":rows,"literalEdges":edges,"unresolved":[],"externalTestToolSpecifiers":sorted(external),"commands":commands,"strictRoots":[str(p.relative_to(root))for p in strictRoots],"scope":"Initial genuine funded flatten five laws exercise every17original vector and independentVector2/SQLite, plus actual exactschema regression, original28flag/transform Sharp law and genuine parent Runtime exact ceiling/denied-resume. FULL originalSharp allarcflags/transform and async/retirement/caller migrations remain mandatory; no fullowner/productclaim."}
out=ticket/"drawing-ts-flatten-owning-4-command.json";out.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n")
print(json.dumps({"command":str(out),"sha256":hashlib.sha256(out.read_bytes()).hexdigest(),"inputs":len(rows),"edges":len(edges)},ensure_ascii=False))
