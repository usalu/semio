from pathlib import Path
import re,json,hashlib
root=Path.cwd().resolve()
ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
start=root/"🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml"
workspace=root/"🧰️framework/Cargo.toml"
workspace_text=workspace.read_text()
dependency_pattern=re.compile(r'([A-Za-z0-9_-]+)\s*=\s*\{([^}]+)\}',re.S)
workspace_paths={}
for name,body in dependency_pattern.findall(workspace_text):
 match=re.search(r'\bpath\s*=\s*"([^"]+)"',body)
 if match:workspace_paths[name]=(workspace.parent/match.group(1)/"Cargo.toml").resolve()
pending=[start];manifests={};external=set();sources=set();missing=[]
while pending:
 path=pending.pop().resolve()
 if path in manifests:continue
 text=path.read_text();manifests[path]=text
 section="";sections={}
 for line in text.splitlines():
  header=re.match(r'\s*\[([^\]]+)\]',line)
  if header:section=header.group(1);sections.setdefault(section,[])
  else:sections.setdefault(section,[]).append(line)
 selected=[]
 for section,lines in sections.items():
  if section=="dependencies" or section=="build-dependencies" or section.endswith(".dependencies") or section.endswith(".build-dependencies") or (path==start and (section=="dev-dependencies" or section.endswith(".dev-dependencies"))):
   selected.append("\n".join(lines))
 for block in selected:
  for name,body in dependency_pattern.findall(block):
   match=re.search(r'\bpath\s*=\s*"([^"]+)"',body)
   dependency=(path.parent/match.group(1)/"Cargo.toml").resolve()if match else workspace_paths.get(name)if re.search(r'workspace\s*=\s*true',body)else None
   if dependency is not None:
    if dependency.exists():pending.append(dependency)
    else:missing.append(str(dependency.relative_to(root)))
   else:external.add(name)
  for name in re.findall(r'^([A-Za-z0-9_-]+)\s*=\s*"[^"]+"',block,re.M):external.add(name)
 lib="\n".join(sections.get("lib",[]));match=re.search(r'\bpath\s*=\s*"([^"]+)"',lib)
 candidate=(path.parent/match.group(1)).resolve()if match else path.parent/"src/lib.rs"
 if candidate.exists():sources.add(candidate)
 build=re.search(r'^build\s*=\s*"([^"]+)"',text,re.M)
 candidate=(path.parent/build.group(1)).resolve()if build else path.parent/"build.rs"
 if candidate.exists():sources.add(candidate)
seen=set();inputs=set(manifests)|{workspace};pending=list(sources);unresolved_modules=[]
path_pattern=re.compile(r'#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]')
include_pattern=re.compile(r'(?:include_str|include_bytes|include|fixture_text|fixture_path)\s*!\s*\(\s*"([^"]+)"')
while pending:
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
rows=[]
for path in sorted(inputs):
 data=path.read_bytes();rows.append({"path":str(path.relative_to(root)),"bytes":len(data),"sha256":hashlib.sha256(data).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2})
packet={"state":"source-only-prepared-unexecuted","selectedPackage":"semio-framework-value-persistent","selectedDoor":"--lib","rootManifest":str(start.relative_to(root)),"workspaceManifest":str(workspace.relative_to(root)),"inputs":rows,"firstPartyManifests":len(manifests),"rustSourceFiles":len(seen),"externalDependencyNames":sorted(external),"missingManifestPaths":missing,"limitations":["Conservative literal module/include walk includes cfg-gated source; it is not the canonical compiler Source admission plan.","Proc-macro emitted and computed include/source paths are not independently resolved here; Root must use canonical selected Source admission.","External registry/toolchain content is owned by canonical repository dependency/image receipts; workspace locks are captured, no cache/source copy is made."]}
text=json.dumps(packet,ensure_ascii=False,indent=2)+"\n"
(ticket/"drawing-native-primitives-prep-inputs.json").write_text(text)
(ticket/"drawing-native-primitives-prep.md").write_text("# Isolated Native Primitive Source Preparation\n\nPREP ONLY. No Cargo/test/compiler/metadata/setup/sync/runtime action was launched. Root owns any exact canonical command and Source admission.\n\nSelected actual owning test door: semio-framework-value-persistent --lib, including moved primitive heap laws and the new nested contiguous depth denial/admitted-copy law. The current package library root exports the defining work module and its cfg(test) module; the native test file is not merely compiled as a dependency. Native work root/tests and actual selected lower implementation can freeze while Drawing/2d consumer Source continues.\n\nPacket SHA "+hashlib.sha256(text.encode()).hexdigest()+". Source-only walker captured "+str(len(rows))+" exact current inputs, "+str(len(manifests))+" first-party Cargo manifests and "+str(len(seen))+" Rust files, with workspace manifest/lock when present. Missing dependency paths: "+str(len(missing))+". All file bytes/hashes and physical UTF16 path units are explicit.\n\nThis is conservative literal Source/manifest evidence with the limits recorded in the packet. It does not replace canonical selected Source-bound fit/admission or prove that proc-macro/computed inputs have been exhaustively found. External registry/toolchain custody stays with normal repository receipts. No bypass, byte-budget manufacture, compatibility alias or new runtime library was added.\n\nRust native nested-slot depth repair is not covered by the actual scoped6pass/81assertion TypeScript result. That actual green remains retained separately. This preparation does not establish full Drawing, native composition, Layout19 or browser/publication completion.\n")
print(hashlib.sha256(text.encode()).hexdigest(),len(rows),len(manifests),len(seen),len(missing))
