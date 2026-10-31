from pathlib import Path
import re,json,hashlib
root=Path.cwd().resolve();ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END";area=ticket/"🗑️generated/drawing-job-context-owning-6";metadata_path=area/"cargo-metadata.json";raw=metadata_path.read_bytes();metadata=json.loads(raw)
limits={"maximumManifests":128,"maximumRustSources":2048,"maximumInputs":4096,"maximumFileBytes":2097152,"maximumTotalBytes":33554432}
manifest_rows=metadata["manifests"]
if len(manifest_rows)>limits["maximumManifests"]:raise RuntimeError("Metadata manifest count exceeded")
workspace=root/metadata["workspace"]["path"]
if hashlib.sha256(workspace.read_bytes()).hexdigest()!=metadata["workspace"]["sha256"]:raise RuntimeError("Workspace changed after table parse")
inputs={workspace};queue=[];seen=set();rust=set();unresolved=[]
for row in metadata.get("workspaces",[]):
 p=root/row["path"];assert hashlib.sha256(p.read_bytes()).hexdigest()==row["sha256"],row["path"];inputs.add(p)
for row in manifest_rows:
 p=root/row["path"]
 if hashlib.sha256(p.read_bytes()).hexdigest()!=row["sha256"]:raise RuntimeError("Manifest changed after table parse: "+row["path"])
 inputs.add(p)
 for source in row["sourceRoots"]:queue.append((root/source,p.parent))
while queue:
 p,manifest_dir=queue.pop();p=p.resolve();key=(p,manifest_dir)
 if key in seen:continue
 seen.add(key);rust.add(p);inputs.add(p)
 if len(rust)>limits["maximumRustSources"]:raise RuntimeError("Bounded literal Rust count exceeded")
 text=p.read_text()
 for spec in re.findall(r'#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]',text):
  target=(p.parent/spec).resolve()
  if target.is_file():
   if target.suffix==".rs":queue.append((target,manifest_dir))
   else:inputs.add(target)
  elif not target.is_dir():unresolved.append({"source":str(p.relative_to(root)),"kind":"path","specifier":spec,"manifest":str(manifest_dir.relative_to(root))})
 for kind,spec in re.findall(r'\b(include_str|include_bytes|include|fixture_text|fixture_bytes|fixture_path|repo_text|repo_bytes)\s*!\s*\(\s*"([^"]+)"',text):
  base=manifest_dir if kind.startswith("fixture_")else root if kind.startswith("repo_")else p.parent
  target=(base/spec).resolve()
  if target.is_file():
   if target.suffix==".rs":queue.append((target,manifest_dir))
   else:inputs.add(target)
  elif not target.is_dir():unresolved.append({"source":str(p.relative_to(root)),"kind":kind,"specifier":spec,"manifest":str(manifest_dir.relative_to(root))})
 for name in re.findall(r'\bmod\s+([A-Za-z_]\w*)\s*;',text):
  for target in [p.parent/(name+".rs"),p.parent/name/"mod.rs"]:
   if target.is_file():queue.append((target,manifest_dir))
for p in [root/"Cargo.toml",root/"Cargo.lock",workspace.parent/"Cargo.lock",root/"rust-toolchain.toml"]:
 if p.is_file():inputs.add(p)
if len(inputs)>limits["maximumInputs"]:raise RuntimeError("Bounded literal input count exceeded")
rows=[];total=0
for p in sorted(inputs):
 if not p.is_relative_to(root):raise RuntimeError("Outside-repository literal input")
 raw=p.read_bytes();total+=len(raw)
 if len(raw)>limits["maximumFileBytes"]or total>limits["maximumTotalBytes"]:raise RuntimeError("Bounded literal byte extent exceeded")
 units=len(str(p).encode("utf-16-le"))//2
 if units>256:raise RuntimeError("Input path exceeds256UTF16")
 rows.append({"path":str(p.relative_to(root)),"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"utf16PathUnits":units})
packet={"state":"bounded-metadata-snapshot-only-not-source-fit-or-complete-compiler-input-census","selectedPackage":"semio-framework-job-context","selectedDoor":"--all-targets","rootManifest":metadata["selection"]["manifest"],"workspaceManifest":metadata["workspace"]["path"],"dependencyMetadata":{"path":str(metadata_path.relative_to(root)),"sha256":hashlib.sha256(metadata_path.read_bytes()).hexdigest()},"collectorLimits":limits,"firstPartyManifests":len(manifest_rows),"rustSourceFiles":len(rust),"inputs":rows,"unresolvedLiteralObservations":unresolved,"limitations":["All declared cfg/optional path dependencies and source branches are conservatively observed, not resolved compiler features.","Only selected-root dev dependencies are selected; dependency package test modules are conservatively extra source observations, not executed suites.","Literal fixture macros resolve relative to their actual owning Cargo manifest; literal include macros resolve relative to their source file.","Inline module directory contexts, computed paths/proc-macro output and registry/git/patch resolution remain owned by canonical Cargo Source/dependency admission; unresolved literals are explicit, never silently counted resolved."]}
out=ticket/"drawing-job-context-owning-6-inputs.json";out.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n")
print(json.dumps({"packetSha256":hashlib.sha256(out.read_bytes()).hexdigest(),"inputs":len(rows),"manifests":len(manifest_rows),"rust":len(rust),"bytes":total,"unresolved":len(unresolved),"maximumPath":max(row["utf16PathUnits"]for row in rows)}))
