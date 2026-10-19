import { readFileSync } from "node:fs";
const [listPath, conc = "4"] = process.argv.slice(2) as string[];
const files = readFileSync(listPath!, "utf8").split("\n").filter(f => /\.(ts|tsx|mjs)$/u.test(f) && !f.includes("📺️renderer"));
let i = 0, fail = 0;
const worker = async (): Promise<void> => {
  for (;;) {
    const f = files[i++];
    if (!f) return;
    const p = Bun.spawn(["bun", "build", "--target=bun", f], { stdout: "ignore", stderr: "pipe", cwd: "/Users/ueli/Documents/semio" });
    const timer = setTimeout(() => p.kill(), 120000);
    const err = await new Response(p.stderr).text();
    await p.exited; clearTimeout(timer);
    if (p.exitCode !== 0) { fail++; console.log("FAIL", f, "::", err.split("\n").slice(0, 4).join(" | ").slice(0, 400)); }
  }
};
await Promise.all(Array.from({ length: +conc }, worker));
console.log("checked", files.length, "failed", fail);
