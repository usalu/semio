import { readFileSync } from "node:fs";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map((f) => "/Users/ueli/Documents/semio/" + f);
let bad = 0, ok = 0;
for (const f of files) {
  const r = await Bun.build({ entrypoints: [f], target: "bun", throw: false, packages: "external" });
  if (!r.success) { bad++; console.log("FAIL", f.split("/").slice(-5).join("/"), r.logs.slice(0, 2).map((l) => String(l.message).slice(0, 160)).join(" | ")); } else ok++;
}
console.log({ ok, bad });
