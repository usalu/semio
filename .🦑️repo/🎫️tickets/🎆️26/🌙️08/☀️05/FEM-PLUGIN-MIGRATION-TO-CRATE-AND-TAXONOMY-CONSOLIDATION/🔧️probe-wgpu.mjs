import { readdirSync, existsSync, statSync } from "fs";
import { join } from "path";
const p = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu";
console.log("exists", existsSync(p));
if (existsSync(p)) {
  for (const n of readdirSync(p)) {
    const s = statSync(join(p,n));
    console.log(s.isDirectory()?"D":"F", n, s.size);
  }
}
console.log("Cargo.toml?", existsSync(join(p,"Cargo.toml")));
