/** 🔥️ Loads each edited `📜️script.ts` with no arguments and reports module-resolution failures (usage errors are expected). Usage: bun script-smoke.ts <list> */
import { readFileSync } from "node:fs";
const files = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean);
let next = 0, bad = 0;
async function worker() {
  while (next < files.length) {
    const f = files[next++];
    const p = Bun.spawn(["bun", "./" + f], { cwd: "/Users/ueli/Documents/semio", stdout: "pipe", stderr: "pipe", env: { ...process.env, SEMIO_SCRIPT_POLICY: "" } });
    const timer = setTimeout(() => p.kill(), 60000);
    const [o, e] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text()]);
    await p.exited; clearTimeout(timer);
    const t = o + e;
    const resolution = /Cannot find module|Could not resolve|Cannot find package|not found in module|does not provide an export|SyntaxError|Export named|ResolveMessage|BuildMessage/i.test(t);
    if (resolution) { bad++; console.log("RESOLUTION", f.slice(-80), t.split("\n").find(l => /Cannot find|resolve|SyntaxError|Export named/i.test(l))?.slice(0, 200)); }
    else console.log("ok", p.exitCode, f.slice(-60), (t.split("\n").find(l => /usage|error/i.test(l)) ?? "").slice(0, 100));
  }
}
await Promise.all([worker(), worker()]);
console.log({ files: files.length, bad });
