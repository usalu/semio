import {createHash} from "node:crypto";
import {admitCargoPreparationObservationV1,assertCargoPreparationObservationCurrentV1,cargoPreparationProgramSourcesV1,type CargoPreparationObservationV1} from "./🛠️preparation/🧾️custody/🟦️.ts";
import invocationSchema from "./🧬️schema/🏃️invocation/🔣️.json";
import { existsSync, lstatSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";

export type CargoWorkspaceContributionV1 = Readonly<{ schemaVersion: 1; members: readonly string[]; exclude: readonly string[] }>;
export type CargoWorkspaceScope = Readonly<{ directory: string; manifest: string; lock: string; contribution: CargoWorkspaceContributionV1; memberManifests: readonly string[] }>;
export type CargoWorkspacePackage = Readonly<{ directory: string; manifest: string; name: string; workspace: string }>;
const slash = (value: string): string => value.replaceAll("\\", "/");
const pathPattern = (value: unknown): value is string => typeof value === "string" && value.length > 0 && !isAbsolute(value) && !/^[A-Za-z]:\//u.test(value) && !value.includes("\\") && !value.split("/").includes("..");
const object = (value: unknown): Record<string, any> => { if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Cargo workspace requires an object"); return value as Record<string, any>; };

/** 🧬️ Admits the language-neutral physical membership contract without optional owner identities. */
export function parseCargoWorkspaceContribution(value: unknown): CargoWorkspaceContributionV1 {
  const row = object(value);
  if (Object.keys(row).some(k => !["schemaVersion", "members", "exclude"].includes(k)) || row.schemaVersion !== 1 || !Array.isArray(row.members) || !row.members.length || !Array.isArray(row.exclude)) throw new Error("Invalid Cargo workspace contribution");
  for (const values of [row.members, row.exclude]) if (values.some(v => !pathPattern(v)) || new Set(values).size !== values.length) throw new Error("Invalid Cargo workspace member patterns");
  return { schemaVersion: 1, members: [...row.members], exclude: [...row.exclude] };
}

/** 📁️ Refuses escaped or symlink-backed manifest ancestry before reading owner authority. */
function physical(root: string, path: string): string {
  const full = resolve(root, path), rel = slash(relative(root, full));
  if (rel.startsWith("../") || isAbsolute(rel)) throw new Error(`Cargo workspace path escapes repository: ${path}`);
  let current = resolve(root);
  for (const part of rel.split("/").filter(Boolean)) { current = join(current, part); if (lstatSync(current).isSymbolicLink()) throw new Error(`Cargo workspace path contains a symlink: ${path}`); }
  if (!lstatSync(full).isFile()) throw new Error(`Cargo manifest is not a regular file: ${path}`);
  return full;
}
const read = (root: string, path: string): Record<string, any> => object(Bun.TOML.parse(readFileSync(physical(root, path), "utf8")));
const scope = (directory: string, workspace: Record<string, any>): CargoWorkspaceScope => {
  const admission = workspace.metadata?.semio?.repository;
  if (admission !== undefined && (Object.keys(object(admission)).some(k => !["schema-version", "owner-manifests", "member-manifests", "exclude-patterns"].includes(k)) || admission["schema-version"] !== 1)) throw new Error(`Unknown repository workspace admission at ${directory}`);
  const memberManifests = admission?.["member-manifests"];
  if (!Array.isArray(memberManifests) || !memberManifests.length || memberManifests.some(p => !pathPattern(p) || !p.endsWith("Cargo.toml")) || new Set(memberManifests).size !== memberManifests.length) throw new Error(`Invalid native member manifest authority: ${directory}`);
  const prefix = directory === "." ? "" : `${directory}/`;
  return { directory, manifest: `${prefix}Cargo.toml`, lock: `${prefix}Cargo.lock`, memberManifests, contribution: parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members, exclude: admission["exclude-patterns"] }) };
};

