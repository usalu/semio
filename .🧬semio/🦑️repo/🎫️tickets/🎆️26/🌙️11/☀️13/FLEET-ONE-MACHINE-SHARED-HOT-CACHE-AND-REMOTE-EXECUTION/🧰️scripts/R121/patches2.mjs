const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
export const OB=L+"👁️observation/🟨️.mjs",OBD=L+"👁️observation/🟨️.d.mts",PN=L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs",SV=L+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs",FM=L+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs";
const edit=(from,to)=>text=>{const at=text.indexOf(from);if(at<0||text.indexOf(from,at+1)>=0)throw Error("anchor not unique: "+from.slice(0,70));return text.replace(from,()=>to);};
const helpers=`
/** 🫳️ What the parent of a read did for the Cargo-manifest assembly authority of its owners: ${"`farm`"} answers a farm child gave (rows replayed into the read), ${"`parent`"} computed by the parent itself (no child, a late or failing child). */
export const ownerAuthorityCountsV1 = { farm: 0, parent: 0 };

/** 🫳️ Computes the assembly authority of one owner (the digest of the native dependency owners its Cargo manifests name and of the component roots) in a farm child, the Cargo manifests read through the observation of a detached scope, and answers the digest with the rows the owner read itself and the digests of the shared layers it read: every layer is filed once in the verdict directory of the read (${"`authority-layer-<digest>`"}), so the parent replays each distinct layer once per read and not once per owner (the layers of one workspace weigh megabytes and every owner reads the same ones). The manifests of the workspace are read in a child process and no longer on the loop of the parent.
 * @see ../🧪️tests/🫳️authority-job */
export async function authorityOfOwnerV1(environment, job) {
  const cpu = process.cpuUsage(), { workspaceRoot, projectsByRoot, componentRoots, nativeCache, nativeClosures } = environment;
  const done = authorityDetachedV1(() => projectAssemblyAuthorityV1(workspaceRoot, job.nativeRoot, projectsByRoot, job.pipeline ? componentRoots : undefined, nativeCache, nativeClosures)), layerDigests = [];
  if (typeof job.directory !== "string" && done.layers.length > 0) return { names: [], weight: 0.01 };
  for (const layer of done.layers) {
    const text = JSON.stringify(layer), digest = createHash("sha256").update(text).digest("hex"), path = join(job.directory, "authority-layer-" + digest);
    layerDigests.push(digest);
    try { rawStatSync(path); } catch { const temporary = path + "." + process.pid + ".tmp"; rawWriteFileSync(temporary, text); rawRenameSync(temporary, path); }
  }
  const used = process.cpuUsage(cpu);
  return { names: [], digest: done.value, rows: done.rows, layerDigests, verifier: process.pid, cpuMilliseconds: Math.round((used.user + used.system) / 1000), peakMegabytes: 0, weight: 0.01 };
}

/** 🫳️ One shared layer a farm child filed for the authority of its owner, or undefined when the file is gone or torn. */
export function readAuthorityLayerV1(directory, digest) {
  try { return JSON.parse(rawReadFileSync(join(directory, "authority-layer-" + digest), "utf8")); } catch { return undefined; }
}

/** 🫳️ The authority of an owner for the parent of a read: while a farm child is ready (and the read has a verdict directory) the first owner of each distinct question (native root, pipeline) asks a child once and every other owner of the read takes the same answer; the rows of the owner and each distinct shared layer are replayed once into the read; with no child, a late, a failing or a torn one the parent computes it itself, as before. ${"`farm`"} is read at each call because the farm opens after the resolver is made.
 * @see ../🧪️tests/🫳️authority-job */
export function createAuthorityResolverV1({ farm, own, retain, directory = () => closureVerdictScopeV1(), layer = readAuthorityLayerV1 }) {
  const held = new Map(), seen = new Set();
  return async (nativeRoot, pipeline) => {
    const where = directory();
    if (!(farm()?.width?.() > 0) || where === undefined) return own(nativeRoot, pipeline);
    const key = (nativeRoot ?? "") + "|" + (pipeline ? 1 : 0);
    let digest = held.get(key);
    if (digest === undefined) {
      digest = verdictWithinV1(farm().authority({ authority: true, nativeRoot, pipeline, directory: where }), OWNER_VERDICT_WAIT_MILLISECONDS_V1).then(reply => {
        if (typeof reply?.digest !== "string" || !Array.isArray(reply.rows) || !Array.isArray(reply.layerDigests)) return null;
        const fresh = [];
        for (const name of new Set(reply.layerDigests)) { if (seen.has(name)) continue; const rows = layer(where, name); if (!Array.isArray(rows)) return null; fresh.push([name, rows]); }
        retain([...reply.rows, ...fresh.flatMap(([, rows]) => rows)]);
        for (const [name] of fresh) seen.add(name);
        ownerAuthorityCountsV1.farm++;
        return reply.digest;
      });
      held.set(key, digest);
    }
    const answer = await digest;
    if (answer !== null) return answer;
    ownerAuthorityCountsV1.parent++;
    return own(nativeRoot, pipeline);
  };
}

`;
export const all=[
 {file:OB,apply:edit(`/** 🤔️ Whether the shared rows of one reused computation still hold`,`/** 🪩️ Records rows that a detached computation read in another process (a farm child) into the observation the caller runs in, so what was computed there goes stale exactly when one of those rows changes. */
export function retainObservedRowsV1(rows){retainObservationsV1(scopes.getStore(),rows);retainSourceFactsV1(rows);}
/** 🤔️ Whether the shared rows of one reused computation still hold`)},
 {file:OBD,apply:edit(`export function retainDetachedFailureV1(error:unknown):void;`,`export function retainDetachedFailureV1(error:unknown):void;
export function retainObservedRowsV1(rows:readonly unknown[]):void;`)},
 {file:PN,apply:edit(`import { brotliDecompressSync as refsBrotliV1 } from "node:zlib";`,`import { brotliDecompressSync as refsBrotliV1 } from "node:zlib";
import { observeDetachedSyncV1 as authorityDetachedV1, retainObservedRowsV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";`)},
 {file:PN,apply:edit(`
/** 🪘️ The resolved result a farm child filed for the record the parent adopted`,`${helpers}/** 🪘️ The resolved result a farm child filed for the record the parent adopted`)},
 {file:PN,apply:edit(`const inferProject=async({result,name,json},away=work=>work())=>{`,`const authorityOfV1=createAuthorityResolverV1({farm:()=>farm,own:(authorityRoot,withPipeline)=>projectAssemblyAuthorityV1(workspaceRoot,authorityRoot,projectsByRoot,withPipeline?componentRoots:undefined,nativeCache,nativeClosures),retain:retainObservedRowsV1});
  const inferProject=async({result,name,json},away=work=>work())=>{`)},
 {file:PN,apply:edit(`const assemblyAuthority=native||pipeline?projectAssemblyAuthorityV1(workspaceRoot,native?nativeRoot:undefined,projectsByRoot,pipeline?componentRoots:undefined,nativeCache,nativeClosures):null;`,`const assemblyAuthority=native||pipeline?await authorityOfV1(native?nativeRoot:undefined,pipeline):null;`)},
 {file:SV,apply:edit(`  if (job.observe === true) return observeProjectRecordV1(environment, job);`,`  if (job.observe === true) return observeProjectRecordV1(environment, job);
  if (job.authority === true) return authorityOfOwnerV1(environment, job);`)},
 {file:SV,apply:edit(`import { createProjectInferenceEnvironmentV1, observeProjectRecordV1,`,`import { authorityOfOwnerV1, createProjectInferenceEnvironmentV1, observeProjectRecordV1,`)},
 {file:FM,apply:edit(`    async observe(job) { return this.warm(job); },`,`    async observe(job) { return this.warm(job); },
    async authority(job) { return this.warm(job); },`)},
];
export const apply=(file,text,list=all)=>{for(const item of list)if(item.file===file)text=item.apply(text);return text;};
