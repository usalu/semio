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
report="# Cargo Ownership Audit\n\n2026-10-28. Read-only Python tomllib parsing of the bounded layout artifact Cargo tree and root build-owner registry. No compiler, graph or test launched. Checked exact package.workspace resolution, explicit workspace.members membership, unique package names within this artifact tree, one matching build-owner row, library entry existence, every declared workspace dependency alias, and relative dependency Cargo path/package identity for all six new owners.\n\n```json\n"+json.dumps(results,ensure_ascii=False,indent=2)+"\n```\n\nThe managed default Python is3.9without tomllib; the retained audit script ran under the installed Python3.12 through the managed repo tools Python entry.\n"
(ticket/"cargo-ownership-audit.md").write_text(report)
print(json.dumps(results,ensure_ascii=False))
