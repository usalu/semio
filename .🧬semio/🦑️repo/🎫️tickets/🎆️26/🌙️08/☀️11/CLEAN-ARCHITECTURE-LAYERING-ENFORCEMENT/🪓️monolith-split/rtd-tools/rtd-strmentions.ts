// 🔤️ Lists mentions of the ui alias inside string literals (not comments) of the given crate's rewritten files (argv: apply report json, crate suffix). Source-text laws must not pin rewritten text.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {mask} from "./rtd-rust.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rep = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as any[];
let n = 0;
for (const c of rep) {
  if (process.argv[3] && !c.crate.endsWith(process.argv[3])) continue;
  for (const f of c.files) {
    const raw = readFileSync(join(ROOT, f), "utf8");
    const m = mask(raw);
    const rx = /ui_wgpu/g;
    let x: RegExpExecArray | null;
    while ((x = rx.exec(raw))) {
      if (m.slice(x.index, x.index + 7) === "ui_wgpu") continue;
      const lineStart = raw.lastIndexOf("\n", x.index) + 1;
      const line = raw.slice(lineStart, raw.indexOf("\n", x.index));
      const before = line.slice(0, x.index - lineStart);
      if (/\/\/|\/\*|^\s*\*/.test(before.replace(/"[^"]*"/g, ""))) continue;
      n++;
      console.log(`${c.short}: ${f.split("/").slice(-2).join("/")}: ${line.trim().slice(0, 200)}`);
    }
  }
}
console.log(n, "string mentions");
