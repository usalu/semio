import { constants, copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, statSync, statfsSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const generated=resolve(import.meta.dirname,"🗑️generated");
const output = resolve(generated,"bitmap-direct-law");
mkdirSync(output, { recursive: true });
const nativeTarget=process.env.SEMIO_BITMAP_DIRECT_NATIVE_TARGET ?? join(generated,"native-mcp");
const build=resolve(nativeTarget,"debug","build");
const registry = Bun.spawnSync(["repo", "tools", "list", "--json"]);
const rustup = JSON.parse(registry.stdout.toString()).find((tool: { id: string }) => tool.id === "rustup").command;
const channel = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/m)![1];
const all = new Map<string,string>();
const plugin: string[] = [];
const native = new Set<string>();
for (const name of readdirSync(build, { withFileTypes: true }).filter(entry => entry.isDirectory())) {
  for (const hash of readdirSync(join(build, name.name))) {
    const unit = join(build, name.name, hash, "out");
    if (!existsSync(unit)) continue;
    if (readdirSync(unit).some(file=>file.endsWith(".a"))) native.add(unit);
    for (const file of readdirSync(unit)) {
      const id = /^lib(.+-[0-9a-f]+)\.(rmeta|dylib|so|dll)$/u.exec(file)?.[1];
      if (!id) continue;
      const path = join(unit, file);
      all.set(id, path);
      if (/^semio_plugin-[a-f0-9]+$/.test(id)) plugin.push(path);
    }
  }
}
const libraries = new Map<string,string>();
const paths = new Set<string>();
const snapshot=join(output,"cache");

const anchors = plugin.sort((a,b) => statSync(b).mtimeMs - statSync(a).mtimeMs);
const critical = new Set(["semio_os_kernel","semio_framework_value","semio_framework_job","semio_framework_job_context","semio_framework_value_native_owned","semio_framework_pack_json"]);
const bin=(file:string)=>Bun.spawnSync([rustup,"run",channel,"rustc","-Zls=root",file]);
const hashOf=(metadata:string)=>metadata.match(/^hash ([a-f0-9]+)/mu)?.[1];
let chosen;
for(const file of anchors){
  const metadata=bin(file);
  if(metadata.exitCode)continue;
  const text=metadata.stdout.toString();
  const edges=[...text.matchAll(/^\d+ (([\w]+)-[a-f0-9]+) hash ([a-f0-9]+)/gmu)];
  const names=new Set(edges.map(row=>row[2]));
  if(![...critical].every(name=>names.has(name)))continue;
  const stale=edges.find(([,id,name,hash])=>{
    if(!critical.has(name))return false;
    const path=all.get(id);
    return !path||hashOf(bin(path).stdout.toString())!==hash;
  });
  if(stale){console.error("[DEBUG] Old native anchor",file.split("/").at(-1),stale[1],"requires",stale[3]);continue}
  chosen={file,text,edges};
  break;
}
if(!chosen)throw Error("No stable native closure for paid bitmap fixture");
const {file:anchor,text:rootMetadata,edges:entries}=chosen;
function clone(source:string,directory:string){
  const basename=source.split("/").at(-1)!;
  const target=join(directory,basename);
  if(!existsSync(target))copyFileSync(source,target,constants.COPYFILE_FICLONE_FORCE);
  return target;
}
const graphHash=hashOf(rootMetadata)!;
const graph=join(snapshot,graphHash.slice(0,16));
const linkRoot=join(output,"r"+graphHash.slice(0,16));
mkdirSync(linkRoot,{recursive:true});
paths.add(linkRoot);
mkdirSync(graph,{recursive:true});
for (const [,id,name] of entries) {
  const file=all.get(id);
  if (!file) continue;
  const target=clone(file,graph);
  const rlib=file.replace(/\.rmeta$/u,".rlib");
  if(rlib!==file&&existsSync(rlib))clone(rlib,graph);
  if (!libraries.has(name)) libraries.set(name,target);
}
paths.add(graph);
const pluginFile=clone(anchor,graph);
if(existsSync(anchor.replace(/\.rmeta$/u,".rlib")))clone(anchor.replace(/\.rmeta$/u,".rlib"),graph);
libraries.set("semio_plugin",pluginFile);
let nativeIndex=0;
const nativeSources=[...native];
native.clear();
for(const source of nativeSources){
  const directory=join(graph,"n"+nativeIndex++);
  mkdirSync(directory,{recursive:true});
  for(const file of readdirSync(source).filter(file=>file.endsWith(".a")))clone(join(source,file),directory);
  native.add(directory);
}
console.error("[DEBUG] Cloned a checked native graph",graphHash,libraries.size,"rust externs and",native.size,"native library groups");

