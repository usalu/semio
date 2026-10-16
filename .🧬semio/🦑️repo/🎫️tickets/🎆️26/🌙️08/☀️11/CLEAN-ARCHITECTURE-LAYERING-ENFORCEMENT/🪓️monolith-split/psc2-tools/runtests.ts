/** 🧪️ Runs `bun test` per file and records pass/fail sets. Usage: bun runtests.ts <files.txt> <out.json> [concurrency] */
import { readFileSync, writeFileSync } from "node:fs";
const [list, out, conc = "3"] = process.argv.slice(2);
const files = readFileSync(list, "utf8").split("\n").filter(Boolean);
const result: Record<string, any> = {};
let next = 0;
async function worker() {
  while (next < files.length) {
    const f = files[next++];
    const p = Bun.spawn(["bun", "test", "./" + f], { cwd: "/Users/ueli/Documents/semio", stdout: "pipe", stderr: "pipe" });
    const timer = setTimeout(() => p.kill(), 240000);
    const [o, e] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text()]);
    await p.exited; clearTimeout(timer);
    const text = o + "\n" + e;
    const pass = Number(text.match(/\n\s*(\d+) pass/)?.[1] ?? 0), fail = Number(text.match(/\n\s*(\d+) fail/)?.[1] ?? 0), errors = Number(text.match(/\n\s*(\d+) errors?/)?.[1] ?? 0);
    const failed = [...text.matchAll(/^\(fail\) (.*?)(?: \[[\d.]+ms\])?$/gm)].map(m => m[1]);
    const tail = pass + fail === 0 ? text.split("\n").filter(l => /error|Cannot|not found|Unexpected/i.test(l)).slice(0, 3).join(" | ").slice(0, 300) : "";
    result[f] = { exit: p.exitCode, pass, fail, errors, failed, tail };
    console.log(`${p.exitCode} pass=${pass} fail=${fail} err=${errors} ${f.slice(-80)} ${tail}`);
  }
}
await Promise.all(Array.from({ length: Number(conc) }, worker));
writeFileSync(out, JSON.stringify(result, null, 1));
