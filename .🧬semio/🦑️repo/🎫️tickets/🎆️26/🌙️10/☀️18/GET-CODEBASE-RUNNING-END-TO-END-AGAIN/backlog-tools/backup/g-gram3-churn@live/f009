import{physicalSourceOriginV1}from"./🔏️revision/🟨️.mjs";
const javascriptManagerV1 = createRequire(import.meta.url)("../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").selectedManagerV1("javascript");
import{captureSourceInputInventoryV1,captureCargoLockInventoryV1}from"./📋️data/📥️load/🟨️.mjs";
import {repositoryBuildGraphWriterDirectoryV1, withBuildGraphOptionalPublicationV1, withBuildGraphOptionalPublicationsV1, noteBuildGraphStagedLeafV1} from "./💾️storage/🟨️.mjs";
import { brotliDecompressSync } from "node:zlib";
import { createRequire } from "node:module";
import { createCargoResolutionContextV1, cargoResolutionPartsV1, flushCargoReceiptsV1, readCargoResolutionPolicyV1 } from "./🦀️resolution/🟦️.ts";
import { createCargoReceiptWriterV1 } from "./🦀️resolution/💾️receipt/🟨️.mjs";
import resolutionDocument from "./🦀️resolution/🔣️.json" with { type: "json" };
import { createHash, randomBytes, randomUUID } from "node:crypto";
import { mkdirSync, writeFileSync, readFileSync, lstatSync, rmSync, existsSync as persistedExists } from "node:fs";
import { renameOverBusyFileSyncV1 } from "../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { fileURLToPath } from "node:url";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { adoptVerifiedBlockV1, observedFs } from "../👁️observation/🟨️.mjs";
import { ownerOfProjectV1, ownerTreeGlobsV1, ownerViewV1 } from "./🌳️owner-view/🟨️.mjs";
import { toolchainsActivatedByV1 } from "./🪛️toolchain-activation/🟨️.mjs";
import { GRAPH_MEMORY_POLICY_V1 } from "../🧠️memory/🟨️.mjs";

const { existsSync, readdirSync } = observedFs;

function compileBuildInputPatternV1(pattern) {
  const compile=(source)=>{
    let result="";
    for(let index=0;index<source.length;index++){
      const character=source[index];
      if(["!","?","+","*","@"].includes(character)&&source[index+1]==="("){
        let depth=1,end=index+2;for(;end<source.length&&depth;end++){if(source[end]==="(")depth++;else if(source[end]===")")depth--;}
        if(depth)throw Error("Unclosed source input glob");
        const contents=source.slice(index+2,end-1),alternatives=[];let nested=0,start=0;
        for(let cursor=0;cursor<=contents.length;cursor++){if(contents[cursor]==="(")nested++;else if(contents[cursor]===")")nested--;if(cursor===contents.length||contents[cursor]==="|"&&!nested){alternatives.push(compile(contents.slice(start,cursor)));start=cursor+1;}}
        const group="(?:"+alternatives.join("|")+")";
        if(character==="!"){const suffix=source.slice(end).split("/")[0];result+="(?!"+group+compile(suffix)+"(?:/|$))[^/]*";}
        else result+=group+(character==="@"?"":character);
        index=end-1;continue;
      }
      if(character==="{"){
        const end=source.indexOf("}",index+1);if(end<0)throw Error("Unclosed source input alternatives");result+="(?:"+source.slice(index+1,end).split(",").map(compile).join("|")+")";index=end;continue;
      }
      if(character==="["){
        const end=source.indexOf("]",index+1);if(end<0)throw Error("Unclosed source input class");result+="["+(source[index+1]==="!"?"^"+source.slice(index+2,end):source.slice(index+1,end))+"]";index=end;continue;
      }
      if(character==="*"&&source[index+1]==="*"){index++;if(source[index+1]==="/"){result+="(?:.*/)?";index++;}else result+=".*";}
      else if(character==="*")result+="[^/]*";
      else if(character==="?")result+="[^/]";
      else result+=character.replace(/[\^$.*+?()[\]{}|]/g,"\\$&");
    }
    return result;
  };
  return new RegExp("^"+compile(pattern)+"$","u");
}

