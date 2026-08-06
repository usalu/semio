import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

const cargoPath = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml";
const dir = cargoPath.replace(/\/Cargo\.toml$/, "");
const text = readFileSync(cargoPath, "utf8");
const pathRe = /path\s*=\s*"([^"]+)"/g;
let m: RegExpExecArray | null;
let n = 0;
let bad = 0;
while ((m = pathRe.exec(text))) {
  n++;
  const p = m[1]!;
  const abs = resolve(dir, p);
  const ok = existsSync(abs);
  if (!ok) bad++;
  console.log(`${ok ? "OK  " : "MISS"} ${p}`);
}
console.log(`\n${n} path entries checked, ${bad} missing`);