const compiling = new Set<string>();
const testCrate = "semio-s-artifact-wfc-bitmap-inferences";
let testingFacade: string|undefined;
let contextFacade: string|undefined;
const normalized = (name: string) => name.replaceAll("-", "_");
const local = resolve(process.cwd());

function buildMetadata(manifest: string): string {
  const body = readFileSync(manifest, "utf8");
  const pkg = body.match(/^name\s*=\s*"([^"]+)"/m)![1];
  const section = body.split("[lib]")[1]?.split("\n[")[0];
  const crate = section?.match(/^name\s*=\s*"([^"]+)"/m)?.[1] ?? normalized(pkg);
  if (compiling.has(crate)) throw Error("Manifest dependency cycle for " + crate);
  compiling.add(crate);
  const cached = libraries.get(crate);
  if (cached) { compiling.delete(crate); return cached; }
  const directory = dirname(manifest);
  const workspace = body.match(/^workspace\s*=\s*"([^"]+)"/m)?.[1];
  const workspaceDir = workspace ? resolve(directory, workspace) : directory;
  const workspaceBody = readFileSync(join(workspaceDir, "Cargo.toml"), "utf8");
  const regular = body.split("[dependencies]")[1]?.split("\n[")[0] ?? "";
  const dev = pkg===testCrate ? (body.split("[dev-dependencies]")[1]?.split("\n[")[0] ?? "") : "";
  const dependencies = regular+"\n"+dev.split("\n").filter(line=>/^(serde_json|semio-framework-value-clone-source-testing|semio-framework-value-snapshot-projection)\s*=/.test(line)).join("\n");
  const externs = new Map<string,string>();
  for (const [, alias, entry] of dependencies.matchAll(/^([\w-]+)(?:\.workspace)?\s*=\s*(.+)$/gm)) {
    if (/optional\s*=\s*true/u.test(entry)) continue;
    let childDirectory = directory;
    let detail = entry;
    if (entry === "true" || /workspace\s*=\s*true/u.test(entry)) {
      const workspaceDeps = workspaceBody.split("[workspace.dependencies]")[1]?.split("\n[")[0] ?? "";
      const escaped = alias.replaceAll("-", "\\-");
      detail = workspaceDeps.match(new RegExp(`^${escaped}\\s*=\\s*(.+)$`, "m"))?.[1]
        ?? workspaceBody.split(`[workspace.dependencies.${alias}]`)[1]?.split("\n[")[0]
        ?? entry;
      childDirectory = workspaceDir;
    }
    const packageId = detail.match(/\bpackage\s*=\s*"([^"]+)"/u)?.[1] ?? alias;
    const name = normalized(packageId);
    const path = detail.match(/\bpath\s*=\s*"([^"]+)"/u)?.[1];
    const child = path ? resolve(childDirectory, path, "Cargo.toml") : undefined;
    const native = libraries.get(name);
    const file = native ?? (child?.startsWith(local) && existsSync(child) ? buildMetadata(child) : undefined);
    if (file) externs.set(normalized(alias), file);
  }
  if (pkg==="semio-framework-job-testing-caller" && contextFacade) externs.set("semio_framework_job_context",contextFacade);
  if(pkg===testCrate && testingFacade) externs.set("semio_framework_job",testingFacade);
  const entry = resolve(directory, section?.match(/^path\s*=\s*"([^"]+)"/m)?.[1] ?? "src/lib.rs");
  const metadata = join(linkRoot, pkg===testCrate ? "bitmap-paid-route" : `lib${crate}.rlib`);
  if(pkg!==testCrate && existsSync(metadata) && statSync(metadata).mtimeMs >= statSync(entry).mtimeMs) {compiling.delete(crate);libraries.set(crate,metadata);return metadata;}
  const rsp = join(linkRoot, `${crate}.args`);
  const comp = [entry, "--crate-name", crate, ...(pkg===testCrate ? ["--test"] : ["--crate-type=rlib"]), "--edition=2021", "--emit=link", "-Cmetadata=bitmap_direct", "-Copt-level=1", "-Zthreads=1", "-o", metadata];
  const storage=statfsSync(local);if(storage.bavail*storage.bsize<7*2**30)throw Error("Native direct law reached the workspace disk reserve");
  for(const path of native)comp.push("-L",`native=${path}`);
  for (const path of paths) comp.push("-L", `dependency=${path}`);
  for (const [alias,file] of externs) {
    comp.push("--extern", `${alias}=${file}`);
    const library=file.replace(/\.rmeta$/u,".rlib");
    if (library!==file && existsSync(library)) comp.push("--extern", `${alias}=${library}`);
  }
  writeFileSync(rsp, comp.join("\n")+"\n");
  console.error(`[DEBUG] Selective native build of ${pkg} with ${externs.size} direct dependencies`);
  const child = Bun.spawnSync([rustup, "run", channel, "rustc", `@${rsp}`], {
    stdout: "inherit", stderr: "inherit", env: { ...process.env, CARGO_MANIFEST_DIR: directory, CARGO_PKG_NAME: pkg, CARGO_PKG_VERSION: "0.1.0", SEMIO_BUILD_MODE: "prototype" },
  });
  if (child.exitCode) throw Error("Pinned bitmap metadata failed for " + pkg);
  compiling.delete(crate);
  libraries.set(crate, metadata);
  return metadata;
}