/** 🧵️ Matches portable source globs consistently across Node and Bun without build-tool imports. */
export function buildInputPathMatchesV1(path,pattern) { return compileBuildInputPatternV1(pattern).test(path); }

/** 🎯️ Compiles one portable predicate for an operation-owned input pattern. */
export function createBuildInputPathMatcherV1(pattern){const expression=compileBuildInputPatternV1(pattern);return path=>expression.test(path);}

/** 🧮️ Interns equivalent project/workspace coordinates before backends compile native glob sets. */
export function deduplicateProjectInputs(inputs, root) {
  const keys = new Set(), projectRoot = root === "." ? "{workspaceRoot}" : `{workspaceRoot}/${root}`;
  return inputs.filter(input => {
    const key = typeof input === "string" ? input.replaceAll("{projectRoot}", projectRoot) : JSON.stringify(input);
    if (keys.has(key)) return false;
    keys.add(key);
    return true;
  });
}

/** 🧾️ Publishes identical content-addressed closure bytes atomically across independent graph writers. */
const VERIFIED_CLOSURES_V1=new Map();

/** 🧾️ Publishes identical content-addressed closure bytes atomically across independent graph writers; a closure whose stamp equals the one last verified equal is not read again. */
function publishInputClosureV1(path,contract){
 const verify=()=>{const row=lstatSync(path);if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1||row.size!==Buffer.byteLength(contract))throw Error("Original immutable input closure bytes required");const known=VERIFIED_CLOSURES_V1.get(path);if(known&&known.ino===row.ino&&known.size===row.size&&known.mtimeMs===row.mtimeMs&&known.ctimeMs===row.ctimeMs)return;if(!readFileSync(path).equals(Buffer.from(contract)))throw Error("Original immutable input closure bytes required");VERIFIED_CLOSURES_V1.set(path,{ino:row.ino,size:row.size,mtimeMs:row.mtimeMs,ctimeMs:row.ctimeMs});};
 if(persistedExists(path)){verify();return;}
 const temporary=path+"~"+randomUUID();if(temporary.length>256||[...temporary].length>256)throw Error("Input closure staging path exceeds256 characters");try{writeFileSync(temporary,contract,{flag:"wx"});renameOverBusyFileSyncV1(temporary,path);verify();noteBuildGraphStagedLeafV1(path);}finally{rmSync(temporary,{force:true});}
}

