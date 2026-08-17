import { readdirSync, readFileSync } from "fs";
import { join } from "path";
const MODULES="/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const interesting = [];
for (const n of readdirSync(MODULES)) {
  if (/manifest|kernel|action|platform|mesh|core/.test(n)) {
    console.log(JSON.stringify(n), Buffer.from(n).toString("hex"));
    interesting.push(n);
  }
}
const manifestDir = interesting.find(n => n.includes("manifest"));
console.log("manifestDir", manifestDir);

for (const dir of interesting) {
  const f = join(MODULES, dir, "🟦️component.ts");
  try {
    const text = readFileSync(f, "utf8");
    const bad = [...text.matchAll(/from ["']([^"']+)["']/g)]
      .map(m => m[1])
      .filter(p => p.includes("manifest") || p.includes("kernel") || p.includes("platform") || p.includes("mesh") || p.includes("action") || p.includes("generated") || p.includes("ui-axes"));
    console.log("\n==", dir, "==");
    for (const p of [...new Set(bad)]) {
      // check for replacement character or wrong bytes
      const hasBad = /�/.test(p) || p.includes("�️");
      console.log(hasBad ? "BAD" : "ok ", JSON.stringify(p));
    }
  } catch (e) {
    console.log("skip", dir, e.message);
  }
}
