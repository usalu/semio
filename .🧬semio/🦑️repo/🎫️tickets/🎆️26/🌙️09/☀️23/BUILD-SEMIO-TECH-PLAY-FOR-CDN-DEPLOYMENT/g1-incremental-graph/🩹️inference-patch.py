"""🩹️ Applies the G1 incremental-graph changes to the build inference owner with exact, asserted anchors (argv: <inference .mjs path>)."""
import re
import sys

path = sys.argv[1]
source = open(path, encoding="utf-8").read()


def replace(old, new, count=1):
    global source
    found = source.count(old)
    if found != count:
        raise SystemExit(f"anchor found {found}x (expected {count}): {old[:90]!r}")
    source = source.replace(old, new)


replace('import { mkdirSync, writeFileSync } from "node:fs";', 'import { mkdirSync, writeFileSync, statSync as rawStatSync } from "node:fs";')

replace(
    "observeInference, observeProjectInferenceV1 } from \"../../../🏗️build/👁️observation/🟨️.mjs\";",
    "observeInference, observeProjectInferenceV1, settleObservedComputationV1 } from \"../../../🏗️build/👁️observation/🟨️.mjs\";\nimport { availableParallelism } from \"node:os\";",
)

replace(
    """/** 📄️ Reads policy and implementation authority through current real ancestors and leaf. */
function readPhysicalSource(path) {
  const ancestors = [];
  for (let directory = dirname(path); ; directory = dirname(directory)) { ancestors.push(directory); if (directory === dirname(directory)) break; }
  for (const directory of ancestors.reverse()) { const value = lstatSync(directory); if (!value.isDirectory() || value.isSymbolicLink()) throw Error(`Build inference requires real authority ancestry: ${directory}`); }
  const value = lstatSync(path);
  if (!value.isFile() || value.isSymbolicLink()) throw Error(`Build inference requires real authority file: ${path}`);
  return readFileSync(path);
}""",
    """/** 🏛️ Requires current real ancestors and a real leaf before an authority source is consumed. */
function requirePhysicalSourceV1(path) {
  const ancestors = [];
  for (let directory = dirname(path); ; directory = dirname(directory)) { ancestors.push(directory); if (directory === dirname(directory)) break; }
  for (const directory of ancestors.reverse()) { const value = lstatSync(directory); if (!value.isDirectory() || value.isSymbolicLink()) throw Error(`Build inference requires real authority ancestry: ${directory}`); }
  const value = lstatSync(path);
  if (!value.isFile() || value.isSymbolicLink()) throw Error(`Build inference requires real authority file: ${path}`);
}

/** 📄️ Reads policy and implementation authority through current real ancestors and leaf. */
function readPhysicalSource(path) {
  requirePhysicalSourceV1(path);
  return readFileSync(path);
}

/** 🗺️ The only taxonomy facts graph inference consumes, so edits elsewhere in the taxonomy keep every inference record and parser revision valid. */
function taxonomyProjectionV1(taxonomy) {
  return { testDeliveryScopeDirectoryNames: taxonomy.testDeliveryScopeDirectoryNames, generatorContracts: taxonomy.generatorContracts ?? {} };
}

const TAXONOMY_PROJECTION = registerSourceAnalysisV1({ id: `build:taxonomy-projection:v1:${createHash("sha256").update(import.meta.url).digest("hex")}`, revision: createHash("sha256").update(taxonomyProjectionV1.toString()).digest("hex"), analyze: (_path, bytes) => taxonomyProjectionV1(JSON.parse(bytes.toString("utf8"))) });

/** 🧭️ Reads the consumed taxonomy projection through physical authority ancestry as a semantic fact rather than whole-file bytes. */
function readTaxonomyProjectionV1(path) {
  requirePhysicalSourceV1(path);
  return observedSourceAnalysisV1(TAXONOMY_PROJECTION, path);
}""",
)

replace(
    'const TAXONOMY = JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, "🔣️taxonomy.json")).toString("utf8"));',
    'const TAXONOMY = readTaxonomyProjectionV1(join(LIBRARY_ROOT, "🔣️taxonomy.json"));',
)

