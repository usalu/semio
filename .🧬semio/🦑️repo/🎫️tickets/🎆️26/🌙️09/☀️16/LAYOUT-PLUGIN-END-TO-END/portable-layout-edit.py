from pathlib import Path
import ast,copy,hashlib,json,re,sys
ROOT=next(parent for parent in Path(__file__).resolve().parents if (parent/".agents/skills/build/SKILL.md").is_file() and (parent/"bunfig.toml").is_file())
TICKET=Path(__file__).resolve().parent
DEPS=Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies")
PATCH=TICKET/"🗑️generated/portable-layout-guarded-patch.json"
sha=lambda value:hashlib.sha256(value).hexdigest()
read_utf8=lambda path:path.read_bytes().decode("utf-8")
def dump(value,original):
 levels=[len(match.group(1))for match in re.finditer(r"\n( +)\S",original)]
 indent=min(levels) if levels else 2
 return (json.dumps(value,ensure_ascii=False,indent=indent)+"\n").replace("\n","\r\n" if "\r\n" in original else "\n")
def diff(old,new,path=""):
 if type(old) is not type(new) or not isinstance(old,(dict,list)):
  return [] if old==new else [{"pointer":path,"expected":old,"replacement":new}]
 if isinstance(old,dict):
  if list(old)!=list(new):return [{"pointer":path,"expected":old,"replacement":new}]
  return [row for key in old for row in diff(old[key],new[key],path+"/"+key.replace("~","~0").replace("/","~1"))]
 if len(old)!=len(new):return [{"pointer":path,"expected":old,"replacement":new}]
 return [row for i in range(len(old)) for row in diff(old[i],new[i],path+"/"+str(i))]
def alter(document,change):
 steps=[x.replace("~1","/").replace("~0","~") for x in change["pointer"].split("/")[1:]]
 if not steps:
  if document!=change["expected"]:raise RuntimeError("Changed whole-document JSON precondition")
  return copy.deepcopy(change["replacement"])
 parent=document
 for part in steps[:-1]:parent=parent[int(part)] if isinstance(parent,list) else parent[part]
 key=int(steps[-1]) if isinstance(parent,list) else steps[-1]
 if parent[key]!=change["expected"]:raise RuntimeError("Changed exact JSON precondition: "+change["pointer"])
 parent[key]=change["replacement"]
 return document
