from pathlib import Path
import re,json,hashlib
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
start=root/"🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml"
workspace=root/"🧰️framework/Cargo.toml"
limits={"maximumManifests":128,"maximumRustSources":2048,"maximumInputs":4096,"maximumFileBytes":2097152,"maximumTotalBytes":33554432}
metadata_path=ticket/"🗑️generated/drawing-native-primitives-2/cargo-metadata.json"
metadata_bytes=metadata_path.read_bytes();metadata=json.loads(metadata_bytes)
if metadata["selection"]!={"manifest":str(start.relative_to(root)),"door":"--lib"}:raise RuntimeError("Metadata selection mismatch")
if hashlib.sha256(workspace.read_bytes()).hexdigest()!=metadata["workspace"]["sha256"]:raise RuntimeError("Workspace drift after TOML metadata parse")
manifests={}
for row in metadata["manifests"]:
 path=root/row["path"];raw=path.read_bytes()
 if hashlib.sha256(raw).hexdigest()!=row["sha256"]:raise RuntimeError("Manifest drift after TOML metadata parse: "+row["path"])
 manifests[path.resolve()]=raw.decode("utf8")
if len(manifests)>limits["maximumManifests"]:raise RuntimeError("Source collector manifest bound reached")
external={edge["alias"]for edge in metadata["edges"]if edge["manifest"]is None}
sources={root/path for path in metadata["sourceRoots"]};missing=metadata["missingManifestPaths"]
seen=set();inputs=set(manifests)|{workspace};pending=list(sources);unresolved_modules=[]
path_pattern=re.compile(r'#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]')
include_pattern=re.compile(r'(?:include_str|include_bytes|include|fixture_text|fixture_path)\s*!\s*\(\s*"([^"]+)"')
while pending:
 if len(seen)>=limits["maximumRustSources"]:raise RuntimeError("Source collector Rust bound reached")
 path=pending.pop().resolve()
 if path in seen:continue
 seen.add(path);inputs.add(path)
 text=path.read_text()
 for name in path_pattern.findall(text):
  candidate=(path.parent/name).resolve()
  if candidate.is_file():
   if candidate.suffix==".rs":pending.append(candidate)
   else:inputs.add(candidate)
 for name in include_pattern.findall(text):
  candidate=(path.parent/name).resolve()
  if candidate.is_file():
   if candidate.suffix==".rs":pending.append(candidate)
   else:inputs.add(candidate)
 for name in re.findall(r'\bmod\s+([A-Za-z_]\w*)\s*;',text):
  for candidate in [path.parent/(name+".rs"),path.parent/name/"mod.rs"]:
   if candidate.is_file():pending.append(candidate)
for path in [workspace.parent/"Cargo.lock",root/"Cargo.lock",root/"rust-toolchain.toml"]:
 if path.is_file():inputs.add(path)
if len(inputs)>limits["maximumInputs"]:raise RuntimeError("Source collector input bound reached")
if any(p.stat().st_size>limits["maximumFileBytes"]for p in inputs):raise RuntimeError("Source collector file byte bound reached")
if sum(p.stat().st_size for p in inputs)>limits["maximumTotalBytes"]:raise RuntimeError("Source collector total byte bound reached")
rows=[]
for path in sorted(inputs):
 data=path.read_bytes();rows.append({"path":str(path.relative_to(root)),"bytes":len(data),"sha256":hashlib.sha256(data).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2})
packet={"dependencyMetadata":{"path":str(metadata_path.relative_to(root)),"sha256":hashlib.sha256(metadata_bytes).hexdigest(),"edges":metadata["edges"],"manifests":metadata["manifests"],"limitations":metadata["limitations"]},"collectorDialect":"registered Bun.TOML metadata and literal source module/include walk","collectorLimits":limits,"state":"source-only-prepared-unexecuted","selectedPackage":"semio-framework-value-persistent","selectedDoor":"--lib","rootManifest":str(start.relative_to(root)),"workspaceManifest":str(workspace.relative_to(root)),"inputs":rows,"firstPartyManifests":len(manifests),"rustSourceFiles":len(seen),"externalDependencyNames":sorted(external),"missingManifestPaths":missing,"limitations":["Conservative literal module/include walk includes cfg-gated source; it is not the canonical compiler Source admission plan.","Proc-macro emitted and computed include/source paths are not independently resolved here; Root must use canonical selected Source admission.","External registry/toolchain content is owned by canonical repository dependency/image receipts; workspace locks are captured, no cache/source copy is made."]}
text=json.dumps(packet,ensure_ascii=False,indent=2)+"\n"
(ticket/"drawing-native-primitives-2-inputs.json").write_text(text)
(ticket/"drawing-native-primitives-2-prep.md").write_text("# Isolated Native Primitive Source Preparation 2\n\nPREP ONLY. No Cargo/test/compiler/metadata/setup/sync/runtime action was launched. Root owns any exact canonical command and Source admission.\n\nSelected actual owning test door: semio-framework-value-persistent --lib, including the primitive heap laws and new nested absolute cached-demand/held-copy repeated-denial law. The current package library root exports the defining work module and its cfg(test) module; the native test file is not merely compiled as a dependency. Native work root/tests and actual selected lower implementation can freeze while Drawing/2d consumer Source continues.\n\nPacket SHA "+hashlib.sha256(text.encode()).hexdigest()+". Source-only walker captured "+str(len(rows))+" exact current inputs, "+str(len(manifests))+" first-party Cargo manifests and "+str(len(seen))+" Rust files, with workspace manifest/lock when present. Missing dependency paths: "+str(len(missing))+". All file bytes/hashes and physical UTF16 path units are explicit.\n\nThis is conservative literal Source/manifest evidence with the limits recorded in the packet. It does not replace canonical selected Source-bound fit/admission or prove that proc-macro/computed inputs have been exhaustively found. External registry/toolchain custody stays with normal repository receipts. No bypass, byte-budget manufacture, compatibility alias or new runtime library was added.\n\nThe prior actual native29 proof and prior TypeScript proofs remain historical under their recorded scopes. This snapshot includes newer native absolute-demand Source and does not inherit those passes. This preparation does not establish full Drawing, native composition, Layout19 or browser/publication completion.\n")
print(hashlib.sha256(text.encode()).hexdigest(),len(rows),len(manifests),len(seen),len(missing))