/** 🛰️ Adopts the leaves a farm worker staged into the graph under this process's writer custody, in the order that keeps every reference satisfied: content-addressed closures and blocks first (the first writer wins and every later copy is verified against it), the record that names them last. A receipt or pack that stands is the first writer's and keeps standing (receipts are sealed and checked when they are used); the optional leaves of one call are one batched publication. A block the adoption verified is registered as verified, so its first reader does not validate it again. Answers how many leaves were published, how many already stood and whether the quota declined them. */
export function adoptStagedGraphLeavesV1(workspaceRoot, stagingRoot, names) {
  const root = resolve(stagingRoot), closures = [], optional = [], plan = { published: 0, present: 0, declined: false };
  for (const name of names) {
    const [namespace, ...rest] = name.split("/"), leaf = rest.join("/"), source = join(root, namespace, ...rest);
    if (!leaf || rest.some(part => part === "" || part === ".." || part === ".") || !["inference", "build-input-closures", "build-lock-closures"].includes(namespace)) throw Error("Relative staged graph leaf of a declared namespace required: " + name);
    if (namespace !== "inference") {
      const bytes = readFileSync(source);
      if (!/^[a-f0-9]{64}\.json$/u.test(leaf) || createHash("sha256").update(bytes).digest("hex") !== leaf.slice(0, -5)) throw Error("Closure content differs from its name: " + name);
      closures.push([namespace, leaf, bytes]);
      continue;
    }
    const target = join(repositoryBuildGraphWriterDirectoryV1(workspaceRoot, "inference"), ...rest), block = /(?:^|\/)ob\/([a-f0-9]{64})\.br$/u.exec(leaf), record = /(?:^|\/)np\/[a-f0-9]{24}\.json$/u.test(leaf);
    let standing;
    try { standing = lstatSync(target); } catch (error) { if (error?.code !== "ENOENT") throw error; }
    if (standing !== undefined && !block && !record) { plan.present++; continue; }
    const bytes = readFileSync(source);
    let text;
    if (block) {
      const inflated = brotliDecompressSync(bytes);
      if (createHash("sha256").update(inflated).digest("hex") !== block[1]) throw Error("Block content differs from its name: " + name);
      text = inflated.toString("utf8");
    }
    if (standing !== undefined && standing.size === bytes.byteLength && readFileSync(target).equals(bytes)) { plan.present++; if (text !== undefined) adoptVerifiedBlockV1(target, text); continue; }
    optional.push({ target, bytes, text, previous: standing?.size ?? 0, last: record });
  }
  for (const [namespace, leaf, bytes] of closures) publishInputClosureV1(join(repositoryBuildGraphWriterDirectoryV1(workspaceRoot, namespace), leaf), bytes);
  if (!optional.length) return plan;
  optional.sort((a, b) => Number(a.last) - Number(b.last));
  const peakBytes = optional.reduce((sum, row) => sum + row.bytes.byteLength, 0), retainedDeltaBytes = optional.reduce((sum, row) => sum + Math.min(row.bytes.byteLength, row.bytes.byteLength - row.previous), 0);
  plan.declined = !withBuildGraphOptionalPublicationsV1(optional.map(row => row.target), { peakBytes, retainedDeltaBytes }, () => {
    for (const { target, bytes } of optional) {
      const temporary = /\.(?:pack|idx)$/u.test(target) ? `${target}.${randomBytes(4).toString("hex")}.tmp` : `${target}.${process.pid}.${Date.now()}.tmp`;
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(temporary, bytes, { flag: "wx" });
      renameOverBusyFileSyncV1(temporary, target);
    }
  });
  if (!plan.declined) { plan.published = optional.length; for (const row of optional) if (row.text !== undefined) adoptVerifiedBlockV1(row.target, row.text); }
  return plan;
}

/** 📍️ Projects the project-root sentinel into a repository-relative Cargo manifest. */
export function cargoManifestInputV1(root){return root==="."?"Cargo.toml":`${root}/Cargo.toml`;}

/** 🔒️ A lock projection is batched by the same native hasher without compiler subprocesses. */
export function cargoLockInput(manifest, workspaceRoot) {
  const captured = captureCargoLockInventoryV1({ version: 1, manifest },workspaceRoot), contract = captured.text, key = createHash("sha256").update(contract).digest("hex"), directory = repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-lock-closures"), path = join(directory, `${key}.json`);
  publishInputClosureV1(path,contract);
  return { env: `SEMIO_CARGO_LOCK_${key}` };
}

const INVENTORY_MEMO_V1 = new Map(), INVENTORY_MEMO_LIMIT_V1 = 4096;
/** 🕊️ Forgets the in-process memo of captured inventories and verified closure stamps (they only save recomputation). */
export function releaseInputMemoryV1() { INVENTORY_MEMO_V1.clear(); VERIFIED_CLOSURES_V1.clear(); }

