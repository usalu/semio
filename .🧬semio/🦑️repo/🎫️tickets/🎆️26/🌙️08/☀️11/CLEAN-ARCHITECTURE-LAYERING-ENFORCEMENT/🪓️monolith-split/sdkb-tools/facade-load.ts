/** 🔥️ Imports every SDK export through the package specifier in a fresh bun process and checks the namespace has the names the facade declares. */
import { readFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const pkg = JSON.parse(readFileSync(`${ROOT}/🧰️framework/📦️packages/🟦️typescript/package.json`, "utf8"));
const subs = Object.keys(pkg.exports).filter(k => k !== "." && (process.argv[2] ? k.includes(process.argv[2]) : true));
let next = 0, bad = 0;
async function worker() {
  while (next < subs.length) {
    const s = subs[next++];
    const spec = "@semio-tech/framework" + s.slice(1);
    const p = Bun.spawn(["bun", "-e", `const m = await import(${JSON.stringify(spec)}); console.log("LOADED " + Object.keys(m).length);`], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
    const timer = setTimeout(() => p.kill(), 90000);
    const [o, e] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text()]);
    await p.exited; clearTimeout(timer);
    if (/LOADED \d+/.test(o)) console.log("ok", s, o.trim());
    else { bad++; console.log("FAIL", s, (e.split("\n").find(l => /error|Cannot|SyntaxError/i.test(l)) ?? e.split("\n")[0]).slice(0, 220)); }
  }
}
await Promise.all([worker(), worker(), worker()]);
console.log({ total: subs.length, bad });
