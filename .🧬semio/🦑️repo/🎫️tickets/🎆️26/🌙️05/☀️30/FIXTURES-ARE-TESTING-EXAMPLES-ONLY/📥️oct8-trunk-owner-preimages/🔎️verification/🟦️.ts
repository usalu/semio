import { existsSync, readFileSync, realpathSync, writeFileSync, mkdirSync, readdirSync, statSync } from "node:fs";
import { createHash } from "node:crypto";
import { homedir } from "node:os";
import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";
import { inspectRuntimeGraphV1, runtimeFixturePathV1, type RuntimeGraphEvidenceV1, type RuntimeGraphFindingV1 } from "../🟦️.ts";
import { policyWalkRelFiles } from "../../🚶️file-walk/🟦️.ts";
import { repoCacheDirectory } from "../../../⚡️caching/🟦️.ts";
import { cargoDirectories } from "../../../⚡️caching/🦀️cargo/🟦️.ts";
import { runTool } from "../../../⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts";
import { prepareCargoWorkspaceInvocation } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";

interface CargoPackageV1 { readonly id: string; readonly name: string; readonly manifest_path: string; readonly source: string | null; readonly targets: readonly { readonly src_path: string; readonly kind: readonly string[] }[] }
interface CargoNodeV1 { readonly id: string; readonly features: readonly string[]; readonly deps: readonly { readonly pkg: string; readonly dep_kinds: readonly { readonly kind: string | null }[] }[] }
interface CompilerInputV1 { readonly package_id: string; readonly features: readonly string[]; readonly manifest_path?:string; readonly target: { readonly src_path: string; readonly kind: readonly string[] }; readonly profile: { readonly test: boolean }; readonly filenames: readonly string[]; readonly runtimeTarget: string | null; readonly runtimeWorkspace: string | null }
export interface RuntimeCargoBuildScriptV1 { readonly reason: "build-script-executed"; readonly package_id: string; readonly cfgs: readonly string[]; readonly env: readonly (readonly [string, string])[]; readonly out_dir: string }
export interface RuntimeCargoUnitV1 { readonly message: CompilerInputV1; readonly observedAtMs: number; readonly depInfo: readonly { readonly path: string; readonly text: string; readonly baseDirectory:string }[]; readonly inputs: readonly { readonly path: string; readonly sha256: string }[]; readonly artifacts: readonly { readonly path: string; readonly sha256: string; readonly stagedPath?: string; readonly stagedSha256?: string }[] }
export interface RuntimeCargoObservationV1 { readonly version: 1; readonly invocationInputs?: readonly {readonly path:string;readonly sha256:string|null}[]; readonly buildDirectory?:string|null; readonly manifest: string; readonly cwd: string; readonly command: "cargo"; readonly args: readonly string[]; readonly builtAtMs: number; readonly observedAtMs: number; readonly status: number; readonly cancelled: boolean; readonly units: readonly RuntimeCargoUnitV1[]; readonly buildScripts: readonly RuntimeCargoBuildScriptV1[]; readonly buildResources?: readonly RuntimeCargoBuildResourcesV1[] }
export type RuntimeBuildResourceObservationV1 = { readonly kind: "directory"; readonly path: string; readonly entries: readonly string[] } | { readonly kind: "copy" | "read"; readonly path: string; readonly output: string };
export interface RuntimeCargoBuildResourcesV1 { readonly package_id: string; readonly out_dir: string; readonly path: string; readonly text: string; readonly sha256: string; readonly observedAtMs: number; readonly resources: readonly { readonly input: RuntimeBuildResourceObservationV1; readonly sha256: string | null; readonly outputSha256: string | null; readonly observedEntries?: readonly string[] }[] }

/** 🖥️ Keeps fresh compiler output distinct from the bytes the actual server mount serves. */
export function runtimeBrowserArtifactV1(path:string,content:string,read:(path:string)=>string|Uint8Array|undefined):{readonly path:string;readonly compiledSha256:string;readonly mountedSha256:string|null;readonly current:boolean}{
  const compiledSha256=createHash("sha256").update(content).digest("hex"),bytes=read(path),mountedSha256=bytes===undefined?null:createHash("sha256").update(bytes).digest("hex");
  return {path,compiledSha256,mountedSha256,current:compiledSha256===mountedSha256};
}

/** 🧾️ Retained producer snapshots refuse coordinated original/output edits after actual Cargo execution. */
export function runtimeRetainedBuildResourceInputsV1(observation: RuntimeCargoObservationV1, resource: RuntimeCargoBuildResourcesV1, context: Parameters<typeof runtimeBuildResourceInputsV1>[1]): ReturnType<typeof runtimeBuildResourceInputsV1> & { readonly verified: boolean } {
  const evidence = runtimeBuildResourceInputsV1(resource.text, context), findings = [...evidence.findings];
  const path = (value: string): string => relative(context.root, resolve(observation.cwd,value)).replaceAll("\\","/");
  const mismatch = (value: string, detail: string): void => { findings.push({code:"runtime-input-mismatch",path:path(value),detail}); };
  if (!observation.buildScripts.some(script=>script.package_id===resource.package_id && resolve(script.out_dir)===resolve(resource.out_dir)) || resolve(context.outDirectory)!==resolve(resource.out_dir) || dirname(resolve(resource.path))!==resolve(resource.out_dir)) mismatch(resource.path,"Resource observation is not bound to the same actual Cargo build-script OUT_DIR");
  if (createHash("sha256").update(resource.text).digest("hex")!==resource.sha256 || resource.observedAtMs<observation.builtAtMs || resource.observedAtMs>observation.observedAtMs) mismatch(resource.path,"Retained resource JSONL digest/time disagrees with actual Cargo observation");
  if (resource.resources.length!==evidence.observations.length) mismatch(resource.path,"Retained source/copy snapshots do not cover the actual producer observation roster");
  for (const [index,row] of evidence.observations.entries()) {
    const retained = resource.resources[index];
    if (!retained || JSON.stringify(retained.input)!==JSON.stringify(row)) { mismatch(row.path,"Retained resource snapshot identifies a different producer input"); continue; }
    if (row.kind==="directory") {
      if (!retained.observedEntries || JSON.stringify(retained.observedEntries.map(path).sort())!==JSON.stringify(row.entries.map(path).sort())) mismatch(row.path,"Retained directory snapshot differs from actual producer entries");
    } else {
      const bytes = context.read(path(row.path)), output = context.read(path(row.output));
      if (!retained.sha256 || bytes===undefined || createHash("sha256").update(bytes).digest("hex")!==retained.sha256) mismatch(row.path,"Current original resource differs from retained build-time source digest");
      if (!retained.outputSha256 || output===undefined || createHash("sha256").update(output).digest("hex")!==retained.outputSha256 || retained.sha256!==retained.outputSha256) mismatch(row.output,"Current copied/read bytes differ from retained build-time output digest");
    }
  }
  return {...evidence,findings,verified:findings.length===0 && observation.status===0 && !observation.cancelled};
}

