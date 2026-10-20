import { reexportsOf } from "../exports-stage-tools/glue-ops.ts";
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
import { lane } from "../exports-stage-tools/model.ts";
import { model } from "./lib.ts";
import { listAll } from "../exports-stage-tools/lib.ts";
const policy = loadPolicy();
const G = "🧰️framework/📦️packages/🟦️typescript";
const pk = (f: string): string => { const s = ownerOfFile(policy, f); return s ? lane(s.owner)?.name ?? s.names[0]! : "-"; };
const fw = model.byName.get("@semio-tech/framework")!;
const dirs = process.argv.slice(2);
const keysByFile = new Map<string, string[]>();
for (const [k, v] of Object.entries(fw.exports)) { const s = typeof v === "string" ? v : ""; if (s) (keysByFile.get(G + "/" + s.slice(2)) ?? keysByFile.set(G + "/" + s.slice(2), []).get(G + "/" + s.slice(2))!).push(k); }
for (const d of dirs) {
  const prefix = G + "/" + d + "/";
  const files = listAll().filter(f => f.startsWith(prefix) && /\.(ts|tsx|mjs|cjs)$/u.test(f));
  const rows: string[] = [];
  let nonGlue = 0;
  for (const f of files) {
    let rx: ReturnType<typeof reexportsOf> = [];
    try { rx = reexportsOf(f); } catch { /* unreadable */ }
    const targets = [...new Set(rx.map(r => r.target).filter(Boolean) as string[])];
    const owners = [...new Set(targets.map(pk))];
    if (!targets.length) { nonGlue++; rows.push(`  NONGLUE ${f.slice(prefix.length)}`); continue; }
    rows.push(`  ${(keysByFile.get(f) ?? ["(unkeyed)"]).join(",")} -> ${owners.join("|")}${targets.length > 1 ? " [" + targets.length + " targets]" : ""}`);
  }
  console.log("==", d, "glue files", files.length, "nonglue", nonGlue);
  console.log(rows.join("\n"));
}
