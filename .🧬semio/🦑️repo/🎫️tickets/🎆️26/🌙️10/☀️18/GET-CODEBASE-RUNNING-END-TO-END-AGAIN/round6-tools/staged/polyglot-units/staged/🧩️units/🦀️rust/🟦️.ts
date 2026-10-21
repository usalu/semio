import {sha256V1} from "../🔏️key/🟦️.ts";
import {compareCodePointsV1, sortedCodePointsV1, type UnitDependencyV1, type UnitGraphV1, type UnitsPolicyV1, type UnitV1} from "../🕸️graph/🟦️.ts";
import type {ExecuteOutcomeV1} from "../🏃️engine/🟦️.ts";

/** 🦀️ The part of `cargo metadata --format-version 1` the planner reads. */
export type CargoMetadataV1 = {
  readonly workspace_root: string;
  readonly workspace_members: readonly string[];
  readonly packages: readonly {readonly id: string; readonly name: string; readonly version: string; readonly manifest_path: string; readonly source: string | null; readonly targets: readonly {readonly name: string; readonly kind: readonly string[]; readonly src_path: string}[]}[];
  readonly resolve: {readonly nodes: readonly {readonly id: string; readonly deps: readonly {readonly pkg: string; readonly name: string; readonly dep_kinds: readonly {readonly kind: string | null; readonly target: string | null}[]}[]}[]} | null;
};
const LIBRARY_KINDS = new Set(["lib", "rlib", "dylib", "cdylib", "staticlib", "proc-macro"]);
const slash = (path: string): string => path.split("\\").join("/");

