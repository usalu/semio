import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import TOML from "@iarna/toml";

const root = resolve(import.meta.dir, "../../../../../../../..");
const ticket = resolve(import.meta.dir, "..");
const output = join(ticket, "🗑️generated", "workspace-partition");
const slash = (value: string) => value.replaceAll("\\", "/");
const digest = (value: string) => createHash("sha256").update(value).digest("hex");
const frames = new Map<string, { path: string; body: string; inverse: string; sha256: string; parsed: Record<string, any> }>();

/** 📸 Captures actual physical manifest authority with independent TOML agreement. */
function capture(path: string) {
  const normalized = slash(path);
  if (frames.has(normalized)) return frames.get(normalized)!;
  const full = resolve(root, normalized), local = slash(relative(root, full));
  if (isAbsolute(local) || local === ".." || local.startsWith("../")) throw new Error(`Escaped manifest: ${normalized}`);
  let ancestor = root;
  for (const part of local.split("/")) {
    ancestor = join(ancestor, part);
    if (lstatSync(ancestor).isSymbolicLink()) throw new Error(`Symlink manifest ancestry: ${normalized}`);
  }
  if (!lstatSync(full).isFile()) throw new Error(`Non-file manifest: ${normalized}`);
  const body = readFileSync(full, "utf8"), parsed = Bun.TOML.parse(body) as Record<string, any>;
  if (!isDeepStrictEqual(parsed, TOML.parse(body))) throw new Error(`TOML disagreement: ${normalized}`);
  const row = { path: normalized, body, inverse: body, sha256: digest(body), parsed };
  frames.set(normalized, row);
  return row;
}

/** 🧭 Records dependency and inheritance edges without running preparation or Cargo. */
function dependencyEdges(frame: ReturnType<typeof capture>) {
  const groups: { kind: string; dependencies: Record<string, any> }[] = [];
  for (const kind of ["dependencies", "dev-dependencies", "build-dependencies"]) {
    if (frame.parsed[kind]) groups.push({ kind, dependencies: frame.parsed[kind] });
    for (const [target, row] of Object.entries(frame.parsed.target ?? {}) as [string, Record<string, any>][]) {
      if (row[kind]) groups.push({ kind: `target.${target}.${kind}`, dependencies: row[kind] });
    }
  }
  return groups.flatMap(({ kind, dependencies }) => Object.entries(dependencies).map(([key, declaration]) => {
    const row = typeof declaration === "object" && declaration !== null ? declaration : {};
    const path = typeof row.path === "string" ? slash(relative(root, resolve(root, dirname(frame.path), row.path, "Cargo.toml"))) : null;
    return { source: frame.path, kind, key, package: row.package ?? key, path, physical: path === null ? null : existsSync(resolve(root, path)), inherited: row.workspace === true, optional: row.optional === true, declaration };
  }));
}