/** 🗂️ Discovers only present, explicitly authored repository workspaces through physical Cargo documents. */
export function discoverCargoWorkspaces(root: string): readonly CargoWorkspaceScope[] {
  const rootDocument = read(root, "Cargo.toml");
  const patterns = rootDocument.workspace?.metadata?.semio?.repository?.["owner-manifests"] ?? [];
  if (!Array.isArray(patterns) || patterns.some(p => !pathPattern(p) || !p.endsWith("Cargo.toml")) || new Set(patterns).size !== patterns.length) throw new Error("Invalid repository workspace owner patterns");
  const files = ["Cargo.toml"], matchers=patterns.map(pattern=>new Bun.Glob(pattern)), opaque=new Set(["node_modules","target","dist","build","🤖️generated","🗑️generated","coverage","temp","compose"]);
  const prefixes=patterns.map(pattern=>pattern.split(/[*?\[{]/u)[0]!.replace(/\/$/u,""));
  const walk=(directory:string):void=>{
    if(directory && !prefixes.some(prefix=>!prefix || directory===prefix || directory.startsWith(prefix+"/") || prefix.startsWith(directory+"/")))return;
    const depth=directory?directory.split("/").length:0;
    if(!patterns.some(pattern=>pattern.split("/").includes("**") || depth<pattern.split("/").length))return;
    for(const entry of readdirSync(join(root,directory),{withFileTypes:true})){
      if(entry.name.startsWith(".") || opaque.has(entry.name) || entry.isSymbolicLink())continue;
      const path=directory?directory+"/"+entry.name:entry.name;
      if(entry.isDirectory())walk(path);
      else if(entry.isFile() && entry.name==="Cargo.toml" && matchers.some(pattern=>pattern.match(path)))files.push(path);
    }
  };
  walk("");
  const found: CargoWorkspaceScope[] = [];
  for (const path of files.sort()) {
    if (path.split("/").some(p => p.startsWith(".") || ["node_modules", "target", "dist", "build", "🤖️generated", "coverage", "temp", "compose"].includes(p))) continue;
    const document = read(root, path);
    if (path === "Cargo.toml" || document.workspace?.metadata?.semio?.repository !== undefined) found.push(scope(slash(dirname(path)), object(document.workspace)));
  }
  if (!found.some(row => row.directory === ".")) throw new Error("Root Cargo workspace lacks authored repository admission");
  return found.sort((a, b) => a.manifest.localeCompare(b.manifest));
}

/** 🦀️ Expands each admitted Cargo-native pattern against current physical manifests and rejects absent exact owners. */
export function cargoWorkspaceMembers(root: string, owner: CargoWorkspaceScope): readonly CargoWorkspacePackage[] {
  const documents = new Map<string, Record<string, any>>(), scopes = new Map<string, CargoWorkspaceScope>();
  const readDocument = (path: string): Record<string, any> => { const current = documents.get(path); if (current) return current; const document = read(root, path); documents.set(path, document); return document; };
  const cwd = resolve(root, owner.directory), paths = new Set<string>();
  const patterns = owner.memberManifests.map(pattern => new Bun.Glob(pattern));
  const leaves = owner.memberManifests.map(pattern => new Bun.Glob(pattern.slice(0, -"/Cargo.toml".length)));
  const prefixes = owner.memberManifests.map(pattern => pattern.split(/[*?\[{]/u)[0]!.replace(/\/$/u, ""));
  const excluded = owner.contribution.exclude.map(pattern => new Bun.Glob(pattern));
  const excludedRoots = owner.contribution.exclude.map(pattern => pattern.endsWith("/**") ? new Bun.Glob(pattern.slice(0, -3)) : undefined);
  const opaque = new Set(["node_modules", "target", "dist", "build", "🤖️generated", "coverage", "🗑️generated"]);
  const walk = (directory: string): void => {
    if (directory && !prefixes.some(prefix => !prefix || directory.startsWith(prefix + "/") || prefix === directory || prefix.startsWith(directory + "/"))) return;
    if (directory && leaves.some(pattern => pattern.match(directory))) {
      const manifest = join(cwd, directory, "Cargo.toml");
      if (existsSync(manifest) && readDocument(slash(relative(root,manifest))).package !== undefined) { physical(root, slash(relative(root,manifest))); paths.add(slash(relative(root,manifest))); return; }
    }
    for (const entry of readdirSync(join(cwd,directory),{withFileTypes:true})) {
      const name=entry.name;
      if(!entry.isDirectory() && (!entry.isFile() || name!=="Cargo.toml"))continue;
      const local=directory?`${directory}/${name}`:name;
      if(name.startsWith(".") || opaque.has(name) || excluded.some((pattern,index)=>pattern.match(local) || pattern.match(`${local}/`) || excludedRoots[index]?.match(local)))continue;
      if(entry.isDirectory())walk(local);
      else if(local!=="Cargo.toml" && patterns.some(pattern=>pattern.match(local)) && !excluded.some(pattern=>pattern.match(slash(dirname(local)))))paths.add(slash(relative(root,join(cwd,local))));
    }
  };
  if (patterns.some(pattern => pattern.match("Cargo.toml")) && !excluded.some(pattern => pattern.match(".") || pattern.match("./") || pattern.match("Cargo.toml"))) {
    const manifest = slash(relative(root, join(cwd, "Cargo.toml")));
    if (readDocument(manifest).package !== undefined) paths.add(manifest);
  }
  walk("");
  const owned = [...paths].filter(manifest => readDocument(manifest).package !== undefined && selectCargoWorkspace(root, manifest, readDocument, scopes).directory === owner.directory);
  if (!owned.length) throw new Error(`Cargo workspace has no current source-bound members: ${owner.manifest}`);
  const names = new Set<string>();
  return owned.sort().map(manifest => {
    const document = readDocument(manifest), name = document.package?.name;
    if (typeof name !== "string" || !name || names.has(name)) throw new Error(`Cargo workspace member has a missing or duplicate name: ${manifest}`);
    names.add(name);
    return { directory: slash(dirname(manifest)), manifest, name, workspace: owner.directory };
  });
}

/** 🧭️ Selects the actual nearest Cargo workspace for a physical package, including isolated native test workspaces. */
export function cargoWorkspaceForManifest(root: string, manifest: string): CargoWorkspaceScope {
  return selectCargoWorkspace(root, manifest, path => read(root, path), new Map());
}

/** 🏎️ Selects the physical native test profile authority for a repository workspace. */
export function cargoNextestConfiguration(root:string,directory:string):string {
 const path=join(root,directory,".config/nextest.toml");
 if(!existsSync(path))throw Error(`Selected Cargo test configuration is missing: ${path}`);
 return physical(root,relative(root,path));
}

/** 📇️ Reuses parsed authority only within the caller's current physical membership scan. */
function selectCargoWorkspace(root: string, manifest: string, readDocument: (path: string) => Record<string, any>, scopes: Map<string, CargoWorkspaceScope>): CargoWorkspaceScope {
  const full = physical(root, manifest), packageRow = readDocument(manifest).package;
  let directory = packageRow?.workspace ? resolve(dirname(full), packageRow.workspace) : dirname(full);
  while (true) {
    const path = join(directory, "Cargo.toml"), relativePath = slash(relative(root, path));
    if (relativePath.startsWith("../") || isAbsolute(relativePath)) throw new Error(`Cargo workspace escapes repository: ${manifest}`);
    if (existsSync(path)) {
      const admitted = scopes.get(relativePath);
      if (admitted) return admitted;
      const workspace = readDocument(relativePath).workspace;
      if (workspace) {
        const relativeDirectory = slash(relative(root, directory)) || ".", prefix = relativeDirectory === "." ? "" : `${relativeDirectory}/`;
        const selected = workspace.metadata?.semio?.repository !== undefined ? scope(relativeDirectory, object(workspace)) : { directory: relativeDirectory, manifest: `${prefix}Cargo.toml`, lock: `${prefix}Cargo.lock`, memberManifests: (workspace.members ?? ["."]).map((path: string) => `${path}/Cargo.toml`), contribution: parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members ?? ["."], exclude: workspace.exclude ?? [] }) };
        scopes.set(relativePath, selected);
        return selected;
      }
    }
    if (directory === resolve(root)) throw new Error(`Cargo package has no workspace: ${manifest}`);
    directory = dirname(directory);
  }
}

/** 📦️ Inventories all current repository member identities without returning stale removed owners. */
export function cargoRepositoryPackages(root: string): readonly CargoWorkspacePackage[] {
  const packages = discoverCargoWorkspaces(root).flatMap(owner => cargoWorkspaceMembers(root, owner));
  const names = new Set<string>();
  for (const row of packages) { if (names.has(row.name)) throw new Error(`Duplicate repository Cargo package: ${row.name}`); names.add(row.name); }
  return packages;
}

/** 📦️ Binds one current repository inventory to an ordered group of named packages. */
export function cargoRepositoryPackageSelections(root: string, names: readonly string[]): readonly CargoWorkspacePackage[] {
  if (!names.length) return [];
  const packages = new Map(cargoRepositoryPackages(root).map(row => [row.name, row]));
  return names.map(name => { const row = packages.get(name); if (!row) throw new Error(`Unknown current repository Cargo package: ${name}`); return row; });
}

/** 🎯️ Binds a named current repository package to its physical manifest and selected native workspace. */
export function cargoRepositoryPackage(root: string, name: string): CargoWorkspacePackage {
  const row = cargoRepositoryPackages(root).find(row => row.name === name);
  if (!row) throw new Error(`Unknown current repository Cargo package: ${name}`);
  return row;
}

/** 📐️ Checks declared native member patterns independently of physical package discovery. */
export function cargoWorkspaceDeclaresMemberV1(source: string, directory: string): boolean {
  const workspace = object(object(Bun.TOML.parse(source)).workspace);
  const contribution = parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members, exclude: workspace.exclude ?? [] });
  return contribution.members.some(pattern => new Bun.Glob(pattern).match(directory)) && !contribution.exclude.some(pattern => new Bun.Glob(pattern).match(directory));
}

