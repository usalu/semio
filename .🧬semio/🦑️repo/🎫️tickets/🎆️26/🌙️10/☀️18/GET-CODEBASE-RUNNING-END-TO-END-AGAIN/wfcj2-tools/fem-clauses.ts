import { readFileSync } from "node:fs";
const [lawPath, checkPath] = process.argv.slice(2);
const check = readFileSync(checkPath, "utf8").split("\n");
const s = check.findIndex((l) => l.includes("const femMountedSession = ["));
const e = check.findIndex((l) => l.includes("const femExact ="));
const vars = new Function(`${check.slice(s, e).join("\n")}; return {femMountedSession,femMountedEditor,femMountedModel,femMountedMesh,femMountedRoot,femMountedGlue,femMountedJobs,femMountedAnalyses,femMountedReactor,femMountedStore,femMountedFramework};`)();
const law = readFileSync(lawPath, "utf8").split("\n");
const ls = law.findIndex((l) => l.startsWith("export function"));
const rs = law.findIndex((l) => l.trim() === "return (");
const prelude = law.slice(ls + 1, rs).join("\n");
const clauses = law.slice(rs + 1).filter((l) => /^    \S/.test(l)).map((l) => l.trim().replace(/ &&$/, ""));
const src = `const toolJobRustBlock = globalThis.__rb; const session=v.femMountedSession, editor=v.femMountedEditor, model=v.femMountedModel, mesh=v.femMountedMesh, pluginRoot=v.femMountedRoot, glue=v.femMountedGlue, jobs=v.femMountedJobs, analyses=v.femMountedAnalyses, reactor=v.femMountedReactor, store=v.femMountedStore, frameworkPlugin=v.femMountedFramework;\n${prelude}\nreturn [${clauses.map((c) => `[${JSON.stringify(c)}, (()=>{try{return !!(${c})}catch(x){return "ERR "+x.message}})()]`).join(",")}];`;
const inv = await import(process.argv[4] ?? "");
(globalThis as any).__rb = inv.toolJobRustBlock;
const out = new Function("v", src)(vars) as [string, unknown][];
let bad = 0;
for (const [c, ok] of out) if (ok !== true) { bad++; console.log("FAIL", ok, c); }
console.log(`${out.length} clauses, ${bad} failing`);
