"""Anchored edit of the Cargo planning memo law: a workspace whose members are a glob gains a member without any recorded text moving. Usage: edit_law.py <root> [check|apply]."""
import pathlib,sys
root=pathlib.Path(sys.argv[1]);mode=sys.argv[2] if len(sys.argv)>2 else "apply"
d=root/"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🧪️tests"
law=d/"🟦️.ts";fix=d/"🔣️.json"
L=[
 ('/** 🧠 Contract: Cargo planning serves `cargo metadata` and `--unit-graph` from its memo exactly while nothing Cargo derived them from moved (source text and unrelated files or environment keep it; manifests, lock, auto-discovered targets, build scripts, Cargo-read environment, configuration and features move it), and whatever it serves equals what Cargo itself plans right now.',
  '/** 🧠 Contract: Cargo planning serves `cargo metadata` and `--unit-graph` from its memo exactly while nothing Cargo derived them from moved (source text and unrelated files or environment keep it; manifests, lock, auto-discovered targets, build scripts, Cargo-read environment, configuration and features move it; a workspace whose members Cargo expands from a glob is never remembered, because a new member moves no recorded text and the lock receipt only re-reads the manifests it recorded), and whatever it serves equals what Cargo itself plans right now.'),
 (' registerCargoLockEnsurerV1(async()=>"current");',
  ' registerCargoLockEnsurerV1(async request=>{if(!request.force)return "current";expect(spawnSync("cargo",["generate-lockfile","--offline","--manifest-path",join(request.workspaceRoot,"Cargo.toml")],{env:request.environment,encoding:"utf8"}).status).toBe(0);return "refreshed";});'),
 ('  expect(spawnSync("cargo",["generate-lockfile","--offline","--manifest-path",join(root,"Cargo.toml")],{env:environment,encoding:"utf8"}).status).toBe(0);',
  '  if(row.edit==="glob-member"){put("Cargo.toml",\'[workspace]\\nresolver="2"\\nmembers=["app","core","extra/*"]\\n\');put("extra/one/Cargo.toml",\'[package]\\nname="memo-one"\\nversion="0.1.0"\\nedition="2021"\\n\');put("extra/one/src/lib.rs","pub fn one() -> u8 { 1 }\\n");args=["test","--no-run","--manifest-path",join(root,"Cargo.toml"),"--workspace"];}\n  expect(spawnSync("cargo",["generate-lockfile","--offline","--manifest-path",join(root,"Cargo.toml")],{env:environment,encoding:"utf8"}).status).toBe(0);'),
 ('  if(row.edit==="features")args=[...args,"--features","memo-core/extra"];',
  '  if(row.edit==="features")args=[...args,"--features","memo-core/extra"];\n  if(row.edit==="glob-member"){put("extra/more/Cargo.toml",\'[package]\\nname="memo-more"\\nversion="0.1.0"\\nedition="2021"\\n\');put("extra/more/src/lib.rs","pub fn more() -> u8 { 3 }\\n");}'),
]
F=[('    {"id": "selected features", "edit": "features", "served": false}\n','    {"id": "selected features", "edit": "features", "served": false},\n    {"id": "workspace glob member", "edit": "glob-member", "served": false}\n')]
def run(path,pairs):
  text=path.read_text(encoding="utf8")
  if all(new in text for _,new in pairs):return "applied",None
  for old,new in pairs:
    if text.count(old)!=1:return "anchor lost: "+old[:80],None
    text=text.replace(old,new)
  return "base",text
res=[(p,)+run(p,pairs) for p,pairs in ((law,L),(fix,F))]
for p,s,_ in res:print(s,p.name)
if mode=="check":sys.exit(0)
if any(s not in("base","applied") for _,s,_ in res):sys.exit(1)
for p,s,t in res:
  if t is not None:p.write_text(t,encoding="utf8")
print("done")