replace(
    "]) hash.update(readPhysicalSource(join(LIBRARY_ROOT, path)));",
    ']) hash.update(path === "🔣️taxonomy.json" ? JSON.stringify(readTaxonomyProjectionV1(join(LIBRARY_ROOT, path))) : readPhysicalSource(join(LIBRARY_ROOT, path)));',
)

contracts = re.compile(r'JSON\.parse\(readFileSync\((\w+), *"utf8"\)\)\.generatorContracts *\?\? *\{\}')
if len(contracts.findall(source)) != 5:
    raise SystemExit("expected five generator contract reads")
source = contracts.sub(lambda match: f"readTaxonomyProjectionV1({match.group(1)}).generatorContracts", source)

replace(
    "const error=new Error(\"Observed TOML parse refused: \"+JSON.stringify(tomlObservation),{cause});error.tomlObservation=tomlObservation;",
    "const error=Object.assign(new Error(\"Observed TOML parse refused: \"+JSON.stringify(tomlObservation),{cause}),{code:\"BUILD_SOURCE_UNPARSEABLE\",source:tomlObservation.path,sha256:tomlObservation.sha256});error.tomlObservation=tomlObservation;",
)

replace(
    """async function emojiProjectJsonNodes(configFiles, options, context) {
  const { workspaceRoot } = context;""",
    """/** ⏱️ A source modified this recently may be half-written, so its parse failure is retried by settlement instead of withholding its project (the observation racy rule). */
const UNSETTLED_SOURCE_MS = 2000;

/** 🚧️ Withholds one project whose consumed source does not parse, naming the exact bytes the snapshot observed; a freshly modified source rethrows so settlement waits for it. */
function unavailableProjectV1(root, project, workspaceRoot, error) {
  let modified = Infinity;
  try { modified = rawStatSync(error.source).mtimeMs; } catch {}
  if (modified >= Date.now() - UNSETTLED_SOURCE_MS) throw error;
  return { root, ...(project ? { project } : {}), source: buildPath(relative(workspaceRoot, error.source)), sha256: error.sha256, reason: String(error.message).slice(0, 2048) };
}

/** 🧵️ Bounds concurrently inferred projects; one worker is the serial reference order. */
function projectConcurrencyV1(options) {
  const value = options?.projectConcurrency ?? Math.max(1, Math.min(8, availableParallelism()));
  if (!Number.isSafeInteger(value) || value < 1 || value > 64) throw Error("Build project inference concurrency must be an integer from 1 to 64");
  return value;
}

/** 🪨️ Reports and cancels inference settlement rounds through the original invocation. */
function settlementOptionsV1(context) {
  return { signal: context.invocation?.control.signal, onRound: async ({ round, conflict, failure }) => { if (process.env.SEMIO_BUILD_TRACE_AUTHORITY === "true") console.error("[DEBUG] build inference settlement round " + JSON.stringify({ round, conflict, failure: failure === undefined ? undefined : String(failure) })); if (context.invocation) await advanceScriptInvocation(context.invocation, "build:settle", "running"); } };
}

async function emojiProjectJsonNodes(configFiles, options, context) {
  const { workspaceRoot } = context, unavailable = new Map();""",
)

replace(
    """      let json;
      try {
        json = JSON.parse(readFileSync(abs, "utf8"));
      } catch (error) {
        throw new Error(`Invalid Build project metadata ${configFile}: ${error.message}`);
      }""",
    """      const bytes = readFileSync(abs);
      let json;
      try {
        json = JSON.parse(bytes.toString("utf8"));
      } catch (error) {
        const root = buildPath(relative(workspaceRoot, dirname(abs))) || ".";
        unavailable.set(root, unavailableProjectV1(root, undefined, workspaceRoot, Object.assign(Error(`Invalid Build project metadata ${configFile}: ${error.message}`), { code: "BUILD_SOURCE_UNPARSEABLE", source: abs, sha256: createHash("sha256").update(bytes).digest("hex") })));
        return null;
      }""",
)

