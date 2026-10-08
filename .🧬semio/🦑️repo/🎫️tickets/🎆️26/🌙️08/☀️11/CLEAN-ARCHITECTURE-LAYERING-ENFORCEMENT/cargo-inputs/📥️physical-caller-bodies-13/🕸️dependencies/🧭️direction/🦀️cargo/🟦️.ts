import {CargoController} from "../../../🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts";
import { cargoRepositoryPackages,cargoWorkspaceForManifest,cargoManifestDocument,type CargoDiscoveryOperation } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { readFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";

export type CargoDirectionDependency = Readonly<{ name: string; alias: string; owner: string | null; kind: "normal" | "dev" | "build"; optional: boolean; platform: string | null }>;
export type CargoDirectionPackage = Readonly<{ name: string; owner: string; role: string | null; dependencies: readonly CargoDirectionDependency[] }>;
export type CargoDirectionPolicy = Readonly<{ areaLayers: Readonly<Record<string, "framework" | "implementation" | "repo-wide">>; roles: readonly string[]; ownerRoles: readonly Readonly<{ path: string; roles: readonly string[] }>[]; rules: Readonly<Record<string, Readonly<{ fromRoles: readonly string[]; toRoles: readonly string[]; fromOwnerPaths?: readonly string[]; toOwnerSegments?: readonly string[] }>>> }>;
export type CargoDirectionViolation = Readonly<{ rule: string; from: string; to: string; alias: string; kind: CargoDirectionDependency["kind"]; optional: boolean; platform: string | null }>;
export type CargoDirectionProblem = Readonly<{ code: "missing-role" | "unknown-role" | "unclassified-owner" | "owner-role-mismatch" | "unclassified-role-owner"; owner: string }>;
export type CargoDirectionReport = Readonly<{ packages: number; localDependencies: number; violations: readonly CargoDirectionViolation[]; problems: readonly CargoDirectionProblem[] }>;

const object = (value: unknown): Record<string, any> => {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Cargo direction requires readable object metadata");
  return value as Record<string, any>;
};
const text = (value: unknown): string => {
  if (typeof value !== "string" || !value) throw new Error("Cargo direction requires a nonempty name or owner path");
  return value;
};
const array = (value: unknown): unknown[] => {
  if (!Array.isArray(value)) throw new Error("Cargo direction requires complete array metadata");
  return value;
};
const ownerPath = (root: string, path: string): string => {
  const owner = relative(root, path).replaceAll("\\", "/");
  if (!owner || owner.startsWith("..") || isAbsolute(owner)) throw new Error(`Cargo direction owner escapes the authored workspace: ${path}`);
  return owner;
};
const role = (manifest: Record<string, any>): string | null => manifest.package?.metadata?.semio?.role ?? null;

/** 🧾️ Inventories all authored Cargo members and declaration kinds independently of the Cargo graph. */
export async function cargoDirectionInventory(root:string,operation:CargoDiscoveryOperation):Promise<readonly CargoDirectionPackage[]> {
  const control=new CargoController(operation,operation.workspace);await control.step("direction-inventory-owner",root,1,256);
  const read=async(path:string):Promise<Record<string,any>>=>object(await cargoManifestDocument(root,relative(root,path).replaceAll("\\","/"),operation));
  const members:string[]=[];for(const member of await cargoRepositoryPackages(root,operation)){await control.step("direction-member",member.manifest,1,32+member.directory.length*2);members.push(member.directory);}
  const found: CargoDirectionPackage[] = [], pending = [...members], visited = new Set<string>();
  operation.workspace.partialRecords.push(members,found,pending,visited);
  for (let index = 0; index < pending.length; index++) {
    await control.step("direction-pending",pending[index]!,1,64);
    const member = pending[index]!;
    const owner = ownerPath(root, resolve(root, member));
    if (visited.has(owner)) continue;
    visited.add(owner);
    const manifest = await read(join(root, owner, "Cargo.toml"));
    const authority = await cargoWorkspaceForManifest(root, `${owner}/Cargo.toml`,operation);
    const workspace = object((await read(join(root,authority.manifest))).workspace);
    const workspaceRoot = resolve(root, authority.directory);
    const dependencies: CargoDirectionDependency[] = [];
    const collect=async(tables:Record<string,any>,platform:string|null):Promise<void>=>{
      for (const [table, kind] of [["dependencies", "normal"], ["dev-dependencies", "dev"], ["build-dependencies", "build"]] as const) {
        for(const alias in tables[table]??{}) {
          if(!Object.hasOwn(tables[table],alias))continue;await control.step("direction-declaration",owner,1,256+alias.length*2);const declaration=tables[table][alias];
          const local = typeof declaration === "string" ? { version: declaration } : object(declaration);
          const inherited = local.workspace === true;
          const source = inherited ? object(workspace.dependencies ?? {})[alias] : local;
          if (inherited && source === undefined) throw new Error(`Cargo direction cannot read inherited dependency: ${owner} → ${alias}`);
          const base = typeof source === "string" ? { version: source } : object(source);
          const value = { ...base, ...local };
          if (value.optional !== undefined && typeof value.optional !== "boolean") throw new Error(`Cargo direction requires a boolean optional declaration: ${owner} → ${alias}`);
          dependencies.push({ name: text(value.package ?? alias), alias, owner: value.path === undefined ? null : ownerPath(root, resolve(inherited ? workspaceRoot : join(root, owner), text(value.path))), kind, optional: value.optional ?? false, platform });
        }
      }
    };
    operation.workspace.partialRecords.push(dependencies);await collect(manifest,null);
    for(const platform in manifest.target??{})if(Object.hasOwn(manifest.target,platform)){await control.step("direction-platform",platform,platform.length+1,64);await collect(object(manifest.target[platform]),platform);}
    found.push({ name: text(object(manifest.package).name), owner, role: role(manifest), dependencies });
    for (const dep of dependencies) if (dep.owner !== null && !visited.has(dep.owner)) pending.push(dep.owner);
  }
  await control.finish("direction-inventory-complete",root);return found;
}

/** 🦀️ Adapts Cargo's declared metadata behind a repository-owned language-neutral contract. */
export function cargoDirectionMetadata(value: unknown, root: string): readonly CargoDirectionPackage[] {
  if (Array.isArray(value)) {
    const rows = value.flatMap(entry => cargoDirectionMetadata(entry, root));
    if (!rows.length || new Set(rows.map(row => row.owner)).size !== rows.length || new Set(rows.map(row => row.name)).size !== rows.length) throw new Error("Cargo direction repeats workspace contribution identities");
    return rows;
  }
  const metadata = object(value), packages = array(metadata.packages), members = array(metadata.workspace_members).map(text);
  const selectedRoot = relative(root, resolve(text(metadata.workspace_root)));
  if (selectedRoot.startsWith("..") || isAbsolute(selectedRoot)) throw new Error("Cargo direction metadata belongs to a different repository");
  if (metadata.version !== 1 || !packages.length || members.length !== packages.length || new Set(members).size !== members.length) throw new Error("Cargo direction metadata is empty or incomplete");
  const ids = packages.map((value) => text(object(value).id));
  if (new Set(ids).size !== ids.length || JSON.stringify([...ids].sort()) !== JSON.stringify([...members].sort())) throw new Error("Cargo direction metadata omits authored workspace members");
  return packages.map((value) => {
    const pkg = object(value), owner = ownerPath(root, dirname(text(pkg.manifest_path)));
    return { name: text(pkg.name), owner, role: pkg.metadata?.semio?.role ?? null, dependencies: array(pkg.dependencies).map((value) => {
      const dep = object(value), kind = dep.kind === null ? "normal" : dep.kind;
      if (kind !== "normal" && kind !== "dev" && kind !== "build") throw new Error("Cargo direction metadata has an unknown dependency kind");
      if (typeof dep.optional !== "boolean" || (dep.target !== null && typeof dep.target !== "string")) throw new Error("Cargo direction metadata omits dependency activation declarations");
      return { name: text(dep.name), alias: text(dep.rename ?? dep.name), owner: dep.path === undefined || dep.path === null ? null : ownerPath(root, text(dep.path)), kind, optional: dep.optional, platform: dep.target };
    }) };
  });
}

/** 🧱️ Enforces physical and semantic direction for every local declaration without role omissions or exceptions. */
export function cargoDependencyDirectionReport(graph: readonly CargoDirectionPackage[], inventory: readonly CargoDirectionPackage[], policy: CargoDirectionPolicy): CargoDirectionReport {
  const validate = (packages: readonly CargoDirectionPackage[]): void => {
    if (!Array.isArray(packages) || !packages.length) throw new Error("Cargo direction requires a complete authored package inventory");
    if (new Set(packages.map((pkg) => pkg.owner)).size !== packages.length || new Set(packages.map((pkg) => pkg.name)).size !== packages.length) throw new Error("Cargo direction repeats a package owner or name");
    for (const pkg of packages) {
      text(pkg.name); text(pkg.owner);
      if (pkg.role !== null && typeof pkg.role !== "string") throw new Error("Cargo direction requires readable semantic role metadata");
      for (const value of array(pkg.dependencies)) {
        const dep = object(value);
        text(dep.name); text(dep.alias);
        if (!["normal", "dev", "build"].includes(dep.kind) || typeof dep.optional !== "boolean" || (dep.platform !== null && typeof dep.platform !== "string")) throw new Error("Cargo direction requires complete declaration activation metadata");
        if (dep.owner !== null) {
          const target = packages.find((pkg) => pkg.owner === text(dep.owner));
          if (!target || target.name !== dep.name) throw new Error(`Cargo direction omits a declared local target: ${pkg.owner} → ${dep.owner}`);
        }
      }
    }
  };
  validate(graph); validate(inventory);
  const signature = (packages: readonly CargoDirectionPackage[]): string => JSON.stringify(packages.map((pkg) => [pkg.name, pkg.owner, pkg.role, pkg.dependencies.map((dep) => [dep.name, dep.alias, dep.owner, dep.kind, dep.optional, dep.platform]).sort()]).sort());
  if (signature(graph) !== signature(inventory)) throw new Error("Cargo direction metadata disagrees with the independently inventoried authored declarations");
  const areas = Object.entries(object(policy.areaLayers)).sort(([a], [b]) => b.length - a.length);
  if (!areas.length || areas.some(([, layer]) => !["framework", "implementation", "repo-wide"].includes(layer)) || !Array.isArray(policy.roles) || !policy.roles.length || new Set(policy.roles).size !== policy.roles.length) throw new Error("Cargo direction requires declared physical areas and semantic roles");
  const layer = (owner: string): string | undefined => areas.find(([area]) => owner === area || owner.startsWith(`${area}/`))?.[1];
  if (!Array.isArray(policy.ownerRoles) || !policy.ownerRoles.length || new Set(policy.ownerRoles.map((rule) => rule.path)).size !== policy.ownerRoles.length || policy.ownerRoles.some((rule) => !rule || typeof rule.path !== "string" || !rule.path || !Array.isArray(rule.roles) || !rule.roles.length || new Set(rule.roles).size !== rule.roles.length || rule.roles.some((role: string) => !policy.roles.includes(role)))) throw new Error("Cargo direction requires complete distinct owner role classifications");
  const owners = policy.ownerRoles.map((rule) => ({ pattern: new RegExp(rule.path, "u"), roles: rule.roles }));
  const rules = Object.entries(object(policy.rules));
  if (!rules.length || rules.some(([name, rule]) => !name || Object.keys(object(rule)).some(key => !["fromRoles", "toRoles", "fromOwnerPaths", "toOwnerSegments"].includes(key)))) throw new Error("Cargo direction requires closed named rules");
  if (rules.some(([, rule]) => [rule.fromRoles, rule.toRoles].some(roles => !Array.isArray(roles) || new Set(roles).size !== roles.length || roles.some((role: unknown) => typeof role !== "string" || !policy.roles.includes(role))))) throw new Error("Cargo direction policy requires distinct known semantic roles");
  if (rules.some(([, rule]) => (!rule.fromRoles.length && !rule.fromOwnerPaths?.length) || (!rule.toRoles.length && !rule.toOwnerSegments?.length))) throw new Error("Cargo direction requires nonempty source and target selectors");
  if (rules.some(([, rule]) => rule.toOwnerSegments !== undefined && (!Array.isArray(rule.toOwnerSegments) || !rule.toOwnerSegments.length || new Set(rule.toOwnerSegments).size !== rule.toOwnerSegments.length || rule.toOwnerSegments.some((segment: unknown) => typeof segment !== "string" || !segment || /[/\\]/u.test(segment) || segment === "." || segment === "..")))) throw new Error("Cargo direction policy requires exact owner segments");
  if (rules.some(([, rule]) => rule.fromOwnerPaths !== undefined && (!Array.isArray(rule.fromOwnerPaths) || !rule.fromOwnerPaths.length || new Set(rule.fromOwnerPaths).size !== rule.fromOwnerPaths.length || rule.fromOwnerPaths.some((path: unknown) => typeof path !== "string" || !path)))) throw new Error("Cargo direction policy requires distinct source owner patterns");
  const sourceOwners = new Map<string, readonly RegExp[]>(rules.map(([name, rule]) => [name, (rule.fromOwnerPaths ?? []).map((pattern: string) => new RegExp(pattern, "u"))]));
  const problems: CargoDirectionProblem[] = [], violations: CargoDirectionViolation[] = [];
  let localDependencies = 0;
  for (const pkg of graph) {
    if (!layer(pkg.owner)) problems.push({ code: "unclassified-owner", owner: pkg.owner });
    if (!pkg.role) problems.push({ code: "missing-role", owner: pkg.owner });
    else if (!policy.roles.includes(pkg.role)) problems.push({ code: "unknown-role", owner: pkg.owner });
    const owner = owners.find((rule) => rule.pattern.test(pkg.owner));
    if (!owner) problems.push({ code: "unclassified-role-owner", owner: pkg.owner });
    else if (pkg.role && policy.roles.includes(pkg.role) && !owner.roles.includes(pkg.role)) problems.push({ code: "owner-role-mismatch", owner: pkg.owner });
    for (const dep of pkg.dependencies) {
      if (dep.owner === null) continue;
      const target = graph.find((pkg) => pkg.owner === dep.owner)!;
      localDependencies++;
      const forbidden = layer(pkg.owner) === "framework" && layer(target.owner) === "implementation" ? ["cargo-framework-no-implementation"] : [];
      for (const [name, rule] of rules) if ((rule.fromRoles.includes(pkg.role) || sourceOwners.get(name)!.some((pattern) => pattern.test(pkg.owner))) && (rule.toRoles.includes(target.role) || rule.toOwnerSegments?.some((segment: string) => target.owner.split("/").includes(segment)))) forbidden.push(name);
      for (const rule of forbidden) violations.push({ rule, from: pkg.owner, to: target.owner, alias: dep.alias, kind: dep.kind, optional: dep.optional, platform: dep.platform });
    }
  }
  violations.sort((a, b) => {
    const first = JSON.stringify([a.rule, a.from, a.to, a.alias, a.kind, a.platform]), second = JSON.stringify([b.rule, b.from, b.to, b.alias, b.kind, b.platform]);
    return first < second ? -1 : first > second ? 1 : 0;
  });
  return { packages: graph.length, localDependencies, violations, problems };
}