/** 🗂 Captures the finite caller-authored root/member/owner roster without inventing missing bodies. */
function main() {
  const rootFrame = capture("Cargo.toml"), rootWorkspace = rootFrame.parsed.workspace;
  const declared = rootWorkspace.metadata.semio.repository;
  const ownerPaths = new Set<string>(["Cargo.toml"]);
  for (const pattern of declared["owner-manifests"] as string[]) {
    for (const path of new Bun.Glob(pattern).scanSync({ cwd: root, dot: true, onlyFiles: true, followSymlinks: false })) ownerPaths.add(path);
  }
  const missing: { owner: string; member: string }[] = [];
  const ownerRows: { manifest: string; workspace: boolean; admission: boolean; members: string[] }[] = [];
  const manifestPaths = new Set<string>();
  for (const ownerPath of [...ownerPaths].sort()) {
    const owner = capture(ownerPath), workspace = owner.parsed.workspace, members: string[] = [];
    if (workspace) {
      const directory = dirname(ownerPath);
      for (const member of workspace.members ?? []) {
        const candidate = slash(join(directory, member, "Cargo.toml"));
        if (!/[*?\[{]/u.test(member)) {
          if (existsSync(resolve(root, candidate))) { manifestPaths.add(candidate); members.push(candidate); }
          else missing.push({ owner: ownerPath, member: candidate });
        } else {
          for (const path of new Bun.Glob(candidate).scanSync({ cwd: root, dot: true, onlyFiles: true, followSymlinks: false })) { manifestPaths.add(path); members.push(path); }
        }
      }
    }
    ownerRows.push({ manifest: ownerPath, workspace: !!workspace, admission: workspace?.metadata?.semio?.repository !== undefined, members: [...new Set(members)].sort() });
  }
  for (const path of [...manifestPaths].sort()) capture(path);
  const rows = [...frames.values()].sort((a, b) => a.path.localeCompare(b.path));
  const pointers = rows.filter(row => typeof row.parsed.package?.workspace === "string").map(row => {
    const pointer = row.parsed.package.workspace;
    const target = slash(relative(root, resolve(root, dirname(row.path), pointer, "Cargo.toml")));
    return { source: row.path, package: row.parsed.package.name, pointer, target, physical: existsSync(resolve(root, target)) };
  });
  const edges = rows.flatMap(dependencyEdges);
  const generalPrefix = "🧰️framework/", productPrefix = `${generalPrefix}🛍️products/`;
  const generalMembers = (rootWorkspace.members as string[]).filter(path => path.startsWith(generalPrefix) && !path.startsWith(productPrefix));
  const productGroups = new Map<string, string[]>();
  for (const path of rootWorkspace.members as string[]) {
    if (!path.startsWith(productPrefix)) continue;
    const owner = path.slice(0, path.indexOf("/", productPrefix.length));
    const members = productGroups.get(owner) ?? []; members.push(path); productGroups.set(owner, members);
  }
  const generalEdges = edges.filter(row => row.source.startsWith(generalPrefix) && !row.source.startsWith(productPrefix) && row.path?.startsWith(productPrefix));
  mkdirSync(output, { recursive: true });
  const proof = { schemaVersion: 1, observedAt: new Date().toISOString(), sourceWrites: 0, cargoExecuted: false, inputAuthority: { rootManifest: "Cargo.toml", rootManifestSha256: rootFrame.sha256, ownerPatterns: declared["owner-manifests"], memberPatterns: declared["member-manifests"] }, frames: rows, ownerRows, missing, pointers, edges, partitionCandidates: { general: { root: "🧰️framework", members: generalMembers }, products: [...productGroups].map(([root, members]) => ({ root, members })) }, generalProductEdges: generalEdges };
  const path = join(output, "finite-current-manifest-ownership-capture-1.json");
  if (existsSync(path)) throw new Error("Immutable capture already exists");
  writeFileSync(path, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`[DEBUG] ${JSON.stringify({ proof: path, frames: rows.length, ownerCandidates: ownerRows.length, missing: missing.length, pointers: pointers.length, absentPointerTargets: pointers.filter(row => !row.physical).length, generalMembers: generalMembers.length, products: proof.partitionCandidates.products.map(row => ({ root: row.root, members: row.members.length })), generalProductEdges: generalEdges.length, sourceWrites: 0, cargoExecuted: false })}`);
}

/** 🔗 Qualifies inherited providers against fresh full manifest frames and preserves capture advances. */
function qualify() {
  const predecessorPath = join(output, "finite-current-manifest-ownership-capture-1.json");
  const predecessorBody = readFileSync(predecessorPath, "utf8"), predecessor = JSON.parse(predecessorBody);
  const advances: { path: string; before: string; current: string; inverse: string; beforeSha256: string; currentSha256: string }[] = [];
  for (const before of predecessor.frames) {
    if (digest(before.body) !== before.sha256 || before.inverse !== before.body) throw new Error(`Invalid retained frame: ${before.path}`);
    const current = capture(before.path);
    if (current.sha256 !== before.sha256) advances.push({ path: before.path, before: before.body, current: current.body, inverse: before.body, beforeSha256: before.sha256, currentSha256: current.sha256 });
  }
  const resolved = [...frames.values()].flatMap(frame => dependencyEdges(frame).map(edge => {
    if (!edge.inherited) return { ...edge, workspaceProvider: null, workspaceProviderSha256: null, resolvedDeclaration: edge.declaration, resolution: "direct" };
    const pointer = frame.parsed.package?.workspace;
    if (typeof pointer !== "string") return { ...edge, resolution: "unqualified-workspace-pointer", workspaceProvider: null, workspaceProviderSha256: null, resolvedDeclaration: null };
    const providerPath = slash(relative(root, resolve(root, dirname(frame.path), pointer, "Cargo.toml")));
    if (!existsSync(resolve(root, providerPath))) return { ...edge, resolution: "absent-workspace-provider", workspaceProvider: providerPath, workspaceProviderSha256: null, resolvedDeclaration: null };
    const provider = capture(providerPath), declaration = provider.parsed.workspace?.dependencies?.[edge.key];
    if (declaration === undefined) return { ...edge, resolution: "absent-workspace-dependency", workspaceProvider: providerPath, workspaceProviderSha256: provider.sha256, resolvedDeclaration: null };
    const dependency = typeof declaration === "object" && declaration !== null ? declaration : {};
    const path = typeof dependency.path === "string" ? slash(relative(root, resolve(root, dirname(providerPath), dependency.path, "Cargo.toml"))) : null;
    return { ...edge, package: dependency.package ?? edge.package, path, physical: path === null ? null : existsSync(resolve(root, path)), workspaceProvider: providerPath, workspaceProviderSha256: provider.sha256, resolvedDeclaration: declaration, resolution: "inherited-qualified" };
  }));
  const general = "🧰️framework/", products = `${general}🛍️products/`;
  const upward = resolved.filter(edge => edge.source.startsWith(general) && !edge.source.startsWith(products) && edge.path?.startsWith(products));
  const inheritedGeneral = resolved.filter(edge => edge.source.startsWith(general) && !edge.source.startsWith(products) && edge.inherited);
  const proof = { schemaVersion: 1, observedAt: new Date().toISOString(), predecessorPath, predecessorSha256: digest(predecessorBody), sourceWrites: 0, cargoExecuted: false, frames: [...frames.values()].sort((a, b) => a.path.localeCompare(b.path)), observedAdvances: advances, resolvedEdges: resolved, generalProductEdges: upward, generalInheritedDependencies: inheritedGeneral, unqualifiedInheritedEdges: resolved.filter(edge => edge.inherited && edge.resolution !== "inherited-qualified") };
  const path = join(output, "finite-inherited-provider-qualification-1.json");
  if (existsSync(path)) throw new Error("Immutable qualification already exists");
  writeFileSync(path, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`[DEBUG] ${JSON.stringify({ proof: path, frames: proof.frames.length, observedAdvances: advances.length, edges: resolved.length, upward: upward.map(edge => ({ source: edge.source, key: edge.key, kind: edge.kind, resolution: edge.resolution, path: edge.path })), generalInheritedKeys: [...new Set(inheritedGeneral.map(edge => edge.key))].sort(), unqualifiedInheritedEdges: proof.unqualifiedInheritedEdges.length, sourceWrites: 0, cargoExecuted: false })}`);
}

/** 🧮 Qualifies each proposed owner against the retained dependency and profile declarations. */
function requirements() {
  const path=join(output,"finite-inherited-provider-qualification-1.json"),body=readFileSync(path,"utf8"),proof=JSON.parse(body);
  const inventory=JSON.parse(readFileSync(join(output,"finite-current-manifest-ownership-capture-1.json"),"utf8"));
  const retained=new Map<string,any>(proof.frames.map((frame:any)=>[frame.path,frame]));
  const rootFrame=retained.get("Cargo.toml");
  if(!rootFrame||digest(rootFrame.body)!==rootFrame.sha256)throw new Error("Invalid retained root manifest");
  const profile=rootFrame.parsed.profile;
  const groups=[inventory.partitionCandidates.general,...inventory.partitionCandidates.products];
  const owners=groups.map((group:any)=>{
    const members=new Set<string>(group.members.map((member:string)=>slash(join(member,"Cargo.toml"))));
    const inherited=proof.resolvedEdges.filter((edge:any)=>members.has(edge.source)&&edge.inherited);
    const providerKeys=[...new Set<string>(inherited.map((edge:any)=>edge.key))].sort();
    const visited=new Set<string>(),queue=[...members];
    const absent=new Set<string>();
    while(queue.length){const source=queue.shift()!;if(visited.has(source))continue;visited.add(source);if(!retained.has(source)){absent.add(source);continue;}for(const edge of proof.resolvedEdges.filter((edge:any)=>edge.source===source)){if(edge.path&&!visited.has(edge.path))queue.push(edge.path);}}
    const packages=[...new Set<string>([...visited].flatMap(source=>retained.get(source)?.parsed.package?.name?[retained.get(source).parsed.package.name]:[]))].sort();
    const packageOverrides=Object.entries(profile).flatMap(([name,row]:[string,any])=>Object.entries(row.package??{}).map(([packageName,declaration])=>({profile:name,package:packageName,declaration,firstParty:packageName.startsWith("semio-"),selectedPathPackage:packages.includes(packageName)})));
    return {root:group.root,members:[...members].sort(),providerKeys,providers:providerKeys.map(key=>({key,declaration:rootFrame.parsed.workspace.dependencies[key]})),pathClosure:[...visited].sort(),packages,absentPathClosure:[...absent].sort(),packageOverrides};
  });
  const destination=join(output,"finite-owner-partition-requirements-1.json");if(existsSync(destination))throw new Error("Immutable requirements already exist");
  writeFileSync(destination,`${JSON.stringify({schemaVersion:1,observedAt:new Date().toISOString(),sourceWrites:0,cargoExecuted:false,inputPath:path,inputSha256:digest(body),profile,owners},null,2)}\n`);
  console.log(`[DEBUG] ${JSON.stringify({proof:destination,owners:owners.map((owner:any)=>({root:owner.root,members:owner.members.length,providerKeys:owner.providerKeys,absentPathClosure:owner.absentPathClosure,firstPartyOverrides:owner.packageOverrides.filter((row:any)=>row.firstParty)})),sourceWrites:0,cargoExecuted:false})}`);
}

/** 📚 Captures handcrafted partition candidates with exact independent provider/profile observations. */
function candidates(){
  const requirements=JSON.parse(readFileSync(join(output,"finite-owner-partition-requirements-1.json"),"utf8"));
  const names=["general","quiz","os","server","repo"];
  const rows=names.map((name,index)=>{
    const owner=requirements.owners[index],path=join(import.meta.dir,`${name}-workspace-authored-2.toml`),body=readFileSync(path,"utf8"),parsed=Bun.TOML.parse(body)as any;assertParsed(parsed,TOML.parse(body),name);
    const manifest=slash(join(owner.root,"Cargo.toml")),members=(parsed.workspace.members as string[]).map(member=>slash(join(owner.root,member,"Cargo.toml"))).sort();
    if(!isDeepStrictEqual(members,owner.members))throw new Error(`Member mismatch: ${name}`);
    const providerGaps=owner.providerKeys.filter((key:string)=>!(key in parsed.workspace.dependencies));
    const providerRows=Object.entries(parsed.workspace.dependencies).map(([key,declaration]:[string,any])=>{
      const original=owner.providers.find((row:any)=>row.key===key)?.declaration;if(original===undefined)throw new Error(`Undeclared provider ${name}:${key}`);
      const originalPath=typeof original.path==="string"?slash(join(original.path,"Cargo.toml")):null,currentPath=typeof declaration.path==="string"?slash(relative(root,resolve(root,owner.root,declaration.path,"Cargo.toml"))):null;
      const withoutPath=(value:any)=>typeof value==="object"&&value!==null?Object.fromEntries(Object.entries(value).filter(([key])=>key!=="path")):value;
      if(!isDeepStrictEqual(withoutPath(original),withoutPath(declaration))||originalPath!==currentPath)throw new Error(`Provider mismatch ${name}:${key}`);
      return {key,original,declaration,physicalProvider:currentPath,physical:currentPath===null?null:existsSync(resolve(root,currentPath))};
    });
    const externalOverrides=owner.packageOverrides.filter((row:any)=>!row.firstParty);for(const row of externalOverrides)if(!isDeepStrictEqual(parsed.profile[row.profile]?.package?.[row.package],row.declaration))throw new Error(`External profile mismatch ${name}:${row.package}`);
    const profileGaps=owner.packageOverrides.filter((row:any)=>row.firstParty&&row.selectedPathPackage&&!isDeepStrictEqual(parsed.profile[row.profile]?.package?.[row.package],row.declaration));
    for(const[name,row]of Object.entries(requirements.profile)as[string,any][]){const base=(value:any)=>Object.fromEntries(Object.entries(value).filter(([key])=>key!=="package"));if(!isDeepStrictEqual(base(row),base(parsed.profile[name])))throw new Error(`Base profile mismatch ${name}`);}
    return {name,manifest,body,inverse:existsSync(resolve(root,manifest))?readFileSync(resolve(root,manifest),"utf8"):null,sha256:digest(body),parsed,members,providerRows,providerGaps,externalOverrides:externalOverrides.length,profileGaps,productionManifestPresent:existsSync(resolve(root,manifest))};
  });
  const path=join(output,"handcrafted-partition-candidate-admission-2.json");if(existsSync(path))throw new Error("Immutable candidate admission already exists");writeFileSync(path,JSON.stringify({schemaVersion:1,observedAt:new Date().toISOString(),sourceWrites:0,cargoExecuted:false,publicationReady:false,rows},null,2)+"\n");console.log(`[DEBUG] ${JSON.stringify({proof:path,rows:rows.map(row=>({name:row.name,members:row.members.length,providerGaps:row.providerGaps,externalOverrides:row.externalOverrides,profileGaps:row.profileGaps,productionManifestPresent:row.productionManifestPresent})),sourceWrites:0,publicationReady:false})}`);
}

/** 🧷 Requires independent exact parser agreement for authored owner assets. */
function assertParsed(first:unknown,second:unknown,name:string){if(!isDeepStrictEqual(first,second))throw new Error(`TOML disagreement: ${name}`);}

switch (process.argv[2]) {
  case "capture": main(); break;
  case "qualify": qualify(); break;
  case "requirements": requirements(); break;
  case "candidates": candidates(); break;
  default: throw new Error("Expected capture, qualify, requirements or candidates command");
}
