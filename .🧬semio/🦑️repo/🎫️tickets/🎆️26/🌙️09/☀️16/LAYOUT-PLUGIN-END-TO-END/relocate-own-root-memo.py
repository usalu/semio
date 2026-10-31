from pathlib import Path
import hashlib,json,os,stat
root=Path.cwd()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
entry=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/🗑️generated/neutral-manager/seal-wsracY/tool/drift.ts"
expected="70262955e53136c7ad4c1ffe42d48430771e85d5c493073f847e3188a5126788"
source=root/".memo.json"
def stamp(p):
 s=p.lstat()
 return dict(dev=s.st_dev,ino=s.st_ino,mode=s.st_mode,nlink=s.st_nlink,size=s.st_size,mtime_ns=s.st_mtime_ns,ctime_ns=s.st_ctime_ns)
before=stamp(source)
assert stat.S_ISREG(before["mode"]) and before["nlink"]==1 and before["size"]<=65536
body=source.read_bytes()
assert stamp(source)==before and hashlib.sha256(body).hexdigest()==expected
memo=json.loads(body)
assert set(memo)=={"version","installed","implementation","root","entry","directories","observations","manifest","marks","imports"}
assert memo["version"]==1 and memo["installed"] is True and memo["root"]==str(root) and memo["entry"]==entry and memo["marks"]=={}
manifest=memo["manifest"]
assert set(manifest)=={"version","mode","entry","resolutions","sources"} and manifest["version"]==1 and manifest["mode"]=="release" and manifest["entry"]==entry and manifest["resolutions"]==[]
def relative(p):
 return isinstance(p,str) and not p.startswith("/") and "\\" not in p and all(x not in {"",".",".."} for x in p.split("/"))
paths=[]
for row in manifest["sources"]:
 assert set(row)=={"path","sha256"} and relative(row["path"]) and len(row["sha256"])==64 and all(c in "0123456789abcdef" for c in row["sha256"])
 paths.append(row["path"])
assert len(paths)==len(set(paths)) and set(memo["imports"])==set(paths)
private=entry.rsplit("/",1)[0]+"/"
assert {x for x in paths if x.startswith(".🧬semio/")}=={entry,private+"helper.ts"}
assert all(x.startswith("🧰️framework/") or x.startswith(private) for x in paths)
reached=set()
pending=[entry]
while pending:
 current=pending.pop()
 if current in reached:continue
 reached.add(current)
 imports=memo["imports"][current]
 assert isinstance(imports,dict) and all(isinstance(k,str) and v in paths for k,v in imports.items())
 pending.extend(imports.values())
companions=set(paths)-reached
assert all("/🧬️schema/" in x and any(y.startswith(x.split("/🧬️schema/")[0]+"/") for y in reached) for x in companions)
parents={""}
for path in paths:
 parts=path.split("/")
 parents.update("/".join(parts[:n]) for n in range(1,len(parts)))
for directory in memo["directories"]:
 assert set(directory)=={"path","dev","ino"} and directory["path"] in parents and directory["dev"].isdigit() and directory["ino"].isdigit()
schema_roots={(path.rsplit("/",1)[0]+"/" if "/" in path else "")+"🧬️schema" for path in paths}
observed_directories={row["path"]:row["entries"] for row in memo["observations"] if row["kind"]=="directory"}
schema_namespaces=set(schema_roots)
pending=list(schema_roots)
while pending:
 current=pending.pop()
 for name,kind in observed_directories.get(current,[]):
  assert isinstance(name,str) and name not in {"",".",".."} and "/" not in name and "\\" not in name
  if kind=="d":
   child=current+"/"+name
   if child not in schema_namespaces:schema_namespaces.add(child);pending.append(child)
for observation in memo["observations"]:
 kind=observation["kind"]
 if kind=="package":assert set(observation)=={"kind"}
 else:
  assert kind in {"absent","directory"} and relative(observation["path"])
  assert observation["path"] in parents or observation["path"] in schema_namespaces
  assert set(observation)==({"kind","path","entries"} if kind=="directory" else {"kind","path"})
  if kind=="directory":
   assert all(isinstance(row,list) and len(row)==2 and isinstance(row[0],str) and row[1] in {"d","f"} for row in observation["entries"])
   folder=root/observation["path"]
   assert folder.is_dir() and not folder.is_symlink()
   current=[]
   for child in folder.iterdir():
    info=child.lstat()
    assert not stat.S_ISLNK(info.st_mode) and (stat.S_ISDIR(info.st_mode) or stat.S_ISREG(info.st_mode))
    current.append([child.name,"d" if stat.S_ISDIR(info.st_mode) else "f"])
   assert sorted(current)==sorted(observation["entries"])
  else:
   try:(root/observation["path"]).lstat()
   except FileNotFoundError:pass
   else:raise AssertionError("Recorded absence changed")
generated=ticket/"🗑️generated"
backup=generated/"root-memo-66533.exact.json"
metadata=generated/"root-memo-66533.stat.json"
destination=generated/"root-memo-66533.moved.json"
journal=ticket/"root-memo-66533.md"
assert not any(p.exists() for p in [backup,metadata,destination,journal])
proof={"source":str(source),"entry":entry,"sha256":expected,"before":before,"sourceCount":len(paths),"reachable":len(reached),"schemaCompanions":sorted(companions),"initialPathExistence":"unknown before this task","priorStateRestored":False,"destination":str(destination)}
with backup.open("xb") as f:f.write(body);f.flush();os.fsync(f.fileno())
with metadata.open("x") as f:json.dump(proof,f,ensure_ascii=False,indent=2);f.write("\n");f.flush();os.fsync(f.fileno())
with journal.open("x") as f:f.write("# Owned Root Memo Relocation\n\nThe entire bounded current closed capture record identifies only normal66533 ticket-generated entry "+entry+". Its regular single-link identity, exact bytes, SHA-256 and stat are preserved under generated. Root pathname preexistence is unknown; no prior contents or restoration are claimed. No shared-store, receipt or lease operation is involved. The relocation uses a no-overwrite hardlink and same-record checks before removing only the original scratch name.\n")
assert stamp(source)==before and hashlib.sha256(source.read_bytes()).hexdigest()==expected
os.link(source,destination,follow_symlinks=False)
linked=stamp(source)
assert all(linked[k]==before[k] for k in ["dev","ino","mode","size","mtime_ns"]) and linked["nlink"]==2
assert stamp(destination)==linked and hashlib.sha256(source.read_bytes()).hexdigest()==expected
assert stamp(source)==linked
source.unlink()
after=stamp(destination)
assert after["dev"]==before["dev"] and after["ino"]==before["ino"] and after["nlink"]==1 and not source.exists() and hashlib.sha256(destination.read_bytes()).hexdigest()==expected
with journal.open("a") as f:f.write("\nCompleted: current owned generated record moved without destination overwrite; exact backup and original stat remain available. This is reversible by a separately authorized own-file restoration if ever needed, and is not restoration of unknown prior state.\n")
print("owned current memo moved; source absent; exact backup/stat retained; no prior-state restoration claim")
