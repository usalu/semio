// 🧪 Bundles every script this session wrote to prove its imports resolve and its syntax holds. usage: bun verify-written.ts <out.json>
import {readFileSync, writeFileSync} from "node:fs";
import {spawn} from "node:child_process";
const GEN = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tax2";
const own = Object.keys(JSON.parse(readFileSync(`${GEN}/own-mtimes.json`, "utf8")));
const modules = new Set<string>();
for (const l of readFileSync(`${GEN}/ledger.jsonl`, "utf8").split("\n").filter(Boolean)) { const r = JSON.parse(l); if (r.module) modules.add(r.module); }
const files = [...new Set([...own.filter((p) => p.endsWith("📜️script.ts")), ...modules])].filter((p) => { try { readFileSync("/Users/ueli/Documents/semio/" + p); return true; } catch { return false; } });
const results: Record<string, string> = {};
let i = 0;
const run = (p: string): Promise<void> => new Promise((done) => {
  const c = spawn("bun", ["build", "--target=bun", "--outdir", `${GEN}/bundle-probe`, p], {cwd: "/Users/ueli/Documents/semio", stdio: ["ignore", "pipe", "pipe"]});
  let err = ""; c.stderr.on("data", (d) => (err += d)); c.stdout.on("data", (d) => (err += d));
  c.on("close", (code) => { results[p] = code === 0 ? "ok" : err.slice(0, 400); done(); });
});
const worker = async () => { while (i < files.length) await run(files[i++]!); };
await Promise.all([worker(), worker(), worker()]);
writeFileSync(process.argv[2]!, JSON.stringify(results, null, 1));
const bad = Object.entries(results).filter(([, v]) => v !== "ok");
console.log({checked: files.length, bad: bad.length});
for (const [p, v] of bad.slice(0, 40)) console.log("BAD", p, "\n   ", v.split("\n").slice(0, 3).join(" | ").slice(0, 300));