/** 🗃️ Reuses the immutable captured inventory of an identical file roster: it is a pure function of the roster and the workspace root, and thousands of projects share the same command closure. */
function capturedInventoryV1(files, workspaceRoot) {
  const hash = createHash("sha256").update(workspaceRoot).update("\0");
  for (const file of files) hash.update(file).update("\0");
  const key = hash.digest("hex"), known = INVENTORY_MEMO_V1.get(key);
  if (known !== undefined) return known;
  const captured = captureSourceInputInventoryV1({ version: 1, files }, workspaceRoot);
  if (INVENTORY_MEMO_V1.size >= INVENTORY_MEMO_LIMIT_V1) INVENTORY_MEMO_V1.delete(INVENTORY_MEMO_V1.keys().next().value);
  INVENTORY_MEMO_V1.set(key, captured);
  return captured;
}

const resolutionPolicy = readCargoResolutionPolicyV1(resolutionDocument), resolutionRuns = new WeakMap();
let tomlParser;

/** 🦀️ Refreshes the resolution receipts of one crate while the graph is built, under the graph writer's custody: every manifest of one graph construction is read and digested once through plain reads (the receipts are validated by digest when they are used, so these reads need no observation), a receipt whose digests still hold costs no derivation, and a failing crate simply has no receipt. */
async function warmCargoResolutionV1(workspaceRoot, scripts, manifest, context) {
  try {
    const key = typeof scripts === "object" && scripts !== null ? scripts : context;
    let run = resolutionRuns.get(key);
    if (!run) {
      tomlParser ??= createRequire(import.meta.url)("@iarna/toml").parse;
      const bytes = (path) => { try { return readFileSync(join(workspaceRoot, path)); } catch (error) { if (error?.code === "ENOENT" || error?.code === "ENOTDIR") return null; throw error; } };
      const digest = (buffer) => createHash("sha256").update(buffer).digest("hex");
      const host = { read: async (path) => { const buffer = bytes(path); return buffer === null ? null : { value: tomlParser(buffer.toString("utf8")), sha: digest(buffer) }; }, digest: async (path) => { const buffer = bytes(path); return buffer === null ? null : digest(buffer); } };
      resolutionRuns.set(key, run = createCargoResolutionContextV1(host, resolutionPolicy, createCargoReceiptWriterV1(repositoryBuildGraphWriterDirectoryV1(workspaceRoot, "inference"), withBuildGraphOptionalPublicationV1), GRAPH_MEMORY_POLICY_V1.maximumParsedManifests));
    }
    await cargoResolutionPartsV1(run, manifest, { tolerant: true });
    await flushCargoReceiptsV1(run);
  } catch {}
}

/** ✂️ Cuts a sorted roster into content-defined chunks: a chunk ends after the first path whose digest byte is a multiple of 128 once it holds 16 paths, so one added or removed path changes one or two chunks and every other chunk stays the identical content-addressed file. Rosters of thousands of targets differ in a few paths of a shared base; whole-roster files made every such difference a new 180 KB file (4 GB, +16 MiB per minute of graph building), chunks make it one new 12 KB file. */
export function splitInputClosureV1(files) {
  const chunks = []; let current = [];
  for (const file of files) {
    current.push(file);
    if (current.length >= INPUT_CLOSURE_CHUNK_MINIMUM_V1 && createHash("sha256").update(file).digest()[0] % INPUT_CLOSURE_CHUNK_MODULUS_V1 === 0) { chunks.push(current); current = []; }
  }
  if (current.length) chunks.push(current);
  return chunks;
}
const INPUT_CLOSURE_CHUNK_MINIMUM_V1 = 16, INPUT_CLOSURE_CHUNK_MODULUS_V1 = 128;