def prepare():
 before={};after={};structural=set()
 def read(path):
  path=str(path)
  if path not in before:before[path]=read_utf8(ROOT/path);after[path]=before[path]
  return after[path]
 def replace(path,old,new,count=1):
  text=read(path)
  if text.count(old)!=count:raise RuntimeError("Changed exact text precondition: "+str(path)+" "+repr(old))
  after[str(path)]=text.replace(old,new)
 def modify(path,body):
  text=read(path);value=json.loads(text);body(value);after[str(path)]=dump(value,text);structural.add(str(path))
 descriptor=DEPS/"🔌️managers/🍞️bun/🔣️.json"
 old=json.loads(read(descriptor))
 assert old["recipe"]["version"]==6 and old["recipe"]["linker"]=="isolated"
 def bun(value):
  value["recipe"]["version"]=7;value["recipe"]["linker"]="hoisted"
  value["stagedConfiguration"]=value["stagedConfiguration"].replace('linker = "isolated"','linker = "hoisted"')
  value["installed"]=copy.deepcopy(json.loads(read_utf8(ROOT/DEPS/"🔌️managers/📦️npm/🔣️.json"))["installed"])
 modify(descriptor,bun)
 replace("bunfig.toml",'linker = "isolated"','linker = "hoisted"')
 core=DEPS/"🔌️managers/🟦️.ts"
 replace(core,"JSON.stringify([...fields,manager.runtimeIdentity(input.host),input.host.platform,input.host.arch])","JSON.stringify([manager.recipe(),...fields,manager.runtimeIdentity(input.host),input.host.platform,input.host.arch])")
 replace(core,"Derives the bootstrap receipt key from the manager's declared manifest fields, its runtime identity and the lock bytes.","Derives the bootstrap receipt key from the declared recipe, manifest fields, runtime and lock bytes.")
 modify(DEPS/"🔌️managers/🧬️schema/🔣️.json",lambda value:value["$defs"]["Protocol"]["const"]["bootstrapKeyShape"].insert(0,"recipe"))
 oracle=DEPS/"🔌️managers/🧪️tests/🐍️.py"
 replace(oracle,'compact(fields + [ident, host["platform"], host["arch"]])','compact([d["recipe"]] + fields + [ident, host["platform"], host["arch"]])')
 tests=DEPS/"🔌️managers/🧪️tests/🟦️.ts"
 replace(tests,"the bun keys equal the formulas the live code used before the seam (no cold re-install from the swap itself)","the bun generation and bootstrap keys include the complete declared recipe")
 replace(tests,"JSON.stringify([manifest.dependencies,manifest.devDependencies,undefined,manifest.patchedDependencies,process.versions.bun,process.platform,process.arch])","JSON.stringify([recipe,manifest.dependencies,manifest.devDependencies,undefined,manifest.patchedDependencies,process.versions.bun,process.platform,process.arch])")
 replace(tests,"expect(recipe.version).toBe(layoutVector.recipeVersion);","expect<unknown>(installSchema.$defs.ProducerConfiguration.properties.install.properties.linker.const).toBe(recipe.linker);expect<unknown>(installSchema.$defs.ProducerCompletionProtocol.const.supportedLinkers).toEqual([recipe.linker]);expect(recipe.version).toBe(layoutVector.recipeVersion);")
 install=DEPS/"💾️storage/🧬️schema/📥️install.json"
 def schema(value):
  defs=value["$defs"];defs["ProducerCompletionProtocol"]["const"]["supportedLinkers"]=["hoisted"]
  defs["ProducerConfigurationProtocol"]["const"]["purpose"]="fixed-generation-local-hoisted-layout"
  defs["ProducerConfiguration"]["properties"]["install"]["properties"]["linker"]["const"]=json.loads(after[str(descriptor)])["recipe"]["linker"]
  defs["SourceOwnerBootstrapProtocol"]["const"]["providerLayout"]="canonical-hoisted-provider"
 modify(install,schema)
 source=DEPS/"💾️storage/🟦️.ts"
 text=read(source);assert text.count("isolated")==4
 after[str(source)]=text.replace("isolated","hoisted")
 corpus=DEPS/"💾️storage/🧪️tests/📥️install.json"
 def cases(value):
  for row in value["producerConfigurations"]:
   if row["name"] in ("hoisted","runtime-guard-wrong-install"):
    row["source"]=row["source"].replace('linker = "hoisted"','linker = "isolated"')
    if row["name"]=="hoisted":row["name"]="isolated"
   elif row["source"] is not None:row["source"]=row["source"].replace('linker = "isolated"','linker = "hoisted"')
  def walk(item):
   if isinstance(item,list):
    for i,row in enumerate(item):
     if row=="--linker=isolated":item[i]="--linker=hoisted"
     else:walk(row)
   elif isinstance(item,dict):
    for row in item.values():walk(row)
  walk(value)
 modify(corpus,cases)
 st=DEPS/"💾️storage/🧪️tests/🟦️.ts"
 replace(st,"dependency producer isolated recipe admits only fixed generation-local layout","dependency producer hoisted recipe admits only fixed generation-local layout")
 replace(st,"json_extract(?4,'$.install.linker')='isolated'","json_extract(?4,'$.install.linker')='hoisted'")
 for rel in ["💾️storage/🧪️tests/👓️reading/🟦️.ts","💾️storage/🧪️tests/🔭️providers/🟦️.ts"]:
  path=DEPS/rel;text=read(path)
  after[str(path)]=text.replace("node_modules/.bun/dep","node_modules/dep").replace("node_modules/.bun/postcss","node_modules/postcss").replace("--linker=isolated","--linker=hoisted")
 for rel in ["🪞️adopt/🧪️tests/🟦️.ts","🧾️incremental/🧪️tests/🟦️.ts","🧪️tests/🧫️fixtures/🔣️.json"]:
  path=DEPS/rel;text=read(path);assert "--linker=isolated" in text
  after[str(path)]=text.replace("--linker=isolated","--linker=hoisted")
 path=DEPS/"🪞️adopt/🧪️tests/🔣️.json";text=read(path);assert 'linker = \\"isolated\\"' in text;after[str(path)]=text.replace('linker = \\"isolated\\"','linker = \\"hoisted\\"')
 ns={"__file__":str(ROOT/oracle),"__name__":"portable_layout_oracle"}
 tree=ast.parse(after[str(oracle)],filename=str(ROOT/oracle))
 assert isinstance(tree.body[-1],ast.Expr) and isinstance(tree.body[-1].value,ast.Call) and getattr(tree.body[-1].value.func,"id",None)=="main"
 tree.body.pop();exec(compile(tree,str(ROOT/oracle),"exec"),ns)
 original_load=ns["load"]
 def load(path):
  relative=str(Path(path).resolve().relative_to(ROOT))
  return json.loads(after[relative]) if relative in after else original_load(path)
 ns["load"]=load
 vectors=ns["mint"]()
 path=DEPS/"🔌️managers/🧪️tests/🔣️.json";text=read(path);after[str(path)]=dump(vectors,text);structural.add(str(path))
 bun_descriptor=json.loads(after[str(descriptor)])
 for key,base in ns["schema_bases"]().items():
  path=DEPS/("🔌️managers/🧪️tests/🧫️fixtures/"+("📥️install.original.json" if key=="install" else "🧾️generation.original.json"))
  text=read(path);after[str(path)]=dump(ns["regions"](base,ns["schema_view"](bun_descriptor)),text);structural.add(str(path))
 rows=[]
 for path in before:
  if after[path]==before[path]:continue
  is_json=path.endswith(".json")
  if is_json and json.loads(before[path])==json.loads(after[path]):continue
  json_edit=path in structural
  changes=diff(json.loads(before[path]),json.loads(after[path])) if json_edit else None
  rows.append({"path":path,"beforeSha256":sha(before[path].encode()),"afterSha256":sha(after[path].encode()),"kind":"json" if json_edit else "text","changes":changes if json_edit else [{"start":a,"end":b,"expected":before[path][a:b],"replacement":after[path][c:d]} for a,b,c,d in changed_regions(before[path],after[path])],"utf16Units":len(str(ROOT/path).encode("utf-16-le"))//2})
 assert all(row["utf16Units"]<=256 for row in rows)
 PATCH.write_bytes((json.dumps({"version":1,"purpose":"portable-layout-source-input","authority":False,"sourceApplied":False,"files":rows},ensure_ascii=False,indent=2)+"\n").encode("utf-8"))
 print(json.dumps({"sourceApplied":False,"files":len(rows),"patch":str(PATCH),"maximumUtf16":max(row["utf16Units"]for row in rows)}))
def changed_regions(before,after):
 import difflib
 left=before.splitlines(keepends=True);right=after.splitlines(keepends=True);a=[0];b=[0]
 for line in left:a.append(a[-1]+len(line))
 for line in right:b.append(b[-1]+len(line))
 return [(a[i],a[j],b[k],b[l])for tag,i,j,k,l in difflib.SequenceMatcher(None,left,right,autojunk=False).get_opcodes()if tag!="equal"]
def apply(write=False):
 packet_bytes=PATCH.read_bytes();packet=json.loads(packet_bytes.decode("utf-8"));outputs=[]
 for row in packet["files"]:
  path=ROOT/row["path"];text=read_utf8(path)
  if sha(text.encode())!=row["beforeSha256"]:raise RuntimeError("Changed source: "+row["path"])
  if row["kind"]=="json":
   value=json.loads(text)
   for change in row["changes"]:value=alter(value,change)
   result=dump(value,text)
  else:
   result=text
   for change in reversed(row["changes"]):
    if result[change["start"]:change["end"]]!=change["expected"]:raise RuntimeError("Changed exact source splice: "+row["path"])
    result=result[:change["start"]]+change["replacement"]+result[change["end"]:]
  if sha(result.encode())!=row["afterSha256"]:raise RuntimeError("Proposed bytes mismatch: "+row["path"])
  outputs.append((path,result))
 if not write:
  print(json.dumps({"sourceApplied":False,"guardsValid":True,"files":len(outputs),"patchSha256":sha(packet_bytes)}));return
 for path,result in outputs:
  row=next(row for row in packet["files"]if ROOT/row["path"]==path)
  if sha(path.read_bytes())!=row["beforeSha256"]:raise RuntimeError("Source changed immediately before write: "+row["path"])
  path.write_bytes(result.encode("utf-8"))
 print("Guarded portable source edits applied; no install/setup/retirement/publication was invoked.")
if __name__=="__main__":
 if Path.cwd().resolve()!=ROOT:raise RuntimeError("Exact repository cwd required")
 if sys.argv[1:]==["--prepare"]:prepare();apply(False)
 elif sys.argv[1:]==["--check-prepared"]:apply(False)
 elif sys.argv[1:]==["--apply-after-root-release"]:apply(True)
 else:raise SystemExit("Use --prepare/--check-prepared under normal admission; --apply-after-root-release only after explicit Root release.")
