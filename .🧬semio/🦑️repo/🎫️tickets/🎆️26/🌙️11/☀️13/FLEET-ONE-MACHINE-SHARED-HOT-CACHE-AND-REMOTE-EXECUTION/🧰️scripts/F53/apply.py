import os, sys, uuid
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
G = B + "🕸️graph/"
dry = "--dry" in sys.argv
plan = {}
def edit(path, old, new):
    text = plan.get(path)
    if text is None:
        text = open(path, encoding="utf-8").read()
    n = text.count(old)
    assert n == 1, f"{n} anchors in {path}: {old[:70]!r}"
    plan[path] = text.replace(old, new)

CMD = "repo ./📜️script.ts verify build-owners sync"
# 1. catalogue + schema + helper
cat = B + "⚙️operations/🔣️catalog.json"
edit(cat, '\n  "operations": [\n', '''\n  "repairs": [
    {
      "id": "build-owners-sync",
      "command": "%s",
      "description": {
        "en": "Bring the build owner registry in line with the tree: add a row for every crate or project manifest without one, drop and move rows of vanished modules.",
        "de": "Bringt das Eigentümerregister des Builds mit dem Baum in Einklang: trägt für jedes Crate- oder Projekt-Manifest ohne Zeile eine ein, entfernt und verschiebt Zeilen verschwundener Module."
      }
    }
  ],
  "operations": [
''' % CMD)
sch = B + "⚙️operations/🧬️schema/🔣️.json"
edit(sch, '''    "operations": { "type": "array", "minItems": 1, "items": { "$ref": "#/definitions/Operation" } }
  },''', '''    "repairs": { "type": "array", "minItems": 1, "items": { "$ref": "#/definitions/Repair" } },
    "operations": { "type": "array", "minItems": 1, "items": { "$ref": "#/definitions/Operation" } }
  },''')
edit(sch, '    "Example": {', '''    "Repair": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "command", "description"],
      "properties": {
        "id": { "type": "string", "pattern": "^[a-z][a-z0-9-]*$", "maxLength": 48 },
        "command": { "type": "string", "minLength": 6, "pattern": "^repo " },
        "description": { "$ref": "#/definitions/LocalizedText" }
      }
    },
    "Example": {''')
opt = B + "⚙️operations/🔤️options/🟦️.ts"
edit(opt, '/** 🧭️ Projects operation arguments after global selections', '''/** 🩹️ The command line the neutral operations catalogue declares to repair one named drift (`repairs`), the single source every message that advises a repair spells its command from; an undeclared repair is a programming error, never a guessed command. */
export function buildRepairCommandV1(id:string):string{
 const row=(catalogue as unknown as {repairs?:readonly {id:string;command:string}[]}).repairs?.find(candidate=>candidate.id===id);
 if(row===undefined)throw Error("Declared repair required: "+id);
 return row.command;
}

/** 🧭️ Projects operation arguments after global selections''')

# 2. texts
gs = G + "🧬️schema/🔣️.json"
edit(gs, "Run `repo run workspace:verify -- build-owners sync`, then read again. Cause: {cause}", "Run `{command}`, then read again. Cause: {cause}")
edit(gs, "`repo run workspace:verify -- build-owners sync` ausführen und erneut lesen.", "`{command}` ausführen und erneut lesen.")
gen = G + "🕰️generations/"
for p in (gen + "🧬️schema/🔣️.json", gen + "🔌️load/🟨️.mjs"):
    edit(p, "until its owner row is registered (repo verify syncs build-owners)", "until its owner row is registered (run {command})")
    edit(p, "bis seine Eigner-Zeile eingetragen ist (repo verify gleicht build-owners ab)", "bis seine Eigner-Zeile eingetragen ist ({command} ausführen)")
lm = gen + "🔌️load/🟨️.mjs"
edit(lm, 'host.say("pendingCrate",{path});', 'host.say("pendingCrate",{path,command:buildRepairCommandV1("build-owners-sync")});')
edit(lm, 'import{createBoundedCacheV1}from"../../../🧊️bounded/🟨️.mjs";\n', 'import{createBoundedCacheV1}from"../../../🧊️bounded/🟨️.mjs";\nimport{buildRepairCommandV1}from"../../../⚙️operations/🔤️options/🟦️.ts";\n')

# 3. registry consistency
rc = G + "🧾️registry-consistency/🟦️.ts"
edit(rc, 'import { randomUUID } from "node:crypto";\nimport { existsSync, lstatSync, readdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";\n',
 'import { createHash, randomUUID } from "node:crypto";\nimport { existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";\nimport { tmpdir } from "node:os";\n')