/** 🧮️ Shares exact inventories while backends hash their current byte digest once per unique closure. */
export function compactSourceInputs(inputs, workspaceRoot, root = ".") {
  const candidates = [...new Set(inputs.filter(input => typeof input === "string" && input.startsWith("{workspaceRoot}/") && !/[!*?{}[\]]/u.test(input.slice("{workspaceRoot}/".length))))].sort();
  const selected = new Set(candidates), remaining = inputs.filter(input => !selected.has(input));
  const residual = remaining.some(input => typeof input === "string" && !input.startsWith("!")) ? remaining : remaining.filter(input => typeof input !== "string" || !input.startsWith("!"));
  let exact = candidates;
  if (exact.length < 1) return inputs;
  const projectRoot = root === "." ? "." : root, normalize = input => input.replaceAll("{projectRoot}", projectRoot).replaceAll("{workspaceRoot}/", ""), exclusions = inputs.filter(input => typeof input === "string" && input.startsWith("!") && !input.startsWith("!^"));
  if (exclusions.length) {
    const paths = exact.map(normalize), excluded = paths.map(() => false), patterns = new Map();
    for (const input of exclusions) {
      const pattern=normalize(input.slice(1));
      for (const [index,path] of paths.entries()) {
        if(excluded[index])continue;
        let matcher=patterns.get(pattern);
        if(!matcher){matcher=compileBuildInputPatternV1(pattern);patterns.set(pattern,matcher);}
        excluded[index]=matcher.test(path);
      }
    }
    exact = exact.filter((input, index) => !excluded[index]);
  }
  if (exact.length < 1) return residual;
  const helpers = ["./🟨️.mjs", "./🔏️revision/🟨️.mjs", "./🔏️revision/🧬️schema/🔣️.json", "./📋️data/🟨️.mjs", "./📋️data/📥️load/🟨️.mjs", "./📋️data/🧬️schema/🔣️.json", "./🧮️digest/📥️read/🟨️.mjs", "./🧮️digest/📥️read/🧬️schema/🔣️.json", "./🔒️lock/🔣️schema.json", "./💾️storage/🟨️.mjs", "./💾️storage/📒️ledger/🟨️.mjs", "./💾️storage/🧬️schema/🔣️.json", "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json", "./🧮️digest/🟨️.mjs", "./🧮️digest/🔣️schema.json", "./🔒️lock/📥️read/🟨️.mjs", "./🔒️lock/🟨️.mjs"].map(path => relative(workspaceRoot, fileURLToPath(new URL(path, physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)))).split("\\").join("/"));
  const files = [...new Set([...exact.map(input => input.slice("{workspaceRoot}/".length)), ...helpers])].sort(), directory = repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-input-closures");
  const tokens = splitInputClosureV1(files).map(chunk => { const contract = capturedInventoryV1(chunk, workspaceRoot).text, key = createHash("sha256").update(contract).digest("hex"); publishInputClosureV1(join(directory, `${key}.json`), contract); return { env: `SEMIO_SOURCE_CLOSURE_${key}` }; });
  const script = fileURLToPath(new URL("./🧮️digest/🟨️.mjs", physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL))), schema = fileURLToPath(new URL("./🧮️digest/🔣️schema.json", physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)));
  return [...residual, `{workspaceRoot}/${relative(workspaceRoot, script).split("\\").join("/")}`, `{workspaceRoot}/${relative(workspaceRoot, schema).split("\\").join("/")}`, ...tokens];
}