/** 🖼️ Binds actual build-script reads/copies and complete directory observations to current original bytes. */
export function runtimeBuildResourceInputsV1(source: string, context: { readonly root: string; readonly cwd: string; readonly outDirectory: string; readonly fixtureCollections?: readonly string[]; readonly read: (path: string) => string | Uint8Array | undefined; readonly directory: (path: string) => readonly string[] | undefined }): { readonly inputs: readonly string[]; readonly observations: readonly RuntimeBuildResourceObservationV1[]; readonly findings: readonly RuntimeGraphFindingV1[] } {
  const observations = source.split("\n").filter(Boolean).map(line => JSON.parse(line)), findings: RuntimeGraphFindingV1[] = [], inputs = new Set<string>();
  const normalized = (path: string): string => relative(context.root, resolve(context.cwd, path)).replaceAll("\\", "/"), out = normalized(context.outDirectory);
  const fixture = (path: string): void => { if (runtimeFixturePathV1(path, context.fixtureCollections)) findings.push({code:"runtime-fixture-edge",path,detail:"Actual build-producer filesystem input belongs to a fixture collection"}); };
  const digest = (bytes: string | Uint8Array | undefined): string | undefined => bytes === undefined ? undefined : createHash("sha256").update(bytes).digest("hex");
  for (const row of observations) {
    if (typeof row.path !== "string" || !["directory","copy","read"].includes(row.kind)) throw Error("Build resource observation lost its input identity");
    const path = normalized(row.path); inputs.add(path); fixture(path);
    if (row.kind === "directory") {
      if (!Array.isArray(row.entries) || !row.entries.every((value: unknown) => typeof value === "string") || Object.keys(row).sort().join(",") !== "entries,kind,path") throw Error("Build resource directory observation lost its entry roster");
      const entries = row.entries.map(normalized).sort(), actual = context.directory(path)?.map(value => normalized(resolve(context.root,value))).sort();
      for (const entry of entries) { fixture(entry); if (dirname(entry).replaceAll("\\","/") !== path) findings.push({code:"runtime-input-mismatch",path:entry,detail:"Observed entry does not belong to its actual resource directory"}); }
      if (!actual || JSON.stringify(entries) !== JSON.stringify(actual)) findings.push({code:"runtime-input-mismatch",path,detail:"Current resource directory differs from the actual build-producer entry roster"});
    } else {
      if (typeof row.output !== "string" || Object.keys(row).sort().join(",") !== "kind,output,path") throw Error("Build resource byte observation lost original/output identity");
      const output = normalized(row.output), original = digest(context.read(path)), copied = digest(context.read(output));
      if (!output.startsWith(out + "/")) findings.push({code:"runtime-input-mismatch",path:output,detail:"Observed resource output does not belong to the selected actual build-script OUT_DIR"});
      if (!original || original !== copied) findings.push({code:"runtime-input-mismatch",path,detail:"Current original resource differs from the actual copied/read snapshot bytes"});
    }
  }
  return {inputs:[...inputs].sort(),observations,findings};
}

/** 📦️ Retains actual completed Cargo unit observations without requiring deleted capture directories. */
export function runtimeCargoProvenanceV1(source: string): readonly RuntimeCargoObservationV1[] {
  const values = JSON.parse(source), observations = Array.isArray(values) ? values : [values];
  const digest = (value: unknown): boolean => typeof value === "string" && /^[0-9a-f]{64}$/u.test(value);
  for (const row of observations) {
    if (row.version !== 1 || row.command !== "cargo" || typeof row.manifest !== "string" || !isAbsolute(row.manifest) || typeof row.cwd !== "string" || !isAbsolute(row.cwd) || !Array.isArray(row.args) || !row.args.every((value: unknown) => typeof value === "string") || row.status !== 0 || row.cancelled !== false || !Number.isFinite(row.builtAtMs) || !Number.isFinite(row.observedAtMs) || row.observedAtMs < row.builtAtMs || !Array.isArray(row.units) || !Array.isArray(row.buildScripts)) throw Error("Cargo observation lost completed invocation identity");
    if (row.invocationInputs !== undefined && (!Array.isArray(row.invocationInputs) || !row.invocationInputs.every((input:any)=>typeof input.path==="string" && isAbsolute(input.path) && (input.sha256===null || digest(input.sha256))))) throw Error("Cargo observation lost invocation input digests");
    for (const unit of row.units) {
      const message = unit.message;
      if (message?.reason !== "compiler-artifact" || typeof message.package_id !== "string" || !Array.isArray(message.features) || !message.features.every((value: unknown) => typeof value === "string") || !Array.isArray(message.filenames) || !message.filenames.every((value: unknown) => typeof value === "string") || typeof message.target?.src_path !== "string" || !Array.isArray(message.target.kind) || !message.target.kind.every((value: unknown) => typeof value === "string") || typeof message.profile?.test !== "boolean" || !Number.isFinite(unit.observedAtMs) || unit.observedAtMs < row.builtAtMs || unit.observedAtMs > row.observedAtMs || !Array.isArray(unit.depInfo) || !Array.isArray(unit.inputs) || !Array.isArray(unit.artifacts)) throw Error("Cargo observation lost production unit identity");
      for (const dep of unit.depInfo) if (typeof dep.path !== "string" || typeof dep.text !== "string" || (typeof dep.baseDirectory!=="string" || !isAbsolute(dep.baseDirectory))) throw Error("Cargo observation lost dep-info content");
      for (const input of unit.inputs) if (typeof input.path !== "string" || !digest(input.sha256)) throw Error("Cargo observation lost input digest");
      for (const artifact of unit.artifacts) if (!message.filenames.includes(artifact.path) || !digest(artifact.sha256) || (artifact.stagedPath !== undefined && (typeof artifact.stagedPath !== "string" || artifact.stagedSha256 !== artifact.sha256))) throw Error("Cargo observation lost original/staged artifact binding");
    }
    for (const script of row.buildScripts) if (script.reason !== "build-script-executed" || typeof script.package_id !== "string" || typeof script.out_dir !== "string" || !Array.isArray(script.cfgs) || !script.cfgs.every((value: unknown) => typeof value === "string") || !Array.isArray(script.env) || !script.env.every((pair: unknown) => Array.isArray(pair) && pair.length === 2 && pair.every(value => typeof value === "string"))) throw Error("Cargo observation lost build-script cfg/env/OUT_DIR identity");
    if (row.buildResources !== undefined) {
      if (!Array.isArray(row.buildResources)) throw Error("Cargo observation lost build resource snapshots");
      for (const resource of row.buildResources) if (typeof resource.package_id !== "string" || typeof resource.out_dir !== "string" || typeof resource.path !== "string" || typeof resource.text !== "string" || !digest(resource.sha256) || !Number.isFinite(resource.observedAtMs) || resource.observedAtMs < row.builtAtMs || resource.observedAtMs > row.observedAtMs || !Array.isArray(resource.resources) || !resource.resources.every((value: any) => value && typeof value.input === "object" && (value.sha256 === null || digest(value.sha256)) && (value.outputSha256 === null || digest(value.outputSha256)) && (value.observedEntries === undefined || Array.isArray(value.observedEntries) && value.observedEntries.every((path: unknown) => typeof path === "string")))) throw Error("Cargo observation lost build-time resource identity/digests");
    }
  }
  return observations;
}