/** 🦀️ Wraps Cargo's own resolution into the neutral graph: one library unit per workspace or path package (`rust-rmeta` interface, `interface` edges to its normal dependencies, `implementation` edges to proc-macros and build dependencies), one binary unit per binary target, one test unit per package; registry packages become exact `name@version` externals. The per-crate resolution identity (`identity`, from the gran-keys engine) is folded into the toolchain parts when given. The unit-cache executes at rustc granularity underneath; this graph is the crate-level view Nx and Bazel plan with. */
export function rustUnitsFromMetadataV1(metadata: CargoMetadataV1, options: {readonly policy: UnitsPolicyV1; readonly files: readonly string[]; readonly toolchain: string; readonly identity?: (packageId: string) => string | undefined; readonly registryIdentity?: (name: string, version: string) => string}): UnitGraphV1 {
  const root = slash(metadata.workspace_root).replace(/\/$/, ""), relative = (path: string): string => { const p = slash(path); return p.startsWith(root + "/") ? p.slice(root.length + 1) : p; };
  const local = metadata.packages.filter(row => row.source === null), byId = new Map(metadata.packages.map(row => [row.id, row] as const));
  const dirOf = new Map(local.map(row => { const manifest = relative(row.manifest_path); return [row.id, manifest.includes("/") ? manifest.slice(0, manifest.lastIndexOf("/")) : ""] as const; }));
  const dirs = [...dirOf.values()];
  const filesOf = (id: string): string[] => {
    const dir = dirOf.get(id)!, prefix = dir === "" ? "" : dir + "/";
    return options.files.filter(path => {
      if (!path.startsWith(prefix)) return false;
      const rest = path.slice(prefix.length), segments = rest.split("/");
      if (segments.slice(0, -1).some(part => options.policy.rust.skipDirectories.includes(part))) return false;
      if (dirs.some(other => other !== dir && other !== "" && path.startsWith(other + "/") && other.startsWith(prefix))) return false;
      return options.policy.rust.sourceExtensions.some(extension => path.endsWith(extension));
    }).sort(compareCodePointsV1);
  };
  const nodes = new Map((metadata.resolve?.nodes ?? []).map(node => [node.id, node] as const));
  const hasLibrary = (id: string): boolean => byId.get(id)!.targets.some(target => target.kind.some(kind => LIBRARY_KINDS.has(kind)));
  const isProcMacro = (id: string): boolean => byId.get(id)!.targets.some(target => target.kind.includes("proc-macro"));
  const libId = (id: string): string => `rust:${byId.get(id)!.name}@${byId.get(id)!.version}`;
  const units: UnitV1[] = [];
  const toolchain = (id: string): UnitV1["toolchain"] => ({id: options.toolchain, parts: [...(options.identity?.(id) ? [`resolution=${options.identity(id)}`] : [])]});
  for (const row of local.sort((a, b) => compareCodePointsV1(a.id, b.id))) {
    const normal = new Map<string, UnitDependencyV1>(), development = new Map<string, UnitDependencyV1>(), externals = new Set<string>(), devExternals = new Set<string>();
    for (const dependency of nodes.get(row.id)?.deps ?? []) {
      const target = byId.get(dependency.pkg);
      if (!target) continue;
      for (const kind of dependency.dep_kinds) {
        const into = kind.kind === "dev" ? development : normal, ext = kind.kind === "dev" ? devExternals : externals;
        if (target.source !== null) {ext.add(options.registryIdentity?.(target.name, target.version) ?? `cargo-registry:${target.name}@${target.version}`); continue;}
        if (!hasLibrary(target.id)) continue;
        const use: UnitDependencyV1["use"] = kind.kind === "build" || isProcMacro(target.id) || kind.kind === "dev" ? "implementation" : "interface";
        const existing = into.get(libId(target.id));
        if (!existing || (existing.use === "interface" && use === "implementation")) into.set(libId(target.id), {unit: libId(target.id), use});
      }
    }
    const sources = filesOf(row.id), manifest = `${dirOf.get(row.id)! === "" ? "" : dirOf.get(row.id)! + "/"}Cargo.toml`;
    const library = hasLibrary(row.id);
    if (library) units.push({
      id: libId(row.id), language: "rust", kind: "library", label: row.name, sources, inputs: [], dependencies: [...normal.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: sortedCodePointsV1(externals),
      interface: {kind: "rust-rmeta"}, outputs: [`lib${row.name.replace(/-/g, "_")}.rlib`], toolchain: toolchain(row.id), action: {argv: ["repo", "cargo", "--store=shared", "--", "build", "-p", row.name, "--lib"], env: {}},
    });
    for (const target of row.targets.filter(entry => entry.kind.includes("bin"))) {
      const dependencies = new Map<string, UnitDependencyV1>();
      for (const [id] of normal) dependencies.set(id, {unit: id, use: "implementation"});
      if (library) dependencies.set(libId(row.id), {unit: libId(row.id), use: "implementation"});
      units.push({
        id: `rust:${row.name}@${row.version}#bin:${target.name}`, language: "rust", kind: "binary", label: target.name, sources: library ? [] : sources, inputs: library ? sources : [manifest].filter(path => !sources.includes(path)), dependencies: [...dependencies.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: sortedCodePointsV1(externals),
        interface: null, outputs: [target.name], toolchain: toolchain(row.id), action: {argv: ["repo", "cargo", "--store=shared", "--", "build", "-p", row.name, "--bin", target.name], env: {}},
      });
    }
    if (library || row.targets.some(entry => entry.kind.includes("bin"))) {
      const dependencies = new Map<string, UnitDependencyV1>();
      for (const [id] of normal) dependencies.set(id, {unit: id, use: "implementation"});
      for (const [id] of development) dependencies.set(id, {unit: id, use: "implementation"});
      if (library) dependencies.set(libId(row.id), {unit: libId(row.id), use: "implementation"});
      units.push({
        id: `rust:${row.name}@${row.version}#test`, language: "rust", kind: "test", label: row.name + " (test)", sources: [], inputs: sources, dependencies: [...dependencies.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: sortedCodePointsV1(new Set([...externals, ...devExternals])),
        interface: null, outputs: [], toolchain: toolchain(row.id), action: {argv: ["repo", "cargo", "--store=shared", "--", "test", "-p", row.name], env: {}},
      });
    }
  }
  return {version: 1, units: units.filter((unit, index) => units.findIndex(other => other.id === unit.id) === index)};
}

/** 🦀️ Maps a unit-cache entry (the rustc-granularity cache of `⚡️caching/🦀️cargo/🗃️unit-cache`) onto the neutral outcome: the `.rmeta` output is the interface, the digest of all outputs the implementation. */
export function rustEntryOutcomeV1(entry: {readonly outputs: readonly {readonly name: string; readonly sha256: string}[]}, unit: Pick<UnitV1, "interface">): ExecuteOutcomeV1 {
  const rmeta = entry.outputs.find(output => output.name.endsWith(".rmeta"));
  if (unit.interface && !rmeta) throw Error("unit-cache entry has no .rmeta output");
  return {ok: true, diagnostics: "", outputs: sha256V1(entry.outputs.map(output => `${output.name}:${output.sha256}`).sort(compareCodePointsV1).join("\n")), interface: unit.interface ? rmeta!.sha256 : null};
}
