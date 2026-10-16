/** 🔥️ Imports every planned export through its package specifier in a fresh bun process and checks the namespace has every name the plan declares. Usage: bun facade-load.ts <plan.json> [filter] */
import { readFileSync } from "node:fs";
import { PKG_NAME, type OwnerKey } from "./model.ts";
const ROOT = "/Users/ueli/Documents/semio";
const plan = JSON.parse(readFileSync(process.argv[2], "utf8"));
const filter = process.argv[3];
const rows = plan.facades.map((f: any) => ({ spec: PKG_NAME[f.owner as OwnerKey] + (f.subpath === "." ? "" : f.subpath.slice(1)), names: f.adds.flatMap((a: any) => [...a.names.filter((n: any) => n.value).map((n: any) => n.name), ...a.json.map((j: any) => j.exportName)]) })).filter((r: any) => !filter || r.spec.includes(filter));
let next = 0, bad = 0;
async function worker() {
  while (next < rows.length) {
    const r = rows[next++];
    const code = `const m = await import(${JSON.stringify(r.spec)}); const want = ${JSON.stringify(r.names)}; const missing = want.filter(n => !(n in m)); console.log("LOADED " + Object.keys(m).length + (missing.length ? " MISSING " + missing.join(",") : ""));`;
    const p = Bun.spawn(["bun", "-e", code], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
    const timer = setTimeout(() => p.kill(), 120000);
    const [o, e] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text()]);
    await p.exited; clearTimeout(timer);
    if (/LOADED \d+/u.test(o) && !/MISSING/u.test(o)) console.log("ok", r.spec, o.trim());
    else { bad++; console.log("FAIL", r.spec, (o.includes("MISSING") ? o.trim() : e.split("\n").find(l => /error|Cannot|SyntaxError/iu.test(l)) ?? e.split("\n")[0]).slice(0, 260)); }
  }
}
await Promise.all([worker(), worker(), worker()]);
console.log({ total: rows.length, bad });
process.exit(bad ? 1 : 0);
