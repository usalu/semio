import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
const p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts";
const dir = p.replace(/\/[^/]+$/, "");
const text = readFileSync(p, "utf8");
const re = /from\s+"(\.\.[^"]+)"/g;
let m: RegExpExecArray | null;
while ((m = re.exec(text))) {
  const rel = m[1]!;
  const abs = resolve(dir, rel);
  console.log(`${existsSync(abs) ? "OK  " : "MISS"} ${rel}`);
}
