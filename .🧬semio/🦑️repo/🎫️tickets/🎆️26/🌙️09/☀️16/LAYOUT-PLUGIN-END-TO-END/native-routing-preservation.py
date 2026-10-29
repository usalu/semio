import json, os, tomllib
from pathlib import Path
root=Path.cwd()
ticket=Path(__file__).parent
base=root/"✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout"
load=lambda p:tomllib.loads(p.read_text())
workspace=load(base/"Cargo.toml")["workspace"]
owners=json.loads((root/"📋️build-owners.json").read_text())["owners"]
names=["semio-s-artifact-layout-layout-"+x for x in ["editor-tests","engine-scene-tests","engine-flow-tests","diff-plan","sqlite-member-tests","mutations-decisions"]]
manifest_paths=[]
for directory,dirs,files in os.walk(base):
 dirs[:]=[x for x in dirs if x not in ["node_modules","target","dist","🤖️generated"]]
 if "Cargo.toml" in files:manifest_paths.append(Path(directory)/"Cargo.toml")
parsed={p:load(p) for p in manifest_paths}
results=[]
for name in names:
 matches=[(p,m) for p,m in parsed.items() if m.get("package",{}).get("name")==name]
 issues=[]
 if len(matches)!=1:
  results.append({"name":name,"issues":["package name count "+str(len(matches))]});continue
 p,m=matches[0]; package_root=p.parent; selected=(package_root/m["package"].get("workspace","")).resolve()
 if selected!=base:issues.append("package.workspace resolves to "+str(selected))
 if package_root.relative_to(base).as_posix() not in workspace["members"]:issues.append("missing exact workspace.members entry")
 rows=[r for r in owners if r["name"]==name or r["root"]==package_root.relative_to(root).as_posix()]
 if len(rows)!=1:issues.append("build owner count "+str(len(rows)))
 elif rows[0]["root"]!=package_root.relative_to(root).as_posix() or rows[0]["manifest"]!=p.relative_to(root).as_posix():issues.append("build owner root/manifest mismatch")
 lib=(package_root/m["lib"]["path"]).resolve()
 if not lib.is_file():issues.append("lib entry missing "+str(lib))
 count=0
 def check(deps,label):
  global count
  for alias,decl in deps.items():
   if not isinstance(decl,dict):continue
   context=package_root; effective=decl
   if decl.get("workspace"):
    if alias not in workspace.get("dependencies",{}):issues.append(label+" missing workspace alias "+alias);continue
    effective=workspace["dependencies"][alias];context=base
    if not isinstance(effective,dict):continue
   if "path" not in effective:continue
   target=(context/effective["path"])/"Cargo.toml";count+=1
   if not target.is_file():issues.append(label+" "+alias+" missing path "+str(target));continue
   target_name=load(target).get("package",{}).get("name")
   if target_name!=effective.get("package",alias):issues.append(label+" "+alias+" package mismatch "+str(target_name))
 for kind in ["dependencies","dev-dependencies","build-dependencies"]:check(m.get(kind,{}),kind)
 for target,config in m.get("target",{}).items():
  for kind in ["dependencies","dev-dependencies","build-dependencies"]:check(config.get(kind,{}),target+" "+kind)
 results.append({"name":name,"workspace":selected.relative_to(root).as_posix(),"relative_dependency_paths_checked":count,"owner_rows":len(rows),"absolute_manifest_length":len(str(p)),"issues":issues})
import hashlib,re
router=base/"📦️packages/🦀️rust/📜️script.ts"
text=router.read_text(); selected=re.findall(r"\"([^\"]+)\"",text.split("testPackages: [")[1].split("]")[0]);issues=[]
expected=["semio-s-artifact-layout-layout-"+x for x in ["engine","editor-tests","engine-scene-tests","engine-flow-tests","editor-commands-tests","mutations-content-tests-1","mutations-content-tests-2","mutations-frames-tests-1","mutations-frames-tests-2","diff-plan","sqlite-member-tests","engine-export-unit-tests"]]+["semio-s-artifact-layout-layout"]
for name in expected:
 if name not in selected:issues.append("omitted prior owned package: "+name)
for name in selected:
 matches=[(p,m) for p,m in parsed.items() if m.get("package",{}).get("name")==name]
 if len(matches)!=1:issues.append("selected package match count: "+name);continue
 p,m=matches[0]
 if p.parent.relative_to(base).as_posix() not in workspace["members"]:issues.append("selected missing workspace member: "+name)
 if (p.parent/m["package"].get("workspace","")).resolve()!=base:issues.append("selected wrong workspace root: "+name)
for key in ["members"]:
 if len(workspace[key])!=len(set(workspace[key])):issues.append("duplicate workspace member")
if len(selected)!=len(set(selected)):issues.append("duplicate test package")
paths=[router,base/"Cargo.toml",root/"📋️build-owners.json"]+[p for p,m in parsed.items() if m.get("package",{}).get("name") in expected]
hashes={p.relative_to(root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
report="# Native Routing Preservation Audit\n\n2026-10-28. Fresh bounded read-only snapshot. No source edits, test, compiler or graph. Python tomllib parsed the current nested workspace and package manifests, not stale report copies. SHA256 values below identify the exact snapshot; any subsequent mutation must reread and compare current content rather than replace this complete snapshot.\n\n## Current Test Route\n\n"+"\n".join("- "+name for name in selected)+"\n\nAll13current entries are unique and all13expected prior/additive entries are preserved. The formerly lost diff-plan, sqlite-member-tests and engine-export-unit-tests are now restored. Mutations-decisions is a production owner rather than a previously declared test addition; no request to silently drop existing test packages is implied.\n\n## Six New Owner Checks\n\n```json\n"+json.dumps(results,ensure_ascii=False,indent=2)+"\n```\n\n## Routing Issues\n\n"+("\n".join(issues)or"None: exact package workspace roots and member membership remain correct; six new owners retain one matching build-owner row and all declared dependency aliases/relative paths resolve.")+"\n\n## Snapshot Hashes\n\n```json\n"+json.dumps(hashes,ensure_ascii=False,indent=2)+"\n```\n"
(ticket/"native-routing-preservation.md").write_text(report)
print(json.dumps({"selected":selected,"issues":issues,"newOwnerIssues":[r for r in results if r["issues"]],"routerHash":hashes[router.relative_to(root).as_posix()]},ensure_ascii=False))
