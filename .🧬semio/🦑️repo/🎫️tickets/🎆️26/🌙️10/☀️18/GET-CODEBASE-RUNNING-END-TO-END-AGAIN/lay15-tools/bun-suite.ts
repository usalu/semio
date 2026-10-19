import { readFileSync, writeFileSync } from "node:fs";
const [listPath, outPath, conc = "2"] = process.argv.slice(2) as string[];
const files = readFileSync(listPath!, "utf8").split("\n").filter(Boolean);
const results: Record<string, { pass: number; fail: number; failed: string[]; ms: number; timeout?: boolean }> = {};
let i = 0;
const art = "/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/ta15";
const worker = async (): Promise<void> => {
  for (;;) {
    const f = files[i++];
    if (!f) return;
    const t0 = Date.now();
    const p = Bun.spawn(["bun", "test", "./" + f], { stdout: "pipe", stderr: "pipe", cwd: "/Users/ueli/Documents/semio", env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: art } });
    let timeout = false;
    const timer = setTimeout(() => { timeout = true; p.kill(); }, 150000);
    const [o, e] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text()]);
    await p.exited; clearTimeout(timer);
    const txt = o + e;
    const pass = +(/^\s*(\d+) pass/mu.exec(txt)?.[1] ?? 0), fail = +(/^\s*(\d+) fail/mu.exec(txt)?.[1] ?? 0);
    const failed = [...txt.matchAll(/^\(fail\) (.*?)(?: \[[\d.]+ms\])?$/gmu)].map(m => m[1]!);
    results[f] = { pass, fail, failed, ms: Date.now() - t0, ...(timeout ? { timeout } : {}) };
  }
};
await Promise.all(Array.from({ length: +conc }, worker));
writeFileSync(outPath!, JSON.stringify(results, null, 1));
const tot = Object.values(results).reduce((a, r) => ({ pass: a.pass + r.pass, fail: a.fail + r.fail }), { pass: 0, fail: 0 });
console.log("files", files.length, JSON.stringify(tot));
