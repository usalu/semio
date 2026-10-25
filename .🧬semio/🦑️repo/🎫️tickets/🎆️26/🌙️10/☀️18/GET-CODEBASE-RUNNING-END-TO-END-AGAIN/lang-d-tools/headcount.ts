import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const law = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts");
for (const f of JSON.parse(readFileSync(process.argv[2]!, "utf8"))) {
  const r = spawnSync("git", ["show", `HEAD:${f.path}`], { cwd: "/Users/ueli/Documents/semio", encoding: "utf8", maxBuffer: 1 << 28 });
  console.log(r.status === 0 ? law.proseSitesV1(r.stdout) : "nohead", f.detail, f.path.slice(-70));
}
