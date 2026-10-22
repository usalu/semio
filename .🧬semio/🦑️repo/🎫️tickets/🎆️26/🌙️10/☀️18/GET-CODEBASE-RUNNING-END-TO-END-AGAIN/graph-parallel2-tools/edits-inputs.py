L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
I = L + "/🏗️build/📥️inputs/🟨️.mjs"
D = L + "/🏗️build/📥️inputs/🟨️.d.mts"
adopt = '''/** 🛰️ Adopts the leaves a farm worker staged into the graph under this process's writer custody, in the order that keeps every reference satisfied: content-addressed closures and blocks first (the first writer wins and every later copy is verified against it), the leaves that name them last. The optional leaves of one call are one batched publication; answers how many leaves were published, how many already stood and whether the quota declined them. */
export function adoptStagedGraphLeavesV1(workspaceRoot, stagingRoot, names) {
  const root = resolve(stagingRoot), closures = [], optional = [], plan = { published: 0, present: 0, declined: false };
  for (const name of names) {
    const [namespace, ...rest] = name.split("/"), leaf = rest.join("/"), source = join(root, namespace, ...rest);
    if (!leaf || rest.some(part => part === "" || part === ".." || part === ".") || !["inference", "build-input-closures", "build-lock-closures"].includes(namespace)) throw Error("Relative staged graph leaf of a declared namespace required: " + name);
    const bytes = readFileSync(source);
    if (namespace !== "inference") {
      if (!/^[a-f0-9]{64}\\.json$/u.test(leaf) || createHash("sha256").update(bytes).digest("hex") !== leaf.slice(0, -5)) throw Error("Closure content differs from its name: " + name);
      closures.push([namespace, leaf, bytes]);
      continue;
    }
    const target = join(repositoryBuildGraphWriterDirectoryV1(workspaceRoot, "inference"), ...rest);
    let previous;
    try { previous = readFileSync(target); } catch (error) { if (error?.code !== "ENOENT") throw error; }
    const block = /(?:^|\\/)ob\\/([a-f0-9]{64})\\.br$/u.exec(leaf);
    if (block && previous !== undefined && previous.equals(bytes)) { plan.present++; continue; }
    if (block && createHash("sha256").update(brotliDecompressSync(bytes)).digest("hex") !== block[1]) throw Error("Block content differs from its name: " + name);
    optional.push({ target, bytes, previous: previous?.byteLength ?? 0, last: /(?:^|\\/)np\\/[a-f0-9]{24}\\.json$/u.test(leaf) });
  }
  for (const [namespace, leaf, bytes] of closures) publishInputClosureV1(join(repositoryBuildGraphWriterDirectoryV1(workspaceRoot, namespace), leaf), bytes);
  if (!optional.length) return plan;
  optional.sort((a, b) => Number(a.last) - Number(b.last));
  const peakBytes = optional.reduce((sum, row) => sum + row.bytes.byteLength, 0), retainedDeltaBytes = optional.reduce((sum, row) => sum + Math.min(row.bytes.byteLength, row.bytes.byteLength - row.previous), 0);
  plan.declined = !withBuildGraphOptionalPublicationsV1(optional.map(row => row.target), { peakBytes, retainedDeltaBytes }, () => {
    for (const { target, bytes } of optional) {
      const temporary = /\\.(?:pack|idx)$/u.test(target) ? `${target}.${randomBytes(4).toString("hex")}.tmp` : `${target}.${process.pid}.${Date.now()}.tmp`;
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(temporary, bytes, { flag: "wx" });
      renameSync(temporary, target);
    }
  });
  if (!plan.declined) plan.published = optional.length;
  return plan;
}

'''
EDITS = [
 (I, 'import {repositoryBuildGraphWriterDirectoryV1, withBuildGraphOptionalPublicationV1} from "./💾️storage/🟨️.mjs";', 'import {repositoryBuildGraphWriterDirectoryV1, withBuildGraphOptionalPublicationV1, withBuildGraphOptionalPublicationsV1, noteBuildGraphStagedLeafV1} from "./💾️storage/🟨️.mjs";\nimport { brotliDecompressSync } from "node:zlib";'),
 (I, 'import { createHash, randomUUID } from "node:crypto";', 'import { createHash, randomBytes, randomUUID } from "node:crypto";'),
 (I, 'try{writeFileSync(temporary,contract,{flag:"wx"});renameSync(temporary,path);verify();}finally{rmSync(temporary,{force:true});}', 'try{writeFileSync(temporary,contract,{flag:"wx"});renameSync(temporary,path);verify();noteBuildGraphStagedLeafV1(path);}finally{rmSync(temporary,{force:true});}'),
 (I, '/** 📍️ Projects the project-root sentinel into a repository-relative Cargo manifest. */', adopt + '/** 📍️ Projects the project-root sentinel into a repository-relative Cargo manifest. */'),
 (D, '/** 📍️ Projects the project-root sentinel to its exact Cargo manifest coordinate. */', '/** 🛰️ Adopts the leaves a farm worker staged into the graph under this process\'s writer custody. */\nexport function adoptStagedGraphLeavesV1(workspaceRoot:string,stagingRoot:string,names:readonly string[]):Readonly<{published:number;present:number;declined:boolean}>;\n/** 📍️ Projects the project-root sentinel to its exact Cargo manifest coordinate. */'),
]