replace(
    """    const manifest = readToml(join(projectDir, "Cargo.toml"));
    if (!manifest.package?.name) continue;""",
    """    let manifest;
    try { manifest = readToml(join(projectDir, "Cargo.toml")); }
    catch (error) { if (error?.code !== "BUILD_SOURCE_UNPARSEABLE") throw error; unavailable.set(root, unavailableProjectV1(root, undefined, workspaceRoot, error)); continue; }
    if (!manifest.package?.name) continue;""",
)

replace(
    "  const commandRevision=createHash(\"sha256\").update(JSON.stringify(commandInputs??null)).digest(\"hex\");",
    "  const commandRevision=createHash(\"sha256\").update(JSON.stringify(commandInputs??null)).digest(\"hex\"),registryDigest=createHash(\"sha256\").update(JSON.stringify(context.registry??null)).digest(\"hex\");",
)

old_loop_head = """  let projectIndex=0;
  for(const[,result]of results)for(let[name,json]of Object.entries(result.projects)){
    const started=performance.now();"""
new_loop_head = """  const jobs=results.flatMap(([,result])=>Object.entries(result.projects).map(([name,json])=>({result,name,json})));let projectIndex=0,cursor=0,stopped=false;
  const inferProject=async({result,name,json})=>{
    const started=performance.now();"""
replace(old_loop_head, new_loop_head)

replace(
    """    result.projects[name]=await observeProjectInferenceV1({cachePath,key:[projectAuthority,native?commandRevision:null,native||pipeline?assemblyAuthority:null,root,json],owner:root},async()=>{
      const sourceGroups=await declaredSourceInputs(activeJson,workspaceRoot,context);""",
    """    result.projects[name]=await observeProjectInferenceV1({cachePath,key:[projectAuthority,registryDigest,native?commandRevision:null,native||pipeline?assemblyAuthority:null,root,json,selectedTargets??null],owner:root},async()=>{
      try{
      const sourceGroups=await declaredSourceInputs(activeJson,workspaceRoot,context);""",
)

replace(
    """      for(const[name,target]of Object.entries(project.targets))project.targets[name]=targetBuildModes(name,target);
      return project;
    });
    result.projects[name].targets={...inactive,...result.projects[name].targets};""",
    """      for(const[name,target]of Object.entries(project.targets))project.targets[name]=targetBuildModes(name,target);
      return project;
      }catch(error){if(error?.code!=="BUILD_SOURCE_UNPARSEABLE")throw error;return {unavailable:unavailableProjectV1(root,name,workspaceRoot,error)};}
    });
    if(result.projects[name].unavailable){unavailable.set(root,result.projects[name].unavailable);delete result.projects[name];return;}
    result.projects[name].targets={...inactive,...result.projects[name].targets};""",
)

replace(
    """    if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true")console.error("[DEBUG] build project complete "+JSON.stringify({index:projectIndex,total:projects.length,project:name,authorityMilliseconds:Math.round(authorityMilliseconds),elapsedMilliseconds:Math.round(performance.now()-started)}));
  }
  if (options?.analyzeLockfile""",
    """    if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true")console.error("[DEBUG] build project complete "+JSON.stringify({index:projectIndex,total:projects.length,project:name,authorityMilliseconds:Math.round(authorityMilliseconds),elapsedMilliseconds:Math.round(performance.now()-started)}));
  };
  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length)await inferProject(jobs[cursor++]);});
  await Promise.all(workers).catch(async error=>{stopped=true;await Promise.allSettled(workers);throw error;});
  for(const row of [...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0))results.push([row.source,{projects:{},unavailable:[row]}]);
  if (options?.analyzeLockfile""",
)

replace("() => emojiProjectJsonNodes(...args))", "() => settleObservedComputationV1(() => emojiProjectJsonNodes(...args), settlementOptionsV1(args[2])))")
replace("() => createDependenciesImplementation(...args))", "() => settleObservedComputationV1(() => createDependenciesImplementation(...args), settlementOptionsV1(args[1])))")

open(path, "w", encoding="utf-8").write(source)
print("patched", path)
