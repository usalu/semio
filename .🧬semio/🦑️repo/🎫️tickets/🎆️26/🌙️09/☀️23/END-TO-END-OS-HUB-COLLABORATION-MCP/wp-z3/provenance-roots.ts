import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { decodeCargoDepInfo } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🧾️provenance/🟦️.ts";
const build = process.argv[2]!;
const tally = new Map<string, number>();
let files = 0;
const walk = (dir: string, depth: number): void => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory() && depth < 7 && entry.name !== "out" && entry.name !== "deps") walk(path, depth + 1);
    else if (entry.isFile() && entry.name.startsWith("dep-") && dir.endsWith("fingerprint")) {
      files++;
      for (const f of decodeCargoDepInfo(readFileSync(path))) if (f.path.startsWith("/")) { const k = f.path.split("/").slice(0, 5).join("/"); tally.set(k, (tally.get(k) ?? 0) + 1); }
    }
  }
};
walk(build, 0);
console.log(files, [...tally.entries()].sort((a, b) => b[1] - a[1]).slice(0, 20));