/** 🧭️ Derives target and requested feature selection from the original completed Cargo invocation. */
export function runtimeCargoInvocationV1(observation: RuntimeCargoObservationV1, root: string): { readonly target: string | null; readonly workspace: string; readonly features: readonly string[]; readonly packages: readonly string[]; readonly defaults: boolean; readonly allFeatures: boolean } {
  const values = (name: string, short?: string): string[] => observation.args.flatMap((value, index) => value === name || value === short ? [observation.args[index + 1] ?? ""] : value.startsWith(name + "=") ? [value.slice(name.length + 1)] : []);
  const manifests = values("--manifest-path");
  if (manifests.length > 1 || (manifests.length && resolve(observation.cwd, manifests[0]!) !== resolve(observation.manifest))) throw Error("Cargo observation manifest disagrees with original invocation");
  const targets = values("--target");
  if (targets.length > 1) throw Error("Cargo observation has ambiguous target selection");
  return { target: targets[0] ?? null, workspace: relative(root, observation.manifest).replaceAll("\\", "/"), features: [...new Set(values("--features").flatMap(value => value.split(/[\s,]+/u).filter(Boolean)))].sort(), packages: values("--package", "-p").sort(), defaults: !observation.args.includes("--no-default-features"), allFeatures: observation.args.includes("--all-features") };
}

/** ♻️ Reuses only the newest current observation of each exact declared runtime invocation. */
export function runtimeCargoObservationsForChecksV1(observations:readonly RuntimeCargoObservationV1[],checks:readonly {workspace:string;target:string;packages:readonly string[];features:readonly string[]}[],root:string,current:(observation:RuntimeCargoObservationV1)=>boolean):readonly RuntimeCargoObservationV1[]{
  return checks.flatMap(check=>{
    const candidates=observations.filter(observation=>{
      const selection=runtimeCargoInvocationV1(observation,root);
      return selection.workspace===check.workspace && selection.target===check.target && selection.defaults && !selection.allFeatures && JSON.stringify(selection.packages)===JSON.stringify([...check.packages].sort()) && JSON.stringify(selection.features)===JSON.stringify([...check.features].sort()) && current(observation);
    }).sort((a,b)=>b.observedAtMs-a.observedAtMs);
    return candidates[0]?[candidates[0]]:[];
  });
}

/** 🌱️ Requires surviving artifacts and complete current Cargo configuration before automatic reuse. */
export function runtimeCargoObservationCurrentV1(observation:RuntimeCargoObservationV1,context:Parameters<typeof runtimeBuildResourceInputsV1>[1]&{buildDirectory:string;cargoHome:string;digest:(path:string)=>string|undefined}):boolean {
  if (!observation.buildDirectory || resolve(observation.buildDirectory)!==resolve(context.buildDirectory) || !observation.invocationInputs?.length) return false;
  const units=observation.units.filter(unit=>!unit.message.profile.test), manifests=[observation.manifest,...units.map(unit=>unit.message.manifest_path)];
  if (!units.length || manifests.some(path=>typeof path!=="string")) return false;
  const captured=new Map(observation.invocationInputs.map(input=>[resolve(input.path),input.sha256]));
  if (observation.invocationInputs.some(input=>(context.digest(input.path)??null)!==input.sha256)) return false;
  for (let path of [observation.cwd,...(manifests as string[]).map(dirname)]) for (;;) {
    for (const name of ["Cargo.toml","Cargo.lock",".cargo/config.toml",".cargo/config","rust-toolchain.toml","rust-toolchain"]) if (!captured.has(join(path,name))) return false;
    const parent=dirname(path); if(parent===path) break; path=parent;
  }
  if (["config.toml","config"].some(name=>!captured.has(join(context.cargoHome,name)))) return false;
  if ((manifests as string[]).some(path=>!captured.has(resolve(path)))) return false;
  const packageNames=new Set(units.filter(unit=>unit.message.target.kind.some(kind=>["lib","rlib","cdylib","bin","proc-macro"].includes(kind))).flatMap(unit=>{try{const name=(Bun.TOML.parse(String(context.read(unit.message.manifest_path!))) as {package?:{name?:string}}).package?.name;return name?[name]:[]}catch{return []}}));
  if (runtimeCargoInvocationV1(observation,context.root).packages.some(name=>!packageNames.has(name))) return false;
  if (units.some(unit=>unit.artifacts.some(artifact=>context.digest(artifact.stagedPath??artifact.path)!==artifact.sha256) || !runtimeCargoUnitInputsV1(observation,unit,context.root,path=>context.digest(resolve(context.root,path))).verified)) return false;
  return (observation.buildResources??[]).every(resource=>runtimeRetainedBuildResourceInputsV1(observation,resource,{...context,outDirectory:resource.out_dir}).verified);
}

/** 🏭️ Preserves actual build-script cfg/environment records in their selected guest invocation. */
export function runtimeCargoBuildScriptsV1(source: string): readonly (RuntimeCargoBuildScriptV1 & { readonly runtimeTarget: string | null; readonly runtimeWorkspace: string | null })[] {
  let runtimeTarget: string | null = null, runtimeWorkspace: string | null = null;
  return source.split("\n").flatMap(line => {
    const selection = /guest-framework-check (wasm32-[a-z0-9-]+) \(([^)]+)\):/u.exec(line);
    if (selection) { runtimeTarget = selection[1]!; runtimeWorkspace = selection[2]!; }
    const start = line.indexOf('{"reason":"build-script-executed"');
    if (start < 0) return [];
    const row = JSON.parse(line.slice(start));
    if (typeof row.package_id !== "string" || typeof row.out_dir !== "string" || !Array.isArray(row.cfgs) || !Array.isArray(row.env)) throw Error("Build-script record lost cfg/env/OUT_DIR identity");
    return [{ ...row, runtimeTarget, runtimeWorkspace }];
  });
}