edit(rc, "export function syncOwnerRegistry(registry: BuildOwnerRegistryV1, tree: OwnerTree, policy: OwnerTreePolicy, scope?: OwnerScope): OwnerSync {\n  const findings = findOwnerFindings(registry, tree, policy, scope);",
 "export function syncOwnerRegistry(registry: BuildOwnerRegistryV1, tree: OwnerTree, policy: OwnerTreePolicy, scope?: OwnerScope, additionsOnly = false): OwnerSync {\n  const findings = findOwnerFindings(registry, tree, policy, scope).filter((finding) => !additionsOnly || finding.kind === \"project-without-row\" || finding.kind === \"cargo-without-row\");")
edit(rc, "for (const owner of [...owners].filter((row) => within(row.root, scope) && tree.exists(row.manifest))) {", "for (const owner of [...owners].filter((row) => !additionsOnly && within(row.root, scope) && tree.exists(row.manifest))) {")
edit(rc, "/** 📚️ Brings the rows below `scope` in line with the tree:", "/** 📚️ Brings the rows below `scope` in line with the tree (with `additionsOnly` it only adds the rows that are missing and touches no other row):")
src = open(rc, encoding="utf-8").read() if rc not in plan else plan[rc]
i = src.index("/** 🔄️ Re-reads the registry file at call time")
tail_old = src[i:]
assert "return { ...result, changed, findings: findOwnerFindings(result.registry, tree, policy, options.scope) };" in tail_old
tail_new = '''class OwnerRegistryRaceV1 extends Error {}

const ownerRegistryLockPathV1 = (file: string): string => join(tmpdir(), `semio-build-owners-${createHash("sha256").update(file).digest("hex").slice(0, 16)}.lock`);

const ownerRegistryLockHolderAliveV1 = (pid: number): boolean => {
  try { process.kill(pid, 0); return true; } catch (error) { return (error as NodeJS.ErrnoException).code === "EPERM"; }
};

/** 🔒️ Runs `run` as the only writer of the registry file on this machine: an atomic lock directory in the temporary directory (never inside the replicated work tree) names its holder, a lock whose holder ended or that is older than `staleMilliseconds` is taken over by an atomic rename, and a writer that waits longer than `waitMilliseconds` refuses instead of hanging. */
export function withOwnerRegistryLockV1<T>(file: string, run: () => T, options: { staleMilliseconds?: number; waitMilliseconds?: number } = {}): T {
  const lock = ownerRegistryLockPathV1(file), stale = options.staleMilliseconds ?? 60_000, wait = options.waitMilliseconds ?? 30_000, began = Date.now();
  for (;;) {
    try { mkdirSync(lock); writeFileSync(join(lock, "owner"), `${process.pid} ${Date.now()}\\n`); break; } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") { rmSync(lock, { recursive: true, force: true }); throw error; }
    }
    const facts = statSync(lock, { throwIfNoEntry: false });
    if (facts !== undefined) {
      let pid = Number.NaN;
      try { pid = Number(readFileSync(join(lock, "owner"), "utf8").split(" ")[0]); } catch {}
      if (Date.now() - facts.mtimeMs > stale || (Number.isInteger(pid) && !ownerRegistryLockHolderAliveV1(pid))) {
        try { renameSync(lock, `${lock}.${randomUUID()}.stale`); } catch {}
        for (const name of readdirSync(tmpdir()).filter((entry) => entry.startsWith(`${basename(lock)}.`) && entry.endsWith(".stale"))) rmSync(join(tmpdir(), name), { recursive: true, force: true });
        continue;
      }
    }
    if (Date.now() - began > wait) throw new Error(`Build owner registry is locked by another writer: ${lock}`);
    Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 25);
  }
  try { return run(); } finally { rmSync(lock, { recursive: true, force: true }); }
}

/** 🔄️ Re-reads the registry file at call time because peers edit it concurrently, syncs the rows below `scope` (with `additionsOnly` only adds missing rows), replaces the file atomically when anything changed and lists the findings that remain (catalog providers are authored, not synced; native roots follow their row). A write runs under the registry lock of this machine; a hand edit that still lands between its read and its rename is detected and the reconciliation runs again on the new file, so a peer's write is never lost and never fails the caller. */
export function reconcileBuildOwnerRegistry(repoRoot: string, options: { scope?: OwnerScope; write: boolean; additionsOnly?: boolean }, policy: OwnerTreePolicy = OWNER_TREE_POLICY): OwnerSync & { changed: boolean; findings: OwnerFinding[] } {
  const file = join(repoRoot, BUILD_OWNER_REGISTRY_FILE);
  const once = (): OwnerSync & { changed: boolean; findings: OwnerFinding[] } => {
    const source = readFileSync(file, "utf8");
    const registry = readBuildOwnerRegistryV1(JSON.parse(source));
    const tree = readOwnerTree(repoRoot, policy, options.scope);
    const result = syncOwnerRegistry(registry, tree, policy, options.scope, options.additionsOnly === true);
    const text = `${JSON.stringify(result.registry, null, 2)}\\n`;
    const changed = JSON.stringify(result.registry) !== JSON.stringify(registry);
    if (changed && options.write) {
      const pending = `${file}.${randomUUID()}.tmp`;
      try {
        writeFileSync(pending, text, { flag: "wx" });
        if (readFileSync(file, "utf8") !== source) throw new OwnerRegistryRaceV1("Build owner registry changed during reconciliation");
        renameSync(pending, file);
      } finally {
        rmSync(pending, { force: true });
      }
    }
    return { ...result, changed, findings: findOwnerFindings(result.registry, tree, policy, options.scope) };
  };
  if (!options.write) return once();
  return withOwnerRegistryLockV1(file, () => {
    for (let attempt = 0; ; attempt++) {
      try { return once(); } catch (error) { if (!(error instanceof OwnerRegistryRaceV1) || attempt >= 3) throw error; }
    }
  });
}

/** 🔎️ Why the registry cannot give the native root `path` a row of its own, in one line for the refusal of a read: the manifest is absent, virtual, in an excluded tree, its package name belongs to another row, the root is only the native root of another row, or nothing stands in the way. */
export function explainOwnerlessRoot(repoRoot: string, path: string, policy: OwnerTreePolicy = OWNER_TREE_POLICY): string {
  const manifest = at(path, CARGO), tree = readOwnerTree(repoRoot, policy, path);
  if (!tree.exists(manifest)) return `${manifest} is absent`;
  const name = ownerManifestName(tree, manifest);
  if (name === undefined) return `${manifest} declares no [package] name (virtual, unreadable or half written)`;
  if (excludedFromOwners(manifest, policy)) return `${manifest} lies in a tree the owner discovery excludes`;
  const registry = readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(repoRoot, BUILD_OWNER_REGISTRY_FILE), "utf8")));
  const same = registry.owners.find((owner) => owner.name === name && owner.root !== path);
  if (same) return `the package name ${name} already belongs to the row at ${same.root}`;
  const host = registry.owners.find((owner) => owner.nativeRoots?.includes(path));
  if (host) return `${path} is only the native root of ${host.name} and has no row of its own`;
  if (registry.owners.some((owner) => owner.root === path)) return `the row of ${name} exists; the read that failed did not see it`;
  return `nothing stands in the way of a row for ${name}`;
}

/** 🧱️ The crate manifests without any owner row, as the consistency law sees them: the cheap check for every tool that writes crate manifests. */
export function ownerlessCrateManifests(repoRoot: string, scope?: OwnerScope, policy: OwnerTreePolicy = OWNER_TREE_POLICY): string[] {
  return checkBuildOwnerRegistry(repoRoot, scope, policy).findings.filter((finding) => finding.kind === "cargo-without-row").map((finding) => finding.path!);
}
'''
plan[rc] = (plan[rc] if rc in plan else src)[:i] + tail_new