/** 🎛️ Binds package selectors to one current physical workspace and preserves explicit Cargo manifest control. */
export function selectedCargoArguments(root: string, args: readonly string[]): string[] {
  if (args.some(arg => arg === "--manifest-path" || arg.startsWith("--manifest-path="))) return [...args];
  const names: string[] = [];
  for (let index = 1; index < args.length; index++) {
    const arg = args[index]!;
    if (arg === "--") break;
    if (arg === "-p" || arg === "--package") { const value = args[++index]; if (!value) throw new Error("Cargo package selector requires a current name"); names.push(value); }
    else if (arg.startsWith("--package=")) names.push(arg.slice(10));
  }
  if (!names.length) return [...args];
  const packages = cargoRepositoryPackages(root), rows = names.map(name => { const row = packages.find(row => row.name === name); if (!row) throw new Error(`Unknown current repository Cargo package: ${name}`); return row; });
  if (new Set(rows.map(row => row.workspace)).size !== 1) throw new Error("Cargo command selects packages from different workspaces; invoke each exact owner separately");
  return [args[0]!, "--manifest-path", join(root, rows[0]!.manifest), ...args.slice(1)];
}

/** 📣️ Publishes only the admitted current member array, preserving every other Cargo declaration. */
export function publishCargoWorkspaceMembership(root: string, owner: CargoWorkspaceScope, mode: "check" | "write"): boolean {
  return publishMembership(root, owner, mode, discoverCargoWorkspaces(root));
}