/** 🔒️ Binds retained dep-info to every current compiler input and any surviving staged artifact. */
export function runtimeCargoUnitInputsV1(observation: RuntimeCargoObservationV1, unit: RuntimeCargoUnitV1, root: string, digest: (path: string) => string | undefined): { readonly inputs: readonly string[]; readonly findings: readonly RuntimeGraphFindingV1[]; readonly verified: boolean } {
  const findings: RuntimeGraphFindingV1[] = [], path = (value: string): string => relative(root, resolve(observation.cwd, value)).replaceAll("\\", "/");
  const inputs = [...new Set(unit.depInfo.flatMap(dep => runtimeDepInfoInputsV1(dep.text, dep.baseDirectory).map(value => relative(root,resolve(dep.baseDirectory,value)).replaceAll("\\","/"))))].sort();
  const hashes = new Map(unit.inputs.map(input => [path(input.path), input.sha256]));
  if (!inputs.length || !inputs.includes(path(unit.message.target.src_path)) || !unit.artifacts.length) findings.push({ code: "runtime-unresolved-edge", path: path(unit.message.target.src_path), detail: "Retained production unit lacks complete root/dep-info/artifact observation" });
  for (const input of inputs) if (!hashes.has(input) || digest(input) !== hashes.get(input)) findings.push({ code: "runtime-input-mismatch", path: input, detail: "Current compiler input differs from retained actual unit digest" });
  for (const input of hashes.keys()) if (!inputs.includes(input)) findings.push({ code: "runtime-input-mismatch", path: input, detail: "Retained input digest is absent from actual dep-info roster" });
  for (const artifact of unit.artifacts) {
    const actual = digest(path(artifact.stagedPath ?? artifact.path));
    if ((artifact.stagedPath !== undefined && actual === undefined) || (actual !== undefined && actual !== artifact.sha256)) findings.push({ code: "runtime-input-mismatch", path: path(artifact.stagedPath ?? artifact.path), detail: "Current artifact differs from observed original/staged artifact digest" });
  }
  return { inputs, findings, verified: findings.length === 0 && unit.message.profile.test === false };
}

/** 🏗️ Tracks host build/proc-macro units separately from target libraries and their dependency closure. */
export function runtimeCargoUnitsV1(roots: readonly string[], packages: readonly CargoPackageV1[], nodes: readonly CargoNodeV1[]): readonly { readonly id: string; readonly host: boolean }[] {
  const pending = roots.map(id => ({ id, host: false })), units = new Map<string, { id: string; host: boolean }>();
  for (const unit of pending) {
    if (packages.find(pkg => pkg.id === unit.id)?.targets.some(target => target.kind.includes("proc-macro"))) unit.host = true;
    const key = unit.id + "|" + unit.host;
    if (units.has(key)) continue;
    units.set(key, unit);
    for (const dependency of nodes.find(node => node.id === unit.id)?.deps ?? []) for (const kind of dependency.dep_kinds) if (kind.kind !== "dev") pending.push({ id: dependency.pkg, host: unit.host || kind.kind === "build" });
  }
  return [...units.values()];
}

/** 📎️ Decodes actual rustc dep-info input paths without splitting escaped filesystem spaces. */
export function runtimeDepInfoInputsV1(source: string, cwd: string): readonly string[] {
  const line = source.replaceAll("\\\n", "").split("\n").find(row => row.includes(": "));
  if (!line) throw Error("Actual rustc dep-info has no input record");
  const body = line.slice(line.indexOf(": ") + 2), paths: string[] = [];
  let path = "";
  for (let index = 0; index <= body.length; index++) {
    const character = body[index];
    if (character === "\\" && index + 1 < body.length) path += body[++index];
    else if (character === undefined || /\s/u.test(character)) { if (path) paths.push(relative(cwd, resolve(cwd, path)).replaceAll("\\", "/")); path = ""; }
    else path += character;
  }
  return [...new Set(paths)].sort();
}

/** 🧾️ Normal and build dependency edges exclude test-only dependencies without guessing from crate names. */
export function runtimeCargoPackagesV1(roots: readonly string[], nodes: readonly CargoNodeV1[]): readonly string[] {
  const index = new Map(nodes.map(node => [node.id, node])), reached = new Set<string>(), pending = [...roots];
  for (const id of pending) { if (reached.has(id)) continue; reached.add(id); for (const dependency of index.get(id)?.deps ?? []) if (dependency.dep_kinds.some(kind => kind.kind !== "dev")) pending.push(dependency.pkg); }
  return [...reached].sort();
}

/** 🧊️ Reads complete compiler-artifact records, preserving resolved feature and target identities. */
export function runtimeCompilerArtifactsV1(source: string): readonly CompilerInputV1[] {
  let runtimeTarget: string | null = null, runtimeWorkspace: string | null = null;
  return source.split("\n").flatMap(line => {
    const selection = /guest-framework-check (wasm32-[a-z0-9-]+) \(([^)]+)\):/u.exec(line);
    if (selection) { runtimeTarget = selection[1]!; runtimeWorkspace = selection[2]!; }
    const start = line.indexOf('{"reason":"compiler-artifact"');
    if (start < 0) return [];
    let row; try { row = JSON.parse(line.slice(start)); } catch { throw Error("Incomplete compiler-artifact JSON record"); }
    if (!Array.isArray(row.features) || typeof row.package_id !== "string" || typeof row.target?.src_path !== "string" || typeof row.profile?.test !== "boolean") throw Error("Compiler artifact lost feature/target/profile identity");
    return [{ ...row, runtimeTarget, runtimeWorkspace }];
  });
}

/** 🧩️ Reconciles only unknown macro expansion against the complete selected production unit input roster. */
export function reconcileRuntimeMacroInputsV1(evidence: RuntimeGraphEvidenceV1, inputs: readonly string[]): RuntimeGraphEvidenceV1 {
  const compiled = new Set(inputs), root = evidence.roots[0];
  if (!root || !evidence.roots.every(path => compiled.has(path))) return evidence;
  return { ...evidence, nodes: [...new Set([...evidence.nodes, ...inputs])].sort(), edges: [...evidence.edges, ...inputs.filter(path => !evidence.nodes.includes(path)).map(to => ({ from: root, to, kind: "dependency" as const }))], findings: evidence.findings.filter(finding => !(finding.code === "runtime-unresolved-edge" && compiled.has(finding.path) && finding.detail.endsWith("! requires actual expansion/input provenance"))) };
}

