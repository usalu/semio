import { readFileSync } from "node:fs";
const ws = "/Users/ueli/Documents/semio";
const m = await import(ws + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs");
const c = JSON.parse(readFileSync(ws + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/🔣️.json", "utf8"));
const added = [];
for (;;) {
  try { const r = await m.relativeSourceInputsAsyncV1(c, ws); console.log("OK files", r.files.length, "added", added); break; }
  catch (e) {
    const mm = /Undeclared external source import (\S+) in (.*)$/.exec(e.message);
    if (!mm) { console.log("ERR", e.message); break; }
    const s = mm[1], n = s.startsWith("@") ? s.split("/").slice(0, 2).join("/") : s.split("/")[0];
    console.log("missing", n, "via", s, "in", mm[2].replace(ws, ""));
    added.push(n); c.externalDependencies = [...c.externalDependencies, n];
  }
}