/** 🧬️ Keeps output negations only where a positive source scope can observe those outputs. */
export function sourceOutputOverlaps(output, inputs, root) {
  return inputs.some(input => {
    if (typeof input !== "string" || input.startsWith("!") || !/^\{(?:projectRoot|workspaceRoot)\}\//u.test(input)) return false;
    const source = input.replaceAll("{projectRoot}/", root === "." ? "" : `${root}/`).replaceAll("{workspaceRoot}/", ""), wildcard = source.search(/[!*?{}[\]]/u), prefix = wildcard < 0 ? source : source.slice(0, wildcard).replace(/[^/]*$/u, "").replace(/\/$/u, "");
    return !prefix || output === prefix || output.startsWith(prefix + "/") || prefix.startsWith(output + "/");
  });
}

/** 🗂️ Shares repeated arrays inside one project while retaining native batching and exact membership. */
export function internProjectInputGroups(groups) {
  const names = Object.keys(groups), entries = new Map();
  for (const name of names) for (const input of groups[name]) {
    const key = JSON.stringify(input), entry = entries.get(key) ?? { input, names: [] };
    if (!entry.names.includes(name)) entry.names.push(name);
    entries.set(key, entry);
  }
  const shared = new Map();
  for (const [key, entry] of entries) if (entry.names.length > 1) {
    const membership = JSON.stringify(entry.names), group = shared.get(membership) ?? { names: entry.names, keys: new Set(), inputs: [] };
    group.keys.add(key); group.inputs.push(entry.input); shared.set(membership, group);
  }
  const result = { ...groups };
  for (const group of shared.values()) {
    if (group.inputs.length < 4) continue;
    const name = `sharedSources${createHash("sha256").update(JSON.stringify(group.inputs)).digest("hex").slice(0, 16)}`;
    if (Object.hasOwn(result, name)) throw Error("Shared source group identity collision");
    result[name] = group.inputs;
    for (const owner of group.names) result[owner] = [...result[owner].filter(input => !group.keys.has(JSON.stringify(input))), name];
  }
  return result;
}

/** 📥️ Projects source and compiler input ownership independently of target orchestration. The named input `cargoToolchain` of a Rust project is generated here (raw tool files plus the crate's resolution marker, which only a persisted graph carries), never authored: an authored definition of it is replaced; a persisted graph also refreshes the crate's resolution receipts. */
export async function inferProjectInputs(json, root, workspaceRoot, facts, scripts, context) {
  const { POLICY, TAXONOMY, LIBRARY_ROOT, SCRIPT_BASENAME, cargoSourceInputs, goManifest, buildPath, relativeScriptInputs, closureOf, resolveOutputPath, exampleCollectionInputs, workspaceNegationClosed, declaredSourceInputs, owned } = context;
  const nativeRoot = json.metadata?.nativeRoot ?? root, nativeManifest=cargoManifestInputV1(nativeRoot);
  const tools = ["javascript"];
  if (existsSync(join(workspaceRoot, root, "Cargo.toml")) || json.targets?.wasm) tools.push("cargo");
  if (json.targets?.wasm || json.targets?.["extension-package"] || (tools.includes("cargo") && json.targets?.package)) tools.push("wasm");
  if (goManifest(root, workspaceRoot)) tools.push("go");
  const workspaceProject = root === "." || root === "";
  if (!workspaceProject && existsSync(join(workspaceRoot, root, "pyproject.toml"))) tools.push("python");
  if (readdirSync(join(workspaceRoot, root)).some((file) => /\.[cfv]sproj$/.test(file))) tools.push("dotnet");
  if (!workspaceProject && existsSync(join(workspaceRoot, root, "CMakeLists.txt"))) tools.push("cmake");
  for (const tool of toolchainsActivatedByV1(json.targets, POLICY.toolchains)) if (!tools.includes(tool)) tools.push(tool);
  const nativeSources = tools.includes("cargo") ? cargoSourceInputs(nativeRoot, workspaceRoot, facts, false) : undefined;
  const nativeTests = tools.includes("cargo") ? cargoSourceInputs(nativeRoot, workspaceRoot, facts, true) : undefined;
  const runner = buildPath(relative(workspaceRoot, LIBRARY_ROOT));
  const inputs = [
    "{projectRoot}/**/*",
    { runtime: 'node -p "process.platform.concat(process.arch)"' },
  ];
  const owner = ownerOfProjectV1(root, TAXONOMY.testDeliveryScopeDirectoryNames), ownerView = owner === root ? undefined : ownerViewV1(owner, POLICY.ownerViews);
  let ownerPrivate = [];
  if (owner !== root) {
    const extensions = tools.includes("cargo") ? "{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,jsx,mjs,cjs,css,scss,html,svg,sql}" : tools.includes("go") ? "{go,mod,sum,json,ts}" : tools.includes("dotnet") ? "{cs,fs,vb,csproj,fsproj,vbproj,props,targets,resx,json}" : tools.includes("python") ? "{py,pyi,toml,json}" : "{ts,tsx,js,jsx,mjs,cjs,json,css,scss,html,svg,wit,sql}";
    const native = nativeTests;
    const ownerExtensions = native ? "{json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,jsx,mjs,cjs,css,scss,html,svg,sql}" : extensions;
    let ownerTree = [`{workspaceRoot}/${owner}/**/*.${ownerExtensions}`];
    if (ownerView) {
      const split = ownerTreeGlobsV1(owner, ownerExtensions, ownerView.private, readdirSync(join(workspaceRoot, owner), { withFileTypes: true }).filter((entry) => !POLICY.generatedDirectories.includes(entry.name)).map((entry) => ({ name: entry.name, directory: entry.isDirectory() })));
      ownerTree = split.shared;
      ownerPrivate = split.private;
    }
    inputs.push(...(native ? [...native, ...ownerTree] : ownerTree));
  }
  for (const tool of tools) {
    const contract = POLICY.toolchains[tool];
    inputs.push(...contract.files.map((file) => `{workspaceRoot}/${file}`));
    inputs.push(...contract.environment.map((env) => ({ env })));
    inputs.push(...contract.commands.map((runtime) => ({ runtime: tool === "python" ? `${runtime} ${JSON.stringify(root)}` : runtime })));
  }
  for (const path of POLICY.runtimeStateDirectories ?? []) inputs.push(`!{workspaceRoot}/${path}/**/*`);
  for (const directory of POLICY.generatedDirectories) {
    inputs.push(`!{projectRoot}/**/${directory}/**/*`);
    if (owner !== root) inputs.push(`!{workspaceRoot}/${owner}/**/${directory}/**/*`);
  }
  if (owner !== runner) inputs.push(`!{workspaceRoot}/${runner}/**/🧪️tests/**/*`);
  const examples = [...exampleCollectionInputs("{projectRoot}"), ...exampleCollectionInputs("{workspaceRoot}")];
  const production = ["!{projectRoot}/**/🧪️tests/**/*", ...examples, "!{projectRoot}/**/*.feature", "!{projectRoot}/**/*.stories.{ts,tsx}"];
  if (owner !== root) production.push(`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`);
  const declarations = json.namedInputs ?? {}, exclusions = [];
  const native = (sources, productionOnly = false) => !tools.includes("cargo") ? ["production"] : [...new Set(sources ? [`{workspaceRoot}/${nativeManifest}`, ...sources] : ["{projectRoot}/**/*", ...(owner !== root ? [`{workspaceRoot}/${owner}/**/*`] : [])]), ...(productionOnly ? ["!{projectRoot}/**/🧪️tests/**/*", ...examples, ...(owner !== root ? [`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`] : [])] : []), ...POLICY.generatedDirectories.flatMap((directory) => [`!{projectRoot}/**/${directory}/**/*`, ...(owner !== root ? [`!{workspaceRoot}/${owner}/**/${directory}/**/*`] : [])]), ...exclusions];
  const artifactTypeScript = json.tags?.includes("role:artifact") && json.tags.includes("language:typescript") && owner !== root && existsSync(join(workspaceRoot, root, SCRIPT_BASENAME));
  const artifactSource = join(workspaceRoot, owner, "🟦️.ts");
  const artifactSources = artifactTypeScript ? await closureOf([artifactSource], ["{projectRoot}/package.json"]) : [];
  const javascript = POLICY.toolchains.javascript;
  const artifactCommandSources = artifactTypeScript ? await closureOf([join(workspaceRoot, root, SCRIPT_BASENAME)], [`{workspaceRoot}/${javascriptManagerV1.scopeConfigName}`, { externalDependencies: ["typescript"] }, ...javascript.environment.map((env) => ({ env })), ...javascript.commands.map((runtime) => ({ runtime })), { runtime: 'node -p "process.platform.concat(process.arch)"' }]) : [];
  const cargoManifest = existsSync(join(workspaceRoot, nativeManifest)) ? nativeManifest : undefined, persisted = context.sourceRepresentation !== "source";
  if (persisted && cargoManifest && tools.includes("cargo")) await warmCargoResolutionV1(workspaceRoot, scripts, cargoManifest, context);
  const cargoToolchain = tools.includes("cargo") ? [...POLICY.toolchains.cargo.files.map((file) => `{workspaceRoot}/${file}`), ...(persisted && cargoManifest ? [cargoLockInput(cargoManifest, workspaceRoot)] : [])] : undefined;
  const groups = (sourceContracts) => ({ ...declarations, ...sourceContracts, ...(cargoToolchain ? { cargoToolchain } : {}), ...(ownerPrivate.length ? { ownerPrivate } : {}), default: [...inputs, ...(declarations.default ?? []), ...exclusions], production: [...inputs, ...(declarations.default ?? []), ...exclusions, ...production, ...(declarations.production ?? [])], nativeSources: workspaceNegationClosed([...native(nativeSources, true), ...(declarations.nativeSources ?? [])]), nativeTestSources: workspaceNegationClosed([...native(nativeTests), ...(declarations.nativeSources ?? []), ...(declarations.nativeTestSources ?? [])]), ...(artifactTypeScript ? { artifactSources: [...artifactSources, ...exclusions, ...(declarations.artifactSources ?? [])], artifactCommandSources: [...artifactCommandSources, ...(declarations.artifactCommandSources ?? [])] } : {}) });
  const sourceContracts = await declaredSourceInputs(json, workspaceRoot, Object.fromEntries(Object.entries(groups({})).map(([name, inputs]) => [name, inputs.filter(item => !(item !== null && typeof item === "object" && typeof item.closureRef === "string"))])));
  const sourceScopes = [...inputs, ...Object.values(declarations).flat(), ...Object.values(sourceContracts).flat(), ...(nativeSources ?? []), ...(nativeTests ?? [])];
  for (const target of Object.values(json.targets ?? {})) for (const output of target.outputs ?? []) {
    const absolute = resolveOutputPath(output, root, workspaceRoot);
    if (!absolute) continue;
    const path = buildPath(relative(workspaceRoot, absolute));
    if (path === "" || path.startsWith("../")) throw new Error(`Invalid output ownership for ${json.name}: ${output}`);
    if (!sourceOutputOverlaps(path, sourceScopes, root)) continue;
    if ((owned(path, root) || owned(path, owner)) && path.split("/").some(part => POLICY.generatedDirectories.includes(part))) continue;
    exclusions.push(`!{workspaceRoot}/${path}`, `!{workspaceRoot}/${path}/**/*`);
    if (owned(path, root)) {
      const local = buildPath(relative(join(workspaceRoot, root), absolute));
      exclusions.push(`!{projectRoot}/${local}`, `!{projectRoot}/${local}/**/*`);
    }
  }
  if (owner !== root && tools.includes("cargo")) {
    for (const glob of POLICY.ephemeralOwnerGlobs ?? []) {
      exclusions.push(`!{workspaceRoot}/${owner}/${glob}`, `!{workspaceRoot}/${owner}/${glob}/**/*`);
    }
  }
  const named = Object.entries(groups(sourceContracts)), symbolic = named.filter(([, inputs]) => inputs.some(item => item !== null && typeof item === "object" && typeof item.closureRef === "string"));
  const interned = internProjectInputGroups(Object.fromEntries(named.filter(([name]) => !symbolic.some(([other]) => other === name)).map(([name, inputs]) => [name, context.sourceRepresentation === "source" ? deduplicateProjectInputs(inputs, root) : compactSourceInputs(deduplicateProjectInputs(inputs, root), workspaceRoot, root)])));
  return Object.assign(interned, Object.fromEntries(symbolic));
}