/** 🛡️ Executes actual Cargo target metadata and runtime source graphs, retaining qualified evidence rather than inferring compilation. */
export async function verifyRuntimeFixtureGraphV1(root: string, args: readonly string[]): Promise<void> {
  for (let index = 0; index < args.length; index += 2) if (!["--report", "--compiler-artifacts", "--compiler-provenance", "--asset-owners", "--bundle-inputs", "--dynamic-imports"].includes(args[index]!) || !args[index + 1]) throw Error("runtime-graph accepts --report, --compiler-artifacts, --compiler-provenance, --asset-owners, --dynamic-imports and --bundle-inputs paths");
  const option = (name: string): string | undefined => { const index = args.indexOf(name); if (index < 0) return; if (!args[index + 1]) throw Error(`${name} needs a path`); return resolve(root, args[index + 1]!); };
  const report = option("--report") ?? join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? repoCacheDirectory(root,"runtime-fixture-graph","reports"),"runtime-fixture-graph.json");
  const artifacts = option("--compiler-artifacts"), artifactText = artifacts ? readFileSync(artifacts, "utf8") : "", compiler = [...runtimeCompilerArtifactsV1(artifactText)], buildScripts = [...runtimeCargoBuildScriptsV1(artifactText)];
  const provenance = option("--compiler-provenance"), receiptText = provenance ? readFileSync(provenance, "utf8") : undefined;
  const receiptPaths = receiptText && Array.isArray(JSON.parse(receiptText)) && JSON.parse(receiptText).every((value: unknown) => typeof value === "string") ? JSON.parse(receiptText).map((path: string) => resolve(dirname(provenance!), path)) : provenance ? [provenance] : [];
  const observations = receiptPaths.flatMap((path: string) => runtimeCargoProvenanceV1(readFileSync(path, "utf8")));
  const retainedUnits = new Map<CompilerInputV1, { readonly observation: RuntimeCargoObservationV1; readonly unit: RuntimeCargoUnitV1; readonly selection: ReturnType<typeof runtimeCargoInvocationV1> }>();
  const retainedBuildScripts = new Map<(typeof buildScripts)[number], RuntimeCargoObservationV1>();
  const assets = option("--asset-owners"), assetOwners = assets ? JSON.parse(readFileSync(assets, "utf8")) : {};
  const bundle = option("--bundle-inputs"), bundledInputs = bundle ? JSON.parse(readFileSync(bundle, "utf8")) : undefined;
  const dynamic = option("--dynamic-imports"), dynamicImports = dynamic ? JSON.parse(readFileSync(dynamic, "utf8")) : undefined;
  const config = JSON.parse(readFileSync(join(import.meta.dir, "../🔣️.json"), "utf8"));
  const declaration = JSON.parse(readFileSync(join(root, config.guestChecks), "utf8"));
  const taxonomy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
  const fixtureCollections = ["fixtures", "test-tube-fixtures"].flatMap(id => ["fixtures", "examples"].filter(slug => new RegExp(taxonomy.semanticDirectoryKinds[id].slugPattern, "u").test(slug)).map(slug => taxonomy.semanticDirectoryKinds[id].emoji + slug));
  const checks = declaration.guestFrameworkChecks as readonly { readonly target: string; readonly workspace: string; readonly packages: readonly string[]; readonly features: readonly string[] }[];
  const durableLedger=join(cargoDirectories(root).build,"semio-cargo-provenance");
  const findings: RuntimeGraphFindingV1[] = [], graphs: { readonly selection: unknown; readonly evidence: RuntimeGraphEvidenceV1; readonly resolvedFeatures?: readonly string[] }[] = [], identities = new Map<string, string>();
  const signal = new AbortController(), cancel = (): void => signal.abort();
  process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
  const checkCancellation = (): void => { if (signal.signal.aborted) throw Error("Runtime graph canceled"); };
  const read = (path: string): string | undefined => {
    checkCancellation(); const absolute = resolve(root, path);
    if (!existsSync(absolute)) return;
    const actual = realpathSync(absolute), rel = relative(root, actual).replaceAll("\\", "/");
    if (runtimeFixturePathV1(rel, fixtureCollections) && !runtimeFixturePathV1(path, fixtureCollections)) findings.push({ code: "runtime-fixture-edge", path, detail: `Physical source/resource resolves to fixture owner ${rel}` });
    try { const bytes = readFileSync(actual),sha256=createHash("sha256").update(bytes).digest("hex"),previous=identities.get(rel); if(previous && previous!==sha256)findings.push({code:"runtime-input-mismatch",path:rel,detail:"Source/resource changed during actual runtime graph observation"});else identities.set(rel,sha256); return bytes.toString("utf8"); } catch { return; }
  };
  const execute = async (command: string[]): Promise<string> => {
    checkCancellation(); if(command[0]==="cargo") prepareCargoWorkspaceInvocation(root,command.slice(1),root,process.env); const child = Bun.spawn(command, { cwd: root, stdout: "pipe", stderr: "pipe", signal: signal.signal });
    const [stdout, stderr, exit] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    if (exit !== 0) throw Error(`${command.join(" ")} failed ${exit}: ${stderr}`); return stdout;
  };
  try {
    if (!provenance && !artifacts) {
      const ledger = durableLedger, digest=(path:string):string|undefined=>{try{return createHash("sha256").update(readFileSync(path)).digest("hex")}catch{return undefined}};
      const load=():readonly RuntimeCargoObservationV1[]=>existsSync(ledger)?readdirSync(ledger).filter(name=>name.startsWith("cargo-unit-provenance-")&&name.endsWith(".json")).flatMap(name=>{try{return runtimeCargoProvenanceV1(readFileSync(join(ledger,name),"utf8"))}catch{return []}}):[];
      const current=(observation:RuntimeCargoObservationV1):boolean=>runtimeCargoObservationCurrentV1(observation,{root,cwd:observation.cwd,outDirectory:"",buildDirectory:cargoDirectories(root).build,cargoHome:resolve(process.env.CARGO_HOME??join(homedir(),".cargo")),digest,fixtureCollections,read:path=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directory:path=>{try{return readdirSync(resolve(root,path)).map(name=>path+"/"+name)}catch{return undefined}}});
      let completed=runtimeCargoObservationsForChecksV1(load(),checks,root,current);
      if (completed.length!==checks.length) {
        const producer=declaration.steps.find((step:any)=>step.command?.[0]==="nx"&&step.command?.[1]==="run"&&step.command?.[2]?.endsWith(":guest-framework-check"));
        if (!producer) throw Error("Runtime graph has no registered actual guest compiler producer");
        console.log(`[runtime graph] acquiring current exact guest compiler/resource witnesses through ${producer.command[2]}`);
        mkdirSync(ledger,{recursive:true});
        await runTool(process.execPath,[join(root,"node_modules/nx/dist/bin/nx.js"),...producer.command.slice(1),"--skip-nx-cache","--outputStyle=static"],root,signal.signal,false,{...process.env,CARGO_TARGET_DIR:cargoDirectories(root).target,CARGO_BUILD_BUILD_DIR:cargoDirectories(root).build,SEMIO_TEST_ARTIFACT_DIR:ledger});
        completed=runtimeCargoObservationsForChecksV1(load(),checks,root,current);
        if (completed.length!==checks.length) throw Error("Registered guest compiler producer did not retain all current exact invocation/input/resource witnesses");
      }
      observations.push(...completed);
    }
  for (const observation of observations) {
    const selection = runtimeCargoInvocationV1(observation, root);
    for (const unit of observation.units) { const message = { ...unit.message, runtimeTarget: selection.target, runtimeWorkspace: selection.workspace }; compiler.push(message); retainedUnits.set(message, { observation, unit, selection }); }
    for (const script of observation.buildScripts) { const record = { ...script, runtimeTarget: selection.target, runtimeWorkspace: selection.workspace }; buildScripts.push(record); retainedBuildScripts.set(record, observation); }
  }
    const workspaceFiles = policyWalkRelFiles(root, ["🧰️framework", "✏️s", "🌎️hub"], (_, name) => name === "package.json");
    const aliases: Record<string, string> = {};
    for (const path of workspaceFiles) {
      const pkg = JSON.parse(readFileSync(join(root, path), "utf8"));
      const exported = (value: unknown): string | undefined => {
        if (typeof value === "string") return value;
        if (!value || typeof value !== "object") return;
        for (const [condition, target] of Object.entries(value)) if (["browser", "import", "default", "module"].includes(condition)) { const path = exported(target); if (path) return path; }
      };
      for (const [key, value] of Object.entries(typeof pkg.exports === "string" ? { ".": pkg.exports } : pkg.exports ?? {})) { const entry = exported(value); if (entry && pkg.name) aliases[pkg.name + (key === "." ? "" : key.slice(1))] = relative(root, resolve(dirname(join(root, path)), entry)).replaceAll("\\", "/"); }
    }
    const productionBoundary = read(config.productionTestBoundaryOwner);
    if (!productionBoundary?.includes('"import.meta.vitest": "undefined"')) throw Error("Actual browser production test boundary is unavailable");
    const ts = inspectRuntimeGraphV1(config.typescriptEntries, { read, fixtureCollections, aliases, assetOwners, dynamicImports, compiledInputs: bundledInputs, workingDirectory: ".", productionTests: "excluded", checkCancellation });
    graphs.push({ selection: { language: "typescript", entries: config.typescriptEntries }, evidence: ts }); findings.push(...ts.findings);
    try {
      const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
      const { renderWgpuBrowserBundles } = await import(join(root, config.browserProducer));
      const browser = await renderWgpuBrowserBundles(root, profile, { taxonomy, isCancelled: () => signal.signal.aborted });
      const browserAliases = Object.fromEntries(Object.entries(profile.workspaceImports).map(([name, value]: [string, any]) => [name, value.entryPath]));
      const browserRoots = profile.entries.map((entry: any) => profile.ownerPath + "/" + entry.sourceRelativePath);
      const outputOwners = Object.fromEntries(profile.entries.map((entry: any)=>[profile.ownerPath+"/"+entry.outputRelativePath,profile.ownerPath+"/"+entry.sourceRelativePath]));
      const { wgpuBrowserMounts } = await import(join(root,config.browserMountSource));
      const { wgpuCompletedFrameworkRootsV1, wgpuCompletedProfileV1 } = await import(join(root,config.browserMountConfig));
      read(config.browserMountSource); read(config.browserMountConfig);
      const completedProfile = wgpuCompletedProfileV1(), roots = wgpuCompletedFrameworkRootsV1(completedProfile);
      const resourceMounts = Object.fromEntries(wgpuBrowserMounts(roots).filter((row: unknown[])=>typeof row[1]==="string").map(([route,path]:[string,string])=>[route,relative(root,path).replaceAll("\\","/")]));
      const outputs = new Map<string,string>(browser.nodes.map((node:any)=>[node.path,node.content]));
      const mountedOutputs=browser.nodes.map((node:any)=>runtimeBrowserArtifactV1(node.path,node.content,read));
      for(const artifact of mountedOutputs)if(!artifact.current)findings.push({code:"runtime-unresolved-edge",path:artifact.path,detail:artifact.mountedSha256===null?"Fresh browser compiler output is not present at its actual server mount":"Actual server-mounted browser bytes differ from fresh compiler output"});
      const outputUrls = Object.fromEntries(browser.entryInputs.map((entry:any)=>{
        const mount = Object.entries(resourceMounts).find(([,path])=>path===dirname(entry.outputPath).replaceAll("\\","/"));
        if (!mount) throw Error("Actual browser output has no server mount owner: "+entry.outputPath);
        return [entry.outputPath,"https://runtime.invalid"+mount[0]+"/"+entry.outputPath.slice(entry.outputPath.lastIndexOf("/")+1)];
      }));
      const browserRead = (path:string):string|undefined=>outputs.get(path)??read(path);
      const browserNodes = new Set<string>(), browserEdges: {from:string;to:string;kind:"import"|"module"|"resource"|"dependency"|"generated"}[] = [];
      for (const entry of browser.entryInputs) {
        const source = outputOwners[entry.outputPath], moduleUrls = {...outputUrls,...Object.fromEntries(entry.inputs.map((path:string)=>[path,outputUrls[entry.outputPath]]))};
        const evidence = inspectRuntimeGraphV1([source], {read:browserRead,fixtureCollections,aliases:browserAliases,assetOwners,dynamicImports,moduleUrls,resourceMounts,workingDirectory:".",productionTests:"excluded",checkCancellation});
        evidence.nodes.forEach(path=>browserNodes.add(path)); browserEdges.push(...evidence.edges); findings.push(...evidence.findings);
        graphs.push({selection:{language:"typescript",producer:config.browserProducer,entry,source,realm:outputUrls[entry.outputPath],completedProfile,mountOwners:{source:config.browserMountSource,config:config.browserMountConfig,resourceMounts},outputs:mountedOutputs},evidence});
      }
      for (const binding of Object.values(profile.workspaceImports) as { readonly entryPath: string; readonly manifestPath: string }[]) if (browserNodes.has(binding.entryPath)) { browserNodes.add(binding.manifestPath); browserEdges.push({ from: binding.entryPath, to: binding.manifestPath, kind: "dependency" }); read(binding.manifestPath); }
      for (const path of browser.inputs) { read(path); if (!browserNodes.has(path)) findings.push({ code: "runtime-input-mismatch", path, detail: "Actual WGPU browser compiler input has no resolved source/resource graph edge" }); }
      const mountedResources = new Set(browserEdges.filter(edge=>edge.kind==="resource").map(edge=>edge.to));
      for (const path of browserNodes) if (!browser.inputs.includes(path) && !outputs.has(path)) findings.push({ code: mountedResources.has(path)?"runtime-unresolved-edge":"runtime-input-mismatch", path, detail: mountedResources.has(path)?"Actual browser runtime mounted resource needs current native producer/artifact ownership evidence":"WGPU source graph input is absent from actual browser compiler input roster" });
    } catch (error) { findings.push({ code: "runtime-unresolved-edge", path: config.browserProducer, detail: `Actual WGPU browser producer failed: ${String(error)}${error instanceof AggregateError ? "\n" + error.errors.map(String).join("\n") : ""}` }); }
    const hostCfg = (await execute(["rustc", "--print", "cfg"])).trim().split("\n");
    for (const [index, check] of checks.entries()) {
      console.log(`[runtime graph] actual Cargo metadata ${index + 1}/${checks.length} ${check.target}: ${check.packages.join(",")}`);
      const arguments_ = ["cargo", "metadata", "--locked", "--manifest-path", check.workspace, "--format-version", "1", "--filter-platform", check.target, ...(check.features.length ? ["--features", check.features.join(",")] : [])];
      const metadataText = await execute(arguments_), metadata = JSON.parse(metadataText) as { workspace_root:string; packages: CargoPackageV1[]; resolve: { nodes: CargoNodeV1[] } };
      mkdirSync(dirname(report), { recursive: true }); writeFileSync(join(dirname(report), `runtime-cargo-metadata-${index}.json`), metadataText);
      const cfg = (await execute(["rustc", "--print", "cfg", "--target", check.target])).trim().split("\n");
      const selected = check.packages.map(name => { const packages = metadata.packages.filter(pkg => pkg.name === name); if (packages.length !== 1) throw Error(`Selected Cargo package ${name} is not unique`); return packages[0]!.id; });
      for (const { id, host } of runtimeCargoUnitsV1(selected, metadata.packages, metadata.resolve.nodes)) {
        const pkg = metadata.packages.find(pkg => pkg.id === id)!;
        if (pkg.source !== null) continue;
        const manifest = relative(root, pkg.manifest_path).replaceAll("\\", "/"); read(manifest);
        const messages = compiler.filter(row => {
          const retained = retainedUnits.get(row), selection = retained?.selection;
          return row.package_id === id && row.profile.test === false && row.runtimeTarget === check.target && row.runtimeWorkspace === check.workspace && (!selection || selection.defaults && !selection.allFeatures && JSON.stringify(selection.features) === JSON.stringify([...check.features].sort()) && JSON.stringify(selection.packages) === JSON.stringify([...check.packages].sort())) && pkg.targets.some(target => target.src_path === row.target.src_path) && row.target.kind.some(kind => ["lib", "rlib", "cdylib", "proc-macro"].includes(kind)) && row.filenames.some(path => host !== path.replaceAll("\\", "/").includes(`/${check.target}/`));
        });
        if (!messages.length) findings.push({ code: "runtime-unresolved-edge", path: manifest, detail: `No actual compiler-artifact feature witness for ${check.target} ${pkg.name}; metadata workspace/dev feature union is not runtime compilation proof` });
        const features = messages.length ? [...new Set(messages.flatMap(row => row.features))].sort() : metadata.resolve.nodes.find(node => node.id === id)?.features ?? [];
        const variants = new Set(messages.map(message => JSON.stringify([...message.features].sort())));
        if (variants.size > 1) findings.push({ code: "runtime-unresolved-edge", path: manifest, detail: "Production compiler witnesses have different resolved feature units; a package feature union cannot prove runtime ownership" });
        const roots = pkg.targets.filter(target => target.kind.some(kind => ["lib", "rlib", "cdylib", "proc-macro"].includes(kind))).map(target => relative(root, target.src_path).replaceAll("\\", "/"));
        const boundObservations = new Set(messages.flatMap(message => retainedUnits.has(message) ? [retainedUnits.get(message)!.observation] : []));
        const scripts = [...new Map(buildScripts.filter(script => script.package_id === id && script.runtimeTarget === check.target && script.runtimeWorkspace === check.workspace && (!retainedBuildScripts.has(script) || boundObservations.has(retainedBuildScripts.get(script)!))).map(script => [JSON.stringify([script.out_dir, script.cfgs, script.env]), script])).values()];
        if (scripts.length > 1) findings.push({ code: "runtime-unresolved-edge", path: manifest, detail: "Actual build-script records have multiple cfg/env/OUT_DIR units; production units cannot share an aggregate environment" });
        const script = scripts.length === 1 ? scripts[0] : undefined;
        const digest = (path: string): string | undefined => { try { return createHash("sha256").update(readFileSync(resolve(root, path))).digest("hex"); } catch { return; } };
        const resourceWitnesses = [...boundObservations].flatMap(observation=>(observation.buildResources ?? []).filter(resource=>resource.package_id===id && script && resolve(resource.out_dir)===resolve(script.out_dir)).map(resource=>{
          const producers = observation.units.filter(unit=>unit.message.package_id===id && unit.message.target.kind.includes("custom-build") && !unit.message.profile.test);
          const producerInputs = producers.map(unit=>runtimeCargoUnitInputsV1(observation,unit,root,digest));
          const evidence = runtimeRetainedBuildResourceInputsV1(observation,resource,{root,cwd:observation.cwd,outDirectory:resource.out_dir,fixtureCollections,read:path=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directory:path=>{try{return readdirSync(resolve(root,path)).map(name=>path+"/"+name)}catch{return undefined}}});
          findings.push(...evidence.findings,...producerInputs.flatMap(unit=>unit.findings));
          const verified = evidence.verified && producers.length===1 && producerInputs.every(unit=>unit.verified);
          if (!verified) findings.push({code:"runtime-unresolved-edge",path:manifest,detail:"Original build resource snapshots lack the same current verified custom-build production unit"});
          return {receipt:resource,producerUnits:producers.map(unit=>({source:unit.message.target.src_path,features:unit.message.features,inputs:unit.inputs,artifacts:unit.artifacts})),evidence,verified};
        }));
        const generated = Object.fromEntries(resourceWitnesses.filter(witness=>witness.verified).flatMap(witness=>witness.evidence.observations.flatMap(row=>row.kind==="directory"?[]:[[relative(root,resolve(row.output)).replaceAll("\\","/"),[relative(root,resolve(row.path)).replaceAll("\\","/")]]])));
        const resourceInputs = new Set(resourceWitnesses.filter(witness=>witness.verified).flatMap(witness=>witness.evidence.inputs));
        const context = { read, fixtureCollections, features, cfg: [...(host ? hostCfg : cfg), ...(script?.cfgs ?? [])], environment: script ? { ...Object.fromEntries(script.env), OUT_DIR: script.out_dir.replaceAll("\\", "/") } : undefined, generated, rootDirectory: root.replaceAll("\\", "/"), manifestDirectory: dirname(pkg.manifest_path).replaceAll("\\", "/"), checkCancellation };
        let evidence = inspectRuntimeGraphV1(roots, context);
        const inputs = new Set<string>();
        const witnesses: { readonly artifact: string; readonly artifactSha256: string; readonly depInfo: string; readonly depInfoSha256: string }[] = [];
        let fresh = variants.size <= 1 && scripts.length <= 1;
        const retainedWitnesses: unknown[] = [];
        for (const message of messages) {
          const retained = retainedUnits.get(message);
          if (retained) {
            const observed = runtimeCargoUnitInputsV1(retained.observation, retained.unit, root, digest);
            findings.push(...observed.findings); fresh &&= observed.verified;
            retainedWitnesses.push({ manifest: retained.observation.manifest, cwd: retained.observation.cwd, args: retained.observation.args, builtAtMs: retained.observation.builtAtMs, observedAtMs: retained.unit.observedAtMs, depInfo: retained.unit.depInfo, artifacts: retained.unit.artifacts, inputDigests: retained.unit.inputs, verified: observed.verified });
            for (const input of observed.inputs) { inputs.add(input); read(input); if (runtimeFixturePathV1(input, fixtureCollections)) findings.push({ code: "runtime-fixture-edge", path: input, detail: "Actual retained compiler input belongs to a fixture collection" }); }
            continue;
          }
          for (const file of message.filenames) {
          const depInfo = join(dirname(file), basename(file).replace(/^lib/u, "").replace(/\.[^.]+$/u, ".d"));
          if (!existsSync(depInfo) || !existsSync(file)) continue;
          witnesses.push({ artifact: file, artifactSha256: createHash("sha256").update(readFileSync(file)).digest("hex"), depInfo, depInfoSha256: createHash("sha256").update(readFileSync(depInfo)).digest("hex") });
          for (const input of runtimeDepInfoInputsV1(readFileSync(depInfo, "utf8"), metadata.workspace_root).map(path=>relative(root,resolve(metadata.workspace_root,path)).replaceAll("\\","/"))) {
            inputs.add(input); read(input);
            if (runtimeFixturePathV1(input, fixtureCollections)) findings.push({ code: "runtime-fixture-edge", path: input, detail: `Actual compiled ${host ? "host" : check.target} input belongs to a fixture collection` });
            if (!existsSync(join(root, input)) || statSync(join(root, input)).mtimeMs > statSync(file).mtimeMs) { fresh = false; findings.push({ code: "runtime-input-mismatch", path: input, detail: "Current input is missing or changed after actual compiler artifact" }); }
          }
          }
        }
        if (messages.length && !inputs.size) findings.push({ code: "runtime-unresolved-edge", path: manifest, detail: "Actual compiler artifact has no readable dep-info source/resource roster" });
        const macroInputWitnesses = fresh && inputs.size && roots.every(path => inputs.has(path)) ? evidence.findings.filter(finding => finding.code === "runtime-unresolved-edge" && inputs.has(finding.path) && finding.detail.endsWith("! requires actual expansion/input provenance")) : [];
        if (macroInputWitnesses.length) {
          const expanded = inspectRuntimeGraphV1([...roots, ...inputs], context);
          evidence = reconcileRuntimeMacroInputsV1({ ...expanded, roots }, [...inputs]);
        }
        if (inputs.size) {
          for (const input of evidence.nodes) if (!inputs.has(input) && !resourceInputs.has(input)) findings.push({ code: "runtime-input-mismatch", path: input, detail: "Source graph input is absent from actual compiler dep-info or verified original build-resource roster" });
          for (const input of inputs) if (!evidence.nodes.includes(input)) findings.push({ code: "runtime-input-mismatch", path: input, detail: "Actual compiler dep-info input has no resolved source/generated graph edge" });
        }
        graphs.push({ selection: { target: check.target, host, packages: check.packages, features: check.features, manifest, package: pkg.name, featureWitness: messages.length ? "compiler-artifact" : "metadata-broad", compilerUnits: messages.map(message => ({ srcPath: message.target.src_path, profile: message.profile, features: message.features, filenames: message.filenames })), compilerWitnesses: witnesses, retainedWitnesses, buildScript: script ?? null, resourceWitnesses, macroInputWitnesses, compiledInputs: [...inputs] }, evidence, resolvedFeatures: features });
        findings.push(...evidence.findings.map(finding => messages.length && variants.size <= 1 || finding.code !== "runtime-fixture-edge" ? finding : { ...finding, code: "runtime-unresolved-edge" as const, detail: "Potential fixture resource under broad/ambiguous features; a single actual production feature unit is unavailable" }));
        for (const target of pkg.targets.filter(target => target.kind.includes("custom-build"))) {
          const source = relative(root, target.src_path).replaceAll("\\", "/"), build = inspectRuntimeGraphV1([source], { read, features, cfg: hostCfg, manifestDirectory: dirname(pkg.manifest_path).replaceAll("\\", "/"), checkCancellation });
          graphs.push({ selection: { target: "host", package: pkg.name, manifest, kind: "custom-build" }, evidence: build, resolvedFeatures: features });
          findings.push(...build.findings.map(finding => finding.code === "runtime-fixture-edge" ? { ...finding, code: "runtime-unresolved-edge" as const, detail: "Build-script fixture candidate requires actual generated-output/input provenance" } : finding));
        }
      }
    }
  } catch(error) {
    findings.push({code:"runtime-unresolved-edge",path:config.guestChecks,detail:`Actual runtime witness acquisition/verification failed: ${String(error)}`});
  } finally {
    for(const[path,sha256]of identities)try{if(createHash("sha256").update(readFileSync(resolve(root,path))).digest("hex")!==sha256)findings.push({code:"runtime-input-mismatch",path,detail:"Source/resource changed before actual runtime graph completion"});}catch{findings.push({code:"runtime-input-mismatch",path,detail:"Source/resource disappeared before actual runtime graph completion"});}
    process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel);
    mkdirSync(dirname(report), { recursive: true }); writeFileSync(report, JSON.stringify({ generatedAt: new Date().toISOString(), declarations: config, compilerArtifacts: artifacts ?? null, compilerProvenance: receiptPaths, durableLedger, retainedInvocations:observations.map(observation=>({manifest:observation.manifest,cwd:observation.cwd,args:observation.args,builtAtMs:observation.builtAtMs,observedAtMs:observation.observedAtMs,buildDirectory:observation.buildDirectory,invocationInputs:observation.invocationInputs})), graphs, sourceIdentities: Object.fromEntries(identities), findings }, null, 2) + "\n");
  }
  console.log(`[runtime graph] graphs=${graphs.length} sources/resources=${identities.size} findings=${findings.length} report=${report}`);
  if (findings.length) { for (const finding of findings) console.error(JSON.stringify(finding)); throw Error(`Runtime source/resource graph refused ${findings.length} findings`); }
}