/** 📣️ Publishes one freshly admitted repository inventory without rediscovering every child for each owner. */
export function publishCargoWorkspaceMemberships(root: string, mode: "check" | "write"): readonly Readonly<{ owner: CargoWorkspaceScope; changed: boolean }>[] {
  const owners = discoverCargoWorkspaces(root);
  return owners.map(owner => ({ owner, changed: publishMembership(root, owner, mode, owners) }));
}

/** 📣️ Applies the current admitted native child boundaries to one guarded owner publication. */
function publishMembership(root: string, owner: CargoWorkspaceScope, mode: "check" | "write", owners: readonly CargoWorkspaceScope[]): boolean {
  const path=physical(root,owner.manifest),source=readFileSync(path,"utf8"),document=object(Bun.TOML.parse(source));
  const members=cargoWorkspaceMembers(root,owner).map(row=>slash(relative(resolve(root,owner.directory),resolve(root,row.directory))) || ".");
  const children=owners.filter(row=>row.directory!==owner.directory && (owner.directory==="." || row.directory.startsWith(owner.directory+"/"))).map(row=>slash(relative(resolve(root,owner.directory),resolve(root,row.directory))));
  const exclude=[...new Set([...owner.contribution.exclude,...children])].sort();
  if(JSON.stringify(document.workspace.members)===JSON.stringify(members) && JSON.stringify(document.workspace.exclude??[])===JSON.stringify(exclude))return false;
  if(mode==="check")throw Error(`Cargo source membership is stale: ${owner.manifest}`);
  let after=source;
  for(const [name,values] of [["members",members],["exclude",exclude]] as const){
    const heading=/^\[workspace\]\r?$/m.exec(after);if(!heading)throw Error(`Cargo workspace table is absent: ${owner.manifest}`);
    const bodyStart=heading.index+heading[0].length,next=/^\[/m.exec(after.slice(bodyStart)),section=after.slice(bodyStart,next?bodyStart+next.index:after.length),field=new RegExp("^"+name+"\\s*=\\s*\\[","m").exec(section);
    const replacement=name+" = [\n"+values.map(value=>"    "+JSON.stringify(value)+",").join("\n")+"\n]";
    if(!field){after=after.slice(0,bodyStart)+"\n"+replacement+after.slice(bodyStart);continue;}
    const start=bodyStart+field.index;let end=start+field[0].length,quote="",escaped=false;
    for(;end<after.length;end++){const character=after[end]!;if(quote){if(escaped)escaped=false;else if(quote==='"' && character==="\\")escaped=true;else if(character===quote)quote="";}else if(character==='"' || character==="'")quote=character;else if(character==="]"){end++;break;}}
    if(end>=after.length)throw Error(`Unterminated Cargo workspace array: ${owner.manifest}`);
    after=after.slice(0,start)+replacement+after.slice(end);
  }
  if(readFileSync(physical(root,owner.manifest),"utf8")!==source)throw Error(`Cargo workspace changed during member discovery: ${owner.manifest}`);
  writeFileSync(path,after);return true;
}

export type CargoPreparationV1 = Readonly<{ script: string; command: readonly string[] }>;
export type CargoPreparationDiagnosticPhaseV1 = "lease-wait" | "lease" | "discovery" | "inventory" | "closure" | "recipe" | "finished" | "publication";
/** 🕰️ Emits opt-in preparation observations separately from machine-readable stdout. */
export function logCargoPreparationDiagnosticV1(phase: CargoPreparationDiagnosticPhaseV1, owner: string, elapsedMs: number, items: number): void {
 if(process.env.SEMIO_CARGO_PREPARATION_TIMING==="1")console.error(`[DEBUG] cargo-preparation ${JSON.stringify({phase,owner,elapsedMs,items})}`);
}
/** 🧬️ Admits an owner-authored executable recipe with literal argv and no implicit producer identity. */
export function parseCargoPreparation(value: unknown): CargoPreparationV1 {
 const row=object(value);if(Object.keys(row).some(k=>!["script","command"].includes(k)) || (typeof row.script!=="string" || !row.script || isAbsolute(row.script) || /^[A-Za-z]:\//.test(row.script) || row.script.includes("\\")) || row.script.split("/").at(-1)!=="📜️script.ts" || !Array.isArray(row.command) || !row.command.length || row.command.some((v:unknown)=>typeof v!=="string" || !v || v.includes("\0")))throw new Error("Invalid Cargo preparation recipe");return {script:row.script,command:[...row.command]};
}
/** 🔗️ Selects only native scopes reached by the current workspace's authored local dependency paths. */
export function prepareCargoOwners(root: string, selected: CargoWorkspaceScope, names: readonly string[] = [], observationDirectory?:string): Readonly<{manifests:readonly string[];owners:readonly string[];recipes:readonly CargoPreparationObservationV1[]}> {
 const timed=process.env.SEMIO_CARGO_PREPARATION_TIMING==="1",started=timed?performance.now():0;
 const diagnostic=(phase:CargoPreparationDiagnosticPhaseV1,owner:string,start:number,items:number):void=>{if(timed)logCargoPreparationDiagnosticV1(phase,owner,performance.now()-start,items);};
 const recipes:CargoPreparationObservationV1[]=[];
 const scopes=discoverCargoWorkspaces(root), nativeScopes=new Map(scopes.map(scope=>[scope.manifest,scope])), inventories=new Map<string,readonly CargoWorkspacePackage[]>(), documents=new Map<string,Record<string,any>>(), executed=new Set<string>();
 diagnostic("discovery",selected.manifest,started,scopes.length);
 const document=(manifest:string):Record<string,any>=>{let value=documents.get(manifest);if(!value){value=read(root,manifest);documents.set(manifest,value);}return value;};
 const members=(owner:CargoWorkspaceScope):readonly CargoWorkspacePackage[]=>{let value=inventories.get(owner.directory);if(!value){const start=timed?performance.now():0;value=cargoWorkspaceMembers(root,owner);inventories.set(owner.directory,value);diagnostic("inventory",owner.manifest,start,value.length);}return value;};
 const binding=(manifest:string)=>{const native=selectCargoWorkspace(root,manifest,document,nativeScopes),owner=scopes.find(scope=>scope.manifest===native.manifest);if(!owner)throw Error(`Unknown selected Cargo scope: ${manifest}`);const pkg=members(owner).find(pkg=>pkg.manifest===manifest);if(!pkg)throw Error(`Cargo dependency lacks authored member authority: ${manifest}`);return{owner,pkg,current:document(manifest)};};
 const localDependencies=(manifest:string,development:boolean):string[]=>{
 const {owner,pkg,current}=binding(manifest),authority=document(owner.manifest).workspace,groups=[current,...Object.values(current.target??{})] as Record<string,any>[],paths:string[]=[];
 for(const group of groups)for(const kind of development?["dependencies","dev-dependencies","build-dependencies"]:["dependencies","build-dependencies"])for(const [alias,value] of Object.entries(group[kind]??{})){
 const entry=value as any;if(!entry || typeof entry!=="object")continue;const dependency=entry.workspace?authority.dependencies?.[alias]:entry;
 if(!dependency)throw Error(`Missing inherited Cargo dependency: ${manifest} ${alias}`);if(typeof dependency.path!=="string")continue;
 const path=resolve(root,entry.workspace?owner.directory:pkg.directory,dependency.path,"Cargo.toml"),local=slash(relative(root,path));
 if(local.startsWith("../") || isAbsolute(local))throw Error(`Cargo dependency escapes preparation authority: ${alias}`);physical(root,local);paths.push(local);
 }return paths;};
 const initial=members(selected),initialRoots=new Set(initial.map(pkg=>pkg.manifest));let selectedPackages=initial;
 if(names.some(name=>!initial.some(pkg=>pkg.name===name))){const reachable=new Set(initialRoots);for(const manifest of reachable)for(const dependency of localDependencies(manifest,initialRoots.has(manifest)))reachable.add(dependency);selectedPackages=[...reachable].map(manifest=>binding(manifest).pkg);}
 const roots=new Set((names.length?names.map(name=>{const candidates=selectedPackages.filter(pkg=>pkg.name===name);if(candidates.length!==1)throw Error(`Unknown or ambiguous selected Cargo package: ${selected.manifest} ${name}`);return candidates[0]!;}):initial).map(pkg=>pkg.manifest)),pending=new Set(roots);
 for(const manifest of pending){
 const start=timed?performance.now():0,{owner,pkg,current}=binding(manifest);
 for(const dependency of localDependencies(manifest,roots.has(manifest)))pending.add(dependency);
 diagnostic("closure",manifest,start,pending.size);
 const value=current.package?.metadata?.semio?.preparation;if(value===undefined)continue;
 const recipe=parseCargoPreparation(value),script=physical(root,slash(relative(root,resolve(root,pkg.directory,recipe.script)))),key=JSON.stringify([script,...recipe.command]);
 if(slash(relative(resolve(root,owner.directory),script)).startsWith("../"))throw Error(`Cargo preparation leaves its owner workspace: ${manifest}`);if(executed.has(key))continue;executed.add(key);
 console.log(`[cargo-preparation] ${manifest} ${recipe.command.join(" ")}`);
 const recipeStart=timed?performance.now():0;
 const observation=observationDirectory?join(observationDirectory,createHash("sha256").update(key).digest("hex")+".json"):undefined, before=createHash("sha256").update(readFileSync(script)).digest("hex"), program=observation?cargoPreparationProgramSourcesV1(root,script):undefined;
 const result=Bun.spawnSync([process.execPath,script,...recipe.command],{cwd:resolve(root,pkg.directory),env:{...process.env,NX_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_ACTIVE:script,...(observation?{SEMIO_CARGO_PREPARATION_OBSERVATION:observation}:{})},stdout:"pipe",stderr:"inherit",timeout:30_000});
 if(result.stdout.byteLength)process.stderr.write(result.stdout);if(result.exitCode!==0)throw Error(`Cargo owner preparation failed: ${manifest} (${result.exitCode})`);
 if(observation){const captured=admitCargoPreparationObservationV1(JSON.parse(readFileSync(observation,"utf8")));if(captured.root!==root||captured.script!==script||JSON.stringify(captured.command)!==JSON.stringify(recipe.command)||captured.sources.find(input=>input.path===script)?.sha256!==before)throw Error("Preparation observation does not bind its original producer");assertCargoPreparationObservationCurrentV1(captured);assertCargoPreparationObservationCurrentV1({...captured,sources:program!.sources,inputs:program!.inputs,outputs:[]});const merged={...captured,sources:[...new Map([...program!.sources,...captured.sources].map(input=>[input.path,input])).values()],inputs:[...new Map([...program!.inputs,...captured.inputs].map(input=>[JSON.stringify([input.path,input.kind]),input])).values()]};assertCargoPreparationObservationCurrentV1(merged);recipes.push(merged);}
 diagnostic("recipe",manifest,recipeStart,1);
 }
 diagnostic("finished",selected.manifest,started,pending.size);
 return {manifests:[...new Set([...documents.keys(),...scopes.map(scope=>scope.manifest)])],owners:[...inventories.keys()],recipes};
}

/** 🦀️ Uses the owned schema authority for native operations that consume current package inputs. */
export function cargoCommandRequiresOwnerPreparationV1(command: string): boolean {
  return invocationSchema.allOf[0]!.if.properties.command.enum.includes(command);
}

/** 🧭️ Resolves the manifest selected by the repository's existing Cargo invocation grammar. */
export function cargoInvocationManifestV1(args: readonly string[], cwd: string): string {
  const index = args.indexOf("--manifest-path"), inline = args.find(arg => arg.startsWith("--manifest-path="));
  const selected = index >= 0 ? args[index+1] : inline?.slice(16);
  return selected ? resolve(cwd, selected) : join(cwd, "Cargo.toml");
}

/** 🛠️ Refreshes the selected native workspace before each repository Cargo operation. */
export function prepareCargoWorkspaceInvocation(root: string, args: readonly string[], cwd: string, environment: Readonly<Record<string,string|undefined>> = process.env): void {
  if (!cargoCommandRequiresOwnerPreparationV1(args[0] ?? "")) return;
  const path = cargoInvocationManifestV1(args, cwd);
  if (environment.SEMIO_CARGO_PREPARATION_ACTIVE) throw new Error(`Cargo recursion in owner preparation: ${environment.SEMIO_CARGO_PREPARATION_ACTIVE}`);
  const owner = cargoWorkspaceForManifest(root, slash(relative(root, path))), source = read(root, owner.manifest), names:string[]=[];
  for(let cursor=1;cursor<args.length && args[cursor]!=="--";cursor++){const arg=args[cursor]!;if(arg==="-p" || arg==="--package"){const name=args[++cursor];if(!name)throw Error("Cargo package selector requires a current name");names.push(name);}else if(arg.startsWith("--package="))names.push(arg.slice(10));else if(arg.startsWith("-p") && arg.length>2)names.push(arg.slice(2));}
  const selectedPackage=read(root,slash(relative(root,path))).package?.name;
  if(!names.length && !args.includes("--workspace") && typeof selectedPackage==="string")names.push(selectedPackage);
  if (source.workspace?.metadata?.semio?.repository !== undefined) {
    const result=Bun.spawnSync([process.execPath,fileURLToPath(new URL("./🛠️preparation/📜️script.ts",import.meta.url)),"prepare","--manifest",owner.manifest,...names.flatMap(name=>["--package",name])],{cwd:root,env:{...environment,NX_WORKSPACE_ROOT:root},stdout:"pipe",stderr:"inherit"});
    if (result.stdout.byteLength) process.stderr.write(result.stdout);
    if(result.exitCode!==0)throw new Error(`Selected Cargo preparation failed: ${owner.manifest} (${result.exitCode})`);
  }
}