const bitmap = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";


function compileGlue(name:string, source:string, externs:Map<string,string>){
   const lib=join(linkRoot,"lib"+name+".rlib");
   const root=join(linkRoot,name+".rs");
   if(existsSync(lib)&&existsSync(root)&&readFileSync(root,"utf8")===source+"\n")return lib;
   writeFileSync(root,source+"\n");
   const args=[root,"--crate-name",name,"--edition=2021","--crate-type=rlib","-o",lib];
   for(const [alias,file] of externs){args.push("--extern",alias+"="+file);const rlib=file.replace(/\.rmeta$/u,".rlib");if(rlib!==file&&existsSync(rlib))args.push("--extern",alias+"="+rlib);}
   for(const path of paths)args.push("-L","dependency="+path);
   const response=join(linkRoot,name+".args");
   writeFileSync(response,args.join("\n")+"\n");
   const ready=Bun.spawnSync([rustup,"run",channel,"rustc","@"+response],{stdout:"inherit",stderr:"inherit"});
   if(ready.exitCode)process.exit(ready.exitCode);
   return lib;
}
const actualContext = libraries.get("semio_framework_job_context");
if (!actualContext) throw Error("Missing actual job context");
contextFacade=compileGlue("bitmap_test_context",
 "pub use semio_framework_job_context::*; pub mod testkit { pub fn take_issued_event_stock(_:u64)->Option<super::RetainedCloneGrant>{panic!(\"fixture called an unused initiating-event method\")} }",
 new Map([["semio_framework_job_context",actualContext]]));
const caller = buildMetadata(resolve("🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/📦️packages/🦀️rust/Cargo.toml"));
const actualJob = libraries.get("semio_framework_job");
if (!actualJob) throw Error("Missing actual job facade");
testingFacade=compileGlue("bitmap_test_job",
 "pub use semio_framework_job::*; pub use semio_framework_job_testing_caller::OriginalJobFixtureCaller;",
 new Map([["semio_framework_job",actualJob],["semio_framework_job_testing_caller",caller]]));
const law=buildMetadata(resolve(bitmap,"💡️inferences/📦️packages/🦀️rust/Cargo.toml"));
const run=Bun.spawnSync([law,process.argv[2]??"paid_bitmap_route_solves_the_pinned_document_and_closes_its_sources","--nocapture"],{stdout:"inherit",stderr:"inherit"});
process.exit(run.exitCode);

