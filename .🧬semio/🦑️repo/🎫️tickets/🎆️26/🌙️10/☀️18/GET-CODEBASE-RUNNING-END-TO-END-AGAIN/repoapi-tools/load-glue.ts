import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, ROOT } from "./lib.ts";
const glue = JSON.parse(readFileSync(resolve(GENERATED, "glue.json"), "utf8")) as { key: string; file: string; members: { names: string[]; types: string[] }[] }[];
const only = process.argv[2];
for (const g of glue) {
  if (only && g.key !== only) continue;
  if (g.file.endsWith(".cjs")) continue;
  const t0 = performance.now();
  try {
    const m = await import(resolve(ROOT, g.file));
    const want = g.members.flatMap(x => x.names).filter(n => n !== "*");
    const missing = want.filter(n => !(n in m));
    console.log(g.key.padEnd(22), "ok", `${Math.round(performance.now() - t0)} ms`, `${Object.keys(m).length} exports`, missing.length ? "MISSING " + missing.join(",") : "");
  } catch (error) {
    console.log(g.key.padEnd(22), "FAIL", String(error).split("\n")[0]!.slice(0, 220));
  }
}
