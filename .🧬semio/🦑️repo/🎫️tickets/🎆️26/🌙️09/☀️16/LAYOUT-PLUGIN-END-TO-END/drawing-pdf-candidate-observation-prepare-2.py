from pathlib import Path
import hashlib,json,os,re
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
base=root/"✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene"
law=base/"📋️prepare/🎟️owned/🧪️tests/owned.test.ts"
textLaw=base/"🔤️text/🧪️tests/🎟️owned/text.test.ts"
paintLaw=base.parent/"🎨️attributes/🧪️tests/paint.test.ts"
valueLaw=base.parent/"🎨️attributes/🧪️tests/🎯️value/value.test.ts"
corpusLaw=base/"🔤️text/🧪️tests/🎨️corpus/corpus.test.ts"
oracle=base/"🔤️text/🧪️tests/🔮️oracle/🟦️.ts"
svg=base.parent.parent/"🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧵️write"
svgLaw=svg/"🧪️tests/🟦️.ts"
svgWorkLaw=svg/"🎟️work/🧪️tests/owned.test.ts"
pdf=base.parent.parent/"🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write"
pdfLaw=pdf/"🧪️tests/🟦️.ts"
pdfWorkLaw=pdf/"🎟️work/🧪️tests/owned.test.ts"
pages=base.parent.parent/"🚪️io/📤️export/📦️owned/📜️pages"
pageLaw=pages/"🧪️tests/pages.test.ts"
candidateLaw=pdf/"🎟️work/🧪️tests/admission.test.ts"
flattenSchemaLaw=root/"🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🧪️tests/🎟️work/schema.test.ts"
entry=ticket/"drawing-pdf-candidate-observation-2.test.ts"
entry.write_text('/** 🏗️ Actual Scene25, standalone value guard and all original fourteen owned glyph registrations. */\n'+''.join('import '+json.dumps(os.path.relpath(p,entry.parent),ensure_ascii=False)+';\n' for p in [candidateLaw,flattenSchemaLaw]))
strictRoots=[]
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
prior=json.loads((ticket/"drawing-text-owned-command-2.json").read_text())
for row in prior["inputs"]:
 p=root/row["path"]
 if p.suffix not in [".ts",".js",".mjs",".cjs",".json",".mts",".cts"]:seen.add(p)
seen.update([root/"package.json",root/"bun.lock"])
rows=[]
for p in sorted(seen):
 b=p.read_bytes();units=len(str(p).encode("utf-16-le"))//2
 if units>256:raise RuntimeError("Overlong path "+str(p))
 rows.append({"path":str(p.relative_to(root)),"bytes":len(b),"sha256":hashlib.sha256(b).hexdigest(),"absoluteUtf16Units":units})
commands=[]
for kind in ["test"]:
 area=ticket/"🗑️generated/drawing-pdf-candidate-observation-2"/kind
 (area/"tmp").mkdir(parents=True,exist_ok=True)
 env={"SEMIO_BUILD_MODE":"prototype","SEMIO_TEST_ARTIFACT_DIR":str(area),**{k:str(area/"tmp") for k in ["TMPDIR","TMP","TEMP","TEMPDIR"]}}
 argv=["repo","--service=off","tools","exec","bun","--","test",str(entry),"--test-name-pattern","PDF path formatter|closed flatten neutral schema"] if kind=="test" else ["repo","--service=off","tools","exec","bun","--",str(root/"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","preserve","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",*map(str,strictRoots)]
 commands.append({"kind":kind,"cwd":str(root),"argv":argv,"environment":env,"log":str(area/("test.log" if kind=="test" else "strict.log"))})
packet={"state":"prepared-review-only-unexecuted","authority":False,"attemptsExecuted":0,"expectedOwningLaws":2,"expectedBreakdown":{"actualPdfCandidateRegression":1,"actualNeutralSchemaValidation":1},"inputs":rows,"literalEdges":edges,"unresolved":[],"externalTestToolSpecifiers":sorted(external),"commands":commands,"strictRoots":[str(p.relative_to(root))for p in strictRoots],"scope":"Focused owning regression of actual PDF formatter candidate continuity before the production fix, plus authorized actual shared flatten schema validation. Expected candidate failure is distinct from actual observed outcome; no full SVG/PDF or lower acceptance. Production unchanged from the immutable first63/strict pair."}
out=ticket/"drawing-pdf-candidate-observation-2-command.json";out.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n")
print(json.dumps({"command":str(out),"sha256":hashlib.sha256(out.read_bytes()).hexdigest(),"inputs":len(rows),"edges":len(edges),"external":sorted(external)},ensure_ascii=False))
