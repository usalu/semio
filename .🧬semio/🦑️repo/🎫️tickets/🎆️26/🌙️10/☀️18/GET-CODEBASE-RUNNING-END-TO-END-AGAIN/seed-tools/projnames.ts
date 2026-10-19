import { readFileSync, existsSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { basename } from "node:path";
const R = "/Users/ueli/Documents/semio/";
const t0 = performance.now();
const ls = (a: string[]) => spawnSync("git", a, { cwd: R, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\0").filter(Boolean);
const files = ls(["ls-files", "-z", "--", "*project.json", "*Cargo.toml", "*package.json"]).filter((f) => !f.includes("node_modules/"));
const names = new Set<string>();
for (const f of files) {
  if (!existsSync(R + f)) continue;
  const b = basename(f);
  try {
    if (b === "Cargo.toml") { const n = Bun.TOML.parse(readFileSync(R + f, "utf8")) as any; if (n.package?.name) names.add(n.package.name); }
    else { const n = JSON.parse(readFileSync(R + f, "utf8")); if (n.name) names.add(n.name); }
  } catch {}
}
console.log("names", names.size, Math.round(performance.now() - t0), "ms");
const N = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const miss = new Map<string, number>();
for (const c of N.configurations) for (const m of String(c.command ?? "").matchAll(/^bun repo run ([^\s:"]+):/g)) {
  const p = m[1]!.replace(/^["']|["']$/g, "");
  if (!names.has(p) && !p.includes("${") && !p.includes("*")) miss.set(p, (miss.get(p) ?? 0) + 1);
}
console.log([...miss].slice(0, 30));
