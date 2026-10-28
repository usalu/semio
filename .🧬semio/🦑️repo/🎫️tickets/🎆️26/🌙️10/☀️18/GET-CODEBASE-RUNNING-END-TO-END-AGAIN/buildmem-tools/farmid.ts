// [buildmem] preload: dumps the module-load authority rows of every process at its first inferenceRecordIdentityV1() call (parent AND farm worker), and makes the farm start its workers with this preload. Env BM_DUMP=<dir>. Not a library file.
import { mkdirSync, writeFileSync } from "node:fs";
const DUMP = process.env.BM_DUMP!, SELF = import.meta.path;
let dumped = false;
(globalThis as any).__bmDumpLoad = (rows: Map<string, any>, stamp: unknown, fragments: string[], identity: (row: any) => unknown, meaning: (row: any) => unknown) => {
  if (dumped) return; dumped = true; mkdirSync(DUMP, { recursive: true });
  const kept = [...rows.values()].filter(row => !(row.kind === "file" && fragments.some(f => row.path.includes(f))));
  writeFileSync(DUMP + "/" + process.pid + (process.send ? ".worker" : ".parent") + ".json", JSON.stringify({ stamp, fragments, rows: kept.map(row => [row.kind, row.path, JSON.stringify(identity(row)), JSON.stringify(meaning(row))]).sort() }));
  console.error("[DEBUG] bm dumped " + kept.length + " load rows pid " + process.pid);
};
Bun.plugin({ name: "bm-farmid", setup(b) {
  b.onLoad({ filter: /observation\/[^/]*\.mjs$/ }, async args => {
    if (!args.path.endsWith("📚️library/🏗️build/👁️observation/🟨️.mjs")) return undefined;
    const t = await Bun.file(args.path).text(), a = "export function inferenceRecordIdentityV1(){return ";
    if (!t.includes(a)) throw Error("bm identity anchor missing");
    return { contents: t.replace(a, a + "(globalThis.__bmDumpLoad?.(loadObservations,recordStampV1(),stampedFragments,observationIdentityV1,observationMeaningV1),0)||"), loader: "js" };
  });
  b.onLoad({ filter: /project-farm\/[^/]*\.mjs$/ }, async args => {
    if (!args.path.endsWith("🏊️project-farm/🟨️.mjs")) return undefined;
    const t = await Bun.file(args.path).text(), a = "...(commandArguments === undefined ? {} : { commandArguments })";
    if (!t.includes(a)) throw Error("bm farm anchor missing");
    return { contents: t.replace(a, 'commandArguments: ["--preload", ' + JSON.stringify(SELF) + "]"), loader: "js" };
  });
} });