# 4. registry follow
rf = G + "📥️load/🧭️registry-follow/🟦️.ts"
edit(rf, 'import {reconcileBuildOwnerRegistry} from "../../🧾️registry-consistency/🟦️.ts";', 'import {explainOwnerlessRoot,reconcileBuildOwnerRegistry} from "../../🧾️registry-consistency/🟦️.ts";\nimport {buildRepairCommandV1} from "../../../⚙️operations/🔤️options/🟦️.ts";')
edit(rf, '''/** 🤝️ Reconciles missing or vanished owners named''', '''/** 🧹️ Adds a row for every crate or project manifest of the tree that has none (additions only: no row is removed, moved or renamed), under the registry lock, so one fault repairs every ownerless crate a peer wrote by hand instead of one per read round. */
function adoptOwnerlessCratesV1(root:string):boolean{
  return reconcileBuildOwnerRegistry(root,{write:true,additionsOnly:true}).added.length>0;
}

/** 🤝️ Reconciles missing or vanished owners named''')
edit(rf, '.replace("{cause}",String((input.cause as Error)?.message??input.cause).slice(0,400));', '.replace("{command}",buildRepairCommandV1("build-owners-sync")).replace("{cause}",String((input.cause as Error)?.message??input.cause).slice(0,400));')
src = plan.get(rf) or open(rf, encoding="utf-8").read()
j = src.index("/** 🧭️ Retries a refused native-owner lookup")
plan[rf] = src[:j] + '''/** 🔎️ The reasons the repair of a refused read left each named root without a row, plus the error the repair itself met (it is never swallowed), so the refusal says what stands in the way. */
function registryRepairCauseV1(root:string,roots:readonly string[],fault:Error,failure:unknown):Error{
  const reasons=roots.map(path=>{try{return path+": "+explainOwnerlessRoot(root,path);}catch(error){return path+": "+(error as Error).message;}});
  const repair=failure===undefined?"":"; the repair failed: "+((failure as Error)?.message??String(failure));
  return Object.assign(new Error("["+reasons.join("; ")+repair+"] "+fault.message),{cause:fault});
}

/** 🧭️ Retries a refused native-owner lookup when the registry changed or reconciliation repaired the named roots, carrying the taken memo of the read from one attempt to the next. The first refusal of a read also adds every other ownerless crate of the tree (one sweep, additions only), and only a round that added no row counts against the repair budget, so peers adding crates by hand never exhaust it. */
export async function settleOwnerRegistryV1<T>(root:string,attempt:(carried:RegistryCarryV1)=>Promise<T>,options:{aborted?:()=>boolean;say?:(line:string)=>void}={}):Promise<T>{
  const {readBuildOwnerRegistrySourceV1}=await import("../../📥️inference/🟨️.mjs"),carried:RegistryCarryV1=Object.freeze({takenMemo:createReadTakenMemoV1()});
  let stalled=0,swept=false;
  for(let round=0;;round++){
    const before=readBuildOwnerRegistrySourceV1(root);
    try{return await attempt(carried);}
    catch(error){
      if(!(error instanceof ClosedOwnerRegistryInvalidationV1))throw error;
      const fault=error.fault,message=registryFaultTextV1(fault)??fault.message,pending=pendingOwnerRootsV1(message),vanished=vanishedOwnerRootsV1(message);
      const roots=[...new Set([...pending,...vanished])];
      if(roots.length===0||options.aborted?.())throw fault;
      const current=readBuildOwnerRegistrySourceV1(root,"current");
      let moved=current!==before,adopted=false,failure:unknown;
      let unresolved:readonly string[]=roots;
      try{
        const {owners}=readBuildOwnerRegistryV1(JSON.parse(current));
        const owned=(path:string)=>owners.some(owner=>owner.root===path||owner.nativeRoots?.includes(path));
        unresolved=[...pending.filter(path=>!owned(path)),...vanished.filter(owned)];
        if(unresolved.length<roots.length)moved=true;
      }catch{moved=true;}
      try{adopted=adoptOwnerRegistryRootsV1(root,moved?unresolved:roots);}catch(problem){failure=problem;}
      if(!swept&&pending.length>0){swept=true;try{adopted=adoptOwnerlessCratesV1(root)||adopted;}catch(problem){failure??=problem;}}
      if(!moved&&!adopted)moved=readBuildOwnerRegistrySourceV1(root,"current")!==before;
      stalled=adopted?0:stalled+1;
      if(!moved&&!adopted||stalled>GRAPH_LIMITS_V1.repairRounds)throw ownerRegistryRefusalV1({roots,rounds:round,moved,cause:registryRepairCauseV1(root,roots,fault,failure)});
      clearPinnedAuthorityV1();
      emitGraphPassNoticeV1({round:round+1,moved:roots.length,evicted:0,heap:0});
      options.say?.("[graph-repair] "+JSON.stringify({round:round+1,registry:"followed",roots,adopted,moved,...(failure===undefined?{}:{failure:(failure as Error)?.message??String(failure)})})+"\\n");
    }
  }
}
'''

for p, t in plan.items():
    print(("DRY " if dry else "WRITE ") + p[-60:], len(t))
if not dry:
    for p, t in plan.items():
        tmp = f"{p}.{uuid.uuid4().hex}.tmp"
        with open(tmp, "w", encoding="utf-8") as f:
            f.write(t)
        os.replace(tmp, p)
